#!/usr/bin/env python3
# ▶️ One rank of the flip: pull the re-homed items into the crate, transform and mount its modules, update the Shell parent and the manifest.
# usage: run_flip.py <crate-suffix> [--dry]
import sys, os, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import flip as F, mv

suffix = sys.argv[1]
if "--dry" in sys.argv:
    res, srcs = mv.pull(suffix, dry=True)
    print("pull (dry): new modules", [r["name"] for r in res])
    print(F.flip(suffix, dry=True)[0][:3000])
    sys.exit(0)
st = F.load_state()
if suffix in st["flipped"]:
    raise SystemExit(f"{suffix} already flipped")
backup_paths = []
mv.TOUCHED.clear()
res, srcs = mv.pull(suffix, dry=False)
st = F.load_state()
st["pulled"][suffix] = res
touched_files = set(mv.TOUCHED)
F.save_state(st)
added = F.apply(suffix, glob_files=touched_files)
print("pulled modules:", [r["name"] for r in res])
print("sources touched:", len(srcs))
print("deps added:", added)
