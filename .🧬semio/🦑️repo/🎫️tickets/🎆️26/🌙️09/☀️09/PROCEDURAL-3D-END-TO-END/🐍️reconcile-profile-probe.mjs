import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🔬️ Names what the HOST main thread runs during the ~10.4 s of console silence that follows every
// `reactor more-work sources=["reconcile"]` answer. The silence-probe proved the thread is neither
// blocked (no long task > 120 ms) nor idle (the MessageChannel heartbeat costs ~20 ms per event-loop
// turn), so the only remaining witness is a sampling profile aggregated over the silent windows.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 130);
const outDir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.env.SEMIO_PROBE_OUT ?? "profile");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
await context.addInitScript(() => { try { window.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const page = await context.newPage();
const t0 = performance.now();
page.on("console", (msg) => lines.push(`${(performance.now() - t0).toFixed(2)} ${msg.type()} ${msg.text().slice(0, 4000)}`));
const cdp = await context.newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.setSamplingInterval", { interval: 1000 });
await cdp.send("Profiler.start");
const profileStart = performance.now() - t0;
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);
const { profile } = await cdp.send("Profiler.stop");
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "profile.cpuprofile"), JSON.stringify(profile));
writeFileSync(join(outDir, "meta.json"), JSON.stringify({ url, seconds, profileStart, startTime: profile.startTime, endTime: profile.endTime, samples: profile.samples.length, nodes: profile.nodes.length }, null, 2));
console.log("DONE lines", lines.length, "samples", profile.samples.length, "profileStartMs", profileStart.toFixed(1));
await browser.close();
