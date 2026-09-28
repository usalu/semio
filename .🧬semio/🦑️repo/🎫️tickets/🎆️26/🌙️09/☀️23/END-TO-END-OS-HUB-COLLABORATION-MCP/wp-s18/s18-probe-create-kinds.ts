/** 🔬️ S18 §14c hub leg: why the space index's `createArtifact` offers 0 kinds on 7800 (p24). Opens the sweep space through the
 * sweep's own journey, stages `createArtifact`, and dumps the form's controls, kind options, console refusals and the
 * hub requests the shell made for its creation catalog.
 * usage: OS_HUB_PROBE_EMAIL=… OS_HUB_PROBE_PASSWORD=… bun s18-probe-create-kinds.ts <serve url> <space name> */
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { openSweepSpace, stagedKinds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts";
import { click, unfoldActionsRail } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const [baseUrl = "http://127.0.0.1:6540/", spaceName = "S18 Hub Sweep p24"] = process.argv.slice(2);
const profileDir = mkdtempSync(join(tmpdir(), "s18-create-kinds-"));
const context = await chromium.launchPersistentContext(profileDir, { headless: true, args: ["--use-angle=metal"], viewport: { width: 1600, height: 1000 }, locale: "en-US" });
const page = context.pages()[0] ?? (await context.newPage());
const lines: string[] = [];
const started = Date.now();
page.on("console", (message) => { const text = message.text(); if (/refus|catalog|creat|kind|fault|error/iu.test(text) && !/DevTools/u.test(text)) lines.push(`${Date.now() - started} ${message.type()} ${text.slice(0, 300)}`); });
page.on("pageerror", (error) => lines.push(`${Date.now() - started} pageerror ${String(error).slice(0, 300)}`));
page.on("response", (response) => { const url = response.url(); if (/_semio\/hub|:7800\//u.test(url) && !/sessions\/me/u.test(url)) lines.push(`${Date.now() - started} http ${response.status()} ${response.request().method()} ${url.replace(/^https?:\/\/[^/]+/u, "").slice(0, 160)}`); });
try {
  const row = { kindId: "-", plugin: "-", pass: false, faults: [], notices: [] } as never;
  const blocked = await openSweepSpace(page, { baseUrl, locale: "en", email: process.env.OS_HUB_PROBE_EMAIL ?? "", password: process.env.OS_HUB_PROBE_PASSWORD ?? "", spaceName, kinds: [], sagaMs: 0, puzzleSagaMs: 0, reopen: false, cancel: null, profileDir, outDir: profileDir, signal: new AbortController().signal } as never, row);
  console.log("OPEN", blocked ?? "ok", JSON.stringify(row));
  console.log("RAIL-TOGGLES", await unfoldActionsRail(page));
  console.log("CREATE", await click(page, '[data-slot="window-action-pane"] [id="action.createArtifact"]'));
  await page.waitForTimeout(4_000);
  console.log("FORM", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id*="createArtifact"], [data-slot="window-action-pane"] [id*=".arg."]')].map((element) => `${element.tagName}#${element.id} role=${element.getAttribute("role")} text=${(element.textContent ?? "").trim().slice(0, 80)}`).slice(0, 30))));
  console.log("SELECT-OPTIONS", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("select")].map((select) => `${select.id}: ${[...select.options].map((option) => option.value.slice(0, 60)).join(" | ").slice(0, 600)}`))));
  console.log("STAGED-KINDS", (await stagedKinds(page)).length);
  const combobox = page.locator('[data-slot="window-action-pane"] [id$=".arg.kindChoice"] [role="combobox"], [data-slot="window-action-pane"] [role="combobox"]').first();
  if ((await combobox.count()) > 0) {
    await combobox.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_000);
    console.log("COMBO-OPTIONS", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((option) => `${option.getAttribute("data-value")?.slice(0, 80)} :: ${(option.textContent ?? "").trim().slice(0, 40)}`).slice(0, 40))));
  }
  await page.screenshot({ path: "/Users/ueli/Documents/semio/.tmp-ticket/wp-s18/generated/s18-14c-create-kinds.png" });
} catch (error) {
  console.log("ERROR", String(error).slice(0, 400));
} finally {
  for (const line of lines.slice(-60)) console.log("LINE", line);
  await context.close();
  rmSync(profileDir, { recursive: true, force: true });
}
