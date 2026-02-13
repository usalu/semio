// #region 🔖Header

// [⚙️semio/desktop/postcss.config.ts](semiorepo://file/semio/desktop/postcss.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

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

// PostCSS configuration for the desktop app with Tailwind and autoprefixer.

// #endregion 🔖Header

// #region 🔖Configuration

// [🔖semio/desktop/postcss.config.ts#Configuration](semiorepo://section/semio/desktop/postcss.config.ts/CONFIGURATION)
// PostCSS plugin configuration for the desktop application.
// Configuration MUST use postcss-import and postcss-nesting plugins.

import { Config } from "postcss-load-config";

// PostCSS configuration with import and nesting plugins.
// Config MUST include postcss-import and postcss-nesting plugins.
const config: Config = {
  plugins: {
    "postcss-import": {},
    "postcss-nesting": {},
  },
};

// Default export of the PostCSS configuration.
// Export MUST be the config object.
export default config;
// #endregion 🔖Configuration
