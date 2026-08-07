import { chromium } from "playwright";
import { join } from "path";
const ticketDir = import.meta.dirname;
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
const pageErrors = [];
const consoleMsgs = [];
const requests = [];
page.on("pageerror", (err) => pageErrors.push({ message: String(err.message || err).slice(0, 800), stack: String(err.stack || "").slice(0, 1500) }));
page.on("console", (msg) => consoleMsgs.push({ type: msg.type(), text: msg.text().slice(0, 1000) }));
page.on("requestfailed", (req) => requests.push({ url: req.url().slice(0, 200), failure: req.failure()?.errorText }));
page.on("response", async (res) => {
  if (res.status() >= 400) requests.push({ url: res.url().slice(0, 200), status: res.status() });
});
let crash = null;
page.on("crash", () => { crash = "page crashed"; });
try {
  const resp = await page.goto("http://127.0.0.1:6078/", { waitUntil: "domcontentloaded", timeout: 60000 });
  console.log("goto status", resp?.status());
  for (let i = 0; i < 6; i++) {
    await page.waitForTimeout(2000);
    const snap = {
      i,
      title: await page.title().catch(() => ""),
      canvasCount: await page.locator("canvas").count().catch(() => -1),
      text: (await page.locator("body").innerText().catch(() => "")).split("\n").map(s=>s.trim()).filter(Boolean).slice(0, 50),
      pageErrors: pageErrors.slice(),
      consoleErrors: consoleMsgs.filter(m => m.type === "error").map(m => m.text).slice(0, 20),
      crash,
      failedReqs: requests.slice(0, 30),
    };
    await Bun.write(join(ticketDir, `🧪lowpoly-snap-${i}.json`), JSON.stringify(snap, null, 2));
    await page.screenshot({ path: join(ticketDir, `🧪lowpoly-snap-${i}.png`) }).catch((e) => console.log("shot fail", e.message));
    console.log(JSON.stringify({ i: snap.i, canvas: snap.canvasCount, text: snap.text.slice(0, 20), errs: snap.pageErrors.map(e=>e.message).slice(0,5), cerrs: snap.consoleErrors.slice(0,5), crash, failed: snap.failedReqs.length }, null, 2));
    if (crash || page.isClosed()) break;
  }
} catch (e) {
  console.error("PROBE FAIL", e.message);
  await Bun.write(join(ticketDir, "🧪lowpoly-probe-fail.json"), JSON.stringify({ error: String(e), pageErrors, consoleMsgs: consoleMsgs.slice(0,40), requests }, null, 2));
}
await browser.close().catch(()=>{});
