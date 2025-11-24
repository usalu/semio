#!/usr/bin/env tsx
import { execSync } from "child_process";
import { writeFileSync, existsSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const pyEngineDir = join(rootDir, "py", "engine");
const reportPath = join(rootDir, "reports", "ruff.md");

if (!existsSync(pyEngineDir)) {
  console.log("⚠️ Python engine directory not found, skipping Ruff check");
  process.exit(0);
}

console.log("🔍 Running Ruff linter...");

try {
  const output = execSync("ruff check .", {
    cwd: pyEngineDir,
    encoding: "utf-8",
  });
  
  const report = `# Ruff Linter Report\n\nGenerated: ${new Date().toISOString()}\n\n## ✅ No linting errors found!\n\n${output || "All checks passed."}\n`;
  writeFileSync(reportPath, report);
  
  console.log("✅ Ruff check passed");
  process.exit(0);
} catch (error: any) {
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";
  
  const report = `# Ruff Linter Report\n\nGenerated: ${new Date().toISOString()}\n\n## ❌ Linting Issues Found\n\n${stdout}\n${stderr}\n\nRun \`ruff check --fix .\` in py/engine to auto-fix issues.\n`;
  writeFileSync(reportPath, report);
  
  console.error("❌ Ruff check failed");
  console.error(`📝 Check ${reportPath} for details`);
  process.exit(1);
}
