import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { AvcIntraPcmEncoder, avcCodecString, avcConfigurationFromRecord, avcDecoderConfigurationRecord, avcLevelIdc, bt601Cb, bt601Cr, bt601Luma, encodeVideoRuns, VideoEncodeError, videoDurationMilliseconds, videoMacroblocks, writeAvcMp4 } from "../../🟦️.ts";

/** 🎥️ TypeScript twin of `🧪️tests/🔬️unit/🦀️.rs`, driven from the SAME fixture (`🧫️fixtures/🔣️.json`): both twins of
 * the raster video tier write byte-identical H.264 + MP4 for every case, and the pinned bytes are the ones the
 * third-party oracle (ffprobe + ffmpeg, `fixture.oracle`) accepted with the declared size, frame count, duration,
 * level and decoded colours. */

interface VideoCase {
  readonly id: string;
  readonly width: number;
  readonly height: number;
  readonly fps: number;
  readonly runs: readonly { readonly rgb: readonly [number, number, number]; readonly frames: number }[];
  readonly expected: {
    readonly frameCount: number;
    readonly durationMs: number;
    readonly macroblocks: { readonly width: number; readonly height: number };
    readonly levelIdc: number;
    readonly codec: string;
    readonly spsHex: string;
    readonly ppsHex: string;
    readonly sampleSizes: readonly number[];
    readonly mp4Bytes: number;
    readonly yuv: readonly (readonly [number, number, number])[];
    readonly mp4Hex?: string;
  };
}

interface VideoFixture {
  readonly cases: readonly VideoCase[];
  readonly refusals: readonly { readonly id: string; readonly width: number; readonly height: number; readonly fps: number; readonly error: string }[];
}

function loadFixture(): VideoFixture {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🔣️.json"), "utf8")) as VideoFixture;
}

const hex = (bytes: Uint8Array): string => [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");

/** ⚖️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export function testRasterVideoAvcIntraPcmContract(): void {
  const fixture = loadFixture();
  for (const testCase of fixture.cases) {
    const parameters = { width: testCase.width, height: testCase.height, fps: testCase.fps };
    const encoder = new AvcIntraPcmEncoder(parameters);
    const sizes: number[] = [];
    const recording = { parameters, configuration: () => encoder.configuration(), encode: (rgba: Uint8Array | Uint8ClampedArray) => { const sample = encoder.encode(rgba); sizes.push(sample.data.length); return sample; } };
    const runs = testCase.runs.map((run) => {
      const rgba = new Uint8Array(parameters.width * parameters.height * 4);
      for (let index = 0; index < rgba.length; index += 4) rgba.set([run.rgb[0], run.rgb[1], run.rgb[2], 255], index);
      return { rgba, frames: run.frames };
    });
    const reported: [number, number][] = [];
    const mp4 = encodeVideoRuns(recording, runs, (done, total) => reported.push([done, total]));
    const { expected } = testCase;
    assert.equal(sizes.length, expected.frameCount, `${testCase.id}: frame count`);
    assert.deepEqual(reported.at(-1), [expected.frameCount, expected.frameCount], `${testCase.id}: progress ends at total`);
    assert.equal(videoDurationMilliseconds(sizes.length, parameters.fps), expected.durationMs, `${testCase.id}: duration`);
    assert.deepEqual(videoMacroblocks(parameters), expected.macroblocks, `${testCase.id}: macroblocks`);
    assert.equal(avcLevelIdc(parameters), expected.levelIdc, `${testCase.id}: level`);
    assert.equal(avcCodecString(encoder.configuration()), expected.codec, `${testCase.id}: codec string`);
    assert.equal(hex(encoder.configuration().sps), expected.spsHex, `${testCase.id}: SPS bytes`);
    assert.equal(hex(encoder.configuration().pps), expected.ppsHex, `${testCase.id}: PPS bytes`);
    assert.deepEqual(sizes, expected.sampleSizes, `${testCase.id}: sample sizes`);
    assert.equal(mp4.length, expected.mp4Bytes, `${testCase.id}: container size`);
    if (expected.mp4Hex !== undefined) assert.equal(hex(mp4), expected.mp4Hex, `${testCase.id}: container bytes`);
    testCase.runs.forEach((run, index) => assert.deepEqual([bt601Luma(...run.rgb), bt601Cb(...run.rgb), bt601Cr(...run.rgb)], expected.yuv[index], `${testCase.id}: BT.601 samples`));
    const record = avcDecoderConfigurationRecord(encoder.configuration());
    const parsed = avcConfigurationFromRecord(record);
    assert.deepEqual([hex(parsed.sps), hex(parsed.pps)], [expected.spsHex, expected.ppsHex], `${testCase.id}: avcC round trip`);
    assert.equal(avcDecoderConfigurationRecord(parsed), record, `${testCase.id}: a platform record is written verbatim`);
  }
  for (const refusal of fixture.refusals) {
    assert.throws(() => new AvcIntraPcmEncoder(refusal), (error: unknown) => error instanceof VideoEncodeError && error.code === refusal.error, refusal.id);
  }
  const parameters = { width: 32, height: 16, fps: 30 };
  const encoder = new AvcIntraPcmEncoder(parameters);
  const rgba = new Uint8Array(32 * 16 * 4);
  const controller = new AbortController();
  let done = 0;
  assert.throws(
    () =>
      encodeVideoRuns(encoder, [{ rgba, frames: 3 }], (frames) => {
        done = frames;
        controller.abort();
      }, controller.signal),
    (error: unknown) => error instanceof VideoEncodeError && error.code === "cancelled",
  );
  assert.equal(done, 1, "cancellation stops before the next frame");
  assert.throws(() => encoder.encode(rgba.subarray(4)), (error: unknown) => error instanceof VideoEncodeError && error.code === "frameBytes");
  assert.throws(() => writeAvcMp4(parameters, encoder.configuration(), []), (error: unknown) => error instanceof VideoEncodeError && error.code === "empty");
  console.log(`raster-video cases=${fixture.cases.length} refusals=${fixture.refusals.length} oracle=ffprobe+ffmpeg(fixture-pinned)`);
}
