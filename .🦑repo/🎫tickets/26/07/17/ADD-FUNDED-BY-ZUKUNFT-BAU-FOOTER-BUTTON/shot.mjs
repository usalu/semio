import { chromium } from "playwright";

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
const url = "http://localhost:6006/iframe.html?id=" + encodeURIComponent("🖱️ui⚛️react-layout--default") + "&viewMode=story";
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180000 });
await page.waitForSelector("#ui\\.footer", { timeout: 180000 });
await page.waitForTimeout(1500);
await page.screenshot({ path: "/private/tmp/claude-501/-Users-ueli-Documents-semio/98532f5c-3786-461a-b45f-80894438428c/scratchpad/footer-full.png" });
const footer = await page.$('#ui\\.footer');
await footer.screenshot({ path: "/private/tmp/claude-501/-Users-ueli-Documents-semio/98532f5c-3786-461a-b45f-80894438428c/scratchpad/footer-crop.png" });
await browser.close();
console.log("done");
