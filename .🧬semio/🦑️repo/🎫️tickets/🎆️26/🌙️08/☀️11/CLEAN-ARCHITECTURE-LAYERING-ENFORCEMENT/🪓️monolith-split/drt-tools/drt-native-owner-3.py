import re
p="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts"
t=open(p,encoding="utf8").read()
old='expect(close.includes("maximum_copy_bytes: copy")).toBe(true);'
assert t.count(old)==1
t=t.replace(old,'expect(close.includes("maximum_copy_bytes: demand.copy_bytes")).toBe(true);')
parts=re.split(r'(?m)^(?=test\()',t)
out=[]
removed=0
for part in parts:
    m=re.search(r'(?m)^  const host ?= ?ownerSource\(owner\);\n',part)
    if m:
        rest=part[:m.start()]+part[m.end():]
        if not re.search(r'\bhost\b',rest):
            part=rest;removed+=1
    out.append(part)
print(removed)
open(p,"w",encoding="utf8").write("".join(out))
