// 🪧️ Runtime recon for the Tool runs pill (ticket 26/09/09, lane tool-run-pace-storm): on the live React
// generation3d playground it opens `framework.panel.toolRun`, waits for the preview to converge, and reports
// the run group's status pill, its progress value text, its step rows and the preview surface's OWN delivered
// mesh census (`data-meshes-json`) — so the pill's phase label and its mesh count can be read against the
// geometry the preview actually published. Also counts `toolRunPace` console lines per minute.
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6027/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "pill-recon");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1200)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });

const converged = async () =>
  page.evaluate(() => {
    const el = document.querySelector('[data-surface-id="window:procedural-preview"][data-status-json]');
    if (!el) return false;
    try {
      const status = JSON.parse(el.getAttribute("data-status-json") ?? "{}");
      return status.phase === "idle" && (status.progress?.nodesTotal ?? 0) > 0 && status.progress.nodesDone === status.progress.nodesTotal;
    } catch {
      return false;
    }
  });

const deadline = Date.now() + seconds * 1000;
let convergedAtMs = null;
while (Date.now() < deadline) {
  if (await converged()) {
    convergedAtMs = Date.now() - t0;
    break;
  }
  await page.waitForTimeout(1000);
}

await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
await page.waitForTimeout(12000);

const read = () =>
  page.evaluate(() => {
    const textOf = (el) => (el ? (el.textContent ?? "").replace(/\s+/g, " ").trim() : null);
    const groups = [...document.querySelectorAll("[id]")]
      .filter((el) => /\bframework\.toolRun\.\d+$/.test(el.id))
      .map((el) => {
        const prefix = el.id;
        const bar = el.querySelector('[role="progressbar"],progress');
        return {
          id: prefix,
          text: textOf(el),
          status: textOf(document.getElementById(`${prefix}.status`)) ?? textOf(el.querySelector('[aria-live="polite"],[aria-live="assertive"]')),
          progress: bar ? { text: textOf(bar), aria: bar.getAttribute("aria-valuetext"), now: bar.getAttribute("aria-valuenow"), max: bar.getAttribute("aria-valuemax") } : null,
          steps: [...document.querySelectorAll(`[id^="${prefix}.steps"]`)].map(textOf),
          traceRows: document.querySelectorAll(`[id^="${prefix}.trace."]`).length,
        };
      });
    const preview = document.querySelector('[data-surface-id="window:procedural-preview"][data-status-json]');
    const meshes = (() => {
      try {
        const v = JSON.parse(preview?.getAttribute("data-meshes-json") ?? "[]");
        return Array.isArray(v) ? v : [];
      } catch {
        return [];
      }
    })();
    return {
      groups,
      panelPresent: [...document.querySelectorAll("[id]")].some((el) => /\bframework\.toolRun$/.test(el.id)),
      preview: {
        status: preview?.getAttribute("data-status-json")?.slice(0, 900) ?? null,
        deliveredMeshes: meshes.length,
        roles: meshes.map((mesh) => (mesh && typeof mesh === "object" ? (mesh.role ?? null) : null)),
        handles: meshes.map((mesh) => (mesh && typeof mesh === "object" ? (mesh.handle ?? mesh.id ?? null) : null)),
      },
    };
  });

const report = { url, convergedAtMs, ...(await read()) };
report.paceLines = lines.filter((line) => line.includes("toolRunPace")).length;
report.paceLinesPerMinuteAfterConvergence = convergedAtMs === null ? null : Math.round((lines.filter((line) => line.includes("toolRunPace") && Number(line.split(" ")[0]) > convergedAtMs).length * 60000) / Math.max(1, Date.now() - t0 - convergedAtMs));
report.pageerrors = lines.filter((line) => line.includes(" pageerror ")).length;
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2).slice(0, 4000));
await browser.close();
