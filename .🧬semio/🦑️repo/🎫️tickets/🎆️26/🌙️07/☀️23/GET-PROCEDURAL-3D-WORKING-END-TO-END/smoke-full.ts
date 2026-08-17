import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const outDir = "/Users/ueli/Documents/semio/.repo/🎫️/26/07/23/GET-PROCEDURAL-3D-WORKING-END-TO-END";

const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
await page.addInitScript(() => {
  const push = (entry: unknown) => {
    (window as unknown as { __SEMIO_ERRS?: unknown[] }).__SEMIO_ERRS ??= [];
    (window as unknown as { __SEMIO_ERRS: unknown[] }).__SEMIO_ERRS.push(entry);
  };
  const origError = console.error;
  console.error = (...args: unknown[]) => {
    push({
      kind: "console.error",
      args: args.map((a) => {
        if (a instanceof Error) return { message: a.message, stack: a.stack };
        if (a && typeof a === "object" && "stack" in (a as object)) {
          const err = a as { message?: string; stack?: string };
          return { message: err.message || String(a), stack: err.stack };
        }
        return typeof a === "string" ? a : (() => { try { return JSON.parse(JSON.stringify(a)); } catch { return String(a); } })();
      }),
    });
    origError.apply(console, args as []);
  };
  const origWarn = console.warn;
  console.warn = (...args: unknown[]) => {
    const text = args.map(String).join(" ");
    if (text.includes("Maximum update") || text.includes("update depth")) {
      push({ kind: "console.warn", text: text.slice(0, 8000) });
    }
    origWarn.apply(console, args as []);
  };
  window.addEventListener("error", (event) => {
    push({ kind: "window.error", message: event.message, stack: event.error?.stack });
  });
});
await page.goto("http://localhost:6018/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(8000);
const bodyText = await page.locator("body").innerText().catch(() => "");
const errs = await page.evaluate(() => (window as unknown as { __SEMIO_ERRS?: unknown[] }).__SEMIO_ERRS || []);
const hasRenderError = /Render error|Renderfehler|Maximum update depth/i.test(bodyText);
const canvasCount = await page.locator("canvas").count();
const windowSlots = await page.locator('[data-slot="window"]').count();
writeFileSync(`${outDir}/smoke-after-fix.json`, JSON.stringify({ hasRenderError, canvasCount, windowSlots, bodyStart: bodyText.slice(0, 2000), errs }, null, 2));
console.log(JSON.stringify({ hasRenderError, canvasCount, windowSlots, errCount: errs.length, bodyStart: bodyText.slice(0, 600) }, null, 2));
for (const e of errs.slice(0, 4)) {
  console.log("====");
  console.log(JSON.stringify(e, null, 2).slice(0, 4000));
}
await browser.close();
