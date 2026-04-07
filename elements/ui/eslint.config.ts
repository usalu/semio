// #region 🔖Header
// [👤semio📚js⚙️eslintconfig](repo://p/u/semio/b/l/js/f/eslint.config.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ESLint configuration for the JavaScript workspace linting rules.

// #endregion 🔖Header

// #region 🔖Configuration
// [👤semio📚js⚙️eslintconfig🔖configuration](repo://p/u/semio/b/l/js/f/eslint.config.ts/s/Configuration)
// ESLint flat configuration for the semio JS package.
// Configuration MUST export a valid ESLint flat config array.

import globals from "globals";
import tseslint from "typescript-eslint";

// Default ESLint flat configuration export with no custom rules.
// Export MUST be an array of ESLint config objects.
export default tseslint.config(
    {
        ignores: ["node_modules/**", "storybook-static/**"],
    },
    {
        files: ["**/*.{ts,tsx}"],
        languageOptions: {
            parser: tseslint.parser,
            ecmaVersion: "latest",
            sourceType: "module",
            globals: {
                ...globals.browser,
                ...globals.node,
            },
            parserOptions: {
                ecmaFeatures: {
                    jsx: true,
                },
            },
        },
    }
);

// #endregion 🔖Configuration
