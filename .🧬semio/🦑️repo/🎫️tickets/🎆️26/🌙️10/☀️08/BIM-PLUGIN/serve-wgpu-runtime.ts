import { openSync, statSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { createServer } from "vite";
import { bimShellModuleMiddleware, rawGuestIsReady } from "./bim-shell-modules.ts";

const root = process.cwd();
const port = Number(process.env.SEMIO_WGPU_PORT ?? 16402);
const rendererOnly = process.argv.includes("--renderer-only");
const rendererPreflight = rendererOnly || process.argv.includes("--renderer-preflight");
process.env.S_OS_PORT = String(port);
process.env.SEMIO_VITE_HMR ??= "0";
process.env.SEMIO_RUNTIME_DIAGNOSTICS ??= "1";
const load = (file: string) => import(pathToFileURL(resolve(root, file)).href);
const { wgpuCompletedFrameworkRootsV1, createCapturedWgpuConfigurationV1: createEmptyWgpuConfigurationV1 } = await load("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts");
const { createCapturedWgpuConfigurationV1 } = await load("✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts");
const renderer = wgpuCompletedFrameworkRootsV1("prototype").compilerRoot;
const rawBindings = resolve(import.meta.dirname, "🗑️generated/wgpu-bindings");
const rendererFiles = ["semio-renderer-wgpu-component.js", "semio-renderer-wgpu-component_bg.wasm"];
const ready = () => (rendererPreflight || rawGuestIsReady()) && rendererFiles.every(file => (statSync(resolve(rawBindings, file), { throwIfNoEntry: false })?.size ?? 0) > 8);
if (process.argv.includes("--wait")) {
  console.log("[DEBUG] Awaiting linked renderer, linked BIM guest and emitted descriptors");
  while (!ready()) await new Promise(resolve => setTimeout(resolve, 5000));
}
if (!ready()) throw new Error("The linked renderer, linked BIM guest or emitted descriptors are unavailable");
const config = (rendererOnly ? createEmptyWgpuConfigurationV1 : createCapturedWgpuConfigurationV1)({
  rootFor: (published: string) => published === renderer ? rawBindings : published,
  openFile: (path: string) => {
    try { return openSync(path, "r"); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  },
}, "bim");
const diagnosticModules = {
  name: "bim-captured-shell-inputs",
  enforce: "pre" as const,
  configureServer(server: Parameters<NonNullable<import("vite").Plugin["configureServer"]>>[0]) {
    server.middlewares.use(bimShellModuleMiddleware);
  },
};
const plugins = config.plugins.filter(plugin => plugin.name !== "wgpu-completed-artifact-reload"
  && !(rendererOnly && plugin.name === "wgpu-browser-selection"));
const server = await createServer({
  ...config, plugins: [diagnosticModules, ...plugins], configFile: false,
  cacheDir: resolve(import.meta.dirname, "🗑️generated", "wgpu-vite", String(port)),
  server: { ...config.server, host: "127.0.0.1", port, strictPort: true },
});
await server.listen();
server.printUrls();
