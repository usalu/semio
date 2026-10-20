"""Copies installed packages and their dependency closure from a checkout's node_modules into <dest>/node_modules as real directories (never symlinks), so a throw-away overlay can run tests without touching the checkout; usage: make-modules.py <repo> <dest> pkg..."""
import json, os, shutil, sys
repo, dest, *wanted = sys.argv[1:]
modules = os.path.join(repo, "node_modules")
target = os.path.join(dest, "node_modules")
done = set()
def resolve(name, base):
    for directory in [base, modules]:
        candidate = os.path.join(directory, *name.split("/"))
        if os.path.exists(candidate): return os.path.realpath(candidate)
    return None
queue = [(name, modules) for name in wanted]
while queue:
    name, base = queue.pop()
    if name in done: continue
    real = resolve(name, base)
    if real is None: print("unresolved", name); continue
    done.add(name)
    out = os.path.join(target, *name.split("/"))
    if not os.path.exists(out):
        os.makedirs(os.path.dirname(out), exist_ok=True)
        shutil.copytree(real, out, symlinks=False)
    manifest = os.path.join(real, "package.json")
    if os.path.exists(manifest):
        data = json.load(open(manifest, encoding="utf-8"))
        parent = os.path.dirname(real) if not name.startswith("@") else os.path.dirname(os.path.dirname(real))
        for key in ("dependencies", "optionalDependencies"):
            for dependency in data.get(key, {}): queue.append((dependency, parent))
print(json.dumps({"packages": sorted(done)}))
