import {CargoController} from "../../🎛️control/🟦️.ts";
import {cargoCommandOwner,cargoCommandOperation} from "../../🎛️control/🚪️cli/🧩️scope/🟦️.ts";
import type {CargoCliOperationOwner} from "../../🎛️control/🚪️cli/🟦️.ts";
import {prepareCargoWorkspaceInvocation,type CargoInvocationLeasePort,type CargoInvocationEnvironment} from "../../🛠️preparation/🏃️invocation/🟦️.ts";
import {CargoPreparationExecutor,type CargoPreparationProcessPort} from "../../🛠️preparation/🏃️execution/🟦️.ts";
import {selectedCargoArguments} from "../../🎛️arguments/🟦️.ts";
import {executeCargoCommand,type CargoExecutionPort} from "../🟦️.ts";

export interface CargoInvocationServices {readonly leases:CargoInvocationLeasePort;readonly preparation:CargoPreparationProcessPort;readonly execution:CargoExecutionPort;}
const bindings=new WeakMap<CargoCliOperationOwner,CargoInvocationServices>();

/** 🔌️ Returns only capabilities admitted to the exact active command owner. */
export function cargoCommandServices():CargoInvocationServices{const owner=cargoCommandOwner(),services=bindings.get(owner);if(!services)throw Error("Cargo command requires its admitted invocation services");return services;}

/** 🔌️ Admits required first-party invocation capabilities under their exact command owner. */
export async function bindCargoInvocationServices(owner:CargoCliOperationOwner,services:CargoInvocationServices):Promise<void>{if(bindings.has(owner))throw Error("Cargo invocation services are already bound");await new CargoController(owner.operation,owner.workspace).step("invocation-services-owner","",1,128);owner.workspace.partialRecords.push(services);bindings.set(owner,services);}

/** 🛠️ Requires the command's admitted capabilities before preparing any physical source. */
export async function prepareRepositoryCargoCommand(root:string,args:readonly string[],cwd:string,environment:CargoInvocationEnvironment):Promise<void>{const owner=cargoCommandOwner(),services=cargoCommandServices();const operation=cargoCommandOperation();await new CargoController(operation,operation.workspace).step("preparation-executor-owner",root,1,128);const executor=new CargoPreparationExecutor(root,services.preparation);operation.workspace.partialRecords.push(executor);await prepareCargoWorkspaceInvocation(root,args,cwd,environment,owner,services.leases,executor);}

/** 🏃️ Runs exact selectors through one prepared command and its actual retained execution capability. */
export async function runRepositoryCargoStatus(root:string,args:readonly string[],cwd:string,environment:CargoInvocationEnvironment):Promise<number>{const services=cargoCommandServices();const selected=await selectedCargoArguments(root,args,cargoCommandOperation());await prepareRepositoryCargoCommand(root,selected,cwd,environment);const result=await executeCargoCommand({arguments:selected,directory:cwd,environment},cargoCommandOperation(),services.execution);return result.code;}

/** ✅️ Requires actual successful Cargo termination after awaited source preparation and execution. */
export async function runRepositoryCargoCommand(root:string,args:readonly string[],cwd:string,environment:CargoInvocationEnvironment):Promise<void>{const code=await runRepositoryCargoStatus(root,args,cwd,environment);if(code!==0)throw Error("Cargo command exited with status "+code);}
