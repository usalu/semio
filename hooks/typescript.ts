#!/usr/bin/env tsx
import { execSync } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "typescript.json");

console.log("🔍 Running TypeScript compiler check...");

try {
  const output = execSync("npx tsc --noEmit --project tsconfig.json", {
    cwd: rootDir,
    encoding: "utf-8",
  });
  
  const report = {
    timestamp: new Date().toISOString(),
    status: "success",
    errors: [],
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
  
  console.log("✅ TypeScript check passed");
  console.log(`📝 Report: ${reportPath}`);
  process.exit(0);
} catch (error: any) {
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";
  const output = stdout || stderr;
  
  const report = {
    timestamp: new Date().toISOString(),
    status: "error",
    errors: output.split("\n").filter((line: string) => line.trim()),
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
  
  console.error("❌ TypeScript check failed");
  console.error(`📝 Report: ${reportPath}`);
  process.exit(1);
}
