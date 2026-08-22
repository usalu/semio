//#region 🔌️Adapters
import storybook from "eslint-plugin-storybook";
import globals from "globals";
import tseslint from "typescript-eslint";
//#endregion 🔌️Adapters

//#region 🔖️OwnedLintContract
export type OwnedLintConfig = Readonly<Record<string, unknown>>;
//#endregion 🔖️OwnedLintContract

//#region 🏭️Factories
/** @emoji 🧹 Builds the UI React flat-lint configuration behind the root manifest that declares its temporary implementations. */
export function createUiReactLintConfig(): OwnedLintConfig[] {
  return tseslint.config(
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
  ) as OwnedLintConfig[];
}
//#endregion 🏭️Factories
