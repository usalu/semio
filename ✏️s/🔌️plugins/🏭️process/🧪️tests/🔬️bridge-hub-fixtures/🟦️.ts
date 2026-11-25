import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../..");
const plugin = resolve(root, "✏️s/🔌️plugins/🏭️process");
const hub = resolve(root, "🌎️hub/🧩️compositions/🏭️process");
const genesis = resolve(plugin, "🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🌱️genesis");
const read = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));

const pairs = [
  { name: "bridge inventory", fixture: resolve(plugin, "🏭️bridge/🧪️tests/🧫️fixtures/inventory.json"), schema: resolve(plugin, "🏭️bridge/🧪️tests/🧫️fixtures/inventory.schema.json") },
  { name: "hub registration", fixture: resolve(hub, "🧪️tests/🧫️fixtures/registration.json"), schema: resolve(hub, "🧪️tests/🧫️fixtures/registration.schema.json") },
  { name: "process document genesis", fixture: resolve(genesis, "🧪️tests/🔣️.json"), schema: resolve(genesis, "🧪️tests/📐️.json") },
];

for (const pair of pairs) {
  test(`${pair.name} fixture validates against its schema (ajv oracle)`, () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(read(pair.schema) as object);
    expect(validate(read(pair.fixture)), JSON.stringify(validate.errors)).toBe(true);
  });
}

test("the hub registration fixture agrees with the editor and viewer app ids the hub registers", () => {
  const fixture = read(pairs[1]!.fixture) as { apps: { id: string; role: string; originalFactory: boolean }[] };
  const source = readFileSync(resolve(hub, "🦀️.rs"), "utf8");
  expect(source).toContain("Process3dEditor(");
  expect(source).toContain("Process3dViewer(");
  expect(source).toContain(".native_editor::<");
  expect(source).toContain(".native_viewer::<");
  expect(source).not.toMatch(/\.(editor|viewer)::</);
  expect(source.match(/OriginalVcsApp</g)).toHaveLength(2);
  expect(fixture.apps.map((app) => app.role)).toEqual(["editor", "viewer"]);
  for (const app of fixture.apps) expect(app.originalFactory).toBe(true);
  for (const app of fixture.apps) expect(app.id).toMatch(/^s\.process\.process3d@1\/\*#(editor|viewer)$/);
});

test("the genesis fixture agrees with the authored genesis source", () => {
  const fixture = read(pairs[2]!.fixture) as { schema: string; maximumInitialPackBytes: number; artifactKind: string };
  const source = readFileSync(resolve(genesis, "🦀️.rs"), "utf8");
  expect(source).toContain(`MAXIMUM_INITIAL_PACK_BYTES: usize = ${fixture.maximumInitialPackBytes.toLocaleString("en-US").replaceAll(",", "_")};`);
  expect(source).toContain("PROCESS_3D_SCHEMA");
  expect(source).toContain("PROCESS3D_DIALECT");
  const model = readFileSync(resolve(plugin, "🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"), "utf8");
  expect(model).toContain(`PROCESS_3D_SCHEMA: &str = "${fixture.schema}"`);
  expect(model).toContain(`artifact_kind: "${fixture.artifactKind}"`);
});
