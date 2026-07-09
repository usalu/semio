import { chromium } from "playwright";

const base = "http://localhost:6050/";
const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(base, { waitUntil: "networkidle" });
await page.waitForSelector(".reveal", { timeout: 15000 });

const introMorphCount = await page.locator('.slides > section > section[data-auto-animate-id="intro"]').count();
const mediaMorphCount = await page.locator('.slides > section > section[data-auto-animate-id="media"]').count();

for (let i = 0; i < 7; i++) {
  await page.keyboard.press("ArrowRight");
  await page.waitForTimeout(400);
}

const visibleSlide = page.locator(".slides section.present");
await visibleSlide.locator('[data-id="catalogue"] img').first().waitFor({ timeout: 10000 });
const catalogueSrc = await visibleSlide.locator('[data-id="catalogue"] img').first().getAttribute("src");
await page.keyboard.press("ArrowRight");
await page.waitForTimeout(800);
const mediaSlide = page.locator(".slides section.present");
await mediaSlide.locator('[data-id="demo-video"] video').first().waitFor({ timeout: 10000 });
const videoSrc = await mediaSlide.locator('[data-id="demo-video"] video').first().getAttribute("src");
await mediaSlide.locator(".react-pdf__Document").first().waitFor({ timeout: 20000 });

const result = {
  introMorphCount,
  mediaMorphCount,
  catalogueSrc,
  videoSrc,
  hasPdf: (await mediaSlide.locator(".react-pdf__Page").count()) > 0,
  positionedFramesOnMediaSlide: await mediaSlide.locator(".presentation-disposition-frame").count(),
};
console.log(JSON.stringify(result, null, 2));
await browser.close();
