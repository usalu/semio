"""Parse proof: `rustc -Zunpretty=normal (parse only, modules loaded)` (loads out-of-line modules) and `rustfmt --check` over an unpatched baseline/stage/live tree.
usage: parsecheck.py <name> <A|stage|live>"""
import os, shutil, subprocess, sys, re
sys.path.insert(0, os.path.dirname(__file__))
from pqslib import *
from snapshot import copy_tree
from oracle import sandbox_under, TOOLCHAIN

name, which = sys.argv[1], sys.argv[2]
t = TARGETS[name]
root = f"{W}/{name}/parse-{which}"
sandbox_under(root)
if os.path.exists(root):
    shutil.rmtree(root)
if which == "live":
    copy_tree(t["tree"], root)
else:
    shutil.copytree(snap_dir(name), root, symlinks=True)
    if which == "stage":
        sd = stage_dir(name)
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                p = os.path.join(dp, f)
                dst = os.path.join(root, os.path.relpath(p, sd))
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copyfile(p, dst)
lib = os.path.join(root, t["lib"])
p = subprocess.run(["rustc", TOOLCHAIN, "-Zunpretty=normal", "--edition", "2021", "--crate-type", "lib", lib], capture_output=True, text=True, cwd=root)
errs = [l for l in p.stderr.split("\n") if l.startswith("error")]
print(f"[{name}/{which}] rustc -Zunpretty=normal (parse only, modules loaded): exit={p.returncode} errors={len(errs)}", errs[:5])
p2 = subprocess.run(["rustfmt", "--edition", "2021", "--check", lib], capture_output=True, text=True, cwd=root)
fe = [l for l in (p2.stderr + p2.stdout).split("\n") if l.startswith("error")]
print(f"[{name}/{which}] rustfmt --check: exit={p2.returncode} parse-errors={len(fe)} diff-hunks={p2.stdout.count('Diff in')}", fe[:5])
