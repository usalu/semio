/** 🔬️ Traces every command turn on the actor's lane: who runs, how long each answer takes, what the guest answers per continuation. */
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = performance.now();
const names = process.env.ADDS ? process.env.ADDS.split(",") : ["Add Ellipse", "Add Rectangle", "Add Polygon", "Add Line"];
let patched = 0;
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
  body = body.replace(/ledgerEntry\.step\(step, progressed\.value\);/, m => `if (step % 64 === 0) console.log("[TRACE] job-step", String(job), step);${m}`);
  body = body.replace(/const deliverJobCompletionTurn = async \(instanceId, actorId, job, outcome\) => \{/, m => `${m}console.log("[TRACE] job-completion-turn", String(job));`);
  body = body.replace(/turnOutcomes\.push\(\{ instanceId, frames: outFrames \}\);/, m => `console.log("[TRACE] frames-out", inspected.operation, JSON.stringify(outFrames.map(f => { const d = decodeAppFrame(f); const k = Object.keys(d)[0]; return k === "Error" ? [k, d.Error.fault] : k === "Done" ? [k, d.Done] : [k, JSON.stringify(d[k], (a, v) => typeof v === "bigint" ? String(v) : v instanceof Uint8Array ? "u8[" + v.length + "]" : v).slice(0, 200)]; }), (a, v) => typeof v === "bigint" ? String(v) : v instanceof Uint8Array ? new TextDecoder().decode(v) : v));${m}`);
  patched += body === before ? 0 : 1;
  await route.fulfill({ response, body });
});
page.on("console", message => { if (message.text().startsWith("[TRACE]") || message.type() === "error") console.log(Math.round(performance.now() - t0), message.text().slice(0, 600)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 120000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const hash = async () => createHash("sha256").update(await page.screenshot({ clip: { x: 310, y: 85, width: 980, height: 820 } })).digest("hex").slice(0, 10);
const rows = () => page.locator('[role="treeitem"][id*="drawing-play-layers.shape."]').count();
for (const name of names) {
  const before = await hash(), count = await rows(), start = performance.now();
  console.log(Math.round(performance.now() - t0), "CLICK", name);
  await page.getByText(name, { exact: true }).first().click();
  let row = null, canvas = null;
  while (performance.now() - start < 40000 && (row === null || canvas === null)) {
    if (row === null && await rows() > count) row = Math.round(performance.now() - start);
    if (canvas === null && await hash() !== before) canvas = Math.round(performance.now() - start);
    await page.waitForTimeout(150);
  }
  console.log(Math.round(performance.now() - t0), "PATCHED", patched, name, "row ms", row, "canvas ms", canvas);
  await page.waitForTimeout(4000);
}
await browser.close();
