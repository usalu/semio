import type * as DemonstratorBrand from "../../🪧️brand.ts";

type TestDependencies = Pick<typeof DemonstratorBrand, "demonstratorPaneBootVariants">;

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies): Promise<void> {
  const { demonstratorPaneBootVariants } = dependencies;

  const { describe, expect, it } = vitest;

  //#region 🧪️DemonstratorPaneBootTests
  describe("demonstratorPaneBootVariants", () => {
    it("preserves branded app ids when panes load standalone modules", () => {
      expect(demonstratorPaneBootVariants("generator")).toEqual({ runtime: "generation3d", manifest: "generator" });
      expect(demonstratorPaneBootVariants("koordinator")).toEqual({ runtime: "cad", manifest: "koordinator" });
    });
  });
  //#endregion 🧪️DemonstratorPaneBootTests
}
