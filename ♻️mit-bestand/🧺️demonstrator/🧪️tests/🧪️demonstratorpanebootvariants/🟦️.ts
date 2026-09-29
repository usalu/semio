import type * as DemonstratorBrand from "../../🪧️brand.ts";

type TestDependencies = Pick<typeof DemonstratorBrand, "demonstratorPaneBootVariants">;

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies): Promise<void> {
  const { demonstratorPaneBootVariants } = dependencies;

  const { describe, expect, it } = vitest;

  //#region 🧪️DemonstratorPaneBootTests
  describe("demonstratorPaneBootVariants", () => {
    it("keeps Generator's branded app id while loading the standalone procedural module", () => {
      expect(demonstratorPaneBootVariants("generator")).toEqual({ runtime: "generation3d", manifest: "generator" });
      expect(demonstratorPaneBootVariants("koordinator")).toEqual({ runtime: "koordinator", manifest: "koordinator" });
    });
  });
  //#endregion 🧪️DemonstratorPaneBootTests
}
