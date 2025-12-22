#!/usr/bin/env tsx
// #region Header

// hooks/typescript.ts

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
const reportPath = join(rootDir, "reports", "typescript.json");

console.log("?? Running TypeScript compiler check...");

try {
  execSync("npx tsc --noEmit --project tsconfig.json", {
    cwd: rootDir,
    encoding: "utf-8",
  });
  const report = {
    timestamp: new Date().toISOString(),
    status: "success",
    errors: [],
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
  console.log("? TypeScript check passed");
  console.log(`?? Report: ${reportPath}`);
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
  console.error("? TypeScript check failed");
  console.error(`?? Report: ${reportPath}`);
  process.exit(1);
}
