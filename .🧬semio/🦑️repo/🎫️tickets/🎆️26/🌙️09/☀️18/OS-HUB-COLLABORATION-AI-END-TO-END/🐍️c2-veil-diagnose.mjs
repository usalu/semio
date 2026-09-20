/** 🎓️ C2 — is the introduction veil intercepting the sign-in submit the introduction itself elevated?
 * Reads the computed stacking of both, and asks the document who owns the button's centre point.
 * Usage: bun 🐍️c2-veil-diagnose.mjs <shellUrl> */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 120_000 });
await page.locator('[data-semio-hub-sign-in=""]').first().click();
await page.locator("[data-semio-hub-workspace]").waitFor({ state: "visible", timeout: 30_000 });
await page.waitForTimeout(1_500);

const report = await page.evaluate(() => {
  const button = document.querySelector('[data-semio-hub-workspace] button[type="submit"][aria-label="Sign in"]');
  const veil = document.querySelector('[data-slot="introduction-veil"]');
  const describe = (el) => {
    if (!el) return null;
    const style = getComputedStyle(el);
    const rect = el.getBoundingClientRect();
    return { position: style.position, zIndex: style.zIndex, pointerEvents: style.pointerEvents, rect: { x: rect.x, y: rect.y, w: rect.width, h: rect.height } };
  };
  const centre = button ? { x: button.getBoundingClientRect().x + button.getBoundingClientRect().width / 2, y: button.getBoundingClientRect().y + button.getBoundingClientRect().height / 2 } : null;
  const owner = centre ? document.elementFromPoint(centre.x, centre.y) : null;
  const chain = [];
  for (let node = button; node && node !== document.body; node = node.parentElement) {
    const style = getComputedStyle(node);
    chain.push({ slot: node.getAttribute("data-slot") ?? node.tagName, cls: (node.getAttribute("class") ?? "").slice(0, 120), position: style.position, zIndex: style.zIndex, pointerEvents: style.pointerEvents, elevated: node.hasAttribute("data-introduction-elevated") });
  }
  const box = document.querySelector('[data-slot="introduction-info-box-title"]');
  return {
    button: describe(button),
    buttonElevated: button?.hasAttribute("data-introduction-elevated") ?? false,
    veil: describe(veil),
    ownerSlot: owner ? `${owner.tagName}[${owner.getAttribute("data-slot") ?? ""}]` : null,
    chain,
    introductionStep: box?.textContent?.trim() ?? null,
  };
});
console.log(JSON.stringify(report, null, 2));
await page.screenshot({ path: `${OUT}c2-veil-diagnose.png` });
await browser.close();
