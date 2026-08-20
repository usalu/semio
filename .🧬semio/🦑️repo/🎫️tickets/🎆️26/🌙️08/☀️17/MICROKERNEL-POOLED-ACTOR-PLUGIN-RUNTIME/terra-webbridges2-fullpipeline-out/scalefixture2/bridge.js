/** @generated semio actor jco component bridge */
import * as hostShim from "./🟨️host-shim.js";
const { reactor, jobs, checkpoint, describe } = await import("./scalefixture2.js");

export async function createActorApi(actorId) {
  hostShim.__bindHostBridge(actorId);
  return {
    poll: async (events, budget) => reactor.poll(events, budget),
    startJob: async (job, kind, input) => jobs.startJob(job, kind, input),
    stepJob: async (job, budget) => jobs.stepJob(job, budget),
    cancelJob: async (job) => jobs.cancelJob(job),
    checkpoint: async () => checkpoint.checkpoint(),
    restore: async (state) => checkpoint.restore(state),
    describe: async () => describe.describe(),
    resolveEffect: (requestId, value) => hostShim.__resolveEffect(requestId, value),
    rejectEffect: (requestId, message) => hostShim.__rejectEffect(requestId, message),
  };
}
