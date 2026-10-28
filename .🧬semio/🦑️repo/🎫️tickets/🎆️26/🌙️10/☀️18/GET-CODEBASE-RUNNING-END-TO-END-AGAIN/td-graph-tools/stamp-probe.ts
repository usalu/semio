/** 🔎 [td-graph] prints the current output of output-stamps probes (same procedure as the law's runProbe). usage: bun stamp-probe.ts <root> <probe-id>... */
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync,readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
const [root,...ids]=process.argv.slice(2),inf=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference"),library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixture=JSON.parse(readFileSync(join(inf,"🧪️tests/🔖️output-stamps/🔣️.json"),"utf8"));
const plugin:any=await import(join(inf,"🟨️.mjs"));await plugin.libraryBootstrap;const api=plugin.cacheInternals;
const revive=(value:any,workspace:string):any=>typeof value==="string"?value.split("{ws}").join(workspace):Array.isArray(value)?value.map(item=>revive(item,workspace)):value&&typeof value==="object"?("$scripts" in value?plugin.createScriptInputCache({workspaceRoot:workspace}):"$map" in value?new Map(value.$map.map((row:any)=>revive(row,workspace))):Object.fromEntries(Object.entries(value).map(([key,item])=>[key,revive(item,workspace)]))):value;
for(const probe of fixture.probes.filter((p:any)=>ids.includes(p.id))){
 const workspace=mkdtempSync(join(realpathSync(tmpdir()),"output-stamps-"));
 try{for(const[path,text]of Object.entries(probe.files??{})){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text as string);}
  let output:unknown;try{output={value:await api[probe.call](...revive(probe.args,workspace))};}catch(error){console.log("[DEBUG]",(error as any).code,(error as any).cause?.code,(error as Error).stack?.split("\n").slice(0,8).join(" | "));output={error:String((error as Error).message)};}
  const text=JSON.stringify(output,(_k,v)=>v instanceof Map?[...v.entries()]:v);
  console.log(probe.id,text.split(workspace).join("{ws}").split(library).join("{library}"));
 }finally{rmSync(workspace,{recursive:true,force:true});}
}
