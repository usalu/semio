#!/usr/bin/env tsx
// #region 🔖Header
// [👤semio📚gh🛅semiograsshopper🗃️yak📜publish](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts)

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

// Publishes the Grasshopper plugin package to the Yak server.

// #endregion 🔖Header

// #region 🔖Publish
// [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish)
// Yak publish script. MUST push the built package to the Yak server.

import { execSync } from "child_process";
import { readFileSync } from "fs";
import { join } from "path";

/**
 * Distribution directory containing the built Yak package.
 * MUST contain the manifest.yml and built .yak file.
 **/
/**
// cwd holds the data fields for a cwd record.
 * [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish🪨cwd](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish/d/i/cwd)
 **/
const cwd = join(__dirname, "dist");

/**
 * Manifest content read from the distribution folder.
 * MUST contain a version field.
 **/
/**
// manifestContent holds the data fields for a manifestContent record.
 * [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish🪨manifestcontent](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish/d/i/manifestContent)
 **/
const manifestContent = readFileSync(join(cwd, "manifest.yml"), "utf-8");
/**
 * Version regex match result from the manifest.
// [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish🪨versionmatch](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish/d/i/versionMatch)
 * MUST successfully extract the version string.
 **/
const versionMatch = manifestContent.match(/version:\s*(.+)/);
if (!versionMatch) {
  throw new Error("Could not find version in manifest.yml");
}
/**
 * Extracted version string from the manifest.
// [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish🪨version](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish/d/i/version)
 * MUST be trimmed of whitespace.
 **/
const version = versionMatch[1].trim();
/**
 * Yak package filename following the naming convention.
 * MUST match the built package name pattern.
 **/
 * buildName holds the data fields for a buildName record.
 * [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish🪨buildname](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish/d/i/buildName)
 **/
const buildName = `semio-${version}-rh8_10-win.yak`;

/**
 * Yak CLI executable path for Rhino 8.
 * MUST point to the installed Yak binary.
 **/
 * yak holds the data fields for a yak record.
 * [👤semio📚gh🛅semiograsshopper🗃️yak💻publish🔖publish🪨yak](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts/s/Publish/d/i/yak)
 **/
const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" push ${buildName}`, { cwd, stdio: "inherit" });

console.log("✅ Yak package published");

// #endregion 🔖Publish
