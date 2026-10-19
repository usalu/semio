import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const N=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph");
const owner=await import(join(N,"🟦️.ts"));
const fixture=(await import(join(N,"🧪️tests/🔣️.json"),{with:{type:"json"}})).default as any;
const base=JSON.parse(JSON.stringify(fixture.currentArtifactObservations[0]).replaceAll("/workspace",process.cwd()));
const value=base.observation;value.capturePurpose="current-bounded-complete-payload-stream";
try{(owner as any).readNxGraphCurrentArtifactObservationV1(process.cwd(),base.intent,value);console.log("ok");}catch(error){console.log("ERR",(error as Error).message);}
