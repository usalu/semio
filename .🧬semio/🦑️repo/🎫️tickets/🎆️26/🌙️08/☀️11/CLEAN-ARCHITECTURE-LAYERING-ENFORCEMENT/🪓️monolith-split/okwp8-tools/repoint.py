"""🧭 For every TS/JSON reader of the old store unit test file: which new files hold the strings it looks for."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L
FE = '️'
OLD = '🏪️store/🧪️tests/🔬️unit/🦀️.rs'
census = json.load(open(L.GEN + '/okwp8-census.json'))
cand = {}
for crate in census:
    if crate == 'os-kernel': continue
    d = json.load(open(L.GEN + '/okwp8-census-%s.json' % crate))
    for r in d['test native']:
        if r['kind'] != 'embedded' and r['path'].endswith('.rs'):
            p = L.R + '/' + r['path']
            if p not in cand: cand[p] = open(p, encoding='utf8').read()
# unit-origin files: chunk files and F/B modules (not the other moved tests)
unitish = {p: t for p, t in cand.items() if '/🔬️unit/🦀️.rs' in p and ('bin-' in p or 'companion' in p) or '/🧪️testing/' in p}
def literals(window):
    out = []
    for m in re.finditer(r'"((?:[^"\\\n]|\\.){8,200})"', window):
        s = m.group(1).encode().decode('unicode_escape', 'ignore') if '\\' in m.group(1) else m.group(1)
        out.append(s)
    for m in re.finditer(r'`((?:[^`\\\n]|\\.){8,200})`', window):
        out.append(m.group(1))
    return out
def report(ts_path, line_no, window_chars=3000):
    t = open(ts_path, encoding='utf8').read().split('\n')
    start = line_no - 1
    window = '\n'.join(t[start:start + 40])[:window_chars]
    lits = literals(window)
    cov = collections.Counter(); per = {}
    for l in lits:
        hit = [p for p, tx in unitish.items() if l in tx]
        per[l] = hit
        for p in hit: cov[p] += 1
    return lits, per, cov
if __name__ == '__main__':
    refs = [l.split(':', 2) for l in sys.stdin.read().split('\n') if l.strip()]
    for f, ln, rest in refs:
        lits, per, cov = report(L.R + '/' + f, int(ln))
        print('###', f.split('🔨️modules/')[-1][-70:], ln, 'literals', len(lits))
        for p, n in cov.most_common(4): print('    ', n, L.rel(p).split('🔨️modules/')[-1][-70:])
        miss = [l[:60] for l, h in per.items() if not h][:6]
        if miss: print('     no hit:', miss)
