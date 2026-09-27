/** 🎥️ AV2 real-browser proof of `🎥️VideoRenderHost` (overlay): bundles `src/browser-entry.ts`, serves it with the demo figure on
 * `--port` (AV2 range 6590–6599), runs it in the repository's Playwright Chromium, saves both tiers' MP4s under
 * `generated/browser/` and has FFmpeg read them: stream facts, key frames, full decode, and the two tiers' decoded frames
 * compared (PSNR) at one frame per slide. Usage: bun av2-browser-host.ts [--port 6591] [--headed] */
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const port = Number(process.argv[process.argv.indexOf("--port") + 1] ?? 6591) || 6591;
process.env.PLAYWRIGHT_BROWSERS_PATH ??= "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright";
const { chromium } = await import("/Users/ueli/Documents/semio/node_modules/playwright/index.mjs");
const out = join(import.meta.dir, "generated", "browser");
mkdirSync(out, { recursive: true });
const built = await Bun.build({ entrypoints: [join(import.meta.dir, "src", "browser-entry.ts")], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, minify: { syntax: true }, plugins: [{ name: "av2-node-only", setup(build) { build.onResolve({ filter: /^(node|bun):sqlite$/ }, (args) => ({ path: args.path, external: true })); } }] });
if (!built.success) throw new Error(built.logs.map(String).join("\n"));
const bundle = await built.outputs[0]!.text();
const figure = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🖼️images/🏙️architecture/🏘️habitat-67.png");
const server = Bun.serve({
  port,
  hostname: "127.0.0.1",
  async fetch(request) {
    const url = new URL(request.url);
    if (url.pathname === "/") return new Response(`<!doctype html><meta charset="utf-8"><title>av2</title><script type="module" src="/bundle.js"></script>`, { headers: { "content-type": "text/html" } });
    if (url.pathname === "/bundle.js") return new Response(bundle, { headers: { "content-type": "text/javascript" } });
    if (url.pathname === "/figure.png") return new Response(figure, { headers: { "content-type": "image/png" } });
    if (url.pathname.startsWith("/result/") && request.method === "POST") {
      writeFileSync(join(out, url.pathname.slice("/result/".length)), new Uint8Array(await request.arrayBuffer()));
      return new Response("ok");
    }
    return new Response("not found", { status: 404 });
  },
});
const browser = await chromium.launch({ headless: !process.argv.includes("--headed") });
let result: unknown;
try {
  const page = await browser.newPage();
  page.on("console", (message: { text(): string }) => console.log(`[page] ${message.text()}`));
  page.on("pageerror", (error: Error) => console.log(`[pageerror] ${error.message}`));
  await page.goto(`http://127.0.0.1:${port}/`);
  result = await page.evaluate(async () => await (globalThis as { __av2?: Promise<unknown> }).__av2, undefined, { timeout: 300_000 });
  console.log(`[av2-browser] ${JSON.stringify(result)}`);
} finally {
  await browser.close();
  server.stop(true);
}

const run = (command: string, args: string[]): string => {
  const done = spawnSync(command, args, { maxBuffer: 1 << 30 });
  if (done.status !== 0) throw new Error(`${command} ${args.join(" ")}: ${done.stderr.toString()}`);
  return done.stdout.toString();
};
const checks: string[] = [];
for (const tier of ["platform", "first-party"]) {
  const file = join(out, `deck-${tier}.mp4`);
  try {
    const stream = JSON.parse(run("ffprobe", ["-v", "error", "-count_frames", "-select_streams", "v:0", "-show_entries", "stream=codec_name,profile,width,height,pix_fmt,r_frame_rate,nb_read_frames", "-of", "json", file])).streams[0];
    const keys = (JSON.parse(run("ffprobe", ["-v", "error", "-select_streams", "v:0", "-show_entries", "frame=key_frame", "-of", "json", file])).frames as { key_frame: number }[]).flatMap((frame, index) => (frame.key_frame ? [index + 1] : []));
    run("ffmpeg", ["-v", "error", "-i", file, "-f", "null", "-"]);
    checks.push(`${tier}: ${stream.codec_name} ${stream.profile} ${stream.width}x${stream.height} ${stream.pix_fmt} ${stream.r_frame_rate} frames=${stream.nb_read_frames} keys=${keys.join(",")} full-decode=ok bytes=${readFileSync(file).length}`);
  } catch (error) {
    checks.push(`${tier}: FAILED ${error instanceof Error ? error.message : String(error)}`);
  }
}
const gray = (file: string, frame: number): Uint8Array => spawnSync("ffmpeg", ["-v", "error", "-i", file, "-vf", `select=eq(n\\,${frame})`, "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "gray", "-"], { maxBuffer: 1 << 28 }).stdout;
const stats = (plane: Uint8Array): { mean: number; deviation: number } => {
  let sum = 0;
  let squares = 0;
  for (const value of plane) {
    sum += value;
    squares += value * value;
  }
  const mean = sum / Math.max(1, plane.length);
  return { mean: Math.round(mean * 10) / 10, deviation: Math.round(Math.sqrt(Math.max(0, squares / Math.max(1, plane.length) - mean * mean)) * 10) / 10 };
};
const slides = [20, 60, 90, 120, 150];
const firstParty = slides.map((frame) => gray(join(out, "deck-first-party.mp4"), frame));
const platformFrames = slides.map((frame) => gray(join(out, "deck-platform.mp4"), frame));
for (const [index, frame] of slides.entries()) {
  const a = firstParty[index]!;
  const b = platformFrames[index]!;
  let error = 0;
  for (let at = 0; at < Math.min(a.length, b.length); at += 1) error += (a[at]! - b[at]!) ** 2;
  const mse = error / Math.max(1, Math.min(a.length, b.length));
  const psnr = mse === 0 ? Infinity : 10 * Math.log10((255 * 255) / mse);
  checks.push(`frame ${frame}: first-party luma ${JSON.stringify(stats(a))} platform luma ${JSON.stringify(stats(b))} psnr(first-party,platform)=${psnr.toFixed(1)} dB bytes=${a.length}/${b.length}`);
}
const distinct = new Set(firstParty.map((plane) => Bun.hash(plane).toString())).size;
checks.push(`distinct slide pictures (first-party) = ${distinct}/${slides.length}`);
writeFileSync(join(out, "checks.txt"), `${JSON.stringify(result, null, 1)}\n${checks.join("\n")}\n`);
console.log(checks.join("\n"));
