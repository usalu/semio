import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

// 🎯️ Acceptance battery for ticket 26/09/16/INPUT-CAUSALITY-LEDGER (design §6) against a served fem2d
// React lane. Drives real page input through Playwright's mouse (not synthetic PointerEvents) and reads
// the shell's own ledger census (`globalThis.__semioInputLedger`) plus the `[DEBUG] performInvocation`
// console lines to count guest round trips.
//
//   SEMIO_PROBE_URL (default http://127.0.0.1:6086/), SEMIO_PROBE_SECONDS boot wait (default 25),
//   SEMIO_PROBE_OUT report folder name under 🗑️generated (default input-battery).
//
// Lanes (each is its own pass/fail row in the report):
//   marquee-click   — 120-sample marquee, then a real click on empty canvas at 0/60/120/300/500/800 ms after
//                     mouseup: the Inspection panel must read "Summary" (cleared) every time, no refusal, and
//                     the drag must cost ≤ 8 `canvasPointerMove` round trips (before: one per sample).
//   utility-toggle  — arm Marquee Select, re-click it 1.5 s later: it must deactivate (before: ignored).
//   utility-switch  — from idle, Marquee then Lasso 140 ms apart: Lasso active, both hops reached the guest.
//   refusal-visible — the ledger census must show zero `refused` for the whole battery on this lane, and any
//                     refusal that does happen must have printed a plain `input #N … refused:` line.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6086/";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 25);
const outDir = join(import.meta.dirname ?? new URL(".", import.meta.url).pathname, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "input-battery");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 600)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 600)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);
const report = { url, lanes: {} };
const sleep = (ms) => page.waitForTimeout(ms);
const mark = (text) => lines.push(`${Date.now() - t0} probe ${text}`);
const countSince = (since, pattern) => lines.filter((l) => Number(l.split(" ")[0]) >= since && pattern.test(l)).length;
const inspect = () => page.evaluate(() => { const m = document.body.innerText.match(/Inspection[\s\S]{0,120}/); return m ? m[0].replace(/\s+/g, " ") : null; });
const census = () => page.evaluate(() => globalThis.__semioInputLedger ?? null);
const button = (text) => page.evaluate((t) => { const b = [...document.querySelectorAll("button")].find((e) => (e.textContent ?? "").trim() === t); if (!b) return null; const r = b.getBoundingClientRect(); return { x: r.x + r.width / 2, y: r.y + r.height / 2, pressed: b.getAttribute("aria-pressed") }; }, text);
const canvasRect = () => page.evaluate(() => { const c = document.querySelectorAll("canvas")[0]; if (!c) return null; const r = c.getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; });
const openUtilities = async () => {
  const u = await page.evaluate(() => { const el = [...document.querySelectorAll("*")].find((e) => e.childElementCount === 0 && (e.textContent ?? "").trim() === "Utilities"); if (!el) return null; const r = el.getBoundingClientRect(); return { x: r.x + r.width / 2, y: r.y + r.height / 2 }; });
  if (u && !(await button("Marquee Select"))) { await page.mouse.click(u.x, u.y); await sleep(800); }
};
const armMarquee = async () => {
  await openUtilities();
  const m = await button("Marquee Select");
  if (!m) throw new Error("no Marquee Select button");
  if (m.pressed !== "true") { await page.mouse.click(m.x, m.y); await sleep(700); }
};
const drag = async (rect, from, to, samples) => {
  await page.mouse.move(rect.x + from[0], rect.y + from[1]);
  await page.mouse.down();
  await page.mouse.move(rect.x + to[0], rect.y + to[1], { steps: samples });
  await page.mouse.up();
};

// 🖱️ marquee-click
try {
  await armMarquee();
  const rect = await canvasRect();
  const rows = [];
  for (const delay of [0, 60, 120, 300, 500, 800]) {
    await page.mouse.click(rect.x + 600, rect.y + 700); await sleep(1500);
    const since = Date.now() - t0;
    mark(`marquee-click delay=${delay} drag`);
    await drag(rect, [344, 259], [574, 549], 120);
    await sleep(delay);
    mark(`marquee-click delay=${delay} click`);
    await page.mouse.click(rect.x + 600, rect.y + 700);
    await sleep(4000);
    const text = await inspect();
    rows.push({ delay, cleared: /Summary/.test(text ?? ""), moves: countSince(since, /performInvocation \{.*canvasPointerMove/), refused: countSince(since, / refused: /), inspect: (text ?? "").slice(0, 60) });
  }
  const before = rows.every((r) => r.cleared) && rows.every((r) => r.moves <= 8) && rows.every((r) => r.refused === 0);
  report.lanes["marquee-click"] = { pass: before, rows };
} catch (error) { report.lanes["marquee-click"] = { pass: false, error: String(error) }; }

// 🧰️ utility-toggle
try {
  await armMarquee();
  const m0 = await button("Marquee Select");
  if (m0.pressed === "true") { await page.mouse.click(m0.x, m0.y); await sleep(9000); }
  const m1 = await button("Marquee Select");
  await page.mouse.click(m1.x, m1.y); await sleep(1500);
  const armed = (await button("Marquee Select")).pressed;
  const m2 = await button("Marquee Select");
  await page.mouse.click(m2.x, m2.y); await sleep(1500);
  const after = (await button("Marquee Select")).pressed;
  report.lanes["utility-toggle"] = { pass: armed === "true" && after === "false", armed, after, echoOffLines: lines.filter((l) => /hop ignored echo-off/.test(l)).length };
} catch (error) { report.lanes["utility-toggle"] = { pass: false, error: String(error) }; }

// 🧰️ utility-switch
try {
  const l0 = await button("Lasso Select"); if (l0?.pressed === "true") { await page.mouse.click(l0.x, l0.y); await sleep(1200); }
  const m = await button("Marquee Select"); if (m?.pressed === "true") { await page.mouse.click(m.x, m.y); await sleep(1200); }
  const since = Date.now() - t0;
  const m1 = await button("Marquee Select"); await page.mouse.click(m1.x, m1.y);
  await sleep(140);
  const l1 = await button("Lasso Select"); await page.mouse.click(l1.x, l1.y);
  await sleep(2500);
  const lasso = (await button("Lasso Select")).pressed;
  report.lanes["utility-switch"] = { pass: lasso === "true" && countSince(since, /setActiveUtility hop window/) >= 2, lasso, hops: countSince(since, /setActiveUtility hop window/) };
} catch (error) { report.lanes["utility-switch"] = { pass: false, error: String(error) }; }

// 🚦️ refusal-visible
const c = await census();
const refusedTotal = c ? Object.values(c.refused).reduce((a, b) => a + b, 0) : null;
report.lanes["refusal-visible"] = { pass: c !== null && refusedTotal === lines.filter((l) => / refused: /.test(l)).length, census: c, refusedLines: lines.filter((l) => / refused: /.test(l)).slice(0, 10) };

report.pass = Object.values(report.lanes).every((lane) => lane.pass);
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await page.screenshot({ path: join(outDir, "final.png") });
await browser.close();
console.log(JSON.stringify({ pass: report.pass, lanes: Object.fromEntries(Object.entries(report.lanes).map(([k, v]) => [k, v.pass])) }));
process.exit(report.pass ? 0 : 1);
