// #region 🧭️Header
/** @emoji 🚀️ `@semio-tech/cad-js/runtime` — CAD composition root: assets glob + module registration. */
// #endregion 🧭️Header

import { ephemeralBox } from "@semio-tech/framework";
import type { ProgramContributionEntry } from "@semio-tech/framework";
import { registerModelDefinitionAssets, type ModelDefinitionAssetModules } from "../🟦️index.ts";
import * as spatialShape from "@semio-tech/cad-js-module-spatial-shape";
import * as aecBuilding from "@semio-tech/cad-js-module-aec-building";
import * as aecBuildingEnergy from "@semio-tech/cad-js-module-aec-building-energy";
import * as aecBuildingStructure from "@semio-tech/cad-js-module-aec-building-structure";

// #region 🧩️Contributions
export const CAD_PLAY_APP_ID = "cad-play";

/** @emoji 📋️ Declarative computer/import metadata carried in `Contribution::CadComputer.computersJson`. */
export type CadComputersManifest = {
  modelDefinitionIds: string[];
  statComputers: string[];
  propertyComputers: string[];
  importProfiles: readonly {
    modelDefinitionId: string;
    layerTypology: Record<string, string>;
    fallbackTypology: string;
    preferPresentationLayers?: boolean;
    presentationGeometry?: string;
    namespacedDomain?: string;
  }[];
  transformationAppliers: string[];
};

const CAD_MODULE_REGISTRARS: Readonly<Record<string, () => void>> = {
  "spatial-shape": () => spatialShape.register(),
  "aec-building": () => aecBuilding.register(),
  "aec-building-energy": () => aecBuildingEnergy.register(),
  "aec-building-structure": () => aecBuildingStructure.register(),
};

const cadComputerModulesSynced = ephemeralBox<Set<string>>("s.plugins.cad.modules.runtime.component.ts.cadComputerModulesSynced", new Set());

/** @emoji 🚢️ Default shipped `ProgramContributionEntry[]` JSON when the host has not pushed contributions yet. */
export function shippedCadComputerContributionsJson(): string {
  const entries: ProgramContributionEntry[] = [
    {
      pluginId: "cad-extension-spatial-shape",
      contribution: {
        kind: "cadComputer",
        appId: CAD_PLAY_APP_ID,
        moduleId: "spatial-shape",
        label: "Spatial Shape",
        iconId: "box",
        computersJson: JSON.stringify({
          modelDefinitionIds: ["spatial.shape"],
          statComputers: ["spatial.shape.geometry"],
          propertyComputers: ["spatial.shape.volume"],
          importProfiles: [],
          transformationAppliers: [],
        } satisfies CadComputersManifest),
      },
    },
    {
      pluginId: "cad-extension-aec-building",
      contribution: {
        kind: "cadComputer",
        appId: CAD_PLAY_APP_ID,
        moduleId: "aec-building",
        label: "AEC Building",
        iconId: "building",
        computersJson: JSON.stringify({
          modelDefinitionIds: ["aec.building"],
          statComputers: [],
          propertyComputers: [],
          importProfiles: [{ modelDefinitionId: "aec.building", layerTypology: {}, fallbackTypology: "building.building.slab" }],
          transformationAppliers: [],
        } satisfies CadComputersManifest),
      },
    },
    {
      pluginId: "cad-extension-aec-building-energy",
      contribution: {
        kind: "cadComputer",
        appId: CAD_PLAY_APP_ID,
        moduleId: "aec-building-energy",
        label: "AEC Building Energy",
        iconId: "zap",
        computersJson: JSON.stringify({
          modelDefinitionIds: ["aec.building.energy"],
          statComputers: ["energy.demand"],
          propertyComputers: ["energy.heatedvolume"],
          importProfiles: [],
          transformationAppliers: [],
        } satisfies CadComputersManifest),
      },
    },
    {
      pluginId: "cad-extension-aec-building-structure",
      contribution: {
        kind: "cadComputer",
        appId: CAD_PLAY_APP_ID,
        moduleId: "aec-building-structure",
        label: "AEC Building Structure",
        iconId: "landmark",
        computersJson: JSON.stringify({
          modelDefinitionIds: ["aec.building.structure"],
          statComputers: ["structure.stability"],
          propertyComputers: [],
          importProfiles: [],
          transformationAppliers: ["aec.building.structure/from_building"],
        } satisfies CadComputersManifest),
      },
    },
  ];
  return JSON.stringify(entries);
}

/** @emoji 🧩️ Applies `cad.computer` contributions by `moduleId` via existing cad-js `register()` hooks. */
export function syncCadComputerContributions(contributionsJson: string): void {
  let entries: ProgramContributionEntry[];
  try {
    entries = JSON.parse(contributionsJson) as ProgramContributionEntry[];
  } catch {
    return;
  }
  const synced = cadComputerModulesSynced.current;
  for (const { contribution } of entries) {
    if (contribution.kind !== "cadComputer" || contribution.appId !== CAD_PLAY_APP_ID) continue;
    if (synced.has(contribution.moduleId)) continue;
    const register = CAD_MODULE_REGISTRARS[contribution.moduleId];
    if (!register) continue;
    register();
    synced.add(contribution.moduleId);
  }
}
// #endregion 🧩️Contributions

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

/** @emoji 🚀️ Loads model-definition assets and registers CAD computer modules from contributions once. */
export function bootstrapCadModules(contributionsJson?: string): void {
  if (cadModulesBootstrapped.current) return;
  registerModelDefinitionAssets(shippedModelDefinitionAssets());
  syncCadComputerContributions(contributionsJson ?? shippedCadComputerContributionsJson());
  cadModulesBootstrapped.current = true;
}

export { spatialShape, aecBuilding, aecBuildingEnergy, aecBuildingStructure };

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { defaultModelDefinitionId, listModelDefinitionManifests, listTransformationsIntoModelDefinition, loadPropertyDefinition, loadStatDefinition, resolveModelDefinitionScope } = await import("../🟦️index.ts");
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
