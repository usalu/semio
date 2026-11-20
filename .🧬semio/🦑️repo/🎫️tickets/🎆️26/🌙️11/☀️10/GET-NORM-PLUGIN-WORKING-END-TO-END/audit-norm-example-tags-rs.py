import re,sys,glob,os
N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm"
exec(open("audit-norm-example-tags.py").read().split("camel=lambda")[0].split("bad=0")[0].split("def walk")[0].replace("import json,re,sys,glob,os",""))
def kebab(n):
    c=list(n);o=""
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
def rsfiles(root):
    for dp,dn,fn in os.walk(root):
        dn[:]=[d for d in dn if "🧪" not in d and d not in("target","node_modules")]
        for f in fn:
            if f=="🦀️.rs": yield os.path.join(dp,f)
def parse(files):
    enums={};fields={}
    for f in files:
        t=open(f).read()
        for m in re.finditer(r"pub enum (\w+)\s*\{(.*?)\n\}",t,re.S):
            tags=set();pend=None
            for l in m.group(2).split("\n"):
                l=l.strip()
                k=re.match(r'#\[dsl\(key = "([^"]+)"',l)
                if k: pend=k.group(1);continue
                v=re.match(r"(\w+)\s*[,({]|(\w+)\s*$",l)
                if v and not l.startswith("#") and not l.startswith("//"):
                    tags.add(pend or kebab(v.group(1)or v.group(2)));pend=None
            enums.setdefault(m.group(1),set()).update(tags)
        for m in re.finditer(r"pub struct (\w+)\s*\{(.*?)\n\}",t,re.S):
            for fm in re.finditer(r"pub (\w+): ([^,\n]+),",m.group(2)):
                ty=re.sub(r"(Vec|Option|Box)<","",fm.group(2)).replace(">","").split("::")[-1].strip()
                fields.setdefault(fm.group(1).replace("_","-"),set()).add(ty)
    return enums,fields
shared=list(rsfiles(N+"/⚖️compliance"))+[f for f in rsfiles(N+"/📦️packages")]
bad=0
for fam in sorted(os.listdir(N+"/🗿️artifacts")):
    root=f"{N}/🗿️artifacts/{fam}"
    enums,fields=parse(list(rsfiles(root))+shared)
    for f in glob.glob(root+"/**/🗣️.dsl.semio",recursive=True):
        rel=f.split("🖼️assets/")[-1];stack=[]
        for i,l in enumerate(open(f).read().split("\n"),1):
            if i==2 and not l.startswith(" "):
                for k,v in re.findall(r"([a-z0-9-]+)=(\S+)",l):
                    if k not in fields: print(fam,rel,i,"unknown top field",k);bad+=1
                    ts=[t for t in fields.get(k,()) if t in enums]
                    if ts and v.strip('"') not in set().union(*[enums[t] for t in ts]): print(fam,rel,i,"top",k,v,sorted(set().union(*[enums[t] for t in ts])));bad+=1
                continue
            m=re.match(r"^(\s*)([a-z0-9-]+) \[(.*)\] \{",l)
            if m:
                for c in m.group(3).split():
                    cn=c.rsplit(":",1)[0]
                    if cn not in fields: print(fam,rel,i,"unknown column",m.group(2),cn);bad+=1
                stack.append((len(m.group(1)),[c.rsplit(":",1) for c in m.group(3).split()]));continue
            if l.strip()=="}" and stack: stack.pop();continue
            if stack and len(l)-len(l.lstrip())==stack[-1][0]+2 and "{" not in l:
                toks=re.findall(r'"(?:[^"\\]|\\.)*"|\S+',l);cs=stack[-1][1]
                if len(toks)>=len(cs):
                    for (c,t),v in zip(cs,toks):
                        if t=="ENUM":
                            ts=[x for x in fields.get(c,()) if x in enums]
                            ok=set().union(*[enums[x] for x in ts]) if ts else set()
                            if v not in ok: print(fam,rel,i,"col",c,v,sorted(ok)[:12]);bad+=1
print("issues",bad)
