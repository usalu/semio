import { chromium } from "playwright";

const origin = process.argv[2] ?? "http://127.0.0.1:6041/";
const started = performance.now();
const log = (kind, data) => console.log(JSON.stringify({ at: Math.round(performance.now() - started), kind, ...data }));
const browser = await chromium.launch({ channel: "chrome", headless: true, timeout: 10000 });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1280, height: 720 } });
  if (process.argv.includes("--trace")) {
    await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  }
  page.on("pageerror", error => log("page-error", { message: error.message, stack: error.stack?.split("\n").slice(0, 6).join("\n") }));
  page.on("console", message => {
    const text = message.text();
    if (["error", "warning"].includes(message.type()) || (process.argv.includes("--trace") && /\bTRACE\b|setActiveExample/.test(text))) log(message.type(), { message: text.slice(0, 2500) });
  });
  page.on("requestfailed", request => log("request-failed", { path: new URL(request.url()).pathname, error: request.failure()?.errorText }));
  page.on("response", response => {
    const path = new URL(response.url()).pathname;
    if (response.status() >= 400 || path.endsWith(".wasm") || path.endsWith("/🌉️bridge.js")) log("response", { path, status: response.status() });
  });
  page.on("worker", worker => log("worker", { path: new URL(worker.url()).pathname }));
  const response = await page.goto(origin, { waitUntil: "domcontentloaded", timeout: 60000 });
  log("document", { status: response?.status() });
  for (const delay of [0, 5_000, 10_000, 15_000]) {
    if (delay) await new Promise(resolve => setTimeout(resolve, delay));
    const state = await page.evaluate(trace => ({
      ready: document.readyState,
      shell: Array.from(document.querySelectorAll("[data-shell-ready],[data-shell-error],[data-shell-not-found]"), node => ({
        ready: node.getAttribute("data-shell-ready"),
        error: node.getAttribute("data-shell-error"),
        notFound: node.getAttribute("data-shell-not-found"),
      })),
      text: document.body?.innerText.slice(0, 600),
      canvases: document.querySelectorAll("canvas").length,
      assets: performance.getEntriesByType("resource").length,
      catalog: window.__semioOsCatalogProbe,
      ...(trace ? {
        crossings: globalThis.__semioCrossingCensus,
        acknowledgements: globalThis.__semioTypedOpAckCensus,
        measures: performance.getEntriesByType("measure").filter(entry => entry.name.startsWith("semio")).slice(-40)
          .map(entry => ({ name: entry.name, start: Math.round(entry.startTime), ms: Math.round(entry.duration), detail: entry.detail })),
      } : {}),
      comboboxes: [...document.querySelectorAll('[role="combobox"]')].map(box => ({ id: box.id, label: box.getAttribute("aria-label"), text: box.innerText, value: box.getAttribute("data-published-value") })),
    }), process.argv.includes("--trace"));
    log("state", state);
    if (state.shell.length) break;
    if (delay === 5_000 && process.argv.includes("--open-example")) {
      try {
        const example = page.locator('[id="playground.navbar.fixture"], [data-element-alias~="playground.navbar.fixture"]').first();
        await example.click({ timeout: 4_000 });
        const options = page.getByRole("option");
        await options.first().waitFor({ timeout: 4_000 });
        log("examples", { values: await options.allTextContents() });
        await options.first().click({ timeout: 4_000 });
      } catch (error) {
        log("example-error", { message: error.message });
      }
    }
  }
} finally {
  await browser.close();
}
