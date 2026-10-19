import type { RuntimeTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";
import * as vitestRuntime from "vitest";
import { bootstrapCadModules } from "../../🟦️.ts";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: RuntimeTestDependencies, source: TestSource): Promise<void> {
  const { bootstrapCadModules } = dependencies;

  const { describe, expect, it } = vitest;
  const { defaultModelDefinitionId, listModelDefinitionManifests } = await import("@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog");
  const { listTransformationsIntoModelDefinition } = await import("@semio-tech/s-spatial-kernel-ts/geometry/transformation-spec");
  const { loadPropertyDefinition } = await import("@semio-tech/s-spatial-kernel-ts/geometry/typology");
  const { loadStatDefinition } = await import("@semio-tech/s-spatial-kernel-ts/geometry/stats");
  const { resolveModelDefinitionScope } = await import("@semio-tech/s-spatial-kernel-ts/geometry/selection-operations");
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

const dependencies: RuntimeTestDependencies = { bootstrapCadModules };
await registerTests1(vitestRuntime, dependencies, { url: new URL("../../🟦️.ts", import.meta.url).href });
