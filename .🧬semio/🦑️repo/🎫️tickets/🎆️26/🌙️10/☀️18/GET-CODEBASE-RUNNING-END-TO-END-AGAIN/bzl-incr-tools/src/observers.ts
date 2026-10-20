import * as zlib from "node:zlib";
import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {incrementalLabelV1,type IncrementalObservationV1} from "../@@TS@@";

const policy=schema.definitions.Policy.const,order=(left:string,right:string)=>left<right?-1:left>right?1:0,sorted=(values:Iterable<string>)=>[...new Set(values)].sort(order);

/** 🧵️ One executed or restored Bazel action as the compact execution log records it. */
export type IncrementalSpawnV1=Readonly<{label:string;mnemonic:string;runner:string;cacheHit:boolean}>;

/** 🗜️ Decompresses a zstd stream with the decoder the runtime provides and refuses output beyond the bound. */
export function incrementalZstdV1(bytes:Uint8Array):Uint8Array{
 const bun=(globalThis as {Bun?:{zstdDecompressSync?:(input:Uint8Array)=>Uint8Array}}).Bun,node=(zlib as unknown as {zstdDecompressSync?:(input:Uint8Array)=>Uint8Array}).zstdDecompressSync,decode=bun?.zstdDecompressSync??node;
 if(!decode)throw Error("A zstd decoder is required to read the Bazel execution log");
 const output=decode(bytes);
 if(output.length>policy.maximumLogBytes)throw Error("Bounded Bazel execution log required");
 return output;
}

function varint(bytes:Uint8Array,at:{position:number}):number{
 let value=0,scale=1;
 for(let count=0;count<8;count++){
  if(at.position>=bytes.length)throw Error("Truncated Bazel execution log");
  const byte=bytes[at.position++]!;value+=(byte&0x7f)*scale;scale*=128;
  if(!(byte&0x80))return value;
 }
 throw Error("Oversized varint in Bazel execution log");
}

type Field=Readonly<{number:number;wire:number;value:number;bytes:Uint8Array}>;

function fields(bytes:Uint8Array):Field[]{
 const at={position:0},found:Field[]=[],none=new Uint8Array();
 while(at.position<bytes.length){
  const key=varint(bytes,at),wire=key%8,number=Math.floor(key/8);
  if(wire===0)found.push({number,wire,value:varint(bytes,at),bytes:none});
  else if(wire===2){const length=varint(bytes,at);if(at.position+length>bytes.length)throw Error("Truncated Bazel execution log");found.push({number,wire,value:0,bytes:bytes.subarray(at.position,at.position+length)});at.position+=length;}
  else if(wire===1){at.position+=8;}
  else if(wire===5){at.position+=4;}
  else throw Error("Unsupported wire type in Bazel execution log");
 }
 return found;
}

const text=new TextDecoder("utf-8",{fatal:true});

/** 🪡 Reads the spawns of a compact Bazel execution log (zstd of length-delimited entries; entry field 7 is the spawn; spawn fields 7 label, 8 mnemonic, 11 runner, 12 cache hit). */
export function incrementalBazelSpawnsV1(compact:Uint8Array):IncrementalSpawnV1[]{
 const bytes=incrementalZstdV1(compact),at={position:0},spawns:IncrementalSpawnV1[]=[];
 let entries=0;
 while(at.position<bytes.length){
  if(++entries>policy.maximumLogEntries)throw Error("Bounded Bazel execution log required");
  const length=varint(bytes,at);
  if(at.position+length>bytes.length)throw Error("Truncated Bazel execution log");
  const entry=bytes.subarray(at.position,at.position+length);at.position+=length;
  const spawn=fields(entry).find(field=>field.number===7&&field.wire===2);
  if(!spawn)continue;
  const row=fields(spawn.bytes),string=(number:number)=>{const field=row.find(candidate=>candidate.number===number&&candidate.wire===2);return field?text.decode(field.bytes):"";};
  spawns.push({label:string(7),mnemonic:string(8),runner:string(11),cacheHit:row.some(field=>field.number===12&&field.wire===0&&field.value===1)});
 }
 return spawns;
}

/** 🔭 Maps Bazel spawns to tasks: a task with a spawn that did not hit a cache executed, a task whose spawns all hit a cache was restored, a task without a spawn was an action cache hit; unmapped labels are support actions. */
export function incrementalBazelObservationV1(spawns:readonly IncrementalSpawnV1[],taskIds:readonly string[]):IncrementalObservationV1{
 const tasks=new Map(taskIds.map(id=>[`//:${incrementalLabelV1("t",id)}`,id])),executed=new Set<string>(),hit=new Set<string>(),unmapped=new Set<string>();
 for(const spawn of spawns){
  const label=spawn.label.replace(/^@@?\/\//u,"//"),id=tasks.get(label);
  if(!id){if(label)unmapped.add(label);continue;}
  if(spawn.cacheHit)hit.add(id);else executed.add(id);
 }
 const restored=[...hit].filter(id=>!executed.has(id));
 return Object.freeze({tool:"bazel" as const,executed:sorted(executed),restored:sorted(restored),cached:sorted(taskIds.filter(id=>!executed.has(id)&&!hit.has(id))),unmapped:sorted(unmapped)});
}

type NxTask=Readonly<{taskId:string;cacheStatus:string;status:number}>;

/** 🗒 Maps the run record Nx stores after every run (task id, cache status, exit code) to the neutral observation: a cache miss executed, a local or remote hit restored. */
export function incrementalNxObservationV1(record:unknown,taskIds:readonly string[]):IncrementalObservationV1{
 const tasks=(record as {tasks?:unknown})?.tasks;
 if(!Array.isArray(tasks))throw Error("Closed Nx run record required");
 for(const task of tasks as NxTask[])if(typeof task?.taskId!=="string"||!["local-cache-hit","remote-cache-hit","cache-miss"].includes(task.cacheStatus))throw Error("Closed Nx run record required");
 const known=new Set(taskIds),rows=(tasks as NxTask[]).filter(task=>known.has(task.taskId));
 return Object.freeze({tool:"nx" as const,executed:sorted(rows.filter(task=>task.cacheStatus==="cache-miss").map(task=>task.taskId)),restored:sorted(rows.filter(task=>task.cacheStatus!=="cache-miss").map(task=>task.taskId)),cached:[],unmapped:sorted((tasks as NxTask[]).filter(task=>!known.has(task.taskId)).map(task=>task.taskId))});
}
