import sys,os
src,out=sys.argv[1:3]
s=set()
for l in open(src,encoding='utf8'):
    x=os.path.dirname(l.rstrip('\n'))
    while x and x not in s:
        s.add(x); x=os.path.dirname(x)
open(out,'w',encoding='utf8').write('\n'.join(sorted(s))+'\n')
print(len(s))
