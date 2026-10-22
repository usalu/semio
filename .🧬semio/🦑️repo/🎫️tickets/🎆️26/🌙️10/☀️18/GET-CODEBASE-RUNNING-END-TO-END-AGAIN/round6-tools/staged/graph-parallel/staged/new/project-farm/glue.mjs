import { availableParallelism, totalmem } from "node:os";
import { fileURLToPath } from "node:url";
import { adoptStagedGraphFilesV1 } from "../../../../../🏗️build/📥️inputs/🟨️.mjs";
import { canonicalObservationRowV1, currentProjectRecordDigestV1, exportSourceAnalysisRecordsV1, preimagedProjectValueV1, replayDetachedProjectInferenceV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { acquireBuildFarmCapacityV1 } from "../../../../../🏗️build/🌡️host-capacity/🟦️.ts";
import { createFarmV1, createInternDecoderV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "../../../../../🏗️build/🏊️farm/🟨️.mjs";

const MEGABYTE = 1048576, ENTRY = fileURLToPath(new URL("../../../../../🏗️build/🏊️farm/👷️worker/🟨️.mjs", import.meta.url)), SERVICE = new URL("./👷️service/🟨️.mjs", import.meta.url).href;

/** 🏊️ Plans the farm that infers the projects of one graph construction in worker processes, or answers undefined when the work is small, the farm is switched off or fewer than two workers fit: the caller then infers every project itself, exactly as before. Nothing is asked of the host and no process starts before the first project that is not stored yet needs a worker. */
export async function openProjectFarmV1({ workspaceRoot, context, jobs, identity, commandInputs, projectsByRoot, componentRoots, readRoot, environment = process.env, command, commandArguments, spawnProcess }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) process.stderr.write(line + "\n"); };
  const plan = { ...planGraphFarmV1({ projects: jobs, environment, host: { cores: availableParallelism(), totalMegabytes: Math.floor(Math.min(totalmem(), process.constrainedMemory?.() || Infinity) / MEGABYTE) }, parentMegabytes: process.memoryUsage().rss / MEGABYTE }) };
  if (plan.workers === 0) { if (plan.reason === "single") say("serial", { projects: jobs, reason: plan.reason }); return undefined; }
  const traffic = { replies: 0, bodies: 0, aliases: 0, files: 0, fileBytes: 0, records: 0, recordBytes: 0, rowIds: 0, learnMs: 0, replayMs: 0 }, width = plan.workers, tables = new Map(), byDigest = new Map(), log = [], blockLog = [], knownBlocks = new Set(), cursors = new Map(), adopted = new Set(), seedBytes = new Map(), session = { version: 1, workspaceRoot, identity, registry: context.registry ?? null, commandInputs: commandInputs ?? null, projectsByRoot: [...projectsByRoot], componentRoots, readRoot };
  let opening;
  const open = () => opening ??= (async () => {
    const capacity = await acquireBuildFarmCapacityV1({ preferredJobs: plan.workers, waitMilliseconds: policy.capacityWaitMilliseconds, environment });
    if (capacity === undefined || capacity.jobs < (plan.reason === "forced" ? 1 : 2)) { capacity?.release(); say("serial", { projects: jobs, reason: capacity === undefined ? "no host capacity in time" : "host capacity" }); return undefined; }
    plan.workers = Math.min(plan.workers, capacity.jobs);
    session.seed = exportSourceAnalysisRecordsV1();
    for (const [key, bytes] of session.seed) seedBytes.set(key, bytes);
    const farm = createFarmV1({ entry: ENTRY, service: SERVICE, session, workers: plan.workers, environment, signal: context.invocation?.control.signal, workerMegabytes: plan.workerMegabytes, accept: actual => actual === identity, report: (event, fields) => { if (event === "recycled") say("recycled", fields); else if (event === "disabled") say("fallback", { count: 0, reason: fields.reason }); }, ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }), ...(spawnProcess === undefined ? {} : { spawnProcess }) });
    say("starting", { projects: jobs, workers: plan.workers, megabytes: plan.workerMegabytes });
    return { farm, capacity, started: performance.now() };
  })().catch(error => { say("serial", { projects: jobs, reason: String(error?.message ?? error) }); return undefined; });
  const tableOf = worker => { let table = tables.get(worker); if (!table) tables.set(worker, table = { rows: createInternDecoderV1(), records: createInternDecoderV1({ canonical: item => item }), recordBytes: 0 }); return table; };
  return Object.freeze({
    width,
    async infer(job) {
      const opened = await open();
      if (opened === undefined) return undefined;
      const digest = currentProjectRecordDigestV1();
      const outcome = await opened.farm.infer(({ slot, generation }) => {
        const cursor = cursors.get(slot), current = cursor?.generation === generation;
        cursors.set(slot, { generation, length: log.length, blocks: blockLog.length });
        return { ...job, digest, known: log.slice(current ? cursor.length : 0), knownBlocks: blockLog.slice(current ? cursor.blocks : 0) };
      });
      if (!outcome?.reply) return undefined;
      const reply = outcome.reply, table = tableOf(reply.worker), learning = performance.now();
      traffic.replies++; traffic.rowIds += reply.rowIds.length; for (const fresh of reply.freshRows) fresh[2] === undefined ? traffic.aliases++ : traffic.bodies++; traffic.files += reply.files.length; for (const file of reply.files) traffic.fileBytes += file.bytes.byteLength; traffic.records += reply.freshRecords.length;
      table.rows.learn(reply.freshRows.map(([id, digest, row]) => {
        let known = byDigest.get(digest);
        if (known === undefined) {
          if (row === undefined) throw Error("Known fact digest required: " + digest);
          byDigest.set(digest, known = canonicalObservationRowV1(row));
          log.push(digest);
        }
        return [id, known];
      }));
      table.records.learn(reply.freshRecords.map(([id, row]) => { if (row.seeded) return [id, { key: row.key, bytes: seedBytes.get(row.key) }]; if (row.bytes === undefined || table.recordBytes + row.bytes.byteLength > policy.maximumRecordBytes) return [id, { key: row.key, bytes: undefined }]; table.recordBytes += row.bytes.byteLength; return [id, row]; }));
      for (const block of reply.sealed.blockDigests) if (!knownBlocks.has(block)) { knownBlocks.add(block); blockLog.push(block); }
      traffic.learnMs += performance.now() - learning;
      return reply;
    },
    replay(reply) {
      const replaying = performance.now();
      const table = tableOf(reply.worker);
      const files = reply.files.filter(file => !adopted.has(file.name));
      adoptStagedGraphFilesV1(workspaceRoot, files);
      for (const file of files) adopted.add(file.name);
      replayDetachedProjectInferenceV1(table.rows.decode(reply.rowIds), table.records.decode(reply.putIds).flatMap(row => row.bytes === undefined ? [] : [[row.key, row.bytes]]), reply.sealed);
      traffic.replayMs += performance.now() - replaying;
      return preimagedProjectValueV1(reply.image);
    },
    async close() {
      const opened = opening === undefined ? undefined : await opening;
      if (opened === undefined) return;
      await opened.farm.close();
      opened.capacity.release();
      const stats = opened.farm.stats();
      say("finished", { projects: jobs, workers: plan.workers, seconds: Math.round((performance.now() - opened.started) / 1000), jobs: stats.jobs, fallbacks: stats.fallbacks, recycled: stats.recycled });
      if (environment[policy.phasesEnvironment] === "1") process.stderr.write("[graph-farm] " + JSON.stringify({ ...stats, projects: jobs, reason: plan.reason, ...traffic, distinctRows: log.length, learnMs: Math.round(traffic.learnMs), replayMs: Math.round(traffic.replayMs) }) + "\n");
    },
  });
}
