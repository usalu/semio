import re, sys, collections, os

def lead(s):
    out = ""
    for ch in s.strip():
        if ord(ch) > 0x2000 and not ch.isalnum():
            out += ch
        else:
            break
    return out

FN = re.compile(r"^\s*(export\s+)?(default\s+)?(async\s+)?(function\b|class\b|const\s+\w+\s*=\s*(async\s*)?(\(|function)|const\s+\w+\s*(:[^=]+)?=\s*\()")
for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    if not p.strip() or not os.path.exists(p):
        continue
    src = open(p, encoding="utf-8").read()
    short = p.split("architecture/")[-1]
    seen = collections.defaultdict(list)
    lines = src.splitlines()
    for m in re.finditer(r"/\*\*(.*?)\*/", src, re.S):
        body = m.group(1)
        first = next((re.sub(r"^\s*\*\s?", "", l) for l in body.splitlines() if re.sub(r"^\s*\*?\s*", "", l).strip()), "")
        ln = src.count("\n", 0, m.start()) + 1
        end = src.count("\n", 0, m.end()) + 1
        nxt = lines[end] if end < len(lines) else ""
        kind = "fn" if (FN.match(nxt) or ln == 1) else "other"
        e = lead(first)
        if kind == "fn" and e:
            seen[e].append((ln, nxt.strip()[:50]))
    for e, v in seen.items():
        if len(v) > 1:
            print(short, e, v)
