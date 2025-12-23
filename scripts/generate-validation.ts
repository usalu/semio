#!/usr/bin/env npx tsx
// #region Header

// scripts/generate-validation.ts

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

import { InvalidKit } from "@semio/assets";
import * as fs from "fs";
import * as path from "path";
import { Kit, serializeValidationResult, validateSemioKit } from "../js/js/semio";

const main = () => {
  const kit = InvalidKit as unknown as Kit;
  const result = validateSemioKit(kit);
  const json = serializeValidationResult(result);

  const outputPath = path.join(__dirname, "..", "assets", "semio", "validation.json");
  fs.writeFileSync(outputPath, json + "\n");

  console.log(`Generated ${outputPath}`);
  console.log(`Found ${result.issues.length} validation issues`);
};

main();
