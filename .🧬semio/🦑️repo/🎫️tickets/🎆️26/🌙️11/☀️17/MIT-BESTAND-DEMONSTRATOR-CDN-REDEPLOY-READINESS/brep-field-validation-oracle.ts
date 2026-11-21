import { strict as assert } from "node:assert";
const [fixture, binary] = process.argv.slice(2);
const corpus: { scalars: string[]; vectors: string[][] } = await Bun.file(fixture!).json();
const bytes = new Uint8Array(await Bun.file(binary!).arrayBuffer());

function* modules() {
  if (bytes[4] === 1) { yield bytes; return; }
  for (let offset = 8; offset < bytes.length;) {
    const section = bytes[offset++];
    let length = 0;
    for (let shift = 0;; shift += 7) {
      const octet = bytes[offset++]!;
      length += (octet & 127) * 2 ** shift;
      if ((octet & 128) === 0) break;
    }
    if (section === 1 && bytes[offset] === 0 && bytes[offset + 1] === 97) yield bytes.subarray(offset, offset + length);
    offset += length;
  }
}

const core = [...modules()].map(blob => new WebAssembly.Module(blob))
  .find(module => WebAssembly.Module.exports(module).some(entry => entry.name === "finite_vector"));
assert.ok(core, "the linked core owns the four Rust validation procedures");
const imports: Record<string, Record<string, WebAssembly.ImportValue>> = {};
for (const { module, name, kind } of WebAssembly.Module.imports(core)) {
  assert.equal(kind, "function");
  (imports[module] ??= {})[name] = () => { throw Error(module + "." + name); };
}
const methods = new WebAssembly.Instance(core, imports).exports as Record<string, (...args: number[]) => number>;
let tested = 0;
function check(name: string, expected: boolean, inputs: number[]) {
  assert.equal(methods[name]!(...inputs), Number(!expected), name + " " + inputs.map(x => Object.is(x, -0) ? "-0" : String(x)).join(","));
  tested++;
}
for (const raw of corpus.scalars) {
  const n = Number(raw);
  const finite = Number.isFinite(n);
  check("finite_scalar", finite, [n]);
  check("positive_size", finite && Math.sign(n) === 1, [n]);
  check("scale_factor", finite && Math.abs(n) > 0, [n]);
}
for (const input of corpus.vectors) {
  const values = input.map(Number);
  check("finite_vector", values.every(Number.isFinite), values);
}
console.log(tested + " BREP validation cases matched the independent JavaScriptCore numeric reference through the kernel contract and registry reexports");
