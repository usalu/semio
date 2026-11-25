import Ajv from "ajv";
import { constants, mkdirSync, mkdtempSync, readFileSync, statfsSync } from "node:fs";
import { copyFile, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { componentModelDirectoryKeyV1 } from "@semio-tech/framework-plugin/model/contract";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import descriptorSchema from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json" with { type: "json" };
import runSchema from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🧬️schema/🔣️.json" with { type: "json" };
import {
  actorPreparationImageIdentityV1,
  actorPreparationPolicyV1,
  assertActorPreparationBindingV1,
} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";
import {
  cargoProfileInvocationArgumentsV1,
  renderCargoProfileConfigurationV1,
  verifyCargoProfileConfigurationV1,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";

const workspace = process.cwd();
const manifest = "🌎️hub/Cargo.toml";
const target = "wasm32-wasip2";
const generated = resolve(import.meta.dirname, "🗑️generated");
const directory = join(generated, "wfc");
const diskReserve = 7 * 1024 ** 3;
const [operation, emitter] = process.argv.slice(2);
type Owner = "wfc" | "note";
if (operation === "emit" || operation === "emit-note") {
  if (!emitter) throw Error("Repository descriptor emitter required");
  await emitModel(operation === "emit-note" ? "note" : "wfc", resolve(emitter));
  process.exit(0);
}
if (!["plan", "plan-note", "plan-all", "build", "build-note", "build-all"].includes(operation)) {
  throw Error("Use plan, plan-note, plan-all, build, build-note, build-all, emit, or emit-note");
}
const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/m)?.[1];
const registry = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (registry.exitCode) process.exit(registry.exitCode);
const rustup = JSON.parse(registry.stdout.toString()).find((row: { id: string }) => row.id === "rustup")?.command;
if (!rustup || !toolchain) throw Error("Repository Rust toolchain unavailable");
const compiler = Bun.spawnSync([rustup, "which", "--toolchain", toolchain, "rustc"], { stderr: "inherit" });
if (compiler.exitCode) process.exit(compiler.exitCode);
mkdirSync(directory, { recursive: true });
const environment = { ...process.env };
for (const key of Object.keys(environment)) if (key.startsWith("CARGO_") || key.startsWith("RUSTC") || key === "RUSTFLAGS") delete environment[key];
Object.assign(environment, {
  CARGO_HOME: resolve(workspace, ".cache/b/r/m/cargo/home"),
  CARGO_TARGET_DIR: directory,
  CARGO_BUILD_BUILD_DIR: directory,
  CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS: "-Z threads=1 -C link-arg=--max-memory=536870912 -C link-arg=-zstack-size=8388608",
  SEMIO_BUILD_MODE: "prototype",
  RUSTC: compiler.stdout.toString().trim(),
  RUSTC_WRAPPER: "",
});
const profile = renderCargoProfileConfigurationV1(
  { version: 1, mode: "prototype", target, producer: "wasm-component", operation: "build" },
  manifest,
  "repository",
);
const cargo = (args: string[]) => [rustup, "run", toolchain, "cargo", ...args];
const effective = Bun.spawnSync(cargo(["-Zunstable-options", ...profile.selectors.flatMap(selector => ["--config", selector]), "config", "get", "--format=json", "profile.prototype"]), {
  env: environment,
  stderr: "inherit",
});
if (effective.exitCode) process.exit(effective.exitCode);
verifyCargoProfileConfigurationV1(profile, JSON.parse(effective.stdout.toString()));
const args = (owners: readonly Owner[]) => [...cargoProfileInvocationArgumentsV1([
  "-Ztrim-paths", "build", "--locked", "--offline", "--manifest-path", manifest,
  "--target", target, "-j", "1", ...owners.flatMap(owner => ["-p", `semio-hub-${owner}-component`]),
  ...(owners.length > 1 ? ["--keep-going"] : []),
], profile)];
console.log(`[DEBUG] WFC diagnostic with profile digest ${profile.digest} at ${directory}`);
type Unit = { pkg_id: string; mode: string; dependencies: ({ index: number } & Record<string, unknown>)[] };
function unitGraph(owners: readonly Owner[]): { units: Unit[]; roots: number[] } {
  const graph = Bun.spawnSync(cargo(["-Zunstable-options", ...args(owners), "--unit-graph"]), { env: environment, stderr: "inherit" });
  if (graph.exitCode) process.exit(graph.exitCode);
  return JSON.parse(graph.stdout.toString());
}
if (operation === "plan") {
  const { units, roots } = unitGraph(["wfc"]);
  const modes = Object.fromEntries(
    Object.entries(Object.groupBy(units, unit => unit.mode)).map(([mode, group]) => [mode, group?.length]),
  );
  console.log(`[DEBUG] ${units.length} Cargo units ${JSON.stringify(modes)}, roots ${JSON.stringify(roots.map(index => units[index].pkg_id))}`);
} else if (operation === "plan-note") {
  const shared = new Set(unitDigests(unitGraph(["wfc"]).units));
  const note = unitDigests(unitGraph(["note"]).units);
  const common = note.filter(digest => shared.has(digest)).length;
  console.log(`[DEBUG] ${common} of ${note.length} Note units match the ${shared.size} WFC units; ${note.length - common} Note-only units`);
} else if (operation === "plan-all") {
  const owners = ["wfc", "note"] as const;
  const { units, roots } = unitGraph(owners);
  const combined = unitDigests(units);
  for (const owner of owners) {
    const single = unitGraph([owner]);
    const one = single.roots[0];
    const included = roots.find(index => units[index].pkg_id === single.units[one].pkg_id);
    const same = included !== undefined && combined[included] === unitDigests(single.units)[one];
    console.log(`[DEBUG] ${owner} combined unit closure ${same ? "matches" : "differs from"} its standalone plan`);
  }
} else {
  const owners: readonly Owner[] = operation === "build-all" ? ["wfc", "note"] : [operation === "build-note" ? "note" : "wfc"];
  const label = owners.join(" and ");
  const disk = statfsSync(directory);
  if (disk.bavail * disk.bsize < diskReserve) throw Error(`Less than 7 GiB free for isolated ${label} build`);
  const child = Bun.spawn(cargo(args(owners)), { env: environment, stdout: "inherit", stderr: "inherit" });
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
  const diskMonitor = setInterval(() => {
    const disk = statfsSync(directory);
    if (disk.bavail * disk.bsize < diskReserve) {
      console.error(`[DEBUG] Stopping isolated ${label} build at 7 GiB shared-volume reserve`);
      child.kill("SIGINT");
    }
  }, 5000);
  process.exitCode = await child.exited;
  clearInterval(diskMonitor);
}

/** 🧬️ Compares complete Cargo unit identities across two package closures. */
function unitDigests(units: Unit[]): string[] {
  const digests = new Map<number, string>();
  const digest = (index: number): string => {
    const cached = digests.get(index);
    if (cached) return cached;
    const { dependencies, ...unit } = units[index];
    const edges = dependencies.map(({ index: next, ...edge }) => JSON.stringify({ ...edge, next: digest(next) })).sort();
    const value = new Bun.CryptoHasher("sha256").update(JSON.stringify({ unit, edges })).digest("hex");
    digests.set(index, value);
    return value;
  };
  return units.map((_, index) => digest(index));
}

/** 🀄️ Retains the genuine emitter output for an isolated SDK probe. */
async function emitModel(owner: Owner, emitter: string) {
  const folder = owner === "wfc" ? "🀄️wfc" : "🗒️note";
  const crate = `🌎️hub/🧩️compositions/${folder}/🧩️component/📦️packages/🦀️rust`;
  const root = mkdtempSync(join(generated, `${owner}-model-`));
  const model = join(root, "browser", "prototype", "components", componentModelDirectoryKeyV1(crate));
  const linked = join(directory, target, "prototype", `semio_hub_${owner}_component.wasm`);
  const component = join(model, "📦️component.wasm");
  mkdirSync(model, { recursive: true });
  await copyFile(linked, component, constants.COPYFILE_FICLONE | constants.COPYFILE_EXCL);
  const componentSha256 = await sha256(component);
  console.log(`[DEBUG] Exact linked ${owner} bytes SHA-256 ${componentSha256}; emitter SHA-256 ${await sha256(emitter)}`);
  const child = Bun.spawn([emitter, "describe", component, "--out", model, "--prepare-actor"], {
    stdout: "inherit", stderr: "inherit",
  });
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => child.kill(signal));
  const code = await child.exited;
  if (code !== 0) throw Error(`${owner} descriptor emitter exited ${code}`);
  const json = await readFile(join(model, "🔣️.json"));
  const pair = verifySemanticDescriptorPairBytesV1(json, await readFile(join(model, "🛂️.descriptor.semio")));
  if (pair.packageId !== `semio:${owner}` || pair.pluginId !== owner) throw Error(`Expected the ${owner}-owned descriptor`);
  const validate = new Ajv({ strict: false, allErrors: true }).addSchema(runSchema).compile(descriptorSchema);
  if (!validate(JSON.parse(json.toString("utf8")))) throw Error(JSON.stringify(validate.errors));
  const authored = join(crate, "../..");
  const authoredPair = verifySemanticDescriptorPairBytesV1(
    await readFile(join(authored, "🔣️.json")),
    await readFile(join(authored, "🛂️.descriptor.semio")),
  );
  const drift = pair.semanticSha256 !== authoredPair.semanticSha256;
  console.log(`[DEBUG] ${owner} output passed Ajv; authored descriptor ${drift ? "differs at " + authoredPair.semanticSha256 : "matches"}`);
  const image = join(model, actorPreparationPolicyV1.image);
  assertActorPreparationBindingV1(new Uint8Array(await Bun.file(image).slice(0, 4096).arrayBuffer()), componentSha256);
  const actor = await actorPreparationImageIdentityV1(image, new AbortController().signal);
  console.log(`[DEBUG] Isolated ${owner} model at ${root}; descriptor ${pair.semanticSha256}; actor ${actor.imageSha256} (${actor.imageBytes} bytes)`);
}

/** 🔒️ Reads identities without materializing a whole Wasm artifact. */
async function sha256(path: string) {
  const hash = new Bun.CryptoHasher("sha256");
  for await (const bytes of Bun.file(path).stream()) hash.update(bytes);
  return hash.digest("hex");
}
