import { preciseSpatialKernelMath } from "../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧮️preview/🟦️.ts";
import type { core } from "@semio-tech/cad-js";
import type { SpatialShapeTestDependencies } from "../../🟦️.ts";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: SpatialShapeTestDependencies, source: TestSource): Promise<void> {
  const { SPATIAL_SHAPE_GEOMETRY_STAT_ID, SPATIAL_SHAPE_VOLUME_PROPERTY_ID, core, solidRef } = dependencies;
  type TypologyRef = core.TypologyRef;

  const { describe, expect, it } = vitest;
  const runtime = await import("../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");
  const brepjs = await import("../../../../../../🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🟦️.ts");
  const { bootstrapCadModules } = runtime;
  const { BrepjsKernel } = brepjs;
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
