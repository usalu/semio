import json,os,re,sys,collections
log,mutations=sys.argv[1:3]
rows={}
for l in open(log,encoding="utf-8",errors="replace"):
    m=re.match(r'^([a-z0-9-]+): the descriptor declares \{([^}]*)\}, production dispatch reaches \{([^}]*)\}',l)
    if m: rows[m.group(1)]=(set(x.strip().strip('"') for x in m.group(2).split(",")),set(x.strip().strip('"') for x in m.group(3).split(",")))
src={}
for n in os.listdir(mutations):
    p=os.path.join(mutations,n,"🔣️.json")
    if not os.path.isfile(p): continue
    d=json.load(open(p,encoding="utf-8"));dd=os.path.join(mutations,n)
    f=os.path.join(dd,"🔺️diff","🦀️.rs");f=f if os.path.isfile(f) else os.path.join(dd,"🦀️.rs")
    t=open(f,encoding="utf-8").read()
    src[d["semanticKind"]]=({"applied"}|({"no-op"} if '"mutation.no-op"' in t else set())|({"rejected"} if "MutationOutcome::error(" in t or "MutationOutcome::fatal(" in t else set()), os.path.basename(dd))
c=collections.Counter();need=[];drop=[]
for k,(decl,reach) in rows.items():
    s,dn=src[k]
    miss=s-reach  # source can emit, vectors do not reach
    extra=reach-s
    if extra: c["reached but source cannot emit: %s"%sorted(extra)]+=1
    if miss: need.append((k,sorted(miss),dn))
    if decl-s: drop.append((k,sorted(decl-s)))
print("breaches",len(rows),"need vectors",len(need),"declared but source cannot emit",len(drop))
print(collections.Counter(tuple(m) for _,m,_ in need));print(collections.Counter(tuple(m) for _,m in drop));print(c)
json.dump({"need":need,"drop":drop},open(sys.argv[3],"w"),ensure_ascii=False,indent=1)
