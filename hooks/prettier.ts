#!/usr/bin/env tsx
import { execSync } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "prettier.md");

console.log("🎨 Running Prettier format check...");

try {
  const output = execSync("npx prettier --check .", {
    cwd: rootDir,
    encoding: "utf-8",
  });
  
  const report = `# Prettier Format Check Report\n\nGenerated: ${new Date().toISOString()}\n\n## ✅ All files formatted correctly!\n\n${output}\n`;
  writeFileSync(reportPath, report);
  
  console.log("✅ Prettier check passed");
  process.exit(0);
} catch (error: any) {
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";
  
  const report = `# Prettier Format Check Report\n\nGenerated: ${new Date().toISOString()}\n\n## ❌ Format Issues Found\n\n${stdout}\n${stderr}\n\nRun \`npx prettier --write .\` to fix formatting.\n`;
  writeFileSync(reportPath, report);
  
  console.error("❌ Prettier check failed");
  console.error(`📝 Check ${reportPath} for details`);
  process.exit(1);
}
