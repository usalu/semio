import re,sys,collections
G=sys.argv[1]
d=collections.defaultdict(collections.Counter)
for l in open(G+'/errs.txt'):
    m=re.match(r'(.*?):\d+:\d+: error\[(E\d+)\]: cannot find (\w+) `([^`]+)`',l)
    if m: d[m.group(1)][m.group(4)]+=1
for f,c in sorted(d.items()):
    print(f); print('   ',' '.join(sorted(c)))
