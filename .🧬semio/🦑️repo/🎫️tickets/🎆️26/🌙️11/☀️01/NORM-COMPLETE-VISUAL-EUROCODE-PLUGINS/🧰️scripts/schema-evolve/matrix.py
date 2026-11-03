#!/usr/bin/env python3
"""🧮 Matrix: in-process dry runs of add-collection (throwaway spec) and add-leaf against every artifact; prints the UNSUPPORTED layers (python3 -I matrix.py [--leaf] [artifact ...])."""
import json, os, sys, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import probe, snapshot
from core import Unsupported

WORK = tempfile.mkdtemp(prefix="evolve-matrix-")


def leaf_collections(short):
    import ctx as c
    from spec import Spec
    spec = Spec.__new__(Spec)
    spec.reference = spec.nested_reference = None
    spec.collection = None
    return [p for p, _ in c.Ctx(short, spec).collections]


def main(argv):
    leaf = "--leaf" in argv
    reasons, total = {}, {}
    names = [a for a in argv if not a.startswith("--")] or probe.ALL
    for short in names:
        try:
            if leaf:
                for plural in leaf_collections(short):
                    path = os.path.join(WORK, f"{short}-{plural}.json")
                    json.dump({"leaf": {"field": {"name": "probe_value", "type": "f64", "unit": "m", "label": probe.LB("Probe value"), "sample": 0.0}, "entity": {"singular": probe.LB("probe entity")}}}, open(path, "w"))
                    try:
                        ctx = snapshot.plan_of(short, "add-leaf", path, plural)
                    except Exception as error:
                        reasons.setdefault(short, {}).setdefault(f"EXCEPTION {type(error).__name__}: {str(error)[:150]}", []).append(plural)
                        total.setdefault(short, [0, 0])
                        total[short][0] += 1
                        total[short][1] += 1
                        continue
                    reasons.setdefault(short, {})
                    for u in ctx.unsupported:
                        reasons[short].setdefault(u[:200], []).append(plural)
                    total.setdefault(short, [0, 0])
                    total[short][0] += 1
                    total[short][1] += bool(ctx.unsupported)
            else:
                path = os.path.join(WORK, f"{short}-coll.json")
                json.dump(probe.collection_spec(short), open(path, "w"))
                ctx = snapshot.plan_of(short, "add-collection", path)
                print(f"{short} collection (ref {ctx.ref.plural_camel}): {len(ctx.plan.files)} files, {len(ctx.unsupported)} unsupported")
                for u in ctx.unsupported:
                    print("     ", u[:230])
        except Unsupported as error:
            print(short, "ERROR", str(error)[:230])
    if leaf:
        summary(reasons, total)


def summary(reasons, total):
    for short in total:
        print(f"{short} leaf: {total[short][1]} of {total[short][0]} collections report unsupported layers")
        for reason, plurals in reasons[short].items():
            print(f"     {len(plurals):>3}x {reason}  e.g. {plurals[0]}")


if __name__ == "__main__":
    main(sys.argv[1:])
