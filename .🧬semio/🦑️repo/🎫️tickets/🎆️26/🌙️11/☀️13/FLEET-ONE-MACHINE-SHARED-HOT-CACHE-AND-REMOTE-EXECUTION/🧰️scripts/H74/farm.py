"""H74: builds a symlink farm of the live checkout at FARM where the given repo-relative paths are real (copied) files/dirs, so a staged module resolves its relative imports without touching the live tree. Usage: farm.py FARM path..."""
import os, shutil, sys
LIVE = "/Users/ueli/Documents/semio"
farm, real = sys.argv[1], sys.argv[2:]
def build(rel, live_dir, farm_dir, real_paths):
    os.makedirs(farm_dir, exist_ok=True)
    wanted = {p.split("/")[0] for p in real_paths}
    for name in os.listdir(live_dir):
        src, dst = os.path.join(live_dir, name), os.path.join(farm_dir, name)
        if name in wanted:
            continue
        if not os.path.lexists(dst):
            os.symlink(src, dst)
    for first in wanted:
        rest = [p.split("/", 1)[1] for p in real_paths if p.split("/")[0] == first and "/" in p]
        src, dst = os.path.join(live_dir, first), os.path.join(farm_dir, first)
        if not rest:
            if os.path.lexists(dst):
                continue
            if os.path.isdir(src): shutil.copytree(src, dst, symlinks=True)
            elif os.path.exists(src): shutil.copy2(src, dst)
            else: os.makedirs(dst, exist_ok=True)
        else:
            if os.path.islink(dst): os.unlink(dst)
            build(rel + first + "/", src, dst, rest)
build("", LIVE, farm, real)
