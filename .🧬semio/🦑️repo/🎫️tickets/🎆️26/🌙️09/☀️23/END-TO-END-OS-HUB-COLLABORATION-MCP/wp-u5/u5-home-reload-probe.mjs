#!/usr/bin/env bun
/** 🔁️ U5 — Home's space table on a boot that RESTORES a hub session (sign in, reload): records hub rows over time, the
 * directory socket frontier, and whether a forced re-render (Viewer → Editor role round trip) brings the rows back.
 * Usage: bun u5-home-reload-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6580/", tag = "reload"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => { lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 400)); if (m.text().includes("[DEBUG] u5")) console.log(`${stamp()} ${m.text().slice(0, 400)}`); });
page.on("response", (response) => { if (response.status() >= 400) lines.push(`${stamp()} http ${response.status()} ${response.url().replace(/^https?:\/\/[^/]+/u, "").slice(0, 200)}`); });
page.on("pageerror", (e) => lines.push(`${stamp()} pageerror: ${String(e)}`.slice(0, 400)));
page.on("websocket", (ws) => {
  const url = ws.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 80);
  lines.push(`${stamp()} ws-open ${url}`);
  ws.on("close", () => lines.push(`${stamp()} ws-close ${url}`));
});
const read = () => page.evaluate(() => ({
  hub: document.querySelector('[role="status"][data-semio-hub-connection]')?.getAttribute("data-semio-hub-connection") ?? null,
  rows: [...new Set([...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((el) => el.getAttribute("data-ui-node-key")))],
  rowcount: Number(document.querySelector('[role="grid"][data-ui-node-key="framework.window.table"]')?.getAttribute("aria-rowcount") ?? 0),
  status: (document.querySelector('[role="grid"] [role="status"]')?.textContent ?? "").trim(),
  geometry: (() => {
    const grid = document.querySelector('[role="grid"][data-ui-node-key="framework.window.table"]');
    const scroller = grid?.querySelector('[data-slot="table-window-scroll"]');
    const container = grid?.querySelector("[data-tree-window-key]");
    const rect = (element) => element ? (({ top, height, width }) => ({ top: Math.round(top), height: Math.round(height), width: Math.round(width) }))(element.getBoundingClientRect()) : null;
    return grid ? { grid: rect(grid), scroller: scroller ? { ...rect(scroller), client: scroller.clientHeight, scroll: scroller.scrollHeight } : null, container: container ? { ...rect(container), total: container.getAttribute("data-tree-window-total"), offset: container.getAttribute("data-tree-window-offset"), length: container.getAttribute("data-tree-window-length"), path: container.getAttribute("data-tree-window-path") } : null } : null;
  })(),
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((el) => `${el.getAttribute("data-directory-bootstrap")}:${el.getAttribute("data-directory-bootstrap-code") ?? ""}`),
  role: document.querySelector('[aria-pressed="true"][id*="role" i], [data-state="on"][id*="role" i]')?.id ?? null,
}));
const timeline = [];
const sample = async (at) => {
  const now = await read();
  timeline.push({ at, hubRows: now.rows.filter((row) => row !== "space:default").length, ...now });
  lines.push(`${stamp()} sample ${at}`);
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
await page.waitForTimeout(3_000);
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
for (const wait of [0, 5_000]) {
  await page.waitForTimeout(wait);
  await sample(`signed-in +${wait}`);
}
lines.push(`${stamp()} probe: reload`);
await page.reload({ waitUntil: "commit" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 }).catch(() => undefined);
for (const wait of [2_000, 5_000, 10_000]) {
  await page.waitForTimeout(wait);
  await sample(`reloaded +${wait}`);
}
await page.screenshot({ path: out(`u5-home-${tag}-stuck.png`) });
const viewer = page.locator('[id="playground.navbar.roles.viewer"]').first();
const editor = page.locator('[id="playground.navbar.roles.editor"]').first();
lines.push(`${stamp()} probe: viewer`);
await viewer.click({ force: true }).catch((error) => lines.push(`viewer click failed ${error}`));
await page.waitForTimeout(4_000);
await sample("viewer role");
lines.push(`${stamp()} probe: editor`);
await editor.click({ force: true }).catch((error) => lines.push(`editor click failed ${error}`));
await page.waitForTimeout(4_000);
await sample("editor role again");
await page.waitForTimeout(6_000);
await sample("editor role again +10000");
await page.screenshot({ path: out(`u5-home-${tag}-after-role-trip.png`) });
writeFileSync(out(`u5-home-${tag}.json`), JSON.stringify({ baseUrl, timeline, lines: lines.filter((l) => !/agent-bridge|\[vite\]/u.test(l)).slice(-200) }, null, 1));
for (const row of timeline) console.log(JSON.stringify({ at: row.at, hub: row.hub, hubRows: row.hubRows, rowcount: row.rowcount, status: row.status, geometry: row.geometry }));
const consoleErrors = lines.filter((line) => / error: |pageerror:| http [45]\d\d /u.test(line));
console.log(`console errors: ${consoleErrors.length}`);
for (const line of consoleErrors) console.log(line.slice(0, 300));
await browser.close();
