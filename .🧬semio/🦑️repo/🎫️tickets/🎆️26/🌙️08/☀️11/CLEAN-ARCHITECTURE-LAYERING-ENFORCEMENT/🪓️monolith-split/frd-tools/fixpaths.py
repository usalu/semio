import os, json, sys, hashlib, collections
ROOT = "/Users/ueli/Documents/semio/"
F = ROOT + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G = F + "/🗑️generated/"
apply = "--apply" in sys.argv
rows = json.load(open(G + "frd-missing-class.json", encoding="utf-8"))
byfile = collections.defaultdict(dict)
for r in rows:
    if r["class"] == "reanchor": byfile[r["file"]][r["lit"]] = r["new"]
done = []; skipped = []
for f, m in byfile.items():
    p = ROOT + f
    t = open(p, encoding="utf-8").read()
    n = t
    for old, new in m.items():
        n = n.replace('"' + old + '"', '"' + new + '"')
    if n == t: skipped.append((f, "no-change")); continue
    if apply:
        if open(p, encoding="utf-8").read() != t: skipped.append((f, "raced")); continue
        key = hashlib.sha1(f.encode()).hexdigest()
        op = F + "/frd-tools/orig/" + key + ".rs"
        if not os.path.exists(op): open(op, "w", encoding="utf-8").write(t)
        open(p, "w", encoding="utf-8").write(n)
    done.append(f)
json.dump(done, open(G + "frd-fixpaths-applied.json", "w", encoding="utf-8"), ensure_ascii=False)
print(len(done), "files", "applied" if apply else "planned", "skipped", skipped[:5])
