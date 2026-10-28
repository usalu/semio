import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedImportsV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{resolve,dirname}from"node:path";
const root=resolve(import.meta.dirname,"../../../../../../../..");
const c=await captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal,{installed:true});
const im=nxAuthorizerCapturedImportsV1(root,c);
console.log("resolutions",JSON.stringify(c.manifest.resolutions,null,0).slice(0,1500));
const inst=new Set<string>();for(const m of im.values())for(const t of Object.values(m))if(t.startsWith("installed:"))inst.add(t);
console.log("installed",[...inst]);
const dirs=new Set<string>();for(const s of c.manifest.sources){let d=dirname(s.path);while(d!=="."&&!dirs.has(d)){dirs.add(d);d=dirname(d);}}
console.log("files",c.manifest.sources.length,"dirs",dirs.size);
const roots=new Set<string>();for(const s of c.manifest.sources)if(!s.path.startsWith("🧰️framework"))roots.add(s.path);console.log("outside framework",[...roots].slice(0,20));
