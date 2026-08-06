// #region 🧭️Header
/** @emoji 🚀️ `@semio-tech/cad-js/runtime` — CAD composition root: assets glob + module registration. */
// #endregion 🧭️Header

import { ephemeralBox } from "@semio-tech/framework-core";
import { registerModelDefinitionAssets, type ModelDefinitionAssetModules } from "../🫀️core/🟦️component.ts";
import * as spatialShape from "@semio-tech/cad-js-module-spatial-shape";
import * as aecBuilding from "@semio-tech/cad-js-module-aec-building";
import * as aecBuildingEnergy from "@semio-tech/cad-js-module-aec-building-energy";
import * as aecBuildingStructure from "@semio-tech/cad-js-module-aec-building-structure";

// #region 📥️ModelDefinitionAssets
const emptyModelDefinitionAssets = (): ModelDefinitionAssetModules => ({
  typologies: {},
  actions: {},
  interactions: {},
  manifests: {},
  extensions: {},
  attributes: {},
  propertyDefinitions: {},
  properties: {},
  statDefinitions: {},
  transformations: {},
});

const shippedModelDefinitionAssetsCache = ephemeralBox<ModelDefinitionAssetModules | null>("s.plugins.cad.modules.runtime.component.ts.shippedModelDefinitionAssetsCache", null);

function shippedModelDefinitionAssets(): ModelDefinitionAssetModules {
  if (shippedModelDefinitionAssetsCache.current) return shippedModelDefinitionAssetsCache.current;
  if (typeof import.meta.glob !== "function") {
    shippedModelDefinitionAssetsCache.current = emptyModelDefinitionAssets();
    return shippedModelDefinitionAssetsCache.current;
  }
  // 🩹 Base was stale `./asset/modelDefinition` (ASCII, singular — pre-dates the emoji/plural asset-tree rename;
  // never matched anything, and being relative-without-`./` even threw a vite:import-glob parse error). Real tree:
  // `🖼️assets/🏗️modelDefinitions/<modelDefinition>/{🎬️actions,🎬️interactions,🗂️typologies/*/🔣️typology.json,
  // 🏷️attributeDefinitions,🔧️propertyDefinitions,🏷️propertyKinds,📊️statDefinitions,🔀️transformations,🔣️modelDefinition.json}`.
  shippedModelDefinitionAssetsCache.current = {
    typologies: import.meta.glob(["../../🖼️assets/🏗️modelDefinitions/**/🗂️typologies/**/🔣️typology.json"], {
      eager: true,
      import: "default",
    }) as Record<string, unknown>,
    actions: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/🎬️actions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    interactions: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/🎬️interactions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    manifests: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/🔣️modelDefinition.json", { eager: true, import: "default" }) as Record<string, unknown>,
    extensions: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/🔣️extension.json", { eager: true, import: "default" }) as Record<string, unknown>,
    attributes: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/🏷️attributeDefinitions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    propertyDefinitions: import.meta.glob(["../../🖼️assets/🏗️modelDefinitions/**/🔧️propertyDefinitions/*.json", "../../🖼️assets/🏗️modelDefinitions/**/🏷️propertyKinds/*.json"], { eager: true, import: "default" }) as Record<string, unknown>,
    properties: import.meta.glob(["../../🖼️assets/🏗️modelDefinitions/**/🏷️properties/*.json", "../../🖼️assets/🏗️modelDefinitions/**/🔧️properties/*.json"], { eager: true, import: "default" }) as Record<string, unknown>,
    statDefinitions: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/📊️statDefinitions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    transformations: import.meta.glob("../../🖼️assets/🏗️modelDefinitions/**/🔀️transformations/**/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
  };
  return shippedModelDefinitionAssetsCache.current;
}
// #endregion 📥️ModelDefinitionAssets

const cadModulesBootstrapped = ephemeralBox("s.plugins.cad.modules.runtime.component.ts.cadModulesBootstrapped", false);

/** @emoji 🚀️ Loads model-definition assets and registers all CAD modules once. */
export function bootstrapCadModules(): void {
  if (cadModulesBootstrapped.current) return;
  registerModelDefinitionAssets(shippedModelDefinitionAssets());
  spatialShape.register();
  aecBuilding.register();
  aecBuildingEnergy.register();
  aecBuildingStructure.register();
  cadModulesBootstrapped.current = true;
}

export { spatialShape, aecBuilding, aecBuildingEnergy, aecBuildingStructure };

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { defaultModelDefinitionId, listModelDefinitionManifests, listTransformationsIntoModelDefinition, loadPropertyDefinition, loadStatDefinition, resolveModelDefinitionScope } = await import("../🫀️core/🟦️component.ts");
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
// #endregion 🧪️Tests
