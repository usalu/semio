import { strict as assert } from "node:assert";
const [fixture, binary] = process.argv.slice(2);
const corpus = await Bun.file(fixture!).json();
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
  .find(module => WebAssembly.Module.exports(module).some(entry => entry.name === "mapped"));
assert.ok(core, "the linked core owns the mapped polygon witness");
const imports: Record<string, Record<string, WebAssembly.ImportValue>> = {};
for (const { module, name, kind } of WebAssembly.Module.imports(core)) {
  assert.equal(kind, "function");
  (imports[module] ??= {})[name] = () => { throw Error(module + "." + name); };
}
const { mapped } = new WebAssembly.Instance(core, imports).exports as { mapped: (mode: number, index: number, coordinate: number) => number };
let tested = 0;
for (const [mode, data] of corpus.cases.entries()) {
  if (data.kind === "disconnected" || data.kind === "collapsed") {
    assert.ok(Number.isNaN(mapped(mode, 0, 0)), data.kind);
    tested++;
    continue;
  }
  for (const [index, point] of corpus.points.entries()) {
    for (const [coordinate, value] of point.entries()) {
      const expected = data.kind === "offset" ? value + data.delta[coordinate] : value * data.scale[coordinate];
      assert.ok(mapped(mode, index, coordinate) === expected, mode + ":" + index + ":" + coordinate);
      tested++;
    }
  }
  assert.equal(mapped(mode, 3, 0), Number(data.flipped));
  tested++;
}
console.log(tested + " mapped polygon observations match the independent affine reference");
