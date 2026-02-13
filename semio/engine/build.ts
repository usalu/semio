#!/usr/bin/env tsx
// #region 🔖Header

// [📜semio/engine/build.ts](semiorepo://file/semio/engine/build.ts)

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

// Build script for the semio engine Python package.

// #endregion 🔖Header

// #region 🔖Build

// [🔖semio/engine/build.ts#Build](semiorepo://section/semio/engine/build.ts/BUILD)
// Build script for the engine binary. MUST bundle the engine via PyInstaller.

import { execSync } from "child_process";
import { existsSync, rmSync } from "fs";
import { join } from "path";

// Engine build working directory.
// MUST resolve to the engine folder.
const cwd = __dirname;

if (!existsSync(join(cwd, "../../.venv"))) {
  execSync("uv sync", { cwd: join(cwd, "../.."), stdio: "inherit" });
}

execSync("npx tsx ./generate-schemas.ts", { cwd, stdio: "inherit" });

if (existsSync(join(cwd, "build"))) {
  rmSync(join(cwd, "build"), { recursive: true });
}
if (existsSync(join(cwd, "dist"))) {
  rmSync(join(cwd, "dist"), { recursive: true });
}

// PyInstaller CLI arguments for bundling the engine binary.
// MUST include all required metadata and hidden imports.
const args = [
  "--name",
  "semio-engine",
  "--windowed",
  "--clean",
  "--noconfirm",
  "--copy-metadata",
  "graphene",
  "--copy-metadata",
  "sqlalchemy",
  "--copy-metadata",
  "loguru",
  "--hidden-import=loguru",
  "--add-data",
  "../assets/icons/semio_512x512.png;icons/",
  "--icon",
  "../assets/icons/semio.ico",
  "engine.py",
];

execSync(`uv run pyinstaller ${args.join(" ")}`, { cwd, stdio: "inherit" });

if (!process.argv.includes("--skip-post-build")) {
  execSync("tsx ./post-build.ts", { cwd, stdio: "inherit" });
}

console.log("✅ Build complete");

// #endregion 🔖Build
