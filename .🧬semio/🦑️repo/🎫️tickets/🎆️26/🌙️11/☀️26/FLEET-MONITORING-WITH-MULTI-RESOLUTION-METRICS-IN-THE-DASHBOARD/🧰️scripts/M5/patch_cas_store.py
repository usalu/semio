import os, sys
P = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts"
t = open(P, encoding="utf-8").read()
edits = [
 ('import {appendCasAccessV1} from "../📒️ledger/🟦️.ts";',
  'import {countEventV1} from "../../📈️counter-ledger/🟦️.ts";\nimport {appendCasAccessV1} from "../📒️ledger/🟦️.ts";'),
 ('const allocated=(info:{size:number;blocks?:number}):number=>info.size;',
  '/** 📈️ The directory of the event counters of a store (`localHit`, `localMiss`: one byte per `get`, `read` or `link` that found or missed its object, so the size of the file is the count, see `📈️counter-ledger`); one root serves every process of the machine, so the counts are the machine\'s.\n * @see ../../📈️counter-ledger/🟦️.ts */\nexport const casCounterDirectoryV1=(root:string):string=>join(root,"v1","counters");\n\nconst allocated=(info:{size:number;blocks?:number}):number=>info.size;'),
 (',touched=new Map<string,number>();\n',
  ',touched=new Map<string,number>(),counted=casCounterDirectoryV1(root),hit=():void=>countEventV1(counted,"localHit"),miss=():void=>countEventV1(counted,"localMiss");\n'),
 ('try{const info=await lstat(objectPath(hash));await touch(hash);return{path:objectPath(hash),size:allocated(info)};}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}',
  'try{const info=await lstat(objectPath(hash));await touch(hash);hit();return{path:objectPath(hash),size:allocated(info)};}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"){miss();return undefined;}throw error;}'),
 ('try{const bytes=await readFile(objectPath(hash));await touch(hash);return bytes;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}',
  'try{const bytes=await readFile(objectPath(hash));await touch(hash);hit();return bytes;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"){miss();return undefined;}throw error;}'),
 ('sha(hash);const source=objectPath(hash);if(!await exists(source))return undefined;',
  'sha(hash);const source=objectPath(hash);if(!await exists(source)){miss();return undefined;}'),
 ('await touch(hash);return attempt;', 'await touch(hash);hit();return attempt;'),
]
for o, n in edits:
    c = t.count(o)
    if c != 1:
        print("ANCHOR", c, o[:90]); sys.exit(1)
    t = t.replace(o, n)
if "--check" in sys.argv: print("anchors ok"); sys.exit(0)
tmp = P + ".m5.tmp"; open(tmp, "w", encoding="utf-8").write(t); os.replace(tmp, P); print("ok")
