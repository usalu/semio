/** 🌐️ Headless check of the live Staffage dev server: manifest findings, ready counts, console errors, and desktop strip grids. */
import { chromium } from "playwright";

const origin = "http://127.0.0.1:6071/";
const routes = ["#/", "#/all", "#/components", "#/components/cantilever-balcony", "#/components/glulam-beam", "#/components/steel-column", "#/humans", "#/animals", "#/vegetation", "#/furniture", "#/objects", "#/vehicles", "#/machinery", "#/compare", "#/playground", "#/collections"];
const stripRoutes = new Set(["#/", "#/components", "#/humans"]);
const viewports = [{ width: 1440, height: 900 }, { width: 1280, height: 640 }];
const titles = { "#/components/cantilever-balcony": "Cantilever Balcony", "#/components/glulam-beam": "Glulam Beam", "#/components/steel-column": "Steel Column" };

const browser = await chromium.launch({ headless: true });
const report = [];
for (const viewport of viewports) {
  const page = await browser.newPage({ viewport, locale: "en-US" });
  const consoleErrors = [];
  const pageErrors = [];
  page.on("console", (message) => { if (message.type() === "error") consoleErrors.push(message.text()); });
  page.on("pageerror", (error) => pageErrors.push(error.message));
  for (const route of viewport.width === 1440 ? routes : [...stripRoutes]) {
    consoleErrors.length = 0;
    pageErrors.length = 0;
    await page.goto(`${origin}${route}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    const english = page.getByRole("button", { name: "English", exact: true });
    if (await english.isVisible().catch(() => false)) await english.click();
    await page.waitForFunction(() => {
      const text = document.body.innerText;
      return !text.includes("Loading assets") && !text.includes("Loading…");
    }, null, { timeout: 120_000 });
    const text = await page.locator("body").innerText();
    const modes = await page.locator("[data-layered-overview]").evaluateAll((nodes) => nodes.map((node) => node.getAttribute("data-mode")));
    const overlay = await page.locator("[data-mode='strip'] [data-layered-overlay]").last().evaluate((node) => ({ scrollHeight: node.scrollHeight, clientHeight: node.clientHeight, overflowY: getComputedStyle(node).overflowY })).catch(() => null);
    report.push({
      viewport,
      route,
      modes,
      overlay,
      manifest: text.includes("manifest check reports") || text.includes("has no asset document") || text.includes("Manifest-Eintrag"),
      invalid: text.includes("failed validation") || text.includes("Not authored yet"),
      authored: text.includes("107 authored · 0 planned"),
      shown: (text.match(/Shown: (\d+) of (\d+)/) ?? [])[0] ?? "",
      title: titles[route] === undefined ? true : text.includes(titles[route]),
      consoleErrors: [...consoleErrors],
      pageErrors: [...pageErrors],
    });
  }
  await page.close();
}
await browser.close();
console.log(JSON.stringify(report, null, 2));
