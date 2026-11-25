import fs from "node:fs";

/** ✂️ Writes the biggest core module of a component to its own file, so core-module tools such as `wasm-opt` can read it: `bun wasm-core-extract.mjs <component.wasm> <out.wasm>`.
 * @see ./🧠️pdf-model-compile-memory.md */
const leb = (bytes, offset) => { let value = 0, shift = 0, count = 0; for (;;) { const byte = bytes[offset + count++]; value += (byte & 127) * 2 ** shift; shift += 7; if (!(byte & 128)) break; } return [value, count]; };
const file = fs.readFileSync(process.argv[2]);
let best = null;
for (let offset = 8; offset < file.length;) { const id = file[offset]; const [length, width] = leb(file, offset + 1); const body = file.subarray(offset + 1 + width, offset + 1 + width + length); offset += 1 + width + length; if (id === 1 && (!best || body.length > best.length)) best = body; }
fs.writeFileSync(process.argv[3], best);
