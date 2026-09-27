/** 🎥️ AV1: encodes every `🧫️fixtures/🔣️.json` case of the raster video tier with the TypeScript twin, writes the MP4s under
 * `generated/ts/<id>.mp4` for the ffprobe oracle, and prints the observed expectations (`--print`). */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const overlay = process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2] : "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay";
const video = await import(join(overlay, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts"));
const fixture = JSON.parse(readFileSync(join(overlay, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🧫️fixtures/🔣️.json"), "utf8"));
const out = join(import.meta.dir, "generated", "ts");
mkdirSync(out, { recursive: true });
const hex = (bytes: Uint8Array): string => [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");
const observed: Record<string, unknown> = {};
for (const testCase of fixture.cases) {
  const parameters = { width: testCase.width, height: testCase.height, fps: testCase.fps };
  const encoder = new video.AvcIntraPcmEncoder(parameters);
  const runs = testCase.runs.map((run: { rgb: number[]; frames: number }) => {
    const rgba = new Uint8Array(parameters.width * parameters.height * 4);
    for (let index = 0; index < rgba.length; index += 4) rgba.set([run.rgb[0], run.rgb[1], run.rgb[2], 255], index);
    return { rgba, frames: run.frames };
  });
  const sizes: number[] = [];
  const mp4 = video.encodeVideoRuns({ parameters, configuration: () => encoder.configuration(), encode: (rgba: Uint8Array) => { const sample = encoder.encode(rgba); sizes.push(sample.data.length); return sample; } }, runs, () => undefined);
  writeFileSync(join(out, `${testCase.id}.mp4`), mp4);
  const config = encoder.configuration();
  observed[testCase.id] = {
    frameCount: sizes.length,
    durationMs: video.videoDurationMilliseconds(sizes.length, parameters.fps),
    macroblocks: video.videoMacroblocks(parameters),
    levelIdc: video.avcLevelIdc(parameters),
    codec: video.avcCodecString(config),
    spsHex: hex(config.sps),
    ppsHex: hex(config.pps),
    sampleSizes: sizes,
    mp4Bytes: mp4.length,
    yuv: testCase.runs.map((run: { rgb: number[] }) => [video.bt601Luma(run.rgb[0], run.rgb[1], run.rgb[2]), video.bt601Cb(run.rgb[0], run.rgb[1], run.rgb[2]), video.bt601Cr(run.rgb[0], run.rgb[1], run.rgb[2])]),
    ...(mp4.length <= 4096 ? { mp4Hex: hex(mp4) } : {}),
  };
}
if (process.argv.includes("--print")) console.log(JSON.stringify(observed, null, 1));
console.log(`[av1-emit-ts] cases=${fixture.cases.length} out=${out}`);
