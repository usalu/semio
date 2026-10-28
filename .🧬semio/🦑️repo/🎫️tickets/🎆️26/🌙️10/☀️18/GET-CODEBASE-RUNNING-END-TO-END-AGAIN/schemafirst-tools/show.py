import sys,re,os
import os
R=os.environ.get("SF_SRC","/Users/ueli/Documents/semio")+"/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
P="🧰️framework/🔨️modules/🏃️process/"
f=sys.argv[1].replace("L/",L,1) if sys.argv[1].startswith("L/") else sys.argv[1].replace("P/",P,1)
t=open(R+f,encoding="utf8").read()
w=int(sys.argv[3]) if len(sys.argv)>3 else 160
for m in re.finditer(sys.argv[2],t):
    a=max(0,m.start()-w);b=min(len(t),m.end()+w)
    line=t.count("\n",0,m.start())+1
    print(f"--- L{line}: ...{t[a:b]}...".replace("\n","⏎"))
