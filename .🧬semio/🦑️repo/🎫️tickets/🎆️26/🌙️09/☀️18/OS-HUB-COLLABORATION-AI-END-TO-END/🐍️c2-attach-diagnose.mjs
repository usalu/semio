/** 🔗️ C2 — one signed-in human attaching ONE hub document through the sync card, step by step.
 * Usage: bun 🐍️c2-attach-diagnose.mjs <shellUrl> <hubHostPort> <spaceId> <documentId> */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6191";
const HUB = process.argv[3] ?? "127.0.0.1:7611";
const SPACE = process.argv[4] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const DOCUMENT = process.argv[5] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (message) => console.log(`[${message.type()}] ${message.text().slice(0, 400)}`));
page.on("pageerror", (error) => console.log(`[pageerror] ${String(error)}\n${(error && error.stack) ? String(error.stack).slice(0, 2000) : ""}`));
page.on("websocket", (ws) => console.log(`[ws] ${ws.url()}`));
page.on("response", (response) => {
  const url = response.url();
  if (url.includes("open-plan") || url.includes("/documents/") || url.includes("/spaces/")) console.log(`[response] ${response.request().method()} ${url} — ${response.status()}`);
});

await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let attempt = 0; attempt < 180; attempt += 1) {
  await page.waitForTimeout(1_000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break;
}
await page.waitForTimeout(3_000);

await page.locator('[data-semio-hub-sign-in=""]').first().click();
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 30_000 });
await form.locator('input[type="email"]').fill("user1@semio.dev");
await form.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
await form.locator('button[type="submit"][aria-label="Sign in"]').click();
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
await page.waitForTimeout(2_000);
console.log("signed in");

const click = async (selector) =>
  (await page.locator(selector).count())
    ? page.locator(selector).first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90))
    : "absent";
for (let attempt = 0; attempt < 6; attempt += 1) {
  if (await page.locator('[id="framework.sync.remote.path"]').count()) break;
  if (await page.locator('[id="framework.sync.remote"]').count()) console.log(`remote:${await click('[id="framework.sync.remote"]')}`);
  else if (await page.locator('[id="ui.utilities.group.sync"]').count()) console.log(`group:${await click('[id="ui.utilities.group.sync"]')}`);
  else console.log(`tab:${await click('[id="s-sync-status"]')}`);
  await page.waitForTimeout(1_500);
}
const input = page.locator('[id="framework.sync.remote.path"]');
console.log(`input count=${await input.count()}`);
await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
await page.waitForTimeout(600);
console.log(`input value=${JSON.stringify(await input.inputValue())}`);

const attach = page.locator('[id="framework.sync.remote.path"]').locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
console.log(`scoped attach count=${await attach.count()}`);
if (await attach.count()) await attach.first().click({ force: true });
for (let tick = 0; tick < 12; tick += 1) {
  await page.waitForTimeout(5_000);
  const state = await page.evaluate(() => ({
    pill: (document.querySelector('[id="s-sync-status"]')?.textContent ?? "").trim(),
    sync: (document.querySelector("[data-semio-sync-status]")?.textContent ?? "").trim(),
    windows: [...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")),
    executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((el) => `${el.getAttribute("data-semio-execution-target-status")}=${(el.textContent ?? "").trim().slice(0, 90)}`),
  }));
  console.log(`tick ${(tick + 1) * 10}s ${JSON.stringify(state)}`);
}

console.log(
  `AFTER ${JSON.stringify(
    await page.evaluate(() => ({
      syncStatus: (document.querySelector("[data-semio-sync-status]")?.textContent ?? "").trim(),
      pill: (document.querySelector('[id="s-sync-status"]')?.textContent ?? "").trim(),
      activeUri: (document.querySelector('[data-slot="popover-content"]')?.textContent ?? "").trim().slice(0, 200),
    })),
  )}`,
);
await page.screenshot({ path: `${OUT}c2-attach-diagnose.png` });
await browser.close();
