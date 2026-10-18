import sys, json, re, collections
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
label = sys.argv[1]
res = json.load(open(f"{G}/tsl2-{label}.json", encoding="utf8"))
out = open(f"{G}/tsl2-{label}-fails.tsv", "w", encoding="utf8")
msgs = collections.Counter()
for x in res:
    f = x["file"].replace("🧰️framework/", "")
    if x["rc"] == 0: continue
    lg = open(f"{G}/tsl2-{label}/{x['key']}.log", encoding="utf8", errors="replace").read().split("\n")
    cur = ""; nfail = 0
    for l in lg:
        if l.startswith("error:"):
            if not cur: cur = l[:240]
        elif l.startswith("(fail)"):
            nfail += 1
            out.write("\t".join([str(x["rc"]), f, re.sub(r"\s*\[[\d.]+m?s\]$", "", l[7:])[:200], cur.replace(R, "")]) + "\n")
            msgs[re.sub(r"\d+", "N", cur.replace(R, ""))[:200]] += 1
            cur = ""
    if nfail == 0:
        out.write("\t".join([str(x["rc"]), f, "(no fail lines; rows=%s)" % dict(collections.Counter(s for s, _ in x["rows"])), "\n".join(lg[:6])[:240].replace("\n", " | ")]) + "\n")
for m, n in msgs.most_common(80): print(n, m)
