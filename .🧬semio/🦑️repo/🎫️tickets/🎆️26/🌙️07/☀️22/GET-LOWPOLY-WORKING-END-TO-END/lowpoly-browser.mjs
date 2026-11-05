/** 🔎 Browser evidence for the ticket's running lowpoly editor. */
import { chromium } from "playwright";
import { mkdir, realpath, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const generated = join(ticket, "\u{1F5D1}\uFE0Fgenerated");
const args = process.argv.slice(2);
const option = (name) => {
  const index = args.indexOf(name);
  return index < 0 ? undefined : args[index + 1];
};
const url = option("--url") ?? process.env.LOWPOLY_URL;
if (!url) throw new Error("Supply --url or LOWPOLY_URL");
const target = new URL(url);
if (!["http:", "https:"].includes(target.protocol)) throw new Error("Expected an HTTP URL");
const output = resolve(option("--out") ?? process.env.LOWPOLY_PROBE_OUTPUT ?? generated);
const inside = (root, path) => {
  const child = relative(root, path);
  return child === "" || (!child.startsWith("..") && !isAbsolute(child));
};
if (!inside(generated, output)) throw new Error("Probe output must stay inside this ticket's generated folder");
if ([output, join(output, "browser-report.json"), join(output, "boot.png")].some(path => path.length > 256)) throw new Error("Probe output exceeds the repository path limit");
await mkdir(generated, { recursive: true });
await mkdir(output, { recursive: true });
if (!inside(await realpath(ticket), await realpath(generated)) || !inside(await realpath(generated), await realpath(output))) throw new Error("Probe output resolves outside the generated folder");
const timeout = Number(option("--timeout") ?? process.env.LOWPOLY_BOOT_TIMEOUT_MS ?? 120000);
if (!Number.isFinite(timeout) || timeout < 1000 || timeout > 300000) throw new Error("Timeout must be between 1000 and 300000 ms");
const temporary = join(output, "t");
for (const leaf of ["p", "t", "d", "r"]) {
  const directory = join(output, leaf);
  await mkdir(directory, { recursive: true });
  if (!inside(await realpath(generated), await realpath(directory))) throw new Error("Browser storage resolves outside the generated folder");
}
for (const name of ["TEMP", "TMP", "TMPDIR"]) process.env[name] = temporary;
const report = { url, verificationScope: "boot-observation", renderedGeometryVerified: false, requestFailureAllowlist: [], pendingJourneys: ["viewer", "UV paint", "host import/export", "English/German locales", "edit/undo/redo", "reload persistence"], startedAt: new Date().toISOString(), errors: [], console: [], failedRequests: [], failedResponses: [] };
const bounded = (list, value) => { if (list.length < 200) list.push(value); };
let context;
try {
  context = await chromium.launchPersistentContext(join(output, "p"), {
  headless: true,
  serviceWorkers: "block",
  downloadsPath: join(output, "d"),
  tracesDir: join(output, "r"),
  env: { ...process.env },
  viewport: { width: 1440, height: 1000 },
  locale: process.env.LOWPOLY_BROWSER_LOCALE ?? "en-US",
  ...(process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH ? { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH } : {}),
});
  const page = await context.newPage();
  page.on("pageerror", error => bounded(report.errors, String(error)));
  page.on("console", message => bounded(report.console, { type: message.type(), text: message.text().slice(0, 8000) }));
  page.on("requestfailed", request => bounded(report.failedRequests, { url: request.url(), failure: request.failure()?.errorText }));
  page.on("response", response => { if (response.status() >= 400) bounded(report.failedResponses, { url: response.url(), status: response.status() }); });
  const response = await page.goto(url, { waitUntil: "domcontentloaded", timeout });
  report.status = response?.status();
  report.ready = await page.waitForFunction(() => {
    const root = document.getElementById("root");
    const body = document.body?.innerText ?? "";
    const canvas = [...document.querySelectorAll("canvas")].some(node => node.width > 0 && node.height > 0 && node.getBoundingClientRect().width > 0 && node.getBoundingClientRect().height > 0);
    return Boolean(root?.childElementCount && !/loading plugins/i.test(body) && canvas);
  }, undefined, { timeout }).then(() => true).catch(error => { report.readinessError = String(error); return false; });
  report.title = await page.title();
  report.body = (await page.locator("body").innerText()).slice(0, 40000);
  report.ariaSnapshot = await page.locator("body").ariaSnapshot().catch(error => String(error));
  report.dom = await page.evaluate(() => ({
    rootChildren: document.getElementById("root")?.childElementCount ?? 0,
    controls: [...document.querySelectorAll("button,input,select,textarea,[role],[aria-label],[title]")].slice(0, 500).map(element => ({
      tag: element.tagName, role: element.getAttribute("role"), text: element.textContent?.trim().slice(0, 200),
      label: element.getAttribute("aria-label"), title: element.getAttribute("title"), testId: element.getAttribute("data-testid"),
      disabled: element.matches(":disabled"), visible: Boolean(element.getBoundingClientRect().width && element.getBoundingClientRect().height),
    })),
    canvases: [...document.querySelectorAll("canvas")].map(canvas => ({
      width: canvas.width, height: canvas.height, label: canvas.getAttribute("aria-label"),
      rect: { x: canvas.getBoundingClientRect().x, y: canvas.getBoundingClientRect().y, width: canvas.getBoundingClientRect().width, height: canvas.getBoundingClientRect().height },
    })),
    links: [...document.querySelectorAll("a[href]")].map(link => ({ text: link.textContent, href: link.getAttribute("href") })).slice(0, 100),
  }));
  report.screenshot = join(output, "boot.png");
  await page.screenshot({ path: report.screenshot, fullPage: true });
  report.consoleErrors = report.console.filter(message => message.type === "error");
  report.success = report.ready && report.errors.length === 0 && report.consoleErrors.length === 0 && report.failedRequests.length === 0 && report.failedResponses.length === 0 && report.status < 400 && Boolean(report.dom.canvases.length);
} catch (error) {
  report.success = false;
  report.fatal = String(error);
} finally {
  report.finishedAt = new Date().toISOString();
  await writeFile(join(output, "browser-report.json"), JSON.stringify(report, null, 2) + "\n");
  await context?.close();
}
console.log(JSON.stringify({ success: report.success, ready: report.ready, title: report.title, errors: report.errors.length, failedRequests: report.failedRequests.length, failedResponses: report.failedResponses.length, renderedGeometryVerified: report.renderedGeometryVerified, output }));
if (!report.success) process.exitCode = 1;
