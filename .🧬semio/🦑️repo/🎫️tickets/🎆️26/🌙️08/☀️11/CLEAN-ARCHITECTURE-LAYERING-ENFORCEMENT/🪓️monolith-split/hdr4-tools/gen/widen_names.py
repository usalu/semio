import json, re, sys, os
# widen_names.py <err-file> <companion-path-fragment> <owner-closure-list> [--apply] : unresolved names in the companion that name non-pub items of the owner lib closure
R = "/Users/ueli/Documents/semio/"
errf, frag, listf = sys.argv[1:4]
apply = "--apply" in sys.argv
files = [l.split("\t")[-1].strip() for l in open(listf) if l.count("\t") == 4]
names = {}
for l in open(errf, encoding="utf8"):
    l = l.strip()
    if not l.startswith("{"): continue
    try: d = json.loads(l)
    except Exception: continue
    if d.get("level") != "error": continue
    code = (d.get("code") or {}).get("code")
    if code not in ("E0425", "E0422", "E0433", "E0412", "E0423", "E0432", "E0531", "E0599"): continue
    prim = [s for s in d["spans"] if s["is_primary"]]
    if not prim or frag not in prim[0]["file_name"]: continue
    m = re.search(r"cannot find (?:function|value|type|struct, variant or union type|tuple struct or tuple variant|unit struct, unit variant or constant|trait|macro|attribute macro|derive macro|struct or variant)\s+`([^`]+)`", d["message"]) or re.search(r"failed to resolve: use of undeclared type `([^`]+)`", d["message"]) or re.search(r"unresolved import `[^`]*?(\w+)`", d["message"])
    if m: names.setdefault(m.group(1), set()).add(code)
pat = re.compile(r"(?m)^([ \t]*)(?:#\[[^\n]*\]\n[ \t]*)*?((?:pub(?:\([^)]*\))?[ \t]+)?)((?:async[ \t]+|unsafe[ \t]+|const[ \t]+)*(?:fn|struct|enum|static|trait|type|union|const)[ \t]+)(\w+)")
plan = []
for n in sorted(names):
    found = []
    for f in files:
        txt = open(R + f).read()
        for m in re.finditer(r"(?m)^([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?)((?:async[ \t]+|unsafe[ \t]+|const[ \t]+)*(?:fn|struct|enum|static|trait|type|union|const)[ \t]+)%s\b" % re.escape(n), txt):
            line_no = txt.count("\n", 0, m.start()) + 1
            found.append((f, line_no, m.group(2).strip() or "priv", m.group(0)))
        for m in re.finditer(r"(?m)^([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?)macro_rules!\s+%s\b" % re.escape(n), txt):
            pass
    pubs = [x for x in found if x[2] == "pub"]
    non = [x for x in found if x[2] != "pub"]
    print(n, sorted(names[n]), "->", [(x[0][-40:], x[1], x[2]) for x in found][:4])
    if non and not pubs:
        plan += [(x[0], x[1]) for x in non] if len(non) <= 2 else []
if apply:
    byf = {}
    for f, ln in plan: byf.setdefault(f, set()).add(ln)
    for f, lns in byf.items():
        lines = open(R + f).read().split("\n")
        for ln in lns:
            l = lines[ln - 1]
            m = re.match(r"^([ \t]*)(pub(?:\([^)]*\))?[ \t]+)?(.*)$", l)
            lines[ln - 1] = m.group(1) + "pub " + m.group(3)
        open(R + f, "w").write("\n".join(lines))
    print("widened", sum(len(v) for v in byf.values()))
