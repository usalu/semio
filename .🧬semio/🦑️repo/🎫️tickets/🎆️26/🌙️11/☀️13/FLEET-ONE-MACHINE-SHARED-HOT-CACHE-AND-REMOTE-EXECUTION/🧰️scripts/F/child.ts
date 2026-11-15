import {graphFleetClientV1,decideGraphEntryV1,publishGraphEntryV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🛰️exchange/🟦️.ts";
import {nxGraphStateV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🧶️nx/🟦️.ts";
import {mkdtempSync} from "node:fs";import {tmpdir} from "node:os";import {join} from "node:path";
const [endpoint,machine]=process.argv.slice(2);
const client=graphFleetClientV1({endpoint:endpoint!,instance:"main",token:"w",upload:true,stateDirectory:mkdtempSync(join(tmpdir(),"c-"))});
const state=nxGraphStateV1({graphDigest:"a".repeat(64),implementation:"b".repeat(64),tooling:"c".repeat(64),identity:"d".repeat(64)},"e".repeat(64));
const started=Date.now(),d=await decideGraphEntryV1({client,state,owner:`${machine}:${process.pid}`});
if(d.outcome==="hit"){console.log(JSON.stringify({machine,outcome:"hit",via:d.via,ms:Date.now()-started}));process.exit(0);}
await Bun.sleep(3000);
const ok=await publishGraphEntryV1({client,state,owner:`${machine}:${process.pid}`,documents:[{name:"nx/project-graph.json",bytes:Buffer.from("{}")}],hold:d.hold});
console.log(JSON.stringify({machine,outcome:"computed",published:ok,ms:Date.now()-started}));
