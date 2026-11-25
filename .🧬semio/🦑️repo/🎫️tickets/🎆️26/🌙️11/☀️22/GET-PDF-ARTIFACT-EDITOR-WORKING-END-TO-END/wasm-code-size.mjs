import fs from "node:fs";

/** 📏️ Sizes the biggest core module of a component (or a plain core module): module, code and data bytes, defined function count and largest bodies: `bun wasm-code-size.mjs <file.wasm>`.
 * @see ./🧠️pdf-model-compile-memory.md */
const leb = (bytes, offset) => { let value = 0, shift = 0, count = 0; for (;;) { const byte = bytes[offset + count++]; value += (byte & 127) * 2 ** shift; shift += 7; if (!(byte & 128)) break; } return [value, count]; };
const sections = (bytes) => { const out = []; for (let offset = 8; offset < bytes.length;) { const id = bytes[offset]; const [length, width] = leb(bytes, offset + 1); out.push({ id, body: bytes.subarray(offset + 1 + width, offset + 1 + width + length) }); offset += 1 + width + length; } return out; };
const file = fs.readFileSync(process.argv[2]);
const component = file[4] === 0x0d;
const core = component ? sections(file).filter((section) => section.id === 1).map((section) => section.body).sort((a, b) => b.length - a.length)[0] : file;
const parts = sections(core);
const code = parts.find((section) => section.id === 10)?.body ?? new Uint8Array();
const data = parts.filter((section) => section.id === 11).reduce((sum, section) => sum + section.body.length, 0);
const [count, skip] = leb(code, 0);
const sizes = [];
for (let at = skip, index = 0; index < count; index++) { const [size, width] = leb(code, at); sizes.push(size); at += width + size; }
sizes.sort((a, b) => b - a);
const mb = (bytes) => (bytes / 1e6).toFixed(1);
console.log(JSON.stringify({ file: process.argv[2].split("/").pop(), fileMB: mb(file.length), coreMB: mb(core.length), codeMB: mb(code.length), dataMB: mb(data), functions: count, medianBytes: sizes[Math.floor(sizes.length / 2)] ?? 0, largestBytes: sizes[0] ?? 0, above100KB: sizes.filter((size) => size > 100_000).length }));
