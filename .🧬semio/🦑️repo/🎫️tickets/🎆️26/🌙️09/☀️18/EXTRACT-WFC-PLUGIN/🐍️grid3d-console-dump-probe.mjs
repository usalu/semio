/** 🩺️ grid3d boot probe: loads the wfc grid3d react dev playground headless, captures console + page
 * errors + failed requests, the shell readiness beacon, every rendered World3d host with its mesh AND
 * INSTANCE counts, its status line and whether its canvas actually painted anything, plus a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6044/?plugin=wfc SEMIO_PROBE_OUT=playground-grid3d/boot bun 🐍️grid3d-console-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6044/?plugin=wfc";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-grid3d/boot");
mkdirSync(outDir, { recursive: true });
const lines = [];
const failures = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 40000 : 3000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
page.on("requestfailed", (req) => failures.push(`${Date.now() - t0} requestfailed ${req.url()} ${req.failure()?.errorText ?? ""}`));
page.on("response", (res) => { if (res.status() >= 400) failures.push(`${Date.now() - t0} http${res.status()} ${res.url()}`); });

export const snapshotScript = () => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const count = (attr, el) => { const v = parse(el.getAttribute(attr)); return Array.isArray(v) ? v.length : 0; };
  const painted = (el) => {
    const canvas = el.querySelector("canvas");
    if (!canvas) return { canvas: false, painted: false, w: 0, h: 0 };
    const w = canvas.width, h = canvas.height;
    try {
      const probe = document.createElement("canvas");
      probe.width = Math.min(w, 320); probe.height = Math.min(h, 320);
      const ctx = probe.getContext("2d");
      ctx.drawImage(canvas, 0, 0, probe.width, probe.height);
      const data = ctx.getImageData(0, 0, probe.width, probe.height).data;
      const seen = new Set();
      for (let i = 0; i < data.length; i += 4) seen.add(`${data[i]},${data[i + 1]},${data[i + 2]},${data[i + 3]}`);
      return { canvas: true, painted: seen.size > 1, distinct: seen.size, w, h };
    } catch (err) { return { canvas: true, painted: null, w, h, error: String(err).slice(0, 200) }; }
  };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json"));
    const rect = el.getBoundingClientRect();
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      w: Math.round(rect.width), h: Math.round(rect.height),
      meshes: count("data-meshes-json", el),
      instances: count("data-instances-json", el),
      instancesDelta: el.hasAttribute("data-instances-delta-json"),
      status: st,
      ...painted(el),
    };
  });
  const html = document.documentElement;
  return { ready: html.getAttribute("data-semio-os-ready"), error: html.getAttribute("data-semio-os-error"), title: document.title, hosts, combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null, bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 1200) };
};
const snap = () => page.evaluate(snapshotScript);

await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < seconds; i++) {
  await page.waitForTimeout(1000);
  last = await snap();
  if (last.ready && last.hosts.length && last.hosts.every((h) => h.instances > 0) && i > 12) break;
}
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "network-failures.txt"), failures.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify(last, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "failures", failures.length, JSON.stringify(last.hosts));
await browser.close();
