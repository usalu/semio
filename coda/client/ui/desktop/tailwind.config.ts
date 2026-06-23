// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Tailwind CSS configuration for the coda desktop app styling.

// #endregion 🧲Header

// #region 🔌Adapters
// Tailwind CSS configuration for the coda desktop application.
// Configuration MUST include content glob patterns.

import type { Config } from "tailwindcss";
// #endregion 🔌Adapters

// #region 🗄️Configuration
/**
 * Tailwind CSS configuration with content paths for the coda desktop app.
 * Config MUST include content glob patterns.
 **/
const config: Pick<Config, "content" | "presets"> = {
  content: ["./**/*.{ts,tsx}", "../../../../elements/ui/**/*.{ts,tsx}"],
};

export default config;
// #endregion 🗄️Configuration
