#!/usr/bin/env bun
import { chromium } from "playwright";

const port = process.env.S_OS_PORT ?? "7401";
const plugin = process.argv[2] ?? "draw";
const baseUrl = `http://127.0.0.1:${port}/?plugin=${plugin}`;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (m) => console.log(`[console:${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => console.log(`[pageerror] ${e.message}`));
console.log(`navigating to ${baseUrl}`);
await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(5000);
const bodyText = await page.locator("body").innerText().catch(() => "<no body>");
console.log("=== body innerText (first 2000 chars) ===");
console.log(bodyText.slice(0, 2000));
const navbarCount = await page.locator('[data-slot="navbar"]').count();
const footerCount = await page.locator('[data-slot="footer"]').count();
const appNameText = await page.locator('[data-slot="app-name"]').textContent().catch(() => null);
console.log(`navbarCount=${navbarCount} footerCount=${footerCount} appName=${JSON.stringify(appNameText)}`);
await page.screenshot({ path: "/private/tmp/claude-501/-Users-ueli-Documents-semio/e57c06c1-4105-4486-b074-dcbde9029be5/scratchpad/debug-boot.png" });
await browser.close();
