import { availableParallelism, totalmem } from "node:os";
import { fileURLToPath } from "node:url";
import { acquireBuildFarmCapacityV1 } from "../../../../../🏗️build/🌡️host-capacity/🟦️.ts";
import { createFarmV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "../../../../../🏗️build/🏊️farm/🟨️.mjs";
import { missingSourceAnalysisRecordsV1, putSourceAnalysisRecordsV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";

const MEGABYTE = 1048576, BATCH = 64, FILES_PER_UNIT = 250, PROBE_FILES = 64, ENTRY = fileURLToPath(new URL("../../../../../🏗️build/🏊️farm/👷️worker/🟨️.mjs", import.meta.url)), SERVICE = new URL("./👷️service/🟨️.mjs", import.meta.url).href;

/** 🏊️ Prepares the parser records of the source files whose analysis is not stored yet, in worker processes, so the dependency pass that follows finds every record in the store and parses nothing; answers the number of records prepared (0 when the work is small, the farm is switched off or fewer than two workers fit). */
export async function prefillModuleAnalysesV1({ workspaceRoot, files, provider, signal, environment = process.env, command, commandArguments, spawnProcess }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) process.stderr.write(line + "\n"); };
  const plan = { ...planGraphFarmV1({ projects: Math.floor(files.length / FILES_PER_UNIT), environment, host: { cores: availableParallelism(), totalMegabytes: Math.floor(Math.min(totalmem(), process.constrainedMemory?.() || Infinity) / MEGABYTE) }, parentMegabytes: process.memoryUsage().rss / MEGABYTE }) };
  if (plan.workers === 0) return 0;
  const stride = Math.max(1, Math.ceil(files.length / PROBE_FILES));
  if (plan.reason !== "forced" && missingSourceAnalysisRecordsV1(provider, files.filter((_, index) => index % stride === 0)).length === 0) return 0;
  const missing = missingSourceAnalysisRecordsV1(provider, files);
  if (missing.length === 0 || plan.reason !== "forced" && missing.length < FILES_PER_UNIT * policy.minimumProjects) return 0;
  const capacity = await acquireBuildFarmCapacityV1({ preferredJobs: plan.workers, waitMilliseconds: policy.capacityWaitMilliseconds, environment });
  if (capacity === undefined || capacity.jobs < (plan.reason === "forced" ? 1 : 2)) { capacity?.release(); return 0; }
  plan.workers = Math.min(plan.workers, capacity.jobs);
  const farm = createFarmV1({ entry: ENTRY, service: SERVICE, session: { version: 1, workspaceRoot, identity: provider.revision }, workers: plan.workers, environment, signal, workerMegabytes: plan.workerMegabytes, accept: actual => actual === provider.revision, ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }), ...(spawnProcess === undefined ? {} : { spawnProcess }) });
  say("starting", { projects: missing.length, workers: plan.workers, megabytes: plan.workerMegabytes });
  const started = performance.now();
  let prepared = 0;
  try {
    const batches = [];
    for (let at = 0; at < missing.length; at += BATCH) batches.push(missing.slice(at, at + BATCH));
    await Promise.all(batches.map(async batch => {
      const outcome = await farm.infer({ files: batch });
      if (!outcome?.reply) return;
      prepared += putSourceAnalysisRecordsV1(outcome.reply.records.map(row => [row.key, row.bytes]));
    }));
  } finally { await farm.close(); capacity.release(); }
  const stats = farm.stats();
  say("finished", { projects: missing.length, workers: plan.workers, seconds: Math.round((performance.now() - started) / 1000), jobs: stats.jobs, fallbacks: stats.fallbacks, recycled: stats.recycled });
  return prepared;
}
