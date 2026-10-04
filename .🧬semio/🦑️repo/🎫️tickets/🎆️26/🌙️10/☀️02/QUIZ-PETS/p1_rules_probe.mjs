/** 📜️ Ticket tool of work package P1: which rules of the stylesheets the home overview has loaded (with calm pets) name a given text — to find which rule makes a change on the document element restyle the whole page.
 *
 * Usage (from the repository root): node ".../p1_rules_probe.mjs" [--url http://127.0.0.1:6245/] [--text data-pet-cursor]
 */
import { chromium } from "playwright";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const url = option("url", "http://127.0.0.1:6245/");
const text = option("text", "data-pet-cursor");
const browser = await chromium.launch({ channel: "chromium" });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US" })).newPage();
await page.goto(url);
const intro = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
await intro.waitFor({ timeout: 60_000 });
await intro.click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 90_000 });
const found = await page.evaluate((text) => {
  const rules = [];
  const visit = (list, from) => {
    for (const rule of list) {
      if (rule.cssRules !== undefined && !(rule instanceof CSSStyleRule)) visit(rule.cssRules, from);
      else if (rule.cssText.includes(text)) rules.push(`${from}: ${rule.cssText.slice(0, 200)}`);
    }
  };
  for (const sheet of document.styleSheets) {
    try {
      visit(sheet.cssRules, sheet.href ?? "inline");
    } catch {
      rules.push(`${sheet.href}: unreadable`);
    }
  }
  return rules;
}, text);
process.stdout.write(`${found.length} rules name ${text}\n${found.join("\n")}\n`);
await browser.close();
