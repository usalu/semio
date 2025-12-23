#!/usr/bin/env tsx
// #region Header

// hooks/ruff.ts

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
  execSync("ruff format .", {
    cwd: pyEngineDir,
    stdio: "inherit",
  });

  execSync("ruff check --fix .", {
    cwd: pyEngineDir,
    stdio: "inherit",
  });

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
