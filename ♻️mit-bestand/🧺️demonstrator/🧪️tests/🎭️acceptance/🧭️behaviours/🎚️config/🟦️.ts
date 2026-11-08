// #region 🧲️Header
// 🎚️ Playwright config of the behaviour acceptance: the baseline config with only its `testMatch` replaced, so the baseline's 50 executions stay untouched.
// Run: PLAYWRIGHT_BASE_URL=<served demonstrator> playwright test --config <this file> [--grep @<pane>]
// #endregion 🧲️Header

import { defineConfig } from "@playwright/test";
import baseline from "../../../../🔨️modules/🧪️e2e/🎚️config/🟦️.ts";

export default defineConfig({ ...baseline, testMatch: ["🎭️acceptance/🧭️behaviours/🎭️.ts", "🎭️acceptance/🧭️behaviours/🎭️cycle.ts"] });
