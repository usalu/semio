import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe: rewrites ONLY served modules (repo untouched) to log, per ui refresh, generation, superseded or not, requestedEffects and where each effect goes, from boot (no manual ask unless PROBE_ASK=1) through 150 s. One browser, hard cap. */
const out = join(import.meta.dirname, "🗑️generated/effect-loss"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "a";
const bootWait = Number(process.env.PROBE_BOOT ?? 40000), after = Number(process.env.PROBE_AFTER ?? 150000);
const tracePack = process.env.PROBE_PACK_WIRE === "1";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 330000);
const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const L = `(globalThis.__el ??= [])`;
const sum = `((e) => e.dispatchAction ? "dispatchAction:" + e.dispatchAction.action : e.tag ?? e.kind ?? (typeof e === "string" ? e : Object.keys(e)[0]))`;
const patched = {};
const rewrite = (name, needleFn) => ctx.route((u) => decodeURIComponent(u.toString()).split("?")[0].includes(name), async (route) => {
  const res = await route.fetch(); let body = await res.text(); const before = body; body = needleFn(body); patched[name] = body !== before; await route.fulfill({ response: res, body });
});
await rewrite("PluginRuntime/🧵️turn-dispatch/🟦️.ts", (b) => b.replace("row.effects += result?.effects.length ?? 0;", `$& { const __n = performance.now(); if ((__n > 129000 && __n < 160000) || key.includes("request")) ${L}.push(Math.round(__n) + " CROSSING " + key + " ev=" + events.length + " eff=" + (result?.effects?.length ?? "-") + " patches=" + (result?.uiPatches?.length ?? "-") + " status=" + JSON.stringify(result?.status ?? null).slice(0, 50) + " fx=" + (result?.effects ?? []).map((e) => e.tag).slice(0, 4).join(",")); } if (result?.effects?.length) { const t = result.effects.map(${sum}).filter((s) => !s.startsWith("send-message")); if (t.length) ${L}.push(Math.round(performance.now()) + " TURN " + key + " " + t.join(",")); }`));
await rewrite("ShellHost/🔁️contributions-refresh/🟦️.ts", (b) => {
  b = b.replace("const generation = ++refreshGenerationRef.current;", `$& ${L}.push(Math.round(performance.now()) + " REFRESH-START gen=" + generation + " inst=" + nextSession.instanceId);`);
  let n = 0;
  const guard = "if (generation !== refreshGenerationRef.current && !(replaceBodies && generation === replaceBodiesGenerationRef.current)) return;";
  b = b.split(guard).join("@@G@@");
  const where = ["after-body-refresh", "after-slots", "after-sections"];
  const vars = ["response", "response", "sections"];
  b = b.replace(/@@G@@/g, () => { const i = n++; const v = vars[i]; return `{ const __sup = generation !== refreshGenerationRef.current && !(replaceBodies && generation === replaceBodiesGenerationRef.current); ${L}.push(Math.round(performance.now()) + " REFRESH-CHECK ${where[i]} gen=" + generation + " cur=" + refreshGenerationRef.current + " superseded=" + __sup + " requested=[" + (${v}.requestedEffects ?? []).map(${sum}).join(",") + "]"); if (__sup) return; }`; });
  b = b.replace("if (pendingRefreshEffects.length) owedPassEffectsRef.current =", `${L}.push(Math.round(performance.now()) + " REFRESH-OWED gen=" + generation + " pending=[" + pendingRefreshEffects.map(${sum}).join(",") + "]"); if (pendingRefreshEffects.length) owedPassEffectsRef.current =`);
  b = b.replace("if (owedEffects) void applyHostEffectsRef.current(", `${L}.push(Math.round(performance.now()) + " LANE-AFTER-PASS owed=" + (owedEffects ? "[" + owedEffects.effects.map(${sum}).join(",") + "]" : "none")); if (owedEffects) void applyHostEffectsRef.current(`);
  b = b.replace(/const contributionResponse = await pluginEntry\.handle\.handleCommand\(instanceId, wire, environment\.targetViewState\);/, `const __t0 = performance.now(); const contributionResponse = await pluginEntry.handle.handleCommand(instanceId, wire, environment.targetViewState); ${L}.push(Math.round(performance.now()) + " SETCONTRIB plugin=" + pluginEntry.handle.pluginId + " inst=" + instanceId + " jsonlen=" + json.length + " dur=" + Math.round(performance.now() - __t0) + " effects=" + (contributionResponse.requestedEffects ?? []).length);`);
  b = b.replace(/program\.refreshUi\(nextSession\.instanceId, bodyRequest\)/, `(async () => { const __t = performance.now(); try { return await program.refreshUi(nextSession.instanceId, bodyRequest); } finally { ${L}.push(Math.round(performance.now()) + " GUEST-REFRESH dur=" + Math.round(performance.now() - __t) + " gen=" + generation); } })()`);
  b = b.replace(/const contributionsInstalled = publishContributions\(([^;]*)\);/, `const contributionsInstalled = publishContributions($1); ${L}.push(Math.round(performance.now()) + " PASS-LOADED gen=" + generation + " loaded=" + loadedPlugins.map((p) => p.handle.pluginId).join("|")); contributionsInstalled.then((o) => ${L}.push(Math.round(performance.now()) + " PUBLISH-OUTCOME gen=" + generation + " " + JSON.stringify(o).slice(0, 200)), (e) => ${L}.push(Math.round(performance.now()) + " PUBLISH-REJECT " + String(e?.message ?? e).slice(0, 200)));`);
  return b;
});
await rewrite("PluginRuntime/🧩️module-loader/🟦️.ts", (b) => {
  if (tracePack) {
    b = b.replace(/(return withTypedOperationCall\(actorId, `extension-completion#\$\{instanceId\}`, \(call\) => serializeCommandIngressForActor\(actorId, async \(\) => \{)/, `$1 ${L}.push(Math.round(performance.now()) + " PACK-INGRESS req=" + req + " actor=" + actorId + " bytes=" + answer.length);`);
    b = b.replace("await submitTurn(actorId, events, { activation }),", `await (async () => { ${L}.push(Math.round(performance.now()) + " PACK-SUBMIT req=" + req); const result = await submitTurn(actorId, events, { activation }); ${L}.push(Math.round(performance.now()) + " PACK-TURN req=" + req + " status=" + JSON.stringify(result.status) + " effects=" + result.effects.map(${sum}).join(",")); return result; })(),`);
    b = b.replace('return invocationFromFrames(frames.map(decodeAppFrame), effects, "extension completion");', `const decoded = frames.map(decodeAppFrame); ${L}.push(Math.round(performance.now()) + " PACK-SETTLED req=" + req + " status=" + JSON.stringify(settled.status) + " frames=" + decoded.map(${sum}).join(",") + " effects=" + effects.map(${sum}).join(",")); return invocationFromFrames(decoded, effects, "extension completion");`);
  }
  return b.replace("ownedUiRefreshResponse(instanceId, actorId, request, routeHostEffects(instanceId, result.effects, documentPort)),", `{ const __r = routeHostEffects(instanceId, result.effects, documentPort); const __t = __r.map(${sum}).filter((s) => !s.startsWith("send-message")); if (__t.length) ${L}.push(Math.round(performance.now()) + " REFRESHUI-ROUTED inst=" + instanceId + " " + __t.join(",")); return ownedUiRefreshResponse(instanceId, actorId, request, __r).then((x) => { if (x.requestedEffects?.length) ${L}.push(Math.round(performance.now()) + " REFRESHUI-RESPONSE requested=" + x.requestedEffects.map(${sum}).join(",")); return x; }, (e) => { if (__t.length) ${L}.push(Math.round(performance.now()) + " REFRESHUI-PROJECT-THREW lost=" + __t.join(",") + " " + String(e?.message).slice(0, 80)); throw e; }); },`).replace("activation.assertActive();\n    return hopTrace.timeAsync(\"refresh.project\"", `activation.assertActive();\n    return hopTrace.timeAsync("refresh.project"`);
});
await rewrite("ShellHost/⚡️host-effects/🟦️.ts", (b) => { const n = "for (const effect of effects) {"; const i = b.indexOf(n); return i < 0 ? b : b.slice(0, i + n.length) + ` { const __s = ${sum}(effect); if (!__s.startsWith("send-message")) ${L}.push(Math.round(performance.now()) + " APPLY " + __s); }` + b.slice(i + n.length); });
await rewrite("on-demand-request/🟦️.ts", (b) => b.replace(/(export function applyOnDemandRequestEffect\(effect[^)]*\) \{)/, `$1 ${L}.push(Math.round(performance.now()) + " ONDEMAND-APPLY");`));
if (process.env.PROBE_TICK_SCOPE === "1") {
  await rewrite("plumbing/🚀️effect-dispatch/🟦️.ts", (b) => b
    .replace("if (!isCurrent()) return;", `const current = isCurrent(); if (action === "flowEvalTick") console.info("[DEBUG] TICK_FIRE " + JSON.stringify({ current, scope: effectDispatchScope(baseSession), panes: [...document.querySelectorAll("[data-surface-id]")].map((pane) => pane.getAttribute("data-surface-id")).slice(0, 35) })); if (!current) return;`)
    .replace(/(handle(?:Action|Command)\(baseSession\.instanceId, )(encodeEffect(?:Action|Command)Invocation\((?:baseSession|targetSession), action, args\))/g, `$1((wire) => { if (action === "flowEvalTick") console.info("[DEBUG] TICK_WIRE " + JSON.stringify({ invocation: JSON.parse(wire), roster: viewState.windowInstances })); return wire; })($2)`)
    .replace("if (isCurrent()) await applyEffects(", `if (action === "flowEvalTick") console.info("[DEBUG] TICK_RETURN " + JSON.stringify({ current: isCurrent(), scopedView: { id: viewState.windowId, kind: viewState.activeWindowKindId, mode: viewState.activeModeId }, effects: (response.requestedEffects ?? []).map(${sum}), output: response.output, uiScope: response.uiScope }).slice(0, 2000)); if (isCurrent()) await applyEffects(`));
  await rewrite("dispatch-media-and-extension-effects/🟦️.ts", (b) => b.replace("const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;", `$& if (dispatchActionId === "flowEvalTick") console.info("[DEBUG] TICK_ARM " + JSON.stringify({ plugin: baseSession.pluginId, id: baseSession.viewState.windowId, kind: baseSession.viewState.activeWindowKindId, current: isCurrentEffectOwner(effectOwner), delayMs, dispatchArgs }).slice(0, 2000));`));
}

await rewrite("ShellHost/🔌️extension-runtime/🟦️.ts", (b) => {
  b = b.replace(/const \{ extensionId, capability, requestJson, req \} = invocation;/, `$& ${L}.push(Math.round(performance.now()) + " EXT-DISPATCH " + extensionId + " " + capability + " reqlen=" + requestJson.length + " loaded=" + plugins.map((p) => p.handle.pluginId).join("|") + " resolved=" + (extensionProgramForV1(plugins, plugins.find((e) => e.handle.pluginId === requester.pluginId), extensionId)?.handle.pluginId ?? "NONE"));`);
  b = b.replace(/raw = await invoke\.call\(/, `${L}.push(Math.round(performance.now()) + " EXT-INVOKE-START " + extensionId + " " + capability), raw = await invoke.call(`);
  b = b.replace(/outcome = \{ ok:/, `${L}.push(Math.round(performance.now()) + " EXT-INVOKE-OK rawlen=" + raw.length), outcome = { ok:`);
  b = b.replace(/outcome = \{ fault:/, `${L}.push(Math.round(performance.now()) + " EXT-INVOKE-FAULT " + fault.code + " " + String(fault.message).slice(0, 300)), outcome = { fault:`);
  b = b.replace(/const response = await completion\.complete\(outcome\);/, `$& ${L}.push(Math.round(performance.now()) + " EXT-COMPLETED effects=" + (response.requestedEffects ?? []).length);`);
  if (tracePack) {
    b = b.replace(/import \{ encodePackValue \} from/, "import { encodePackValue, decodePackValue } from");
    b = b.replace(/(?=const response = await completion\.complete\(outcome\);)/, `{ const bytes = "ok" in outcome ? outcome.ok : outcome.fault; let value; try { value = bytes.length <= 4096 ? decodePackValue(bytes) : { bytes: bytes.length }; } catch (error) { value = String(error?.message); } ${L}.push(Math.round(performance.now()) + " PACK-QUEUE req=" + req + " extension=" + extensionId + " kind=" + ("ok" in outcome ? "ok" : "fault") + " body=" + (JSON.stringify(value, (_, item) => typeof item === "bigint" ? String(item) : item) ?? "undefined").slice(0, 360)); } `);
  }
  return b;
});
if (process.env.PROBE_TICK_SCOPE !== "1") await rewrite("dispatch-media-and-extension-effects/🟦️.ts", (b) => b.replace(/\.catch\(\(error\) => \{/, `.catch((error) => { ${L}.push(Math.round(performance.now()) + " EXT-EFFECT-CATCH " + String(error?.stack ?? error).slice(0, 400));`));
const netLog = []; const tStart = Date.now();
ctx.on("request", (r) => { if (/wasm|component|extension|shard|brep|math/i.test(decodeURIComponent(r.url()))) r.__t = Date.now(); });
ctx.on("requestfinished", async (r) => { if (r.__t) { let size = "?"; try { size = (await r.sizes()).responseBodySize; } catch {} netLog.push(`${r.__t - tStart} +${Date.now() - r.__t}ms size=${size} ${decodeURIComponent(r.url()).slice(-95)}`); } });
await rewrite("on-demand-activation/🟦️.ts", (b) => b.replace(/const outcome = await installPlugin\(entry\.pluginId\);/, `${L}.push(Math.round(performance.now()) + " INSTALL-START " + entry.pluginId); let outcome; try { outcome = await installPlugin(entry.pluginId); } catch (e) { ${L}.push(Math.round(performance.now()) + " INSTALL-THREW " + entry.pluginId + " " + String(e?.stack ?? e).slice(0, 300)); throw e; } ${L}.push(Math.round(performance.now()) + " INSTALL-DONE " + entry.pluginId + " " + outcome);`).replace(/onStatus: \(status\) => \{/, `onStatus: (status) => { ${L}.push(Math.round(performance.now()) + " ONDEMAND-STATUS " + JSON.stringify(status).slice(0, 160));`));
await rewrite("shard-client/📨️messages/🟦️.ts", (b) => b.replace(/export function publishShardWorkerTurnSpans\(timings, answeredAtEpochMs, actorId\) \{/, `$& { const __n = performance.now(); if (__n > 125000 || String(actorId).includes("extension")) ${L}.push(Math.round(__n) + " WORKER-TURN actor=" + actorId + " polls=" + timings.drivePolls + " stopped=" + timings.driveStopped + " status=" + timings.status + " guestMs=" + Math.round((timings.guestLeftAtEpochMs - timings.guestEnteredAtEpochMs) * 10) / 10 + " events=" + timings.events + " kinds=" + timings.eventKinds); }`));
const page = await ctx.newPage();
const lines = []; const consoleLines = []; const t0 = Date.now(); const stamp = () => `+${Date.now() - t0}ms`;
const state = () => page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); return `${p.phase} ${p.fault?.code ?? ""} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal} cancellable=${p.cancellable ?? ""}`; });
const act = () => page.evaluate(() => { try { return String(window.__semioOsInstalledContributions?.(1) ? JSON.parse(window.__semioOsInstalledContributions(1)).map((r) => r.pluginId).join(",") : "none"); } catch (e) { return "ERR"; } });
page.on('console', (m) => { if (/^\[DEBUG\] TICK_/.test(m.text())) { console.log(`${stamp()} ${m.text().slice(0, 2000)}`); consoleLines.push(`${stamp()} ${m.text().slice(0, 2000)}`); } else if (/error|warn/.test(m.type()) || /\[(os-shell|flow|TRACE)|failed|fault|extension/i.test(m.text())) consoleLines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 400)}`); });
page.on('pageerror', (e) => { consoleLines.push(`${Date.now() - t0} pageerror ${e.message.slice(0, 300)}`); console.log(`${stamp()} [DEBUG] PAGE_ERROR ${e.message.slice(0, 300)}`); });
await page.goto(url, { waitUntil: "load" });
import { execSync } from "node:child_process";
const sample = async (ms) => { for (let t = 0; t < ms; t += 10000) { const free = process.platform === "darwin" ? Number((execSync("memory_pressure | tail -1").toString().match(/(\\d+)%/) ?? [0, 100])[1]) : 100; if (free < 30) { lines.push(`ABORT free=${free}%`); break; } const status = `${stamp()} ${await state()} installed=${await act()}`; lines.push(status); if (process.env.PROBE_TICK_SCOPE === "1") console.log(`[DEBUG] ${status}`); await page.waitForTimeout(10000); } };
await sample(bootWait);
lines.push(`patched=${JSON.stringify(patched)}`);
if (process.env.PROBE_ASK === "1") {
  for (const k of ["brep.solid.extrude", "math.vector"]) lines.push(`${stamp()} ASK ${k} ${await page.evaluate((n) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: n })), k)}`);
}
await sample(after);
const log = await page.evaluate(() => (globalThis.__el ?? []).join("\n") + "\n--- census ---\n" + JSON.stringify(globalThis.__semioCrossingCensus ?? null));
writeFileSync(join(out, `${tag}-console.txt`), consoleLines.join("\n"));
writeFileSync(join(out, `${tag}-net.txt`), netLog.join("\n"));
writeFileSync(join(out, `${tag}-log.txt`), lines.join("\n") + "\n--- events (performance.now ms) ---\n" + log);
console.log(lines.join("\n")); console.log(log.slice(0, 12000));
clearTimeout(cap); await browser.close();
