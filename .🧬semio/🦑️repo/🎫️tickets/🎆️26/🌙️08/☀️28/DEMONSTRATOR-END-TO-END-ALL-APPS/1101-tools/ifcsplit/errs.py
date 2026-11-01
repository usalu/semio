import re,sys
t=open(sys.argv[1]).read().split("\n")
errs=[]
for i,l in enumerate(t):
    if l.startswith("error") and "aborting" not in l:
        loc=""
        for k in range(i+1,min(i+4,len(t))):
            if "-->" in t[k]: loc=t[k].strip();break
        errs.append((l,loc))
print(len(errs))
for m,l in errs: print(re.sub(r"^.*?(details|ifc)/","",l)[:90],"|",m[:170])
