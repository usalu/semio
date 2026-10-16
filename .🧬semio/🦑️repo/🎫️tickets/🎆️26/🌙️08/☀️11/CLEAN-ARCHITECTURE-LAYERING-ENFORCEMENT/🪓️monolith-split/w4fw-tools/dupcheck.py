import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import SourceFile
from imports import file_imports
from cfg import *
def check(R):
    out = []
    roots = {c: os.path.join(R, v['root']) for c, v in CLUSTERS.items()}
    for c, p in roots.items():
        f = SourceFile(c, p)
        imps = collections.defaultdict(list); defs = collections.defaultdict(list)
        for u in f.units:
            if u.kind == 'use':
                for pub, items in file_imports(u.text):
                    for path, alias in items:
                        if alias in ('*', '_'): continue
                        imps[alias].append(('::'.join(x for x in path if x), u.test))
            elif u.kind in ('fn', 'struct', 'enum', 'type', 'trait', 'const', 'static', 'mod', 'union', 'macro_rules!') and u.name and not u.test:
                defs[u.name].append(u.kind)
        for a, lst in imps.items():
            if len(lst) > 1: out.append(('dup-import', c, a, lst))
            if a in defs: out.append(('import-vs-def', c, a, lst, defs[a]))
        for a, kinds in defs.items():
            if len(kinds) > 1 and not (set(kinds) <= {'fn', 'const'} and len(set(kinds)) == 1) and not (set(kinds) == {'mod'}): pass
    return out
if __name__ == '__main__':
    for x in check(sys.argv[1]): print(x)
    print('dupcheck done')
