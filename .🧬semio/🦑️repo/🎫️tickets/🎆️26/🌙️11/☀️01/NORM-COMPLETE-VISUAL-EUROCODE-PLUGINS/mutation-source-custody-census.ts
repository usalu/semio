import fs from "node:fs/promises";
import path from "node:path";
const root=process.cwd(),ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS";
const {readRustSourceClosureV1}=await import(path.join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const {buildInputPathMatchesV1}=await import(path.join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs"));
const policyPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json";
const policy=JSON.parse(await fs.readFile(policyPath,"utf8"));
function boundary(v:any):any{if(v&&typeof v==="object"){if(v.maxSourceFiles&&v.maxActiveLines)return v;for(const x of Object.values(v)){const found=boundary(x);if(found)return found;}}}
const ordinary=boundary(policy),rows:any[]=[],catalogues:any[]=[];
const cfg=["debug_assertions","target_arch=x86_64","target_os=linux","target_family=unix","unix","target_env=gnu","target_pointer_width=64","target_endian=little","target_has_atomic=8","target_has_atomic=16","target_has_atomic=32","target_has_atomic=64","target_has_atomic=ptr","feature=test-support"];
const artifacts="✏️s/🔌️plugins/📕️norm/🗿️artifacts";
for(const name of await fs.readdir(artifacts)){
 const artifact=path.join(artifacts,name),owner=path.join(artifact,"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"),manifest=path.join(owner,"📦️packages/🦀️rust/Cargo.toml");
 const doc=Bun.TOML.parse(await fs.readFile(manifest,"utf8")) as any;
 const declaration=JSON.parse(await fs.readFile(path.join(artifact,"📦️packages/🦀️rust/📋️project.json"),"utf8"));
 const catalogue=path.join(artifact,"🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json");
 const patterns=declaration.namedInputs?.nativeSources??[];
 catalogues.push({artifact:name,path:catalogue,declaredNativeSources:patterns,covered:patterns.some((p:string)=>buildInputPathMatchesV1(catalogue,p.replace("{workspaceRoot}/","")))});
 const custom=doc.package?.metadata?.semio?.["rust-source-boundary"];
 const limits=custom?Object.fromEntries(["maxSourceBytes","maxSourceFiles","maxActiveLines"].map(k=>[k,Math.min(ordinary[k],custom[k])])):ordinary;
 const entries=[{kind:"lib",file:doc.lib?.path??"src/lib.rs",test:false},{kind:"lib-test",file:doc.lib?.path??"src/lib.rs",test:true},...(doc.test??[]).map((v:any)=>({kind:"test:"+v.name,file:v.path,test:true}))];
 for(const entry of entries){let lastRead="";try{const inputs=await readRustSourceClosureV1([path.resolve(path.dirname(manifest),entry.file)],root,entry.test?[...cfg,"test"]:cfg,async(file:string)=>{lastRead=file;try{return new Uint8Array(await fs.readFile(file))}catch(e:any){if(e.code==="ENOENT")return null;throw e}},{manifestDirectory:path.resolve(path.dirname(manifest)),generated:()=>{}});
 const bytes=inputs.reduce((n:number,x:any)=>n+x.bytes,0),activeLines=inputs.reduce((n:number,x:any)=>n+x.activeLines,0);
 rows.push({owner:doc.package.name,manifest,kind:entry.kind,entry:entry.file,files:inputs.length,bytes,activeLines,limits,within:inputs.length<=limits.maxSourceFiles&&bytes<=limits.maxSourceBytes&&activeLines<=limits.maxActiveLines});
 }catch(e){rows.push({owner:doc.package.name,manifest,kind:entry.kind,lastRead,error:String(e)})}}
}
const generated=path.join(ticket,"🗑️generated/mutation-custody");await fs.mkdir(generated,{recursive:true});
await fs.writeFile(path.join(generated,"census.json"),JSON.stringify({cfg,policyPath,ordinary,catalogues,rows},null,2)+"\n");
const lines=["# Mutation Aggregate Source Custody Census","","Read-only source collection under conservative Linux lib/test cfg with test-support. No compiler cfg probing, compilation or native test pass is claimed. Ordinary policy: "+JSON.stringify(ordinary)+". Actual per-manifest overrides, when present, are intersected with that policy.","","Build input evidence: build/📥️inputs/🟨️.mjs:322 adds declarations.nativeSources to nativeTestSources; build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:70 selects nativeTestSources for native tests. Each artifact declaration below was checked against its actual unchanged catalogue URI using the existing input matcher; no graph planner was run.","","| Artifact | Catalogue Covered by Declared nativeSources |","| --- | --- |",...catalogues.map(x=>"| "+x.artifact+" | "+x.covered+" |"),"","| Actual Cargo Owner | Unit | Files | Bytes | Active Lines | Within |","| --- | --- | ---: | ---: | ---: | --- |",...rows.map(x=>x.error?"| "+x.owner+" | "+x.kind+" | — | — | — | "+x.error.replaceAll("|","/")+" |":"| "+x.owner+" | "+x.kind+" | "+x.files+" | "+x.bytes+" | "+x.activeLines+" | "+x.within+" |"),"","Detailed manifest paths, effective limits, declared input patterns and any source refusal are in the generated census JSON. This report checks source custody only; it does not infer native acceptance."];
await fs.writeFile(path.join(ticket,"🔍️audits/mutation-source-custody-census.md"),lines.join("\n")+"\n");
console.log(JSON.stringify({catalogues:catalogues.length,uncovered:catalogues.filter(x=>!x.covered),units:rows.length,refused:rows.filter(x=>x.error||!x.within)}));
