/** 🧭️ Observes actual Flow request, reply and retained publication boundaries in a stock editor page. */
import { chromium } from "playwright";
import { appendFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const base = process.env.SEMIO_CUSTODY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const maxMs = Number(process.env.SEMIO_CUSTODY_MS ?? 720000);
const pumpOnlyMs = Number(process.env.SEMIO_CUSTODY_PUMP_ONLY_MS ?? 0);
const capture = process.env.SEMIO_CUSTODY_CAPTURE === "1";
const started = Date.now();
const out = join(import.meta.dirname, "🗑️generated", `${process.env.SEMIO_CUSTODY_OUT ?? "react-eval-custody"}.ndjson`);
writeFileSync(out, "");
const write = (label, value) => {
  const record = JSON.stringify({ ms: Date.now() - started, label, value });
  appendFileSync(out, record + "\n");
  console.log("[DEBUG]", record);
};
const browserServer = await chromium.launchServer({ host: "127.0.0.1", port: 0, headless: true, args: ["--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
const deadline = setTimeout(() => void browserServer.kill(), maxMs + 15000);
const browser = await chromium.connect(browserServer.wsEndpoint());
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const hook = (path, rewrite) => context.route((url) => url.origin === new URL(base).origin && decodeURIComponent(url.pathname).endsWith(path), async (route) => {
  const response = await route.fetch();
  const source = await response.text();
  const body = rewrite(source);
  write("instrumentation", { path, matched: source !== body });
  const logger = 'const __g3dTrace = (tag, detail) => console.info("[DEBUG] EVAL " + JSON.stringify({ tag, detail }, (_, value) => typeof value === "bigint" ? String(value) : value));\n';
  await route.fulfill({ response, body: logger + body });
});

try {
  await hook("/🔌️extension-runtime/🟦️.ts", (source) => source
    .replace(/import \{ encodePackValue \} from/, "import { encodePackValue, decodePackValue } from")
    .replace("const { extensionId, capability, requestJson, req } = invocation;", `
      $&
      let request; try { const value = JSON.parse(requestJson); request = { operatorId: value.operatorId, nodeHash: /"nodeHash"\\s*:\\s*"?([0-9]+)"?/.exec(requestJson)?.[1], windowId: value.windowId, windowKindId: value.windowKindId, resume: value.resume, roundUnits: value.roundUnits }; } catch {}
      __g3dTrace("request-queued", { extensionId, capability, req, request, requester, original: ${capture} && String(req) === "1" ? requestJson : undefined });`)
    .replace("raw = await invoke.call", '__g3dTrace("callee-start", { req, extensionId, capability }); raw = await invoke.call')
    .replace("outcome = { ok: raw };", `
      $&
      let decoded; try { const body = raw.length <= 4096 ? decodePackValue(raw) : {}; decoded = { phase: body?.phase, done: body?.done, unitsDone: body?.unitsDone, unitsTotal: body?.unitsTotal, keys: Object.keys(body ?? {}), outputKeys: Object.keys(body?.output ?? {}) }; } catch (error) { decoded = { error: String(error) }; }
      __g3dTrace("callee-answer", { req, extensionId, capability, bytes: raw.length, prefix: Array.from(raw.slice(0, 12)), decoded, original: ${capture} && String(req) === "1" && raw.length <= 4096 ? Array.from(raw) : undefined });`)
    .replace("outcome = { fault: encodePackValue(fault) };", '$& __g3dTrace("callee-fault", { req, extensionId, code: fault.code, message: fault.message });')
    .replace("const response = await completion.complete(outcome);", `
      __g3dTrace("completion-start", { req, extensionId, capability }); 
      $&
      __g3dTrace("completion-result", { req, extensionId, capability, scope: response.uiScope, output: response.output, next: (response.requestedEffects ?? []).map((effect) => effect.dispatchAction?.action ?? effect.invokeExtension?.capability ?? (typeof effect === "object" ? Object.keys(effect)[0] : effect)) });`));
  await hook("/🧩️module-loader/🟦️.ts", (source) => source
    .replace(/return withTypedOperationCall\(actorId, `extension-completion#\$\{instanceId\}`/, `
      __g3dTrace("requester-ingress", { req, actorId, answerBytes: answer.length, events: events.map((event) => ({ kind: event.kind, request: event.payload.req, outcome: event.payload.outcome?.tag, bytes: event.payload.outcome?.val?.length ?? event.payload.params?.bytes?.length })), original: ${capture} && String(req) === "1" && answer.length <= 4096 ? events : undefined });
      $&`)
    .replace('return invocationFromFrames(frames.map(decodeAppFrame), effects, "extension completion");', `
      const decodedFrames = frames.map(decodeAppFrame);
      globalThis.__g3dCompletionAt ??= Date.now();
      __g3dTrace("requester-settle", { req, actorId, status: wireTurnStatusTag(settled.status), frames: decodedFrames.map((frame) => Object.keys(frame)), effects: effects.map((effect) => effect.tag) });
      return invocationFromFrames(decodedFrames, effects, "extension completion");`));
  await hook("/🚚️handle-turn-drive/🟦️.ts", (source) => source
    .replace('const inspected = inspectEncodedAppCommand(events);', `
      $&
      if (inspected.actionId === "flowEvalTick") __g3dTrace("typed-ingress-enter", { actorId, action: inspected.actionId, seq: inspected.seq });`)
    .replace(/return (settlePluginTurn\(actorId, await submitTurn\(actorId, \[\], \{ lane: "UserVisible", activation \}\),[^\n]+false, activation, call\));/, `
      const r = await $1;
      if (globalThis.__g3dCompletionAt !== undefined) {
        globalThis.__g3dPumpTurns = (globalThis.__g3dPumpTurns ?? 0) + 1;
        if (globalThis.__g3dPumpTurns <= 16 || globalThis.__g3dPumpTurns % 64 === 0) __g3dTrace("background-guest-turn", { actorId, ordinal: globalThis.__g3dPumpTurns, status: wireTurnStatusTag(r.status), effects: r.effects.map((effect) => effect.tag), patches: r.uiPatches.length });
      }
      return r;`)
    .replace("pendingCompletionEffects.set(instanceId, leftover);", `
      $&
      if (frames.length > 0 || leftover.length > 0) __g3dTrace("background-carry", { actorId, instanceId, status: wireTurnStatusTag(settled.status), frames: frames.map((frame) => Object.keys(decodeAppFrame(frame))), effects: leftover.map((effect) => ({ tag: effect.tag, action: effect.val?.action })) });`)
    .replace('if (outcome.stopped === "idle" && outcome.nextWake !== null && live()) {', `
      if (globalThis.__g3dCompletionAt !== undefined && outcome.stopped !== "budget") __g3dTrace("background-stop", { instanceId, outcome, carried: pendingCompletionEffects.get(instanceId)?.map((effect) => effect.tag) });
      $&`));
  await hook("/🚀️effect-dispatch/🟦️.ts", (source) => source
    .replace("if (!isCurrent()) return;", 'if (action === "flowEvalTick" || action === "toolRunStart") __g3dTrace("self-dispatch-start", { action, args, current: isCurrent() }); $&')
    .replace("const viewState = resolveViewState(baseSession);", `
      if (${pumpOnlyMs} > 0 && action === "flowEvalTick" && args?.windowId === "generation3d-generate-preview") {
        __g3dTrace("typed-deferral-start", { action, args, pumpOnlyMs: ${pumpOnlyMs} });
        while (isCurrent() && (globalThis.__g3dCompletionAt === undefined || Date.now() - globalThis.__g3dCompletionAt < ${pumpOnlyMs})) await new Promise((resolve) => setTimeout(resolve, 50));
        if (!isCurrent()) return;
        __g3dTrace("typed-deferral-end", { action, args });
      }
      $&`)
    .replace("if (isCurrent()) await applyResponse(response, targetSession);", `
      if (action === "flowEvalTick" || action === "toolRunStart") __g3dTrace("self-dispatch-return", { action, args, current: isCurrent(), isAppCommand, scope: response.uiScope, output: response.output, next: (response.requestedEffects ?? []).map((effect) => effect.dispatchAction?.action ?? effect.invokeExtension?.capability ?? (typeof effect === "object" ? Object.keys(effect)[0] : effect)) });
      $&`));
  await hook("/📡️operation-subscriptions/🟦️.ts", (source) => source
    .replace("settleOperation(completion);", '$& __g3dTrace("operation-completion", { id: completion.operation, revision: completion.revision, instanceId: completion.instanceId, scope: completion.uiScope, output: completion.terminalOutput, next: completion.requestedEffects?.map((effect) => effect.dispatchAction?.action ?? (typeof effect === "object" ? Object.keys(effect)[0] : effect)) });')
    .replace("const { scope, historyPatch } = operationProgressPartsV1(progress);", '$& if (scope.kind !== "none") __g3dTrace("operation-progress", { scope, historyCursor: historyPatch?.cursor, keys: Object.keys(progress) });'));
  await hook("/📨️messages/🟦️.ts", (source) => source.replace("if (!finite(timings.receivedAtEpochMs)) return;", `
    $&
    if ((String(timings.eventKinds).includes("completed") || String(actorId).includes("#request")) && timings.events > 0) __g3dTrace("worker-turn", { actorId, events: timings.events, eventKinds: timings.eventKinds, driveStopped: timings.driveStopped, driveSteps: timings.driveSteps, status: timings.status, patches: timings.patches, guestMs: Math.round(timings.guestLeftAtEpochMs - timings.guestEnteredAtEpochMs) });`));

  const page = await context.newPage();
  page.on("console", (message) => {
    const text = message.text();
    if (text.startsWith("[DEBUG] EVAL ")) {
      const event = JSON.parse(text.slice(13));
      const payload = event.detail?.original;
      const prefix = out.slice(0, -".ndjson".length);
      if (event.tag === "request-queued" && typeof payload === "string") writeFileSync(prefix + "-request.json", payload);
      if (event.tag === "callee-answer" && payload) writeFileSync(prefix + "-answer.pack", Uint8Array.from(payload));
      if (event.tag === "requester-ingress" && payload) writeFileSync(prefix + "-completed.json", JSON.stringify(payload, null, 2) + "\n");
      write("boundary", event);
    }
    else if (message.type() === "error" || /command stalled/.test(text)) write("console", text.slice(0, 900));
  });
  page.on("pageerror", (error) => write("page-error", String(error)));
  context.on("requestfinished", (request) => {
    const path = decodeURIComponent(new URL(request.url()).pathname);
    if (path.includes("/🧩️extension-modules/") && path.endsWith(".core.wasm")) void request.sizes().then((sizes) => write("extension-core", { path, bytes: sizes.responseBodySize })).catch(() => {});
  });
  const navigation = await page.goto(base, { waitUntil: "domcontentloaded", timeout: 90000 });
  if (!navigation?.ok()) throw new Error(`Stock editor returned ${navigation?.status()}`);
  while (Date.now() - started < maxMs) {
    const state = await page.evaluate(() => {
      const preview = document.querySelector('[data-surface-id="window:procedural-preview"]');
      const read = (name, fallback) => { try { return JSON.parse(preview?.getAttribute(name) ?? fallback); } catch { return null; } };
      const status = read("data-status-json", "{}");
      const counts = (read("data-meshes-json", "[]") ?? []).map(({ id, role, data }) => ({ id, role, triangles: (data?.indices?.length ?? 0) / 3 }));
      return { status, meshes: counts, vertices: Number(preview?.querySelector("canvas")?.dataset.worldGeometryVertices ?? 0) };
    });
    write("stock-preview", state);
    if (state.vertices > 0 && state.meshes.some((mesh) => mesh.role === "solid") && !state.status.progress?.inFlight) break;
    await page.waitForTimeout(12000);
  }
} finally {
  clearTimeout(deadline);
  await browserServer.kill();
}
