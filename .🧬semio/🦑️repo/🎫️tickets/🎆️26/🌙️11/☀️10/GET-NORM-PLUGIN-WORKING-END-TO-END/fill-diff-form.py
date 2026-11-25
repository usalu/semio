import json,sys,os,subprocess
def load(p): return json.load(open(p,encoding="utf-8"))
def make(root):
    defs=root.get("$defs",{})
    def res(s):
        while isinstance(s,dict) and "$ref" in s and s["$ref"].startswith("#"): s=defs.get(s["$ref"].split("/")[-1])
        return s
    def empty(s):
        s=res(s)
        if isinstance(s,dict) and s.get("type")=="array": return []
        if isinstance(s,dict) and s.get("type")=="object" and "properties" in s:
            return {k:(empty(v) if k in s.get("required",[]) else None) for k,v in s["properties"].items()}
        return None
    def fill(v,s):
        s=res(s)
        if s is None: return v
        for k in ("oneOf","anyOf"):
            if k in s and "properties" not in s:
                for b in s[k]:
                    b=res(b)
                    if isinstance(b,dict) and (isinstance(v,dict) and b.get("type")=="object" or isinstance(v,list) and b.get("type")=="array"): return fill(v,b)
                return v
        if isinstance(v,dict) and "properties" in s:
            p=s["properties"];req=s.get("required",[])
            out={}
            for k in p:
                if k in v: out[k]=fill(v[k],p[k])
                else: out[k]=empty(p[k]) if k in req else None
            for k in v:
                if k not in p: out[k]=v[k]
            return out
        if isinstance(v,list) and "items" in s: return [fill(x,s["items"]) for x in v]
        return v
    return fill
if __name__=="__main__":
    mode,schema,fx=sys.argv[1:4]
    fill=make(load(schema))
    n=ch=0
    for p in sorted(os.popen('ls "%s"/*/*/🔣️.diff.json'%fx).read().split("\n")):
        if not p: continue
        d=load(p);f=fill(d,load(schema))
        n+=1
        if f!=d:
            ch+=1
            if mode=="write": open(p,"w",encoding="utf-8",newline="\n").write(json.dumps(f,indent=2,ensure_ascii=False)+"\n")
    print(n,ch)
