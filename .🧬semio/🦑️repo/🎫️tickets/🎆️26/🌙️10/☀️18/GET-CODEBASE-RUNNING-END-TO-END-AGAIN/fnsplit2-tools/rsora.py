"""🔬 Resolution differential for fnsplit2 Rust splits (reuses the c6fin2 oracle5 machinery).
usage: rsora.py <spec.json> [cfg,...]   spec: {tag, crate, copy:[dir...], root, restore:{relpath: snapshotRelativeToTools}, remove:[relpath...], cfgs:[lib,test,testkit]}
Sandbox A = live copy with the snapshots restored and the new folders removed; sandbox B = live copy. rustc +nightly-2026-07-20 --emit=metadata on both, error and warning multisets compared."""
import json, os, shutil, sys
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c6fin2-tools"
sys.path.insert(0, T)
import oracle5 as O
HERE = os.path.dirname(os.path.abspath(__file__))
spec = json.load(open(sys.argv[1], encoding="utf8"))
O.SBX = "/Users/ueli/.cache/fnsplit2/sbx"
O.OUT = "/Users/ueli/.cache/fnsplit2/orc"
O.CONFIGS["testkit"] = ["--cfg", 'feature="testkit"']
O.CONFIGS["testtestkit"] = ["--test", "--cfg", 'feature="testkit"']
O.GROUPS[spec["tag"]] = dict(crate=spec["crate"], copy=spec["copy"], root=spec["root"], tags=[], cfgs=spec.get("cfgs", ["lib", "test"]))
def baseline_into(g, dst):
    for rel, snap in spec["restore"].items():
        p = os.path.join(dst, rel)
        O.under(os.path.dirname(p))
        os.makedirs(os.path.dirname(p), exist_ok=True)
        shutil.copyfile(os.path.join(HERE, snap), p)
    for rel in spec["remove"]:
        d = os.path.join(dst, rel)
        O.under(d)
        shutil.rmtree(d, ignore_errors=True)
O.baseline_into = baseline_into
sys.argv = ["rsora.py", spec["tag"]] + sys.argv[2:]
O.main()
