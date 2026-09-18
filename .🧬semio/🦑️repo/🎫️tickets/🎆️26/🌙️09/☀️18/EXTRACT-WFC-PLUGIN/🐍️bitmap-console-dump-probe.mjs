/** 🩺️ Bitmap boot probe: loads the wfc bitmap react dev playground headless, captures console + page
 * errors, the shell readiness beacon, every rendered window host, the chrome ids the interact probe
 * addresses, and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6041/?plugin=wfc SEMIO_PROBE_OUT=playground-bitmap/boot bun 🐍️bitmap-console-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6041/?plugin=wfc";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-bitmap/boot");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 40000 : 3000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return s?.slice(0, 300); } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json"));
    const rect = el.getBoundingClientRect();
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height),
      children: el.childElementCount, canvases: el.querySelectorAll("canvas").length,
      textLength: (el.innerText ?? "").length, phase: st?.phase, fault: st?.fault?.code ?? null,
    };
  });
  const html = document.documentElement;
  return {
    ready: html.getAttribute("data-semio-os-ready"), error: html.getAttribute("data-semio-os-error"), title: document.title, hosts,
    combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    engagementToggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    windowIds: [...document.querySelectorAll('[id^="framework.window."]')].map((el) => el.id).slice(0, 40),
    bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 1500),
  };
});
await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); last = await snap(); if (last.ready && last.hosts.length && i > 20) break; }
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
const faults = lines.filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey/.test(l)).map((l) => l.slice(0, 500));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify({ ...last, faults }, null, 2));
console.log("[DEBUG] DONE lines", lines.length, "faults", faults.length, JSON.stringify(last).slice(0, 2000));
await browser.close();
