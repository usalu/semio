/** 🎛️ Note interaction probe: boots the note react playground (6080) and proves document mutations through the shell.
 * Block counts are the distinct `data-ink-block-id`s inside the Canvas window host, so counting never toggles a panel
 * (panel toggles land their own shell ledger entries, which mod+z would pop instead of the document edit).
 * Steps: (1) boot, (2) Canvas Actions pane `addBlock` (kind=text) → +1, (3) `addBlock` + immediate mod+z → back,
 * (4) Artifact panel quick-add row "Add Math" → +1 and a new tree row, (5) click a block tree row → aria-selected,
 * (6) Delete key (`deleteSelection`) → −1, (7) mod+z → restored, (8) Pencil, (9) export and reopen the document archive.
 * Usage: SEMIO_PROBE_OUT=note-interact-1 repo tools exec --light bun -- <ticket>/🐍️note-acceptance.mjs
 */
import { chromium } from "playwright";
import { useLinkedCapture, usePickerExperiment } from "./note-picker-experiment-host.mjs";
import assert from "node:assert/strict";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6080/?plugin=note&role=editor";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "note-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 15);
const extended = process.env.SEMIO_PROBE_EXTENDED === "1";
const pencilOnly = process.env.SEMIO_PROBE_PENCIL_ONLY === "1";
const cameraOnly = process.env.SEMIO_PROBE_CAMERA_ONLY === "1";
const transferOnly = cameraOnly || process.env.SEMIO_PROBE_TRANSFER_ONLY === "1";
const exercisePencil = !transferOnly && (pencilOnly || process.env.SEMIO_PROBE_WITHOUT_PENCIL !== "1");
const exerciseCamera = extended && (cameraOnly || !transferOnly && process.env.SEMIO_PROBE_WITHOUT_CAMERA !== "1");
const downloadTimeout = Number(process.env.SEMIO_PROBE_DOWNLOAD_TIMEOUT ?? 60000);
const textMarker = "Note acceptance round trip";
const archiveCellMarker = "Note archive table cell";
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
process.on("exit", () => writeFileSync(join(outDir, "console.txt"), lines.join("\n")));
const browser = await chromium.launch({ headless: true, downloadsPath: outDir, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
await usePickerExperiment(context, process.env.SEMIO_PROBE_PICKER_EXPERIMENT, new URL(url).origin);
await useLinkedCapture(context, process.env.SEMIO_PROBE_LINKED_CAPTURE, new URL(url).origin);
const archivePollDelay = Number(process.env.SEMIO_PROBE_ARCHIVE_POLL_DELAY);
if (Number.isSafeInteger(archivePollDelay) && archivePollDelay >= 0) {
  await context.route(url => decodeURIComponent(url.pathname).endsWith("/🗃️archive-load-host/🟦️.ts"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const timer = source.match(/setTimeout\(resolve,\s*\d+\)/)?.[0];
    assert.ok(timer, "Archive polling timer was not found");
    console.log("[DEBUG] archive poll interval for this browser case", archivePollDelay);
    await route.fulfill({response, body:source.replace(timer, `setTimeout(resolve, ${archivePollDelay})`)});
  });
}
if (process.env.SEMIO_PROBE_NATIVE_REFUSAL === "1") {
  await context.route(url => decodeURIComponent(url.pathname).endsWith("/🌉️bridge.js"), async (route) => {
    const response = await route.fetch();
    let source = await response.text();
    if (process.env.SEMIO_PROBE_ORIGINAL_GRANT_MEMORY === "1" || process.env.SEMIO_PROBE_WORKER_RECEIPT_MEMORY === "1") {
      const importComponent = "const { reactor, jobs, checkpoint, describe, codec } = await import(componentUrl.href);";
      source = source.replace(importComponent, `const originalInstantiate = WebAssembly.instantiate;
        globalThis.__noteWasmMemory = [];
        WebAssembly.instantiate = async (...args) => {
          const result = await originalInstantiate(...args);
          for (const value of Object.values((result.instance ?? result).exports)) {
            if (value instanceof WebAssembly.Memory) globalThis.__noteWasmMemory.push(value);
          }
          return result;
        };
        ${importComponent}
        WebAssembly.instantiate = originalInstantiate;`);
    }
    const eventlessTurns = Number(process.env.SEMIO_PROBE_EVENTLESS_TURNS);
    if (Number.isSafeInteger(eventlessTurns) && eventlessTurns > 0) {
      const poll = "result = await reactor.poll(slice);";
      const state = "const heldOutput = new Map();";
      assert.ok(source.includes(poll) && source.includes(state), "The guest bridge did not expose its polling entry");
      source = source.replace(state, `let noteDiagnosticTurns = 0; ${state}`).replace(poll, `${poll}
        if (actorId === "note#2") {
          if (commandPage) noteDiagnosticTurns = ${eventlessTurns};
          if (noteDiagnosticTurns > 0 && result.status?.tag === "idle") {
            noteDiagnosticTurns--;
            result = { ...result, status: { tag: "more-work" } };
          }
        }`);
    }
    const marker = "function nativeRefusalError(failure) {";
    assert.ok(source.includes(marker), "The guest bridge did not expose its refusal diagnostic entry");
    const diagnostic = `try { console.error("[DEBUG] native refusal", JSON.stringify(failure?.payload ?? failure, (_key, value) => typeof value === "bigint" ? String(value) : value).slice(0, 4400)); } catch {}`;
    await route.fulfill({ response, body: source.replace(marker, marker + diagnostic) });
  });
}
if (process.env.SEMIO_PROBE_WASM_STACK === "1") {
  await context.route(url => decodeURIComponent(url.pathname).endsWith("/🪟️preview2-shim/cli.js"), async (route) => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = `const reportGuestError = (line) => {
      if (line.includes("panicked at") || line.includes("root must be explicitly retired")) {
        Error.stackTraceLimit = 60;
        const positions = new Error(line).stack.split("\\n").filter(frame => frame.includes("wasm-function"));
        console.error("[DEBUG] guest panic stack", line, positions.map(frame => frame.trim().replace(/^.*(wasm-function.*)$/, "$1")).join(" | "));
      }
      console.error(line);
    };
`;
    await route.fulfill({ response, body: diagnostic + source.replaceAll("console.error(line)", "reportGuestError(line)") });
  });
}
if (process.env.SEMIO_PROBE_ARCHIVE_TRACE === "1") {
  await context.addInitScript((trackStock) => {
    const WorkerImpl = globalThis.Worker;
    globalThis.Worker = class ArchiveTraceWorker extends WorkerImpl {
      archiveTraceRequests = new Map();
      originalStock = new Map();
      trackReceipt(data) {
        if (!trackStock || data?.kind !== "original-turn-receipt") return;
        const receipt = data.receipt;
        if (!receipt?.spent) return;
        const key = data.actorId;
        let stock = this.originalStock.get(key);
        if (!stock) {
          stock = {count: 0, total: 0, maximum: 0, buckets: new Map()};
          this.originalStock.set(key, stock);
        }
        const bytes = Number(receipt.spent?.copiedBytes ?? 0);
        stock.count++;
        stock.total += bytes;
        stock.maximum = Math.max(stock.maximum, bytes);
        stock.buckets.set(bytes, (stock.buckets.get(bytes) ?? 0) + 1);
        const remaining = receipt.nativeRemaining?.maximumCopyBytes ?? receipt.nativeRemaining?.copyBytes;
        if (stock.count < 8 || stock.count % 2000 === 0 || (remaining !== undefined && Number(remaining) < 250000)) {
          const frequent = [...stock.buckets.entries()].sort((left, right) => right[1] - left[1]).slice(0, 6);
          console.info("[DEBUG] archive case original stock", key, JSON.stringify({ count: stock.count, total: stock.total,
            maximum: stock.maximum, bytes, remaining: String(remaining), grant: String(receipt.input?.grant?.maximumCopyBytes), frequent }));
        }
      }
      constructor(url, options) {
        super(url, options);
        this.archiveTraceEnabled = decodeURIComponent(String(url)).includes("shard-worker.js");
        if (!this.archiveTraceEnabled) return;
        this.addEventListener("message", ({data}) => {
          this.trackReceipt(data);
          const operation = this.archiveTraceRequests.get(data?.requestId);
          if (!operation) return;
          this.archiveTraceRequests.delete(data.requestId);
          console.info("[DEBUG] archive case worker result", data.requestId, operation, data.ok, data.error ?? data.value?.status?.tag ?? "",
            data.timings?.drivePolls ?? "", data.timings?.driveStopped ?? "", data.value?.commandIngress?.tag ?? "");
        });
      }
      postMessage(message, ...args) {
        const events = message?.events?.map(event => event.kind) ?? [];
        const actorId = message?.actorId;
        const track = this.archiveTraceEnabled && message?.requestId && (message.kind === "activate" ||
          (actorId !== undefined && events.length > 0 && (actorId !== "note#1" || events.includes("instance-close"))) ||
          (actorId === "note#2" && message.commandPage !== undefined));
        if (track) {
          const operation = [message.kind, actorId, ...(message.commandPage === undefined ? [] : ["command-page"]), ...events].join(" ");
          this.archiveTraceRequests.set(message.requestId, operation);
          console.info("[DEBUG] archive case worker sent", message.requestId, operation);
        }
        return super.postMessage(message, ...args);
      }
    };
  }, process.env.SEMIO_PROBE_ARCHIVE_STOCK === "1");
  await context.route(address => decodeURIComponent(address.pathname).endsWith("/🧳️archive-transfer/🟦️.ts"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const request = "const replyFrames = await this.ledger.send(step.command);";
    const marker = "progress?.(answer.status);";
    assert.ok(source.includes(marker) && source.includes(request), "The document archive exchange is not observable");
    const traced = source.replace("let completed = 0;", "let completed = 0, observed = 0, priorStatus;")
      .replace(request, `if (observed < 4 || observed % 128 === 0) {
        console.info("[DEBUG] archive case command", seq, Object.keys(step.command).join(","));
      }
      ${request}`)
      .replace(marker, `const status = JSON.stringify(answer.status);
        observed++;
        if (status !== priorStatus || observed % 128 === 0) {
          console.info("[DEBUG] archive case status", observed, status);
          priorStatus = status;
        }
        ${marker}`);
    await route.fulfill({ response, body: traced });
  });
  await context.route(address => {
    const path = decodeURIComponent(address.pathname);
    return path.includes("document-transfer-tabs/") && path.endsWith("scope/🟦️.tsx");
  }, async route => {
    const response = await route.fetch();
    let source = await response.text();
    const stages = [
      ["const task = trackDocumentTransfer(picked.name, owned.target.pluginId);", "decoded"],
      ["instanceId = await owned.handle.createApp(owned.target.app.id);", "create app"],
      ["await load(instanceId, archive, task.signal,", "load archive"],
      ["await core.adoptPreparedPrimarySession(owned.target.app.id,", "native load complete and standalone adoption requested"],
      ["const identity = await owner.handle.readAppDocumentIdentity(owner.session.instanceId);", "read predecessor document identity"],
      ["await keepOrReattachProgramDocument({ handle: owner.handle, app: owner.session.app }, owner.session.instanceId,", "keep predecessor archive"],
      ['notifyDocumentTransfer("imported", picked.name, "info");', "focused imported app"],
      ["if (instanceId !== null && !published) await owned.handle.destroyApp(instanceId)", "cleanup following failure", ", String(error), error?.stack"],
    ];
    for (const [marker, name, details = ""] of stages) {
      if (!source.includes(marker)) {
        console.log("[DEBUG] Archive trace marker not found", name);
        continue;
      }
      source = source.replace(marker, `console.info("[DEBUG] archive case stage ${name}"${details});${marker}`);
    }
    await route.fulfill({response, body: source});
  });
  await context.route(address => decodeURIComponent(address.pathname).endsWith("/🟠️switch-plugin-app/🟦️.ts"), async route => {
    const response = await route.fetch();
    let source = await response.text();
    const stages = [
      ["retirement = await admitProgramRetirementRef.current(owners, current, prepared?.retirement);", "request predecessor retirement"],
      ["const outcome = await sessionSwitchGateRef.current.run(", "enter session switch"],
      ["prepared?.onPublished();", "publish initialized session"],
    ];
    for (const [marker, stage] of stages) {
      if (!source.includes(marker)) {
        console.log("[DEBUG] Standalone adoption trace marker not found", stage);
        continue;
      }
      source = source.replace(marker, `if (prepared !== undefined) console.info("[DEBUG] archive case stage ${stage}");${marker}`);
    }
    await route.fulfill({response, body: source});
  });
  await context.route(address => decodeURIComponent(address.pathname).endsWith("/🟠️close-document/🟦️.ts"), async route => {
    const response = await route.fetch();
    let source = await response.text();
    const stages = [
      ["await (work?.persist ?? core.persistProgramDocumentRef.current)(owner, signal);", "persist predecessor", ""],
      ['setClosePrompt({ runtimeKey: owner.session.pluginId + "#" + owner.session.instanceId, name: owner.name, phase: "confirm",', "confirm predecessor persistence failure", ", String(failure)"],
    ];
    for (const [marker, stage, details] of stages) {
      if (!source.includes(marker)) {
        console.log("[DEBUG] Standalone retirement trace marker not found", stage);
        continue;
      }
      source = source.replace(marker, `if (work !== undefined) console.info("[DEBUG] archive case stage ${stage}"${details});${marker}`);
    }
    await route.fulfill({response, body: source});
  });
}
const tracePencil = process.env.SEMIO_PROBE_PENCIL_TRACE === "1";
if (tracePencil) {
  await context.addInitScript(() => {
    performance.setResourceTimingBufferSize(10_000);
    for (const type of ["pointerdown", "pointerup", "pointercancel", "lostpointercapture"]) {
      document.addEventListener(type, (event) => {
        const target = event.target instanceof Element ? event.target : null;
        const surface = target?.closest("[data-surface-id]")?.getAttribute("data-surface-id");
        const button = target?.closest("button")?.id;
        if (surface || button) console.info("[DEBUG] pencil trace DOM", JSON.stringify({ type, surface, button, trusted: event.isTrusted, x: event.clientX, y: event.clientY }));
      }, true);
    }
  });
  await context.route(address => {
    const path = decodeURIComponent(address.pathname);
    return (path.includes("/🖋️InkCanvasHost/") && (path.endsWith("/🔶️abort-gesture/🟦️.ts") || path.endsWith("/🎬️ink-gestures/🟦️.ts") || path.endsWith("/🧰️ink-session-state/🟦️.ts"))) ||
      path.endsWith("/🧰️utility-and-tool-activation/🟦️.ts") || path.endsWith("/⚡️host-effects/🟦️.ts") || path.endsWith("/🔁️contributions-refresh/🟦️.ts");
  }, async route => {
    const response = await route.fetch();
    let source = await response.text();
    const path = decodeURIComponent(route.request().url());
    if (!path.includes("/🖋️InkCanvasHost/")) console.log("[DEBUG] pencil trace module", path.split("/").slice(-2).join("/"));
    const traceBefore = (marker, trace) => {
      if (!source.includes(marker)) console.log("[DEBUG] Pencil trace marker missing", marker);
      source = source.replace(marker, trace + marker);
    };
    if (path.includes("/🧰️utility-and-tool-activation/")) {
      traceBefore("const forwarded = ", 'console.info("[DEBUG] pencil activation", JSON.stringify({windowId, utilityGuestWindowId, utilitySpawnedId, controller: action.controllerId, appController: utilitySession.app.controllerId, next, refreshedBody, viewState}));');
      traceBefore("applyHistoryPatch(response.historyPatch", 'console.info("[DEBUG] pencil activation response", JSON.stringify({uiScope:response.uiScope, effects:response.requestedEffects, current:isCurrentEffectOwner(primaryActionOwner)}));');
      traceBefore('return refuse("dispatch-failed", String(error));', 'console.info("[DEBUG] pencil activation failed", String(error));');
    } else if (path.includes("/⚡️host-effects/")) {
      traceBefore("if (isSpawnedPluginSession) {", 'if (windowViewInputBody) console.info("[DEBUG] pencil host refresh", JSON.stringify({isSpawnedPluginSession, refreshScope, windowViewInputBody, viewState:nextViewState}));');
    } else if (path.includes("/🔁️contributions-refresh/")) {
      traceBefore("const request = browserActorForSessionV1", 'if (Object.values(viewState.activeUtilityByWindowId ?? {}).some(Boolean)) console.info("[DEBUG] pencil refresh request", JSON.stringify({scope, windows:built?.windows, replaceBodies, viewState}));');
      traceBefore("applyUiRefreshResponseToCache(cache", 'if (Object.values(viewState.activeUtilityByWindowId ?? {}).some(Boolean)) console.info("[DEBUG] pencil refresh response", JSON.stringify((resolvedWindows ?? []).map(row => ({key: row.key, revision: row.revision, utility: JSON.stringify(row.value)?.match(/activeUtility.{0,25}/)?.[0]}))));');
    } else if (path.includes("/🧰️ink-session-state/")) {
      traceBefore("const windowUtility =", 'if (utilityRegister?.utilityId) console.info("[DEBUG] pencil register scope", JSON.stringify({surface: node.surfaceId, controller: node.controllerId, utilityRegister}));');
    } else if (path.includes("/🔶️abort-gesture/")) {
      const marker = "if (!rootRef.current || !doc || isNavigator || !interactive || isInkEditorTarget(event.target)) return;";
      if (!source.includes(marker)) console.log("[DEBUG] Pencil pointer marker missing");
      source = source.replace(marker, `console.info("[DEBUG] pencil trace pointer", JSON.stringify({utility, interactive, isNavigator, hasRoot: !!rootRef.current, hasDoc: !!doc, editor: isInkEditorTarget(event.target), trusted: event.isTrusted, button: event.button, pointerType: event.pointerType}));${marker}`);
    } else {
      const marker = "if (!node.controllerId) return;";
      if (!source.includes(marker)) console.log("[DEBUG] Pencil dispatch marker missing");
      source = source.replace(marker, `if (action !== "interactionHover") console.info("[DEBUG] pencil trace dispatch", JSON.stringify({action, controller: node.controllerId, surface: node.surfaceId, phase: args?.phase}));${marker}`);
    }
    await route.fulfill({ response, body: source });
  });
}
const page = await context.newPage();
let archiveLoadStarted = false;
let sampleArchiveWorkers;
let enableArchiveDebugging;
const nativeBreaks = process.env.SEMIO_PROBE_WASM_BREAKS?.split(",").map(Number) ?? [];
const traceArchiveStacks = process.env.SEMIO_PROBE_ARCHIVE_STACKS === "1";
if (nativeBreaks.length || traceArchiveStacks) {
  const session = await context.newCDPSession(page);
  const workers = new Set();
  const replies = new Map();
  let nextId = 0;
  const send = (sessionId, method, params = {}, onReply) => {
    const id = ++nextId;
    if (onReply) replies.set(id, onReply);
    return session.send("Target.sendMessageToTarget", {sessionId, message: JSON.stringify({id, method, params})})
      .catch(error => console.log("[DEBUG] native breakpoint dispatch", method, String(error)));
  };
  const request = (sessionId, method, params) => new Promise(resolve => {
    const timeout = setTimeout(resolve, 800);
    void send(sessionId, method, params, packet => {
      clearTimeout(timeout);
      resolve(packet?.result);
    });
  });
  const wasmLocal = async (sessionId, frame, name) => {
    const scope = frame.scopeChain.find(scope => scope.type === "local")?.object.objectId;
    if (!scope) return;
    const locals = await request(sessionId, "Runtime.getProperties", {objectId: scope, ownProperties: true});
    const pointer = locals?.result?.find(local => local.name === name)?.value?.objectId;
    if (!pointer) return;
    const value = await request(sessionId, "Runtime.getProperties", {objectId: pointer, ownProperties: true});
    return value?.result?.find(local => local.name === "value")?.value?.value;
  };
  const inspectWorkerReceipt = async (sessionId, frame) => {
    const address = await wasmLocal(sessionId, frame, "$var3");
    if (!Number.isSafeInteger(address)) return;
    const memory = await request(sessionId, "Runtime.evaluate", {returnByValue:true, expression:`(globalThis.__noteWasmMemory ?? []).map(memory => {
      const data = new DataView(memory.buffer);
      const word = (base, offset) => data.getUint32(base + offset, true);
      const words = (base, start, count) => Array.from({length: count}, (_, index) => word(base, start + index * 4));
      const base = ${address};
      const issuedPointer = word(base, 0);
      const progressPointer = word(base, 4);
      const valid = (pointer, count) => pointer > 0 && pointer + count * 4 < data.byteLength;
      return {base, issuedPointer, progressPointer, granted:words(base,44,5),
        issued:valid(issuedPointer,5) ? words(issuedPointer,0,5) : [],
        reported:valid(progressPointer,4) ? words(progressPointer,0,4) : [],
        outcome:words(base,64,5), before:words(base,8,4), beforeReserved:words(base,24,4),
        after:words(base,1728,4), afterReserved:words(base,1744,4)};
    })`});
    console.log("[DEBUG] native worker receipt memory", JSON.stringify(memory));
  };
  const inspectConstructionGrant = async (sessionId, frame) => {
    const locals = await Promise.all(Array.from({ length: 24 }, (_, index) => wasmLocal(sessionId, frame, "$var" + index)));
    const numeric = locals.filter(Number.isSafeInteger);
    const memory = await request(sessionId, "Runtime.evaluate", {returnByValue: true, expression: `(globalThis.__noteWasmMemory ?? []).map(memory => {
      const data = new DataView(memory.buffer);
      return ${JSON.stringify(numeric)}.map(address => address > 1024 && address + 96 <= data.byteLength ? {
        address, words: Array.from({length: 24}, (_, index) => data.getUint32(address + index * 4, true))
      } : address);
    })`});
    console.log("[DEBUG] native construction grant memory", JSON.stringify({locals, memory}));
  };
  const inspectOriginalGrant = async (sessionId, frame) => {
    const address = await wasmLocal(sessionId, frame, "$var1");
    if (!Number.isSafeInteger(address)) return;
    const memory = await request(sessionId, "Runtime.evaluate", {returnByValue: true, expression: `(globalThis.__noteWasmMemory ?? []).map(memory => {
      const data = new DataView(memory.buffer);
      const word = (base, offset) => data.getUint32(base + offset, true);
      const words = (base, start, count) => Array.from({length: count}, (_, index) => word(base, start + index * 4));
      const address = ${address};
      const progressPointer = word(address, 512);
      const source = words(address, 552, 5);
      const spent = words(progressPointer, 0, 4);
      const reserved = words(address, 208, 4);
      const start = words(address, 272, 4);
      const raw = source.map((amount, index) => index >= spent.length ? amount : Math.max(0, amount - Math.max(0, spent[index] - reserved[index] - start[index])));
      const budgetTag = word(address, 408);
      const budgetPointer = word(address, 412);
      return {address, progressPointer, source, spent, reserved, start, raw, budgetTag, budgetPointer,
        attached: budgetTag === 2 || !budgetPointer ? [] : words(budgetPointer, 0, 284)};
    })`});
    console.log("[DEBUG] native original grant memory", JSON.stringify(memory));
  };
  if (traceArchiveStacks) sampleArchiveWorkers = () => {
    for (const delay of [5_000, 12_000, 25_000]) {
      setTimeout(() => {
        if (page.isClosed()) return;
        for (const sessionId of workers) void send(sessionId, "Debugger.pause");
      }, delay).unref();
    }
  };
  const delayDebugger = process.env.SEMIO_PROBE_BREAK_AFTER_ARCHIVE_OPEN === "1";
  if (delayDebugger) enableArchiveDebugging = () => {
    for (const sessionId of workers) void send(sessionId, "Debugger.enable");
  };
  session.on("Target.attachedToTarget", ({sessionId, targetInfo}) => {
    if (targetInfo.type !== "worker" || !decodeURIComponent(targetInfo.url).includes("shard-worker.js")) return;
    workers.add(sessionId);
    if (!delayDebugger || archiveLoadStarted) void send(sessionId, "Debugger.enable");
  });
  session.on("Target.receivedMessageFromTarget", ({sessionId, message}) => {
    if (!workers.has(sessionId)) return;
    const packet = JSON.parse(message);
    const onReply = replies.get(packet.id);
    if (onReply) {
      replies.delete(packet.id);
      onReply(packet);
    }
    if (packet.method === "Debugger.scriptParsed" && packet.params.scriptLanguage === "WebAssembly") {
      for (const columnNumber of nativeBreaks) {
        const location = {scriptId: packet.params.scriptId, lineNumber: 0, columnNumber};
        void send(sessionId, "Debugger.setBreakpoint", {location});
      }
    }
    if (packet.result?.breakpointId || packet.error) console.log("[DEBUG] native breakpoint receipt", JSON.stringify(packet));
    if (packet.method === "Debugger.paused") {
      const {callFrames, hitBreakpoints = []} = packet.params;
      if (traceArchiveStacks && hitBreakpoints.length === 0) {
        console.log("[DEBUG] active archive worker stack", JSON.stringify(callFrames.slice(0, 32).map(frame => [frame.functionName, frame.location.lineNumber, frame.location.columnNumber])));
        void send(sessionId, "Debugger.resume");
        return;
      }
      const inInitializer = callFrames.some(frame => frame.functionName === "$func68275" || frame.functionName === "$func69298");
      if ((process.env.SEMIO_PROBE_BREAK_AFTER_ARCHIVE_OPEN === "1" && !archiveLoadStarted) ||
          (process.env.SEMIO_PROBE_BREAK_INITIALIZER_ONLY === "1" && !inInitializer)) {
        void send(sessionId, "Debugger.resume");
        return;
      }
      console.log("[DEBUG] native breakpoint stack", JSON.stringify(callFrames.map(frame => [frame.functionName, frame.location.lineNumber, frame.location.columnNumber])));
      const work = hitBreakpoints.map(breakpointId => send(sessionId, "Debugger.removeBreakpoint", {breakpointId}));
      if (process.env.SEMIO_PROBE_ORIGINAL_GRANT_MEMORY === "1" && callFrames[0]?.functionName === "$func10949") {
        work.push(inspectOriginalGrant(sessionId, callFrames[0]));
      }
      if (process.env.SEMIO_PROBE_CONSTRUCTION_GRANT_MEMORY === "1" && callFrames[0]?.functionName === "$func12111") {
        work.push(inspectConstructionGrant(sessionId, callFrames[0]));
      }
      if (process.env.SEMIO_PROBE_WORKER_RECEIPT_MEMORY === "1" && ["$func70277", "$func70322", "$func74776", "$func74791"].includes(callFrames[0]?.functionName)) {
        work.push(inspectWorkerReceipt(sessionId, callFrames[0]));
      }
      void Promise.all(work).then(() => send(sessionId, "Debugger.resume"));
    }
  });
  await session.send("Target.setAutoAttach", {autoAttach: true, waitForDebuggerOnStart: false, flatten: false});
}
if (process.env.SEMIO_PROBE_DIAGNOSTICS === "1") await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
page.on("console", (m) => {
  const content = m.text();
  if (content === "[DEBUG] archive case stage load archive" && !archiveLoadStarted) {
    archiveLoadStarted = true;
    enableArchiveDebugging?.();
    sampleArchiveWorkers?.();
  }
  if (process.env.SEMIO_PROBE_CLEANUP_TRACE_ONLY === "1" && /reactor more-work streak|reactor guest retained pressure|typed-operation phase/.test(content)) return;
  if (process.env.SEMIO_PROBE_DIAGNOSTICS === "1" && /cleanup continuation|cleanup semantic failure|cleanup.*zero-progress|runtime close.*phase/.test(content)) console.log(content.slice(0, 2300));
  if (process.env.SEMIO_PROBE_CAPTURE_ERRORS === "1" && m.type() === "error" && /role switch|surface-document/.test(content)) {
    const error = m.args().at(-1);
    void error?.evaluate(value => {
      const envelope = (error, depth) => error instanceof Error && depth < 3 ? {
        name: error.name, message: error.message, stack: error.stack?.slice(0, 2200),
        cause: error.cause === undefined ? undefined : envelope(error.cause, depth + 1),
        errors: error instanceof AggregateError ? [...error.errors].slice(0, 6).map(child => envelope(child, depth + 1)) : undefined,
      } : String(error).slice(0, 500);
      return envelope(value, 0);
    }).then(detail => console.log("[DEBUG] nested shell error", JSON.stringify(detail)), () => {});
  }
  if (content.startsWith("[DEBUG] pencil ")) console.log(content.slice(0, 2300));
  if (content.startsWith("[DEBUG] archive case") || process.env.SEMIO_PROBE_ARCHIVE_TRACE === "1" && /native refusal|archive.*refus|panic|trap|shell.documentTransfer.import-failed/i.test(content)) console.log(content.slice(0, 1500));
  if (process.env.SEMIO_PROBE_DIAGNOSTICS === "1" && /reactor more-work streak|publication-stalled|typed-operation setCamera|command stalled/.test(content)) console.log(content.slice(0, 1500));
  if ((m.type() === "log" || m.type() === "debug") && /\[(TRACE|DEBUG)\]/.test(content) && !/typed.operation|publication-stalled|reactor more-work streak|cleanup continuation|(?:document|artifact)[ -]archive|archive[ -](?:load|replay|replace)/i.test(content)) return;
  lines.push(`${Date.now() - t0} ${m.type()} ${content.slice(0, m.type() === "error" || m.type() === "warning" ? 6000 : 800)}`);
});
let bootstrapModuleError;
page.on("pageerror", (error) => {
  const message = String(error);
  lines.push(`${Date.now() - t0} pageerror ${message.slice(0, 2000)}`);
  if (/Failed to fetch dynamically imported module:|The requested module .* does not provide an export named/.test(message)) bootstrapModuleError = message;
});
const report = { url, exercisePencil, exerciseCamera, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not-ui-safe|missing-owned|refused|invalid-args|unsupported/.test(l)).map((l) => l.slice(0, 500));
const mod = process.platform === "darwin" ? "Meta" : "Control";
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1200)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const readState = () => page.evaluate((marker) => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const canvas = [...document.querySelectorAll('[data-surface-id="window:note-composite"]')].find(node => node.getClientRects().length > 0);
  const art = canvas?.querySelector(":scope > div.origin-top-left");
  const navigatorArt = document.querySelector('[data-surface-id="window:note-navigator"] > div.origin-top-left');
  const strokeSelector = 'svg path[stroke-linecap="round"][stroke-linejoin="round"]';
  const transform = art ? new DOMMatrix(art.style.transform) : null;
  const images = canvas ? [...canvas.querySelectorAll("[data-ink-block-id] img")].filter((image) => image.complete && image.naturalWidth > 0) : null;
  const rows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => ({ text: el.innerText.replace(/\s+/g, " ").trim().slice(0, 60), selected: el.getAttribute("aria-selected") }));
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, svg: el.querySelectorAll("svg *").length, text: (el.innerText ?? "").slice(0, 80) }));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready") || document.querySelector("[data-semio-os-ready]") !== null,
    error: document.querySelector("[data-semio-os-shell-error]")?.textContent?.trim() || document.documentElement.getAttribute("data-semio-os-error"),
    loadingPlugins: document.querySelector('[aria-label="Loading plugins…"]') !== null,
    blocks: canvas ? new Set([...canvas.querySelectorAll('[data-ink-block-id]')].map((el) => el.getAttribute('data-ink-block-id'))).size : null,
    hasTextMarker: Boolean(canvas && [...canvas.querySelectorAll("[data-ink-block-id]")].some((block) => block.textContent?.trim() === marker)),
    hasBoldMarker: Boolean(canvas && [...canvas.querySelectorAll("[data-ink-block-id] span")].some((run) => run.style.fontWeight === "bold" && run.textContent === marker)),
    images: images?.length ?? null,
    imageSizes: images?.map((image) => [image.naturalWidth, image.naturalHeight]) ?? null,
    tables: art?.querySelectorAll("[data-ink-block-id] table").length ?? null,
    tableCells: art ? [...art.querySelectorAll("[data-ink-block-id] table tbody td")].map(cell => cell.textContent ?? "") : [],
    math: art?.querySelectorAll("[data-ink-block-id] .ink-math").length ?? null,
    strokes: art?.querySelectorAll(strokeSelector).length ?? null,
    navigatorStrokes: navigatorArt?.querySelectorAll(strokeSelector).length ?? null,
    zoom: transform?.a ?? null,
    cameraOffset: transform ? [transform.e, transform.f] : null,
    noScene: (body.match(/No scene/g) ?? []).length,
    docBlocks: (() => { const row = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim()).find((t) => /^Blocks \d+$/.test(t)); return row ? Number(row.split(" ")[1]) : null; })(),
    utility: [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim()).find((t) => /^Utility /.test(t)) ?? null,
    rows,
    hosts,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    example: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
  };
}, textMarker);
const state = async () => {
  for (let attempt = 0; ; attempt++) {
    try { return await readState(); }
    catch (error) {
      if (attempt >= 3 || !String(error).includes("Execution context was destroyed")) throw error;
      await page.waitForLoadState("domcontentloaded");
    }
  }
};
const settle = async (predicate) => { let after = null; for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
const togglePanel = async (label) => { const tab = page.locator(`button:has-text("${label}")`).first(); return (await tab.count()) ? tab.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const submitAction = async (actionId, args) => {
  const from = lines.length;
  const before = await state();
  const engagement = before.engagements.find((id) => /composite|canvas/i.test(id)) ?? before.engagements[0];
  let toggled = "already-open";
  if (!before.actionRows.length) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120)) : `no-toggle:${engagement}`;
    await page.waitForTimeout(1200);
  }
  const opened = await state();
  let clicked = "absent";
  if (await page.locator(`[id$=".action.${actionId}.execute"]`).count()) clicked = "already-expanded";
  else if (opened.actionRows.includes(`action.${actionId}`)) clicked = await page.locator(`[id="action.${actionId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  const form = page.locator('[data-slot="window-action-pane"]').filter({ has: submit }).first();
  for (const [key, value] of Object.entries(args)) {
    const control = form.locator(`[id$=".action.${actionId}.arg.${key}.input"]`).first();
    if (!await control.waitFor({ state: "visible", timeout: 12_000 }).then(() => true, () => false)) {
      const fields = await form.evaluate(root => [...root.querySelectorAll('input,textarea,select,[role="combobox"]')].map(element => element.id)).catch(() => []);
      console.log("[DEBUG] action form argument unavailable", JSON.stringify({actionId, key, fields}));
      return { from, before: before.blocks, engagement, toggled, clicked, filled: [...filled, `${key}:absent`], submitted: "missing-argument-control", actionRows: opened.actionRows.slice(0, 40) };
    }
    if (await control.evaluate(element => element instanceof HTMLSelectElement)) {
      await control.selectOption(String(value));
    } else if (await control.getAttribute("role") === "combobox") {
      const label = String(value).replace(/^./, c => c.toUpperCase());
      if ((await control.innerText()).trim() !== label) {
        await control.click({ timeout: 8000 });
        await page.getByRole("option", { name: label, exact: true }).click({ timeout: 8000 });
      }
    } else {
      await control.fill(String(value));
    }
    filled.push(`${key}=${value}`);
  }
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  return { from, before: before.blocks, engagement, toggled, clicked, filled, submitted, actionRows: opened.actionRows.slice(0, 40) };
};

const canvas = () => page.locator('[data-surface-id="window:note-composite"]').first();
const blurEditorOnCanvas = async (host) => {
  const point = await host.evaluate((surface) => {
    const bounds = surface.getBoundingClientRect();
    const left = Math.max(0, bounds.left);
    const right = Math.min(innerWidth, bounds.right);
    const top = Math.max(0, bounds.top);
    const bottom = Math.min(innerHeight, bounds.bottom);
    for (const horizontal of [0.78, 0.64, 0.50, 0.34, 0.22]) {
      for (const vertical of [0.72, 0.82, 0.55, 0.34, 0.22]) {
        const x = left + (right - left) * horizontal;
        const y = top + (bottom - top) * vertical;
        const hit = document.elementFromPoint(x, y);
        if (hit?.closest("[data-surface-id]") === surface &&
            !hit.closest("[data-ink-block-id], [data-ink-editor-overlay], button, input, textarea, [role=toolbar]")) return { x, y };
      }
    }
    return null;
  });
  assert.ok(point, "The Canvas has no visible background outside the editor");
  await page.mouse.click(point.x, point.y);
};
const wheelCanvas = async () => {
  const host = canvas();
  const initial = await state();
  const from = lines.length;
  const point = { x: 64, y: 128 };
  await host.hover({ position: point, timeout: 8000 });
  await page.mouse.wheel(0, -120);
  const zoomed = await settle((x) => Number.isFinite(x.zoom) && x.zoom > initial.zoom);
  const expectedOffset = zoomed.zoom > initial.zoom && initial.cameraOffset && zoomed.cameraOffset
    ? [point.x, point.y].map((coordinate, axis) => coordinate - (coordinate - initial.cameraOffset[axis]) * zoomed.zoom / initial.zoom)
    : null;
  const anchorError = expectedOffset && Math.hypot(...expectedOffset.map((value, axis) => value - zoomed.cameraOffset[axis]));
  await note("zoom-canvas", { before: initial.zoom, after: zoomed.zoom, anchorError, zoomed: zoomed.zoom > initial.zoom && anchorError !== null && anchorError < 1 }, from);
  return { zoom: zoomed.zoom, offset: zoomed.cameraOffset };
};

const importDocumentArchive = async (archivePath) => {
  const artifactPanel = page.locator('[data-slot="panel"][data-active-tab-id="framework.panel.artifact"][data-panel-visible="true"]').first();
  if (await artifactPanel.isVisible()) {
    await togglePanel("Artifact");
    await artifactPanel.waitFor({ state: "hidden", timeout: 10000 });
  }
  const actionToggle = page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first();
  if (!(await state()).actionRows.length) await actionToggle.click();
  const chooser = page.waitForEvent("filechooser", { timeout: 10000 });
  await page.locator('[id="action.importArtifactDocument"]').first().click();
  await (await chooser).setFiles(archivePath);
};

const allowEmptyDemoReplacement = (before) => {
  if (process.env.SEMIO_PROBE_IMPORT_REPLACE_EMPTY_FIXTURE !== "1") return false;
  const assertEmptyDemo = (value) => {
    assert.equal(value.ready, "note", "The fixture only replaces its own standalone Note session");
    assert.equal(value.example, "Demo", "The fixture only replaces the fresh Demo");
    assert.equal(value.blocks, 0, "The fixture only replaces a predecessor without blocks");
    assert.equal(value.strokes, 0, "The fixture only replaces a predecessor without ink");
  };
  assertEmptyDemo(before);
  const closeFixture = page.getByRole("button", { name: "Close and discard", exact: true });
  void closeFixture.waitFor({ state: "visible", timeout: settleSeconds * 1000 }).then(async () => {
    assertEmptyDemo(await state());
    await closeFixture.click({ timeout: 5000 });
    console.log("[DEBUG] Closed the empty fixture predecessor after explicit confirmation");
  }).catch((error) => {
    if (error.name !== "TimeoutError" && !page.isClosed()) console.log("[DEBUG] Fixture predecessor replacement refused", String(error));
  });
  return true;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.blocks !== null && i > 10) break; if (s.error || bootstrapModuleError) break; }
await page.waitForTimeout(4000);
s = await state();
await note("boot", s, 0);
if (s.error || !s.ready || s.blocks === null) {
  await browser.close();
  throw new Error(s.error ? `Note boot failed: ${s.error}` : bootstrapModuleError ?? (s.loadingPlugins ? "Note boot stalled while loading plugins" : "Note canvas did not mount"));
}
if (process.env.SEMIO_PROBE_IMPORT) {
  const from = lines.length;
  const mixed = process.env.SEMIO_PROBE_IMPORT_MIXED === "1";
  const restored = (value) => mixed
    ? value.blocks === 4 && value.math === 1 && value.hasBoldMarker && value.tableCells.includes(archiveCellMarker)
      && value.imageSizes.some(([width, height]) => width === 12 && height === 8)
    : value.blocks === 1 && value.math === 1;
  await importDocumentArchive(process.env.SEMIO_PROBE_IMPORT);
  if (process.env.SEMIO_PROBE_IMPORT_CANCEL === "1") {
    const tasks = page.getByRole("button", { name: "Tasks", exact: true }).first();
    await tasks.click({ timeout: 10000 });
    const transfer = page.locator('[data-semio-task-manager-lane="documentTransfer"]').first();
    await transfer.waitFor({ state: "visible", timeout: 15000 });
    const progress = transfer.getByRole("progressbar");
    const progressState = await progress.getAttribute("aria-busy");
    await transfer.locator('[data-semio-task-manager-cancel^="documentTransfer:"]').click({ timeout: 10000 });
    await transfer.waitFor({ state: "hidden", timeout: 45000 });
    await tasks.click({ timeout: 10000 });
    const after = await state();
    const initialPreserved = after.blocks === s.blocks && after.example === s.example && !after.error;
    const action = await submitAction("addBlock", { kind: "text", x: 320, y: 180 });
    const edited = await settle(value => value.blocks === s.blocks + 1);
    const editable = action.submitted === "ok" && edited.blocks === s.blocks + 1;
    await note("cancel-import", { progressState, initialPreserved, editable }, from);
    await browser.close();
    assert.equal(progressState, "true", "Archive import did not expose its progress");
    assert.equal(initialPreserved, true, "Cancelling archive import changed the open document");
    assert.equal(editable, true, "The open document could not be edited after archive cancellation");
    console.log("[DEBUG] CANCEL IMPORT DONE");
    process.exit(0);
  }
  const replaceEmptyFixture = allowEmptyDemoReplacement(s);
  const reopened = await settle((value) => restored(value) || lines.slice(from).some(line => line.includes("shell.documentTransfer.import-failed")));
  await note("focused-import", { ...reopened, restored: restored(reopened) }, from);
  if (replaceEmptyFixture && restored(reopened)) {
    const edit = await submitAction("addBlock", { kind: "text", x: 320, y: 180 });
    const result = await settle(value => value.blocks === reopened.blocks + 1 && value.math === reopened.math);
    const editable = edit.submitted === "ok" && result.blocks === reopened.blocks + 1 && result.math === reopened.math;
    await note("focused-import-edit", { editable, blocks: result.blocks, math: result.math }, from);
    assert.equal(editable, true, "The restored standalone document could not be edited");
  }
  await browser.close();
  assert.equal(restored(reopened), true, "The archived Note content did not reopen");
  console.log("[DEBUG] IMPORT DONE");
  process.exit(0);
}
let count = s.blocks;
if (process.env.SEMIO_PROBE_HISTORY_ROUND_TRIP === "1") {
  const from = lines.length;
  const added = await submitAction("addBlock", { kind: "text", x: 315, y: 180 });
  assert.equal(added.submitted, "ok");
  assert.equal((await settle(value => value.blocks === 1)).blocks, 1);
  await page.keyboard.press(`${mod}+z`);
  assert.equal((await settle(value => value.blocks === 0)).blocks, 0);
  await page.keyboard.press(`${mod}+Shift+z`);
  assert.equal((await settle(value => value.blocks === 1)).blocks, 1);
  if ((await state()).actionRows.length === 0) {
    await page.locator('[id="framework.window.noteComposite.engagement.toggle"]').click();
  }
  const download = page.waitForEvent("download", { timeout: downloadTimeout });
  await page.locator('[id="action.exportArtifactDocument"]').first().click();
  const archive = join(outDir, "note-history.semio-archive");
  await (await download).saveAs(archive);
  await note("history-fixture-exported", { blocks: 1 }, from);
  await page.reload({ waitUntil: "domcontentloaded" });
  const demo = await settle(value => value.ready && value.blocks === 0 && value.example === "Demo");
  assert.equal(demo.blocks, 0);
  await importDocumentArchive(archive);
  allowEmptyDemoReplacement(demo);
  const reopened = await settle(value => value.blocks === 1);
  await note("history-fixture-reopened", { restored: reopened.blocks === 1 }, from);
  await browser.close();
  assert.equal(reopened.blocks, 1, "Note could not reopen the text block after Undo and Redo");
  process.exit(0);
}
if (process.env.SEMIO_PROBE_VIEWER_ONLY === "1") {
  const from = lines.length;
  const added = await submitAction("addBlock", { kind: "text", x: 300, y: 180 });
  const prepared = await settle(value => value.blocks === count + 1);
  assert.equal(prepared.blocks, count + 1, "Note did not create the block for the viewer");
  const viewer = page.locator('[id="playground.navbar.roles"] [data-role="viewer"]');
  await viewer.click({timeout: 15000});
  const roleSelected = await page.waitForFunction(() => document.querySelector('[id="playground.navbar.roles"] [data-role="viewer"]')?.getAttribute("aria-pressed") === "true", undefined, {timeout: 90000}).then(() => true, () => false);
  if (!roleSelected) {
    const ui = await viewer.evaluate(control => ({pressed: control.getAttribute("aria-pressed"), busy: control.closest('[id="playground.navbar.roles"]')?.getAttribute("aria-busy"), text: document.body.innerText.slice(-1800)}));
    await note("viewer-switch-stalled", { ...ui, state: await state() }, from);
    await browser.close();
    assert.equal(roleSelected, true, "Note did not finish switching to Viewer");
  }
  const viewerCanvas = page.locator('[data-surface-id="window:note-view-composite"], [data-surface-id="note.view.composite"]').first();
  const viewerBlock = viewerCanvas.locator("[data-ink-block-id]").last();
  await viewerBlock.dblclick({timeout: 15000});
  const details = { submitted: added.submitted, blocks: await viewerCanvas.locator("[data-ink-block-id]").count(), editorControls: await viewerCanvas.locator('[contenteditable="true"]').count() };
  await note("read-only-viewer", details, from);
  await browser.close();
  assert.equal(details.blocks, prepared.blocks);
  assert.equal(details.editorControls, 0);
  process.exit(0);
}
if (process.env.SEMIO_PROBE_BLOCK_CLIPBOARD_ONLY === "1") {
  const from = lines.length;
  const action = await submitAction("addBlock", { kind: "text", x: 310, y: 180 });
  assert.equal(action.submitted, "ok");
  const initial = await settle((value) => value.blocks === count + 1);
  assert.equal(initial.blocks, count + 1, "Clipboard fixture block did not publish");
  await page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first().click();
  const host = canvas();
  const block = host.locator("[data-ink-block-id]").first();
  await block.click();
  await host.hover({ position: { x: 64, y: 128 } });
  await host.locator('[data-ink-block-id][data-shared-border-state="active"]').first().waitFor({ timeout: 12000 });
  await host.focus();
  const copied = await host.evaluate((root) => {
    const clipboardData = new DataTransfer();
    const copy = !root.dispatchEvent(new ClipboardEvent("copy", { clipboardData, bubbles: true, cancelable: true }));
    const copyText = clipboardData.getData("text/plain");
    const paste = !root.dispatchEvent(new ClipboardEvent("paste", { clipboardData, bubbles: true, cancelable: true }));
    return { copy, payloadBytes: copyText.length, paste };
  });
  const pasted = await settle((value) => value.blocks === count + 2);
  await note("copy-paste-block", { ...copied, before: initial.blocks, after: pasted.blocks }, from);
  await browser.close();
  assert.equal(copied.copy, true, "Note did not copy the selected block");
  assert.ok(copied.payloadBytes > 0, "Note's clipboard payload was empty");
  assert.equal(copied.paste, true, "Note did not accept its own clipboard payload");
  assert.equal(pasted.blocks, count + 2, "Pasted block did not reach the Canvas");
  assert.deepEqual(faultLines(from), [], "Copy and paste produced a runtime fault");
  console.log("[DEBUG] BLOCK CLIPBOARD DONE");
  process.exit(0);
}
const resizeOnly = process.env.SEMIO_PROBE_RESIZE_ONLY === "1";
if (resizeOnly || process.env.SEMIO_PROBE_GEOMETRY_ONLY === "1") {
  const fixtures = resizeOnly ? [["text", 210, 160]] : [["text", 210, 160], ["text", 590, 320]];
  for (const [kind, x, y] of fixtures) {
    const action = await submitAction("addBlock", { kind, x, y });
    assert.equal(action.submitted, "ok", "Could not submit the geometry fixture");
    const current = await settle((value) => value.blocks === count + 1);
    assert.equal(current.blocks, count + 1, "Geometry fixture block did not publish");
    const positions = await canvas().locator("[data-ink-block-id]").evaluateAll(elements => elements.map(element => [
      Number.parseFloat(element.style.left), Number.parseFloat(element.style.top),
    ]));
    await note("geometry-fixture", { coordinates: [x, y], filled: action.filled, positions }, action.from);
    assert.deepEqual(positions.at(-1), [x, y], "Geometry fixture did not retain the submitted coordinates");
    count = current.blocks;
  }
  await page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first().click();
  const host = canvas();
  const blocks = host.locator("[data-ink-block-id]");
  const navigatorBlocks = page.locator('[data-surface-id="window:note-navigator"] [data-ink-block-id]');
  const layout = (items = blocks) => items.evaluateAll((elements) => elements.map((element) => {
    const { left, top, width, height } = element.style;
    return [left, top, width, height].map(parseFloat);
  }));
  const untilLayout = async (matches, items = blocks) => {
    let current;
    for (let i = 0; i < settleSeconds * 2; i++) {
      await page.waitForTimeout(500);
      current = await layout(items);
      if (matches(current)) break;
    }
    return current;
  };
  const drag = async (box, dx, dy, steps = 8) => {
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width / 2 + dx, box.y + box.height / 2 + dy, { steps });
    await page.mouse.up();
  };
  const selected = host.locator('[data-ink-block-id][data-shared-border-state="active"]');
  const tolerance = (a, b) => Math.abs(a - b) < 1;
  const pickFirst = async () => {
    await blocks.nth(0).click();
    await host.hover({ position: { x: 64, y: 128 } });
    await selected.first().waitFor({ timeout: 12000 });
  };
  if (!resizeOnly) {
    const initial = await layout();
    const geometryStart = lines.length;
    const groupBounds = host.locator(":scope > div.z-20.pointer-events-none").first();
    await pickFirst();
    await blocks.nth(1).click({ modifiers: ["Shift"] });
    await host.hover({ position: { x: 64, y: 128 } });
    await selected.nth(1).waitFor({ timeout: 12000 });
    const groupWidth = await groupBounds.boundingBox().then((box) => box?.width ?? null);
    await note("select-geometry-group", { selected: await selected.count(), groupWidth }, geometryStart);
    const translated = (value) => value.length === initial.length && value.every(([left, top], index) =>
      tolerance(left, initial[index][0] + 68) && tolerance(top, initial[index][1] + 41));
    await drag(await blocks.first().boundingBox(), 68, 41);
    const moved = await untilLayout(translated);
    const publishedMove = await untilLayout(translated, navigatorBlocks);
    await note("move-group", { initial, moved, publishedMove, groupWidth, movedBoth: translated(publishedMove) }, geometryStart);
    await host.focus();
    await page.keyboard.press(`${mod}+z`);
    const same = (value) => JSON.stringify(value) === JSON.stringify(initial);
    const undone = await untilLayout(same, navigatorBlocks);
    const canvasUndone = await untilLayout(same);
    await note("undo-move-group", { navigatorRestored: same(undone), canvasRestored: same(canvasUndone) }, geometryStart);
    assert.equal(translated(publishedMove), true, "Dragging did not publish new positions for both selected blocks");
    assert.equal(same(undone) && same(canvasUndone), true, "Undo did not restore both positions in both views");
    await host.click({ position: { x: 64, y: 128 } });
    await host.locator(':scope > div.z-20.pointer-events-none').first().waitFor({ state: "hidden", timeout: 12000 });
  }
  await pickFirst();
  await selected.nth(1).waitFor({ state: "hidden", timeout: 12000 });
  const corner = host.locator(':scope > div.z-30[style*="nwse-resize"]').last();
  const resizeStart = lines.length;
  const cornerBox = await corner.boundingBox();
  assert.ok(cornerBox, "The selected block did not show a resize handle");
  const before = await layout();
  await drag(cornerBox, 52, 27, Number(process.env.SEMIO_PROBE_RESIZE_STEPS ?? 8));
  const grew = (value) => value.length === before.length
    && tolerance(value[0][2], before[0][2] + 52) && tolerance(value[0][3], before[0][3] + 27)
    && JSON.stringify(value.slice(1)) === JSON.stringify(before.slice(1));
  const resized = await untilLayout(grew);
  const publishedResize = await untilLayout(grew, navigatorBlocks);
  await note("resize-block", { before, resized, publishedResize, grew: grew(publishedResize) }, resizeStart);
  await host.focus();
  await page.keyboard.press(`${mod}+z`);
  const atOriginalSize = (value) => JSON.stringify(value) === JSON.stringify(before);
  const originalSize = await untilLayout(atOriginalSize, navigatorBlocks);
  const originalCanvasSize = await untilLayout(atOriginalSize);
  await note("undo-resize", { originalSize, navigatorRestored: atOriginalSize(originalSize), canvasRestored: atOriginalSize(originalCanvasSize) }, resizeStart);
  await browser.close();
  assert.equal(grew(publishedResize), true, "The resize handle did not publish larger block dimensions");
  assert.equal(atOriginalSize(originalSize) && atOriginalSize(originalCanvasSize), true, "Undo did not restore the block dimensions in both views");
  console.log("[DEBUG] GEOMETRY DONE");
  process.exit(0);
}

if (!pencilOnly && !transferOnly) {
  {
    const a = await submitAction("addBlock", { kind: "text", x: 360, y: 130 });
    let after = await settle((x) => x.blocks === count + 1);
    if (after.blocks !== count + 1) after = await settle((x) => x.blocks === count + 1);
    await note("add-block-text", { ...a, after: after.blocks, added: a.submitted === "ok" && after.blocks === count + 1 }, a.from);
    if (process.env.SEMIO_PROBE_TEXT_ONLY === "1") {
      await browser.close();
      assert.equal(a.submitted === "ok" && after.blocks === count + 1, true, "The Text block did not publish");
      console.log("[DEBUG] TEXT DONE");
      process.exit(0);
    }
    count = after.blocks ?? count;
  }
  {
    const tables = (await state()).tables;
    const a = await submitAction("addBlock", { kind: "table", x: 360, y: 360 });
    const created = await settle((x) => x.blocks === count + 1 && x.tables === tables + 1);
    if (created.blocks === count + 1 && created.tables === tables + 1) {
      const from = lines.length;
      const host = page.locator('[data-surface-id="window:note-composite"]').first();
      const cell = host.locator("[data-ink-block-id] table tbody td").first();
      const editor = host.locator("#ink-table-cell-editor");
      const marker = "Note acceptance table cell";
      const edited = await cell.dblclick({ timeout: 8000 }).then(() => true).catch(() => false);
      const visible = edited && await editor.waitFor({ state: "visible", timeout: 8000 }).then(() => true).catch(() => false);
      let entered = null;
      if (visible) {
        await editor.fill(marker);
        entered = await editor.inputValue();
        await editor.press("Enter");
        const next = host.locator("#ink-table-cell-editor");
        if (await next.count()) await next.press("Escape");
      }
      const containsMarker = (x) => x.tableCells.includes(marker);
      const committed = containsMarker(await settle(containsMarker));
      if (committed) await page.keyboard.press(`${mod}+z`);
      const undone = committed && !(await settle((x) => x.tables === tables + 1 && !containsMarker(x))).tableCells.includes(marker);
      await note("edit-table-cell", { edited, visible, entered, committed, undone }, from);
    }
    await page.keyboard.press(`${mod}+z`);
    const after = await settle((x) => x.blocks === count && x.tables === tables);
    await note("add-block-then-undo", { submitted: a.submitted, filled: a.filled, created: created.blocks, tables: created.tables, after: after.blocks, undone: a.submitted === "ok" && created.blocks === count + 1 && created.tables === tables + 1 && after.blocks === count && after.tables === tables }, a.from);
    count = after.blocks ?? count;
  }
  {
    const from = lines.length;
    const closedActions = await page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
    await page.waitForTimeout(1000);
    const host = page.locator('[data-surface-id="window:note-composite"]').first();
    const block = host.locator("[data-ink-block-id]").first();
    const edit = host.locator('[contenteditable="true"]');
    const textStart = lines.length;
    const openedText = await block.dblclick({ timeout: 8000 }).then(() => true).catch(() => false);
    const editorVisible = openedText && await edit.waitFor({ state: "visible", timeout: 8000 }).then(() => true).catch(() => false);
    let entered = null;
    if (editorVisible) {
      await edit.fill(textMarker);
      await edit.press(`${mod}+a`);
      await edit.evaluate((element) => {
        element.addEventListener("focusout", () => console.info("[DEBUG] Note text focusout", element.innerHTML), { once: true });
      }, undefined, { timeout: 4000 });
      await host.locator("[data-ink-editor-overlay]").getByRole("button", { name: "B", exact: true }).click();
      entered = await edit.evaluate((element) => ({
        html: element.innerHTML, active: element === document.activeElement,
        bold: [...element.querySelectorAll("b, strong, span")].some(run =>
          (run.matches("b, strong") || run.style.fontWeight === "bold") && run.textContent === element.textContent),
      }), undefined, { timeout: 4000 });
      assert.equal(entered.active && entered.bold, true, "The formatting toolbar did not retain focus and mark the selected text");
      await blurEditorOnCanvas(host);
    }
    const edited = await settle((x) => x.hasTextMarker && x.hasBoldMarker && x.blocks === count);
    const committed = edited.hasTextMarker && edited.hasBoldMarker && edited.blocks === count;
    await note("edit-block-text", { opened: openedText, editorVisible, entered, closed: await edit.count() === 0, committed }, textStart);
    if (process.env.SEMIO_PROBE_EDIT_ONLY === "1") {
      await browser.close();
      assert.equal(committed, true, "The Canvas text editor did not commit its change");
      assert.equal(report.steps.find(entry => entry.step === "edit-table-cell")?.detail.committed, true, "The Canvas table editor did not commit its change");
      console.log("[DEBUG] EDIT DONE");
      process.exit(0);
    }
    const opened = await togglePanel("Artifact");
    await page.waitForTimeout(2000);
    const before = await state();
    const rowsBefore = before.rows.length;
    const add = page.locator('[role="treeitem"]', { hasText: "Add Math" }).first();
    const clicked = (await add.count()) ? await add.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
    const added = (x) => clicked === "ok" && x.blocks === count + 1 && x.math === before.math + 1 && x.rows.length > rowsBefore;
    const after = await settle(added);
    await note("panel-add-math", { closedActions, opened, clicked, before: count, after: after.blocks, math: after.math, rowsBefore, rowsAfter: after.rows.length, added: added(after), rows: after.rows.map((r) => r.text) }, from);
    count = after.blocks ?? count;
  }
  const blockRow = (rows) => rows.find((r) => /^(Text|Table|Math|Image|Group|Ink) /.test(r.text));
  {
    const from = lines.length;
    const target = blockRow((await state()).rows);
    const row = target ? page.locator('[role="treeitem"]', { hasText: target.text }).first() : null;
    const clicked = row && (await row.count()) ? await row.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
    const after = await settle((x) => x.rows.some((r) => r.text === target?.text && r.selected === "true"));
    await note("select-block-row", { target: target?.text ?? null, clicked, selected: after.rows.filter((r) => r.selected === "true").map((r) => r.text), selectLines: lines.slice(from).filter((l) => /interactionSelect/.test(l)).slice(0, 3).map((l) => l.slice(0, 200)) }, from);
    const del = lines.length;
    await page.locator('[data-surface-id="window:note-composite"]').first().focus();
    await page.keyboard.press("Delete");
    const deleted = await settle((x) => x.blocks === count - 1);
    await note("delete-selection", { before: count, after: deleted.blocks, deleted: deleted.blocks === count - 1, rows: deleted.rows.map((r) => r.text) }, del);
    if (deleted.blocks === count - 1) {
      const und = lines.length;
      await page.keyboard.press(`${mod}+z`);
      const restored = await settle((x) => x.blocks === count);
      await note("undo-delete", { after: restored.blocks, restored: restored.blocks === count }, und);
      if (restored.blocks === count) {
        const redoStart = lines.length;
        await page.keyboard.press(`${mod}+Shift+z`);
        const redone = await settle((x) => x.blocks === count - 1);
        const redoDeleted = redone.blocks === count - 1;
        if (redoDeleted) await page.keyboard.press(`${mod}+z`);
        const prepared = redoDeleted ? await settle((x) => x.blocks === count) : redone;
        await note("redo-delete", { after: redone.blocks, deleted: redoDeleted, restored: prepared.blocks === count }, redoStart);
        count = prepared.blocks ?? count;
      } else count = restored.blocks ?? count;
    } else count = deleted.blocks ?? count;
  }
}
if (exercisePencil) {
  const from = lines.length;
  if (!pencilOnly) await togglePanel("Artifact");
  await page.waitForTimeout(800);
  const host = page.locator('[data-surface-id="window:note-composite"]').first();
  const clickId = async (id) => { const el = page.locator(`[id="${id}"]`).first(); return (await el.count()) ? el.click({ timeout: 8000, force: !tracePencil }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
  const utilityToggle = await clickId("framework.window.noteComposite.utilityBar.unfold");
  await page.waitForTimeout(1200);
  const drawGroup = await clickId("ui.utilities.note-composite.group.group:Draw");
  await page.waitForTimeout(1200);
  const armed = `${drawGroup}/${await clickId("pencil")}`;
  await page.waitForTimeout(1500);
  if (tracePencil) console.log("[DEBUG] pencil trace resources", await page.evaluate(() => performance.getEntriesByType("resource").map(({name}) => decodeURIComponent(name)).filter(url => /utility-and-tool-activation|host-effects|contributions-refresh/.test(url)).slice(0, 18)));
  const beforeStrokeCount = (await state()).navigatorStrokes;
  const hasStrokeCount = (value, count) => count !== null && value.strokes === count && value.navigatorStrokes === count;
  const box = await host.boundingBox();
  const strokeOrigin = box ? [box.x + box.width * 0.4, box.y + box.height * 0.6] : null;
  if (strokeOrigin) {
    const [x0, y0] = strokeOrigin;
    if (tracePencil) {
      const hit = await page.evaluate(({x, y}) => {
        const pencil = document.getElementById("pencil");
        return {
          pressed: pencil?.getAttribute("aria-pressed"),
          tool: pencil?.getAttribute("data-state"),
          button: pencil?.tagName,
          hits: document.elementsFromPoint(x, y).slice(0, 6).map(node => ({
            id: node.id, slot: node.getAttribute("data-slot"), tag: node.tagName,
            surface: node.closest("[data-surface-id]")?.getAttribute("data-surface-id"),
          })),
        };
      }, {x: x0, y: y0});
      console.log("[DEBUG] pencil trace origin", JSON.stringify(hit));
    }
    await page.mouse.move(x0, y0);
    await page.mouse.down();
    for (let i = 1; i <= 12; i++) { await page.mouse.move(x0 + i * 12, y0 + Math.sin(i / 2) * 30); await page.waitForTimeout(30); }
    await page.mouse.up();
  }
  const drawnCount = beforeStrokeCount === null ? null : beforeStrokeCount + 1;
  const splitCount = beforeStrokeCount === null ? null : beforeStrokeCount + 2;
  const after = await settle((x) => hasStrokeCount(x, drawnCount));
  const drawn = hasStrokeCount(after, drawnCount);
  await note("canvas-pencil-stroke", { utilityToggle, armed, utility: after.utility, before: beforeStrokeCount, after: after.strokes, navigator: after.navigatorStrokes, drawn, inkLines: lines.slice(from).filter((l) => /inkApplyEvents|ink-events/.test(l)).slice(0, 4).map((l) => l.slice(0, 240)) }, from);
  const und = lines.length;
  if (process.env.SEMIO_PROBE_ERASERS === "1" && drawn && strokeOrigin) {
    const [x, y] = strokeOrigin;
    const beforePoint = lines.length;
    const pointTool = await clickId("eraserPoint");
    await page.waitForTimeout(1600);
    await page.mouse.click(x + 6 * 12, y + Math.sin(6 / 2) * 30);
    const split = await settle((value) => hasStrokeCount(value, splitCount));
    await note("erase-stroke-point", { tool: pointTool, strokes: split.strokes, split: hasStrokeCount(split, splitCount) }, beforePoint);
    const beforeWhole = lines.length;
    const strokeTool = await clickId("eraserStroke");
    await page.waitForTimeout(1600);
    await page.mouse.click(x + 2 * 12, y + Math.sin(2 / 2) * 30);
    const erased = await settle((value) => hasStrokeCount(value, drawnCount));
    await note("erase-whole-stroke", { tool: strokeTool, strokes: erased.strokes, erased: hasStrokeCount(erased, drawnCount) }, beforeWhole);
    await host.focus();
    await page.keyboard.press(`${mod}+z`);
    const restoreSplit = await settle((value) => hasStrokeCount(value, splitCount));
    await page.keyboard.press(`${mod}+z`);
    const restoreWhole = await settle((value) => hasStrokeCount(value, drawnCount));
    await note("undo-erasers", { restored: hasStrokeCount(restoreSplit, splitCount) && hasStrokeCount(restoreWhole, drawnCount) }, beforePoint);
  }
  await host.focus();
  await page.keyboard.press(`${mod}+z`);
  const undone = await settle((x) => hasStrokeCount(x, beforeStrokeCount));
  await note("undo-pencil-stroke", { after: undone.strokes, navigator: undone.navigatorStrokes, undone: drawn && hasStrokeCount(undone, beforeStrokeCount) }, und);
  if (pencilOnly) {
    await browser.close();
    assert.equal(drawn, true, "The Pencil stroke did not publish");
    assert.equal(hasStrokeCount(undone, beforeStrokeCount), true, "Undo did not restore the stroke count in both views");
    assert.deepEqual(lines.filter(line => /input #\d+ .+ refused: dispatch-failed/.test(line)), []);
    if (process.env.SEMIO_PROBE_ERASERS === "1") {
      const detail = (name) => report.steps.find((entry) => entry.step === name)?.detail;
      assert.equal(detail("erase-stroke-point")?.split, true, "The point eraser did not split the stroke");
      assert.equal(detail("erase-whole-stroke")?.erased, true, "The stroke eraser did not remove the chosen fragment");
      assert.equal(detail("undo-erasers")?.restored, true, "Undo did not restore the split and then the original stroke");
    }
    console.log("[DEBUG] PENCIL DONE");
    process.exit(0);
  }
  count = undone.blocks ?? count;
  const activeUtility = page.locator(process.env.SEMIO_PROBE_ERASERS === "1" ? '[id="eraserStroke"][aria-pressed="true"]' : '[id="pencil"][aria-pressed="true"]').first();
  if (await activeUtility.count()) await activeUtility.click();
  const foldUtilities = page.locator('[id="framework.window.noteComposite.utilityBar.fold"]').first();
  if (await foldUtilities.isVisible()) await foldUtilities.click();
}
if (!transferOnly) {
  const from = lines.length;
  const captured = await state();
  const added = await submitAction("addBlock", { kind: "table", x: 600, y: 350 });
  const prepared = await settle(value => value.tables === captured.tables + 1);
  await note("prepare-archive-table-insert", { submitted: added.submitted, filled: added.filled, before: captured.tables, after: prepared.tables }, from);
  assert.equal(prepared.tables, captured.tables + 1, "The archive fixture's Table block did not publish");
  if (prepared.actionRows.length > 0) await page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first().click();
  const host = page.locator('[data-surface-id="window:note-composite"]').first();
  await host.locator("table tbody").last().locator("td").first().dblclick();
  const editor = host.locator("#ink-table-cell-editor");
  await editor.fill(archiveCellMarker);
  await blurEditorOnCanvas(host);
  const savedTable = await settle(value => value.tableCells.includes(archiveCellMarker));
  count = savedTable.blocks ?? count;
  await note("prepare-archive-table", { submitted: added.submitted, tableCells: savedTable.tableCells, retained: savedTable.tableCells.includes(archiveCellMarker) }, from);
}
if (extended) {
  const host = page.locator('[data-surface-id="window:note-composite"]').first();
  const wheel = exerciseCamera ? await wheelCanvas() : null;
  const initial = await state();
  const pasteStart = lines.length;
  if (cameraOnly && process.env.SEMIO_PROBE_CAMERA_ADD === "1") {
    const action = await submitAction("addBlock", { kind: "text", x: 300, y: 180 });
    const after = await settle(value => value.blocks === initial.blocks + 1);
    const retained = after.blocks === initial.blocks + 1 && wheel !== null && after.zoom === wheel.zoom;
    await note("camera-after-add", { submitted: action.submitted, blocks: after.blocks, zoom: after.zoom, retained }, pasteStart);
    await browser.close();
    assert.equal(retained, true, "The next block edit did not retain the published camera");
    process.exit(0);
  }
  await host.focus();
  const handled = await host.evaluate((root) => {
    const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="12" height="8"><rect width="12" height="8" fill="#18966d"/></svg>';
    const clipboardData = new DataTransfer();
    clipboardData.items.add(new File([svg], "Note acceptance.svg", { type: "image/svg+xml" }));
    return !root.dispatchEvent(new ClipboardEvent("paste", { clipboardData, bubbles: true, cancelable: true }));
  });
  const beforePaste = count;
  const hasPastedImage = (x) => handled && x.blocks === beforePaste + 1 && x.images === initial.images + 1 && x.imageSizes.some(([w, h]) => w === 12 && h === 8);
  const pasted = await settle(hasPastedImage);
  const imagePublished = hasPastedImage(pasted);
  await note("paste-image", { handled, before: beforePaste, after: pasted.blocks, images: pasted.imageSizes, pasted: imagePublished }, pasteStart);
  count = pasted.blocks ?? count;
  if (wheel) {
    const persisted = imagePublished && pasted.zoom === wheel.zoom && wheel.offset !== null && pasted.cameraOffset !== null
      && Math.hypot(...pasted.cameraOffset.map((coordinate, axis) => coordinate - wheel.offset[axis])) < 1;
    await note("camera-after-edit", { zoom: pasted.zoom, offset: pasted.cameraOffset, persisted }, pasteStart);
    if (cameraOnly) {
      await browser.close();
      assert.equal(persisted, true, "The following document edit did not preserve the published camera");
      console.log("[DEBUG] CAMERA DONE");
      process.exit(0);
    }
  }
}
{
  const from = lines.length;
  const actionToggle = page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first();
  const saved = await state();
  if (saved.actionRows.length === 0) await actionToggle.click();
  const download = page.waitForEvent("download", { timeout: downloadTimeout }).catch(() => null);
  await page.locator('[id="action.exportArtifactDocument"]').first().click();
  const archive = await download;
  const path = join(outDir, "note-acceptance.semio-archive");
  if (archive) await archive.saveAs(path);
  await note("export-document", { filename: archive?.suggestedFilename() ?? null, blocks: saved.blocks, images: saved.images, hasTextMarker: saved.hasTextMarker, hasBoldMarker: saved.hasBoldMarker, tableCells: saved.tableCells }, from);

  await page.reload({ waitUntil: "domcontentloaded" });
  let after = null;
  for (let i = 0; i < 90; i++) {
    await page.waitForTimeout(1000);
    after = await state();
    if (after.error || (after.ready && after.blocks !== null)) break;
  }
  await note("reload", { ready: after.ready, error: after.error, blocks: after.blocks, example: after.example }, from);
  if (archive && after.ready && !after.error && after.blocks !== null) {
    const importStart = lines.length;
    await importDocumentArchive(path);
    allowEmptyDemoReplacement(after);
    const restored = (x) => (transferOnly || x.hasTextMarker && x.hasBoldMarker && !after.hasTextMarker) && x.blocks === saved.blocks && x.blocks !== after.blocks
      && JSON.stringify(x.imageSizes) === JSON.stringify(saved.imageSizes)
      && JSON.stringify(x.tableCells) === JSON.stringify(saved.tableCells)
      && (transferOnly || x.tableCells.includes(archiveCellMarker));
    const reopened = await settle(restored);
    await note("import-document", { blocks: reopened.blocks, images: reopened.images, hasTextMarker: reopened.hasTextMarker, hasBoldMarker: reopened.hasBoldMarker, tableCells: reopened.tableCells, restored: restored(reopened) }, importStart);
  }
}
{
  const final = await state();
  await note("final", final, 0);
}
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
const step = (name) => report.steps.find((entry) => entry.step === name)?.detail;
assert.ok(step("boot")?.ready, "Note did not become ready");
if (transferOnly) {
  assert.equal(step("paste-image")?.pasted, true, "The pasted image did not render in the Note canvas");
  assert.match(step("export-document")?.filename ?? "", /\.semio-archive$/, "Note did not export a document archive");
  assert.equal(step("import-document")?.restored, true, "The pasted image did not reopen");
  console.log("[DEBUG] TRANSFER DONE");
  process.exit(0);
}
assert.equal(step("add-block-text")?.added, true, "Text block was not added");
assert.equal(step("add-block-then-undo")?.undone, true, "Undo did not restore the previous block count");
assert.equal(step("edit-table-cell")?.committed && step("edit-table-cell")?.undone, true, "The Canvas table editor did not commit and undo its cell value");
assert.equal(step("edit-block-text")?.committed, true, "The Canvas text editor did not commit its change");
assert.equal(step("panel-add-math")?.added, true, "Artifact panel did not add a math block");
assert.ok(step("select-block-row")?.selected?.length, "Block selection was not reflected in the artifact tree");
assert.equal(step("delete-selection")?.deleted, true, "Selected block was not deleted");
assert.equal(step("undo-delete")?.restored, true, "Undo did not restore the deleted block");
assert.equal(step("redo-delete")?.deleted && step("redo-delete")?.restored, true, "Redo did not reapply Delete and the second Undo did not restore it");
if (exercisePencil) {
  assert.equal(step("canvas-pencil-stroke")?.drawn, true, "Canvas pencil stroke did not add ink");
  assert.equal(step("undo-pencil-stroke")?.undone, true, "Undo did not remove the pencil stroke");
}
assert.ok(step("reload")?.ready && !step("reload")?.error, "Note did not reopen after reload");
assert.equal(step("export-document")?.hasBoldMarker, true, "The formatted text was missing from the document before export");
assert.equal(step("prepare-archive-table")?.retained, true, "The authored table cell was missing before export");
assert.match(step("export-document")?.filename ?? "", /\.semio-archive$/, "Note did not export a document archive");
assert.equal(step("import-document")?.restored, true, "Reopening the archive did not restore the document");
assert.deepEqual(lines.filter(line => /input #\d+ .+ refused: dispatch-failed| pageerror |shell fault|action failed|panicked at|wasm trap/i.test(line)), [], "Note reported a live runtime fault");
if (exerciseCamera) {
  assert.equal(step("zoom-canvas")?.zoomed, true, "The wheel did not zoom the Note canvas");
  assert.equal(step("camera-after-edit")?.persisted, true, "The following document edit did not preserve the published camera");
}
if (extended) assert.equal(step("paste-image")?.pasted, true, "The pasted image did not render in the Note canvas");
console.log("[DEBUG] INTERACT DONE", report.steps.length);
