// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));
const target = resolve(here, "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx");

/** 🧪️ Scratch-only vitest config (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS,
 * lane W2-B) — exercises the inline `import.meta.vitest` tests in
 * `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`,
 * which has no project of its own (see `📓️w2-b-report.md`). Not wired into any `project.json`/nx
 * target — a throwaway verification harness kept in the ticket folder per CLAUDE.md, not a permanent
 * build artifact. Run from the repo root:
 * `bunx vitest run --config ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/🧪️w2-b-plugin-runtime-vitest.config.ts"`
 */
export default defineConfig({
  test: {
    name: "w2-b-plugin-runtime-scratch",
    mode: "test",
    environment: "node",
    include: [target],
    includeSource: [target],
    passWithNoTests: false,
  },
});
