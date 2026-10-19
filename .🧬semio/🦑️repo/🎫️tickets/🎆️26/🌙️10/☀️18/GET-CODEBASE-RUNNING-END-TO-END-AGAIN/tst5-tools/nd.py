import re, sys
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/rs.idx"
rows = [l.split("\t", 1) for l in open(S, encoding="utf-8")]
t = open(sys.argv[1], encoding="utf-8").read()
pat = re.compile(r'(not\.)?(?:toContain|toMatch|includes|indexOf)\(\s*("(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\'|`(?:[^`\\]|\\.)*`)')
seen = {}
for m in pat.finditer(t):
    raw = m.group(2)[1:-1].replace('\\"', '"').replace("\\'", "'").replace("\\\\", "\\")
    s = re.sub(r"\s+", "", raw)
    if len(s) < 4: continue
    seen[s] = bool(m.group(1)) or seen.get(s, False)
for s, neg in seen.items():
    hits = [p for p, x in rows if s in x]
    prod = [h for h in hits if "/🧪️tests/" not in h and "🧪️testing" not in h]
    tag = ("OK  " if prod else ("TEST" if hits else "MISS")) + (" NOT" if neg else "    ")
    if "-a" in sys.argv or (tag.startswith("MISS") and not neg) or (tag.startswith("OK") and neg):
        print(tag, s[:110], [h.replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "OS/")[-90:] for h in (prod or hits)[:2]])
