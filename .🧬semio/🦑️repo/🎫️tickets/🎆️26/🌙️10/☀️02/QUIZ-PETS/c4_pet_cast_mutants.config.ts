/** 🧪️ Ticket tool of work package C4: the vitest configuration of `c4_pet_cast_mutants.ts` — the site's own configuration (root, alias of `@semio-tech/pets`) with the mutation harness as the only test file.
 *
 * Usage: see `c4_pet_cast_mutants.ts`.
 */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const ticket = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(ticket, "../../../../../../..");
const siteRoot = resolve(repoRoot, "🎓️teaching/🏛️architecture/❓️quiz");

export default defineConfig({
  root: siteRoot,
  resolve: { alias: [{ find: /^@semio-tech\/pets$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts") }] },
  server: { fs: { strict: false } },
  test: { root: siteRoot, name: "c4-mutants", environment: "node", include: ["../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c4_pet_cast_mutants.ts"], passWithNoTests: false },
});
