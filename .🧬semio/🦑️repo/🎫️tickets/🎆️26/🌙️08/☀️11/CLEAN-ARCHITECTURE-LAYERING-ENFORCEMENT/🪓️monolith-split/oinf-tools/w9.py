import sys; sys.path.insert(0,'.')
from wrewrite import *
allfns, files = load_world()
defs, unres = resolve_calls(allfns)
apply_hints(allfns, files)
fixpoint(allfns, defs)
edits, stats = make_edits(allfns, files)
print(stats)
host = [f for f in allfns if f.has_param and f.mode == 'host']
print('host-level:', len(host)); 
for f in host: print('  ', f, len(f.req))
lo = leftover_calls(allfns, files)
print('leftover call-position uses of narrowed names:', len(lo))
for x in lo[:40]: print(x[0].split('/')[-2], x[1], x[2], x[3])
nf = {f.name for f in allfns if f.mode == 'narrow' and f.parent is None}
for path, (text, toks) in files.items():
    for i, t in enumerate(toks):
        if t.k == 'id' and t.s in nf and toks[i-1].s not in ('fn', '.') and (toks[i+1].s != '(') and toks[i+1].s not in (',',) or False:
            pass
cnt = 0
for path, (text, toks) in files.items():
    for i, t in enumerate(toks):
        if t.k == 'id' and t.s in nf and toks[i-1].s != 'fn' and toks[i+1].s != '(':
            cnt += 1
            print('value-use', path.split('/')[-2], text.count('\n', 0, t.a)+1, t.s, toks[i-1].s, toks[i+1].s)
print('value uses', cnt)
