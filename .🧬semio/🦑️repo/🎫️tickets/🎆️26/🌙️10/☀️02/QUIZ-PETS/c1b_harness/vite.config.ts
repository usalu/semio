/** 🧪️ Ticket tool of work package C1b: Vite dev server of the lifting harness on the private port 6255.
 *
 * Usage (from the repository root; stop it by the PID it was started with):
 *   bun node_modules/vite/bin/vite.js --config ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c1b_harness/vite.config.ts"
 *
 * `C1B_PETS=subset` resolves `@semio-tech/pets` to `pets_barrel.ts` (schema, trigonometry, rig, lift path) for a core
 * that is half rebuilt: the lifting loads, the scenery (which needs the stage) does not.
 */
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
  cacheDir: resolve(ticket, "🗑️generated/c1b/vite-cache"),
  server: { port: 6255, strictPort: true, host: "127.0.0.1", fs: { allow: [repoRoot] } },
  resolve: { alias: [{ find: /^@semio-tech\/pets$/, replacement: process.env.C1B_PETS === "subset" ? resolve(harness, "pets_barrel.ts") : resolve(product, "📦️packages/🟦️typescript/🟦️.ts") }], dedupe: ["react", "react-dom"] },
});
