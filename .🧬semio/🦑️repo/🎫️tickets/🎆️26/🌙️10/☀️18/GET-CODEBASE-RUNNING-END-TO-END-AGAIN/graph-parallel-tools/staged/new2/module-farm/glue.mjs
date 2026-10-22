import { GRAPH_FARM_POLICY_V1, renderGraphFarmMessagesV1 } from "../../../../../🏗️build/🏊️farm/🟨️.mjs";
import { sayGraphFarmV1, startGraphPoolV1 } from "../../../../../🏗️build/🏊️farm/🧰️pool/🟨️.mjs";
import { missingSourceAnalysisRecordsV1, putSourceAnalysisRecordsV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";

const BATCH = 64, FILES_PER_UNIT = 250, PROBE_FILES = 64, SERVICE = new URL("./👷️service/🟨️.mjs", import.meta.url).href;

/** 🏊️ Prepares the parser records of the source files whose analysis is not stored yet, in worker processes, so the dependency pass that follows finds every record in the store and parses nothing; answers the number of records prepared (0 when the work is small, the farm is switched off or fewer than two workers fit). */
export async function prefillModuleAnalysesV1({ workspaceRoot, files, provider, signal, environment = process.env, command, commandArguments, spawnProcess }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => sayGraphFarmV1(key, fields, environment);
  const pool = startGraphPoolV1({ workspaceRoot, projects: Math.floor(files.length / FILES_PER_UNIT), services: { module: SERVICE }, environment, signal, command, commandArguments, spawnProcess });
  if (pool.plan.workers === 0) return 0;
  const stride = Math.max(1, Math.ceil(files.length / PROBE_FILES)), forced = pool.plan.reason === "forced";
  if (!forced && missingSourceAnalysisRecordsV1(provider, files.filter((_, index) => index % stride === 0)).length === 0) return 0;
  const missing = missingSourceAnalysisRecordsV1(provider, files);
  if (missing.length === 0 || !forced && missing.length < FILES_PER_UNIT * policy.minimumProjects) return 0;
  const opened = await pool.open();
  if (opened.refused !== undefined) return 0;
  opened.farm.open("module", { version: 1, workspaceRoot, identity: provider.revision }, { accept: (_kind, actual) => actual === provider.revision, service: SERVICE });
  say("starting", { projects: missing.length, workers: opened.workers, megabytes: pool.plan.workerMegabytes });
  const started = performance.now(), before = opened.farm.stats();
  let prepared = 0, fallbacks = 0;
  try {
    const batches = [];
    for (let at = 0; at < missing.length; at += BATCH) batches.push(missing.slice(at, at + BATCH));
    await Promise.all(batches.map(async batch => {
      const outcome = await opened.farm.infer("module", { files: batch });
      if (!outcome.reply) {
        const fields = { project: batch.length + " source files from " + batch[0], reason: outcome.fallback ?? "failure: " + outcome.failure.message, count: ++fallbacks, maximum: policy.maximumFallbacks };
        if (fallbacks > policy.maximumFallbacks) throw Error(renderGraphFarmMessagesV1("fallbackLimit", fields, environment).join(" | "));
        say("fallback", fields);
        return;
      }
      prepared += putSourceAnalysisRecordsV1(outcome.reply.records.map(row => [row.key, row.bytes]));
    }));
  } finally { opened.release(); }
  const stats = opened.farm.stats();
  say("finished", { projects: missing.length, workers: opened.workers, seconds: Math.round((performance.now() - started) / 1000), jobs: stats.jobs - before.jobs, fallbacks: stats.fallbacks, recycled: stats.recycled });
  return prepared;
}
