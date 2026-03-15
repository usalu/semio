#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚engine📜sqliteschema](semiorepo://p/u/semio/b/l/engine/f/sqliteschema.ts)

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

// Exports the SQLite schema definition for the engine database.

// #endregion 🔖Header

// #region 🔖Schema Export
// [👤semio📚engine💻sqliteschema🔖schemaexport](semiorepo://p/u/semio/b/l/engine/f/sqliteschema.ts/s/Schema%20Export)
// SQLite schema export script. MUST dump the database schema to a SQL file.

import { execSync } from "child_process";
import { join } from "path";

/**
 * Path to the debug SQLite database.
// [👤semio📚engine💻sqliteschema🔖schemaexport🪨dbpath](semiorepo://p/u/semio/b/l/engine/f/sqliteschema.ts/s/Schema%20Export/d/i/dbPath)
 * MUST point to the engine debug build output.
 **/
const dbPath = join(__dirname, "debug", "semio.db");
/**
 * Path to the exported SQL schema file.
// [👤semio📚engine💻sqliteschema🔖schemaexport🪨outputpath](semiorepo://p/u/semio/b/l/engine/f/sqliteschema.ts/s/Schema%20Export/d/i/outputPath)
 * MUST resolve to the monorepo sqlite schema location.
 **/
const outputPath = join(__dirname, "..", "..", "sqlite", "schema.sql");

execSync(`sqlite3 ${dbPath} .schema > ${outputPath}`, {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ SQLite schema exported");

// #endregion 🔖Schema Export
