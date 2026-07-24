#!/usr/bin/env bun
/** Runtime check: parent shell edge emphasis matches window silhouette hover. */
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const ticketDir = dirname(fileURLToPath(import.meta.url));

const html = `<!doctype html>
<html>
<head>
<style>
:root {
  --color-gray: #7b827d;
  --color-dark: #06171c;
  --color-primary: #e11d48;
  --border-normal-color: var(--color-gray);
  --border-emphasized-color: var(--color-dark);
  --stroke-hairline: 1px;
  --active-base: var(--color-primary);
}
body { margin: 0; font: 14px sans-serif; background: #f1edde; color: #06171c; }
[data-slot="navbar"], [data-slot="footer"] {
  position: relative;
  height: 40px;
  background: #dfddd0;
  border-width: 0 !important;
  border-style: none !important;
}
[data-slot="navbar"]::after,
[data-slot="footer"]::before {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  height: var(--stroke-hairline);
  background-color: var(--border-normal-color);
  pointer-events: none;
  z-index: 50;
}
[data-slot="navbar"]::after { bottom: 0; }
[data-slot="footer"]::before { top: 0; }
[data-slot="navbar"]:hover::after,
[data-slot="footer"]:hover::before {
  background-color: var(--border-emphasized-color);
}
[data-slot="panel"], [data-slot="pane"] {
  position: relative;
  width: 200px;
  height: 120px;
  margin: 16px;
  background: #c4c4b9;
}
[data-slot="chrome-frame"] {
  pointer-events: none;
  position: absolute;
  inset: 0;
  box-sizing: border-box;
  background: transparent;
  border-width: var(--stroke-hairline);
  border-style: solid;
  border-color: var(--border-normal-color) !important;
}
:is([data-slot="panel"], [data-slot="pane"]):hover [data-slot="chrome-frame"] {
  border-color: var(--border-emphasized-color) !important;
}
[data-slot="mode-dock-stack"] {
  position: relative;
  width: 240px;
  height: 140px;
  margin: 16px;
  background: #c4c4b9;
}
.window-silhouette-border-normal { stroke: var(--border-normal-color); }
[data-slot="mode-dock-stack"]:not([data-active="true"]):hover [data-slot="mode-dock-silhouette-border"][data-kind="normal"] path {
  stroke: var(--border-emphasized-color) !important;
}
</style>
</head>
<body>
  <nav data-slot="navbar">navbar</nav>
  <footer data-slot="footer">footer</footer>
  <div data-slot="panel"><div data-slot="chrome-frame"></div>panel</div>
  <div data-slot="pane"><div data-slot="chrome-frame"></div>pane</div>
  <div data-slot="mode-dock-stack">
    <svg data-slot="mode-dock-silhouette-border" data-kind="normal" width="240" height="140">
      <path class="window-silhouette-border window-silhouette-border-normal" d="M1 1 H239 V139 H1 Z" fill="none" stroke="var(--border-normal-color)" />
    </svg>
    window
  </div>
</body>
</html>`;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(html, { waitUntil: "load" });

async function read(selector: string, pseudo: string | null, prop: string) {
  return page.$eval(
    selector,
    (el, args) => {
      const style = args.pseudo ? getComputedStyle(el, args.pseudo) : getComputedStyle(el);
      return style.getPropertyValue(args.prop).trim();
    },
    { pseudo, prop },
  );
}

const cases = [
  { name: "navbar", hover: '[data-slot="navbar"]', target: '[data-slot="navbar"]', pseudo: "::after", prop: "background-color" },
  { name: "footer", hover: '[data-slot="footer"]', target: '[data-slot="footer"]', pseudo: "::before", prop: "background-color" },
  { name: "panel", hover: '[data-slot="panel"]', target: '[data-slot="panel"] [data-slot="chrome-frame"]', pseudo: null, prop: "border-top-color" },
  { name: "pane", hover: '[data-slot="pane"]', target: '[data-slot="pane"] [data-slot="chrome-frame"]', pseudo: null, prop: "border-top-color" },
  { name: "window", hover: '[data-slot="mode-dock-stack"]', target: '[data-slot="mode-dock-stack"] path', pseudo: null, prop: "stroke" },
] as const;

const results: Record<string, { rest: string; hover: string }> = {};
let failed = false;

for (const c of cases) {
  const rest = await read(c.target, c.pseudo, c.prop);
  await page.hover(c.hover);
  await page.waitForTimeout(30);
  const hover = await read(c.target, c.pseudo, c.prop);
  results[c.name] = { rest, hover };
  if (rest === hover) {
    console.error(`[DEBUG] FAIL ${c.name}: rest===hover (${rest})`);
    failed = true;
  } else {
    console.log(`[DEBUG] OK ${c.name}: ${rest} -> ${hover}`);
  }
  await page.mouse.move(0, 0);
  await page.waitForTimeout(30);
}

await Bun.write(join(ticketDir, "playwright-parent-hover.json"), JSON.stringify(results, null, 2));
await browser.close();
if (failed) process.exit(1);
console.log("[DEBUG] all parent-hover emphasis cases passed");
