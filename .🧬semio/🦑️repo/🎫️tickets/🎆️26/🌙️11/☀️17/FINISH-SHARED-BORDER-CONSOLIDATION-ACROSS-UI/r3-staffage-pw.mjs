import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright-core/index.mjs";
const T = new URL(".", import.meta.url).pathname;
const base = "http://127.0.0.1:5471/";
const browser = await chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", headless: true });
const results = [];
const sizes = process.argv[2] === "mobile" ? [[375, 812]] : [[1440, 900]];
const routes = process.argv[3] ? process.argv[3].split(",") : ["", "all", "animals", "humans/walking-commuter", "animals/domestic-cat", "furniture/cafe-chair", "appliances/circulation-pump", "collections", "compare", "playground"];
for (const [w, hgt] of sizes) {
  const ctx = await browser.newContext({ viewport: { width: w, height: hgt } });
  const page = await ctx.newPage();
  page.on("pageerror", e => results.push("PAGEERROR " + e.message.slice(0, 120)));
  await page.goto(base);
  await page.waitForTimeout(2500);
  const probe = async label => {
    const r = await page.evaluate(async ({ T }) => {
      const h = await import("/@fs" + encodeURI(T + "staffage-helpers.js") + "?t=" + Date.now());
      return await h.go(null, { wait: 500, top: 6 });
    }, { T });
    const m = /"count":(\d+)/.exec(r);
    results.push(`${w}x${hgt} ${label} count=${m ? m[1] : "?"} ${m && m[1] !== "0" ? r.slice(0, 900) : ""}`);
  };
  for (const route of routes) {
    await page.evaluate(r => { location.hash = "#/" + r; }, route);
    await page.waitForTimeout(3500);
    await probe(route || "overview");
    if (route === "collections") {
      await page.evaluate(() => { const p = document.querySelector('[data-page="collections"]'); if (p) p.scrollTop = p.scrollHeight; });
      await page.waitForTimeout(600);
      await probe(route + " bottom");
    }
    if (/\//.test(route) && route !== "") {
      for (const name of ["Three views", "3D model"]) {
        const ok = await page.evaluate(n => { const b = [...document.querySelectorAll("[role=radio]")].find(x => x.textContent.trim() === n); if (!b) return false; b.click(); return true; }, name);
        await page.waitForTimeout(name === "3D model" ? 5000 : 2500);
        if (ok) await probe(`${route} ${name}`);
        if (ok && name === "Three views") results.push("  unavailable cells: " + await page.evaluate(() => document.querySelectorAll("[data-notice=unavailable]").length));
        if (ok && name === "3D model") {
          const st = await page.evaluate(() => { const s = document.querySelector("[data-model-stage]"); return s && s.dataset.state; });
          results.push("  stage state " + st);
          await page.screenshot({ path: `${T}🗑️generated/r3-${route.replace("/", "-")}-${w}.png` });
        }
      }
    }
  }
  await ctx.close();
}
await browser.close();
console.log(results.join("\n"));
