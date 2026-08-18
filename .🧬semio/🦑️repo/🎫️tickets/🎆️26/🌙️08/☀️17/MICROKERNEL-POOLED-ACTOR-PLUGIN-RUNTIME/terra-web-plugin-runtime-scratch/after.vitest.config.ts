// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "..", "..", "..", "..", "..", "..", "..", "..");
const target = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx");

/** 🧪️ Scratch-only vitest config (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet
 * terra-web-plugin-runtime) — exercises the REAL, edited
 * `PluginRuntime/🟦️component.tsx`'s inline `import.meta.vitest` tests, which has no project of its
 * own (`📓️w2-b-report.md`). Not wired into any `project.json`/nx target — a throwaway verification
 * harness kept in the ticket folder per CLAUDE.md, not a permanent build artifact. Run from the repo
 * root:
 * `bunx vitest run --config ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-web-plugin-runtime-scratch/after.vitest.config.ts" --reporter=verbose`
 */
export default defineConfig({
  root: repoRoot,
  test: {
    name: "terra-web-plugin-runtime-after",
    mode: "test",
    environment: "node",
    include: [],
    includeSource: [target],
    passWithNoTests: false,
  },
});
