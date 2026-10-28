// corpus-run: bun corpus-run.ts <library root> <corpus.json> : the verdict of `cargoSourceInputs` (what the live derivation returns: files or undefined = directory fallback) for every case of the mounted-sources corpus, in temp dirs outside the checkout. Prints one JSON row per case; used to show base (red) against the staged batch (green) on the same corpus.
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync,readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
const [library,corpusPath]=process.argv.slice(2) as [string,string];
const inference:any=await import(join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const {cacheInternals}=inference,corpus=JSON.parse(readFileSync(corpusPath,"utf8"));
for(const row of corpus.cases){
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-corpus-"));
 try{
  for(const[path,text]of Object.entries(row.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  const named=row.flavor==="named",files=cacheInternals.cargoSourceInputs(row.crate,root,new Map(),named,!named);
  console.log(JSON.stringify({case:row.name,expected:row.resolved?"files":"refused",got:files===undefined?"undefined (directory fallback)":files.length+" files"+(files.some((f:string)=>!existsSync(join(root,f.replace("{workspaceRoot}/",""))))?", of which NONEXISTENT: "+files.filter((f:string)=>!existsSync(join(root,f.replace("{workspaceRoot}/","")))).map((f:string)=>f.slice(-40)).join(" "):"")}));
 }finally{rmSync(root,{recursive:true,force:true});}
}
