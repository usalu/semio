# ✅️ WP4 static check, part four: a method or associated fn without `pub` that another file of the Shell tree names is a visibility defect (typeck-level E0624 that the resolution oracle cannot see).
import re, sys, os, glob, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *
from units import *

def main(root):
    files = sorted(glob.glob(root + '/📥️staging/*/*/🦀️.rs')) + sorted(glob.glob(root + '/*/🦀️.rs')) + [root + '/🦀️.rs'] + sorted(glob.glob(root + '/*/*/🦀️.rs'))
    files = [f for f in files if '/📥️staging/' in f or f.count('/') >= root.count('/') + 1]
    idents = {}; defs = []
    for p in set(files):
        if not os.path.isfile(p): continue
        t = open(p, encoding='utf-8').read(); s = Src(t)
        idents[p] = set(re.findall(r'[A-Za-z_]\w*', s.b))
        for it in top_items(s):
            if it['kind'] == 'impl' and ' for ' not in it['head'].split('{')[0]:
                for m in top_items(s, it['first'] + 1, it['last'], 4):
                    if m['kind'] in ('fn', 'const') and not re.match(r'\s*(pub|r#)', s.lines[m['first']]):
                        defs.append((p, item_name(m['head'], m['kind']), m['first'] + 1))
            elif it['kind'] in ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait') and not re.match(r'\s*pub', s.lines[it['first']]):
                defs.append((p, item_name(it['head'], it['kind']), it['first'] + 1))
    bad = []
    for p, nm, ln in defs:
        if not nm or nm == '_': continue
        others = [q for q in idents if q != p and nm in idents[q]]
        if others and not nm.startswith('__'): bad.append((p.replace(root + '/', '')[:60], nm, ln, len(others)))
    print(len(defs), 'private definitions,', len(bad), 'named from another file')
    for b in bad[:60]: print(b)
    return bad

if __name__ == '__main__':
    main(sys.argv[1])
