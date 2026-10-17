#!/usr/bin/env python3
# usage: cmp.py <base.short> <cur.short>  compares multisets of (code, message) of rustc short diagnostics, ignoring files/lines
import re,sys,collections
def load(p):
    c=collections.Counter()
    for l in open(p,encoding='utf8'):
        m=re.match(r'^(.*?):(\d+):(\d+): (error|warning)(\[E\d+\])?: (.*)$',l.rstrip('\n'))
        if m and m.group(4)=='error': c[(m.group(5) or '', m.group(6))]+=1
    return c
a,b=load(sys.argv[1]),load(sys.argv[2])
print("base",sum(a.values()),"cur",sum(b.values()))
only_b=b-a; only_a=a-b
print("only cur:",sum(only_b.values()))
for (k,v),n in sorted(only_b.items(),key=lambda x:str(x))[:int(sys.argv[3]) if len(sys.argv)>3 else 60]: print("  +",n,k,v[:200])
print("only base:",sum(only_a.values()))
for (k,v),n in sorted(only_a.items(),key=lambda x:str(x))[:30]: print("  -",n,k,v[:200])
