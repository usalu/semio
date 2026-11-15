import json,glob
P=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json")[0]
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
i=t.index('"representations"'); j=t.index('"100": {',i); f=t.index('"front": [',j); end=t.index('"plan": [',f)
k=t.index('"id": "mast-diagonals"',f); assert k<end
p,e=obj_span(t,k); o=json.loads(t[p:e])
if len(o["points"])!=20:
    o["points"]=[[-1 if n%2==0 else 1, 1+2*n] for n in range(20)]
    s=json.dumps(o,indent=1,ensure_ascii=False).split("\n")
    t=t[:p]+s[0]+"\n"+"\n".join("      "+l for l in s[1:])+t[e:]
    open(P,"w",encoding="utf8").write(t); json.loads(t); print("patched")
