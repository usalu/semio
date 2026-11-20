import assert from "node:assert/strict";
import { dirname, resolve } from "node:path";
import { parse } from "smol-toml";

const packageId = "semio-framework-3d-brep-solid-transaction";
const framework = "🧰️framework";
const workspace = parse(await Bun.file(`${framework}/Cargo.toml`).text()) as {
  workspace: { members: string[]; dependencies: Record<string, { path: string }> };
};
const packagePath = workspace.workspace.dependencies[packageId].path;
assert(workspace.workspace.members.includes(packagePath));
const root = `${framework}/${packagePath}`;
const cargo = parse(await Bun.file(`${root}/Cargo.toml`).text()) as {
  package: { name: string; workspace: string };
  lib: { path: string };
  dependencies: Record<string, { workspace: boolean }>;
};
assert.equal(cargo.package.name, packageId);
assert.equal(resolve(root, cargo.package.workspace), resolve(framework));
assert(resolve(root, cargo.lib.path).length <= 256);
for (const [dependency, row] of Object.entries(cargo.dependencies)) {
  assert(row.workspace && workspace.workspace.dependencies[dependency]);
  assert.notEqual(dependency, "semio-framework-3d-brep-native-registry");
}
const registryPath = workspace.workspace.dependencies["semio-framework-3d-brep-native-registry"].path;
const registry = parse(await Bun.file(`${framework}/${registryPath}/Cargo.toml`).text()) as { dependencies: Record<string, { workspace: boolean }> };
assert.equal(registry.dependencies[packageId].workspace, true);
const owners = await Bun.file("📋️build-owners.json").json() as { owners: { name: string; root: string; manifest: string }[] };
const matching = owners.owners.filter(owner => owner.name === `@semio-tech/${packageId}`);
assert.equal(matching.length, 1);
assert.equal(matching[0].root, root);
assert.equal(dirname(matching[0].manifest), root);
assert.equal((await Bun.file(matching[0].manifest).json()).name, matching[0].name);
console.log("[DEBUG] The native registry's solid transaction is registered once in Cargo and the repository owner registry");
