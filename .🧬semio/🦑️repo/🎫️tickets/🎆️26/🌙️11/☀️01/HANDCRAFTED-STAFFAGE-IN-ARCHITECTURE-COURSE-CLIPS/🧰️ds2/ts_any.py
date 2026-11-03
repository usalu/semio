import re, sys, collections, os

def lead(s):
    out = ""
    for ch in s.strip():
        if ord(ch) > 0x2000 and not ch.isalnum():
            out += ch
        else:
            break
    return out.replace("️", "")

total = 0
for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    p = p.strip()
    if not p or not os.path.exists(p):
        continue
    src = open(p, encoding="utf-8").read()
    seen = collections.defaultdict(list)
    for m in re.finditer(r"/\*\*(.*?)\*/", src, re.S):
        body = m.group(1)
        first = next((re.sub(r"^\s*\*?\s?", "", l) for l in body.splitlines() if re.sub(r"^\s*\*?\s*", "", l).strip()), "")
        e = lead(first)
        if e:
            seen[e].append(src.count("\n", 0, m.start()) + 1)
    groups = {k: v for k, v in seen.items() if len(v) > 1}
    if groups:
        total += len(groups)
        print("FILE", p.split("🏛️architecture/")[-1], {k: v for k, v in groups.items()})
print("TOTAL", total)
