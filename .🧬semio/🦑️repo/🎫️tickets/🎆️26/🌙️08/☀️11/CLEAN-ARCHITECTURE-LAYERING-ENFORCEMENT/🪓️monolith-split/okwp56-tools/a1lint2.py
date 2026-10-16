# 🔬 Delegate lint: every one-line delegate of the composer forwards exactly the callee's declared parameters (needs first, then the original arguments in order).
import re, sys, json
import em, rs, a1lib, a1gen
from a1lib import *
text = em.read()
L = text.split('\n')
ms = a1lib.load_methods(open('pre-a1/root-pre-a1.rs', encoding='utf8').read())
bad = 0; n = 0
for i, l in enumerate(L):
    if 'Delegates to' not in (L[i - 1] if i else ''): continue
    if not l.strip().endswith(' }'): continue
    m = re.search(r' fn (\w+)', l)
    k = l.rindex(' { ') if ' { ' in l else -1
    body = l[k + 3:-2].strip()
    mc = re.match(r'(?:self\.(\w+)\.|(\w+)::<P, Mutation>::)(\w+)(?:::<[^>]*>)?\((.*)\)(?:\.await)?$', body)
    if not m or not mc: print('unparsed', l[:120]); bad += 1; continue
    n += 1
    name = m.group(1); r = ms.get(name)
    if r is None: print('unknown delegate', name); bad += 1; continue
    assert mc.group(3) == name, (name, mc.group(3))
    args = mc.group(4)
    ents = a1gen.split_top(args, 0, len(args)); got = [args[a:b].strip() for a, b in ents]
    want_needs = [('&mut ' if mut_of(r.recv) else '&') + 'self.' + c for c in r.need]
    want_args = [p[0] for p in param_names(r.sig)]
    if got != want_needs + want_args: bad += 1; print('delegate mismatch', name, got, want_needs + want_args)
print(n, 'delegates checked,', bad, 'mismatches')
