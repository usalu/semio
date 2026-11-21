import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { parse } from "smol-toml";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/👆️ui-engine-focus/📦️packages/🦀️rust/Cargo.toml";
const owner = readToml(ownerPath);
const ownerDirectory = dirname(ownerPath);
const workspaceDirectory = resolve(ownerDirectory, owner.package.workspace);
const workspace = readToml(join(workspaceDirectory, "Cargo.toml"));
const lock = readToml(join(workspaceDirectory, "Cargo.lock"));
const engine = resolve(ownerDirectory, owner.lib.path);
const sceneOwner = readFileSync(engine, "utf8").match(/#\[path = "([^"]+)"\]\s*pub mod scene_intents_and_clipboard;/u);
assert.ok(sceneOwner, "the selected crate must mount the actual scene and clipboard implementation");
const scene = readFileSync(resolve(dirname(engine), sceneOwner[1]), "utf8");
assert.match(scene, /struct MountedClipboardIo/u);
assert.match(scene, /semio_framework_async_slots::boxed_fixed_slots/u);

const lockedOwner = lock.package.find((entry) => entry.name === owner.package.name);
assert.ok(lockedOwner, "clipboard owner must have a locked entry");
for (const name of ["semio-framework-job-worker", "semio-framework-job-worker-authority", "semio-framework-value"]) {
  verify(name, owner.dependencies[name]);
}
for (const target of ['cfg(not(target_arch = "wasm32"))', 'cfg(all(target_arch = "wasm32", target_os = "unknown"))']) {
  verify("semio-framework-async-slots", owner.target[target].dependencies["semio-framework-async-slots"]);
}
console.log("Verified the clipboard owner, native and Wasm slots, and exact locked dependencies with two TOML parsers.");

function readToml(path: string) {
  const source = readFileSync(path, "utf8");
  const value = Bun.TOML.parse(source);
  assert.deepEqual(value, parse(source), path);
  return value;
}

function verify(name: string, edge: { workspace?: boolean; path?: string } | undefined) {
  assert.ok(edge, "direct dependency " + name);
  const packagePath = edge.workspace ? workspace.workspace.dependencies[name]?.path : edge.path;
  assert.ok(packagePath, "local package for " + name);
  const directory = edge.workspace ? workspaceDirectory : ownerDirectory;
  assert.equal(readToml(join(directory, packagePath, "Cargo.toml")).package.name, name);
  assert.equal(lockedOwner.dependencies.filter((entry) => entry === name).length, 1, "locked owner edge for " + name);
}
