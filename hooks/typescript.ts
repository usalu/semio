#!/usr/bin/env tsx
import { execSync } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "typescript.md");

console.log("🔍 Running TypeScript compiler check...");

try {
  const output = execSync("npx tsc --noEmit --project tsconfig.json", {
    cwd: rootDir,
    encoding: "utf-8",
  });
  
  const report = `# TypeScript Compiler Report\n\nGenerated: ${new Date().toISOString()}\n\n## ✅ No type errors found!\n\n${output}\n`;
  writeFileSync(reportPath, report);
  
  console.log("✅ TypeScript check passed");
  process.exit(0);
} catch (error: any) {
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";
  
  const report = `# TypeScript Compiler Report\n\nGenerated: ${new Date().toISOString()}\n\n## ❌ Type Errors Found\n\n${stdout}\n${stderr}\n`;
  writeFileSync(reportPath, report);
  
  console.error("❌ TypeScript check failed");
  console.error(`📝 Check ${reportPath} for details`);
  process.exit(1);
}
