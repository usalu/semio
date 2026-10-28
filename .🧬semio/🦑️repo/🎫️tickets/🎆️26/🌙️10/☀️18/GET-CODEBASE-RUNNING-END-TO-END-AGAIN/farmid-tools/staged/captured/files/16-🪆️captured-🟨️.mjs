/** 🏷️ The only argument of a process that serves as a graph farm worker from the snapshot its parent runs from, instead of running a command. */
export const GRAPH_FARM_WORKER_ROLE_V1 = "--semio-role=graph-farm-worker";

const CAPTURED_IMPLEMENTATION_V1 = import.meta.semioCapturedImplementationIdentity ?? null;

/** 🪪️ The identity one process of the farm serves under: the digest of the captured implementation it runs from (by default the snapshot its own loader verified; `null` for a process that loaded its modules from their files) and the identity its project records are written under. */
export function graphFarmIdentityV1(records, implementation = CAPTURED_IMPLEMENTATION_V1) { return Object.freeze({ implementation: implementation ?? null, records }); }

/** 🔍️ Names the parts of an announced identity that differ from `own` (`implementation`, `records`, both, or `unreadable` for an announcement that is no identity); empty when the worker may serve. */
export function graphFarmIdentityDifferenceV1(own, offered) { return offered === null || typeof offered !== "object" ? "unreadable" : ["implementation", "records"].filter(part => offered[part] !== own[part]).join(", "); }

/** ⚖️ Whether a worker that announced `offered` may serve a parent of identity `own`: the session, the jobs and the replies are private to one implementation and the records it adopts must be the ones it would write, so both parts must be equal. */
export function sameGraphFarmIdentityV1(own, offered) { return graphFarmIdentityDifferenceV1(own, offered) === ""; }

/** 👷️ Serves as a graph farm worker inside a process started from a captured snapshot: the service and the worker loop are literal imports, so every snapshot that can start this role carries both, and the worker answers its parent over the channel it inherited. */
export async function runCapturedGraphFarmWorkerV1() { await import("../👷️service/🟨️.mjs"); await import("../👷️worker/🟨️.mjs"); }
