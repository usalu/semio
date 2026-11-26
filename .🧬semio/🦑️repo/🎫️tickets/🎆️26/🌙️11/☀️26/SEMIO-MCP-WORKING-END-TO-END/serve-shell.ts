import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧭️ Native PDF 1.7 dev host: the managed Vite service shape (middleware-mode server on one shared listener) without the repo graph.
 * Usage: `bun serve-pdf-native.ts` from the workspace root; serves http://127.0.0.1:6270/.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const variant = "stdio-pdf-1-7";
const port = Number(process.env.S_OS_PORT ?? 6270);
const owner = join(root, "✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf");
Object.assign(process.env, { SEMIO_PLUGIN: variant, SEMIO_RENDERER: "react", SEMIO_BUILD_MODE: "prototype", VITE_SEMIO_PLUGIN: variant, VITE_SEMIO_RENDERER: "react", VITE_SEMIO_APP_ID: "s.stdio.pdf@1.7/*#editor", SEMIO_VITE_HMR: "0", S_LOCAL_ONLY: "1", S_OS_PORT: String(port), S_HUB_URL: "", S_DATA_DIR: join(root, ".cache/b/p/m/shell-bridge-data"), SEMIO_VITE_CACHE_DIRECTORY: join(root, ".cache/b/p/m/shell-bridge-vite") });
const { createServer } = createRequire(join(root, "package.json"))("vite") as typeof import("vite");
const server = await createServer({ mode: "prototype", configFile: join(owner, "🏗️builder/🌐️vite/🟦️.ts"), configLoader: "native", server: { host: "127.0.0.1", port, strictPort: true, hmr: false } });
await server.listen();
console.log(`PDF editor dev host ready on http://127.0.0.1:${port}/`);
