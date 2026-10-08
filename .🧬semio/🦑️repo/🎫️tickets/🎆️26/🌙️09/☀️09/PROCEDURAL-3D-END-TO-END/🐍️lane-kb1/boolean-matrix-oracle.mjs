import { readFileSync, writeFileSync } from "node:fs";
import loadManifold from "manifold-3d";
import { measureCase } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🟦️.ts";

const [spec, out, chunk = "0", chunks = "1"] = process.argv.slice(2);
const wasm = await loadManifold();
wasm.setup();
const cases = JSON.parse(readFileSync(spec, "utf8")).cases;
const fixture = { oracleSegments: 360, checkSegments: 180, tolerance: { volume: 2e-3, area: 4e-3 }, cases: [] };
for (const [index, entry] of cases.entries()) {
  if (index % Number(chunks) !== Number(chunk)) continue;
  const started = Date.now();
  const identical = JSON.stringify(entry.a) === JSON.stringify(entry.b);
  const far = { kind: "box", size: [1, 1, 1], translate: [1000, 1000, 1000] };
  const expected = identical ? (entry.operation === "cut" ? { empty: true, volume: 0, area: 0, euler: 0, shells: 0 } : measureCase(wasm, { operation: "cut", a: entry.a, b: far }, fixture.oracleSegments)) : measureCase(wasm, entry, fixture.oracleSegments);
  fixture.cases.push({ ...entry, compareTopology: !entry.tags.includes("tangent"), expected });
  console.error(`${entry.id} ${Date.now() - started}ms vol=${expected.volume.toFixed(6)} area=${expected.area.toFixed(5)} euler=${expected.euler} shells=${expected.shells}${expected.empty ? " EMPTY" : ""}`);
}
writeFileSync(out, JSON.stringify(fixture, null, 1));
