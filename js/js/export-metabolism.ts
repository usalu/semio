// #region Header

// export-metabolism.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { readFileSync, readdirSync, writeFileSync, statSync } from "fs";
import { join, relative, dirname } from "path";
import { fileURLToPath } from "url";
import { exportKit, Kit } from "./semio";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const WORKSPACE_ROOT = join(__dirname, "../..");
const KIT_PATH = join(WORKSPACE_ROOT, "assets/semio/kit_metabolism.json");
const METABOLISM_DIR = join(WORKSPACE_ROOT, "examples/metabolism");
const OUTPUT_PATH = join(WORKSPACE_ROOT, "assets/semio/metabolism.zip");

/**
 * Recursively collect all files from a directory
 */
function collectFiles(dir: string, baseDir: string): Map<string, Blob> {
  const files = new Map<string, Blob>();

  const entries = readdirSync(dir);
  for (const entry of entries) {
    const fullPath = join(dir, entry);
    const stat = statSync(fullPath);

    if (stat.isDirectory()) {
      const subFiles = collectFiles(fullPath, baseDir);
      for (const [path, blob] of subFiles) {
        files.set(path, blob);
      }
    } else if (stat.isFile()) {
      const relativePath = relative(baseDir, fullPath);
      const content = readFileSync(fullPath);
      const blob = new Blob([content]);
      files.set(relativePath, blob);
    }
  }

  return files;
}

async function main() {
  console.log("Reading kit from:", KIT_PATH);
  const kitJson = readFileSync(KIT_PATH, "utf-8");
  const kit = JSON.parse(kitJson) as Kit;

  console.log("Collecting files from metabolism examples...");
  const files = new Map<string, Blob>();

  // Collect all files from representations folder
  const representationsDir = join(METABOLISM_DIR, "representations");
  console.log("  - representations:", representationsDir);
  const representationFiles = collectFiles(representationsDir, METABOLISM_DIR);
  for (const [path, blob] of representationFiles) {
    files.set(path, blob);
  }

  // Collect all files from icons folder
  const iconsDir = join(METABOLISM_DIR, "icons");
  console.log("  - icons:", iconsDir);
  const iconFiles = collectFiles(iconsDir, METABOLISM_DIR);
  for (const [path, blob] of iconFiles) {
    files.set(path, blob);
  }

  console.log(`Collected ${files.size} files`);

  console.log("Exporting kit to zip...");
  const zipBlob = await exportKit(kit, files);

  console.log("Writing zip to:", OUTPUT_PATH);
  const arrayBuffer = await zipBlob.arrayBuffer();
  writeFileSync(OUTPUT_PATH, new Uint8Array(arrayBuffer));

  console.log("Done! Metabolism kit exported to:", OUTPUT_PATH);
  console.log(`Final size: ${(arrayBuffer.byteLength / 1024 / 1024).toFixed(2)} MB`);
}

main().catch((error) => {
  console.error("Error exporting metabolism kit:", error);
  process.exit(1);
});
