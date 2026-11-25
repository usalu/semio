import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "./en1996-mutation-package-fixture.json";

const root = resolve(import.meta.dirname, "../../../../../../..");
const family = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996";
const subset = "🏅️standards/🔖️1/🪆️subsets/✳️any";
const schema = join(subset, "🧬️schema");
const mutation = join(schema, "🧬️mutations");
const cargo = "📦️packages/🦀️rust/Cargo.toml";
const read = (...path: string[]) => readFileSync(join(root, ...path), "utf8");
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const name = "semio-s-artifact-norm-en1996";
const twins = ["graphql", "json-schema", "proto", "typescript"].map(kind => `${name}-mutations-${kind}`);

test("the carve preserves all mutation ordinals, kinds, edit rules and wire twins", () => {
  const source = read(family, mutation, "🦀️.rs");
  const enumBlock = source.match(/pub enum En1996Mutation \{[\s\S]*?\n\}/u)![0];
  expect(enumBlock.split("\n").length - 2).toBe(fixture.mutationCount);
  expect(hash(enumBlock)).toBe(fixture.enum);
  expect(hash(source.match(/pub const KINDS:[\s\S]*?\n\];/u)![0])).toBe(fixture.kinds);
  const rules = read(family, mutation, "🧭️edit-rules", "🦀️.rs");
  expect(hash(rules.slice(rules.indexOf("pub const EDIT_RULES")))).toBe(fixture.editRules);
  for (const [file, digest] of Object.entries(fixture.twins)) expect(hash(read(family, mutation, file)), file).toBe(digest);
});

test("the editor owns edit resolution and the schema owns wire leaves in every locked workspace", () => {
  type Package = { package: { name: string; workspace?: string }; lib?: { path: string }; dependencies: Record<string, unknown> };
  const manifest = (...path: string[]) => Bun.TOML.parse(read(family, ...path, cargo)) as Package;
  const editPath = join(mutation, "🧭️edit-rules"), edit = manifest(editPath);
  expect(edit.package.name).toBe(`${name}-mutations-edit-rules`);
  expect(edit.lib?.path).toBe("../../🦀️.rs");
  expect(resolve(root, family, editPath, "📦️packages/🦀️rust", edit.package.workspace!)).toBe(resolve(root, family));
  const workspace = Bun.TOML.parse(read(family, "Cargo.toml")) as { workspace: { members: string[]; dependencies: Record<string, { path: string }> } };
  expect(workspace.workspace.members).toContain(join(editPath, "📦️packages/🦀️rust").replaceAll("\\", "/"));
  expect(workspace.workspace.dependencies[edit.package.name]?.path).toBe(join(editPath, "📦️packages/🦀️rust").replaceAll("\\", "/"));
  expect(Object.keys(manifest(subset, "✏️editor").dependencies)).toContain(edit.package.name);
  for (const excluded of [...twins, "semio-s-plugin-norm-app-surface", edit.package.name]) expect(manifest(mutation).dependencies).not.toHaveProperty(excluded);
  expect(Object.keys(manifest(schema).dependencies)).toEqual(expect.arrayContaining(twins));
  for (const path of [join(family, "Cargo.lock"), "🌎️hub/Cargo.lock", "✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock", "✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock"]) {
    const lock = Bun.TOML.parse(read(path)) as { package: { name: string; dependencies?: string[] }[] };
    const byName = new Map(lock.package.map(row => [row.name, row.dependencies ?? []]));
    for (const twin of twins) expect(byName.get(`${name}-mutations`), path).not.toContain(twin);
    if (byName.has(`${name}-schema`)) expect(byName.get(`${name}-schema`), path).toEqual(expect.arrayContaining(twins));
    if (byName.has(`${name}-editor`)) {
      expect(byName.get(`${name}-editor`), path).toContain(edit.package.name);
      expect(byName.get(edit.package.name), path).toEqual(Object.keys(edit.dependencies).sort());
    }
  }
});
