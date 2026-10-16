"""usage: run.py <stage|apply> [services,transport,remote,bridge] ; stage writes only inside osm-work/stage-*."""
import os, shutil, sys, json, collections
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
import osmeng, plans

mode = sys.argv[1]
names = sys.argv[2].split(",") if len(sys.argv) > 2 else ["services", "transport", "remote", "bridge"]
hashes = json.loads(read(f"{W}/snap-hashes.json"))
applied_path = f"{W}/applied-hashes.json"
applied = json.loads(read(applied_path)) if os.path.exists(applied_path) else {}
report = {}
for name in names:
    files = plans.GROUPS[name]()
    snap = "services" if name == "services" else "mcp"
    for f in files:
        key = "services-tests" if f.label == "services-tests" else f.label
        cur = sha(read(os.path.join(ROOT, f.rel)))
        expect = applied.get(f.rel, hashes[key])
        if cur != expect:
            print(f"WARNING {f.label}: live file changed since the last known state ({cur[:8]} vs {expect[:8]})")
            if mode == "apply":
                sys.exit("refusing to apply over a changed file: " + f.rel)
    if mode == "stage":
        stage = f"{W}/stage-{snap}"
        if not os.path.exists(stage):
            shutil.copytree(f"{W}/snap-{snap}", stage, symlinks=True)
        outroot = stage
    else:
        outroot = ROOT
    written = osmeng.run_group(files, outroot, srcroot=f"{W}/snap-{snap}")
    if mode == "apply":
        for f in files:
            applied[f.rel] = sha(read(os.path.join(ROOT, f.rel)))
        write(applied_path, json.dumps(applied, indent=1, ensure_ascii=False))
    for f in files:
        print(f"[{f.label}] pool={len(f.pool)} children={len(f.ctxs)-1}")
        for c in f.ctxs:
            print(f"   {c.mod or '<root>':28} body={len(c.body):5} defs={len(c.defs):3} widened={len(c.widened):3} leaves={len(c.leaves):3} sibs={len(c.sibs):3} via_trait={getattr(c,'via_trait',[])}")
    report[name] = {k: len(v.split("\n")) for k, v in written.items()}
    for k, v in sorted(written.items()):
        print("  ", len(v.split("\n")), k[-90:])
