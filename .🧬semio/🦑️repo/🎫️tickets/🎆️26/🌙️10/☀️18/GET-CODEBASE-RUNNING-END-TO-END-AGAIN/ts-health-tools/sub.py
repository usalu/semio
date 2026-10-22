import sys
f,old,new=sys.argv[1],sys.argv[2],sys.argv[3]
s=open(f,encoding="utf-8").read()
n=s.count(old)
if n!=1:
    print("COUNT",n,"for",old[:80]); sys.exit(1)
open(f,"w",encoding="utf-8").write(s.replace(old,new))
print("ok")
