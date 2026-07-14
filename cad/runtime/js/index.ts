// #region 🧭Header
/** @emoji 🚀 `@semio-tech/cad-js-runtime` — CAD composition root: assets glob + module registration. */
// #endregion 🧭Header

import { registerModelDefinitionAssets, type ModelDefinitionAssetModules } from "@semio-tech/cad-js-core";
import * as spatialShape from "@semio-tech/cad-js-module-spatial-shape";
import * as aecBuilding from "@semio-tech/cad-js-module-aec-building";
import * as aecBuildingEnergy from "@semio-tech/cad-js-module-aec-building-energy";
import * as aecBuildingStructure from "@semio-tech/cad-js-module-aec-building-structure";

// #region 📥ModelDefinitionAssets
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

let shippedModelDefinitionAssetsCache: ModelDefinitionAssetModules | null = null;

function shippedModelDefinitionAssets(): ModelDefinitionAssetModules {
  if (shippedModelDefinitionAssetsCache) return shippedModelDefinitionAssetsCache;
  console.log("[DEBUG] typeof import.meta.glob:", typeof import.meta.glob);
  if (typeof import.meta.glob !== "function") {
    shippedModelDefinitionAssetsCache = emptyModelDefinitionAssets();
    return shippedModelDefinitionAssetsCache;
  }
  shippedModelDefinitionAssetsCache = {
    typologies: import.meta.glob(["../../asset/modelDefinition/**/typology.json", "../../asset/modelDefinition/**/typology/*.json"], {
      eager: true,
      import: "default",
    }) as Record<string, unknown>,
    actions: import.meta.glob("../../asset/modelDefinition/**/action/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    interactions: import.meta.glob("../../asset/modelDefinition/**/interaction/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    manifests: import.meta.glob("../../asset/modelDefinition/**/modelDefinition.json", { eager: true, import: "default" }) as Record<string, unknown>,
    extensions: import.meta.glob("../../asset/modelDefinition/**/extension.json", { eager: true, import: "default" }) as Record<string, unknown>,
    attributes: import.meta.glob("../../asset/modelDefinition/**/attributeDefinition/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    propertyDefinitions: import.meta.glob(["../../asset/modelDefinition/**/propertyDefinition/*.json", "../../asset/modelDefinition/**/propertyKind/*.json"], { eager: true, import: "default" }) as Record<string, unknown>,
    properties: import.meta.glob("../../asset/modelDefinition/**/property/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    statDefinitions: import.meta.glob("../../asset/modelDefinition/**/statDefinition/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    transformations: import.meta.glob("../../asset/modelDefinition/**/transformation/**/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
  };
  return shippedModelDefinitionAssetsCache;
}
// #endregion 📥ModelDefinitionAssets

let cadModulesBootstrapped = false;

/** @emoji 🚀 Loads model-definition assets and registers all CAD modules once. */
export function bootstrapCadModules(): void {
  if (cadModulesBootstrapped) return;
  registerModelDefinitionAssets(shippedModelDefinitionAssets());
  spatialShape.register();
  aecBuilding.register();
  aecBuildingEnergy.register();
  aecBuildingStructure.register();
  cadModulesBootstrapped = true;
  console.log("[DEBUG] bootstrapCadModules: spatial-shape, aec-building, aec-building-energy, aec-building-structure");
}

export { spatialShape, aecBuilding, aecBuildingEnergy, aecBuildingStructure };

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { defaultModelDefinitionId, listModelDefinitionManifests, listTransformationsIntoModelDefinition, loadPropertyDefinition, loadStatDefinition, resolveModelDefinitionScope } = await import("@semio-tech/cad-js-core");
  const { SPATIAL_SHAPE_VOLUME_PROPERTY_ID } = await import("@semio-tech/cad-js-module-spatial-shape");
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, ENERGY_DEMAND_STAT_ID } = await import("@semio-tech/cad-js-module-aec-building-energy");

  bootstrapCadModules();

  describe("@semio-tech/cad-js-runtime", () => {
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
// #endregion 🧪Tests
