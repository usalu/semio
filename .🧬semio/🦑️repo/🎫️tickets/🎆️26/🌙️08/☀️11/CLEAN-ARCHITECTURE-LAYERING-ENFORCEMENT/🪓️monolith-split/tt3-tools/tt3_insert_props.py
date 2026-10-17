"""➕ Add missing properties to the object literals a diagnostic family names.
usage: tt3_insert_props.py <tsc.txt> <snapdir> <diagnostic-needle> <must-contain-key> <props-text> [apply]"""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks, offset

base, snap, needle, key, props = sys.argv[1:6]
apply = len(sys.argv) > 6 and sys.argv[6] == "apply"

def skip_string(s, i):
    q = s[i]; i += 1
    while s[i] != q:
        if s[i] == "\\": i += 1
        i += 1
    return i + 1

def match_brace(s, i):
    depth = 0
    while True:
        c = s[i]
        if c in "\"'`": i = skip_string(s, i); continue
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return i
        i += 1

files = {}
for b in blocks(base):
    if needle in b["text"] and b["code"] in ("TS2345", "TS2741", "TS2322", "TS2739", "TS2769"):
        files.setdefault(b["file"], []).append((b["line"], b["col"]))

for path, sites in sorted(files.items()):
    text = open(path, encoding="utf8").read()
    starts = [0] + [m.end() for m in re.finditer("\n", text)]
    inserts = []
    for line, col in sites:
        pos = offset(text, starts, line, col)
        brace = pos if text[pos] == "{" else text.find("{", pos)
        end = match_brace(text, brace)
        literal = text[brace:end + 1]
        first = props.split(":")[0]
        if key not in literal[:1500] or re.search(r"\b" + re.escape(first) + r"\s*:", literal):
            print("SKIP", path[-60:], line, col, literal[:90].replace("\n", " ")); continue
        inserts.append((brace, end))
    inserts = sorted(set(inserts), reverse=True)
    print(len(inserts), "of", len(sites), path[-70:])
    if not apply or not inserts: continue
    os.makedirs(snap, exist_ok=True)
    shutil.copy(path, os.path.join(snap, re.sub(r"[^A-Za-z0-9]+", "_", path)[-120:] + ".props"))
    for brace, end in inserts:
        body = text[brace + 1:end]
        stripped = body.rstrip()
        multiline = "\n" in body
        if multiline:
            indent = re.match(r"\s*", stripped.split("\n")[-1]).group(0)
            comma = "" if stripped.endswith(",") else ","
            at = brace + 1 + len(stripped)
            text = text[:at] + comma + "\n" + indent + props + "," + text[at:]
        else:
            comma = "" if stripped.endswith(",") else ","
            at = brace + 1 + len(stripped)
            text = text[:at] + comma + " " + props + text[at:]
    open(path, "w", encoding="utf8").write(text)
