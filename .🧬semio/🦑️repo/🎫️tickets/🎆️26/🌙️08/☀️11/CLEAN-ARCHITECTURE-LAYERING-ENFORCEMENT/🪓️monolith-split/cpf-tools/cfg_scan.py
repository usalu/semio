import os,re,sys,json,collections
G=os.environ["G"]; ROOT="/Users/ueli/Documents/semio"
head=set(l.strip() for l in open(G+"/cpf-head-cfg-lines.txt",encoding="utf-8",errors="replace"))
paths=[l.rstrip("\n").split("\t",1)[1] for l in open(G+"/cpf-diff-name-status.txt",encoding="utf-8")]
paths+=[l.rstrip("\n") for l in open(G+"/cpf-untracked.txt",encoding="utf-8")]
paths=sorted(set(p for p in paths if p.endswith(".rs") and not p.startswith(".🧬semio/") and not p.startswith("Users/")))
PAT=re.compile(r'cfg\((?:any|all)\(([^()]*?(?:\([^()]*\)[^()]*?)*)\)\)')
rows=[]
for p in paths:
    try: t=open(os.path.join(ROOT,p),encoding="utf-8").read()
    except Exception: continue
    if "cfg(any(test" not in t and "cfg(all(test" not in t and "cfg_attr(" not in t: continue
    for i,l in enumerate(t.split("\n"),1):
        if "cfg(" not in l: continue
        for m in re.finditer(r'cfg(?:_attr)?\((any|all)\(',l):
            seg=l[m.start():]
            if not re.search(r'\btest\b',seg[:60]): continue
            feats=re.findall(r'feature\s*=\s*"([^"]+)"',seg.split("]")[0])
            if not feats: continue
            rows.append((p,i,l.strip(),feats,l.strip() in head))
json.dump(rows,open(G+"/cpf-cfg-rows.json","w"),ensure_ascii=False)
c=collections.Counter(); c2=collections.Counter()
for p,i,l,f,inh in rows:
    for x in f: c[(x,inh)]+=1
print(len(rows)); 
for k,v in sorted(c.items(),key=lambda x:-x[1]): print(v,k)
