# 🔎 Recovery audit of A1 (found state): which of the 8 touched files still equal their A1 overlay, which differ from the pre-A1 snapshot, and whether every item of the pre-A1 files is still present (multiset of top-level items, impl headers normalised; methods of `impl ArtifactStore` by name over all impl blocks of the live tree).
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
P56 = HERE + '/../okwp56-tools'
sys.path.insert(0, P56); sys.path.insert(0, HERE + '/../okwp0-tools')
import rs
from okrs import items
R = '/Users/ueli/Documents/semio'
ov = json.load(open(HERE + '/../🗑️generated/okwp56-a1-overlay-applied.json'))['out']
idx = [l.split('\t') for l in open(P56 + '/pre-a1/INDEX.txt', encoding='utf8').read().split('\n') if l.strip()]
def norm(s): return re.sub(r'\s+', ' ', re.sub(r'(?:\b\w+::)+', '', s)).strip()
KIND = ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'union', 'macro_rules', 'impl')
def keys(text, depth=0):
    out = []
    for it in items(text):
        k = it['kind']
        if k in KIND and it['name'] is not None:
            nm = norm(it['name'])
            if k == 'impl': nm = nm.split(' where ')[0]
            out.append((k, nm))
        elif k == 'mod' and it['text'].count('{') and depth < 3:
            body = it['text'][it['text'].index('{') + 1: it['text'].rindex('}')]
            out += keys(body, depth + 1)
    return out
tot_lost = 0
for f, rel in idx:
    p = os.path.join(R, rel)
    live = open(p, encoding='utf8').read(); pre = open(os.path.join(P56, f), encoding='utf8').read()
    same_ov = live == ov.get(p)
    a = collections.Counter(keys(pre)); b = collections.Counter(keys(live))
    lost = a - b; extra = b - a
    print(rel.split('/')[-3] + '/' + rel.split('/')[-2], '| pre', len(pre.split('\n')), 'live', len(live.split('\n')), '| live==A1 overlay', same_ov, '| lost', sum(lost.values()), 'extra', sum(extra.values()))
    for k, v in list(lost.items())[:10]: print('   LOST', v, k)
    for k, v in list(extra.items())[:10]: print('   EXTRA', v, k)
# store root: methods by name
root_pre = open(P56 + '/pre-a1/root-pre-a1.rs', encoding='utf8').read()
root_live = open(R + '/' + idx[0][1], encoding='utf8').read()
def methods(text):
    out = collections.Counter()
    for it in rs.outline(text):
        if it['kind'] != 'impl': continue
        for me in rs.methods(text, it['h'], it['e']):
            if me['kind'] == 'fn': out[me['name']] += 1
    return out
mp = methods(root_pre); ml = methods(root_live)
print('methods pre', sum(mp.values()), 'unique', len(mp), '| live', sum(ml.values()), 'unique', len(ml))
missing = [n for n in mp if n not in ml]
print('pre method names missing in live:', len(missing), missing[:20])
