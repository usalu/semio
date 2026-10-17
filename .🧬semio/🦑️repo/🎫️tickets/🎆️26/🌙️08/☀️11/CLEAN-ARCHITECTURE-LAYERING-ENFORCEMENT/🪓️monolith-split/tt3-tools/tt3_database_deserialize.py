"""🗄️ Replace `new Database(<bytes>)` by the typed `Database.deserialize(<bytes>)` at each Uint8Array-for-string diagnostic."""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks

base, snap = sys.argv[1], sys.argv[2]
apply = len(sys.argv) > 3 and sys.argv[3] == "apply"
sites = {}
for b in blocks(base):
    if b["code"] == "TS2345" and "Argument of type 'Uint8Array<ArrayBufferLike>' is not assignable to parameter of type 'string'" in b["text"]:
        sites.setdefault(b["file"], []).append((b["line"], b["col"]))
OLD = "new Database("
for path, found in sorted(sites.items()):
    lines = open(path, encoding="utf8").read().split("\n")
    todo = []
    for line, col in sorted(set(found), reverse=True):
        text = lines[line - 1]
        if text[:col - 1].endswith(OLD): todo.append((line, col))
        else: print("SKIP", path[-70:], line, col, text[max(0, col - 30):col + 30])
    print(len(todo), path[-70:])
    if apply and todo:
        os.makedirs(snap, exist_ok=True)
        shutil.copy(path, os.path.join(snap, re.sub(r"[^A-Za-z0-9]+", "_", path)[-120:]))
        for line, col in todo:
            text = lines[line - 1]
            lines[line - 1] = text[:col - 1 - len(OLD)] + "Database.deserialize(" + text[col - 1:]
        open(path, "w", encoding="utf8").write("\n".join(lines))
