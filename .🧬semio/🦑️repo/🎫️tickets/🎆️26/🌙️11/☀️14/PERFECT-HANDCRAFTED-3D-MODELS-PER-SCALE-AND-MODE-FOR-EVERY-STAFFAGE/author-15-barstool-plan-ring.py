import json
F="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bar-stool/🔣️.json"
t=open(F,encoding="utf-8").read()
d=json.loads(t)
reps=d["asset"]["representations"]
pens={"500":0.13,"200":0.18,"100":0.25}
for k in pens:
    assert not any(s["id"]=="ring" for s in reps[k]["geometry"]["plan"]), k
keys=['"plan": [']
pos=[];i=0
while True:
    i=t.find('"plan": [',i)
    if i<0: break
    pos.append(i); i+=1
order=[]
for k in ("500","200","100"):
    order.append(k)
# map plan occurrences to scales by denominator position
dens={}
for k in ("500","200","100","50"):
    dens[k]=t.find('"denominator": %s,'%k)
def scale_of(p):
    best=None
    for k,v in dens.items():
        if v<p and (best is None or v>dens[best]): best=k
    return best
edits=[]
for p in pos:
    k=scale_of(p)
    if k in pens:
        j=t.index("[",p); depth=0; q=j
        while True:
            ch=t[q]
            if ch=="[": depth+=1
            elif ch=="]":
                depth-=1
                if depth==0: break
            q+=1
        edits.append((q,k))
for q,k in sorted(edits,reverse=True):
    el='''            {
              "id": "ring",
              "bone": "frame",
              "role": "body",
              "z": 1,
              "filled": false,
              "tone": 0.7,
              "region": "footrest-ring",
              "material": "chrome-steel",
              "kind": "ellipse",
              "cx": 0,
              "cy": 0,
              "rx": 0.195,
              "ry": 0.195,
              "penMm": %s
            }'''%pens[k]
    # find the last '}' before q
    r=t.rindex("}",0,q)
    t=t[:r+1]+",\n"+el+t[r+1:]
json.loads(t)
open(F,"w",encoding="utf-8").write(t)
print("edited",[k for _,k in edits])
