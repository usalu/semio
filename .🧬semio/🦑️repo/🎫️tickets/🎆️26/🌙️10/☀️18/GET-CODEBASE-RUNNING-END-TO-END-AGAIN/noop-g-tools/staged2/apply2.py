#!/usr/bin/env python3
"""Second hot-path batch: `apply2.py [--write]` (dry run by default)."""
import os, re, subprocess, sys
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/noop-g-tools/staged2/"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
write = "--write" in sys.argv
edits, created = {}, {}
def read(p): return open(p, encoding="utf8").read()
def replace_once(path, pairs):
    text = edits.get(path) or read(path)
    for old, new in pairs:
        if text.count(old) != 1: raise SystemExit("anchor count %d in %s: %s" % (text.count(old), path, old[:100]))
        text = text.replace(old, new)
    edits[path] = text

replace_once(B + "🧭️invocation/🕸️graph/🟦️.ts", [(
 '''/** 📝️ Streams a graph slice to the plan file of the parent that is waiting for this child. */
export async function writeBuildGraphPlanV1(file:string,graph:BuildGraphV1,signal?:AbortSignal):Promise<void>{
 const{open,writeFile}=await import("node:fs/promises"),{writeJsonStreamV1}=await import("../../🕸️graph/🕰️generations/🌊️json-stream/🟨️.mjs"),handle=await open(file,"wx",0o600);
 try{await writeJsonStreamV1(graph,text=>handle.write(text).then(()=>undefined),{signal});await handle.sync();}finally{await handle.close();}''',
 '''/** 📝️ Streams a graph slice to the plan file of the parent that is waiting for this child, as the exact compact text of `JSON.stringify(graph)`: each top-level member is written in pieces (every project on its own, the edges 4,096 at a time) so that no step holds the graph as one string, and no step awaits per value as the pretty printer does (3.3 s against 0.6 s for the 550 projects of a covered generation). The file is read back by the waiting parent only, so it is not forced to disk. */
export async function writeBuildGraphPlanV1(file:string,graph:BuildGraphV1,signal?:AbortSignal):Promise<void>{
 const{open,writeFile}=await import("node:fs/promises"),handle=await open(file,"wx",0o600),limit=1048576;let pieces:string[]=[],size=0;
 const flush=async()=>{if(!pieces.length)return;signal?.throwIfAborted();const text=pieces.join("");pieces=[];size=0;await handle.write(text);},put=async(text:string)=>{pieces.push(text);size+=text.length;if(size>=limit)await flush();};
 try{
  await put("{");let first=true;
  for(const[key,value]of Object.entries(graph as unknown as Record<string,unknown>)){
   const whole=value===null||typeof value!=="object"||"toJSON" in value;
   if(whole&&JSON.stringify(value)===undefined)continue;
   await put((first?"":",")+JSON.stringify(key)+":");first=false;
   if(whole){await put(JSON.stringify(value));continue;}
   if(Array.isArray(value)){await put("[");for(let at=0;at<value.length;at+=4096){const text=JSON.stringify(value.slice(at,at+4096));await put((at?",":"")+text.slice(1,-1));}await put("]");continue;}
   await put("{");let inner=true;for(const[name,member]of Object.entries(value)){const text=JSON.stringify(member);if(text===undefined)continue;await put((inner?"":",")+JSON.stringify(name)+":"+text);inner=false;}await put("}");
  }
  await put("}");await flush();
 }finally{await handle.close();}''')])
created[B + "🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts"] = read(C + "plan-file-test.ts")
for path, text in {**edits, **created}.items():
    docs = re.findall(r"/\*\* (\S+)", text)
    if {d for d in docs if docs.count(d) > 1}: raise SystemExit("docstring emoji not unique in " + path)
if "--check" in sys.argv:
    os.makedirs("/private/tmp/noopg/preview2", exist_ok=True)
    for i, (path, text) in enumerate({**edits, **created}.items()):
        t = "/private/tmp/noopg/preview2/%d.ts" % i; open(t, "w", encoding="utf8").write(text)
        done = subprocess.run(["bun", "build", t, "--no-bundle", "--target=bun"], capture_output=True, text=True)
        print(("SYNTAX OK  " if done.returncode == 0 else "SYNTAX BAD ") + path[-50:])
        if done.returncode: sys.exit(1)
for path, text in {**edits, **created}.items():
    print(("WRITE " if write else "DRY   ") + path)
    if write:
        os.makedirs(os.path.dirname(path), exist_ok=True); open(path, "w", encoding="utf8").write(text)
