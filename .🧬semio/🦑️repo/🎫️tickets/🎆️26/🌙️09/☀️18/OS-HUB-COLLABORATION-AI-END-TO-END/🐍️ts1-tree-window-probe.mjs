/** 🪟️ Proves the Interpreter's tree-window observer on a LIVE `s` shell: every windowed container the
 * shell renders must carry a closed row-extent token and a finite geometry, which is exactly what the
 * host prices rows with (`treeWindowRowExtentPx`). Before TS1's fix the observer never read the token,
 * so every container's pitch was `NaN`. One browser context, closed on exit. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const url = process.argv[2] ?? "http://127.0.0.1:6199/";
const out = fileURLToPath(new URL("./🗑️generated/ts1-tree-window-probe.json", import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const errors = [];
page.on("console", (message) => { if (message.type() === "error") errors.push(message.text().slice(0, 300)); });
page.on("pageerror", (error) => errors.push(`pageerror: ${String(error).slice(0, 300)}`));
try {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
  // 🌲️ A windowed container only exists once a plugin has published a panel body; wait for the
  // shell chrome first, then give the tree windows their own bounded wait and report what was there.
  await page.waitForSelector('[role="tree"], [data-tree-window-key], main', { timeout: 180_000 });
  await page.waitForSelector("[data-tree-window-key]", { timeout: 120_000 }).catch(() => undefined);
  const report = await page.evaluate(() => {
    const containers = [...document.querySelectorAll("[data-tree-window-key]")].map((element) => ({
      key: element.getAttribute("data-tree-window-path") || element.getAttribute("data-tree-window-key"),
      rowExtent: element.getAttribute("data-tree-window-row-extent"),
      total: Number(element.getAttribute("data-tree-window-total")),
      offset: Number(element.getAttribute("data-tree-window-offset")),
      length: Number(element.getAttribute("data-tree-window-length")),
      height: element.getBoundingClientRect().height,
      rows: element.querySelectorAll("[data-tree-window-row]").length,
      spacers: element.querySelectorAll('[data-slot="tree-window-spacer"]').length,
    }));
    return { title: document.title, containers, treeRows: document.querySelectorAll('[role="treeitem"]').length };
  });
  report.consoleErrors = errors.slice(0, 12);
  report.windowedContainers = report.containers.length;
  report.withRowExtent = report.containers.filter((row) => row.rowExtent !== null).length;
  report.finiteGeometry = report.containers.every((row) => Number.isFinite(row.height) && Number.isFinite(row.total));
  writeFileSync(out, JSON.stringify(report, null, 1));
  console.log(JSON.stringify({ windowedContainers: report.windowedContainers, withRowExtent: report.withRowExtent, finiteGeometry: report.finiteGeometry, treeRows: report.treeRows, consoleErrors: errors.length }));
} finally {
  await page.close();
  await browser.close();
}
