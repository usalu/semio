console.log("[DEBUG] smoke-start");
const { chromium } = await import("playwright");
console.log("[DEBUG] playwright-imported");
const browser = await chromium.launch();
console.log("[DEBUG] browser-launched");
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const messages: string[] = [];
page.on("console", (msg) => messages.push(`${msg.type()}: ${msg.text()}`));
page.on("pageerror", (err) => messages.push(`pageerror: ${err.message}`));
await page.goto("http://127.0.0.1:4213/", { waitUntil: "domcontentloaded" });
await page.getByText("Spatial play").waitFor({ timeout: 10_000 });
await page.getByPlaceholder("Filter or type an interaction…").focus();
const rows = await page.locator("aside button").evaluateAll((buttons) =>
	buttons.map((button) => (button.textContent ?? "").replace(/\s+/g, " ").trim()).filter(Boolean),
);
console.log(`[DEBUG] rows=${JSON.stringify(rows.slice(0, 8))}`);
console.log(`[DEBUG] console=${JSON.stringify(messages)}`);
await browser.close();
