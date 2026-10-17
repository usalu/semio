import { describe, expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { applyCarveWrites, carveWrites, planArtifactCrates } from "../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { demoteDanglingOuterDocs, parseRustSourceTree, prunedUse, referencedItemNames, rustOuterDocStart } from "../../🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { MeasureUnion, totalsFit } from "../../🏗️authoring/📦️crate-tree/📏️measure-union/🟦️.ts";
import { aliasChains, allItems, crateRelativeChains, crateIdent as crateIdentOf, implHeader, qualifyModuleHeads, rootAliasModules, superChains } from "../../🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
import { addWorkspaceMembers, hidingExcludePattern, insertBuildOwnerRows, patchCargoLock, patchRouterPackages, rebaseManifestDirPaths, rebaseRustIncludes, workspaceAdmissionProblems } from "../../🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { external, hoistInnerAttributes, mountedPathFiles, rewriteCrateAliases, rewriteForCombined, siblingPathCrates, verifyAppliedLayout } from "../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { inlineRootFileModules } from "../../🏗️authoring/📦️crate-tree/🫗️inline-root/🟦️.ts";
import { absolutizeBeyond } from "../../🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts";
import { inlineMountTreeFiles } from "../../🏗️authoring/📦️crate-tree/🌲️inline-mounts/🟦️.ts";
import { externalizeFacetLeaves } from "../../🏗️authoring/📦️crate-tree/🧾️facet-leaves/🟦️.ts";
import { absolutizeSuperPaths, fixturesToRuntime, resolveFixturePlaceholders, transformTestClosure } from "../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { checkOrphanRules, compareModuleTrees, dropUnlinkedCascades, plainModulePath, resolveCrateModules, sharedMountFiles, unlinkedGlobModules } from "../../🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";

const files: Record<string, string> = {
  "Cargo.toml": '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n',
  "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-artifact"\nversion = "0.1.0"\n',
  "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-artifact"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
  "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-artifact",{snapshotSqliteTests:[]});\n',
  "🦀️.rs": [
    "//! 🧪️ Demo artifact root.",
    "pub const DOCUMENT: &str = \"demo\";",
    "#[path = \".\"]",
    "pub mod schema {",
    "    #[path = \"🧬️schema/🦀️.rs\"]",
    "    mod component;",
    "    pub use component::*;",
    "    #[path = \"🧬️schema/📸️snapshot/🦀️.rs\"]",
    "    pub mod snapshot;",
    "    #[path = \"🧬️schema/🔺️diff/🦀️.rs\"]",
    "    pub mod diff;",
    "}",
    "#[path = \".\"]",
    "pub mod io {",
    "    #[path = \"🚪️io/🦀️.rs\"]",
    "    mod component;",
    "    pub use component::*;",
    "}",
    "",
  ].join("\n"),
  "🧬️schema/🦀️.rs": "//! 🧬️ schema component\npub fn describe() -> &'static str { crate::DOCUMENT }\n",
  "🧬️schema/📸️snapshot/🦀️.rs": "//! 📸️ snapshot\n#[derive(Clone, Debug, Default, PartialEq)]\npub struct Snap { pub id: String }\n",
  "🧬️schema/🔺️diff/🦀️.rs": "//! 🔺️ diff\nuse crate::schema::snapshot::Snap;\npub struct Diff { pub before: Snap }\n",
  "🚪️io/🦀️.rs": "//! 🚪️ io\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
  "🚪️io/📝️text/🦀️.rs": "//! 📝️ text codec\nuse crate::schema::snapshot::Snap;\nimpl std::fmt::Display for Snap {\n    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, \"{}\", self.id) }\n}\n",
};

function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "semio-crate-tree-"));
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

describe("artifact crate carve", () => {
  test("counts only identifiers that can name a module-level item as references", () => {
    const names = referencedItemNames("fn f(artifact: &A, count: u8) -> S { let v = artifact.definition.steps(); S { definition: v, artifact: count, extra } }");
    expect([...names].sort()).toEqual(["A", "S", "artifact", "count", "extra", "f", "fn", "let", "u8", "v"]);
    expect([...referencedItemNames("let x = pack::snapshot::Pack::new(crate::forms_steps(&self::helper), super::up());")].sort()).toEqual(["crate", "forms_steps", "helper", "let", "pack", "self", "super", "up", "x"]);
  });

  test("parses the module skeleton with exact offsets", () => {
    const tree = parseRustSourceTree(files["🦀️.rs"]!);
    expect(tree.modules.map((node) => node.name)).toEqual(["schema", "io"]);
    const schema = tree.modules[0]!;
    expect(schema.children.map((node) => `${node.name}:${node.inline}:${node.pathAttribute}`)).toEqual(["component:false:🧬️schema/🦀️.rs", "snapshot:false:🧬️schema/📸️snapshot/🦀️.rs", "diff:false:🧬️schema/🔺️diff/🦀️.rs"]);
    expect(files["🦀️.rs"]!.slice(schema.start, schema.end).startsWith("#[path = \".\"]")).toBe(true);
    expect(tree.items.map((item) => item.name)).toContain("DOCUMENT");
  });

  test("keeps impls of foreign traits with the type they implement and cuts along declarations", async () => {
    const root = fixture();
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { target: { files: 3, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::").endsWith(suffix)));
      expect(holder("schema::snapshot")).toBeDefined();
      expect(holder("io::text")).toBe(holder("schema::snapshot"));
      expect(holder("schema::diff")).not.toBe(holder("schema::snapshot"));
      const { writes } = await carveWrites(plan);
      const paths = writes.map((write) => write.path);
      expect(paths.some((path) => path.endsWith("📦️packages/🦀️rust/🦀️.rs"))).toBe(true);
      const diffCrate = holder("schema::diff")!;
      const mirror = writes.find((write) => write.path === `${diffCrate.packageDir}/🦀️.rs`)!.content;
      expect(mirror).toMatch(/pub use demo_artifact_[a-z0-9_]+::schema::snapshot;/u);
      expect(writes.find((write) => write.path.endsWith("Cargo.lock"))!.content).toContain(`name = "${holder("schema::snapshot")!.name}"`);
      expect(writes.find((write) => write.path === "Cargo.toml")!.content).toContain("members = [");
      const original = resolveCrateModules(root, join(root, "🦀️.rs"));
      applyCarveWrites(root, writes);
      const carved = [resolveCrateModules(root, join(root, "🦀️.rs")), ...plan.crates.map((crate) => resolveCrateModules(root, join(root, crate.packageDir, "🦀️.rs")))];
      expect(compareModuleTrees(original, carved).problems).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("detects orphan-rule violations that a wrong cut would create", () => {
    const sources = (text: string) => new Map([["lib.rs", text]]);
    const violations = checkOrphanRules(
      [{ name: "types", sources: sources("pub struct Snap;") }, { name: "codec", sources: sources("impl std::fmt::Display for Snap { }") }],
      implHeader,
      allItems,
    );
    expect(violations).toEqual([{ crate: "codec", file: "lib.rs", trait: "Display", selfType: "Snap", definedIn: ["types"] }]);
    expect(checkOrphanRules([{ name: "types", sources: sources("pub struct Snap;\nimpl std::fmt::Display for Snap { }") }], implHeader, allItems)).toEqual([]);
  });

  test("moves test fixtures to runtime reads and resolves them against the crate directory", () => {
    const source = 'const BEFORE: &str = include_str!("../../🧪️tests/🧫️fixtures/before.json");\nfn run() { assert_eq!(BEFORE.len(), 3); let text = include_str!("../../🧪️tests/🧫️fixtures/after.json"); }\n';
    const rewritten = fixturesToRuntime(source, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain("static BEFORE: std::sync::LazyLock<&'static str>");
    expect(rewritten).toContain("(*BEFORE).len()");
    expect(rewritten).toContain('runtime_fixture_text("@fixtures/');
    expect(resolveFixturePlaceholders(rewritten, "/repo", "/repo/x/y/pkg")).toContain('runtime_fixture_text("../../🧪️tests/🧫️fixtures/after.json")');
    expect(fixturesToRuntime("fn plain() {}", "/repo/a.rs", "/repo")).toBeNull();
  });
  test("moves any embedded text of a test file to a runtime read, whatever its path", () => {
    const source = 'fn run() { let descriptor = include_str!("../../🧩️component/🔣️.json"); assert!(!descriptor.is_empty()); }\n';
    const rewritten = fixturesToRuntime(source, "/repo/x/y/🧪️tests/🔬️unit/🦀️.rs", "/repo")!;
    expect(rewritten).toContain('runtime_fixture_text("@fixtures/x/y/🧩️component/🔣️.json")');
    expect(fixturesToRuntime(source, "/repo/x/y/lib.rs", "/repo")).toBeNull();
  });
  test("turns a fixture constant of an inline module into a lazily read static that reaches the helper through super", () => {
    const nested = 'mod inner {\n    const CONTRACT: &str = include_str!("../../🧪️tests/🧫️fixtures/c.json");\n    fn f() -> usize { CONTRACT.len() }\n}\n';
    const rewritten = fixturesToRuntime(nested, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain("static CONTRACT: std::sync::LazyLock<&str> = std::sync::LazyLock::new(|| super::runtime_fixture_text(\"@fixtures/");
    expect(rewritten).toContain("(*CONTRACT).len()");
    const array = 'mod inner {\n    const ROWS: [&str; 2] = [include_str!("../../🧪️tests/🧫️fixtures/e.json"), include_str!("../../🧪️tests/🧫️fixtures/f.json")];\n    fn n() -> usize { ROWS.len() }\n}\n';
    const arrayRewrite = fixturesToRuntime(array, "/repo/x/y/z/test.rs", "/repo")!;
    expect(arrayRewrite).toContain("static ROWS: std::sync::LazyLock<[&str; 2]> = std::sync::LazyLock::new(|| [super::runtime_fixture_text(");
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

  test("registers carved crates in workspace members, router packages and the lockfile", () => {
    expect(addWorkspaceMembers('[workspace]\nmembers = [ "b", "a" ]\n', ["c", "a"])).toBe('[workspace]\nmembers = [\n  "a",\n  "b",\n  "c",\n]\n');
    expect(patchRouterPackages('await runArtifactRustPackageMain(import.meta.dir, "demo",{snapshotSqliteTests:[]});', "demo", ["demo-x"])).toContain('packages: ["demo", "demo-x"]');
    expect(patchRouterPackages('await runRepositoryCargoTests(["demo"], this.repoRoot, this.invocation, rest);', "demo", ["demo-mesh", "demo-tests"])).toContain('runRepositoryCargoTests(["demo", "demo-mesh", "demo-tests"], this.repoRoot');
    const lock = patchCargoLock('version = 4\n\n[[package]]\nname = "demo"\nversion = "0.1.0"\ndependencies = [\n "serde",\n]\n', "demo", [{ name: "demo-x", dependsOn: [] }]);
    expect(lock).toContain('name = "demo-x"');
    expect(lock).toMatch(/name = "demo"[\s\S]*"demo-x"/u);
  });
  test("keeps every version of a multi-version package when it patches the lockfile", () => {
    const text = 'version = 4\n\n[[package]]\nname = "bit-vec"\nversion = "0.6.3"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "aa"\n\n[[package]]\nname = "bit-vec"\nversion = "0.8.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "bb"\n\n[[package]]\nname = "demo"\nversion = "0.1.0"\ndependencies = [\n "bit-vec 0.8.0",\n]\n\n[[package]]\nname = "zeta"\nversion = "1.0.0"\n';
    const lock = patchCargoLock(text, "demo", [{ name: "demo-x", dependsOn: [] }, { name: "demo-y", dependsOn: ["demo-x"] }]);
    expect(lock.match(/name = "bit-vec"/gu)?.length).toBe(2);
    expect(lock).toContain('checksum = "aa"');
    expect(lock).toContain('checksum = "bb"');
    expect([...lock.matchAll(/name = "([^"]+)"/gu)].map((entry) => entry[1])).toEqual(["bit-vec", "bit-vec", "demo", "demo-x", "demo-y", "zeta"]);
    expect(lock).toContain('name = "demo-y"\nversion = "0.1.0"\ndependencies = [\n "bit-vec 0.8.0",\n "demo-x",\n]');
    expect(patchCargoLock(lock, "demo", [{ name: "demo-x", dependsOn: [] }, { name: "demo-y", dependsOn: ["demo-x"] }])).toBe(lock);
  });
  test("admits carved members that sit under a blanket standards exclusion so Cargo and the launcher agree", () => {
    const workspace = [
      "[workspace]",
      'members = [ "📦️packages/🦀️rust" ]',
      "exclude = [",
      '    "**/🏅️standards/**",',
      '    "**/🏭️bridge/**",',
      '    "🏅️standards/🔖️1/🏭️generator/📦️packages/🦀️rust",',
      "]",
      "",
      "[workspace.metadata.semio.repository]",
      "schema-version = 1",
      "exclude-patterns = [",
      '  "**/🏅️standards/**",',
      '  "**/🏭️bridge/**"',
      "]",
      'member-manifests = [ "**/📦️packages/🦀️rust/Cargo.toml" ]',
      "",
    ].join("\n");
    const member = "🏅️standards/🔖️1/🧬️schema/📦️packages/🦀️rust";
    expect(hidingExcludePattern(member, ["**/🏅️standards/**"])).toBe("**/🏅️standards/**");
    expect(hidingExcludePattern(member, ["**/🏭️bridge/**"])).toBeUndefined();
    expect(workspaceAdmissionProblems(workspace)).toEqual([]);
    expect(workspaceAdmissionProblems(workspace.replace('members = [ "📦️packages/🦀️rust" ]', `members = [ "📦️packages/🦀️rust", "${member}" ]`))).toEqual([`member ${member} is hidden from repository discovery by exclude pattern **/🏅️standards/**`]);
    const admitted = addWorkspaceMembers(workspace, [member]);
    expect(workspaceAdmissionProblems(admitted)).toEqual([]);
    const parsed = Bun.TOML.parse(admitted) as { workspace: { members: string[]; exclude: string[]; metadata: { semio: { repository: { "exclude-patterns": string[]; "member-manifests": string[] } } } } };
    expect(parsed.workspace.members).toEqual([member, "📦️packages/🦀️rust"]);
    expect(parsed.workspace.exclude).toEqual(["**/🏭️bridge/**", "🏅️standards/🔖️1/🏭️generator/📦️packages/🦀️rust"]);
    expect(parsed.workspace.metadata.semio.repository["exclude-patterns"]).toEqual(["**/🏭️bridge/**"]);
    expect(parsed.workspace.metadata.semio.repository["member-manifests"]).toEqual(["**/📦️packages/🦀️rust/Cargo.toml"]);
    expect(addWorkspaceMembers(admitted, [member])).toBe(admitted);
  });
  test("reports a source file that two crates mount and none once the second crate depends on the owner", () => {
    const root = mkdtempSync(join(tmpdir(), "shared-mounts-"));
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
      const manifest = (name: string, lib: string, extra = ""): string => `[package]\nname = "${name}"\nversion = "0.1.0"\n\n[lib]\npath = "${lib}"\n${extra}`;
      write("shared/🦀️.rs", "pub fn shared() {}\n");
      write("owner/Cargo.toml", manifest("owner", "../shared/🦀️.rs"));
      write("left/🦀️.rs", '#[path = "../shared/🦀️.rs"]\nmod shared;\n');
      write("left/Cargo.toml", manifest("left", "🦀️.rs"));
      write("right/🦀️.rs", '#[path = "../shared/🦀️.rs"]\nmod shared;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n');
      write("right/🧪️tests/🦀️.rs", "fn helper() {}\n");
      write("right/Cargo.toml", manifest("right", "🦀️.rs"));
      const manifests = ["owner/Cargo.toml", "left/Cargo.toml", "right/Cargo.toml"];
      expect(sharedMountFiles(root, manifests)).toEqual([{ file: "shared/🦀️.rs", manifests: ["left/Cargo.toml", "owner/Cargo.toml", "right/Cargo.toml"] }]);
      expect(sharedMountFiles(root, manifests, (path) => path.startsWith("owner/"))).toHaveLength(1);
      expect(sharedMountFiles(root, ["left/Cargo.toml", "right/Cargo.toml"], (path) => path.startsWith("nothing/"))).toEqual([]);
      write("left/🦀️.rs", "pub use owner as shared;\n");
      write("right/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\npub use owner as shared;\n');
      expect(sharedMountFiles(root, manifests)).toEqual([]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
  test("finds the plugin-local path crates an artifact crate depends on and rewrites crate aliases for the combined layout", () => {
    const root = mkdtempSync(join(tmpdir(), "sibling-crates-"));
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
      write("p/a/Cargo.toml", '[workspace]\nmembers = ["artifact"]\n[workspace.dependencies.demo-shared]\npath = "../shared"\n');
      write("p/shared/Cargo.toml", '[package]\nname = "demo-shared"\nversion = "0.1.0"\n');
      write("p/a/artifact/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n[lib]\npath = "🦀️.rs"\n[dependencies]\nsibling_alias = { path = "../../ext", package = "demo-ext" }\noutside = { path = "../../../q/other" }\ndemo-shared = { workspace = true }\n[dev-dependencies]\ndemo-inside = { path = "../inside" }\n');
      write("p/a/inside/Cargo.toml", '[package]\nname = "demo-inside"\nversion = "0.1.0"\n');
      write("p/ext/Cargo.toml", '[package]\nname = "demo-ext"\nversion = "0.1.0"\n[lib]\npath = "🦀️.rs"\n[dependencies]\ndeep = { path = "../deep" }\n');
      write("p/deep/Cargo.toml", '[package]\nname = "deep"\nversion = "0.1.0"\n');
      write("q/other/Cargo.toml", '[package]\nname = "other"\nversion = "0.1.0"\n');
      const found = siblingPathCrates(root, "p", "p/a", ["p/a/artifact/Cargo.toml"]);
      expect(found.map((entry) => entry.name).sort()).toEqual(["deep", "demo-ext", "demo-shared", "other"]);
      expect(found.find((entry) => entry.name === "other")!.dir).toBe("q/other");
      expect(found.find((entry) => entry.name === "demo-ext")!.idents.slice().sort()).toEqual(["demo_ext", "sibling_alias"]);
    } finally { rmSync(root, { recursive: true, force: true }); }
    const modules = new Map([["demo_ext", "__c_demo_ext"]]);
    expect(rewriteCrateAliases("pub use demo_ext as control;\nuse crate as native;\nuse demo_ext::x;", "__c_self", modules)).toBe("pub use crate::__c_demo_ext as control;\nuse crate::__c_self as native;\nuse demo_ext::x;");
  });
  test("registers carved crates as build-owner rows at their byte-order position and never twice", () => {
    const raw = JSON.stringify({ version: 1, owners: [{ name: "@a/x", root: "x", manifest: "x/p.json", packages: [] }, { name: "demo-a", root: "a/📦️", manifest: "a/📦️/Cargo.toml", packages: [] }, { name: "demo-z", root: "z/📦️", manifest: "z/📦️/Cargo.toml", packages: [] }], catalogProviders: [] }, null, 2) + "\n";
    const first = insertBuildOwnerRows(raw, [{ name: "demo-m", root: "m/📦️" }, { name: "demo-a", root: "a/📦️" }]);
    expect(first.added).toEqual(["demo-m"]);
    expect(JSON.parse(first.text).owners.map((owner: { name: string }) => owner.name)).toEqual(["@a/x", "demo-a", "demo-m", "demo-z"]);
    expect(JSON.parse(first.text).owners[2].manifest).toBe("m/📦️/Cargo.toml");
    expect(insertBuildOwnerRows(first.text, [{ name: "demo-m", root: "m/📦️" }]).added).toEqual([]);
    expect(insertBuildOwnerRows(raw, []).text).toBe(raw);
  });
  test("rebases include arguments of items moved to another directory", () => {
    const moved = 'pub const A: &str = include_str!("📜️x.json");\nconst B: &[u8] = include_bytes!("../up/y.bin");\nfn f() { let _ = "include_str!(\\"no\\")"; }';
    const rebased = rebaseRustIncludes(moved, "/repo/a", "/repo/a/deep/er");
    expect(rebased).toContain('include_str!("../../📜️x.json")');
    expect(rebased).toContain('include_bytes!("../../../up/y.bin")');
    expect(rebased).toContain('"include_str!(\\"no\\")"');
    expect(rebaseRustIncludes(moved, "/repo/a", "/repo/a")).toBe(moved);
  });

  test("hoists a root-bound test only when it needs nothing from its host module, and takes the doc comment of the removed mount with it", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": "pub fn host() {}\n/// 🧪️ documents the next item, which is the test mount\n// a plain comment between does not stop the attachment\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️free/🦀️.rs\"]\nmod free;\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️bound/🦀️.rs\"]\nmod bound;\n",
        "a/🧪️tests/🔬️free/🦀️.rs": "#[test]\nfn free() { assert_eq!(crate::DOCUMENT, \"d\"); }\n",
        "a/🧪️tests/🔬️bound/🦀️.rs": "use super::*;\n#[test]\nfn bound() { host(); assert_eq!(crate::DOCUMENT, \"d\"); }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).toContain("mod hoisted_");
      expect(result.rootSource).toContain("🔬️free/🦀️.rs");
      expect(result.rootSource).not.toContain("🔬️bound/🦀️.rs");
      const host = result.edits.get(join(root, "a/🦀️.rs"))!;
      expect(host).not.toContain("mod free;");
      expect(host).not.toContain("documents the next item");
      expect(host).toContain("mod bound;");
      expect(host).toContain("pub fn host() {}");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("folds a root-level mount tree file back into the root with re-based paths and marks the file for deletion", () => {
    const root = "/repo/art";
    const files: Record<string, string> = {
      "/repo/art/🌳️tree/🦀️.rs": '//! 🌳️ The mount tree.\n\n#[path = "."]\npub mod v1 {\n    #[path = "../x/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "."]\n    pub mod deep {\n        #[path = "../x/y/🦀️.rs"]\n        pub mod leaf;\n    }\n}\n',
      "/repo/art/🧾️decl/🦀️.rs": "pub fn declaration() {}\n",
    };
    const source = '#[path = "🌳️tree/🦀️.rs"]\npub mod standards;\n#[path = "🧾️decl/🦀️.rs"]\nmod decl;\n';
    const folded = inlineMountTreeFiles(`${root}/🦀️.rs`, source, (absolute) => files[absolute] ?? null);
    expect(folded.deletes).toEqual(new Set(["/repo/art/🌳️tree/🦀️.rs"]));
    expect(folded.rootSource).toContain('#[path = "."]\npub mod standards {');
    expect(folded.rootSource).toContain('#[path = "x/🦀️.rs"]');
    expect(folded.rootSource).toContain('#[path = "x/y/🦀️.rs"]');
    expect(folded.rootSource).toContain('#[path = "🧾️decl/🦀️.rs"]\nmod decl;');
    expect(folded.rootSource).not.toContain("The mount tree");
    expect(parseRustSourceTree(folded.rootSource).modules.map((node) => [node.name, node.inline])).toEqual([["standards", true], ["decl", false]]);
  });
  test("detaches a test whose host imports an item with a relative super path by carrying the absolute import", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-super-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "."]\npub mod m {\n    #[path = "m/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "m/leaf/🦀️.rs"]\n    pub mod leaf;\n}\n',
        "m/🦀️.rs": "pub struct Agg;\n",
        "m/leaf/🦀️.rs": 'use super::Agg;\nuse crate::m::{self, Agg as Alias};\npub struct Leaf;\npub fn make() -> Agg { Agg }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "m/leaf/🧪️tests/🦀️.rs": "use super::*;\n#[test]\nfn t() { let _: Agg = make(); let _: Alias = Agg; let _ = Leaf; }\n",
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: false, hoist: false, detach: true });
      const test = result.edits.get(join(root, "m/leaf/🧪️tests/🦀️.rs"));
      expect(test).toContain("use crate::m::Agg;");
      expect(test).toContain("use crate::m::Agg as Alias;");
      expect(test).not.toMatch(/\buse super::/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("a brace group names the chains of its members, not the shared prefix module", () => {
    expect(crateRelativeChains("use crate::a::b::{c::X, d::Y};")).toEqual([["a", "b", "c", "X"], ["a", "b", "d", "Y"]]);
    expect(crateRelativeChains("use crate::a::b::*;")).toEqual([["a", "b"]]);
    expect(crateRelativeChains("let _ = crate::a::b::run();")).toEqual([["a", "b", "run"]]);
  });
  test("aggregates sibling leaf tests into the file that holds the inline parent module, not into the lib root", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-aggregate-file-"));
    try {
      const leaf = (name: string): Record<string, string> => ({
        [`tree/${name}/🦀️.rs`]: `pub fn ${name}() {}\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n`,
        [`tree/${name}/🧪️tests/🦀️.rs`]: `use super::*;\n#[test]\nfn runs() { ${name}(); }\n`,
      });
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "tree/🦀️.rs"]\npub mod s;\n',
        "tree/🦀️.rs": '#[path = "."]\npub mod modules {\n    #[path = "a/🦀️.rs"]\n    pub mod a;\n    #[path = "b/🦀️.rs"]\n    pub mod b;\n    #[path = "c/🦀️.rs"]\n    pub mod c;\n}\n',
        ...leaf("a"), ...leaf("b"), ...leaf("c"),
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: true, hoist: false });
      expect(result.rootSource).toBe(layout["🦀️.rs"]);
      const tree = result.edits.get(join(root, "tree/🦀️.rs"))!;
      expect(tree).toContain("mod leaf_tests;");
      expect(tree.indexOf("mod leaf_tests;")).toBeLessThan(tree.lastIndexOf("}"));
      expect(result.notes.some((note) => note.includes("aggregated 3 leaf test files"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("rebases paths joined onto the cargo manifest directory when a file moves to another crate directory", () => {
    const source = 'let a = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../x/y.json"));\nlet b = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../z");\nlet c = env!("CARGO_MANIFEST_DIR");';
    const moved = rebaseManifestDirPaths(source, "/repo/art/📦️packages/🦀️rust", "/repo/art/sub/deeper/📦️packages/🦀️rust");
    expect(moved).toContain('"/../../../../../x/y.json"');
    expect(moved).toContain('.join("../../../../📦️packages/z")');
    expect(moved).toContain('let c = env!("CARGO_MANIFEST_DIR");');
    expect(rebaseManifestDirPaths(moved, "/repo/art/sub/deeper/📦️packages/🦀️rust", "/repo/art/📦️packages/🦀️rust")).toBe(source);
    expect(rebaseManifestDirPaths(source, "/a", "/a")).toBe(source);
  });
  test("leaves fixtures that feed literal-only macros embedded and keeps test modules in place that sibling tests name through super", () => {
    const concatenated = 'let script = concat!(include_str!("../../🧪️tests/🧫️fixtures/a.sql"), "\\nselect 1;");\nlet plain = include_str!("../../🧪️tests/🧫️fixtures/b.json");\nfn f() {}';
    const rewritten = fixturesToRuntime(`fn t() { ${concatenated} }`, "/repo/x/y/z/test.rs", "/repo")!;
    expect(rewritten).toContain('concat!(include_str!("../../🧪️tests/🧫️fixtures/a.sql")');
    expect(rewritten).toContain('runtime_fixture_text("@fixtures/');
    const constConcat = 'const A: &str = include_str!("../../🧪️tests/🧫️fixtures/a.sql");\nfn t() { let _ = concat!(A, "x"); }';
    expect(fixturesToRuntime(constConcat, "/repo/x/y/z/test.rs", "/repo")).toBeNull();
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-ref-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n#[cfg(test)]\n#[path = "🫴️receiving/🧪️tests/🦀️.rs"]\nmod receiving;\n',
        "a/🧪️tests/🦀️.rs": 'pub(super) fn fixture() -> &\'static str { crate::DOCUMENT }\n',
        "a/🫴️receiving/🧪️tests/🦀️.rs": 'fn source() -> &\'static str { super::tests::fixture() }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).not.toContain("mod hoisted_");
      expect(result.notes.some((note) => note.includes("super::tests"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("carries the private absolute imports of the host into a hoisted test that glob-imports the host", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-glob-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Private;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'use crate::a::*;\n#[test]\nfn t() { let _: Private<u8, u8> = Private::new(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).toContain("mod hoisted_");
      expect(result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))).toContain("use std::collections::BTreeMap as Private;");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("retargets the absolute self path of a hoisted test to its new mount", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-self-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'pub fn helper() {}\nmod inner {\n    use crate::a::tests::*;\n    #[test]\n    fn t() { helper(); let _ = crate::DOCUMENT; }\n}\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      const name = /mod (hoisted_[A-Za-z0-9_]+);/u.exec(result.rootSource)![1]!;
      const moved = result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))!;
      expect(moved).toContain(`use crate::${name}::*;`);
      expect(moved).not.toContain("crate::a::tests");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("sees the aliases that a use statement inside a function body binds", () => {
    const chains = aliasChains("pub fn io() {\n    use crate::standards::io::export::serializers::artifacts as export;\n    let _ = export::json::Codec;\n}\n");
    expect(chains).toContainEqual(["standards", "io", "export", "serializers", "artifacts", "json", "Codec"]);
  });
  test("carries the private imports of the host into a nested module of a detached test that globs the host", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-detach-nested-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Private;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'pub mod context {\n    use super::super::*;\n    pub fn make() -> Private<u8, u8> { Private::new() }\n}\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: false, detach: true });
      const moved = result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))!;
      expect(moved).toMatch(/use crate::a::\*;\n\s*use std::collections::BTreeMap as Private;/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("names a private import of the host by its carried import in a hoisted test instead of through the host path", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-private-import-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Private;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
        "a/🧪️tests/🦀️.rs": 'use crate::a::*;\n#[test]\nfn t() { let _ = crate::a::Private::<u8, u8>::new(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      const moved = result.edits.get(join(root, "a/🧪️tests/🦀️.rs"))!;
      expect(moved).toContain("use std::collections::BTreeMap as Private;");
      expect(moved).toContain("let _ = Private::<u8, u8>::new();");
      expect(moved).not.toContain("crate::a::Private");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("widens the private host items a hoisted test uses and carries private absolute imports into hoisted tests", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-private-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'use std::collections::BTreeMap as Map;\nconst SECRET: u8 = 1;\n#[cfg(test)]\n#[path = "🧪️tests/🔬️secret/🦀️.rs"]\nmod secret;\n#[cfg(test)]\n#[path = "🧪️tests/🔬️map/🦀️.rs"]\nmod map;\n',
        "a/🧪️tests/🔬️secret/🦀️.rs": '#[test]\nfn t() { let _ = crate::DOCUMENT; let _ = SECRET; }\n',
        "a/🧪️tests/🔬️map/🦀️.rs": '#[test]\nfn t() { let _: Map<u8, u8> = Map::new(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      expect(result.rootSource).toContain("🔬️map/🦀️.rs");
      expect(result.rootSource).toContain("🔬️secret/🦀️.rs");
      expect(result.edits.get(join(root, "a/🧪️tests/🔬️map/🦀️.rs"))).toContain("use std::collections::BTreeMap as Map;");
      expect(result.edits.get(join(root, "a/🧪️tests/🔬️secret/🦀️.rs"))).toContain("use crate::a::{SECRET};");
      expect(result.edits.get(join(root, "a/🦀️.rs"))).toContain("pub const SECRET: u8 = 1;");
      expect(result.notes.some((note) => note.includes("widened SECRET of the host to pub"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("widens a private host item once when several hoisted tests use it", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-hoist-twice-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod a {\n    #[path = "a/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
        "a/🦀️.rs": 'fn secret() {}\n#[cfg(test)]\n#[path = "🧪️tests/🔬️one/🦀️.rs"]\nmod one;\n#[cfg(test)]\n#[path = "🧪️tests/🔬️two/🦀️.rs"]\nmod two;\n',
        "a/🧪️tests/🔬️one/🦀️.rs": '#[test]\nfn t() { secret(); let _ = crate::DOCUMENT; }\n',
        "a/🧪️tests/🔬️two/🦀️.rs": '#[test]\nfn t() { secret(); let _ = crate::DOCUMENT; }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(["DOCUMENT"]), { fixtures: false, aggregate: false, hoist: true });
      const host = result.edits.get(join(root, "a/🦀️.rs"))!;
      expect(host).toContain("pub fn secret() {}");
      expect(host).not.toContain("pub pub");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("treats the import of an unlinked first-party crate as external noise and keeps every other unresolved import", () => {
    expect(external("E0432\tunresolved import `semio_framework_artifact_playbook_playbook`")).toBe(false);
    expect(external("E0432\tunresolved import `semio_framework_artifact_playbook_playbook`\tsome/file.rs\tno `x` in the root")).toBe(false);
    expect(external("E0432\tunresolved import `crate::editor::forms::support`")).toBe(true);
    expect(external("E0432\tunresolved import `helper`")).toBe(true);
  });
  test("inlines root file modules that only glob-re-export plain items and leaves every other file module mounted", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-inline-root-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": 'pub struct Keep;\n#[path = "📦️records/🦀️.rs"]\nmod records;\npub use records::*;\n#[path = "🧷️nested/🦀️.rs"]\nmod nested;\npub use nested::*;\n',
        "📦️records/🦀️.rs": '//! 📦️ Records.\n\nuse super::*;\n\npub struct Record(pub Keep);\n',
        "🧷️nested/🦀️.rs": 'use super::*;\n#[path = "x/🦀️.rs"]\nmod x;\npub fn f() {}\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = inlineRootFileModules(null, join(root, "🦀️.rs"), layout["🦀️.rs"]!);
      expect(result.rootSource).toContain("//#region 🧩️records");
      expect(result.rootSource).toContain("pub struct Record(pub Keep);");
      expect(result.rootSource).not.toContain("pub use records::*;");
      expect(result.rootSource).not.toContain("use super::*;");
      expect(result.rootSource).toContain("mod nested;");
      const own = inlineRootFileModules(null, join(root, "🦀️.rs"), 'pub struct Keep;\n#[path = "📦️records/🦀️.rs"]\nmod records;\npub use records::*;\n');
      expect(own.rootSource).toContain("//#region 🧩️records");
      writeFileSync(join(root, "📦️records/🦀️.rs"), "use super::*;\n//#region Own\npub struct Record(pub Keep);\n//#endregion Own\n");
      const kept = inlineRootFileModules(null, join(root, "🦀️.rs"), 'pub struct Keep;\n#[path = "📦️records/🦀️.rs"]\nmod records;\npub use records::*;\n');
      expect(kept.rootSource).toContain("//#region Own");
      expect(kept.rootSource).not.toContain("🧩️records");
      expect([...result.deletes]).toEqual([join(root, "📦️records/🦀️.rs")]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("keeps the private component segment when a leaf glob is made absolute for a mutation family", () => {
    const raw = ["editor", "config", "component", "mutations", "set"];
    expect(absolutizeBeyond("use super::*;\n", raw, 0, false)).toBe("use crate::editor::config::component::mutations::*;\n");
    expect(absolutizeBeyond("use super::*;\n", raw, 0)).toBe("use crate::editor::config::mutations::*;\n");
  });
  const weaveFiles = (deep: boolean): Record<string, string> => ({
    "Cargo.toml": files["Cargo.toml"]!,
    "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-weave"\nversion = "0.1.0"\n',
    "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-weave"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
    "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-weave",{snapshotSqliteTests:[]});\n',
    "🦀️.rs": '//! root\npub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n#[path = "."]\npub mod schema {\n    #[path = "🧬️schema/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
    "🚪️io/🦀️.rs": "//! io\npub fn helper() -> u8 { 1 }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
    "🚪️io/📝️text/🦀️.rs": `//! text\npub fn helper_text() -> u8 { crate::io::helper() + crate::schema::value() }\n${deep ? '#[path = "🧱️deep/🦀️.rs"]\npub mod deep;\n' : ""}`,
    ...(deep ? { "🚪️io/📝️text/🧱️deep/🦀️.rs": "//! deep\npub fn run() -> u8 { crate::io::text::helper_text() }\n" } : {}),
    "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { 2 }\n",
  });
  const weaveTree = (deep: boolean): string => {
    const root = mkdtempSync(join(tmpdir(), "semio-weave-"));
    for (const [path, content] of Object.entries(weaveFiles(deep))) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
    return root;
  };
  const tiny = { target: { files: 1, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };

  test("weaves a mounted child into the facade instead of making its parent depend on it", async () => {
    const root = weaveTree(false);
    try {
      const merged = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, weave: false });
      const holder = (plan: typeof merged, suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix));
      expect(holder(merged, "io")).toBeDefined();
      expect(holder(merged, "io::text")).toBeUndefined();
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const io = holder(plan, "io")!;
      const text = holder(plan, "io::text")!;
      expect(text).toBeDefined();
      expect(text).not.toBe(io);
      expect(io.dependsOn).toEqual([]);
      expect(text.dependsOn).toContain(io.id);
      expect(plan.weave.map((entry) => entry.eff.join("::"))).toEqual(["io::text"]);
      const { writes } = await carveWrites(plan);
      const facade = writes.find((write) => write.path === "🦀️.rs")!.content;
      expect(facade).toContain("pub mod io {");
      expect(facade).toMatch(new RegExp(`pub use ${crateIdentOf(io.name)}::io::\\*;`, "u"));
      expect(facade).toContain(`pub use ${crateIdentOf(text.name)}::io::text;`);
      const ioFile = writes.find((write) => write.path === "🚪️io/🦀️.rs")!.content;
      expect(ioFile).not.toContain("pub mod text;");
      expect(ioFile).not.toContain("pub use");
      const original = resolveCrateModules(root, join(root, "🦀️.rs"));
      applyCarveWrites(root, writes);
      const carved = [resolveCrateModules(root, join(root, "🦀️.rs")), ...plan.crates.map((crate) => resolveCrateModules(root, join(root, crate.packageDir, "🦀️.rs")))];
      expect(compareModuleTrees(original, carved).problems).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps a grandchild mounted when its ancestor reaches it through a relative path", async () => {
    const root = weaveTree(true);
    try {
      writeFileSync(join(root, "🚪️io/🦀️.rs"), "//! io\npub fn helper() -> u8 { 1 }\npub fn via() -> u8 { text::deep::run() }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n");
      writeFileSync(join(root, "🚪️io/📝️text/🦀️.rs"), "//! text\npub fn helper_text() -> u8 { 3 }\n#[path = \"🧱️deep/🦀️.rs\"]\npub mod deep;\n");
      writeFileSync(join(root, "🚪️io/📝️text/🧱️deep/🦀️.rs"), "//! deep\npub fn run() -> u8 { 4 }\n");
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems.filter((problem) => !problem.includes("mounts no file"))).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const io = holder("io");
      const deep = holder("io::text::deep");
      expect(io.dependsOn).toContain(deep.id);
      expect(plan.weave.map((entry) => entry.eff.join("::"))).not.toContain("io::text::deep");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("weaves a grandchild below a woven child with a local module that glob-exports its owner", async () => {
    const root = weaveTree(true);
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(new Set([holder("io"), holder("io::text"), holder("io::text::deep")]).size).toBe(3);
      expect(plan.weave.map((entry) => entry.eff.join("::")).sort()).toEqual(["io::text", "io::text::deep"]);
      const { writes } = await carveWrites(plan);
      const facade = writes.find((write) => write.path === "🦀️.rs")!.content;
      expect(facade).toContain("    pub mod text {");
      expect(facade).toContain(`pub use ${crateIdentOf(holder("io::text").name)}::io::text::*;`);
      expect(facade).toContain(`pub use ${crateIdentOf(holder("io::text::deep").name)}::io::text::deep;`);
      expect(facade.indexOf("pub mod text {")).toBeLessThan(facade.indexOf("io::text::deep;"));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps the mount re-export when the parent module uses its child", async () => {
    const root = weaveTree(false);
    try {
      writeFileSync(join(root, "🚪️io/🦀️.rs"), "//! io\npub fn helper() -> u8 { 1 }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\npub fn twice() -> u8 { text::helper_text() * 2 }\n");
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.weave).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  const writeTree = (prefix: string, tree: Record<string, string>): string => {
    const root = mkdtempSync(join(tmpdir(), prefix));
    for (const [path, content] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
    return root;
  };
  const scaffold = (name: string, rootSource: string): Record<string, string> => ({
    "Cargo.toml": files["Cargo.toml"]!,
    "Cargo.lock": `version = 4\n\n[[package]]\nname = "${name}"\nversion = "0.1.0"\n`,
    "📦️packages/🦀️rust/Cargo.toml": `[package]\nworkspace = "../.."\nname = "${name}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n`,
    "📦️packages/🦀️rust/📜️script.ts": `await runArtifactRustPackageMain(import.meta.dir, "${name}",{snapshotSqliteTests:[]});\n`,
    "🦀️.rs": rootSource,
  });

  test("keeps the mount of a module that an item of the artifact root names, because the crate hosting that item cannot see a woven path", async () => {
    const root = weaveTree(false);
    try {
      const rootFile = join(root, "🦀️.rs");
      writeFileSync(rootFile, readFileSync(rootFile, "utf8").replace('pub const DOCUMENT: &str = "d";', 'pub const DOCUMENT: &str = "d";\npub fn go() -> u8 { crate::io::text::helper_text() }'));
      const before = await planArtifactCrates(weaveTree(false), "📦️packages/🦀️rust", { ...tiny });
      expect(before.weave.map((entry) => entry.eff.join("::"))).toEqual(["io::text"]);
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.weave).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("gates the re-exports of a moved root item that exists only under cfg(test), in the crates and in the facade", async () => {
    const root = writeTree("semio-gated-hook-", {
      ...scaffold("demo-gated-hook", '//! root\n#[cfg(test)]\npub fn hook() -> u8 { 3 }\npub fn go() -> u8 { 4 }\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n#[path = "."]\npub mod schema {\n    #[path = "🧬️schema/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": "//! io\npub fn run() -> u8 { crate::schema::value() + crate::go() }\n#[cfg(test)]\nmod checks {\n    #[test]\n    fn uses_the_hook() { assert_eq!(crate::hook(), 3); }\n}\n",
      "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { 2 }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, weave: false, tests: { fixtures: false, aggregate: false, hoist: false, detach: false } });
      const lines = plan.crates.flatMap((crate) => crate.itemImportLines).filter((line) => /\bhook\b/u.test(line));
      expect(lines.length).toBeGreaterThan(0);
      for (const line of lines) expect(line.startsWith("#[cfg(test)]\n")).toBe(true);
      expect(plan.crates.flatMap((crate) => crate.itemImportLines).some((line) => /\bgo\b/u.test(line) && !line.startsWith("#[cfg(test)]"))).toBe(true);
      const { writes } = await carveWrites(plan);
      const facade = writes.find((write) => write.path === "🦀️.rs")!.content;
      expect(facade).toMatch(/#\[cfg\(test\)\]\npub use [a-z_]+::\{hook\};/u);
      expect(facade).toMatch(/\npub use [a-z_]+::\{go\};/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("a sibling that reaches a module through the mount of its parent depends on the module's owner, not on the parent", async () => {
    const root = writeTree("semio-parent-path-", {
      ...scaffold("demo-parent-path", '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": '//! io\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n',
      "🚪️io/📝️text/🦀️.rs": '//! text\n#[path = "🅰️a/🦀️.rs"]\npub mod a;\n#[path = "🅱️b/🦀️.rs"]\npub mod b;\n#[path = "🅲️c/🦀️.rs"]\npub mod c;\n',
      "🚪️io/📝️text/🅰️a/🦀️.rs": "//! a\npub fn value() -> u8 { 1 }\n",
      "🚪️io/📝️text/🅱️b/🦀️.rs": "//! b\nuse crate::io::text::a;\npub fn run() -> u8 { a::value() }\n",
      "🚪️io/📝️text/🅲️c/🦀️.rs": "//! c\nuse crate::io::text::b;\npub fn again() -> u8 { b::run() }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const [text, a, b, c] = [holder("io::text"), holder("io::text::a"), holder("io::text::b"), holder("io::text::c")];
      expect(new Set([text, a, b, c]).size).toBe(4);
      expect(b.dependsOn).toContain(a.id);
      expect(c.dependsOn).toContain(b.id);
      expect(b.dependsOn).not.toContain(text.id);
      expect(c.dependsOn).not.toContain(text.id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps a slice in the crate of the cfg(test) module it reaches, because a dependency never compiles that module", async () => {
    const root = writeTree("semio-cfg-test-module-", {
      ...scaffold("demo-cfg-test-module", '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": '//! io\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n',
      "🚪️io/📝️text/🦀️.rs": '//! text\n#[path = "🅰️a/🦀️.rs"]\npub mod a;\n#[path = "🅱️b/🦀️.rs"]\npub mod b;\n',
      "🚪️io/📝️text/🅰️a/🦀️.rs": '//! a\npub fn value() -> u8 { 1 }\n#[cfg(test)]\n#[path = "🧰️support/🦀️.rs"]\npub mod support;\n',
      "🚪️io/📝️text/🅰️a/🧰️support/🦀️.rs": "//! support\npub fn helper() -> u8 { 2 }\n",
      "🚪️io/📝️text/🅱️b/🦀️.rs": "//! b\npub fn run() -> u8 { 3 }\n#[cfg(test)]\nmod checks {\n    #[test]\n    fn t() { assert_eq!(crate::io::text::a::support::helper(), 2); }\n}\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, tests: { fixtures: false, aggregate: false, hoist: false, detach: false } });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(holder("io::text::a::support").id).toBe(holder("io::text::b").id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("does not make a slice depend on the owner of a prefix when the path names an inline wrapper module that the owner does not own", async () => {
    const root = writeTree("semio-wrapper-prefix-", {
      ...scaffold("demo-wrapper-prefix", '//! root\n#[path = "."]\npub mod top {\n    #[path = "🔝top/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "."]\n    pub mod mid {\n        #[path = "🅰️a/🦀️.rs"]\n        pub mod a;\n        #[path = "🅱️b/🦀️.rs"]\n        pub mod b;\n    }\n}\n'),
      "🔝top/🦀️.rs": "//! top\npub fn here() -> u8 { 0 }\n",
      "🅰️a/🦀️.rs": "//! a\npub fn value() -> u8 { 1 }\n",
      "🅱️b/🦀️.rs": "//! b\nuse crate::top::mid;\npub fn run() -> u8 { mid::a::value() }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const [top, a, b] = [holder("top"), holder("top::mid::a"), holder("top::mid::b")];
      expect(new Set([top, a, b]).size).toBe(3);
      expect(b.dependsOn).toContain(a.id);
      expect(b.dependsOn).not.toContain(top.id);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("glob-exports the owner of a module into the wrapper that hosts its detached test in another crate, so the wrapper does not shadow the module", async () => {
    const root = writeTree("semio-detached-test-host-", {
      ...scaffold("demo-detached-test-host", '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n'),
      "🚪️io/🦀️.rs": "//! io\npub fn helper() -> u8 { 1 }\npub fn both() -> u8 { text::helper_text() + helper() }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
      "🚪️io/📝️text/🦀️.rs": `//! text\n${"// padding that makes this slice too large to share a crate with the test\n".repeat(30)}pub fn helper_text() -> u8 { 2 }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n`,
      "🚪️io/📝️text/🧪️tests/🦀️.rs": "use super::*;\nuse crate::io::helper;\n\n#[test]\nfn text_helper_is_two() { assert_eq!(helper_text() + helper(), 3); }\n",
    });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { target: { files: 8, bytes: 1500, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 }, tests: { fixtures: false, aggregate: false, hoist: true, detach: true } });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      const [host, text] = [holder("io::text::tests"), holder("io::text")];
      expect(host.id).not.toBe(text.id);
      const { writes } = await carveWrites(plan);
      const mirror = writes.find((write) => write.path === `${host.packageDir}/🦀️.rs`)!.content;
      expect(mirror).toContain(`pub use ${crateIdentOf(text.name)}::io::text::*;`);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("resolves a type name that two standards define to the definition in the closest module", async () => {
    const standard = (name: string): Record<string, string> => ({
      [`🅰️${name}/🧬️schema/🦀️.rs`]: "//! schema\npub struct Doc;\n",
      [`🅰️${name}/🚪️io/🦀️.rs`]: `//! io\nuse crate::${name}::schema::Doc;\nimpl std::fmt::Display for Doc {\n    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, "${name}") }\n}\n`,
    });
    const mounts = (name: string): string => `#[path = "."]\npub mod ${name} {\n    #[path = "🅰️${name}/🧬️schema/🦀️.rs"]\n    pub mod schema;\n    #[path = "🅰️${name}/🚪️io/🦀️.rs"]\n    pub mod io;\n}\n`;
    const root = writeTree("semio-closest-type-", { ...scaffold("demo-closest-type", `//! root\n${mounts("v1")}${mounts("v2")}`), ...standard("v1"), ...standard("v2") });
    try {
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems.filter((problem) => problem.includes("ambiguous coherence"))).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(holder("v1::io")).toBe(holder("v1::schema"));
      expect(holder("v2::io")).toBe(holder("v2::schema"));
      expect(holder("v1::io")).not.toBe(holder("v2::io"));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("recognizes root alias modules and resolves the super paths written in inline module bodies", () => {
    const source = [
      "pub mod schema {\n    pub use super::standards::v1::any::schema::*;\n}",
      "pub mod diff {\n    pub use crate::standards::v1::any::schema::diff::*;\n    pub use crate::host::inferences::*;\n}",
      "pub mod busy {\n    pub use super::standards::v1::x::*;\n    pub fn f() {}\n}",
      "pub mod grouped {\n    pub use super::standards::{a, b};\n}",
      "pub mod engine {\n    pub use super::subsets::any::io::register;\n}",
    ].join("\n");
    const aliases = rootAliasModules(source, parseRustSourceTree(source).modules);
    expect([...aliases.keys()].sort()).toEqual(["diff", "engine", "schema"]);
    expect(aliases.get("schema")!.targets).toEqual([["standards", "v1", "any", "schema"]]);
    expect(aliases.get("diff")!.targets).toEqual([["standards", "v1", "any", "schema", "diff"], ["host", "inferences"]]);
    expect(aliases.get("engine")!.text).toContain("pub use super::subsets::any::io::register;");
    const inline = "pub mod engine {\n    pub use super::subsets::any::io::register;\n    pub use super::super::top::{x, y::z};\n    pub mod inner { pub use super::kept::A; }\n}";
    const module = parseRustSourceTree(inline).modules[0]!;
    const nested = module.children.map((child): [number, number] => [child.start, child.end]);
    expect(superChains(inline, module.bodyStart, module.bodyEnd, nested, ["standards", "v_raw", "engine"])).toEqual([["standards", "v_raw", "subsets", "any", "io", "register"], ["standards", "top", "x"], ["standards", "top", "y", "z"]]);
  });

  test("copies a root alias module into every crate that goes through it and depends on the crate that owns the target", async () => {
    const tree = mkdtempSync(join(tmpdir(), "semio-shim-"));
    try {
      for (const [path, content] of Object.entries(weaveFiles(false))) { mkdirSync(dirname(join(tree, path)), { recursive: true }); writeFileSync(join(tree, path), content); }
      writeFileSync(join(tree, "🦀️.rs"), readFileSync(join(tree, "🦀️.rs"), "utf8") + "pub mod views {\n    pub use super::schema::*;\n}\n");
      writeFileSync(join(tree, "🚪️io/📝️text/🦀️.rs"), "//! text\npub fn helper_text() -> u8 { crate::io::helper() + crate::views::value() }\n");
      const plan = await planArtifactCrates(tree, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      const holder = (suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::") === suffix))!;
      expect(holder("io::text").dependsOn).toContain(holder("schema").id);
      expect(holder("io::text").itemImportLines.some((line) => line.includes("pub mod views {"))).toBe(true);
      expect(holder("schema").itemImportLines.some((line) => line.includes("pub mod views {"))).toBe(false);
      const { writes } = await carveWrites(plan);
      expect(writes.find((write) => write.path === "🦀️.rs")!.content).toContain("pub mod views {");
    } finally {
      rmSync(tree, { recursive: true, force: true });
    }
  });

  test("qualifies root module heads of moved items and leaves every other path alone", () => {
    const text = "use schema::snapshot::Snap;\npub fn f() { let _ = standards::v1::run(); let _ = crate::schema::x(); let _ = other::y(); let _ = self::schema::z(); let _ = value.schema::<u8>(); }\n";
    const out = qualifyModuleHeads(text, new Set(["schema", "standards"]));
    expect(out).toContain("use crate::schema::snapshot::Snap;");
    expect(out).toContain("crate::standards::v1::run()");
    expect(out).toContain("crate::schema::x()");
    expect(out).not.toContain("crate::crate::");
    expect(out).toContain("other::y()");
    expect(out).toContain("self::schema::z()");
    expect(qualifyModuleHeads(text, new Set(["schema", "standards"]), new Set(["standards"]))).toContain("let _ = standards::v1::run();");
  });

  test("gates an applied carve: the mounted crates may not add an unresolved name to the pre-carve tree of HEAD", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-sbx-"));
    const artifact = "a/b/c";
    try {
      for (const [path, content] of Object.entries(weaveFiles(false))) { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), content); }
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const plan = await planArtifactCrates(repo, `${artifact}/📦️packages/🦀️rust`, { ...tiny });
      expect(plan.problems).toEqual([]);
      const { writes } = await carveWrites(plan);
      applyCarveWrites(repo, writes);
      const clean = await verifyAppliedLayout(repo, artifact, join(sandbox, "clean"));
      expect(clean.crates).toBeGreaterThan(1);
      expect(clean.baseCrates).toBe(0);
      expect(Object.values(clean.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      const broken = join(repo, artifact, "🚪️io/📝️text/🦀️.rs");
      writeFileSync(broken, readFileSync(broken, "utf8").replace("crate::schema::value()", "crate::schema::missing()"));
      const regressed = await verifyAppliedLayout(repo, artifact, join(sandbox, "regressed"));
      expect(Object.values(regressed.layout).some((row) => row.onlyCombined > 0)).toBe(true);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("lays out the hand-built crates of the pre-carve tree the same way, so only a name the working tree adds is reported", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-base-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-base-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      const manifest = (name: string): string => `[package]\nname = "${name}"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n`;
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust", "🧩️extra/📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", manifest("demo-x"));
      write("🦀️.rs", "//! 🧪️ facade\npub const A: u8 = 1;\n");
      write("🧩️extra/📦️packages/🦀️rust/Cargo.toml", manifest("demo-x-extra"));
      write("🧩️extra/🦀️.rs", "//! 🧩️ hand-built crate\nuse crate::MacroMade;\npub fn a() -> u8 { demo_x::A }\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const clean = await verifyAppliedLayout(repo, artifact, join(sandbox, "clean"));
      expect(clean.crates).toBe(1);
      expect(clean.baseCrates).toBe(1);
      expect(Object.values(clean.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      expect(Object.values(clean.layout).every((row) => row.base === row.combined && row.base > 0)).toBe(true);
      write("🧩️extra/🦀️.rs", "//! 🧩️ hand-built crate\nuse crate::MacroMade;\nuse crate::Added;\npub fn a() -> u8 { demo_x::A }\n");
      const regressed = await verifyAppliedLayout(repo, artifact, join(sandbox, "regressed"), "HEAD", false, 5);
      expect(regressed.layout.native!.onlyCombined).toBe(1);
      expect(regressed.layout.native!.sampleOnlyCombined[0]).toContain("Added");
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("copies the working-tree file that a healed pre-carve file mounts when the pre-carve tree lacks it, so the base still parses", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-target-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-target-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "//! 🧪️ facade\n#[path = \"🪟️views/🦀️.rs\"]\npub mod views;\n");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\npub mod tools {\n    #[path = \"../🛠️tools/🦀️.rs\"]\n    pub mod simulation;\n}\n");
      write("🪟️views/🛠️tools/🦀️.rs", "//! 🛠️ tools\npub const B: u8 = 2;\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\n#[path = \"🛠️tools\"]\npub mod tools {\n    #[path = \"🦀️.rs\"]\n    pub mod simulation;\n}\n#[path = \"🆕️added/🦀️.rs\"]\npub mod added;\n");
      write("🪟️views/🆕️added/🦀️.rs", "//! 🆕️ added\npub const C: u8 = 3;\n");
      const report = await verifyAppliedLayout(repo, artifact, join(sandbox, "target"), "HEAD", false, 5);
      expect(report.healedBase.map((path) => path.split("/").slice(-2).join("/")).sort()).toEqual(["🆕️added/🦀️.rs", "🪟️views/🦀️.rs"]);
      expect(Object.values(report.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      expect(Object.values(report.layout).every((row) => row.base === row.combined)).toBe(true);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("leaves a binary-only package of the artifact out of the layout differential", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-bin-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-bin-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust", "🐚️shell/📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "//! 🧪️ facade\npub const A: u8 = 1;\n");
      write("🐚️shell/📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x-shell"\nversion = "0.1.0"\n\n[[bin]]\nname = "demo-x-shell"\npath = "📦️bin.rs"\n');
      write("🐚️shell/📦️packages/🦀️rust/📦️bin.rs", "fn main() { println!(\"{}\", demo_x::A); }\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const report = await verifyAppliedLayout(repo, artifact, join(sandbox, "bin"));
      expect(report.crates).toBe(0);
      expect(report.baseCrates).toBe(0);
      expect(Object.values(report.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("lists the module files a root mounts with flat path attributes, relative to the root's own directory", () => {
    const text = ['#[path = "🏊️arena-laws/🦀️.rs"]', "mod arena_laws;", '#[path = "../shared/🦀️.rs"] mod shared;', '#[path = "🧪️tests"] mod tests { #[path = "x/🦀️.rs"] mod x; }', ""].join("\n");
    expect(mountedPathFiles("a/b/🦀️.rs", text)).toEqual(["a/b/🏊️arena-laws/🦀️.rs", "a/shared/🦀️.rs", "a/b/x/🦀️.rs"]);
    expect(mountedPathFiles("a/b/🦀️.rs", "pub mod plain;\n")).toEqual([]);
  });

  test("hoists the inner attributes and inner docs that sit behind an item to the head of the file and leaves a clean file alone", () => {
    const broken = ["use std::fmt::Debug;", "#![allow(dead_code)]", "//! 🧪️ facade", "#[cfg(test)] use std::fmt::Display;", "#![allow(unused_imports)]", "pub mod kit {", "    //! inner doc of an inline module", "}", ""].join("\n");
    expect(hoistInnerAttributes(broken)).toBe(["#![allow(dead_code)]", "//! 🧪️ facade", "#![allow(unused_imports)]", "use std::fmt::Debug;", "#[cfg(test)] use std::fmt::Display;", "pub mod kit {", "    //! inner doc of an inline module", "}", ""].join("\n"));
    const clean = ["//! 🧪️ facade", "#![allow(dead_code)]", "use std::fmt::Debug;", "pub const A: u8 = 1;", ""].join("\n");
    expect(hoistInnerAttributes(clean)).toBe(clean);
  });

  test("repairs a pre-carve root with a misplaced inner attribute from its own text, so a carve that rewrote the root is not compared against its own carved root", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-hoist-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-hoist-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "use std::fmt::Debug;\n#![allow(dead_code)]\n//! 🧪️ facade\npub mod kit {\n    pub fn a() {}\n}\nuse crate::kit::missing;\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      write("🦀️.rs", "//! 🧪️ facade\n#![allow(dead_code)]\nuse std::fmt::Debug;\npub mod kit {\n    pub fn a() {}\n}\nuse crate::kit::missing;\nuse crate::kit::added;\n");
      const report = await verifyAppliedLayout(repo, artifact, join(sandbox, "hoist"), "HEAD", false, 5);
      expect(report.healedBase.map((path) => path.split("/").slice(-1)[0])).toEqual(["🦀️.rs"]);
      expect(report.layout.native!.onlyCombined).toBe(1);
      expect(report.layout.native!.sampleOnlyCombined[0]).toContain("added");
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("heals a pre-carve file that rustc cannot parse with its working-tree text, so a repaired tree is still compared name for name", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-heal-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-heal-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "//! 🧪️ facade\n#[path = \"🧰️kit/🦀️.rs\"]\npub mod kit;\n#[path = \"🪟️views/🦀️.rs\"]\npub mod views;\nuse crate::Imported;\n");
      write("🧰️kit/🦀️.rs", "//! 🧰️ kit\nuse std::fmt::Debug;\n#![allow(dead_code)]\npub const A: u8 = 1;\n");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\npub mod tools {\n    #[path = \"../🛠️tools/🦀️.rs\"]\n    pub mod simulation;\n}\n");
      write("🪟️views/🛠️tools/🦀️.rs", "//! 🛠️ tools\npub const B: u8 = 2;\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const broken = await verifyAppliedLayout(repo, artifact, join(sandbox, "broken"));
      expect(broken.healedBase).toEqual([]);
      expect(Object.values(broken.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      write("🧰️kit/🦀️.rs", "//! 🧰️ kit\n#![allow(dead_code)]\nuse std::fmt::Debug;\npub const A: u8 = 1;\n");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\n#[path = \"🛠️tools\"]\npub mod tools {\n    #[path = \"🦀️.rs\"]\n    pub mod simulation;\n}\n");
      const repaired = await verifyAppliedLayout(repo, artifact, join(sandbox, "repaired"));
      expect(repaired.healedBase.map((path) => path.split("/").slice(-2).join("/")).sort()).toEqual(["🧰️kit/🦀️.rs", "🪟️views/🦀️.rs"]);
      expect(Object.values(repaired.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      expect(Object.values(repaired.layout).every((row) => row.base === row.combined)).toBe(true);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("keeps the largest size per path in a measure union and never counts the virtual root", () => {
    const union = new MeasureUnion(new Set(["mirror/🦀️.rs"]), { files: 1, bytes: 0, lines: 0 });
    const left = new Map([["a.rs", { bytes: 10, lines: 2 }], ["b.rs", { bytes: 5, lines: 1 }], ["mirror/🦀️.rs", { bytes: 99, lines: 9 }]]);
    const right = new Map([["a.rs", { bytes: 12, lines: 1 }], ["c.rs", { bytes: 7, lines: 3 }], ["mirror/🦀️.rs", { bytes: 77, lines: 7 }]]);
    union.add([left]);
    expect(union.totals()).toEqual({ files: 3, bytes: 15, lines: 3 });
    expect(union.peek([right, right])).toEqual({ files: 4, bytes: 24, lines: 6 });
    expect(union.totals()).toEqual({ files: 3, bytes: 15, lines: 3 });
    union.add([right]);
    expect(union.totals()).toEqual({ files: 4, bytes: 24, lines: 6 });
    expect(totalsFit(union.totals(), { files: 4, bytes: 24, lines: 6 })).toBe(true);
    expect(totalsFit(union.totals(), { files: 3, bytes: 24, lines: 6 })).toBe(false);
  });

  test("packs without re-reading merged closures that cannot fit, and plans exactly what a full re-measure plans", async () => {
    const names = ["a", "b", "c", "d", "e", "f"];
    const tree: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-pack"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-pack"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-pack",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": `//! root\n${names.map((name) => `#[path = "."]\npub mod ${name} {\n    #[path = "${name}/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n`).join("")}`,
      ...Object.fromEntries(names.map((name, index) => [`${name}/🦀️.rs`, `//! ${name}\npub fn ${name}_value() -> u8 { ${index} }\n`])),
    };
    const root = mkdtempSync(join(tmpdir(), "semio-pack-"));
    try {
      for (const [path, content] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const options = { target: { files: 3, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };
      const incremental = await planArtifactCrates(root, "📦️packages/🦀️rust", options);
      const exact = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...options, incrementalMeasure: false });
      const shape = (plan: typeof incremental): string[] => plan.crates.map((crate) => crate.attach.map((decl) => decl.eff.join("::")).sort().join("+")).sort();
      expect(shape(incremental)).toEqual(shape(exact));
      expect(shape(incremental).length).toBe(3);
      expect(incremental.packing.merged).toBe(exact.packing.merged);
      expect(incremental.packing.merged).toBeGreaterThan(0);
      expect(incremental.packing.pruned).toBeGreaterThan(0);
      expect(incremental.packing.measured).toBeLessThan(exact.packing.measured);
      expect(exact.packing.pruned).toBe(0);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rewrites carved crates into modules of one synthetic crate without touching macro paths", () => {
    const source = "use crate::a::b;\npub use dep_crate::x::y;\nmacro_rules! m { () => { $crate::keep::it() }; }\nfn f() { crate::z(); let _ = std::mem::drop(1); pub(crate) fn g() {} }\n";
    const rewritten = rewriteForCombined(source, "__c_me", new Map([["dep_crate", "__c_dep"]]));
    expect(rewritten).toContain("use crate::__c_me::a::b;");
    expect(rewritten).toContain("pub use crate::__c_dep::x::y;");
    expect(rewritten).toContain("$crate::keep::it()");
    expect(rewritten).toContain("crate::__c_me::z();");
    expect(rewritten).toContain("pub(crate) fn g()");
    expect(rewritten).toContain("std::mem::drop");
  });

  test("hosts moved root regions without importing one name twice or qualifying external use groups", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-units-"));
    const layout: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-units"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-units"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-units",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": [
        "//! root",
        "pub use crate::base::one;",
        "//#region Bridge",
        "pub use ext::Alpha;",
        "use ext::Gamma;",
        "use ext::schema::snapshot::{Edge, Node};",
        "pub struct Holder { pub alpha: Alpha, pub gamma: Gamma, pub edge: Edge, pub node: Node }",
        "//#endregion Bridge",
        "//#region Maker",
        "pub fn make() -> Holder { let _ = one(); Holder { alpha: Alpha, gamma: Gamma, edge: Edge, node: Node } }",
        "//#endregion Maker",
        '#[path = "."]',
        "pub mod base {",
        '    #[path = "🧱️base/🦀️.rs"]',
        "    mod component;",
        "    pub use component::*;",
        '    #[path = "🧱️base/🍃️leaf/🦀️.rs"]',
        "    pub mod leaf;",
        "}",
        '#[path = "."]',
        "pub mod schema {",
        '    #[path = "🧬️schema/🦀️.rs"]',
        "    mod component;",
        "    pub use component::*;",
        "}",
        '#[path = "."]',
        "pub mod io {",
        '    #[path = "🚪️io/🦀️.rs"]',
        "    mod component;",
        "    pub use component::*;",
        "}",
        "",
      ].join("\n"),
      "🧬️schema/🦀️.rs": "//! schema\npub fn hold() -> crate::Holder { crate::make() }\n",
      "🚪️io/🦀️.rs": "//! io\npub fn build() -> crate::Holder { crate::make() }\n",
      "🧱️base/🦀️.rs": "//! base\npub fn one() -> u8 { 1 }\n",
      "🧱️base/🍃️leaf/🦀️.rs": "//! leaf\npub fn two() -> u8 { 2 }\n",
    };
    try {
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      const { writes } = await carveWrites(plan);
      expect(plan.problems).toEqual([]);
      const units = writes.flatMap((write) => [...write.content.matchAll(/pub mod (artifact_items\w*) \{([\s\S]*?)\n\}\n\/\/#endregion/gu)].map((match) => match[2]!));
      expect(units.length).toBeGreaterThan(0);
      for (const unit of units) {
        expect(unit.match(/use ext::Gamma;/gu)?.length ?? 0).toBeLessThanOrEqual(1);
        expect(unit).not.toContain("{crate::");
        expect(unit).not.toMatch(/use ext::\{[^}]*crate::/u);
      }
      const maker = units.find((unit) => unit.includes("pub fn make"))!;
      expect(maker).not.toContain("pub use ext::Alpha;");
      expect(maker).toMatch(/use crate::\{[^}]*Alpha[^}]*\};/u);
      expect(maker).toMatch(/use ext::Gamma;/u);
      expect(maker).toMatch(/use crate::base::one;/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("drops only the re-export cascades of an unlinked external crate from the oracle diagnostics", () => {
    const entry = (code: string, message: string, label: string, sourceLine: string) => ({ code, message, label, file: "a.rs", line: 1, sourceLine });
    const kept = dropUnlinkedCascades([
      entry("E0433", "cannot find module or crate `semio_foo` in this scope", "use of unresolved module or unlinked crate `semio_foo`", "pub use semio_foo::schema::mutations::Op;"),
      entry("E0432", "unresolved import `crate::Op`", "no `Op` in `snapshot`", "pub use snapshot::Op;"),
      entry("E0432", "unresolved import `crate::Real`", "no `Real` in `snapshot`", "pub use snapshot::Real;"),
      entry("E0425", "cannot find type `Op` in this scope", "", "fn f() -> Op"),
    ]);
    expect(kept.map((item) => item.message)).toEqual(["cannot find module or crate `semio_foo` in this scope", "unresolved import `crate::Real`", "cannot find type `Op` in this scope"]);
  });

  test("counts a path through the extern prelude of an unlinked crate as external noise and keeps every other missing name", () => {
    expect(external("E0433\tcannot find `semio_framework_schema_registry` in the crate root\tx.rs\t")).toBe(false);
    expect(external("E0433\tcannot find module or crate `semio_foo` in this scope\tx.rs\tuse of unresolved module or unlinked crate `semio_foo`")).toBe(false);
    expect(external("E0433\tcannot find `schema` in `any`\tx.rs\t")).toBe(true);
    expect(external("E0432\tunresolved import `crate::Real`\tx.rs\t")).toBe(true);
  });

  test("drops the unresolved imports that go through a module which glob-re-exports an unlinked external crate", () => {
    const entry = (code: string, message: string, label: string, sourceLine: string, file = "a.rs") => ({ code, message, label, file, line: 1, sourceLine });
    const entries = [
      entry("E0432", "unresolved import `semio_ext`", "use of unresolved module or unlinked crate `semio_ext`", "pub use semio_ext::{self as graph, *};", "engine/🦀️.rs"),
      entry("E0432", "unresolved import `crate::__c_demo::editor::engine::BoardHost`", "no `BoardHost` in `editor::engine`", "use crate::editor::engine::BoardHost;"),
      entry("E0432", "unresolved imports `crate::__c_demo::editor::engine::BoardNaming`, `crate::__c_demo::editor::engine::BoardFillStage`", "", "use crate::editor::engine::{BoardNaming, BoardFillStage};"),
      entry("E0432", "unresolved imports `crate::__c_demo::editor::engine::BoardNaming`, `crate::__c_demo::editor::other::Real`", "", "use crate::editor::engine::BoardNaming;"),
      entry("E0432", "unresolved import `crate::__c_demo::editor::other::Real`", "", "use crate::editor::other::Real;"),
    ];
    expect(dropUnlinkedCascades(entries).length).toBe(4);
    const kept = dropUnlinkedCascades(entries, new Set(["editor::engine"]));
    expect(kept.map((item) => item.message)).toEqual(["unresolved imports `crate::__c_demo::editor::engine::BoardNaming`, `crate::__c_demo::editor::other::Real`", "unresolved import `crate::__c_demo::editor::other::Real`"]);
    expect(plainModulePath(["crate", "__c_demo", "editor", "engine"])).toBe("editor::engine");
    const root = mkdtempSync(join(tmpdir(), "glob-modules-"));
    try {
      mkdirSync(join(root, "editor/engine"), { recursive: true });
      writeFileSync(join(root, "root.rs"), '#[path = "editor/🦀️.rs"]\npub mod editor;\n');
      writeFileSync(join(root, "editor/🦀️.rs"), '#[path = "engine/🦀️.rs"]\npub mod engine;\n');
      writeFileSync(join(root, "editor/engine/🦀️.rs"), "pub use semio_ext::{self as graph, *};\n");
      const found = unlinkedGlobModules(root, join(root, "root.rs"), [entry("E0432", "unresolved import `semio_ext`", "use of unresolved module or unlinked crate `semio_ext`", "pub use semio_ext::{self as graph, *};", "editor/engine/🦀️.rs")]);
      expect([...found]).toEqual(["editor::engine"]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("copies a root alias of an external crate into every crate that reaches through it with a longer path", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-alias-"));
    const layout: Record<string, string> = {
      "Cargo.toml": files["Cargo.toml"]!,
      "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-alias"\nversion = "0.1.0"\n',
      "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-alias"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
      "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-alias",{snapshotSqliteTests:[]});\n',
      "🦀️.rs": ["//! root", "use external_kernel as kernel;", '#[path = "."]', "pub mod schema {", '    #[path = "🧬️schema/🦀️.rs"]', "    mod component;", "    pub use component::*;", "}", '#[path = "."]', "pub mod io {", '    #[path = "🚪️io/🦀️.rs"]', "    mod component;", "    pub use component::*;", "}", ""].join("\n"),
      "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { crate::kernel::default_value() }\n",
      "🚪️io/🦀️.rs": "//! io\npub fn other() -> u8 { crate::kernel::other_value() }\n",
    };
    try {
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      expect(plan.problems).toEqual([]);
      expect(plan.crates.length).toBe(2);
      const { writes } = await carveWrites(plan);
      for (const crate of plan.crates) expect(writes.find((write) => write.path === `${crate.packageDir.replace("/📦️packages/🦀️rust", "")}/📦️packages/🦀️rust/🦀️.rs`)!.content).toContain("use external_kernel as kernel;");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("measures the rewritten facade against the manifest directory of the artifact package", async () => {
    const root = fixture();
    try {
      writeFileSync(join(root, "data.txt"), "data");
      writeFileSync(join(root, "🦀️.rs"), `${files["🦀️.rs"]}\n#[cfg(test)]\nmod probe { pub const DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data.txt")); }\n`);
      const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny });
      const { writes } = await carveWrites(plan);
      expect(writes.find((write) => write.path === "🦀️.rs")!.content).toContain("mod probe");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("never qualifies the entries of a use group that continues a path prefix", () => {
    const modules = new Set(["schema", "io"]);
    const text = "use ext::{schema::snapshot::Flow, schema::mutations::Op};\nuse ext::schema::snapshot::{Edge, Node};\nuse {schema::Alone, io::Other};\nfn f() { schema::run(); }\n";
    const out = qualifyModuleHeads(text, modules);
    expect(out).toContain("use ext::{schema::snapshot::Flow, schema::mutations::Op};");
    expect(out).toContain("use ext::schema::snapshot::{Edge, Node};");
    expect(out).toContain("use {crate::schema::Alone, crate::io::Other};");
    expect(out).toContain("fn f() { crate::schema::run(); }");
  });

  test("absolutizes super paths of a detached test and widens a pub(super) restriction to pub(crate)", () => {
    const source = "use super::*;\npub(super) fn helper() {}\nmod inner { use super::super::Thing; pub(super) fn deep() {} }\n";
    const rewritten = absolutizeSuperPaths(source, ["editor", "procedure", "unit_tests"]);
    expect(rewritten).toContain("use crate::editor::procedure::*;");
    expect(rewritten).toContain("pub(crate) fn helper()");
    expect(rewritten).toContain("use crate::editor::procedure::Thing;");
    expect(rewritten).toContain("pub(super) fn deep()");
    expect(rewritten).not.toContain("pub(in ");
  });

  test("counts the private component module of a host as one super level when it absolutizes the paths of a detached test", () => {
    const rewritten = absolutizeSuperPaths("use super::*;\nfn t() { super::super::topology::run(); super::super::super::sibling::run(); }\n", ["schema", "inferences", "component", "tests"]);
    expect(rewritten).toContain("use crate::schema::inferences::*;");
    expect(rewritten).toContain("crate::schema::inferences::topology::run()");
    expect(rewritten).toContain("crate::schema::sibling::run()");
  });

  test("removing a mount takes its outer doc comment along and leaves region markers and plain comments alone", () => {
    const source = "//#region R\n/// doc one\n/// doc two\n#[cfg(test)]\nmod tests;\n//#endregion R\n// plain\n#[cfg(test)]\nmod other;\n";
    const mount = source.indexOf("#[cfg(test)]\nmod tests;");
    const cut = rustOuterDocStart(source, mount);
    expect(source.slice(0, cut)).toBe("//#region R\n");
    const plain = source.indexOf("#[cfg(test)]\nmod other;");
    expect(rustOuterDocStart(source, plain)).toBe(plain);
    expect(rustOuterDocStart("x /// inline\nmod a;", 14)).toBe(14);
  });

  test("demotes an outer doc comment that no longer documents an item and keeps every other one", () => {
    const source = "/// kept: documents the fn\n//#region R\npub fn a() {}\n/// orphan one\n/// orphan two\n//#region E\n\n//#endregion E\n";
    const fixed = demoteDanglingOuterDocs(source);
    expect(fixed).toContain("/// kept: documents the fn");
    expect(fixed).toContain("// orphan one\n// orphan two");
    expect(demoteDanglingOuterDocs("mod m {\n    /// orphan in block\n}\n")).toContain("    // orphan in block");
    expect(demoteDanglingOuterDocs("/// doc\n#[cfg(test)]\nmod t;\n")).toBe("/// doc\n#[cfg(test)]\nmod t;\n");
  });
});
