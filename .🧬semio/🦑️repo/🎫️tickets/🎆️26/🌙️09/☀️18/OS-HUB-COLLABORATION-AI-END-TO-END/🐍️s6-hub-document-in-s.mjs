#!/usr/bin/env bun
/** 🔬️ S6 — a HUB document opened and edited inside the real `s` host (ticket 26/09/18).
 *
 * S5 §7 stated the boundary it deliberately did not cross: everything in its per-kind matrix is the
 * LOCAL path — an ephemeral studio minted by `createStudio`, foreign editors spawned into it, no hub
 * document anywhere — because creating an artifact inside a hub space answered `503
 * catalog-unavailable` from `open-plan` until a trusted catalog was published. GM1 published one
 * (`📓️gm1-gis-cold-load-law-and-publish.md` §0: hub 7611 `/readyz` ready, `artifactAuthority.ready`,
 * `features.openPlan`, a gis map document already in a space). So the boundary can be crossed now, and
 * this is the probe that crosses it:
 *
 *   sign in on a shell bound to 7611 → the hub workspace → enter the space that holds the document →
 *   open THAT document through the hub's open plan → it owns windows in the `s` canvas → edit it.
 *
 * Every step records what it saw, so a run that stops halfway still says exactly where.
 *
 * Usage: bun 🐍️s6-hub-document-in-s.mjs <baseUrl> [spaceId]
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6092/";
const spaceId = process.argv[3] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const log = (...parts) => console.log("[s6hub]", ...parts);

const shot = async (page, name) => page.screenshot({ path: `${generated}s6-hub-${name}.png` }).catch(() => undefined);
const windowIds = (page) => page.evaluate(() => [...new Set([...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))].filter((id) => typeof id === "string"));
const surface = (page) =>
  page.evaluate(() => {
    const text = (element) => (element?.innerText ?? "").replace(/\s+/gu, " ").trim();
    return {
      route: window.location.pathname,
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      windowIds: [...new Set([...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))],
      overlay: document.querySelectorAll("[data-semio-hub-workspace]").length,
      signInBadge: document.querySelectorAll('[data-semio-hub-sign-in=""]').length,
      tables: document.querySelectorAll(".semio-table-host").length,
      rows: [...document.querySelectorAll('[role="row"], [data-row-id], [role="treeitem"]')].map((element) => text(element).slice(0, 70)).filter((value) => value.length > 0).slice(0, 30),
      buttons: [...document.querySelectorAll("button")].map((element) => `${element.id}|${text(element).slice(0, 30)}`).filter((value) => value.length > 2).slice(0, 40),
      faults: [...document.querySelectorAll("[data-semio-window-fault]")].map((element) => `${element.getAttribute("data-semio-window-fault-code")}: ${element.getAttribute("data-semio-window-fault")}`),
      body: text(document.body).slice(0, 700),
    };
  });

async function awaitBeacon(page, deadline) {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => (document.documentElement.dataset.semioOsReady !== undefined ? `ready:${document.documentElement.dataset.semioOsReady}` : document.documentElement.dataset.semioOsError !== undefined ? `error:${document.documentElement.dataset.semioOsError}` : null));
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1000);
  }
  return null;
}
async function dismissIntroduction(page) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip/iu }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    await page.waitForTimeout(500);
  }
}
const click = async (page, selector) => {
  const locator = page.locator(selector).first();
  if ((await locator.count()) === 0) return "absent";
  return locator.click({ force: true, timeout: 10_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 70));
};

const browser = await chromium.launch({ headless: process.env.S6_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.setDefaultNavigationTimeout(180_000);
const refusals = [];
const faults = [];
page.on("pageerror", (error) => faults.push(String(error).slice(0, 200)));
page.on("console", (message) => {
  const text = message.text();
  if (/refused:|dropped action|rejected|catalog-unavailable|open-plan|503/iu.test(text)) refusals.push(`${message.type()}: ${text.slice(0, 240)}`);
});
const network = [];
page.on("response", (response) => {
  const url = response.url();
  if (/open-plan|artifact|spaces|auth\/sessions|documents/iu.test(url)) network.push(`${response.status()} ${url.slice(0, 150)}`);
});

const result = { baseUrl, spaceId, steps: [] };
const step = async (name, value) => {
  const view = await surface(page);
  result.steps.push({ name, value, view });
  log(`${name}: ${value ?? ""} | route=${view.route} ready=${view.ready} windows=${JSON.stringify(view.windowIds)} overlay=${view.overlay} tables=${view.tables} faults=${JSON.stringify(view.faults)}`);
  return view;
};

try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await step("boot", result.beacon);

  // 🪪️ Sign in through the shell's own hub badge — the same selectors `🐍️c1c-s-host-probe.mjs` uses.
  const badge = await click(page, '[data-semio-hub-sign-in=""]');
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  await form.locator('input[type="email"]').fill(process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev").catch(() => undefined);
  await form.locator('input[type="password"]').fill(process.env.S6_SIGN_IN_PASSWORD ?? "").catch(() => undefined);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.waitForTimeout(4_000);
  const afterSignIn = await step("sign-in", `badge=${badge}`);
  await shot(page, "signed-in");
  result.signedIn = afterSignIn.signInBadge === 0;

  // 🏘️ The hub workspace lists the spaces this human belongs to. Enter the one that holds the document.
  await click(page, '[data-semio-hub-workspace] button[aria-label]');
  await page.waitForTimeout(4_000);
  const workspace = await step("hub-workspace", null);
  const spaceEntry = page.locator(`[data-space-id="${spaceId}"], [id*="${spaceId}"]`).first();
  let entered = "absent";
  if ((await spaceEntry.count()) > 0) entered = await click(page, `[data-space-id="${spaceId}"], [id*="${spaceId}"]`);
  else {
    // 🧭️ No id-keyed control: the shell's own route is the documented way in (`applyShellUri`'s
    // `/spaces/{id}` branch, which S4 measured reaching the space INDEX).
    await page.goto(`${baseUrl.replace(/\/$/u, "")}/spaces/${spaceId}`, { waitUntil: "commit", timeout: 180_000 }).catch(() => undefined);
    await awaitBeacon(page, Date.now() + 180_000);
    entered = "by-route";
  }
  await page.waitForTimeout(8_000);
  await dismissIntroduction(page);
  const space = await step("enter-space", entered);
  await shot(page, "space");
  result.spaceRows = space.rows;

  // 📄️ The document row. GM1's gis map is `artifact-2fb248125b8b2b4d56de25933d30ed21`, surface
  // `s.gis.gismap@1/*#editor`, window kind `gis2d-main`.
  const before = await windowIds(page);
  let opened = "absent";
  for (const selector of ['[id*="artifact-2fb248125b8b2b4d56de25933d30ed21"]', '[role="row"]:has-text("Map")', '[role="row"]:has-text("map")', '[role="treeitem"]:has-text("Map")', '.semio-table-host [role="row"]']) {
    const locator = page.locator(selector).first();
    if ((await locator.count()) === 0) continue;
    opened = `${selector} → ${await locator.dblclick({ force: true, timeout: 10_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 60))}`;
    await page.waitForTimeout(10_000);
    if ((await windowIds(page)).some((id) => !before.includes(id))) break;
  }
  const afterOpen = await step("open-document", opened);
  await shot(page, "document");
  result.documentWindowIds = afterOpen.windowIds.filter((id) => !before.includes(id));

  if (result.documentWindowIds.length > 0) {
    const toggles = page.locator('[id$=".engagement.toggle"]');
    const toggleCount = await toggles.count();
    for (let index = 0; index < toggleCount; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_500);
    const rows = await page.evaluate(() => [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => element.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./u.test(id)));
    result.railRows = rows;
    const verb = (process.env.S6_HUB_VERB ?? "addFeature").replace(/^action\./u, "");
    const verbRow = rows.includes(`action.${verb}`) ? verb : (rows.find((id) => /^action\.(add|create|insert)/u.test(id)) ?? "").replace(/^action\./u, "");
    if (verbRow) {
      result.editClick = await click(page, `[data-slot="window-action-pane"] [id="action.${verbRow}"]`);
      await page.waitForTimeout(1_500);
      result.editExecute = await click(page, `[data-slot="window-action-pane"] [id$=".action.${verbRow}.execute"]`);
      await page.waitForTimeout(6_000);
      result.editVerb = verbRow;
    }
    await step("edit-document", `${result.editVerb ?? "no verb"} click=${result.editClick ?? "-"} execute=${result.editExecute ?? "-"}`);
    await shot(page, "edited");
  }
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.refusals = [...new Set(refusals)].slice(0, 20);
  result.faults = faults.slice(0, 10);
  result.network = [...new Set(network)].slice(0, 30);
  await browser.close();
}
writeFileSync(`${generated}s6-hub-document.txt`, JSON.stringify(result, null, 2));
log(`network: ${JSON.stringify(result.network.slice(0, 12))}`);
log(`refusals: ${JSON.stringify(result.refusals.slice(0, 5))}`);
log(`documentWindowIds: ${JSON.stringify(result.documentWindowIds ?? [])}`);
process.exit(result.fatal ? 1 : 0);
