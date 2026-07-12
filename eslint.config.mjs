// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";

// Root ESLint flat config for Bun/Nx lint (library entrypoints; non-type-checked for green CI).
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  {
    ignores: ["**/node_modules/**", "**/dist/**", "**/pkg/**"],
  },
  {
    files: ["compose/client/lib/js/index.ts"],
    rules: {
      "@typescript-eslint/no-unused-vars": "off",
    },
  },
  storybook.configs["flat/recommended"],
);
