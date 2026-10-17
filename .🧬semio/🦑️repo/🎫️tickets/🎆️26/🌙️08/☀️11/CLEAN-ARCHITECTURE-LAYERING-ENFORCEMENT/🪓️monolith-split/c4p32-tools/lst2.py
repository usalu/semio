import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p32 as P
n = sys.argv[1]
txt = open(os.path.join(P.T, 'orig', n + '.rs.txt'), encoding='utf-8').read()
d = P.Doc(txt)
prev = None
for it in d.items:
    if it['kind'] in ('use',) :
        continue
    print('%d-%d\t%d\t%s %s %s\t%s' % (it['s']+1, it['e']+1, it['e']-it['s']+1, it['vis'], it['kind'], it['name'][:70], ' '.join(it['attrs'])[:60]))
print('regions', [ (r['name'][:50]) for r in d.regions])
