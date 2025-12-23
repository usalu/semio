#!/usr/bin/env npx tsx
// #region Header

// scripts/temp-migrate-capitals-tambours.ts

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

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";
import { Kit } from "../js/js/semio";

async function main() {
  console.log("Loading kit_metabolism.json...");
  const kitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
  const kitJson = readFileSync(kitPath, "utf-8");
  const kit: Kit = JSON.parse(kitJson);

  console.log("Finding entities...");

  const capital = kit.types?.find((t) => t.name === "Capital" && !t.parent);
  const tambour = kit.types?.find((t) => t.name === "Tambour" && !t.parent);

  const cylindricCapital = kit.types?.find((t) => t.name === "Cylindric Capital");
  const cylindricTambour = kit.types?.find((t) => t.name === "Cylindric Tambour");

  if (!capital) {
    throw new Error("Could not find 'Capital' type");
  }
  if (!tambour) {
    throw new Error("Could not find 'Tambour' type");
  }
  if (!cylindricCapital) {
    throw new Error("Could not find 'Cylindric Capital' type");
  }
  if (!cylindricTambour) {
    throw new Error("Could not find 'Cylindric Tambour' type");
  }

  console.log("\nFound entities:");
  console.log(`  Capital: ${capital.guid}`);
  console.log(`  Tambour: ${tambour.guid}`);
  console.log(`  Cylindric Capital: ${cylindricCapital.guid}`);
  console.log(`  Cylindric Tambour: ${cylindricTambour.guid}`);

  console.log("\nCurrent parent status:");
  console.log(`  Cylindric Capital parent: ${cylindricCapital.parent?.guid ?? "none"}`);
  console.log(`  Cylindric Tambour parent: ${cylindricTambour.parent?.guid ?? "none"}`);

  console.log("\nApplying migrations...");

  cylindricCapital.parent = { guid: capital.guid };
  console.log(`  ✓ Set Cylindric Capital parent to Capital`);

  cylindricTambour.parent = { guid: tambour.guid };
  console.log(`  ✓ Set Cylindric Tambour parent to Tambour`);

  console.log("\nSaving updated kit...");
  const updatedJson = JSON.stringify(kit, null, 2);
  writeFileSync(kitPath, updatedJson, "utf-8");

  console.log("✓ Migration complete!");
  console.log("\nNew parent status:");
  console.log(`  Cylindric Capital parent: ${cylindricCapital.parent.guid}`);
  console.log(`  Cylindric Tambour parent: ${cylindricTambour.parent.guid}`);
}

main().catch((error) => {
  console.error("Migration failed:", error);
  process.exit(1);
});
