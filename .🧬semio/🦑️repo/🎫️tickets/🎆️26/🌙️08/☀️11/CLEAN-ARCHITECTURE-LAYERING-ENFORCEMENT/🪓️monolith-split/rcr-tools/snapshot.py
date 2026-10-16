"""Copy each target module tree (rs files plus small side files) into the baseline snapshot, once, before any edit."""
import os, shutil, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *

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


def snap_dir(name):
    return f"{W}/{name}/snap"


if __name__ == "__main__":
    for name in (sys.argv[1:] or TARGETS):
        t = TARGETS[name]
        d = snap_dir(name)
        if os.path.exists(d):
            print("snapshot exists", d)
            continue
        n = copy_tree(t["tree"], d)
        h = sha(read(os.path.join(ROOT, t["lib"])))
        write(f"{W}/{name}/snap.json", json.dumps(dict(file=t["lib"], sha=h, files=n)))
        print(name, n, "files", h[:12])
