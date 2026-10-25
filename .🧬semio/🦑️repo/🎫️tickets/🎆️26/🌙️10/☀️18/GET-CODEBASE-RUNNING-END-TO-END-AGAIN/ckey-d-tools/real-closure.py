"""Exact captured closure of an entry over a real implementation.json snapshot (the admitted parent): python3 real-closure.py <snapshot> <seed file>... prints size and the contributors of literal/schema edges."""
import json,base64,re,os,sys,collections
snap=json.load(open(sys.argv[1]))
root=snap["root"]
files={f["path"]:f for f in snap["files"]}
LIT=re.compile(rb"""(["'])(\.{1,2}/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1""")
DECL=re.compile(rb"\.d\.[cm]?ts$")
schemas=collections.defaultdict(list)
for p in files:
    parts=p.split("/")
    if "🧬️schema" in parts and p.endswith(".json"):
        i=parts.index("🧬️schema");schemas["/".join(parts[:i])].append(p)
def closure(seeds):
    seen=set();pending=list(seeds);why={}
    while pending:
        p=pending.pop()
        if p in seen or p not in files: continue
        seen.add(p)
        if p.endswith(".json"): continue
        row=files[p];data=base64.b64decode(row["base64"])
        def add(t,k):
            if t not in seen and t in files:
                why.setdefault(t,(p,k));pending.append(t)
        for s in schemas.get(os.path.dirname(p) if os.path.dirname(p) else "",[]):add(s,"schema")
        for spec,t in row["imports"].items():
            if not t.startswith("installed:"): add(t,"value")
        for m in LIT.finditer(data):
            if DECL.search(m.group(2)):continue
            c=os.path.relpath(os.path.normpath(os.path.join(root,os.path.dirname(p),m.group(2).decode())),root)
            add(c,"literal")
    return seen,why
if __name__=="__main__":
    seeds=[os.path.relpath(s,root) if os.path.isabs(s) else s for s in sys.argv[2:]]
    seen,why=closure(seeds)
    print("files",len(seen),"bytes",sum(len(base64.b64decode(files[p]['base64'])) for p in seen))
    by=collections.Counter()
    for t,(p,k) in why.items():
        if k!="value": by[(p[-80:],k)]+=1
    for (p,k),v in by.most_common(14):print(v,k,p)
