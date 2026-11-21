import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const scratch = resolve(import.meta.dir, "🗑️generated");
const owner = resolve("🧰️framework/🔨️modules/🧬️schema/📇️registry");
const build = join(scratch, "native-mcp/debug/build");
const crate = "semio_framework_schema_registry";
const library = join(scratch, `lib${crate}.rlib`);
const search = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).flatMap(entry =>
  readdirSync(join(build, entry.name)).map(variant => join(entry.name, variant, "out")).filter(path => existsSync(join(build, path))))
  .flatMap(path => ["-L", `dependency=${path}`]);
function prebuilt(pkg: string, crate: string, dependencyOf?: string[]) {
  const parent = dependencyOf?.[1].slice(dependencyOf[1].indexOf("=") + 1).split(/[\\/]/u);
  const parentCrate = dependencyOf?.[1].split("=", 1)[0];
  const value = parent && readFileSync(join(build, parent[0], parent[1], "fingerprint", `lib-${parentCrate}.json`), "utf8")
    .match(new RegExp(`"${crate}",false,(\\d+)`, "u"))?.[1];
  if (parent && !value) throw Error(`Missing retained dependency binding for ${crate}`);
  const identity = value && Buffer.from(BigInt(value).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
  const file = readdirSync(join(build, pkg)).flatMap(variant => {
    const stem = join(pkg, variant, "out", `lib${crate}-${variant}`);
    const fingerprint = join(build, pkg, variant, "fingerprint", `lib-${crate}`);
    if (identity && (!existsSync(fingerprint) || readFileSync(fingerprint, "utf8").trim() !== identity)) return [];
    return [".rmeta", ".rlib"].every(extension => existsSync(join(build, stem + extension))) ? [stem] : [];
  }).sort((a, b) => statSync(join(build, b + ".rlib")).mtimeMs - statSync(join(build, a + ".rlib")).mtimeMs)[0];
  if (!file) throw Error(`Missing prebuilt test dependency ${crate}`);
  return ["--extern", `${crate}=${file}.rmeta`, "--extern", `${crate}=${file}.rlib`];
}
const list = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (list.exitCode) process.exit(list.exitCode);
const rustup = JSON.parse(list.stdout.toString()).find((item: { id: string }) => item.id === "rustup")?.command;
const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust compiler unavailable");
function rustc(args: string[]) {
  const result = Bun.spawnSync([rustup, "run", toolchain, "rustc", "--edition=2024", "-Cdebuginfo=0", ...args], { cwd: build, stdout: "inherit", stderr: "inherit" });
  if (result.exitCode) process.exit(result.exitCode);
}
if (process.argv.includes("--actions")) {
  const source = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🦀️.rs");
  const anchor = prebuilt("semio-plugin-window-kits", "semio_plugin_window_kits");
  const [, file] = anchor[1].split("=");
  const [pkg, variant] = file.split(/[\\/]/u);
  const fingerprint = readFileSync(join(build, pkg, variant, "fingerprint/lib-semio_plugin_window_kits.json"), "utf8");
  const bindings = Array.from(fingerprint.matchAll(/"([a-z_0-9]+)",false,(\d+)/gu), ([, dependency, value]) => {
    const depPkg = dependency === "serde_json" ? "serde_json" : dependency.replaceAll("_", "-");
    const identity = Buffer.from(BigInt(value).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
    const depVariant = readdirSync(join(build, depPkg)).find(candidate => {
      const entry = join(build, depPkg, candidate, "fingerprint", `lib-${dependency}`);
      return existsSync(entry) && readFileSync(entry, "utf8").trim() === identity;
    });
    if (!depVariant) throw Error(`Missing exact linked crate ${dependency}`);
    const isWindows = process.platform === "win32";
    const stem = join(depPkg, depVariant, "out", `${isWindows && dependency.endsWith("_derive") ? "" : "lib"}${dependency}-${depVariant}`);
    const extensions = [".rmeta", ".rlib", process.platform === "darwin" ? ".dylib" : isWindows ? ".dll" : ".so"];
    return extensions.filter(extension => existsSync(join(build, stem + extension))).flatMap(extension => ["--extern", `${dependency}=${stem}${extension}`]);
  }).flat();
  rustc(["--emit=metadata", "--crate-type=rlib", "--crate-name", "semio_plugin_window_kits", source, "-o", join(scratch, "mcp-window-kits.rmeta"), ...bindings, ...search]);
  console.log("[DEBUG] Actual window-kits production source typechecked against its retained dependency graph");
  process.exit(0);
}
if (process.argv.includes("--examples")) {
  const source = readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🏗️surface-builders/🦀️.rs", "utf8");
  const helpers = source.slice(source.indexOf("type SubsetExampleKey"));
  if (!helpers.includes("fn subset_examples(")) throw Error("Missing production example registry");
  type Row = { dialect: { artifactKind: string; standard: string; subset: string }; examples: string[] };
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "declaration-example-cache.json"), "utf8")) as { registrations: Row[]; lookups: Row[] };
  const input = join(scratch, "declaration-example-rust.rs");
  const output = join(scratch, "declaration-example-rust" + (process.platform === "win32" ? ".exe" : ""));
  const dialect = ({ artifactKind, standard, subset }: Row["dialect"]) =>
    `Dialect {artifact_kind:${JSON.stringify(artifactKind)},standard:StandardId(${JSON.stringify(standard)}),subset:SubsetId(${JSON.stringify(subset)})}`;
  const list = (values: string[]) => `&[${values.map(value => JSON.stringify(value)).join(", ")}]`;
  writeFileSync(input, `use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_plugin_app_builder::ExampleSource;
use semio_ui_locale::LocalizedLabel;
mod declared_examples {
    use super::*;
    use std::collections::BTreeMap;
    ${helpers}
    pub fn verify(dialect: Dialect, expected: &[&str]) {
        assert_eq!(subset_examples(&dialect).iter().map(ExampleSource::id).collect::<Vec<_>>(), expected);
    }
}
fn register(dialect: Dialect, ids: &[&str]) {
    let examples = ids.iter().map(|id| ExampleSource::new(*id, LocalizedLabel::native(id,id), "{}", "file")).collect::<Vec<_>>();
    declared_examples::remember_subset_examples(dialect, &examples);
}
fn main() {
    ${fixture.registrations.map(row => `register(${dialect(row.dialect)},${list(row.examples)});`).join("\n    ")}
    ${fixture.lookups.map(row => `declared_examples::verify(${dialect(row.dialect)},${list(row.examples)});`).join("\n    ")}
    println!("[DEBUG] actual subset example registry preserves full identity and authored example order");
}
`);
  const builder = prebuilt("semio-plugin-app-builder", "semio_plugin_app_builder");
  rustc(["--crate-name", "declaration_example_rust", input, "-o", output,
    ...prebuilt("semio-framework-artifact-reference", "semio_framework_artifact_reference"),
    ...builder, ...prebuilt("semio-ui-locale", "semio_ui_locale", builder), ...search]);
  process.exit(Bun.spawnSync([output], { stdout: "inherit", stderr: "inherit" }).exitCode);
}
if (process.argv.includes("--languages")) {
  const source = resolve("🧰️framework/🔨️modules/🗣️dsl/🗣️idiom/🦀️.rs");
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "declaration-language-registry.json"), "utf8"));
  const input = join(scratch, "declaration-language-rust.rs");
  const output = join(scratch, "declaration-language-rust" + (process.platform === "win32" ? ".exe" : ""));
  const batches = fixture.batches.map((batch: { languages: { id: string; role: string; extension: string }[]; accepted: boolean }) =>
    `assert_eq!(idiom::register_languages(vec![${batch.languages.map(row => `spec(${JSON.stringify(row.id)}, LanguageRole::${row.role}, ${JSON.stringify(row.extension)})`).join(", ")}]).is_ok(), ${batch.accepted});`);
  writeFileSync(input, `#![allow(dead_code)]
use semio_framework_dsl::{grammar, TextError, TextSpan, TokenClass};
#[path = ${JSON.stringify(source)}] mod idiom;
use idiom::{LanguageRole, LanguageSpec};
fn spec(id: &'static str, role: LanguageRole, extension: &'static str) -> LanguageSpec {
    LanguageSpec { id, extension: Some(extension), role, grammar: None, grammar_path: None,
        protocol: None, protocol_path: None, hooks: idiom::passthrough_hooks(id) }
}
fn main() {
    ${batches.join("\n    ")}
    for id in [${fixture.sortedLanguageIds.map((id: string) => JSON.stringify(id)).join(", ")}] {
        assert_eq!(idiom::language(id).unwrap().id, id);
    }
    assert!(idiom::language("note.ops").is_none());
    assert!(idiom::language("wfc.config").is_none());
    assert_eq!(idiom::language_for_extension(".note").unwrap().id, "note.document");
    assert_eq!(idiom::language_for_role_extension(LanguageRole::Document, "bitmap").unwrap().id, "wfc.bitmap");
    println!("[DEBUG] authored language declaration batches preserve exact IDs and conflict atomicity");
}
`);
  rustc(["--crate-name", "declaration_language_rust", input, "-o", output, ...prebuilt("semio-framework-dsl", "semio_framework_dsl"), ...prebuilt("semio-framework-value", "semio_framework_value"), ...search]);
  process.exit(Bun.spawnSync([output], { stdout: "inherit", stderr: "inherit" }).exitCode);
}

rustc(["--crate-type=rlib", "--crate-name", crate, join(owner, "📦️packages/🦀️rust/🦀️.rs"), "-o", library]);
for (const test of ["📇️catalog", "🧱️neutrality"]) {
  const name = test === "📇️catalog" ? "schema_catalog_rust" : "schema_neutrality_rust";
  const output = join(scratch, name + (process.platform === "win32" ? ".exe" : ""));
  rustc(["--test", "--crate-name", name, join(owner, "🧪️tests", test, "🦀️.rs"), "-o", output, "--extern", `${crate}=${library}`, ...prebuilt("serde_json", "serde_json"), ...search]);
  const observed = Bun.spawnSync([output], { stdout: "inherit", stderr: "inherit" });
  if (observed.exitCode) process.exit(observed.exitCode);
}
