#!/usr/bin/env tsx
// #region Header

// hooks/eslint.tsx

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
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import React from "react";
import { render, Text, Box } from "ink";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "eslint.json");
const nxArgs = process.argv.slice(2);

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");

  React.useEffect(() => {
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

      setStatus("success");
      setTimeout(() => process.exit(0), 100);
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

      setStatus("error");
      setTimeout(() => process.exit(1), 100);
    }
  }, []);

  return (
    <Box flexDirection="column">
      <Text>🔍 Running ESLint...</Text>
      {status !== "running" && (
        <>
          <Text>
            {status === "success" ? "✅" : "❌"} ESLint check {status === "success" ? "passed" : "failed"}
          </Text>
          <Text dimColor>📝 Report: {reportPath}</Text>
        </>
      )}
    </Box>
  );
}

render(<App />);
