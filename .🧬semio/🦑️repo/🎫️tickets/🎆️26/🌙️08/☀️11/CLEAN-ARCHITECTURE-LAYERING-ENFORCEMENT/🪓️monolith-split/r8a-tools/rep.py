import sys,json
# usage: rep.py <file> <old> <new>  (old must occur exactly once)
f,old,new=sys.argv[1:4]
t=open(f,encoding='utf8').read()
n=t.count(old)
assert n==1,f'{f}: anchor count {n}: {old[:80]!r}'
open(f,'w',encoding='utf8').write(t.replace(old,new))
print('ok',f[-60:])
