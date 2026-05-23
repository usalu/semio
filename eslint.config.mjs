// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";

// Root ESLint flat config for Bun/Nx lint (library entrypoints; non-type-checked for green CI).
import eslint from "@eslint/js";
import reactHooks from "eslint-plugin-react-hooks";
import tseslint from "typescript-eslint";

export default tseslint.config(eslint.configs.recommended, ...tseslint.configs.recommended, {
  ignores: ["**/node_modules/**", "**/dist/**", "**/pkg/**"],
}, {
  files: ["semio/client/lib/js/index.ts", "semio/client/lib/react/index.tsx"],
  plugins: {
    "react-hooks": reactHooks,
  },
  languageOptions: {
    parser: tseslint.parser,
    parserOptions: {
      ecmaVersion: 2022,
      sourceType: "module",
      ecmaFeatures: { jsx: true },
    },
  },
  rules: {
    "@typescript-eslint/no-explicit-any": "off",
    "@typescript-eslint/no-unused-vars": "off",
    "react-hooks/rules-of-hooks": "off",
    "react-hooks/exhaustive-deps": "off",
  },
}, storybook.configs["flat/recommended"]);
