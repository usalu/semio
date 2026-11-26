import { readFile, writeFile } from "node:fs/promises";
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
const surfacePath = process.env.SEMIO_NODE_PROBE_SURFACE;
const capturedHost = surfacePath ? dirname(surfacePath) : undefined;
const exactOpen = capturedHost ? JSON.parse(await readFile(join(capturedHost, "tick-open.json"), "utf8")) : undefined;
let exactTick = capturedHost ? JSON.parse(await readFile(join(capturedHost, "tick-ingress-1.json"), "utf8")) : undefined;
let ingressSequence = 1n;
const providers = new Map<string, { pluginId: string; directory: string; manifest: any; api?: any; seq: bigint }>();
let lastOperation: string | undefined;

async function invokePending(): Promise<void> {
  while (requests.length) {
    const request = requests.shift()!;
    const provider = providers.get(request.extensionId);
    if (!provider) throw new Error("No installed extension for exact request " + json(request));
    provider.api ??= await openBridge(provider.directory, provider.pluginId);
    log("extension request", request.req.toString(), request.extensionId, request.capability, request.requestJson);
    const answer = await driveInboundRequest({
      req: ++provider.seq, capability: request.capability, payload: new TextEncoder().encode(request.requestJson),
      originInstanceId: instance, submit: (events) => provider.api.poll(events, undefined, undefined, budget),
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

async function poll(label: string, event?: unknown, page?: unknown): Promise<any> {
  const pending = patchAck ?? event;
  if ((patchAck || operationAcks.length > 0) && event) {
    await poll("acknowledge UI patch");
    return poll(label, event, page);
  }
  patchAck = undefined;
  const now = performance.now();
  const selected = pending ?? operationAcks.shift();
  const events = selected === undefined ? [] : [selected];
  const turn = await api.poll(events, page, undefined, budget);
  crossing += 1;
  const scan = scanTypedOperationPages(turn.effects);
  operationAcks.push(...scan.acknowledgements);
  for (const page of scan.pages) {
    log("typed page", crossing, page.lane, json(page.token), page.payload.length,
      scan.faults.length ? scan.faults.join(" ") : "");
  }
  const effects = scan.kept.map((effect: any) => {
    const bytes = shellFrameBytes(effect, instance);
    if (bytes) {
      const frame = decodeAppFrame(bytes);
      const value = "Invocation" in frame && frame.Invocation.output.length
        ? decodePackValue(Uint8Array.from(frame.Invocation.output))
        : undefined;
      if (value && typeof value === "object" && "operationId" in value && typeof value.operationId === "string") lastOperation = value.operationId;
      if ("OperationCompleted" in frame) completedOperations.add(String(frame.OperationCompleted.operation));
      log("shell frame", crossing, Object.keys(frame)[0], json(value ?? frame)?.slice(0, 500));
      return { tag: "app-frame", frame: Object.keys(frame)[0] };
    }
    if (effect.tag === "invoke-extension") requests.push(wireExtensionInvocation(effect).invokeExtension);
    if (effect.tag === "dispatch-action") {
      const invocation = wireEffectToFriendly(effect, decodePackValue);
      if (invocation && typeof invocation === "object" && "dispatchAction" in invocation && invocation.dispatchAction?.action === "flowEvalTick") {
        const args = invocation.dispatchAction.args as Record<string, unknown>;
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
const inspector = new Session();
const post = (method: string, params = {}): Promise<any> => new Promise((done, fail) => inspector.post(method, params, (error, value) => error ? fail(error) : done(value)));
if (profilePath) {
  inspector.connect();
  await post("Profiler.enable");
  await post("Profiler.setSamplingInterval", { interval: 1000 });
  await post("Profiler.start");
}
async function drive(): Promise<boolean> {
  if ((scenario === "example" || scenario === "integrated") && !await command("setActiveExample", { exampleId: "hexagonal-mushroom-column", windowId: editPreview }, true)) return false;
  if (scenario === "integrated") {
    if (!providerRoot || !registryPath) throw new Error("Installed providers and registry are required");
    const genesis = lastOperation;
    for (let count = 0; count < 256 && genesis && !completedOperations.has(genesis); count += 1) await poll("settle example genesis");
    if (!genesis || !completedOperations.has(genesis)) throw new Error("Authored genesis did not publish");
    const rows = JSON.parse(await readFile(registryPath, "utf8"));
    for (const extensionId of ["math", "brep"]) {
      const row = rows.find((row: { pluginId: string }) => row.pluginId === "flow-extension-" + extensionId);
      if (!row) throw new Error("Installed extension row unavailable " + extensionId);
      const directory = join(resolve(providerRoot), row.directoryName);
      const manifest = JSON.parse(await readFile(join(directory, "🔣️.json"), "utf8")).manifest;
      providers.set(extensionId, { pluginId: row.pluginId, directory, manifest, seq: 0n });
    }
    const loaded = [{ pluginId, manifest: descriptor.manifest }, ...providers.values()];
    const topics = consumedTopicsForApp(rows, loaded, pluginId, app.id);
    const contributionJson = scopeContributionsJson(loaded, pluginId, topics);
    log("selected host contributions", json(topics), contributionJson.length);
    if (!await command("setContributions", { json: contributionJson, page: 0, pageCount: 1 })) return false;
    const contributionOperation = lastOperation;
    for (let count = 0; count < 256 && contributionOperation && !completedOperations.has(contributionOperation); count += 1) await poll("settle contribution installation");
    if (!contributionOperation || !completedOperations.has(contributionOperation)) throw new Error("Contributions did not publish");
    if (surfacePath) {
      const stockSurface = JSON.parse(await readFile(surfacePath, "utf8"));
      await poll("mount captured edit surface", { kind: "surface-visible", payload: {
        surface: stockSurface.binding.surface, bodyKey: stockSurface.binding.bodyKey, viewState: Uint8Array.from(stockSurface.pack),
      } });
    }
  }
  if (scenario !== "tick" && !await command("toolRunStart", { toolId: "previewEval", windowId: editPreview }, true)) return false;
  if (!await command("flowEvalTick", { windowId: editPreview, windowKindId: editPreview })) return false;
  for (let count = 0; count < continuationLimit; count += 1) {
    if (scenario === "integrated") {
      const oldRequests = requests.length;
      await invokePending();
      if (oldRequests) { await poll("deliver extension completion"); continue; }
      const rearm = pendingTicks.shift();
      if (rearm) {
        if (!await command("flowEvalTick", rearm)) return false;
        continue;
      }
    }
    const turn = await poll("post-tick continuation");
    if (turn.status.tag === "idle" && !patchAck) return true;
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
  log("real requester probe frontier", crossing, api.originalSource.fuelRemaining, json(api.originalSource.retainedRemaining));
}
