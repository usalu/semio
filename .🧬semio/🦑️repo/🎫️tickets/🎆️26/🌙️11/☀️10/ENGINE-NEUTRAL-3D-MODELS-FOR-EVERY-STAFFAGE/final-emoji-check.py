"""Docstring emoji audit: every docstring (/** ... */ in TypeScript, triple-quoted first statement in Python is not checked) starts with an emoji that is unique within its file. Usage: final-emoji-check.py [--fix] <files from stdin>"""
import re, sys, subprocess, json
root = "/Users/ueli/Documents/semio"
out = subprocess.run(["git", "-C", root, "-c", "core.quotePath=false", "status", "--short", "-uall", "--", "🎓️teaching/🏛️architecture/🧍️staffage", "🧰️framework/🔨️modules/🧊️3d"], capture_output=True, text=True).stdout
files = []
for line in out.splitlines():
    path = line[3:].strip().strip('"')
    if path.endswith((".ts", ".tsx")) and " -> " not in path and "node_modules" not in path: files.append(path)
pat = re.compile(r"^(\s*)/\*\*\s*(\S+)", re.M)
def emoji_of(tok):
    return tok if tok and ord(tok[0]) > 0x2000 else None
bad = {}
for f in files:
    try: text = open(f"{root}/{f}", encoding="utf8").read()
    except FileNotFoundError: continue
    seen = {}
    for m in pat.finditer(text):
        tok = m.group(2)
        e = emoji_of(tok)
        key = re.sub("️", "", tok[:3]) if e else None
        if not e: bad.setdefault(f, []).append(("no-emoji", text[:m.start()].count("\n") + 1, tok[:20])); continue
        k = ""
        for ch in tok:
            if ord(ch) <= 0x2000 and ch != "‍" and ch != "️": break
            k += ch
        k = k.replace("️", "")
        if k in seen: bad.setdefault(f, []).append(("dup", text[:m.start()].count("\n") + 1, k))
        else: seen[k] = 1
print(len(files), "files;", sum(len(v) for v in bad.values()), "findings in", len(bad), "files")
for f, v in sorted(bad.items()): print(f.split("🧍️staffage/")[-1][:90], v[:6])
