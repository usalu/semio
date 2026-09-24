/** 🔎️ WG7 — does the hub pill still open the hub workspace after the footer Sync panel was opened first? */
import { chromium } from "playwright";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6551/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const lines = [];
page.on("console", (message) => lines.push(message.text()));
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
await page.waitForTimeout(8000);
const keys = async () => JSON.parse((await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility())) || "{}").windows?.flatMap((w) => w.nodes.map((n) => n.key)) ?? [];
const click = (key) => page.evaluate((k) => { const e = document.querySelector(`#semio-wgpu-accessibility [data-node-key="${k}"]`); if (!e) return "absent"; e.focus(); e.dispatchEvent(new MouseEvent("click", { bubbles: true })); return "ok"; }, key);
const sync = await click("s-sync-status");
await page.waitForTimeout(3000);
const syncOpen = (await keys()).some((k) => k.endsWith("framework.sync.remote"));
const hub = await click("framework.hub.signIn");
const started = Date.now();
let hubOpen = false;
while (!hubOpen && Date.now() - started < 30_000) { await page.waitForTimeout(1000); hubOpen = (await keys()).some((k) => k.endsWith("framework.hub.address")); }
console.log(JSON.stringify({ sync, syncOpen, hub, hubOpen, ms: Date.now() - started, faults: lines.filter((l) => /fault|failed|panicked/iu.test(l)).slice(0, 5), hubLines: lines.filter((l) => /hub\.signIn|framework\.hub|window_generation/u.test(l)).slice(-12) }, null, 1));
await browser.close();
