import { chromium } from "playwright";

const url = process.env.GIS_URL ?? "http://127.0.0.1:6040/";
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer", "--use-angle=swiftshader"],
});
const page = await browser.newPage();
const consoleMsgs: string[] = [];
const pageErrors: string[] = [];
page.on("console", (msg) => consoleMsgs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => pageErrors.push(String(err)));
await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 }).catch((e) => pageErrors.push(`goto: ${e}`));
await page.waitForTimeout(12000);
const bodyText = await page.locator("body").innerText().catch(() => "<no body>");
const canvasCount = await page.locator("canvas").count();
const canvasBox = canvasCount > 0 ? await page.locator("canvas").first().boundingBox() : null;
const canvasSample = canvasCount > 0
  ? await page.evaluate(() => {
      const canvas = document.querySelector("canvas");
      if (!canvas) return null;
      return { width: canvas.width, height: canvas.height, cssW: canvas.clientWidth, cssH: canvas.clientHeight };
    })
  : null;
const actionErrors = consoleMsgs.filter((m) => /action failed|Render error|Cannot read|TypeError|NoCompatible/i.test(m));
const result = {
  url,
  title: await page.title().catch(() => ""),
  hasRenderError: /Render error/i.test(bodyText),
  bodySnippet: bodyText.slice(0, 1500),
  canvasCount,
  canvasBox,
  canvasSample,
  pageErrors,
  actionErrors: actionErrors.slice(0, 30),
  debugMsgs: consoleMsgs.filter((m) => /\[DEBUG\]/.test(m)).slice(0, 40),
};
await browser.close();
console.log(JSON.stringify(result, null, 2));
