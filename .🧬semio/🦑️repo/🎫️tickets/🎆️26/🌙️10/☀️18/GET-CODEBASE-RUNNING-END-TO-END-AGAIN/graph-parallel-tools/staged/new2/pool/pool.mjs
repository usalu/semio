import { availableParallelism, totalmem } from "node:os";
import { fileURLToPath } from "node:url";
import { acquireBuildFarmCapacityV1 } from "../../🌡️host-capacity/🟦️.ts";
import { acquireGraphFarmV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "../🟨️.mjs";

const MEGABYTE = 1048576, ENTRY = fileURLToPath(new URL("../👷️worker/🟨️.mjs", import.meta.url));

/** 🗣️ Writes one farm message to stderr in the languages of the environment. */
export function sayGraphFarmV1(key, fields, environment = process.env) { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) process.stderr.write(line + "\n"); }

/** 🏊️ Plans the worker pool of one graph phase and answers a handle whose `open()` asks the machine-wide gate for the processes and leases the resident pool of the workspace (the first phase creates it, later phases and later builds of this process share its booted workers); `prestart` boots the workers at once, so they load while this process still prepares its session. A handle without workers (switched off, small work) opens to a refusal and the caller infers everything itself. */
export function startGraphPoolV1({ workspaceRoot, projects, services = {}, prestart = false, environment = process.env, signal, command, commandArguments, spawnProcess, say = sayGraphFarmV1 }) {
  const policy = GRAPH_FARM_POLICY_V1;
  const plan = { ...planGraphFarmV1({ projects, environment, host: { cores: availableParallelism(), totalMegabytes: Math.floor(Math.min(totalmem(), process.constrainedMemory?.() || Infinity) / MEGABYTE) }, parentMegabytes: process.memoryUsage().rss / MEGABYTE }) };
  if (plan.workers === 0) return Object.freeze({ plan, open: async () => ({ refused: plan.reason }), release() {} });
  let opening;
  const open = () => opening ??= (async () => {
    const capacity = await acquireBuildFarmCapacityV1({ preferredJobs: plan.workers, waitMilliseconds: policy.capacityWaitMilliseconds, environment });
    if (capacity === undefined || capacity.jobs < (plan.reason === "forced" ? 1 : 2)) { capacity?.release(); return { refused: capacity === undefined ? "no host capacity in time" : "host capacity" }; }
    const workers = Math.min(plan.workers, capacity.jobs);
    const report = (event, fields) => { if (event === "recycled") say("recycled", fields, environment); else if (event === "disabled") say("disabled", { reason: fields.reason }, environment); };
    const lease = acquireGraphFarmV1({ key: [workspaceRoot, plan.workerMegabytes].join("\0"), entry: ENTRY, services, workers, environment, signal, workerMegabytes: plan.workerMegabytes, report, ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }), ...(spawnProcess === undefined ? {} : { spawnProcess }) });
    const granted = Math.min(workers, lease.farm.width);
    lease.farm.limit(granted);
    if (prestart) lease.farm.prestart();
    let released = false;
    const release = () => { if (released) return; released = true; lease.release(); capacity.release(); };
    signal?.addEventListener("abort", release, { once: true });
    return { farm: lease.farm, workers: granted, release };
  })().catch(error => ({ refused: String(error?.message ?? error) }));
  if (prestart) void open();
  return Object.freeze({ plan, open, async release() { const opened = opening === undefined ? undefined : await opening; opened?.release?.(); } });
}
