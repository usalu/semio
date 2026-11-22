/** 🎭️ Runs the authored Draw instance lifecycle through the real browser component and bridge. */
import { strict as assert } from "node:assert";
import { join } from "node:path";
import { chromium } from "playwright";
import { encodePackValue } from "@semio-tech/framework/packs";
import { decodeActorInstanceLifecycle, type ActorInstanceLifecycleReceipt } from "@semio-tech/framework-actor/lifetime";

const [moduleRoot, closeProfile, closeObservation] = Bun.argv.slice(2);
if (!moduleRoot) throw Error("Expected the staged Draw browser module");
const root = process.cwd();
const bundle = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const { pluginComponentBridgeSource } = await import(join(bundle, "🏗️materialization/🌉️component-bridge-source/🟦️.ts"));
const name = "semio_hub_draw_component";
const published = process.env.DRAW_PROBE_PUBLISHED === "true";
const simulateWindowWork = process.env.DRAW_PROBE_WINDOW_METADATA_WORK === "true";
if (simulateWindowWork && (published || !closeObservation)) throw Error("Window work simulation requires a staged diagnostic component");
const bridgeFile = published ? "🖍️draw/🌉️bridge.js" : "module/draw-lifecycle-bridge.js";
const bridge = pluginComponentBridgeSource(name, name + ".wasm");
if (published) {
  const receipt = await Bun.file(join(moduleRoot, "🖍️draw", "🧾️receipt.json")).json();
  console.log("[DEBUG] Draw lifecycle product", receipt.wasmSha256, receipt.semanticDescriptorSha256);
}
const fixture = await Bun.file(join(root, "🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/scenarios.json")).json();
const actor = process.env.DRAW_PROBE_ACTOR;
const turnLimit = Number(process.env.DRAW_PROBE_DIAGNOSTIC_TURNS ?? fixture.caller.maximumTurns);
if (!Number.isSafeInteger(turnLimit) || turnLimit <= 0) throw Error("Invalid Draw probe turn bound");
const scenarios = Array.from(fixture.scenarios.entries() as Iterable<[number, { actor: string; appId: string; expectedStatus: string }]>)
  .filter(([, scenario]) => !actor || scenario.actor === actor);
if (!scenarios.length) throw Error("No Draw scenario matched " + actor);
const breakpointFrom = Number(process.env.DRAW_PROBE_BREAK_FROM ?? 1000);
const breakpointAddresses = (process.env.DRAW_PROBE_BREAKS ?? "43671953,43671771,43888749").split(",").map(Number);
const breakpointMax = Number(process.env.DRAW_PROBE_BREAK_MAX ?? 6);
const files = new Set<string>();
for await (const path of new Bun.Glob(published ? "{🖍️draw,🪞️vendor}/**/*" : "{module,vendor}/**/*").scan({ cwd: moduleRoot, type: "file" })) files.add(path);
const server = Bun.serve({
  hostname: "127.0.0.1",
  port: 0,
  fetch(request) {
    const path = decodeURIComponent(new URL(request.url).pathname.slice(1));
    if (!path) return new Response("<!doctype html><html lang=en><title>Draw lifecycle probe</title></html>", { headers: { "content-type": "text/html" } });
    if (!published && path === bridgeFile) return new Response(bridge, { headers: { "content-type": "text/javascript" } });
    if (!files.has(path)) return new Response(null, { status: 404 });
    const mime = path.endsWith(".wasm") ? "application/wasm" : "text/javascript";
    return new Response(Bun.file(join(moduleRoot, path)), { headers: { "content-type": mime } });
  },
});

type LifecycleEvent = { kind: string; payload: object };
type ProbeState = {
  api: { poll(events: LifecycleEvent[], command: undefined, coldPair: undefined, budget: object): Promise<any>; closeOriginal(): Promise<{ sourceClosed: boolean }> };
  receipts: number;
};
declare global { var drawLifecycleProbe: ProbeState; }

let browser;
try {
  browser = await chromium.launch({ headless: true });
  for (const [ordinal, scenario] of scenarios) {
    const page = await browser.newPage();
    const profiler = closeProfile ? await page.context().newCDPSession(page) : undefined;
    let profiling = false;
    const debuggerSession = closeObservation ? await page.context().newCDPSession(page) : undefined;
    let wasmScript: string | undefined;
    let observeOnce = false;
    let closeTurn = 0;
    if (debuggerSession) {
      debuggerSession.on("Debugger.scriptParsed", script => {
        if (script.scriptLanguage === "WebAssembly" && (!wasmScript || script.url.endsWith(".core.wasm"))) wasmScript = script.scriptId;
      });
      await debuggerSession.send("Debugger.enable");
    }
    let diagnostics = 0;
    page.on("console", message => {
      if (diagnostics++ < 40 || /cleanup continuation|close fault|semantic failure/.test(message.text())) console.log("[DEBUG] Draw guest", message.type(), message.text().slice(0, 2000));
    });
    page.on("pageerror", error => console.log("[DEBUG] Chromium page", error.stack?.slice(0, 5000)));
    try {
      await page.goto(server.url.href);
      await page.evaluate(async ({ entry, caller, ordinal, actor, diagnostic }) => {
        if (diagnostic) {
          const { _setEnv } = await import(new URL("/vendor/cli.js", location.href).href);
          _setEnv({ SEMIO_RUNTIME_DIAGNOSTICS: "1" });
        }
        const { createActorApi } = await import(new URL(entry, location.href).href);
        const state = { receipts: 0 } as ProbeState;
        state.api = await createActorApi("draw-lifecycle-" + actor, 1n, {
          operation: BigInt(ordinal + 1), generation: 1n, grant: caller.grant,
          fuel: caller.fuel, maximumElapsedUs: BigInt(caller.lifetimeUs),
        }, () => BigInt(Math.floor(performance.now() * 1000)), {
          receipt() { state.receipts++; },
          hold(value: unknown) { console.error("Draw retained refusal", String(value)); },
        });
        globalThis.drawLifecycleProbe = state;
      }, { entry: "/" + bridgeFile, caller: fixture.caller, ordinal, actor: scenario.actor, diagnostic: !published });
      let captured: ActorInstanceLifecycleReceipt | undefined;
      let retired: ActorInstanceLifecycleReceipt | undefined;
      let next: LifecycleEvent | undefined = { kind: "instance-open", payload: {
        instance: 1, activationGeneration: 1n, requestSequence: 1, appId: scenario.appId,
        actor: scenario.actor, config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})),
      } };
      let closing = false;
      let turns = 0;
      for (; turns < turnLimit; turns++) {
        if (closing) closeTurn++;
        if (debuggerSession && wasmScript && !observeOnce && closeTurn >= breakpointFrom) {
          observeOnce = true;
          const registered = await Promise.all(breakpointAddresses.map(columnNumber => debuggerSession.send("Debugger.setBreakpoint", { location: { scriptId: wasmScript!, lineNumber: 0, columnNumber } })));
          let hits = 0;
          debuggerSession.on("Debugger.paused", async event => {
            try {
              const frame = event.callFrames[0];
              const scope = [];
              const props = async (objectId?: string) => objectId ? (await debuggerSession.send("Runtime.getProperties", { objectId, ownProperties: true })).result : [];
              let pointer: number | undefined, memory: string | undefined;
              const locals: Record<string, unknown> = {};
              const operandStack: unknown[] = [];
              for (const owner of frame.scopeChain) {
                const fields = await props(owner.object.objectId);
                if (owner.type === "wasm-expression-stack") {
                  const items = await props(fields.find(field => field.name === "stack")?.value?.objectId);
                  for (const item of items) {
                    const value = (await props(item.value?.objectId)).find(field => field.name === "value")?.value;
                    operandStack.push({ name: item.name, type: item.value?.description, value: value?.value ?? item.value?.value });
                  }
                }
                if (owner.type === "local") {
                  for (const field of fields.filter(field => /^\$var[0-9]+$/.test(field.name))) {
                    const raw = (await props(field.value?.objectId)).find(property => property.name === "value")?.value ?? field.value;
                    locals[field.name] = raw?.value ?? raw?.unserializableValue ?? raw?.description;
                  }
                  if (typeof locals.$var4 === "number") {
                    pointer = locals.$var4;
                    if (frame.location.columnNumber === 43893185) pointer += 1296;
                    if ([78140965, 78140973, 78141033, 78141278].includes(frame.location.columnNumber ?? -1) && typeof locals.$var5 === "number") pointer = locals.$var5;
                  }
                }
                if (owner.type === "module") {
                  const memories = await props(fields.find(field => field.name === "memories")?.value?.objectId);
                  memory = memories.find(field => field.value?.subtype === "wasm memory" || field.value?.description?.includes("Memory"))?.value?.objectId;
                }
                scope.push({ type: owner.type, fields: fields.map(property => property.name) });
              }
              if (simulateWindowWork && frame.location.columnNumber === 88428724 && memory && typeof locals.$var3 === "number" && locals.$var4 === 0 && locals.$var5 === 2) {
                const call = await debuggerSession.send("Runtime.callFunctionOn", {
                  objectId: memory,
                  functionDeclaration: "function(p) { const view = new DataView(this.buffer); const items = view.getUint32(p + 20, true); if (items === 0 || view.getBigUint64(p, true) !== 0n || view.getBigUint64(p + 8, true) !== 0n) return false; view.setBigUint64(p, 1n, true); return true; }",
                  arguments: [{ value: locals.$var3 }], returnByValue: true,
                });
                if (call.result?.value) console.log("[DEBUG] Simulated a Window metadata Work debit", scenario.actor, closeTurn);
                return;
              }
              let words: unknown;
              if (memory && pointer !== undefined) {
                const response = await debuggerSession.send("Runtime.callFunctionOn", {
                  objectId: memory,
                  functionDeclaration: "function(pointer) { return Array.from(new Uint32Array(this.buffer, pointer, 36)); }",
                  arguments: [{ value: pointer }], returnByValue: true,
                }).catch(error => ({ exceptionDetails: String(error), result: undefined }));
                words = response.result?.value ?? response.exceptionDetails;
              }
              console.log("[DEBUG] Draw close breakpoint", closeTurn, frame.functionName, JSON.stringify(frame.location));
              const file = closeObservation + "-" + scenario.actor + "-" + hits++ + ".json";
              await Bun.write(file, JSON.stringify({ location: frame.location, function: frame.functionName, scope, locals, pointer, memory, words, operandStack }));
              if (hits >= breakpointMax) for (const entry of registered) await debuggerSession.send("Debugger.removeBreakpoint", { breakpointId: entry.breakpointId });
            } finally {
              await debuggerSession.send("Debugger.resume");
            }
          });
        }
        const dispatched = next;
        const reply = await page.evaluate(async event => {
          try {
            const turn = await globalThis.drawLifecycleProbe.api.poll(event ? [event] : [], undefined, undefined, { maxEffects: 16, maxPatchBytes: 1 << 20, maxFrames: 16 });
            return { status: turn.status?.tag ?? turn.status, receipt: turn.lifecycleReceipt ? Array.from(turn.lifecycleReceipt) as number[] : undefined, effects: Array.isArray(turn.effects) ? turn.effects.slice(0, 8).map((effect: { tag?: string }) => effect.tag) : [], patches: turn.uiPatches?.length ?? 0 };
          } catch (error) {
            const primary = error as Error & { originalCall?: unknown; payload?: unknown; cause?: { payload?: unknown } };
            const payload = primary.originalCall ?? primary.payload ?? primary.cause?.payload;
            throw Error(primary.message + " " + JSON.stringify({ event: event?.kind, receipts: globalThis.drawLifecycleProbe.receipts, payload }, (_, value) => typeof value === "bigint" ? value.toString() : value).slice(0, 6000));
          }
        }, dispatched);
        next = undefined;
        if (turns < 90 || dispatched || reply.receipt || turns % 256 === 0) {
          console.log("[DEBUG] Draw lifecycle turn", scenario.actor, turns, closing, reply.status, reply.receipt?.length ?? 0, reply.effects, reply.patches, dispatched?.kind ?? "poll");
        }
        assert.notEqual(reply.status, "faulted", scenario.appId + " produced a faulted actor turn");
        if (reply.receipt) {
          const wire = decodeActorInstanceLifecycle(Uint8Array.from(reply.receipt));
          assert.notEqual(wire.kind, "open");
          assert.notEqual(wire.kind, "close");
          assert.notEqual(wire.kind, "ack");
          const receipt = wire as ActorInstanceLifecycleReceipt;
          console.log("[DEBUG] Draw lifecycle receipt", scenario.actor, turns, receipt.kind, receipt.requestSequence);
          assert.equal(receipt.lifetime.instanceId, 1);
          assert.equal(receipt.lifetime.activationGeneration, 1n);
          assert.equal(receipt.requestSequence, closing ? 2 : 1);
          if (closing) {
            assert.deepEqual(receipt.lifetime, captured?.lifetime);
            if (receipt.kind === "retired") retired = receipt;
          } else {
            assert.equal(receipt.kind, "captured");
            captured = receipt;
          }
          next = { kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt } };
        }
        if (reply.status !== scenario.expectedStatus || next) continue;
        assert(captured, scenario.appId + " settled without a capture");
        if (!closing) {
          if (profiler) {
            await profiler.send("Profiler.enable");
            await profiler.send("Profiler.setSamplingInterval", { interval: 100 });
            await profiler.send("Profiler.start");
            profiling = true;
          }
          closing = true;
          next = { kind: "instance-close", payload: { kind: "close", lifetime: captured.lifetime, requestSequence: 2 } };
          continue;
        }
        assert(retired, scenario.appId + " settled without terminal retirement");
        break;
      }
      assert(turns < turnLimit, "Draw exceeded the browser lifecycle turn bound " + turnLimit);
      const end = await page.evaluate(async () => {
        const state = globalThis.drawLifecycleProbe;
        const { sourceClosed } = await state.api.closeOriginal();
        return { sourceClosed, receipts: state.receipts };
      });
      assert(end.sourceClosed);
      assert(end.receipts > turns);
      console.log(JSON.stringify({ actor: scenario.actor, appId: scenario.appId, turns: turns + 1, ...end }));
    } finally {
      if (profiler && profiling) {
        const { profile } = await profiler.send("Profiler.stop");
        await Bun.write(closeProfile + "-" + scenario.actor + ".cpuprofile", JSON.stringify(profile));
      }
      await page.close();
    }
  }
} finally {
  await browser?.close();
  await server.stop(true);
}
