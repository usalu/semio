import sys,json
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
key=sys.argv[1]; val=int(sys.argv[2])
t=open(R,encoding="utf8").read()
d=json.loads(t)
assert key not in d["entries"]
lines=t.split("\n")
ents=[(i,json.loads("{"+l.strip().rstrip(",")+"}")) for i,l in enumerate(lines) if l.startswith('    "')]
pos=None
for i,e in ents:
    k=list(e)[0]
    if k.encode()>key.encode(): pos=i; break
assert pos is not None
lines.insert(pos,'    '+json.dumps(key,ensure_ascii=False)+': '+str(val)+',')
out="\n".join(lines)
json.loads(out)
open(R,"w",encoding="utf8").write(out)
print("pinned",key,val)
