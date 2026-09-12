// 💍 Proves the element-scoped border clocks still animate: mounts one element per ring state into the
// live app, then reads `document.getAnimations()` and the pseudo-element's own animated custom property
// at two instants. Also asserts the document-wide baseline stays at zero running animations.
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
await page.goto("http://127.0.0.1:6018/?plugin=generation3d", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(20000);
const baseline = await page.evaluate(() => document.getAnimations().length);
const result = await page.evaluate(async () => {
  const host = document.createElement("div");
  host.style.cssText = "position:fixed;left:0;top:0;width:120px;height:60px";
  host.innerHTML = `<div class="border-loading" style="width:40px;height:40px"></div>
    <div class="border-waiting" style="width:40px;height:40px"></div>
    <div data-introduced="true" style="width:40px;height:40px"></div>
    <div data-celebrated="true" style="width:40px;height:40px"></div>`;
  document.body.appendChild(host);
  const read = () => ({
    running: document.getAnimations().filter((a) => a.playState === "running").map((a) => a.animationName ?? "?").sort(),
    loadingAngle: getComputedStyle(host.children[0], "::after").getPropertyValue("--loading-border-angle").trim(),
    waitingAngle: getComputedStyle(host.children[1], "::after").getPropertyValue("--waiting-border-angle").trim(),
    introducedWidth: getComputedStyle(host.children[2]).getPropertyValue("--introduced-border-width").trim(),
    celebrateAngle: getComputedStyle(host.children[3], "::after").getPropertyValue("--celebrate-border-angle").trim(),
    rootAngle: getComputedStyle(document.documentElement).getPropertyValue("--loading-border-angle").trim(),
  });
  await new Promise((r) => requestAnimationFrame(() => r(null)));
  const first = read();
  await new Promise((r) => setTimeout(r, 400));
  const second = read();
  host.remove();
  await new Promise((r) => requestAnimationFrame(() => r(null)));
  return { first, second, afterRemoval: document.getAnimations().filter((a) => a.playState === "running").length };
});
console.log("BASELINE_RUNNING", baseline);
console.log(JSON.stringify(result, null, 2));
await browser.close();
