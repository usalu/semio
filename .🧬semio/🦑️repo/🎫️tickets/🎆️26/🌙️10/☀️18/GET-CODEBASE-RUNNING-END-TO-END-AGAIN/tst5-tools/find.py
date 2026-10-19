import re, sys
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/rs.idx"
needles = [re.sub(r"\s+", "", n) for n in sys.argv[1:]]
rows = [l.split("\t", 1) for l in open(S, encoding="utf-8")]
for n in needles:
    hits = [p for p, t in rows if n in t]
    prod = [h for h in hits if "/🧪️tests/" not in h and "🧪️testing" not in h]
    print(("OK  " if prod else ("TEST" if hits else "MISS")), n[:90], [h.replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "OS/") for h in (prod or hits)[:4]])
