#!/usr/bin/env node
import { chromium } from "playwright";

const baseUrl = process.env.S_STUDIO_URL ?? "http://127.0.0.1:6068/";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const logs = [];
page.on("console", (m) => logs.push(`[${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => logs.push(`[pageerror] ${e}`));

await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120000 });
await page.waitForTimeout(3000);
await page.keyboard.press("Meta+n");
await page.waitForTimeout(5000);

const diag = await page.evaluate(() => {
	const missing = document.body.innerText.match(/Missing window: [^\n]+/g) ?? [];
	const flow = document.querySelector(".semio-flow-canvas-host");
	const windowKinds = [...document.querySelectorAll("[data-window-kind-id]")].map((el) => el.getAttribute("data-window-kind-id"));
	return {
		missing,
		flowCount: flow ? 1 : 0,
		windowKinds,
		bodySnippet: document.body.innerText.slice(0, 800),
	};
});

console.log(JSON.stringify(diag, null, 2));
console.log("console logs:", logs.filter((l) => l.includes("DEBUG") || l.includes("error") || l.includes("render")).slice(0, 20));
await browser.close();
