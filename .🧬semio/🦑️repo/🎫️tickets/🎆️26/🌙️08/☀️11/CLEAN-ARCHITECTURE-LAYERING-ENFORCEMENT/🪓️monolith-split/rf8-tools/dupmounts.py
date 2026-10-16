"""Duplicate-mount scan: a file mounted (`#[path]` or `include!`) more than once inside one crate closure. Reports live duplicates and flags the new ones (absent in the snapshot)."""
import collections, json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from splitter import crate_files, path_mounts


def mounts(lib):
    c = collections.Counter()
    for p in crate_files(lib):
        for kind, target in path_mounts(read(p), os.path.dirname(p)):
            if os.path.isfile(target):
                c[os.path.relpath(target, os.path.commonpath([lib, target]))] += 0
                c[target] += 1
    return {k: v for k, v in c.items() if v}


bad = 0
seen = set()
for name, t in TARGETS.items():
    if not os.path.exists(f"{W}/{name}/installed.json"):
        continue
    key = fam(name)
    if key in seen:
        continue
    seen.add(key)
    live = {k.replace(ROOT + "/", ""): v for k, v in mounts(ROOT + "/" + t["lib"]).items()}
    base = {k.replace(snap_dir(name) + "/", ""): v for k, v in mounts(snap_dir(name) + "/" + t["lib"]).items()}
    dl = {k: v for k, v in live.items() if v > 1}
    db = {k: v for k, v in base.items() if v > 1}
    new = {k: v for k, v in dl.items() if k not in db}
    bad += len(new)
    print(f"[{name}] mounted files: live={len(live)} baseline={len(base)} duplicates: live={len(dl)} baseline={len(db)} new={len(new)}", list(new)[:3])
print("DUPMOUNTS", "OK (0 new)" if bad == 0 else f"{bad} NEW")
