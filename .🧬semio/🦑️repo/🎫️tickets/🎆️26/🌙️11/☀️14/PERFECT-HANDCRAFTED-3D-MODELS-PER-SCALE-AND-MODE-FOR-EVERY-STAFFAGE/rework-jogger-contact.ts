/** 🎞️ Contact sheet of r3-sheet renders of a jogger candidate, so that several poses are read at once.
 *
 * Usage: bun rework-jogger-contact.ts <candidate 🔣️.json> <out.png> "<r3-sheet args>" ...
 * Every argument string is rendered with `r3-sheet.ts --file`, halved (2 x 2 box filter) and placed in two columns.
 */
import { deflateSync, inflateSync } from "node:zlib";
import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

type Image = { width: number; height: number; rgb: Uint8Array };

function decode(path: string): Image {
  const data = readFileSync(path);
  let offset = 8;
  let width = 0;
  let height = 0;
  let channels = 3;
  const parts: Buffer[] = [];
  while (offset < data.length) {
    const length = data.readUInt32BE(offset);
    const type = data.toString("latin1", offset + 4, offset + 8);
    const body = data.subarray(offset + 8, offset + 8 + length);
    if (type === "IHDR") {
      width = body.readUInt32BE(0);
      height = body.readUInt32BE(4);
      if (body[8] !== 8 || (body[9] !== 2 && body[9] !== 6) || body[12] !== 0) throw new Error(`unsupported PNG ${path}`);
      channels = body[9] === 6 ? 4 : 3;
    }
    if (type === "IDAT") parts.push(body);
    offset += 12 + length;
  }
  const raw = inflateSync(Buffer.concat(parts));
  const stride = width * channels;
  const rows = new Uint8Array(height * stride);
  for (let y = 0; y < height; y++) {
    const filter = raw[y * (stride + 1)];
    for (let x = 0; x < stride; x++) {
      const value = raw[y * (stride + 1) + 1 + x];
      const left = x >= channels ? rows[y * stride + x - channels] : 0;
      const up = y > 0 ? rows[(y - 1) * stride + x] : 0;
      const corner = x >= channels && y > 0 ? rows[(y - 1) * stride + x - channels] : 0;
      const p = left + up - corner;
      const paeth = Math.abs(p - left) <= Math.abs(p - up) && Math.abs(p - left) <= Math.abs(p - corner) ? left : Math.abs(p - up) <= Math.abs(p - corner) ? up : corner;
      const predicted = filter === 0 ? 0 : filter === 1 ? left : filter === 2 ? up : filter === 3 ? (left + up) >> 1 : paeth;
      rows[y * stride + x] = (value + predicted) & 255;
    }
  }
  const rgb = new Uint8Array(width * height * 3);
  for (let i = 0; i < width * height; i++) for (let c = 0; c < 3; c++) rgb[i * 3 + c] = rows[i * channels + c];
  return { width, height, rgb };
}

function halve(image: Image): Image {
  const width = image.width >> 1;
  const height = image.height >> 1;
  const rgb = new Uint8Array(width * height * 3);
  for (let y = 0; y < height; y++)
    for (let x = 0; x < width; x++)
      for (let c = 0; c < 3; c++) {
        const at = (dx: number, dy: number) => image.rgb[((y * 2 + dy) * image.width + x * 2 + dx) * 3 + c];
        rgb[(y * width + x) * 3 + c] = (at(0, 0) + at(1, 0) + at(0, 1) + at(1, 1)) >> 2;
      }
  return { width, height, rgb };
}

function crc(bytes: Uint8Array): number {
  let c = ~0;
  for (const b of bytes) {
    c ^= b;
    for (let k = 0; k < 8; k++) c = c & 1 ? (c >>> 1) ^ 0xedb88320 : c >>> 1;
  }
  return ~c >>> 0;
}

function chunk(type: string, body: Uint8Array): Buffer {
  const head = Buffer.alloc(8);
  head.writeUInt32BE(body.length, 0);
  head.write(type, 4, "latin1");
  const tail = Buffer.alloc(4);
  tail.writeUInt32BE(crc(Buffer.concat([head.subarray(4), body])), 0);
  return Buffer.concat([head, body, tail]);
}

function encode(image: Image): Buffer {
  const stride = image.width * 3;
  const raw = Buffer.alloc(image.height * (stride + 1));
  for (let y = 0; y < image.height; y++) raw.set(image.rgb.subarray(y * stride, (y + 1) * stride), y * (stride + 1) + 1);
  const header = Buffer.alloc(13);
  header.writeUInt32BE(image.width, 0);
  header.writeUInt32BE(image.height, 4);
  header.set([8, 2, 0, 0, 0], 8);
  return Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", header), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array(0))]);
}

const [file, out, ...shots] = process.argv.slice(2);
const tiles = shots.map((shot) => {
  const tool = shot.startsWith("2d ") ? "r3-2d.ts" : "r3-sheet.ts";
  const run = spawnSync(process.execPath, [tool, "--file", file, ...shot.replace(/^2d /, "").split(" ").filter(Boolean)], { encoding: "utf8" });
  const path = run.stdout.trim().split("\n").at(-1) ?? "";
  if (!path.endsWith(".png")) throw new Error(`${shot}: ${run.stdout}${run.stderr}`);
  return halve(decode(path));
});
const columns = 2;
const width = tiles[0].width;
const height = tiles[0].height;
const sheet: Image = { width: width * columns, height: height * Math.ceil(tiles.length / columns), rgb: new Uint8Array(width * columns * height * Math.ceil(tiles.length / columns) * 3).fill(255) };
tiles.forEach((tile, i) => {
  const x0 = (i % columns) * width;
  const y0 = Math.floor(i / columns) * height;
  for (let y = 0; y < height; y++) sheet.rgb.set(tile.rgb.subarray(y * width * 3, (y + 1) * width * 3), ((y0 + y) * sheet.width + x0) * 3);
});
writeFileSync(out, encode(sheet));
console.log(out, tiles.length);
