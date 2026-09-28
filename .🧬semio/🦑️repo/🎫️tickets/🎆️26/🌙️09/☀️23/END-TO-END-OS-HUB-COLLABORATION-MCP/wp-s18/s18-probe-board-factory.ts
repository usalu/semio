/** 🔬️ S18 §14c hub leg: which program identity the shell resolves the 2d board factory against for a hub-created puzzle
 * document (the sweep's own journey: open the sweep space, create one `2d.puzzle`, wait for its windows), with the
 * temporary `[DEBUG] s18 board` lines of ShellHost and every board error.
 * usage: OS_HUB_PROBE_EMAIL=… OS_HUB_PROBE_PASSWORD=… bun s18-probe-board-factory.ts <serve url> <space name> [kind] */
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { createKind, openSweepSpace, stagedKinds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts";
import { click, unfoldActionsRail, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const [baseUrl = "http://127.0.0.1:6540/", spaceName = "S18 Hub Sweep p24", kindId = "2d.puzzle"] = process.argv.slice(2);
const profileDir = mkdtempSync(join(tmpdir(), "s18-board-"));
const context = await chromium.launchPersistentContext(profileDir, { headless: true, args: ["--use-angle=metal"], viewport: { width: 1600, height: 1000 }, locale: "en-US" });
const page = context.pages()[0] ?? (await context.newPage());
const started = Date.now();
const lines: string[] = [];
page.on("console", (message) => { const text = message.text(); if (/\[DEBUG\] s18|board session|pageerror|refused/u.test(text)) lines.push(`${Date.now() - started} ${message.type()} ${text.slice(0, 700)}`); });
page.on("pageerror", (error) => lines.push(`${Date.now() - started} pageerror ${String(error).slice(0, 300)}`));
try {
  const row = { kindId, plugin: "puzzle", pass: false, faults: [], notices: [] } as never;
  const options = { baseUrl, locale: "en", email: process.env.OS_HUB_PROBE_EMAIL ?? "", password: process.env.OS_HUB_PROBE_PASSWORD ?? "", spaceName, kinds: [], sagaMs: 0, puzzleSagaMs: 0, reopen: false, cancel: null, profileDir, outDir: profileDir, signal: new AbortController().signal } as never;
  console.log("OPEN", (await openSweepSpace(page, options, row)) ?? "ok");
  await unfoldActionsRail(page);
  await click(page, '[data-slot="window-action-pane"] [id="action.createArtifact"]');
  await page.waitForTimeout(2_000);
  const kind = (await stagedKinds(page)).find((entry) => entry.kindId === kindId);
  console.log("KIND", kind?.value.slice(0, 160) ?? "absent");
  if (kind) {
    console.log("REOPEN", (await openSweepSpace(page, options, row)) ?? "ok");
    await unfoldActionsRail(page);
    console.log("SUBMIT", await createKind(page, kind.value, `S18 board ${Date.now() % 100000}`));
  }
  const deadline = Date.now() + 420_000;
  while (Date.now() < deadline && !(await windowIds(page)).some((id) => /2d-|puzzle/u.test(id))) await page.waitForTimeout(2_000);
  await page.waitForTimeout(15_000);
  console.log("WINDOWS", JSON.stringify(await windowIds(page)), `${Math.round((Date.now() - started) / 1000)} s`);
} catch (error) {
  console.log("ERROR", String(error).slice(0, 400));
} finally {
  for (const line of lines.slice(-30)) console.log("LINE", line);
  await context.close();
  rmSync(profileDir, { recursive: true, force: true });
}
