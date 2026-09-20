import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { testWgpuBootInputs } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts";

const workspace = process.cwd();
const generated = resolve(import.meta.dir, "../🗑️generated/astra-runtime/browser-inputs");
mkdirSync(generated, { recursive: true });
await testWgpuBootInputs(workspace, generated);
