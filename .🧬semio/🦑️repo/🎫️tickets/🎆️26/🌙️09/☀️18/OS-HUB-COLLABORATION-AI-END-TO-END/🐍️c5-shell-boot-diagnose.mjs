/** 🩺️ C5 — why the shell does not reach `data-semio-os-ready`, with the FULL stack.
 *
 * `page.on("pageerror")` stringifies to the message alone, which is how a boot crash in this shell
 * stays anonymous. This captures `error.stack` and the failing module's own frames instead.
 *
 * Usage: bun 🐍️c5-shell-boot-diagnose.mjs [shellUrl]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6191";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });
const t0 = Date.now();
const lines = [];

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newContext({ viewport: { width: 1600, height: 1000 } }).then((context) => context.newPage());
page.on("pageerror", (error) => lines.push(`${Date.now() - t0} PAGEERROR ${error.message}\n${error.stack ?? "<no stack>"}`));
page.on("console", (message) => {
  if (message.type() === "error" || /error|failed|undefined/i.test(message.text())) lines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 400)}`);
});
await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let attempt = 0; attempt < 120; attempt += 1) {
  await page.waitForTimeout(1_000);
  const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
  if (ready) {
    lines.push(`${Date.now() - t0} READY ${ready}`);
    break;
  }
}
writeFileSync(join(OUT, "c5-shell-boot-diagnose.txt"), lines.join("\n\n"));
console.log(lines.slice(0, 6).join("\n\n").slice(0, 4_000));
console.log(`LINES ${lines.length}`);
await browser.close();
