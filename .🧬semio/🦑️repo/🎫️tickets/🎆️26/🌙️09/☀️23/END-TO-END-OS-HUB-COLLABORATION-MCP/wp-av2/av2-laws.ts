/** ⚖️ AV2: runs every TypeScript law of the video-render slice against a tree (the overlay by default) and the FFmpeg
 * differential adapter (subject vs oracle projections, compared exactly). Usage: bun av2-laws.ts [<root>] [--only a,b] */
import assert from "node:assert/strict";
import { mkdirSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2] : "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay";
const only = process.argv.includes("--only") ? new Set(process.argv[process.argv.indexOf("--only") + 1]!.split(",")) : null;
const want = (name: string): boolean => only === null || only.has(name);
const at = (rel: string): string => join(root, rel);
const failures: string[] = [];

async function law(name: string, run: () => unknown | Promise<unknown>): Promise<void> {
  if (!want(name)) return;
  const started = performance.now();
  try {
    await run();
    console.log(`[av2-laws] PASS ${name} ${Math.round(performance.now() - started)} ms`);
  } catch (error) {
    failures.push(name);
    console.log(`[av2-laws] FAIL ${name}: ${error instanceof Error ? (error.stack ?? error.message) : String(error)}`);
  }
}

await law("kernel-program", async () => (await import(at("🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎞️video-render-program/🟦️.ts"))).testVideoRenderProgramContract());
await law("kernel-job", async () => (await import(at("🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️video-render-job/🟦️.ts"))).testVideoRenderJobContract());
await law("raster-video", async () => (await import(at("🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🔬️unit/🟦️.ts"))).testRasterVideoAvcPcmContract());
await law("video-render-host", async () => (await import(at("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts"))).testVideoRenderHostContract());
await law("ffmpeg-decode", async () => {
  const adapter = (await import(at("🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🎞️ffmpeg-decode/🟦️.ts"))).default;
  const workDir = join(import.meta.dir, "generated", "ffmpeg-decode");
  mkdirSync(workDir, { recursive: true });
  const fixtures = at("🧰️framework/🔨️modules/🖌️raster/🎥️video/🧫️fixtures");
  const ctx = { workDir, artifactDir: workDir, fixture: (uri: string) => join(fixtures, uri.replace(/^shared:\/\//, "")), scenario: { id: "decodes-every-fixture-stream", steps: [] } };
  const handlers = adapter.scenarios["decodes-every-fixture-stream"];
  const subject = (await handlers.subject(ctx)).projection as { id: string; frames: number }[];
  const oracle = (await handlers.oracle(ctx)).projection as { id: string; frames: number }[];
  assert.deepEqual(oracle, subject, "FFmpeg's reading equals the encoder's claim");
  console.log(`[av2-laws] ffmpeg-decode cases=${subject.length} frames=${subject.reduce((sum, row) => sum + row.frames, 0)}`);
});
console.log(`[av2-laws] failures=${failures.length}${failures.length ? ` (${failures.join(", ")})` : ""}`);
process.exit(failures.length ? 1 : 0);
