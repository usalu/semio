// #region 🔖Header
// [👤semio🖱️sketchpad⚙️postcssconfig](repo://p/u/semio/b/u/sketchpad/f/postcss.config.ts)

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

// PostCSS configuration for the sketchpad app with Tailwind.

// #endregion 🔖Header

// #region 🔖Configuration
// [👤semio🖱️sketchpad⚙️postcssconfig🔖configuration](repo://p/u/semio/b/u/sketchpad/f/postcss.config.ts/s/Configuration)
// PostCSS plugin configuration for the sketchpad application.
// Configuration MUST use the @tailwindcss/postcss plugin.

import { Config } from "postcss-load-config";

/**
 * PostCSS configuration with the Tailwind CSS PostCSS plugin.
// [👤semio🖱️sketchpad⚙️postcssconfig🔖configuration🪨config](repo://p/u/semio/b/u/sketchpad/f/postcss.config.ts/s/Configuration/d/i/config)
 * Config MUST include the @tailwindcss/postcss plugin.
 **/
const config: Config = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

// Default export of the PostCSS configuration.
// Export MUST be the config object.
export default config;
// #endregion 🔖Configuration
