#!/usr/bin/env bun
/**
 * 🩺️ L1 window-3 train validation (copy of R10's probe) (preamble 14 rule 20 + session-13 rule 33): one `serve s react dev` boot to Home after a
 * kernel-derive-input landing. Starts (or reuses) the local-only `s` serve through S18's shared fixture `ensureDevServe`
 * on L1's port, opens it in ONE headless Chromium, and passes when the shell's ready beacon is set and exactly the Home
 * window `s-home-main` is seated; stops only what it started and always closes the browser.
 * Usage: bun serve-boot-probe.ts [--port 6700] [--hub <url>]
 */
import { awaitBeacon, windowIds } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { ensureDevServe } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const flag = (name: string): string | undefined => (process.argv.indexOf(name) >= 0 ? process.argv[process.argv.indexOf(name) + 1] : undefined);
const port = Number(flag("--port") ?? 6700);
const started = Date.now();
const seconds = (): number => Math.round((Date.now() - started) / 1000);
const serve = await ensureDevServe({ repoRoot: ROOT, port, hubUrl: flag("--hub"), logPath: `${ROOT}/.🧬semio/🌐hub/s14-l1-logs/serve-${port}.log`, onProgress: (_status, line) => console.log(line) });
let verdict = { home: false, beacon: null as string | null, windows: [] as string[], faults: [] as string[] };
try {
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
    page.on("pageerror", (error) => verdict.faults.push(String(error).slice(0, 240)));
    await page.goto(serve.url, { waitUntil: "commit", timeout: 300_000 });
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    const deadline = Date.now() + 120_000;
    let windows: string[] = [];
    while (Date.now() < deadline) {
      windows = await windowIds(page).catch(() => []);
      if (windows.length === 1 && windows[0] === "s-home-main") break;
      await page.waitForTimeout(1_000);
    }
    verdict = { ...verdict, beacon, windows, home: beacon?.startsWith("ready:") === true && windows.length === 1 && windows[0] === "s-home-main" };
  } finally {
    await browser.close();
  }
} finally {
  await serve.stop();
}
console.log(JSON.stringify({ port, reused: serve.reused, seconds: seconds(), ...verdict }));
process.exit(verdict.home ? 0 : 1);
