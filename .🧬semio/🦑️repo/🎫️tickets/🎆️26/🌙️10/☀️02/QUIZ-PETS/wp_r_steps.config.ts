/** 🧾️ Ticket tool of work package R: the Playwright configuration of `wp_j_playwright.config.ts` with the JSON reporter,
 * so that `wp_r_steps.mjs` can read how long every step of a test took (how long a walk was waited for).
 *
 * Usage (from the repository root, with the stack of `wp_r_stack.sh` up):
 *   WP_R_REPORT=<file> PLAYWRIGHT_BASE_URL=http://127.0.0.1:6197 TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR=http://127.0.0.1:8927 node node_modules/playwright/cli.js test --config ".../wp_r_steps.config.ts" --project desktop --grep <title>
 */
import { defineConfig } from "@playwright/test";
import base from "./wp_j_playwright.config.ts";

export default defineConfig({ ...base, reporter: [["json", { outputFile: process.env.WP_R_REPORT ?? "wp-r-report.json" }]] });
