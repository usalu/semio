import { readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";
import { graphWorkerRootsV1 } from "../../../../../🏗️build/📥️inputs/🟨️.mjs";
import { adoptBuildGraphWorkerLeavesV1, buildGraphWriterPresenceV1, deferBuildGraphReceiptV1, grantBuildGraphWorkerBytesV1 } from "../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import { canonicalObservationRowV1, currentProjectRecordDigestV1, currentProjectRecordDirectoryV1, exportSourceAnalysisRecordsV1, preimagedProjectValueV1, replayDetachedProjectInferenceV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { createInternDecoderV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "../../../../../🏗️build/🏊️farm/🟨️.mjs";
import { sayGraphFarmV1, startGraphPoolV1 } from "../../../../../🏗️build/🏊️farm/🧰️pool/🟨️.mjs";

const MEGABYTE = 1048576, SERVICE = new URL("./👷️service/🟨️.mjs", import.meta.url).href, BLOCK = /(?:^|\/)ob\/[a-f0-9]{64}\.br$/u;

/** 🧊️ Whether the record store of this graph is mostly empty, so the workers are worth booting before the first project is asked for. */
function likelyColdV1(readRoot, projects) {
  let stored = 0;
  try { for (const name of readdirSync(join(readRoot, "np"))) if (name.endsWith(".json")) stored++; } catch {}
  return stored * 10000 < projects * GRAPH_FARM_POLICY_V1.prestartColdBasisPoints;
}

/** 🚀️ Plans the worker pool of the project inference at the start of a graph construction and, when the record store is mostly empty, boots the workers at once so they load while this process still discovers the owners; the handle goes to `openProjectFarmV1`. */
export function startProjectFarmV1({ workspaceRoot, projects, readRoot, environment = process.env, signal, command, commandArguments, spawnProcess }) {
  return startGraphPoolV1({ workspaceRoot, projects, services: { project: SERVICE }, prestart: likelyColdV1(readRoot, projects), environment, signal, command, commandArguments, spawnProcess });
}

/** 🏊️ Opens the farm that infers the projects of one graph construction in worker processes, or answers undefined when the work is small, the farm is switched off or fewer than two workers fit: the caller then infers every project itself, exactly as before. Nothing is asked of the host and no process starts before the first project that is not stored yet needs a worker, unless `started` already booted the pool. */
export async function openProjectFarmV1({ workspaceRoot, context, jobs, identity, commandInputs, projectsByRoot, componentRoots, readRoot, started, environment = process.env, command, commandArguments, spawnProcess }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => sayGraphFarmV1(key, fields, environment), pool = started ?? startGraphPoolV1({ workspaceRoot, projects: jobs, services: { project: SERVICE }, environment, signal: context.invocation?.control.signal, command, commandArguments, spawnProcess });
  const plan = pool.plan;
  if (plan.workers === 0) { if (plan.reason === "single") say("serial", { projects: jobs, reason: plan.reason }); return undefined; }
  const grants = new WeakMap(), outstanding = new Set(), traffic = { fallbacks: 0, replies: 0, bodies: 0, aliases: 0, leaves: 0, leafBytes: 0, declined: 0, receipts: 0, records: 0, rowIds: 0, learnMs: 0, replayMs: 0 }, tables = new Map(), byDigest = new Map(), log = [], blockLog = [], knownBlocks = new Set(), cursors = new Map(), seedBytes = new Map(), session = { version: 1, workspaceRoot, identity, registry: context.registry ?? null, commandInputs: commandInputs ?? null, projectsByRoot: [...projectsByRoot], componentRoots, readRoot };
  let opening, width = plan.workers, recordTotal = 0;
  const open = () => opening ??= (async () => {
    const opened = await pool.open();
    if (opened.refused !== undefined) { say("serial", { projects: jobs, reason: opened.refused }); return undefined; }
    width = opened.workers;
    session.seed = exportSourceAnalysisRecordsV1(policy.seedMegabytes * MEGABYTE);
    for (const [key, bytes] of session.seed) seedBytes.set(key, bytes);
    session.roots = graphWorkerRootsV1(workspaceRoot);
    opened.farm.open("project", session, { accept: (_kind, actual) => actual === identity, service: SERVICE });
    say("starting", { projects: jobs, workers: width, megabytes: plan.workerMegabytes });
    return { farm: opened.farm, release: opened.release, started: performance.now(), jobs: opened.farm.stats().jobs };
  })().catch(error => { say("serial", { projects: jobs, reason: String(error?.message ?? error) }); return undefined; });
  const tableOf = worker => { let table = tables.get(worker); if (!table) tables.set(worker, table = { rows: createInternDecoderV1(), records: createInternDecoderV1({ canonical: item => item }) }); return table; };
  const adopt = reply => {
    const adoption = adoptBuildGraphWorkerLeavesV1(workspaceRoot, reply.leaves), presence = buildGraphWriterPresenceV1();
    for (const name of adoption.accepted) if (presence !== undefined && BLOCK.test(name)) presence.add(join(readRoot, name));
    traffic.leaves += adoption.accepted.length; traffic.declined += adoption.refused.length; for (const leaf of reply.leaves) traffic.leafBytes += leaf.bytes;
  };
  const fallback = (name, reason) => {
    traffic.fallbacks++;
    const fields = { project: name, reason, count: traffic.fallbacks, maximum: policy.maximumFallbacks };
    if (traffic.fallbacks > policy.maximumFallbacks) throw Error(renderGraphFarmMessagesV1("fallbackLimit", fields, environment).join(" | "));
    say("fallback", fields);
  };
  return Object.freeze({
    get width() { return width; },
    async infer(job) {
      const opened = await open();
      if (opened === undefined) return undefined;
      const digest = currentProjectRecordDigestV1(), directory = currentProjectRecordDirectoryV1(), release = grantBuildGraphWorkerBytesV1(workspaceRoot, policy.grantBytes);
      let outcome;
      try {
        outcome = await opened.farm.infer("project", ({ slot, generation }) => {
          const cursor = cursors.get(slot), current = cursor?.generation === generation;
          cursors.set(slot, { generation, length: log.length, blocks: blockLog.length });
          return { ...job, digest, directory, grantBytes: release === undefined ? 0 : policy.grantBytes, known: log.slice(current ? cursor.length : 0), knownBlocks: blockLog.slice(current ? cursor.blocks : 0) };
        });
      } catch (error) { release?.(); throw error; }
      if (!outcome.reply) { release?.(); fallback(job.name, outcome.fallback ?? "failure: " + outcome.failure.message); return undefined; }
      const reply = outcome.reply, table = tableOf(reply.worker), learning = performance.now();
      if (release !== undefined) { grants.set(reply, release); outstanding.add(release); }
      traffic.replies++; traffic.rowIds += reply.rowIds.length; for (const fresh of reply.freshRows) fresh[2] === undefined ? traffic.aliases++ : traffic.bodies++; traffic.records += reply.freshRecords.length;
      table.rows.learn(reply.freshRows.map(([id, digest, row]) => {
        let known = byDigest.get(digest);
        if (known === undefined) {
          if (row === undefined) throw Error("Known fact digest required: " + digest);
          byDigest.set(digest, known = canonicalObservationRowV1(row));
          log.push(digest);
        }
        return [id, known];
      }));
      table.records.learn(reply.freshRecords.map(([id, row]) => { if (row.seeded) return [id, { key: row.key, bytes: seedBytes.get(row.key) }]; if (row.bytes === undefined || recordTotal + row.bytes.byteLength > policy.recordTableMegabytes * MEGABYTE) return [id, { key: row.key, bytes: undefined }]; recordTotal += row.bytes.byteLength; return [id, row]; }));
      if (reply.sealed !== undefined) for (const block of reply.sealed.blockDigests) if (!knownBlocks.has(block)) { knownBlocks.add(block); blockLog.push(block); }
      traffic.learnMs += performance.now() - learning;
      return reply;
    },
    replay(reply) {
      const replaying = performance.now();
      const table = tableOf(reply.worker);
      adopt(reply);
      const release = grants.get(reply);
      if (release !== undefined) { outstanding.delete(release); release(); }
      for (const manifest of reply.receipts) deferBuildGraphReceiptV1(manifest);
      replayDetachedProjectInferenceV1(table.rows.decode(reply.rowIds), table.records.decode(reply.putIds).flatMap(row => row.bytes === undefined ? [] : [[row.key, row.bytes]]), reply.sealed);
      traffic.replayMs += performance.now() - replaying; traffic.receipts += reply.receipts.length;
      return preimagedProjectValueV1(reply.image);
    },
    async close() {
      const opened = opening === undefined ? undefined : await opening;
      if (opened === undefined) { await pool.release(); return; }
      for (const release of outstanding) release();
      outstanding.clear();
      opened.release();
      const stats = opened.farm.stats();
      say("finished", { projects: jobs, workers: width, seconds: Math.round((performance.now() - opened.started) / 1000), jobs: stats.jobs - opened.jobs, fallbacks: stats.fallbacks, recycled: stats.recycled });
      if (environment[policy.phasesEnvironment] === "1") process.stderr.write("[graph-farm] " + JSON.stringify({ ...stats, projects: jobs, reason: plan.reason, ...traffic, distinctRows: log.length, learnMs: Math.round(traffic.learnMs), replayMs: Math.round(traffic.replayMs) }) + "\n");
    },
  });
}
