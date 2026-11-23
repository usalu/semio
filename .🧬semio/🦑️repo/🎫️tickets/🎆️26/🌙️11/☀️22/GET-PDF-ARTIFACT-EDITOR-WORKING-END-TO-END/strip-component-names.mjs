import fs from "node:fs";

/** ✂️ Drops the `name`, `.debug*`, `producers` and `target_features` custom sections from a component and its nested core modules, the post-link equivalent of the repo's `strip = "symbols"` component profile.
 * Usage: `node strip-component-names.mjs <in.wasm> <out.wasm>`.
 * @see ./🌐️pdf-browser-journey.md */

const leb = (bytes, offset) => { let value = 0, shift = 0, count = 0; for (;;) { const byte = bytes[offset + count++]; value += (byte & 127) * 2 ** shift; shift += 7; if (!(byte & 128)) break; } return [value, count]; };
const encode = number => { const out = []; do { let byte = number % 128; number = Math.floor(number / 128); if (number) byte |= 128; out.push(byte); } while (number); return Buffer.from(out); };
const dropped = new Set(["name", "producers", "target_features"]);

function strip(buffer, stats) {
  const component = buffer.readUInt32LE(4) >>> 16 === 1;
  const out = [buffer.subarray(0, 8)];
  for (let offset = 8; offset < buffer.length;) {
    const id = buffer[offset];
    const [length, width] = leb(buffer, offset + 1);
    const body = buffer.subarray(offset + 1 + width, offset + 1 + width + length);
    offset += 1 + width + length;
    let next = body;
    if (id === 0) {
      const [nameLength, skip] = leb(body, 0);
      const name = body.subarray(skip, skip + nameLength).toString();
      if (dropped.has(name) || name.startsWith(".debug")) { stats[name] = (stats[name] ?? 0) + body.length; continue; }
    } else if (component && (id === 1 || id === 4)) next = strip(body, stats);
    out.push(Buffer.from([id]), encode(next.length), next);
  }
  return Buffer.concat(out);
}

const stats = {};
const input = fs.readFileSync(process.argv[2]);
const output = strip(input, stats);
fs.writeFileSync(process.argv[3], output);
console.log(`stripped ${input.length} -> ${output.length} bytes`, stats);
