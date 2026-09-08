import { defineConfig } from "vitest/config";

/** 🧪️ Runs the draft-07 ajv oracle spec at its taxonomy location `🧪️tests/✅️draft07-oracle/🟦️.ts`,
 * which vitest's default `**‍/*.{test,spec}.*` include does not match. See
 * `📓️wp3b-validator-keywords.md` §7.1 — the root `📜️script.ts` still names the pre-sweep path. */
export default defineConfig({
  test: { root: process.cwd(), include: ["🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts"] },
});
