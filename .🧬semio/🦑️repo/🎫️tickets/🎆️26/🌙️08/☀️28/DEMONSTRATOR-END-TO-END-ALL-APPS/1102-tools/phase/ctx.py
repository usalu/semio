import sys
sys.path.insert(0,".")
exec(open("res.py").read().split("for r in rows")[0])
i=int(sys.argv[1]);b=int(sys.argv[2]);a=int(sys.argv[3]);W=int(sys.argv[4]) if len(sys.argv)>4 else 600
cr,c,ln,cls=rows[i-1]
print("##",i,cr,c[0],ln)
import os
PP="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/pp/"
L=open((PP if os.path.exists(PP+c[0]) else R+"/")+c[0],encoding="utf8").read().split("\n")
for k in range(max(0,ln-1-b),min(len(L),ln+a)):
    s=L[k];print(f"{k+1}: {s[:W]}"+("…" if len(s)>W else ""))
