import sys
f,old,new=sys.argv[1],sys.argv[2],sys.argv[3]
t=open(f,encoding="utf8").read()
n=t.count(old)
if n!=1: sys.exit("count=%d"%n)
open(f,"w",encoding="utf8").write(t.replace(old,new))
print("ok")
