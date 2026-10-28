/** 🩺️ Replays the graph child's fact check (inference import, libraryBootstrap, prepareSourceAnalysesV1, observationsHold) on one recorded declared-import analysis row of the current generation. Usage: bun verify-row.ts <root> <needle> */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,needle]=process.argv.slice(2) as [string,string];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const generation=await import(L+"/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const store=join(root,".cache/b/g/m/generations"),digest=JSON.parse(readFileSync(join(store,"current.json"),"utf8")).manifest;
const engine=generation.createGenerationStoreV1({root:store,fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(digest),index=await engine.readIndexView(manifest.factIndex);
const rows:unknown[]=[];for(let at=0;at<index.rows.length;at++){const row=index.rows[at];if(row.kind==="analysis"&&row.fact.provider==="build:declared-source-imports:v1"&&row.path.includes(needle))rows.push(row);}
const producer=await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const {observationsHold}=await import(L+"/🏗️build/👁️observation/🟨️.mjs");
console.log("[DEBUG] rows",rows.length,"hold after the graph child's preparation",await observationsHold(rows,64,new Set(),true));
