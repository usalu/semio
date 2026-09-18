/** 👁️ grid3d viewer probe: boots the editor playground, switches the navbar role to Viewer and checks the
 * viewer's scene window host renders without faults. Usage: cd <ticket> && bun 🐍️wfc-viewer-probe.mjs */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-grid3d/viewer");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); let meshes = 0; try { meshes = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]").length; } catch {} let instances = 0; try { instances = JSON.parse(el.getAttribute("data-instances-json") ?? "[]").length; } catch {} return { id: el.getAttribute("data-surface-id"), status: st, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, meshes, instances }; }),
    viewerPressed: document.getElementById("playground.navbar.roles.viewer")?.getAttribute("aria-pressed") ?? null,
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 300),
  };
});
await page.goto(process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6044/?plugin=wfc", { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 150; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 8) break; }
const from = lines.length;
const clicked = await page.locator('[id="playground.navbar.roles.viewer"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
let v = null;
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); v = await state(); if (v.hosts.some((h) => /view/.test(h.id)) && v.hosts.every((h) => h.canvases > 0 || !/view/.test(h.id)) && i > 5) break; }
const faults = lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|pageerror|unreachable|Fault \{/.test(l)).map((l) => l.slice(0, 300));
await page.screenshot({ path: join(outDir, "viewer.png") });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify({ editor: s, clicked, viewer: v, faults }, null, 2));
console.log("[DEBUG] VIEWER", JSON.stringify({ clicked, viewer: v, faults: faults.slice(0, 3) }).slice(0, 1200));
await browser.close();
