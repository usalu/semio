/** @emoji 🌐 Browser smoke check for trinity jack play (graph + writer canvases). */
import { chromium } from "playwright";

const url = process.env.TRINITY_JACK_PLAY_URL ?? "http://127.0.0.1:6054/";
const pageErrors = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("pageerror", (err) => pageErrors.push(String(err)));

try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
  await page.waitForTimeout(5000);
  const canvasCount = await page.locator("canvas").count();
  if (canvasCount < 2) throw new Error(`expected at least 2 canvases, got ${canvasCount}`);

  const metrics = await page.evaluate(() => {
    const canvases = [...document.querySelectorAll("canvas")];
    return canvases.map((canvas) => ({
      w: canvas.width,
      h: canvas.height,
      cw: canvas.clientWidth,
      ch: canvas.clientHeight,
    }));
  });

  const invalid = metrics.filter((m) => m.w < 2 || m.h < 2);
  if (invalid.length > 0) throw new Error(`zero-size canvas backing store: ${JSON.stringify(invalid)}`);

  const deviceMismatch = pageErrors.filter((e) => e.includes("cannot be used with") || e.includes("Invalid CommandBuffer"));
  if (deviceMismatch.length > 0) {
    throw new Error(`webgpu device mismatch: ${deviceMismatch.join(" | ")}`);
  }

  const textarea = page.locator("textarea").first();
  await textarea.focus();
  await textarea.fill("MATCH (a:Piece) RETURN a.name");
  await page.waitForTimeout(2000);

  console.log("[DEBUG] jack-browser-check ok", { canvasCount, metrics, pageErrors });
} finally {
  await browser.close();
}
