import json,sys
order=json.load(open(sys.argv[1],encoding="utf-8"))
def re(v,o):
    if isinstance(v,dict) and isinstance(o,dict):
        keys=[k for k in o if k in v]+[k for k in v if k not in o]
        return {k:re(v[k],o.get(k)) for k in keys}
    if isinstance(v,list) and isinstance(o,list):
        return [re(x,o[i] if i<len(o) else None) for i,x in enumerate(v)]
    return v
for p,o in order.items():
    v=json.load(open(p,encoding="utf-8"))
    open(p,"w",encoding="utf-8",newline="\n").write(json.dumps(re(v,o),indent=2,ensure_ascii=False)+"\n")
print(len(order))
