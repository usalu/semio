/** 🧩️ Ticket tool (work package O2): lays PNG files out as one labelled contact sheet (a page of <img> tags screenshot by Playwright), so many close-ups can be looked at in one image.
 *
 * Usage (from the repository root): node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_montage.mjs" <out.png> <columns> <background> <png>...
 */
import { writeFileSync } from "node:fs";
import { basename, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { chromium } from "playwright";

const [out, columns, background, ...files] = process.argv.slice(2);
const html = `<!doctype html><meta charset="utf-8"><style>body{margin:0;background:${background};font:10px monospace;color:#888;display:grid;grid-template-columns:repeat(${columns},max-content);gap:4px;padding:4px;width:max-content}figure{margin:0}img{display:block}</style>${files.map((file) => `<figure><img src="${pathToFileURL(resolve(file)).href}"><figcaption>${basename(file, ".png")}</figcaption></figure>`).join("")}`;
const page = resolve(out).replace(/\.png$/u, ".html");
writeFileSync(page, html);
const browser = await chromium.launch();
try {
  const tab = await browser.newPage({ viewport: { width: 1600, height: 900 } });
  await tab.goto(pathToFileURL(page).href, { waitUntil: "load" });
  await tab.locator("body").screenshot({ path: resolve(out) });
} finally {
  await browser.close();
}
console.log(`wrote ${resolve(out)}`);
