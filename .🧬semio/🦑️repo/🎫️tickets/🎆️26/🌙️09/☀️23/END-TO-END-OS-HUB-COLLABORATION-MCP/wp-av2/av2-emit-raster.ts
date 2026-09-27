/** 🎥️ AV2: encodes every case of the raster video tier fixture with the TypeScript twin, writes `generated/raster/<id>.mp4`
 * for the FFmpeg oracle, and with `--write-fixture` rewrites each case's `expected` block (sizes, container length, sha256,
 * hex when small) from what the twin produced. Run the oracle BEFORE trusting a rewritten fixture.
 * Usage: bun av2-emit-raster.ts [<overlay root>] [--write-fixture] */
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2] : "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay";
const video = await import(join(root, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts"));
const fixturePath = join(root, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🧫️fixtures/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const out = join(import.meta.dir, "generated", "raster");
mkdirSync(out, { recursive: true });
const hex = (bytes: Uint8Array): string => Buffer.from(bytes).toString("hex");

type Rgb = [number, number, number];
function picture(width: number, height: number, run: { rgb?: Rgb; quadrants?: Rgb[] }): Uint8Array {
  const rgba = new Uint8Array(width * height * 4);
  for (let y = 0; y < height; y += 1) {
    for (let x = 0; x < width; x += 1) {
      const rgb = run.quadrants ? run.quadrants[(y < height / 2 ? 0 : 2) + (x < width / 2 ? 0 : 1)]! : run.rgb!;
      rgba.set([rgb[0], rgb[1], rgb[2], 255], (y * width + x) * 4);
    }
  }
  return rgba;
}

for (const testCase of fixture.cases) {
  const parameters = { width: testCase.width, height: testCase.height, fps: testCase.fps };
  const encoder = new video.AvcPcmEncoder(parameters);
  const sizes: number[] = [];
  const sync: boolean[] = [];
  const port = {
    parameters,
    configuration: () => encoder.configuration(),
    encode: (rgba: Uint8Array) => { const sample = encoder.encode(rgba); sizes.push(sample.data.length); sync.push(sample.sync); return sample; },
    repeat: () => { const sample = encoder.repeat(); sizes.push(sample.data.length); sync.push(sample.sync); return sample; },
  };
  const runs = testCase.runs.map((run: { frames: number }) => ({ rgba: picture(parameters.width, parameters.height, run), frames: run.frames }));
  const mp4 = video.encodeVideoRuns(port, runs, () => undefined);
  writeFileSync(join(out, `${testCase.id}.mp4`), mp4);
  const config = encoder.configuration();
  const colours = (run: { rgb?: Rgb; quadrants?: Rgb[] }): Rgb[] => run.quadrants ?? [run.rgb!];
  testCase.expected = {
    frameCount: sizes.length,
    syncSamples: sync.flatMap((value, index) => (value ? [index + 1] : [])),
    durationMs: video.videoDurationMilliseconds(sizes.length, parameters.fps),
    macroblocks: video.videoMacroblocks(parameters),
    levelIdc: video.avcLevelIdc(parameters),
    codec: video.avcCodecString(config),
    spsHex: hex(config.sps),
    ppsHex: hex(config.pps),
    sampleSizes: sizes,
    mp4Bytes: mp4.length,
    mp4Sha256: createHash("sha256").update(mp4).digest("hex"),
    yuv: testCase.runs.map((run: { rgb?: Rgb; quadrants?: Rgb[] }) => colours(run).map(([r, g, b]) => [video.bt601Luma(r, g, b), video.bt601Cb(r, g, b), video.bt601Cr(r, g, b)])),
    ...(mp4.length <= 4096 ? { mp4Hex: hex(mp4) } : {}),
  };
  console.log(`[av2-emit-raster] ${testCase.id} frames=${sizes.length} bytes=${mp4.length} sync=${testCase.expected.syncSamples.join(",")}`);
}
if (process.argv.includes("--write-fixture")) {
  const lines = ["{", ` "schema": ${JSON.stringify(fixture.schema)},`, ' "cases": ['];
  fixture.cases.forEach((testCase: unknown, index: number) => lines.push(`  ${JSON.stringify(testCase)}${index + 1 < fixture.cases.length ? "," : ""}`));
  lines.push(" ],", ' "refusals": [');
  fixture.refusals.forEach((refusal: unknown, index: number) => lines.push(`  ${JSON.stringify(refusal)}${index + 1 < fixture.refusals.length ? "," : ""}`));
  lines.push(" ],", ` "oracle": ${JSON.stringify(fixture.oracle)}`, "}");
  writeFileSync(fixturePath, `${lines.join("\n")}\n`);
  console.log(`[av2-emit-raster] fixture rewritten: ${fixturePath}`);
}
