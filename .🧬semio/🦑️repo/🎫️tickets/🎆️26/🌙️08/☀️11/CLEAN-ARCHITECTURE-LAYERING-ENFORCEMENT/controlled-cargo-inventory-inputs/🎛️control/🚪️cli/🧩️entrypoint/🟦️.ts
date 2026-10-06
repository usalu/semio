import policy from "./🎛️policy/🔣️.json";
import {CargoCliOperationOwner,parseCargoCliPolicy} from "../🟦️.ts";
import {bindCargoInvocationServices,type CargoInvocationServices} from "../../../🏃️command/🧩️repository/🟦️.ts";
import {runCargoCliCommand} from "../🧩️scope/🟦️.ts";

/** 🚪️ Binds the authored repository command capability to actual progress, signals and awaited dispatch. */
export async function runCargoRepositoryOperation<T>(services:CargoInvocationServices,body:()=>Promise<T>):Promise<T>{
 const owner=new CargoCliOperationOwner(parseCargoCliPolicy(policy),{
  publish:row=>{process.stderr.write(`[cargo-operation] ${row.stage} ${row.completed} units ${row.ownedBytes} retained bytes ${row.path}\n`);},
  yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept)),
 });
 return runCargoCliCommand(owner,async()=>{await bindCargoInvocationServices(owner,services);return body();});
}
