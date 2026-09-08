type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { SPATIAL_SHAPE_GEOMETRY_STAT_ID, SPATIAL_SHAPE_VOLUME_PROPERTY_ID, core, solidRef } = dependencies;
  type TypologyRef = any;

  const { describe, expect, it } = vitest;
  const { runtime, brepjs } = await import("@semio-tech/cad-js");
  const { bootstrapCadModules } = runtime;
  const { BrepjsKernel, preciseSpatialKernelMath } = brepjs;
  const { Model, applyModelDiff, computeStat, defaultModelDefinitionId, derivePropertyValue, loadPropertyDefinition, loadStatDefinition, objectsForStatCompute } = core;
  type ObjectRef = core.ObjectRef;
  type SpatialKernel = core.SpatialKernel;

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
