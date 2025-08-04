// #region Header

// eslint.config.ts

// 2025 Ueli Saluz

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

// #endregion
// import js from "@eslint/js";
// // The following imports cause process not defined error in storybook.
// import eslintConfigPrettier from "eslint-config-prettier";
// import turboPlugin from "eslint-plugin-turbo";
// import tseslint from "typescript-eslint";
// import pluginReactHooks from "eslint-plugin-react-hooks";
// import pluginReact from "eslint-plugin-react";
// import globals from "globals";

// /** @type {import("eslint").Linter.Config} */
// const eslintConfig = [
//     js.configs.recommended,
//     eslintConfigPrettier,
//     ...tseslint.configs.recommended,
//     pluginReact.configs.flat.recommended,
//     {
//         plugins: {
//             turbo: turboPlugin,
//             "react-hooks": pluginReactHooks,
//         },
//         // settings: { react: { version: "detect" } },
//         rules: {
//             ...pluginReactHooks.configs.recommended.rules,
//             // React scope no longer necessary with new JSX transform.
//             // "react/react-in-jsx-scope": "off",
//         },
//         languageOptions: {
//             ...pluginReact.configs.flat.recommended.languageOptions,
//             globals: {
//                 ...globals.serviceworker,
//                 ...globals.browser,
//                 ...globals.node,
//             },
//         },
//         ignores: ["dist/**"],
//     },
// ];

// export default eslintConfig;

// import { defineConfig } from "eslint/config";

// export default defineConfig([
//     {
//         rules: {
//             semi: "error",
//             "prefer-const": "error"
//         }
//     }
// ]);

// import globals from "globals";
// import pluginJs from "@eslint/js";
// import tseslint from "typescript-eslint";
// import pluginReact from "eslint-plugin-react";

/** @type {import('eslint').Linter.Config[]} */
export default [
  // { files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"] },
  // { languageOptions: { globals: { ...globals.browser, ...globals.node } } },
  // pluginJs.configs.recommended,
  // ...tseslint.configs.recommended,
  // pluginReact.configs.flat.recommended,
];
