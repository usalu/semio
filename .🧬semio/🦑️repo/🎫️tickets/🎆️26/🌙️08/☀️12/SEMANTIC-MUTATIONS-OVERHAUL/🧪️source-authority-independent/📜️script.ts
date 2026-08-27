import { mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";

//#region 🧪️ActualSourceAuthority
const workspace = process.cwd();
const productionPath = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs");
const source = readFileSync(productionPath, "utf8");
const start = source.indexOf("//#region 🔖️MutationSourceAuthority");
const marker = "//#endregion 🔖️MutationSourceAuthority";
const end = source.indexOf(marker, start);
if (start < 0 || end < start) throw new Error("Missing actual source-authority region");
const region = source.slice(start, end + marker.length);
const run = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const rust = join(run, "🦀️actual-authority.rs");
writeFileSync(rust, [
  "use std::{fs, path::{Component, Path, PathBuf}};",
  region,
  "fn main() {",
  "let arguments: Vec<String> = std::env::args().collect();",
  "let result = mutation_source_authority(Path::new(&arguments[1]), Path::new(&arguments[2]));",
  "let output = match result {",
  'Ok(facts) => serde_json::json!({"accepted": true, "workspace": facts.workspace_root.display().to_string(), "owner": facts.owner}),',
  'Err(error) => serde_json::json!({"accepted": false, "error": error}),',
  "};",
  'println!("{}", output);',
  "}",
].join("\n"));
const binary = join(run, process.platform === "win32" ? "authority.exe" : "authority");
const library = "serde_json-0caf27179e7b9139";
const externs = ["rlib", "rmeta"].flatMap((extension) => ["--extern", "serde_json=" + join(workspace, "target/debug/deps", "lib" + library + "." + extension)]);
const compile = spawnSync("rustc", ["--edition=2021", "--crate-name", "source_authority_probe", rust, "-L", "dependency=" + join(workspace, "target/debug/deps"), ...externs, "-o", binary], { encoding: "utf8", timeout: 60_000 });
writeFileSync(join(run, "compiler.stdout.log"), compile.stdout ?? "");
writeFileSync(join(run, "compiler.stderr.log"), compile.stderr ?? "");
if (compile.status !== 0) throw new Error("Actual source-authority compiler failure: " + compile.stderr);
const vectors = JSON.parse(readFileSync(join(import.meta.dir, "🔣️vectors.json"), "utf8")) as { owner: string; cases: { name: string; accepted: boolean }[] };
const taxonomy = { fileKinds: { rust: { emoji: "🦀️", extensionChains: [".rs"] }, json: { emoji: "🔣️", extensionChains: [".json"] } }, mutationComponentFileKindId: "rust", mutationDescriptorFileKindId: "json", semanticCollections: { "🧬️mutations": { kind: "mutation" } } };
const manifest = { metadata: { semio: { taxonomy: "authority/🔣️taxonomy.json" } } };
const write = (path: string, value: string | object): void => {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, typeof value === "string" ? value : JSON.stringify(value));
};
const directoryLink = (target: string, path: string): void => symlinkSync(target, path, process.platform === "win32" ? "junction" : "dir");
const results: unknown[] = [];
let mismatches = 0;
for (const vector of vectors.cases) {
  const root = join(run, vector.name);
  const outer = join(root, "outer");
  const nested = vector.name.startsWith("nested-");
  const domain = nested ? join(outer, "inner") : join(outer, "workspace");
  const owner = join(domain, vectors.owner);
  const sourcePath = join(owner, "🦀️.rs");
  write(sourcePath, "pub struct InsertPage;\n");
  write(join(owner, "🔣️.json"), { schemaVersion: 1, owner: nested ? "inner/" + vectors.owner : vectors.owner, semanticKind: "insert-page", displayName: "Insert Page", emoji: "➕️", aggregateVariant: "InsertPage", payloadSchema: "🦀️.rs#InsertPage", textOpcode: null, binaryTag: null, invertibility: "explicit-mutation", diffParticipation: "apply-only", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust"] });
  write(join(nested ? outer : domain, "nx.json"), {});
  write(join(nested ? outer : domain, "📋️project.json"), manifest);
  write(join(nested ? outer : domain, "authority/🔣️taxonomy.json"), taxonomy);
  let compilerSource = sourcePath;
  if (vector.name === "relative-parent") {
    mkdirSync(join(domain, "consumer"));
    compilerSource = "consumer/../" + vectors.owner + "/🦀️.rs";
  }
  if (vector.name === "symlink-parent-erasure") {
    directoryLink(join(domain, "domain"), join(domain, "alias"));
    compilerSource = "alias/../" + vectors.owner + "/🦀️.rs";
  }
  if (vector.name === "file-parent-erasure") {
    write(join(domain, "not-a-directory"), "not a directory");
    compilerSource = "not-a-directory/../" + vectors.owner + "/🦀️.rs";
  }
  if (vector.name === "workspace-ancestor-symlink") {
    directoryLink(outer, join(root, "linked-outer"));
    compilerSource = join(root, "linked-outer/workspace", vectors.owner, "🦀️.rs");
  }
  if (nested) {
    if (vector.name === "nested-symlink-nx") symlinkSync(join(outer, "nx.json"), join(domain, "nx.json"), "file");
    else write(join(domain, "nx.json"), {});
    if (vector.name === "nested-symlink-project") symlinkSync(join(outer, "📋️project.json"), join(domain, "📋️project.json"), "file");
    if (vector.name === "nested-symlink-nx") write(join(domain, "📋️project.json"), manifest);
  }
  const runtime = spawnSync(binary, [compilerSource, domain], { encoding: "utf8", timeout: 30_000 });
  writeFileSync(join(root, "runtime.stdout.log"), runtime.stdout ?? "");
  writeFileSync(join(root, "runtime.stderr.log"), runtime.stderr ?? "");
  if (runtime.status !== 0) throw new Error(vector.name + ": runtime failed: " + runtime.stderr);
  const result = JSON.parse(runtime.stdout);
  if (result.accepted !== vector.accepted) mismatches += 1;
  if (vector.accepted && (result.workspace !== domain || result.owner !== vectors.owner)) throw new Error(vector.name + ": accepted source proof has wrong identity");
  results.push({ name: vector.name, expected: vector.accepted, ...result });
  console.log("[DEBUG] " + JSON.stringify(results.at(-1)));
}
const evidence = { productionPath, regionSha256: createHash("sha256").update(region).digest("hex"), sourceUnchanged: readFileSync(productionPath, "utf8") === source, mismatches, results };
writeFileSync(join(run, "🔣️results.json"), JSON.stringify(evidence, null, 2));
console.log("[DEBUG] source-authority cases=" + results.length + " mismatches=" + mismatches + " region=" + evidence.regionSha256 + " artifacts=" + run);
process.exitCode = mismatches === 0 ? 0 : 1;
//#endregion 🧪️ActualSourceAuthority
