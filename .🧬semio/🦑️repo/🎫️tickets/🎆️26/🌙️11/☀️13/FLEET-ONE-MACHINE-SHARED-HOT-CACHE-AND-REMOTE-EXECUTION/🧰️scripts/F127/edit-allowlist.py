import json,os
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/📒️allowlist/🔣️.json"
s=open(P,encoding="utf8").read();d=json.loads(s)
before=len(d["entries"])
d["entries"]=[e for e in d["entries"] if e["class"]!="unrouted"]
print(before,"->",len(d["entries"]))
tmp=P+".f127tmp";open(tmp,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=1)+"\n");os.replace(tmp,P)
