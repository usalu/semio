import re,collections,sys
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
R="/Users/ueli/Documents/semio/"+F+"/🗑️generated/"
def load(p):
    lines=open(p,encoding="utf8").read().split("\n")
    out=[]; i=0
    while i<len(lines):
        m=re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",lines[i])
        if m:
            cont=[]; j=i+1
            while j<len(lines) and lines[j].startswith("  "): cont.append(lines[j].strip()); j+=1
            out.append((m.group(1),int(m.group(2)),m.group(4),m.group(5),cont)); i=j
        else: i+=1
    return out
a=load(R+sys.argv[1]); b=load(R+sys.argv[2]); sub=sys.argv[3]
ka=collections.Counter((f,c,m) for f,l,c,m,_ in a)
for f,l,c,m,cont in b:
    if ka[(f,c,m)]>0: ka[(f,c,m)]-=1; continue
    if sub in f:
        src=open("/Users/ueli/Documents/semio/"+f,encoding="utf8").read().split("\n")[l-1]
        print(l,c,m[:200]); 
        for x in cont[:2]: print("     ",x[:200])
        print("      >>",src.strip()[:230])
