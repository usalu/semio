/** 🧬️ Bitmap mutations — the externally-tagged dispatch union, one PascalCase key per variant. */

import type { BitmapColor } from "../📸️snapshot/🟦️";

export interface ChangeSeed { seed: number }
export interface ResizeInput { width: number; height: number }
export interface SetInputPixels { x: number; y: number; width: number; height: number; pixels: string }
export interface AddPaletteColor { index: number; color: BitmapColor }
export interface ChangePaletteColor { index: number; color: BitmapColor }
export interface RemovePaletteColor { index: number }
export interface ResizeOutput { width: number; height: number; periodic: boolean }
export interface ChangeModel { patternSize: number; symmetry: number; periodicInput: boolean; ground?: number | null }
export interface PinPixel { x: number; y: number; color: number }
export interface UnpinPixel { x: number; y: number }

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
  | { UnpinPixel: UnpinPixel };

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
] as const;
