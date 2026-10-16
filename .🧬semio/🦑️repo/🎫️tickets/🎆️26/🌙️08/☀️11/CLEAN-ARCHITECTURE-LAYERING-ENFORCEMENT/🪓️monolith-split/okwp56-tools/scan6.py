# receiver analysis: variables typed ArtifactStore (params / lets) and their direct field accesses outside the root
import sys, os, re, collections, json
sys.path.insert(0, '../okwp0-tools')
import em, rs
from okrs import tokenize
files = json.load(open('../🗑️generated/okwp56-kernel-files.json'))
res = collections.defaultdict(list)
for f in files:
    if f == em.ROOT: continue
    src = open(f, encoding='utf8').read()
    if 'ArtifactStore' not in src: continue
    toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
    n = len(toks)
    # find variables bound with a type mentioning ArtifactStore< or ArtifactStore:: constructors
    names = set()
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] == 'ArtifactStore':
            # param/let type: `name : [&] [mut] ... ArtifactStore`
            j = i - 1
            while j >= 0 and toks[j][1] in ('&', 'mut', "'", '(', 'Option', '<', 'Box', 'Arc', 'Vec', 'Result') or (j >= 0 and toks[j][0] == 'life'):
                j -= 1
            if j >= 1 and toks[j][1] == ':' and toks[j - 1][0] == 'id' and toks[j - 1][1] != 'self':
                names.add(toks[j - 1][1])
    # let name = ... ArtifactStore :: new / construct / from_...
    for i, t in enumerate(toks):
        if t[1] == 'let':
            k = i + 1
            if toks[k][1] == 'mut': k += 1
            if toks[k][0] == 'id' and i + 6 < n:
                window = ' '.join(x[1] for x in toks[k: k + 14])
                if re.search(r'ArtifactStore\s*:\s*:\s*(<[^>]*>\s*:\s*:\s*)?(new|construct|from_)', window): names.add(toks[k][1])
    hits = collections.Counter()
    for i in range(1, n - 1):
        if toks[i][1] == '.' and toks[i + 1][0] == 'id' and toks[i + 1][1] in em.FIELD_COMP and toks[i - 1][0] == 'id' and toks[i - 1][1] in names:
            nx = toks[i + 2][1] if i + 2 < n else ''
            if nx == '(': continue
            hits[(toks[i - 1][1], toks[i + 1][1])] += 1
    if hits: res[os.path.relpath(f, em.S)] = (sorted(names), hits)
for f, (names, hits) in sorted(res.items(), key=lambda kv: -sum(kv[1][1].values())):
    print(sum(hits.values()), f[-70:], names[:8], dict(list(hits.items())[:10]))
