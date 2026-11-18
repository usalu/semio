import sys,re
def load(p):
    d={}
    for l in open(p,encoding='utf8'):
        m=re.match(r'(.*) pass=(\d+) fail=(\d+)$',l.strip())
        if m:d[m.group(1)]=(int(m.group(2)),int(m.group(3)))
    return d
a,b=load(sys.argv[1]),load(sys.argv[2]);bad=0
for k in sorted(b):
    x=a.get(k);y=b.get(k)
    flag=''
    if y is None or (x is not None and (y[1]>x[1] or y[0]<x[0])) : flag='WORSE';bad+=1
    print(k,x,'->',y,flag)
sys.exit(1 if bad else 0)
