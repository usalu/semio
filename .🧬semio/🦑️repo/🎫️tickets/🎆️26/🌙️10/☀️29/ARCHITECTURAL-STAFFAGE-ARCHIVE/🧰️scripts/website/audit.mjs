import { chromium } from "playwright";
const [hash, scheme = "light"] = process.argv.slice(2);
const browser = await chromium.launch();
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US", colorScheme: scheme })).newPage();
page.on("pageerror", (e) => console.log("pageerror", String(e).slice(0, 300)));
page.on("console", (m) => m.type() === "error" && console.log("console error", m.text().slice(0, 200)));
await page.goto("http://localhost:6071/" + hash, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(12000);
const out = await page.evaluate(() => {
  const heads = [...document.querySelectorAll("h1,h2,h3,h4")].filter((e) => !e.closest("[inert],[aria-hidden=true]") && e.getClientRects().length > 0 || e.classList.contains("sr-only")).map((e) => `${e.tagName}:${e.textContent.trim().slice(0, 40)}`);
  const lum = (c) => { const [r, g, b] = c.match(/\d+(\.\d+)?/g).map(Number).slice(0, 3).map((v) => { v /= 255; return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; }); return 0.2126 * r + 0.7152 * g + 0.0722 * b; };
  const bgOf = (e) => { for (let n = e; n; n = n.parentElement) { const c = getComputedStyle(n).backgroundColor; if (c && !/rgba\(0, 0, 0, 0\)|transparent/.test(c)) return c; } return "rgb(255,255,255)"; };
  const low = [];
  for (const e of document.querySelectorAll("p,span,label,legend,td,th,li,a,button,h3,figcaption")) {
    if (e.closest("[inert],[aria-hidden=true],svg") || !e.childNodes.length || ![...e.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim()) || e.getClientRects().length === 0) continue;
    const fg = getComputedStyle(e).color, bg = bgOf(e);
    const a = lum(fg), b = lum(bg), ratio = (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
    if (ratio < 4.5) low.push(`${ratio.toFixed(2)} ${e.tagName} ${e.textContent.trim().slice(0, 30)}`);
  }
  return { heads, low: [...new Set(low)].slice(0, 12) };
});
console.log(JSON.stringify(out, null, 1));
await browser.close();
