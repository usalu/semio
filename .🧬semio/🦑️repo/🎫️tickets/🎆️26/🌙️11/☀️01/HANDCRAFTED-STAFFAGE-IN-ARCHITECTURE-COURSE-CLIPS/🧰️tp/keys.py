import json,sys
sys.stdout.reconfigure(encoding="utf-8")
d=json.load(open(sys.argv[1],encoding="utf8"))
def walk(o,p,depth):
    if depth>int(sys.argv[2]): return
    if isinstance(o,dict):
        for k,v in o.items():
            s=json.dumps(v,ensure_ascii=False)
            print(p+"/"+k, type(v).__name__, len(s))
            walk(v,p+"/"+k,depth+1)
    elif isinstance(o,list) and o and isinstance(o[0],(dict,list)):
        print(p,"list",len(o))
        walk(o[0],p+"[0]",depth+1)
walk(d,"",0)
