import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))["asset"]
print(json.dumps(d["thermal"]["assumptions"], ensure_ascii=False))
print(json.dumps(d["thermal"]["environment"], ensure_ascii=False))
print(json.dumps(d["presentation"]), json.dumps(d["summary"], ensure_ascii=False)[:100])
print([ (p["id"], list(p["thermal"].keys())) for p in d["poses"]][:2])
print(json.dumps(d["poses"][3]["thermal"].get("operating")))
print(json.dumps(d["clips"][0]["tracks"])[:600])
for p in d["poses"]:
    print(p["id"], [r for r in p["thermal"]["regions"] if r.startswith("panel-front")], json.dumps(p["thermal"]["regions"]["panel-front"]))
m = json.load(open(sys.argv[2], encoding="utf-8"))
e = [e for e in m["entries"] if e["id"] == "panel-radiator"][0]
print(json.dumps(e, ensure_ascii=False))
