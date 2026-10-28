/** ⏱️ Read-only probe: for a live sealed memo record and one edited path, the derived questions of the record's slice whose support names the path (pseudo owners holding a row of it), asked again with the planner's own deriver (`openGenerationDeriverV1` + `settleSuspectsV1`) in this process: wall, CPU, RSS, verdict, and whether the new support reads only entries the old support read. Usage: bun derive.ts <code root> <live root> <record.json> <relative path> [row kind taken as stale, default every row of the path] */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [code,live,recordFile,target,kind]=process.argv.slice(2) as [string,string,string,string,string|undefined];
const B=code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const rss=()=>Math.round(process.memoryUsage().rss/1048576),cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1e6;};
const step=async<T>(name:string,body:()=>Promise<T>|T):Promise<T>=>{const started=performance.now(),before=cpu();const value=await body();console.log("[DEBUG]",name.padEnd(26),JSON.stringify({ms:Math.round(performance.now()-started),cpu:+(cpu()-before).toFixed(2),rss:rss()}));return value;};
const generation=await import(B+"/🕸️graph/🕰️generations/🟨️.mjs"),cutoff=await import(B+"/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs");
const record=JSON.parse(readFileSync(recordFile,"utf8")),engine=generation.createGenerationStoreV1({root:join(live,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(record.manifest),index=await step("index view",()=>engine.readIndexView(manifest.factIndex));
const absolute=join(live,target),ids=new Set<number>();for(let id=0;id<index.rows.length;id++)if(index.rows[id].path===absolute&&(!kind||index.rows[id].kind===kind))ids.add(id);
const {decodeVerifiedScopeV1}=await import(B+"/🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs"),covered=decodeVerifiedScopeV1(Object.keys(manifest.owners).sort(generation.compareCodePointsV1),JSON.parse(record.pin).covered);
const classes=generation.selectFactClassesV1(index,cutoff.scopeWithDerivedV1(index,new Set([...covered,generation.GENERATION_POLICY_V1.envOwner]))).classes,split=cutoff.splitStaleConsumersV1(index,classes,ids);
console.log("[DEBUG] rows of the path",ids.size,"direct holders",[...split.direct].slice(0,5),"derived suspects",split.suspects.size);
const producer=await step("inference + analyses",async()=>{const module=await import(B+"/🕸️graph/📥️inference/🟨️.mjs");await module.libraryBootstrap;module.prepareSourceAnalysesV1();return module;});
const {readBuildOwnerRegistryV1}=await import(B+"/🕸️graph/🚪️admission/🟦️.ts");
const derive=await step("open deriver",async()=>producer.openGenerationDeriverV1({workspaceRoot:live,registry:readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(live)))}));
const settled=await step("settle suspects",()=>cutoff.settleSuspectsV1({index,suspects:split.suspects,derive}));
const identity=(row:{kind:string;path:string})=>row.kind+"\u0000"+row.path;
for(const [name,rows] of settled.held){const old=new Set(generation.selectFactClassesV1(index,new Set([name])).ids.map((id:number)=>identity(index.rows[id])));const fresh=(rows as {kind:string;path:string}[]).slice(1);console.log("[DEBUG] held",name.slice(0,24),"old support",old.size,"new support",fresh.length,"new ⊆ old",fresh.every(row=>old.has(identity(row))));}
const observation=await import(B+"/👁️observation/🟨️.mjs");
for(const [name,row] of settled.changed){const support=generation.selectFactClassesV1(index,new Set([name])).ids.map((id:number)=>index.rows[id]).filter((item:{kind:string})=>item.kind!=="derived");let stale=0;const kinds:Record<string,number>={};for(let at=0;at<support.length;at+=512){const chunk=support.slice(at,at+512);if(await observation.observationsHold(chunk,64,new Set(),true))continue;for(const item of chunk)if(!await observation.observationsHold([item],1,new Set(),true)){stale++;kinds[item.kind]=(kinds[item.kind]??0)+1;if(stale<4)console.log("[DEBUG] stale support",item.kind,String(item.path).slice(-100));}}console.log("[DEBUG] support",name.slice(0,24),support.length,"stale",stale,JSON.stringify(kinds));const answer=await derive(row);console.log("[DEBUG] changed",name.slice(0,24),String(row.path).slice(0,240),"recorded",JSON.stringify(row.fact).slice(0,80),"now",JSON.stringify(answer.fact).slice(0,80));}
console.log("[DEBUG] verdict",JSON.stringify({held:settled.held.size,changed:settled.changed.size,failed:settled.failed}));
