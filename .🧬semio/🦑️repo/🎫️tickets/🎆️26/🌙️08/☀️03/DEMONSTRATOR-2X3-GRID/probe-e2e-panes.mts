import { chromium } from "playwright";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { writeFileSync } from "node:fs";

const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const errors: string[] = [];
page.on("pageerror", (e) => errors.push(`page: ${e.message.slice(0, 200)}`));

await page.goto("http://127.0.0.1:6029/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(2000);
const introClose = page.locator('[data-slot="introduction-close"]').first();
if (await introClose.count()) {
  await introClose.click({ force: true });
  await page.waitForTimeout(800);
}

// wait for iframes to load something
await page.waitForTimeout(8000);

const report = await page.evaluate(async () => {
  const iframes = [...document.querySelectorAll("iframe")];
  const results = [];
  for (const iframe of iframes) {
    const title = iframe.getAttribute("title") || "";
    const src = iframe.getAttribute("src") || "";
    let access = "ok";
    let bodyText = "";
    let childCount = 0;
    let bg = "";
    let hasCanvas = false;
    let hasRoot = false;
    try {
      const doc = iframe.contentDocument;
      if (!doc) {
        access = "no-document";
      } else {
        bodyText = (doc.body?.innerText || "").slice(0, 120);
        childCount = doc.body?.children.length ?? 0;
        bg = getComputedStyle(doc.body || doc.documentElement).backgroundColor;
        hasCanvas = !!doc.querySelector("canvas");
        hasRoot = !!doc.querySelector("#root") || !!doc.querySelector("[data-semio-root], #app");
        // also check root has children
        const root = doc.querySelector("#root");
        if (root) childCount = Math.max(childCount, root.children.length);
      }
    } catch (e) {
      access = `cross-origin-or-error: ${(e as Error).message}`;
    }
    results.push({ title, src, access, childCount, bg, hasCanvas, hasRoot, bodyText });
  }
  return results;
});

await page.screenshot({ path: join(outDir, "probe-e2e-overview.png"), fullPage: false });

// hover generator to reveal content
const card = page.locator("a[href='/generator/']").first();
const box = await card.boundingBox();
if (box) {
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.waitForTimeout(2000);
  await page.screenshot({ path: join(outDir, "probe-e2e-generator-hover.png") });
}

const out = { iframes: report, errors: errors.slice(0, 20) };
writeFileSync(join(outDir, "probe-e2e-out.json"), JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 2));
await browser.close();
