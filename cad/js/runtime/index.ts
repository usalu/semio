// #region 🧭Header
/** @emoji 🚀 `@cad/js/runtime` — CAD composition root: assets glob + module registration. */
// #endregion 🧭Header

import { registerModelDefinitionAssets, type ModelDefinitionAssetModules } from "@cad/js/core";
import * as spatialShape from "@cad/js/module/spatial-shape";
import * as aecBuilding from "@cad/js/module/aec-building";
import * as aecBuildingEnergy from "@cad/js/module/aec-building-energy";
import * as aecBuildingStructure from "@cad/js/module/aec-building-structure";

// #region 📥ModelDefinitionAssets
const __modelDefinitionTypologyModules = import.meta.glob(
  ["../../assets/modelDefinition/**/typology.json", "../../assets/modelDefinition/**/typology/*.json"],
  { eager: true, import: "default" },
) as Record<string, unknown>;

const __modelDefinitionActionModules = import.meta.glob("../../assets/modelDefinition/**/action/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionInteractionModules = import.meta.glob("../../assets/modelDefinition/**/interaction/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionManifestModules = import.meta.glob("../../assets/modelDefinition/**/modelDefinition.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionAttributeModules = import.meta.glob("../../assets/modelDefinition/**/attributeDefinition/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionPropertyDefinitionModules = import.meta.glob(
  ["../../assets/modelDefinition/**/propertyDefinition/*.json", "../../assets/modelDefinition/**/propertyKind/*.json"],
  { eager: true, import: "default" },
) as Record<string, unknown>;

const __modelDefinitionPropertyModules = import.meta.glob("../../assets/modelDefinition/**/property/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionStatDefinitionModules = import.meta.glob("../../assets/modelDefinition/**/statDefinition/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionTransformationModules = import.meta.glob("../../assets/modelDefinition/**/transformation/**/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const __modelDefinitionExtensionManifestModules = import.meta.glob("../../assets/modelDefinition/**/extension.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

function shippedModelDefinitionAssets(): ModelDefinitionAssetModules {
  return {
    typologies: __modelDefinitionTypologyModules,
    actions: __modelDefinitionActionModules,
    interactions: __modelDefinitionInteractionModules,
    manifests: __modelDefinitionManifestModules,
    extensions: __modelDefinitionExtensionManifestModules,
    attributes: __modelDefinitionAttributeModules,
    propertyDefinitions: __modelDefinitionPropertyDefinitionModules,
    properties: __modelDefinitionPropertyModules,
    statDefinitions: __modelDefinitionStatDefinitionModules,
    transformations: __modelDefinitionTransformationModules,
  };
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

export {
  spatialShape,
  aecBuilding,
  aecBuildingEnergy,
  aecBuildingStructure,
};

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const {
    defaultModelDefinitionId,
    listModelDefinitionManifests,
    listTransformationsIntoModelDefinition,
    loadPropertyDefinition,
    loadStatDefinition,
    resolveModelDefinitionScope,
  } = await import("@cad/js/core");
  const { SPATIAL_SHAPE_VOLUME_PROPERTY_ID } = await import("@cad/js/module/spatial-shape");
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, ENERGY_DEMAND_STAT_ID } = await import("@cad/js/module/aec-building-energy");

  bootstrapCadModules();

  describe("@cad/js/runtime", () => {
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
