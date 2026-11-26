import { installedProceduralVite } from "./installed-procedural-vite.ts";
import { createServer as createHttpServer } from "node:http";
import { createRequire } from "node:module";
import { createReadStream, existsSync, statSync } from "node:fs";
import { join, normalize } from "node:path";
import specificWgpuConfiguration from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts";

/** 🧭️ Graph-free WGPU dev host for the procedural `generation3d` playground: the managed Vite service shape on one listener, serving the completed
 * artifacts (boot, frame worker, plugin modules) with the real generated catalog the registered serve injects (`🧊️wgpu/🟦️.ts`) and the natively built renderer wasm from `S_G3D_WASM`.
 * @see ../📜️serve-generation3d-wgpu-native.sh */
const root = process.cwd(), variant = process.env.SEMIO_PLUGIN ?? "generation3d", port = Number(process.env.S_OS_PORT ?? 6118);
const wasmRoot = process.env.S_G3D_WASM ?? join(root, ".cache/g3d-native/wasm-dev");
const types: Record<string, string> = { ".js": "text/javascript", ".wasm": "application/wasm" };
process.env.SEMIO_PLUGIN = variant;
const config = specificWgpuConfiguration();
const nativeWasm = { name: "g3d-native-renderer-wasm", enforce: "pre" as const, configureServer(server: { middlewares: { use(handler: (request: { url?: string }, response: any, next: () => void) => void): void } }) {
  server.middlewares.use((request, response, next) => {
    const path = decodeURIComponent((request.url ?? "").split("?")[0] ?? ""), prefix = "/renderer-modules/wgpu/";
    if (!path.startsWith(prefix)) return next();
    const file = normalize(join(wasmRoot, path.slice(prefix.length)));
    if (!file.startsWith(wasmRoot) || !existsSync(file) || !statSync(file).isFile()) return next();
    response.setHeader("content-type", types[file.slice(file.lastIndexOf("."))] ?? "application/octet-stream");
    response.setHeader("cache-control", "no-store");
    createReadStream(file).pipe(response);
  });
} };
const { createServer } = createRequire(join(root, "package.json"))("vite") as typeof import("vite");
const listener = createHttpServer();
const server = await createServer({ ...(config as object), configFile: false, mode: "prototype", plugins: [installedProceduralVite("wgpu"), nativeWasm, ...((config as any).plugins ?? [])], server: { ...((config as any).server ?? {}), host: "127.0.0.1", port, strictPort: true, middlewareMode: { server: listener }, hmr: false } } as any);
listener.on("request", (request, response) => server.middlewares(request, response));
listener.listen(port, "127.0.0.1", () => console.log("WGPU generation3d native host ready: http://127.0.0.1:" + port + "/?plugin=" + variant));
