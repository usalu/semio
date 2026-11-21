import assert from "node:assert/strict";
import { chromium, devices } from "playwright";

const address = process.argv[2] ?? "http://127.0.0.1:6271";
const browser = globalThis.staffagePhoneBrowser ?? await chromium.launch({
  ...(process.env.STAFFAGE_CHROME_EXECUTABLE === undefined ? { channel: "chromium" } : { executablePath: process.env.STAFFAGE_CHROME_EXECUTABLE }),
});

function observe(page) {
  if (process.env.SEMIO_PHONE_VERIFY_VITE_TRACE !== "true") return;
  page.on("pageerror", error => console.error("[DEBUG] Staffage page error", error.message));
  page.on("console", message => {
    if (message.type() === "error" || message.type() === "warning") console.error("[DEBUG] Staffage browser console", message.text());
  });
  page.on("response", response => {
    if (response.status() >= 400) console.error("[DEBUG] Staffage response", response.status(), decodeURIComponent(response.url()));
  });
  page.on("requestfailed", request => console.error("[DEBUG] Staffage request failed", decodeURIComponent(request.url()), request.failure()));
}
try {
  const context = await browser.newContext({ ...devices["iPhone 13"], locale: "en-US", viewport: { width: 320, height: 640 } });
  await globalThis.routeTicketBrowser?.(context, address);
  const page = await context.newPage();
  observe(page);
  const assetRequests = new Set();
  page.on("request", request => {
    const url = decodeURIComponent(request.url());
    if (url.includes("staffage-asset:")) assetRequests.add(url);
  });
  await page.goto(address, { waitUntil: "domcontentloaded" });
  const overview = page.getByRole("navigation", { name: "Archive overview" });
  const destinations = overview.getByRole("link");
  try {
    await destinations.last().waitFor();
  } catch (error) {
    console.error("[DEBUG] Staffage overview boot state", await page.evaluate(() => ({
      title: document.title,
      language: document.documentElement.lang,
      body: document.body.innerText.slice(0, 300),
      modules: [...document.scripts].map(script => script.src).filter(Boolean),
    })));
    throw error;
  }
  assert.equal(await destinations.count(), 13);
  assert.equal(assetRequests.size, 0);
  const overviewSize = await page.locator('[data-page="overview"]').evaluate(el => ({ height: el.clientHeight, content: el.scrollHeight, width: el.clientWidth, contentWidth: el.scrollWidth }));
  assert.ok(overviewSize.content > overviewSize.height);
  assert.equal(overviewSize.contentWidth, overviewSize.width);
  assert.ok((await destinations.first().boundingBox()).height >= 44);
  await overview.locator('a[href="#/humans"]').tap();
  const humans = page.locator("[data-staffage-asset-list]");
  await humans.waitFor();
  assert.ok(await humans.getByRole("link").count() >= 12);
  assert.equal(assetRequests.size, 0);
  assert.equal(await page.getByRole("searchbox").evaluate(el => getComputedStyle(el).fontSize), "16px");
  assert.ok((await humans.getByRole("button").first().boundingBox()).width >= 44);
  const category = page.locator('[data-page="category:humans"]');
  const walker = humans.locator('a[href="#/humans/walking-commuter"]');
  await walker.scrollIntoViewIfNeeded();
  const scrollBefore = await category.evaluate(el => el.scrollTop);
  assert.ok(scrollBefore > 0);
  await walker.tap();
  await page.locator("[data-viewer] [data-stage]").waitFor();
  assert.equal(assetRequests.size, 1);
  assert.ok([...assetRequests][0].includes("walking-commuter"));
  const play = page.getByRole("button", { name: "Play", exact: true });
  await play.waitFor();
  assert.ok((await play.boundingBox()).height >= 44);
  await play.tap();
  await page.getByRole("button", { name: "Pause", exact: true }).waitFor();

  await page.goBack();
  await humans.waitFor();
  assert.equal(await category.evaluate(el => el.scrollTop), scrollBefore);
  await page.goto(new URL("#/all", address).href, { waitUntil: "domcontentloaded" });
  await page.locator("[data-staffage-asset-list]").waitFor();
  assert.equal(assetRequests.size, 1);
  assert.ok(await page.locator("[data-staffage-asset-list] li").count() > 100);
  await page.getByRole("button", { name: "Filters" }).tap();
  await page.getByRole("combobox", { name: "Sort by" }).waitFor();
  console.log("[DEBUG] Phone archive, authored asset, playback and lists passed", { viewport: "320x640", overviewSize, assetRequests: assetRequests.size });
  await context.close();

  const landscape = await browser.newContext({ ...devices["iPhone 13 landscape"], locale: "fr-FR", viewport: { width: 844, height: 260 } });
  await globalThis.routeTicketBrowser?.(landscape, address);
  const gate = await landscape.newPage();
  observe(gate);
  await gate.goto(address, { waitUntil: "domcontentloaded" });
  const main = gate.locator("[data-staffage-language-gate] main");
  await main.waitFor();
  await gate.locator("html").evaluate(el => { el.style.fontSize = "200%"; });
  assert.ok(await main.evaluate(el => el.scrollHeight > el.clientHeight && getComputedStyle(el).overflowY === "auto"));
  await main.getByRole("button", { name: "Deutsch" }).tap();
  await gate.locator('[data-staffage-phone] [data-page="overview"]').waitFor();
  assert.equal(await gate.locator("html").getAttribute("lang"), "de");
  console.log("[DEBUG] Landscape language selection and localized phone layout passed", { viewport: "844x260" });
  await landscape.close();
} finally {
  await browser.close();
}
