#!/usr/bin/env python3
"""Round 2 staging: adopts peer edits of the live sb-test.ts (toolchain resolution), re-applies the round-2 test additions on top of it, and records the sha256 of every live new-file target in prev.json so that apply --update refuses on drift. usage: build_round2.py"""
import hashlib,json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import manifest as m
REAL="/Users/ueli/Documents/semio"
live=open(REAL+"/"+m.NEW["sb-test.ts"],encoding="utf8").read()
pairs=[
 ('import {createServer} from "node:net";import {join} from "node:path";import Ajv from ${JSON.stringify(Bun.resolveSync','import {createServer} from "node:net";import {join} from "node:path";import {Database} from "bun:sqlite";import Ajv from ${JSON.stringify(Bun.resolveSync'),
 ('    const fine = await run("enforce", `test("f",async()=>{if(!new Ajv().compile({type:"object"})({}))throw Error("CommonJS dependency broke");mkdirSync(join(tmp,"d"));',
  '    const database = await run("enforce", `test("d",()=>{new Database(":memory:").close();const ok=new Database(join(tmp,"ok.sqlite"));ok.close();new Database(join(checkout,".cache","lease.sqlite")).close();});`, true);\n    expect(database.output).toContain("protected-write refused: Database");\n    expect(database.checkout).not.toContain("lease.sqlite");\n    const fine = await run("enforce", `test("f",async()=>{if(!new Ajv().compile({type:"object"})({}))throw Error("CommonJS dependency broke");new Database(":memory:").close();new Database(join(tmp,"fine.sqlite"),{readonly:false,create:true}).close();mkdirSync(join(tmp,"d"));'),
 ('    for (const name of ["spawn", "spawnSync", "execFile", "execFileSync", "exec", "execSync"]) expect(child, name).toContain(`export const ${name} =`);',
  '    for (const name of ["spawn", "spawnSync", "execFile", "execFileSync", "exec", "execSync"]) expect(child, name).toContain(`export const ${name} =`);\n    for (const name of Object.keys(declared.guard.sqliteTargets)) expect(readFileSync(join(root, "🛡️guard/🩻️sqlite/🟦️.ts"), "utf8"), name).toContain(`class ${name} extends`);'),
]
out=live
for old,new in pairs:
    if out.count(old)!=1: sys.exit("anchor drift in live sb-test.ts: "+old[:80])
    out=out.replace(old,new)
open(HERE+"/staged/sb-test.ts","w",encoding="utf8").write(out)
prev={}
for st,rel in m.NEW.items():
    p=REAL+"/"+rel
    if os.path.exists(p): prev[rel]=hashlib.sha256(open(p,"rb").read()).hexdigest()
json.dump(prev,open(HERE+"/prev.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print("round 2 staged;",len(prev),"live targets fingerprinted")
