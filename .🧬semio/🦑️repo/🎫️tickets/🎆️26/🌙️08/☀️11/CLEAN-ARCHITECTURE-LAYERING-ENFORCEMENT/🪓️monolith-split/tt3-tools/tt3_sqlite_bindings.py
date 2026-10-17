"""🗄️ Pass bun:sqlite bindings as one array: `db.run(sql, a, b)` becomes `db.run(sql, [a, b])` at each SQLQueryBindings diagnostic."""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks, offset

base, snap = sys.argv[1], sys.argv[2]
apply = len(sys.argv) > 3 and sys.argv[3] == "apply"

def skip_string(s, i):
    q = s[i]; i += 1
    while s[i] != q:
        if s[i] == "\\": i += 1
        i += 1
    return i + 1

def call_end(s, i):
    depth = 0
    while True:
        c = s[i]
        if c in "\"'`": i = skip_string(s, i); continue
        if c in "([{": depth += 1
        elif c in ")]}":
            if depth == 0: return i
            depth -= 1
        i += 1

files = {}
for b in blocks(base):
    if b["code"] == "TS2345" and "to parameter of type 'SQLQueryBindings[]'" in b["text"]:
        files.setdefault(b["file"], []).append((b["line"], b["col"]))

for path, sites in sorted(files.items()):
    text = open(path, encoding="utf8").read()
    starts = [0] + [m.end() for m in re.finditer("\n", text)]
    edits = []
    for line, col in sites:
        at = offset(text, starts, line, col)
        end = call_end(text, at)
        args = text[at:end]
        if text[at] == "[" : print("SKIP array", path[-50:], line, col); continue
        edits.append((at, end))
    edits = sorted(set(edits), reverse=True)
    print(len(edits), "of", len(sites), path[-70:])
    if not apply or not edits: continue
    os.makedirs(snap, exist_ok=True)
    shutil.copy(path, os.path.join(snap, re.sub(r"[^A-Za-z0-9]+", "_", path)[-120:] + ".sql"))
    for at, end in edits:
        text = text[:at] + "[" + text[at:end] + "]" + text[end:]
    open(path, "w", encoding="utf8").write(text)
