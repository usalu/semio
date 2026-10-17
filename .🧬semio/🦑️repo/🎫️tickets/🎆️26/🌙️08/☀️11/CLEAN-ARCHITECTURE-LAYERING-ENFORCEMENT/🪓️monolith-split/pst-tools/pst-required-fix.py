import re,json,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ROOT="/Users/ueli/Documents/semio/"
R=ROOT+F+"/🗑️generated/"
def load(p):
    lines=open(p,encoding="utf8").read().split("\n"); out=[]; i=0
    while i<len(lines):
        m=re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",lines[i])
        if m:
            cont=[]; j=i+1
            while j<len(lines) and lines[j].startswith("  "): cont.append(lines[j].strip()); j+=1
            out.append((m.group(1),int(m.group(2)),m.group(4),m.group(5),cont)); i=j
        else: i+=1
    return out
a=load(R+sys.argv[1]); b=load(R+sys.argv[2])
ka=collections.Counter((f,c,m) for f,l,c,m,_ in a)
proposals=collections.defaultdict(set)
for f,l,c,m,cont in b:
    if ka[(f,c,m)]>0: ka[(f,c,m)]-=1; continue
    if c!="TS2322" or not re.match(r"Type '.*\bundefined' is not assignable to type '(?!.*undefined)",m): continue
    src=open(ROOT+f,encoding="utf8").read().split("\n")
    line=src[l-1]
    fm=re.match(r"^\s*(\w+): (?:row\d*\[[^\]]+\]|\w+) === undefined \? undefined :",line) or re.match(r"^\s*(\w+): row\d*\[\"(\w+)\"\] (?:===|==) (?:undefined|null) \? undefined",line)
    if not fm: continue
    field=fm.group(1)
    k=l-1
    while k>=0 and not src[k].startswith("export function parse"): k-=1
    fn=re.match(r"export function parse(\w+)\(",src[k]).group(1)
    proposals[(f,fn)].add(field)
for (f,fn),fields in sorted(proposals.items()):
    print(f.replace("✏️s/🔌️plugins/","")[:100],fn,sorted(fields))
json.dump([[f,fn,sorted(v)] for (f,fn),v in proposals.items()],open(R+"pst-required-proposals.json","w",encoding="utf8"),ensure_ascii=False)
