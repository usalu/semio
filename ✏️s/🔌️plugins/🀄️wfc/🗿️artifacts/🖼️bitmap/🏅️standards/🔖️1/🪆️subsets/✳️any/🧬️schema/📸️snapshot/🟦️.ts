/** 📸️ Bitmap snapshot — the persisted overlapping-model problem. The solved output bitmap, the
 *  contradiction verdict and the entropy map are inferences, never fields of this record.
 *
 *  Palette indices are intrinsic owned bytes. Physical text representations belong to I/O. */

export const WFC_BITMAP_DOCUMENT_SCHEMA = "s.wfc.bitmap";

/** 🎨️ One byte per pixel is the whole point of the index buffer, so 256 entries is the ceiling. */
export const BITMAP_MAX_PALETTE = 256;
export const BITMAP_MAX_EDGE = 512;

export interface BitmapColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface BitmapInput {
  width: number;
  height: number;
  palette: BitmapColor[];
  /** One owned palette index per pixel, row-major, width * height bytes. */
  pixels: Uint8Array;
}

export interface BitmapOutputSpec {
  width: number;
  height: number;
  periodic: boolean;
}

export interface BitmapOverlappingModel {
  patternSize: number;
  symmetry: number;
  periodicInput: boolean;
  ground?: number | null;
}

export interface BitmapPinnedPixel {
  x: number;
  y: number;
  color: number;
}

export interface BitmapSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  seed: bigint;
  /** @state artifact */
  input: BitmapInput;
  /** @state artifact */
  output: BitmapOutputSpec;
  /** @state artifact */
  model: BitmapOverlappingModel;
  /** @state artifact */
  pinned: BitmapPinnedPixel[];
}

/** 📌️ A pin's canonical id-key — pins carry no author-visible id, so coordinates are the key. */
export function pinKey(x: number, y: number): string {
  return `${x}:${y}`;
}

/** 🖼️ The decoded index buffer, or `null` when the committed index buffer is not exactly this bitmap's. */
export function bitmapIndices(input: BitmapInput): Uint8Array | null {
  const decoded = input.pixels.slice();
  return decoded.length === input.width * input.height ? decoded : null;
}
