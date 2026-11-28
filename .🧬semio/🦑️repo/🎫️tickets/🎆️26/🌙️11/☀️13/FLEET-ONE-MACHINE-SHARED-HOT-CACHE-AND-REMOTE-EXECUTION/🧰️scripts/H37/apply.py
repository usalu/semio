import sys,re
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch"
s=open(p,encoding="utf8").read()
old="if (!await verifySource()) return;"
new="if (!await verifySource()) { console.log(`[nx] Source changed during ${task.id}; cache promotion withheld / Quelle während ${task.id} geändert; Cache-Übernahme zurückgehalten`); return; }"
n=s.count(old)
if n!=3: sys.exit(f"expected 3 silent returns, found {n}")
open(p,"w",encoding="utf8").write(s.replace(old,new))
print("patched",n)
