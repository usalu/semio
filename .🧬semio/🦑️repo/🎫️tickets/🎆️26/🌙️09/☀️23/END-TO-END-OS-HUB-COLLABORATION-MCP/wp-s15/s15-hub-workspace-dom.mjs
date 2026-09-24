#!/usr/bin/env bun
/** 🔍️ S15 — signs in on the hub workspace inside `s` and dumps its form controls (create-space form discovery). */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const baseUrl = process.argv[2] ?? "http://127.0.0.1:6541/";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 60_000 });
await form.locator('input[type="email"]').fill("user1@semio.dev");
await form.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true });
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
await page.waitForTimeout(8_000);
const dump = await page.evaluate(() => ({
  inputs: [...document.querySelectorAll("[data-semio-hub-workspace] input, [data-semio-hub-workspace] button, [data-semio-hub-workspace] select")].map((el) => `${el.tagName}#${el.id}[alias=${el.getAttribute("data-element-alias")}][aria=${el.getAttribute("aria-label")}][type=${el.getAttribute("type")}] ${(el.textContent ?? "").trim().slice(0, 30)}`).slice(0, 60),
  text: (document.querySelector("[data-semio-hub-workspace]")?.textContent ?? "").replace(/\s+/gu, " ").slice(0, 600),
}));
await page.screenshot({ path: fileURLToPath(new URL("./generated/s15-hub-workspace.png", import.meta.url)) });
await browser.close();
writeFileSync(fileURLToPath(new URL("./generated/s15-hub-workspace-dom.json", import.meta.url)), JSON.stringify(dump, null, 1));
console.log(JSON.stringify(dump, null, 1));
