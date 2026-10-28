import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { createRequire } from "node:module";
const ticket = resolve(import.meta.dir);
const output = resolve(ticket, "🗑️generated");
mkdirSync(output, { recursive: true });
const root = resolve("🧰️framework/🛍️products/📚️course");
const require = createRequire(resolve(root, "🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json"));
const React = require("react");
const { renderToStaticMarkup } = require("react-dom/server");
const { MockupView } = await import(pathToFileURL(resolve(root, "🎯️targets/⚛️react/🔨️modules/🖥️mockup/🟦️.tsx")).href);
const read = (name: string) => JSON.parse(readFileSync(resolve("🎓️teaching/🏛️architecture", name, "❓️quiz/🔣️.json"), "utf8"));
const excel = read("📊️excel");
const gh = read("🦗️grasshopper");
const items = gh.tasks.flatMap((task: any) => task.items);
const selected = [{ id: "excel", item: excel.tasks[0].items.find((item: any) => item.mockup.target === "formula") ?? excel.tasks[0].items[0] }, ...["parameter", "slider", "panel", "plugin"].map((kind) => ({ id: kind, item: items.find((item: any) => item.mockup.nodes.some((node: any) => node.widget?.kind === kind)) })), { id: "graph", item: items.find((item: any) => item.mockup.wires.length > 0) }];
const css = readFileSync(resolve(root, "🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css"), "utf8");
const html = `<!doctype html><html lang="en"><meta charset="UTF-8"><title>Authored Course Mockups</title><style>:root{--base:white;--foreground:#24312a;--border-normal-color:#9ba79f;--active-base:#d7eadf;--color-warning:#ffd848}*{box-sizing:border-box}body{margin:0;background:#edf1ed;color:var(--foreground);font:16px/1.5 "Segoe UI",sans-serif}main{max-width:1000px;margin:0 auto;padding:16px}article{margin:0 0 16px;padding:20px;background:white;border:1px solid #bbc4bd;border-radius:8px;min-width:0}h1{font-size:20px}h2{margin:0 0 8px;font-size:16px}.sr-only{position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border:0}${css}</style><main><h1>Authored Excel and Grasshopper Reference Workspaces</h1>${selected.map(({ id, item }) => `<article id="${id}"><h2>${item.label.en}</h2>${renderToStaticMarkup(React.createElement(MockupView, { value: item.mockup, locale: "en" }))}</article>`).join("")}</main></html>`;
writeFileSync(resolve(output, "mockup-preview.html"), html);
let chromium;
for (const base of [resolve("package.json"), resolve("🧰️framework/📦️packages/🟦️typescript/package.json"), resolve("🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json")]) {
  for (const name of ["playwright", "@playwright/test"]) {
    try { chromium = createRequire(base)(name).chromium; } catch {}
    if (chromium) break;
  }
  if (chromium) break;
}
if (!chromium) throw Error("No repository Playwright installation resolves");
const systemChromium = ["C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe", "C:/Program Files/Google/Chrome/Application/chrome.exe", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser"].find(existsSync);
const browser = await chromium.launch({ headless: true, ...(systemChromium ? { executablePath: systemChromium } : {}) });
try {
  const page = await browser.newPage({ viewport: { width: 1000, height: 800 }, deviceScaleFactor: 1 });
  await page.setContent(html);
  for (const { id } of selected) await page.locator(`#${id}`).screenshot({ path: resolve(output, `${id}-preview.png`) });
  await page.setViewportSize({ width: 1000, height: await page.evaluate(() => document.body.scrollHeight) });
  await page.screenshot({ path: resolve(output, "mockups-desktop.png"), fullPage: true });
  await page.setViewportSize({ width: 320, height: 720 });
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth > innerWidth);
  if (overflow) console.log("[DEBUG] " + JSON.stringify(await page.evaluate(() => ({ root: document.documentElement.scrollWidth, innerWidth, body: document.body.scrollWidth, offenders: [...document.querySelectorAll("*")].filter((element) => element.getBoundingClientRect().right > innerWidth && !element.closest(".course-mockup-scroll")).map((element) => ({ tag: element.tagName, class: element.className, right: element.getBoundingClientRect().right, width: element.getBoundingClientRect().width, scroll: element.scrollWidth })) }))));
  if (overflow) throw Error("Preview page overflows at 320px");
  await page.setViewportSize({ width: 320, height: await page.evaluate(() => document.body.scrollHeight) });
  await page.screenshot({ path: resolve(output, "mockups-narrow.png"), fullPage: true });
  console.log(JSON.stringify({ output, screenshots: selected.length + 2, narrowOverflow: overflow, status: "passed" }));
} finally { await browser.close(); }
