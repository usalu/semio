#!/usr/bin/env bun
/** 📊️ U5 §6b — Home's windowed table with many hub spaces, live: sign ada in, wait for Home's grid, read its logical
 * extent and materialised window, walk to the LAST space with the keyboard (End) and by scrolling, and read it back in
 * German. Every step also records surface render faults (the `nodes 129 > 128` class) from the console.
 * Usage: bun u5-home-table-probe.mjs <baseUrl> <tag> <expectedSpaces> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6580/", tag = "table", expected = "30"] = process.argv.slice(2);
const expectedSpaces = Number(expected);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 400)));
page.on("pageerror", (e) => lines.push(`${stamp()} pageerror: ${String(e)}`.slice(0, 400)));
const hubAnswers = [];
const hubAsks = [];
page.on("request", (request) => {
  const url = request.url();
  if (/\/auth\/sessions|\/directory\//u.test(url)) hubAsks.push({ at: Date.now(), path: new URL(url).pathname });
});
page.on("response", (response) => {
  const url = response.url();
  if (/\/auth\/sessions|\/directory\//u.test(url)) hubAnswers.push({ at: Date.now(), status: response.status(), path: new URL(url).pathname });
});
const grid = () => page.locator('[role="grid"][data-ui-node-key="framework.window.table"]').first();
const read = () => page.evaluate(() => {
  const table = document.querySelector('[role="grid"][data-ui-node-key="framework.window.table"]');
  if (!table) return null;
  const container = table.querySelector("[data-tree-window-key]");
  const rows = [...table.querySelectorAll('[role="row"][data-table-row-index]')];
  return {
    label: table.getAttribute("aria-label"),
    rowcount: Number(table.getAttribute("aria-rowcount")),
    headers: [...table.querySelectorAll('[role="columnheader"]')].map((header) => (header.textContent ?? "").trim()),
    window: container ? { total: Number(container.getAttribute("data-tree-window-total")), offset: Number(container.getAttribute("data-tree-window-offset")), length: Number(container.getAttribute("data-tree-window-length")) } : null,
    firstCells: rows[0] ? [...rows[0].querySelectorAll('[role="gridcell"]')].map((cell) => (cell.textContent ?? "").trim()) : [],
    firstActions: rows[0] ? [...rows[0].querySelectorAll("button")].map((button) => button.getAttribute("aria-label") ?? (button.textContent ?? "").trim()) : [],
    first: rows[0] ? { index: Number(rows[0].getAttribute("data-table-row-index")), key: rows[0].getAttribute("data-ui-node-key"), name: (rows[0].querySelector('[role="gridcell"]')?.textContent ?? "").trim() } : null,
    last: rows.at(-1) ? { index: Number(rows.at(-1).getAttribute("data-table-row-index")), key: rows.at(-1).getAttribute("data-ui-node-key"), name: (rows.at(-1).querySelector('[role="gridcell"]')?.textContent ?? "").trim() } : null,
    materialised: rows.length,
    focused: document.activeElement?.getAttribute("data-table-row-index") ?? null,
    status: (table.querySelector('[role="status"]')?.textContent ?? "").trim(),
    nodes: document.querySelectorAll('[data-ui-node-key]').length,
  };
});
const timeline = [];
const note = async (at, extra = {}) => {
  const state = await read();
  timeline.push({ at, ...extra, ...state });
  lines.push(`${stamp()} probe: ${at}`);
};
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await page.waitForTimeout(3_000);
await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 60_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await form.locator('input[type="email"]').fill(process.env.U5_EMAIL ?? "ada@example.org");
await form.locator('input[type="password"]').fill(process.env.U5_PASSWORD ?? "correct horse battery staple");
await form.locator('[id="os.hub.signIn.submit"]').click({ force: true });
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
await page.waitForTimeout(2_000);
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
const signedIn = Date.now();
await page.waitForFunction((count) => Number(document.querySelector('[role="grid"][data-ui-node-key="framework.window.table"]')?.getAttribute("aria-rowcount") ?? 0) >= count + 1, expectedSpaces, { timeout: 60_000 }).catch(() => undefined);
const rowsAt = Date.now();
const minted = hubAnswers.find((answer) => answer.path === "/auth/sessions" && answer.status < 300);
const lastDirectoryBeforeRows = hubAnswers.filter((answer) => /(^|\/_semio\/hub)\/directory\//u.test(answer.path) && answer.at <= rowsAt).at(-1);
await note("signed in", { gridMs: rowsAt - signedIn, mintToRowsMs: minted ? rowsAt - minted.at : null, lastDirectoryAnswerToRowsMs: lastDirectoryBeforeRows ? rowsAt - lastDirectoryBeforeRows.at : null, lastDirectoryAnswer: lastDirectoryBeforeRows?.path ?? null, hubAnswers: hubAnswers.map((answer) => `${answer.path} ${answer.status} +${answer.at - (minted?.at ?? answer.at)}ms`).slice(0, 12), hubAsks: hubAsks.map((ask) => `${ask.path} asked +${ask.at - (minted?.at ?? ask.at)}ms`).slice(0, 12) });
const aria = await grid().ariaSnapshot().catch((error) => `ariaSnapshot failed: ${error}`);
timeline.push({ at: "aria snapshot", aria: aria.split("\n").slice(0, 24) });
await page.screenshot({ path: out(`u5-home-${tag}-top.png`) });
const finish = async () => {
  const faults = lines.filter((line) => /render fault|Credits|max_nodes|pageerror|refused/iu.test(line));
  writeFileSync(out(`u5-home-${tag}.json`), JSON.stringify({ baseUrl, expectedSpaces, timeline, faults: faults.slice(-40), lines: lines.filter((line) => !/agent-bridge|\[vite\]|404/u.test(line)).slice(-120) }, null, 1));
  for (const row of timeline) console.log(JSON.stringify(row));
  console.log(`faults: ${faults.length}`);
  for (const fault of faults.slice(-6)) console.log(fault.slice(0, 300));
  await browser.close();
};
if ((await grid().count()) === 0) {
  const home = await page.evaluate(() => ({ spaceRows: document.querySelectorAll('[data-ui-node-key^="space:"]').length, text: (document.querySelector('[data-ui-node-key="s-home-empty"]')?.textContent ?? "").trim() }));
  timeline.push({ at: "no windowed grid", ...home });
  await finish();
  process.exit(0);
}
const firstRow = grid().locator('[role="row"][data-table-row-index]').first();
await firstRow.focus();
const walked = Date.now();
await page.keyboard.press("End");
const lastIndex = (await read())?.rowcount - 2;
await page.waitForFunction((index) => document.activeElement?.getAttribute("data-table-row-index") === String(index), lastIndex, { timeout: 30_000 }).catch(() => undefined);
await note("End pressed", { walkMs: Date.now() - walked, lastIndex });
await page.keyboard.press("PageUp");
await page.waitForTimeout(600);
await note("PageUp");
await page.keyboard.press("ArrowUp");
await page.waitForTimeout(300);
await note("ArrowUp");
await page.keyboard.press("ArrowRight");
await page.waitForTimeout(300);
timeline.push({ at: "ArrowRight", activeName: await page.evaluate(() => document.activeElement?.getAttribute("aria-label") ?? (document.activeElement?.textContent ?? "").trim()), activeRole: await page.evaluate(() => document.activeElement?.tagName) });
await page.keyboard.press("Escape");
await page.waitForTimeout(300);
await note("Escape");
await page.screenshot({ path: out(`u5-home-${tag}-end.png`) });
await page.keyboard.press("Home");
await page.waitForFunction(() => document.activeElement?.getAttribute("data-table-row-index") === "0", undefined, { timeout: 30_000 }).catch(() => undefined);
await note("Home pressed");
const scroller = grid().locator('[data-slot="table-window-scroll"]');
await scroller.evaluate((element) => { element.scrollTop = element.scrollHeight; });
const scrolled = Date.now();
await page.waitForFunction((index) => !!document.querySelector(`[role="grid"][data-ui-node-key="framework.window.table"] [data-table-row-index="${index}"]`), lastIndex, { timeout: 30_000 }).catch(() => undefined);
await note("scrolled to the bottom", { scrollMs: Date.now() - scrolled });
await page.screenshot({ path: out(`u5-home-${tag}-bottom.png`) });
await page.locator('[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], [id="framework.settings"]').first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(1_500);
const control = page.locator('select, [role="combobox"]').filter({ hasText: /english|deutsch|german|englisch/iu }).first();
if ((await control.count()) > 0) {
  if ((await control.evaluate((element) => element.tagName.toLowerCase())) === "select") await control.selectOption("de").catch(() => undefined);
  else {
    await control.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(600);
    await page.locator('[role="option"]').filter({ hasText: /deutsch|german/iu }).first().click({ force: true }).catch(() => undefined);
  }
}
await page.waitForFunction(() => document.documentElement.lang === "de", undefined, { timeout: 15_000 }).catch(() => undefined);
await page.keyboard.press("Escape").catch(() => undefined);
await page.locator('[id="framework.settings"]').first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(4_000);
await note("German");
await page.screenshot({ path: out(`u5-home-${tag}-de.png`) });
await finish();
