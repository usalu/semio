/** 🪪️ [sealprep] Prints the implementation digest a live-mode capture of the sealed entry closure would carry right now (the same capture rows, the same identity function the loader checks). usage: bun identity.ts <root> */
import{join,resolve}from"node:path";
const root=resolve(process.argv[2]!),L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🚪️entry/🟦️.ts";
const a=await import(L+"/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"),closure=await import(L+"/🏗️build/📥️source/🔏️closure/🟦️.ts");
const c=await a.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true}),bytes=a.nxAuthorizerCapturedBytesV1(c),imports=a.nxAuthorizerCapturedImportsV1(root,c);
const files=c.manifest.sources.map((r:{path:string})=>({...r,base64:Buffer.from(bytes.get(r.path)!).toString("base64"),imports:imports.get(r.path)!}));
console.log(closure.buildSourceClosureIdentityV1({version:1,root,entry,files}));
