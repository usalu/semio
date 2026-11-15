import json,glob,sys
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
def zig(n,x0,x1,pitch,a,b):
    pts=[];x=x0;i=0
    while True:
        pts.append((round(x,4),a if i%2==0 else b)); i+=1; x=x0+i*pitch
        if x>x1+1e-9: break
    return "M "+" L ".join(f"{x:g},{y:g}" for x,y in pts)
D={
 "jib-chords":"M 0,-0.8 L 55,-0.8 M 0,0.8 L 55,0.8",
 "jib-diagonals":zig(0,0,54,1.8,-0.8,0.8),
 "jib-end":"M 0,-0.8 L 0,0.8 M 55,-0.8 L 55,0.8",
 "jib-rail":"M 0,-0.8 L 55,-0.8",
 "counter-chords":"M 0,-0.7 L 16.5,-0.7 M 0,0.7 L 16.5,0.7",
 "counter-diagonals":zig(0,0,16.5,1.65,-0.7,0.7),
}
done=[]
for sc in ("100","200"):
    i=t.index('"representations"'); j=t.index(f'"{sc}": {{',i)
    jend=t.index('"front": [',j)
    for sid,d in D.items():
        k=t.find(f'"id": "{sid}"',j)
        if k<0 or k>jend: continue
        p,e=obj_span(t,k)
        o=json.loads(t[p:e])
        if o["kind"]=="path": continue
        n={"kind":"path","d":d}
        for key in ("id","bone","role","z","filled","tone","region","material","penMm"):
            if key in o: n[key]=o[key]
        s=json.dumps(n,indent=1,ensure_ascii=False).split("\n")
        txt=s[0]+"\n"+"\n".join("      "+l for l in s[1:])
        t=t[:p]+txt+t[e:]
        done.append((sc,sid))
open(P,"w",encoding="utf8").write(t); json.loads(t)
print(done)
