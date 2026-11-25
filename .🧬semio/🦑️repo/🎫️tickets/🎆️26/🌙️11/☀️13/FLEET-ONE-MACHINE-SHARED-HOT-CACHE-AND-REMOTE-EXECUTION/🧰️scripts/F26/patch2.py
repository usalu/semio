import hashlib
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts"
t=open(p,encoding="utf8").read();b=hashlib.sha256(t.encode()).hexdigest()[:12]
old='try{role(rel);}catch{proof.refusals.push(rel);continue;}}proof.rows.set'
new='if(!rel.startsWith(P.retirementCatalog+"/"))try{role(rel);}catch{proof.refusals.push(rel);continue;}}proof.rows.set'
assert t.count(old)==1
t=t.replace(old,new);open(p,"w",encoding="utf8").write(t)
print(b,"->",hashlib.sha256(t.encode()).hexdigest()[:12],len(t.encode()))
