import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const load = (path: string) => import(pathToFileURL(join(root, path)).href);
const { decodePackValue, encodePackValue } = await load("🧰️framework/🔨️modules/🎁️packs/🟦️.ts");
const { finalizePluginDescriptor } = await load("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts");
const { validateCatalogDescriptorPair } = await load("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/📁️filesystem/🟦️.ts");
const directory = (id: string) => join(root, "🌎️hub/🧩️compositions", id, "🧩️component");
const packFile = "🛂️.descriptor.semio";
const jsonFile = "🔣️.json";
const validatePair = (folder: string, pluginId: string) => validateCatalogDescriptorPair({
  cratePath: join(folder, "📦️packages", "🦀️rust"),
  pluginId, packageId: `semio:${pluginId}`, role: "plugin",
}, root);

const trinity = directory("🔱️trinity");
const trinityPack = readFileSync(join(trinity, packFile));
const trinityJson = readFileSync(join(trinity, jsonFile), "utf8");
assert.deepEqual(JSON.parse(finalizePluginDescriptor(trinityPack, "trinity").json), JSON.parse(trinityJson));
const producer = JSON.parse(trinityJson).manifest.apps.find((app: { id: string }) => app.id === "s.trinity.rewriting@1/*#editor");
const { label: producerLabel, mediaType, kindId, direction, multiplicity } = producer.io.ports.find((port: { id: string }) => port.id === "rule:out");
assert.deepEqual(producerLabel.native, { en: "Rewrite rule", de: "Umschreibregel" });
assert.deepEqual(mediaType, { class: "computation", form: "value" });
assert.equal(kindId, "trinity.rewrite.invocation");
assert.equal(direction, "out");
assert.equal(multiplicity, "many");

const puzzle = directory("🧩️puzzle");
const beforePack = readFileSync(join(puzzle, packFile));
const beforeJson = readFileSync(join(puzzle, jsonFile), "utf8");
assert.deepEqual(JSON.parse(finalizePluginDescriptor(beforePack, "puzzle").json), JSON.parse(beforeJson));
const descriptor = decodePackValue(beforePack);
const app = descriptor.manifest.apps.find((row: { id: string }) => row.id === "s.puzzle.puzzle5d@1/*#editor");
const label = (en: string, de: string) => ({ native: { en, de }, reuse: { en, de } });
const kitDesign = { class: "kit", form: "design" };
app.io = {
  artifactSchema: "puzzle.5d",
  artifactMediaType: kitDesign,
  ports: [
    { id: "kit:in", label: label("Kit Catalog", "Kit-Katalog"), direction: "in", mediaType: { class: "kit", form: "type" }, kindId: "kit.catalog", required: false, multiplicity: "many" },
    { id: "rewrite:in", label: label("Rewrite Rule", "Umschreibregel"), direction: "in", mediaType, kindId, required: false, multiplicity: "one" },
    { id: "design:out", label: label("5D Puzzle Design", "5D-Puzzle-Entwurf"), direction: "out", mediaType: kitDesign, kindId: "5d.puzzle", required: false, multiplicity: "many" },
  ],
  exportFormats: [],
  importFormats: [],
  artifact: { id: "5d.puzzle", name: "5D Puzzle", dimension: "5d", componentKind: "puzzle5d" },
};
const result = finalizePluginDescriptor(encodePackValue(descriptor), "puzzle");
const updated = JSON.parse(result.json);
const updatedApp = updated.manifest.apps.find((row: { id: string }) => row.id === app.id);
assert.deepEqual(updatedApp.io, app.io);
const old = JSON.parse(beforeJson);
const oldApp = old.manifest.apps.find((row: { id: string }) => row.id === app.id);
updatedApp.io = oldApp.io;
assert.deepEqual(updated, old);
oldApp.io = app.io;
assert.deepEqual(JSON.parse(result.json), old);

if (process.argv.includes("--write")) {
  assert.deepEqual(readFileSync(join(puzzle, packFile)), beforePack);
  assert.equal(readFileSync(join(puzzle, jsonFile), "utf8"), beforeJson);
  writeFileSync(join(puzzle, packFile), result.pack);
  writeFileSync(join(puzzle, jsonFile), JSON.stringify(old, null, 2) + "\n");
}
validatePair(trinity, "trinity");
validatePair(puzzle, "puzzle");
console.log("[DEBUG] Verified canonical Trinity and Puzzle descriptor pairs and the exact Puzzle 5D editor I/O projection.");
