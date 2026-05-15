// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Tailwind CSS configuration for the sketchpad app styling.

// #endregion 🧲Header

// #region 🗄️Configuration
// Tailwind CSS configuration extending the shared semio preset for the sketchpad application.
// Configuration MUST use the shared tailwindConfig preset from @semio/ui.

import type { Config } from "tailwindcss";
import { tailwindConfig } from "../react/ui";

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
