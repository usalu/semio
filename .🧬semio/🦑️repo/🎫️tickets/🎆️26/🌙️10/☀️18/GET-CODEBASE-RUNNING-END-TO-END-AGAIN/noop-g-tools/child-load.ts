import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{buildSourceClosureIdentityV1,buildSourceClosureStreamLoaderV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import{spawn}from"node:child_process";import{resolve}from"node:path";import{readFileSync,writeFileSync}from"node:fs";import{createHash}from"node:crypto";
// usage: bun child-load.ts <entry> <variant.json|-> <runs> <args...>   variant.json = [[from,to],...] literal replacements applied to the loader text
const root=resolve(import.meta.dirname,"../../../../../../../..");const[entry,variant,runs,...args]=process.argv.slice(2);
const c=await captureNxAuthorizerSourcesV1(root,entry!,new AbortController().signal,{installed:true});
const bytes=nxAuthorizerCapturedBytesV1(c),imports=nxAuthorizerCapturedImportsV1(root,c);
const snapshot={version:1 as const,root,entry:entry!,files:c.manifest.sources.map(r=>({path:r.path,sha256:r.sha256,base64:Buffer.from(bytes.get(r.path)!).toString("base64"),imports:imports.get(r.path)!}))};
const text=JSON.stringify(snapshot),sha=buildSourceClosureIdentityV1(snapshot as never);
let loader=buildSourceClosureStreamLoaderV1;if(variant!=="-")for(const[a,b]of JSON.parse(readFileSync(variant!,"utf8"))){if(!loader.includes(a))throw Error("variant anchor missing: "+a.slice(0,60));loader=loader.replace(a,()=>b);}
const hashOf=createHash("sha256").update(loader).digest("hex");
for(let i=0;i<Number(runs);i++){
 const t=performance.now();
 await new Promise<void>(done=>{const child=spawn(process.execPath,["--eval",loader,"--",sha,...args],{stdio:["ignore","pipe","pipe","pipe"],env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",SEMIO_REPO_PROFILE:"1"}});
  let out="";child.stdout!.on("data",d=>out+=d);child.stderr!.on("data",d=>out+=d);(child.stdio[3] as NodeJS.WritableStream).end(text);
  child.on("close",code=>{const m=out.match(/boot\.loaded\t([\d.]+)/);console.log(JSON.stringify({run:i,wallMs:Math.round(performance.now()-t),code,bootLoadedMs:m?Number(m[1]):null,tail:process.env.TAIL?out.slice(0,4000):out.split("\n").filter(l=>!l.startsWith("#")&&!/^\d/.test(l)).slice(0,2).join("|").slice(0,200)}));done();});});
}
