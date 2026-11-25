import { awaitBeacon, dismissIntroduction, chromium, session, url, out, join, root } from "./journey-kit.ts";
const browser = await chromium.launch();
const s = await session(browser, "en");
const { page, t } = s;
await page.goto(url); await awaitBeacon(page, Date.now()+300000); await dismissIntroduction(page); await s.settle(3000);
await page.locator("[id='framework.window.sourcingPool.engagement.toggle']").click(); await s.settle(800);
console.log(await page.evaluate(() => [...document.querySelectorAll("[id*='sourcingPool.action'],[id^='action.']")].map(e => `${e.tagName}|${e.id}|${(e as HTMLElement).innerText.slice(0,40).replace(/\n/g," ")}`)));
await browser.close();
