/** 🩺️ WFC 2D boot probe: loads the wfc2d react dev playground headless, captures console + page errors,
 * the shell readiness beacon, both window hosts (`wfc-graph`, `wfc-2d-preview`), the navbar example
 * picker's options and a screenshot. A zero-length `faults` array is the pass condition.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6043/?plugin=wfc SEMIO_PROBE_OUT=playground-wfc2d/boot bun 🐍️wfc2d-console-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6043/?plugin=wfc";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-wfc2d/boot");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 40000 : 3000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
const faultLines = () => lines.filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|undeclared-action/.test(l)).map((l) => l.slice(0, 500));
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return s?.slice(0, 300); } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json"));
    const rect = el.getBoundingClientRect();
    return { surfaceId: el.getAttribute("data-surface-id"), w: Math.round(rect.width), h: Math.round(rect.height), canvases: el.querySelectorAll("canvas").length, nodes: el.querySelectorAll(".react-flow__node").length, edges: el.querySelectorAll(".react-flow__edge").length, phase: st?.phase, fault: st?.fault?.code ?? null };
  });
  const html = document.documentElement;
  return {
    ready: html.getAttribute("data-semio-os-ready"),
    error: html.getAttribute("data-semio-os-error"),
    title: document.title,
    hosts,
    combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 1200),
  };
});
await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); last = await snap(); if (last.ready && last.hosts.length && i > 20) break; }
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify({ ...last, faults: faultLines() }, null, 2));
console.log("[DEBUG] BOOT", JSON.stringify({ ready: last?.ready, hosts: last?.hosts, faults: faultLines() }).slice(0, 2000));
await browser.close();
