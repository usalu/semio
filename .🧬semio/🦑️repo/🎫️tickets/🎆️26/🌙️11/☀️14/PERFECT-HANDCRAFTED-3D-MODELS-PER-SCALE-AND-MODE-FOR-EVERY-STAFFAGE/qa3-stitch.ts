/** 🧵 Stacks the sheet (overlays + axonometric panels) with the axonometric rows of two further directions into one PNG: bun qa3-stitch.ts out.png sheet.png flt.png rrl.png */
import { readFileSync, writeFileSync } from "node:fs";
import { inflateSync, deflateSync, crc32 } from "node:zlib";
function dec(p: string) {
  const b = readFileSync(p); let o = 8, w = 0, h = 0, ch = 3; const idat: Buffer[] = [];
  while (o < b.length) { const n = b.readUInt32BE(o), t = b.toString("latin1", o + 4, o + 8); if (t === "IHDR") { w = b.readUInt32BE(o + 8); h = b.readUInt32BE(o + 12); ch = b[o + 17] === 6 ? 4 : 3; } if (t === "IDAT") idat.push(b.subarray(o + 8, o + 8 + n)); o += 12 + n; }
  const raw = inflateSync(Buffer.concat(idat)); const st = w * ch; const out = Buffer.alloc(h * st);
  for (let y = 0; y < h; y++) { const f = raw[y * (st + 1)]!; for (let x = 0; x < st; x++) { const v = raw[y * (st + 1) + 1 + x]!; const a = x >= ch ? out[y * st + x - ch]! : 0, u = y > 0 ? out[(y - 1) * st + x]! : 0, c = x >= ch && y > 0 ? out[(y - 1) * st + x - ch]! : 0; let r = v; if (f === 1) r = v + a; else if (f === 2) r = v + u; else if (f === 3) r = v + ((a + u) >> 1); else if (f === 4) { const p = a + u - c, pa = Math.abs(p - a), pb = Math.abs(p - u), pc = Math.abs(p - c); r = v + (pa <= pb && pa <= pc ? a : pb <= pc ? u : c); } out[y * st + x] = r & 255; } }
  return { w, h, ch, data: out };
}
const [out, s, f, r, q] = process.argv.slice(2) as [string, string, string, string, string];
const A = dec(s), B = dec(f), C = dec(r), D = dec(q);
const y0 = 330, y1 = 756; const rows: Buffer[] = [];
const st = A.w * A.ch;
for (let y = 0; y < 756; y++) rows.push(A.data.subarray(y * st, (y + 1) * st));
for (const I of [B, C, D]) for (let y = y0; y < y1; y++) rows.push(I.data.subarray(y * st, (y + 1) * st));
const H = rows.length; const raw = Buffer.alloc(H * (st + 1));
rows.forEach((row, y) => { raw[y * (st + 1)] = 0; row.copy(raw, y * (st + 1) + 1); });
const chunk = (t: string, d: Buffer) => { const l = Buffer.alloc(4); l.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t, "latin1"), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc32(td) >>> 0); return Buffer.concat([l, td, c]); };
const ih = Buffer.alloc(13); ih.writeUInt32BE(A.w, 0); ih.writeUInt32BE(H, 4); ih[8] = 8; ih[9] = A.ch === 4 ? 6 : 2;
writeFileSync(out, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ih), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
