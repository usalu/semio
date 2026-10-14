#!/usr/bin/env python3
"""🔓️ Reads one cargo log and widens exactly the items a cross-crate cut made unreachable: private fields (E0616),
private methods and associated items (E0624), private items (E0603) and `pub(crate)` items named from another crate.
Only files under the given roots are edited.

usage: 🔓️privacy-fix.py <cargo.log> <root>...
"""
import os, re, sys
REPO = "/Users/ueli/Documents/semio"
log = open(sys.argv[1], encoding="utf-8", errors="replace").read()
roots = [os.path.join(REPO, r) for r in sys.argv[2:]]
blocks = re.split(r"\n(?=error(?:\[E\d+\])?: )", log)

def resolve(path):
    if os.path.isabs(path): return os.path.normpath(path)
    for base in [REPO, os.path.join(REPO, "✏️s")] + [os.path.join(REPO, r) for r in ["✏️s/🔌️plugins/🏗️fem", "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d", "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d", "🌎️hub"]]:
        p = os.path.normpath(os.path.join(base, path))
        if os.path.exists(p): return p
    return None

def allowed(p):
    return p and any(p.startswith(r) for r in roots)

files = []
for r in roots:
    for d, _, fs in os.walk(r):
        if "/target" in d or "node_modules" in d: continue
        files += [os.path.join(d, f) for f in fs if f.endswith(".rs")]
edits = {}
def widen_line(path, line):
    s = edits.get(path) or open(path, encoding="utf-8").read()
    lines = s.split("\n")
    l = lines[line - 1]
    nl = re.sub(r"^(\s*)pub\(crate\)\s+", r"\1pub ", l)
    if nl == l and not re.match(r"^\s*pub\b", l):
        nl = re.sub(r"^(\s*)((?:const |async |unsafe )*(?:fn|struct|enum|const|static|type|trait|mod)\b)", r"\1pub \2", l)
    if nl != l:
        lines[line - 1] = nl; edits[path] = "\n".join(lines); return True
    return False

done = set()
for b in blocks:
    m = re.match(r"error\[E0616\]: field `(\w+)` of struct `(?:[\w:]+::)?(\w+)` is private", b)
    if m:
        field, struct = m.groups()
        if (field, struct) in done: continue
        for f in files:
            s = edits.get(f) or open(f, encoding="utf-8").read()
            sm = re.search(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+" + struct + r"\b[^{;]*\{", s)
            if not sm: continue
            i = sm.end(); d = 1; j = i
            while d:
                if s[j] == "{": d += 1
                elif s[j] == "}": d -= 1
                j += 1
            inner = s[i:j - 1]
            new = re.sub(r"(?m)^(\s+)(?:pub\(crate\)\s+)?(" + field + r"\s*:)", r"\1pub \2", inner, count=1)
            if new != inner:
                edits[f] = s[:i] + new + s[j - 1:]; done.add((field, struct)); print("field", struct, field, os.path.relpath(f, REPO)); break
        continue
    m = re.match(r"error\[E06(?:24|03)\]: (?:method|associated function|function|struct|enum|constant|static|type alias|trait|module|associated constant|unit struct|tuple struct) `(\w+)` is private", b)
    if m:
        loc = re.findall(r"\n\s*(?:-->|::: )\s*([^\n:]+\.rs):(\d+):\d+", b)
        target = None
        for path, line in loc[1:] + loc[:1]:
            p = resolve(path)
            if allowed(p):
                src = (edits.get(p) or open(p, encoding="utf-8").read()).split("\n")
                if re.search(r"\b" + m.group(1) + r"\b", src[int(line) - 1]):
                    target = (p, int(line)); break
        if target and target not in done:
            if widen_line(*target): print("item", m.group(1), os.path.relpath(target[0], REPO), target[1])
            done.add(target)
for p, s in edits.items():
    open(p, "w", encoding="utf-8").write(s)
print("edited", len(edits))
