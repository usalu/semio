import sys
sys.path.insert(0, '.')
import c4p8
d = c4p8.Doc(open('orig/%s.rs.txt' % sys.argv[1], encoding='utf-8').read())
for i, it in enumerate(d.items):
    print('%d\t%d-%d\t%d\t%s\t%s\t%s\t%s' % (i, it['s']+1, it['e']+1, it['e']-it['s']+1, it['vis'], it['kind'], it['name'][:50], ' '.join(it['attrs'])[:40]))
print('regions')
for r in d.regions:
    print(r['id'], r['open'])
