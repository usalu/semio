// #region 🧭️Header
/** @emoji 🏛️ `@semio-tech/cad-js-module-aec-building-structure` — structure stat, transformation, and STEP import profiles. */
// #endregion 🧭️Header

import {
  bboxSizesFromPositions,
  cloneModelGeometryShell,
  collectSolidRefsForObjects,
  collectVertexPositionsForObjects,
  qualifiedTransformationId,
  registerImportProfile,
  registerStatComputer,
  registerTransformationApplier,
  solidRef,
  transformationObjectId,
  type Model,
  type StatComputeContext,
  type TransformationSpec,
  type TypologyRef,
} from "@semio-tech/cad-js-core";

// #region 🏷️Ids
export const AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID = "aec.building.structure";
export const AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID = "aec.building.structure.classic";
export const AEC_BUILDING_STRUCTURE_FEM_LINE_MODEL_DEFINITION_ID = "aec.building.structure.fem.line";
export const AEC_BUILDING_STRUCTURE_FEM_SOLID_MODEL_DEFINITION_ID = "aec.building.structure.fem.solid";
export const AEC_BUILDING_STRUCTURE_FEM_SURFACE_MODEL_DEFINITION_ID = "aec.building.structure.fem.surface";
export const STRUCTURE_STABILITY_STAT_ID = "structure.stability";
export const STRUCTURE_FROM_BUILDING_TRANSFORMATION_ID = "from_building";
// #endregion 🏷️Ids

// #region 🪪️ImportProfile
const STRUCTURE_LAYER_TYPOLOGY: Readonly<Record<string, TypologyRef>> = {
  slab: "structure.structure.onewayreinforcedconcreteslab",
  column: "structure.structure.reinforcedconcretecolumn",
  columns: "structure.structure.reinforcedconcretecolumn",
  beam: "structure.structure.reinforcedconcreteinternalwall",
  beams: "structure.structure.reinforcedconcreteinternalwall",
  wall: "structure.structure.reinforcedconcreteexternalwall",
  walls: "structure.structure.reinforcedconcreteexternalwall",
};

const BUILDING_TO_STRUCTURE_TYPOLOGY: Readonly<Record<string, TypologyRef>> = {
  "building.building.column": "structure.structure.reinforcedconcretecolumn",
  "building.building.slab": "structure.structure.onewayreinforcedconcreteslab",
  "building.building.beam": "structure.structure.onewayreinforcedconcreteslab",
  "building.building.wall": "structure.structure.reinforcedconcreteinternalwall",
  "building.building.foundation": "structure.structure.onewayreinforcedconcreteslab",
  "building.building.roof": "structure.structure.onewayreinforcedconcreteslab",
  "building.building.stair": "structure.structure.onewayreinforcedconcreteslab",
  "building.building.ceiling": "structure.structure.onewayreinforcedconcreteslab",
  "building.building.railing": "structure.structure.reinforcedconcreteinternalwall",
  "building.building.door": "structure.structure.reinforcedconcreteexternalwall",
  "building.building.window": "structure.structure.reinforcedconcreteexternalwall",
};
// #endregion 🪪️ImportProfile

// #region 📊️StatComputer
export const STRUCTURE_CONCRETE_DENSITY_KG_M3 = 2500;

async function computeStructureStabilityStat(ctx: StatComputeContext): Promise<Record<string, number>> {
  const solidIds = collectSolidRefsForObjects(ctx.model, ctx.objects);
  await ctx.kernel.syncSolidsFromModel(ctx.model);
  let structuralVolume = 0;
  for (const solidId of solidIds) structuralVolume += await ctx.kernel.solidVolume(solidRef(solidId));
  const bbox = bboxSizesFromPositions(collectVertexPositionsForObjects(ctx.model, ctx.objects));
  const footprint = Math.max(bbox.sizeX * bbox.sizeY, bbox.sizeX * bbox.sizeZ, bbox.sizeY * bbox.sizeZ, 1e-6);
  const height = Math.max(bbox.sizeX, bbox.sizeY, bbox.sizeZ, 1e-6);
  const slenderness = height / Math.sqrt(footprint);
  const stabilityIndex = Math.max(0, Math.min(1, 1 / (1 + slenderness)));
  const estimatedMass = (structuralVolume * STRUCTURE_CONCRETE_DENSITY_KG_M3) / 1000;
  return {
    elementCount: ctx.objects.length,
    structuralVolume,
    estimatedMass,
    stabilityIndex,
  };
}
// #endregion 📊️StatComputer

// #region 🔄️TransformationApplier
function applyBuildingToStructureTransformation(_spec: TransformationSpec, source: Model): Model {
  const target = cloneModelGeometryShell(source);
  const counts = new Map<string, number>();
  target.objects = {};
  for (const row of Object.values(source.objects)) {
    const mapped = BUILDING_TO_STRUCTURE_TYPOLOGY[row.typology];
    if (!mapped) continue;
    const index = counts.get(mapped) ?? 0;
    counts.set(mapped, index + 1);
    const objectId = transformationObjectId(mapped, index);
    target.objects[objectId] = {
      id: objectId,
      typology: mapped,
      primitives: { ...row.primitives },
      ...(row.attributes ? { attributes: { ...row.attributes } } : {}),
    };
  }
  target.bump();
  return target;
}
// #endregion 🔄️TransformationApplier

// #region 📦️Register
function registerStructureImportProfile(modelDefinitionId: string): void {
  registerImportProfile(modelDefinitionId, {
    layerTypology: STRUCTURE_LAYER_TYPOLOGY,
    fallbackTypology: "structure.structure.onewayreinforcedconcreteslab" as TypologyRef,
    preferPresentationLayers: modelDefinitionId === AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID,
    presentationGeometry: modelDefinitionId === AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID ? "wireframe" : undefined,
    namespacedDomain: "structure",
  });
}

/** @emoji 📦️ Registers structure stat, transformation, and STEP import profiles on the core engine. */
export function register(): void {
  registerStatComputer(STRUCTURE_STABILITY_STAT_ID, computeStructureStabilityStat);
  registerTransformationApplier(qualifiedTransformationId(AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID, STRUCTURE_FROM_BUILDING_TRANSFORMATION_ID), applyBuildingToStructureTransformation);
  registerStructureImportProfile(AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID);
  registerStructureImportProfile(AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID);
  registerStructureImportProfile(AEC_BUILDING_STRUCTURE_FEM_LINE_MODEL_DEFINITION_ID);
  registerStructureImportProfile(AEC_BUILDING_STRUCTURE_FEM_SOLID_MODEL_DEFINITION_ID);
  registerStructureImportProfile(AEC_BUILDING_STRUCTURE_FEM_SURFACE_MODEL_DEFINITION_ID);
}
// #endregion 📦️Register

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { bootstrapCadModules } = await import("@semio-tech/cad-js-runtime");
  const { BrepjsKernel, preciseSpatialKernelMath } = await import("@semio-tech/cad-js-kernel-brepjs");
  const core = await import("@semio-tech/cad-js-core");
  const { Model, applyModelDiff, applyTransformation, computeStat, loadStatDefinition, loadTransformation, objectsForStatCompute, qualifiedTransformationId, solidRef } = core;
  type ObjectRef = core.ObjectRef;
  type SpatialKernel = core.SpatialKernel;
  type TypologyRef = core.TypologyRef;
  bootstrapCadModules();
  const M = preciseSpatialKernelMath;

  describe("@semio-tech/cad-js-module-aec-building-structure", () => {
    it("computes structure stability stats with finite outputs", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 0], height: 3 }, solidRef("column-solid")));
      model.objects["column"] = {
        id: "column" as ObjectRef,
        typology: "structure.structure.reinforcedconcretecolumn" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const defn = loadStatDefinition(STRUCTURE_STABILITY_STAT_ID)!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const out = await computeStat(defn, {
        model,
        kernel,
        modelDefinitionId: AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID,
        scope: "model",
        objects: objectsForStatCompute(model, AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID, defn, "model", []),
      });
      expect(out.elementCount).toBe(1);
      expect(out.stabilityIndex).toBeGreaterThan(0);
    });

    it("applies building to structure transformation", () => {
      const source = new Model();
      source.objects["col"] = {
        id: "col" as ObjectRef,
        typology: "building.building.column" as TypologyRef,
        primitives: { solid: "solid-a" },
      };
      const spec = loadTransformation(qualifiedTransformationId(AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID, STRUCTURE_FROM_BUILDING_TRANSFORMATION_ID))!;
      const target = applyTransformation(spec, source, { fuseSolidsToExternalFaces: () => ({ hullSolid: solidRef("h"), externalFaces: [] }) } as never);
      expect(Object.values(target.objects)[0]?.typology).toBe("structure.structure.reinforcedconcretecolumn");
    });
  });
}
// #endregion 🧪️Tests
