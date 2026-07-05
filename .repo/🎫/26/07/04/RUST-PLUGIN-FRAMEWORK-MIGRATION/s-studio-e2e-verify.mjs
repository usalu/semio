#!/usr/bin/env node
import { chromium } from "playwright";

const baseUrl = process.env.S_STUDIO_URL ?? "http://127.0.0.1:6070/";
const timeoutMs = Number(process.env.S_STUDIO_E2E_TIMEOUT_MS ?? 300_000);

function assert(condition, message) {
	if (!condition) throw new Error(message);
}

const HEADLESS_GPU_ERROR_FRAGMENTS = ["NoCompatibleDevice"];

function isIgnorablePageError(message) {
	return HEADLESS_GPU_ERROR_FRAGMENTS.some((fragment) => message.includes(fragment));
}

function relevantPageErrors(errors) {
	return errors.filter((message) => !isIgnorablePageError(message));
}

async function waitFor(page, predicate, label, deadline) {
	while (Date.now() < deadline) {
		const text = await page.locator("body").innerText().catch(() => "");
		const children = await page.locator("#root *").count();
		if (predicate({ text, children })) return { text, children };
		await page.waitForTimeout(500);
	}
	throw new Error(`timeout waiting for ${label}`);
}

async function openStudio(page, deadline) {
	await page.keyboard.press("Meta+n");
	return waitFor(
		page,
		({ text }) => /Catalogue/i.test(text) && /Parameters/i.test(text) && /\/studios\//.test(text),
		"studio workspace",
		deadline,
	);
}

async function activateMediaGraphWindow(page) {
	await page.locator(".semio-flow-canvas-host").first().click({ force: true });
	await page.waitForTimeout(200);
}

async function expandMediaGraphEngagement(page) {
	await activateMediaGraphWindow(page);
	await page.evaluate(() => document.getElementById("s-media-graph-window-engagement-toggle")?.click());
	await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnDrawFromEngagement(page) {
	await expandMediaGraphEngagement(page);
	const engagementInput = page.locator("#s-media-catalogue-hint");
	await engagementInput.fill("draw draw");
	await engagementInput.press("Enter");
	await page.waitForTimeout(1500);
	return "engagement";
}

async function openCommandPalette(page) {
	await page.locator(".semio-flow-canvas-host").first().click({ force: true });
	await page.waitForTimeout(100);
	await page.keyboard.press("Meta+p");
	await page.waitForSelector("[role='dialog'] [data-slot='command-input']", { timeout: 10_000 });
}

async function spawnDrawFromPalette(page) {
	await openCommandPalette(page);
	const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
	await paletteInput.fill("draw");
	await page.waitForTimeout(400);
	const drawSpawn = page.locator("[cmdk-item]").filter({ hasText: /Spawn Draw/i }).first();
	if (await drawSpawn.count()) {
		await drawSpawn.click();
		return "palette";
	}
	await page.keyboard.press("Escape");
	return null;
}

async function main() {
	const browser = await chromium.launch({ headless: true });
	const page = await browser.newPage();
	const pageErrors = [];
	page.on("pageerror", (err) => pageErrors.push(String(err)));

	console.log(`[DEBUG] navigating to ${baseUrl}`);
	await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
	await page.waitForFunction(
		() => document.body.innerText.includes("Home") && document.querySelectorAll("#root *").length > 200,
		{ timeout: 120_000 },
	);

	const deadline = Date.now() + timeoutMs;
	const booted = await waitFor(
		page,
		({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text) && /Demo Studio|New Studio/i.test(text),
		"home shell with studios",
		deadline,
	);
	console.log(`[DEBUG] home loaded (${booted.children} nodes)`);
	assert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

	await openStudio(page, deadline);
	const pathAfterCreate = await page.evaluate(() => location.pathname);
	console.log(`[DEBUG] studio loaded at ${pathAfterCreate}`);
	assert(pathAfterCreate.startsWith("/studios/"), "studio uri should be under /studios/");

	await page.waitForFunction(() => document.querySelector(".semio-flow-canvas-host") != null, { timeout: 30_000 });

	const bodyText = await page.locator("body").innerText();
	assert(!/Missing window:/i.test(bodyText), "all studio windows should render");
	assert((await page.locator(".semio-flow-canvas-host").count()) > 0, "flow canvas host should render");
	assert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");
	console.log("[DEBUG] three studio windows rendered");

	let spawnMode = null;
	try {
		spawnMode = await spawnDrawFromEngagement(page);
		console.log(`[DEBUG] spawn via ${spawnMode}`);
	} catch {
		spawnMode = await spawnDrawFromPalette(page);
		assert(spawnMode === "palette", "draw spawn should work via engagement rail or command palette");
		console.log(`[DEBUG] spawn via ${spawnMode}`);
	}

	await page.keyboard.press("Meta+z");
	await page.waitForTimeout(1500);
	console.log("[DEBUG] undo issued");

	await openCommandPalette(page);
	const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
	await paletteInput.fill("undo");
	await page.waitForTimeout(300);
	assert((await page.locator("[cmdk-item]").filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");
	await paletteInput.fill("checkpoint");
	await page.waitForTimeout(300);
	assert((await page.locator("[cmdk-item]").filter({ hasText: /commitCheckpoint/ }).count()) > 0, "checkpoint command should be in command palette");
	console.log("[DEBUG] studio commands in palette");
	await page.keyboard.press("Escape");

	await page.keyboard.press("Meta+f");
	await page.waitForTimeout(500);
	await page.locator("[id='ui.find.toggle']").first().click({ force: true });
	await page.waitForTimeout(500);
	assert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");
	console.log("[DEBUG] find palette available");
	await page.keyboard.press("Escape");

	await page.locator('[data-slot="breadcrumb-link"]', { hasText: "Home" }).first().click({ force: true });
	await waitFor(page, ({ text }) => text.includes("Demo Studio") || text.includes("New Studio"), "home via breadcrumb", deadline);
	console.log("[DEBUG] breadcrumb home navigation works");

	const demoStudio = page.getByText(/^Demo Studio$/i).first();
	if (await demoStudio.count()) {
		await demoStudio.dblclick();
		await page.waitForTimeout(3000);
		assert((await page.evaluate(() => location.pathname)).startsWith("/studios/"), "vfs open studio should navigate");
		assert(/Catalogue/i.test(await page.locator("body").innerText()), "opened studio from home vfs");
		console.log("[DEBUG] home vfs open studio works");
	}

	const criticalErrors = relevantPageErrors(pageErrors);
	if (criticalErrors.length !== pageErrors.length) {
		console.log(`[DEBUG] ignored headless gpu errors: ${pageErrors.filter(isIgnorablePageError).join(" | ")}`);
	}
	assert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);

	await browser.close();
	console.log("PASS: S studio end-to-end workflows verified");
}

main().catch((error) => {
	console.error("FAIL:", error.message ?? error);
	process.exit(1);
});
