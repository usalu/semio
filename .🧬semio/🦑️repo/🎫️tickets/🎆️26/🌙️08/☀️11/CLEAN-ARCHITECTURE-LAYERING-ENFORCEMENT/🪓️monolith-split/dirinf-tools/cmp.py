import sys,re,collections
def load(p,mono):
    c=collections.Counter()
    for l in open(p,encoding='utf-8'):
        l=l.rstrip('\n')
        m=re.match(r'^(.*?🦀️\.rs|.*?\.rs|[^:]+):(.*)$',l)
        if not m:
            c[l]+=1;continue
        path,rest=m.group(1),m.group(2)
        for mp in mono:
            if path.startswith(mp+'/') or path==mp+'/🦀️.rs':
                path=mp+'/🦀️.rs'
        c[path+':'+rest]+=1
    return c
a=load(sys.argv[1],sys.argv[3:]);b=load(sys.argv[2],sys.argv[3:])
ma=a-b;mb=b-a
print('only-in-orig',sum(ma.values()),'only-in-split',sum(mb.values()))
for k,v in sorted(ma.items()):print('  -',v,k[:260])
for k,v in sorted(mb.items()):print('  +',v,k[:260])
