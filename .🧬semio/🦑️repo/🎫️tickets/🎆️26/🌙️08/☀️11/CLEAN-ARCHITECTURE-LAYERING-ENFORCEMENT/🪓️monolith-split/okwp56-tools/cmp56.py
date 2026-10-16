# 🔎 Keyed comparison of two oracle runs (before/after): internal errors keyed by (code, highlighted text, file); prints the new and the vanished ones
import sys, os, collections, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
import okoracle as O
GEN = HERE + '/../🗑️generated'
SCR = O.SCR
FOLD = {}
def load(tag, cfg, fold=False):
    p = f'{GEN}/okwp0-oracle-{tag}-A-{cfg}.jsonl'
    err = open(p, encoding='utf8', errors='ignore').read()
    sb = f'{SCR}/ok-union-{tag}/A'
    return O.parse_keyed(err, sb, (lambda f: FOLD.get(f, f)) if fold else (lambda f: f))
def main():
    base, after, cfg = sys.argv[1], sys.argv[2], sys.argv[3]
    limit = int(sys.argv[4]) if len(sys.argv) > 4 else 60
    import json
    if '--fold' in sys.argv:
        d = json.load(open(sys.argv[sys.argv.index('--fold') + 1]))
        R = O.ROOT
        FOLD.update({os.path.relpath(k, R): os.path.relpath(v, R) for k, v in d.get('fold', {}).items()})
    a, ae, aex = load(base, cfg); b, be, bex = load(after, cfg, fold=True)
    new = b - a; old = a - b
    print(f'{cfg}: base internal={sum(a.values())} ext={sum(ae.values())} | after internal={sum(b.values())} ext={sum(be.values())} | new={sum(new.values())} old={sum(old.values())}')
    short = lambda f: f.replace('🧰️framework/🛍️products/💻️os/🔨️modules/', 'M/')
    for k, v in sorted(new.items(), key=lambda kv: (kv[0][2], kv[0][0]))[:limit]: print('  +new', v, (k[0], k[1][:50], short(k[2])[-70:]), '|', bex.get(k, '')[:90])
    for k, v in sorted(old.items(), key=lambda kv: (kv[0][2], kv[0][0]))[:limit]: print('  -old', v, (k[0], k[1][:50], short(k[2])[-70:]), '|', aex.get(k, '')[:90])
    extn = be - ae; exto = ae - be
    print('  external new', sum(extn.values()), 'old', sum(exto.values()))
    for k, v in list(extn.items())[:10]: print('   +ext', v, k)
main()
