import os,re,sys,collections
root=sys.argv[1]
exports={}
def ex(p):
    if p not in exports:
        try: t=open(p,encoding='utf8').read()
        except Exception: exports[p]=None; return None
        names=set(re.findall(r"^export (?:declare )?(?:interface|type|const|class|function|enum|abstract class) (\w+)",t,re.M))
        for m in re.finditer(r"^export \{([^}]*)\}",t,re.M):
            for n in m.group(1).split(","):
                n=n.strip()
                if n: names.add(n.split(" as ")[-1].strip().replace("type ",""))
        star=re.findall(r'^export \* from "([^"]+)"',t,re.M)
        exports[p]=(names,star,"Generated from `./🔣️.json`" in t)
    return exports[p]
bad=[];n=0
for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d not in("target","node_modules","🗑️generated")]
    for f in fn:
        if not f.endswith((".ts",".tsx")): continue
        p=os.path.join(dp,f)
        t=open(p,encoding='utf8').read()
        for m in re.finditer(r'import\s+(?:type\s+)?\{([^}]*)\}\s*from\s*"(\.[^"]*)"',t):
            tgt=os.path.normpath(os.path.join(dp,m.group(2)))
            e=ex(tgt)
            if e is None or not e[2]: continue
            for nm in m.group(1).split(","):
                nm=nm.strip()
                if not nm: continue
                nm=re.sub(r"^type\s+","",nm).split(" as ")[0].strip()
                n+=1
                if nm not in e[0]: bad.append((os.path.relpath(p,root),nm,os.path.relpath(tgt,root)))
print("imports checked",n,"missing",len(bad))
for b in bad: print("\t".join(b))
