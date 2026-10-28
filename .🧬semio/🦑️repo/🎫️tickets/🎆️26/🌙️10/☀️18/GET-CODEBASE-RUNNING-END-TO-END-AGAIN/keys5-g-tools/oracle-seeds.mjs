// keys5-g oracle-seeds: the BY-DESIGN read classes a file oracle must not report as undeclared, each computed from the production function that owns the read (graph-free, read-only).
//   markers       the repository root marker pair the launcher resolves the root from (existence only)
//   membership    every JS installation scope manifest and every member package.json: the nested bootstrap replays the dependency-environment membership before it runs anything (gate only, the lock and the root package.json carry the identity)
//   physical      every package.json the physical package scan walks over (laws/physical-manifests.py: the walk of 🗂️workspaces before it drops the packages whose payload a parent owns; untracked wasm pkg outputs included)
//   cargoWorkspace  the Cargo workspace preparation of a native owner-command or of a caller carrying the native block: workspace manifest, lock, member manifests, toolchain pin, cargo configuration (the token SEMIO_CARGO_LOCK_* keys what the output depends on)
//   token         the files a cargo resolution token is DERIVED from (cargo-seed.mjs)
// usage: import {seedClassesV1} from "./oracle-seeds.mjs"; seedClassesV1({root, cargoManifest?}) -> {markers, membership, cargoWorkspace, token}: sorted repository-relative existing files
import {existsSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {spawnSync} from "node:child_process";
const LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const here = import.meta.dir;
const memo = new Map();
export function seedClassesV1({root = "/Users/ueli/Documents/semio", cargoManifest = undefined} = {}) {
  const key = `${resolve(root)}\0${cargoManifest ?? ""}`;
  if (!memo.has(key)) memo.set(key, computeSeedClassesV1(resolve(root), cargoManifest));
  return memo.get(key);
}
async function computeSeedClassesV1(root, cargoManifest) {
  const exists = (path) => existsSync(join(root, path));
  const bun = await import(join(root, LIB, "🗂️workspaces/🟦️bun/🟦️.ts")), cargo = await import(join(root, LIB, "🗂️workspaces/🦀️cargo/🟦️.ts"));
  const membership = bun.bunRepositoryMembership(root);
  const classes = {
    markers: ["📋️project.json", "package.json"],
    membership: [...membership.scopes.map((scope) => scope.manifest), ...membership.packages.map((directory) => `${directory}/package.json`)],
    physical: spawnSync("python3", ["-I", join(here, "laws/physical-manifests.py"), root], {encoding: "utf8", maxBuffer: 64 * 1024 * 1024}).stdout.split("\n").filter(Boolean),
    cargoWorkspace: [".cargo/config.toml", "rust-toolchain.toml", "Cargo.lock", "Cargo.toml"],
    token: [],
  };
  for (const owner of cargo.discoverCargoWorkspaces(root)) classes.cargoWorkspace.push(owner.manifest, owner.lock, ...owner.memberManifests);
  if (cargoManifest) {
    const run = spawnSync("bun", [join(here, "cargo-seed.mjs"), cargoManifest, "--root", root], {encoding: "utf8"});
    if (run.status !== 0) throw new Error(`cargo-seed failed: ${run.stderr.slice(0, 300)}`);
    classes.token = run.stdout.split("\n").filter(Boolean);
  }
  return Object.fromEntries(Object.entries(classes).map(([name, files]) => [name, [...new Set(files)].filter(exists).sort()]));
}
if (import.meta.main) {
  const seeds = await seedClassesV1({cargoManifest: process.argv[2]});
  console.log(JSON.stringify(Object.fromEntries(Object.entries(seeds).map(([name, files]) => [name, files.length]))));
}
