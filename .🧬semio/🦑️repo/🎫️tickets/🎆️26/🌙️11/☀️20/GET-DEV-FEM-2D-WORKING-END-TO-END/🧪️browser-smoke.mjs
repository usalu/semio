/** 🧪️ Drives the actual FEM 2D development component with Playwright. */
import assert from "node:assert/strict";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/?plugin=fem2d";
const resultSurface = "window:fem2d-results";
const probePaint = process.argv.includes("--probe-paint");
const probeEdit = process.argv.includes("--probe-edit");
const probeResults = process.argv.includes("--probe-results");
const probeInitialDocument = process.argv.includes("--probe-initial-document");
const probeWire = process.argv.includes("--probe-wire");
const probeDeferAnalysis = process.argv.includes("--probe-defer-analysis");
const probeDeferModel = probeDeferAnalysis || process.argv.includes("--probe-defer-canvases");
const probeDeferResults = probeDeferModel || process.argv.includes("--probe-defer-results");
assert(!(probeEdit && probeResults), "Choose one diagnostic path per browser session");
assert(!(probeInitialDocument && probeDeferResults), "Choose one startup isolation at a time");
assert(!(probeInitialDocument || probeDeferResults) || (probePaint && new URL(url).origin === "http://127.0.0.1:6095"), "Startup isolation uses only the instrumented preview");
const errors = [];
const diagnostics = [];
const failureReads = [];
const nativeProbeTurns = [];

function describeNativeFailure(value) {
  const seen = new WeakSet();
  function visit(item, depth) {
    if (typeof item === "bigint") return String(item);
    if (typeof item === "string") return item.slice(0, 3000);
    if (item === null || typeof item !== "object") return item;
    if (depth === 7 || seen.has(item)) return "<retained>";
    if (ArrayBuffer.isView(item)) return `<binary ${item.byteLength} bytes>`;
    seen.add(item);
    if (Array.isArray(item)) return item.slice(0, 8).map(child => visit(child, depth + 1));
    const fields = new Set(["name", "message", "stack", "cause", "payload", ...Object.keys(item).slice(0, 20)]);
    return Object.fromEntries([...fields].filter(field => field in item).map(field => [field, visit(item[field], depth + 1)]));
  }
  return visit(value, 0);
}

const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  page.setDefaultTimeout(30000);
  if (probeWire || process.argv.includes("--capture-native-refusal")) {
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🌉️bridge.js"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = "const tag = (failure?.payload ?? failure)?.failure?.tag;";
      if (!source.includes(seam)) {
        await route.fulfill({ response });
        return;
      }
      const recording = `if (tag === "semantic" && !globalThis.fem2dNativeRefusalRecorded) {
        globalThis.fem2dNativeRefusalRecorded = true;
        console.error("[DEBUG] First native semantic refusal", (${describeNativeFailure})(failure));
      }`;
      const poll = "result = await reactor.poll(slice);";
      const wireRecorder = `
        const cursor = result.commandIngress?.cursor;
        const ingress = result.commandIngress?.kind;
        const eventKinds = events.map(event => event.kind);
        const surfaceEvent = eventKinds.some(kind => /surface/i.test(kind));
        if (surfaceEvent || (cursor?.kind === 1 && (ingress === 3 || ingress === 4))) {
          globalThis.fem2dWireSample ??= new Map();
          const key = surfaceEvent ? eventKinds.join(",") : String(cursor.seq);
          const seen = (globalThis.fem2dWireSample.get(key) ?? 0) + 1;
          globalThis.fem2dWireSample.set(key, seen);
          if (seen <= 4 || seen % 64 === 0 || ingress === 4) {
            const receipt = result.retainedReceipt;
            console.error("[DEBUG] FEM2D WIT receipt " + JSON.stringify({
              cursor, ingress, eventKinds, seen,
              epoch: receipt.input.epoch, grant: receipt.input.grant,
              fuel: receipt.input.work.fuel, spent: receipt.spent,
              work: receipt.workUnitsSpent, effects: result.effects.length,
              uiPatches: result.uiPatches.length,
            }, (_, value) => typeof value === "bigint" ? String(value) : value));
          }
        }
      `;
      const observed = source.replace(seam, `${seam}\n  ${recording}`);
      await route.fulfill({ response, body: probeWire ? observed.replace(poll, `${poll}\n${wireRecorder}`) : observed });
    });
  }
  if (probeInitialDocument || probeDeferResults) {
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🐚️Shell/🎬️initial-example/🟦️.ts"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = "const work = start();";
      assert(source.includes(seam), "The real initial-example hook did not contain its expected invocation");
      const deferredRelease = `
        ${seam}
        if (work) void work.then(
          value => {
            globalThis.fem2dExampleTerminal = { outcome: value?.kind ?? "resolved" };
            globalThis.dispatchEvent(new Event("fem2d-startup-example-terminal"));
          },
          error => {
            globalThis.fem2dExampleTerminal = { outcome: "rejected", message: String(error) };
            globalThis.dispatchEvent(new Event("fem2d-startup-example-terminal"));
          },
        );
      `;
      await route.fulfill({ response, body: source.replace(seam, probeInitialDocument ? "const work = Promise.resolve();" : deferredRelease) });
    });
  }
  if (probeDeferResults) {
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🔌️PluginRuntime/🧩️module-loader/🟦️.ts"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = "const events = uiRefreshSurfaceEvents(instanceId, request,";
      assert(source.includes(seam), "The real refresh hook did not contain its expected surface projection");
      const isolate = `
        if (!globalThis.fem2dExampleTerminal && request.windows) {
          const windows = request.windows.filter(target =>
            !target.key.endsWith("fem2d-results") && target.bodyKey !== "fem2d.play.results"
            && (!${probeDeferModel} || (!target.key.endsWith("fem2d-model") && target.bodyKey !== "fem2d.play.model")),
          );
          globalThis.fem2dDeferredWindows ??= 0;
          globalThis.fem2dDeferredWindows += request.windows.length - windows.length;
          request = { ...request, windows };
        }
      `;
      await route.fulfill({ response, body: source.replace(seam, `${isolate}\n${seam}`) });
    });
  }
  if (probePaint) {
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/📐️Canvas2dHost/🔶️scene/🟦️.tsx"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = "layersJsonRef.current = scene?.layersJson;";
      const recording = `
        globalThis.fem2dCanvasScenes ??= {};
        globalThis.fem2dCanvasScenes[node.surfaceId] = scene?.layersJson ?? null;
      `;
      await route.fulfill({ response, body: source.replace(seam, `${seam}\n  ${recording}`) });
    });
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🔌️PluginRuntime/💼️job-ledger/🟦️.ts"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const recording = `
        globalThis.fem2dJobHistory ??= [];
        subscribeSpawnedJobsV1(() => {
          globalThis.fem2dJobHistory.push(spawnedJobsSnapshotV1().map(({ kind, job, steps, reportsProgress, progressAtMs }) =>
            ({ kind, job: String(job), steps, reportsProgress, progressAtMs })));
          if (globalThis.fem2dJobHistory.length > 12) globalThis.fem2dJobHistory.shift();
        });
      `;
      await route.fulfill({ response, body: `${source}\n${recording}` });
    });
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🔌️PluginRuntime/📥️invocation-frames/🟦️.ts"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = "const actionId = typeof id === \"string\" ? id : null;";
      const recording = `
        if (actionId === "noteShellCommand") {
          globalThis.fem2dShellNotes ??= [];
          const note = { seq: String(seq), invocation: (${describeNativeFailure})(invocation) };
          if (globalThis.fem2dShellNotes.at(-1)?.seq !== note.seq) globalThis.fem2dShellNotes.push(note);
          if (globalThis.fem2dShellNotes.length > 16) globalThis.fem2dShellNotes.shift();
        }
      `;
      await route.fulfill({ response, body: source.replace(seam, `${seam}\n${recording}`) });
    });
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🔌️PluginRuntime/🧵️turn-dispatch/🟦️.ts"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = /function submitPluginTurn\([^\n]+\{/;
      const recording = `
        globalThis.fem2dJobTransport ??= [];
        const magic = new TextEncoder().encode("semio.typed-operation-ack.v1\\0");
        for (const event of events) {
          if (globalThis.fem2dJobTransport.length >= 64) break;
          if (event.kind === "job-completed") {
            globalThis.fem2dJobTransport.push({ kind: event.kind, job: String(event.payload.job), verdict: event.payload.outcome.tag });
          } else if (event.kind === "message" && event.payload?.payload?.length === magic.length + 25 && magic.every((byte, index) => byte === event.payload.payload[index])) {
            const token = new DataView(Uint8Array.from(event.payload.payload).buffer, magic.length);
            globalThis.fem2dJobTransport.push({
              kind: "typed-operation-ack", operation: String(token.getBigUint64(4, true)),
              generation: String(token.getBigUint64(12, true)), sequence: token.getUint32(20, true), attempt: token.getUint8(24),
            });
          }
        }
      `;
      await route.fulfill({ response, body: source.replace(seam, captured => `${captured}\n${recording}`) });
    });
    await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const seam = /const routeHostEffects = [^\n]+effects\.filter\([^\n]+\{/;
      const recording = `
        globalThis.fem2dHostEffects ??= [];
        if (globalThis.fem2dHostEffects.length < 80) globalThis.fem2dHostEffects.push({
          instanceId,
          tag: effect.tag,
          detail: effect.val?.message ?? effect.val?.kind ?? effect.val?.target?.tag ?? null,
        });
      `;
      const continuation = "return { status: settled.status, nextWake: settled.nextWake };";
      const continuationRecording = `
        globalThis.fem2dHostContinuations ??= [];
        globalThis.fem2dHostContinuations.push({ status: wireTurnStatusTag(settled.status), nextWake: settled.nextWake });
        if (globalThis.fem2dHostContinuations.length > 24) globalThis.fem2dHostContinuations.shift();
      `;
      const ingress = "const inspected = inspectEncodedAppCommand(events);";
      const settled = "const outFrames = [];";
      const ingressTurn = "const tag = turn.commandIngress?.tag;";
      const ack = "acknowledgements = [...accepted.acknowledgements, ...typedOperationAcknowledgements(turn), ...accepted.turns.flatMap(typedOperationAcknowledgements)];";
      const ackRecorder = `
        if (inspected.actionId === "noteShellCommand" && (acknowledgements.length || turn.effects.length || turn.uiPatches.length)) {
          globalThis.fem2dNoteOutputs.push({
            seq: String(inspected.seq), ingress: turn.commandIngress?.tag,
            effects: turn.effects.map(effect => ({
              tag: effect.tag, job: effect.val?.job == null ? null : String(effect.val.job), kind: effect.val?.kind,
            })), uiPatches: turn.uiPatches.length,
            additionalTurns: accepted.turns.length, acknowledgements: (${describeNativeFailure})(acknowledgements),
          });
          if (globalThis.fem2dNoteOutputs.length > 32) globalThis.fem2dNoteOutputs.shift();
        }
      `;
      const ingressRecording = `
        const trace = JSON.stringify({
          action: inspected.actionId, seq: String(inspected.seq), ingress: tag,
          cursor: turn.commandIngress?.val?.cursor ?? turn.commandIngress?.val,
          actor: wireTurnStatusTag(turn.status),
          effects: turn.effects?.length, uiPatches: turn.uiPatches?.length,
          uiReceipt: turn.uiPatchReceipt !== undefined, lifecycleReceipt: turn.lifecycleReceipt !== undefined,
          deferredEvents: turn.deferredEvents,
          acknowledgements: acknowledgements.map(value => value.kind ?? value.tag),
        }, (_, value) => typeof value === "bigint" ? String(value) : value);
        const ingressLog = globalThis.fem2dIngressTurns;
        if (ingressLog.at(-1) !== trace) ingressLog.push(trace);
        if (ingressLog.length > 32) ingressLog.shift();
      `;
      const commandRecording = phase => `
        if (inspected.actionId) {
          globalThis.fem2dHostCommands.push({ action: inspected.actionId, seq: String(inspected.seq), phase: "${phase}" });
          if (globalThis.fem2dHostCommands.length > 32) globalThis.fem2dHostCommands.shift();
        }
      `;
      const jobEntry = "const driveSpawnedJob = async (instanceId, actorId, job, kind, input) => {";
      const jobStart = "await serializeCommandIngressForActor(actorId, () => shardClient.startJob(actorId, job, kind, input));";
      const jobPoll = "last = await shardClient.stepJob(actorId, job, jobStepBudget);";
      const jobCatch = "} catch (error) {\n      turnOutcomes.push({ instanceId, error });\n    } finally {\n      ledgerEntry.end();";
      const jobRecorder = `
        const recordJob = (phase, detail) => {
          if (globalThis.fem2dJobTurns.length < 24) globalThis.fem2dJobTurns.push({ kind, job: String(job), phase, detail });
        };
        if (${probeDeferAnalysis} && kind === "semio.fem2d.mounted-analysis" && !globalThis.fem2dExampleTerminal) {
          recordJob("deferred-until-example-terminal", { instanceId });
          await new Promise(resolve => globalThis.addEventListener("fem2d-startup-example-terminal", resolve, { once: true }));
          if (globalThis.fem2dExampleTerminal.outcome !== "applied") return;
        }
        recordJob("start-requested", { instanceId });
      `;
      const reservedEntry = "const commitReservedToolJobWhileSerialized = async (instanceId, actorId, job, kind, input) => {";
      const reservedStart = "await shardClient.startJob(actorId, job, kind, input);";
      const reservedStep = "const outcome = await shardClient.stepJob(actorId, job, jobStepBudget);";
      const reservedReturn = "if (live()) await deliverJobCompletionTurn(instanceId, actorId, job, outcome);";
      const reservedRecorder = `
        let reservedSteps = 0;
        const recordReserved = (phase, detail) => {
          if (globalThis.fem2dReservedJobs.length < 64) globalThis.fem2dReservedJobs.push({ job: String(job), kind, phase, steps: reservedSteps, detail });
        };
        recordReserved("start-requested", { instanceId });
      `;
      const completionResult = "const frames = [];\n    const leftover = [];";
      const completionRecorder = `
        globalThis.fem2dJobReturns.push({
          job: String(job), status: wireTurnStatusTag(settled.status),
          effects: settled.effects.map(effect => effect.tag), uiPatches: settled.uiPatches.length,
          uiReceipt: settled.uiPatchReceipt !== undefined, lifecycleReceipt: settled.lifecycleReceipt !== undefined,
        });
      `;
      const observed = source.replace(seam, matched => `${matched}\n${recording}`)
        .replace(continuation, `${continuationRecording}\n${continuation}`)
        .replace(ingress, `${ingress}\n${commandRecording("ingress")}`)
        .replace(ingressTurn, `${ingressTurn}\n${ingressRecording}`)
        .replace(ack, `${ack}\n${ackRecorder}`)
        .replace(settled, `${commandRecording("settled")}\n${settled}`)
        .replace(jobEntry, `${jobEntry}\n${jobRecorder}`)
        .replace(jobStart, `${jobStart}\nrecordJob("started", null);`)
        .replace(jobPoll, `${jobPoll}\nif (step === 0 || last.status !== "running") recordJob("step", { status: last.status, detail: last.status === "failed" ? new TextDecoder().decode(last.value.slice(0, 256)) : null });`)
        .replace(jobCatch, jobCatch.replace("} catch (error) {", `} catch (error) {\nrecordJob("exception", (${describeNativeFailure})(error));`))
        .replace(reservedEntry, `${reservedEntry}\n${reservedRecorder}`)
        .replace(reservedStart, `${reservedStart}\nrecordReserved("started", null);`)
        .replace(reservedStep, `${reservedStep}\nreservedSteps++; if (reservedSteps === 1 || outcome.status !== "running") recordReserved("step", { status: outcome.status, detail: outcome.status === "failed" ? new TextDecoder().decode(outcome.value.slice(0, 256)) : null });`)
        .replace(reservedReturn, `${reservedReturn}\nrecordReserved("completion-returned", { live: live() });`)
        .replace(completionResult, `${completionRecorder}\n${completionResult}`);
      const globals = ["HostEffects", "HostContinuations", "HostCommands", "IngressTurns", "JobTurns", "NoteOutputs", "ReservedJobs", "JobReturns"];
      await route.fulfill({ response, body: globals.map(name => `globalThis.fem2d${name} ??= [];`).join("\n") + "\n" + observed });
    });
  }
  page.on("pageerror", error => errors.push(JSON.stringify(describeNativeFailure(error))));
  page.on("console", message => {
    const text = message.text().slice(0, 1600);
    if (text.startsWith("[DEBUG] FEM2D WIT receipt ")) {
      nativeProbeTurns.push(text.slice("[DEBUG] FEM2D WIT receipt ".length));
      if (nativeProbeTurns.length > 56) nativeProbeTurns.shift();
    }
    if (/semantic|refused|command-failed|fault|panicked|Command exchange|Manifest command|Store replacement/i.test(text)) {
      diagnostics.push(text);
      if (failureReads.length < 8) {
        failureReads.push(Promise.allSettled(message.args().map(arg => arg.evaluate(describeNativeFailure))).then(results => results.flatMap(result => result.status === "fulfilled" ? [result.value] : [])));
      }
    }
    if (message.type() === "error" && !text.startsWith("[DEBUG]")) errors.push(text);
  });
  page.on("response", response => {
    if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`);
  });
  await page.addInitScript(resultSurface => {
    const resultFrames = new WeakMap();
    const context = CanvasRenderingContext2D.prototype;
    const names = ["clearRect", "fillText", "beginPath", "moveTo", "lineTo", "stroke", "drawImage"];
    const originals = Object.fromEntries(names.map(name => [name, context[name]]));
    function frame(ctx) {
      if (ctx.canvas.closest(`[data-surface-id="${resultSurface}"]`) === null) return;
      if (!resultFrames.has(ctx.canvas)) resultFrames.set(ctx.canvas, { labels: [], deformation: [], path: [], calls: {} });
      return resultFrames.get(ctx.canvas);
    }
    for (const name of names) {
      context[name] = function (...args) {
        const paint = frame(this);
        if (paint) {
          paint.calls[name] = (paint.calls[name] ?? 0) + 1;
          if (name === "clearRect") {
            paint.labels.length = 0;
            paint.deformation.length = 0;
          }
          if (name === "fillText") paint.labels.push(args[0]);
          if (name === "beginPath") paint.path.length = 0;
          if (name === "moveTo" || name === "lineTo") paint.path.push(args.slice(0, 2));
          if (name === "stroke" && this.strokeStyle === "#f472b6" && paint.path.length > 1) {
            paint.deformation.push(paint.path.slice());
          }
        }
        return originals[name].apply(this, args);
      };
    }
    window.readFem2dResults = () => {
      const canvas = document.querySelector(`[data-surface-id="${resultSurface}"] canvas`);
      return canvas ? resultFrames.get(canvas) : null;
    };
  }, resultSurface);
  const response = await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  assert(response?.ok(), `Development entry returned ${response?.status()}`);
  try {
    await page.waitForFunction(() => Boolean(document.querySelector("[data-semio-os-ready], [data-semio-os-error]")), null, { timeout: 120000 });
    assert.equal(await page.locator("html").getAttribute("data-semio-os-error"), null);
    await page.locator('[data-surface-id="window:fem2d-model"] canvas').waitFor({ state: "visible" });
    const results = page.locator(`[data-surface-id="${resultSurface}"] canvas`);
    await results.waitFor({ state: "visible" });
    const solved = async previous => {
      const frame = await page.waitForFunction(previous => {
        const frame = window.readFem2dResults();
        if (!frame?.labels.some(text => /^(Converged|Konvergiert) ·/.test(text)) || !frame.deformation.length) return false;
        const deformation = JSON.stringify(frame.deformation);
        return deformation !== previous ? { deformation, label: frame.labels.find(text => /^(Converged|Konvergiert) ·/.test(text)) } : false;
      }, previous, { timeout: probePaint ? 30000 : 120000 });
      return frame.jsonValue();
    };
    const original = probeEdit ? null : await solved(null);
    if (original) console.log("[DEBUG] Initial FEM 2D", original.label);
    if (original && !probeInitialDocument && !probeDeferResults) {
      const example = page.getByRole("combobox", { name: "Example", exact: true });
      await example.getByText("Demo", { exact: true }).waitFor({ state: "visible" });
    }
    if (!probeResults) {
      const artifact = page.getByRole("button", { name: "Artifact", exact: true }).first();
      await artifact.click();
      const node = page.getByText(/^n1 · \(0[.,]00,/).first();
      await node.click();
      await page.getByRole("button", { name: "Inspection", exact: true }).first().click();
      const x = page.getByRole("spinbutton", { name: "X", exact: true }).or(page.getByLabel("X", { exact: true })).first();
      assert.equal(Number(await x.inputValue()), 0);
      await x.fill("1.5");
      await x.press("Enter");
      await artifact.click();
      await page.getByText(/^n1 · \(1[.,]50,/).first().waitFor({ state: "visible" });
    }
    if (probeEdit) {
      console.log("[DEBUG] FEM 2D Artifact n1 X updated to 1.5", JSON.stringify(await page.evaluate(() => ({
        jobHistory: globalThis.fem2dJobHistory,
        hostEffects: globalThis.fem2dHostEffects,
        hostContinuations: globalThis.fem2dHostContinuations,
        hostCommands: globalThis.fem2dHostCommands,
        ingressTurns: globalThis.fem2dIngressTurns?.map(JSON.parse),
        jobTurns: globalThis.fem2dJobTurns,
        shellNotes: globalThis.fem2dShellNotes,
        noteOutputs: globalThis.fem2dNoteOutputs,
        reservedJobs: globalThis.fem2dReservedJobs,
        jobReturns: globalThis.fem2dJobReturns,
        jobTransport: globalThis.fem2dJobTransport,
      }))));
    } else {
      if (!probeResults) {
        const updated = await solved(original.deformation);
        console.log("[DEBUG] Updated FEM 2D", updated.label);
      }
      await results.click({ position: { x: 16, y: 16 } });
      await page.getByRole("button", { name: "Results", exact: true }).first().click();
      await page.getByRole("combobox", { name: "Source", exact: true }).click();
      await page.getByRole("option", { name: "Live Load", exact: true }).click();
      await page.waitForFunction(() => {
        const labels = window.readFem2dResults()?.labels ?? [];
        return labels.some(label => label.startsWith("Converged ·")) && labels.includes("Ty: 12000 N");
      }, null, { timeout: 120000 });
      console.log("[DEBUG] FEM 2D Live Load reaction", "Ty: 12000 N");
      if (process.argv.includes("--eigen-modes")) {
        const mode = page.getByRole("combobox", { name: "Mode", exact: true });
        const selectMode = async (label, caption) => {
          await mode.click();
          await page.getByRole("option", { name: label, exact: true }).click();
          const handle = await page.waitForFunction(caption => {
            const frame = window.readFem2dResults();
            if (!frame?.deformation.length || !frame.labels.some(label => label.startsWith("Converged ·"))) return false;
            const pattern = new RegExp(caption);
            return frame.labels.find(text => Number(text.match(pattern)?.[1]) > 0) ?? false;
          }, caption.source, { timeout: 120000 });
          console.log("[DEBUG] FEM 2D", await handle.jsonValue());
        };
        await selectMode("Modal", /^Mode 1: (\d+(?:\.\d+)?) Hz$/);
        await selectMode("Buckling", /^Buckling mode 1: (\d+(?:\.\d+)?)$/);
      }
    }
    assert.deepEqual(errors, []);
  } catch (error) {
    console.error("[DEBUG] Browser probe", JSON.stringify(await page.evaluate(() => ({
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      surfaces: Array.from(document.querySelectorAll("[data-surface-id]")).map(node => node.getAttribute("data-surface-id")),
      results: window.readFem2dResults(),
      scenes: Object.fromEntries(Object.entries(globalThis.fem2dCanvasScenes ?? {}).map(([id, source]) => {
        const layers = JSON.parse(source ?? "[]");
        return [id, { count: layers.length, ids: layers.slice(0, 70).map(layer => [layer.id, layer.kind]), text: layers.filter(layer => layer.text !== undefined).map(layer => layer.text).slice(0, 12) }];
      })),
      jobHistory: globalThis.fem2dJobHistory,
      hostEffects: globalThis.fem2dHostEffects,
      hostContinuations: globalThis.fem2dHostContinuations,
      hostCommands: globalThis.fem2dHostCommands,
      ingressTurns: globalThis.fem2dIngressTurns?.map(JSON.parse),
      jobTurns: globalThis.fem2dJobTurns,
      shellNotes: globalThis.fem2dShellNotes,
      noteOutputs: globalThis.fem2dNoteOutputs,
      reservedJobs: globalThis.fem2dReservedJobs,
      jobReturns: globalThis.fem2dJobReturns,
      jobTransport: globalThis.fem2dJobTransport,
      deferredWindows: globalThis.fem2dDeferredWindows,
      exampleTerminal: globalThis.fem2dExampleTerminal,
      canvases: Array.from(document.querySelectorAll("[data-surface-id] canvas")).map(canvas => {
        const ctx = canvas.getContext("2d");
        const pixels = canvas.width && canvas.height ? ctx?.getImageData(0, 0, canvas.width, canvas.height).data : undefined;
        let opaque = 0;
        for (let index = 3; pixels && index < pixels.length; index += 4) {
          if (pixels[index] > 0) opaque++;
        }
        return { surface: canvas.closest("[data-surface-id]").getAttribute("data-surface-id"), width: canvas.width, height: canvas.height, opaque };
      }),
      inputs: Array.from(document.querySelectorAll("input")).slice(0, 20).map(node => ({ id: node.id, label: node.getAttribute("aria-label") })),
      text: document.body?.innerText?.slice(0, 1000),
    })).catch(() => null)));
    const nativeFailures = (await Promise.all(failureReads)).flat().filter(value => value && typeof value === "object");
    if (nativeFailures.length) console.error("[DEBUG] Native failure details", JSON.stringify(nativeFailures));
    if (nativeProbeTurns.length) console.error("[DEBUG] FEM2D sampled WIT receipts", JSON.stringify(nativeProbeTurns.map(JSON.parse)));
    throw new Error(`${error}\n${[...new Set([...errors.slice(-12), ...diagnostics.slice(-12)])].join("\n")}`, { cause: error });
  }
} finally {
  await browser.close();
}
