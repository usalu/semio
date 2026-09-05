// #region 🧭️Header
/** @emoji 🚀️ `@semio-tech/cad-js/runtime` — CAD composition root: assets glob + module registration. */
// #endregion 🧭️Header

import { ephemeralBox } from "@semio-tech/framework";
import type { ProgramContributionEntry } from "@semio-tech/framework";
import { registerModelDefinitionAssets, type ModelDefinitionAssetModules } from "../📔️registry/🟦️.ts";
import * as spatialShape from "@semio-tech/cad-js-module-spatial-shape";
import * as aecBuilding from "@semio-tech/cad-js-module-aec-building";
import * as aecBuildingEnergy from "@semio-tech/cad-js-module-aec-building-energy";
import * as aecBuildingStructure from "@semio-tech/cad-js-module-aec-building-structure";

// #region 🧩️Contributions
export const CAD_PLAY_APP_ID = "cad-play";

/** @emoji 📋️ Declarative computer/import metadata carried in the `cadComputer` `TopicContribution` payload's
 * `computersJson` field (ex `Contribution::CadComputer.computersJson`, pre open-contribution-mechanism migration). */
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

/** @emoji 🗂️ `cad.computer` open-`TopicContribution` payload shape — mirrors Rust `CadComputerTopicPayload` (`💡️inferences/🦀️.rs`). */
const CAD_COMPUTER_TOPIC = "cad.computer";

type CadComputerTopicPayload = {
  readonly appId: string;
  readonly moduleId: string;
  readonly computersJson: string;
};

/** @emoji 🚢️ Default shipped `ProgramContributionEntry[]` JSON when the host has not pushed contributions yet. */
export function shippedCadComputerContributionsJson(): string {
  const entries: ProgramContributionEntry[] = [
    {
      pluginId: "cad-extension-spatial-shape",
      topicContribution: {
        topic: CAD_COMPUTER_TOPIC,
        payload: {
          appId: CAD_PLAY_APP_ID,
          moduleId: "spatial-shape",
          computersJson: JSON.stringify({
            modelDefinitionIds: ["spatial.shape"],
            statComputers: ["spatial.shape.geometry"],
            propertyComputers: ["spatial.shape.volume"],
            importProfiles: [],
            transformationAppliers: [],
          } satisfies CadComputersManifest),
        } satisfies CadComputerTopicPayload,
      },
    },
    {
      pluginId: "cad-extension-aec-building",
      topicContribution: {
        topic: CAD_COMPUTER_TOPIC,
        payload: {
          appId: CAD_PLAY_APP_ID,
          moduleId: "aec-building",
          computersJson: JSON.stringify({
            modelDefinitionIds: ["aec.building"],
            statComputers: [],
            propertyComputers: [],
            importProfiles: [{ modelDefinitionId: "aec.building", layerTypology: {}, fallbackTypology: "building.building.slab" }],
            transformationAppliers: [],
          } satisfies CadComputersManifest),
        } satisfies CadComputerTopicPayload,
      },
    },
    {
      pluginId: "cad-extension-aec-building-energy",
      topicContribution: {
        topic: CAD_COMPUTER_TOPIC,
        payload: {
          appId: CAD_PLAY_APP_ID,
          moduleId: "aec-building-energy",
          computersJson: JSON.stringify({
            modelDefinitionIds: ["aec.building.energy"],
            statComputers: ["energy.demand"],
            propertyComputers: ["energy.heatedvolume"],
            importProfiles: [],
            transformationAppliers: [],
          } satisfies CadComputersManifest),
        } satisfies CadComputerTopicPayload,
      },
    },
    {
      pluginId: "cad-extension-aec-building-structure",
      topicContribution: {
        topic: CAD_COMPUTER_TOPIC,
        payload: {
          appId: CAD_PLAY_APP_ID,
          moduleId: "aec-building-structure",
          computersJson: JSON.stringify({
            modelDefinitionIds: ["aec.building.structure"],
            statComputers: ["structure.stability"],
            propertyComputers: [],
            importProfiles: [],
            transformationAppliers: ["aec.building.structure/from_building"],
          } satisfies CadComputersManifest),
        } satisfies CadComputerTopicPayload,
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
  for (const { topicContribution } of entries) {
    if (topicContribution?.topic !== CAD_COMPUTER_TOPIC) continue;
    const payload = topicContribution.payload as Partial<CadComputerTopicPayload> | null;
    if (!payload || payload.appId !== CAD_PLAY_APP_ID || typeof payload.moduleId !== "string") continue;
    if (synced.has(payload.moduleId)) continue;
    const register = CAD_MODULE_REGISTRARS[payload.moduleId];
    if (!register) continue;
    register();
    synced.add(payload.moduleId);
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
  // `🖼️assets/🏗️modelDefinitions/<modelDefinition>/{🎬️actions,🕹️interactions,🗂️typologies/*/🔣️typology.json,
  // 🏷️attributeDefinitions,🔧️propertyDefinitions,🏷️propertyKinds,📊️statDefinitions,🔀️transformations,🔣️modelDefinition.json}`.
  shippedModelDefinitionAssetsCache.current = {
    typologies: import.meta.glob(["../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🗂️typologies/**/🔣️typology.json"], {
      eager: true,
      import: "default",
    }) as Record<string, unknown>,
    actions: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🎬️actions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    interactions: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🕹️interactions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    manifests: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🔣️modelDefinition.json", { eager: true, import: "default" }) as Record<string, unknown>,
    extensions: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🔣️extension.json", { eager: true, import: "default" }) as Record<string, unknown>,
    attributes: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🏷️attributeDefinitions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    propertyDefinitions: import.meta.glob(["../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🔧️propertyDefinitions/*.json", "../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🏷️propertyKinds/*.json"], { eager: true, import: "default" }) as Record<string, unknown>,
    properties: import.meta.glob(["../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🏷️properties/*.json", "../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🔧️properties/*.json"], { eager: true, import: "default" }) as Record<string, unknown>,
    statDefinitions: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/📊️statDefinitions/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
    transformations: import.meta.glob("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/**/🔀️transformations/**/*.json", { eager: true, import: "default" }) as Record<string, unknown>,
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
  const { defaultModelDefinitionId, listModelDefinitionManifests, listTransformationsIntoModelDefinition, loadPropertyDefinition, loadStatDefinition, resolveModelDefinitionScope } = await import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts");
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
