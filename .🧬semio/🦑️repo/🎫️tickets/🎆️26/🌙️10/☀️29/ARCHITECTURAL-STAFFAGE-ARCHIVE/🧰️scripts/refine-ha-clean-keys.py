import json,sys
G="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/🔣️.json"
B="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/refine-ha/humans-before-hands.json"
d=json.load(open(G)); b=json.load(open(B))
T={("seated-reader","stand-up"),("worker-carrying-box","squat-lift"),("jogger","stretch")}
for x,y in zip(b["assets"],d["assets"]):
    for c,e in zip(x["clips"],y["clips"]):
        e["keys"]=c["keys"] if (x["id"],c["id"]) in T or True else e["keys"]
open(G,"w").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
