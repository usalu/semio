/** 📸️ W14e: one React-twin screenshot + console/error census for a playground.
 * Usage: SEMIO_PROBE_URL=http://127.0.0.1:6080/?plugin=note SEMIO_PROBE_OUT=w14e-note-react bun 🐍️w14e-react-shot.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6080/?plugin=note";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 40);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w14e-react");
mkdirSync(outDir, { recursive: true });
const lines = [];
const failed = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
const t0 = Date.now();
page.on("console", (m) => lines.push(`${Date.now() - t0} page ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} page pageerror ${String(e).slice(0, 4000)}`));
page.on("worker", (w) => { lines.push(`worker created ${w.url()}`); w.on("console", (m) => lines.push(`${Date.now() - t0} worker ${m.type()} ${m.text().slice(0, 2000)}`)); });
page.on("requestfailed", (r) => failed.push(`FAILED ${r.method()} ${r.url()} ${r.failure()?.errorText ?? ""}`));
page.on("response", (r) => { if (r.status() >= 400) failed.push(`${r.status()} ${r.request().method()} ${r.url()}`); });
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 300000 });
await page.waitForTimeout(seconds * 1000);
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
const state = await page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  title: document.title,
  canvases: [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height, cssW: c.clientWidth, cssH: c.clientHeight })),
  text: document.body.innerText.slice(0, 3000),
}));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "requests-failed.txt"), failed.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify(state, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "failed", failed.length, JSON.stringify(state).slice(0, 1200));
await browser.close();
