/** 🧬️ Bitmap mutations — the externally-tagged dispatch union, one PascalCase key per variant. */

import type { BitmapColor } from "../📸️snapshot/🟦️";

export interface ChangeSeed { seed: bigint }
export interface ResizeInput { width: number; height: number }
export interface SetInputPixels { x: number; y: number; width: number; height: number; pixels: string }
export interface AddPaletteColor { index: number; color: BitmapColor }
export interface ChangePaletteColor { index: number; color: BitmapColor }
export interface RemovePaletteColor { index: number }
export interface ResizeOutput { width: number; height: number; periodic: boolean }
export interface ChangeModel { patternSize: number; symmetry: number; periodicInput: boolean; ground?: number | null }
export interface PinPixel { x: number; y: number; color: number }
export interface UnpinPixel { x: number; y: number }
export interface BitmapStrokePoint { x: number; y: number }
export interface PaintInputStroke { points: BitmapStrokePoint[]; color: number }

export type BitmapMutation =
  | { ChangeSeed: ChangeSeed }
  | { ResizeInput: ResizeInput }
  | { SetInputPixels: SetInputPixels }
  | { AddPaletteColor: AddPaletteColor }
  | { ChangePaletteColor: ChangePaletteColor }
  | { RemovePaletteColor: RemovePaletteColor }
  | { ResizeOutput: ResizeOutput }
  | { ChangeModel: ChangeModel }
  | { PinPixel: PinPixel }
  | { UnpinPixel: UnpinPixel }
  | { PaintInputStroke: PaintInputStroke };

/** 🏷️ The kebab-case kind vocabulary, in BitmapMutation declaration order. */
export const BITMAP_MUTATION_KINDS = [
  "change-seed",
  "resize-input",
  "set-input-pixels",
  "add-palette-color",
  "change-palette-color",
  "remove-palette-color",
  "resize-output",
  "change-model",
  "pin-pixel",
  "unpin-pixel",
  "paint-input-stroke",
] as const;

/** 🧮️ The most points one stroke carries, as the Rust leaf and the payload schema bound it. */
export const BITMAP_STROKE_MAXIMUM_POINTS = 4096;

/** ✍️ Decodes one `paint-input-stroke` payload, refusing what its schema refuses: no or too many points, a
 * non-integer or negative coordinate, an unknown key, or a palette index outside 0..255. */
export function parsePaintInputStroke(value: unknown): PaintInputStroke {
  const cell = (raw: unknown): number => {
    if (typeof raw !== "number" || !Number.isInteger(raw) || raw < 0 || raw > 4294967295) throw new TypeError("Invalid stroke coordinate");
    return raw;
  };
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new TypeError("Invalid paint-input-stroke payload");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => key !== "points" && key !== "color")) throw new TypeError("Invalid paint-input-stroke payload");
  if (!Array.isArray(row.points) || row.points.length < 1 || row.points.length > BITMAP_STROKE_MAXIMUM_POINTS) throw new TypeError("A stroke carries 1 to 4096 points");
  const points = row.points.map((point): BitmapStrokePoint => {
    if (!point || typeof point !== "object" || Array.isArray(point)) throw new TypeError("Invalid stroke point");
    const entry = point as Record<string, unknown>;
    if (Object.keys(entry).some((key) => key !== "x" && key !== "y")) throw new TypeError("Invalid stroke point");
    return { x: cell(entry.x), y: cell(entry.y) };
  });
  if (typeof row.color !== "number" || !Number.isInteger(row.color) || row.color < 0 || row.color > 255) throw new TypeError("Invalid palette index");
  return { points, color: row.color };
}

/** 〰️ Every cell a stroke covers, each once, in drawing order — the first point, then an inclusive Bresenham line
 * to every following point; the twin of the Rust leaf's `stroke_cells`. */
export function strokeCells(points: readonly BitmapStrokePoint[]): BitmapStrokePoint[] {
  const cells: BitmapStrokePoint[] = [];
  const seen = new Set<string>();
  const visit = (x: number, y: number) => {
    const key = `${x},${y}`;
    if (!seen.has(key)) {
      seen.add(key);
      cells.push({ x, y });
    }
  };
  const first = points[0];
  if (!first) return cells;
  visit(first.x, first.y);
  for (let index = 1; index < points.length; index += 1) {
    let { x, y } = points[index - 1]!;
    const end = points[index]!;
    const dx = Math.abs(end.x - x), dy = -Math.abs(end.y - y);
    const stepX = x < end.x ? 1 : -1, stepY = y < end.y ? 1 : -1;
    let error = dx + dy;
    for (;;) {
      visit(x, y);
      if (x === end.x && y === end.y) break;
      const doubled = 2 * error;
      if (doubled >= dy) {
        error += dy;
        x += stepX;
      }
      if (doubled <= dx) {
        error += dx;
        y += stepY;
      }
    }
  }
  return cells;
}
