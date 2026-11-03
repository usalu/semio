import sys,re,ast
sys.path.insert(0,".")
exec(open("res.py").read().split("for r in rows")[0])
pat=re.compile(r'\.unwrap\(\)|\.expect\(|panic!\(|unreachable!')
i=int(sys.argv[1]);cr,c,ln,cls=rows[i]
print(i,cr,c[0][-60:] ,ln)
import os
PP="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/pp/"
L=open((PP if os.path.exists(PP+c[0]) else R+"/")+c[0],encoding="utf8").read().split("\n")
w=int(sys.argv[2]) if len(sys.argv)>2 else 400
print("LINES",len(L))
for k in range(max(0,ln-1-int(sys.argv[3] if len(sys.argv)>3 else 0)),ln):
    s=L[k];print(k+1,len(s))
    for m in pat.finditer(s):print("  >>",s[max(0,m.start()-w):m.end()+w//2])
