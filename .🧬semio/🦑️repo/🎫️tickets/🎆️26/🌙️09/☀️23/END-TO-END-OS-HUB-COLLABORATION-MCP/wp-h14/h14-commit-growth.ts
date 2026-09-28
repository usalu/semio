#!/usr/bin/env bun
/** 📈️ H14 14c one-off: does a document's commit cost grow with the document? One writer socket submits `batches` batches of
 * `perBatch` chained envelopes (one observer socket open, as in a collaboration), each waited for; every batch's Ack latency,
 * its frontier head and any socket end are logged with a wall-clock stamp so a sampler can be aligned. Credentials only from
 * env (`OS_HUB_PROBE_EMAIL` / `OS_HUB_PROBE_PASSWORD`), never printed.
 *   bun h14-commit-growth.ts <hub origin> [perBatch=256] [batches=40] [kind=s.note.note] [observer=1] */
import { hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenDocument, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";

const [origin = "http://127.0.0.1:8161", perBatchText = "256", batchesText = "40", kindWanted = "s.note.note", observerText = "1"] = process.argv.slice(2);
const perBatch = Number(perBatchText);
const batches = Number(batchesText);
const email = process.env.OS_HUB_PROBE_EMAIL ?? "";
const password = process.env.OS_HUB_PROBE_PASSWORD ?? "";
if (!email || !password) throw new Error("OS_HUB_PROBE_EMAIL / OS_HUB_PROBE_PASSWORD required");
const log = (line: string): void => console.log(`[commit-growth] ${new Date().toISOString().slice(11, 23)} ${line}`);
const token = await hubProbeSignIn(origin, email, password, "h14-growth-writer-");
const spaceId = await hubProbeCreateSpace(origin, token, `Commit growth ${Date.now()}`);
const catalog = await hubProbeCreationCatalog(origin, token, spaceId);
const kind = catalog.kinds.find((entry) => entry.kindId === kindWanted || entry.schema.startsWith(kindWanted));
if (!kind) throw new Error(`no ${kindWanted} kind`);
const created = await hubProbeCreateArtifact(origin, token, spaceId, catalog.generationId, kind.kindId, "Commit growth");
log(`space ${spaceId} ${kind.kindId} ${created.artifactId} created in ${created.ms} ms`);
const writer = await hubProbeOpenDocument(origin, token, spaceId, created.artifactId, "h14-growth-writer");
const observer = observerText === "1" ? await hubProbeOpenDocument(origin, await hubProbeSignIn(origin, email, password, "h14-growth-observer-"), spaceId, created.artifactId, "h14-growth-observer") : null;
const schema = writer.plan.artifact.schema;
let previous = "";
let sequence = 0;
const envelope = (id: string): any => {
  const built = { mutation_id: id, document_id: created.artifactId, actor: writer.actorId, dependencies: previous ? [previous] : [], observed: null, target: [], diff: { schema, payload: Array.from(new TextEncoder().encode(`outbox:${id}:${"k".repeat(24)}`)) }, inverse: { schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: sequence } };
  previous = id;
  sequence += 1;
  return built;
};
const ended = writer.ended(3_600_000).then((end) => ({ end }));
const hubPid = process.env.HUB_PID ?? "";
const hubCpuMs = (): number => {
  if (!hubPid) return -1;
  const text = Bun.spawnSync(["ps", "-o", "cputime=", "-p", hubPid]).stdout.toString().trim();
  const [clock, fraction = "0"] = text.split(".");
  const parts = clock!.split(":").map(Number);
  const seconds = parts.reduce((sum, part) => sum * 60 + part, 0);
  return Math.round(seconds * 1000 + Number(`0.${fraction}`) * 1000);
};
const cpu: number[] = [];
const timings: number[] = [];
for (let batch = 0; batch < batches; batch += 1) {
  const outbox = Array.from({ length: perBatch }, (_, index) => envelope(`h14g-${batch}-${index}`));
  const cpuBefore = hubCpuMs();
  const started = Date.now();
  const answer = await Promise.race([writer.submitEnvelopes(batch + 1, outbox).then((result) => ({ result })), ended]).catch((error) => ({ error: String(error).slice(0, 300) }));
  const ms = Date.now() - started;
  if ("result" in answer) {
    timings.push(ms);
    const cpuMs = hubCpuMs() - cpuBefore;
    cpu.push(cpuMs);
    log(`batch ${batch} ${answer.result.accepted ? "accepted" : "REFUSED"} in ${ms} ms hub-cpu ${cpuMs} ms head ${answer.result.ack?.frontier?.head_edit_ordinal}${answer.result.accepted ? "" : ` ${JSON.stringify(answer.result.ack).slice(0, 300)}`}`);
    if (!answer.result.accepted) break;
  } else if ("end" in answer) {
    log(`batch ${batch}: writer socket ENDED after ${ms} ms ${JSON.stringify(answer.end)}`);
    break;
  } else {
    log(`batch ${batch}: ${answer.error} after ${ms} ms`);
    break;
  }
}
log(`timings ${JSON.stringify(timings)}`);
log(`hub-cpu ${JSON.stringify(cpu)}`);
writer.close();
observer?.close();
process.exit(0);
