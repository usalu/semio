/** 🔬️ Traces every command turn on the actor's lane: who runs, how long each answer takes, what the guest answers per continuation. */
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 }, permissions: ["clipboard-read", "clipboard-write"] })).newPage();
const t0 = performance.now();
const names = process.env.ADDS ? process.env.ADDS.split(",") : ["Add Ellipse", "Add Rectangle", "Add Polygon", "Add Line"];
let patched = 0;
await page.route(url => decodeURIComponent(url.href).includes("🏛️bootstrap/🟦️.ts"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  const before = body;
  body = body.replace(/record\.fuel\s*-=\s*receipt\.workUnitsSpent;?/, m => `${m}(globalThis.__ops ??= {})[String(original.operation)] = { fuel: record.fuel, items: record.remaining.maximumItems, copy: record.remaining.maximumCopyBytes, cap: record.remaining.maximumCapacityBytes, rel: record.remaining.maximumReleaseBytes, epoch: String(record.epoch), spentLast: receipt.workUnitsSpent };`);
  console.log("BOOTSTRAP PATCHED", body !== before);
  await route.fulfill({ response, body });
});
await page.route(url => decodeURIComponent(url.href).includes("handle-turn-drive"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  const before = body;
  const show = `(t => JSON.stringify({ tag: t.commandIngress?.tag, page: t.commandIngress?.val?.pageIndex, st: wireTurnStatusTag(t.status), def: t.deferredEvents, eff: t.effects.map(e => e.tag), ui: t.uiPatches.length, rcpt: !!t.uiPatchReceipt }, (k, v) => typeof v === "bigint" ? String(v) : v))`;
  body = body.replace(/await acceptTurn\(offered\);/, m => `console.log("[TRACE] offered", inspected.operation, ${show}(offered), "ack", acknowledgements.length);${m}`);
  body = body.replace(/await acceptTurn\(continued\);/, m => `if (continuations < 3 || continuations % 500 === 0 || continued.commandIngress?.tag !== "command-pending" || continued.effects.length || continued.deferredEvents || continued.uiPatches.length) console.log("[TRACE] cont#" + continuations, inspected.operation, ${show}(continued), "ack", acknowledgements.length);${m}`);
  body = body.replace(/activation\.assertActive\(\);\s*importSignal\?\.throwIfAborted\(\);\s*const results/, m => `console.log("[TRACE] lane-acquired", inspected.operation, inspected.actionId);${m}`);
  body = body.replace(/await commitReservedToolSpawnsWhileSerialized\(instanceId, actorId, settled\.effects\);/, m => `console.log("[TRACE] lane-release", inspected.operation, "settled-eff", settled.effects.map(e => e.tag).join(","));${m}`);
  body = body.replace(/\} catch \(error\) \{\s*turnOutcomes\.push\(\{ instanceId, error \}\);\s*\}\s*\};\s*\/\*\*/, m => `} catch (error) { console.log("[TRACE] runQueuedTurn-error", String(error).slice(0, 300));\n${m.slice(m.indexOf("turnOutcomes"))}`);
  body = body.replace(/await shardClient\.startJob\(actorId, job, kind, input\);\s*runningJobs\.add\(key\);\s*\}\);/, m => `console.log("[TRACE] job-start", String(job), kind);${m}`);
  body = body.replace(/if \(outcome\.status !== "running"\) \{\s*if \(live\(\)\) \{/, m => `console.log("[TRACE] job-terminal", String(job), outcome.status, step);${m}`);
  body = body.replace(/ledgerEntry\.step\(step, progressed\.value\);/, m => `if (step % 64 === 0 && false) console.log("[TRACE] job-step", String(job), step);${m}`);
  body = body.replace(/const deliverJobCompletionTurn = async \(instanceId, actorId, job, outcome\) => \{/, m => `${m}console.log("[TRACE] job-completion-turn", String(job));`);
  body = body.replace(/turnOutcomes\.push\(\{ instanceId, frames: outFrames \}\);/, m => `console.log("[TRACE] frames-out", inspected.operation, JSON.stringify(outFrames.map(f => { const d = decodeAppFrame(f); const k = Object.keys(d)[0]; return k === "Error" ? [k, d.Error.fault] : k === "Done" ? [k, d.Done] : [k, JSON.stringify(d[k], (a, v) => typeof v === "bigint" ? String(v) : v instanceof Uint8Array ? "u8[" + v.length + "]" : v).slice(0, 200)]; }), (a, v) => typeof v === "bigint" ? String(v) : v instanceof Uint8Array ? new TextDecoder().decode(v) : v));${m}`);
  body = body.replace(/return \{ status: settled\.status, nextWake: settled\.nextWake \};/, m => `globalThis.__drain = (globalThis.__drain ?? 0) + 1; if (globalThis.__drain % 50 === 1 || settled.effects.length || frames.length) console.log("[TRACE] drain#" + globalThis.__drain, wireTurnStatusTag(settled.status), "eff", settled.effects.map(e => e.tag).join(","), "frames", frames.length, "pendingCompletion", (pendingCompletionEffects.get(instanceId) ?? []).map(e => e.tag).join(","), "wake", String(settled.nextWake));${m}`);
  body = body.replace(/\} while \(rearmedDrains\.has\(instanceId\) && live\(\)\);/, m => `console.log("[TRACE] drain-outcome", JSON.stringify(outcome), "rearmed", rearmedDrains.has(instanceId));${m}`.replace("console.log", "console.log"));
  body = body.replace(/if \(wireTurnStatusTag\(result\.status\) === "more-work"\) void drainTypedOperations\(instanceId\);\s*\} catch/, m => `console.log("[TRACE] command-end", inspected.operation, "status", wireTurnStatusTag(result.status), "nextWake", String(result.nextWake), "leftover", [...pendingTurnEffects.get(instanceId) ?? []].map(e => e.tag).join(","));${m}`);
  body = body.replace(/const drainTypedOperations = async \(instanceId\) => \{/, m => `${m}console.log("[TRACE] drain-call", instanceId, "draining", drainingInstances.has(instanceId), "live", state.disposing, closingInstances.has(instanceId), actorIdByInstance.has(instanceId));`);
  patched += body === before ? 0 : 1;
  await route.fulfill({ response, body });
});
await page.route(url => decodeURIComponent(url.href).includes("turn-dispatch"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  const before = body;
  body = body.replace(/export function serializeCommandIngressForActor\(actorId, run, lane = "Interactive", order\) \{/, match => `${match}
    { const inner = run; const id = (globalThis.__laneId = (globalThis.__laneId ?? 0) + 1); const stack = new Error().stack.split("\\n").slice(2, 6).map(line => decodeURIComponent(line.trim().replace(/http:[^ ]*\\//, "").slice(0, 110))).join(" < ");
      (globalThis.__lane ??= new Map()).set(id, { state: "queued", at: performance.now(), lane, stack });
      run = async () => { const entry = globalThis.__lane.get(id); entry.state = "running"; entry.started = performance.now(); try { return await inner(); } finally { const sig = stack.replace(/\\?t=\\d+/g, "").slice(-70); const d = (globalThis.__laneDone ??= {}); const e = (d[sig] ??= { n: 0, ms: 0, max: 0 }); const ms = performance.now() - entry.started; e.n++; e.ms += ms; e.max = Math.max(e.max, ms); globalThis.__lane.delete(id); } }; }`);
  console.log("LANE PATCHED", body !== before);
  await route.fulfill({ response, body });
});
page.on("console", message => { const t = message.text(); if ((t.startsWith("[TRACE]") && !/cont#[0-9]*[1-9] |job-step|offered LocalInteractionQuery \{"tag":"backpressure/.test(t)) || message.type() === "error" || t.includes("stalled")) console.log(Math.round(performance.now() - t0), message.text().slice(0, 600)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 120000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const rows = () => page.locator('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers"]').evaluateAll(nodes => nodes.map(node => node.id).filter(id => !id.includes(".add.")).map(id => id.split("drawing-play-layers.").pop().slice(0, 26)));
await page.locator('[id$="drawing-play-layers.path.emblem-teal"]').first().click();
await page.waitForTimeout(3000);
const focus = () => page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().focus();
console.log("boot", (await rows()).length);
for (const key of (process.env.KEYS ?? "Meta+c,Meta+v,Meta+x,Meta+v").split(",")) {
  console.log(Math.round(performance.now() - t0), "KEY", key, "rows before", (await rows()).length);
  await focus(); await page.keyboard.press(key); const ops0 = await page.evaluate(() => JSON.stringify(globalThis.__ops)); console.log(Math.round(performance.now() - t0), "OPS before", key, ops0); await page.waitForTimeout(Number(process.env.WAIT ?? 9000));
  console.log(Math.round(performance.now() - t0), "LANE", JSON.stringify(await page.evaluate(() => [globalThis.__laneDone, ...(globalThis.__lane ?? new Map()).values()].map(l => ({ s: l.state, age: Math.round(performance.now() - (l.started ?? l.at)), stack: String(l.stack).slice(0, 200) })))));
  console.log(Math.round(performance.now() - t0), "OPS after", JSON.stringify(await page.evaluate(() => globalThis.__ops)), "AFTER", key, "rows", (await rows()).length);
}
await browser.close();
