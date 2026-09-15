/** 🎯 Boot-selection recon — who dispatches the `interactionSelect` that lands milliseconds BEFORE the
 * node-graph host mounts. Installs a console hook before any app code runs and prints the JS stack of
 * every `performInvocation … interactionSelect` and every `node-graph host mount`, so the dispatcher is
 * named instead of guessed.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6023/?plugin=generation3d bun 🐍️boot-selection-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "boot-selection");
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 60);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  const marked = ["interactionSelect", "node-graph host mount", "clearSelection", "flow interaction publish", "selectionDomains"];
  for (const level of ["log", "warn", "error", "debug", "info"]) {
    const original = console[level].bind(console);
    console[level] = (...args) => {
      const text = args.map((a) => (typeof a === "string" ? a : (() => { try { return JSON.stringify(a); } catch { return String(a); } })())).join(" ");
      if (marked.some((needle) => text.includes(needle))) original(`[STACK] ${text.slice(0, 300)} :: ${String(new Error("trace").stack).replace(/\n/g, " | ").slice(0, 1600)}`);
      else original(...args);
    };
  }
});
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 800)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);

const selection = await page.evaluate(() => {
  const host = document.querySelector('[data-surface-id="window:procedural-main"]');
  const probe = window.__semioFlowGraphProbe?.["window:procedural-main"];
  let nodeIds = null;
  try { nodeIds = probe?.nodeIds?.() ?? null; } catch {}
  return { hostPresent: !!host, nodeIds, selectionAttributes: [...document.querySelectorAll("[data-selected-ids-json]")].map((e) => e.getAttribute("data-selected-ids-json")).slice(0, 6) };
});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "stacks.txt"), lines.filter((l) => l.includes("[STACK]")).join("\n\n"));
writeFileSync(join(outDir, "selection.json"), JSON.stringify(selection, null, 2));
console.log(`[DEBUG] recon lines=${lines.length} stacks=${lines.filter((l) => l.includes("[STACK]")).length} selection=${JSON.stringify(selection).slice(0, 400)}`);
await browser.close();
