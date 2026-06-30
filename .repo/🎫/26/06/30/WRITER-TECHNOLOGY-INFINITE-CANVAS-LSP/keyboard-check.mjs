/** @emoji ⌨️ Keyboard/caret diagnostic for writer play (dev server on 6062). */
import { chromium } from "playwright";

const url = process.env.WRITER_PLAY_URL ?? "http://127.0.0.1:6062/";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const logs = [];
page.on("console", (msg) => logs.push(msg.text()));

try {
	await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
	await page.waitForSelector("canvas", { timeout: 60_000 });
	await page.waitForTimeout(4000);

	const canvas = page.locator("canvas").first();
	const box = await canvas.boundingBox();
	if (!box) throw new Error("no canvas box");
	await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
	await page.waitForTimeout(200);

	const beforeFocus = await page.evaluate(() => ({
		active: document.activeElement?.tagName ?? "none",
		activeClass: document.activeElement?.className ?? "",
		textareaCount: document.querySelectorAll("textarea").length,
	}));

	await page.keyboard.press("End");
	await page.waitForTimeout(300);
	await page.keyboard.type("X");
	await page.waitForTimeout(800);

	const afterType = await page.evaluate(() => ({
		active: document.activeElement?.tagName ?? "none",
		textareaValue: document.querySelector("textarea")?.value ?? "",
		textLength: document.querySelector("textarea")?.value?.length ?? 0,
	}));

	await page.keyboard.down("Shift");
	await page.keyboard.press("ArrowLeft");
	await page.keyboard.press("ArrowLeft");
	await page.keyboard.up("Shift");
	await page.waitForTimeout(500);

	const afterSelect = await page.evaluate(() => {
		const ta = document.querySelector("textarea");
		return {
			selectionStart: ta?.selectionStart ?? -1,
			selectionEnd: ta?.selectionEnd ?? -1,
			value: ta?.value ?? "",
		};
	});

	console.log("[DEBUG] keyboard-check", { beforeFocus, afterType, afterSelect, logs: logs.slice(-8) });
	if (!afterType.textareaValue.includes("X")) {
		throw new Error(`typing failed: textarea=${JSON.stringify(afterType.textareaValue)}`);
	}
	if (afterType.textLength < 20) {
		throw new Error(`document truncated after type: length=${afterType.textLength}`);
	}
	if (afterSelect.selectionStart === afterSelect.selectionEnd) {
		throw new Error(`selection failed: ${JSON.stringify(afterSelect)}`);
	}
} finally {
	await browser.close();
}
