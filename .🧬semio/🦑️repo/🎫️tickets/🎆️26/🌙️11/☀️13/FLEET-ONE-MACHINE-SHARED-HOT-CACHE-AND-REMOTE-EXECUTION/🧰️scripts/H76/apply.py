#!/usr/bin/env python3
"""H76 landing (process area, slot only): termination identity law, reaper, shared children, owned-execution wiring. Atomic writes, sha guards on whole-file replacements, anchored edits elsewhere."""
import hashlib,os,sys,tempfile
ROOT=os.environ.get("H76_ROOT","/Users/ueli/Documents/semio")
P=ROOT+"/🧰️framework/🔨️modules/🏃️process"
HERE=os.path.dirname(os.path.abspath(__file__))
STAGE=HERE+"/stage/🪓️termination"
BEFORE={"🟦️.ts":"cfacedfae796","🧬️schema/🔣️.json":"fc3430009fc4","🧪️tests/🟦️.ts":"5104fd9c7940","🧪️tests/🐍️.py":"26d0aa1c4064","🧪️tests/🧫️fixtures/🔣️.json":"4f08780b9385"}
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def write(path,text):
    os.makedirs(os.path.dirname(path),exist_ok=True)
    d=tempfile.NamedTemporaryFile("w",encoding="utf8",dir=os.path.dirname(path),delete=False,suffix=".tmp",newline="")
    d.write(text);d.close();os.replace(d.name,path)
def edit(path,pairs):
    t=open(path,encoding="utf8").read()
    for old,new in pairs:
        if t.count(old)!=1:sys.exit("anchor count %d: %s in %s"%(t.count(old),old[:90],path))
        t=t.replace(old,new)
    return t
plan=[]
for rel,want in BEFORE.items():
    real=P+"/🪓️termination/"+rel
    if sha(real)!=want:sys.exit("peer changed %s (%s != %s)"%(rel,sha(real),want))
new=[]
for dp,_,fs in os.walk(STAGE):
    for f in fs:
        rel=os.path.relpath(dp+"/"+f,STAGE)
        if rel in BEFORE:continue
        real=P+"/🪓️termination/"+rel
        if os.path.exists(real):
            if sha(real)!=sha(dp+"/"+f):sys.exit("differs: "+rel)
            continue
        new.append((real,open(dp+"/"+f,encoding="utf8").read()))
OE=P+"/🎛️owned-execution/🟦️.ts"
OC=P+"/🎛️owned-execution/📥️capture/🟦️.ts"
OU=P+"/📥️capture/🟦️.ts"
oe=edit(OE,[
 ('import { ownedKillRecordLineV1, terminateOwnedChildTreeGracefullyV1 } from "../🪓️termination/🟦️.ts";','import { ownedKillRecorderV1, terminateOwnedChildTreeGracefullyV1 } from "../🪓️termination/🟦️.ts";\nimport { reapWithParentV1 } from "../🪓️termination/🧹️reaper/🟦️.ts";'),
 ('{ record: (row, signal) => { const line = ownedKillRecordLineV1({ pid: row.pid, signal, killer: process.pid, label, reason: stopped }); process.stderr.write(`${line}\\n`); } }','{ record: ownedKillRecorderV1(label, () => stopped) }'),
 ('  const child = bunChild ?? nativeChild!;\n','  const child = bunChild ?? nativeChild!;\n  const unreap = reapWithParentV1(child.pid);\n'),
 ('  } finally {\n    bunChild?.auxiliary?.destroy();\n','  } finally {\n    unreap();\n    bunChild?.auxiliary?.destroy();\n'),
])
oc=edit(OC,[
 ('import {terminateOwnedChildTree} from "../../🪓️termination/🟦️.ts";','import {terminateOwnedChildTree} from "../../🪓️termination/🟦️.ts";\nimport {reapWithParentV1} from "../../🪓️termination/🧹️reaper/🟦️.ts";'),
 (' let failed=false,stopped:unknown,closed=false,channel:ChannelOwner|undefined;const stop=',' const unreap=reapWithParentV1(child.pid);\n let failed=false,stopped:unknown,closed=false,channel:ChannelOwner|undefined;const stop='),
 (' finally{if(release)release();clearTimeout(timer);',' finally{unreap();if(release)release();clearTimeout(timer);'),
])
ou=edit(OU,[
 ('import {terminateOwnedProcessTree} from "../🪓️termination/🟦️.ts";','import {terminateOwnedProcessTree} from "../🪓️termination/🟦️.ts";\nimport {reapWithParentV1} from "../🪓️termination/🧹️reaper/🟦️.ts";'),
 ('    const terminate = (cause: NonNullable<OwnedProcessCaptureResult["reason"]>): void => {\n      if (reason !== "exit") return;','    const unreap = reapWithParentV1(child.pid);\n    const terminate = (cause: NonNullable<OwnedProcessCaptureResult["reason"]>): void => {\n      if (reason !== "exit") return;'),
 ('      if (finished) return;\n      finished = true;\n      clearTimeout(timer);','      if (finished) return;\n      finished = true;\n      unreap();\n      clearTimeout(timer);'),
])
if "--check" in sys.argv:print("anchors ok, new files:",len(new));sys.exit(0)
for real,text in new:write(real,text)
for rel in BEFORE:write(P+"/🪓️termination/"+rel,open(STAGE+"/"+rel,encoding="utf8").read())
write(OE,oe);write(OC,oc);write(OU,ou)
print("applied; new files:",len(new))
