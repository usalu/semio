import {createRequire} from "node:module";
import buildPolicy from "../../🔣️policy.json" with {type:"json"};
import {readHostCapacityPolicyV1,type HostCapacityPolicyV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 📖️ The cancellation texts load when a wait is cancelled, never at startup: the leases, capacity and doctor entries stay inside their module budgets. */
export const refuseCapacityV1=(...input:Parameters<typeof import("../📖️texts/🟦️.ts").refuseCapacityV1>):ReturnType<typeof import("../📖️texts/🟦️.ts").refuseCapacityV1>=>(createRequire(import.meta.url)("../📖️texts/🟦️.ts") as typeof import("../📖️texts/🟦️.ts")).refuseCapacityV1(...input);

/** 🌡️ The authored host capacity policy of the neutral build policy. */
export const BUILD_HOST_CAPACITY_POLICY_V1:HostCapacityPolicyV1=readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity);

/** 🛑️ A control for the wait before any invocation exists: it is cancelled by an interrupt or termination signal and has no deadline of its own, so the invocation's budget starts when the grant does. */
export function createCapacityControlV1(process_:Pick<NodeJS.Process,"once"|"removeListener">=process):Readonly<{control:ResourceLeaseControl;dispose:()=>void}>{
 const controller=new AbortController(),handlers=(["SIGINT","SIGTERM"] as const).map(name=>[name,()=>controller.abort(refuseCapacityV1("cancelled","wait-cancelled",{signal:name}))] as const);
 for(const [name,handler] of handlers)process_.once(name,handler);
 return Object.freeze({control:Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))}),dispose:()=>{for(const [name,handler] of handlers)process_.removeListener(name,handler);}});
}
