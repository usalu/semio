import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { renderWgpuPackageArtifacts } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📽️projection/🟦️.ts";

/** 🚀️ Graph-free twin of the `generate-browser-boot`, `generate-renderer-boot` and `generate-frame-worker` targets: renders each producer's artifacts and writes them where the targets would. */
const root = process.cwd();
for (const target of ["generate-browser-boot", "generate-renderer-boot", "generate-frame-worker"]) {
  const { nodes } = await renderWgpuPackageArtifacts(root, { producerTarget: "@semio-tech/framework-renderer-wgpu:" + target });
  for (const node of nodes) { const path = resolve(root, node.path); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, node.content, "utf8"); console.log("[g3d-native] " + target + " wrote " + node.path + " (" + node.content.length + ")"); }
}
