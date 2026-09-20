/** 🔬️ Slice B3a2 — block DOM diagnosis: the block2d shell boots (`ready=block2d`, window
 * `block2d-board`, example combobox populated) but publishes NO surface pane, NO `#s-checkin` and
 * NO `action.*` row after the engagement toggle is pressed. This dumps what the window body, the
 * engagement rail and the network layer actually carry so the missing link can be named rather
 * than guessed. `SEMIO_BLOCK_VARIANT`/`SEMIO_BLOCK_PORT` retarget it at block3d/block5d.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";

const VARIANT = process.env.SEMIO_BLOCK_VARIANT ?? "block2d";
const PORT = process.env.SEMIO_BLOCK_PORT ?? "6024";
const OUT = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3a2-${VARIANT}-diagnose.txt`;
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`console ${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`pageerror ${Date.now() - t0} ${String(e).slice(0, 2000)}`));
page.on("requestfailed", (r) => lines.push(`requestfailed ${Date.now() - t0} ${r.url().slice(0, 200)} ${r.failure()?.errorText}`));
page.on("response", (r) => { if (r.status() >= 400) lines.push(`http${r.status()} ${Date.now() - t0} ${r.url().slice(0, 300)}`); });

await page.goto(`http://127.0.0.1:${PORT}/?plugin=${VARIANT}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break;
}
await page.waitForTimeout(8000);

lines.push("\n## root attributes");
lines.push(JSON.stringify(await page.evaluate(() => Object.fromEntries([...document.documentElement.attributes].map((a) => [a.name, a.value.slice(0, 300)])))));

lines.push("\n## window bodies");
lines.push(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="window"]')].map((el) => ({
  id: el.id,
  attrs: Object.fromEntries([...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => [a.name, a.value.slice(0, 200)])),
  text: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 600),
  surfaces: el.querySelectorAll("[data-surface-id]").length,
  canvases: el.querySelectorAll("canvas").length,
  html: el.innerHTML.slice(0, 2500),
}))), null, 1));

lines.push("\n## engagement rail with the probe's panels open (does a docked panel cover the rail?)");
const toggles = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id));
lines.push(`toggles ${JSON.stringify(toggles)}`);
for (const tab of ["framework.panel.inspection", "framework.panel.artifact", "framework.panel.history"]) {
  await page.locator(`[data-slot="panel-tab-button"][id="${tab}"], [id="${tab}"]`).first().click({ force: true, timeout: 8000 }).catch((e) => lines.push(`${tab} click failed ${String(e).split("\n")[0]}`));
  await page.waitForTimeout(1200);
}
lines.push(JSON.stringify(await page.evaluate((ids) => {
  const box = (el) => { const r = el?.getBoundingClientRect(); return r === undefined ? null : { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) }; };
  const toggle = document.querySelector(`[id="${ids[0]}"]`);
  const rect = toggle?.getBoundingClientRect();
  const hit = rect === undefined ? null : document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2);
  return {
    toggleBox: box(toggle),
    elementAtToggleCentre: hit === null ? null : `${hit.tagName}#${hit.id || "-"}[${hit.getAttribute("data-slot") ?? ""}]`,
    coveredBy: hit === null || toggle === null ? null : (toggle.contains(hit) ? "(the toggle itself)" : `${hit.closest("[data-slot]")?.getAttribute("data-slot") ?? "?"}#${hit.closest("[id]")?.id ?? "?"}`),
    panels: [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => el instanceof HTMLElement && el.offsetParent !== null).map((el) => `${el.id} ${JSON.stringify(box(el))}`),
    checkin: document.querySelector("#s-checkin")?.textContent ?? null,
  };
}, toggles), null, 1));
for (const toggle of toggles) {
  const outcome = await page.locator(`[id="${toggle}"]`).first().click({ force: true, timeout: 8000 }).then(() => "ok").catch((e) => String(e).split("\n")[0]);
  lines.push(`toggle press with panels open: ${toggle} → ${outcome}`);
  await page.waitForTimeout(2500);
}
lines.push(`actions with panels open: ${JSON.stringify(await page.evaluate(() => ({ actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].length, folded: document.querySelector('[data-slot="window-engagement-overlay"]')?.getAttribute("data-folded"), checkin: document.querySelector("#s-checkin")?.textContent ?? null })))}`);
lines.push(JSON.stringify(await page.evaluate(() => ({
  actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))],
  engagementHtml: (document.querySelector('[id$=".engagement"]')?.innerHTML ?? "(none)").slice(0, 4000),
  checkin: document.querySelector("#s-checkin")?.textContent ?? null,
  allPanelIds: [...document.querySelectorAll('[data-slot="panel"], [data-slot="panel-tab-button"]')].map((el) => el.id),
})), null, 1));

lines.push("\n## every id on the page (first 400)");
lines.push(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[id]")].map((el) => el.id).slice(0, 400)), null, 1));

writeFileSync(OUT, lines.join("\n"));
console.log("written", OUT);
await browser.close();
