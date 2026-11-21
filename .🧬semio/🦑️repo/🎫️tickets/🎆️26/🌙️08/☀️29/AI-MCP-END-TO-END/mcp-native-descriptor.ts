import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";

const workspace = process.cwd();
const owner = process.argv[2];
const owners = new Map([["dag", "🕸️dag"], ["reasoning", "💡️reasoning"], ["remodel", "📸️remodel"], ["trinity", "🔱️trinity"], ["wfc", "🀄️wfc"]]);
const directory = owners.get(owner);
if (!directory || process.argv.length !== 3) throw Error("One known component owner is required");

const generated = join(import.meta.dir, "🗑️generated");
const scratch = join(generated, "nd", owner);
const target = join(generated, "native-mcp");
const output = join(generated, "descriptor-refresh", owner);
const name = "mcp-native-descriptor";
const framework = "🧰️framework";
const plugin = framework + "/🛍️products/💻️os/🔨️modules/🔌️plugin";
const dependencies = {
  "semio-framework-async": framework + "/🔨️modules/⏳️async",
  "semio-framework-value": framework + "/🔨️modules/🌱️value",
  "semio-framework-pack-json": framework + "/🔨️modules/🎒️pack/🔤️json",
  "semio-os-kernel": framework + "/🛍️products/💻️os",
  "semio-plugin": plugin,
  "semio-plugin-describe": plugin + "/🖨️describe",
  ["semio-hub-" + owner]: "🌎️hub/🧩️compositions/" + directory,
};
const rows = Object.entries(dependencies).map(([key, path]) =>
  key + " = { path = " + JSON.stringify(resolve(workspace, path, "📦️packages", "🦀️rust")) + " }");
const source = JSON.stringify(join(import.meta.dir, name + ".rs"));
const manifest = join(scratch, "Cargo.toml");
mkdirSync(scratch, { recursive: true });
writeFileSync(join(scratch, "Cargo.lock"), readFileSync(resolve(workspace, "🌎️hub/Cargo.lock")));
writeFileSync(manifest, [
  "[workspace]", 'resolver = "2"', "[package]", 'name = "' + name + '"', 'version = "0.0.0"', 'edition = "2021"',
  "[[bin]]", 'name = "' + name + '"', "path = " + source, "[features]", owner + " = []", "[dependencies]", ...rows, "",
].join("\n"));

const registry = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (registry.exitCode) process.exit(registry.exitCode);
const rustup = JSON.parse(registry.stdout.toString()).find((row: { id: string }) => row.id === "rustup")?.command;
const toolchain = readFileSync(resolve(workspace, "rust-toolchain.toml"), "utf8").match(/^channel\s*=\s*"([^"]+)"/m)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust toolchain required");
const rustc = Bun.spawnSync([rustup, "which", "--toolchain", toolchain, "rustc"], { stderr: "inherit" });
if (rustc.exitCode) process.exit(rustc.exitCode);
const environment: NodeJS.ProcessEnv = {
  ...process.env,
  SEMIO_BUILD_MODE: "prototype",
  CARGO_HOME: resolve(workspace, ".cache/b/r/m/cargo/home"),
  CARGO_TARGET_DIR: target,
  CARGO_BUILD_BUILD_DIR: target,
  CARGO_INCREMENTAL: "0",
  CARGO_PROFILE_DEV_DEBUG: "0",
  RUSTC: rustc.stdout.toString().trim(),
  RUSTC_WRAPPER: "",
  RUSTFLAGS: "-Zthreads=1",
};
delete environment.CARGO_BUILD_TARGET_DIR;
delete environment.CARGO_ENCODED_RUSTFLAGS;
delete environment.RUSTC_WORKSPACE_WRAPPER;
const cargo = Bun.spawn([rustup, "run", toolchain, "cargo", "build", "--offline", "--manifest-path", manifest, "--features", owner, "-j", "1"], {
  cwd: workspace, env: environment, stdout: "inherit", stderr: "inherit",
});
for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => cargo.kill(signal));
const built = await cargo.exited;
if (built) process.exit(built);
const executable = join(target, "debug", name + (process.platform === "win32" ? ".exe" : ""));
const producer = Bun.spawn([executable, owner, output], { stdout: "inherit", stderr: "inherit" });
for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => producer.kill(signal));
const emitted = await producer.exited;
if (emitted) process.exit(emitted);

const pair = verifySemanticDescriptorPairBytesV1(readFileSync(join(output, "🔣️.json")), readFileSync(join(output, "🛂️.descriptor.semio")));
if (pair.pluginId !== owner || pair.packageId !== "semio:" + owner) throw Error("Descriptor producer owner differs from the request");
console.log("[DEBUG] emitted verified native source pair " + owner + " " + pair.semanticSha256 + " at " + output);
