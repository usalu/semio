/** 📸️ Bitmap snapshot — the persisted overlapping-model problem. The solved output bitmap, the
 *  contradiction verdict and the entropy map are inferences, never fields of this record.
 *
 *  The base64 codec below is written from RFC 4648 §4, not ported from the Rust: the pixel carrier
 *  is the one place a cross-language disagreement would be invisible in a diff, so both halves
 *  implement it independently and the fixture oracle compares the results. */

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
  /** Base64 of one palette index per pixel, row-major, width * height bytes. */
  pixels: string;
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
  seed: number;
  /** @state artifact */
  input: BitmapInput;
  /** @state artifact */
  output: BitmapOutputSpec;
  /** @state artifact */
  model: BitmapOverlappingModel;
  /** @state artifact */
  pinned: BitmapPinnedPixel[];
}

const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const VALUES = new Map<string, number>([...ALPHABET].map((character, index) => [character, index]));

/** 🔤️ RFC 4648 §4 standard base64, padded. */
export function encodeBase64(bytes: Uint8Array): string {
  let out = "";
  for (let start = 0; start < bytes.length; start += 3) {
    const b0 = bytes[start]!;
    const b1 = bytes[start + 1] ?? 0;
    const b2 = bytes[start + 2] ?? 0;
    const triple = (b0 << 16) | (b1 << 8) | b2;
    out += ALPHABET[(triple >> 18) & 63]! + ALPHABET[(triple >> 12) & 63]!;
    out += start + 1 < bytes.length ? ALPHABET[(triple >> 6) & 63]! : "=";
    out += start + 2 < bytes.length ? ALPHABET[triple & 63]! : "=";
  }
  return out;
}

/** 🔤️ The exact inverse; a malformed buffer throws rather than truncating. */
export function decodeBase64(text: string): Uint8Array {
  if (text.length % 4 !== 0) throw new Error("base64 length is not a multiple of four");
  const out: number[] = [];
  for (let start = 0; start < text.length; start += 4) {
    const quad = text.slice(start, start + 4);
    const pad = [...quad].filter((character) => character === "=").length;
    if (pad > 2 || (pad > 0 && !quad.endsWith("=".repeat(pad)))) throw new Error("misplaced base64 padding");
    let triple = 0;
    for (let offset = 0; offset < 4; offset += 1) {
      const character = quad[offset]!;
      if (character === "=") continue;
      const value = VALUES.get(character);
      if (value === undefined) throw new Error(`byte ${character} is outside the base64 alphabet`);
      triple |= value << (18 - 6 * offset);
    }
    out.push((triple >> 16) & 0xff);
    if (pad < 2) out.push((triple >> 8) & 0xff);
    if (pad < 1) out.push(triple & 0xff);
  }
  return Uint8Array.from(out);
}

/** 📌️ A pin's canonical id-key — pins carry no author-visible id, so coordinates are the key. */
export function pinKey(x: number, y: number): string {
  return `${x}:${y}`;
}

/** 🖼️ The decoded index buffer, or `null` when the committed base64 is not exactly this bitmap's. */
export function bitmapIndices(input: BitmapInput): Uint8Array | null {
  const decoded = decodeBase64(input.pixels);
  return decoded.length === input.width * input.height ? decoded : null;
}
