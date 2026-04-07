// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vitest configuration for the semio domain workspace.

// #endregion 🧲Header

// #region 🗄️Configuration
import { defineConfig } from "vitest/config";

// Vitest configuration for the domain-only semio package.
// Export MUST target only domain tests and avoid sketchpad/browser plugins.
export default defineConfig({
  test: {
    name: "semio",
    environment: "node",
    testTimeout: 30000,
    include: ["index.ts"],
    exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
  },
});
// #endregion 🗄️Configuration
