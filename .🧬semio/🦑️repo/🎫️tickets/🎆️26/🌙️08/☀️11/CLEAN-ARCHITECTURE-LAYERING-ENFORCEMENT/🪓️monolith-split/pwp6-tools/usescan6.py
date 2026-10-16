import sys; sys.path.insert(0, '.')
from fields6 import *
fs = parse_fields(); names = {f['name'] for f in fs}
pat = re.compile(r'(?<![\w.])(\w+(?:\([^()]*\))?|\)|\])\s*\.\s*(' + '|'.join(sorted(names, key=len, reverse=True)) + r')\b(?!\s*(?:\(|::))')
res = collections.defaultdict(lambda: collections.Counter())
byrecv = collections.Counter()
rows = []
for rel in plug_files():
    try: raw = read(rel)
    except FileNotFoundError: continue
    S = pt.strip(raw)
    its, L, R = pt.outline(ROOT + rel)
    ranges = []
    for it in its:
        if it['kind'] == 'impl':
            tgt, tr = pt.impl_target(it['head'])
            if tgt == 'VcsArtifactApp': ranges.append((it['s'], it['e']))
    lines = S.split('\n')
    for ln, l in enumerate(lines, 1):
        for m in re.finditer(r'(\bself|\b\w+|\)|\])\s*\.\s*(' + '|'.join(sorted(names, key=len, reverse=True)) + r')\b(?!\s*(?:\(|::))', l):
            recv, nm = m.group(1), m.group(2)
            inside = any(s <= ln <= e for s, e in ranges)
            rows.append((rel.replace(PLUGP, ''), ln, recv, nm, inside))
c = collections.Counter((r[4], r[2] == 'self') for r in rows)
print(c)
out = collections.Counter((r[0], r[4], r[2]) for r in rows if not (r[4] and r[2] == 'self'))
files = collections.Counter(r[0] for r in rows if not (r[4] and r[2]=='self'))
print(len(files), 'files with non-self-in-impl hits')
for f, n in files.most_common(60): print(n, f)
json.dump(rows, open(GEN + '/pwp6-fielduse.json', 'w'), ensure_ascii=False)
