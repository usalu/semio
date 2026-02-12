// #region 🔖Header

// 💻semio/js/sketchpad/apps/index.ts

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

// #region 🔖Exports
// Re-exports of app plugin utilities and types from the shared module.
// Exports MUST expose only the public API surface of the shared module.

export { composePluginContributions, getAppPlugin, getAppPlugins, hasAppPlugin, registerAppPlugin } from "../shared";
export type { AppMachineContribution, AppPlugin } from "../shared";

// #endregion 🔖Exports
