/** 👁️ Grid2d viewer probe: boots the editor playground, switches the navbar role to Viewer, and checks
 * the viewer's own `Canvas2d` pane renders a PAINTED bitmap (not just a mounted canvas) with no fault
 * lines — including the hover the shell sends at it, which used to be `refused: undeclared-action`.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6042/?plugin=wfc SEMIO_PROBE_OUT=playground-grid2d/grid2d-viewer bun 🐍️grid2d-viewer-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6042/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-grid2d/grid2d-viewer");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const state = () =>
  page.evaluate(() => {
    const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
    const fingerprint = (el) => {
      const canvas = el.querySelector("canvas");
      if (!canvas) return null;
      try {
        const probe = document.createElement("canvas");
        probe.width = 64;
        probe.height = 64;
        const ctx = probe.getContext("2d");
        ctx.drawImage(canvas, 0, 0, 64, 64);
        const data = ctx.getImageData(0, 0, 64, 64).data;
        const colors = new Set();
        for (let i = 0; i < data.length; i += 4) colors.add(`${data[i]},${data[i + 1]},${data[i + 2]}`);
        return { colors: colors.size };
      } catch (error) {
        return { error: String(error).slice(0, 120) };
      }
    };
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, ink: fingerprint(el) }; }),
      viewerPressed: document.getElementById("playground.navbar.roles.viewer")?.getAttribute("aria-pressed") ?? null,
      bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 400),
    };
  });
await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 12) break; }
const from = lines.length;
const clicked = await page.locator('[id="playground.navbar.roles.viewer"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
let v = null;
for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); v = await state(); if (v.hosts.some((h) => /view/.test(h.id ?? "")) && i > 5) break; }
// 🖱️ Hover the viewer pane: the read-only surface still receives the host's pointer lane.
const box = await page.locator('[data-surface-id^="window:wfc-grid2d-view"]').first().boundingBox().catch(() => null);
if (box) { await page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.5); await page.waitForTimeout(400); await page.mouse.move(box.x + box.width * 0.4, box.y + box.height * 0.6); }
await page.waitForTimeout(2500);
v = await state();
const faults = lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|pageerror|unreachable|Fault \{|refused|Render error|DuplicateSiblingKey/.test(l)).map((l) => l.slice(0, 400));
await page.screenshot({ path: join(outDir, "viewer.png") });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify({ url, editor: s, clicked, viewer: v, faults }, null, 2));
console.log("[DEBUG] VIEWER", JSON.stringify({ clicked, viewer: v, faults: faults.slice(0, 4) }).slice(0, 1600));
await browser.close();
