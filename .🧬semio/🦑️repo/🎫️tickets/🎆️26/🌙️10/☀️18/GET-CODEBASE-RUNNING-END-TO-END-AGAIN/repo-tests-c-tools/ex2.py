import sys
S=sys.argv[1]; n=int(sys.argv[2]); k=int(sys.argv[3]) if len(sys.argv)>3 else 12; mx=int(sys.argv[4]) if len(sys.argv)>4 else 2
t=[l for l in open("%s/logs/%04d.log"%(S,n),errors="replace").read().split("\n") if l.strip() and not l.startswith("[script]")]
c=0
for i,l in enumerate(t):
    if l.startswith("(fail)"):
        print("\n".join(x[:240] for x in t[max(0,i-k):i+1])); print("-----"); c+=1
        if c>=mx: break
