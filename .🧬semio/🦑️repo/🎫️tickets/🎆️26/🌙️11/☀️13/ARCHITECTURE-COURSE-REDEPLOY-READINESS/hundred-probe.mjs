/** 🧮️ Measures what one real browser learner costs at rest on the course home: opens PROBE_PAGES pages (one context each) of the static stack at PROBE_BASE in one Chromium launched with PROBE_ARGS (space-separated), closes the introduction, waits, and prints the summed resident memory of the Chromium process tree per page. PROBE_VIEWPORT=WxH sets the window, PROBE_TRANSPARENCY=reduce asks for reduced transparency, PROBE_SHARED=1 opens every page in one context on its own origin pN.localhost (same site, so pages may share renderers, see --renderer-process-limit). Usage: PROBE_ARGS="--enable-low-end-device-mode" node hundred-probe.mjs */
import { execFileSync } from "node:child_process";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const repo = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const { chromium } = await import(resolve(repo, ".cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/index.mjs"));
const pages = Number(process.env.PROBE_PAGES ?? 10);
const base = process.env.PROBE_BASE ?? "http://127.0.0.1:6071/";
const extra = (process.env.PROBE_ARGS ?? "").split(" ").filter(Boolean);
const [width, height] = (process.env.PROBE_VIEWPORT ?? "1280x800").split("x").map(Number);
const viewport = { width, height };

/** 🌳️ Every process below `root`, the root included. */
function tree(root) {
  const rows = execFileSync("ps", ["-axo", "pid=,ppid=,rss="], { encoding: "utf8" }).trim().split("\n").map((line) => line.trim().split(/\s+/u).map(Number));
  const found = new Set([root]);
  for (let grew = true; grew; ) {
    grew = false;
    for (const [pid, ppid] of rows) if (found.has(ppid) && !found.has(pid)) (found.add(pid), (grew = true));
  }
  return rows.filter(([pid]) => found.has(pid));
}

const browser = await chromium.launch({ headless: true, args: ["--disable-gpu", "--disable-dev-shm-usage", "--mute-audio", "--no-first-run", ...extra] });
const opened = [];
const shared = process.env.PROBE_SHARED === "1" ? await browser.newContext({ viewport }) : undefined;
for (let index = 0; index < pages; index++) {
  const context = shared ?? (await browser.newContext({ viewport }));
  const page = await context.newPage();
  if (process.env.PROBE_TRANSPARENCY === "reduce") await (await context.newCDPSession(page)).send("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-transparency", value: "reduce" }] });
  await page.goto(shared === undefined ? base : base.replace("127.0.0.1", `p${index}.localhost`));
  await page.locator('[data-slot="introduction-close"]').first().click({ timeout: 120_000 }).catch(() => undefined);
  opened.push(page);
}
await new Promise((rest) => setTimeout(rest, 15_000));
const procs = tree(process.pid).filter(([pid]) => pid !== process.pid);
const totalMb = procs.reduce((sum, [, , rss]) => sum + rss, 0) / 1024;
const renderers = execFileSync("ps", ["-axo", "pid=,command="], { encoding: "utf8" }).split("\n").filter((line) => procs.some(([pid]) => line.trim().startsWith(`${pid} `)) && line.includes("--type=renderer")).length;
console.log(JSON.stringify({ args: extra, pages, renderers, processes: procs.length, totalMb: Math.round(totalMb), perPageMb: Math.round(totalMb / pages) }));
await browser.close();
