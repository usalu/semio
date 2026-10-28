/** 🚏️ Says which phase the run of this process enters (`assembly`, `plan`, `rebuild`, `execution`), so the host capacity gate can cut the run into phases, re-size its reservation at the boundary and learn each phase under its own class; a process that runs under no grant says nothing and costs nothing (the gate module loads only when a grant exists), one under an inherited grant (phase marks off) marks nothing. Entering `rebuild` first grows a quiet forecast grant into its full request in every process of the run, the inheriting ones included (a graph rebuild is work that has to run), so the rebuild is booked only after it fits. */
export async function markBuildCapacityPhaseV1(phase:"assembly"|"plan"|"rebuild"|"execution",environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return false;
 if(phase==="rebuild")await(await import("../🟦️.ts")).upgradeBuildCapacityV1(undefined,environment);
 if(environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 return(await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts")).markHostCapacityPhaseV1(environment,phase);
}

/** ⏫️ Grows a quiet forecast grant into its full request BEFORE the graph builder custody is taken: the wait for capacity is a fair queue with no timer, so it must not happen while a custody is held that every other builder queues for, and not while other runs hold full grants and wait for that custody (each would wait for the other). Every process of the run asks, the one that only inherits the grant included (an inherited quiet grant is not grown when it is inherited, so the rebuild is where it grows); answers whether a live grant was asked, a process under no grant asks for nothing. */
export async function upgradeBuildCapacityBeforeRebuildV1(environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return false;
 return(await(await import("../🟦️.ts")).upgradeBuildCapacityV1(undefined,environment))!=="absent";
}
