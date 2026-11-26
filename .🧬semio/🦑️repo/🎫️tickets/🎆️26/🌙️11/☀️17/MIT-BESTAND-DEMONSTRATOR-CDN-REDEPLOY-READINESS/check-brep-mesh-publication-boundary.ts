import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(import.meta.dirname, "../../../../../../..");
const engine = "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine";
const paths = [`${engine}/🦀️.rs`, `${engine}/📦️mesh-io/🦀️.rs`];
const policy = await Bun.file(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json")).json();
const cfgs = {
  darwin: ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "target_family=unix", "unix"],
  wasi: ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm"],
};
let passed = true;
for (const [platform, cfg] of Object.entries(cfgs)) {
  for (const path of paths) {
    const inputs = await readRustSourceClosureV1([resolve(root, path)], root, cfg, async path => {
      const file = Bun.file(path);
      return await file.exists() ? file.bytes() : null;
    });
    const activeLines = inputs.reduce((sum, input) => sum + input.activeLines, 0);
    const bytes = inputs.reduce((sum, input) => sum + input.bytes, 0);
    const { maxActiveLines, maxSourceBytes, maxSourceFiles } = policy.storage.sourceBoundary;
    console.log(platform, path, `${activeLines}/${maxActiveLines} active lines, ${bytes}/${maxSourceBytes} bytes, ${inputs.length}/${maxSourceFiles} files`);
    passed &&= activeLines <= maxActiveLines && bytes <= maxSourceBytes && inputs.length <= maxSourceFiles;
  }
}
process.exitCode = passed ? 0 : 1;
