/** 🧿️ [DEBUG] Temporary visual check of the 3D stage: renders five catalog assets at every authored scale in body, thermal and structure through the real stage and the software rasteriser into PNGs under VERIFY_OUT. Deleted after the ticket. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { deflateSync } from "node:zlib";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { authoredScales } from "@teaching/staffage";
import type { View } from "@teaching/staffage";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { StaffageClock } from "../../🔨️modules/⏱️clock/🟦️.ts";
import { ModelStage, type StageOptions } from "../../🔨️modules/🧊️spatial/🖥️stage/🟦️.ts";

const OUT = process.env.VERIFY_OUT!;
const ASSETS = (process.env.VERIFY_ASSETS ?? "jogger,city-car,patio-heater,stair-flight,cooking-pot").split(",");
const W = Number(process.env.VERIFY_W ?? 480);
const H = Number(process.env.VERIFY_H ?? 360);
const MODES: readonly [string, View][] = [["body", "light-solid"], ["thermal", "thermal"], ["structure", "forces"]];
const CAMERA = (process.env.VERIFY_CAMERA ?? "axonometric") as StageOptions["preset"];

const crc = (() => {
  const table = new Uint32Array(256).map((_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
  return (buffer: Uint8Array) => { let c = 0xffffffff; for (const byte of buffer) c = table[(c ^ byte) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
})();
const chunk = (type: string, data: Uint8Array) => {
  const out = Buffer.alloc(12 + data.length);
  out.writeUInt32BE(data.length, 0);
  out.write(type, 4, "ascii");
  Buffer.from(data).copy(out, 8);
  out.writeUInt32BE(crc(out.subarray(4, 8 + data.length)), 8 + data.length);
  return out;
};
const png = (pixels: Uint8ClampedArray, width: number, height: number) => {
  const raw = Buffer.alloc((width * 4 + 1) * height);
  for (let y = 0; y < height; y++) { raw[y * (width * 4 + 1)] = 0; Buffer.from(pixels.buffer, pixels.byteOffset + y * width * 4, width * 4).copy(raw, y * (width * 4 + 1) + 1); }
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0); header.writeUInt32BE(height, 4); header[8] = 8; header[9] = 6;
  return Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", header), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array())]);
};
const stats = (pixels: Uint8ClampedArray, width: number, height: number) => {
  const bg = [pixels[0]!, pixels[1]!, pixels[2]!];
  let minX = width, minY = height, maxX = -1, maxY = -1, covered = 0;
  const colours = new Set<number>();
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    const o = (y * width + x) * 4;
    if (Math.abs(pixels[o]! - bg[0]!) + Math.abs(pixels[o + 1]! - bg[1]!) + Math.abs(pixels[o + 2]! - bg[2]!) > 12) {
      covered++; minX = Math.min(minX, x); maxX = Math.max(maxX, x); minY = Math.min(minY, y); maxY = Math.max(maxY, y);
      colours.add(((pixels[o]! >> 4) << 8) | ((pixels[o + 1]! >> 4) << 4) | (pixels[o + 2]! >> 4));
    }
  }
  return { coverage: Number((covered / (width * height)).toFixed(3)), box: [minX, minY, maxX, maxY], margins: covered === 0 ? null : { left: minX, right: width - 1 - maxX, top: minY, bottom: height - 1 - maxY }, colours: colours.size };
};

beforeEach(() => {
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ x: 0, y: 0, left: 0, top: 0, right: W, bottom: H, width: W, height: H, toJSON: () => ({}) });
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
});
afterEach(() => vi.restoreAllMocks());

const entries = await assetPathsFromFiles();
const report: Record<string, unknown> = {};

describe("[DEBUG] visual check", () => {
  for (const id of ASSETS) {
    it(`${id}: every authored scale in body, thermal and structure, and the default clip`, async () => {
      const entry = entries.find((candidate) => candidate.folder === id)!;
      const { asset } = await readStreamedAsset(entry);
      mkdirSync(join(OUT, id), { recursive: true });
      const host = document.createElement("div");
      document.body.append(host);
      const clip = asset.clips.find((candidate) => candidate.id === asset.presentation.defaultClip) ?? asset.clips[0]!;
      const clock = new StaffageClock({ fps: Math.min(clip.fps, 30), durationS: clip.durationS, loop: true, rate: 1, playing: false, reducedMotion: true });
      const base = { asset, query: { kind: "pose", pose: asset.presentation.defaultPose } as const, view: "light-solid", scale: 100, language: "en", preset: CAMERA, settleMs: 0 } as StageOptions;
      const stage = new ModelStage(host, base);
      await vi.waitFor(() => expect(host.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 600000, interval: 500 });
      const rows: unknown[] = [];
      for (const scale of authoredScales(asset.volumes)) {
        for (const [mode, view] of MODES) {
          stage.update({ scale, view });
          const scene = stage.viewer!.activeScene;
          stage.viewer!.requestRender();
          await new Promise((resolve) => setTimeout(resolve, 150));
          const pixels = stage.viewer!.readPixels();
          writeFileSync(join(OUT, id, `${id}.s${scale}.${mode}.png`), png(pixels, W, H));
          rows.push({ scale, mode, view, scene, ...stats(pixels, W, H) });
        }
      }
      stage.update({ query: { kind: "clip", clip: clip.id, timeS: 0 }, clock, view: "light-solid" });
      await vi.waitFor(() => expect(host.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 600000, interval: 500 });
      for (const scale of authoredScales(asset.volumes)) {
        for (const fraction of [0, 0.5]) {
          stage.update({ scale });
          clock.seek(clip.durationS * fraction);
          stage.viewer!.requestRender();
          await new Promise((resolve) => setTimeout(resolve, 150));
          const pixels = stage.viewer!.readPixels();
          writeFileSync(join(OUT, id, `${id}.clip-${clip.id}.s${scale}.t${fraction * 100}.png`), png(pixels, W, H));
          rows.push({ scale, clip: clip.id, t: fraction, scene: stage.viewer!.activeScene, ...stats(pixels, W, H) });
        }
      }
      report[id] = rows;
      writeFileSync(join(OUT, id, `${id}.report.json`), JSON.stringify(rows, null, 1));
      stage.dispose();
      host.remove();
    }, 3_600_000);
  }
});
