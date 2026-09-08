type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { AEC_BUILDING_MODEL_DEFINITION_ID, typologyFromStepLayer } = dependencies;

  const { describe, expect, it } = vitest;
  const { runtime } = await import("@semio-tech/cad-js");

  runtime.bootstrapCadModules();

  describe("@semio-tech/cad-js-module-aec-building", () => {
    it("maps STEP layer names to building typologies", () => {
      expect(typologyFromStepLayer("Beams", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.beam");
      expect(typologyFromStepLayer("Column", AEC_BUILDING_MODEL_DEFINITION_ID)).toBe("building.building.column");
    });
  });

}
