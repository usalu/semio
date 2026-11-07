const fs = require("fs");
const postcss = require("postcss");
const { chromium } = require("playwright");
const base = "🧰️framework/🔨️modules/🖱️ui/";
const fixtures = JSON.parse(fs.readFileSync(base + "🧱️elements/🗂️WindowChrome/compositor.cases.json")).nativeSeams;
const css = postcss.parse(fs.readFileSync(base + "🎨️styling/🖌️ui/🎨️.css", "utf8"));
const layers = css.nodes.filter(n => n.type === "atrule" && n.name === "layer" && n.params === "shared-border-priority").map(n => n.toString()).join("\n");
(async () => {
  const browser = await chromium.launch({ headless: true, channel: process.platform === "win32" && !fs.existsSync(chromium.executablePath()) ? "msedge" : undefined });
  try {
    const page = await browser.newPage();
    for (const f of fixtures) {
      await page.setContent('<style>' + layers + '</style><div dir="' + f.dir + '" data-shared-border-group data-orientation="' + f.orientation + '" style="--active-base:red;--loading-border-color:blue;--introduced-border-color:green"><button data-shared-border-cell ' + f.first + '></button><button data-shared-border-cell ' + f.second + '>' + (f.secondContent ?? '') + '</button></div>');
      const actual = await page.locator("[data-shared-border-cell]").first().evaluate(e => getComputedStyle(e).borderColor);
      if (actual !== f.color) throw Error(f.name + ": " + actual + " != " + f.color);
      console.log(f.name + ": " + actual);
    }
    await page.setContent('<style>' + layers + '</style><div aria-disabled="true" style="--border-emphasized-color:red"><button data-shared-border-cell>one</button><button data-shared-border-cell>two</button></div>');
    await page.locator("[data-shared-border-cell]").last().hover();
    const disabled = await page.locator("[data-shared-border-cell]").first().evaluate(e => getComputedStyle(e).borderColor);
    if (disabled === "rgb(255, 0, 0)") throw Error("disabled ancestor hover");
    console.log("disabled ancestor hover: " + disabled);
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
