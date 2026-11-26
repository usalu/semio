/** 🧩️ Opens, edits and closes the actual JCO Puzzle editor through the production component bridge. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { startedTypedOperationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🏁️operation-settle/🟦️.ts";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import law from "../../../../../../../🌎️hub/🧩️compositions/🧩️puzzle/🧪️tests/🧫️fixtures/🧩️guest-io/🔣️.json" with { type: "json" };
import { KernelOriginalOperationIssuer, KernelOriginalResourcePool, optionValue } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts";
import { actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, decodeActorInstanceLifecycle, type ActorInstanceLifecycleReceipt } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { createShardCommandIngressPages } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { DEFAULT_SHARD_BUDGET } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import { coerceWireBytes, scanTypedOperationPages, type TypedOperationAckEvent } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { wireSpawnJob } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { driveSpawnedJob, spawnedJobCompletedEvent, wireJobStep } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🧵️spawned-job/🟦️.ts";
import { spawnedJobCompletion, SPAWNED_JOB_DEADLINE_MS, SPAWNED_JOB_FUEL } from "../../../../../../../🧰️framework/🔨️modules/🎠️kernel/⚡️effect/🛠️spawned-job/🟦️.ts";
import { decodeAppFrame, encodeAppCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts";
import { DocumentArchiveLoadHost } from "../../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🗃️archive-load-host/🟦️.ts";
import { decodePackValue, encodePackValue, packValueToExactJson, viewContextWireValue } from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";

const actor = "puzzle-js-pi";
const activationGeneration = 1n;
const instance = 73;
const appId = process.argv.slice(3).find(argument => !argument.startsWith("--")) ?? law.component.appId;
const resources = { grant: law.operationSource, fuel: 1 << 27, maximumElapsedUs: BigInt(law.execution.maximumMilliseconds) * 1000n };
const tenures = 4;
const grant = {
  maximumItems: resources.grant.maximumItems * tenures,
  maximumCopyBytes: resources.grant.maximumCopyBytes * tenures,
  maximumCapacityBytes: resources.grant.maximumCapacityBytes * tenures,
  maximumReleaseBytes: resources.grant.maximumReleaseBytes * tenures,
  maximumDepth: resources.grant.maximumDepth,
};
const pool = new KernelOriginalResourcePool({ ...resources, grant, fuel: resources.fuel * tenures });
const issuer = new KernelOriginalOperationIssuer(request => pool.admit(request));
const admission = issuer.admit(resources);
const { createActorApi } = await import(pathToFileURL(resolve(process.argv[2])).href);
let receipts = 0;
const api = await createActorApi(actor, activationGeneration, admission, () => process.hrtime.bigint() / 1000n, {
  receipt() { receipts += 1; },
  hold(fault: unknown) { console.error("[DEBUG] Original return held", fault); },
  succeed() {
    const successor = issuer.admit(resources);
    console.error(`[DEBUG] Kernel admitted finite successor stock ${successor.operation}`);
    return successor;
  },
}, async (url: string) => WebAssembly.compile(await readFile(new URL(url))));
const shape = DEFAULT_SHARD_BUDGET;
const wake = { kind: "wake" };
const operationAcks: TypedOperationAckEvent[] = [];
const operationFaults: string[] = [];
const operationCompletions = new Map<bigint, readonly number[]>();
const spawnedJobs = new Map<bigint, ReturnType<typeof wireSpawnJob>["spawnJob"]>();
const finishedJobs = new Set<bigint>();
let resultPages = 0;
const observedFrames = new Map<string, number>();
const unsolicitedFaults: unknown[] = [];

async function poll(events: readonly unknown[], page?: unknown) {
  const result = await api.poll(events, page, undefined, shape);
  assert(["idle", "more-work"].includes(result.status.tag), JSON.stringify(result.status, (_, value) => typeof value === "bigint" ? String(value) : value));
  assert.equal(result.uiPatches.length, 0);
  const operation = scanTypedOperationPages(result.effects);
  for (const typed of operation.pages) assert.equal(typed.token.receiver, instance);
  operationAcks.push(...operation.acknowledgements);
  operationFaults.push(...operation.faults);
  resultPages += operation.pages.length;
  const frames = operation.kept.flatMap(effect => {
    if (effect.tag === "spawn-job") {
      const { spawnJob } = wireSpawnJob(effect);
      if (!finishedJobs.has(spawnJob.job)) spawnedJobs.set(spawnJob.job, spawnJob);
      return [];
    }
    if (effect.tag !== "send-message") return [];
    const value = effect.val as { target: { tag: string; val: number }; payload: Uint8Array };
    if (value.target.tag !== "shell" || Number(value.target.val) !== instance) return [];
    const frame = decodeAppFrame(value.payload);
    const kind = Object.keys(frame)[0]!;
    observedFrames.set(kind, (observedFrames.get(kind) ?? 0) + 1);
    if ("Error" in frame && unsolicitedFaults.length < 8) unsolicitedFaults.push(frame.Error);
    if ("OperationCompleted" in frame) operationCompletions.set(frame.OperationCompleted.operation, frame.OperationCompleted.history_patch);
    return [frame];
  });
  return { ...result, frames };
}

function continueTurn() {
  return operationAcks.shift() ?? wake;
}

async function settleSpawnedJob(spawned: ReturnType<typeof wireSpawnJob>["spawnJob"]) {
  const port = { startJob: api.startJob, stepJob: api.stepJob, cancelJob: api.cancelJob };
  const started = performance.now();
  if (!process.argv.includes("--long-job-diagnostic")) {
    let turns = 0;
    return driveSpawnedJob({ ...spawned, port, signal: AbortSignal.timeout(law.execution.maximumMilliseconds), onStep() { turns += 1; }, continueAfterAdmission: async () => {
      assert(turns < law.execution.maximumTurns && performance.now() - started < law.execution.maximumMilliseconds, `job ${spawned.job} exhausted its caller allowance`);
      await new Promise<void>(resolve => setImmediate(resolve));
    } });
  }
  await port.startJob(spawned.job, spawned.kind, spawned.input);
  let complete = false;
  const progressKinds = new Map<string, number>();
  try {
    for (let steps = 1; steps <= law.execution.maximumTurns && performance.now() - started < law.execution.maximumMilliseconds; steps += 1) {
      const raw = await port.stepJob(spawned.job, { fuel: SPAWNED_JOB_FUEL, deadlineMs: SPAWNED_JOB_DEADLINE_MS });
      const step = wireJobStep(raw);
      if (step.status === "running") {
        const progress = optionValue(raw.progress ?? raw.val);
        const bytes = progress === undefined ? undefined : coerceWireBytes(progress);
        const identity = bytes === undefined ? "absent" : `${bytes.length} bytes ${createHash("sha256").update(bytes).digest("hex").slice(0, 16)}`;
        progressKinds.set(identity, (progressKinds.get(identity) ?? 0) + 1);
        if (steps <= 4 || steps === 32 || steps % 32 === 0) console.error(`[DEBUG] Genuine job ${spawned.job} step ${steps} progress ${identity} (${Object.keys(raw).join(",")})`);
      }
      if (step.status !== "running") {
        console.error(`[DEBUG] Genuine job ${spawned.job} running progress sequence ${JSON.stringify([...progressKinds])}`);
        complete = true;
        return { ...spawnedJobCompletion([step]), steps };
      }
      if (steps === 32 || steps % 256 === 0) console.error(`[DEBUG] Genuine job ${spawned.job} (${spawned.kind}) still running at ${steps} steps`);
    }
    assert.fail(`diagnostic job ${spawned.job} reached its fixture limit`);
  } finally {
    if (!complete) await port.cancelJob(spawned.job);
  }
}

async function acknowledge(receipt: ActorInstanceLifecycleReceipt) {
  return poll([{ kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt } }]);
}

const open = { kind: "open" as const, activationGeneration, instanceId: instance, requestSequence: 1 };
let turn = await poll([{ kind: "instance-open", payload: {
  purpose: { tag: "ordinary" }, instance, activationGeneration, requestSequence: open.requestSequence,
  appId, actor, config: [], assets: [], capabilities: [], quotas: encodePackValue({}),
} }]);
for (let attempt = 0; !turn.lifecycleReceipt && attempt < law.execution.maximumTurns; attempt += 1) turn = await poll([wake]);
assert(turn.lifecycleReceipt, "editor publishes paid capture");
const captured = decodeActorInstanceLifecycle(turn.lifecycleReceipt);
assert(captured.kind === "captured" && actorInstanceCapturedReceiptMatches(open, captured));
await acknowledge(captured);
console.error("[DEBUG] Editor captured and acknowledged");

async function exchange(seq: number, command: Parameters<typeof encodeAppCommand>[0], resultKinds: readonly string[]) {
  const pages = createShardCommandIngressPages({ owner: BigInt(instance), generation: 1n, commandIndex: 0, commandCount: 1, instance, seq: BigInt(seq), command: encodeAppCommand(command) });
  let answer;
  let complete = false;
  let offered = 0;
  const ingress = new Set<string>();
  for (let attempt = 0; attempt < law.execution.maximumTurns; attempt += 1) {
    const page = pages[offered];
    const result = await poll(page && operationAcks.length === 0 ? [] : [continueTurn()], page);
    const status = result.commandIngress.tag;
    assert.notEqual(status, "fault", "actual command ingress accepts the page");
    ingress.add(status);
    if (page && status !== "backpressure" && (result.deferredEvents ?? 0) === 0) {
      offered = status === "page-accepted" ? offered + 1 : pages.length;
    }
    complete ||= status === "command-complete";
    for (const frame of result.frames) {
      for (const [kind, body] of Object.entries(frame) as [string, { in_reply_to?: number }][]) {
        if (Number(body.in_reply_to) !== seq) continue;
        if (resultKinds.includes(kind)) answer = frame;
        else assert.notEqual(kind, "Error", JSON.stringify(body));
      }
    }
    if (complete && answer) return answer;
  }
  assert.fail(`Editor never settled ${resultKinds.join("/")} (ingress: ${[...ingress].join(", ")}; ${offered}/${pages.length} pages accepted; complete: ${complete}; matching reply: ${Boolean(answer)})`);
}

async function history(seq: number) {
  const frame = await exchange(seq, { ReadHistory: { seq } }, ["HistorySnapshot"]);
  assert("HistorySnapshot" in frame);
  return packValueToExactJson(decodePackValue(Uint8Array.from(frame.HistorySnapshot.history_patch))) as { activeAlternativeId?: string; currentCheckpointId?: string; upserts?: { actionId: string }[] };
}

let sequence = 10;
const nextSeq = () => ++sequence;
async function openGuestGenesis() {
  const artifactKind = process.env.PUZZLE_GENESIS_KIND ?? appId.split("@")[0]!;
  const answer = await api.codec({ operation: "genesis", artifactKind, documentId: "artifact-0123456789abcdef0123456789abcdef" });
  if ("fault" in answer) {
    const fault = Buffer.from(answer.fault);
    assert.fail(`guest genesis ${artifactKind} fault ${JSON.stringify(fault.toString("utf8").slice(0, 320))} (${fault.subarray(0, 36).toString("hex")})`);
  }
  const { pack, spr } = answer.ok;
  assert(pack.length > 0 && spr.length > 0);
  console.error(`[DEBUG] Guest authored genesis pair ${pack.length} PACK/${spr.length} SPR bytes`);
  const loader = new DocumentArchiveLoadHost({ parent_pack: pack, parent_spr: spr, members: [] });
  for (let attempt = 0; attempt < law.execution.maximumTurns; attempt += 1) {
    const next = loader.step(nextSeq);
    if (next.kind === "finished") {
      assert.equal(next.outcome.kind, "ready", JSON.stringify(next.outcome));
      console.error("[DEBUG] Guest acknowledged authored document archive");
      return;
    }
    const frame = await exchange(next.seq, next.command, ["Done", "DocumentArchiveLoad", "Error"]);
    const receipt = loader.answer(next.seq, frame);
    assert.equal(receipt.kind, "status", JSON.stringify(receipt));
  }
  assert.fail("guest did not acknowledge its genesis archive");
}

if (process.argv.includes("--genesis")) await openGuestGenesis();
const before = await history(nextSeq());
console.error(`[DEBUG] Editor returned initial history ${JSON.stringify(before)}`);
let editResult: { error: unknown } | undefined;
const addNode = process.argv.includes("--add-node");
if (process.argv.includes("--edit") || addNode) {
  try {
    const action = addNode ? "addNode" : law.afterCancel.command;
    const args = addNode ? { kind: "node" } : law.afterCancel.args;
    const seq = nextSeq();
    const windowKindId = "2d-overview";
    const windowId = "overview";
    const modeId = "edit";
    const address = { pluginId: law.component.pluginId, appId, modeId, windowKindId, windowInstanceId: windowId, actionId: action };
    const view = viewContextWireValue({ locale: law.client.locale, terminology: law.client.terminology, activeModeId: modeId, activeWindowKindId: windowKindId, windowId, windowInstances: [{ id: windowId, windowKindId }] });
    const command = addNode
      ? { Command: { seq, command: Array.from(encodePackValue({ address, arguments: args })), view_state: Array.from(encodePackValue(view)) } }
      : { ArtifactCommand: { seq, command: Array.from(encodePackValue({ action, args })) } };
    const reply = await exchange(seq, command, ["Invocation"]);
    assert("Invocation" in reply);
    const admission = packValueToExactJson(decodePackValue(Uint8Array.from(reply.Invocation.output)));
    const operation = startedTypedOperationV1(admission);
    assert(operation && operation.operation > 0n, `actual operation admission ${JSON.stringify(admission)}`);
    const operationId = operation.operation;
    console.error(`[DEBUG] Editor admitted ${action} as operation ${operationId}`);
    const activities = new Map<string, number>();
    let idleTurns = 0;
    for (let attempt = 0; (!operationCompletions.has(operationId) || operationAcks.length > 0) && attempt < law.execution.maximumTurns; attempt += 1) {
      const acknowledgement = operationAcks.shift();
      const spawned = spawnedJobs.values().next().value;
      let result;
      if (acknowledgement) {
        result = await poll([acknowledgement]);
      } else if (spawned) {
        spawnedJobs.delete(spawned.job);
        finishedJobs.add(spawned.job);
        const completion = await settleSpawnedJob(spawned);
        console.error(`[DEBUG] Guest job ${spawned.job} (${spawned.kind}) completed after ${completion.steps} steps`);
        result = await poll([spawnedJobCompletedEvent(spawned.job, completion)]);
      } else {
        result = await poll([]);
      }
      const activity = `${result.status.tag}/${result.commandIngress.tag}/${result.nextWake ?? "none"}/${result.frames.map(frame => Object.keys(frame)[0]).join(",")}/${result.effects.map((effect: { tag: string }) => effect.tag).join(",")}`;
      activities.set(activity, (activities.get(activity) ?? 0) + 1);
      idleTurns = result.status.tag === "idle" && result.nextWake === null && operationAcks.length === 0 && spawnedJobs.size === 0 ? idleTurns + 1 : 0;
      if ((operationFaults.length > 0 && operationAcks.length === 0) || idleTurns >= 16) break;
    }
    assert.deepEqual(operationFaults, [], "guest operation fault");
    assert.equal(operationAcks.length, 0);
    const after = await history(nextSeq());
    const completed = operationCompletions.get(operationId);
    const patch = completed?.length ? packValueToExactJson(decodePackValue(Uint8Array.from(completed))) : undefined;
    console.error(`[DEBUG] Guest operation ${operationId} yielded ${resultPages} result pages and completion history ${JSON.stringify(patch)}`);
    assert(completed, `guest did not publish OperationCompleted; activities ${JSON.stringify([...activities])}; frames ${JSON.stringify([...observedFrames])}; completed IDs ${[...operationCompletions.keys()].join(",")}; unsolicited faults ${JSON.stringify(unsolicitedFaults)}; history ${JSON.stringify(after)}`);
    if (!addNode) {
      assert(after.activeAlternativeId, `history after ${action} ${JSON.stringify(after)}`);
      assert.notEqual(after.activeAlternativeId, before.activeAlternativeId);
    }
    assert(after.upserts?.some(entry => entry.actionId === action), `history after ${action} ${JSON.stringify(after)}`);
    console.error(`[DEBUG] Editor returned changed history after ${action}`);
  } catch (error) {
    editResult = { error };
    console.error("[DEBUG] Editor edit assertion", error);
  }
}

const close = { kind: "close" as const, lifetime: captured.lifetime, requestSequence: 20 };
turn = await poll([{ kind: "instance-close", payload: close }]);
let accepted: ActorInstanceLifecycleReceipt | null = null;
let retired = false;
for (let attempt = 0; attempt < law.execution.maximumTurns; attempt += 1) {
  if (turn.lifecycleReceipt) {
    const receipt = decodeActorInstanceLifecycle(turn.lifecycleReceipt);
    assert(receipt.kind !== "ack" && receipt.kind !== "open" && receipt.kind !== "close" && actorInstanceCloseReceiptMatches(close, accepted, receipt));
    turn = await acknowledge(receipt);
    if (receipt.kind === "retired") { retired = true; break; }
    accepted = receipt;
  } else {
    turn = await poll([wake]);
  }
}
assert(retired, "editor publishes paid retirement");
console.error(`[DEBUG] Editor accepted and retired its instance after ${receipts} receipts (${turn.status.tag}; ${turn.deferredEvents ?? 0} deferred events)`);
if (process.argv.includes("--drain-after-retire")) {
  for (let attempt = 0; (turn.status.tag === "more-work" || (turn.deferredEvents ?? 0) > 0) && attempt < law.execution.maximumTurns; attempt += 1) turn = await poll([wake]);
  console.error(`[DEBUG] Post-retirement maintenance ${turn.status.tag}; ${turn.deferredEvents ?? 0} deferred events`);
}
let source = await api.closeOriginal();
console.error(`[DEBUG] Original source ${source.operation}/${source.generation} closed=${source.sourceClosed}`);
if (process.argv.includes("--retry-source-close")) {
  for (let attempt = 0; !source.sourceClosed && attempt < 8; attempt += 1) {
    source = await api.closeOriginal();
    console.error(`[DEBUG] Retried Original source ${source.operation}/${source.generation} closed=${source.sourceClosed}`);
  }
}
assert(source.sourceClosed, "production bridge publishes paid Original source closure");
assert(receipts > 0);
if (editResult) throw editResult.error;
