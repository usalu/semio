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
LINES={"jib-chord-l":"M 0,0.8 L 55,0.8","jib-chord-r":"M 0,-0.8 L 55,-0.8","jib-rail-l":"M 0,0.45 L 55,0.45","jib-rail-r":"M 0,-0.45 L 55,-0.45","counter-chord-l":"M 0,0.7 L 16.5,0.7","counter-chord-r":"M 0,-0.7 L 16.5,-0.7"}
DROP=("jib-diagonals","counter-diagonals")
done=[]
for sc in ("100","200"):
    i=t.index('"representations"'); j=t.index(f'"{sc}": {{',i)
    a=t.index('"plan": [',j)
    b=t.index('"symbols"',a) if False else a+400000
    # end of plan list: find matching ]
    depth=0;q=t.index("[",a)
    e=q
    while True:
        if t[e]=="[": depth+=1
        elif t[e]=="]":
            depth-=1
            if depth==0: break
        e+=1
    seg_start,seg_end=q,e
    for sid in list(LINES)+list(DROP):
        k=t.find(f'"id": "{sid}"',seg_start)
        if k<0 or k>seg_end: continue
        p,e2=obj_span(t,k); o=json.loads(t[p:e2])
        if sid in LINES:
            n={"kind":"path","d":LINES[sid]}
            for key in ("id","bone","role","z","filled","tone","region","material","penMm"):
                if key in o: n[key]=o[key]
        else:
            if "solid" not in o: continue
            n=dict(o); del n["solid"]
        s=json.dumps(n,indent=1,ensure_ascii=False).split("\n")
        new=s[0]+"\n"+"\n".join("      "+l for l in s[1:])
        t=t[:p]+new+t[e2:]
        seg_end+=len(new)-(e2-p)
        done.append((sc,sid))
open(P,"w",encoding="utf8").write(t); json.loads(t); print(done)
