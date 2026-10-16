"""🎯 Exact-once text patcher for TS/JSON readers of the removed store unit test file."""
import os, sys, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L
R = L.R
SP = '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
S = R + '/' + SP
def bin_(n):
    return SP + '🧪️tests/' + [d for d in os.listdir(S + '🧪️tests') if d.endswith('bin-%02d' % n)][0] + '/🔬️unit/🦀️.rs'
def sbin(n):
    d = S + '🔄️sync/🧪️tests/'
    return SP + '🔄️sync/🧪️tests/' + [x for x in os.listdir(d) if x.endswith('bin-%d' % n)][0] + '/🔬️unit/🦀️.rs'
def comp(group):
    base = S + '🧩️composition/' + group + '/🧪️tests/'
    return SP + '🧩️composition/' + group + '/🧪️tests/' + [x for x in os.listdir(base) if x.endswith('companion')][0] + '/🔬️unit/🦀️.rs'
def dbl(name):
    base = S + '🧪️testing/🎭️doubles/'
    return SP + '🧪️testing/🎭️doubles/' + [x for x in os.listdir(base) if x.endswith(name)][0] + '/🦀️.rs'
def patch(rel, old, new):
    p = R + '/' + rel
    t = open(p, encoding='utf8').read()
    assert t.count(old) == 1, (rel, t.count(old), old[:80])
    open(p, 'w', encoding='utf8').write(t.replace(old, new))
    with open(os.path.dirname(os.path.abspath(__file__)) + '/touched.txt', 'a', encoding='utf8') as f:
        f.write('M\t' + rel + '\n')
    print('patched', rel[-70:])
def all_unit_paths():
    a = [SP + '🧪️testing/' + x + '/🦀️.rs' for x in ['🧭️prelude','🧮️snapshot','🧵️operations']]
    d = S + '🧪️testing/🎭️doubles/'
    b = [SP + '🧪️testing/🎭️doubles/🦀️.rs'] + [SP + '🧪️testing/🎭️doubles/' + x + '/🦀️.rs' for x in sorted(os.listdir(d)) if os.path.isfile(d + x + '/🦀️.rs')]
    bins = [bin_(n) for n in range(1, 11) if os.path.isfile(R + '/' + bin_(n))]
    return a + b + bins + [comp('🚪️open')]
def ts_list(paths):
    return '[' + ','.join('"%s"' % p for p in paths) + ']'
def patch_many(rel, pairs, expected):
    p = R + '/' + rel
    t = open(p, encoding='utf8').read()
    for old, new, n in pairs:
        assert t.count(old) == n, (rel, t.count(old), n, old[:90])
        t = t.replace(old, new)
    open(p, 'w', encoding='utf8').write(t)
    with open(os.path.dirname(os.path.abspath(__file__)) + '/touched.txt', 'a', encoding='utf8') as f:
        f.write('M\t' + rel + '\n')
    print('patched', rel[-70:])
