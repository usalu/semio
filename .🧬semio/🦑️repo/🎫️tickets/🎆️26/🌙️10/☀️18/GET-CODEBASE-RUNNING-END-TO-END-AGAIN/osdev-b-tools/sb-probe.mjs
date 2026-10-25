import { chromium } from "playwright";
const base = process.argv[2] ?? "http://127.0.0.1:6010";
const count = Number(process.argv[3] ?? 8);
const idx = await (await fetch(`${base}/index.json`)).json();
const entries = Object.values(idx.entries).filter((e) => e.type === "story");
console.log(`[probe] ${entries.length} stories indexed`);
const step = Math.max(1, Math.floor(entries.length / count));
const picks = entries.filter((_, i) => i % step === 0).slice(0, count);
const browser = await chromium.launch({ headless: true });
const retries = 2;
const results = [];
for (const e of picks) {
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (x) => errors.push(String(x).slice(0, 200)));
  page.on("console", (m) => m.type() === "error" && errors.push(m.text().slice(0, 200)));
  await page.goto(`${base}/iframe.html?id=${e.id}&viewMode=story`, { waitUntil: "commit", timeout: 300000 }).catch((x) => errors.push("goto " + x.message.slice(0, 100)));
  let state = "timeout";
  for (let i = 0; i < 240; i++) {
    state = await page.evaluate(() => {
      if (!document.body) return "pending"; const b = document.body.className;
      if (document.querySelector(".sb-errordisplay") && getComputedStyle(document.querySelector(".sb-errordisplay")).display !== "none") return "error: " + (document.querySelector(".sb-errordisplay").innerText || "").slice(0, 300);
      const r = document.querySelector("#storybook-root");
      if (b.includes("sb-show-main") && r && r.children.length) return "ok";
      return "pending";
    }).catch(() => "pending");
    if (state !== "pending") break;
    await page.waitForTimeout(1000);
  }
  results.push({ id: e.id, state, errors: errors.slice(0, 3) });
  console.log(JSON.stringify(results.at(-1)));
  await page.close();
}
await browser.close();
const bad = results.filter((r) => r.state !== "ok");
console.log(`[probe] ${results.length - bad.length}/${results.length} ok`);
process.exit(bad.length ? 1 : 0);
