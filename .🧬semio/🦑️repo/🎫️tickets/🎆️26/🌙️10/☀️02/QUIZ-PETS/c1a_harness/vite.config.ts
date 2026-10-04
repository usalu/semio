/** 🧪️ Ticket tool of work package C1a (second round): Vite dev server of the hand harness on port 6253.
 *
 * Usage (from the repository root):
 *   bun node_modules/vite/bin/vite.js --config ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c1a_harness/vite.config.ts"
 *
 * `@semio-tech/pets` is aliased to `c1a_pets_recorder.ts`: the package itself, with `advance` and `frameOf` wrapped so
 * that the page can read back every event the layer hands the stage and the last frame it painted.
 */
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const harness = dirname(fileURLToPath(import.meta.url));
const ticket = resolve(harness, "..");
const repoRoot = resolve(ticket, "../../../../../../..");

export default defineConfig({
  root: harness,
  base: "/",
  publicDir: false,
  cacheDir: resolve(ticket, "🗑️generated/c1a/vite-cache"),
  plugins: [react()],
  server: { port: 6253, strictPort: true, host: "127.0.0.1", hmr: false, watch: null, fs: { allow: [repoRoot] } },
  resolve: {
    alias: [{ find: /^@semio-tech\/pets$/, replacement: resolve(harness, "c1a_pets_recorder.ts") }],
    dedupe: ["react", "react-dom"],
  },
});
