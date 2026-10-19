import json, re, sys
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/rs.idx"
rows = [l.split("\t", 1) for l in open(S, encoding="utf-8")]
pat = re.compile(r'(not\.)?(?:toContain|toMatch|includes|indexOf)\(\s*("(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\'|`(?:[^`\\]|\\.)*`)')
cache = {}
for f in sys.argv[1:]:
    t = open("/Users/ueli/Documents/semio/" + f, encoding="utf-8").read()
    out = []
    for m in pat.finditer(t):
        raw = m.group(2)[1:-1].replace('\\"', '"').replace("\\'", "'").replace("\\\\", "\\")
        s = re.sub(r"\s+", "", raw)
        if len(s) < 4 or (s, bool(m.group(1))) in cache and False: continue
        if s not in cache:
            hits = [p for p, x in rows if s in x]
            cache[s] = [h for h in hits if "/🧪️tests/" not in h and "🧪️testing" not in h] or [("T:" + h) for h in hits[:2]]
        h = cache[s]
        neg = bool(m.group(1))
        if (not h and not neg) or (h and neg and not h[0].startswith("T:")):
            out.append(("MISS " if not h else "PRES ") + s[:100] + " " + str([x.replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "OS/")[-70:] for x in h[:1]]))
    print("==", f[-100:])
    for o in dict.fromkeys(out): print("  ", o[:230])
