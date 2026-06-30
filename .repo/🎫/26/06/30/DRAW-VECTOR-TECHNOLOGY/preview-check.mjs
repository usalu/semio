import { chromium } from "playwright";

const url = "http://127.0.0.1:6064/";
const logs = [];
const errors = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => errors.push(String(err)));

try {
	await page.goto(url, { waitUntil: "networkidle", timeout: 90_000 });
	await page.waitForTimeout(5000);

	const fixtureSelect = page.locator("#playground\\.navbar\\.fixture");
	if (await fixtureSelect.count()) {
		await fixtureSelect.selectOption("semio");
		await page.waitForTimeout(5000);
	}

	const bodyText = await page.locator("body").innerText();
	const unsupportedTexts = await page.getByText("Unsupported UiNode").allTextContents();
	const unsupportedSurface = await page.getByText(/Unsupported .* surface/).allTextContents();
	const unsupported = unsupportedTexts.length + unsupportedSurface.length;
	const title = await page.title();
	const debugFixture = logs.some((line) => line.includes("[DEBUG] draw fixture loaded"));
	const debugBoolean = logs.some((line) => line.includes("[DEBUG] draw boolean resolved"));
	const debugTrace = logs.some((line) => line.includes("[DEBUG] draw trace resolved"));

	console.log(
		JSON.stringify(
			{
				title,
				unsupported,
				unsupportedTexts,
				unsupportedSurface,
				pathCount: await page.locator("svg path").count(),
				errors,
				debugFixture,
				debugBoolean,
				debugTrace,
				hasSemio: bodyText.includes("Semio") || bodyText.includes("Emblem"),
				logs: logs.filter((l) => l.includes("DEBUG") || l.toLowerCase().includes("error")),
			},
			null,
			2,
		),
	);

	process.exitCode = errors.length || unsupported > 0 ? 1 : 0;
} finally {
	await browser.close();
}
