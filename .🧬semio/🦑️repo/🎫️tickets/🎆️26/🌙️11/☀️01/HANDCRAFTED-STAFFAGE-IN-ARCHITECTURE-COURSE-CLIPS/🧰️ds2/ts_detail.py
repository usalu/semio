import re, sys, collections, os

def lead_and_text(first):
    out = ""
    s = first.strip()
    for ch in s:
        if ord(ch) > 0x2000 and not ch.isalnum():
            out += ch
        else:
            break
    return out.replace("️", ""), s[len(out):].strip()

for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    p = p.strip()
    if not p or not os.path.exists(p):
        continue
    src = open(p, encoding="utf-8").read()
    rows = []
    for m in re.finditer(r"/\*\*(.*?)\*/", src, re.S):
        body = m.group(1)
        first = next((re.sub(r"^\s*\*?\s?", "", l) for l in body.splitlines() if re.sub(r"^\s*\*?\s*", "", l).strip()), "")
        ln = src.count("\n", 0, m.start()) + 1
        e, text = lead_and_text(first)
        rows.append((ln, e, text))
    seen = collections.defaultdict(list)
    for ln, e, text in rows:
        if e:
            seen[e].append((ln, text))
    groups = {k: v for k, v in seen.items() if len(v) > 1}
    if not groups:
        continue
    print("FILE", p.split("🏛️architecture/")[-1])
    print("  LEADS", " ".join("%d:%s" % (ln, e) for ln, e, _ in rows if e))
    for k, v in groups.items():
        print("  GROUP", k)
        for ln, text in v:
            print("    %d | %s" % (ln, text[:95]))
