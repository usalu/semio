/** 🔺️ Bitmap diff — a sparse delta over BitmapSnapshot, plus the TS half of its APPLY. The input buffer and the palette carry ORDERED edit ops (a
 * coordinate-preserving resize, rectangular writes, single-pixel cells; palette insert/remove/recolour), pins carry id-keyed row deltas. Apply is re-implemented
 * here rather than declared, because it is the one step where two languages can genuinely disagree: op order, the pad rule of a resize, and the canonical pin
 * position are all decisions, and the cross-language oracle is what pins them. */

import { pinKey, type BitmapColor, type BitmapOutputSpec, type BitmapOverlappingModel, type BitmapPinnedPixel, type BitmapSnapshot } from "../📸️snapshot/🟦️.ts";

export interface BitmapPixelRegion {
  x: number;
  y: number;
  width: number;
  height: number;
  pixels: Uint8Array;
}

export interface BitmapPixelCell {
  x: number;
  y: number;
  value: number;
}

export type BitmapInputOp = { op: "resize"; width: number; height: number } | { op: "region"; region: BitmapPixelRegion } | { op: "cells"; cells: BitmapPixelCell[] };

export type BitmapPaletteOp = { op: "insert"; index: number; color: BitmapColor } | { op: "remove"; index: number } | { op: "recolor"; index: number; color: BitmapColor };

/** 🩹 One patched row, addressed by its identity. */
export interface BitmapRowPatch<Q> {
  id: string;
  patch: Q;
}

/** 📂 Id-keyed row delta: removed identities, added rows (landing at their canonical position) and per-row field patches. */
export interface BitmapRows<T, Q> {
  removed: string[];
  added: T[];
  patched: BitmapRowPatch<Q>[];
}

/** 🕳️ An empty row delta. */
export function emptyBitmapRows<T, Q>(): BitmapRows<T, Q> {
  return { removed: [], added: [], patched: [] };
}

/** 🧬️ One keyed collection's apply — the TS twin of the Rust rows apply: removals first, then canonical-position insertions, then field patches. */
function applyRows<T, Q>(base: readonly T[], rows: BitmapRows<T, Q>, key: (row: T) => string, before: (existing: T, added: T) => boolean, patched: (row: T, patch: Q) => T): T[] {
  const items = [...base];
  for (const id of rows.removed) {
    const at = items.findIndex((item) => key(item) === id);
    if (at === -1) throw new RangeError(`removed ${id} does not exist`);
    items.splice(at, 1);
  }
  for (const row of rows.added) {
    if (items.some((item) => key(item) === key(row))) throw new RangeError(`added ${key(row)} already exists`);
    const at = items.findIndex((item) => before(item, row));
    items.splice(at === -1 ? items.length : at, 0, row);
  }
  for (const entry of rows.patched) {
    const at = items.findIndex((item) => key(item) === entry.id);
    if (at === -1) throw new RangeError(`patched ${entry.id} does not exist`);
    items[at] = patched(items[at]!, entry.patch);
  }
  return items;
}

export interface BitmapPinnedPatch {
  color: number | null;
}

export interface BitmapDiff {
  schema?: string | null;
  seed?: bigint | null;
  inputOps: BitmapInputOp[];
  paletteOps: BitmapPaletteOp[];
  output?: BitmapOutputSpec | null;
  model?: BitmapOverlappingModel | null;
  pinned: BitmapRows<BitmapPinnedPixel, BitmapPinnedPatch>;
}

/** 📐️ Keeps the overlapping top-left region and pads the new margin with palette index `0` — the exact rule the Rust `apply` performs for a resize op. */
export function resizedBuffer(buffer: Uint8Array, fromWidth: number, fromHeight: number, toWidth: number, toHeight: number): Uint8Array {
  const out = new Uint8Array(toWidth * toHeight);
  for (let y = 0; y < Math.min(fromHeight, toHeight); y += 1) {
    for (let x = 0; x < Math.min(fromWidth, toWidth); x += 1) out[y * toWidth + x] = buffer[y * fromWidth + x]!;
  }
  return out;
}

/** 🩹 Carries `before` to `after`. Input ops replay in order, then palette ops, then the output and model, then the pin rows. */
export function applyBitmapDiff(base: BitmapSnapshot, diff: BitmapDiff): BitmapSnapshot {
  const next: BitmapSnapshot = structuredClone(base);
  if (diff.schema != null) next.schema = diff.schema;
  if (diff.seed != null) next.seed = diff.seed;

  let buffer: Uint8Array = next.input.pixels.slice();
  if (buffer.length !== next.input.width * next.input.height) throw new Error("the base input pixel buffer is not width * height bytes");
  let { width, height } = next.input;
  for (const op of diff.inputOps) {
    if (op.op === "resize") {
      if (op.width <= 0 || op.height <= 0) throw new Error("an input bitmap may not have a zero edge");
      buffer = resizedBuffer(buffer, width, height, op.width, op.height);
      width = op.width;
      height = op.height;
    } else if (op.op === "region") {
      const { region } = op;
      if (region.width <= 0 || region.height <= 0 || region.pixels.length !== region.width * region.height) throw new Error("a region payload does not match its own extent");
      if (region.x + region.width > width || region.y + region.height > height) throw new Error("a region write falls outside the input bitmap");
      for (let row = 0; row < region.height; row += 1) {
        buffer.set(region.pixels.subarray(row * region.width, (row + 1) * region.width), (region.y + row) * width + region.x);
      }
    } else {
      for (const cell of op.cells) {
        if (cell.x >= width || cell.y >= height || cell.value > 255) throw new Error("a pixel write falls outside the input bitmap");
        buffer[cell.y * width + cell.x] = cell.value;
      }
    }
  }
  const palette = next.input.palette.map((color) => ({ ...color }));
  for (const op of diff.paletteOps) {
    if (op.op === "insert") {
      if (op.index > palette.length) throw new Error("a palette edit names an index outside the palette");
      palette.splice(op.index, 0, { ...op.color });
    } else if (op.index >= palette.length) {
      throw new Error("a palette edit names an index outside the palette");
    } else if (op.op === "remove") {
      palette.splice(op.index, 1);
    } else {
      palette[op.index] = { ...op.color };
    }
  }
  if (palette.length === 0) throw new Error("a palette may not be empty");
  next.input.width = width;
  next.input.height = height;
  next.input.palette = palette;
  next.input.pixels = buffer;

  if (diff.output != null) next.output = { ...diff.output };
  if (diff.model != null) next.model = { ...diff.model };

  next.pinned = applyRows(
    next.pinned ?? [],
    diff.pinned,
    (pin) => pinKey(pin.x, pin.y),
    (existing, added) => existing.y > added.y || (existing.y === added.y && existing.x > added.x),
    (row, patch) => ({ ...row, color: patch.color ?? row.color }),
  );
  return next;
}
