#!/usr/bin/env bun
/** 🔍 Live browser verification for puzzle 3d brush/fill/example dedupe. */
import { chromium, type Page } from "playwright";

const BASE_URL = process.env.PUZZLE_3D_URL ?? "http://127.0.0.1:6013/";

async function waitForPuzzle3d(page: Page): Promise<void> {
	await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: 120_000 });
	await page.waitForSelector('[data-slot="app-name"]', { timeout: 120_000 });
	const exampleSelect = page.locator('select, [role="combobox"]').filter({ hasText: /Concrete Forest|Empty/i }).first();
	await exampleSelect.waitFor({ timeout: 60_000 });
}

async function selectConcreteForest(page: Page): Promise<void> {
	const trigger = page.getByRole("combobox").first();
	if (await trigger.isVisible().catch(() => false)) {
		await trigger.click();
		await page.getByRole("option", { name: "Concrete Forest" }).click();
	} else {
		const select = page.locator("select").first();
		await select.selectOption({ label: "Concrete Forest" });
	}
	await page.waitForTimeout(8_000);
}

async function main(): Promise<void> {
	const browser = await chromium.launch({ headless: true });
	const page = await browser.newPage();
	const logs: string[] = [];
	page.on("console", (msg) => {
		if (msg.type() === "error") logs.push(`[console.error] ${msg.text()}`);
	});

	await waitForPuzzle3d(page);
	const exampleLabels = await page.locator("select option, [role='option']").allTextContents();
	const concreteCount = exampleLabels.filter((label) => label.includes("Concrete Forest")).length;
	if (concreteCount > 1) {
		throw new Error(`[DEBUG] duplicate Concrete Forest entries: ${concreteCount}`);
	}
	console.log("[DEBUG] example dedupe ok:", exampleLabels.filter(Boolean).join(", "));

	await selectConcreteForest(page);
	await page.waitForSelector("canvas", { timeout: 60_000 });

	const fillButton = page.getByRole("button", { name: /fill/i }).first();
	await fillButton.click({ timeout: 15_000 });
	await page.waitForTimeout(1_000);

	const slider = page.getByRole("slider").first();
	await slider.focus();
	for (let step = 0; step < 5; step += 1) {
		await page.keyboard.press("ArrowRight");
	}
	await page.waitForTimeout(2_000);
	const sliderValue = await slider.getAttribute("aria-valuenow");
	console.log("[DEBUG] fill slider aria-valuenow:", sliderValue);
	if (!sliderValue || Number(sliderValue) <= 0) {
		throw new Error(`[DEBUG] fill slider stuck at ${sliderValue ?? "null"}`);
	}

	const brushButton = page.getByRole("button", { name: /brush/i }).first();
	await brushButton.click({ timeout: 15_000 });
	await page.waitForTimeout(1_500);

	const canvas = page.locator("canvas").first();
	const box = await canvas.boundingBox();
	if (!box) throw new Error("[DEBUG] canvas missing bounding box");
	await page.mouse.move(box.x + box.width * 0.52, box.y + box.height * 0.48);
	await page.waitForTimeout(2_000);
	console.log("[DEBUG] brush hover gesture sent");

	const duplicateKeyErrors = logs.filter((line) => line.includes("duplicate key") || line.includes("Encountered two children"));
	if (duplicateKeyErrors.length > 0) {
		throw new Error(`[DEBUG] react duplicate key errors: ${duplicateKeyErrors.join("; ")}`);
	}

	console.log("[DEBUG] browser-verify passed");
	await browser.close();
}

main().catch((error) => {
	console.error(error);
	process.exit(1);
});
