import os, sys
P = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/📊️status/🟦️.ts"
t = open(P, encoding="utf-8").read()
edits = [
 ('import{mkdirSync,readFileSync,renameSync,writeFileSync}from"node:fs";', 'import{mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";'),
 ('import{OWNER_POOL_POLICY_V1 as P}from"../🟦️.ts";', 'import{OWNER_POOL_POLICY_V1 as P}from"../🟦️.ts";\nimport{ORIGIN_FILES_POLICY_V1 as R,retireOriginFilesV1}from"../🧹️origin-files/🟦️.ts";'),
 ('written=0,timer:ReturnType<typeof setTimeout>|undefined;\n const write=()=>{\n  timer=undefined;written=now();\n  if(input.store===undefined||input.store==="")return;\n  try{const directory=join(input.store,"v1","origins",input.origin),file=join(directory,P.statusName),temporary=file+"."+process.pid+".tmp";mkdirSync(directory,{recursive:true});writeFileSync(temporary,JSON.stringify({...doc,updatedMs:now()}));renameSync(temporary,file);}catch{}\n };',
  'written=0,swept=-Infinity,timer:ReturnType<typeof setTimeout>|undefined;\n const retire=(directory:string):void=>{const at=now();if(at-swept<R.sweepIntervalMilliseconds)return;swept=at;void retireOriginFilesV1({directory,now:Date.now()}).catch(()=>undefined);};\n const write=()=>{\n  timer=undefined;written=now();\n  if(input.store===undefined||input.store==="")return;\n  let temporary:string|undefined;\n  try{const directory=join(input.store,"v1","origins",input.origin),file=join(directory,P.statusName);temporary=file+"."+process.pid+".tmp";mkdirSync(directory,{recursive:true});retire(directory);writeFileSync(temporary,JSON.stringify({...doc,updatedMs:now()}));renameSync(temporary,file);}catch{if(temporary!==undefined)try{rmSync(temporary,{force:true});}catch{}}\n };'),
]
for o, n in edits:
    c = t.count(o)
    if c != 1:
        print("ANCHOR", c, o[:80]); sys.exit(1)
    t = t.replace(o, n)
tmp = P + ".m5.tmp"; open(tmp, "w", encoding="utf-8").write(t); os.replace(tmp, P); print("ok")
