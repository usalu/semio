# ✅️ WP4 static check, part three: no direct use of a removed ShellState field or moved method on any ShellState-typed variable in the external consumers.
import re, sys, os, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1ext import *
from ext_overlay import candidate_files, R

def main(overlay):
    comps = set(FIELD_OF.values())
    moved_methods = {n for n in TIER1 if PLAN1[n].get('recv') in ('mut', 'ref')}
    bad = []
    n = 0
    for rel in candidate_files():
        ov = os.path.join(overlay, rel)
        p = ov if os.path.exists(ov) else os.path.join(R, rel)
        t = open(p, encoding='utf-8').read(); n += 1
        b = blank(t)
        for v in shell_vars(t):
            for m in re.finditer(r'(?<![\w])' + re.escape(v) + r'\s*\.\s*([A-Za-z_]\w*)', b):
                x = m.group(1)
                if x in comps: continue
                if x in F2S: bad.append((rel[-60:], v, 'field', x))
                elif x in moved_methods and re.match(r'\s*(::\s*<[^>]*>)?\s*\(', b[m.end():]): bad.append((rel[-60:], v, 'method', x))
    print(n, 'files scanned', len(bad), 'leftovers')
    for x in bad[:40]: print(x)

if __name__ == '__main__':
    main(sys.argv[1])
