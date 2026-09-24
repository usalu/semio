#!/usr/bin/env bun
/** 🔭️ G10 probe: a human signs in inside a `note` window and attaches a hub note through the footer sync card; every
 * hub request, toast and the sync pill are recorded step by step. usage: bun g10-remote-attach-probe.ts <shell> <hub> <space> <document> [out] */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";

const [SHELL, HUB, SPACE, DOCUMENT, OUT = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-g10-logs"] = process.argv.slice(2);
const EMAIL = process.env.G10_HUMAN_EMAIL ?? "user2@semio.dev";
const PASSWORD = process.env.G10_HUMAN_PASSWORD ?? "gm1-local-dev-pass-2";
const log: string[] = [];
const note = (line: string) => {
  log.push(`${new Date().toISOString().slice(11, 23)} ${line}`);
  console.log(line);
};
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (message) => {
  const text = message.text();
  if (message.type() !== "debug" && !text.includes("[vite]") && !text.startsWith("Failed to load resource")) note(`console.${message.type()} ${text.slice(0, 300)}`);
});
page.on("response", (response) => {
  if (response.url().startsWith(HUB) || response.status() >= 400) note(`http ${response.status()} ${response.request().method()} ${response.url().replace(HUB, "<hub>").slice(0, 200)}`);
});
page.on("websocket", (socket) => {
  note(`ws open ${socket.url().replace(HUB.replace(/^http/u, "ws"), "<hub>").slice(0, 200)}`);
  socket.on("close", () => note(`ws close ${socket.url().slice(0, 120)}`));
});
const toasts = async () => (await page.locator('[data-sonner-toast], [role="status"], [role="alert"]').allInnerTexts().catch(() => [])).map((text) => text.replace(/\s+/gu, " ").slice(0, 200));
const footer = async () => (await page.locator('[id="s-sync-status"]').first().innerText().catch(() => "")).replace(/\s+/gu, " ");
try {
  await page.goto(`${SHELL}/?plugin=note`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") !== null, undefined, { timeout: 300_000 });
  note(`ready; sign-in buttons=${await page.locator('[data-semio-hub-sign-in=""]').count()} footer="${await footer()}"`);
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(EMAIL);
  await form.locator('input[type="password"]').fill(PASSWORD);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  note(`signed in; toasts=${JSON.stringify(await toasts())}`);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await form.waitFor({ state: "hidden", timeout: 15_000 });
  await page.waitForTimeout(3_000);
  note(`after sign-in settle: sign-in buttons=${await page.locator('[data-semio-hub-sign-in=""]').count()} footer="${await footer()}" toasts=${JSON.stringify(await toasts())}`);
  for (let attempt = 0; attempt < 6 && !(await page.locator('[id="framework.sync.remote.path"]').count()); attempt += 1) {
    for (const selector of ['[id="framework.sync.remote"]', '[id="ui.utilities.group.sync"]', '[id="s-sync-status"]']) {
      if (await page.locator(selector).count()) {
        await page.locator(selector).first().click({ force: true, timeout: 8_000 }).catch(() => undefined);
        note(`clicked ${selector}`);
        break;
      }
    }
    await page.waitForTimeout(1_500);
  }
  const input = page.locator('[id="framework.sync.remote.path"]');
  const path = `${HUB.replace(/^https?:\/\//u, "")}/${SPACE}/${DOCUMENT}`;
  await input.fill(path);
  note(`filled ${path}; placeholder="${await input.getAttribute("placeholder")}"`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  note(`attach buttons=${await attach.count()} disabled=${await attach.first().isDisabled().catch(() => "?")}`);
  await attach.first().click({ force: true, timeout: 8_000 });
  for (let tick = 0; tick < 30; tick += 1) {
    await page.waitForTimeout(2_000);
    note(`t+${(tick + 1) * 2}s footer="${await footer()}" toasts=${JSON.stringify(await toasts())} sign-in=${await page.locator('[data-semio-hub-sign-in=""]').count()}`);
  }
  await page.screenshot({ path: join(OUT, "g10-remote-attach-probe.png") });
} catch (error) {
  note(`ERROR ${error instanceof Error ? error.stack : String(error)}`);
  await page.screenshot({ path: join(OUT, "g10-remote-attach-probe-error.png") }).catch(() => undefined);
} finally {
  writeFileSync(join(OUT, "g10-remote-attach-probe.txt"), log.join("\n"));
  await browser.close();
}
