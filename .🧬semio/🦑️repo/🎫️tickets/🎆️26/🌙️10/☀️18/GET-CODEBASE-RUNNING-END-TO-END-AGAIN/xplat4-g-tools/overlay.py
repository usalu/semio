"""🗂️ Builds a graph-free overlay of the repository for single-file runs: every entry is a symlink except the directories on the way to a staged file; staged files are real copies. Later staged roots override earlier ones (xplat3-g's batch first, then this lane's).
Usage: python3 -I overlay.py <overlay-dir> <staged-files-dir>..."""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
OV = sys.argv[1]
ROOTS = sys.argv[2:]
shutil.rmtree(OV, ignore_errors=True)
chosen = {}
for staged in ROOTS:
    for base, _, names in os.walk(staged):
        for name in names:
            if name != ".DS_Store":
                path = os.path.join(base, name)
                chosen[os.path.relpath(path, staged)] = path
tree = {}
for rel in chosen:
    node = tree
    parts = rel.split(os.sep)
    for part in parts[:-1]:
        node = node.setdefault(part, {})
    node[parts[-1]] = None
COPY = ["🧰️framework/🔨️modules/🏃️process", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"]
SKIP = {"node_modules", "target", ".cache"}
def build(real, over, node, rel):
    os.makedirs(over, exist_ok=True)
    if rel in COPY:
        for base, dirs, names in os.walk(real):
            dirs[:] = [d for d in dirs if d not in SKIP]
            target = os.path.join(over, os.path.relpath(base, real))
            os.makedirs(target, exist_ok=True)
            for name in names:
                source = os.path.join(base, name)
                if os.path.islink(source):
                    os.symlink(os.readlink(source), os.path.join(target, name)) if not os.path.exists(os.path.join(target, name)) else None
                else:
                    shutil.copyfile(source, os.path.join(target, name))
        for name, child in node.items():
            key = os.path.join(rel, name)
            apply(os.path.join(over, name), key, child)
        return
    existing = os.listdir(real) if os.path.isdir(real) else []
    inside = over != OV
    for entry in existing:
        if entry in node:
            continue
        source = os.path.join(real, entry)
        if inside and os.path.isfile(source) and not os.path.islink(source) and os.path.getsize(source) < 2_000_000:
            shutil.copyfile(source, os.path.join(over, entry))
        else:
            os.symlink(source, os.path.join(over, entry))
    for name, child in node.items():
        key = os.path.join(rel, name) if rel else name
        if child is None:
            shutil.copyfile(chosen[key], os.path.join(over, name))
        else:
            build(os.path.join(real, name), os.path.join(over, name), child, key)
def apply(path, key, child):
    if child is None:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        shutil.copyfile(chosen[key], path)
    else:
        for name, grandchild in child.items():
            apply(os.path.join(path, name), os.path.join(key, name), grandchild)
build(ROOT, OV, tree, "")
print(OV, len(chosen), "staged files")
