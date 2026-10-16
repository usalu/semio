"""Copy a target crate source tree (rs files plus small side files) into the baseline snapshot, once, before any edit."""
import os, shutil, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

SKIP = {"node_modules", "target", "🗑️generated", ".git", "dist"}


def copy_tree(src_rel, dst_root):
    n = 0
    base = os.path.join(ROOT, src_rel)
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            p = os.path.join(dp, f)
            if os.path.islink(p):
                continue
            if not f.endswith(".rs") and os.path.getsize(p) > 300_000:
                continue
            rel = os.path.relpath(p, ROOT)
            dst = os.path.join(dst_root, rel)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(p, dst)
            n += 1
    return n


if __name__ == "__main__":
    name = sys.argv[1]
    t = TARGETS[name]
    d = snap_dir(name)
    if os.path.exists(d) and "--force" not in sys.argv:
        print("snapshot exists", d)
        sys.exit(0)
    if os.path.exists(d):
        shutil.rmtree(d)
    n = copy_tree(t["tree"], d)
    h = sha(read(os.path.join(ROOT, t["file"])))
    write(f"{W}/{name}/snap.json", json.dumps(dict(file=t["file"], sha=h, files=n)))
    print(name, n, "files", h[:12])
