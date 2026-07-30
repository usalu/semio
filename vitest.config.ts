// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Root Vitest configuration for the monorepo test runner.

// #endregion 🧲Header

// #region 🗄️Configuration
// Root Vitest configuration aggregating all workspace test projects.
// Configuration MUST reference all workspace vite config files that define tests.

// #region 🔌Adapters
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

export default defineConfig({
  test: {
    projects: [
      "./compose/client/lib/js/vite.config.ts",
      "./compose/client/lib/sketchpad/js/vitest.config.ts",
      "./compose/dev/algorithm/js/vitest.config.ts",
      "./framework/core/js/vitest.config.ts",
      "./framework/os/core/js/vitest.config.ts",
      "./framework/os/kernel/2d/js/vitest.config.ts",
      "./framework/os/kernel/3d/brep/js/vitest.config.ts",
      "./framework/os/kernel/infinite/canvas/react-renderer/vitest.config.ts",
      "./framework/os/kernel/infinite/world/r3f/vitest.config.ts",
      "./framework/os/kernel/math/graph/dsl/core/js/vitest.config.ts",
      "./framework/os/renderer/js/react/vitest.config.ts",
      "./framework/os/renderer/wgpu/vitest.config.ts",
      "./framework/ui/js/react/vitest.config.ts",
      "./framework/ui/styling/vitest.config.ts",
      "./s/plugin/cad/core/js/vitest.config.ts",
      "./s/plugin/cad/kernel/brepjs/js/vitest.config.ts",
      "./s/plugin/cad/machine/stately/js/vitest.config.ts",
      "./s/plugin/cad/module/aec-building/js/vitest.config.ts",
      "./s/plugin/cad/module/aec-building-energy/js/vitest.config.ts",
      "./s/plugin/cad/module/aec-building-structure/js/vitest.config.ts",
      "./s/plugin/cad/module/spatial-shape/js/vitest.config.ts",
      "./s/plugin/cad/query/js/vitest.config.ts",
      "./s/plugin/cad/renderer/js/vitest.config.ts",
      "./s/plugin/cad/runtime/js/vitest.config.ts",
    ],
  },
});

// #endregion 🗄️Configuration
