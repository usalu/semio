/** 🔬️ S18 §14c (S20 relay): after ONE guest refusal on a hub document, is the NEXT action admitted — and does the refusal
 * say why? Creates one hub document of `<kind>` through the sweep's journey, stages + submits `<refusedVerb>` (S20: the
 * draw example load is refused on a hub document), then drives `<nextVerb>` → undo → redo with the matrix's own witness.
 * usage: OS_HUB_PROBE_EMAIL=… OS_HUB_PROBE_PASSWORD=… bun s18-probe-refusal-next.ts <serve> <space> <kind> <refusedVerb> <nextVerb> */
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { createKind, openSweepSpace, stagedKinds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts";
import { click, clickUncovered, mutateUndoRedo, submitStagedVerb, unfoldActionsRail, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const [baseUrl = "http://127.0.0.1:6540/", spaceName = "S18 Hub Sweep p24", kindId = "2d.drawing", refusedVerb = "setActiveExample", nextVerb = "addLayer"] = process.argv.slice(2);
const profileDir = mkdtempSync(join(tmpdir(), "s18-refusal-"));
const context = await chromium.launchPersistentContext(profileDir, { headless: true, args: ["--use-angle=metal"], viewport: { width: 1600, height: 1000 }, locale: "en-US" });
const page = context.pages()[0] ?? (await context.newPage());
const started = Date.now();
const lines: string[] = [];
const refusals: string[] = [];
page.on("console", (message) => { const text = message.text(); if (/refused|owner-mismatch|unconfirmed|guest-refused|Cannot update a component|setState/u.test(text)) { lines.push(`${Date.now() - started} ${message.type()} ${text.slice(0, 400)}`); if (/refused:|rejected/u.test(text)) refusals.push(text.slice(0, 240)); } });
page.on("pageerror", (error) => lines.push(`${Date.now() - started} pageerror ${String(error).slice(0, 300)}`));
try {
  const row = { kindId, plugin: "-", pass: false, faults: [], notices: [] } as never;
  const options = { baseUrl, locale: "en", email: process.env.OS_HUB_PROBE_EMAIL ?? "", password: process.env.OS_HUB_PROBE_PASSWORD ?? "", spaceName, kinds: [], sagaMs: 0, puzzleSagaMs: 0, reopen: false, cancel: null, profileDir, outDir: profileDir, signal: new AbortController().signal } as never;
  console.log("OPEN", (await openSweepSpace(page, options, row)) ?? "ok");
  await unfoldActionsRail(page);
  await click(page, '[data-slot="window-action-pane"] [id="action.createArtifact"]');
  await page.waitForTimeout(2_000);
  const kind = (await stagedKinds(page)).find((entry) => entry.kindId === kindId);
  console.log("REOPEN", (await openSweepSpace(page, options, row)) ?? "ok");
  await unfoldActionsRail(page);
  console.log("SUBMIT", kind ? await createKind(page, kind.value, `S18 refusal ${Date.now() % 100000}`) : "no kind");
  const deadline = Date.now() + 300_000;
  while (Date.now() < deadline && (await windowIds(page)).every((id) => id === "framework.window.table" || id === "s-home-main")) await page.waitForTimeout(2_000);
  await page.waitForTimeout(8_000);
  console.log("WINDOWS", JSON.stringify(await windowIds(page)));
  await unfoldActionsRail(page);
  if (refusedVerb === "navbar-example") {
    const picker = page.locator('[id="playground.navbar.fixture"]').first();
    console.log("PICKER", await picker.count());
    await picker.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(800);
    const options = await page.locator('[role="option"]').evaluateAll((rows) => rows.map((row) => `${row.getAttribute("data-value")}|${(row.textContent ?? "").trim()}|${row.getAttribute("aria-selected")}`));
    console.log("OPTIONS", JSON.stringify(options));
    const index = options.findIndex((option) => !/^\|/u.test(option) && !/^_+none_+\|/u.test(option) && !/\|(empty|leer|none|keine)\|/iu.test(option));
    if (index >= 0) await page.locator('[role="option"]').nth(index).click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape");
  } else {
    console.log("REFUSED-VERB", await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${refusedVerb}"]`));
    await page.waitForTimeout(1_000);
    console.log("REFUSED-SUBMIT", await submitStagedVerb(page, refusedVerb));
  }
  await page.waitForTimeout(6_000);
  const cursor = refusals.length;
  const verb = await mutateUndoRedo(page, refusals, "probe", { probe: nextVerb }, {}, {}, "@liveId", 1);
  console.log("NEXT", JSON.stringify({ verb: verb.mutation, detail: verb.mutationDetail, edits: "edits" in verb ? verb.edits : null, attempts: verb.attempts.map((attempt) => `${attempt.verbId}:${attempt.edits.join(",")}:${attempt.refusal ?? "-"}`) }));
  console.log("REFUSALS-AFTER-FIRST", refusals.length - cursor);
} catch (error) {
  console.log("ERROR", String(error).slice(0, 400));
} finally {
  for (const line of lines.slice(-20)) console.log("LINE", line);
  await context.close();
  rmSync(profileDir, { recursive: true, force: true });
}
