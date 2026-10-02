/** 🧪️ Ticket tool of work package G: Vite dev server of the pet layer harness on a private port.
 *
 * Usage (from the repository root):
 *   bun node_modules/vite/bin/vite.js --config ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_g_harness/vite.config.ts"
 *
 * `WP_G_PETS=ticket` aliases `@semio-tech/pets` to `wp_g_pets_barrel.ts` (the package glue plus behaviour and stage).
 */
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const harness = dirname(fileURLToPath(import.meta.url));
const ticket = resolve(harness, "..");
const repoRoot = resolve(ticket, "../../../../../../..");
const product = resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets");

export default defineConfig({
  root: harness,
  base: "/",
  publicDir: false,
  cacheDir: resolve(ticket, "🗑️generated/wp-g/vite-cache"),
  plugins: [react()],
  server: { port: 6199, strictPort: true, host: "127.0.0.1", fs: { allow: [repoRoot] } },
  resolve: {
    alias: [{ find: /^@semio-tech\/pets$/, replacement: process.env.WP_G_PETS === "ticket" ? resolve(ticket, "wp_g_pets_barrel.ts") : resolve(product, "📦️packages/🟦️typescript/🟦️.ts") }],
    dedupe: ["react", "react-dom"],
  },
});
