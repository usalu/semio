"""🔤 c4p16 module-name collision scan: a child module name must not equal any identifier the original declares or imports at the same level (type, value or macro namespace, conservatively all identifiers)."""
import importlib, os, re, sys
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p16 as C
from rsitems import lex

bad = 0
for key in ['part21', 'document', 'header_diff', 'zip_io', 'mp4_io', 'bcf_oracles', 'step_geometry', 'xlsx_oracles']:
    sp = importlib.import_module('specs.' + key).SPEC
    orig = C.load_orig(sp)
    ids = set(x for k, x, a, b, c in lex(orig) if k == 'id')
    def walk(spec, scope_ids, who):
        global bad
        for c in spec['children']:
            if c['mod'] in scope_ids:
                print('COLLISION', key, who, c['mod'])
                bad += 1
        for u in spec.get('unwrap', []):
            if u['name'] in [c['mod'] for c in spec['children']]:
                print('COLLISION', key, u['name'])
                bad += 1
            walk(u, scope_ids, who + '/' + u['name'])
    walk(sp, ids, key)
print('collisions', bad)
