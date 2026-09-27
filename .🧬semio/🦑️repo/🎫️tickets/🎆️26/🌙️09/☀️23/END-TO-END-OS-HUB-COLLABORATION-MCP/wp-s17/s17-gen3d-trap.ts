/** 🪤️ S17 one-off reproduction of the generation3d `toolRunStart` guest trap in the served `s` shell: open generation3d from
 * Home, let the contributions push start the preview run, then drive the matrix's own verb/undo/redo sequence and keep EVERY
 * console line in full (the trap's JS error stack names the wasm frames). usage: bun s17-gen3d-trap.ts <baseUrl> <out.txt> [rounds] */
import { appendFileSync, writeFileSync } from "node:fs";
import type { Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { awaitBeacon, dismissIntroduction, mutateUndoRedo } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const [baseUrl, out, roundsArg] = process.argv.slice(2) as [string, string, string | undefined];
const rounds = Number(roundsArg ?? 3);
writeFileSync(out, "");
const log = (line: string): void => appendFileSync(out, `${line}\n`);
ensureParityPlaywrightBrowsersPath();
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page: Page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
let trapped = false;
page.on("console", (message) => {
  const text = message.text();
  if (/panic|trap|unreachable|ordered-map/iu.test(text)) trapped = true;
  if (message.type() === "error" || message.type() === "warning" || /panic|toolRun|flowEval|contribut/iu.test(text)) log(`[${new Date().toISOString()}] ${message.type()}: ${text}`);
});
page.on("pageerror", (error) => log(`[pageerror] ${String(error)}\n${error.stack ?? ""}`));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
for (let round = 0; round < rounds && !trapped; round += 1) {
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill("generation3d");
  await page.waitForTimeout(1_200);
  await page.locator('[data-slot="command-item"][data-command-item-id="spawn.procedural.s.procedural.generation3d@1/*#editor"]').first().click();
  log(`[round ${round}] opened`);
  await page.waitForTimeout(3_000);
  const result = await mutateUndoRedo(page, [], "procedural/generation3d", { "procedural/generation3d": "addWidget" }, {}, "@liveId", 2);
  log(`[round ${round}] ${JSON.stringify({ mutation: result.mutation, detail: result.mutationDetail, attempts: result.attempts.map((attempt) => attempt.verbId) })}`);
  await page.waitForTimeout(8_000);
}
log(`[done] trapped=${trapped}`);
await browser.close();
