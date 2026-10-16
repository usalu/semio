import sys, os, re, collections, json
sys.path.insert(0, '../okwp23-tools'); sys.path.insert(0, '../okwp0-tools')
import em, ns
w = ns.world()
files = sorted(w.files)
print(len(files), 'files in kernel world')
json.dump(files, open(em.HERE + '/../🗑️generated/okwp56-kernel-files.json', 'w'), ensure_ascii=False)
t = em.read()
res = collections.defaultdict(lambda: collections.Counter())
for f in files:
    src = open(f, encoding='utf8').read()
    if f == em.ROOT:
        continue
    toks = em.code_tokens(src)
    for i in range(1, len(toks) - 1):
        if toks[i][1] == '.' and toks[i + 1][0] == 'id' and toks[i + 1][1] in em.FIELD_COMP:
            nx = toks[i + 2][1] if i + 2 < len(toks) else ''
            if nx == '(' : continue
            res[os.path.relpath(f, ns.M)][toks[i + 1][1]] += 1
for f, c in sorted(res.items(), key=lambda kv: -sum(kv[1].values()))[:60]:
    print(sum(c.values()), f[-80:], dict(c.most_common(8)))
