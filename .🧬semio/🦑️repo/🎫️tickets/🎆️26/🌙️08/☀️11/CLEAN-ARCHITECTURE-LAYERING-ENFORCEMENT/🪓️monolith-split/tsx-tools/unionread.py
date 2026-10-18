import sys,re,os,hashlib,shutil
F=os.path.dirname(os.path.abspath(__file__))
R="/Users/ueli/Documents/semio/"
IMP='import { readdirSync as tsxReaddir, readFileSync as tsxRead } from "node:fs";\nconst tsxOwner = import.meta.dir.slice(0, import.meta.dir.indexOf(OWNER) + OWNER.length);\nconst tsxTree = (directory: string): string => tsxReaddir(directory, { withFileTypes: true }).filter(entry => !["node_modules", "🧪️tests", "🧪️testing", "📦️packages"].includes(entry.name)).flatMap(entry => entry.isDirectory() ? [tsxTree(`${directory}/${entry.name}`)] : entry.name === "🦀️.rs" ? [tsxRead(`${directory}/${entry.name}`, "utf8")] : []).join("\\n");\n'
def args_end(s,i):
    d=0;q=None;j=i
    while j<len(s):
        c=s[j]
        if q:
            if c=="\\": j+=2;continue
            if c==q: q=None
        elif c in "\"'`": q=c
        elif c in "([{": d+=1
        elif c in ")]}":
            if d==0: return j
            d-=1
        j+=1
    return -1
def split_top(s):
    d=0;q=None;out=[];cur="";j=0
    while j<len(s):
        c=s[j]
        if q:
            cur+=c
            if c=="\\": cur+=s[j+1]; j+=2; continue
            if c==q: q=None
        elif c in "\"'`": q=c;cur+=c
        elif c in "([{": d+=1;cur+=c
        elif c in ")]}": d-=1;cur+=c
        elif c=="," and d==0: out.append(cur);cur=""
        else: cur+=c
        j+=1
    out.append(cur);return out
def convert(path,write):
    s=open(path,encoding="utf8").read()
    n=0;out="";pos=0
    for m in re.finditer(r"\b(?:readFileSync|Bun\.file)\(",s):
        pass
    i=0
    res=[];last=0
    for m in re.finditer(r"\breadFileSync\(",s):
        st=m.end(); e=args_end(s,st)
        if e<0: continue
        parts=split_top(s[st:e])
        if len(parts)==2 and "🦀️.rs" in parts[0] and parts[1].strip().strip("\"'")  in ("utf8","utf-8"):
            res.append((m.start(),e+1,"tsxTree(tsxOwner)"));n+=1
    for m in re.finditer(r"(?:\bawait\s+)?\bBun\.file\(",s):
        st=m.end(); e=args_end(s,st)
        if e<0: continue
        tail=s[e+1:e+9]
        mt=re.match(r"\s*\.text\(\)",s[e+1:e+12])
        if mt and "🦀️.rs" in s[st:e]:
            res.append((m.start(),e+1+mt.end(),"tsxTree(tsxOwner)"));n+=1
    if not n: return 0
    for a,b,t in reversed(res): s=s[:a]+t+s[b:]
    if "const tsxTree" not in s:
        lines=s.split("\n"); idx=0
        for k,l in enumerate(lines[:40]):
            if re.match(r"\s*import\b",l): idx=k+1
        lines.insert(idx,IMP.replace("OWNER",repr(os.environ["TSX_OWNER"]).replace("'","\"")).rstrip("\n")); s="\n".join(lines)
    if write:
        h=hashlib.sha1(path.encode()).hexdigest()[:10]
        bk=os.path.join(F,"orig",h+".ts")
        if not os.path.exists(bk): shutil.copy(path,bk); open(os.path.join(F,"orig","index.tsv"),"a",encoding="utf8").write(h+"\t"+path+"\n")
        open(path,"w",encoding="utf8").write(s)
    return n
write="--write" in sys.argv
for p in sys.argv[1:]:
    if p.startswith("--"): continue
    fp=p if p.startswith("/") else R+p
    print(convert(fp,write),p[-100:])
