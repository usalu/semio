/** @emoji 🧪 Boot lowpoly react playground and assert plugin/app chrome loads. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const url = process.env.LOWPOLY_URL ?? "http://127.0.0.1:6078/";
const timeoutMs = Number(process.env.LOWPOLY_BOOT_TIMEOUT_MS ?? 120_000);
const ticketDir = dirname(fileURLToPath(import.meta.url));

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const errors = [];
const logs = [];
page.on("pageerror", (error) => errors.push(String(error)));
page.on("console", (msg) => {
  const text = msg.text();
  logs.push(`[${msg.type()}] ${text}`);
  if (msg.type() === "error") errors.push(text);
});

await page.goto(url, { waitUntil: "domcontentloaded", timeout: timeoutMs });

const pluginsLoaded = await page.waitForFunction(
  () => {
    const body = document.body?.innerText ?? "";
    if (body.includes("Loading plugins")) return false;
    const root = document.getElementById("root");
    return Boolean(root && root.childElementCount > 0);
  },
  { timeout: timeoutMs },
).then(() => true).catch(() => false);

// Wait a bit more for canvas / world3d / measures to settle.
await page.waitForTimeout(3000);

const bodyText = await page.locator("body").innerText().catch(() => "");
const title = await page.title();
const rootHtml = await page.locator("#root").innerHTML().catch(() => "");
const canvasCount = await page.locator("canvas").count().catch(() => 0);
const screenshotPath = join(ticketDir, "boot.png");
await page.screenshot({ path: screenshotPath, fullPage: true }).catch(() => {});

const markers = {
  loadingPlugins: bodyText.includes("Loading plugins"),
  showEdges: /show edges|edges|kanten/i.test(bodyText),
  selection: /selection|auswahl|mesh|face|edge|vertex/i.test(bodyText),
  brush: /brush|pinsel|paint/i.test(bodyText),
  hasCanvas: canvasCount > 0,
};

const report = {
  url,
  title,
  pluginsLoaded,
  markers,
  canvasCount,
  rootChildren: await page.locator("#root > *").count().catch(() => 0),
  rootHtmlLength: rootHtml.length,
  bodySnippet: bodyText.slice(0, 800),
  screenshotPath,
  errors,
  logs: logs.filter((line) =>
    /\[DEBUG\]|lowpoly|error|fail|plugin|manifest|instantiate/i.test(line)
  ),
};

writeFileSync(join(ticketDir, "verify-boot-report.json"), `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify(report, null, 2));
await browser.close();

const ok = pluginsLoaded && !markers.loadingPlugins && errors.length === 0 && (markers.hasCanvas || markers.showEdges || markers.selection);
if (!ok) process.exit(1);
