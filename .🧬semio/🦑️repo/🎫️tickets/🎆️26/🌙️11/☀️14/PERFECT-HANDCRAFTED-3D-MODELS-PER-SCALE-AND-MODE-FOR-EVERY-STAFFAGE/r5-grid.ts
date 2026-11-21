/** 🧩 Round 5: tiles rendered PNG sheets into one contact sheet so that many poses or scales can be opened at once; each tile is the source reduced by an integer factor (box average).
 * Usage: bun r5-grid.ts <out.png> <factor> <columns> <in.png>...
 * @see ./qa2-montage.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { crc32, deflateSync, inflateSync } from "node:zlib";

type Img = { w: number; h: number; ch: number; data: Buffer };

function decode(path: string): Img {
  const b = readFileSync(path);
  let o = 8;
  let w = 0;
  let h = 0;
  let ch = 3;
  const idat: Buffer[] = [];
  while (o < b.length) {
    const n = b.readUInt32BE(o);
    const t = b.toString("latin1", o + 4, o + 8);
    if (t === "IHDR") { w = b.readUInt32BE(o + 8); h = b.readUInt32BE(o + 12); ch = b[o + 17] === 6 ? 4 : 3; }
    if (t === "IDAT") idat.push(b.subarray(o + 8, o + 8 + n));
    o += 12 + n;
  }
  const raw = inflateSync(Buffer.concat(idat));
  const st = w * ch;
  const out = Buffer.alloc(h * st);
  for (let y = 0; y < h; y++) {
    const f = raw[y * (st + 1)]!;
    for (let x = 0; x < st; x++) {
      const v = raw[y * (st + 1) + 1 + x]!;
      const a = x >= ch ? out[y * st + x - ch]! : 0;
      const u = y > 0 ? out[(y - 1) * st + x]! : 0;
      const c = x >= ch && y > 0 ? out[(y - 1) * st + x - ch]! : 0;
      let r = v;
      if (f === 1) r = v + a;
      else if (f === 2) r = v + u;
      else if (f === 3) r = v + ((a + u) >> 1);
      else if (f === 4) { const p = a + u - c; const pa = Math.abs(p - a); const pb = Math.abs(p - u); const pc = Math.abs(p - c); r = v + (pa <= pb && pa <= pc ? a : pb <= pc ? u : c); }
      out[y * st + x] = r & 255;
    }
  }
  return { w, h, ch, data: out };
}

function chunk(t: string, d: Buffer): Buffer {
  const l = Buffer.alloc(4);
  l.writeUInt32BE(d.length);
  const td = Buffer.concat([Buffer.from(t, "latin1"), d]);
  const c = Buffer.alloc(4);
  c.writeUInt32BE(crc32(td) >>> 0);
  return Buffer.concat([l, td, c]);
}

const [out, fs, cs, ...files] = process.argv.slice(2) as string[];
const k = Number(fs);
const cols = Number(cs);
const imgs = files.map(decode);
const tw = Math.floor(imgs[0]!.w / k);
const th = Math.floor(imgs[0]!.h / k);
const rows = Math.ceil(imgs.length / cols);
const W = tw * cols;
const H = th * rows;
const px = Buffer.alloc(W * H * 3, 255);
imgs.forEach((im, i) => {
  const ox = (i % cols) * tw;
  const oy = Math.floor(i / cols) * th;
  for (let y = 0; y < th; y++) for (let x = 0; x < tw; x++) for (let c = 0; c < 3; c++) {
    let s = 0;
    for (let dy = 0; dy < k; dy++) for (let dx = 0; dx < k; dx++) s += im.data[((y * k + dy) * im.w + x * k + dx) * im.ch + c]!;
    px[((oy + y) * W + ox + x) * 3 + c] = Math.round(s / (k * k));
  }
  for (let y = 0; y < th; y++) for (let c = 0; c < 3; c++) px[((oy + y) * W + ox) * 3 + c] = 0;
  for (let x = 0; x < tw; x++) for (let c = 0; c < 3; c++) px[(oy * W + ox + x) * 3 + c] = 0;
});
const raw = Buffer.alloc(H * (W * 3 + 1));
for (let y = 0; y < H; y++) px.copy(raw, y * (W * 3 + 1) + 1, y * W * 3, (y + 1) * W * 3);
const ih = Buffer.alloc(13);
ih.writeUInt32BE(W, 0);
ih.writeUInt32BE(H, 4);
ih[8] = 8;
ih[9] = 2;
writeFileSync(out, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ih), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
console.log(out);
