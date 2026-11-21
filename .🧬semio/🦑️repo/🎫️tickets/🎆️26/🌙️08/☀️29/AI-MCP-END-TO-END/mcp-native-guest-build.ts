import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const workspace = process.cwd();
const arguments_ = process.argv.slice(2);
const canonical = arguments_.includes("--canonical");
const registry = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (registry.exitCode) process.exit(registry.exitCode);
const rustup = JSON.parse(registry.stdout.toString()).find((row: { id: string }) => row.id === "rustup")?.command;
const toolchain = readFileSync(resolve(workspace, "rust-toolchain.toml"), "utf8").match(/^channel\s*=\s*"([^"]+)"/m)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust toolchain required");
const cargo = (args: readonly string[]) => [rustup, "run", toolchain, "cargo", ...args];
const compiler = Bun.spawnSync([rustup, "which", "--toolchain", toolchain, "rustc"], { stderr: "inherit" });
if (compiler.exitCode) process.exit(compiler.exitCode);
const generated = resolve(import.meta.dirname, "🗑️generated");
const target = resolve(generated, canonical ? "cg" : "native-guests");
const environment: NodeJS.ProcessEnv = {
  ...process.env,
  CARGO_HOME: resolve(workspace, ".cache/b/r/m/cargo/home"),
  CARGO_TARGET_DIR: target,
  CARGO_BUILD_BUILD_DIR: target,
  CARGO_INCREMENTAL: "0",
  CARGO_PROFILE_DEV_DEBUG: "0",
  SEMIO_BUILD_MODE: "prototype",
  RUSTC: compiler.stdout.toString().trim(),
  RUSTC_WRAPPER: "",
  CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS: "-Zthreads=1 -Clink-arg=--max-memory=536870912 -Clink-arg=-zstack-size=8388608",
};
delete environment.RUSTFLAGS;
delete environment.CARGO_ENCODED_RUSTFLAGS;
delete environment.RUSTC_WORKSPACE_WRAPPER;
delete environment.CARGO_BUILD_TARGET_DIR;
if (canonical) {
  const owners = arguments_.filter(argument => !["--canonical", "--dry-run", "--serial"].includes(argument));
  if (owners.some(owner => !["note", "wfc"].includes(owner)) || new Set(owners).size !== owners.length) {
    throw Error("--canonical accepts only --dry-run, --serial, and unique Note or WFC owner IDs");
  }
  const selected = owners.length ? owners : ["note", "wfc"];
  const manifest = resolve(generated, "nd/guest/Cargo.toml");
  if (!existsSync(resolve(dirname(manifest), "Cargo.lock"))) throw Error("Existing isolated guest lock required");
  const { cargoProfileInvocationArgumentsV1, renderCargoProfileConfigurationV1, verifyCargoProfileConfigurationV1 } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts");
  const wasm = "wasm32-wasip2";
  const profile = renderCargoProfileConfigurationV1(
    { version: 1, mode: "prototype", target: wasm, producer: "wasm-component", operation: "build" },
    "🌎️hub/Cargo.toml",
    "repository",
  );
  for (const key of Object.keys(environment)) {
    if (key.startsWith("CARGO_PROFILE_") || ["CARGO_INCREMENTAL", "CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS"].includes(key)) {
      delete environment[key];
    }
  }
  const selectors = profile.selectors.flatMap(selector => ["--config", selector]);
  const metadata = Bun.spawnSync(cargo(["-Zunstable-options", ...selectors, "config", "get", "--format=json", "profile.prototype"]), {
    cwd: workspace, env: environment, stderr: "inherit",
  });
  if (metadata.exitCode) process.exit(metadata.exitCode);
  verifyCargoProfileConfigurationV1(profile, JSON.parse(metadata.stdout.toString()));
  const dry = arguments_.includes("--dry-run");
  const args = cargoProfileInvocationArgumentsV1([
    "-Ztrim-paths", "build", "--locked", "--offline", "--manifest-path", manifest, "--target", wasm,
    "-j", arguments_.includes("--serial") ? "1" : "2",
    ...selected.flatMap(owner => ["-p", `semio-hub-${owner}-component`]),
  ], profile);
  if (dry) {
    const units = Bun.spawnSync(cargo(["-Zunstable-options", ...args, "--unit-graph"]), {
      cwd: workspace, env: environment, stderr: "inherit",
    });
    if (units.exitCode) process.exit(units.exitCode);
    const graph = JSON.parse(units.stdout.toString());
    console.log(JSON.stringify({ command: cargo(args), target, role: profile.plan.role, digest: profile.digest, units: graph.units.length, roots: graph.roots.map((index: number) => graph.units[index]) }, null, 2));
  } else {
    const child = Bun.spawn(cargo(args), { cwd: workspace, env: environment, stdout: "inherit", stderr: "inherit" });
    for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
    process.exitCode = await child.exited;
  }
  process.exit(process.exitCode ?? 0);
}

mkdirSync(target, { recursive: true });
const refreshLock = arguments_.includes("--refresh-lock");
const optimizeNote = arguments_.includes("--optimize-note");
const optimizeReactor = arguments_.includes("--optimize-reactor");
const optimizeReceiving = arguments_.includes("--optimize-typed-receiving");
const serial = arguments_.includes("--serial");
const seedLock = arguments_.find(argument => argument.startsWith("--seed-lock="))?.slice("--seed-lock=".length);
const owners = arguments_.filter(argument => !argument.startsWith("--seed-lock=") && !["--refresh-lock", "--optimize-note", "--optimize-reactor", "--optimize-typed-receiving", "--serial"].includes(argument));
if (owners.some(owner => !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(owner))) throw Error("Component owner IDs required");
const selected = owners.length ? owners : ["note", "wfc"];
if (optimizeNote && !selected.includes("note")) throw Error("--optimize-note requires the Note component");
const packages = selected.flatMap(owner => ["-p", `semio-hub-${owner}-component`]);
const wfcProfile = [
  "semio-hub-wfc", "semio-hub-wfc-component",
  ...["2d", "3d", "bitmap", "grid2d", "grid3d", "bitmap-bitmap", "grid3d-editor", "2d-v1",
    "3d-wfc3d", "grid2d-tests-cuts-pinned-corner", "grid2d-inferences", "2d-mutations", "grid2d-v1",
    "grid3d-snapshot", "3d-mutations", "3d-snapshot", "2d-schema", "bitmap-snapshot", "grid2d-snapshot"]
    .map(name => `semio-s-artifact-wfc-${name}`),
  ...["binary-io-registry", "txt-tests", "png-snapshot", "json-mutations-2"]
    .map(name => `semio-s-artifact-stdio-${name}`),
];
const noteProfile = [
  "semio-hub-note", "semio-hub-note-component",
  ...["", "-v1", "-schema", "-mutations"].map(suffix => `semio-s-artifact-note-note${suffix}`),
  "semio-plugin-builder", "semio-plugin-decl-registry", "semio-framework-manifest-arg-schema",
];
const reactorProfile = [
  "semio-framework-job-context", "semio-framework-job-return-budget",
  "semio-framework-value-control-contract", "semio-framework-actor-receipt",
  "semio-framework-actor-retained-turn", "semio-framework-job-native-turn",
  "semio-plugin-reactor-context", "semio-plugin-guest-contract", "semio-plugin-actor-input",
  "semio-ui-contract-values",
];
const receivingProfile = [
  "semio-framework-job", "semio-framework-job-context", "semio-framework-job-native-decode",
  "semio-framework-pack-json-output-receiving", "semio-framework-value-native-decoding",
  "semio-framework-value-native-outcome", "semio-framework-value-native-receiver-protocol",
  "semio-framework-value-native-receiving-frame", "semio-framework-value-native-receiving-original",
  "semio-plugin-actor-input",
];
const optimizedPackages = new Set([
  ...(selected.includes("wfc") ? wfcProfile : []),
  ...(optimizeNote ? noteProfile : []),
  ...(optimizeReactor ? reactorProfile : []),
  ...(optimizeReceiving ? receivingProfile : []),
]);
const optimize = [...optimizedPackages].flatMap(name => ["--config", `profile.dev.package.${name}.opt-level=${optimizeReceiving && receivingProfile.includes(name) ? 2 : 1}`]);
let manifest = resolve(workspace, "🌎️hub/Cargo.toml");
if (seedLock) {
  const isolated = resolve(import.meta.dirname, "🗑️generated/nd/guest");
  const memberDirs = new Map([["note", "🗒️note"], ["wfc", "🀄️wfc"]]);
  const hub = readFileSync(manifest, "utf8");
  const sections = [...hub.matchAll(/^\[(workspace\.(?:package|lints(?:\.[\w-]+)?))\]\r?\n((?:(?!\[)[^\n]*\n)*)/gmu)].map(section => section[0]);
  const dependencies = new Set<string>();
  const absolutePaths = (document: string, directory: string) => document.replace(/\bpath\s*=\s*"([^"]+)"/gu, (_, path: string) => `path = ${JSON.stringify(resolve(directory, path))}`);
  for (const owner of selected) {
    const directory = memberDirs.get(owner);
    if (!directory) throw Error(`No isolated workspace recipe for ${owner}`);
    const source = resolve(workspace, "🌎️hub/🧩️compositions", directory, "🧩️component/📦️packages/🦀️rust/Cargo.toml");
    const component = readFileSync(source, "utf8").replace(/^workspace = "[^"]+"\r?\n/mu, "");
    for (const [, dependency] of component.matchAll(/^([\w-]+)\s*=\s*\{\s*workspace\s*=\s*true\b/gmu)) dependencies.add(dependency);
    const member = resolve(isolated, owner);
    mkdirSync(member, { recursive: true });
    writeFileSync(resolve(member, "Cargo.toml"), absolutePaths(component, dirname(source)));
  }
  const inherited = [...dependencies].map(name => {
    const line = hub.split(/\r?\n/u).find(candidate => candidate.startsWith(`${name} = `));
    if (!line) throw Error(`Missing Hub dependency ${name}`);
    return absolutePaths(line, dirname(manifest));
  });
  manifest = resolve(isolated, "Cargo.toml");
  writeFileSync(manifest, `[workspace]\nresolver = "2"\nmembers = ${JSON.stringify(selected)}\n\n${sections.join("\n")}\n[workspace.dependencies]\n${inherited.join("\n")}\n`);
  const lock = resolve(isolated, "Cargo.lock");
  if (!existsSync(lock)) copyFileSync(resolve(seedLock), lock);
} else if (refreshLock) {
  copyFileSync(resolve(workspace, "🌎️hub/Cargo.lock"), resolve(target, "previous-Cargo.lock"));
}
const child = Bun.spawn([rustup, "run", toolchain, "cargo", "build", ...(refreshLock ? [] : ["--locked"]), "--offline", "--manifest-path", manifest, "--target", "wasm32-wasip2", "-j", serial ? "1" : "2", ...optimize, ...packages], {
  cwd: workspace, env: environment, stdout: "inherit", stderr: "inherit",
});
for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
process.exitCode = await child.exited;
