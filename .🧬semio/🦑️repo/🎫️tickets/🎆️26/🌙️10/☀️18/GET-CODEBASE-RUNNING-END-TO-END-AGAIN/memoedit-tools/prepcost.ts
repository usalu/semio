/** ⏱️ Read-only probe: what registering the source analyses (the planner's `prepareSourceAnalysesV1`) costs in a fresh process, and whether the engine then holds the analysis rows of one path in a sealed memo record's generation. Usage: bun prepcost.ts <root> <record.json> <relative path> */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,recordFile,target]=process.argv.slice(2) as [string,string,string];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const rss=()=>Math.round(process.memoryUsage().rss/1048576),cpu=()=>{const usage=process.cpuUsage();return +((usage.user+usage.system)/1e6).toFixed(2);};
let started=performance.now();
const observation=await import(L+"/👁️observation/🟨️.mjs");
console.log("[DEBUG] observation",JSON.stringify({ms:Math.round(performance.now()-started),cpu:cpu(),rss:rss()}));
started=performance.now();
const producer=await import(L+"/🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
console.log("[DEBUG] prepared",JSON.stringify({ms:Math.round(performance.now()-started),cpu:cpu(),rss:rss()}));
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs");
const record=JSON.parse(readFileSync(recordFile,"utf8"));
const engine=generation.createGenerationStoreV1({root:join(root,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(record.manifest),index=await engine.readIndexView(manifest.factIndex);
const absolute=join(root,target),rows:Array<{kind:string;path:string;fact:unknown}>=[];
for(let id=0;id<index.rows.length;id++){const row=index.rows[id];if(row.path===absolute)rows.push(row);}
started=performance.now();
for(const row of rows)console.log("[DEBUG] holds",row.kind,await observation.observationsHold([row],64,new Set(),true));
console.log("[DEBUG] checked",JSON.stringify({ms:Math.round(performance.now()-started),cpu:cpu(),rss:rss(),rows:rows.length}));
const analysis=rows.find(row=>row.kind==="analysis") as {fact:{source:Record<string,unknown>}}|undefined;
if(analysis){const changed={...analysis,fact:{...analysis.fact,source:{...analysis.fact.source,mtimeMs:1}}};started=performance.now();console.log("[DEBUG] replay holds",await observation.observationsHold([changed],64,new Set(),true),JSON.stringify({ms:Math.round(performance.now()-started),cpu:cpu(),rss:rss()}));}
