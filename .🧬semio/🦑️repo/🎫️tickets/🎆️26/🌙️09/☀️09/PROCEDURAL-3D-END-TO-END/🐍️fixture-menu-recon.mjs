// 🧭️ Lists the generation3d playground's example fixtures and how long each one's `previewEval` tool run lasts,
// so a mid-run Abort/Finalize probe can pick a run long enough to press a button inside (ticket 26/09/09, lane
// concurrent-patch-intake). Run from the ticket folder: `bun 🐍️fixture-menu-recon.mjs`.
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6024/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "fixture-menu");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
const lines = [];
page.on("console", (msg) => lines.push(`+${Date.now() - t0}ms ${msg.type()} ${msg.text().slice(0, 400)}`));
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
const deadline = Date.now() + 240000;
while (Date.now() < deadline && !(await converged())) await page.waitForTimeout(1000);
await page.evaluate(() => document.getElementById("playground.navbar.fixture")?.click());
await page.waitForTimeout(1200);
const items = await page.evaluate(() =>
  [...document.querySelectorAll('[role="menuitem"],[role="option"],[id^="playground.navbar.fixture"]')].map((node) => ({ id: node.id, label: (node.textContent ?? "").replace(/\s+/g, " ").trim() })),
);
writeFileSync(join(outDir, "fixtures.json"), JSON.stringify(items, null, 2));
console.log(JSON.stringify(items, null, 2).slice(0, 4000));
await page.screenshot({ path: join(outDir, "menu.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
