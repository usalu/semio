#!/usr/bin/env bun
/** 🔎️ G10 probe: does the `s` Space app list a hub space's documents and offer the creatable kinds?
 * usage: bun g10-space-app-probe.ts <shellOrigin> <spaceId> <capture> */
import { writeFileSync } from "node:fs";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const [SHELL, SPACE, CAPTURE] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines: string[] = [];
const t0 = Date.now();
page.on("console", (m) => lines.push(`${Date.now() - t0}ms ${m.type()} ${m.text().slice(0, 500)}`));
page.on("response", (r) => { if (/artifact-creations|\/documents|directory/.test(r.url())) lines.push(`${Date.now() - t0}ms response ${r.status()} ${r.request().method()} ${r.url().replace(/^https?:\/\/[^/]+/, "").slice(0, 160)}`); });
page.on("websocket", (ws) => lines.push(`${Date.now() - t0}ms ws ${ws.url().replace(/^wss?:\/\/[^/]+/, "").slice(0, 160)}`));
const out: Record<string, unknown> = {};
try {
  await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.locator('input[type="email"]').fill(process.env.G10_HUMAN_EMAIL ?? "user2@semio.dev");
  await form.locator('input[type="password"]').fill(process.env.G10_HUMAN_PASSWORD ?? "gm1-local-dev-pass-2");
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator(`[data-ui-node-key="space:${SPACE}"]`).first().waitFor({ state: "attached", timeout: 120_000 });
  out.homeRows = await page.locator('[data-ui-node-key^="space:"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-ui-node-key")));
  await page.goto(`${SHELL}/spaces/${SPACE}`, { waitUntil: "domcontentloaded" });
  await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.waitForTimeout(30_000);
  out.artifactRows = await page.locator('[data-ui-node-key^="artifact:"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-ui-node-key")));
  out.tableText = (await page.locator('[data-slot="window"]').first().innerText().catch(() => "")).slice(0, 400);
  const create = page.locator('[data-ui-node-key="s-space-create-artifact"]').first();
  await create.focus();
  await create.press("Enter");
  await page.waitForTimeout(15_000);
  out.dialog = (await page.locator('[role="dialog"]').first().innerText().catch(() => "")).slice(0, 600);
  out.kindChoiceDisabled = await page.locator('[id="kindChoice"]').first().isDisabled().catch(() => null);
  await page.screenshot({ path: CAPTURE.replace(/\.txt$/u, ".png") });
} catch (error) {
  out.error = String(error).slice(0, 400);
}
writeFileSync(CAPTURE, `${JSON.stringify(out, null, 2)}\n# console\n${lines.filter((l) => !/status of 404|DevTools|vite\]/.test(l)).join("\n")}`);
console.log(JSON.stringify(out, null, 2));
await browser.close();
