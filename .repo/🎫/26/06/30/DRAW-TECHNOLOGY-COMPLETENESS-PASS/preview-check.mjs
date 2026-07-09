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
  const hasEmblemUnion = (await page.getByText("Emblem Union").count()) > 0;
  let hasBooleanChild = (await page.getByText("Orange Wedge").count()) > 0;
  if (hasEmblemUnion && !hasBooleanChild) {
    await page.getByText("Emblem Union").first().click();
    await page.waitForTimeout(500);
    hasBooleanChild = (await page.getByText("Orange Wedge").count()) > 0;
  }

  const canvas = page.locator(".bg-neutral-950");
  if (await canvas.count()) {
    const box = await canvas.first().boundingBox();
    if (box) {
      await page.keyboard.down("Shift");
      await page.mouse.move(box.x + box.width * 0.2, box.y + box.height * 0.2);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width * 0.8, box.y + box.height * 0.8);
      await page.mouse.up();
      await page.keyboard.up("Shift");
      await page.waitForTimeout(1000);
    }
  }

  const selectionLogs = logs.filter((line) => line.includes("[DEBUG] draw selection"));
  const hasSelection = selectionLogs.length > 0;
  const pathCount = await page.locator("svg path").count();

  console.log(
    JSON.stringify(
      {
        title,
        unsupported,
        unsupportedTexts,
        unsupportedSurface,
        pathCount,
        errors,
        debugFixture,
        debugBoolean,
        debugTrace,
        hasSemio: bodyText.includes("Semio") || bodyText.includes("Emblem"),
        hasBooleanChild,
        hasEmblemUnion,
        hasSelection,
        logs: logs.filter((l) => l.includes("DEBUG") || l.toLowerCase().includes("error")),
      },
      null,
      2,
    ),
  );

  process.exitCode = errors.length || unsupported > 0 || !debugFixture || !debugBoolean || !debugTrace || pathCount < 1 ? 1 : 0;
} finally {
  await browser.close();
}
