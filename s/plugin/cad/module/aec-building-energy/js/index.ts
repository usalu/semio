// #region 🧭Header
/** @emoji ⚡ `@semio-tech/cad-js-module-aec-building-energy` — energy stat, property, and STEP import profile. */
// #endregion 🧭Header

import {
  collectFaceRefsForObjects,
  collectSolidRefsForObjects,
  registerImportProfile,
  registerPropertyComputer,
  registerStatComputer,
  resolvePrimitiveRefKind,
  solidRef,
  type FaceRef,
  type PropertyComputeContext,
  type SolidRef,
  type StatComputeContext,
  type TypologyRef,
} from "@semio-tech/cad-js-core";

// #region 🏷️Ids
export const AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID = "aec.building.energy";
export const ENERGY_DEMAND_STAT_ID = "energy.demand";
export const ENERGY_HEATEDVOLUME_PROPERTY_ID = "energy.heatedvolume";
// #endregion 🏷️Ids

// #region 🪪ImportProfile
const ENERGY_LAYER_TYPOLOGY: Readonly<Record<string, TypologyRef>> = {
  slab: "energy.energy.baseplate",
  baseplate: "energy.energy.baseplate",
  roof: "energy.energy.roof",
  wall: "energy.energy.externalwall",
  walls: "energy.energy.externalwall",
  hull: "energy.energy.hull",
  window: "energy.energy.windows",
  windows: "energy.energy.windows",
};
// #endregion 🪪ImportProfile

// #region 📊StatComputer
const ENERGY_DEFAULT_U_VALUE = 0.3;
const ENERGY_HEATING_DEGREE_HOURS = 3000;
const ENERGY_VENTILATION_FACTOR = 12;

async function computeEnergyDemandStat(ctx: StatComputeContext): Promise<Record<string, number>> {
  const solidIds = collectSolidRefsForObjects(ctx.model, ctx.objects);
  const faceIds = collectFaceRefsForObjects(ctx.model, ctx.objects);
  await ctx.kernel.syncSolidsFromModel(ctx.model);
  let heatedVolume = 0;
  for (const solidId of solidIds) heatedVolume += await ctx.kernel.solidVolume(solidRef(solidId));
  let envelopeArea = 0;
  for (const faceId of faceIds) envelopeArea += await ctx.kernel.faceArea(faceId as FaceRef, ctx.model);
  const transmissionLoss = envelopeArea * ENERGY_DEFAULT_U_VALUE * ENERGY_HEATING_DEGREE_HOURS;
  const ventilationLoss = heatedVolume * ENERGY_VENTILATION_FACTOR;
  const annualHeatingDemand = transmissionLoss + ventilationLoss;
  const specificDemand = heatedVolume > 0 ? annualHeatingDemand / heatedVolume : 0;
  return { heatedVolume, envelopeArea, annualHeatingDemand, specificDemand };
}
// #endregion 📊StatComputer

// #region 📐PropertyComputer
async function computeHeatedVolumeProperty(ctx: PropertyComputeContext): Promise<Record<string, unknown>> {
  const primaryPrimitiveRef = Object.values(ctx.object.primitives).find(Boolean) ?? null;
  const kind = primaryPrimitiveRef ? resolvePrimitiveRefKind(ctx.model, primaryPrimitiveRef) : null;
  if (kind !== "solid") return { heatedvolume: 0 };
  await ctx.kernel.syncSolidsFromModel(ctx.model);
  const amount = await ctx.kernel.solidVolume(primaryPrimitiveRef as SolidRef);
  return { heatedvolume: amount };
}
// #endregion 📐PropertyComputer

// #region 📦Register
/** @emoji 📦 Registers energy stat, property, and STEP import profile on the core engine. */
export function register(): void {
  registerStatComputer(ENERGY_DEMAND_STAT_ID, computeEnergyDemandStat);
  registerPropertyComputer(ENERGY_HEATEDVOLUME_PROPERTY_ID, computeHeatedVolumeProperty);
  registerImportProfile(AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, {
    layerTypology: ENERGY_LAYER_TYPOLOGY,
    fallbackTypology: "energy.energy.hull" as TypologyRef,
    preferPresentationLayers: true,
    presentationGeometry: "wireframe",
    namespacedDomain: "energy",
  });
}
// #endregion 📦Register

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { bootstrapCadModules } = await import("@semio-tech/cad-js-runtime");
  const { BrepjsKernel, preciseSpatialKernelMath } = await import("@semio-tech/cad-js-kernel-brepjs");
  const core = await import("@semio-tech/cad-js-core");
  const { Model, applyModelDiff, computeStat, loadStatDefinition, objectsForStatCompute, solidRef } = core;
  type ObjectRef = core.ObjectRef;
  type SpatialKernel = core.SpatialKernel;
  type TypologyRef = core.TypologyRef;

  bootstrapCadModules();
  const M = preciseSpatialKernelMath;

  describe("@semio-tech/cad-js-module-aec-building-energy", () => {
    it("computes energy demand stats with finite outputs", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("hull-solid")));
      model.objects["hull"] = {
        id: "hull" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: "hull-solid" },
      };
      const defn = loadStatDefinition(ENERGY_DEMAND_STAT_ID)!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const out = await computeStat(defn, {
        model,
        kernel,
        modelDefinitionId: AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID,
        scope: "model",
        objects: objectsForStatCompute(model, AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, defn, "model", []),
      });
      expect(Number.isFinite(out.heatedVolume)).toBe(true);
      expect(out.heatedVolume).toBeGreaterThan(0);
    });
  });
}
// #endregion 🧪Tests
