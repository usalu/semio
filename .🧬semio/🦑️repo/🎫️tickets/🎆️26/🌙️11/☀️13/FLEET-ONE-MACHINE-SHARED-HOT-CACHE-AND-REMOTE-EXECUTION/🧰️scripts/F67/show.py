import sys,re
d=sys.argv[1]; maxl=int(sys.argv[2]) if len(sys.argv)>2 else 6; w=int(sys.argv[3]) if len(sys.argv)>3 else 260
m=d[:-6]+".mjs"
src=open(d).read()
i=src.find("Declared from the script by `repo typecheck`")
blk=src[src.rfind("\n",0,i)+1:]
print("=== STUB",d.split("library/")[-1])
print("\n".join(l for l in blk.split("\n")[1:]))
names=re.findall(r"export (?:async )?(?:function|const|class) (\w+)",blk)
ms=open(m).read().split("\n")
for n in names:
    for k,l in enumerate(ms):
        if re.search(r"(function\*? |const |class )%s\b"%n,l) and "export" in l:
            print("--- %s @%d"%(n,k+1))
            for x in ms[k:k+maxl]: print(x[:w])
            break
    else: print("--- %s not found"%n)
