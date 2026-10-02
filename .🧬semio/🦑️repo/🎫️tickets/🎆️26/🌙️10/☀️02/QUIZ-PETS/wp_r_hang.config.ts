/** 🧷️ Ticket tool of work package R: the Playwright configuration that runs a probe of this folder — `WP_R_PROBE`, by
 * default `wp_r_hang_probe.ts` — against a private stack of `wp_r_stack.sh`.
 *
 * Usage (from the repository root, with the stack up):
 *   PLAYWRIGHT_BASE_URL=http://127.0.0.1:6197 TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR=http://127.0.0.1:8927 node node_modules/playwright/cli.js test --config ".../wp_r_hang.config.ts" --repeat-each 8 --output <dir>
 */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL;
if (!baseURL) throw new Error("PLAYWRIGHT_BASE_URL must name the site origin of the private stack");

export default defineConfig({
  testDir: dirname(fileURLToPath(import.meta.url)),
  testMatch: [process.env.WP_R_PROBE ?? "wp_r_hang_probe.ts"],
  fullyParallel: true,
  forbidOnly: true,
  retries: 0,
  workers: 4,
  timeout: 600_000,
  expect: { timeout: 20_000 },
  reporter: [["list"]],
  use: { baseURL, viewport: { width: 1440, height: 900 }, trace: "off", screenshot: "off", actionTimeout: 20_000, navigationTimeout: 60_000 },
});
