/** 🩺 Browser startup evidence without waiting for the canvas to mount. */
import { chromium } from "playwright";
import { usePickerExperiment } from "./note-picker-experiment-host.mjs";

const browser = await chromium.launch({headless: true});
try {
  const context = await browser.newContext({viewport: {width: 1600, height: 1000}});
  const page = await context.newPage();
  await usePickerExperiment(context, process.env.SEMIO_PROBE_PICKER_EXPERIMENT, "http://127.0.0.1:6080");
  let finishTrace;
  let armCameraBreakpoints;
  if (process.env.SEMIO_NOTE_CPU_PROFILE === "1") {
    const trace = await browser.newBrowserCDPSession();
    const models = new Map();
    trace.on("Tracing.dataCollected", ({value}) => {
      for (const event of value) {
        const profile = event.args?.data?.cpuProfile;
        if (!profile) continue;
        const name = [event.pid, event.tid, event.id].join("/");
        let model = models.get(name);
        if (!model) models.set(name, model = {nodes: new Map(), counts: new Map()});
        for (const node of profile.nodes ?? []) model.nodes.set(node.id, node);
        for (const sample of profile.samples ?? []) model.counts.set(sample, (model.counts.get(sample) ?? 0) + 1);
      }
    });
    const complete = new Promise(resolve => trace.once("Tracing.tracingComplete", resolve));
    await trace.send("Tracing.start", {categories: "disabled-by-default-v8.cpu_profiler,disabled-by-default-v8.cpu_profiler.hires,v8.execute", transferMode: "ReportEvents"});
    finishTrace = async () => {
      await trace.send("Tracing.end");
      await complete;
      for (const [name, {nodes, counts}] of models) {
        for (const [id, count] of [...counts].sort(([, a], [, b]) => b - a).slice(0, 7)) {
          const stack = [];
          for (let node = nodes.get(id); node && stack.length < 28; node = nodes.get(node.parent)) stack.push(node.callFrame);
          if (stack.some(frame => /wasm|reactor.poll|shard-worker/.test(frame?.url ?? "") || /wasm/.test(frame?.functionName ?? ""))) {
            console.log("[DEBUG] worker profile stack", name, count, JSON.stringify(stack));
          }
        }
      }
    };
  }
  if (process.env.SEMIO_NOTE_CDP === "1") {
    const cdp = await context.newCDPSession(page);
    let nextId = 0;
    const shardSessions = new Set();
    const coreScripts = new Map();
    const pending = new Map();
    let cameraPhase;
    let cameraSteps = [];
    const send = (sessionId, method, params = {}, resolve) => {
      const id = ++nextId;
      pending.set(id, {method, resolve});
      return cdp.send("Target.sendMessageToTarget", {sessionId, message: JSON.stringify({id, method, params})})
        .catch(error => {
          console.log("[DEBUG] worker CDP message failed", method, String(error));
          resolve?.();
        });
    };
    const request = (sessionId, method, params) => new Promise(resolve => {
      const timeout = setTimeout(resolve, 500);
      void send(sessionId, method, params, packet => {
        clearTimeout(timeout);
        resolve(packet);
      });
    });
    const inspectWasmMemory = async (sessionId, frame) => {
      const locals = frame.scopeChain.find(scope => scope.type === "local")?.object.objectId;
      if (!locals) return;
      const packet = await request(sessionId, "Runtime.getProperties", {objectId: locals, ownProperties: true});
      const inspectWork = process.env.SEMIO_NOTE_CAMERA_WORK === "1";
      const wanted = inspectWork ? ["$var2", "$var4", "$var15"] : ["$var0", "$var1", "$var2", "$var3"];
      const values = {};
      for (const field of packet?.result?.result ?? []) {
        if (!wanted.includes(field.name) || !field.value?.objectId) continue;
        const nested = await request(sessionId, "Runtime.getProperties", {objectId: field.value.objectId, ownProperties: true});
        const number = nested?.result?.result?.find(property => property.name === "value")?.value;
        values[field.name] = number?.value ?? number?.unserializableValue ?? number?.description;
      }
      console.log("[DEBUG] camera Wasm raw locals", JSON.stringify(values));
      const address = inspectWork ? values.$var4 : values.$var1;
      if (!Number.isSafeInteger(address)) return;
      const offsets = inspectWork ? [568] : [10832, 15447, 15467];
      const sample = await request(sessionId, "Runtime.evaluate", {
        expression: `(globalThis.__noteWasmMemory ?? []).map(memory => ({ size: memory.buffer.byteLength, words: ${JSON.stringify(offsets)}.map(offset => Array.from(new Uint8Array(memory.buffer).slice(${address} + offset, ${address} + offset + 32))) }))`,
        returnByValue: true,
      });
      console.log("[DEBUG] camera Wasm memory", JSON.stringify(sample?.result ?? sample?.error));
    };
    cdp.on("Target.attachedToTarget", ({sessionId, targetInfo}) => {
      if (targetInfo.type !== "worker" || !decodeURIComponent(targetInfo.url).includes("shard-worker.js")) return;
      shardSessions.add(sessionId);
      console.log("[DEBUG] worker CDP attached", sessionId);
      void send(sessionId, "Debugger.enable");
    });
    cdp.on("Target.receivedMessageFromTarget", ({sessionId, message}) => {
      if (!shardSessions.has(sessionId)) return;
      const packet = JSON.parse(message);
      if (packet.id) {
        const {method: requested, resolve} = pending.get(packet.id) ?? {};
        resolve?.(packet);
        if (requested === "Runtime.getProperties" && !resolve) {
          const fields = packet.result?.result ?? [];
          if (process.env.SEMIO_NOTE_CAMERA_CANONICAL === "1") {
            for (const field of fields.filter(field => field.name === "$var1" || field.name === "$var3")) {
              if (field.value?.objectId) void send(sessionId, "Runtime.getProperties", {objectId: field.value.objectId, ownProperties: true});
            }
          }
          const values = fields.slice(0, 25).map(field => [field.name, field.value?.value ?? field.value?.description]);
          console.log("[DEBUG] camera Wasm locals", sessionId, JSON.stringify(values.length ? values : packet.error));
        } else {
          console.log("[DEBUG] worker CDP reply", sessionId, requested, JSON.stringify(packet).slice(0, 450));
        }
        pending.delete(packet.id);
      }
      if (packet.method === "Debugger.scriptParsed" && packet.params.url?.includes("semio_hub_note_component.core.wasm")) {
        coreScripts.set(sessionId, packet.params.scriptId);
      }
      if (packet.method === "Debugger.paused") {
        const frames = packet.params.callFrames;
        const captureLocals = process.env.SEMIO_NOTE_CAMERA_LOCALS === "1" && frames[0]?.functionName === "$func181795"
          || process.env.SEMIO_NOTE_CAMERA_CANONICAL === "1" && frames[0]?.functionName === "$func177534" && cameraSteps.length === 0;
        if (captureLocals && cameraPhase === "after attempted edit") {
          for (const scope of frames[0].scopeChain.filter(scope => scope.type === "local" || scope.type === "wasm-expression-stack")) {
            if (scope.object.objectId) void send(sessionId, "Runtime.getProperties", {objectId: scope.object.objectId, ownProperties: true});
          }
        }
        const cleared = (packet.params.hitBreakpoints ?? []).map(breakpointId => send(sessionId, "Debugger.removeBreakpoint", { breakpointId }));
        if (process.env.SEMIO_NOTE_CAMERA_CALLEE === "1" && cameraPhase === "after attempted edit" && frames[0]?.location.columnNumber === 73675298) {
          console.log("[DEBUG] camera operation callee breakpoint", sessionId);
          for (const scope of frames[0].scopeChain.filter(scope => scope.type === "local" || scope.type === "wasm-expression-stack")) {
            if (scope.object.objectId) void send(sessionId, "Runtime.getProperties", {objectId: scope.object.objectId, ownProperties: true});
          }
          void Promise.all(cleared).then(() => send(sessionId, "Debugger.stepInto"));
          return;
        }
        const steppedFunction = process.env.SEMIO_NOTE_CAMERA_AUTHORITY === "1" ? "$func175811"
          : process.env.SEMIO_NOTE_CAMERA_CANONICAL === "1" ? "$func177534"
          : process.env.SEMIO_NOTE_CAMERA_SEAL === "1" ? "$func177502"
          : process.env.SEMIO_NOTE_CAMERA_PREPARATION === "1" ? "$func178336" : "$func181795";
        const step = process.env.SEMIO_NOTE_CAMERA_STEP === "1" && cameraPhase === "after attempted edit"
          && frames[0]?.functionName === steppedFunction && cameraSteps.length < 200;
        if (step) {
          if (cameraSteps.length === 0) {
            console.log("[DEBUG] camera publication step stack", sessionId, JSON.stringify(frames.slice(0, 20).map(frame => [frame.functionName, frame.location.columnNumber])));
            if (process.env.SEMIO_NOTE_CAMERA_MEMORY === "1") cleared.push(inspectWasmMemory(sessionId, frames[0]));
          }
          cameraSteps.push(frames[0].location.columnNumber);
          void Promise.all(cleared).then(() => send(sessionId, "Debugger.stepOver"));
        } else {
          if (cameraSteps.length > 0) {
            console.log("[DEBUG] camera publication stepped offsets", JSON.stringify(cameraSteps));
            cameraSteps = [];
            cameraPhase = undefined;
          }
          console.log("[DEBUG] worker CDP stack", sessionId, JSON.stringify(frames.slice(0, 50).map(frame => [frame.functionName, frame.url, frame.location])));
          void Promise.all(cleared).then(() => send(sessionId, "Debugger.resume"));
        }
      } else if (packet.error || packet.method === "Debugger.scriptParsed" && packet.params.scriptLanguage === "WebAssembly") {
        console.log("[DEBUG] worker CDP", sessionId, JSON.stringify(packet).slice(0, 850));
      }
    });
    await cdp.send("Target.setAutoAttach", {autoAttach: true, waitForDebuggerOnStart: false, flatten: false});
    const sampleShards = () => {
      for (const sessionId of shardSessions) {
        void send(sessionId, "Runtime.evaluate", { expression: "performance.now()", returnByValue: true });
        void send(sessionId, "Debugger.pause");
      }
    };
    if (process.env.SEMIO_NOTE_CAMERA_BREAKS === "1") {
      armCameraBreakpoints = async (phase) => {
        cameraPhase = phase;
        console.log("[DEBUG] arm camera breakpoints", phase, coreScripts.size);
        for (const [sessionId, scriptId] of coreScripts) {
          const locations = [31374888, 37714908, 37753829, 37757852, 37793868, 73642405, 73673312, 73673685, 73933555, 74097550];
          if (process.env.SEMIO_NOTE_CAMERA_LOCALS === "1") locations.push(73686405, 73686511, 73686545, 73687226, 73697063);
          if (process.env.SEMIO_NOTE_CAMERA_CALLEE === "1" && phase === "after attempted edit") locations.push(73675298);
          if (process.env.SEMIO_NOTE_CAMERA_PREPARATION === "1" && phase === "after attempted edit") locations.push(71768402, 71770436, 71808200, 71704014, 71804073);
          if (process.env.SEMIO_NOTE_CAMERA_SEAL === "1" && phase === "after attempted edit") locations.push(71339196, 71338748, 74171855, 71699963, 71386366);
          if (process.env.SEMIO_NOTE_CAMERA_CANONICAL === "1" && phase === "after attempted edit") locations.push(71386366);
          if (process.env.SEMIO_NOTE_CAMERA_AUTHORITY === "1" && phase === "after attempted edit") {
            locations.push(process.env.SEMIO_NOTE_CAMERA_WORK === "1" ? 70516742 : 70516253);
          }
          for (const columnNumber of locations) {
            await send(sessionId, "Debugger.setBreakpoint", {location: {scriptId, lineNumber: 0, columnNumber}});
          }
        }
      };
    } else {
      const samples = [9500, 14000, 21000].map(delay => setTimeout(sampleShards, delay));
      page.once("close", () => samples.forEach(clearTimeout));
    }
  }
  const moduleRequest = address => /semio_hub_note|(?:program|plugin|backbone)[-/].*worker|worker.*(?:program|plugin|backbone)|core[0-9]*\.wasm|bridge\.js|semio-plugins|plugin-module\b/.test(decodeURIComponent(new URL(address).pathname));
  context.on("request", request => {
    if (moduleRequest(request.url())) console.log("[DEBUG] module request", request.method(), decodeURIComponent(new URL(request.url()).pathname));
  });
  context.on("requestfailed", request => console.log("[DEBUG] request failed", request.failure()?.errorText, decodeURIComponent(new URL(request.url()).pathname)));
  page.on("worker", worker => console.log("[DEBUG] worker", decodeURIComponent(new URL(worker.url()).pathname)));
  if (process.env.SEMIO_NOTE_TRACE_INSTALL === "1") {
    await context.addInitScript(() => {
      const describe = event => ({
        kind: event?.kind ?? event?.wire?.kind,
        requestId: event?.requestId,
        actorId: typeof event?.actorId === "string" ? event.actorId : undefined,
        ok: event?.ok,
        error: event?.error ?? event?.message,
        phase: event?.phase ?? event?.beat?.phase,
        status: event?.value?.status,
        drivePolls: event?.timings?.drivePolls,
        driveStopped: event?.timings?.driveStopped,
        resultKeys: event?.value ? Object.keys(event.value) : undefined,
        events: event?.events?.map(input => ({
          keys: Object.keys(input ?? {}),
          kind: input?.kind ?? input?.tag ?? input?.payload?.kind ?? input?.payload?.tag,
          operation: input?.operation ?? input?.val?.operation,
        })),
        commandPage: event?.commandPage?.byteLength,
        returnDrive: event?.returnDrive?.byteLength,
        receipt: event?.receipt ? {
          keys: Object.keys(event.receipt),
          work: event.receipt.workUnitsSpent,
          nativeRemaining: event.receipt.nativeRemaining,
          nativeDisposition: event.receipt.nativeReceipt?.disposition,
          spent: event.receipt.spent,
          inputTurn: event.receipt.input?.turn,
          inputSequence: event.receipt.input?.sequence,
        } : undefined,
      });
      const counts = new Map();
      const wireLog = (label, event) => {
        const detail = describe(event);
        const key = JSON.stringify([label, detail.kind, detail.phase, detail.status, detail.events?.map(input => input.kind)]);
        const count = (counts.get(key) ?? 0) + 1;
        counts.set(key, count);
        if ((globalThis.__noteCaptureCamera && count <= 70) || count <= 5 || count % 1000 === 0) console.warn("[DEBUG] shard", label, count, JSON.stringify(detail, (_key, value) => typeof value === "bigint" ? String(value) : value));
      };
      const NativeWorker = globalThis.Worker;
      globalThis.Worker = class TraceWorker extends NativeWorker {
        constructor(url, options) {
          super(url, options);
          this.traceShard = decodeURIComponent(String(url)).includes("shard-worker.js");
          if (this.traceShard) {
            this.addEventListener("message", ({data}) => wireLog("received", data));
            this.addEventListener("error", ({message}) => console.warn("[DEBUG] shard error", message));
          }
        }
        postMessage(message, ...args) {
          if (this.traceShard) wireLog("sent", message);
          return super.postMessage(message, ...args);
        }
      };
    });
    await context.route(url => decodeURIComponent(url.pathname).endsWith("/🗒️note/🌉️bridge.js"), async route => {
      const response = await route.fetch();
      let source = await response.text();
      if (process.env.SEMIO_NOTE_CAMERA_MEMORY === "1") {
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
      const before = "result = await reactor.poll(slice);";
      if (source.includes(before)) source = source.replace(before, `if (events.length > 0 || slice.fuel > 49994570n) console.warn("[DEBUG] guest poll begin", JSON.stringify({events: events.map(value => value.kind), fuel: String(slice.fuel)}));
        if (!globalThis.__noteProbeInterval) globalThis.__noteProbeInterval = setInterval(() => console.warn("[DEBUG] guest worker timer", performance.now()), 1800);
        ${before}
        const summary = {
          status: result.status?.tag,
          patches: result.uiPatches?.length,
          lifecycleReceipt: result.lifecycleReceipt !== undefined,
          effects: result.effects?.map(effect => [effect.tag ?? effect.kind, effect.val?.tag ?? effect.val?.kind, Object.keys(effect.val ?? effect)]),
          work: result.retainedReceipt?.workUnitsSpent,
          nextWake: result.nextWake,
        };
        const key = JSON.stringify([summary.status, summary.patches, summary.lifecycleReceipt, summary.effects]);
        const counts = globalThis.__noteProbeCounts ??= new Map();
        const count = (counts.get(key) ?? 0) + 1;
        counts.set(key, count);
        if (count <= 6 || count % 1000 === 0) console.warn("[DEBUG] guest poll end", JSON.stringify({...summary, count}, (_key, value) => typeof value === "bigint" ? String(value) : value));`);
      else console.log("[DEBUG] guest poll trace marker absent");
      await route.fulfill({response, body: source});
    });
    await context.route(url => decodeURIComponent(url.pathname).includes("lifecycle-state-and-install/"), async route => {
      const response = await route.fetch();
      let source = await response.text();
      const markers = [
        ["async (pluginId, rebuiltAt) => {", "install entered", "pluginId, rebuiltAt"],
        ["await hubPluginSource?.ready()", "awaiting hub", "pluginId"],
        ["if (!entry) {", "lookup resolved", "pluginId, Boolean(entry)"],
        ["acquired = await pluginSource.acquireModule", "acquire start", "pluginId"],
        ["const handle = await loadPluginModuleResilient", "load start", "pluginId, acquired.moduleUrl"],
      ];
      for (const [marker, label, values] of markers) {
        const index = source.indexOf(marker);
        if (index === -1) {
          console.log("[DEBUG] install trace marker absent", label);
          continue;
        }
        const position = marker.endsWith("=> {") ? index + marker.length : index;
        source = source.slice(0, position) + `console.warn("[DEBUG] install ${label}", ${values});\n` + source.slice(position);
      }
      await route.fulfill({response, body: source});
    });
  }
  page.on("pageerror", error => console.log("[DEBUG] page error", String(error)));
  page.on("console", message => {
    if (message.type() === "error" || message.type() === "warning" || /semio dev|shell|boot/i.test(message.text()) && !/typed-operation/.test(message.text())) {
      console.log("[DEBUG] console", message.type(), message.text().slice(0, 1800));
    }
  });
  page.on("websocket", socket => {
    console.log("[DEBUG] socket", socket.url());
    socket.on("framereceived", ({payload}) => console.log("[DEBUG] socket received", String(payload).slice(0, 400)));
    socket.on("framesent", ({payload}) => console.log("[DEBUG] socket sent", String(payload).slice(0, 400)));
    socket.on("socketerror", error => console.log("[DEBUG] socket error", error));
    socket.on("close", () => console.log("[DEBUG] socket closed", socket.url()));
  });
  page.on("response", async response => {
    if (decodeURIComponent(response.url()).includes("identity-binding") && response.ok()) {
      const body = await response.text();
      console.log("[DEBUG] identity announcement", body.split("\n").filter(line => /STREAM_MUX_ANNOUNCEMENT|stream-mux|SEMIO_STREAM/.test(line)).map(line => line.slice(0, 700)).slice(0, 12));
    }
  });
  context.on("response", response => {
    const path = decodeURIComponent(new URL(response.url()).pathname);
    if (path.startsWith("/🔌️plugin-modules/") || response.status() >= 400) {
      console.log("[DEBUG] response", response.status(), response.headers()["content-length"], path);
    }
  });
  await page.goto("http://127.0.0.1:6080/?plugin=note&role=editor", {waitUntil:"domcontentloaded",timeout:35000});
  for (let i=0;i<8;i++) {
    const snapshot=await page.evaluate(() => ({
      ready: document.querySelector("[data-semio-os-ready]")?.getAttribute("data-semio-os-ready") ?? null,
      error: document.querySelector("[data-semio-os-shell-error], [data-semio-os-error], [role=alert]")?.textContent ?? null,
      notFound: document.querySelector("[data-semio-os-not-found]") !== null,
      canvas: [...document.querySelectorAll("[data-surface-id]")].map(element=>element.getAttribute("data-surface-id")),
      aria: [...document.querySelectorAll("[aria-label]")].map(element => element.getAttribute("aria-label")).slice(-30),
      resources: performance.getEntriesByType("resource").map(entry => new URL(entry.name).pathname)
        .filter(path => path.includes("wasm") || path.includes("semio_hub_note") || path.includes("bridge")).slice(-20),
      text: document.body.innerText.slice(0, 800)
    }));
    console.log("[DEBUG] boot sample",JSON.stringify(snapshot));
    if (snapshot.error || (snapshot.ready !== null && snapshot.canvas.includes("window:note-composite"))) break;
    await page.waitForTimeout(4500);
  }
  if (process.env.SEMIO_NOTE_TRACE_CAMERA === "1") {
    const viewport = page.locator('[data-surface-id="window:note-composite"]').first();
    await viewport.waitFor({state: "visible", timeout: 40000});
    await page.evaluate(() => { globalThis.__noteCaptureCamera = true; });
    const rect = await viewport.boundingBox();
    const point = {x: rect.x + Math.min(180, rect.width / 3), y: rect.y + Math.min(180, rect.height / 3)};
    await page.mouse.move(point.x, point.y);
    await armCameraBreakpoints?.("before wheel");
    await page.mouse.wheel(0, -120);
    await page.waitForTimeout(3500);
    await viewport.focus();
    const handled = await viewport.evaluate(root => {
      const files = new DataTransfer();
      files.items.add(new File(['<svg xmlns="http://www.w3.org/2000/svg" width="12" height="8"><rect width="12" height="8"/></svg>'], "probe.svg", {type: "image/svg+xml"}));
      return !root.dispatchEvent(new ClipboardEvent("paste", {clipboardData: files, bubbles: true, cancelable: true}));
    });
    for (let i = 0; i < 5; i++) {
      const scene = await viewport.evaluate(root => ({
        transform: root.querySelector(":scope > div.origin-top-left")?.style.transform,
        images: [...root.querySelectorAll("img")].map(image => [image.naturalWidth, image.naturalHeight]),
      }));
      console.log("[DEBUG] camera witness", JSON.stringify({handled, ...scene}));
      if (scene.images.some(([width, height]) => width === 12 && height === 8)) break;
      if (i === 2) await armCameraBreakpoints?.("after attempted edit");
      await page.waitForTimeout(3500);
    }
  }
  await finishTrace?.();
} finally {
  await browser.close();
}
