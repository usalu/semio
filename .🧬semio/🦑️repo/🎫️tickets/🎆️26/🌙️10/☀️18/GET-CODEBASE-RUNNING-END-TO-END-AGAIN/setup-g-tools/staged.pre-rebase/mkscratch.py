"""Builds a scratch copy of the checkout in which only the overlay files are real files and every other entry is a symlink to the checkout, so the staged batch can be tested before it lands. usage: python3 mkscratch.py <scratch-dir>   (the directory is emptied first; it must lie outside the checkout)"""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
OVER = os.path.join(HERE, "overlay")
target = os.path.abspath(sys.argv[1])
if target.startswith(ROOT + "/"): raise SystemExit("scratch must lie outside the checkout")
files = []
for base, _dirs, names in os.walk(OVER):
    for name in names: files.append(os.path.relpath(os.path.join(base, name), OVER))
spine = {""}
for f in files:
    parts = f.split(os.sep)
    for i in range(1, len(parts)): spine.add(os.sep.join(parts[:i]))
if os.path.lexists(target): shutil.rmtree(target)
leaves = {os.path.dirname(f) for f in files}
def build(rel):
    here = os.path.join(target, rel); os.makedirs(here, exist_ok=True)
    real = os.path.join(ROOT, rel)
    names = set(os.listdir(real)) if os.path.isdir(real) else set()
    for f in files:
        d = os.path.dirname(f)
        if d == rel: names.add(os.path.basename(f))
    for s in spine:
        if s and os.path.dirname(s) == rel: names.add(os.path.basename(s))
    for name in sorted(names):
        child = os.path.join(rel, name) if rel else name
        if rel == "" and name == "bunfig.toml": continue
        real_child = os.path.join(ROOT, child)
        if child in spine: build(child)
        elif child in files: shutil.copy2(os.path.join(OVER, child), os.path.join(target, child))
        elif ((rel != "" and rel in spine) or child in ("package.json", "📋️project.json")) and os.path.isfile(real_child) and not os.path.islink(real_child): shutil.copy2(real_child, os.path.join(target, child))
        else: os.symlink(real_child, os.path.join(target, child))
build("")
print(target, len(files), "overlay files")
