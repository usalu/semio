/** 🧪️ Ticket tool of work package Q: what each import of the site's pet-cast test costs Vitest to transform and load. Every probe file imports one thing and holds one empty test; run one at a time and read the `transform` and `import` figures of the summary.
 *
 * Usage (from `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`):
 *   bun ../../../../../node_modules/vitest/vitest.mjs run --config "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_q_import_cost/vitest.config.ts" <barrel|menagerie|ajv|validation|nothing>
 */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../../..");

export default defineConfig({
  root: here,
  resolve: {
    alias: [{ find: /^@semio-tech\/pets$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts") }],
  },
  test: { root: here, name: "wp-q-import-cost", environment: "node", include: ["*.probe.ts"], passWithNoTests: false },
});
