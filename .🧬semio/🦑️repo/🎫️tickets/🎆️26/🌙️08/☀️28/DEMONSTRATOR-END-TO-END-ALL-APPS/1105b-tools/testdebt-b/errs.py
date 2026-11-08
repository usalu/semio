import re,sys
t=open(sys.argv[1],errors="ignore").read()
blocks=re.split(r"\n(?=error(?:\[E\d+\])?: )",t)
n=0
for b in blocks:
    ls=b.split("\n")
    if ls[0].startswith("error") and len(ls)>1 and "-->" in ls[1]:
        n+=1
        print(ls[0][:110],"@",ls[1].split("rust/")[-1][-70:])
        for l in ls[2:14]:
            if "^" in l and len(l)<500: print("    ",l.strip()[:220])
print("errors",n)
