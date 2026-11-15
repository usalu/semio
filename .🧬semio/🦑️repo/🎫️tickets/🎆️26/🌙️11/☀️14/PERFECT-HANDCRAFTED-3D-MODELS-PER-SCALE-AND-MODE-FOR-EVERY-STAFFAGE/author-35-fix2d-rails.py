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
i=t.index('"representations"'); j=t.index('"200": {',i); pl=t.index('"plan": [',j)
if '"id": "jib-rail-l"' in t[pl:pl+60000] and t.index('"id": "jib-rail-l"',pl)<t.index('"symbols"',pl) if False else False: pass
end=pl+80000
seg=t[pl:end]
if '"jib-rail-l"' not in seg.split('"symbols"')[0][:60000]:
    k=t.index('"id": "jib-chord-r"',pl)
    p,e=obj_span(t,k); o=json.loads(t[p:e])
    new=[]
    for side,sg in (("l",1),("r",-1)):
        n=dict(o); n["id"]=f"jib-rail-{side}"; n["d"]=f"M 0,{0.45*sg:g} L 55,{0.45*sg:g}"; n["z"]=3.1; n["penMm"]=0.5
        new.append(n)
    txt=""
    for n in new:
        s=json.dumps(n,indent=1,ensure_ascii=False).split("\n")
        txt+=",\n      "+s[0]+"\n"+"\n".join("      "+l for l in s[1:])
    t=t[:e]+txt+t[e:]
    open(P,"w",encoding="utf8").write(t); json.loads(t); print("rails added")
