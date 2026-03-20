// #region 🔖Header
// [🔬coda🖱️desktop⚙️postcssconfig](repo://p/r/coda/b/u/desktop/f/postcss.config.ts)

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

// PostCSS configuration for the coda desktop app.

// #endregion 🔖Header

// #region 🔖Configuration
// [🔬coda🖱️desktop⚙️postcssconfig🔖configuration](repo://p/r/coda/b/u/desktop/f/postcss.config.ts/s/Configuration)
// PostCSS plugin configuration for the coda desktop application.
// Configuration MUST use postcss-import and postcss-nesting plugins.

import { Config } from "postcss-load-config";

/**
 * PostCSS configuration with import and nesting plugins.
// [🔬coda🖱️desktop⚙️postcssconfig🔖configuration🪨config](repo://p/r/coda/b/u/desktop/f/postcss.config.ts/s/Configuration/d/i/config)
 * Config MUST include postcss-import and postcss-nesting plugins.
 **/
const config: Config = {
  plugins: {
    "postcss-import": {},
    "postcss-nesting": {},
  },
};

export default config;
// #endregion 🔖Configuration
