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
      "./🧰framework/⚡️implementation/🟦typescript/vitest.config.ts",
      "./🧰framework/🛍️product/💻os/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔨module/◻️2d/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔨module/🧊3d/⚡️implementation/🟦typescript/📐brep/vitest.config.ts",
      "./🧰framework/🛍️product/💻os/🔨module/♾️infinite/⚡️implementation/🟦typescript/🖼️canvas/🎨react-renderer/vitest.config.ts",
      "./🧰framework/🛍️product/💻os/🔨module/♾️infinite/⚡️implementation/🟦typescript/🌍world/🎨r3f/vitest.config.ts",
      "./🧰framework/🔨module/🧮math/⚡️implementation/🟦typescript/🕸️graph/🗣️dsl/🫀core/🟦typescript/vitest.config.ts",
      "./🧰framework/🛍️product/💻os/🔨module/📺renderer/⚡️implementation/🟦typescript/🧑‍🎨engine/⚛️react/vitest.config.ts",
      "./🧰framework/🛍️product/💻os/🔨module/📺renderer/⚡️implementation/🦀rust/🧑‍🎨engine/🧊wgpu/vitest.config.ts",
      "./🧰framework/🔨module/🖱️ui/⚡️implementation/🟦typescript/⚛️react/vitest.config.ts",
      "./🧰framework/🔨module/🖱️ui/⚡️implementation/🦀rust/🎨styling/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🔨module/🫀core/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🔨module/📐brepjs/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🔨module/🎰stately/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🧩extension/🏢aec-building/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🧩extension/🔥aec-building-energy/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🧩extension/🏛️aec-building-structure/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🧩extension/📐spatial-shape/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🔨module/🔍query/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🔨module/📺renderer/⚡️implementation/🟦typescript/vitest.config.ts",
      "./✏️s/🔌plugin/📐cad/🔨module/🏃runtime/⚡️implementation/🟦typescript/vitest.config.ts",
    ],
  },
});

// #endregion 🗄️Configuration
