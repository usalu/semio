/** 🩺️ Grid2d boot probe: loads the wfc react dev playground for variant `grid2d`, captures console +
 * page errors, the shell readiness beacon, every rendered window host AND a pixel fingerprint of each
 * canvas (a `Canvas2d` pane carries no DOM text, so "did it paint" is only answerable from its bitmap).
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6042/?plugin=wfc SEMIO_PROBE_OUT=playground-grid2d/grid2d-boot bun 🐍️grid2d-console-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6042/?plugin=wfc";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-grid2d/grid2d-boot");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 40000 : 3000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
const faultLines = () => lines.filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|Render error/.test(l)).map((l) => l.slice(0, 500));
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return s?.slice(0, 300); } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json"));
    const rect = el.getBoundingClientRect();
    const canvas = el.querySelector("canvas");
    // 🎨️ A canvas pane's only honest "it painted" signal: how many distinct colours its bitmap holds.
    let ink = null;
    if (canvas) {
      try {
        const probe = document.createElement("canvas");
        probe.width = 64;
        probe.height = 64;
        const ctx = probe.getContext("2d");
        ctx.drawImage(canvas, 0, 0, 64, 64);
        const data = ctx.getImageData(0, 0, 64, 64).data;
        const colors = new Set();
        for (let i = 0; i < data.length; i += 4) colors.add(`${data[i]},${data[i + 1]},${data[i + 2]},${data[i + 3]}`);
        ink = { colors: colors.size, width: canvas.width, height: canvas.height };
      } catch (error) { ink = { error: String(error).slice(0, 120) }; }
    }
    return { surfaceId: el.getAttribute("data-surface-id"), w: Math.round(rect.width), h: Math.round(rect.height), canvases: el.querySelectorAll("canvas").length, textLength: (el.innerText ?? "").length, phase: st?.phase, fault: st?.fault?.code ?? null, ink };
  });
  const html = document.documentElement;
  return { ready: html.getAttribute("data-semio-os-ready"), error: html.getAttribute("data-semio-os-error"), title: document.title, hosts, combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null, bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 1200) };
});
await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); last = await snap(); if (last.ready && last.hosts.length >= 2 && i > 20) break; }
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify({ ...last, faults: faultLines() }, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "faults", faultLines().length, JSON.stringify(last).slice(0, 1800));
await browser.close();
