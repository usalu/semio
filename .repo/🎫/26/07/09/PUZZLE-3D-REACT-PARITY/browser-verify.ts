#!/usr/bin/env bun
/** 🔍 Live browser verification for puzzle 3d brush/fill/example dedupe. */
import { chromium, type Page } from "playwright";

const BASE_URL = process.env.PUZZLE_3D_URL ?? "http://127.0.0.1:6013/";

async function waitForPuzzle3d(page: Page): Promise<void> {
	await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: 120_000 });
	await page.waitForSelector('[data-slot="app-name"]', { timeout: 120_000 });
	await page.waitForFunction(() => document.querySelector('[data-slot="app-name"]')?.textContent?.includes("puzzle"), undefined, {
		timeout: 120_000,
	});
}

async function selectConcreteForest(page: Page): Promise<void> {
	const trigger = page.locator("#playground\\.navbar\\.fixture\\.trigger");
	await trigger.waitFor({ timeout: 30_000 });
	await trigger.click();
	await page.getByRole("option", { name: "Concrete Forest" }).click();
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
	await page.locator("#playground\\.navbar\\.fixture\\.trigger").waitFor({ timeout: 30_000 });
	const exampleLabels = await page.locator('[role="option"]').allTextContents();
	const concreteCount = exampleLabels.filter((label) => label.includes("Concrete Forest")).length;
	if (concreteCount > 1) {
		throw new Error(`[DEBUG] duplicate Concrete Forest entries: ${concreteCount}`);
	}
	console.log("[DEBUG] example dedupe ok:", exampleLabels.filter(Boolean).join(", "));

	await selectConcreteForest(page);
	await page.waitForSelector("canvas", { timeout: 60_000 });

	const fillButton = page.locator("#puzzle3d\\.tool\\.fill");
	await fillButton.waitFor({ timeout: 30_000 });
	await fillButton.click();
	await page.waitForTimeout(1_500);
	const engagementHtml = await page.locator('[data-slot="engagement"]').first().innerHTML();
	console.log("[DEBUG] engagement after fill click contains slider:", engagementHtml.includes("data-control-kind=\"slider\""));
	await page.waitForSelector('[data-control-kind="slider"]', { timeout: 15_000 });

	const slider = page.locator('[data-control-kind="slider"] [role="slider"]').first();
	await slider.focus();
	for (let step = 0; step < 8; step += 1) {
		await page.keyboard.press("ArrowRight");
	}
	await page.waitForTimeout(2_000);
	const sliderValue = await slider.getAttribute("aria-valuenow");
	console.log("[DEBUG] fill slider aria-valuenow:", sliderValue);
	if (!sliderValue || Number(sliderValue) <= 0) {
		throw new Error(`[DEBUG] fill slider stuck at ${sliderValue ?? "null"}`);
	}

	const brushButton = page.locator('[data-slot="engagement"] button', { hasText: "Brush" }).first();
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
