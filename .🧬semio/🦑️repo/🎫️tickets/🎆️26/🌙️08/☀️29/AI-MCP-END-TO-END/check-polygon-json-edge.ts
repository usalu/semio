import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";

const framework = "🧰️framework/🔨️modules";
const mesh = `${framework}/🏗️mesh-engine/🚪️io`;
const owner = readFileSync(`${mesh}/📝️text/📤️polygon-json/🦀️.rs`, "utf8");
const projection = readFileSync(`${framework}/🎒️pack/🔤️json/👁️borrow/🦀️.rs`, "utf8");
const derive = readFileSync(`${framework}/🌱️value/✨️derive/👁️borrow/🦀️.rs`, "utf8");
const fixture = JSON.parse(readFileSync(`${mesh}/🧪️tests/🧫️fixtures/🔣️.json`, "utf8"));
assert.match(owner, /#\[derive\(semio_framework_value::NativeJsonBorrow\)\]\s*struct PolygonView<'a>\s*\{\s*vertices:\s*&'a Vec<\[f32;\s*3\]>/u);
assert.match(derive, /let mut remaining\s*=\s*index;/u);
assert.match(derive, /JsonWriteEdge\s*\{\s*child:\s*index,\s*cursor:\s*0\s*\}/u);
assert.match(projection, /impl<T:JsonNativeBorrow,const N:usize> JsonNativeBorrow for \[T;N\]\s*\{[^}]+\}[^\n]*\n[^\n]+self\.get\(i\)/u);
const parameters = derive.match(/fn json_edge\(&self,([^)]*)\)/u)?.[1].split(",").map(parameter => parameter.split(":")[0]);
const dispatch = projection.match(/Self::Native\(value,mode\)=>value\.json_edge\(([^)]+)\)/u)?.[1].split(",");
assert.ok(parameters && dispatch);
assert.equal(parameters.length, dispatch.length);
const childParameter = parameters.indexOf("index");
assert.equal(dispatch[childParameter], "index");
const db = new Database(":memory:");
try {
  const oracle = db.query("SELECT key, value FROM json_each(?) ORDER BY key");
  for (const vertex of fixture.authoredJson.source.vertices) {
    const selected = vertex.map((_: number, index: number) => {
      const values = { mode: 0, index, cursor: vertex.length + index, context: null };
      return vertex[values[dispatch[childParameter] as keyof typeof values] as number];
    });
    assert.deepEqual(selected, oracle.all(JSON.stringify(vertex)).map(row => (row as { value: number }).value));
  }
} finally {
  db.close();
}
console.log("[DEBUG] Borrowed mesh edge selects tuple index independently of the traversal cursor");
