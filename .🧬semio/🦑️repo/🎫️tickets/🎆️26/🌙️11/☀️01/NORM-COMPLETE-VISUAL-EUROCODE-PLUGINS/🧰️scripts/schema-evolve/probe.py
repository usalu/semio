#!/usr/bin/env python3
"""🔬 Probe: runs a dry-run add-collection and add-leaf with throwaway specs against artifacts and lists the UNSUPPORTED layers (python3 -I probe.py [artifact ...])."""
import json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from core import Layout, Unsupported
ALL = ["en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"]
LB = lambda en, de=None: {"en": en, "de": de or en + " DE"}


TYPES = {"en1991": ("f64", "String"), "en1994": ("f64", "enum")}


def collection_spec(short):
    spec = full_spec()
    keep = TYPES.get(short)
    if keep:
        spec["collection"]["fields"] = [f for f in spec["collection"]["fields"] if f["type"] in keep]
    return spec


def full_spec():
    return {"collection": {"name": "probeRows", "entity": "ProbeRow", "emoji": "🧪", "doc": "Probe entity.", "label": LB("Probe rows"), "id": LB("Probe ID"), "singular": LB("probe row"), "kindSlug": "probe-row",
        "fields": [{"name": "span", "type": "f64", "unit": "m", "label": LB("Span"), "sample": 1.0}, {"name": "kind", "type": "enum", "label": LB("Kind"), "enum": {"name": "ProbeKind", "values": [{"token": "a", "label": LB("A")}, {"token": "b", "label": LB("B")}]}}, {"name": "note", "type": "String", "label": LB("Note")}]}}


def leaf_target(short):
    from ctx import Ctx
    from spec import Spec
    spec = Spec.__new__(Spec)
    spec.reference = spec.nested_reference = None
    spec.collection = None
    ctx = Ctx(short, spec)
    return ctx.collections


def run(short, command, spec_path, extra=()):
    cmd = [sys.executable, "-I", HERE + "/evolve.py", command, short, *extra, spec_path, "--dry-run"]
    done = subprocess.run(cmd, capture_output=True, text=True)
    out = done.stdout + done.stderr
    return [l.strip() for l in out.splitlines() if "UNSUPPORTED" in l or "Traceback" in l or "Error" in l], out.splitlines()[-1] if out else ""


def main(argv):
    work = tempfile.mkdtemp(prefix="evolve-probe-")
    names = argv or ALL
    for short in names:
        spec = os.path.join(work, short + "-coll.json")
        json.dump(collection_spec(short), open(spec, "w"))
        bad, tail = run(short, "add-collection", spec)
        print(f"== {short} add-collection: {tail}")
        for b in bad:
            print("   ", b[:260])
        try:
            colls = leaf_target(short)
        except Unsupported as e:
            print("   leaf: ", e)
            continue
        for plural, entity in colls[:]:
            leaf = os.path.join(work, short + "-" + plural + "-leaf.json")
            json.dump({"leaf": {"field": {"name": "probe_value", "type": "f64", "unit": "m", "label": LB("Probe value"), "sample": 0.0}, "entity": {"singular": LB("probe entity")}}}, open(leaf, "w"))
            bad, tail = run(short, "add-leaf", leaf, (plural,))
            print(f"== {short} add-leaf {plural}: {tail}")
            for b in bad:
                print("   ", b[:260])
            if len(argv) != 1:
                break


if __name__ == "__main__":
    main(sys.argv[1:])
