import {randomUUID} from "node:crypto";
import {join,resolve} from "node:path";
import {Writable} from "node:stream";
import {acquireQueuedResourceLease} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {terminateOwnedProcessTree} from "../../../../../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import {runCargoRepositoryOperation} from "../🟦️.ts";
import type {CargoInvocationServices} from "../../../../🏃️command/🧩️repository/🟦️.ts";

/** 📋️ Admits one explicit finite system subprocess policy. */
export function cargoSystemMilliseconds(value:unknown):number{if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).some(key=>!["schemaVersion","maximumMilliseconds"].includes(key)))throw Error("Cargo system policy must be closed");const row=value as Record<string,unknown>;if(row.schemaVersion!==1||typeof row.maximumMilliseconds!=="number"||!Number.isSafeInteger(row.maximumMilliseconds)||row.maximumMilliseconds<1||row.maximumMilliseconds>2147483647)throw Error("Cargo system policy requires a finite process deadline");return row.maximumMilliseconds;}

/** 🌊️ Awaits completion of the actual caller-owned stream write and its backpressure. */
async function forward(stream:Writable,bytes:Uint8Array):Promise<void>{await new Promise<void>((accept,reject)=>stream.write(bytes,error=>error?reject(error):accept()));}

/** 🔌️ Constructs required first-party lease, process and output capabilities for one literal root. */
export function cargoSystemServices(root:string,maximumMilliseconds:number):CargoInvocationServices{
 cargoSystemMilliseconds({schemaVersion:1,maximumMilliseconds});const repository=resolve(root);
 return {leases:{async acquire(selected,operation){if(resolve(selected)!==repository)throw Error("Cargo system lease root differs from command root");const lease=await acquireQueuedResourceLease({resource:`cargo-preparation:${repository}`,mode:"exclusive",owner:randomUUID(),directory:join(repository,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),signal:operation.signal,timeoutMs:maximumMilliseconds,onWait:row=>{console.error(`Waiting for exclusive access to ${row.resource} (${row.elapsedMs}ms)`);operation.onProgress({stage:"invocation-lease-wait",path:repository,completed:operation.workspace.completed,ownedBytes:operation.workspace.ownedBytes});}});return{resource:lease.resource,mode:lease.mode,async release(){lease.release();}};}},preparation:{maximumMilliseconds,terminate:terminateOwnedProcessTree,forward:bytes=>forward(process.stderr,bytes)},execution:{maximumMilliseconds,async terminate(pid){terminateOwnedProcessTree(pid);},forward:(channel,bytes)=>forward(channel==="stdout"?process.stdout:process.stderr,bytes)}};
}

/** 🚦️ Admits actual system capabilities before any awaited physical repository command. */
export async function runCargoSystemOperation<T>(root:string,maximumMilliseconds:number,body:()=>Promise<T>):Promise<T>{return runCargoRepositoryOperation(cargoSystemServices(root,maximumMilliseconds),body);}
