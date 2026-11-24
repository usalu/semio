import fs from "node:fs";

/** 🔎️ Prints the names of wasm function indexes from the `name` section of the biggest core module in an unstripped component: `node wasm-function-names.mjs <component.wasm> <index>...`.
 * @see ./🌐️pdf-browser-journey.md */
const leb = (bytes, offset) => { let value = 0, shift = 0, count = 0; for (;;) { const byte = bytes[offset + count++]; value += (byte & 127) * 2 ** shift; shift += 7; if (!(byte & 128)) break; } return [value, count]; };
const buffer = fs.readFileSync(process.argv[2]);
const wanted = new Set(process.argv.slice(3).map(Number));
let best = null;
for (let offset = 8; offset < buffer.length;) {
  const id = buffer[offset];
  const [length, width] = leb(buffer, offset + 1);
  const body = buffer.subarray(offset + 1 + width, offset + 1 + width + length);
  offset += 1 + width + length;
  if (id === 1 && (best === null || body.length > best.length)) best = body;
}
let found = 0;
for (let offset = 8; offset < best.length;) {
  const id = best[offset];
  const [length, width] = leb(best, offset + 1);
  const body = best.subarray(offset + 1 + width, offset + 1 + width + length);
  offset += 1 + width + length;
  if (id !== 0) continue;
  const [nameLength, skip] = leb(body, 0);
  if (body.subarray(skip, skip + nameLength).toString() !== "name") continue;
  for (let at = skip + nameLength; at < body.length;) {
    const sub = body[at];
    const [size, sw] = leb(body, at + 1);
    const data = body.subarray(at + 1 + sw, at + 1 + sw + size);
    at += 1 + sw + size;
    if (sub !== 1) continue;
    let [count, c] = leb(data, 0);
    let p = c;
    for (let i = 0; i < count; i++) {
      const [index, ic] = leb(data, p); p += ic;
      const [len, lc] = leb(data, p); p += lc;
      const name = data.subarray(p, p + len).toString(); p += len;
      if (wanted.has(index)) { console.log(index, name); found++; }
    }
  }
}
if (found === 0) console.log("no names found");
