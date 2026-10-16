"""Re-point every external call of a whole-state wrapper to the component form (explicit component borrows)."""
import re, sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wunwrap1 as U
import wapi
from wext import external_world_names
from wrewrite import comp_list

def plan():
    E = set(external_world_names())
    api, _ = wapi.narrow_api(E)
    names = U.wrapper_names()
    spec = {}
    for nm in names:
        f, _fd = api[nm]
        spec[nm] = (comp_list(f), f.mut)
    return names, spec

def rewrite(apply):
    names, spec = plan()
    cs = U.calls(names)
    byfile = collections.defaultdict(list)
    left = []
    for c in cs:
        a = c['arg0']
        m = re.fullmatch(r'(?:&mut\s+)?([A-Za-z_][A-Za-z0-9_.]*)', a)
        if not m: left.append(c); continue
        comps, mut = spec[c['name']]
        base = m.group(1)
        if comps:
            rep = ', '.join(f"&{'mut ' if mut else ''}{base}.{x}" for x in comps)
            byfile[c['file']].append((c['a'], c['b'], rep))
        else:
            byfile[c['file']].append((c['a'], c['b'], None, c))
    n = 0
    for f, edits in byfile.items():
        p = U.R + f
        t = open(p, encoding='utf-8').read()
        out = t
        for e in sorted(edits, key=lambda e: -e[0]):
            a, b = e[0], e[1]
            rep = e[2]
            if rep is None:
                j = b
                while out[j] in ' \t\n': j += 1
                if out[j] == ',':
                    j += 1
                    while out[j] in ' \t\n': j += 1
                out = out[:a] + out[j:]
            else:
                out = out[:a] + rep + out[b:]
            n += 1
        if apply: open(p, 'w', encoding='utf-8').write(out)
    return n, left, spec

if __name__ == '__main__':
    apply = len(sys.argv) > 1 and sys.argv[1] == 'apply'
    n, left, spec = rewrite(apply)
    print('edits', n, 'apply' if apply else 'dry')
    for c in left: print('LEFT', c['file'].split('/')[-2], c['line'], c['name'], c['arg0'][:100])
    print(sum(1 for k, v in spec.items() if not v[0]), 'wrappers need no component')
