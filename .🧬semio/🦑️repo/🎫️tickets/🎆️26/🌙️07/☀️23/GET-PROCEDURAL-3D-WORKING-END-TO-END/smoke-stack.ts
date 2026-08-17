import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const out = "/Users/ueli/Documents/semio/.repo/🎫️/26/07/23/GET-PROCEDURAL-3D-WORKING-END-TO-END/smoke-stack.json";

const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
await page.addInitScript(() => {
  const orig = console.error;
  console.error = (...args: unknown[]) => {
    const enriched = args.map((a) => {
      if (a instanceof Error) return { __error: true, message: a.message, stack: a.stack };
      if (a && typeof a === "object" && "stack" in (a as object)) {
        const err = a as { message?: string; stack?: string };
        return { __error: true, message: err.message || String(a), stack: err.stack };
      }
      try {
        return typeof a === "string" ? a : JSON.parse(JSON.stringify(a));
      } catch {
        return String(a);
      }
    });
    (window as unknown as { __SEMIO_ERRS?: unknown[] }).__SEMIO_ERRS ??= [];
    (window as unknown as { __SEMIO_ERRS: unknown[] }).__SEMIO_ERRS.push(enriched);
    orig.apply(console, args as []);
  };
});
await page.goto("http://localhost:6018/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(7000);
const errs = await page.evaluate(() => (window as unknown as { __SEMIO_ERRS?: unknown[] }).__SEMIO_ERRS || []);
writeFileSync(out, JSON.stringify(errs, null, 2));
console.log("captured", errs.length);
for (const e of errs) {
  console.log("====");
  console.log(JSON.stringify(e, null, 2).slice(0, 5000));
}
await browser.close();
