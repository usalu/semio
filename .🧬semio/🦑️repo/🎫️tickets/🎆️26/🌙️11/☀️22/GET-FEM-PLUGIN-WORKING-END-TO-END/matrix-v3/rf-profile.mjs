/** ⏱️ Selection round-trip profile: wraps Worker messaging and long tasks before boot, then clicks two canvas picks N times and prints per-click timelines. Usage: node rf-profile.mjs [clicks] */
import { chromium } from "playwright";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { APPS } from "./apps.mjs";
import { Ctx, sleep } from "./lib.mjs";
const out = fileURLToPath(new URL("../🗑️generated/rf/", import.meta.url));
const clicks = Number(process.argv[2] ?? 6);
const app = { ...APPS["3d"] };
const browser = await chromium.launch();
const x = new Ctx(app, browser, { out, shots: out });
await x.init();
if (process.argv[3] === "patched") {
  const patch = readFileSync(new URL("./rf-job-drive.txt", import.meta.url), "utf8");
  await x.context.route(/shard-worker\.js/, async route => {
    const res = await route.fetch(); let body = await res.text();
    const old = "const step = await actor.api.stepJob(msg.job, msg.budget);\n";
    if (!body.includes(old)) { console.log("PATCH ANCHOR MISSING"); return route.fulfill({ response: res, body }); }
    body = body.replace(old, () => patch.trimStart()); console.log("worker patched");
    return route.fulfill({ response: res, body, headers: { ...res.headers(), "content-length": String(Buffer.byteLength(body)) } });
  });
}
await x.context.addInitScript(() => {
  window.__prof = { msgs: [], long: [], t0: performance.now() };
  const NativeWorker = window.Worker;
  const brief = d => { try { if (d && typeof d === "object") { const t = d.kind ?? d.type ?? ""; const p = d.payload ?? d.turn ?? d.input ?? d.value ?? d; let det = ""; try { det = JSON.stringify(d, (k, v) => (typeof v === "bigint" ? "n" + String(v) : typeof v === "string" && v.length > 60 ? v.slice(0, 60) : v instanceof ArrayBuffer || ArrayBuffer.isView(v) ? "bytes" + v.byteLength : v)).slice(0, 260); } catch { det = "?"; } return t + "|" + det; } return typeof d; } catch { return "?"; } };
  window.Worker = class extends NativeWorker {
    constructor(url, o) { super(url, o); const id = (window.__prof.workers = (window.__prof.workers ?? 0) + 1); this.__id = id; this.addEventListener("message", e => { const now = performance.now(); const tm = e.data && e.data.timings; if (tm && typeof tm.guestEnteredAtEpochMs === "number" && typeof tm.guestLeftAtEpochMs === "number") (window.__prof.tm = window.__prof.tm ?? []).push({ t: now, guest: tm.guestLeftAtEpochMs - tm.guestEnteredAtEpochMs, post: (tm.receivedAtEpochMs ?? 0) - (tm.postedAtEpochMs ?? 0), recv: performance.timeOrigin + now - (tm.repliedAtEpochMs ?? 0), polls: tm.drivePolls ?? 0, stopped: tm.driveStopped ?? "" }); window.__prof.msgs.push({ t: now, dir: "in", w: id, d: brief(e.data) }); }); }
    postMessage(m, t) { window.__prof.msgs.push({ t: performance.now(), dir: "out", w: this.__id, d: brief(m) }); return super.postMessage(m, t); }
  };
  try { new PerformanceObserver(l => { for (const e of l.getEntries()) window.__prof.long.push({ t: e.startTime, dur: e.duration }); }).observe({ entryTypes: ["longtask"] }); } catch {}
});
console.log("boot", await x.boot(300000));
await x.toSummary();
const results = [];
await x.page.evaluate(() => { window.__prof.msgs.length = 0; });
await sleep(4000);
const idle = await x.page.evaluate(() => ({ n: window.__prof.msgs.length, sample: window.__prof.msgs.slice(0, 12).map(m => m.dir + " " + m.d.slice(0, 200)) }));
console.log("idle 4s msgs", idle.n); writeFileSync(`${out}idle.json`, JSON.stringify(idle, null, 1));
for (let i = 0; i < clicks; i++) {
  const spec = i % 2 ? app.nodeA : app.nodeB;
  await x.page.evaluate(() => { window.__prof.msgs.length = 0; window.__prof.long.length = 0; window.__prof.tm = []; });
  const before = (await x.selIds()).join(",");
  await x.hover(spec.at[0], spec.at[1], 1800);
  const tDown = await x.page.evaluate(() => performance.now());
  await x.page.mouse.down(); await x.page.mouse.up();
  const t0 = Date.now(); let ms = -1;
  while (Date.now() - t0 < 30000) { const ids = (await x.selIds()).join(","); if (ids && ids !== before) { ms = Date.now() - t0; break; } await sleep(100); }
  const prof = await x.page.evaluate(d => ({ tm: (() => { const a = (window.__prof.tm ?? []).filter(m => m.t >= d); const sum = k => Math.round(a.reduce((q, m) => q + (m[k] > 0 ? m[k] : 0), 0)); return { n: a.length, guestMs: sum("guest"), postMs: sum("post"), recvMs: sum("recv"), polls: sum("polls") }; })(), now: performance.now(), msgs: window.__prof.msgs.filter(m => m.t >= d).map(m => ({ dt: Math.round(m.t - d), dir: m.dir, w: m.w, d: m.d })), long: window.__prof.long.filter(l => l.t + l.dur >= d).map(l => ({ at: Math.round(l.t - d), dur: Math.round(l.dur) })) }), tDown);
  console.log("timings", JSON.stringify(prof.tm)); results.push({ i, ms, tm: prof.tm, before, after: (await x.selIds()).join(","), msgCount: prof.msgs.length, msgs: prof.msgs.filter(m => m.dir === "out").slice(0, 400), ins: prof.msgs.filter(m => m.dir === "in").slice(0, 60), hist: Object.entries(prof.msgs.reduce((h, m) => { const k = m.dir + " " + m.d.split("|")[0]; h[k] = (h[k] ?? 0) + 1; return h; }, {})), long: prof.long });
  console.log(`click ${i}: ${ms} ms, ${prof.msgs.length} worker msgs, long tasks ${prof.long.map(l => `${l.dur}@${l.at}`).join(" ")}`);
}
writeFileSync(`${out}profile.json`, JSON.stringify(results, null, 1));
await browser.close();
