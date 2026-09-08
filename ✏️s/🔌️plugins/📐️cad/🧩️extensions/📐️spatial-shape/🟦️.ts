// #region 🧭️Header
/** @emoji 📐️ `@semio-tech/cad-js-module-spatial-shape` — shape model-definition stat and property computers. */
// #endregion 🧭️Header

import { core } from "@semio-tech/cad-js";
const { bboxSizesFromPositions, collectFaceRefsForObjects, collectSolidRefsForObjects, collectVertexPositionsForObjects, registerPropertyComputer, registerStatComputer, resolvePrimitiveRefKind, solidRef } = core;
type FaceRef = core.FaceRef;
type PropertyComputeContext = core.PropertyComputeContext;
type SolidRef = core.SolidRef;
type StatComputeContext = core.StatComputeContext;
type TypologyRef = core.TypologyRef;

// #region 🏷️Ids
export const SPATIAL_SHAPE_MODEL_DEFINITION_ID = "spatial.shape";
export const SPATIAL_SHAPE_GEOMETRY_STAT_ID = "spatial.shape.geometry";
export const SPATIAL_SHAPE_VOLUME_PROPERTY_ID = "spatial.shape.volume";
// #endregion 🏷️Ids

// #region 📊️StatComputer
async function computeShapeGeometryStat(ctx: StatComputeContext): Promise<Record<string, number>> {
  const solidIds = collectSolidRefsForObjects(ctx.model, ctx.objects);
  const faceIds = collectFaceRefsForObjects(ctx.model, ctx.objects);
  await ctx.kernel.syncSolidsFromModel(ctx.model);
  let totalVolume = 0;
  for (const solidId of solidIds) totalVolume += await ctx.kernel.solidVolume(solidRef(solidId));
  let totalSurfaceArea = 0;
  for (const faceId of faceIds) totalSurfaceArea += await ctx.kernel.faceArea(faceId as FaceRef, ctx.model);
  const bbox = bboxSizesFromPositions(collectVertexPositionsForObjects(ctx.model, ctx.objects));
  return {
    objectCount: ctx.objects.length,
    solidCount: solidIds.length,
    totalVolume,
    totalSurfaceArea,
    sizeX: bbox.sizeX,
    sizeY: bbox.sizeY,
    sizeZ: bbox.sizeZ,
  };
}
// #endregion 📊️StatComputer

// #region 📐️PropertyComputer
async function computeSolidVolumeProperty(ctx: PropertyComputeContext, outputKey: string): Promise<Record<string, unknown>> {
  const primaryPrimitiveRef = Object.values(ctx.object.primitives).find(Boolean) ?? null;
  const kind = primaryPrimitiveRef ? resolvePrimitiveRefKind(ctx.model, primaryPrimitiveRef) : null;
  if (kind !== "solid") return { [outputKey]: 0 };
  await ctx.kernel.syncSolidsFromModel(ctx.model);
  const amount = await ctx.kernel.solidVolume(primaryPrimitiveRef as SolidRef);
  return { [outputKey]: amount };
}

async function computeShapeVolumeProperty(ctx: PropertyComputeContext): Promise<Record<string, unknown>> {
  return computeSolidVolumeProperty(ctx, "volume");
}
// #endregion 📐️PropertyComputer

// #region 📦️Register
/** @emoji 📦️ Registers spatial.shape stat and property computers on the core engine. */
export function register(): void {
  registerStatComputer(SPATIAL_SHAPE_GEOMETRY_STAT_ID, computeShapeGeometryStat);
  registerPropertyComputer(SPATIAL_SHAPE_VOLUME_PROPERTY_ID, computeShapeVolumeProperty);
}
// #endregion 📦️Register

// #region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-module-spatial-shape/🟦️.ts");
  await registerTests1(import.meta.vitest, { SPATIAL_SHAPE_GEOMETRY_STAT_ID, SPATIAL_SHAPE_VOLUME_PROPERTY_ID, core, solidRef }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🧪️Tests
