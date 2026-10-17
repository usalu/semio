import re,os
A="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any"
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def rep(t,old,new,count=1):
    assert t.count(old)==count,(t.count(old),old[:80])
    return t.replace(old,new)
fx=A+"/🧪️tests/🧫️fixtures/🟦️.ts"
t=read(fx)
t=rep(t,'import type { ValidateFunction } from "ajv";\n','import { readFileSync } from "node:fs";\nimport type { ValidateFunction } from "ajv";\nimport { unicodeSourceUrls } from "@semio-tech/framework/2d/text/font/shape/unicode";\n')
assert t.endswith("\n")
t+='\n/** 🌐️ The original Unicode record of the framework, joined from its two data parts in byte order exactly as the Rust crates join them. */\nexport function unicodeSourceBytes(): Uint8Array {\n  return new Uint8Array(Buffer.concat(unicodeSourceUrls.map(url => readFileSync(url))));\n}\n'
write(fx,t)
S=A+"/🧬️schema/🎬️scene"
imp='import {unicodeSourceUrl} from "@semio-tech/framework/2d/text/font/shape/unicode";'
q=S+"/🎨️paint/📋️prepare/🎯️query/🧪️tests/🔬️unit/🟦️.ts"
t=read(q)
t=rep(t,imp+"\n","")
t=rep(t,'import {admitted,drawingArtifactFixture} from','import {admitted,drawingArtifactFixture,unicodeSourceBytes} from')
t=rep(t,"new Uint8Array(readFileSync(unicodeSourceUrl))","unicodeSourceBytes()")
write(q,t)
for f,rel in [(S+"/🎨️paint/📋️prepare/🧪️tests/🔬️unit/🟦️.ts","../../../../../../🧪️tests/🧫️fixtures/🟦️.ts"),(S+"/📋️prepare/🧪️tests/🔬️unit/🟦️.ts","../../../../../🧪️tests/🧫️fixtures/🟦️.ts")]:
    t=read(f)
    t=rep(t,imp,'import {unicodeSourceBytes} from "%s";'%rel)
    t=rep(t,"new Uint8Array(readFileSync(unicodeSourceUrl))","unicodeSourceBytes()")
    write(f,t)
f=S+"/🔤️text/🧪️tests/🟦️.ts"
t=read(f)
t=rep(t,imp,'import {unicodeSourceBytes} from "../../../../🧪️tests/🧫️fixtures/🟦️.ts";')
t=rep(t,"new Uint8Array(await Bun.file(unicodeSourceUrl).arrayBuffer())","unicodeSourceBytes()")
write(f,t)
print("draw ok")
