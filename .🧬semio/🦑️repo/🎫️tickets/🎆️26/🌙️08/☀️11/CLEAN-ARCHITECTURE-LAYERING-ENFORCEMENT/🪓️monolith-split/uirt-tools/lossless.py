"""🔎️ Losslessness: multiset of non-blank trimmed lines of the original runtime .rs tree against the carved tree."""
import os,collections,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/orig"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
NEW=f"/Users/ueli/Documents/semio/{RT}"
def bag(root):
    c=collections.Counter()
    for dp,dn,fn in os.walk(root):
        dn[:]=[d for d in dn if d not in("target","node_modules")]
        for f in fn:
            if f.endswith(".rs"):
                for l in open(os.path.join(dp,f),encoding="utf-8").read().split("\n"):
                    l=l.strip()
                    if l and l not in("}","{","};","});",")","],"): c[l]+=1
    return c
a=bag(f"{S}/{RT}"); b=bag(NEW)
only_a=a-b; only_b=b-a
print("original lines",sum(a.values()),"carved lines",sum(b.values()))
print("only in original:",sum(only_a.values()),"distinct",len(only_a)); 
print("only in carved:",sum(only_b.values()),"distinct",len(only_b))
if "-v" in sys.argv:
    print("--- only original"); 
    for l,n in sorted(only_a.items(),key=lambda x:-x[1])[:int(sys.argv[2]) if len(sys.argv)>2 else 80]: print(n,l[:170])
    print("--- only carved")
    for l,n in sorted(only_b.items(),key=lambda x:-x[1])[:int(sys.argv[2]) if len(sys.argv)>2 else 80]: print(n,l[:170])
