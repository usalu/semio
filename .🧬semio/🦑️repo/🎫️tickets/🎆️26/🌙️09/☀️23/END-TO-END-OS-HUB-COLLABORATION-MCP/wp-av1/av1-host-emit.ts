/** 🎥️ AV1: runs the browser video host (`🎥️VideoRenderHost`) over the kernel fixture programs with the law's injected
 * environment (intra-PCM tier and a WebCodecs-shaped tier with non-key frames), writes `generated/host/<id>-<tier>.mp4` and
 * prints `id width height fps frames` rows for `av1-oracle.py --rows`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const overlay = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay";
const host = await import(join(overlay, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🟦️.ts"));
const video = await import(join(overlay, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts"));
const fixture = JSON.parse(readFileSync(join(overlay, "🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🎞️video-render-program/🔣️.json"), "utf8"));
const out = join(import.meta.dir, "generated", "host");
mkdirSync(out, { recursive: true });
const canvas = (width: number, height: number) => {
  const pixels = new Uint8ClampedArray(width * height * 4);
  const context: Record<string, unknown> = { fillStyle: "", strokeStyle: "", lineWidth: 1, setTransform() {}, clearRect() {}, stroke() {}, getImageData: () => ({ data: pixels }) };
  const paint = () => {
    const match = /rgba\((\d+), (\d+), (\d+)/.exec(String(context.fillStyle));
    if (match) for (let index = 0; index < pixels.length; index += 4) pixels.set([Number(match[1]), Number(match[2]), Number(match[3]), 255], index);
  };
  context.fillRect = paint;
  context.fill = paint;
  return { context, source: pixels };
};
const path = () => ({ moveTo() {}, lineTo() {}, quadraticCurveTo() {}, bezierCurveTo() {}, closePath() {} });
const webCodecs = {
  isConfigSupported: async () => true,
  open: (config: { width: number; height: number; framerate: number }, output: (sample: unknown, record: Uint8Array | null) => void) => {
    const encoder = new video.AvcIntraPcmEncoder({ width: config.width, height: config.height, fps: config.framerate });
    let first = true;
    return {
      encode: (source: Uint8ClampedArray, _t: number, _d: number, keyFrame: boolean) => {
        const sample = encoder.encode(source);
        output({ data: sample.data, sync: keyFrame }, first ? video.avcDecoderConfigurationRecord(encoder.configuration()) : null);
        first = false;
      },
      queueSize: 0,
      flush: async () => undefined,
      close: () => undefined,
    };
  },
};
const rows: string[] = [];
for (const row of fixture.valid) {
  for (const tier of ["intra-pcm", "webcodecs"]) {
    const outcome = await host.runVideoRenderExportV1(
      { filename: `${row.id}-${tier}.mp4`, owner: "av1", program: row.program },
      { environment: { createCanvas: canvas, createPath: path, webCodecs: tier === "webcodecs" ? webCodecs : null, yieldToHost: () => Promise.resolve() }, deliver: (filename: string, bytes: Uint8Array) => writeFileSync(join(out, filename), bytes), locale: () => "en" },
    );
    if (outcome.status !== "done" || outcome.tier !== tier) throw new Error(`${row.id} ${tier}: ${outcome.status} ${outcome.text}`);
    rows.push(`${row.id}-${tier} ${row.program.width} ${row.program.height} ${row.program.fps} ${row.frameCount}`);
  }
}
writeFileSync(join(out, "rows.txt"), rows.join("\n") + "\n");
console.log(`[av1-host-emit] files=${rows.length} out=${out}`);
