"""🧮 Test-name multiset gate: old kernel test closure (pristine snapshot list) against the union of the new crate closures."""
import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, testnames
R = L.R + '/'
def names_of(paths):
    out = collections.Counter()
    for f in paths:
        p = R + f
        if not os.path.exists(p): continue
        for n in testnames.names(open(p, encoding='utf8').read()): out[n] += 1
    return out
before = collections.Counter()
rows_before = [l.rstrip('\n').split('\t') for l in open(L.GEN + '/okwp8-tests-before.tsv', encoding='utf8')]
for n, f in rows_before: before[n] += 1
census = json.load(open(L.GEN + '/okwp8-census.json'))
seen = set(); after = collections.Counter(); per = {}
for crate in census:
    d = json.load(open(L.GEN + '/okwp8-census-%s.json' % crate))
    files = [r['path'] for r in d['test native'] if r['kind'] != 'embedded' and r['path'].endswith('.rs')]
    fresh = [f for f in files if f not in seen]
    seen |= set(files)
    c = names_of(fresh)
    per[crate] = (len(fresh), sum(c.values()))
    after.update(c)
KT = ['🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs','🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/♻️retirement/🧪️tests/🚪️public/🦀️.rs']
import mounts
kt_files = []
stack = list(KT)
while stack:
    f = stack.pop()
    if f in kt_files or not os.path.exists(R + f): continue
    kt_files.append(f)
    for name, tgt, cfg in mounts.children(f): stack.append(tgt)
old_files = {f for n, f in rows_before}
fresh_kt = [f for f in kt_files if f not in seen and f in old_files]
ck = names_of(fresh_kt); per['kernel [[test]] targets'] = (len(fresh_kt), sum(ck.values())); after.update(ck)
print('before', sum(before.values()), 'after', sum(after.values()))
for k, v in per.items(): print('  ', k, v)
miss = before - after; extra = after - before
print('missing', sum(miss.values()), dict(list(miss.items())[:20]))
print('extra', sum(extra.values()), dict(list(extra.items())[:20]))
