import re,json,sys,collections
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ROOT="/Users/ueli/Documents/semio/"
R=ROOT+F+"/🗑️generated/"
def load(p):
    out=[]
    for line in open(p,encoding="utf8"):
        m=re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",line.rstrip("\n"))
        if m: out.append((m.group(1),int(m.group(2)),m.group(4),m.group(5)))
    return out
a=load(R+sys.argv[1]); b=load(R+sys.argv[2])
ka=collections.Counter((f,c,m) for f,l,c,m in a)
rows=set(); files=set()
cache={}
for f,l,c,m in b:
    if ka[(f,c,m)]>0: ka[(f,c,m)]-=1; continue
    src=cache.setdefault(f,open(ROOT+f,encoding="utf8").read().split("\n"))
    k=l-1
    while k>=0 and not re.match(r"^export function parse\w+",src[k]): k-=1
    if k<0: print("NO-ENCLOSING",f[-60:],l,m[:80]); continue
    name=re.match(r"^export function parse(\w+)",src[k]).group(1)
    rows.add((f,name)); files.add(f)
out=[{"path":f,"name":n} for f,n in sorted(rows)]
json.dump(out,open(R+sys.argv[3],"w",encoding="utf8"),ensure_ascii=False)
print(len(out),"parsers in",len(files),"files")
for f in sorted(files): print("  ",f.replace("✏️s/🔌️plugins/","").replace("🗿️artifacts/","")[:100],[n for ff,n in sorted(rows) if ff==f][:12])
