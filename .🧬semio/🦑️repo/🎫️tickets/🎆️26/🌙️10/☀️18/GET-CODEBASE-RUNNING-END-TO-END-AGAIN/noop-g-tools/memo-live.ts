import{resolve,join}from"node:path";import{mkdtempSync}from"node:fs";import{tmpdir}from"node:os";
const stage="/private/tmp/noopg/stage/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
const m=await import(stage+"🟦️.ts"),memoMod=await import(stage+"🧊️memo/🟦️.ts");
const root=resolve(import.meta.dirname,"../../../../../../../..");const entry=process.argv[2]!;
const base=mkdtempSync(join(tmpdir(),"noopg-memo-"));const file=memoMod.nxAuthorizerMemoFileV1(base,root,entry,true);
for(let i=0;i<Number(process.argv[3]??3);i++){
 let served="";const t=performance.now();
 const c=await m.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true,memo:{file,served:s=>served=s}});
 const t1=performance.now();const imp=m.nxAuthorizerCapturedImportsV1(root,c);const t2=performance.now();
 console.log(JSON.stringify({run:i,served,files:c.manifest.sources.length,digest:c.digest.slice(0,16),captureMs:Math.round(t1-t),importsMs:Math.round(t2-t1),imports:imp.size}));
}
const plain=await m.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});console.log("plain digest",plain.digest.slice(0,16));
