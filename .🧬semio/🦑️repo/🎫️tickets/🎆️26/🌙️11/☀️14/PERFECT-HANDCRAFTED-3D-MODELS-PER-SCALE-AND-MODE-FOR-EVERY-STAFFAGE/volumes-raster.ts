/** 🖼️ Owned software rasteriser for the volume authoring renders: RGB canvas, z-buffered flat-shaded orthographic triangles with a per-pixel clip, a 5x7 bitmap font and a PNG encoder (system zlib only).
 *
 * No external libraries; deterministic under bun and node.
 * @see ./volumes-tools.md */
import { deflateSync } from "node:zlib";

export type RGB = readonly [number, number, number];

/** 🎨️ An RGB canvas with a depth buffer. */
export class Canvas {
  readonly data: Uint8ClampedArray;
  readonly depth: Float32Array;
  constructor(readonly width: number, readonly height: number, background: RGB = [255, 255, 255]) {
    this.data = new Uint8ClampedArray(width * height * 3);
    this.depth = new Float32Array(width * height).fill(-Infinity);
    this.fill(background);
  }

  fill(color: RGB): void {
    for (let i = 0; i < this.width * this.height; i++) {
      this.data[i * 3] = color[0];
      this.data[i * 3 + 1] = color[1];
      this.data[i * 3 + 2] = color[2];
    }
  }

  clearDepth(): void {
    this.depth.fill(-Infinity);
  }

  set(x: number, y: number, c: RGB): void {
    if (x < 0 || y < 0 || x >= this.width || y >= this.height) return;
    const i = (y * this.width + x) * 3;
    this.data[i] = c[0];
    this.data[i + 1] = c[1];
    this.data[i + 2] = c[2];
  }

  get(x: number, y: number): RGB {
    const i = (y * this.width + x) * 3;
    return [this.data[i]!, this.data[i + 1]!, this.data[i + 2]!];
  }

  rect(x: number, y: number, w: number, h: number, c: RGB): void {
    for (let j = Math.max(0, y); j < Math.min(this.height, y + h); j++) for (let i = Math.max(0, x); i < Math.min(this.width, x + w); i++) this.set(i, j, c);
  }

  frame(x: number, y: number, w: number, h: number, c: RGB): void {
    this.rect(x, y, w, 1, c);
    this.rect(x, y + h - 1, w, 1, c);
    this.rect(x, y, 1, h, c);
    this.rect(x + w - 1, y, 1, h, c);
  }

  line(x0: number, y0: number, x1: number, y1: number, c: RGB): void {
    const n = Math.max(1, Math.ceil(Math.max(Math.abs(x1 - x0), Math.abs(y1 - y0))));
    for (let k = 0; k <= n; k++) this.set(Math.round(x0 + ((x1 - x0) * k) / n), Math.round(y0 + ((y1 - y0) * k) / n), c);
  }

  /** 🔺️ Fills one screen triangle `[x, y, depth, clipValue]` per corner; larger depth is nearer; pixels whose interpolated clip value exceeds `clipMax` are dropped. `shade` multiplies the colour. */
  triangle(a: readonly number[], b: readonly number[], c: readonly number[], color: RGB, clipMax = Infinity, depthEps = 0, hatch: RGB | null = null): void {
    const minX = Math.max(0, Math.floor(Math.min(a[0]!, b[0]!, c[0]!)));
    const maxX = Math.min(this.width - 1, Math.ceil(Math.max(a[0]!, b[0]!, c[0]!)));
    const minY = Math.max(0, Math.floor(Math.min(a[1]!, b[1]!, c[1]!)));
    const maxY = Math.min(this.height - 1, Math.ceil(Math.max(a[1]!, b[1]!, c[1]!)));
    const den = (b[1]! - c[1]!) * (a[0]! - c[0]!) + (c[0]! - b[0]!) * (a[1]! - c[1]!);
    if (Math.abs(den) < 1e-12) return;
    for (let y = minY; y <= maxY; y++) {
      const py = y + 0.5;
      for (let x = minX; x <= maxX; x++) {
        const px = x + 0.5;
        const l0 = ((b[1]! - c[1]!) * (px - c[0]!) + (c[0]! - b[0]!) * (py - c[1]!)) / den;
        const l1 = ((c[1]! - a[1]!) * (px - c[0]!) + (a[0]! - c[0]!) * (py - c[1]!)) / den;
        const l2 = 1 - l0 - l1;
        if (l0 < -1e-9 || l1 < -1e-9 || l2 < -1e-9) continue;
        if (clipMax !== Infinity && l0 * a[3]! + l1 * b[3]! + l2 * c[3]! > clipMax) continue;
        const d = l0 * a[2]! + l1 * b[2]! + l2 * c[2]!;
        const i = y * this.width + x;
        if (d <= this.depth[i]! + depthEps) continue;
        this.depth[i] = d;
        const paint = hatch !== null && (x + y) % 8 < 4 ? hatch : color;
        this.data[i * 3] = paint[0];
        this.data[i * 3 + 1] = paint[1];
        this.data[i * 3 + 2] = paint[2];
      }
    }
  }

  /** 🔽️ Box-filter downsample by an integer factor. */
  downsample(factor: number): Canvas {
    const out = new Canvas(Math.floor(this.width / factor), Math.floor(this.height / factor));
    const n = factor * factor;
    for (let y = 0; y < out.height; y++) for (let x = 0; x < out.width; x++) {
      let r = 0;
      let g = 0;
      let b = 0;
      for (let j = 0; j < factor; j++) for (let i = 0; i < factor; i++) {
        const k = ((y * factor + j) * this.width + x * factor + i) * 3;
        r += this.data[k]!;
        g += this.data[k + 1]!;
        b += this.data[k + 2]!;
      }
      out.set(x, y, [r / n, g / n, b / n]);
    }
    return out;
  }

  blit(source: Canvas, x: number, y: number): void {
    for (let j = 0; j < source.height; j++) for (let i = 0; i < source.width; i++) this.set(x + i, y + j, source.get(i, j));
  }

  text(x: number, y: number, value: string, color: RGB = [20, 20, 20], scale = 2): number {
    let cx = x;
    for (const ch of value.toUpperCase()) {
      const rows = FONT[ch] ?? FONT["?"]!;
      for (let r = 0; r < 7; r++) for (let q = 0; q < 5; q++) if ((rows[r]! >> (4 - q)) & 1) this.rect(cx + q * scale, y + r * scale, scale, scale, color);
      cx += 6 * scale;
    }
    return cx;
  }

  png(): Uint8Array {
    return encodePng(this.width, this.height, this.data);
  }
}

const FONT: Record<string, readonly number[]> = {
  " ": [0, 0, 0, 0, 0, 0, 0],
  A: [0x0e, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
  B: [0x1e, 0x11, 0x11, 0x1e, 0x11, 0x11, 0x1e],
  C: [0x0e, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0e],
  D: [0x1e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1e],
  E: [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x1f],
  F: [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x10],
  G: [0x0e, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0f],
  H: [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
  I: [0x0e, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0e],
  J: [0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0c],
  K: [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
  L: [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1f],
  M: [0x11, 0x1b, 0x15, 0x15, 0x11, 0x11, 0x11],
  N: [0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11],
  O: [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
  P: [0x1e, 0x11, 0x11, 0x1e, 0x10, 0x10, 0x10],
  Q: [0x0e, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0d],
  R: [0x1e, 0x11, 0x11, 0x1e, 0x14, 0x12, 0x11],
  S: [0x0f, 0x10, 0x10, 0x0e, 0x01, 0x01, 0x1e],
  T: [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
  U: [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
  V: [0x11, 0x11, 0x11, 0x11, 0x11, 0x0a, 0x04],
  W: [0x11, 0x11, 0x11, 0x15, 0x15, 0x15, 0x0a],
  X: [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11],
  Y: [0x11, 0x11, 0x11, 0x0a, 0x04, 0x04, 0x04],
  Z: [0x1f, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1f],
  "0": [0x0e, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0e],
  "1": [0x04, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x0e],
  "2": [0x0e, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1f],
  "3": [0x1f, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0e],
  "4": [0x02, 0x06, 0x0a, 0x12, 0x1f, 0x02, 0x02],
  "5": [0x1f, 0x10, 0x1e, 0x01, 0x01, 0x11, 0x0e],
  "6": [0x06, 0x08, 0x10, 0x1e, 0x11, 0x11, 0x0e],
  "7": [0x1f, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
  "8": [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e],
  "9": [0x0e, 0x11, 0x11, 0x0f, 0x01, 0x02, 0x0c],
  "-": [0, 0, 0, 0x1f, 0, 0, 0],
  ".": [0, 0, 0, 0, 0, 0x0c, 0x0c],
  ":": [0, 0x0c, 0x0c, 0, 0x0c, 0x0c, 0],
  "/": [0x01, 0x01, 0x02, 0x04, 0x08, 0x10, 0x10],
  "%": [0x18, 0x19, 0x02, 0x04, 0x08, 0x13, 0x03],
  "(": [0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02],
  ")": [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08],
  "=": [0, 0, 0x1f, 0, 0x1f, 0, 0],
  ",": [0, 0, 0, 0, 0x0c, 0x04, 0x08],
  "+": [0, 0x04, 0x04, 0x1f, 0x04, 0x04, 0],
  _: [0, 0, 0, 0, 0, 0, 0x1f],
  "<": [0x02, 0x04, 0x08, 0x10, 0x08, 0x04, 0x02],
  ">": [0x08, 0x04, 0x02, 0x01, 0x02, 0x04, 0x08],
  "*": [0, 0x04, 0x15, 0x0e, 0x15, 0x04, 0],
  "|": [0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
  "?": [0x0e, 0x11, 0x01, 0x02, 0x04, 0, 0x04],
};

const CRC = (() => {
  const table = new Uint32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    table[n] = c >>> 0;
  }
  return table;
})();

const crc32 = (bytes: Uint8Array): number => {
  let c = 0xffffffff;
  for (let i = 0; i < bytes.length; i++) c = CRC[(c ^ bytes[i]!) & 0xff]! ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
};

/** 📦️ Encodes 8-bit RGB pixels as a PNG file. */
export function encodePng(width: number, height: number, rgb: Uint8ClampedArray): Uint8Array {
  const raw = new Uint8Array((width * 3 + 1) * height);
  for (let y = 0; y < height; y++) {
    raw[y * (width * 3 + 1)] = 0;
    raw.set(rgb.subarray(y * width * 3, (y + 1) * width * 3), y * (width * 3 + 1) + 1);
  }
  const chunk = (type: string, body: Uint8Array): Uint8Array => {
    const out = new Uint8Array(12 + body.length);
    const view = new DataView(out.buffer);
    view.setUint32(0, body.length);
    for (let i = 0; i < 4; i++) out[4 + i] = type.charCodeAt(i);
    out.set(body, 8);
    view.setUint32(8 + body.length, crc32(out.subarray(4, 8 + body.length)));
    return out;
  };
  const header = new Uint8Array(13);
  const hv = new DataView(header.buffer);
  hv.setUint32(0, width);
  hv.setUint32(4, height);
  header[8] = 8;
  header[9] = 2;
  const parts = [new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", header), chunk("IDAT", new Uint8Array(deflateSync(raw, { level: 6 }))), chunk("IEND", new Uint8Array(0))];
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let at = 0;
  for (const p of parts) {
    out.set(p, at);
    at += p.length;
  }
  return out;
}

/** 🌈️ HSL to RGB, h in degrees, s and l in 0..1. */
export function hsl(h: number, s: number, l: number): RGB {
  const k = (n: number) => (n + h / 30) % 12;
  const a = s * Math.min(l, 1 - l);
  const f = (n: number) => l - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));
  return [Math.round(f(0) * 255), Math.round(f(8) * 255), Math.round(f(4) * 255)];
}

export const hashHue = (text: string): number => {
  let h = 2166136261;
  for (let i = 0; i < text.length; i++) h = Math.imul(h ^ text.charCodeAt(i), 16777619);
  return (h >>> 0) % 360;
};
