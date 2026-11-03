import os, re

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
PARTS = {
    "gen_duck.py": {"neck-1": "neck-1", "neck-2": "neck-2", "neck-3": "neck-3", "neck-lower": "neck-1", "neck-upper": "neck-3", "body": "trunk"},
    "gen_pigeon.py": {"neck-1": "neck-1", "neck-2": "neck-2", "neck-3": "neck-3", "neck": "neck-2"},
    "gen_deer.py": {"neck": "neck"},
    "gen_sheep.py": {"neck": "neck"},
    "gen_fox.py": {"brush-root": "tail-1", "brush-mid": "tail-2", "brush-tip": "tail-3", "brush": "tail-1", "brush-2": "tail-2"},
    "gen_squirrel.py": {"rump": "spine-r", "chest": "spine-f", "neck": "neck", "head": "head", "tail-1": "tail-1", "tail-2": "tail-2"},
}
for name, parts in PARTS.items():
    p = os.path.join(A2, name)
    out, n = [], 0
    for l in open(p, encoding="utf-8").read().split("\n"):
        m = re.match(r'^(\s*)([FP])\.(ell|ext|poly|blob|cap)\("([^"]+)", "([^"]+)",', l)
        if m and parts.get(m.group(4)) == m.group(5):
            like = re.search(r"like=(\"[^\"]*\"|[^,)]+)", l).group(1)
            z = re.search(r"z=([0-9.]+)", l).group(1)
            extra = ""
            if "filled=False" in l:
                extra = ", filled=False"
            l = '%s%s.rev("%s", "%s", like=%s, z=%s%s)' % (m.group(1), m.group(2), m.group(4), m.group(5), like, z, extra)
            n += 1
        out.append(l)
    open(p, "w", encoding="utf-8").write("\n".join(out))
    print(name, n)
