"""Builds the graph-free overlay of the xplat3-g batch: every entry of the repository is a symlink except the directories on the way to a staged file; staged files are real copies. Usage: python3 -I overlay.py <overlay-dir>"""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
TOOLS = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(TOOLS, "staged", "files")
OV = sys.argv[1]
shutil.rmtree(OV, ignore_errors=True)
files = []
for base, _, names in os.walk(STAGED):
    for name in names:
        if name != ".DS_Store":
            files.append(os.path.relpath(os.path.join(base, name), STAGED))
tree = {}
for rel in files:
    node = tree
    parts = rel.split(os.sep)
    for part in parts[:-1]:
        node = node.setdefault(part, {})
    node[parts[-1]] = None
EXTRA = [line.strip() for line in open(os.path.join(TOOLS, "overlay-extra.txt"), encoding="utf-8")] if os.path.exists(os.path.join(TOOLS, "overlay-extra.txt")) else []
def build(real, over, node):
    os.makedirs(over, exist_ok=True)
    existing = os.listdir(real) if os.path.isdir(real) else []
    inside = over != OV
    for entry in existing:
        if entry in node:
            continue
        source = os.path.join(real, entry)
        relative = os.path.relpath(source, ROOT)
        if os.path.isdir(source) and not os.path.islink(source) and relative in EXTRA:
            shutil.copytree(source, os.path.join(over, entry), symlinks=True)
        elif inside and os.path.isfile(source) and not os.path.islink(source) and os.path.getsize(source) < 2_000_000:
            shutil.copyfile(source, os.path.join(over, entry))
        else:
            os.symlink(source, os.path.join(over, entry))
    for name, child in node.items():
        if child is None:
            shutil.copyfile(os.path.join(STAGED, os.path.relpath(os.path.join(over, name), OV)), os.path.join(over, name))
        else:
            build(os.path.join(real, name), os.path.join(over, name), child)
build(ROOT, OV, tree)
print(OV, len(files), "staged files")
