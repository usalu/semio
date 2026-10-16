import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import MARK
def norm_lines(path):
    out = []
    for ln in open(path, encoding='utf-8').read().split('\n'):
        s = ln.strip()
        if not s or MARK.match(ln): continue
        s = s.replace('pub(super)', 'pub(crate)')
        out.append(s)
    return out
def collect(R, rels):
    c = collections.Counter()
    for r in rels:
        p = os.path.join(R, r)
        if os.path.exists(p) and r.endswith('.rs'): c.update(norm_lines(p))
    return c
def run(base, work, base_list, work_list):
    b = collect(base, base_list); w = collect(work, work_list)
    return b - w, w - b
if __name__ == '__main__':
    S = sys.argv[1]
    base_list = [l.strip() for l in open(sys.argv[2], encoding='utf-8') if l.strip()]
    census = json.load(open(sys.argv[3]))
    work_list = sorted({p for r in census for p, k, b, l in r.get('list', []) if p.endswith('.rs')} | {'🧰️framework/📦️packages/🦀️rust/🦀️.rs', '🧰️framework/🔨️modules/🧬️schema/📽️projection/🧪️tests/🦀️.rs', '🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs'})
    m, e = run(S + 'base', S + 'work', base_list, work_list)
    print('missing', sum(m.values()), 'extra', sum(e.values()))
    out = open(sys.argv[4], 'w')
    out.write('--- MISSING\n')
    for k, v in sorted(m.items()): out.write(f'{v}\t{k}\n')
    out.write('--- EXTRA\n')
    for k, v in sorted(e.items()): out.write(f'{v}\t{k}\n')
