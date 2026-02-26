#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚engine🥼testts](semiorepo://p/u/semio/b/l/engine/f/test.ts)

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

// #endregion 🔖Header

// #region 🔖Test Runner
import { execSync } from "child_process";

execSync("poetry run pytest --cov --cov-config=pyproject.toml --cov-report html", {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ Tests complete");

// #endregion 🔖Test Runner
