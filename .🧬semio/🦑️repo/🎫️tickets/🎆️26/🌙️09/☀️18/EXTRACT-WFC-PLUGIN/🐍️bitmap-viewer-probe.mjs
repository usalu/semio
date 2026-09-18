/** 👁️ Bitmap viewer probe: boots the editor playground, switches the navbar role to Viewer and checks
 * BOTH viewer window hosts render a canvas without faults.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=playground-bitmap/viewer bun 🐍️bitmap-viewer-probe.mjs */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6041/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-bitmap/viewer");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length }; }),
    viewerPressed: document.getElementById("playground.navbar.roles.viewer")?.getAttribute("aria-pressed") ?? null,
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 400),
  };
});
await page.goto(url, { waitUntil: "domcontentloaded" });
let editor = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); editor = await state(); if (editor.ready && editor.hosts.length >= 2 && i > 10) break; }
const from = lines.length;
const clicked = await page.locator('[id="playground.navbar.roles.viewer"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 140));
let viewer = null;
for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); viewer = await state(); if (viewer.hosts.length >= 2 && viewer.hosts.every((h) => h.canvases > 0) && i > 5) break; }
const faults = lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|pageerror|unreachable|Fault \{|refused/.test(l)).map((l) => l.slice(0, 400));
await page.screenshot({ path: join(outDir, "viewer.png") });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify({ editor, clicked, viewer, faults }, null, 2));
console.log("[DEBUG] VIEWER", JSON.stringify({ clicked, viewer, faults: faults.slice(0, 4) }).slice(0, 1500));
await browser.close();
