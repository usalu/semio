// #region 🔖Header
// [👤semio📚js💻dev](semiorepo://p/u/semio/b/l/js/f/dev.ts)

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

// Development server entry point for the JavaScript workspace.

// #endregion 🔖Header

// #region 🔖Dev
// [👤semio📚js💻dev🔖dev](semiorepo://p/u/semio/b/l/js/f/dev.ts/s/Dev)
// Spawns parallel sketchpad and storybook dev servers.
// MUST kill both child processes on SIGINT and SIGTERM.

import { spawn } from "child_process";

/**
 * Whether the current platform is Windows.
// [👤semio📚js💻dev🔖dev🪨iswindows](semiorepo://p/u/semio/b/l/js/f/dev.ts/s/Dev/d/i/isWindows)
 *
 * MUST be checked before spawning npm commands.
 **/
const isWindows = process.platform === "win32";

/**
 * Platform-specific npm command name.
// [👤semio📚js💻dev🔖dev🪨npmcmd](semiorepo://p/u/semio/b/l/js/f/dev.ts/s/Dev/d/i/npmCmd)
 *
 * MUST use .cmd extension on Windows.
 **/
const npmCmd = isWindows ? "npm.cmd" : "npm";

/**
 * Spawned sketchpad dev server process.
// [👤semio📚js💻dev🔖dev🪨vite](semiorepo://p/u/semio/b/l/js/f/dev.ts/s/Dev/d/i/vite)
 *
 * MUST inherit stdio for live output.
 **/
const vite = spawn(npmCmd, ["run", "dev:sketchpad"], {
  stdio: "inherit",
  shell: true,
});

/**
 * Spawned storybook dev server process.
// [👤semio📚js💻dev🔖dev🪨storybook](semiorepo://p/u/semio/b/l/js/f/dev.ts/s/Dev/d/i/storybook)
 *
 * MUST inherit stdio for live output.
 **/
const storybook = spawn(npmCmd, ["run", "dev:storybook"], {
  stdio: "inherit",
  shell: true,
});

process.on("SIGINT", () => {
  vite.kill();
  storybook.kill();
  process.exit();
});

process.on("SIGTERM", () => {
  vite.kill();
  storybook.kill();
  process.exit();
});

// #endregion 🔖Dev
