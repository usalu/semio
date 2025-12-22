#!/usr/bin/env tsx
// #region Header

// hooks/eslint.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

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
