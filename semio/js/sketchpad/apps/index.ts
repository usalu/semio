// #region 🔖Header

// [👤semio📚js🗃️sketchpad🗃️apps💻indexts](semiorepo://file/SEMIO/JS/SKETCHPAD/APPS/INDEX.TS)

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

// Barrel export for all sketchpad app components.

// #endregion 🔖Header

// #region 🔖Exports

// [🔖semio/js/sketchpad/apps/index.ts#Exports](semiorepo://section/semio/js/sketchpad/apps/index.ts/EXPORTS)
// Re-exports of app plugin utilities and types from the shared module.
// Exports MUST expose only the public API surface of the shared module.

export { composePluginContributions, getAppPlugin, getAppPlugins, hasAppPlugin, registerAppPlugin } from "../shared";
export type { AppMachineContribution, AppPlugin } from "../shared";

// #endregion 🔖Exports
