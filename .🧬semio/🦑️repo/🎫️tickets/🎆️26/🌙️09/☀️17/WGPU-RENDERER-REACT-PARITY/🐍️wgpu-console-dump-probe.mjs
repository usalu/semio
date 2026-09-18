/** 🩺️ Wgpu boot probe: loads a wgpu playground headless with WebGPU on, captures page + worker console,
 * page errors, failed requests, the canvas size and a screenshot at several moments.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d SEMIO_PROBE_OUT=wgpu-boot bun 🐍️wgpu-console-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const shots = (process.env.SEMIO_PROBE_SHOTS ?? "10,30,60").split(",").map(Number);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-boot");
mkdirSync(outDir, { recursive: true });
const lines = [];
const failed = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
const seed = process.env.SEMIO_PROBE_LOCALSTORAGE;
if (seed) await page.addInitScript((entries) => { try { for (const [key, value] of Object.entries(entries)) localStorage.setItem(key, value); } catch {} }, JSON.parse(seed));
const t0 = Date.now();
const keep = (text) => !/^\[DEBUG\] (frame build admitted|wgpu-shell engine surfaces|os_host frame gate)/.test(text);
const record = (source, type, text) => { if (keep(text)) lines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, type === "error" ? 40000 : 3000)}`); };
page.on("console", (msg) => record("page", msg.type(), msg.text()));
page.on("pageerror", (err) => record("page", "pageerror", String(err)));
page.on("worker", (worker) => {
  record("page", "info", `worker created ${worker.url()}`);
  worker.on("console", (msg) => record("worker", msg.type(), msg.text()));
  worker.on("close", () => record("worker", "info", `worker closed ${worker.url()}`));
});
page.on("requestfailed", (req) => failed.push(`${Date.now() - t0} FAILED ${req.method()} ${req.url()} ${req.failure()?.errorText ?? ""}`));
page.on("response", (res) => { if (res.status() >= 400) failed.push(`${Date.now() - t0} ${res.status()} ${res.request().method()} ${res.url()}`); });
const snap = () => page.evaluate(() => {
  const html = document.documentElement;
  const canvases = [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height, cssW: c.clientWidth, cssH: c.clientHeight }));
  return { ready: html.getAttribute("data-semio-os-ready"), error: html.getAttribute("data-semio-os-error"), title: document.title, dpr: devicePixelRatio, canvases, text: document.body.innerText.slice(0, 500) };
});
await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 1; i <= seconds; i++) {
  await page.waitForTimeout(1000);
  if (shots.includes(i)) await page.screenshot({ path: join(outDir, `shot-${i}s.png`), type: "png" });
  if (i % 10 === 0) { last = await snap(); lines.push(`${Date.now() - t0} snapshot ${JSON.stringify(last)}`); }
}
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "requests-failed.txt"), failed.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify(last, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "failed", failed.length, JSON.stringify(last).slice(0, 800));
await browser.close();
