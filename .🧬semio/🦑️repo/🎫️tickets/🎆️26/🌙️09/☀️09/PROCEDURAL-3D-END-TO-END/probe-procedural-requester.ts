import { replayEditorDocument } from "./replay-editor-document.ts";
import { readFile, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { Session } from "node:inspector";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { decodeAppFrame, encodeAppCommand } from "@semio-tech/framework/app-channel";
import { parseResolvedPluginViewState } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts";
import { decodePackValue, encodePackValue, viewContextWireValue } from "@semio-tech/framework/packs";
import { createShardCommandIngressPages } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { KernelOriginalBootstrapStock, originalBootstrapStockFromWire } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";
import { decodeActorInstanceLifecycle } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { decodeActorUiPatchReceipt } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";

import { scanTypedOperationPages, shellFrameBytes } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";

import { wireExtensionInvocation, wireEffectToFriendly } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { driveInboundRequest } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/📥️inbound-request/🟦️.ts";
import { guestAnswerPages } from "../../../../../../../🧰️framework/🔨️modules/⏱️trace/🧮️memory/🟦️.ts";
import { consumedTopicsForApp, scopeContributionsJson } from "../../../../../../../🧰️framework/🔨️modules/🎠️kernel/🧾️contributions-json/🟦️.ts";

Error.stackTraceLimit = 160;
const moduleRoot = resolve(process.argv[2]!);
const scenario = process.argv[4] ?? "example";
const recordedClock = process.env.SEMIO_NODE_PROBE_RECORDED_CLOCK === "1";
const stockInput = JSON.parse(await readFile(process.argv[3]!, "utf8"));
const descriptor = JSON.parse(await readFile(join(moduleRoot, "🔣️.json"), "utf8"));
const pluginId = descriptor.manifest.pluginId;
const app = descriptor.manifest.apps.find(({ id }: { id: string }) => id === "s.procedural.generation3d@1/*#editor");
if (!app) throw new Error("missing actual Generation 3D editor");
const actor = "local-procedural-js-pi";
const started = performance.now();
const elapsed = () => Math.round(performance.now() - started);
const json = (value: unknown) => JSON.stringify(value, (_, item) => typeof item === "bigint" ? item.toString() : item);
const log = (...parts: unknown[]) => console.log("[DEBUG]", elapsed(), ...parts);
async function openBridge(directory: string, pluginId: string) {
  const stock = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stockInput));
  const admission = stock.admit();
  const bridge = await import(pathToFileURL(join(directory, "🌉️bridge.js")).href);
  return bridge.createActorApi(pluginId + "#1", 1n, admission, () => recordedClock ? 1000n : BigInt(Math.floor(performance.now() * 1000)), {
    receipt: (value: Parameters<typeof stock.receive>[1]) => stock.receive(admission, value),
    hold: (fault: unknown) => stock.hold(admission, fault),
    succeed: async () => stock.succeed(admission),
  }, async (url: string) => WebAssembly.compile(await readFile(new URL(url))));
}
if (process.env.SEMIO_NODE_PROBE_NATIVE_DIAGNOSTICS === "1") {
  const env = await import(pathToFileURL(join(moduleRoot,"../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/environment.js")).href);
  env._setEnv({SEMIO_RUNTIME_DIAGNOSTICS:"1"});
  for (const level of ["debug","log","warn","error"] as const) {
    const original = console[level].bind(console);
    console[level] = (...args) => {
      const message=args.find(value=>typeof value==="string" && value.includes("[TRACE]"));
      if (!message || /cleanup continuation for instance|maintenance stage=.*outcome=Err|runtime live cleanup faulted/i.test(message)) original(...args);
    };
  }
}
const api = await openBridge(moduleRoot, pluginId);
log("compiled requester bridge activated", app.id);

const budget = { maxEffects: 32, maxPatchBytes: 1048576, maxFrames: 8, fuel: 50000000 };
const instance = 1;
let sequence = 1;
let crossing = 0;
let patchAck: unknown;
const operationAcks: unknown[] = [];
const completedOperations = new Set<string>();
const pendingTicks: Record<string, unknown>[] = [];
const requests: ReturnType<typeof wireExtensionInvocation>["invokeExtension"][] = [];
const providerRoot = process.env.SEMIO_NODE_PROBE_PROVIDERS;
const registryPath = process.env.SEMIO_NODE_PROBE_REGISTRY;
const recordedMathPrefix = process.env.SEMIO_NODE_PROBE_RECORDED_MATH;
const surfacePath = process.env.SEMIO_NODE_PROBE_SURFACE;
const capturedHost = surfacePath ? dirname(surfacePath) : undefined;
const exactOpen = capturedHost ? JSON.parse(await readFile(join(capturedHost, "tick-open.json"), "utf8")) : undefined;
let exactTick = capturedHost ? JSON.parse(await readFile(join(capturedHost, "tick-ingress-1.json"), "utf8")) : undefined;
let ingressSequence = 1n;
const providers = new Map<string, { pluginId: string; directory: string; manifest: any; api?: any; seq: bigint }>();
const replayArchives = process.env.SEMIO_NODE_PROBE_ARCHIVES === "1";
const archives: Array<{parent_pack:number[];parent_spr:number[];members:[]}> = [];
let archiveFrames: ReturnType<typeof decodeAppFrame>[] | undefined;
let lastOperation: string | undefined;
const recordedMath = recordedMathPrefix ? {
  request: (await readFile(recordedMathPrefix + "-request.json", "utf8")).trim(),
  answer: await readFile(recordedMathPrefix + "-answer.pack"),
  completed: JSON.parse(await readFile(recordedMathPrefix + "-completed.json", "utf8")),
} : undefined;
if (recordedMath) {
  if (scenario !== "integrated" || !exactTick) throw new Error("Recorded Math requires integrated mode and captured host ingress");
  const [completion, ...others] = recordedMath.completed;
  if (others.length || completion?.kind !== "completed" || completion.payload.req !== "1" ||
      completion.payload.outcome.tag !== "ok" || !Buffer.from(completion.payload.outcome.val).equals(recordedMath.answer) ||
      createHash("sha256").update(recordedMath.answer).digest("hex") !== "b1ddfe8470f5147ecf7c3a919efe7f4a09d87038e6df60c583ca6082cf22b396") {
    throw new Error("Recorded Math capture lost its real completion bytes");
  }
}
let recordedMathStage: "waiting-request" | "delivering" | "waiting-rearm" | "rearm-ready" | "dispatching-rearm" | "proved" = "waiting-request";

function parseRecordedRequest(requestJson: string): Record<string, unknown> {
  return JSON.parse(requestJson.replace(/"nodeHash":\s*(\d+)/, '"nodeHash":"$1"'));
}

function recordedMathRequestMatches(received: Record<string, unknown>, expected: Record<string, unknown>, resume: boolean): boolean {
  expected = { ...expected };
  expected.resume = resume;
  if (resume) {
    expected.inputJson = "";
    expected.dependencyJson = "";
  }
  const keys = Object.keys(expected).sort();
  return json(keys) === json(Object.keys(received).sort()) && keys.every((key) => received[key] === expected[key]);
}

async function invokePending(): Promise<void> {
  while (requests.length) {
    const request = requests.shift()!;
    const provider = providers.get(request.extensionId);
    if (!provider) throw new Error("No installed extension for exact request " + json(request));
    log("extension request", request.req.toString(), request.extensionId, request.capability, request.requestJson);
    if (recordedMath && request.extensionId === "flow-extension-math" && request.capability === "evaluate") {
      const received = parseRecordedRequest(request.requestJson);
      const expected = parseRecordedRequest(recordedMath.request);
      if (recordedMathStage === "dispatching-rearm" && received.windowId === expected.windowId && received.neuronId === expected.neuronId) {
        if (request.req <= 1n || !recordedMathRequestMatches(received, expected, true)) throw new Error("Rearmed Math request changed the recorded identity or payload");
        recordedMathStage = "proved";
        log("recorded Math continuation emitted a distinct resumed request", request.req.toString());
        return;
      }
      if (recordedMathStage === "waiting-request" && request.req === 1n) {
        if (!recordedMathRequestMatches(received, expected, false)) throw new Error("Live Math request does not own captured progress");
        recordedMathStage = "delivering";
        const { prologue, terminal } = guestAnswerPages(recordedMath.answer);
        const events = [
          ...prologue.map((page) => ({ kind: "http-chunk", payload: { req: request.req, params: { bytes: Array.from(page), done: false } } })),
          { kind: "completed", payload: { req: request.req, outcome: { tag: "ok", val: Array.from(terminal) } } },
        ];
        await poll("deliver byte-identical Math progress", undefined, undefined, events);
        continue;
      }
    }
    provider.api ??= await openBridge(provider.directory, provider.pluginId);
    const answer = await driveInboundRequest({
      req: ++provider.seq, capability: request.capability, payload: new TextEncoder().encode(request.requestJson),
      originInstanceId: instance, submit: async (events) => {
        const original=await provider.api.poll(events, undefined, undefined, budget);
        return {...original,original};
      },
    });
    if (answer.status !== "answered") throw new Error("Provider request " + answer.status);
    const answerBytes = "ok" in answer.result ? answer.result.ok : answer.result.fault;
    log("extension reply", request.req.toString(), request.extensionId, answer.turns, answerBytes.length, json(decodePackValue(answerBytes))?.slice(0, 500));
    const { prologue, terminal } = guestAnswerPages(answerBytes);
    for (const page of prologue) operationAcks.push({ kind: "http-chunk", payload: { req: request.req, params: { bytes: Array.from(page), done: false } } });
    operationAcks.push({ kind: "completed", payload: { req: request.req, outcome: { tag: "ok" in answer.result ? "ok" : "fault", val: Array.from(terminal) } } });
  }
}

let lifecycle: any;

async function poll(label: string, event?: unknown, page?: unknown, batch?: readonly unknown[]): Promise<any> {
  const pending = patchAck ?? event;
  if ((patchAck || operationAcks.length > 0) && (event || batch)) {
    await poll("acknowledge UI patch");
    return poll(label, event, page, batch);
  }
  patchAck = undefined;
  const now = performance.now();
  const selected = pending ?? operationAcks.shift();
  const events = batch ?? (selected === undefined ? [] : [selected]);
  const turn = await api.poll(events, page, undefined, budget);
  if (batch && recordedMathStage === "delivering") recordedMathStage = "waiting-rearm";
  crossing += 1;
  const scan = scanTypedOperationPages(turn.effects);
  operationAcks.push(...scan.acknowledgements);
  for (const page of scan.pages) {
    log("typed page", crossing, page.lane, json(page.token), page.payload.length,
      scan.faults.length ? scan.faults.join(" ") : "");
  }
  if (scan.faults.length) throw new Error(label + " at crossing " + crossing + ": " + scan.faults.join(", "));
  const effects = scan.kept.map((effect: any) => {
    const bytes = shellFrameBytes(effect, instance);
    if (bytes) {
      const frame = decodeAppFrame(bytes);
      if (archiveFrames) archiveFrames.push(frame);
      const value = "Invocation" in frame && frame.Invocation.output.length
        ? decodePackValue(Uint8Array.from(frame.Invocation.output))
        : undefined;
      if (value && typeof value === "object" && "operationId" in value && typeof value.operationId === "string") lastOperation = value.operationId;
      if ("OperationCompleted" in frame) completedOperations.add(String(frame.OperationCompleted.operation));
      log("shell frame", crossing, Object.keys(frame)[0], json(value ?? frame)?.slice(0, 500));
      return { tag: "app-frame", frame: Object.keys(frame)[0] };
    }
    if (replayArchives && effect.tag === "load-document") {
      archives.push({parent_pack:Array.from(effect.val.docPack),parent_spr:Array.from(effect.val.spr),members:[]});
    }
    if (effect.tag === "invoke-extension") requests.push(wireExtensionInvocation(effect).invokeExtension);
    if (effect.tag === "dispatch-action") {
      const invocation = wireEffectToFriendly(effect, decodePackValue);
      if (invocation && typeof invocation === "object" && "dispatchAction" in invocation && invocation.dispatchAction?.action === "flowEvalTick") {
        const args = invocation.dispatchAction.args as Record<string, unknown>;
        if (recordedMathStage === "waiting-rearm" && args.windowId === editPreview && args.windowKindId === editPreview) {
          log("captured Math progress rearmed its actual Edit preview", crossing, json(args));
          recordedMathStage = "rearm-ready";
        }
        log("flow tick rearm", crossing, json(args));
        pendingTicks.push(args);
      }
    }
    return { tag: effect.tag, value: json(effect.val)?.slice(0, 330) };
  });
  const ui = turn.uiPatches.map((patch: any) => patch.surface);
  if (crossing <= 20 || crossing % 16 === 0 || turn.lifecycleReceipt || effects.length || ui.length) {
    log(label, crossing, Math.round(performance.now() - now), turn.status.tag, json(turn.commandIngress), "work", turn.retainedReceipt.workUnitsSpent.toString(), json(effects), json(ui), json(turn.lifecycleReceipt));
  }
  if (turn.lifecycleReceipt) lifecycle = decodeActorInstanceLifecycle(turn.lifecycleReceipt);
  if (turn.uiPatchReceipt) {
    const patch = turn.uiPatches[0];
    const surface = patch?.surface;
    if (!surface) throw new Error("UI patch receipt has no surface owner");
    patchAck = { kind: "patch-ack", payload: { surface, revision: patch.revision ?? 0n, receipt: decodeActorUiPatchReceipt(turn.uiPatchReceipt) } };
  }
  return turn;
}

const first = await poll("open", { kind: "instance-open", payload: {
  instance, purpose: { tag: "ordinary" }, activationGeneration: 1n, requestSequence: 1,
  appId: app.id, actor, config: [], assets: [], capabilities: [], quotas: encodePackValue({}),
  ...exactOpen?.input,
} });
for (let count = 0; count < 256 && !lifecycle; count += 1) await poll("opening continuation");
if (lifecycle?.kind !== "captured") throw new Error("compiled editor did not capture its real instance");
await poll("open receipt acknowledgement", { kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: lifecycle } });

const windows = app.windowKinds.map(({ id }: { id: string }) => ({ id, windowKindId: id }));
const editPreview = "procedural-preview";
const view = parseResolvedPluginViewState({
  locale: "en",
  terminology: "native",
  focusedWindowId: editPreview,
  activeModeId: app.defaultModeId,
  activeWindowKindId: editPreview,
  ...(process.env.SEMIO_NODE_PROBE_EXPLICIT_WINDOW === "1" ? { windowId: editPreview } : {}),
  windowInstances: windows,
});

async function command(id: string, args: Record<string, unknown>, action = false): Promise<boolean> {
  const savedTick = id === "flowEvalTick" && args.windowId === editPreview ? exactTick : undefined;
  if (savedTick) exactTick = undefined;
  const seq = savedTick ? savedTick.inspected.seq : sequence + 1;
  sequence = Math.max(sequence, seq);
  const address = action
    ? { pluginId, appId: app.id, modeId: app.defaultModeId, windowKindId: editPreview, windowInstanceId: editPreview, actionId: id }
    : { owner: { app: { pluginId, appId: app.id } }, commandId: id };
  const call = encodePackValue({ address, arguments: args });
  const encoded = encodeAppCommand({
    Command: { seq, command: Array.from(call), view_state: process.env.SEMIO_NODE_PROBE_EMPTY_VIEW === "1" ? [] : Array.from(encodePackValue(viewContextWireValue(view))) },
  });
  const pages = savedTick ? savedTick.pages.map((page: any) => {
    const cursor = { ...page.cursor, owner: BigInt(page.cursor.owner), generation: BigInt(page.cursor.generation), seq: BigInt(page.cursor.seq) };
    ingressSequence = cursor.seq;
    return { ...page, cursor, bytes: Uint8Array.from(page.bytes) };
  }) : createShardCommandIngressPages({
    owner: BigInt(instance), generation: 1n, commandIndex: 0, commandCount: 1,
    instance, seq: ++ingressSequence, command: encoded,
  });
  if (savedTick) log("replay exact browser view and tick page", seq, ingressSequence.toString(), pages[0].bytes.length);
  log("encoded command", id, seq, pages.length);
  let terminal: string | undefined;
  for (const page of pages) {
    let turn;
    for (let count = 0; count < 64; count += 1) {
      turn = await poll("offer " + id, undefined, page);
      if (turn.commandIngress.tag !== "backpressure") break;
    }
    if (turn?.commandIngress.tag === "fault") throw new Error(json(turn.commandIngress));
    terminal = turn?.commandIngress.tag;
    if (terminal === "backpressure") throw new Error("compiled requester refused command page " + id);
  }
  if (terminal === "command-complete") return true;
  for (let count = 0; count < 4096; count += 1) {
    const turn = await poll("settle " + id);
    if (turn.commandIngress.tag === "fault") throw new Error(json(turn.commandIngress));
    if (turn.commandIngress.tag === "command-complete" || turn.status.tag === "idle") return true;
  }
  log("command kept standing continuation", id);
  return false;
}

log("real open frontier", first.status.tag, json(lifecycle), "recorded clock", recordedClock, "scenario", scenario);
const profilePath = process.argv[5];
const continuationLimit = Number(process.env.SEMIO_NODE_PROBE_CONTINUATIONS ?? 192);
const setupLimit = Number(process.env.SEMIO_NODE_PROBE_SETUP_TURNS ?? 256);
if (!Number.isSafeInteger(setupLimit) || setupLimit <= 0) throw new Error("Invalid setup turn limit");
const inspector = new Session();
const post = (method: string, params = {}): Promise<any> => new Promise((done, fail) => inspector.post(method, params, (error, value) => error ? fail(error) : done(value)));
if (profilePath) {
  inspector.connect();
  await post("Profiler.enable");
  await post("Profiler.setSamplingInterval", { interval: 1000 });
  await post("Profiler.start");
}
async function applyEditorArchive() {
  const archive = archives.shift();
  if (!archive) throw Error("No authored load-document effect from the compiled editor");
  log("applying emitted document", archive.parent_pack.length, archive.parent_spr.length);
  await replayEditorDocument(archive, () => ++sequence, async (command, seq) => {
    archiveFrames = [];
    const pages = createShardCommandIngressPages({ owner: BigInt(instance), generation: 1n,
      commandIndex:0, commandCount:1, instance, seq:++ingressSequence, command:encodeAppCommand(command) });
    for (const page of pages) {
      let turn;
      do { turn = await poll("offer "+Object.keys(command)[0],undefined,page); }
      while (turn.commandIngress.tag === "backpressure");
      if (turn.commandIngress.tag === "fault") throw Error(json(turn.commandIngress));
    }
    for (let n=0; n<20000 && !archiveFrames.some(frame => Object.values(frame)[0]?.in_reply_to === seq); n++)
      await poll("archive command response");
    const result=archiveFrames; archiveFrames=undefined; return result;
  }, async () => {
    for (let n=0;n<64;n++) {
      const turn=await poll("advance native archive");
      if (turn.status.tag==="idle" && !patchAck) break;
    }
  });
  log("native editor accepted document", crossing);
}

async function drive(): Promise<boolean> {
  if ((scenario === "example" || scenario === "integrated") && !await command("setActiveExample", { exampleId: "hexagonal-mushroom-column", windowId: editPreview }, true)) return false;
  if (scenario === "integrated") {
    const genesis = lastOperation;
    for (let count = 0; count < setupLimit && genesis && !completedOperations.has(genesis); count += 1) await poll("settle example genesis");
    if (!genesis || !completedOperations.has(genesis)) throw new Error("Authored genesis did not publish");
    if (replayArchives) await applyEditorArchive();
    if (process.env.SEMIO_NODE_PROBE_SWITCH_ONLY === "1") {
      if (!await command("setActiveExample",{exampleId:"rectangle-wire-preview",windowId:editPreview},true)) return false;
      const operation=lastOperation;
      for (let n=0; n<20000 && !archives.length; n++) await poll("settle rectangle admission");
      await applyEditorArchive();
      log("settled rectangle original archive",operation);
      return true;
    }
    if (!providerRoot || !registryPath) throw new Error("Installed providers and registry are required");
    const rows = JSON.parse(await readFile(registryPath, "utf8"));
    for (const extensionId of ["math", "brep"]) {
      const row = rows.find((row: { pluginId: string }) => row.pluginId === "flow-extension-" + extensionId);
      if (!row) throw new Error("Installed extension row unavailable " + extensionId);
      const directory = join(resolve(providerRoot), row.directoryName);
      const manifest = JSON.parse(await readFile(join(directory, "🔣️.json"), "utf8")).manifest;
      providers.set(row.pluginId, { pluginId: row.pluginId, directory, manifest, seq: 0n });
    }
    if (process.env.SEMIO_NODE_PROBE_NO_CONTRIBUTIONS !== "1") {
      const loaded = [{ pluginId, manifest: descriptor.manifest }, ...providers.values()];
      const topics = consumedTopicsForApp(rows, loaded, pluginId, app.id);
      const contributionJson = scopeContributionsJson(loaded, pluginId, topics);
      log("selected host contributions", json(topics), contributionJson.length);
      if (!await command("setContributions", { json: contributionJson, page: 0, pageCount: 1 })) return false;
      const contributionOperation = lastOperation;
      for (let count = 0; count < setupLimit && contributionOperation && !completedOperations.has(contributionOperation); count += 1) await poll("settle contribution installation");
      if (!contributionOperation || !completedOperations.has(contributionOperation)) throw new Error("Contributions did not publish");
    }
    if (surfacePath) {
      const stockSurface = JSON.parse(await readFile(surfacePath, "utf8"));
      await poll("mount captured edit surface", { kind: "surface-visible", payload: {
        surface: stockSurface.binding.surface, bodyKey: stockSurface.binding.bodyKey, viewState: Uint8Array.from(stockSurface.pack),
      } });
    }
  }
  if (scenario !== "tick" && !await command("toolRunStart", { toolId: "previewEval", windowId: editPreview }, true)) return false;
  if (!await command("flowEvalTick", { windowId: editPreview, windowKindId: editPreview })) return false;
  if (process.env.SEMIO_NODE_PROBE_SWITCH_DURING_EVALUATION === "1") {
    if (!replayArchives) throw Error("Overlapping switch requires original archive delivery");
    if (!await command("setActiveExample",{exampleId:"rectangle-wire-preview",windowId:editPreview},true)) return false;
    for (let n=0; n<20000 && !archives.length; n++) await poll("settle live rectangle admission");
    await applyEditorArchive();
    log("settled rectangle archive during evaluation",crossing,requests.map(request=>request.req.toString()));
    return true;
  }
  for (let count = 0; count < continuationLimit; count += 1) {
    if (scenario === "integrated") {
      const oldRequests = requests.length;
      await invokePending();
      if (recordedMathStage === "proved") return true;
      if (oldRequests) { await poll("deliver extension completion"); continue; }
      const rearm = pendingTicks.shift();
      if (rearm) {
        if (recordedMathStage === "rearm-ready" && rearm.windowId === editPreview && rearm.windowKindId === editPreview) {
          recordedMathStage = "dispatching-rearm";
        }
        if (!await command("flowEvalTick", rearm)) return false;
        continue;
      }
    }
    const turn = await poll("post-tick continuation");
    if (turn.status.tag === "idle" && !patchAck) return !recordedMath;
  }
  return false;
}
try {
  if (!await drive()) process.exitCode = 2;
} finally {
  if (profilePath) {
    const { profile } = await post("Profiler.stop");
    inspector.disconnect();
    await writeFile(profilePath, JSON.stringify(profile));
  }
  log("real requester probe frontier", crossing, api.originalSource.fuelRemaining, json(api.originalSource.retainedRemaining), recordedMath ? recordedMathStage : "");
}
