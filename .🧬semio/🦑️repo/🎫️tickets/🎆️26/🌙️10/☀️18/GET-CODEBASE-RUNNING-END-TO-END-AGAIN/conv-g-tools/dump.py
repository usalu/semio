import sys, lib
root = "/Users/ueli/Documents/semio/"
for p in open("files.txt", encoding="utf8").read().split("\n"):
    if not p or (len(sys.argv) > 1 and sys.argv[1] not in p): continue
    t = open(root + p, encoding="utf8").read(); lines = t.split("\n")
    r = lib.reuse(p, t)
    if not r: continue
    print("##", p.replace("🧰️framework/", "")[-100:])
    bad = {l for ls in r.values() for l in ls}
    for ln, c in lib.openers(p, t):
        if ln in bad:
            nxt = lines[ln].strip() if ln < len(lines) else ""
            print(f"{ln}: {c[:100]}  ||  {nxt[:80]}")
