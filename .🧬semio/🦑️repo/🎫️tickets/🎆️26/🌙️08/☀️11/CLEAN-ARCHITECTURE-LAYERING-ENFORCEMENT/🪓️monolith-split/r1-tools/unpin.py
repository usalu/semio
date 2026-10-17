import sys,json
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
key=sys.argv[1]
t=open(R,encoding="utf8").read()
lines=t.split("\n")
hit=[i for i,l in enumerate(lines) if l.strip().startswith(json.dumps(key,ensure_ascii=False)+":")]
assert len(hit)==1,hit
i=hit[0]
last=lines[i].rstrip().endswith(",")==False
assert not last, "last entry; handle trailing comma by hand"
del lines[i]
out="\n".join(lines)
json.loads(out)
open(R,"w",encoding="utf8").write(out)
print("removed",key)
