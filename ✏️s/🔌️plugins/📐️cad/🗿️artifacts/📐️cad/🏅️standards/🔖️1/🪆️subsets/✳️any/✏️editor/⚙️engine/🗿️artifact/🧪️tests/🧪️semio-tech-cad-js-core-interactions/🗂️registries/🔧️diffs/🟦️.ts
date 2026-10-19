/** 🔧️ The nurbs pole model diffs, box commit and typology mode action tests. */
import { ObjectRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { loadTypology } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { typologyConstructAssetIds, typologyConstructModeActionIds, typologyHasNativeConstructKit } from "../../../../../🧬️typology/🟦️.ts";
import { selectionTargetsPointTransformDiff } from "../../../../../🎬️actions/🧲️selection-targets/🟦️.ts";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { Vec3 } from "@semio-tech/framework/3d";
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";

/** 📐 Registers the nurbs pole sync, selection target transforms, box commit binding and typology mode action tests. */
export function registerModelDiffAndTypologyModeTests(kit: RegistriesTestKit): void {
  const { BrepjsKernel, Model, applyModelDiff, createInteractionRuntime, defaultModelDefinitionId, expect, it, loadSpatialInteraction, typologyConstructCommitActionForMode, typologyConstructKitByInteraction } = kit;
  it("applyModelDiff syncs nurbs through-curve poles when endpoint vertices move", async () => {
    const model = new Model();
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const created = await kernel.executeCommandDiff("curve.interpolateCurve", {
      model,
      points: [
        [0, 0, 0],
        [2, 1, 0],
        [4, 0, 0],
      ],
    });
    applyModelDiff(model, created.diff);
    const edge = Object.values(model.edges)[0]!;
    const startId = edge.vertexIds[0]!;
    applyModelDiff(model, { vertices: { modified: [{ id: startId, position: [0, 3, 0] }] } });
    const updated = model.edges[edge.id]!;
    expect(updated.curve?.kind).toBe("nurbs");
    if (updated.curve?.kind === "nurbs") {
      expect(updated.curve.poles[0]).toEqual([0, 3, 0]);
    }
  });
  it("selectionTargetsPointTransformDiff moves all nurbs poles when an edge is selected", async () => {
    const model = new Model();
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const created = await kernel.executeCommandDiff("curve.controlPointCurve", {
      model,
      points: [
        [0, 0, 0],
        [1, 2, 0],
        [3, 0, 0],
      ],
    });
    applyModelDiff(model, created.diff);
    const edge = Object.values(model.edges)[0]!;
    const before: readonly Vec3[] = edge.curve?.kind === "nurbs" ? edge.curve.poles.map((pole: Vec3) => [...pole] as Vec3) : [];
    applyModelDiff(
      model,
      selectionTargetsPointTransformDiff(model, [{ kind: "edge", id: edge.id, editable: true }], (point) => [point[0] + 1, point[1], point[2]]),
    );
    const updated = model.edges[edge.id]!;
    expect(updated.curve?.kind).toBe("nurbs");
    if (updated.curve?.kind === "nurbs") {
      expect(updated.curve.poles).toEqual(before.map((pole: Vec3) => [pole[0] + 1, pole[1], pole[2]]));
    }
  });
  it("selectionTargetsPointTransformDiff leaves interior poles when only a vertex is selected", async () => {
    const model = new Model();
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const created = await kernel.executeCommandDiff("curve.controlPointCurve", {
      model,
      points: [
        [0, 0, 0],
        [1, 2, 0],
        [3, 0, 0],
      ],
    });
    applyModelDiff(model, created.diff);
    const edge = Object.values(model.edges)[0]!;
    const midPole = edge.curve?.kind === "nurbs" ? [...edge.curve.poles[1]!] : null;
    const startId = edge.vertexIds[0]!;
    applyModelDiff(
      model,
      selectionTargetsPointTransformDiff(model, [{ kind: "vertex", id: startId, editable: true }], (point) => [point[0], point[1] + 5, point[2]]),
    );
    const updated = model.edges[edge.id]!;
    expect(updated.curve?.kind).toBe("nurbs");
    if (updated.curve?.kind === "nurbs" && midPole) {
      expect(updated.curve.poles[1]).toEqual(midPole);
    }
  });
  it("primitive.box commit binds typology object rows for document", async () => {
    const typology = "spatial.shape.primitive.box";
    const spec = loadSpatialInteraction("primitive.box")!;
    const model = new Model();
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const rt = createInteractionRuntime(spec, {
      kernel,
      document: { model, nodes: [] },
      activeModelDefinitionId: defaultModelDefinitionId(),
    });
    await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
    await rt.send({ kind: "pointer.down", point: [2, 3, 0], modifiers: {} });
    await rt.send({ kind: "set.height", value: 4, modifiers: {} });
    await rt.send({ kind: "confirm", modifiers: {} });
    const res = rt.getSnapshot().lastResponse!;
    expect(res.ok).toBe(true);
    expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(1);
    expect(model.objects[typology as ObjectRef]?.typology).toBe(typology);
    expect(model.objects[typology as ObjectRef]?.primitives.solid).toBeTruthy();
    const rt2 = createInteractionRuntime(spec, {
      kernel,
      document: { model, nodes: [] },
      activeModelDefinitionId: defaultModelDefinitionId(),
    });
    await rt2.send({ kind: "pointer.down", point: [5, 0, 0], modifiers: {} });
    await rt2.send({ kind: "pointer.down", point: [7, 2, 0], modifiers: {} });
    await rt2.send({ kind: "set.height", value: 2, modifiers: {} });
    await rt2.send({ kind: "confirm", modifiers: {} });
    expect(rt2.getSnapshot().lastResponse?.ok).toBe(true);
    expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(2);
  });
  it("typologyConstructCommitActionForMode resolves exactly one mode construct action", () => {
    const ids = typologyConstructAssetIds("energy.energy.hull", "Hull");
    const kit = typologyConstructKitByInteraction().get(ids.interaction)!;
    expect(typologyConstructCommitActionForMode(kit, "2PointsAndHeight")).toBe(ids.constructFrom2PointsAndHeight);
    expect(typologyConstructCommitActionForMode(kit, "curveAndHeight")).toBe(ids.constructFromCurveAndHeight);
    expect(typologyConstructCommitActionForMode(kit, "surface")).toBe(ids.constructFromSurface);
  });
  it("base plate typology lists only constructFromSurface among mode actions", () => {
    const typology = loadTypology("energy.energy.baseplate")!;
    const ids = typologyConstructAssetIds(typology.id, typology.label);
    expect(typologyConstructModeActionIds(typology.id, typology.label)).toEqual([ids.constructFromSurface]);
    expect(typologyHasNativeConstructKit(typology)).toBe(true);
  });
}
