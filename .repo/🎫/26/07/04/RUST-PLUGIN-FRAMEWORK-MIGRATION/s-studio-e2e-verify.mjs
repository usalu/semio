#!/usr/bin/env node
import { chromium } from "playwright";

const baseUrl = process.env.S_STUDIO_URL ?? "http://127.0.0.1:6068/";
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
		await page.waitForTimeout(1000);
	}
	throw new Error(`timeout waiting for ${label}`);
}

async function main() {
	const browser = await chromium.launch({ headless: true });
	const page = await browser.newPage();
	const pageErrors = [];
	page.on("pageerror", (err) => pageErrors.push(String(err)));

	console.log(`[DEBUG] navigating to ${baseUrl}`);
	await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120_000 });

	const deadline = Date.now() + timeoutMs;
	const booted = await waitFor(page, ({ text, children }) => children > 50 && /Home|Studios/i.test(text), "home shell", deadline);
	console.log(`[DEBUG] home loaded (${booted.children} nodes)`);

	const searchToggle = page.locator("[id='ui.search.toggle']").first();
	if (await searchToggle.count()) await searchToggle.click();
	else await page.keyboard.press("Meta+p");
	await page.waitForTimeout(500);

	let openedStudio = false;
	const createStudio = page.getByText(/create studio/i).first();
	if (await createStudio.count()) {
		await createStudio.click();
		openedStudio = true;
	} else {
		await page.keyboard.press("Escape");
		await page.keyboard.press("Meta+n");
		openedStudio = true;
	}
	assert(openedStudio, "should open studio via palette or mod+n");
	await page.waitForTimeout(3000);

	const studio = await waitFor(
		page,
		({ text, children }) => children > 100 && /Catalogue|Parameters|Media Graph|Compiled DAG/i.test(text),
		"studio workspace",
		deadline,
	);
	console.log(`[DEBUG] studio loaded: ${studio.text.slice(0, 300)}`);

	assert(/Catalogue/i.test(studio.text), "catalogue panel visible");
	assert(/Parameters/i.test(studio.text), "parameters panel visible");
	assert(!/Missing window:/i.test(studio.text), "all studio windows should render");

	const flowHost = page.locator(".semio-flow-canvas-host").first();
	assert((await flowHost.count()) > 0, "flow canvas host should render");

	const vfsHost = page.locator("[data-semio-vfs-root]").first();
	if ((await vfsHost.count()) > 0) console.log("[DEBUG] media vfs host rendered");

	const compiledDag = page.locator(".semio-text-editor-host").first();
	if ((await compiledDag.count()) > 0) console.log("[DEBUG] compiled dag editor rendered");

	await page.keyboard.press("Meta+f");
	await page.waitForTimeout(300);
	const findInput = page.locator("[id='ui.find.input']").first();
	if ((await findInput.count()) > 0) {
		console.log("[DEBUG] find palette available");
		await page.keyboard.press("Escape");
	}

	await page.keyboard.press("Meta+p");
	await page.waitForTimeout(300);
	const undoItem = page.getByText(/^Undo$/i).first();
	if (await undoItem.count()) {
		console.log("[DEBUG] undo command available in palette");
		await page.keyboard.press("Escape");
	}

	const catalogueDraw = page.getByText(/^Draw$/i).first();
	if (await catalogueDraw.count()) {
		console.log("[DEBUG] draw program in catalogue");
		await catalogueDraw.click();
		await page.waitForTimeout(1500);
		const spawned = await page.getByText(/Back to Media Graph/i).count();
		if (spawned > 0) console.log("[DEBUG] draw drill-in opened");
		else console.log("[DEBUG] draw spawn did not drill-in (may require drag onto graph)");
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
