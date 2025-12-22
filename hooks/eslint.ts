#!/usr/bin/env tsx
import { execSync } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "eslint.json");
const nxArgs = process.argv.slice(2);

console.log("🔍 Running ESLint...");

try {
  const output = execSync(["npx", "nx", "run-many", "-t", "lint", "--parallel=1", "--output-style=stream", ...nxArgs].join(" "), {
    cwd: rootDir,
    encoding: "utf-8",
  });

  const report = {
    timestamp: new Date().toISOString(),
    status: "success",
    output: output,
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));

  console.log("✅ ESLint check passed");
  console.log(`📝 Report: ${reportPath}`);
  process.exit(0);
} catch (error: any) {
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";

  const report = {
    timestamp: new Date().toISOString(),
    status: "error",
    stdout: stdout,
    stderr: stderr,
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));

  console.error("❌ ESLint check failed");
  console.error(`📝 Report: ${reportPath}`);
  process.exit(1);
}
