/** 🌐️ WG7 (ticket-local) — the wgpu note release serve with its plugin-module root replaced by `wg7-catalog-module.ts`'s
 * durable root, so the served note IS the catalog's note. Everything else is the product config's own inputs. */
import { join, resolve } from "node:path";
import { createWgpuBrowserConfig } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts";
import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { semioAssetsVitePlugin } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

const workspace = "/Users/ueli/Documents/semio";
const wgpu = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu");
export default () => {
  const playground = PLAYGROUND_BUILD_TARGETS.find((row) => row.variant === "note")!;
  const runtime = developmentRuntimeRoot(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), "note", "release", "wgpu");
  const config = createWgpuBrowserConfig({
    workspace, root: join(wgpu, "🌐️server"), profile: "release", variant: "note",
    compilerRoot: join(wgpu, "📦️packages/🦀️rust/dist/wasm-release"),
    bootRoot: join(wgpu, "🚀️browser-boot/🤖️generated"),
    workerRoot: join(wgpu, "🎞️frame-worker/🤖️generated"),
    moduleRoot: join(workspace, ".🧬semio/🌐hub", process.env.WG7_MODULE_ROOT ?? "s11-wg7-catalog-modules", "release/🔌️plugin-modules"),
    extensionRoot: join(runtime, "extensions"),
    reloadFile: join(runtime, "activation", ACTIVATION_RECEIPT_FILE),
    assets: playground.assets,
  });
  return { ...config, plugins: [...config.plugins!, ...semioAssetsVitePlugin(workspace)] };
};
