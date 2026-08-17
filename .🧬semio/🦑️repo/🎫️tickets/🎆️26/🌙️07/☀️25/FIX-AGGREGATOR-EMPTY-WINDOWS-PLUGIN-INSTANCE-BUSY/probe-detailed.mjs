import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const outDir = new URL(".", import.meta.url).pathname;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const events = [];

await page.addInitScript(() => {
  const push = (kind, detail) => {
    (window.__semioProbe ??= []).push({ t: performance.now(), kind, detail });
  };
  const origError = console.error.bind(console);
  const origLog = console.log.bind(console);
  console.error = (...args) => {
    push("console.error", args.map((a) => (a instanceof Error ? { message: a.message, stack: a.stack } : String(a))));
    origError(...args);
  };
  console.log = (...args) => {
    const text = args.map(String).join(" ");
    if (/DEBUG|action|render|loadDocument|setActive|busy|unreachable/i.test(text)) {
      push("console.log", text.slice(0, 500));
    }
    origLog(...args);
  };
});

page.on("console", (msg) => {
  const text = msg.text();
  if (/busy|unreachable|action failed|render failed|loadDocument|setActive|DEBUG/i.test(text)) {
    events.push({ type: msg.type(), text: text.slice(0, 800) });
  }
});

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForTimeout(5000);

const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
if (await skip.count()) {
  await skip.first().click({ timeout: 5000 }).catch(() => {});
  await page.waitForTimeout(2000);
}

const probe = await page.evaluate(() => window.__semioProbe ?? []);
const report = { events, probe };
writeFileSync(`${outDir}probe-detailed.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
