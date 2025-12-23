#!/usr/bin/env tsx
// #region Header

// hooks/ruff.tsx

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
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import React from "react";
import { render, Text, Box } from "ink";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const pyEngineDir = join(rootDir, "py", "engine");
const reportPath = join(rootDir, "reports", "ruff.json");

function App() {
  const [status, setStatus] = React.useState<"running" | "skipped" | "success" | "error">("running");

  React.useEffect(() => {
    if (!existsSync(pyEngineDir)) {
      setStatus("skipped");
      setTimeout(() => process.exit(0), 100);
      return;
    }

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

      setStatus("success");
      setTimeout(() => process.exit(0), 100);
    } catch (error) {
      setStatus("error");
      setTimeout(() => process.exit(1), 100);
    }
  }, []);

  return (
    <Box flexDirection="column">
      {status === "skipped" ? (
        <Text dimColor>⚠️ Python engine directory not found, skipping Ruff</Text>
      ) : (
        <>
          <Text>🐍 Formatting and linting Python with Ruff...</Text>
          {status !== "running" && (
            <>
              <Text>{status === "success" ? "✅" : "❌"} Ruff formatting {status === "success" ? "complete" : "failed"}</Text>
              <Text dimColor>📝 Report: {reportPath}</Text>
            </>
          )}
        </>
      )}
    </Box>
  );
}

render(<App />);
