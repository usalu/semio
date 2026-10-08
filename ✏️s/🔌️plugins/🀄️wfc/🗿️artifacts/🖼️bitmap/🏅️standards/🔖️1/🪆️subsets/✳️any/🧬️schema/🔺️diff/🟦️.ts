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

export interface BitmapSize {
  width: number;
  height: number;
}

/** 🖼️ One write into the input index buffer, in the coordinates of the buffer AFTER the patch's resize; writes layer in list order. */
export type BitmapInputWrite = { op: "region"; region: BitmapPixelRegion } | { op: "cells"; cells: BitmapPixelCell[] };

/** 🖼️ The input patch: an optional coordinate-preserving resize of the base buffer, then the writes. */
export interface BitmapInputPatch {
  size: BitmapSize | null;
  writes: BitmapInputWrite[];
}

export interface BitmapPaletteInsertion {
  index: number;
  color: BitmapColor;
}

export interface BitmapPaletteRecolor {
  index: number;
  color: BitmapColor;
}

/** 🎨️ The positional palette delta: `removed` are BASE indices, `inserted` carry AFTER indices, `recolored` are BASE indices of surviving colours. */
export interface BitmapPaletteDelta {
  removed: number[];
  inserted: BitmapPaletteInsertion[];
  recolored: BitmapPaletteRecolor[];
}

/** 🩹 One patched row, addressed by its identity. */
export interface BitmapRowPatch<Q> {
  id: string;
  patch: Q;
}

/** 📂 Positional row delta (`protocol::list_delta`): removed keys at their BASE index, inserted rows at their AFTER index, moved keys, and key-addressed patches. */
export interface BitmapRows<T, Q> {
  removed: { id: string; index: number }[];
  inserted: { index: number; row: T }[];
  moved: { id: string; from: number; to: number }[];
  modified: BitmapRowPatch<Q>[];
}

/** 🕳️ An empty row delta. */
export function emptyBitmapRows<T, Q>(): BitmapRows<T, Q> {
  return { removed: [], inserted: [], moved: [], modified: [] };
}

/** 🧬️ One keyed collection's apply — the TS twin of the Rust positional list-delta apply: removed and moved keys are checked at their base index, inserted and moved rows take their after slots, survivors fill the rest in base order, then the patches write. */
function applyRows<T, Q>(base: readonly T[], rows: BitmapRows<T, Q>, key: (row: T) => string, patched: (row: T, patch: Q) => T): T[] {
  const taken = new Set<number>();
  for (const entry of [...rows.removed, ...rows.moved.map((move) => ({ id: move.id, index: move.from }))]) {
    if (base[entry.index] === undefined || key(base[entry.index]!) !== entry.id || taken.has(entry.index)) throw new RangeError(`${entry.id} is not at base index ${entry.index}`);
    taken.add(entry.index);
  }
  const slots: (T | undefined)[] = new Array(base.length - rows.removed.length + rows.inserted.length).fill(undefined);
  for (const entry of rows.inserted) {
    if (entry.index >= slots.length || slots[entry.index] !== undefined) throw new RangeError(`inserted ${key(entry.row)} has no free after slot ${entry.index}`);
    slots[entry.index] = entry.row;
  }
  for (const move of rows.moved) slots[move.to] = base[move.from];
  const survivors = base.filter((_, index) => !taken.has(index));
  const items = slots.map((slot) => slot ?? survivors.shift()!);
  if (new Set(items.map(key)).size !== items.length) throw new RangeError("two rows of the after list carry the same key");
  for (const entry of rows.modified) {
    const at = items.findIndex((item) => key(item) === entry.id);
    if (at === -1) throw new RangeError(`modified ${entry.id} does not exist`);
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
  input: BitmapInputPatch;
  palette: BitmapPaletteDelta;
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

/** 🩹 Carries `before` to `after`. The input patch (resize, then writes in order) runs first, then the palette delta, then the output and model, then the pin rows. */
export function applyBitmapDiff(base: BitmapSnapshot, diff: BitmapDiff): BitmapSnapshot {
  const next: BitmapSnapshot = structuredClone(base);
  if (diff.schema != null) next.schema = diff.schema;
  if (diff.seed != null) next.seed = diff.seed;

  const baseBuffer: Uint8Array = next.input.pixels.slice();
  if (baseBuffer.length !== next.input.width * next.input.height) throw new Error("the base input pixel buffer is not width * height bytes");
  const width = diff.input.size?.width ?? next.input.width;
  const height = diff.input.size?.height ?? next.input.height;
  if (width <= 0 || height <= 0) throw new Error("an input bitmap may not have a zero edge");
  const buffer = resizedBuffer(baseBuffer, next.input.width, next.input.height, width, height);
  for (const write of diff.input.writes) {
    if (write.op === "region") {
      const { region } = write;
      if (region.width <= 0 || region.height <= 0 || region.pixels.length !== region.width * region.height) throw new Error("a region payload does not match its own extent");
      if (region.x + region.width > width || region.y + region.height > height) throw new Error("a region write falls outside the input bitmap");
      for (let row = 0; row < region.height; row += 1) {
        buffer.set(region.pixels.subarray(row * region.width, (row + 1) * region.width), (region.y + row) * width + region.x);
      }
    } else {
      for (const cell of write.cells) {
        if (cell.x >= width || cell.y >= height || cell.value > 255) throw new Error("a pixel write falls outside the input bitmap");
        buffer[cell.y * width + cell.x] = cell.value;
      }
    }
  }
  const basePalette = next.input.palette;
  const { removed, inserted, recolored } = diff.palette;
  if (removed.some((index, at) => index >= basePalette.length || removed.indexOf(index) !== at)) throw new Error("a palette removal names an index outside the palette or twice");
  if (recolored.some((entry, at) => entry.index >= basePalette.length || removed.includes(entry.index) || recolored.findIndex((prior) => prior.index === entry.index) !== at)) throw new Error("a palette recolour names an index outside the palette, a removed colour or twice");
  const afterLength = basePalette.length + inserted.length - removed.length;
  if (afterLength < 0 || inserted.some((entry, at) => entry.index >= afterLength || inserted.findIndex((prior) => prior.index === entry.index) !== at)) throw new Error("a palette insertion lies past the end of the after palette or twice");
  const survivors = basePalette.map((color, index) => ({ color, index })).filter(({ index }) => !removed.includes(index)).map(({ color, index }) => ({ ...(recolored.find((entry) => entry.index === index)?.color ?? color) }));
  const palette: BitmapColor[] = [];
  for (let slot = 0; slot < afterLength; slot += 1) {
    const entry = inserted.find((candidate) => candidate.index === slot);
    const color = entry ? { ...entry.color } : survivors.shift();
    if (!color) throw new Error("the after palette has more free slots than surviving colours");
    palette.push(color);
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
    (row, patch) => ({ ...row, color: patch.color ?? row.color }),
  );
  return next;
}
