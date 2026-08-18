import { defineConfig } from "vitest/config";

/** 🧪️ T1 diagnostic-only vitest config: `🎠️kernel/🟦️component.ts` has no dedicated vitest project of
 * its own (confirmed by repo-wide grep — no package.json/vitest.config.ts includes it), unlike
 * `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s own dedicated config. This runs its inline
 * `import.meta.vitest` tests (including this packet's new 🐚️ActivationRegistry ones) directly. */
export default defineConfig({
  root: "/Users/ueli/Documents/semio",
  test: {
    environment: "jsdom",
    include: [],
    includeSource: ["🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts"],
    passWithNoTests: false,
  },
});
