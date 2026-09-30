import type { AecBuildingTestDependencies } from "../../🟦️.ts";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: AecBuildingTestDependencies, source: TestSource): Promise<void> {
  const { AEC_BUILDING_MODEL_DEFINITION_ID, typologyFromStepLayer } = dependencies;

  const { describe, expect, it } = vitest;
  const runtime = await import("../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");

  runtime.bootstrapCadModules();

  describe("@semio-tech/cad-js-module-aec-building", () => {
    it("maps STEP layer names to building typologies", () => {
      expect(typologyFromStepLayer("Beams", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.beam");
      expect(typologyFromStepLayer("Column", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.column");
    });
  });

}
