/** 🧪️ Public tool-run codecs bound the complete planned browser workload, including retained plugin trace. */
import Ajv from "ajv";
import {readFileSync, readdirSync} from "node:fs";
import {join} from "node:path";
import {encodeToolRunTick, decodeToolRunTick} from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🔤️codec/🟦️.ts";
import {toolRunTickToJson, toolRunTickFromJson} from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🧾️json/🟦️.ts";
import {TOOL_RUN_TICK_BYTES_MAX} from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts";
import type {ToolRunTick} from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🟦️.ts";
const cases=JSON.parse(readFileSync(new URL("./wfc-browser-cases.json",import.meta.url),"utf8"));
const roots="✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/";
const identities={bitmap:["🖼️bitmap","🌡️fill"],grid2d:["🔲️grid2d",""],grid3d:["🧱️grid3d","🪣️fill"],wfc2d:["◻️2d","🔣️fill"],wfc3d:["🧊️3d","📑️fill"]};
const ajv=new Ajv({strict:false,allErrors:true}),revision=Array(32).fill(255),tileId="scenario-tile";
const maximum=BigInt(Number.MAX_SAFE_INTEGER);
const identity={id:{appInstanceId:1,run:maximum},generation:0xffffffff,baseRevision:Uint8Array.from(revision)};
const step={sequence:maximum,kind:"info" as const,stage:65535,reason:65535,subject:maximum,repeat:1,args:Array.from({length:4},()=>({unsigned:maximum}))};
for(const variant of cases.variants){
 const name=variant.variant as keyof typeof identities,[artifact,tool]=identities[name];
 const dimensions=variant.workload.forms[0]?.args;
 let payload:Record<string,unknown>;
 if(name==="bitmap"){
  const width=Number(dimensions.width),height=Number(dimensions.height),cells=width*height;
  payload={pixels:Buffer.from(Array.from({length:cells},(_,i)=>i%2)).toString("base64"),decided:Buffer.alloc(cells,1).toString("base64"),width,height,contradiction:false,done:true,trace:Array.from({length:512},(_,i)=>({index:i,color:i%2,discarded:false}))};
 }else if(name==="grid2d"||name==="grid3d"){
  const width=Number(dimensions.width),height=Number(dimensions.height),depth=name==="grid3d"?Number(dimensions.depth):1;
  const cell=(i:number)=>({x:i%width,y:Math.floor(i/width)%height,...(name==="grid3d"?{z:Math.floor(i/(width*height))}:{}),tileId});
  payload={...(name==="grid3d"?{sourceRevision:revision}:{}),assignments:Array.from({length:width*height*depth},(_,i)=>cell(i)),contradiction:false,done:true,trace:Array.from({length:512},(_,i)=>({...cell(width*height*depth-1-i),discarded:false}))};
 }else{
  const slots=variant.workload.slots;
  const keys=Array.from({length:slots.count},(_,i)=>slots.idPrefix+"de-"+i.toString().padStart(3,"0")).concat(["boot-slot-a","boot-slot-b"]);
  payload={sourceRevision:revision,assignments:Object.fromEntries(keys.map(key=>[key,tileId])),contradiction:false,done:true,trace:Array.from({length:512},(_,i)=>({slotId:keys[i%keys.length],tileId,discarded:false}))};
 }
 const base=roots+artifact+"/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/";
 const selected=tool||readdirSync(base).find(entry=>entry.includes("fill"))!;
 const schema=JSON.parse(readFileSync(join(base,selected,"🧬️schema/🔣️.json"),"utf8"));
 if(!ajv.validate(schema,payload))throw Error(name+" payload: "+JSON.stringify(ajv.errors));
 const text=JSON.stringify(payload),raw=new TextEncoder().encode(text);
 const tick:ToolRunTick={identity,sequence:maximum,progress:{identity,sequence:maximum,state:"complete",stage:65535,completed:maximum,total:maximum,counters:Array.from({length:9},(_,counter)=>({counter,value:maximum})),unitsPerSecond:1,conflicts:0xffffffff,steps:Array(64).fill(step)},steps:Array(64).fill(step),trace:[],appendOps:[],appendEntities:[],payload:raw,payloadUpdate:{kind:"replace"}};
 const binary=encodeToolRunTick(tick),decoded=decodeToolRunTick(binary);
 if(binary.length>TOOL_RUN_TICK_BYTES_MAX||!Buffer.from(decoded.payload!).equals(Buffer.from(raw)))throw Error(name+" tick carrier");
 const json=JSON.stringify(toolRunTickToJson(tick));
 const mirrored=encodeToolRunTick(toolRunTickFromJson(JSON.parse(json)));
 if(!Buffer.from(mirrored).equals(Buffer.from(binary)))throw Error(name+" JSON/hex mirror changed");
 const outer=Buffer.byteLength(json),quoted=Buffer.byteLength(JSON.stringify({payloadJson:text})),base64=Buffer.byteLength(Buffer.from(binary).toString("base64"));
 if(outer>2*TOOL_RUN_TICK_BYTES_MAX+16384||quoted>2*TOOL_RUN_TICK_BYTES_MAX+16384)throw Error(name+" retained command/outer envelope");
 console.log(JSON.stringify({variant:name,pluginPayloadBytes:raw.length,fullBinaryTickBytes:binary.length,tickMaximumBytes:TOOL_RUN_TICK_BYTES_MAX,quotedPayloadBytes:quoted,base64TransportBytes:base64,jsonHexTransportBytes:outer,traceRecords:512,stepRecords:128,meaning:"carrier-shaped upper bound over planned extents and longest declared tile ID; binary tick cap is distinct from outer transports"}));
}
