import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { MEDIA_VIDEO_RENDER_CAPABILITY, VideoRenderJobLedger, type VideoRenderProgram } from "../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { AvcPcmEncoder, avcDecoderConfigurationRecord } from "../../../../../../../../../🔨️modules/🖌️raster/🎥️video/🟦️.ts";
import { cancelVideoRenderExportV1, drawVideoRenderSceneV1, runVideoRenderExportV1, subscribeVideoRenderExportJobsV1, type VideoRenderContext2dV1, type VideoRenderEnvironmentV1, videoRenderExportJobLogV1, videoRenderExportJobsSnapshotV1, videoRenderExportTaskIdV1, videoRenderPath2dV1, type WebCodecsVideoEncoderPortV1 } from "../../🟦️.ts";

/** 🎥️ Laws of the browser video host, run without a browser: a recording 2D context stands in for the canvas (it fills
 * its pixels with the last fill colour, and a drawn image with a colour derived from its crop, so the first-party tier
 * encodes real content), and a WebCodecs port backed by the first-party encoder stands in for the platform one. The
 * programs are the kernel fixture's (`🎠️kernel/🧫️fixtures/🎞️video-render-program/🔣️.json`). */

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
  const paint = (r: number, g: number, b: number): void => {
    for (let index = 0; index < pixels.length; index += 4) pixels.set([r, g, b, 255], index);
  };
  const paintCss = (css: unknown): void => {
    const match = /rgba\((\d+), (\d+), (\d+)/.exec(String(css));
    if (match) paint(Number(match[1]), Number(match[2]), Number(match[3]));
  };
  const context = {
    log,
    fillStyle: "" as unknown,
    strokeStyle: "" as unknown,
    lineWidth: 1,
    globalAlpha: 1,
    setTransform: (...values: number[]) => log.push(`T${values.join(",")}`),
    clearRect: () => log.push("clear"),
    fillRect: () => {
      log.push(`rect ${String(context.fillStyle)}`);
      paintCss(context.fillStyle);
    },
    fill: () => {
      log.push(`fill ${String(context.fillStyle)}`);
      paintCss(context.fillStyle);
    },
    stroke: () => log.push(`stroke ${String(context.strokeStyle)} ${context.lineWidth}`),
    drawImage: (_image: unknown, ...box: number[]) => {
      log.push(`image ${box.join(",")} alpha=${context.globalAlpha}`);
      paint(Math.round(box[0]!) % 256, Math.round(box[1]!) % 256, Math.round(context.globalAlpha * 255));
    },
    getImageData: () => ({ data: pixels }),
  };
  return { context: context as unknown as VideoRenderContext2dV1 & { readonly log: string[] }, source: pixels };
}

function environment(webCodecs: WebCodecsVideoEncoderPortV1 | null, loaded: string[] = []): VideoRenderEnvironmentV1 {
  let clock = 0;
  return {
    createCanvas: recordingCanvas,
    createPath: () => new RecordingPath() as unknown as Path2D,
    loadImage: async (url) => {
      loaded.push(url);
      if (url.includes("missing")) throw new Error(`image ${url} answered 404`);
      return { source: url as unknown as CanvasImageSource, width: 200, height: 100 };
    },
    webCodecs,
    yieldToHost: () => Promise.resolve(),
    now: () => (clock += 30),
  };
}

/** 🔌️ A WebCodecs port backed by the first-party encoder: key frames only where asked, `avcC` on the first chunk. */
function firstPartyWebCodecs(): WebCodecsVideoEncoderPortV1 & { readonly opened: string[]; readonly keys: boolean[] } {
  const opened: string[] = [];
  const keys: boolean[] = [];
  return {
    opened,
    keys,
    isConfigSupported: async () => true,
    open: (config, output) => {
      opened.push(config.codec);
      const encoder = new AvcPcmEncoder({ width: config.width, height: config.height, fps: config.framerate });
      let first = true;
      return {
        encode: (source, _timestamp, _duration, keyFrame) => {
          keys.push(keyFrame);
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

/** 🔢️ The `stss` entries of an MP4 this host wrote (`[]` when every sample is a sync sample and the box is omitted). */
function syncSamples(mp4: Uint8Array): number[] {
  const at = Buffer.from(mp4).indexOf("stss");
  if (at < 0) return [];
  const view = new DataView(mp4.buffer, mp4.byteOffset, mp4.byteLength);
  return Array.from({ length: view.getUint32(at + 8) }, (_, index) => view.getUint32(at + 12 + index * 4));
}

/** ⚖️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export async function testVideoRenderHostContract(): Promise<void> {
  const fixture = programs();
  const program = (id: string): VideoRenderProgram => fixture.valid.find((row) => row.id === id)!.program;
  const square = program("square-then-stroke");
  const slides = program("tile-slides");
  const still = program("still-deck");

  const path = videoRenderPath2dV1(square.paths[1]!, () => new RecordingPath() as unknown as Path2D) as unknown as RecordingPath;
  assert.deepEqual(path.calls, ["M0,0", "Q8,8,16,0", "C20,4,24,4,28,0"], "path verbs map onto Path2D calls in order");

  const canvas = recordingCanvas(slides.width, slides.height);
  const images = [{ source: "img" as unknown as CanvasImageSource, width: 200, height: 100 }];
  drawVideoRenderSceneV1(canvas.context, slides, 0, slides.paths.map((entry) => videoRenderPath2dV1(entry, () => new RecordingPath() as unknown as Path2D)), images);
  drawVideoRenderSceneV1(canvas.context, slides, 2, [], images);
  assert.deepEqual(
    canvas.context.log,
    ["T1,0,0,1,0,0", "clear", "rect rgba(26, 26, 26, 1)", "T48,0,0,32,0,0", "image 0,0,200,100,0,0,1,1 alpha=1", "T24,0,0,16,0,0", "stroke rgba(255, 255, 255, 1) 0.05", "T1,0,0,1,0,0", "clear", "rect rgba(26, 26, 26, 1)", "T48,0,0,32,0,0", "image 100,50,100,50,0,0,1,1 alpha=0.75"],
    "a scene paints background then ops in list order; an image op draws its crop onto the unit square at its opacity",
  );

  const delivered: { filename: string; bytes: Uint8Array }[] = [];
  const seen: number[] = [];
  const unsubscribe = subscribeVideoRenderExportJobsV1(() => seen.push(videoRenderExportJobsSnapshotV1()[0]?.completed ?? -1));
  const loaded: string[] = [];
  const host = (webCodecs: WebCodecsVideoEncoderPortV1 | null, locale = "de") => ({ environment: environment(webCodecs, loaded), deliver: (filename: string, bytes: Uint8Array) => delivered.push({ filename, bytes }), locale: () => locale });
  const granted = [MEDIA_VIDEO_RENDER_CAPABILITY];

  const firstParty = await runVideoRenderExportV1({ filename: "deck.mp4", owner: "animate", program: slides, capabilities: granted }, host(null));
  assert.deepEqual(firstParty.outcome, { status: "done", bytes: delivered[0]!.bytes.length, tier: "first-party" });
  assert.equal(firstParty.text, "Video deck.mp4 exportiert (14 Bilder).");
  assert.deepEqual(loaded, ["/🖼️assets/🖼️images/🏙️architecture/🏘️habitat-67.png"], "every program image is loaded once");
  assert.equal(String.fromCharCode(...delivered[0]!.bytes.subarray(4, 8)), "ftyp");
  assert.deepEqual(syncSamples(delivered[0]!.bytes), [1, 7, 11], "one IDR per timeline run, P_Skip repeats in between");
  assert.ok(seen.includes(14), "progress reached the last frame");
  assert.deepEqual(videoRenderExportJobsSnapshotV1(), [], "a finished export leaves the task list");

  const platform = firstPartyWebCodecs();
  const coded = await runVideoRenderExportV1({ filename: "square.mp4", owner: "animate", program: square, capabilities: granted }, host(platform));
  assert.deepEqual([coded.outcome.status, coded.outcome.status === "done" ? coded.outcome.tier : null], ["done", "platform"]);
  assert.deepEqual(platform.opened, ["avc1.42E00A"], "Constrained Baseline at the stream's own level");
  assert.deepEqual(platform.keys.flatMap((key, index) => (key ? [index] : [])), [0, 12, 24], "a key frame starts every run (and every two seconds)");
  assert.deepEqual(syncSamples(delivered[1]!.bytes), [1, 13, 25], "the platform's key frames are declared through stss");

  const withoutCapability = await runVideoRenderExportV1({ filename: "sneaky.mp4", owner: "draw", program: still, capabilities: ["artifacts.write"] }, host(null, "en"));
  assert.deepEqual([withoutCapability.outcome, withoutCapability.text], [{ status: "refused", code: "capability" }, "Video export sneaky.mp4 refused: draw did not request the media.video-render capability."]);
  const refused = await runVideoRenderExportV1({ filename: "bad.mp4", owner: "animate", program: { ...still, fps: 0 }, capabilities: granted }, host(null));
  assert.deepEqual([refused.outcome, refused.text], [{ status: "refused", code: "frameRate" }, "Videoexport bad.mp4 abgelehnt: das Programm ist ungültig (frameRate)."]);
  const broken = await runVideoRenderExportV1({ filename: "broken.mp4", owner: "animate", program: { ...slides, images: [{ url: "/missing.png" }] }, capabilities: granted }, host(null, "en"));
  assert.deepEqual([broken.outcome.status, broken.text], ["failed", "Video export broken.mp4 failed: image /missing.png answered 404"]);
  assert.equal(delivered.length, 2, "refused and failed exports deliver nothing");

  const cancelling = subscribeVideoRenderExportJobsV1(() => {
    const row = videoRenderExportJobsSnapshotV1()[0];
    if (row && !row.cancelling && row.completed >= 3) assert.equal(cancelVideoRenderExportV1(videoRenderExportTaskIdV1(row.job)), true);
  });
  const cancelled = await runVideoRenderExportV1({ filename: "long.mp4", owner: "animate", program: { ...still, timeline: [{ scene: 0, frames: 30 }] }, capabilities: granted }, host(null, "en"));
  cancelling();
  unsubscribe();
  assert.equal(cancelled.outcome.status, "cancelled");
  assert.ok(cancelled.outcome.status === "cancelled" && cancelled.outcome.completed >= 3 && cancelled.outcome.completed < 30, "cancellation stops before the next frame");
  assert.equal(cancelled.text, "Video export long.mp4 cancelled.");
  assert.equal(delivered.length, 2, "a cancelled export delivers nothing");
  assert.equal(cancelVideoRenderExportV1(videoRenderExportTaskIdV1(cancelled.job)), false, "a finished job cannot be cancelled");
  assert.equal(cancelVideoRenderExportV1("job:1"), false, "a foreign task id is not this host's");

  const events = videoRenderExportJobLogV1();
  assert.deepEqual(
    events.filter((event) => event.kind !== "progressed").map((event) => `${event.kind}#${event.job}${event.kind === "finished" ? `:${event.outcome.status}` : ""}`),
    ["started#1", "finished#1:done", "started#2", "finished#2:done", "started#3", "finished#3:refused", "started#4", "finished#4:refused", "started#5", "finished#5:failed", "started#6", "cancelRequested#6", "finished#6:cancelled"],
    "every export is one started…finished job in the log",
  );
  const refolded = VideoRenderJobLedger.fold(events);
  assert.ok(refolded instanceof VideoRenderJobLedger && refolded.running().length === 0 && refolded.lastJob() === 6, "the task list IS the fold of the log");
  console.log(`video-render-host tiers=first-party,platform outcomes=done,refused(capability,program),failed,cancelled events=${events.length}`);
}
