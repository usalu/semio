#!/usr/bin/env tsx
// #region 🔖Header

// [👤semio📚engine📜generateschemasts](semiorepo://file/SEMIO/ENGINE/GENERATE-SCHEMAS.TS)

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

// Generates JSON schemas from the engine's Python models.

// #endregion 🔖Header

// #region 🔖Schema Generation

// [🔖semio/engine/generate-schemas.ts#Schema Generation](semiorepo://section/semio/engine/generate-schemas.ts/SCHEMA-GENERATION)
// Schema generation script. MUST invoke the Python engine schema generator.

import { execSync } from "child_process";

execSync('uv run python -c "from engine import generateSchemas; generateSchemas()"', {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ Schemas generated");

// #endregion 🔖Schema Generation
