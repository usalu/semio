import sys,os,difflib,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import build
root="/Users/ueli/Documents/semio"
ch=build.build(root)
exts=tuple(sys.argv[1].split(","))
maxl=int(sys.argv[2]) if len(sys.argv)>2 else 200
for rel,(b,n,c) in sorted(ch.items()):
    if not rel.endswith(exts): continue
    a=b.decode().split("\n"); z=n.decode().split("\n")
    print("###",rel.split("/",3)[-1][-110:],c)
    sm=difflib.SequenceMatcher(None,a,z,autojunk=False)
    k=0
    for op,i1,i2,j1,j2 in sm.get_opcodes():
        if op=="equal": continue
        for i,j in zip(range(i1,i2),range(j1,j2)):
            x,y=a[i],z[j]
            s=difflib.SequenceMatcher(None,x,y,autojunk=False)
            segs=[f"[{x[max(0,p1-15):p2]}|{y[max(0,q1-15):q2]}]" for o,p1,p2,q1,q2 in s.get_opcodes() if o!="equal"]
            print("  ", " ".join(segs)[:maxl]); k+=1
            if k>25: break
        if k>25: print("   …"); break
