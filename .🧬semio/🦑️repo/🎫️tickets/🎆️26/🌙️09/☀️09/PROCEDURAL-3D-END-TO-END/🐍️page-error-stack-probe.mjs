/** 🩺️ Captures the FULL stack of every uncaught page error while the shell mounts and unmounts its
 * node-graph host — the `Cannot read properties of null (reading 'addEventListener')` the s4 journey
 * logged twice carried no stack, so its owner was unknown.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-u64/page-errors bun 🐍️page-error-stack-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "page-errors");
const boot = Number(process.env.SEMIO_PROBE_BOOT ?? 45);
mkdirSync(outDir, { recursive: true });
const lines = [];
const errors = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 800)}`));
page.on("pageerror", (e) => {
  const row = { t: Date.now() - t0, message: String(e?.message ?? e).slice(0, 500), stack: String(e?.stack ?? "").slice(0, 4000) };
  errors.push(row);
  lines.push(`${row.t} pageerror ${row.message}`);
  console.log(`[DEBUG] pageerror ${row.t}ms ${row.message}\n${row.stack}`);
});
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(boot * 1000);
const step = async (label, action) => {
  const before = errors.length;
  await action();
  await page.waitForTimeout(8000);
  console.log(`[DEBUG] step ${label} newPageErrors=${errors.length - before}`);
};
await step("generate-mode", () => page.keyboard.press("Meta+Alt+ArrowRight"));
await step("back-to-edit", () => page.keyboard.press("Meta+Alt+ArrowLeft"));
await step("viewer-role", () => page.keyboard.press("Meta+Alt+V"));
await step("editor-role", () => page.keyboard.press("Meta+Alt+E"));
await step("generate-mode-2", () => page.keyboard.press("Meta+Alt+ArrowRight"));
await step("back-to-edit-2", () => page.keyboard.press("Meta+Alt+ArrowLeft"));
writeFileSync(join(outDir, "errors.json"), JSON.stringify(errors, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await page.screenshot({ path: join(outDir, "final.png") });
console.log("DONE pageErrors", errors.length, "lines", lines.length);
await browser.close();
