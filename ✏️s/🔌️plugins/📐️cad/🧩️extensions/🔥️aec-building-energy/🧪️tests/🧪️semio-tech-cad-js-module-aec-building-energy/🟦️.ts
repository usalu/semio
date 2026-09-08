type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, ENERGY_DEMAND_STAT_ID, core, solidRef } = dependencies;
  type TypologyRef = any;

  const { describe, expect, it } = vitest;
  const { runtime, brepjs } = await import("@semio-tech/cad-js");
  const { bootstrapCadModules } = runtime;
  const { BrepjsKernel, preciseSpatialKernelMath } = brepjs;
  const { Model, applyModelDiff, computeStat, loadStatDefinition, objectsForStatCompute } = core;
  type ObjectRef = core.ObjectRef;
  type SpatialKernel = core.SpatialKernel;

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
