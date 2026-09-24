#!/usr/bin/env bun
/** 🏡️ U5 — Home's own space table after sign-in, then a space created by ANOTHER client of the same user while Home is
 * open: measures when its `space:<id>` row appears and captures every directory-lane console line (fold refusals,
 * frontier races, page acks). Usage: bun u5-home-live-probe.mjs <baseUrl> <hubOrigin> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6580/", hubOrigin = "http://127.0.0.1:8080", tag = "live"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const seed = fileURLToPath(new URL("./u5-hub-seed.ts", import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${new Date().toISOString().slice(11, 23)} ${m.type()}: ${m.text()}`.slice(0, 400)));
page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 400)));
page.on("response", (response) => { if (response.status() >= 400) lines.push(`${new Date().toISOString().slice(11, 23)} http ${response.status()} ${response.request().method()} ${response.url()}`.slice(0, 300)); });
let navigations = 0;
let documentLoads = 0;
page.on("framenavigated", (frame) => { if (frame === page.mainFrame()) { navigations += 1; lines.push(`${new Date().toISOString().slice(11, 23)} nav ${navigations}`); } });
page.on("websocket", (ws) => {
  const url = ws.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 120);
  if (url.startsWith("/?token=")) documentLoads += 1;
  lines.push(`${new Date().toISOString().slice(11, 23)} ws-open ${url}`);
  let frames = 0;
  ws.on("framereceived", (frame) => { frames += 1; if (/directory/u.test(url)) lines.push(`${new Date().toISOString().slice(11, 23)} ws-frame ${url} #${frames} ${String(frame.payload).slice(0, 160)}`); });
  ws.on("close", () => lines.push(`${new Date().toISOString().slice(11, 23)} ws-close ${url} after ${frames} frames`));
});
const read = () => page.evaluate(() => ({
  hub: document.querySelector('[role="status"][data-semio-hub-connection]')?.getAttribute("data-semio-hub-connection") ?? null,
  rows: [...new Set([...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((el) => `${el.getAttribute("data-ui-node-key")}=${(el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)}`))],
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((el) => `${el.getAttribute("data-directory-bootstrap")}:${(el.textContent ?? "").slice(0, 80)}`),
}));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await page.waitForTimeout(3_000);
const timeline = [{ at: "signed-out", ...(await read()) }];
await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 60_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await form.locator('input[type="email"]').fill("ada@example.org");
await form.locator('input[type="password"]').fill("correct horse battery staple");
await page.screenshot({ path: out(`u5-home-${tag}-form.png`) });
await form.locator('[id="os.hub.signIn.submit"]').click({ force: true });
await page.waitForTimeout(2_000);
await page.screenshot({ path: out(`u5-home-${tag}-submitted.png`) });
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
await page.waitForTimeout(3_000);
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
await page.keyboard.press("Escape").catch(() => undefined);
const hubRows = (rows) => rows.filter((row) => !row.startsWith("space:default="));
const waitHubRows = async (from) => {
  while (Date.now() - from < 60_000) {
    if (hubRows((await read()).rows).length > 0) return Date.now() - from;
    await page.waitForTimeout(100);
  }
  return null;
};
const signedInAt = Date.now();
const firstRowsMs = await waitHubRows(signedInAt);
timeline.push({ at: "signed-in", firstRowsMs, documentLoads, ...(await read()) });
await page.screenshot({ path: out(`u5-home-${tag}-signed-in.png`) });
const before = new Set((await read()).rows.map((row) => row.split("=")[0]));
const name = `Probe ${new Date().toISOString().slice(11, 19)}`;
const createdAt = Date.now();
const created = spawnSync("bun", [seed, hubOrigin, name], { encoding: "utf8" });
const seedMs = Date.now() - createdAt;
let liveRowMs = null;
while (Date.now() - createdAt < 45_000) {
  const now = await read();
  if (now.rows.some((row) => !before.has(row.split("=")[0]))) { liveRowMs = Date.now() - createdAt; break; }
  await page.waitForTimeout(100);
}
timeline.push({ at: "after remote create", name, seed: `${created.stdout}${created.stderr}`.slice(0, 600), seedMs, liveRowMs, documentLoads, ...(await read()) });
await page.screenshot({ path: out(`u5-home-${tag}-after-create.png`) });
const ownBefore = new Set((await read()).rows.map((row) => row.split("=")[0]));
const ownName = `Own ${new Date().toISOString().slice(11, 19)}`;
const createNode = page.locator('[data-ui-node-key="s-home-create-space"]').first();
await createNode.waitFor({ state: "attached", timeout: 30_000 });
await createNode.focus();
await createNode.press("Enter");
const dialog = page.locator('[role="dialog"][data-slot="dialog-content"]');
await dialog.waitFor({ state: "visible", timeout: 15_000 });
await page.locator('[id="name"]').fill(ownName);
const submittedAt = Date.now();
lines.push(`${new Date().toISOString().slice(11, 23)} probe: own create submitted`);
await page.locator('[id="ui.dialog.submit"]').click();
await dialog.waitFor({ state: "hidden", timeout: 20_000 }).catch(() => undefined);
let ownRowMs = null;
while (Date.now() - submittedAt < 45_000) {
  const now = await read();
  if (now.rows.some((row) => !ownBefore.has(row.split("=")[0]))) { ownRowMs = Date.now() - submittedAt; break; }
  await page.waitForTimeout(100);
}
await page.waitForTimeout(3_000);
timeline.push({ at: "after own create", name: ownName, ownRowMs, documentLoads, ...(await read()) });
await page.screenshot({ path: out(`u5-home-${tag}-after-own-create.png`) });
const reloadedAt = Date.now();
lines.push(`${new Date().toISOString().slice(11, 23)} probe: reload`);
await page.reload({ waitUntil: "commit" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 }).catch(() => undefined);
const reloadRowsMs = await waitHubRows(reloadedAt);
timeline.push({ at: "after reload", reloadRowsMs, documentLoads, ...(await read()) });
await page.screenshot({ path: out(`u5-home-${tag}-after-reload.png`) });
const directoryLines = lines.filter((l) => !/agent-bridge/u.test(l)).filter((l) => /directory|fold|BatchOnly|frontier|dispatch-failed|interactive-job|pageerror|probe:|http [45]|ws-|nav /iu.test(l));
writeFileSync(out(`u5-home-${tag}.json`), JSON.stringify({ baseUrl, hubOrigin, timeline, directoryLines: directoryLines.slice(-120) }, null, 1));
for (const row of timeline) console.log(JSON.stringify(row).slice(0, 700));
console.log(`directory console lines: ${directoryLines.length}`);
for (const line of directoryLines.slice(-12)) console.log(line.slice(0, 300));
await browser.close();
