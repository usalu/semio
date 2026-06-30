/** @emoji 🌐 Browser smoke check for writer play (requires dev server on 6062). */
import { chromium } from "playwright";

const url = process.env.WRITER_PLAY_URL ?? "http://127.0.0.1:6062/";
const debugLogs = [];
const pageErrors = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => {
	const text = msg.text();
	if (text.includes("[DEBUG]")) {
		debugLogs.push(text);
		console.log(text);
	}
});
page.on("pageerror", (err) => pageErrors.push(String(err)));

try {
	await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
	await page.waitForSelector("canvas", { timeout: 60_000, state: "attached" });
	await page.waitForTimeout(3000);

	if (!debugLogs.some((line) => line.includes("writer play surface mount"))) {
		throw new Error("missing writer play surface mount log");
	}

	const textarea = page.locator("textarea").first();
	await textarea.focus();
	await textarea.fill("RETURN x");
	await page.waitForTimeout(2000);

	const hasDiagnostics =
		debugLogs.some((line) => line.includes("lint diagnostics")) ||
		(await page.locator(".text-destructive").count()) > 0;
	if (!hasDiagnostics) {
		console.log("[DEBUG] browser-check note: no diagnostics visible yet", { pageErrors });
	}

	console.log("[DEBUG] browser-check ok", { debugLogCount: debugLogs.length, pageErrors });
} finally {
	await browser.close();
}
