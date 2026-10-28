"""🐍 Independent oracle of the foreign crate inventory: Python's own directory walk and sha-256 (stdlib only). usage: python -I this.py <crate directory> <skipped names...>"""
import hashlib, json, os, sys

root, skip = sys.argv[1], set(sys.argv[2:])
rows = {}
for base, directories, files in os.walk(root):
    directories[:] = [name for name in directories if name not in skip]
    for name in files:
        path = os.path.join(base, name)
        data = open(path, "rb").read()
        rows[path] = {"digest": hashlib.sha256(data).hexdigest(), "bytes": len(data)}
print(json.dumps(rows, ensure_ascii=False))
