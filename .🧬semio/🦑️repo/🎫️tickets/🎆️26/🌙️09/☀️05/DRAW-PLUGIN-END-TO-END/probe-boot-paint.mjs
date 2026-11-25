/** 🖼️ Boots Draw and reports when the canvas first shows the example, with the job events and the guest's own debug volume on the way. */
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = performance.now();
const at = () => Math.round((performance.now() - t0) / 100) / 10;
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.route(url => decodeURIComponent(url.href).includes("handle-turn-drive"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  body = body.replace(/await shardClient\.startJob\(actorId, job, kind, input\);\s*runningJobs\.add\(key\);\s*\}\);/, m => `console.log("[TRACE] job-start", String(job), kind);${m}`);
  body = body.replace(/if \(outcome\.status !== "running"\) \{\s*if \(live\(\)\) \{/, m => `console.log("[TRACE] job-terminal", String(job), outcome.status, step, JSON.stringify(outcome, (k, v) => typeof v === "bigint" ? String(v) : v instanceof Uint8Array ? new TextDecoder().decode(v.slice(0, 120)) : v).slice(0, 300));${m}`);
  await route.fulfill({ response, body });
});
let debug = 0;
page.on("console", message => { const text = message.text(); if (text.startsWith("[DEBUG]")) debug += 1; if (text.startsWith("[TRACE]") || message.type() === "error" && !text.startsWith("[DEBUG]")) console.log(at(), text.slice(0, 420)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 120000 });
console.log(at(), "ready, debug lines", debug);
let last = "";
for (let index = 0; index < 30; index += 1) {
  const hash = createHash("sha256").update(await page.screenshot({ clip: { x: 320, y: 100, width: 900, height: 700 } })).digest("hex").slice(0, 8);
  if (hash !== last) { console.log(at(), "canvas changed", hash, "debug lines", debug); last = hash; }
  await page.waitForTimeout(1000);
}
await page.screenshot({ path: resolve(process.argv[2]) });
await browser.close();
