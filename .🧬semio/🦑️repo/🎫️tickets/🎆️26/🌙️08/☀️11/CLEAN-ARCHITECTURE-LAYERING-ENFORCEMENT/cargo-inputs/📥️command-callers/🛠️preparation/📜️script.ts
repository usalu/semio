#!/usr/bin/env bun
import {randomUUID} from "node:crypto";
import {join} from "node:path";
import {acquireQueuedResourceLease} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {Script,ScriptRouter} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {getWorkspaceRoot} from "../../🟦️.ts";
import {CargoCliOperationOwner,CargoPreparationExecutor,cargoSystemMilliseconds,cargoSystemServices,parseCargoCliPolicy,runCargoCliCommand,cargoWorkspaceForManifest,prepareCargoOwners,publishCargoWorkspaceMembership,emitCargoPreparationDiagnostic} from "../🟦️.ts";
import operationPolicy from "../🎛️control/🚪️cli/🧩️entrypoint/🎛️policy/🔣️.json";
import systemPolicy from "../🎛️control/🚪️cli/🧩️entrypoint/🧩️system/🎛️policy/🔣️.json";

/** 🛠️ Prepares exact selected package roots through real finite command, process and queued lease capabilities. */
export class PreparationScript extends Script {
 async run(args:string[]):Promise<void>{
  if(args.length<2||args[0]!=="--manifest"||(args.length-2)%2||args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw Error("prepare --manifest <selected-manifest> [--package <name>...]");
  const owner=new CargoCliOperationOwner(parseCargoCliPolicy(operationPolicy),{publish:row=>process.stderr.write(`[cargo-operation] ${row.stage} ${row.completed} units ${row.ownedBytes} retained bytes ${row.path}\n`),yieldContinuation:()=>new Promise(resolve=>setImmediate(resolve))}),maximumMilliseconds=cargoSystemMilliseconds(systemPolicy),names=args.filter((_,index)=>index>=3&&index%2===1);
  await runCargoCliCommand(owner,async()=>{
   const started=performance.now(),lease=await acquireQueuedResourceLease({directory:join(this.root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:`cargo-preparation:${this.root}`,mode:"exclusive",owner:randomUUID(),signal:owner.operation.signal,timeoutMs:maximumMilliseconds,onWait:()=>emitCargoPreparationDiagnostic("lease-wait",args[1]!,performance.now()-started,1)});
   try{
    emitCargoPreparationDiagnostic("lease",args[1]!,performance.now()-started,1);
    await prepareCargoOwners(this.root,{schemaVersion:1,manifest:args[1]!,packages:names},owner.operation,new CargoPreparationExecutor(this.root,cargoSystemServices(this.root,maximumMilliseconds).preparation));
    if(owner.workspace.state==="prepared")await owner.advanceWorkspace();
    const scope=await cargoWorkspaceForManifest(this.root,args[1]!,owner.operation),publishing=performance.now(),changed=await publishCargoWorkspaceMembership(this.root,scope,"write",{...owner.operation,lease});
    emitCargoPreparationDiagnostic("publication",scope.manifest,performance.now()-publishing,Number(changed));
   }finally{lease.release();}
  });
 }
}

if(import.meta.main)await new ScriptRouter(getWorkspaceRoot()).register("prepare",PreparationScript).run(process.argv.slice(2));
