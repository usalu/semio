import { chromium } from "playwright";

const url = "http://127.0.0.1:6060/";
const logs = [];
const errors = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => errors.push(String(err)));

try {
	await page.goto(url, { waitUntil: "networkidle", timeout: 60_000 });
	await page.waitForSelector("text=Composite", { timeout: 30_000 });
	await page.waitForTimeout(2000);

	const fixtureSelect = page.locator("#playground\\.navbar\\.fixture");
	if (await fixtureSelect.count()) {
		await fixtureSelect.selectOption("paint");
		await page.waitForTimeout(2000);
	}

	const bodyText = await page.locator("body").innerText();
	const unsupported = (await page.getByText("Unsupported UiNode").count()) + (await page.getByText(/Unsupported .* surface/).count());
	const title = await page.title();
	const debugFixture = logs.some((line) => line.includes("[DEBUG] raster fixture loaded"));
	const debugPaint = logs.some((line) => line.includes("[DEBUG] raster fixture loaded paint"));
	const threeWarnings = logs.filter((line) => line.includes("THREE.WARNING")).length;

	console.log(
		JSON.stringify(
			{
				title,
				unsupported,
				errors,
				debugFixture,
				debugPaint,
				threeWarnings,
				hasComposite: bodyText.includes("Composite"),
				hasPaperLayer: bodyText.includes("Paper") || bodyText.includes("Paint"),
				logs: logs.filter((l) => l.includes("DEBUG") || l.toLowerCase().includes("error") || l.includes("THREE")),
			},
			null,
			2,
		),
	);

	process.exitCode = errors.length || unsupported > 0 ? 1 : 0;
} finally {
	await browser.close();
}
