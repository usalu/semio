/** 🔬️ Same as probe-local-query-trace.mjs, plus an experiment: after 200 idle backpressure answers to the same page the host steps the actor's running job once while still holding the lane, to test whether the job step releases what the page waits for. MODE=step steps the job, MODE=none only counts. */
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
  body = body.replace(/await shardClient\.startJob\(actorId, job, kind, input\);\s*runningJobs\.add\(key\);/, m => `${m}(globalThis.__jobKeys ??= new Set()).add(key);`);
  body = body.replace(/runningJobs\.delete\(key\);\s*drivingJobs\.delete\(key\);\s*lastJobProgressRefreshMs/, m => `globalThis.__jobKeys?.delete(key);${m}`);
  body = body.replace(/await acceptTurn\(offered\);/, m => `${m}
            if (inspected.operation === "LocalInteractionQuery" && offered.commandIngress?.tag === "backpressure" && wireTurnStatusTag(offered.status) === "idle") {
              const spin = (globalThis.__spin = (globalThis.__spin ?? 0) + 1);
              if (spin === 1) console.log("[TRACE] SPIN begins", Math.round(performance.now()), "jobs", [...(globalThis.__jobKeys ?? [])].join(","));
              if (spin === 200 && "${process.env.MODE ?? "step"}" === "continue") {
                for (let i = 0; i < 8; i += 1) {
                  const t = await submitTurn(actorId, acknowledgements, { activation });
                  await acceptTurn(t);
                  console.log("[TRACE] SPIN pageless turn", i, JSON.stringify({ tag: t.commandIngress?.tag, st: wireTurnStatusTag(t.status), def: t.deferredEvents, eff: t.effects.map(e => e.tag), ui: t.uiPatches.length }));
                  await yieldPluginUiContinuation();
                }
              }
              if (spin === 200 && "${process.env.MODE ?? "step"}" === "step") {
                for (const key of [...(globalThis.__jobKeys ?? [])]) {
                  const started = performance.now();
                  const answer = await shardClient.stepJob(actorId, BigInt(key.split("#")[1]), { fuel: BigInt(DEFAULT_SHARD_BUDGET.fuel), deadlineMs: DEFAULT_SHARD_BUDGET.wallMs }).then(r => "ok " + r.status, e => "threw " + String(e).slice(0, 200));
                  console.log("[TRACE] SPIN stepped job", key, answer, Math.round(performance.now() - started), "ms");
                }
              }
              if (spin % 2000 === 0 || (spin > 200 && spin < 204)) console.log("[TRACE] SPIN count", spin, "status", wireTurnStatusTag(offered.status));
            } else if (globalThis.__spin) { console.log("[TRACE] SPIN ended after", globalThis.__spin, "op", inspected.operation, "answer", offered.commandIngress?.tag, wireTurnStatusTag(offered.status), offered.effects.map(e => e.tag).join(",")); globalThis.__spin = 0; }`);
  patched += body === before ? 0 : 1;
  await route.fulfill({ response, body });
});
page.on("console", message => { const t = message.text(); if ((t.startsWith("[TRACE]") && !/offered|cont#|frames-out|job-step|job-terminal.*running|lane-/.test(t)) || t.startsWith("[TRACE] SPIN") || message.type() === "error") console.log(Math.round(performance.now() - t0), message.text().slice(0, 600)); });
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
