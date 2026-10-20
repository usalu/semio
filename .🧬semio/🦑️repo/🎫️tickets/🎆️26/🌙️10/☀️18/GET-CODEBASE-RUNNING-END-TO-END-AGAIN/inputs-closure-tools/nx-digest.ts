// 🧮 Nx-side proof: the library's own closure digest (sourceClosureDigest, the value Nx hashes for SEMIO_SOURCE_CLOSURE_<key>) over the plugin's closure of a fixture moves with an imported module and not with an unimported one.
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";import {tmpdir} from "node:os";import {dirname,join} from "node:path";
const W=process.env.INCLOS_BASE??"/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const corpus=JSON.parse(await Bun.file(join(import.meta.dir,"staged/plugin/tests/fixture.json")).text());
const {receiveScriptProcessInvocation,createIssuedResourceControlV1}:any=await import(join(W,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs")),{sourceClosureDigest}:any=await import(join(L,"🏗️build/📥️inputs/🧮️digest/🟨️.mjs"));
await inference.libraryBootstrap;
const root=mkdtempSync(join(realpathSync(tmpdir()),"nx-digest-"));
const build=()=>{for(const[p,t]of Object.entries<string>(corpus.files)){mkdirSync(dirname(join(root,p)),{recursive:true});writeFileSync(join(root,p),t);}for(const[p,t]of Object.entries<string>(corpus.links)){mkdirSync(dirname(join(root,p)),{recursive:true});try{symlinkSync(t,join(root,p));}catch{}}};
try{
 build();
 await receiveScriptProcessInvocation(process.env,async(invocation:any)=>{
  const control=createIssuedResourceControlV1(invocation,"inputs-closure:digest",0);
  const files=async()=>(await inference.cacheInternals.relativeScriptInputs([join(root,corpus.entry)],root,inference.createScriptInputCache(),true)).map((x:string)=>x.replace("{workspaceRoot}/","")).sort();
  const digest=async()=>sourceClosureDigest(root,{version:1,files:await files()},control);
  const before=await digest();
  writeFileSync(join(root,"pkg/src/unused.ts"),"export const unused = 30;\n");const unrelated=await digest();
  writeFileSync(join(root,"pkg/src/deep.ts"),"export const deep = 20;\n");const imported=await digest();
  console.log(JSON.stringify({files:await files(),unimportedChangeKeepsDigest:before===unrelated,importedChangeMovesDigest:before!==imported,before,imported}));
 });
}finally{rmSync(root,{recursive:true,force:true});}
