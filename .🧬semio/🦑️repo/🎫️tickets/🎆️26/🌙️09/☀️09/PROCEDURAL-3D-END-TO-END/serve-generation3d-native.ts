import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧭️ Native generation3d React dev host: the managed Vite service shape (middleware-mode server on one shared listener) without the repo graph.
 * Usage: `bun serve-generation3d-native.ts` from the workspace root; serves http://127.0.0.1:6018/?plugin=generation3d.
 * Needs the staged `procedural` component and its activation receipt (see 📓️react-native-serve-2026-11-23.md).
 * @see ../../../../../../../✏️s/🧑‍💻dev/🎭️variants/🌀️procedural/🧬️schema/🔣️.json */

const root = process.cwd();
const variant = "generation3d";
const port = Number(process.env.S_OS_PORT ?? 6018);
const owner = join(root, "✏️s/🧑‍💻dev/🎭️variants/🌀️procedural");
Object.assign(process.env, { SEMIO_PLUGIN: variant, SEMIO_RENDERER: "react", SEMIO_BUILD_MODE: "prototype", VITE_SEMIO_PLUGIN: variant, VITE_SEMIO_RENDERER: "react", VITE_SEMIO_APP_ID: "s.procedural.generation3d@1/*#editor", SEMIO_VITE_HMR: "0", S_LOCAL_ONLY: "1", S_OS_PORT: String(port), S_HUB_URL: "", S_DATA_DIR: join(root, ".cache/b/p/m/generation3d-data"), SEMIO_VITE_CACHE_DIRECTORY: join(root, ".cache/b/p/m/generation3d-preview-vite") });
const { createServer } = createRequire(join(root, "package.json"))("vite") as typeof import("vite");
const server = await createServer({ mode: "prototype", configFile: join(owner, "🏗️builder/🌐️vite/🟦️.ts"), configLoader: "native", server: { host: "127.0.0.1", port, strictPort: true, hmr: false } });
await server.listen();
console.log(`generation3d React dev host ready on http://127.0.0.1:${port}/?plugin=generation3d`);
