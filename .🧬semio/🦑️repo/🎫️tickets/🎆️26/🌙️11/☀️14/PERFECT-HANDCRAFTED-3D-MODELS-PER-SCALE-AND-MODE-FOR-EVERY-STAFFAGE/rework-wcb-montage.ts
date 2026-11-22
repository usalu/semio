/** 🖼️ Montage of chosen panels of `r3-sheet` PNGs for reading several poses at once: `bun rework-wcb-montage.ts <out.png> <columns> <panels 0..5, comma> <sheet.png>...` */
import { deflateSync, inflateSync } from "node:zlib";
import { readFileSync, writeFileSync } from "node:fs";

type Image = { w: number; h: number; c: number; d: Uint8Array };

const crcTable = Array.from({ length: 256 }, (_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
const crc = (b: Uint8Array) => { let c = 0xffffffff; for (const x of b) c = crcTable[(c ^ x) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };

function decode(path: string): Image {
  const b = readFileSync(path);
  let o = 8, w = 0, h = 0, c = 3;
  const parts: Buffer[] = [];
  while (o < b.length) {
    const n = b.readUInt32BE(o), type = b.toString("latin1", o + 4, o + 8);
    if (type === "IHDR") { w = b.readUInt32BE(o + 8); h = b.readUInt32BE(o + 12); c = b[o + 17] === 6 ? 4 : 3; }
    if (type === "IDAT") parts.push(b.subarray(o + 8, o + 8 + n));
    o += 12 + n;
  }
  const raw = inflateSync(Buffer.concat(parts)), s = w * c, d = new Uint8Array(h * s);
  for (let y = 0; y < h; y++) {
    const f = raw[y * (s + 1)]!, r = y * (s + 1) + 1, q = y * s;
    for (let x = 0; x < s; x++) {
      const a = x >= c ? d[q + x - c]! : 0, up = y ? d[q - s + x]! : 0, ul = y && x >= c ? d[q - s + x - c]! : 0;
      const p = a + up - ul, pa = Math.abs(p - a), pb = Math.abs(p - up), pc = Math.abs(p - ul);
      d[q + x] = (raw[r + x]! + (f === 0 ? 0 : f === 1 ? a : f === 2 ? up : f === 3 ? (a + up) >> 1 : pa <= pb && pa <= pc ? a : pb <= pc ? up : ul)) & 255;
    }
  }
  return { w, h, c, d };
}

function encode(path: string, w: number, h: number, rgb: Uint8Array) {
  const raw = new Uint8Array(h * (w * 3 + 1));
  for (let y = 0; y < h; y++) raw.set(rgb.subarray(y * w * 3, (y + 1) * w * 3), y * (w * 3 + 1) + 1);
  const chunk = (type: string, data: Uint8Array) => { const body = Buffer.concat([Buffer.from(type, "latin1"), data]), head = Buffer.alloc(4), tail = Buffer.alloc(4); head.writeUInt32BE(data.length); tail.writeUInt32BE(crc(body)); return Buffer.concat([head, body, tail]); };
  const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(w, 0); ihdr.writeUInt32BE(h, 4); ihdr[8] = 8; ihdr[9] = 2;
  writeFileSync(path, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array(0))]));
}

const [out, columnsText, panelsText, ...files] = process.argv.slice(2);
const columns = Number(columnsText), panels = panelsText!.split(",").map(Number), ph = 740;
const tiles = files.flatMap((file) => { const image = decode(file); return panels.map((panel) => ({ image, x0: (panel % 3) * Math.floor(image.w / 3), y0: Math.floor(panel / 3) * ph })); });
const pw = Math.floor(tiles[0]!.image.w / 3), rows = Math.ceil(tiles.length / columns), W = columns * pw, H = rows * ph, rgb = new Uint8Array(W * H * 3).fill(255);
tiles.forEach(({ image, x0, y0 }, i) => {
  const ox = (i % columns) * pw, oy = Math.floor(i / columns) * ph;
  for (let y = 0; y < ph && y0 + y < image.h; y++) for (let x = 0; x < pw; x++) { const s = ((y0 + y) * image.w + x0 + x) * image.c, t = ((oy + y) * W + ox + x) * 3; rgb[t] = image.d[s]!; rgb[t + 1] = image.d[s + 1]!; rgb[t + 2] = image.d[s + 2]!; }
});
encode(out!, W, H, rgb);
console.log(out);
