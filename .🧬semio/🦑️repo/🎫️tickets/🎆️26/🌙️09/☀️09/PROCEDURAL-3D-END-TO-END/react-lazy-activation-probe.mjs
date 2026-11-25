import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Boots the React 6018 page, optionally asks the on-demand door for a node kind by hand (`SMOKE_ASK=1`), and reports activation requests, notices, console errors and the artifact panel's node states over time. */
const out = join(import.meta.dir, "🗑️generated/react-native-smoke"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
page.on("console", m => lines.push(`${m.type()} ${m.text()}`));
page.on("pageerror", e => lines.push(`pageerror ${e.message}`));
page.on("requestfailed", r => lines.push(`requestfailed ${r.url()} ${r.failure()?.errorText}`));
page.on("response", r => { if (r.status() >= 400) lines.push(`http ${r.status()} ${r.url()}`); });
await page.goto(url, { waitUntil: "load" });
const sample = async (label) => {
  const text = await page.evaluate(() => document.body.innerText.replace(/\s+/g, " ").slice(0, 100000));
  const states = [...text.matchAll(/(Brep|Math)\.[A-Za-z.]+ ?(Queued|Computing|Ok|Done|Error|Failed)?/g)].map(m => m[0]).slice(0, 12);
  const notices = [...document_notice(text)];
  lines.push(`[probe] ${label} states=${JSON.stringify(states)} notices=${JSON.stringify(notices)}`);
};
const document_notice = (text) => text.match(/(Loading|Preparing) extension [^.…]+…|Extension [^.]+ is (ready|not available)\.|Erweiterung [^.…]+ (wird [a-z]+|ist [a-z]+)…?/g) ?? [];
const total = Number(process.env.SMOKE_MS ?? 90000);
let elapsed = 0;
while (elapsed < total) {
  await page.waitForTimeout(10000); elapsed += 10000;
  if (elapsed === 20000 && process.env.SMOKE_ASK === "1") for (const nodeKind of ["brep.solid.extrude", "math.vector"]) lines.push(`[probe] ask ${nodeKind} ${await page.evaluate((kind) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: kind })), nodeKind)}`);
  await sample(`t=${elapsed / 1000}s`);
}
await page.screenshot({ path: join(out, process.env.SMOKE_SHOT ?? "lazy.png") });
writeFileSync(join(out, process.env.SMOKE_LOG ?? "lazy-console.txt"), lines.join("\n"));
const bad = lines.filter(l => /^error|pageerror|panicked|^http [45]|requestfailed/.test(l));
console.log(lines.filter(l => l.startsWith("[probe]")).join("\n"));
console.log(`${lines.length} console lines; ${bad.length} errors`); console.log(bad.slice(0, 15).join("\n"));
await browser.close();
