/** 🖼️ Wgpu PRESENT probe: the black-canvas bisector.
 *
 * The boot is intermittently black on the very same build (`🗑️generated/w3c-plain-1..4`: three
 * painted, one black, identical console), so a single run proves nothing. This probe runs the same
 * boot N times in one process, arms `SEMIO_RUNTIME_DIAGNOSTICS` (optional), screenshots at a fixed
 * cadence, hashes every shot and reports PAINTED/BLACK per run next to the frame-stats dump — so a
 * present-path `[DEBUG]` counter can be read against a known verdict.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=w3c-present SEMIO_PROBE_RUNS=6 bun 🐍️w3c-wgpu-present-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const runs = Number(process.env.SEMIO_PROBE_RUNS ?? 4);
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 25);
const shots = (process.env.SEMIO_PROBE_SHOTS ?? "8,16").split(",").map(Number);
const diagnostics = process.env.SEMIO_PROBE_DIAGNOSTICS !== "0";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w3c-present");
mkdirSync(outDir, { recursive: true });

const digest = (path) => createHash("md5").update(readFileSync(path)).digest("hex").slice(0, 12);
const verdicts = [];
const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
for (let run = 1; run <= runs; run += 1) {
  const runDir = join(outDir, `run-${run}`);
  mkdirSync(runDir, { recursive: true });
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
  const page = await context.newPage();
  const lines = [];
  const t0 = Date.now();
  const record = (source, type, text) => lines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, 4000)}`);
  page.on("console", (msg) => record("page", msg.type(), msg.text()));
  page.on("pageerror", (error) => record("page", "pageerror", String(error)));
  page.on("worker", (worker) => {
    record("page", "info", `worker created ${worker.url()}`);
    worker.on("console", (msg) => record("worker", msg.type(), msg.text()));
  });
  if (diagnostics) {
    await page.addInitScript(() => {
      try {
        globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
      } catch {}
    });
  }
  await page.goto(url, { waitUntil: "domcontentloaded" });
  const hashes = [];
  for (let i = 1; i <= seconds; i += 1) {
    await page.waitForTimeout(1000);
    if (shots.includes(i)) {
      const path = join(runDir, `shot-${i}s.png`);
      await page.screenshot({ path, type: "png" });
      hashes.push(`${i}s:${digest(path)}`);
    }
  }
  if (process.env.SEMIO_PROBE_JIGGLE === "1") {
    for (let step = 0; step < 40; step += 1) {
      await page.mouse.move(200 + (step % 17) * 31, 120 + (step % 11) * 47);
      await page.waitForTimeout(60);
    }
    const jigglePath = join(runDir, "after-jiggle.png");
    await page.screenshot({ path: jigglePath, type: "png" });
    hashes.push(`jiggle:${digest(jigglePath)}`);
  }
  const finalPath = join(runDir, "final.png");
  await page.screenshot({ path: finalPath, type: "png" });
  const finalHash = digest(finalPath);
  const painted = await page.evaluate(() => {
    const canvas = document.querySelector("canvas");
    if (!canvas) return { canvas: false };
    const probe = document.createElement("canvas");
    probe.width = canvas.width;
    probe.height = canvas.height;
    const context = probe.getContext("2d");
    context.drawImage(canvas, 0, 0);
    const data = context.getImageData(0, 0, probe.width, probe.height).data;
    let lit = 0;
    for (let i = 0; i < data.length; i += 4 * 97) {
      if (data[i] > 8 || data[i + 1] > 8 || data[i + 2] > 8) lit += 1;
    }
    return { canvas: true, lit, sampled: Math.ceil(data.length / (4 * 97)) };
  });
  const dumps = await page.evaluate(async () => {
    const introspection = globalThis.semioWgpuIntrospection;
    if (!introspection) return { available: false };
    const read = async (name) => {
      try {
        return await introspection[name]();
      } catch (error) {
        return `<${name} failed: ${error}>`;
      }
    };
    return { available: true, frameStats: await read("dumpFrameStats") };
  });
  writeFileSync(join(runDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(runDir, "dumps.json"), JSON.stringify(dumps, null, 2));
  const verdict = { run, final: finalHash, shots: hashes, painted, frameStats: dumps.frameStats };
  verdicts.push(verdict);
  console.log(`[DEBUG] run ${run} final=${finalHash} ${hashes.join(" ")} lit=${painted.lit}/${painted.sampled} stats=${String(dumps.frameStats ?? "").slice(0, 200)}`);
  await context.close();
}
writeFileSync(join(outDir, "verdicts.json"), JSON.stringify(verdicts, null, 2));
await browser.close();
