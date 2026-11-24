/** 🔎️ Prints on the dev page the outline of the element tree under a selector of a quiz's page: boxes, tags and data attributes. Usage: node dom-probe.mjs [quiz] [selector] [depth] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const quiz = process.argv[2] ?? "heating";
const selector = process.argv[3] ?? "#course-main";
const depth = Number(process.argv[4] ?? 6);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[5] ?? 1280), height: Number(process.argv[6] ?? 720) } });
await page.goto("http://127.0.0.1:6061/#" + quiz);
await page.waitForSelector(`[data-layered-pane="${quiz}"][data-opened] [data-pet-prop]`, { timeout: 180000 });
await page.waitForTimeout(2000);
console.log(await page.evaluate(({ selector, depth }) => {
  const lines = [];
  const walk = (e, level) => {
    const r = e.getBoundingClientRect();
    if (r.width === 0 || r.right <= 0 || r.left >= innerWidth || e.closest("[inert]")) return;
    const data = [...e.attributes].filter((a) => a.name.startsWith("data-") || a.name === "id" || a.name === "role").map((a) => `[${a.name}${a.value ? "=" + a.value.slice(0, 20) : ""}]`).join("");
    const own = [...e.childNodes].filter((n) => n.nodeType === 3).map((n) => n.textContent.trim()).join(" ").slice(0, 40);
    lines.push("  ".repeat(level) + [r.x, r.y, r.width, r.height].map(Math.round).join(",") + " " + e.tagName.toLowerCase() + data + (own ? ` "${own}"` : ""));
    if (level < depth) for (const child of e.children) walk(child, level + 1);
  };
  for (const e of document.querySelectorAll(selector)) walk(e, 0);
  return lines.join("\n");
}, { selector, depth }));
await browser.close();
