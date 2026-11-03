import json,sys
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/"
d=json.load(open(B+sys.argv[1]+"/🔣️.json",encoding="utf-8"))["asset"]
print([ (p["id"],p.get("key")) for p in d["poses"]])
for c in d["clips"]:
    print("CLIP",c["id"],c["durationS"],c["fps"],"loop",c["loop"],c["title"]["en"])
    print("  desc",c["description"]["en"])
    print("  keys",json.dumps(c["keys"],ensure_ascii=False)[:1400])
    print("  tracks",json.dumps(c.get("tracks"),ensure_ascii=False)[:1200])
    print("  events",[ (e["t"],e["kind"]) for e in c.get("events",[])], "phases",[(x["id"],x["fromS"],x["toS"]) for x in c.get("phases",[])])
