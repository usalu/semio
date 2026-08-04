// #region 🧭️Header
/** @emoji 📐️ `@semio-tech/cad-js-module-spatial-shape` — shape model-definition stat and property computers. */
// #endregion 🧭️Header

import {
  bboxSizesFromPositions,
  collectFaceRefsForObjects,
  collectSolidRefsForObjects,
  collectVertexPositionsForObjects,
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
  const { describe, expect, it } = import.meta.vitest;
  const { bootstrapCadModules } = await import("@semio-tech/cad-js-runtime");
  const { BrepjsKernel, preciseSpatialKernelMath } = await import("@semio-tech/cad-js-kernel-brepjs");
  const core = await import("@semio-tech/cad-js-core");
  const { Model, applyModelDiff, computeStat, defaultModelDefinitionId, derivePropertyValue, loadPropertyDefinition, loadStatDefinition, objectsForStatCompute } = core;
  type ObjectRef = core.ObjectRef;
  type SpatialKernel = core.SpatialKernel;
  type TypologyRef = core.TypologyRef;

  bootstrapCadModules();
  const M = preciseSpatialKernelMath;

  describe("@semio-tech/cad-js-module-spatial-shape", () => {
    it("computes geometry stats for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadStatDefinition(SPATIAL_SHAPE_GEOMETRY_STAT_ID)!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const out = await computeStat(defn, {
        model,
        kernel,
        modelDefinitionId: defaultModelDefinitionId(),
        scope: "model",
        objects: objectsForStatCompute(model, defaultModelDefinitionId(), defn, "model", []),
      });
      expect(out.objectCount).toBe(1);
      expect(out.totalVolume).toBeCloseTo(1, 3);
    });

    it("derives volume property for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const object = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadPropertyDefinition(SPATIAL_SHAPE_VOLUME_PROPERTY_ID)!;
      const kernel = {
        syncSolidsFromModel: async () => {},
        solidVolume: async () => 42,
      } as unknown as SpatialKernel;
      const out = await derivePropertyValue(defn, { model, kernel, object });
      expect(out.volume).toBe(42);
    });
  });
}
// #endregion 🧪️Tests
