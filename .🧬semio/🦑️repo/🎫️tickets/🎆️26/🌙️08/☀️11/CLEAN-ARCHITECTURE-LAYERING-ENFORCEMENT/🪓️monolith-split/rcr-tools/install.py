"""Installs the staged split into the real tree: children first (new paths, never overwritten), then the root through an atomic replace, only if the live root still equals the original."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC

names = list(TARGETS) if sys.argv[1] == "all" else sys.argv[1].split(",")
for name in names:
    t = TARGETS[name]
    live = f"{ROOT}/{t['lib']}"
    orig = read(f"{F}/rcr-tools/orig/{name}.rs.txt")
    cur = read(live)
    if cur != orig:
        print(f"SKIP {name}: live root differs from the original (a peer edited it); not installing")
        continue
    stage_pk = f"{W}/{name}/stage/{t['pkgdir']}"
    real_pk = f"{ROOT}/{t['pkgdir']}"
    plan = []
    for c in SPEC[name]["children"]:
        dst = f"{real_pk}/{c['folder']}/🦀️.rs"
        src = f"{stage_pk}/{c['folder']}/🦀️.rs"
        assert not os.path.exists(dst), f"child exists: {dst}"
        plan.append((src, dst))
    for src, dst in plan:
        write(dst, read(src))
    root_new = read(f"{stage_pk}/🦀️.rs")
    if read(live) != orig:
        print(f"ABORT {name}: root changed while installing children; children left in place, root untouched")
        continue
    tmp = live + ".rcr-tmp"
    write(tmp, root_new)
    os.replace(tmp, live)
    ok = read(live) == root_new and all(read(d) == read(s) for s, d in plan)
    print(f"INSTALLED {name}: root + {len(plan)} children; verified identical to stage: {ok}")
