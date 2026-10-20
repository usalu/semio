#!/usr/bin/env python3
# Builds a COPY overlay at /private/tmp/parity/ov with the staged parity module at its destination, copies of the few repo files it imports, and stubs for the process-control modules; then run the tests there (never symlinks).
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
OV = "/private/tmp/parity/ov"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
layout = json.load(open(HERE + "/layout.json", encoding="utf-8"))
if os.path.exists(OV): shutil.rmtree(OV)
def put(rel, data, binary=False):
    p = os.path.join(OV, rel); os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "wb" if binary else "w", **({} if binary else {"encoding": "utf-8"})).write(data)
for flat, dest in layout["new"].items():
    put(dest, open(HERE + "/staged/" + flat, "rb").read(), True)
def copy(rel):
    src = os.path.join(ROOT, rel); dst = os.path.join(OV, rel); os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.isdir(src): shutil.copytree(src, dst, ignore=shutil.ignore_patterns("node_modules", "🧪️tests"))
    else: shutil.copy2(src, dst)
copy("🧰️framework/🔨️modules/🧬️schema/✅️validator")
copy(L + "/🏗️build/⚙️operations/📖️help/🟦️.ts")
copy(L + "/🏗️build/⚙️operations/🔣️catalog.json")
copy(L + "/🏃️process/🌿️environment/🧪️test-output/🟦️.ts")
P = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"
put(P + "/🧭️routing/🟦️.ts", '''export type ScriptInvocation={policy:unknown;capabilities:unknown;control:{signal:AbortSignal}};
export function createScriptProcessEnvelope(policy:unknown,capabilities:unknown,_now:number):ScriptInvocation{return {policy,capabilities,control:{signal:new AbortController().signal}};}
export async function withScriptProcessEnvelope<T>(envelope:ScriptInvocation,body:(original:ScriptInvocation)=>Promise<T>):Promise<T>{return body(envelope);}
export function checkScriptInvocation(invocation:ScriptInvocation):void{if(invocation.control.signal.aborted)throw invocation.control.signal.reason??Error("cancelled");}
export async function advanceScriptInvocation(_invocation:ScriptInvocation,_label:string,_stage:string):Promise<void>{}
''')
put(P + "/🎛️execution/🟦️.ts", '''import {spawn} from "node:child_process";
import {writeFileSync} from "node:fs";
export async function captureOwnedProcess(command:string,args:string[],options:{cwd:string;env:Record<string,string|undefined>;budgetMs:number;maxOutputBytes:number;stdoutPath:string;stderrPath:string}):Promise<{status:number|null;signal:string|null;stdout:string;stderr:string;reason?:"exit"|"timeout"|"cancelled"|"output-limit"|"spawn-error"}>{
 return new Promise(resolve=>{
  const child=spawn(command,args,{cwd:options.cwd,env:options.env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let out="",err="";
  child.stdout.on("data",chunk=>{out+=chunk;});child.stderr.on("data",chunk=>{err+=chunk;});
  child.on("close",(status,signal)=>{writeFileSync(options.stdoutPath,out);writeFileSync(options.stderrPath,err);resolve({status,signal,stdout:out,stderr:err,reason:"exit"});});
 });
}
''')
put("package.json", '{"name":"parity-overlay","private":true}\n')
nm = os.path.join(ROOT, "node_modules/.bun")
for name, store in [("ajv", "ajv@8.20.0"), ("fast-deep-equal", "fast-deep-equal@3.1.3"), ("json-schema-traverse", "json-schema-traverse@1.0.0"), ("require-from-string", "require-from-string@2.0.2"), ("fast-uri", "fast-uri@3.1.8")]:
    shutil.copytree(os.path.join(nm, store, "node_modules", name), os.path.join(OV, "node_modules", name))
print("overlay ready:", OV)
