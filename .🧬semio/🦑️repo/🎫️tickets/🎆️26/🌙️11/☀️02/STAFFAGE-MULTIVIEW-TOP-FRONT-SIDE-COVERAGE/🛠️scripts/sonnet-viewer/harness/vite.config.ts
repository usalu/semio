import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const here = dirname(fileURLToPath(import.meta.url));
const staffage = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export default defineConfig({
  root: here,
  define: { "process.env": "{}", "process.argv": "[]", "process.platform": "\"browser\"" },
  plugins: [react()],
  server: { port: 6071, strictPort: true, fs: { strict: false } },
  preview: { port: 6071, strictPort: true },
  build: { outDir: "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🗑️generated/sonnet-viewer/dist", emptyOutDir: true, minify: false, rollupOptions: { shimMissingExports: true } },
  resolve: {
    alias: [
      { find: /^@teaching\/staffage$/u, replacement: resolve(staffage, "📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^(node:)?path$/u, replacement: resolve(here, "node-path-shim.ts") },
      { find: /^(node:)?url$/u, replacement: resolve(here, "node-url-shim.ts") },
      { find: /^@semio-tech\/framework-transient$/u, replacement: "C:/git/semio/🧰️framework/🔨️modules/🫧️transient/🟦️.ts" },
    ],
    dedupe: ["react", "react-dom"],
  },
});
