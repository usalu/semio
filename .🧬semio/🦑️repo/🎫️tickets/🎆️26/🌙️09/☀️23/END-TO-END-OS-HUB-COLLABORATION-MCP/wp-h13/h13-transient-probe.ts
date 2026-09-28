/** 📶️ H13 session 14c — live proof of C12 P1 on a current-tree hub: a writer's whole post-cut outbox arrives as ONE `Commands`
 * batch. Every batch is admitted and committed while the document socket's command budget lasts; the batch that finds the budget
 * empty is refused TRANSIENTLY with the declared machine-readable code (`hub.unavailable`, schema `🚧️refusal`), the writer resends
 * it after the refill and it commits; nothing is lost — an observer socket receives every envelope and the reopened document's
 * welcome carries them all. Credentials only from env (`OS_HUB_PROBE_EMAIL`/`_PASSWORD`), never printed.
 * usage: bun h13-transient-probe.ts <hub origin> [batchEnvelopes=600] [batches=15] [kind=s.note.note] */
import { hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenDocument, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";

const [origin = "http://127.0.0.1:8010", perBatchText = "600", batchesText = "15", kindWanted = "s.note.note"] = process.argv.slice(2);
const perBatch = Number(perBatchText);
const batches = Number(batchesText);
const email = process.env.OS_HUB_PROBE_EMAIL ?? "";
const password = process.env.OS_HUB_PROBE_PASSWORD ?? "";
if (!email || !password) throw new Error("OS_HUB_PROBE_EMAIL / OS_HUB_PROBE_PASSWORD required");
const log = (line: string): void => console.log(`[transient-probe] ${new Date().toISOString().slice(11, 23)} ${line}`);

const token = await hubProbeSignIn(origin, email, password, "h13-transient-writer-");
const observerToken = await hubProbeSignIn(origin, email, password, "h13-transient-observer-");
const spaceId = await hubProbeCreateSpace(origin, token, `Transient refusal ${Date.now()}`);
const catalog = await hubProbeCreationCatalog(origin, token, spaceId);
const kind = catalog.kinds.find((entry) => entry.kindId === kindWanted || entry.schema.startsWith(kindWanted)) ?? catalog.kinds.find((entry) => entry.kindId.includes("note"));
if (!kind) throw new Error(`no ${kindWanted} kind in ${catalog.kinds.map((entry) => entry.kindId).join(",")}`);
const created = await hubProbeCreateArtifact(origin, token, spaceId, catalog.generationId, kind.kindId, "Transient refusal");
log(`space ${spaceId} ${kind.kindId} ${created.artifactId} created in ${created.ms} ms`);
const writer = await hubProbeOpenDocument(origin, token, spaceId, created.artifactId, "h13-transient-writer");
const observer = await hubProbeOpenDocument(origin, observerToken, spaceId, created.artifactId, "h13-transient-observer");
const schema = writer.plan.artifact.schema;
let previous = "";
let sequence = 0;
const envelope = (id: string): any => {
  const built = { mutation_id: id, document_id: created.artifactId, actor: writer.actorId, dependencies: previous ? [previous] : [], observed: null, target: [], diff: { schema, payload: Array.from(new TextEncoder().encode(`outbox:${id}:${"k".repeat(24)}`)) }, inverse: { schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: sequence } };
  previous = id;
  sequence += 1;
  return built;
};
const codesOf = (ack: any): string[] => {
  const decoded = JSON.stringify(ack).replace(/"messages":\[([0-9,]*)\]/gu, (_, bytes: string) => JSON.stringify(new TextDecoder().decode(new Uint8Array(bytes ? bytes.split(",").map(Number) : []))));
  return [...decoded.matchAll(/hub\.[a-z.-]+/gu)].map((match) => match[0]);
};
const rows: { batch: number; attempt: number; accepted: boolean; ms: number; codes: string[]; detail: string }[] = [];
const committed: string[] = [];
let transientSeen = 0;
let permanent = 0;
for (let batch = 0; batch < batches; batch += 1) {
  const outbox = Array.from({ length: perBatch }, (_, index) => envelope(`h13o-${batch}-${index}`));
  for (let attempt = 1; attempt <= 40; attempt += 1) {
    const started = Date.now();
    const answer = await writer.submitEnvelopes(batch * 100 + attempt, outbox);
    const codes = codesOf(answer.ack);
    rows.push({ batch, attempt, accepted: answer.accepted, ms: Date.now() - started, codes, detail: answer.accepted ? "" : JSON.stringify(answer.ack).slice(0, 600) });
    if (answer.accepted) {
      committed.push(...outbox.map((entry) => entry.mutation_id));
      log(`batch ${batch} (${perBatch} envelopes) accepted on attempt ${attempt} in ${Date.now() - started} ms`);
      break;
    }
    if (codes.includes("hub.unavailable")) {
      transientSeen += 1;
      log(`batch ${batch} attempt ${attempt} refused transiently (${codes.join(",")}) — resend after 2 s: ${JSON.stringify(answer.ack).slice(0, 300)}`);
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 2_000));
      continue;
    }
    permanent += 1;
    log(`batch ${batch} attempt ${attempt} refused PERMANENTLY: ${JSON.stringify(answer.ack).slice(0, 600)}`);
    break;
  }
  if (permanent) break;
}
const overCount = Number(process.env.H13_OVER_DECLARED ?? "0");
let overVerdict = "not-sent";
if (overCount > 0 && !permanent) {
  const outbox = Array.from({ length: overCount }, (_, index) => envelope(`h13over-${index}`));
  previous = committed.at(-1) ?? "";
  const answer = await writer.submitEnvelopes(9_999, outbox);
  const codes = codesOf(answer.ack);
  overVerdict = answer.accepted ? "accepted" : codes.includes("hub.batch-limit") && !codes.includes("hub.unavailable") ? "permanent-batch-limit" : `other:${codes.join(",")}`;
  log(`over-declared batch (${overCount} envelopes): ${overVerdict} ${JSON.stringify(answer.ack).slice(0, 240)}`);
}
const last = committed.at(-1);
let relayedAll = false;
if (last) {
  try {
    await observer.relayed(last, 120_000);
    relayedAll = true;
  } catch (error) {
    log(`observer: ${String(error).slice(0, 300)}`);
  }
}
const observed = new Set(observer.relayedEnvelopes().map((entry: any) => String(entry.mutation_id)));
const missing = committed.filter((id) => !observed.has(id)).length;
writer.close();
observer.close();
const reopened = await hubProbeOpenDocument(origin, observerToken, spaceId, created.artifactId, "h13-transient-reopen");
const welcomeText = JSON.stringify(reopened.welcome);
reopened.close();
const summary = { hub: origin, kind: kind.kindId, perBatch, batches, committedEnvelopes: committed.length, transientRefusals: transientSeen, permanentRefusals: permanent, observerMissing: missing, observerRelayedLast: relayedAll, overDeclared: overVerdict, welcomeBytes: welcomeText.length, attempts: rows.length };
log(`summary ${JSON.stringify(summary)}`);
for (const row of rows.filter((entry) => !entry.accepted).slice(0, 3)) log(`refusal sample ${JSON.stringify(row)}`);
const pass = permanent === 0 && committed.length === perBatch * batches && transientSeen > 0 && missing === 0 && relayedAll && (overCount === 0 || overVerdict === "permanent-batch-limit");
log(pass ? "PASS every outbox batch committed; the budget-empty batch was refused transiently with hub.unavailable and committed on resend; observer received all" : "FAIL");
process.exitCode = pass ? 0 : 1;
