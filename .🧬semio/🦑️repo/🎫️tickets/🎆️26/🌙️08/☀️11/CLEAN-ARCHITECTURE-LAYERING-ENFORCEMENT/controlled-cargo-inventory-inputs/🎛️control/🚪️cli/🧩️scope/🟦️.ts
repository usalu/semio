import {AsyncLocalStorage} from "node:async_hooks";
import {CargoCliOperationOwner} from "../🟦️.ts";
import type {CargoDiscoveryOperation} from "../../../📁️physical/🟦️.ts";
class CommandScope {active=true;constructor(readonly owner:CargoCliOperationOwner){} }
const scopes=new AsyncLocalStorage<CommandScope>();
export class CargoCommandFailure extends Error {
 constructor(readonly owner:CargoCliOperationOwner,override readonly cause:unknown,readonly cleanupError:unknown){super("Cargo command refused with retained ownership"+(cause instanceof Error?": "+cause.message:""));}
}

/** 🎛️ Requires the explicitly admitted live command owner without creating a fallback capability. */
export function cargoCommandOwner():CargoCliOperationOwner{const scope=scopes.getStore();if(!scope||!scope.active)throw Error("Cargo command requires its explicit admitted operation scope");return scope.owner;}

/** 🧭️ Binds canonical API calls to their exact active physical discovery view. */
export function cargoCommandOperation():CargoDiscoveryOperation{const owner=cargoCommandOwner();if(owner.workspace.state!=="active")throw Error("Cargo command requires an explicit fresh discovery view");return owner.operation;}

/** 🧩️ Preserves nested and concurrent owner identity across awaited continuations. */
export async function runCargoCommandScope<T>(owner:CargoCliOperationOwner,body:()=>Promise<T>):Promise<T>{const scope=new CommandScope(owner);try{return await scopes.run(scope,body);}finally{scope.active=false;}}

/** 🚪️ Retains command refusal and cleanup ownership while closing real command capabilities. */
export async function runCargoCliCommand<T>(owner:CargoCliOperationOwner,body:()=>Promise<T>):Promise<T>{let value:T|undefined,cause:unknown=null,cleanupError:unknown=null,refused=false;try{value=await runCargoCommandScope(owner,body);}catch(error){refused=true;cause=error;}owner.cancelDiscovery();try{await owner.closeSystemOwners();}catch(error){cleanupError=error;owner.detachSignals();}if(refused||cleanupError!==null)throw new CargoCommandFailure(owner,cause,cleanupError);return value!;}
