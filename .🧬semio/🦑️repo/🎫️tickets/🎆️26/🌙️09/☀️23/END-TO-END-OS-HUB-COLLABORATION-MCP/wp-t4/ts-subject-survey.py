import os, json, sys, collections
root = "/Users/ueli/Documents/semio"
tax = json.load(open(f"{root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"))
kinds = set(tax["testOwnerKinds"])
lang = sys.argv[1] if len(sys.argv) > 1 else "🟦️typescript"
adapter = {"🟦️typescript": "🟦️.ts", "🐍️python": "🐍️.py", "🐹️go": "🐹️.go"}[lang]
res = collections.Counter(); diff = []
for dp, dns, fns in os.walk(root):
    dns[:] = [d for d in dns if not d.startswith(".") and d not in ("node_modules", "target")]
    if os.path.basename(os.path.dirname(dp)) != "🧪️tests" or adapter not in fns: continue
    owner = os.path.relpath(os.path.dirname(os.path.dirname(dp)), root)
    segs = owner.split("/")
    anc = ["/".join(segs[:len(segs)-i]) for i in range(len(segs))]
    old = next((a for a in anc if os.path.isdir(f"{root}/{a}/📦️packages/{lang}")), None)
    members = [a for a in anc if len(a.split("/")) >= 2 and a.split("/")[-2] in ("🛍️products", "🔌️plugins")]
    top = members[0] if members else anc[-1]
    bounded = anc[:anc.index(top)+1]
    new = next((a for a in bounded if os.path.isdir(f"{root}/{a}/📦️packages/{lang}")), None)
    res[(old is not None, new is not None)] += 1
    if old != new: diff.append((os.path.relpath(dp, root), old))
print(res)
for d in sorted(diff): print(d)
