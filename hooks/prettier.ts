#!/usr/bin/env tsx
import { execSync } from "child_process";
import { join } from "path";

const rootDir = join(__dirname, "..");

console.log("🎨 Formatting with Prettier...");

try {
  execSync("npx prettier --ignore-path .prettierignore --write .", {
    cwd: rootDir,
    stdio: "inherit",
  });
  console.log("✅ Prettier formatting complete");
  process.exit(0);
} catch (error) {
  console.error("❌ Prettier formatting failed");
  process.exit(1);
}
