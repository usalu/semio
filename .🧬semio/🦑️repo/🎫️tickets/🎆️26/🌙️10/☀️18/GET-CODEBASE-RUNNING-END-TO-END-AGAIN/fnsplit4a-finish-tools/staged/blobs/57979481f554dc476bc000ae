import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { insertDevDependencyRow, withFixtureReaderDevDependency } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { rebaseManifestDirPaths } from "../../../🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { dropBaseUnresolvedNames, unresolvedSubjects } from "../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { externalizeFacetLeaves } from "../../../🏗️authoring/📦️crate-tree/🧾️facet-leaves/🟦️.ts";
import { FIXTURE_READER_CRATE, FIXTURE_READER_PACKAGE_DIR, resolveFixturePlaceholders } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { fixturesToRuntime, importFixtureMacros } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟢️shared/🟦️.ts";

describe("artifact crate carve", () => {
  test("moves test fixtures to runtime reads and resolves them against the crate directory", () => {
    const source = 'const BEFORE: &str = include_str!("../../🧪️tests/🧫️fixtures/before.json");\nfn run() { assert_eq!(BEFORE.len(), 3); let text = include_str!("../../🧪️tests/🧫️fixtures/after.json"); }\n';
    const rewritten = fixturesToRuntime(source, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain("static BEFORE: std::sync::LazyLock<&'static str>");
    expect(rewritten).toContain("(*BEFORE).len()");
    expect(rewritten).toContain('fixture_text!("@fixtures/');
    expect(rewritten).toContain("use semio_framework_test_fixtures::fixture_text;");
    expect(rewritten).not.toContain("runtime_fixture");
    expect(resolveFixturePlaceholders(rewritten, "/repo", "/repo/x/y/pkg")).toContain('fixture_text!("../../🧪️tests/🧫️fixtures/after.json")');
    expect(fixturesToRuntime("fn plain() {}", "/repo/a.rs", "/repo")).toBeNull();
  });

  test("imports the shared fixture macros once, after the file header and before the first item", () => {
    expect(importFixtureMacros("//! doc\n\nuse super::*;\n\n#[test]\nfn a() {}\n", ["fixture_text"])).toBe("//! doc\n\nuse super::*;\nuse semio_framework_test_fixtures::fixture_text;\n\n#[test]\nfn a() {}\n");
    expect(importFixtureMacros("use super::*;\nuse serde_json::Value;\nuse std::fs;\nfn a() {}\n", ["fixture_bytes"])).toBe("use super::*;\nuse semio_framework_test_fixtures::fixture_bytes;\nuse serde_json::Value;\nuse std::fs;\nfn a() {}\n");
    expect(importFixtureMacros("//! doc\n#![allow(dead_code)]\n\nfn a() {}\n", ["fixture_text", "fixture_bytes", "fixture_text"])).toBe("//! doc\n#![allow(dead_code)]\n\nuse semio_framework_test_fixtures::{fixture_bytes, fixture_text};\n\nfn a() {}\n");
    expect(importFixtureMacros("//#region x\n//! doc\n\nuse super::*;\nfn a() {}\n", ["fixture_text"])).toBe("//#region x\n//! doc\n\nuse super::*;\nuse semio_framework_test_fixtures::fixture_text;\nfn a() {}\n");
    expect(importFixtureMacros("/// outer\nfn a() {}\n", ["fixture_text"])).toBe("use semio_framework_test_fixtures::fixture_text;\n\n/// outer\nfn a() {}\n");
    const imported = importFixtureMacros("fn a() {}\n", ["fixture_path"]);
    expect(imported).toBe("use semio_framework_test_fixtures::fixture_path;\n\nfn a() {}\n");
    expect(importFixtureMacros(imported, ["fixture_path"])).toBe(imported);
    expect(importFixtureMacros("use semio_framework_test_fixtures::{fixture_bytes, fixture_text};\nfn a() {}", ["fixture_text"])).toBe("use semio_framework_test_fixtures::{fixture_bytes, fixture_text};\nfn a() {}");
    expect(importFixtureMacros("use semio_framework_test_fixtures::fixture_text;\nfn a() {}", ["fixture_text", "fixture_bytes"])).toBe("use semio_framework_test_fixtures::{fixture_bytes, fixture_text};\nfn a() {}");
    expect(importFixtureMacros("use super::*;\nuse semio_framework_test_fixtures::fixture_path;\nfn a() {}", ["fixture_text", "manifest_dir"])).toBe("use super::*;\nuse semio_framework_test_fixtures::{fixture_path, fixture_text, manifest_dir};\nfn a() {}");
  });

  test("names a fixture reader package that exists and carries the crate name the generator writes", () => {
    const repoRoot = join(import.meta.dir, "../../../../../../../..");
    const manifest = readFileSync(join(repoRoot, FIXTURE_READER_PACKAGE_DIR, "Cargo.toml"), "utf8");
    expect(manifest).toContain(`name = "${FIXTURE_READER_CRATE}"`);
  });

  test("emits the shared reader for bytes and text instead of a per-file helper", () => {
    const source = 'fn run() { let a = include_str!("../../🧪️tests/🧫️fixtures/a.json"); let b = include_bytes!("../../🧪️tests/🧫️fixtures/b.bin"); }\n';
    const rewritten = fixturesToRuntime(source, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain("use semio_framework_test_fixtures::{fixture_bytes, fixture_text};");
    expect(rewritten).toContain('fixture_text!("@fixtures/');
    expect(rewritten).toContain('fixture_bytes!("@fixtures/');
    expect(rewritten).not.toMatch(/\bfn (runtime_)?fixture_/u);
    expect(rewritten).not.toContain("CARGO_MANIFEST_DIR");
  });

  test("rebases the paths handed to the shared fixture reader and leaves fixture placeholders alone", () => {
    const source = 'let a = fixture_text!("../../x/y.json");\nlet b = fixture_bytes!("@fixtures/x/y.bin");\nlet c = fixture_path!("..");\nlet d = "fixture_text!(\\"no\\")";';
    const moved = rebaseManifestDirPaths(source, "/repo/art/📦️packages/🦀️rust", "/repo/art/sub/deeper/📦️packages/🦀️rust");
    expect(moved).toContain('fixture_text!("../../../../x/y.json")');
    expect(moved).toContain('fixture_bytes!("@fixtures/x/y.bin")');
    expect(moved).toContain('fixture_path!("../../../../📦️packages")');
    expect(moved).toContain('"fixture_text!(\\"no\\")"');
    expect(rebaseManifestDirPaths(moved, "/repo/art/sub/deeper/📦️packages/🦀️rust", "/repo/art/📦️packages/🦀️rust")).toBe(source);
  });

  test("adds the shared fixture reader as a sorted dev-dependency and never twice", () => {
    const manifest = '[package]\nname = "x"\n\n[dependencies]\na = { path = "../a" }\n\n[dev-dependencies]\nalpha = "1"\nzeta = { path = "../zeta" }\n\n[features]\ntestkit = []\n';
    const added = withFixtureReaderDevDependency(manifest, "/repo/p/q/📦️packages/🦀️rust", "/repo");
    const pathFromManifest = relative("/repo/p/q/📦️packages/🦀️rust", `/repo/${FIXTURE_READER_PACKAGE_DIR}`);
    expect(added).toContain(`alpha = "1"\n${FIXTURE_READER_CRATE} = { path = "${pathFromManifest}" }\nzeta = `);
    expect(withFixtureReaderDevDependency(added, "/repo/p/q/📦️packages/🦀️rust", "/repo")).toBe(added);
    const bare = withFixtureReaderDevDependency('[package]\nname = "x"\n\n[dependencies]\n', "/repo/p", "/repo");
    expect(bare).toMatch(/\[dev-dependencies\]\nsemio-framework-test-fixtures = \{ path = "\.\.\/🧰️framework\//u);
    expect(insertDevDependencyRow("[dev-dependencies]\nb = 1\n\n[features]\n", "c = 2")).toBe("[dev-dependencies]\nb = 1\nc = 2\n\n[features]\n");
    expect(insertDevDependencyRow("[dev-dependencies]\nb = 1\nz = 3\nc = 2\n", "c = 2")).toBe("[dev-dependencies]\nb = 1\nz = 3\nc = 2\n");
  });

  test("moves any embedded text of a test file to a runtime read, whatever its path", () => {
    const source = 'fn run() { let descriptor = include_str!("../../🧩️component/🔣️.json"); assert!(!descriptor.is_empty()); }\n';
    const rewritten = fixturesToRuntime(source, "/repo/x/y/🧪️tests/🔬️unit/🦀️.rs", "/repo")!;
    expect(rewritten).toContain('fixture_text!("@fixtures/x/y/🧩️component/🔣️.json")');
    expect(fixturesToRuntime(source, "/repo/x/y/lib.rs", "/repo")).toBeNull();
  });

  test("reads fixture bytes at run time, also from a reference constant of an inline module", () => {
    const source = 'const TILES: &[(&str, &[u8])] = &[("a", include_bytes!("../../🧪️tests/🧫️fixtures/a.pbf")), ("b", include_bytes!("../../🧪️tests/🧫️fixtures/b.pbf"))];\nfn n() -> usize { TILES.len() }\n';
    const rewritten = fixturesToRuntime(source, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain("static TILES: std::sync::LazyLock<&[(&str, &[u8])]> = std::sync::LazyLock::new(|| &*Box::leak(Box::new( [(\"a\", fixture_bytes!(\"@fixtures/");
    expect(rewritten).toContain(")));");
    expect(rewritten).toContain("(*TILES).len()");
    expect(rewritten).toContain("use semio_framework_test_fixtures::fixture_bytes;");
    expect(rewritten).not.toContain("fixture_text");
    expect(rewritten).not.toContain("fn fixture_bytes");
  });

  test("does not count a re-worded unresolved name as a layout regression but keeps a new unresolved name", () => {
    const base = ["E0425\tcannot find type `GisMapFeaturesDelta` in this scope", "E0433\tcannot find type `Other` in this scope"];
    const carved = ["E0432\tunresolved import `crate::diff::GisMapFeaturesDelta`", "E0432\tunresolved imports `crate::a::Other`, `crate::a::GisMapFeaturesDelta`", "E0432\tunresolved import `crate::diff::Fresh`", "E0603\tmodule `x` is private"];
    expect(dropBaseUnresolvedNames(base, carved)).toEqual(["E0432\tunresolved import `crate::diff::Fresh`", "E0603\tmodule `x` is private"]);
    expect(unresolvedSubjects("E0432\tunresolved import `crate::diff::GisMapFeaturesDelta`: no `X` in `diff`")).toEqual(["GisMapFeaturesDelta"]);
  });

  test("turns a fixture constant of an inline module into a lazily read static that reaches the helper through super", () => {
    const nested = 'mod inner {\n    const CONTRACT: &str = include_str!("../../🧪️tests/🧫️fixtures/c.json");\n    fn f() -> usize { CONTRACT.len() }\n}\n';
    const rewritten = fixturesToRuntime(nested, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain("static CONTRACT: std::sync::LazyLock<&str> = std::sync::LazyLock::new(|| super::fixture_text!(\"@fixtures/");
    expect(rewritten).toContain("use semio_framework_test_fixtures::fixture_text;");
    expect(rewritten).toContain("(*CONTRACT).len()");
    const array = 'mod inner {\n    const ROWS: [&str; 2] = [include_str!("../../🧪️tests/🧫️fixtures/e.json"), include_str!("../../🧪️tests/🧫️fixtures/f.json")];\n    fn n() -> usize { ROWS.len() }\n}\n';
    const arrayRewrite = fixturesToRuntime(array, "/repo/x/y/z/test.rs", "/repo")!;
    expect(arrayRewrite).toContain("static ROWS: std::sync::LazyLock<[&str; 2]> = std::sync::LazyLock::new(|| [super::fixture_text!(");
    expect(arrayRewrite).toContain("]);");
    expect(arrayRewrite).toContain("(*ROWS).len()");
  });

  test("serves typescript, graphql and proto facet companions from their documents", () => {
    const source = [
      "pub fn descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {",
      "    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {",
      '        id: "s.demo.demo",',
      '        artifact: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("a.rs"), typescript: include_str!("🟦️.ts"), graphql: "", json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },',
      '        snapshot: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      '        diff: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      '        mutations: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      "    }",
      "}",
      "",
    ].join("\n");
    const result = externalizeFacetLeaves(source, "/repo/x/🦀️.rs", "/repo")!;
    expect(result.leaves.map((leaf) => `${leaf.export}/${leaf.format}/${leaf.path}`)).toEqual(["artifact/typescript/x/🟦️.ts", "artifact/protobuf/x/🛰️.proto"]);
    expect(result.text).toContain('typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: ""');
    expect(result.text).toContain("register_schema_leaf_documents(&[");
    expect(result.text).toContain('rust: include_str!("a.rs")');
    expect(externalizeFacetLeaves("fn plain() {}", "/repo/a.rs", "/repo")).toBeNull();
  });

  test("serves the JSON Schema leaves too when the plugin guest receives them as package assets", () => {
    const source = [
      "pub fn descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {",
      "    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {",
      '        id: "s.demo.demo",',
      '        artifact: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("a.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },',
      '        snapshot: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      '        diff: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      '        mutations: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      "    }",
      "}",
      "",
    ].join("\n");
    expect(externalizeFacetLeaves(source, "/repo/x/🦀️.rs", "/repo")).toBeNull();
    const result = externalizeFacetLeaves(source, "/repo/x/🦀️.rs", "/repo", { json: true })!;
    expect(result.leaves.map((leaf) => `${leaf.export}/${leaf.format}/${leaf.path}`)).toEqual(["artifact/jsonschema/x/🔣️.json"]);
    expect(result.text).toContain('json_schema: "", proto: ""');
    expect(result.text).toContain("SchemaFormat::JsonSchema");
    expect(result.text).toContain('rust: include_str!("a.rs")');
    const withRust = externalizeFacetLeaves(source, "/repo/x/🦀️.rs", "/repo", { json: true, rust: true })!;
    expect(withRust.leaves.map((leaf) => `${leaf.export}/${leaf.format}/${leaf.path}`)).toEqual(["artifact/rust/x/a.rs", "artifact/jsonschema/x/🔣️.json"]);
    expect(withRust.text).toContain("SchemaFormat::Rust");
    expect(withRust.text).not.toContain('include_str!("a.rs")');
  });

  test("serves a concat of embedded companions as several ordered documents under an id constant", () => {
    const source = [
      "pub fn descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {",
      "    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {",
      "        id: crate::snapshot::DEMO_SCHEMA_ID,",
      '        artifact: ::semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("a.rs"), include_str!("b.rs")), typescript: "", graphql: "", json_schema: "", proto: "" },',
      '        snapshot: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      '        diff: ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },',
      "        mutations: ::semio_framework_schema_registry::FacetLeaves {",
      '            rust: include_str!("m.rs"),',
      "            typescript: concat!(",
      '                include_str!("../m/🟦️.ts"),',
      '                include_str!("../m/g/🟦️.ts"),',
      "            ),",
      '            graphql: include_str!("m.graphql"),',
      '            json_schema: include_str!("m.json"),',
      '            proto: "",',
      "        },",
      "    }",
      "}",
      "",
    ].join("\n");
    const result = externalizeFacetLeaves(source, "/repo/x/y/🦀️.rs", "/repo")!;
    expect(result.leaves.map((leaf) => `${leaf.export}/${leaf.format}/${leaf.path}`)).toEqual(["mutations/typescript/x/m/🟦️.ts", "mutations/typescript/x/m/g/🟦️.ts", "mutations/graphql/x/y/m.graphql"]);
    expect(result.text).toContain("scope: crate::snapshot::DEMO_SCHEMA_ID, export: \"mutations\", format: ::semio_framework_schema_registry::SchemaFormat::Typescript");
    expect(result.text).toContain('typescript: "",\n            graphql: "",\n            json_schema: include_str!("m.json")');
    expect(result.text).toContain('rust: concat!(include_str!("a.rs"), include_str!("b.rs"))');
  });

  test("serves the facet companions of an inference descriptor under the inference export", () => {
    const source = [
      "pub fn descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {",
      "    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {",
      '        id: "s.demo.demo.inference",',
      '        inference: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("a.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },',
      "    }",
      "}",
      "",
    ].join("\n");
    const result = externalizeFacetLeaves(source, "/repo/x/🦀️.rs", "/repo")!;
    expect(result.leaves.map((leaf) => `${leaf.scope}/${leaf.export}/${leaf.format}/${leaf.path}`)).toEqual([
      "s.demo.demo.inference/inference/typescript/x/🟦️.ts",
      "s.demo.demo.inference/inference/graphql/x/🔗️.graphql",
      "s.demo.demo.inference/inference/protobuf/x/🛰️.proto",
    ]);
    expect(result.text).toContain('rust: include_str!("a.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: ""');
    expect(result.text.indexOf("register_schema_leaf_documents")).toBeLessThan(result.text.lastIndexOf("ArtifactInferenceDescriptor {\n"));
  });

  test("serves the facet companions of an app descriptor under the config and presence exports", () => {
    const source = [
      "pub fn app_schema_descriptor() -> ::semio_framework_schema_registry::AppSchemaDescriptor {",
      "    ::semio_framework_schema_registry::AppSchemaDescriptor {",
      '        id: "s.demo.demo",',
      '        config: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("a.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },',
      '        presence: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("p/a.rs"), typescript: include_str!("p/🟦️.ts"), graphql: include_str!("p/🔗️.graphql"), json_schema: include_str!("p/🔣️.json"), proto: include_str!("p/🛰️.proto") },',
      "    }",
      "}",
      "",
    ].join("\n");
    const result = externalizeFacetLeaves(source, "/repo/x/🦀️.rs", "/repo")!;
    expect(result.leaves.map((leaf) => `${leaf.scope}/${leaf.export}/${leaf.format}/${leaf.path}`)).toEqual([
      "s.demo.demo/config/typescript/x/🟦️.ts",
      "s.demo.demo/config/graphql/x/🔗️.graphql",
      "s.demo.demo/config/protobuf/x/🛰️.proto",
      "s.demo.demo/presence/typescript/x/p/🟦️.ts",
      "s.demo.demo/presence/graphql/x/p/🔗️.graphql",
      "s.demo.demo/presence/protobuf/x/p/🛰️.proto",
    ]);
    expect(result.text).toContain('presence: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("p/a.rs"), typescript: "", graphql: "", json_schema: include_str!("p/🔣️.json"), proto: "" }');
  });
});
