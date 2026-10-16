// 🧮️ Source-closure rows of the WP2 crates for the lib target (native, wasm32) and the unit-test selection (cfg test), via the canonical counter.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return Buffer.alloc(0); throw e; } };
const crates: [string, string, string][] = [
  ["semio-framework-os-renderer-wgpu-engine-registry", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🗺️engine-registry/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🗺️engine-registry/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-engine-text", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/✍️engine-text/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/✍️engine-text/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-engine-board", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎲️engine-board/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎲️engine-board/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-engine-graph", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🕸️engine-graph/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🕸️engine-graph/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-engine-attach", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔗️engine-attach/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔗️engine-attach/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-state", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎭️scene-state/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎭️scene-state/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-canvas2d-host", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔄️canvas2d-host/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔄️canvas2d-host/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-canvas2d", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎨️scene-canvas2d/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎨️scene-canvas2d/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-ink", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/✒️scene-ink/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/✒️scene-ink/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-feeds", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📰️scene-feeds/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📰️scene-feeds/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-tables", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧮️scene-tables/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧮️scene-tables/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-text-vfs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📁️scene-text-vfs/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📁️scene-text-vfs/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-icon-host", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/💠️icon-host/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/💠️icon-host/📦️packages/🦀️rust"],
  ["semio-framework-os-renderer-wgpu-scene-render", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪞️scene-render/🦀️.rs", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪞️scene-render/📦️packages/🦀️rust"]
];
for (const [name, entry, dir] of crates) {
  for (const [label, extra] of [["lib", []], ["lib-wasm32", ["target_arch=wasm32"]], ["test", ["test"]], ["test-wasm32", ["test", "target_arch=wasm32"]]] as [string, string[]][]) {
    const cfg = extra.includes("target_arch=wasm32") ? [...host.filter(v => !v.startsWith("target_arch=") && !v.startsWith("target_os=") && !v.startsWith("target_family=")), ...extra] : [...host, ...extra];
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...cfg, "debug_assertions"], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([name.replace("semio-framework-os-renderer-wgpu-", ""), label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : (inputs.length > 28 || bytes > 230000 || lines > 3500) ? "over-target" : "ok"].join("\t"));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
