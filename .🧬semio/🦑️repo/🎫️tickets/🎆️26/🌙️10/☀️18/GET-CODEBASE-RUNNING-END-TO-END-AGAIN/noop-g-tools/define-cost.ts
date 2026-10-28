import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{resolve}from"node:path";
const root=resolve(import.meta.dirname,"../../../../../../../..");const entry=process.argv[2]!;
const c=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});
const b=nxAuthorizerCapturedBytesV1(c),im=nxAuthorizerCapturedImportsV1(root,c);
const files=c.manifest.sources.map(r=>({path:r.path,sha256:r.sha256,imports:im.get(r.path)}));
const phys=JSON.stringify({version:1,root,sources:files.map(r=>({path:r.path,sha256:r.sha256}))}),res=JSON.stringify({version:1,root,files});
console.log("physBytes",phys.length,"resBytes",res.length);
const src=Buffer.from(b.get(entry)!).toString("utf8");
for(const n of [50]){
 let t=performance.now();for(let i=0;i<n;i++){const comp=new Bun.Transpiler({loader:"ts",target:"bun",define:{'import.meta.semioCapturedPhysicalSources':phys,'import.meta.semioCapturedSourceResolutions':res,'import.meta.url':'"file:///x"'}});comp.transformSync(src);}
 console.log("withDefine per module ms",(performance.now()-t)/n);
 t=performance.now();for(let i=0;i<n;i++){const comp=new Bun.Transpiler({loader:"ts",target:"bun",define:{'import.meta.url':'"file:///x"'}});comp.transformSync(src);}
 console.log("smallDefine per module ms",(performance.now()-t)/n);
 t=performance.now();const comp=new Bun.Transpiler({loader:"ts",target:"bun"});for(let i=0;i<n;i++){comp.transformSync(src);}
 console.log("sharedTranspiler per module ms",(performance.now()-t)/n, "srcBytes",src.length);
}
