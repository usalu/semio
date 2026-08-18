// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "..", "..", "..", "..", "..", "..", "..", "..");
const target = resolve(here, "before-component.tsx");

/** 🧪️ Scratch-only vitest config (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet
 * terra-web-plugin-runtime) — measures the BASELINE: the committed (pre-edit, `git show HEAD:...`)
 * copy of `PluginRuntime/🟦️component.tsx`'s inline `import.meta.vitest` tests, which has no real
 * project of its own (same gap `📓️w2-b-report.md` already documented). The copy lives in the ticket
 * folder (CLAUDE.md), so its relative imports are re-pointed via alias to the real absolute targets.
 * Run from the repo root:
 * `bunx vitest run --config ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-web-plugin-runtime-scratch/baseline.vitest.config.ts" --reporter=verbose`
 */
export default defineConfig({
  root: repoRoot,
  resolve: {
    alias: [
      { find: "../../../../../../../🔨️modules/🎠️kernel/🟦️component.ts", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts") },
      { find: "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts") },
      { find: "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts") },
      { find: "../Shell/🟦️component.tsx", replacement: resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx") },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts") },
      { find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts") },
    ],
  },
  test: {
    name: "terra-web-plugin-runtime-baseline",
    mode: "test",
    environment: "node",
    include: [],
    includeSource: [target],
    passWithNoTests: false,
  },
});
