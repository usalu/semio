/** 🃏️ Ticket tool of work package J: enters the dev site as an anonymous learner and prints the boxes of one card of the
 * home screen four levels deep (tag, `data-*` hooks, left, top, width × height), to see which element's top edge is the
 * edge a pet should stand on: the section (`data-card`) starts with the cap row that holds the title chip, the visible
 * body (`data-slot="window-chrome-body-surface"`) starts one cap height lower.
 *
 * Usage (from the repository root, with a dev stack running):
 *   node ".../wp_j_card_dom_probe.mjs" [--url http://127.0.0.1:6191/] [--card quiz:heating]
 */
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6191/");
const card = option("card", "quiz:heating");

const browser = await chromium.launch();
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB" })).newPage();
  const front = (name) => page.locator(`#quiz-main [data-card="${name}"]`).and(page.locator(":not([inert] *)"));
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await front("introduction").waitFor({ timeout: 150000 });
  await front("introduction").locator('[data-overview-card-action="primary"]').click();
  await front("identity").locator('input[type="radio"][value="anonymous"]').check();
  await front("identity").locator('[data-overview-card-action="primary"]').click();
  await page.locator("[data-layered-overview]").waitFor();
  await page.waitForTimeout(1500);
  const boxes = await page.evaluate((name) => {
    const section = [...document.querySelectorAll(`#quiz-main [data-card="${name}"]`)].find((element) => element.closest("[inert]") === null);
    const describe = (element, depth) => {
      const box = element.getBoundingClientRect();
      const hooks = [...element.attributes].filter((attribute) => attribute.name.startsWith("data-")).map((attribute) => `${attribute.name}=${attribute.value}`).join(" ");
      const lines = [`${"  ".repeat(depth)}<${element.tagName.toLowerCase()} ${hooks}> ${Math.round(box.left)},${Math.round(box.top)} ${Math.round(box.width)}x${Math.round(box.height)}`];
      if (depth < 4) for (const child of element.children) lines.push(...describe(child, depth + 1));
      return lines;
    };
    return section === undefined ? `no card ${name}` : describe(section, 0).join("\n");
  }, card);
  process.stdout.write(`${boxes}\n`);
} finally {
  await browser.close();
}
