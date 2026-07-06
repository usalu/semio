#!/usr/bin/env bun
import { chromium } from "playwright";
import { join } from "node:path";

const port = process.env.S_OS_PORT ?? "6288";
const ticketDir = import.meta.dir;
const logs: string[] = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (m) => logs.push(`[${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}\n${e.stack ?? ""}`));

await page.goto(`http://127.0.0.1:${port}/?plugin=lowpoly`, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(5000);

const rootHtml = await page.locator("#root").innerHTML();
const bodyText = await page.locator("body").innerText();
const shot = join(ticketDir, "check-lowpoly-screenshot.png");
await page.screenshot({ path: shot, fullPage: true });

await Bun.write(
	join(ticketDir, "check-lowpoly-console.txt"),
	`${logs.join("\n")}\n\nrootLen=${rootHtml.length}\nrootPreview=${rootHtml.slice(0, 800)}\nbodyText=${bodyText.slice(0, 400)}\n`,
);
console.log({ rootLen: rootHtml.length, errors: logs.filter((l) => l.includes("error") || l.includes("pageerror")), shot });
await browser.close();
