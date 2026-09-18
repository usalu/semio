/** 🩺️ wfc3d boot probe: loads the wfc3d react dev playground headless, captures console + page errors,
 * the shell's readiness beacon, every rendered window host, the World3d preview's OWN instance/status
 * lanes (a mesh catalogue is not a solve — only `instances_json` proves the solve reached the browser)
 * and the `wfc-graph` canvas' DOM shape, plus a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6045/?plugin=wfc SEMIO_PROBE_OUT=playground-wfc3d/boot bun 🐍️wfc3d-console-dump-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6045/?plugin=wfc";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-wfc3d/boot");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 40000 : 3000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const count = (s) => { const v = parse(s); return Array.isArray(v) ? v.length : 0; };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const rect = el.getBoundingClientRect();
    const status = parse(el.getAttribute("data-status-json"));
    const instances = parse(el.getAttribute("data-instances-json"));
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      w: Math.round(rect.width),
      h: Math.round(rect.height),
      canvases: el.querySelectorAll("canvas").length,
      meshes: count(el.getAttribute("data-meshes-json")),
      instances: Array.isArray(instances) ? instances.length : 0,
      instanceHeads: Array.isArray(instances) ? instances.slice(0, 4).map((i) => ({ id: i.id, meshId: i.meshId, position: i.position, scale: i.scale })) : [],
      camera: parse(el.getAttribute("data-camera-json")),
      status: status?.message ?? status?.phase ?? null,
      fault: status?.fault?.code ?? null,
      reactFlowNodes: el.querySelectorAll(".react-flow__node").length,
      reactFlowEdges: el.querySelectorAll(".react-flow__edge").length,
      canvasSizes: [...el.querySelectorAll("canvas")].map((c) => `${c.width}x${c.height}`),
      textLength: (el.innerText ?? "").length,
    };
  });
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    title: document.title,
    hosts,
    combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id).slice(0, 80),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 1200),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let last = null;
for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); last = await snap(); if (last.ready && last.hosts.length && i > 20) break; }
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
const faults = lines.filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|interactive-ceiling|quarantine/.test(l)).map((l) => l.slice(0, 500));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "state.json"), JSON.stringify({ url, state: last, faults }, null, 2));
console.log("[DEBUG] BOOT faults", faults.length, JSON.stringify(faults.slice(0, 4)).slice(0, 1200));
console.log("[DEBUG] BOOT state", JSON.stringify(last).slice(0, 2500));
await browser.close();
