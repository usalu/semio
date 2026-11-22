F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts"
t=open(F,encoding="utf8").read()
def rep(a,b):
    global t
    assert t.count(a)==1,(a[:70],t.count(a));t=t.replace(a,b)
if "tmpdir" not in t:
    rep('import {dirname,join} from "node:path";','import {tmpdir} from "node:os";\nimport {dirname,join} from "node:path";')
    rep('join(context.storeRoot??context.root??".",`.fleet-fetch-${randomBytes(6).toString("hex")}.tmp`)','join(context.storeRoot??context.root??tmpdir(),`.fleet-fetch-${randomBytes(6).toString("hex")}.tmp`)')
    rep(':machinePathV1(context.root??".",".cache","f","fetch-tmp",`${randomBytes(6).toString("hex")}.tmp`),hash=',':context.root!==undefined?machinePathV1(context.root,".cache","f","fetch-tmp",`${randomBytes(6).toString("hex")}.tmp`):join(tmpdir(),`.fleet-fetch-${randomBytes(6).toString("hex")}.tmp`),hash=')
    open(F,"w",encoding="utf8").write(t)
print("fleet-fetch staging applied")
