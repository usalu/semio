type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { bootstrapCadModules } = dependencies;

  const { describe, expect, it } = vitest;
  const { defaultModelDefinitionId, listModelDefinitionManifests, listTransformationsIntoModelDefinition, loadPropertyDefinition, loadStatDefinition, resolveModelDefinitionScope } = await import("../../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts");
  const { SPATIAL_SHAPE_VOLUME_PROPERTY_ID } = await import("@semio-tech/cad-js-module-spatial-shape");
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, ENERGY_DEMAND_STAT_ID } = await import("@semio-tech/cad-js-module-aec-building-energy");

  bootstrapCadModules();

  describe("@semio-tech/cad-js/runtime", () => {
    it("loads model definition manifests and catalogs", () => {
      const manifests = listModelDefinitionManifests();
      expect(manifests.some((row) => row.id === defaultModelDefinitionId())).toBe(true);
      expect(manifests.some((row) => row.id === AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID)).toBe(true);
      expect(loadPropertyDefinition(SPATIAL_SHAPE_VOLUME_PROPERTY_ID)?.unit).toBe("volume");
      expect(loadStatDefinition(ENERGY_DEMAND_STAT_ID)?.outputs.some((row) => row.key === "heatedVolume")).toBe(true);
      expect(listTransformationsIntoModelDefinition(AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID).some((row) => row.id === "from_geometry")).toBe(true);
    });

    it("scopes catalogs to active model definition", () => {
      const shape = resolveModelDefinitionScope(defaultModelDefinitionId());
      const energy = resolveModelDefinitionScope(AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID);
      expect(shape.interactions.some((row) => row.id === "primitive.box")).toBe(true);
      expect(energy.interactions.every((row) => row.id.startsWith("energy.energy.construct"))).toBe(true);
      expect(energy.typologies.some((row) => row.id === "energy.energy.hull")).toBe(true);
    });
  });

}
