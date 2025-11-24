#!/usr/bin/env tsx
import { execSync } from "child_process";
import { existsSync, writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const pyEngineDir = join(rootDir, "py", "engine");
const reportPath = join(rootDir, "reports", "ruff.json");

if (!existsSync(pyEngineDir)) {
  console.log("⚠️ Python engine directory not found, skipping Ruff");
  process.exit(0);
}

console.log("🐍 Formatting and linting Python with Ruff...");

try {
  // Format first
  execSync("ruff format .", {
    cwd: pyEngineDir,
    stdio: "inherit",
  });
  
  // Then fix auto-fixable issues
  execSync("ruff check --fix .", {
    cwd: pyEngineDir,
    stdio: "inherit",
  });
  
  // Generate JSON report for remaining issues
  try {
    const output = execSync("ruff check --output-format=json .", {
      cwd: pyEngineDir,
      encoding: "utf-8",
    });
    const report = {
      timestamp: new Date().toISOString(),
      status: "success",
      issues: JSON.parse(output || "[]"),
    };
    writeFileSync(reportPath, JSON.stringify(report, null, 2));
  } catch (checkError: any) {
    const output = checkError.stdout?.toString() || "[]";
    const report = {
      timestamp: new Date().toISOString(),
      status: "warning",
      issues: JSON.parse(output || "[]"),
    };
    writeFileSync(reportPath, JSON.stringify(report, null, 2));
  }
  
  console.log("✅ Ruff formatting and auto-fixes applied");
  console.log(`📝 Report: ${reportPath}`);
  process.exit(0);
} catch (error) {
  console.error("❌ Ruff formatting failed");
  process.exit(1);
}
