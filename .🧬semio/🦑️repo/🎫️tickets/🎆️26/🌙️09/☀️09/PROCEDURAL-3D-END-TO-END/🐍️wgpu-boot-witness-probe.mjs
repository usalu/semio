/** 🫀️ wgpu BOOT WITNESS — does `boot_shell` ever return, and where does it stop?
 *
 * The wgpu host ticks on input, so a probe that does not nudge the pointer measures a runtime that
 * was never asked to run (`🐍️console-dump-probe.mjs` on 6118 prints five lines in 170 s for exactly
 * that reason). This one nudges twice a second for the whole window, stops the moment
 * `wgpu-worker boot_shell leave` appears, and reports the LAST line before the silence plus the
 * length of that silence — the two numbers that tell a wedge apart from a slow boot.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_MODE=generate SEMIO_PROBE_OUT=wgpu-input/boot-1 bun 🐍️wgpu-boot-witness-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const mode = process.env.SEMIO_PROBE_MODE ?? "generate";
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${mode ? `&mode=${mode}` : ""}`;
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-input/boot");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

let booted = 0;
for (let tick = 0; tick < seconds * 2; tick += 1) {
  await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
  await page.waitForTimeout(500);
  if (!booted && lines.some((line) => line.includes("boot_shell leave"))) {
    booted = at();
    break;
  }
}

const overlay = await page
  .evaluate(() => {
    const node = Array.from(document.body.children).find((child) => child.tagName === "DIV" && child.id !== "root");
    return node ? (node.textContent ?? "").slice(0, 200) : null;
  })
  .catch(() => null);
await page.screenshot({ path: join(outDir, "shot.png"), type: "png" }).catch(() => {});

const stamp = (line) => Number(line.split(" ", 1)[0]);
const last = lines.at(-1) ?? "";
const verdict = {
  url,
  seconds: Math.round(at() / 1000),
  bootShellLeaveAtMs: booted || null,
  consoleLines: lines.length,
  lastLine: last,
  silenceMs: at() - (Number.isFinite(stamp(last)) ? stamp(last) : 0),
  overlay,
  counts: Object.fromEntries(
    ["boot_shell enter", "boot_shell leave", "render begin", "render leave", "renderSurface", "dock plan", "os_host handle_event", "os_host pointer hit", "dispatch_normalized_event", "world3d surface", "frame build", "surface fault", "panicked"].map((needle) => [
      needle,
      lines.filter((line) => line.includes(needle)).length,
    ]),
  ),
  tail: lines.slice(-15),
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2));
await browser.close();
