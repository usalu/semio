import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🖼️ Boots the WGPU prototype in a real-GPU Chromium and records console lines, canvas geometry and screenshots into 🗑️generated/wgpu-paint. */
const out = join(import.meta.dir, "🗑️generated/wgpu-paint"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const tag = process.env.PAINT_TAG ?? "run";
const browser = await chromium.launch({ headless: process.env.HEADED !== "1", channel: process.env.CHANNEL, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", "--enable-features=Vulkan,WebGPU"] });
const page = await (await browser.newContext({ viewport: { width: 1280, height: 800 } })).newPage();
const lines = [];
page.on("console", m => lines.push(`${m.type()} ${m.text()}${m.type() === "error" || m.type() === "warning" ? " @ " + m.location().url : ""}`));
page.on("pageerror", e => lines.push(`pageerror ${e.message}`));
page.on("requestfailed", r => lines.push(`requestfailed ${r.url()} ${r.failure()?.errorText}`));
page.on("response", r => { if ((r.headers()["content-type"] ?? "").includes("text/html")) lines.push(`html-response ${r.status()} ${r.url()}`); if (r.status() >= 400) lines.push(`http ${r.status()} ${r.url()}`); });
if (process.env.FAULT_TRACE === "1") await page.route(/frame-worker\.js/, async route => { const r = await route.fetch(); let body = await r.text(); body = body.replace("function beginClose() {", "function beginClose() {\n  console.log('[DEBUG] beginClose ' + new Error().stack);").replace('setPendingFault({ code: "original-frame-failure"', "console.log('[DEBUG] failure bytes ' + new TextDecoder('latin1').decode(originalFrameOutput.bytes).replace(/[^\\x20-\\x7e]+/g, '|')); setPendingFault({ code: \"original-frame-failure\"").replace("  tick(timestamp, sequence, presentation) {", "  tick(timestamp, sequence, presentation) {\n    globalThis.__t = (globalThis.__t ?? 0) + 1; if (globalThis.__t % 400 === 1) console.log('[DEBUG] tick#' + globalThis.__t + ' fuel=' + this.source.fuelRemaining + ' grant=' + JSON.stringify(this.source.retainedRemaining));").replace("function requestFault(code, detail) {", "function requestFault(code, detail) {\n  console.log('[DEBUG] requestFault ' + code + ' ' + detail);"); await route.fulfill({ response: r, body }); });
await page.goto(url, { waitUntil: "load" });
const probe = () => page.evaluate(async () => ({ gpu: !!navigator.gpu, adapter: !!(await navigator.gpu?.requestAdapter?.()), canvases: [...document.querySelectorAll("canvas")].map(c => ({ w: c.width, h: c.height, cw: c.clientWidth, ch: c.clientHeight })) }));
const ms = Number(process.env.SMOKE_MS ?? 45000);
await page.waitForTimeout(ms);
const info = await probe();
await page.screenshot({ path: join(out, `${tag}.png`) });
writeFileSync(join(out, `${tag}-console.txt`), lines.join("\n") + "\n" + JSON.stringify(info));
console.log(JSON.stringify(info), lines.length, "lines");
console.log(lines.filter(l => /^(error|warning|pageerror|http|requestfailed)/.test(l)).slice(0, 30).join("\n"));
await browser.close();
