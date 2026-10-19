/** 〰️ The nurbs poles, scene visibility and gumball transform tests. */
import type { EdgeRef, SolidRef, VertexRef, WireRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { Vec3 } from "@semio-tech/cad-engine-ts/registry";
import type { ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 👁 Registers the wire segments, nurbs poles, scene visibility toggles and gumball translate, rotate and scale tests. */
export function registerNurbsPolesAndSceneVisibilityTests(kit: ReplTestKit): void {
  const { BrepjsKernel, M, Model, applyModelDiff, collectGeometryEdgeSegments, createSpatialPickTargets, defaultModelDefinitionId, defaultSpatialPrimitiveToggles, defaultSpatialTypologyTogglesForModelDefinition, ensureTypologyObjectFromCreateDiff, expect, filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, geometryBuckets, geometryEntityNurbsPoles, geometryEntityWireSegments, isRenderableMeshTransfer, it, modelDefinitionTypologyIds, nurbsPolesFromEdge, resolveSpatialSceneVisibility, solidRef, source, spatialPickKindTogglesFromTypologyFilteredTargets, spatialSceneKindTogglesForModelDefinition, transformGumballMatrixDiff } = kit;
  it("polylineWireSegments tessellates nurbs edge samples for factory wireframe", () => {
    const model = new Model();
    const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
    const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
    const edge = {
      id: "e0" as EdgeRef,
      vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
      curve: {
        kind: "nurbs" as const,
        poles: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ] as Vec3[],
        degree: 2,
        through: true,
      },
    };
    const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
    applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
    const buckets = geometryBuckets(model);
    const segments = collectGeometryEdgeSegments(buckets);
    const edgeSegments = geometryEntityWireSegments(buckets, "edge", edge.id);
    expect(segments.length).toBeGreaterThan(2);
    expect(edgeSegments.length).toBeGreaterThan(2);
    expect(segments[0]).toEqual(edgeSegments[0]);
  });
  it("ensureTypologyObjectFromCreateDiff binds wire primitive for interpolate curve", () => {
    const model = new Model();
    const diff: ModelDiff = {
      wires: { added: [{ id: "w0" as WireRef, edgeIds: ["e0" as EdgeRef] }] },
      edges: { added: [{ id: "e0" as EdgeRef, vertexIds: ["v0" as VertexRef, "v1" as VertexRef] }] },
    };
    ensureTypologyObjectFromCreateDiff(model, "spatial.shape.curve.interpolate-curve", diff);
    expect(model.objects["spatial.shape.curve.interpolate-curve"]?.primitives.curve).toBe("w0");
  });
  it("geometryEntityNurbsPoles returns interpolation poles for through curves", () => {
    const model = new Model();
    const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
    const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
    const edge = {
      id: "e0" as EdgeRef,
      vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
      curve: {
        kind: "nurbs" as const,
        poles: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ] as Vec3[],
        degree: 2,
        through: true,
      },
    };
    const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
    applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
    const buckets = geometryBuckets(model);
    expect(geometryEntityNurbsPoles(buckets, "edge", edge.id)).toHaveLength(3);
    expect(geometryEntityNurbsPoles(buckets, "wire", wire.id)).toHaveLength(3);
    expect(nurbsPolesFromEdge(edge)).toEqual(edge.curve!.kind === "nurbs" ? edge.curve.poles : null);
  });
  it("geometryEntityNurbsPoles returns control poles for control-point curves", () => {
    const model = new Model();
    const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
    const v1 = { id: "v1" as VertexRef, position: [4, 0, 0] as Vec3 };
    const edge = {
      id: "e0" as EdgeRef,
      vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef],
      curve: {
        kind: "nurbs" as const,
        poles: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 1, 0],
          [4, 0, 0],
        ] as Vec3[],
        degree: 3,
        through: false,
      },
    };
    const wire = { id: "w0" as WireRef, edgeIds: [edge.id] };
    applyModelDiff(model, { vertices: { added: [v0, v1] }, edges: { added: [edge] }, wires: { added: [wire] } });
    const buckets = geometryBuckets(model);
    expect(geometryEntityNurbsPoles(buckets, "edge", edge.id)).toHaveLength(4);
    expect(nurbsPolesFromEdge(edge)).toEqual(edge.curve!.kind === "nurbs" ? edge.curve.poles : null);
  });
  it("resolveSpatialSceneVisibility switches edit wireframe vs committed object mesh", () => {
    expect(resolveSpatialSceneVisibility(defaultModelDefinitionId(), { edge: true, face: true })).toEqual({
      showFactoryWireframe: true,
      showCommittedFaces: true,
      showCommittedEdges: true,
    });
    expect(resolveSpatialSceneVisibility("aec.building.energy", { edge: true, face: true, object: true })).toEqual({
      showFactoryWireframe: true,
      showCommittedFaces: true,
      showCommittedEdges: true,
    });
  });
  it("spatialSceneKindTogglesForModelDefinition keeps committed faces on when typology picks are empty", () => {
    const toggles = spatialSceneKindTogglesForModelDefinition("aec.building.energy", defaultSpatialPrimitiveToggles());
    expect(resolveSpatialSceneVisibility("aec.building.energy", toggles).showCommittedFaces).toBe(true);
  });
  it("concrete forest fixture keeps committed face visibility toggles", async () => {
    const { readFileSync } = await import("node:fs");
    const { ModelSpace } = await import("@semio-tech/s-spatial-kernel-ts/geometry/model-space");
    const json = JSON.parse(readFileSync(new URL("../../../📚️examples/🖼️assets/🎮️play/🔣️.json", source.url), "utf8"));
    const model = (ModelSpace.fromJSON(json).models[defaultModelDefinitionId()] ?? ModelSpace.fromJSON(json).models[""])!;
    const mdId = defaultModelDefinitionId();
    expect(Object.keys(model.solids).length).toBeGreaterThan(0);
    let targets = createSpatialPickTargets(model, mdId);
    targets = filterSpatialPickTargetsForActiveView(targets, mdId);
    targets = filterSpatialPickTargetsForPrimitiveToggles(targets, defaultSpatialPrimitiveToggles());
    targets = filterSpatialPickTargetsForTypologyToggles(targets, defaultSpatialTypologyTogglesForModelDefinition(mdId), modelDefinitionTypologyIds(mdId));
    const toggles = spatialPickKindTogglesFromTypologyFilteredTargets(mdId, targets);
    expect(resolveSpatialSceneVisibility(mdId, toggles).showCommittedFaces).toBe(true);
    const kernel = new BrepjsKernel();
    await kernel.resetDerivedPipelineForTest();
    const solid = Object.keys(model.solids)[0]! as SolidRef;
    const mesh = await kernel.tessellate(solid, 0.02, model);
    expect(isRenderableMeshTransfer(mesh)).toBe(true);
  });
  it("transformGumballMatrixDiff translates solid selection vertices", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
    const solidId = Object.keys(model.solids)[0]!;
    const diff = transformGumballMatrixDiff(model, [{ kind: "solid", id: solidId, editable: true }], { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] });
    applyModelDiff(model, diff);
    for (const v of Object.values(model.vertices)) {
      expect(v.position[0]).toBeGreaterThanOrEqual(1.5);
    }
  });
  it("transformGumballMatrixDiff rotates solid selection vertices", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
    const solidId = Object.keys(model.solids)[0]!;
    const before = Object.values(model.vertices).map((v) => v.position.join(","));
    const diff = transformGumballMatrixDiff(
      model,
      [{ kind: "solid", id: solidId, editable: true }],
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0, 0.7071068, 0.7071068], scale: [1, 1, 1] },
    );
    applyModelDiff(model, diff);
    const after = Object.values(model.vertices).map((v) => v.position.join(","));
    expect(after.sort().join("|")).not.toBe(before.sort().join("|"));
  });
  it("transformGumballMatrixDiff scales solid selection vertices", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
    const solidId = Object.keys(model.solids)[0]!;
    const diff = transformGumballMatrixDiff(model, [{ kind: "solid", id: solidId, editable: true }], { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 2, 2] });
    applyModelDiff(model, diff);
    for (const v of Object.values(model.vertices)) {
      expect(v.position[0]).toBeGreaterThanOrEqual(0);
      expect(v.position[0]).toBeLessThanOrEqual(2.01);
    }
  });
}
