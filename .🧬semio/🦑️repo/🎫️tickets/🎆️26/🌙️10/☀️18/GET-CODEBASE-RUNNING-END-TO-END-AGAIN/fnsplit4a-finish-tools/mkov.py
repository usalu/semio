"""🧱 Builds a write-isolated overlay of the repository: every ancestor of a target directory is a real directory whose entries are symlinks to the live tree, except the path to the target; a target directory is a real directory holding copies of its files and symlinks to its subdirectories. Tools run with cwd=<overlay> never write into the live tree. usage: mkov.py <overlay> <repoRoot> <targetDir>... (repo-relative; --add keeps an existing overlay)"""
import os, shutil, sys

args = sys.argv[1:]
add = "--add" in args
args = [a for a in args if a != "--add"]
ov, root, targets = args[0], args[1], [t.rstrip("/") for t in args[2:]]
assert ov.startswith("/private/tmp/fnsplit4a-finish/"), ov
if os.path.isdir(ov) and not add:
    shutil.rmtree(ov)
os.makedirs(ov, exist_ok=True)

def build(rel, node):
    dst, src = os.path.join(ov, rel), os.path.join(root, rel)
    if os.path.islink(dst):
        os.unlink(dst)
    os.makedirs(dst, exist_ok=True)
    isTarget = "__target__" in node
    for name in sorted(os.listdir(src)):
        if name.startswith(".") and rel == "":
            continue
        s, d = os.path.join(src, name), os.path.join(dst, name)
        if name in node:
            build(os.path.join(rel, name), node[name])
        elif os.path.lexists(d) and not os.path.islink(d):
            continue
        elif isTarget and os.path.islink(s):
            os.symlink(os.readlink(s), d)
        elif isTarget and not os.path.isdir(s):
            if os.path.islink(d):
                os.unlink(d)
            shutil.copy2(s, d)
        elif not os.path.lexists(d):
            os.symlink(s, d)

t = {}
for p in targets:
    node = t
    for part in p.split("/"):
        node = node.setdefault(part, {})
    node["__target__"] = True
build("", t)
print("overlay", ov, "targets", len(targets))
