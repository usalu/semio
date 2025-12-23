#!/usr/bin/env npx tsx
// #region Header

// scripts/regen-metabolism.ts

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

import { readFileSync, readdirSync, writeFileSync } from "fs";
import { join } from "path";
import { MetabolismKit } from "../assets/index";
import { exportKit, importKit } from "../js/js/semio";

const INCLUDE_FOLDERS = ["representations", "icons", "images"];

function collectFiles(dir: string, basePath: string = ""): Map<string, Blob> {
  const files = new Map<string, Blob>();
  const entries = readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = join(dir, entry.name);
    const relativePath = basePath ? `${basePath}/${entry.name}` : entry.name;

    if (entry.isDirectory()) {
      if (entry.name === ".semio" || entry.name === ".git") continue;

      if (!basePath && !INCLUDE_FOLDERS.includes(entry.name)) continue;

      const subFiles = collectFiles(fullPath, relativePath);
      Array.from(subFiles.entries()).forEach(([path, blob]) => {
        files.set(path, blob);
      });
    } else {
      if (!basePath) continue;

      const buffer = readFileSync(fullPath);
      const blob = new Blob([buffer]);
      files.set(relativePath, blob);
    }
  }

  return files;
}

async function main() {
  console.log("Regenerating metabolism.zip...");

  const kit = MetabolismKit;

  const metabolismDir = join(__dirname, "..", "examples", "metabolism");
  const files = collectFiles(metabolismDir);

  console.log(`Found ${files.size} files to include:`);
  Array.from(files.keys())
    .slice(0, 10)
    .forEach((path) => {
      console.log(`  - ${path}`);
    });
  if (files.size > 10) {
    console.log(`  ... and ${files.size - 10} more`);
  }

  const tambourBefore = kit.types?.find((t) => t.name === "Tambour");
  console.log("Tambour models in source:", tambourBefore?.models?.length ?? 0);

  const zipBlob = await exportKit(kit, files);
  const buffer = Buffer.from(await zipBlob.arrayBuffer());

  const outputPath = join(__dirname, "..", "assets", "semio", "metabolism.zip");
  writeFileSync(outputPath, buffer);

  console.log("Exported to:", outputPath);
  console.log("Size:", (buffer.length / 1024).toFixed(2), "KB");

  const { kit: imported, files: importedFiles } = await importKit(buffer);
  const tambourAfter = imported.types?.find((t) => t.name === "Tambour");
  console.log("Tambour models after import:", tambourAfter?.models?.length ?? 0);
  console.log("Files in imported zip:", importedFiles.size);
}

main().catch(console.error);
