import {prepareCargoWorkspaceMembership} from "../../🗂️workspaces/🦀️cargo/📣️publication/🟦️.ts";
import {publishPreparedCargoMembership} from "../../🗂️workspaces/🦀️cargo/📣️publication/📁️physical/🟦️.ts";
import {CargoCliOperationOwner,cargoSystemServices,bindCargoInvocationServices,runCargoCliCommand,CargoPreparationExecutor,prepareCargoOwners,publishCargoWorkspaceMembership,discoverCargoWorkspaces,cargoCommandServices,type CargoPreparationSelection} from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import policy from "../🎛️normalization-owner/🧫️fixtures/🔣️.json";
export type CargoProbeManifest=Readonly<{path:string;source:string;document:unknown}>;
export type CargoProbeObservation<T>=Readonly<{value:T;manifests:readonly CargoProbeManifest[];completed:number;ownedBytes:number;patternCompiles:number}>;

/** 🧪️ Captures actual admitted manifest parses before cleanup under unchanged explicit test controls. */
export async function observeCargoProbe<T>(root:string,body:(owner:CargoCliOperationOwner)=>Promise<T>):Promise<CargoProbeObservation<T>>{
 let patternCompiles=0;const owner=new CargoCliOperationOwner({...policy,schemaVersion:1},{publish:()=>{},yieldContinuation:()=>new Promise(resolve=>setImmediate(resolve))});
 return runCargoCliCommand(owner,async()=>{await bindCargoInvocationServices(owner,cargoSystemServices(root,60000));const value=await body(owner),manifests:CargoProbeManifest[]=[];let completed=0,ownedBytes=0;for(const generation of owner.generations){completed+=generation.completed;ownedBytes+=generation.ownedBytes;for(const record of generation.partialRecords)if(record&&typeof record==="object"&&record.constructor?.name==="PatternOwners"&&"alternatives" in record&&Array.isArray(record.alternatives)&&record.alternatives.length)patternCompiles++;for(const manifest of generation.manifestOwners)if(manifest.document!==null)manifests.push({path:manifest.path,source:manifest.pages.join(""),document:structuredClone(manifest.document)});}return{value,manifests,completed,ownedBytes,patternCompiles};});
}

/** 🧩️ Binds one exact test invocation to its real finite operation and invocation capabilities. */
export async function runCargoProbe<T>(root:string,body:(owner:CargoCliOperationOwner)=>Promise<T>):Promise<T>{return(await observeCargoProbe(root,body)).value;}

/** 🛠️ Executes explicit test preparation through the actual bound subprocess capability. */
export async function executeCargoProbePreparation(root:string,request:CargoPreparationSelection,owner:CargoCliOperationOwner):Promise<void>{const executor=new CargoPreparationExecutor(root,cargoCommandServices().preparation);owner.workspace.partialRecords.push(executor);await prepareCargoOwners(root,request,owner.operation,executor);}

/** 📣️ Publishes one preserved test roster through one actual lease and cumulative operation. */
export async function publishCargoProbeOwners(root:string):Promise<readonly string[]>{return runCargoProbe(root,async owner=>{const lease=await cargoCommandServices().leases.acquire(root,owner.operation);try{const scopes=await discoverCargoWorkspaces(root,owner.operation),drafts=[],published:string[]=[];for(const scope of scopes)drafts.push(await prepareCargoWorkspaceMembership(root,scope,owner.operation));for(const draft of drafts){await publishPreparedCargoMembership(root,draft,"write",{...owner.operation,lease});published.push(draft.path);}return published;}finally{await lease.release();}});}
