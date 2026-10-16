"""🧭 Namespace of the kernel store root S/🦀️.rs: explicit use bindings, glob sources, inline hub modules, defined items."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import unitgraph, usetree
from okrs import tokenize
R = '/Users/ueli/Documents/semio/'
S = R + '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
def build(path=S + '🦀️.rs'):
    src, its = unitgraph.parse(path)
    explicit = {}      # name -> dict(path=[...], vis, cfg, line)
    globs = []         # (path, vis, line)
    defs = {}          # name -> kind
    hubs = {}          # module name -> list of use statements (paths, vis)
    for r in its:
        if r['kind'] == 'use':
            vis = r['vis']
            cfg = any('cfg' in a for a in r['attrs'])
            for segs, alias, glob in usetree.parse_use(r['text'].split('\n', 0)[0] if False else _strip_attrs(r['text'])):
                if glob: globs.append((segs, vis, r['a']))
                else:
                    nm = alias or (segs[-2] if segs[-1] == 'self' and len(segs) > 1 else segs[-1])
                    explicit[nm] = dict(path=segs if segs[-1] != 'self' else segs[:-1], vis=vis, line=r['a'], alias=alias, cfg=cfg)
        elif r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'type', 'macro_rules') and r['name']:
            defs[r['name']] = r['kind']
        elif r['kind'] == 'mod' and r['name']:
            defs[r['name']] = 'mod'
            if '{' in r['text']:
                inner = []
                for m in re.finditer(r'((?:pub(?:\([^)]*\))?\s+)?use\s[^;]*;)', r['text']):
                    for segs, alias, glob in usetree.parse_use(m.group(1)):
                        inner.append((segs, alias, glob))
                hubs[r['name']] = dict(uses=inner, vis=r['vis'], line=r['a'])
    return dict(explicit=explicit, globs=globs, defs=defs, hubs=hubs, src=src, items=its)
def _strip_attrs(t):
    # drop leading comments/attributes of an item text so parse_use starts at `use`
    return t
if __name__ == '__main__':
    ns = build()
    print(len(ns['explicit']), len(ns['globs']), len(ns['defs']), len(ns['hubs']))
    print('hubs', list(ns['hubs']))
    print('globs', [(g[0], g[1]) for g in ns['globs']])
    print('defs', sorted(ns['defs']))
