/** 🎭️ Runs the end-to-end project `layout` (the adaptive layout in a real browser) against a stack that already runs —
 * `bun layout_spec.ts [site origin] [Playwright arguments…]`, by default the steady site of `steady_site.sh` on 6065 —
 * instead of the throw-away stacks of the gate, which need the proctor built from a tree other sessions are changing.
 * Traces and screenshots of a failure go to `🗑️generated/spec`. */
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const [origin = "http://localhost:6065", ...forwarded] = process.argv.slice(2);
const done = spawnSync(
  process.execPath,
  [resolve(repoRoot, "node_modules/playwright/cli.js"), "test", "--config", resolve(repoRoot, "🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🎚️config/🟦️.ts"), "--output", resolve(import.meta.dir, "🗑️generated/spec"), "--project", "layout", "--no-deps", ...forwarded],
  { cwd: repoRoot, stdio: "inherit", env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH, PLAYWRIGHT_BASE_URL: origin } },
);
process.exit(done.status ?? 1);
