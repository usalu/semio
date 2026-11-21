import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import { expect } from "playwright/test";

const recoveryPath = /^\/__staffage_recovery-\d+-[0-9a-f-]+$/u;
const routeQuery = "?ref=release%20smoke&flag";

async function withLostModule(browser, module, inspect) {
  const context = await browser.newContext({ locale: "en-US", viewport: { width: 1440, height: 900 } });
  try {
    const page = await context.newPage();
    const reloads = [];
    const fallbackStatuses = [];
    const responses = [];
    page.on("framenavigated", (frame) => {
      if (frame !== page.mainFrame()) return;
      const url = new URL(frame.url());
      if (recoveryPath.test(url.pathname)) reloads.push(url);
    });
    page.on("response", (response) => {
      if (response.url() === module.href) responses.push(response.status());
      if (response.request().isNavigationRequest() && recoveryPath.test(new URL(response.url()).pathname)) {
        fallbackStatuses.push(response.status());
      }
    });
    await page.route(module.href, (route) => route.fulfill({
      status: 404, contentType: "text/html", body: "<!doctype html><title>Missing old module</title>",
    }), { times: 1 });
    const { restored, historyLength } = await inspect(page);
    await expect(page).toHaveURL(restored.href);
    assert.deepEqual(responses, [404, 200]);
    assert.deepEqual(fallbackStatuses, [404]);
    assert.equal(reloads.length, 1, "Expected one recovery navigation");
    assert.equal(reloads[0].hash, restored.hash);
    assert.equal(reloads[0].search, routeQuery);
    assert.equal(await page.evaluate(() => history.length), historyLength);
  } finally {
    await context.close();
  }
}

/** 🛟️ Proves recovery from actual entry and lazy-import HTTP failures in a built artifact. */
export async function exerciseRecovery(browser, release, base, check) {
  await check("stale entry 404 reloads exactly once and restores the original URL", async () => {
    const html = await readFile(join(release, "index.html"), "utf8");
    const tag = html.match(/<script\b[^>]*\bid="staffage-entry"[^>]*>/u)?.[0];
    const source = tag?.match(/\bsrc="([^"]+)"/u)?.[1];
    assert.ok(source, "Release document is missing the tagged entry module");
    await withLostModule(browser, new URL(source, base), async (page) => {
      const restored = new URL(routeQuery + "#/collections", base);
      await page.goto("data:text/html,<title>Previous page</title>");
      const historyLength = await page.evaluate(() => history.length);
      await page.goto(restored.href, { waitUntil: "commit" });
      await expect(page.locator('[data-page="collections"]:visible').getByRole("heading", { name: "Collections", exact: true })).toBeVisible({ timeout: 30000 });
      return { restored, historyLength: historyLength + 1 };
    });
  });
  await check("stale lazy 3D playground chunk 404 reloads once and restores the tab", async () => {
    const assetDir = join(release, "assets");
    const modules = (await readdir(assetDir)).filter((name) => name.endsWith(".js"));
    const chunks = await Promise.all(modules.map(async (name) => {
      return (await readFile(join(assetDir, name), "utf8")).includes("data-model-playground") ? name : null;
    }));
    const sources = chunks.filter(Boolean);
    assert.equal(sources.length, 1, "Expected one built module implementing the 3D playground");
    await withLostModule(browser, new URL("assets/" + encodeURIComponent(sources[0]), base), async (page) => {
      await page.goto(new URL(routeQuery + "#/playground", base).href, { waitUntil: "domcontentloaded" });
      const tab = page.locator('[data-page="playground"]:visible a[data-tab="model"]');
      await expect(tab).toBeVisible();
      const historyLength = await page.evaluate(() => history.length);
      await tab.click();
      await expect(page.locator('[data-page="playground"]:visible [data-model-playground]')).toBeVisible({ timeout: 30000 });
      return { restored: new URL(routeQuery + "#/playground/3d", base), historyLength: historyLength + 1 };
    });
  });
}
