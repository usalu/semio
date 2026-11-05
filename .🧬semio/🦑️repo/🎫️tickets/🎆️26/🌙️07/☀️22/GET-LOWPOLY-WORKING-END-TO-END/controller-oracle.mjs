import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import {pathToFileURL} from "node:url";

const base = "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/";
const fixture = JSON.parse(fs.readFileSync(base + "📌️panels/🛍️catalogue/🧪️tests/🧫️fixtures/controller.json", "utf8"));
const descriptor = JSON.parse(fs.readFileSync("🌎️hub/🧩️compositions/💠️lowpoly/🧩️component/🔣️.json", "utf8"));
const {surfaceAppId, parseSurfaceAppId} = await import(pathToFileURL(path.resolve("🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🟦️.ts")).href);
const definitions = [];
function visit(value) {
    if (value === null || typeof value !== "object") return;
    if (value.id === fixture.controller && typeof value.controllerId === "string") definitions.push(value);
    for (const child of Object.values(value)) visit(child);
}
visit(descriptor);
assert.equal(definitions.length, 1);
const editor = definitions[0];
assert.equal(editor.controllerId, fixture.controller);
assert.equal(surfaceAppId(editor.dialect, editor.role), fixture.controller);
assert.deepEqual(parseSurfaceAppId(fixture.controller), {dialect: editor.dialect, role: editor.role});
console.log("[DEBUG] Owning TypeScript formatter independently validates the canonical editor identity fixture and descriptor.");
