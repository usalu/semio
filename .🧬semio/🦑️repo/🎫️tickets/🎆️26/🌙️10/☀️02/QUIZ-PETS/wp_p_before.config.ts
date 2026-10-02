/** 🕰️ Ticket tool of work package P: a Playwright configuration that runs `wp_p_pet_walk_before.ts` — the pet spec as
 * it stood before 08:41 UTC on 2026-10-02, recovered from the session transcript, with only its two paths rewritten —
 * against a private stack of today's code. It reproduces what another ticket's gate saw between the change of
 * `castOf` (a rotation visitor at home) and the change of the spec that allows one.
 *
 * Usage (from the repository root, with the stack of `wp_p_stack.sh` up):
 *   PLAYWRIGHT_BASE_URL=http://127.0.0.1:6193 TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR=http://127.0.0.1:8923 node node_modules/playwright/cli.js test --config ".../wp_p_before.config.ts" --output <dir>
 */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL;
if (!baseURL) throw new Error("PLAYWRIGHT_BASE_URL must name the site origin of the private stack");

export default defineConfig({
  testDir: dirname(fileURLToPath(import.meta.url)),
  testMatch: ["wp_p_pet_walk_before.ts"],
  fullyParallel: true,
  forbidOnly: true,
  retries: 0,
  workers: 4,
  timeout: 600_000,
  expect: { timeout: 20_000 },
  reporter: [["list"]],
  use: { baseURL, viewport: { width: 1440, height: 900 }, trace: "off", screenshot: "off", actionTimeout: 20_000, navigationTimeout: 60_000 },
});
