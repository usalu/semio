import { cargoWorkspacePreparationInvocationV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { repositoryCargoPreparationStorageV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
const root = process.cwd(), n = "semio-s-artifact-gis-gismap", m = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml";
const st = repositoryCargoPreparationStorageV1(root);
for (const args of [["test", "-p", n, "--lib"], ["test", "--manifest-path", root + "/" + m, "-p", n, "--lib"]]) {
  const r = cargoWorkspacePreparationInvocationV1(st, root, args, root, {});
  console.log("[DEBUG]", args.join(" ").slice(0, 60), "=>", r?.args.slice(1).join(" "));
}
