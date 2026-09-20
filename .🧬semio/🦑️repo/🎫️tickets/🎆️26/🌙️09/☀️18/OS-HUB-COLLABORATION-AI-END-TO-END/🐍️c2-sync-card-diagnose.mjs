/** 🔄️ C2 — what the sync utility tab actually renders, so the `remote://` attach can be driven.
 * Usage: bun 🐍️c2-sync-card-diagnose.mjs <shellUrl> */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6191";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let attempt = 0; attempt < 180; attempt += 1) {
  await page.waitForTimeout(1_000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break;
}
await page.waitForTimeout(4_000);

const census = () =>
  page.evaluate(() => ({
    ids: [...new Set([...document.querySelectorAll("[id]")].map((el) => el.id).filter((id) => /sync|framework\.sync|s-sync/.test(id)))],
    tabButtons: [...document.querySelectorAll('[data-slot="panel-tab-button"], [data-slot="utility-tab-button"], [data-tab-id]')].map((el) => `${el.getAttribute("data-slot") ?? el.tagName}#${el.id}|${el.getAttribute("data-tab-id") ?? ""}|${(el.textContent ?? "").trim().slice(0, 30)}`),
    footer: (document.querySelector('[data-slot="footer"]')?.innerText ?? "").replace(/\s+/g, " ").slice(0, 400),
  }));

console.log(`BEFORE ${JSON.stringify(await census(), null, 2)}`);
for (const selector of ['[id="s-sync-status"]', '[data-tab-id="s-sync-status"]', '[data-slot="panel-tab-button"][id="s-sync-status"]']) {
  const count = await page.locator(selector).count();
  console.log(`selector ${selector} count=${count}`);
  if (count > 0) {
    await page.locator(selector).first().click({ force: true, timeout: 8_000 }).catch((error) => console.log(`click failed ${String(error).split("\n")[0]}`));
    await page.waitForTimeout(1_500);
    break;
  }
}
console.log(`AFTER ${JSON.stringify(await census(), null, 2)}`);
console.log(
  `CARD ${JSON.stringify(
    await page.evaluate(() => {
      const card = document.querySelector('[id="framework.sync.card"]') ?? document.querySelector('[id="ui.utilities.group.sync"]');
      const controls = [...(card?.querySelectorAll("button,[role=button],[role=treeitem],input") ?? [])].map((el) => `${el.tagName}#${el.id}|role=${el.getAttribute("role") ?? ""}|aria=${el.getAttribute("aria-label") ?? ""}|slot=${el.getAttribute("data-slot") ?? ""}|${(el.textContent ?? "").trim().slice(0, 30)}`);
      return { controls, html: (card?.innerHTML ?? "").slice(0, 1500) };
    }),
    null,
    2,
  )}`,
);
await page.screenshot({ path: `${OUT}c2-sync-card-diagnose.png` });
await browser.close();
