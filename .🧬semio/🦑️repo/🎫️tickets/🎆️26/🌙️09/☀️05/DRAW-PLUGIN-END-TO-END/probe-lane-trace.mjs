/** ⏱️ Measures how long a panel-added layer takes to reach the layer tree and the canvas. */
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = performance.now();
let patched = 0;
await page.route(url => decodeURIComponent(url.href).includes("handle-turn-drive"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  const before = body;
  body = body.replace(/await shardClient\.startJob\(actorId, job, kind, input\);\s*runningJobs\.add\(key\);\s*\}\);/, match => `console.log("[TRACE] start", String(job), kind);${match}console.log("[TRACE] started", String(job));`);
  body = body.replace(/if \(outcome\.status !== "running"\) \{/, match => `${match}console.log("[TRACE] terminal", String(job), outcome.status, step, outcome.status === "failed" ? new TextDecoder().decode(outcome.value).slice(0, 60) : "");`);
  body = body.replace(/const cancelSpawnedJob = \(instanceId, actorId, job\) => \{/, match => `${match}console.log("[TRACE] cancel", String(job), runningJobs.has(actorId + "#" + job));`);
  body = body.replace(/ledgerEntry\.step\(step, progressed\.value\);/, match => `if (step % 32 === 0 || step < 8) console.log("[TRACE] step", String(job), step);${match}`);
  body = body.replace(/last = await shardClient\.stepJob\(actorId, job, jobStepBudget\);/, 'globalThis.__at = ["step", String(job), step, performance.now()]; last = await shardClient.stepJob(actorId, job, jobStepBudget); globalThis.__at = ["stepped", String(job), step, performance.now(), last.status];');
  body = body.replace(/await yieldPluginUiContinuation\(\);(\s*\}\s*\} catch \(error\) \{)/, 'globalThis.__at = ["yield", String(job), step, performance.now()]; await yieldPluginUiContinuation(); globalThis.__at = ["yielded", String(job), step, performance.now(), live()];$1');
  body = body.replace(/turnOutcomes\.push\(\{ instanceId, error \}\);\s*\} finally \{/, match => `console.log("[TRACE] error", String(job), String(error));${match}`);
  body = body.replace(/ledgerEntry\.end\(\);/, match => `console.log("[TRACE] end", String(job), step_(), live());${match}`.replace("step_()", '""'));
  body = body.replace(/const inspected = inspectEncodedAppCommand\(events\);/, match => `${match} console.log("[TRACE] command", JSON.stringify(inspected)); globalThis.__command = { inspected, turns: [] };`);
  body = body.replace(/terminal = foldIngress\(\);/g, match => `${match} globalThis.__command?.turns.push(Math.round(performance.now()) + ":" + terminal + ":" + (typeof lastTurnStatus === "undefined" ? "" : wireTurnStatusTag(results.at(-1)?.status)));`);
  body = body.replace(/settlePluginTurn\(/g, '((globalThis.__command?.turns.push("settle@" + Math.round(performance.now()))), settlePluginTurn)(');
  body = body.replace(/await submitTurn\(/g, 'await submitTurn(') ;
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
      run = async () => { const entry = globalThis.__lane.get(id); entry.state = "running"; entry.started = performance.now(); try { return await inner(); } finally { globalThis.__lane.delete(id); } }; }`);
  console.log("LANE PATCHED", body !== before);
  await route.fulfill({ response, body });
});
page.on("console", message => { if (message.text().startsWith("[TRACE]") || message.type() === "error") console.log(Math.round(performance.now() - t0), message.text().slice(0, 260)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const hash = async () => createHash("sha256").update(await page.screenshot({ clip: { x: 310, y: 85, width: 980, height: 820 } })).digest("hex").slice(0, 10);
const rows = () => page.locator('[role="treeitem"][id*="drawing-play-layers.shape."]').count();
for (const name of ["Add Ellipse", "Add Rectangle", "Add Polygon"]) {
  const before = await hash(), count = await rows(), start = performance.now();
  console.log(Math.round(performance.now() - t0), "CLICK", name);
  await page.getByText(name, { exact: true }).first().click();
  let row = null, canvas = null;
  while (performance.now() - start < 35000 && (row === null || canvas === null)) {
    if (row === null && await rows() > count) row = Math.round(performance.now() - start);
    if (canvas === null && await hash() !== before) canvas = Math.round(performance.now() - start);
    await page.waitForTimeout(100);
  }
  console.log(Math.round(performance.now() - t0), "PATCHED", patched, name, "row ms", row, "canvas ms", canvas);
  await page.waitForTimeout(6000);
  console.log(Math.round(performance.now() - t0), "AT", await page.evaluate(() => JSON.stringify([globalThis.__at, performance.now(), globalThis.__command?.inspected, (globalThis.__command?.turns ?? []).slice(-6), globalThis.__command?.turns.length, [...(globalThis.__lane ?? new Map()).values()].slice(0, 8)], null, 1)));
}
await browser.close();
