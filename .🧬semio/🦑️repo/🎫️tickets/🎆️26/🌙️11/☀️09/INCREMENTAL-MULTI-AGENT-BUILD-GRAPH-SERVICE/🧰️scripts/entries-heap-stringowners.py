import sys,collections,re
F=sys.argv[1]
typ={};size={}
mode=None
for line in open(F,encoding="utf8"):
    if line.startswith("## All Objects"):mode="o";continue
    if line.startswith("## All Edges"):mode="e";break
    if mode=="o" and line.startswith("|") and line.split("|")[1].strip().isdigit():
        p=line.split("|");typ[int(p[1])]=p[2].strip().strip("`")
owner=collections.Counter();cnt=collections.Counter()
mode=None
for line in open(F,encoding="utf8"):
    if line.startswith("## All Edges"):mode="e";continue
    if line.startswith("## String Values"):break
    if mode=="e" and line.startswith("|") and line.split("|")[1].strip().isdigit():
        p=line.split("|");a=int(p[1]);b=int(p[2])
        if typ.get(b)=="string":
            k=typ.get(a,"?")+"."+p[4].strip()[:30];cnt[k]+=1
print("edges into strings by (owner type . property):")
for k,v in cnt.most_common(25):print(v,k)
