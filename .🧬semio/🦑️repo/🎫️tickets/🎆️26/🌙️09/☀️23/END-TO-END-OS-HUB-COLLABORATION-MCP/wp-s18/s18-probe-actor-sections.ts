/** 🧩️ S18 §14c (C13 P1) live proof: a hub draw document's window engagement ("N layers · 0 selected") follows committed
 * edits in every open window — a fresh 2d.drawing in a fresh space, every window's engagement status at rest, after
 * `addLayer` through the rendered Actions rail, after undo and after redo; per locale. Captures go to <captureDir>.
 * usage: bun s18-probe-actor-sections.ts <serveUrl> <en|de> <captureDir> */
import { join } from "node:path";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { USERS } from "../wp-c11/c11-lib.mjs";
import { clickUncovered, readShell, submitStagedVerb, unfoldActionsRail, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { createKind, openSweepSpace, stagedKinds, type HubDocumentSweepOptions, type HubSweepRow } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts";

const [baseUrl = "http://127.0.0.1:6540/", locale = "en", captures = "."] = process.argv.slice(2) as [string, "en" | "de", string];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: locale === "de" ? "de-DE" : "en-US" });
const page = await context.newPage();
const faults: string[] = [];
page.on("pageerror", (error) => faults.push(`pageerror ${String(error)}`.slice(0, 300)));
page.on("console", (message) => { if (/refused|fault|panicked|trap/iu.test(message.text()) && message.type() !== "debug") faults.push(`${message.type()} ${message.text()}`.slice(0, 300)); });
const status = (): Promise<Record<string, string[]>> =>
  page.evaluate(() => Object.fromEntries([...document.querySelectorAll("[data-window-id]")].map((window) => [window.getAttribute("data-window-id") ?? "?", [...window.querySelectorAll("*")].filter((element) => element.childElementCount === 0 && /\d+\s+layers?\s+·/u.test(element.textContent ?? "")).map((element) => (element.textContent ?? "").trim())])));
const ledger = async (): Promise<string[]> => (await readShell(page)).ledger.slice(-3).map((entry) => entry.label);
const verdict: Record<string, unknown> = { locale };
try {
  const user = USERS[0] as { email: string; password: string };
  const options = { baseUrl, locale, email: user.email, password: user.password, spaceName: `S18 Sections ${locale}`, kinds: [], sagaMs: 240_000, puzzleSagaMs: 240_000, reopen: false, cancel: null, profileDir: "", outDir: captures, signal: new AbortController().signal } as HubDocumentSweepOptions;
  const row = { kindId: "2d.drawing", plugin: "draw", pass: false, faults: [], notices: [] } as HubSweepRow;
  verdict.space = (await openSweepSpace(page, options, row)) ?? row.spaceId;
  await unfoldActionsRail(page);
  const draw = (await stagedKinds(page)).find((kind) => kind.kindId === "2d.drawing");
  if (!draw) throw new Error("2d.drawing not staged");
  verdict.created = await createKind(page, draw.value, `S18 Sections ${locale} ${Date.now() % 100000}`);
  const deadline = Date.now() + 240_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => id !== "framework.window.table" && id !== "s-home-main").length === 0) await page.waitForTimeout(1_000);
  await page.waitForTimeout(8_000);
  verdict.windows = await windowIds(page);
  verdict.atRest = await status();
  await unfoldActionsRail(page);
  const started = Date.now();
  verdict.addLayer = `${await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.addLayer"]')}/${await submitStagedVerb(page, "addLayer")}`;
  let afterAdd = await status();
  while (Date.now() - started < 15_000 && !Object.values(afterAdd).flat().some((text) => text.startsWith("2 layers"))) {
    await page.waitForTimeout(250);
    afterAdd = await status();
  }
  verdict.afterAdd = afterAdd;
  verdict.addToStatusMs = Date.now() - started;
  verdict.ledgerAfterAdd = await ledger();
  await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]');
  await page.waitForTimeout(3_000);
  verdict.afterUndo = await status();
  await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.redo"]');
  await page.waitForTimeout(3_000);
  verdict.afterRedo = await status();
  verdict.ledgerTail = await ledger();
  await page.screenshot({ path: join(captures, `s18-14c-sections-${locale}.png`) });
} catch (error) {
  verdict.error = String((error as Error)?.stack ?? error).slice(0, 600);
} finally {
  verdict.faults = faults.slice(0, 12);
  console.log("VERDICT", JSON.stringify(verdict));
  await browser.close();
}
