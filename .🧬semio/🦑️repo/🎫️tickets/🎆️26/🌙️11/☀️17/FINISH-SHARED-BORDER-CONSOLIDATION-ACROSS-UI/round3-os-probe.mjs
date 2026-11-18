import { createRequire } from "node:module";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { chromium } = require("playwright-core");
const here = dirname(fileURLToPath(import.meta.url));
const parts = process.argv.slice(2);
const browser = await chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", headless: true });
for (const [w, h] of [[1440, 900], [375, 812]]) {
  const page = await browser.newPage({ viewport: { width: w, height: h } });
  page.on("pageerror", (e) => console.log("PAGEERROR", String(e).slice(0, 300)));
  await page.goto("http://127.0.0.1:5481/", { waitUntil: "load" });
  for (const part of parts) {
    try {
      const out = await page.evaluate(async ([dir, part]) => { const m = await import("/@fs" + encodeURI(dir) + "/round3-os-run.js?t=" + Date.now()); return await m.run(part, { wait: 900 }); }, [here, part]);
      console.log(w, out);
    } catch (e) { console.log(w, part, "ERR", String(e).slice(0, 400)); }
  }
  if (process.env.SHOT) await page.screenshot({ path: `${here}/🗑️generated/r3os-${process.env.SHOT}-${w}.png` });
  await page.close();
}
await browser.close();
