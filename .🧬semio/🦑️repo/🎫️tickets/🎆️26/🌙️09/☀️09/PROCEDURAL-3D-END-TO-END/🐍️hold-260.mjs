import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = "http://127.0.0.1:6018/?plugin=generation3d";
const holdMs = 260_000;
const outDir = process.argv[2];
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-webgl", "--ignore-gpu-blocklist", "--disable-gpu-sandbox", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});
const page = await browser.newPage();
page.on("console", (msg) => {
  const text = msg.text();
  lines.push({ at: new Date().toISOString(), level: msg.type(), text });
});
page.on("pageerror", (err) => {
  lines.push({ at: new Date().toISOString(), level: "pageerror", text: String(err) });
});
const t0 = Date.now();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForTimeout(holdMs);
const elapsed = Date.now() - t0;
const rustOom = lines.some((row) => /rust_oom|process::abort|unreachable/.test(row.text));
const pushes = lines.filter((row) => row.text.includes("[DEBUG] contributions push") || row.text.includes("contributions document sources"));
const invoke = lines.filter((row) => /invokeExtension/.test(row.text));
const summary = {
  elapsedMs: elapsed,
  rustOom,
  pushLines: pushes.map((row) => row.text.slice(0, 500)),
  invokeCount: invoke.length,
  totalConsole: lines.length,
};
writeFileSync(join(outDir, "hold-260-console.jsonl"), lines.map((row) => JSON.stringify(row)).join("\n") + "\n");
writeFileSync(join(outDir, "hold-260-summary.json"), JSON.stringify(summary, null, 2));
console.log(JSON.stringify(summary));
await browser.close();
