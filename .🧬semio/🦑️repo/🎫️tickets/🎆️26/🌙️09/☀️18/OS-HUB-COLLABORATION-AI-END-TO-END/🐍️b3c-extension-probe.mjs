/** 🧩️ Extension-cascade probe (slice B3c): does an `on-extension-request` child actually reach the
 * running shell, and does anything it contributes become usable there?
 *
 * Two witnesses, both from the live page:
 *   1. `window.__semioOsCatalogProbe.plugins` — the shell's own install roster, the only place an
 *      EXTENSION plugin id appears at all (`🏛️ShellHost/🟦️.tsx:705`).
 *   2. the catalogue panel's rendered rows — a flow/procedural extension contributes node kinds
 *      through its `flow.*` topic, so an extension-owned kind showing up as a clickable catalogue
 *      row is proof the contribution survived the host's install filter. The probe then adds one
 *      such node and reads the History ledger to prove it mutates the document.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const OUT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated";

const read = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  const probe = window.__semioOsCatalogProbe ?? null;
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    installed: probe === null ? [] : probe.plugins.map((row) => `${row.pluginId}:${row.status}${row.routerFault ? `/${row.routerFault.code}` : ""}`),
    panelTabs: [...document.querySelectorAll('[data-slot="panel-tab-button"]')].map((el) => el.id),
    panels: [...document.querySelectorAll('[data-slot="panel"]')].map((el) => el.id),
    catalogueRows: [...document.querySelectorAll('[id*="catalogue"], [id*="Catalogue"]')].map((el) => `${el.id}|${text(el).slice(0, 80)}`).slice(0, 400),
    treeRows: [...document.querySelectorAll('[role="treeitem"]')].map((el) => `${el.id}|${text(el).slice(0, 60)}`).slice(0, 400),
    ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 60)),
    checkin: text(document.querySelector("#s-checkin")) || null,
  };
});

const config = { plugin: process.env.B3C_PLUGIN ?? "generation3d", variant: process.env.B3C_VARIANT ?? "generation3d", port: Number(process.env.B3C_PORT ?? 6018) };
const url = `http://127.0.0.1:${config.port}/?plugin=${config.variant}`;
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
page.setDefaultNavigationTimeout(240_000);
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240_000 });

let shell = null;
for (let i = 0; i < 240; i++) {
  await page.waitForTimeout(1000);
  shell = await read(page);
  if (shell.error) break;
  if (shell.ready && shell.installed.length && i > 8) break;
}
await page.waitForTimeout(4000);

// 🧩️ Open every panel tab so a catalogue the shell folds away still renders its rows.
for (const id of (await read(page)).panelTabs) {
  await page.locator(`[id="${id}"]`).first().click({ timeout: 4000, force: true }).catch(() => {});
  await page.waitForTimeout(700);
}
for (const toggle of await page.locator('[id$=".engagement.toggle"]').all()) await toggle.click({ timeout: 4000, force: true }).catch(() => {});
await page.waitForTimeout(1500);
shell = await read(page);

// 🧩️ The decisive step: pick a catalogue row whose id carries an EXTENSION-owned node family
// (`brep.*`/`math.*` come from `flow-extension-brep`/`-math`'s `flow.*` topic, nothing in the host
// crate declares them) and add it. A growing History ledger proves the contribution is not just
// rendered but dispatchable into the document.
const owned = new RegExp(`\\.(${(process.env.B3C_EXT_FAMILIES ?? "brep|math").split("|").join("|")})\\.`);
const target = shell.catalogueRows.map((row) => row.split("|")[0]).find((id) => owned.test(id) && id.includes("neuron"));
const ledgerBefore = shell.ledger.length;
let added = { target: target ?? null, clicked: "absent", ledgerBefore, ledgerAfter: ledgerBefore, checkin: shell.checkin };
if (target) {
  added.clicked = await page.locator(`[id="${target}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 120));
  for (let i = 0; i < 20 && (await read(page)).ledger.length === ledgerBefore; i++) await page.waitForTimeout(700);
  const after = await read(page);
  added = { ...added, ledgerAfter: after.ledger.length, checkin: after.checkin, tail: after.ledger.slice(-2) };
  shell = after;
}
console.log("EXTENSION-NODE-ADD", JSON.stringify(added));
shell.extensionNodeAdd = added;

mkdirSync(join(OUT, `b3c-ext-${config.plugin}`), { recursive: true });
writeFileSync(join(OUT, `b3c-ext-${config.plugin}.json`), JSON.stringify(shell, null, 2));
writeFileSync(join(OUT, `b3c-ext-${config.plugin}-console.txt`), lines.join("\n"));
console.log("READY", shell.ready, "ERROR", shell.error);
console.log("INSTALLED", JSON.stringify(shell.installed));
console.log("PANELS", JSON.stringify(shell.panels));
console.log("CATALOGUE", shell.catalogueRows.length, JSON.stringify(shell.catalogueRows.slice(0, 80)));
await page.screenshot({ path: join(OUT, `b3c-ext-${config.plugin}.png`) });
await browser.close();
