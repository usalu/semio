import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join, resolve } from "node:path";

//#region 🧭️Sources
const workspace = process.cwd();
const artifact = join(workspace, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt");
const schema = join(artifact, "🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema");
const roots = ["📸️snapshot", "🔺️diff", "🧬️mutations", "🔨️modules/🧬️mutation-support"].map((path) => join(schema, path));
const run = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const compiler = process.env.RUSTC ?? "rustc";
const fingerprint = (): string => {
  const records: { path: string; sha256: string }[] = [];
  const walk = (path: string): void => {
    const state = lstatSync(path);
    if (state.isSymbolicLink()) throw new Error(`No-follow source guard: ${path}`);
    if (state.isDirectory()) for (const name of readdirSync(path).sort()) walk(join(path, name));
    else if (state.isFile()) records.push({ path, sha256: createHash("sha256").update(readFileSync(path)).digest("hex") });
  };
  roots.forEach(walk);
  return createHash("sha256").update(JSON.stringify(records)).digest("hex");
};
const before = fingerprint();
const mount = (name: string, path: string): string => `#[path = ${JSON.stringify(join(schema, path))}] pub mod ${name};`;
const moduleWithFacets = (name: string, directory: string, extra = ""): string => `pub mod ${name} {
${mount("component", `${directory}/🦀️component.rs`)}
pub use component::*;
${mount("text", `${directory}/📝️text/🦀️component.rs`)}
${mount("binary", `${directory}/💾️binary/🦀️component.rs`)}
${extra}
}`;
const leafMounts = [["set_trailing_newline", "✏️set-trailing-newline"], ["set_line_ending", "✏️set-line-ending"], ["insert_line", "📥️insert-line"], ["remove_line", "🗑️remove-line"], ["set_line", "✏️set-line"]].map(([name, directory]) => mount(name!, `🧬️mutations/${directory}/🦀️component.rs`)).join("\n");
const artifactSource = readFileSync(join(artifact, "🦀️component.rs"), "utf8");
const schemaConstant = artifactSource.match(/pub const STDIO_TXT_DOCUMENT_SCHEMA: &str = "[^"]+";/u)?.[0];
if (!schemaConstant) throw new Error("Missing production TXT schema constant");
const source = `extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
pub mod artifacts { pub mod txt {
${schemaConstant}
pub use schema::{snapshot::TxtSnapshot, diff::TxtDiff, mutations::TxtMutation};
pub mod schema {
${moduleWithFacets("snapshot", "📸️snapshot")}
${moduleWithFacets("diff", "🔺️diff")}
${moduleWithFacets("mutations", "🧬️mutations", leafMounts)}
${mount("mutation_support", "🔨️modules/🧬️mutation-support/🦀️component.rs")}
}
}}
`;
const fixture = join(run, "🦀️component.rs");
writeFileSync(fixture, source);
//#endregion 🧭️Sources

//#region 🧪️Execution
const dependencies = join(workspace, "target/debug/deps");
const exactDependencies: Record<string, string> = {
  semio_framework_os_kernel: "libsemio_framework_os_kernel.rlib",
  semio_framework_schema: "libsemio_framework_schema-29dee7f327975e5f.rlib",
  semio_framework_async_macros: "libsemio_framework_async_macros-4945efd0a40a2b35.dylib",
  serde: "libserde-73de109b1e55818a.rlib",
  serde_json: "libserde_json-0caf27179e7b9139.rlib",
};
const externs = Object.entries(exactDependencies).flatMap(([name, file]) => {
  const path = join(dependencies, file);
  if (!existsSync(path)) throw new Error(`Missing checkpoint dependency ${path}`);
  const metadata = path.replace(/\.rlib$/u, ".rmeta");
  return metadata !== path && existsSync(metadata) ? ["--extern", `${name}=${path}`, "--extern", `${name}=${metadata}`] : ["--extern", `${name}=${path}`];
});
const binary = join(run, process.platform === "win32" ? "txt-leaf-tests.exe" : "txt-leaf-tests");
const args = ["--crate-name", "txt_production_leaf_runtime", "--edition", "2021", "--test", fixture, "-C", "debuginfo=0", "-L", `dependency=${dependencies}`, "-L", `native=${join(workspace, "target/debug/build/blake3-f1fb3a1b01038ea4/out")}`, ...externs, "-o", binary];
console.log(`[DEBUG] retained run: ${run}`);
console.log(`[DEBUG] source fingerprint before: ${before}`);
console.log(`[DEBUG] compiler command: ${JSON.stringify([compiler, ...args])}`);
console.log("[DEBUG] This diagnostic mounts production TXT sources against existing checkpoint dependencies; it is not a replacement for registered STDIO integration.");
const compiled = spawnSync(compiler, args, { cwd: resolve(workspace), encoding: "utf8", timeout: 180000, maxBuffer: 16 * 1024 * 1024 });
writeFileSync(join(run, "🧪️compiler.log"), `${compiled.stdout ?? ""}${compiled.stderr ?? ""}`);
console.log(`[DEBUG] compiler status=${compiled.status} signal=${compiled.signal ?? "none"} error=${compiled.error?.message ?? "none"}`);
if (compiled.status !== 0) {
  process.stderr.write(compiled.stderr ?? "");
  process.exit(1);
}
const after = fingerprint();
console.log(`[DEBUG] source fingerprint after: ${after}`);
if (before !== after) throw new Error("Production TXT source changed during compilation; diagnostic result is invalid");
const executed = spawnSync(binary, ["--nocapture"], { cwd: workspace, encoding: "utf8", timeout: 60000, maxBuffer: 16 * 1024 * 1024 });
writeFileSync(join(run, "🧪️tests.log"), `${executed.stdout ?? ""}${executed.stderr ?? ""}`);
process.stdout.write(executed.stdout ?? "");
process.stderr.write(executed.stderr ?? "");
console.log(`[DEBUG] test status=${executed.status} signal=${executed.signal ?? "none"} error=${executed.error?.message ?? "none"}`);
process.exit(executed.status === 0 ? 0 : 1);
//#endregion 🧪️Execution
