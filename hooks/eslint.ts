#!/usr/bin/env tsx
import { execSync } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "eslint.md");

console.log("🔍 Running ESLint...");

try {
  const output = execSync("npx nx run-many -t lint --parallel=1", {
    cwd: rootDir,
    encoding: "utf-8",
  });
  
  const report = `# ESLint Report\n\nGenerated: ${new Date().toISOString()}\n\n## ✅ No linting errors found!\n\n${output}\n`;
  writeFileSync(reportPath, report);
  
  console.log("✅ ESLint check passed");
  process.exit(0);
} catch (error: any) {
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";
  
  const report = `# ESLint Report\n\nGenerated: ${new Date().toISOString()}\n\n## ❌ Linting Issues Found\n\n${stdout}\n${stderr}\n`;
  writeFileSync(reportPath, report);
  
  console.error("❌ ESLint check failed");
  console.error(`📝 Check ${reportPath} for details`);
  process.exit(1);
}
