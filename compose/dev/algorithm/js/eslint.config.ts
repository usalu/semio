// #region 🧲Header
// 💻 compose/algorithm/🟦eslint.config.ts
// Specs: Provide a minimal flat ESLint config for this bundle.
// Summary: Enables TS/TSX linting for the algorithms Storybook bundle.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

// #region 🔌Adapters
// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";
import globals from "globals";
import tseslint from "typescript-eslint";
// #endregion 🔌Adapters

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
  },
  storybook.configs["flat/recommended"],
);
