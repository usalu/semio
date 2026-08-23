//#region 🔌️Adapters
import storybook from "eslint-plugin-storybook";
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

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("owned UI React lint config", () => {
    it("does not depend on predefined globals or enable no-undef", () => {
      const config = createUiReactLintConfig();

      expect(config.every((entry) => !("globals" in ((entry.languageOptions as Record<string, unknown> | undefined) ?? {})))).toBe(true);
      expect(config.every((entry) => !("no-undef" in ((entry.rules as Record<string, unknown> | undefined) ?? {})))).toBe(true);
    });
  });
}
//#endregion 🧪️Tests
