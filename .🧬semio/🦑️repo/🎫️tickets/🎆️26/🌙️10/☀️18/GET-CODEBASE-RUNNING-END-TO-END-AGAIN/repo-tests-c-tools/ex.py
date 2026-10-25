import sys,re
S=sys.argv[1]; n=int(sys.argv[2]); k=int(sys.argv[3]) if len(sys.argv)>3 else 30
t=open("%s/logs/%04d.log"%(S,n),errors="replace").read().split("\n")
t=[l for l in t if l.strip() and not l.startswith("[script]")]
idx=[i for i,l in enumerate(t) if l.startswith("error:") or l.startswith("(fail)") ]
seen=0
out=[]
for i in idx:
    if t[i].startswith("error:"):
        out+=t[max(0,i-3):i+k]; out.append("-----"); seen+=1
        if seen>=2: break
for l in out: print(l[:230])
