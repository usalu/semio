/** ➰️ The interpolate curve step, numeric entry and commit tests. */
import { ObjectRef, VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { listModelObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionNumericEntryLockedValue, interactionStepFinalizeEvent } from "../../../../../🎬️actions/🎰️interaction-state/🟦️.ts";
import type { InteractionEvent } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { Vec3 } from "@semio-tech/framework/3d";
import type { RegistriesTestKit } from "../🧰️kit/🟦️.ts";

/** 🧵 Registers the interpolate curve finalize, numeric lock, one point confirm and commit binding tests. */
export function registerCurveInterpolationStepTests(kit: RegistriesTestKit): void {
  const { BrepjsKernel, M, Model, createInteractionRuntime, defaultModelDefinitionId, expect, it, loadSpatialInteraction, typologyIdForInteractionCommit } = kit;
  it("interactionStepFinalizeEvent confirms interpolate curve with two points instead of pointer.down", () => {
    const spec = loadSpatialInteraction("curve.interpolateCurve")!;
    const ctx = { points: [[0, 0, 0] as Vec3, [2, 1, 0] as Vec3], cursor: [5, 5, 0] as Vec3 };
    expect(interactionStepFinalizeEvent(spec, "next_point", ctx, M)?.kind).toBe("confirm");
    expect(interactionNumericEntryCommitEvent(spec, "next_point", ctx, M)?.kind).toBe("pointer.down");
  });
  it("interactionNumericEntryExplicitLockValue ignores live rubber-band distance", () => {
    const spec = loadSpatialInteraction("curve.interpolateCurve")!;
    const ctx = { points: [[0, 0, 0] as Vec3], cursor: [4, 0, 0] as Vec3, __lengthLock: 2 };
    expect(interactionNumericEntryExplicitLockValue(spec, "next_point", ctx)).toBe(2);
    const liveOnly = { points: [[0, 0, 0] as Vec3], cursor: [4, 0, 0] as Vec3 };
    expect(interactionNumericEntryExplicitLockValue(spec, "next_point", liveOnly)).toBeNull();
    expect(interactionNumericEntryLockedValue(spec, "next_point", liveOnly)).toBeCloseTo(4, 5);
  });
  it("curve.interpolateCurve confirm with one point stays in next_point", async () => {
    const spec = loadSpatialInteraction("curve.interpolateCurve")!;
    const model = new Model();
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const rt = createInteractionRuntime(spec, {
      kernel,
      document: { model, nodes: [] },
      activeModelDefinitionId: defaultModelDefinitionId(),
    });
    await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
    await rt.send({ kind: "confirm", modifiers: {} });
    expect(rt.getSnapshot().state).toBe("next_point");
    expect(Object.keys(model.edges).length).toBe(0);
  });
  it("curve.interpolateCurve commit binds typology object rows for document", async () => {
    const typology = "spatial.shape.curve.interpolate-curve";
    expect(typologyIdForInteractionCommit("curve.interpolateCurve")).toBe(typology);
    const spec = loadSpatialInteraction("curve.interpolateCurve")!;
    const model = new Model();
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const rt = createInteractionRuntime(spec, {
      kernel,
      document: { model, nodes: [] },
      activeModelDefinitionId: defaultModelDefinitionId(),
    });
    await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
    await rt.send({ kind: "pointer.down", point: [2, 1, 0], modifiers: {} });
    await rt.send({ kind: "confirm", modifiers: {} });
    const res = rt.getSnapshot().lastResponse!;
    expect(res.ok).toBe(true);
    expect(Object.keys(model.edges).length).toBeGreaterThan(0);
    expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId())).toHaveLength(1);
    expect(model.objects[typology as ObjectRef]?.typology).toBe(typology);
    expect(model.objects[typology as ObjectRef]?.primitives.curve).toBeTruthy();
  });
  it("curve.interpolateCurve commit uses live snapped vertex positions after vertex move", async () => {
    const spec = loadSpatialInteraction("curve.interpolateCurve")!;
    const model = new Model();
    const vertexId = "v-live" as VertexRef;
    model.vertices[vertexId] = { id: vertexId, position: [0, 0, 0] };
    const kernel = new BrepjsKernel() as unknown as SpatialKernel;
    const rt = createInteractionRuntime(spec, {
      kernel,
      document: { model, nodes: [] },
      activeModelDefinitionId: defaultModelDefinitionId(),
    });
    await rt.send({
      kind: "pointer.down",
      point: [0, 0, 0],
      modifiers: {},
      snap: { kind: "vertex", id: vertexId, point: [0, 0, 0] },
    } as InteractionEvent);
    await rt.send({ kind: "pointer.down", point: [2, 0, 0], modifiers: {} });
    model.vertices[vertexId] = { id: vertexId, position: [0, 4, 0] };
    await rt.send({ kind: "confirm", modifiers: {} });
    const edge = Object.values(model.edges)[0];
    expect(edge?.curve?.kind).toBe("nurbs");
    if (edge?.curve?.kind === "nurbs") {
      expect(edge.curve.poles[0]).toEqual([0, 4, 0]);
    }
  });
}
