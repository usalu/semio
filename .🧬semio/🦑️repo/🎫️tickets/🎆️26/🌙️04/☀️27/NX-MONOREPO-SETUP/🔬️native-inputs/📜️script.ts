import assert from "node:assert/strict";
import { readFileSync, mkdirSync, writeFileSync, mkdtempSync, appendFileSync, symlinkSync, readdirSync, rmSync, renameSync, existsSync, lstatSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";
const here = dirname(fileURLToPath(import.meta.url)), require = createRequire(import.meta.url), workspace = process.cwd();
const root = mkdtempSync(join(dirname(here), "🗑️generated/native-inputs-"));
const cases = JSON.parse(readFileSync(join(here, "🧫️cases.json"), "utf8"));
assert.equal(require("jsonschema").validate(cases, JSON.parse(readFileSync(join(here, "🧬️schema.json"), "utf8"))).valid, true);
for (const [path, content] of Object.entries(cases.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { cacheInternals } = await import(pathToFileURL(join(workspace, library, "🟨️.mjs")).href);
const inputs = cacheInternals.projectInputs({ name: "fixture", targets: {} }, "domain/📦️packages/🦀️rust", root, new Map());
assert.ok(Array.isArray(inputs.nativeSources), "Cargo projects need separate native source inputs");
const match = (path: string) => {
  const positive = inputs.nativeSources.filter((item: unknown) => typeof item === "string" && !item.startsWith("!")).map((s: string) => s.replace("{workspaceRoot}/", "").replace("{projectRoot}/", "domain/📦️packages/🦀️rust/"));
  const negative = inputs.nativeSources.filter((item: unknown) => typeof item === "string" && item.startsWith("!")).map((s: string) => s.slice(1).replace("{workspaceRoot}/", "").replace("{projectRoot}/", "domain/📦️packages/🦀️rust/"));
  return require("minimatch").minimatch(path, "{" + positive.join(",") + "}") && !negative.some((pattern: string) => require("minimatch").minimatch(path, pattern));
};
for (const path of cases.included) assert.ok(match(path), `missing native input ${path}`);
for (const path of cases.excluded) assert.ok(!match(path), `unrelated native input ${path}`);
const depfile = join(root, "native.d"), rust = Bun.spawnSync(["rustc", "--edition=2021", "--crate-type=lib", "--crate-name=native_inputs", "--emit=dep-info=" + depfile, join(root, "domain/🦀️.rs")], { cwd: root, stdout: "pipe", stderr: "pipe" });
assert.equal(rust.exitCode, 0, rust.stderr.toString());
const dependencies = readFileSync(depfile, "utf8");
for (const path of cases.included.filter((s: string) => !s.endsWith("Cargo.toml"))) assert.ok(dependencies.includes(join(root, path)), `rustc did not consume ${path}`);
const entry = join(workspace, library, "⚡️caching/🦀️cargo/📜️script.ts"), actual = cacheInternals.relativeScriptInputs([entry], workspace);
const built = await require("esbuild").build({ entryPoints: [entry], bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
assert.deepEqual(actual, Object.keys(built.metafile.inputs).map((path) => "{workspaceRoot}/" + relative(workspace, resolve(path))).sort());
console.log("[DEBUG] Native input closure preserves rustc-consumed assets, excludes frontend and separate tests, and matches the esbuild command import oracle PASS");

if (process.argv.includes("cache")) {
  const copy = (path: string) => { const output = join(root, path); mkdirSync(dirname(output), { recursive: true }); writeFileSync(output, readFileSync(join(workspace, path))); };
  for (const path of [library + "/🟨️.mjs", library + "/⚡️caching/🔣️policy.json", ...actual.map((path: string) => path.replace("{workspaceRoot}/", ""))]) copy(path);
  writeFileSync(join(root, "Cargo.toml"), '[workspace]\nmembers=["domain/📦️packages/🦀️rust"]\nresolver="2"\n');
  writeFileSync(join(root, "Cargo.lock"), 'version = 4\n[[package]]\nname = "native-input-fixture"\nversion = "0.1.0"\n');
  writeFileSync(join(root, "nx.json"), JSON.stringify({ plugins: [{ plugin: "./" + library + "/🟨️.mjs" }], useDaemonProcess: false, cacheDirectory: ".nx/cache", maxCacheSize: "64MB" }));
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  writeFileSync(join(root, ".gitignore"), "node_modules\n.nx\ntarget\nstate\n**/dist\n*.log\n");
  writeFileSync(join(root, ".nxignore"), "!domain/**/*\n!" + library + "/**/*\n!Cargo.toml\n!Cargo.lock\n!package.json\n");
  mkdirSync(join(root, "state"));
  const script = join(root, library, "⚡️caching/🦀️cargo/📜️script.ts");
  appendFileSync(script, '\nif (import.meta.main) (await import("node:fs")).appendFileSync("state/executions", "native\\n");\n');
  const count = () => existsSync(join(root, "state/executions")) ? readFileSync(join(root, "state/executions"), "utf8").trim().split("\n").length : 0;
  const env: Record<string, string | undefined> = { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "true", NX_NATIVE_COMMAND_RUNNER: "false", NX_TUI: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), CARGO_TARGET_DIR: join(root, "target"), RUSTFLAGS: undefined, CARGO_ENCODED_RUSTFLAGS: undefined, REPO_ROOT: root, FORCE_COLOR: "0", NX_VERBOSE_LOGGING: "false" };
  for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SOCKET_DIR", "NX_DAEMON_SOCKET_DIR", "NX_FORCE_REUSE_CACHED_GRAPH", "NX_SKIP_NX_CACHE", "npm_lifecycle_event", "npm_lifecycle_script", "NO_COLOR"].includes(key)) delete env[key];
  const cli = require.resolve("nx/bin/nx.js"), rows: any[] = [];
  const run = async (scenario: string, expected: number, extra = {}) => {
    const child = Bun.spawn(["node", cli, "run", "native-input-fixture:build", "--output-style=static"], { cwd: root, env: { ...env, ...extra }, stdout: "pipe", stderr: "pipe" });
    const cancel = () => child.kill(); process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    let stdout, stderr, status;
    try { [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]); }
    finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
    writeFileSync(join(root, scenario + ".log"), stdout + stderr); assert.equal(status, 0, `${scenario}: inspect ${root}`);
    assert.equal(count(), expected, scenario); rows.push({ scenario, executions: count() }); console.log(`[DEBUG] Native Nx ${scenario}: executions=${count()}`);
  };
  await run("cold", 1); await run("warm", 1);
  writeFileSync(join(root, "domain/🟦️.ts"), "export const ui = 2;\n"); await run("frontend", 1);
  writeFileSync(join(root, "domain/🧪️tests/🦀️.rs"), "#[test] fn changed() { assert!(true); }\n"); await run("test-source", 1);
  writeFileSync(join(root, library, "⚡️caching/📜️script.ts"), "export const auditTest = 2;\n"); await run("tooling-test", 1);
  await run("variant-env", 1, { SEMIO_PLUGIN: "other", SEMIO_RENDERER: "other", SEMIO_TEST_BUDGET_MS: "77" });
  const output = join(root, "domain/📦️packages/🦀️rust/dist/build"), digest = () => Object.fromEntries(readdirSync(output).filter((path) => lstatSync(join(output, path)).isFile()).map((path) => [path, require("node:crypto").createHash("sha256").update(readFileSync(join(output, path))).digest("hex")]));
  const before = digest(); renameSync(join(root, "target"), join(root, "state/compiler-store")); rmSync(output, { recursive: true });
  try { await run("restore-without-compiler-store", 1); assert.deepEqual(digest(), before); }
  finally { if (existsSync(join(root, "target"))) rmSync(join(root, "target"), { recursive: true }); renameSync(join(root, "state/compiler-store"), join(root, "target")); }
  writeFileSync(join(root, "state/consumer.rs"), 'fn main() { println!("{}", std::str::from_utf8(native_input_fixture::DATA).unwrap()); }\n');
  const executable = join(root, "state/consumer" + (process.platform === "win32" ? ".exe" : ""));
  const consumer = Bun.spawnSync(["rustc", "--edition=2021", ...["rlib", "rmeta"].flatMap((extension) => ["--extern", "native_input_fixture=" + join(output, "libnative_input_fixture." + extension)]), join(root, "state/consumer.rs"), "-o", executable], { cwd: root, stdout: "pipe", stderr: "pipe" });
  assert.equal(consumer.exitCode, 0, consumer.stderr.toString());
  const consumed = Bun.spawnSync([executable], { stdout: "pipe", stderr: "pipe" }); assert.equal(consumed.exitCode, 0); assert.equal(consumed.stdout.toString().trim(), "shared-data");
  appendFileSync(join(root, "domain/🦀️.rs"), "pub const CHANGED: u32 = 2;\n"); await run("native-source", 2);
  writeFileSync(join(root, "domain/shared.bin"), "changed-data"); await run("embedded-asset", 3);
  await run("compiler-flags", 4, { RUSTFLAGS: "-Copt-level=1" });
  appendFileSync(join(root, "Cargo.toml"), "# compiler contract changed\n"); await run("toolchain-contract", 5);
  appendFileSync(join(root, library, "⚡️caching/📦️artifacts/🟦️.ts"), "\n"); await run("producer-implementation", 6);
  writeFileSync(join(root, "observations.json"), JSON.stringify(rows, null, 2));
  writeFileSync(join(dirname(here), "📓️native-input-restoration.md"), "# Native Input Invalidation and Restoration\n\nThe actual repository plugin and native producer, copied into an isolated ticket workspace, passed " + rows.length + " Nx scenarios on " + process.platform + "/" + process.arch + ". A fixture-only completion counter verified real producer executions. Unrelated frontend, separate test, audit/test implementation and variant environment changes reused the cached build. Native source, embedded asset, compiler flags, compiler contract and producer implementation changes executed the producer again. Deleted outputs restored byte-identically while the compiler store was absent; rustc independently linked a consumer against the restored library and its executable printed shared-data.\n\n| Scenario | Executions |\n| --- | ---: |\n" + rows.map((row) => "| " + row.scenario + " | " + row.executions + " |").join("\n") + "\n\nThis qualifies the isolated generic Cargo producer. Conservative real-package source fallbacks and browser-specific commands remain separately tracked.\n");
  console.log("[DEBUG] Native Nx invalidation, warm reuse, deletion restoration and independent linked consumer PASS");
}
