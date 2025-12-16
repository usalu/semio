#!/usr/bin/env tsx
// SPDX-License-Identifier: AGPL-3.0-only
import { execSync } from "child_process";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportDir = join(rootDir, "reports");

console.log("🔍 Running i18n validation...");

try {
  execSync("npx tsx scripts/i18n.ts", {
    cwd: rootDir,
    stdio: "inherit",
  });
  console.log("✅ i18n validation passed");
  console.log(`📝 Report: ${reportDir}/i18n.json`);
  process.exit(0);
} catch (error) {
  console.error("❌ i18n validation failed");
  console.error(`📝 Check ${reportDir}/i18n.json for details`);
  process.exit(1);
}
