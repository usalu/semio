import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 60);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "console-dump");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 2000)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
// 🩺️ SEMIO_PROBE_GUEST_DIAGNOSTICS=1 arms the guest's own `[DEBUG]` hot-path lines: the page reads
// this localStorage key (`shardRuntimeDiagnosticsArmed`, 🎭️actor/🧵️shard-runtime), stamps
// `?diagnostics=1` on the shard worker url, and the worker seeds `SEMIO_RUNTIME_DIAGNOSTICS` into
// `wasi:cli/environment` so `semio_framework_trace::runtime_diagnostics_enabled` answers true.
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);
const hosts = await page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => ({
  surfaceId: el.getAttribute("data-surface-id"),
  status: el.getAttribute("data-status-json")?.slice(0, 1500),
  meshes: (() => { try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); return Array.isArray(v) ? v.length : 0; } catch { return 0; } })(),
})));
await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "hosts.json"), JSON.stringify(hosts, null, 2));
console.log("DONE lines", lines.length, "hosts", JSON.stringify(hosts).slice(0, 600));
await browser.close();
