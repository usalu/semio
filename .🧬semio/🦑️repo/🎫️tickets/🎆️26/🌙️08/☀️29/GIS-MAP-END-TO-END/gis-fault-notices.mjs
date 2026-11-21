import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import vm from "node:vm";
import Ajv from "ajv";

const editor = path.resolve("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
const declarations = fs.readFileSync(path.join(editor, "🦀️.rs"), "utf8")
  .match(/fn fault_notices\(\)[\s\S]+?NOTICES\.as_slice\(\)/u)?.[0] ?? "";
const branches = ["import_elevation", "geojson", "contours", "configure_terrain_brush"];
for (const branch of branches) assert.ok(declarations.includes(`${branch}::fault_notices()`), branch);

const commands = path.join(editor, "🎮️commands");
const literals = ["📥️import-elevation", "🌍️geojson", "📉️contours"].flatMap((directory) =>
  [...fs.readFileSync(path.join(commands, directory, "🦀️.rs"), "utf8")
    .matchAll(/Self::[^\n]+ => \("(gis[^"]+)"/gu)].map((match) => match[1]),
);
const view = fs.readFileSync(path.join(commands, "👁️view", "🦀️.rs"), "utf8");
const brush = [...view.matchAll(/const (INVALID|CONTEXT): &str = "([^"]+)";/gu)].map((match) => match[2]);
assert.equal(brush.length, 2);
assert.equal(literals.length, 18);
const codes = [...literals, ...brush];

const validatorPath = "🧰️framework/🔨️modules/🛂️manifest/🔁️host-effects-choices-and-fault-notices/🟦️.ts";
const validator = fs.readFileSync(validatorPath, "utf8");
const definition = validator.match(/^const FAULT_NOTICE_CODE = .+;$/mu)?.[0];
assert.ok(definition);
const manifestPattern = vm.runInNewContext(`${definition}\nFAULT_NOTICE_CODE`);
const oracle = new Ajv().compile({
  type: "string",
  pattern: "^(?:[a-z][a-z0-9]*(?:-[a-z0-9]+)*\\.){2,}[a-z][a-z0-9]*(?:-[a-z0-9]+)*$",
});
const rejected = codes.filter((code) => !manifestPattern.test(code));
assert.deepEqual(rejected, codes.filter((code) => !oracle(code)), "The manifest and JSON Schema results must agree");
assert.deepEqual(rejected, [], "The TypeScript host must accept every published terrain notice code");
assert.equal(new Set(codes).size, codes.length);
assert.deepEqual(brush, ["gis.terrain.brush.settings-invalid", "gis.terrain.window.context-required"]);
assert.match(view, /FaultCode::new\(INVALID\)/u);
assert.match(view, /FaultCode::new\(CONTEXT\)/u);

const config = fs.readFileSync(path.join(editor, "🎭️modes", "👁️view", "🪟️windows", "🏔️terrain", "🎚️config", "🦀️.rs"), "utf8");
assert.match(config, /use crate::editor::gis3d::commands::view::configure_terrain_brush::missing_window;/u);
assert.match(config, /window_id\.as_deref\(\)\.ok_or_else\(missing_window\)/u);
console.log(`Validated ${codes.length} unique terrain fault notice codes with the TypeScript manifest and Ajv`);
