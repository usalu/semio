import sys,re
log,filt=sys.argv[1],sys.argv[2]
blocks=open(log).read().split("\n\n")
n=0
for b in blocks:
    if b.startswith("error") and filt in b and "aborting" not in b[:30]:
        lines=b.split("\n")
        out=[]
        for l in lines[:14]:
            out.append(re.sub(r'(rust/)(\.\./)+','',l)[:230])
        print("\n".join(out));print();n+=1
print(n,"blocks")
