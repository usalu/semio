"""Shared driver of the rf8 plans: builds, stages, and prints the multiset and per-child reports; checks every folder emoji against the repo folder-emoji set."""
import os, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k
EMOJIS = {l.split("\t")[0] for l in read(F + "/🗑️generated/rf8-folder-emojis.tsv").split("\n") if l}


def run(SPEC, apply_stub=True):
    for c in SPEC["children"]:
        em = c["folder"].split("️")[0] + "️"
        assert em in EMOJIS, f"folder emoji not used as a folder prefix in the repo: {c['folder']}"
        assert c["doc"][0].startswith("//! "), c["folder"]
    firsts = [c["doc"][0][4] for c in SPEC["children"]]
    assert len(set(firsts)) == len(firsts), f"duplicate docstring emoji {firsts}"
    folders = [c["folder"].split("️")[0] for c in SPEC["children"]]
    assert len(set(folders)) == len(folders), "duplicate folder emoji among siblings"
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and k.count("/") > 0 and not k == plan.t["file"] else "PARENT": len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:3000])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:5000])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:200])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])
        if r.get("super_refs"):
            print("   SUPER/SELF REFS", r["super_refs"][:12])
        if r.get("pub_super_converted"):
            print("   pub(super) converted", r["pub_super_converted"])
    return plan
