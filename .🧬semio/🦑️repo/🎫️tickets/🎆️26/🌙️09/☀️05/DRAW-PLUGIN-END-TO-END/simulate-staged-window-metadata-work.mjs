/** 🔬️ Injects one measured metadata-work report into a diagnostic-only staged core. */
import { createHash } from "node:crypto";
import { cp, mkdir } from "node:fs/promises";
import { join } from "node:path";

const [source, output] = Bun.argv.slice(2);
if (!source || !output) throw Error("Expected original and diagnostic stage directories");
const module = "semio_hub_draw_component";
const file = join("module", module + ".core.wasm");
const wasm = Buffer.from(await Bun.file(join(source, file)).arrayBuffer());
if (createHash("sha256").update(wasm).digest("hex") !== "34e266b1dee28ce0ae60e0a2c0cb2919901393dec30e17988edca9bc46856058") {
  throw Error("Only the witnessed staged Draw core can be instrumented");
}
const functionBody = 88_427_764;
const insertAt = 88_428_723;
if (wasm[insertAt] !== 0x0b) throw Error("Expected the completed Window result block");
const instructions = Buffer.from([
  0x20, 3, 0x29, 3, 0, 0x50,
  0x20, 3, 0x29, 3, 8, 0x50, 0x71,
  0x20, 3, 0x28, 2, 20, 0x45, 0x45, 0x71,
  0x20, 4, 0x45, 0x71,
  0x20, 5, 0x41, 2, 0x46, 0x71,
  0x04, 0x40, 0x20, 3, 0x42, 1, 0x37, 3, 0, 0x0b,
]);
let p = 8;
function readSize() {
  let amount = 0;
  for (let shift = 0; shift <= 28; shift += 7) {
    const byte = wasm[p++];
    amount += (byte & 127) * 2 ** shift;
    if (byte < 128) return amount;
  }
  throw Error("Unbounded Wasm size");
}
function unsignedBytes(amount) {
  const bytes = [];
  while (amount >= 128) { bytes.push((amount & 127) | 128); amount >>>= 7; }
  return Buffer.from([...bytes, amount]);
}

let changed;
while (p < wasm.length) {
  const type = wasm[p++];
  const sectionLengthAt = p, sectionLength = readSize(), sectionStart = p;
  if (type !== 10) { p += sectionLength; continue; }
  for (let n = readSize(); n; --n) {
    const lengthAt = p, length = readSize(), start = p;
    p += length;
    if (start !== functionBody) continue;
    if (insertAt <= start || insertAt >= p) throw Error("Metadata report is outside the selected function");
    const newLength = unsignedBytes(length + instructions.length);
    const newSectionLength = unsignedBytes(sectionLength + instructions.length + newLength.length - (start - lengthAt));
    changed = Buffer.concat([wasm.subarray(0, sectionLengthAt), newSectionLength, wasm.subarray(sectionStart, lengthAt), newLength, wasm.subarray(start, insertAt), instructions, wasm.subarray(insertAt)]);
    break;
  }
  break;
}
if (!changed || !WebAssembly.validate(changed)) throw Error("Instrumented Draw core is invalid");
await mkdir(join(output, "module"), { recursive: true });
await cp(join(source, "vendor"), join(output, "vendor"), { recursive: true });
for (const name of [module + ".js", "🟨️.js"]) await cp(join(source, "module", name), join(output, "module", name));
await Bun.write(join(output, file), changed);
console.log("[DEBUG] Diagnostic Draw metadata-work variant", createHash("sha256").update(changed).digest("hex"));
