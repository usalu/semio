// #region 🧭️Header
/** @emoji ⚡️ `@semio-tech/cad-js-module-aec-building-energy` — energy stat, property, and STEP import profile. */
// #endregion 🧭️Header

import { core } from "@semio-tech/cad-js";
const { collectFaceRefsForObjects, collectSolidRefsForObjects, registerImportProfile, registerPropertyComputer, registerStatComputer, resolvePrimitiveRefKind, solidRef } = core;
type FaceRef = core.FaceRef;
type PropertyComputeContext = core.PropertyComputeContext;
type SolidRef = core.SolidRef;
type StatComputeContext = core.StatComputeContext;
type TypologyRef = core.TypologyRef;

// #region 🏷️Ids
export const AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID = "aec.building.energy";
export const ENERGY_DEMAND_STAT_ID = "energy.demand";
export const ENERGY_HEATEDVOLUME_PROPERTY_ID = "energy.heatedvolume";
// #endregion 🏷️Ids

// #region 🪪️ImportProfile
const ENERGY_LAYER_TYPOLOGY: Readonly<Record<string, TypologyRef>> = {
  slab: "energy.energy.baseplate" as TypologyRef,
  baseplate: "energy.energy.baseplate" as TypologyRef,
  roof: "energy.energy.roof" as TypologyRef,
  wall: "energy.energy.externalwall" as TypologyRef,
  walls: "energy.energy.externalwall" as TypologyRef,
  hull: "energy.energy.hull" as TypologyRef,
  window: "energy.energy.windows" as TypologyRef,
  windows: "energy.energy.windows" as TypologyRef,
};
// #endregion 🪪️ImportProfile

// #region 📊️StatComputer
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
// #endregion 📊️StatComputer

// #region 📐️PropertyComputer
async function computeHeatedVolumeProperty(ctx: PropertyComputeContext): Promise<Record<string, unknown>> {
  const primaryPrimitiveRef = Object.values(ctx.object.primitives).find(Boolean) ?? null;
  const kind = primaryPrimitiveRef ? resolvePrimitiveRefKind(ctx.model, primaryPrimitiveRef) : null;
  if (kind !== "solid") return { heatedvolume: 0 };
  await ctx.kernel.syncSolidsFromModel(ctx.model);
  const amount = await ctx.kernel.solidVolume(primaryPrimitiveRef as SolidRef);
  return { heatedvolume: amount };
}
// #endregion 📐️PropertyComputer

// #region 📦️Register
/** @emoji 📦️ Registers energy stat, property, and STEP import profile on the core engine. */
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
// #endregion 📦️Register

// #region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-module-aec-building-energy/🟦️.ts");
  await registerTests1(import.meta.vitest, { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, ENERGY_DEMAND_STAT_ID, core, solidRef }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🧪️Tests
