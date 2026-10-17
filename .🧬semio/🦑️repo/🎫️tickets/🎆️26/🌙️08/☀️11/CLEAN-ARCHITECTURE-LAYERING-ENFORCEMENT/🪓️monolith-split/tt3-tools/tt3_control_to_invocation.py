"""🎛️ Replace `this.invocation.control` by `this.invocation` at each ScriptControl-for-ScriptInvocation diagnostic."""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks

base, snap = sys.argv[1], sys.argv[2]
apply = len(sys.argv) > 3 and sys.argv[3] == "apply"
sites = {}
for b in blocks(base):
    if b["code"] == "TS2345" and "Argument of type 'ScriptControl' is not assignable to parameter of type 'ScriptInvocation" in b["text"]:
        sites.setdefault(b["file"], []).append((b["line"], b["col"]))
OLD = "this.invocation.control"
for path, found in sorted(sites.items()):
    lines = open(path, encoding="utf8").read().split("\n")
    todo = []
    for line, col in sorted(set(found), reverse=True):
        text = lines[line - 1]
        if text[col - 1:col - 1 + len(OLD)] != OLD:
            print("SKIP", path, line, col, text[col - 1:col + 40]); continue
        todo.append((line, col))
    print(len(todo), path[-60:])
    if apply and todo:
        os.makedirs(snap, exist_ok=True)
        shutil.copy(path, os.path.join(snap, re.sub(r"[^A-Za-z0-9]+", "_", path)[-120:]))
        for line, col in todo:
            text = lines[line - 1]
            lines[line - 1] = text[:col - 1] + "this.invocation" + text[col - 1 + len(OLD):]
        open(path, "w", encoding="utf8").write("\n".join(lines))
