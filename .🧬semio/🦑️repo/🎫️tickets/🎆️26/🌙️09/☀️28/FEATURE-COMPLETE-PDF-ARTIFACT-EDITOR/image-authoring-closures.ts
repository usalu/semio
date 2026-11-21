/** 📏 Checks staged source ownership and Cargo edges through the repository reader. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {join, relative, resolve} from "node:path";
const {measureRustCrate} = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));

const {files, cfg} = await Bun.stdin.json() as {files: [string, string][], cfg: string[]};
const root = process.cwd();
const overlay = new Map(files.map(([file, value]) => [resolve(root, file), value]));
const pdf = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets";
const base = join(pdf, "🧱️base");
const vt = join(pdf, "🧾️vt");
const packages = "📦️packages/🦀️rust";
const owners = {
  document: join(base, "✏️editor/📄️document", packages),
  page: join(base, "✏️editor/🖼️page", packages),
  controller: join(vt, "🧬️schema", packages),
  bridge: join(vt, "🧬️schema/🧬️mutations/🧪️tests/🔬️unit", packages),
};

function manifest(dir: string): {package: {name: string}; dependencies: Record<string, {path?: string; workspace?: boolean}>} {
  const file = resolve(dir, "Cargo.toml");
  return Bun.TOML.parse(overlay.get(file) ?? readFileSync(file, "utf8")) as ReturnType<typeof manifest>;
}

function check(dir: string, provider: string, target?: string) {
  const doc = manifest(dir);
  const edge = doc.dependencies[provider];
  assert(edge, doc.package.name + " lacks the normal Cargo edge " + provider);
  if (target) {
    assert(edge.path, provider + " lacks a concrete path from " + doc.package.name);
    assert.equal(resolve(dir, edge.path), resolve(target), "Unexpected Cargo dependency target");
  }
  return doc.package.name + " -> " + provider;
}

const edges = [
  check(owners.document, "semio-plugin"),
  check(owners.document, "semio-framework-pixels", join("🧰️framework/🔨️modules/🔲️pixels", packages)),
  check(owners.document, "semio-s-artifact-stdio-contract"),
  check(owners.document, "semio-s-artifact-stdio-pdf-mut-family-8"),
  check(owners.document, "semio-s-artifact-stdio-pdf-snapshot-2"),
  check(owners.page, "semio-framework-pixels"),
  check(owners.page, "semio-s-artifact-stdio-pdf-editor-document", owners.document),
  check(owners.controller, "semio-plugin"),
  check(owners.controller, "semio-s-artifact-stdio-pdf-editor-document", owners.document),
  check(owners.controller, "semio-framework-value"),
  check(owners.controller, "semio-framework-value-derive"),
  check(owners.controller, "semio-framework-dsl-record-derive"),
  check(owners.controller, "semio-s-artifact-stdio-contract"),
  check(owners.controller, "semio-os-kernel"),
  check(owners.controller, "semio-ui-locale"),
  check(owners.controller, "semio-s-artifact-stdio-pdf-tests-2", owners.bridge),
  check(owners.bridge, "semio-s-artifact-stdio-pdf-page-editor", owners.page),
];
for (const edge of edges) console.log("[DEBUG] " + edge);
type LockedPackage = {name: string; version: string; source?: string; dependencies?: string[]};
const rowEdges = [
  [owners.document, "semio-s-artifact-stdio-pdf-editor-document", ["semio-framework-pixels", "semio-s-artifact-stdio-contract"]],
  [owners.controller, "semio-s-artifact-stdio-pdf-schema-5", ["semio-s-artifact-stdio-pdf-editor-document"]],
] as const;
const locks = [...overlay].filter(([file]) => file.endsWith("Cargo.lock"));
assert(locks.some(([file]) => file === resolve(pdf, "../../../Cargo.lock")), "The PDF workspace lock is absent");
const parseLock = (text: string) => (Bun.TOML.parse(text) as {package: LockedPackage[]}).package;
for (const [file, value] of locks) {
  const before = parseLock(readFileSync(file, "utf8"));
  const updated = parseLock(value);
  assert.equal(updated.length, before.length, file);
  for (const [dir, owner, dependencies] of rowEdges) {
    const prior = before.find(row => row.name === owner);
    if (!prior) continue;
    const next = updated.find(row => row.name === owner);
    assert(next?.dependencies && prior.dependencies, "Absent owner dependencies in " + file);
    const expected = manifest(dir).dependencies;
    const missing: readonly string[] = dependencies.filter(dependency => !prior.dependencies?.includes(dependency));
    for (const dependency of dependencies) {
      assert(expected[dependency], "Lock edge lacks a normal manifest declaration: " + owner + " -> " + dependency);
      assert(next.dependencies.includes(dependency), "Missing lock edge " + owner + " -> " + dependency);
    }
    assert.deepEqual(next.dependencies.filter(dependency => !missing.includes(dependency)), prior.dependencies, file);
    next.dependencies = prior.dependencies;
  }
  assert.deepEqual(updated, before, "Unexpected package edits in " + file);
  console.log("[DEBUG] Validated PDF dependency rows in " + relative(root, file));
}
const rootLock = parseLock(readFileSync(resolve(root, "Cargo.lock"), "utf8"));
assert(!rootLock.some(row => rowEdges.some(([, owner]) => row.name === owner)), "The root lock gained a PDF editor owner");
console.log("[DEBUG] Root Cargo.lock contains neither changed PDF package");

const absoluteMirrors = {
  document: [join(base, "✏️editor/📄️document/🖼️image-authoring/🦀️.rs"), join(base, "✏️editor/📄️document/🖼️image-authoring/🎛️replace-button.rs"), join(base, "✏️editor/📄️document/🖼️image-authoring/🧵️decode.rs"), join(base, "✏️editor/📄️document/🖼️image-authoring/📥️admission.rs")],
  controller: [join(base, "✏️editor/📥️local-image/🦀️.rs")],
  page: [join(base, "✏️editor/🖼️page/🪟️windows/🦀️.rs")],
};
for (const name of ["document", "page", "controller"] as const) {
  const directory = resolve(root, owners[name]);
  for (const withTest of [false, true]) {
    const measured = await measureRustCrate(root, join(directory, "🦀️.rs"), withTest, overlay, directory, cfg);
    for (const path of absoluteMirrors[name]) assert(measured.paths.includes(path.replaceAll("\\", "/")), "Source reader missed " + path);
    console.log("[DEBUG] " + name + (withTest ? " test" : " library") + " closure " + measured.files + " files, " + measured.lines + " active lines, " + measured.bytes + " bytes");
    assert(measured.files <= 32 && measured.lines <= 4_000, name + " exceeds the current file/line boundary");
    const base = await measureRustCrate(root, join(directory, "🦀️.rs"), withTest, new Map(), directory, cfg);
    console.log("[DEBUG] baseline " + name + (withTest ? " test" : " library") + " closure " + base.files + " files, " + base.lines + " active lines");
  }
}
