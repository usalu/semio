// #region 🔖Header
// 💻 semio/algorithms/eslint.config.ts
// Specs: Provide a minimal flat ESLint config for this bundle.
// Summary: Enables TS/TSX linting for the algorithms Storybook bundle.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import globals from "globals";
import tseslint from "typescript-eslint";

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
);
