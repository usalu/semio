import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p19 as P
n = sys.argv[1]
doc = P.Doc(open(os.path.join(P.T, 'orig', n + '.rs.txt'), encoding='utf-8').read())
P.listing(doc)
print('REGIONS')
for r in doc.regions: print(r['id'], r['open'][:80])
