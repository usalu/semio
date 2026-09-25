import { chromium } from "/home/user/semio/node_modules/playwright/index.mjs";
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium-1194/chrome-linux/chrome" });
const page = await browser.newPage({ viewport: { width: 1500, height: 900 } });
await page.goto("http://127.0.0.1:5174/", { waitUntil: "domcontentloaded" });
await page.locator('[id="ui.panelToggle.chat"]').click({ timeout: 60_000 });
await page.waitForTimeout(1000);
const tabs = await page.locator("button").evaluateAll((els) => els.filter((e) => e.getBoundingClientRect().x > 1150 && e.getBoundingClientRect().y < 60).map((e) => `${e.id}|${e.getAttribute("data-testid")}|${e.getAttribute("aria-label")}|${e.getAttribute("title")}|${e.getAttribute("data-tab-id") ?? ""}|${e.outerHTML.slice(0, 200)}`));
console.log(tabs.join("\n"));
await browser.close();
