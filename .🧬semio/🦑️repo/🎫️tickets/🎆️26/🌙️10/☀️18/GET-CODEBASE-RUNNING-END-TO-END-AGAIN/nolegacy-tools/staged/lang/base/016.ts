import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {BazelSlotStateV1} from "../🟦️.ts";

/** 📖️ The read-only leaf of the session slots: where the servers live, which slots exist, which server process lives in a slot and what a slot remembers. It imports no lease, no storage and no tool module, so a status command can read the Bazel servers without loading the build tools.
 * @see ../🔌️port/🟦️.ts the claims and the writes, ../🧬️schema/🔣️.json the names and the slot state */
const policy=schema.definitions.Policy.const,slotPattern=new RegExp(policy.names.slotPattern),missing=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??"");
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🗂️ The directory that holds the output bases of every slot, below the products partition. */
export function bazelServerDirectoryV1(products:string):string{return join(products,"bazel/outputs/server");}
/** 🔢️ The slots that have an output base, in directory order. */
export function listBazelSlotsV1(server:string):readonly string[]{
 try{return readdirSync(server,{withFileTypes:true}).filter(row=>row.isDirectory()&&slotPattern.test(row.name)).map(row=>row.name).sort();}catch(error){if(missing(error))return [];throw error;}
}
/** 🔎️ The pid of the server living in an output base: its pid file names a process that still exists; undefined when there is none or the file is stale. */
export function readBazelServerPidSyncV1(outputBase:string):number|undefined{
 try{const pid=Number(readFileSync(join(outputBase,policy.stop.pidFile),"utf8").trim());return Number.isSafeInteger(pid)&&pid>0&&alive(pid)?pid:undefined;}catch(error){if(missing(error))return undefined;throw error;}
}
/** 🧠️ What a slot remembers, or undefined when it remembers nothing valid or its output base is gone. */
export function readBazelSlotStateSyncV1(server:string,slot:string):BazelSlotStateV1|undefined{
 if(!slotPattern.test(slot))throw Error(`Bazel session slot required: ${slot}`);
 try{const value=JSON.parse(readFileSync(join(server,slot+policy.names.stateSuffix),"utf8"));return validateJsonSchemaSubset(schema.definitions.SlotState as never,value,schema as never).length||value.slot!==slot?undefined:value as BazelSlotStateV1;}
 catch(error){if(missing(error)||error instanceof SyntaxError)return undefined;throw error;}
}
