import os, re, json, collections
ROOT = "/Users/ueli/Documents/semio"
F = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split")
G = os.path.join(F, "🗑️generated")
files = json.load(open(os.path.join(G, "frd-files.json"), encoding="utf-8"))
ctx = collections.Counter(); ex = {}
for rel in files:
    t = open(os.path.join(ROOT, rel), encoding="utf-8").read()
    if "runtime_fixture" not in t: continue
    for m in re.finditer(r'(.{0,24})(runtime_fixture[a-z_]*)(.{0,12})', t):
        pre = m.group(1); name = m.group(2); post = m.group(3)
        if pre.rstrip().endswith("fn"): continue
        kind = None
        mm = re.search(r'((?:[A-Za-z_][A-Za-z0-9_]*::)*|use )$', pre)
        pr = mm.group(1) if mm else "?"
        key = (pr, name, post[:1])
        ctx[key] += 1
        ex.setdefault(key, rel)
for k, n in ctx.most_common(60): print(n, k, ex[k][-70:])
print(len(ctx))
