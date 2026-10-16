# 🔎️ Name-based definition scan: private definitions in lower crates whose names occur in an upper crate's sources.
import re, sys, os
TM = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map"
def strip(src):
    out = []; i = 0; n = len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i); j = n if j < 0 else j; out.append(' ' * (j - i)); i = j
        elif src.startswith('/*', i):
            j = src.find('*/', i + 2); j = n if j < 0 else j + 2; out.append(''.join(ch if ch == '\n' else ' ' for ch in src[i:j])); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == '\\' else 1
            j = min(n, j + 1); out.append(''.join(ch if ch == '\n' else ' ' for ch in src[i:j])); i = j
        else: out.append(c); i += 1
    return ''.join(out)
LOWER = sys.argv[1].split(",")
UPPER = sys.argv[2].split(",")
def rd(p): return strip(open(TM + "/" + p, encoding="utf-8").read())
upper_text = "\n".join(rd(p) for p in UPPER)
cands = []
for p in LOWER:
    t = rd(p)
    for m in re.finditer(r'^(\s*)(?!pub\b)(?:(?:async|unsafe|const)\s+)*(fn|const|static|struct|enum|trait|type|mod)\s+(\w+)', t, re.M):
        cands.append((p, m.group(2), m.group(3), len(m.group(1))))
    for m in re.finditer(r'^(\s*)pub\((?:super|crate|in [^)]*)\)\s+(?:(?:async|unsafe|const)\s+)?(?:fn|const|static|struct|enum|trait|type|mod)\s+(\w+)', t, re.M):
        cands.append((p, "pub(restricted)", m.group(2), len(m.group(1))))
    # fields: lines in struct bodies `name: Type,` without pub
    for sm in re.finditer(r'\bstruct\s+(\w+)[^;{]*\{', t):
        depth = 1; i = sm.end(); j = i
        while j < len(t) and depth:
            depth += (t[j] == '{') - (t[j] == '}'); j += 1
        body = t[i:j - 1]
        d = 0
        for line in body.split("\n"):
            if d == 0:
                fm = re.match(r'\s*(?!pub\b)(\w+)\s*:', line)
                if fm: cands.append((p, "field of " + sm.group(1), fm.group(1), 0))
                fm = re.match(r'\s*pub\((?:super|crate|in [^)]*)\)\s+(\w+)\s*:', line)
                if fm: cands.append((p, "field(restricted) of " + sm.group(1), fm.group(1), 0))
            d += line.count("{") + line.count("(") + line.count("[") - line.count("}") - line.count(")") - line.count("]")
seen = set()
for p, kind, name, ind in cands:
    if (p, name) in seen: continue
    if re.search(r'(?<![\w])' + re.escape(name) + r'(?![\w])', upper_text):
        seen.add((p, name)); print(f"{p}\t{kind}\t{name}")
