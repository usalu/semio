import re,sys,collections
def norm(path):
    c=collections.Counter(); files=collections.defaultdict(collections.Counter)
    cur=None
    for line in open(path,encoding="utf8"):
        m=re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",line.rstrip("\n"))
        if m:
            f,_,_,code,msg=m.groups()
            if "typedwire" in f: key=("typedwire",code,msg)
            else: key=("*",code,msg)
            c[key]+=1
    return c
b=norm(sys.argv[1]); a=norm(sys.argv[2])
print("baseline errors",sum(b.values()),"after",sum(a.values()))
print("only baseline:",sum((b-a).values()))
for k,v in (b-a).items(): print("  -",v,k[1],k[2][:150])
print("only after:",sum((a-b).values()))
for k,v in (a-b).items(): print("  +",v,k[1],k[2][:150])
