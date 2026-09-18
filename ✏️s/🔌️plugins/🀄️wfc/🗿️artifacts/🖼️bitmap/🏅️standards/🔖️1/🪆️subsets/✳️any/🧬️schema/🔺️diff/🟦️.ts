/** 🔺️ Bitmap diff — a sparse, lane-addressed delta over BitmapSnapshot, plus the TS half of its
 * APPLY. Apply is re-implemented here rather than declared, because it is the one step where two
 * languages can genuinely disagree: lane order, the pad rule of a resize, and the index a pin is
 * restored at are all decisions, and the cross-language oracle is what pins them. */

import { decodeBase64, encodeBase64, pinKey, type BitmapColor, type BitmapOutputSpec, type BitmapOverlappingModel, type BitmapPinnedPixel, type BitmapSnapshot } from "../📸️snapshot/🟦️.ts";

export interface BitmapPixelRegion {
  x: number;
  y: number;
  width: number;
  height: number;
  pixels: string;
}

export interface BitmapDiff {
  schema?: string | null;
  seed?: number | null;
  inputWidth?: number | null;
  inputHeight?: number | null;
  inputPixels?: string | null;
  inputRegions: BitmapPixelRegion[];
  palette?: BitmapColor[] | null;
  output?: BitmapOutputSpec | null;
  model?: BitmapOverlappingModel | null;
  pinnedRemoved: string[];
  pinnedUpserted: [number, BitmapPinnedPixel][];
}

/** 📐️ Keeps the overlapping top-left region and pads the new margin with palette index `0` — the
 * exact rule the Rust `apply` performs when a resize lane is present. */
export function resizedBuffer(buffer: Uint8Array, fromWidth: number, fromHeight: number, toWidth: number, toHeight: number): Uint8Array {
  const out = new Uint8Array(toWidth * toHeight);
  for (let y = 0; y < Math.min(fromHeight, toHeight); y += 1) {
    for (let x = 0; x < Math.min(fromWidth, toWidth); x += 1) out[y * toWidth + x] = buffer[y * fromWidth + x]!;
  }
  return out;
}

/** 🩹 Carries `before` to `after`. Lane order is load-bearing: a resize relays the buffer out, a
 * whole-buffer replacement supersedes it, and only then do the sparse region writes land. */
export function applyBitmapDiff(base: BitmapSnapshot, diff: BitmapDiff): BitmapSnapshot {
  const next: BitmapSnapshot = JSON.parse(JSON.stringify(base)) as BitmapSnapshot;
  if (diff.schema != null) next.schema = diff.schema;
  if (diff.seed != null) next.seed = diff.seed;

  let buffer = decodeBase64(next.input.pixels);
  if (buffer.length !== next.input.width * next.input.height) throw new Error("the base input pixel buffer is not width * height bytes");
  if (diff.inputWidth != null || diff.inputHeight != null) {
    const width = diff.inputWidth ?? next.input.width;
    const height = diff.inputHeight ?? next.input.height;
    if (width <= 0 || height <= 0) throw new Error("an input bitmap may not have a zero edge");
    buffer = resizedBuffer(buffer, next.input.width, next.input.height, width, height);
    next.input.width = width;
    next.input.height = height;
  }
  if (diff.inputPixels != null) {
    const replacement = decodeBase64(diff.inputPixels);
    if (replacement.length !== next.input.width * next.input.height) throw new Error("the replacement buffer does not match the input size");
    buffer = replacement;
  }
  if (diff.palette != null) {
    if (diff.palette.length === 0) throw new Error("a palette may not be empty");
    next.input.palette = diff.palette.map((color) => ({ ...color }));
  }
  for (const region of diff.inputRegions ?? []) {
    const pixels = decodeBase64(region.pixels);
    if (region.width <= 0 || region.height <= 0 || pixels.length !== region.width * region.height) throw new Error("a region payload does not match its own extent");
    if (region.x + region.width > next.input.width || region.y + region.height > next.input.height) throw new Error("a region write falls outside the input bitmap");
    for (let row = 0; row < region.height; row += 1) {
      const start = (region.y + row) * next.input.width + region.x;
      buffer.set(pixels.subarray(row * region.width, (row + 1) * region.width), start);
    }
  }
  next.input.pixels = encodeBase64(buffer);

  if (diff.output != null) next.output = { ...diff.output };
  if (diff.model != null) next.model = { ...diff.model };

  const removed = new Set(diff.pinnedRemoved ?? []);
  const pins = (next.pinned ?? []).filter((pin) => !removed.has(pinKey(pin.x, pin.y)));
  for (const [index, pin] of diff.pinnedUpserted ?? []) {
    const key = pinKey(pin.x, pin.y);
    const existing = pins.findIndex((candidate) => pinKey(candidate.x, candidate.y) === key);
    if (existing >= 0) pins[existing] = { ...pin };
    else pins.splice(index, 0, { ...pin });
  }
  next.pinned = pins;
  return next;
}
