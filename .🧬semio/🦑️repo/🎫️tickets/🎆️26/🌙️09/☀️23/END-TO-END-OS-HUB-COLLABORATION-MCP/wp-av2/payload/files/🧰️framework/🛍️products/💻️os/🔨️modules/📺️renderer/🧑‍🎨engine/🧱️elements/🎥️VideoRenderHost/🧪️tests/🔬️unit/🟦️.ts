import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { type VideoRenderProgram } from "../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { AvcIntraPcmEncoder, avcDecoderConfigurationRecord } from "../../../../../../../../../🔨️modules/🖌️raster/🎥️video/🟦️.ts";
import { cancelVideoRenderExportV1, drawVideoRenderSceneV1, runVideoRenderExportV1, subscribeVideoRenderExportTasksV1, type VideoRenderContext2dV1, type VideoRenderEnvironmentV1, videoRenderExportTasksSnapshotV1, videoRenderPath2dV1, type WebCodecsVideoEncoderPortV1 } from "../../🟦️.ts";

/** 🎥️ Laws of the browser video host, run without a browser: a recording 2D context stands in for the canvas (it fills
 * its pixels with the last fill colour so the intra-PCM tier encodes real content), and a WebCodecs port backed by the
 * first-party encoder stands in for the platform one (it marks every non-key frame non-sync, exactly the shape a platform
 * encoder answers). The programs are the kernel fixture's (`🎠️kernel/🧫️fixtures/🎞️video-render-program/🔣️.json`). */

interface ProgramFixture {
  readonly valid: readonly { readonly id: string; readonly program: VideoRenderProgram; readonly frameCount: number }[];
}

function programs(): ProgramFixture {
  const here = dirname(fileURLToPath(import.meta.url));
  return JSON.parse(readFileSync(join(here, "../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🎞️video-render-program/🔣️.json"), "utf8")) as ProgramFixture;
}

class RecordingPath {
  readonly calls: string[] = [];
  moveTo(x: number, y: number): void {
    this.calls.push(`M${x},${y}`);
  }
  lineTo(x: number, y: number): void {
    this.calls.push(`L${x},${y}`);
  }
  quadraticCurveTo(...values: number[]): void {
    this.calls.push(`Q${values.join(",")}`);
  }
  bezierCurveTo(...values: number[]): void {
    this.calls.push(`C${values.join(",")}`);
  }
  closePath(): void {
    this.calls.push("Z");
  }
}

function recordingCanvas(width: number, height: number): { readonly context: VideoRenderContext2dV1 & { readonly log: string[] }; readonly source: unknown } {
  const pixels = new Uint8ClampedArray(width * height * 4);
  const log: string[] = [];
  const paint = (css: unknown): void => {
    const match = /rgba\((\d+), (\d+), (\d+)/.exec(String(css));
    if (!match) return;
    for (let index = 0; index < pixels.length; index += 4) pixels.set([Number(match[1]), Number(match[2]), Number(match[3]), 255], index);
  };
  const context = {
    log,
    fillStyle: "" as unknown,
    strokeStyle: "" as unknown,
    lineWidth: 1,
    setTransform: (...values: number[]) => log.push(`T${values.join(",")}`),
    clearRect: () => log.push("clear"),
    fillRect: () => {
      log.push(`rect ${String(context.fillStyle)}`);
      paint(context.fillStyle);
    },
    fill: () => {
      log.push(`fill ${String(context.fillStyle)}`);
      paint(context.fillStyle);
    },
    stroke: () => log.push(`stroke ${String(context.strokeStyle)} ${context.lineWidth}`),
    getImageData: () => ({ data: pixels }),
  };
  return { context: context as unknown as VideoRenderContext2dV1 & { readonly log: string[] }, source: pixels };
}

function environment(webCodecs: WebCodecsVideoEncoderPortV1 | null): VideoRenderEnvironmentV1 {
  return { createCanvas: recordingCanvas, createPath: () => new RecordingPath() as unknown as Path2D, webCodecs, yieldToHost: () => Promise.resolve() };
}

/** 🔌️ A WebCodecs port backed by the first-party encoder: key frames only where asked, `avcC` on the first chunk. */
function firstPartyWebCodecs(): WebCodecsVideoEncoderPortV1 & { readonly opened: string[] } {
  const opened: string[] = [];
  return {
    opened,
    isConfigSupported: async () => true,
    open: (config, output) => {
      opened.push(config.codec);
      const encoder = new AvcIntraPcmEncoder({ width: config.width, height: config.height, fps: config.framerate });
      let first = true;
      return {
        encode: (source, _timestamp, _duration, keyFrame) => {
          const sample = encoder.encode(source as Uint8ClampedArray);
          output({ data: sample.data, sync: keyFrame }, first ? avcDecoderConfigurationRecord(encoder.configuration()) : null);
          first = false;
        },
        queueSize: 0,
        flush: async () => undefined,
        close: () => undefined,
      };
    },
  };
}

/** ⚖️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export async function testVideoRenderHostContract(): Promise<void> {
  const fixture = programs();
  const square = fixture.valid.find((row) => row.id === "square-then-stroke")!.program;
  const still = fixture.valid.find((row) => row.id === "still-deck")!.program;

  const path = videoRenderPath2dV1(square.paths[1]!, () => new RecordingPath() as unknown as Path2D) as unknown as RecordingPath;
  assert.deepEqual(path.calls, ["M0,0", "Q8,8,16,0", "C20,4,24,4,28,0"], "path verbs map onto Path2D calls in order");

  const canvas = recordingCanvas(square.width, square.height);
  drawVideoRenderSceneV1(canvas.context, square, 1, square.paths.map((entry) => videoRenderPath2dV1(entry, () => new RecordingPath() as unknown as Path2D)));
  assert.deepEqual(canvas.context.log, ["T1,0,0,1,0,0", "clear", "rect rgba(0, 0, 0, 1)", "T1,0,0,1,16,0", "fill rgba(255, 0, 0, 1)", "T1,0,0,1,0,2", "stroke rgba(0, 255, 0, 0.5) 2"], "a scene paints background then ops in list order under their own transforms");

  const delivered: { filename: string; bytes: Uint8Array }[] = [];
  const seen: number[] = [];
  const unsubscribe = subscribeVideoRenderExportTasksV1(() => seen.push(videoRenderExportTasksSnapshotV1()[0]?.completed ?? -1));
  const host = (webCodecs: WebCodecsVideoEncoderPortV1 | null) => ({ environment: environment(webCodecs), deliver: (filename: string, bytes: Uint8Array) => delivered.push({ filename, bytes }), locale: () => "de" });

  const pcm = await runVideoRenderExportV1({ filename: "deck.mp4", owner: "semio:animate", program: still }, host(null));
  assert.equal(pcm.status, "done");
  assert.equal(pcm.tier, "intra-pcm");
  assert.equal(pcm.text, "Video deck.mp4 exportiert (7 Bilder).");
  assert.equal(delivered.length, 1);
  assert.equal(delivered[0]!.bytes.length, pcm.bytes);
  assert.equal(String.fromCharCode(...delivered[0]!.bytes.subarray(4, 8)), "ftyp");
  assert.ok(seen.includes(7), "progress reached the last frame");
  assert.deepEqual(videoRenderExportTasksSnapshotV1(), [], "a finished export leaves the task list");

  const platform = firstPartyWebCodecs();
  const coded = await runVideoRenderExportV1({ filename: "square.mp4", owner: "semio:animate", program: square }, host(platform));
  assert.equal(coded.status, "done");
  assert.equal(coded.tier, "webcodecs");
  assert.deepEqual(platform.opened, ["avc1.42E00A"], "Constrained Baseline at the stream's own level");
  const bytes = delivered[1]!.bytes;
  const text = String.fromCharCode(...bytes);
  assert.ok(text.includes("stss"), "non-key platform frames are declared through stss");

  const refused = await runVideoRenderExportV1({ filename: "bad.mp4", owner: "semio:animate", program: { ...still, fps: 0 } }, host(null));
  assert.deepEqual([refused.status, refused.text], ["refused", "Videoexport bad.mp4 abgelehnt: das Programm ist ungültig (frameRate)."]);

  const cancelling = subscribeVideoRenderExportTasksV1(() => {
    const task = videoRenderExportTasksSnapshotV1()[0];
    if (task && !task.cancelling) cancelVideoRenderExportV1(task.id);
  });
  const cancelled = await runVideoRenderExportV1({ filename: "long.mp4", owner: "semio:animate", program: { ...still, timeline: [{ scene: 0, frames: 30 }] } }, { ...host(null), locale: () => "en" });
  cancelling();
  unsubscribe();
  assert.deepEqual([cancelled.status, cancelled.text], ["cancelled", "Video export long.mp4 cancelled."]);
  assert.equal(delivered.length, 2, "a cancelled export delivers nothing");
  assert.deepEqual(videoRenderExportTasksSnapshotV1(), [], "a cancelled export leaves the task list");
  assert.equal(cancelVideoRenderExportV1("export:video#unknown"), false);
  console.log(`video-render-host tiers=intra-pcm,webcodecs outcomes=done,refused,cancelled bytes=${pcm.bytes}+${coded.bytes}`);
}
