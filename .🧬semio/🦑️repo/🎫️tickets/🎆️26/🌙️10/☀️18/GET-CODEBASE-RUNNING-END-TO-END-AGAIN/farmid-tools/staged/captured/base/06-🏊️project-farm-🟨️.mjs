import { mkdtempSync, realpathSync, rmSync } from "node:fs";
import { availableParallelism, tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { adoptStagedGraphLeavesV1 } from "../../../../../🏗️build/📥️inputs/🟨️.mjs";
import { physicalSourceOriginV1 } from "../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { requestBuildFarmCapacityV1 } from "../../../../../🏗️build/🌡️host-capacity/🟦️.ts";
import { choosePoolV1, createFarmV1, grantedPoolV1, ownGrantPoolV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "./🏊️pool/🟨️.mjs";

const ORIGIN = physicalSourceOriginV1(import.meta.url, import.meta.semioPhysicalSourceURL), ENTRY = fileURLToPath(new URL("./👷️worker/🟨️.mjs", ORIGIN)), SERVICE = new URL("./👷️service/🟨️.mjs", ORIGIN).href, REPORTED_FAILURES = 8;

/**
 * 🏊️ Plans the farm that pre-infers the projects of one graph construction in worker processes, or answers undefined when the work is small or the farm is switched off. Nothing is asked of the host before the work proves to be cold: the farm engages once at least `minimumProjects` projects had no valid record and they are a quarter of the projects seen so far, so a warm run costs nothing.
 * The farm then starts the worker the capacity grant of this process already pays for (so the parent, whose heap only grows over hundreds of projects, never infers the whole delta alone) and asks the host capacity governor for more workers in the fair machine-wide queue without waiting for the answer; any grant of at least one worker starts a pool of its own. Each worker runs the real project inference into a staging directory of its own; the parent adopts the finished leaves under its writer custody (`prepare`) and then reads the record exactly as it would read one a previous run left, so the farm changes how fast a record appears, never what it holds.
 */
export function openProjectFarmV1({ workspaceRoot, context, projects, commandInputs, lateClosures = false, projectsByRoot, componentRoots, environment = process.env, signal = undefined, processed = () => 0, command = undefined, commandArguments = undefined, spawnProcess = undefined, request = requestBuildFarmCapacityV1, cores = availableParallelism(), write = line => { process.stderr.write(line + "\n"); } }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) write(line); };
  const plan = planGraphFarmV1({ projects, environment, cores });
  if (plan.workers === 0) { if (plan.reason === "work" || plan.reason === "cpu") say("serial", { projects, reason: plan.reason }); return undefined; }
  const listeners = [], pools = [], counts = { adopted: 0, failed: 0, lost: 0, declined: 0, recycled: 0, leaves: 0, adoptMilliseconds: 0, misses: 0, lostJobs: 0, workerCpuMilliseconds: 0 };
  let state = "idle", started, closing = false, capacity, opening = Promise.resolve();
  const stop = (key, fields) => { state = "off"; say(key, fields); };
  const loops = () => pools.reduce((total, pool) => total + (pool.ready ? pool.workers * policy.loopsPerWorker : 0), 0);
  const start = ({ source, workers, megabytes }, release) => {
    const base = mkdtempSync(join(realpathSync(tmpdir()), "semio-graph-farm-")), pool = { source, workers, base, release, ready: false, farm: undefined };
    pool.farm = createFarmV1({
      entry: ENTRY, service: SERVICE, workers, megabytes, environment, signal,
      session: { version: 2, workspaceRoot, registry: context.registry ?? null, commandInputs: commandInputs ?? null, lateClosures, projectsByRoot: [...projectsByRoot], componentRoots, stagingBase: base },
      accept: identity => identity === inferenceRecordIdentityV1(),
      report: (event, fields) => {
        if (event === "recycled") { counts.recycled++; say("recycled", fields); }
        else if (event === "lost") { counts.lost++; say("lost", fields); }
        else if (event === "disabled") say(String(fields.reason).startsWith("identity") ? "refused" : "degraded", fields);
      },
      retired: (slot, generation) => { try { rmSync(join(base, "w" + slot + "-g" + generation), { recursive: true, force: true }); } catch {} },
      ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }), ...(spawnProcess === undefined ? {} : { spawnProcess }),
    });
    pools.push(pool);
    started ??= performance.now();
    say(source === "own" ? "own" : "starting", { workers, megabytes });
    pool.farm.ready.then(usable => { if (usable && state !== "off" && !closing) { pool.ready = true; for (const listener of listeners) listener(loops()); } });
    return pool;
  };
  const open = () => {
    state = "waiting";
    const own = ownGrantPoolV1(environment, policy);
    if (own !== undefined) start(own, () => {});
    say("waiting", { workers: plan.workers, projects });
    capacity = request({ preferredJobs: plan.workers, environment, signal, write });
    opening = capacity.granted.then(granted => {
      if (granted === undefined) return;
      if (closing) { capacity.release(); return; }
      const hosted = grantedPoolV1(granted, policy);
      if (hosted === undefined) { capacity.release(); if (pools.length === 0) stop("serial", { projects, reason: "host capacity" }); return; }
      start(hosted, capacity.release);
    }).catch(error => { if (pools.length === 0) stop("serial", { projects, reason: String(error?.message ?? error) }); });
  };
  return Object.freeze({
    /** 🔔️ Calls the listener with the loop count of the pools once a pool is ready and again whenever the farm grows. @param {(loops: number) => void} listener */
    onReady(listener) { listeners.push(listener); if (pools.some(pool => pool.ready)) listener(loops()); },
    async prepare(job) {
      if (state === "idle") { counts.misses++; if (counts.misses >= policy.minimumProjects && counts.misses * 10000 >= processed() * policy.missBasisPoints) open(); return false; }
      if (state === "off") return false;
      const at = choosePoolV1(pools.map(pool => ({ ready: pool.ready, usable: pool.farm.usable, idle: pool.farm.idle, pending: pool.farm.pending, width: pool.farm.width })));
      if (at < 0) return false;
      const pool = pools[at], outcome = await pool.farm.run(job, { signal });
      if (outcome.failure) { if (++counts.failed <= REPORTED_FAILURES) say("failed", { project: job.name, reason: outcome.failure.message.slice(0, 200) }); return false; }
      if (outcome.lost !== undefined) { counts.lostJobs++; (counts.lostReasons ??= {})[outcome.lost] = (counts.lostReasons[outcome.lost] ?? 0) + 1; return false; }
      const adopting = performance.now(), adoption = adoptStagedGraphLeavesV1(workspaceRoot, join(pool.base, "w" + outcome.slot + "-g" + outcome.generation), outcome.reply.names);
      counts.adoptMilliseconds += performance.now() - adopting;
      counts.leaves += outcome.reply.names.length; counts.workerCpuMilliseconds += outcome.reply.cpuMilliseconds;
      if (adoption.declined) { counts.declined++; say("declined", { project: job.name }); return false; }
      if (adoption.published + adoption.present === 0) return false;
      counts.adopted++;
      return true;
    },
    async close() {
      closing = true;
      if (pools.every(pool => pool.source === "own")) capacity?.release();
      await opening;
      if (pools.length === 0) return;
      const stats = pools.map(pool => pool.farm.stats());
      const failures = [];
      for (const pool of pools) { try { await pool.farm.close(); } catch (error) { failures.push(error); } finally { try { pool.release(); } finally { rmSync(pool.base, { recursive: true, force: true }); } } }
      say("finished", { adopted: counts.adopted, projects, workers: pools.reduce((total, pool) => total + pool.workers, 0), seconds: Math.round((performance.now() - started) / 1000), failed: counts.failed, lost: counts.lost, recycled: counts.recycled });
      if (environment[policy.phasesEnvironment] === "1") write("[graph-farm] " + JSON.stringify({ pools: stats, ...counts, adoptMilliseconds: Math.round(counts.adoptMilliseconds), projects, reason: plan.reason }));
      if (failures.length === 1) throw failures[0];
      if (failures.length > 1) throw new AggregateError(failures, "Closing the graph farm failed for " + failures.length + " pools");
    },
  });
}
