// #region 🔖Header

// 💻semio/js/dev.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

import { spawn } from "child_process";

const isWindows = process.platform === "win32";
const npmCmd = isWindows ? "npm.cmd" : "npm";

const vite = spawn(npmCmd, ["run", "dev:sketchpad"], {
  stdio: "inherit",
  shell: true,
});

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
