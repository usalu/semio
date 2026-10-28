/** 🚏️ Says which phase the run of this process enters (`assembly`, `plan`, `rebuild`, `execution`), so the host capacity gate can cut the run into phases, re-size its reservation at the boundary and learn each phase under its own class; a process that runs under no grant, or under an inherited one, says nothing and costs nothing (the gate module loads only when a grant exists). */
export async function markBuildCapacityPhaseV1(phase:"assembly"|"plan"|"rebuild"|"execution",environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined||environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 return(await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts")).markHostCapacityPhaseV1(environment,phase);
}
