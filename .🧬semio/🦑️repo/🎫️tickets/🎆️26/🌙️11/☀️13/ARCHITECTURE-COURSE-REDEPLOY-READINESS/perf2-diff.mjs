/** 🔍️ Pixel difference of the screenshots of two builds (`perf2-look.mjs` folders): pixels that differ at all and by more than 8 of 255 in a channel, per image. Usage: node perf2-diff.mjs <folder A> <folder B>. */
import { createRequire } from "node:module";
import { readdirSync, readFileSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const [a, b] = process.argv.slice(2);
const browser = await chromium.launch();
const page = await browser.newPage();
for (const name of readdirSync(a).filter((f) => f.endsWith(".png"))) {
  const one = readFileSync(`${a}/${name}`).toString("base64"), two = readFileSync(`${b}/${name}`).toString("base64");
  const r = await page.evaluate(async ([x, y]) => {
    const load = (s) => new Promise((ok) => { const i = new Image(); i.onload = () => ok(i); i.src = "data:image/png;base64," + s; });
    const [p, q] = await Promise.all([load(x), load(y)]);
    const data = (i) => { const c = document.createElement("canvas"); c.width = i.width; c.height = i.height; const g = c.getContext("2d"); g.drawImage(i, 0, 0); return g.getImageData(0, 0, i.width, i.height).data; };
    const d1 = data(p), d2 = data(q); let any = 0, big = 0;
    for (let i = 0; i < d1.length; i += 4) { const m = Math.max(Math.abs(d1[i] - d2[i]), Math.abs(d1[i + 1] - d2[i + 1]), Math.abs(d1[i + 2] - d2[i + 2])); if (m > 0) any++; if (m > 8) big++; }
    return { pixels: d1.length / 4, any, big };
  }, [one, two]);
  console.log(name.padEnd(22), JSON.stringify(r));
}
await browser.close();
