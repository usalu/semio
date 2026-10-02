/** 🎚️ Ticket tool of work package J: a Playwright configuration that runs the pet specs of the architecture quiz site
 * alone against a stack that is already up (the private stack of `wp_j_private_stack.ts`), with the window, the limits
 * and the fixture of the site's own end-to-end configuration. `PLAYWRIGHT_BASE_URL` is the site origin, as in the gate;
 * `WP_J_SPECS` names other spec folders (comma-separated) to run instead.
 *
 * Usage (from the repository root):
 *   PLAYWRIGHT_BASE_URL=http://127.0.0.1:6191 TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY=dev node node_modules/playwright/cli.js test --config ".../wp_j_playwright.config.ts" --output <dir>
 */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "@playwright/test";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const baseURL = process.env.PLAYWRIGHT_BASE_URL;
if (!baseURL) throw new Error("PLAYWRIGHT_BASE_URL must name the site origin of the private stack");
const specs = (process.env.WP_J_SPECS ?? "🐕️pet-walk").split(",").map((name) => `${name}/🟦️.ts`);

export default defineConfig({
  testDir: resolve(repoRoot, "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests"),
  fullyParallel: true,
  forbidOnly: true,
  retries: 0,
  workers: 2,
  timeout: 600_000,
  expect: { timeout: 20_000 },
  reporter: [["list"]],
  use: { baseURL, viewport: { width: 1440, height: 900 }, trace: "retain-on-failure", screenshot: "only-on-failure", actionTimeout: 20_000, navigationTimeout: 60_000 },
  projects: [
    { name: "desktop", testMatch: specs },
    { name: "phone", testMatch: ["📱️phone/🟦️.ts"], use: { viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true } },
  ],
});
