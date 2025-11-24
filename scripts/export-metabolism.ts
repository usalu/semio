#!/usr/bin/env tsx
import { execSync } from "child_process";
import { existsSync, statSync } from "fs";
import { join } from "path";

console.log("Exporting Metabolism Kit...");

const rootDir = join(__dirname, "..");
const assetsPath = join(rootDir, "assets", "metabolism.zip");

// Set environment variable and run test
process.env.EXPORT_TO_ASSETS = "true";

try {
  execSync('npx vitest run --no-coverage -t "roundtrip export and import"', {
    cwd: join(rootDir, "js", "js"),
    stdio: "inherit",
  });

  if (existsSync(assetsPath)) {
    const stats = statSync(assetsPath);
    const sizeKB = (stats.size / 1024).toFixed(2);
    console.log(`✅ Successfully exported metabolism.zip (${sizeKB} KB)`);
  } else {
    console.error("❌ File not created");
    process.exit(1);
  }
} catch (error) {
  console.error("❌ Export failed");
  process.exit(1);
}
