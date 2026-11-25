import { awaitBeacon, dismissIntroduction, chromium, url, session, out, join } from "./journey-kit.ts";
const browser = await chromium.launch();
const s = await session(browser, "en");
const { page } = s;
await page.goto(url);
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await s.settle(3000);
const code = await Bun.file(process.env.PROBE_FILE!).text();
const fn = new (Object.getPrototypeOf(async function(){}).constructor)("s", "page", "out", "join", code);
try { console.log(JSON.stringify(await fn(s, page, out, join), null, 1)); } catch (e) { console.log("ERR", String(e).slice(0, 1500)); }
console.log("faults", s.faults.slice(0, 10));
await browser.close();
