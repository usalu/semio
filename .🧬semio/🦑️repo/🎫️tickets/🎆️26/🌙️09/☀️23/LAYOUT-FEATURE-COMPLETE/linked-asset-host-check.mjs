import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import Ajv from "ajv";
import sharp from "sharp";
import ts from "typescript";

const subset = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any";
const example = resolve(subset, "📚️examples/🔗️linked-asset/🦀️.rs");
const journeyRoot = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END";
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance";
const load = path => readFile(path, "utf8");
const [registration, examples, app, journey, contract, panel, catalogue, render, command] = await Promise.all([
  load(example),
  load(resolve(subset, "📚️examples/🦀️.rs")),
  load(resolve(subset, "✏️editor/🕹️layout-play-app/🦀️.rs")),
  load(resolve(journeyRoot, "browser-journey.mjs")),
  load(resolve(journeyRoot, "browser-journey.json")).then(JSON.parse),
  load(resolve(subset, "✏️editor/📌️panels/🗿️artifact/🦀️.rs")),
  load(resolve(subset, "✏️editor/📌️panels/🛍️catalogue/🦀️.rs")),
  load(resolve(plugin, "🔌️plugin-app-impl/🖼️render-surface/🦀️.rs")),
  load(resolve(plugin, "🧷️command-routes/📥️staging/🚦️actions/🦀️.rs")),
]);
const linked = contract.physicalInteractions.linkedAsset;
assert.equal(registration.match(/pub const ID: &str = "([^"]+)"/)?.[1], linked.exampleId);
assert.match(examples, /pub mod linked_asset;/);
assert.match(app, /layout_examples::linked_asset::source\(\)/);
assert.match(app, /catalogue_panel::render\(&document\.links, doc\.parent_document_id\(\)/);
assert.match(render, /let parent_document_id = self\.stores\.store\.envelope\(\)\.id\.clone\(\)/);
assert.match(command, /parent_document_id: self\.stores\.store\.envelope\(\)\.id\.clone\(\)/);
const payload = await load(resolve(dirname(example), registration.match(/include_str!\("([^"]+)"\)/)[1]));
const link = payload.match(/\{ id=([^\s]+) path=([^\s]+).*? width=(\d+) height=(\d+).*?proxy-data-url="data:image\/png;base64,([^"]+)"/);
assert.equal(link[1], linked.linkId);
assert.equal(link[2], linked.rowLabel);
const [, initialFrameId, frameLinkId] = payload.match(/image id=([^\s]+) layer-id=[^\s]+ link-id=([^\s]+)/);
assert.equal(frameLinkId, linked.linkId);
assert.equal(initialFrameId, linked.originalFrameId);
assert.match(initialFrameId, /^frame-/);
assert.match(payload, new RegExp(`\\[ ${initialFrameId} \\]`));
const pageId = payload.match(/\{ id=([^\s]+) name="Page [^"]+" spread-id=/)[1];
const pagePrefix = panel.match(/format!\("(layout-document\.page\.)\{page_id\}"\)/)[1];
assert.equal(pagePrefix + pageId, linked.pageRowKey);
const cataloguePanel = catalogue.match(/link_presentation_key\("(layout-catalogue)"/)[1];
const rowPrefix = cataloguePanel + ".link.";
assert.equal(rowPrefix + linked.linkId, linked.rowKey);
const layout = "✏️s/🔌️plugins/📏️layout";
const [wireSchema, fixture] = await Promise.all([
  load(resolve(layout, "🧬️schema/🛍️canvas-catalogue/linked-asset.json")).then(JSON.parse),
  load(resolve(layout, "🧪️tests/🧫️fixtures/🛍️canvas-catalogue/linked-asset.json")).then(JSON.parse),
]);
const validWire = new Ajv().compile(wireSchema);
const { maximumTextBytes, links, selection } = fixture.catalogueBoundary;
assert.ok(validWire(fixture.expected.drag));
assert.ok(Buffer.byteLength(JSON.stringify(fixture.expected.drag)) <= maximumTextBytes);
for (const link of links) {
  const id = link.character.repeat(link.count);
  const exactWire = { ...fixture.expected.drag, sourceLinkId: id };
  assert.ok(validWire(exactWire));
  assert.ok(Buffer.byteLength(JSON.stringify(exactWire)) > maximumTextBytes);
  assert.equal(Buffer.byteLength(rowPrefix + id) <= maximumTextBytes, link.rowKeyFits);
}
assert.deepEqual(selection.map(target => target.id), [initialFrameId]);
assert.ok(Buffer.byteLength(JSON.stringify(selection)) <= maximumTextBytes);
const png = await sharp(Buffer.from(link[5], "base64")).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
assert.equal(png.info.width, Number(link[3]));
assert.equal(png.info.height, Number(link[4]));
const colors = new Set();
for (let offset = 0; offset < png.data.length; offset += 4) {
  const pixel = png.data.subarray(offset, offset + 4);
  assert.equal(pixel[3], 255);
  colors.add(pixel.toString("hex"));
}
assert.equal(colors.size, 3);
const source = ts.createSourceFile("browser-journey.mjs", journey, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
assert.deepEqual(source.parseDiagnostics, []);
assert.match(journey, /next\.searchParams\.set\("example", linked\.exampleId\)/);
assert.match(journey, /state\.frameIds\.includes\(linked\.originalFrameId\)/);
assert.match(journey, /getByRole\("treeitem", \{ name: linked\.rowLabel, exact: true \}\)/);
assert.match(journey, /dragStoredLayoutLink\(page, owner, linked\.rowKey\)/);
console.log("[DEBUG] linked asset registration, two UTF-8 transfer boundary cases, exact frame selection, physical journey keys and embedded PNG verified");
