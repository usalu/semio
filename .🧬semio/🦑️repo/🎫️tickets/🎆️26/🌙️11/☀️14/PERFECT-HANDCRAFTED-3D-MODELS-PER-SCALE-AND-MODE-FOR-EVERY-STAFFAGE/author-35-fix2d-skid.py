import json,glob
P=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/oil-pumpjack/🔣️.json")[0]
t=open(P,encoding="utf8").read()
def obj_span(t,k):
    depth=0;p=k
    while True:
        p-=1
        if t[p]=="}": depth+=1
        elif t[p]=="{":
            if depth==0: break
            depth-=1
    e=k;depth=0
    while True:
        if t[e]=="{": depth+=1
        elif t[e]=="}":
            if depth==0: break
            depth-=1
        e+=1
    return p,e+1
n=0
for sc in ("50","100","200","500"):
    i=t.index('"representations"'); j=t.index(f'"{sc}": {{',i); f=t.index('"front": [',j); end=t.index('"plan": [',f)
    k=t.find('"id": "skid"',f)
    if k<0 or k>end: continue
    p,e=obj_span(t,k); o=json.loads(t[p:e])
    if o.get("w")==2.2:
        o["x"]=-1.0; o["w"]=2.0
        s=json.dumps(o,indent=1,ensure_ascii=False).split("\n")
        t=t[:p]+s[0]+"\n"+"\n".join("      "+l for l in s[1:])+t[e:]; n+=1
open(P,"w",encoding="utf8").write(t); json.loads(t); print("patched",n)
