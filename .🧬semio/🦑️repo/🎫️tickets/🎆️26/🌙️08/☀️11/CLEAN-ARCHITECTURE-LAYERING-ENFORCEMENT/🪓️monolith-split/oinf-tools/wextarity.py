"""Arity of every external call of a re-pointed world function against its definition in the world crates."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import warity, wunwrap1 as U
from wengine import *
from warity import count_args
from oinf_rs import lex

def run(names):
    inf = U.R + "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world"
    defs = []
    for dp, dn, fn in os.walk(inf):
        if 'node_modules' in dp or '🧪️tests' in dp or '♻️resources' in dp: continue
        for f in fn:
            if f.endswith('.rs'): defs.append(os.path.join(dp, f))
    free, meth, typemeth, texts = warity.scan(defs)
    bad = []; n = 0
    for f in U.files():
        t = open(U.R + f, encoding='utf-8').read()
        toks = [x for x in lex(t) if x.k != 'comment']
        for i, x in enumerate(toks):
            if x.k == 'id' and x.s in names and i + 1 < len(toks) and toks[i+1].s == '(' and toks[i-1].s != 'fn':
                a = count_args(toks, i + 1); n += 1
                if a not in free[x.s]: bad.append((f.split('/')[-2], t.count('\n', 0, x.a) + 1, x.s, a, sorted(free[x.s])))
    return n, bad

if __name__ == '__main__':
    n, bad = run(set(open(U.R + '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/oinf-wrapper-names.txt', encoding='utf-8').read().split()))
    print('checked', n, 'mismatches', len(bad))
    for b in bad: print(b)
