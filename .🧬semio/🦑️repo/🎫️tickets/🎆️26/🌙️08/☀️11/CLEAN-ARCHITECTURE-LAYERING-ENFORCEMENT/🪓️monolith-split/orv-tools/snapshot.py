"""Baseline snapshot of the orv targets: every .rs/side file of each crate closure (module-tree walk) copied to W/<name>/snap/<repo-relative path>. Once, before any edit."""
import os, shutil, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from reach import reach


def closure_files(root_abs, text_override=None):
    order, side, children, plain = reach(root_abs)
    return order, sorted(side), children


def copy_files(paths, dst_root):
    n = 0
    for p in paths:
        if not os.path.isfile(p) or os.path.islink(p):
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
        order, side, children = closure_files(f"{ROOT}/{t['root']}")
        n = copy_files(order + side, d)
        write(f"{W}/{name}/snap.json", json.dumps(dict(file=t["file"], sha=sha(read(f"{ROOT}/{t['file']}")), files=n, rs=[os.path.relpath(p, ROOT) for p in order], side=[os.path.relpath(p, ROOT) for p in side]), ensure_ascii=False))
        print(name, n, "files", sha(read(f"{ROOT}/{t['file']}"))[:12])
