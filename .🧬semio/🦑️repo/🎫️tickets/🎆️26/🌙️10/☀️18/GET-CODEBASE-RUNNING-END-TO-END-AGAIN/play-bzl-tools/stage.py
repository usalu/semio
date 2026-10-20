#!/usr/bin/env python3
"""Records the base hashes and writes the edited copies under staged/ (no repo write)."""
import hashlib, json, os, sys
D = os.path.dirname(os.path.abspath(__file__))
os.chdir("/Users/ueli/Documents/semio")
src = open(D + "/apply.py").read().replace("\nmain()\n", "\n")
ns = {"__file__": D + "/apply.py"}
exec(compile(src, "apply", "exec"), ns)
bases = {}
for path, fn in ns["EDITS"]:
    text = open(path, encoding="utf8").read()
    bases[path] = hashlib.sha256(text.encode()).hexdigest()
    out = D + "/staged/" + hashlib.sha256(path.encode()).hexdigest()[:8]
    os.makedirs(out, exist_ok=True)
    open(out + "/" + os.path.basename(path), "w", encoding="utf8").write(fn(text))
    open(out + "/SOURCE", "w", encoding="utf8").write(path + "\n")
json.dump(bases, open(D + "/bases.json", "w"), ensure_ascii=False, indent=1)
print("staged", len(bases))
