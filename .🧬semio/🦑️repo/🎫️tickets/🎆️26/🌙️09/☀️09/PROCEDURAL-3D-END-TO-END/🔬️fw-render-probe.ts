/** @emoji 🔬️ Ticket probe: renders the wgpu frame worker and reports its sha, for cwd/env determinism runs. */
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
import { renderFrameWorker } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts";

const bundleRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust";
const artifact = await renderFrameWorker(bundleRoot);
writeFileSync(process.argv[2]!, artifact.content, "utf8");
console.log(`cwd=${process.cwd()} path=${artifact.path} bytes=${Buffer.byteLength(artifact.content)} sha=${createHash("sha256").update(artifact.content).digest("hex").slice(0, 12)}`);
