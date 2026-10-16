import sys; sys.path.insert(0, '.')
from methods6 import *
ms = extract(); names = collections.defaultdict(list)
for m in ms: names[m['name']].append(m)
rx = re.compile(r'(\bself|\b\w+|\)|\]|>)\s*(\.|::)\s*(' + '|'.join(sorted(map(re.escape, names), key=len, reverse=True)) + r')\b\s*(?=\(|::<)')
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
    for ln, l in enumerate(S.split('\n'), 1):
        for m in rx.finditer(l):
            rows.append((rel.replace(PLUGP, ''), ln, m.group(1), m.group(2), m.group(3), any(s <= ln <= e for s, e in ranges)))
out = [r for r in rows if not (r[5] and r[2] in ('self', 'Self'))]
print(len(rows), 'total', len(out), 'non-self-in-impl')
cn = collections.Counter(r[4] for r in out)
print(len(cn), 'distinct names')
for n, k in cn.most_common(80): print(k, n, names[n][0]['vis'] or 'priv', names[n][0]['file'])
json.dump(rows, open(GEN + '/pwp6-calluse.json', 'w'), ensure_ascii=False)
