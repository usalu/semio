import json,re,sys,glob,os
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
def kebab(n):
    n=n[:1].upper()+n[1:];c=list(n);o=""
    for i,ch in enumerate(c):
        if ch in "_-":
            if o and not o.endswith("-"): o+="-"
            continue
        if ch.isupper():
            p=c[i-1] if i else None;nx=c[i+1] if i+1<len(c) else None
            b=(p is not None and (p.islower() or p.isdigit())) or (p is not None and p.isupper() and nx is not None and nx.islower())
            if b and o and not o.endswith("-"): o+="-"
            o+=ch.lower()
        else: o+=ch
    return o
camel=lambda s:re.sub(r"-(\w)",lambda m:m.group(1).upper(),s)
def walk(n,out):
    if isinstance(n,dict):
        for k,v in n.items():
            if k=="properties" and isinstance(v,dict): out.setdefault("__props",{}).update({})
            walk(v,out)
    elif isinstance(n,list):
        for v in n: walk(v,out)
def resolve(defs,n,depth=0):
    if depth>6 or not isinstance(n,dict): return set()
    s=set()
    if "$ref" in n: s|=resolve(defs,defs.get(n["$ref"].split("/")[-1],{}),depth+1)
    if "enum" in n: s|=set(kebab(str(x)) for x in n["enum"])
    for k in("items","anyOf","oneOf","allOf"):
        v=n.get(k)
        for x in (v if isinstance(v,list) else [v] if v else []): s|=resolve(defs,x,depth+1)
    return s
bad=0
for fam in sorted(os.listdir(R)):
    sp=glob.glob(f"{R}/{fam}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json")
    if not sp: print("NO SCHEMA",fam); continue
    sc=json.load(open(sp[0])); defs=sc.get("$defs",{})
    props={};allenum=set()
    def coll(n):
        if isinstance(n,dict):
            for k,v in n.items():
                if k=="properties" and isinstance(v,dict):
                    for p,pv in v.items(): props.setdefault(p,set()).update(resolve(defs,pv)); props[p]
                coll(v)
        elif isinstance(n,list):
            for v in n: coll(v)
    coll(sc)
    for f in glob.glob(f"{R}/{fam}/**/🗣️.dsl.semio",recursive=True):
        rel=f.split("🖼️assets/")[-1]
        stack=[];cols=[]
        for i,l in enumerate(open(f).read().split("\n"),1):
            if i==2 and not l.startswith(" "):
                for kv in re.findall(r"([a-z0-9-]+)=(\S+)",l):
                    k=camel(kv[0])
                    if k not in props: print(fam,rel,i,"unknown top field",kv[0]);bad+=1
                    elif props[k] and kv[1].strip('"') not in props[k]: print(fam,rel,i,"bad top enum",kv[0],kv[1],sorted(props[k]));bad+=1
                continue
            m=re.match(r"^(\s*)([a-z0-9-]+) \[(.*)\] \{",l)
            if m:
                cs=[c.rsplit(":",1) for c in m.group(3).split()]
                for c,t in cs:
                    if camel(c) not in props: print(fam,rel,i,"unknown column",m.group(2),c);bad+=1
                stack.append((len(m.group(1)),cs));continue
            if l.strip()=="}" and stack: stack.pop();continue
            if stack:
                cs=stack[-1][1]
                if len(l)-len(l.lstrip())!=stack[-1][0]+2: continue
                toks=re.findall(r'"(?:[^"\\]|\\.)*"|\S+',l)
                if len(toks)>=len(cs) and "{" not in l:
                    for (c,t),v in zip(cs,toks):
                        if t=="ENUM":
                            ok=props.get(camel(c),set())
                            if v not in ok: print(fam,rel,i,"bad enum",c,v,sorted(ok));bad+=1
print("issues",bad)
