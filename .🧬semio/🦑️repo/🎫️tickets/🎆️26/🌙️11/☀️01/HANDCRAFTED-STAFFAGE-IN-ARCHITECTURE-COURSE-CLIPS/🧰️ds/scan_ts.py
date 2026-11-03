import re, sys, collections

def lead(s):
    s = s.strip()
    out = ""
    for ch in s:
        if ord(ch) > 0x2000 and not ch.isalnum():
            out += ch
        else:
            break
    return out

for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    if not p.strip() or not __import__("os").path.exists(p):
        continue
    src = open(p, encoding="utf-8").read()
    short = p.split("architecture/")[-1]
    seen = collections.defaultdict(list)
    for m in re.finditer(r"/\*\*(.*?)\*/", src, re.S):
        body = m.group(1)
        first = next((re.sub(r"^\s*\*\s?", "", l) for l in body.splitlines() if re.sub(r"^\s*\*?\s*", "", l).strip()), "")
        ln = src.count("\n", 0, m.start()) + 1
        e = lead(first)
        if not e:
            print(short, "NOEMOJI", ln, first.strip()[:50])
        else:
            seen[e].append(ln)
    for e, v in seen.items():
        if len(v) > 1:
            print(short, "DUP", e, v)
    for i, l in enumerate(src.splitlines(), 1):
        if re.match(r"^\s+//", l) or re.search(r"\S\s+//\s", l) and "http" not in l and "'" not in l and '"' not in l and "`" not in l:
            print(short, "CMT", i, l.strip()[:60])
        if re.search(r"console\.(log|debug|info)", l) and "[DEBUG]" not in l:
            print(short, "LOG", i, l.strip()[:60])
