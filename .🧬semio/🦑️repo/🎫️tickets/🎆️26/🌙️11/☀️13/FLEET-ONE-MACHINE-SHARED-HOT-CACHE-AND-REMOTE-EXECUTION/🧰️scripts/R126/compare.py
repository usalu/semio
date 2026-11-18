import re,sys
def rows(p):
    d={}
    for l in open(p,encoding="utf8"):
        f,_,r=l.rstrip("\n").partition(" :: ")
        d[f]=(int((re.search(r"(\d+) pass",r) or [0,0])[1]),int((re.search(r"(\d+) fail",r) or [0,0])[1]))
    return d
b,a=rows(sys.argv[1]),rows(sys.argv[2]);bad=0;worse=[]
for f,(p,x) in a.items():
    if f in b:
        bp,bx=b[f]
        if x>bx or p<bp: print("WORSE",f[-60:],b[f],"->",(p,x));bad+=1;worse.append(f)
    elif x>0 or p==0: print("NEW RED",f[-60:],(p,x));bad+=1;worse.append(f)
open(sys.argv[3],"w",encoding="utf8").write("\n".join(worse)+("\n" if worse else "")) if len(sys.argv)>3 else None
print("compared",len(a),"files; worse:",bad)
sys.exit(1 if bad else 0)
