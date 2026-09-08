type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID, STRUCTURE_FROM_BUILDING_TRANSFORMATION_ID, STRUCTURE_STABILITY_STAT_ID, core, qualifiedTransformationId, solidRef } = dependencies;
  type Model = any;
  type TypologyRef = any;

  const { describe, expect, it } = vitest;
  const { runtime, brepjs } = await import("@semio-tech/cad-js");
  const { bootstrapCadModules } = runtime;
  const { BrepjsKernel, preciseSpatialKernelMath } = brepjs;
  const { Model, applyModelDiff, applyTransformation, computeStat, loadStatDefinition, loadTransformation, objectsForStatCompute } = core;
  type ObjectRef = core.ObjectRef;
  type SpatialKernel = core.SpatialKernel;
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
