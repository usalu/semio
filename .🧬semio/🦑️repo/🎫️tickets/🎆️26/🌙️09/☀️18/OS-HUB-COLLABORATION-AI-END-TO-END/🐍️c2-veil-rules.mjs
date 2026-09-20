/** 🎓️ C2 — which CSS rule turns the introduction-elevated sign-in submit into `pointer-events: none`.
 * Usage: bun 🐍️c2-veil-rules.mjs <shellUrl> */
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 120_000 });
await page.locator('[data-semio-hub-sign-in=""]').first().click();
await page.locator("[data-semio-hub-workspace]").waitFor({ state: "visible", timeout: 30_000 });
await page.waitForTimeout(1_500);

console.log(
  JSON.stringify(
    await page.evaluate(() => {
      const button = document.querySelector('[data-semio-hub-workspace] button[type="submit"][aria-label="Sign in"]');
      const hits = [];
      const walk = (rules) => {
        for (const rule of rules) {
          if (rule.cssRules) walk(rule.cssRules);
          if (!rule.selectorText || !rule.style || !rule.style.getPropertyValue("pointer-events")) continue;
          for (const selector of rule.selectorText.split(",")) {
            try {
              if (button.matches(selector.trim())) hits.push({ selector: selector.trim(), value: rule.style.getPropertyValue("pointer-events"), priority: rule.style.getPropertyPriority("pointer-events") });
            } catch {
              /* 🏁️ Unsupported selector syntax in this engine. */
            }
          }
        }
      };
      for (const sheet of document.styleSheets) {
        try {
          walk(sheet.cssRules);
        } catch {
          /* 🏁️ Cross-origin sheet. */
        }
      }
      return { disabled: button.disabled, ariaDisabled: button.getAttribute("aria-disabled"), inline: button.getAttribute("style"), hits };
    }),
    null,
    2,
  ),
);
await browser.close();
