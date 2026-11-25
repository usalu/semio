/** 🎬️ Law: the FEM 2D editor opens its Demo example even when the first `start-job` round also pays the guest's deferred
 * return housekeeping (`copiedItems > 1`), the round after which the 08:16 build refused the `source: null` retry with
 * `authority (invariant-violated)`. Each run boots a fresh context, picks Demo and fails on any refused job call, failed
 * document load or editor load error; the law also fails when no run exercised the housekeeping round.
 * Usage: node demo-load-law.mjs [url] [runs]. See demo-load-refusal.md. */
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/", runs = Number(process.argv[3] ?? 6);
const probe = 'guest.jobs.startJob(job, budget).then((r) => (console.warn("[LAW] start-round " + String(r.spent.copiedItems) + " " + String(r.admitted) + " " + JSON.stringify(r.refusal ?? null)), r))';
const browser = await chromium.launch();
const results = [];
try {
  for (let run = 0; run < runs; run++) {
    const context = await browser.newContext({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
    let probed = false;
    await context.route(u => decodeURIComponent(u.pathname).endsWith("/🌉️bridge.js"), async route => {
      const response = await route.fetch(), source = await response.text(), body = source.replace("guest.jobs.startJob(job, budget)", probe);
      probed = body !== source;
      await route.fulfill({ response, body });
    });
    const page = await context.newPage(), rounds = [], faults = [];
    page.on("console", m => {
      const text = m.text();
      if (text.startsWith("[LAW] start-round ")) rounds.push(text.slice(18));
      else if (/original-job\.call-refused|load-failed|native-return-refused/.test(text)) faults.push(text.slice(0, 300));
    });
    page.on("pageerror", error => faults.push(`pageerror ${error.message.slice(0, 300)}`));
    try {
      await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
      const combo = page.getByRole("combobox", { name: "Example", exact: true });
      await combo.waitFor({ state: "visible", timeout: 180000 });
      await combo.click();
      await page.getByRole("option", { name: "Demo", exact: true }).click();
      await page.waitForTimeout(20000);
      if (await page.getByText("could not be loaded").count() > 0) faults.push("editor could not be loaded");
    } finally {
      await context.close();
    }
    const hazard = rounds.some(round => Number(round.split(" ")[0]) > 1);
    const pass = probed && faults.length === 0 && rounds.some(round => round.split(" ")[1] === "true");
    results.push({ run, pass, hazard, rounds: rounds.slice(0, 4), faults: faults.slice(0, 4) });
    console.log(JSON.stringify(results.at(-1)));
  }
} finally {
  await browser.close();
}
const passed = results.every(result => result.pass), exercised = results.filter(result => result.hazard).length;
console.log(JSON.stringify({ runs, passed, exercised }));
process.exit(passed && exercised > 0 ? 0 : 1);
