import {resolve} from "node:path";
const root=process.cwd();
const I=resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference");
const inference=await import(I+"/🟨️.mjs");await inference.libraryBootstrap;
const scripts=await import(I+"/🧭️script-inputs/🟨️.mjs");
const cache=inference.createScriptInputCache({workspaceRoot:root});
const entries=process.argv.slice(2);
const union=new Set<string>();
for(const e of entries){
  const closure:string[]=await scripts.relativeScriptInputs([resolve(root,e)],root,cache as never,false);
  for(const f of closure)union.add(f);
  const t=closure.filter(x=>x.includes("🧪️tests/")).length;
  console.log(String(closure.length).padStart(5),"tests",t,e.slice(-70));
}
console.log("union",union.size);
if(process.env.OUT)await Bun.write(process.env.OUT,JSON.stringify([...union].sort()));
