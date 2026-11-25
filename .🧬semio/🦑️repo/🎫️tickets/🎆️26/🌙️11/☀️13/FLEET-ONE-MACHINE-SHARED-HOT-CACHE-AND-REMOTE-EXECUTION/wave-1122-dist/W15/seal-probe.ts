/** 🔬️ W15: memory of the selected exchange on the real 609-owner generation of the Ready workspace scope, read-only from the fleet: the fetched generation held during a repaired read, and the seal's publication build (documents) on the same data. Prints one JSON row per step (rss, peak, heap, external after a full collection). Env: STEP=all|fetch. */
import {heapStats} from "bun:jsc";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",X=L+"🕸️graph/📥️load/🤝️selected-exchange/";
const ex=await import(X+"🟦️.ts"),at=await import(X+"🔌️attempt/🟦️.ts"),port=await import(L+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts"),fi=await import(L+"🕸️graph/🧭️file-owner/🟨️.mjs");
const MB=1048576,scope="2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64",host=port.hostOfRootV1(root);
const mark=(label:string,extra:Record<string,unknown>={})=>{Bun.gc(true);const m=process.memoryUsage(),h=heapStats();console.log(JSON.stringify({label,rssMB:Math.round(m.rss/MB),peakMB:Math.round(process.resourceUsage().maxRSS/1024),heapMB:Math.round(m.heapUsed/MB),externalMB:Math.round(m.external/MB),objects:h.objectCount,strings:h.objectTypeCounts.string,...extra}));};
mark("baseline");
const client=at.selectedFleetClientV1(root);if(!client){console.log("no client");process.exit(0);}
let fetched:any=await ex.lookSelectedGenerationV1({client,scope,platform:"darwin-arm64",host,lookMaximumMilliseconds:120000});
if(typeof fetched==="string"){console.log("look",fetched);process.exit(0);}
let ownerRows=0;for(const rows of fetched.rowsByOwner.values())ownerRows+=rows.length;
const admRows=fetched.admission.reduce((s:number,e:any)=>s+e.rows.length,0);
mark("fetched-held",{created:new Date(fetched.head.createdMs).toISOString(),owners:Object.keys(fetched.head.owners).length,documents:fetched.head.documents.map((d:any)=>[d.name,Math.round(d.size/MB)]),graphChars:fetched.graph.length,ownerRows,admissionEntries:fetched.admission.length,admissionRowRefs:admRows});
const factsChars=JSON.stringify(fetched.admission.map((e:any)=>e.facts)).length;mark("facts-measured",{factsJsonMB:Math.round(factsChars/MB)});
if(process.env.STEP==="fetch")process.exit(0);
const projects=JSON.parse(fetched.graph).projects,ownerOf=fi.createProjectFileIndexV1(projects);
const rows=[...fetched.rowsByOwner.values()].flat();
const sealed={graph:fetched.graph,rows,membership:fetched.head.membership,admission:fetched.admission};
mark("sealed-input",{rows:rows.length});
let built:any=ex.buildSelectedGenerationV1({scope,platform:"darwin-arm64",host,sealed,ownerOf,now:Date.now()});
mark("generation-built",{documents:built.documents.map((d:any)=>[d.name,Math.round(d.bytes.length/MB)])});
built=undefined;mark("generation-released");
fetched=undefined;(sealed as any).admission=undefined;(sealed as any).rows=undefined;rows.length=0;mark("fetched-released");
