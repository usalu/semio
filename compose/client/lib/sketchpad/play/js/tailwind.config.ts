// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Tailwind CSS configuration for the playground app styling.

// #endregion 🧲Header

// #region 🗄️Configuration
// Tailwind CSS configuration extending the shared compose preset for the play application.
// Configuration MUST use the shared tailwindConfig preset from `@semio-tech/ui-styling`.

import type { Config } from "tailwindcss";
import { tailwindConfig } from "../../../../../elements/client/lib/styling/tailwind.config";

/**
 * Tailwind CSS configuration with content paths and shared preset.
 * Config MUST include content glob patterns and the tailwindConfig preset.
 **/
const config: Pick<Config, "content" | "presets"> = {
  content: ["./**/*.{ts,tsx,mdx}"],
  presets: [tailwindConfig],
};

// Default export of the Tailwind CSS configuration.
// Export MUST be the config object.
export default config;
// #endregion 🗄️Configuration
