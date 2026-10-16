import sys; sys.path.insert(0,'.')
from wtests import *
from w10 import load_world_from, SNAP
allfns, files = load_world_from(SNAP)
defs, unres = resolve_calls(allfns)
apply_hints(allfns, files)
fixpoint(allfns, defs)
assign_modes(allfns)
tfns, tfiles = load_tests(SNAP)
print('test fns using state:', sum(1 for f in tfns if f.has_param), 'of', len(tfns))
fl = collections.Counter(); ex = {}
for f in tfns:
    for x in f.flags:
        fl[(x[0], x[1])] += 1; ex.setdefault((x[0], x[1]), (f, tfiles[f.file][0].count('\n', 0, tfiles[f.file][1][x[2]].a)+1))
for k, v in fl.most_common(30): print(v, k, ex[k][0].file.split('/')[-2], ex[k][1])
# resolve test calls against the world fns
alld = allfns + tfns
defs2, unres2 = resolve_calls(alld)
print('unresolved test calls', len(unres2))
for f, c in unres2[:10]: print(f, c['name'], c['kind'])
amb = collections.defaultdict(list)
for f in tfns:
    for c in f.calls:
        if len(c['targets']) > 1: amb[c['name']].append((f, c))
for nm, lst in amb.items():
    print('==', nm, len(lst))
    for f, c in lst[:6]:
        text, toks = tfiles[f.file]; pos = toks[c['callee_tok']].a
        ls = text.rfind('\n', 0, pos) + 1; le = text.find('\n', pos)
        print('   ', f.file.split('/')[-2], text.count('\n', 0, pos)+1, text[ls:le].strip()[:150])
miss = resolve_ambiguous_tests(tfns, tfiles)
print('still ambiguous in tests:', len(miss))
for f, c, r, ty in miss[:30]:
    text, toks = tfiles[f.file]; pos = toks[c['callee_tok']].a
    ls = text.rfind('\n', 0, pos) + 1; le = text.find('\n', pos)
    print('  ', f.file.split('/')[-2], text.count('\n', 0, pos)+1, r, ty, text[ls:le].strip()[:130])
print(collections.Counter(c['name'] for f, c in unres2))
