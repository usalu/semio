/** 📏 Row seams of the shared ui Table in a served Storybook: heights of header and rows, native border widths and the compositor seams painted per row. Usage (repo root): node table-rows-measure.mjs <port> <label> */
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
const root = join(import.meta.dirname, "../../../../../../..");
const { chromium } = createRequire(join(root, "package.json"))("playwright");
const [port, label] = process.argv.slice(2);
const base = `http://127.0.0.1:${port}`;
const index = await (await fetch(`${base}/index.json`)).json();
const ids = Object.keys(index.entries).filter((id) => /table--(with-avatars|selectable-with-avatars|compact-with-avatars)$/u.test(id));
const out = join(import.meta.dirname, "🗑️generated/table");
mkdirSync(out, { recursive: true });
const browser = await chromium.launch({ channel: "chromium" });
const result = {};
for (const id of ids) {
  const page = await (await browser.newContext({ viewport: { width: 1200, height: 800 }, deviceScaleFactor: 1, reducedMotion: "reduce" })).newPage();
  await page.goto(`${base}/iframe.html?id=${id}&viewMode=story`, { waitUntil: "load", timeout: 90000 });
  await page.waitForSelector("tbody > tr", { state: "attached", timeout: 60000 });
  await page.waitForFunction(() => [...document.querySelectorAll("table")].some((t) => t.getBoundingClientRect().height > 0), null, { timeout: 60000 }); await page.waitForTimeout(2500);
  result[id] = await page.evaluate(() => {
    const rect = (element) => { const r = element.getBoundingClientRect(); return [r.top, r.height]; };
    const table = [...document.querySelectorAll("table")].find((t) => t.getBoundingClientRect().height > 0);
    const rows = [...table.querySelectorAll("tbody > tr")];
    const seams = (element) => [...element.querySelectorAll(":scope > svg path, :scope > [data-shared-border-overlay] path")].map((path) => path.getAttribute("d"));
    return {
      table: rect(table), header: rect(table.querySelector("thead")), rows: rows.map(rect),
      rowBorderBottom: rows.map((row) => getComputedStyle(row).borderBottomWidth), headerBorderBottom: getComputedStyle(table.querySelector("thead")).borderBottomWidth,
      rowFrames: rows.map((row) => row.getAttribute("data-shared-border-edges")),
      rowSeams: rows.map(seams), headerSeams: seams(table.querySelector("thead")),
      rootSeams: [...document.querySelectorAll("[data-shared-border-root] > svg path, [data-shared-border-root] svg path")].length,
    };
  });
  await page.screenshot({ path: join(out, `${label}-${id}.png`), animations: "disabled" });
  await page.context().close();
}
await browser.close();
writeFileSync(join(out, `${label}.json`), JSON.stringify(result, null, 1));
console.log(JSON.stringify(result).slice(0, 1500));
