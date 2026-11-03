import json
p="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/herring-gull/🔣️.json"
d=json.load(open(p,encoding="utf-8")); a=d["asset"]
ys=[-0.102,-0.245,-0.245,-0.102]
for sc in ("100","200"):
    for s in a["representations"][sc]["geometry"]["front"]:
        if s["bone"] in ("shank-n","shank-f") and s["kind"]=="poly":
            s["points"]=[[x,y] for (x,_),y in zip(s["points"],ys)]
open(p,"w",encoding="utf-8",newline="\n").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
