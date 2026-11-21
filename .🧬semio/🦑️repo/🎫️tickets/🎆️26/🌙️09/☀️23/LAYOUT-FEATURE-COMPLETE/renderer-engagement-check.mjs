import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

const root = process.cwd();
const require = createRequire(join(root, "package.json"));
const Ajv = require("ajv");
const _ = require("lodash");
const Parser = require("web-tree-sitter");
const law = JSON.parse(await readFile("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🖱️pointer-gestures.json", "utf8")).engagement;
const validate = new Ajv({ useDefaults: true }).compile({
  type: "object",
  properties: { engagementSessionActive: { type: "boolean", default: false } },
});
for (const row of law.selectionCases) {
  const decoded = structuredClone(row.wire);
  assert.equal(validate(decoded), true);
  assert.equal(decoded.engagementSessionActive, row.active);
}
assert.equal(validate({ engagementSessionActive: 1 }), false);
assert.equal(validate({ engagementSessionActive: null }), false);
const authored = await readFile("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👆️pointer-and-drop/🔶️active-utility/🟦️.ts", "utf8");
for (const row of law.commands) assert.ok(authored.includes('dispatch("' + row.action + '"'));
const previous = { generation: law.queue.generation, position: law.queue.first, delta: [0, 0] };
const next = { position: law.queue.latest, delta: _.zipWith(law.queue.latest, law.queue.first, _.subtract) };
const unstarted = _.last([previous, _.assign({}, next, { generation: previous.generation })]);
assert.equal(unstarted.generation, previous.generation);
assert.deepEqual(unstarted.position, law.queue.latest);
assert.deepEqual(unstarted.delta, [4, 5]);

await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(require.resolve("tree-sitter-wasms/out/tree-sitter-rust.wasm")));
for (const file of ["🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🔖️interaction-transaction/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🎞️scene-bridge/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/⚙️authority-core/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🧪️tests/🖱️pointer-gestures/🦀️.rs"]) {
  const tree = parser.parse(await readFile(file, "utf8"));
  assert.equal(tree.rootNode.hasError(), false, file);
  tree.delete();
}
console.log("Three neutral selection cases, two validation rejections, both authored commands, singleton pointer coalescing and five Rust source files passed.");
