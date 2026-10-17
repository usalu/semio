import sys
sys.path.insert(0, '.')
import c4p8
name, a, b, ded = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
L = open('orig/%s.rs.txt' % name, encoding='utf-8').read().split('\n')
body = [l[ded:] if l.startswith(' ' * ded) else l for l in L[a-1:b]]
d = c4p8.Doc('\n'.join(body))
for i, it in enumerate(d.items):
    print('%d\t%d-%d\t%d\t%s\t%s\t%s\t%s' % (i, it['s']+a, it['e']+a, it['e']-it['s']+1, it['vis'], it['kind'], it['name'][:50], ' '.join(it['attrs'])[:40]))
print('regions')
for r in d.regions:
    print(r['id'], r['open'])
