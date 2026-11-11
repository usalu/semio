import re,sys
G=sys.argv[1]
fs=[l.split(" ")[1] for l in open(G+"/alloc-remove.log") if l.startswith("EDIT")]
tot=0
for f in fs:
    s=open(f).read()
    n=s
    for t in ["RequestedAllocator","HeapWitness"]:
        occ=[m for m in re.finditer(r'\b'+t+r'\b',n)]
        if not occ:continue
        lines=n.split("\n")
        idx=[i for i,l in enumerate(lines) if re.search(r'\b'+t+r'\b',l)]
        if not all(lines[i].lstrip().startswith("use ") and lines[i].rstrip().endswith(";") for i in idx):continue
        for i in idx:
            l=lines[i]
            m=re.match(r'^(\s*)use\s+([\w:]+)::\{([^}]*)\}\s*;\s*$',l)
            if m:
                items=[x.strip() for x in m.group(3).split(",") if x.strip() and x.strip()!=t]
                if not items:lines[i]=None
                elif len(items)==1 and "::" not in items[0] and " as " not in items[0]:lines[i]=f"{m.group(1)}use {m.group(2)}::{items[0]};"
                else:lines[i]=f"{m.group(1)}use {m.group(2)}::{{{','.join(items)}}};"
            elif re.match(r'^\s*use\s+[\w:]+::'+t+r'\s*;\s*$',l):lines[i]=None
        n="\n".join(x for x in lines if x is not None)
    if n!=s:
        tot+=1;print(f[-70:]);open(f,"w").write(n)
print(tot)
