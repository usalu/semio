import json,sys,os
produced,fixtures,schema_path=sys.argv[1:4]
root=json.load(open(schema_path,encoding="utf-8"))
defs=root.get("$defs") or root.get("definitions") or {}
def res(s):
    while isinstance(s,dict) and "$ref" in s and s["$ref"].startswith("#"):
        s=defs.get(s["$ref"].split("/")[-1])
    return s
def order(v,s):
    s=res(s)
    if s is None: return v
    for k in ("oneOf","anyOf"):
        if k in s and not ("properties" in s):
            for b in s[k]:
                b=res(b)
                if not isinstance(b,dict): continue
                if isinstance(v,dict) and b.get("type")=="object" or isinstance(v,list) and b.get("type")=="array":
                    return order(v,b)
            return v
    if isinstance(v,dict) and "properties" in s:
        p=s["properties"];keys=[k for k in p if k in v]+[k for k in v if k not in p]
        return {k:order(v[k],p.get(k)) for k in keys}
    if isinstance(v,list) and "items" in s: return [order(x,s["items"]) for x in v]
    return v
n=0
for name in os.listdir(produced):
    leaf,case=name[:-5].split("__");p=os.path.join(fixtures,leaf,case,"🔣️.diff.json")
    d=json.load(open(p,encoding="utf-8"))
    d=order(d,root)
    open(p,"w",encoding="utf-8",newline="\n").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n");n+=1
print(n)
