import {pathToFileURL} from "node:url";
import {readFileSync} from "node:fs";
import {resolve,relative} from "node:path";
const root=process.cwd(),lib=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const mod=await import(pathToFileURL(lib+"🟨️.mjs").href);
const s=await import(pathToFileURL(lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts").href);
const files=mod.nxBuildEngineClosureV1().files,set=new Set(files),ports=s.buildSchedulingPortsV1(),v=s.buildSchedulingV1();
const viol=new Map(),short=p=>p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","LIB/").replace("🧰️framework/🔨️modules/","FM/");
let keyedToSched=0;
for(const f of files){ if(f.endsWith(".json")||s.buildSchedulingDomainOfV1(f,v))continue; const abs=resolve(root,f);let refs=[];try{refs=mod.nxBuildEngineReferencesV1(readFileSync(abs,"utf8"));}catch{continue}
 for(const r of refs){let t;try{ if(r.kind==="file-url"||(r.kind==="module"&&r.specifier.startsWith(".")))t=relative(root,mod.nxBuildEngineSourceFileV1(abs,r.specifier)); else if(r.kind==="module"){const w=mod.nxBuildEngineWorkspacePackageFileV1(root,r.specifier); if(w)t=relative(root,w);} }catch{}
  if(t&&set.has(t)&&s.buildSchedulingDomainOfV1(t,v)){keyedToSched++;if(!ports.has(t)){const k=short(t);(viol.get(k)??viol.set(k,[]).get(k)).push(short(f));}}}}
console.log("keyed->scheduling edges",keyedToSched,"non-port targets",viol.size);
for(const [t,fs] of [...viol].slice(0,Number(process.argv[2]??25)))console.log(t,"<-",fs.length,fs[0]);
