import json,sys
# usage: edit.py design.json 'id:key=value;key=value' ... (value JSON)
f=sys.argv[1]; d=json.load(open(f)); sh=d["geometry"]["elevation"]
for spec in sys.argv[2:]:
    i,kv=spec.split(":",1); s=next(x for x in sh if x["id"]==i)
    for p in kv.split(";"):
        k,v=p.split("=",1); s[k]=json.loads(v)
open(f,"w").write('{"geometry":{"elevation":[\n'+",\n".join(json.dumps(s,separators=(",",":")) for s in sh)+"\n]}}\n")
