import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import oracle as O
g, cfg = sys.argv[1], sys.argv[2]
d = os.path.join(O.SCR, 'orc')
a = open(os.path.join(d, '%s-orig-%s.err' % (g, cfg)), encoding='utf-8').read()
b = open(os.path.join(d, '%s-new-%s.err' % (g, cfg)), encoding='utf-8').read()
ae, aw = O.parse(a); be, bw = O.parse(b)
print(g, cfg, 'errors orig', sum(ae.values()), 'new', sum(be.values()), 'only-orig', sum((ae - be).values()), 'only-new', sum((be - ae).values()), '| warnings orig', sum(aw.values()), 'new', sum(bw.values()), 'only-orig', sum((aw - bw).values()), 'only-new', sum((bw - aw).values()))
for k, v in list((be - ae).items())[:10]: print('  +new', v, k[:200])
for k, v in list((ae - be).items())[:10]: print('  -orig', v, k[:200])
