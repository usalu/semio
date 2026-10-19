/** 🎨️ The pick rendering, reveal, palette and typology style tests. */
import type { EdgeRef, FaceRef, SolidRef, VertexRef, WireRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { SpatialPickTarget } from "../../../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import type { MeshTransfer, Vec3 } from "@semio-tech/cad-engine-ts/registry";
import type { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 🖌 Registers the hidden and locked entity flags, pick rendering, reveal, palette, planar face shading and typology style tests. */
export function registerPickRenderingAndTypologyStyleTests(kit: ReplTestKit): void {
  const { M, Model, applyModelDiff, buildGeometryObjectIndex, buildPlanarFaceMeshTransfer, createSolidTypologyStyleResolver, createSpatialPickTargets, createTypologyStyledMaterial, defaultInteractionSpatialViewTheme, defaultModelDefinitionId, emptyMeshTransfer, expect, filterCommittedMeshesForModelDefinition, filterSpatialPickTargetsForEntityFlags, isRenderableMeshTransfer, it, listFactoryFaceMeshesForModelDefinition, pruneSelectionTargetsForEntityFlags, resetSpatialSceneColorCache, resolveSpatialEntityFlags, resolveSpatialPickTargetsToRender, resolveTypologyStyle, revealedObjectIdsFromPickKeys, solidRef, spatialAutoFitShouldRun, spatialPickTargetKey, spatialSceneColors, spatialSelectionTarget, typologyStyleCacheKey, typologyStyleToMaterialProps, visibleSolidRefsForModelDefinition } = kit;
  it("filterSpatialPickTargetsForEntityFlags excludes hidden and locked targets", () => {
    const targets: SpatialPickTarget[] = [
      { kind: "object", id: "visible", point: [0, 0, 0] },
      { kind: "object", id: "hidden", point: [0, 0, 0] },
      { kind: "face", geometryKind: "face", id: "locked", point: [0, 0, 0] },
    ];
    const flagsForId = (id: string) => ({ ...(id === "hidden" ? { hidden: true } : {}), ...(id === "locked" ? { locked: true } : {}) });
    expect(filterSpatialPickTargetsForEntityFlags(targets, flagsForId).map((row) => row.id)).toEqual(["visible"]);
    expect(pruneSelectionTargetsForEntityFlags(targets.map(spatialSelectionTarget), flagsForId).map((row) => row.id)).toEqual(["visible"]);
  });
  it("resolveSpatialPickTargetsToRender skips hidden unless pinned", () => {
    const targets: SpatialPickTarget[] = [
      { kind: "object", id: "hidden", point: [0, 0, 0] },
      { kind: "object", id: "visible", point: [1, 0, 0] },
    ];
    const flagsForId = (id: string) => ({ ...(id === "hidden" ? { hidden: true } : {}) });
    expect(resolveSpatialPickTargetsToRender(targets, {}, new Set(), flagsForId).map((row) => row.id)).toEqual(["visible"]);
    expect(
      resolveSpatialPickTargetsToRender(targets, {}, new Set(["object:hidden"]), flagsForId)
        .map((row) => row.id)
        .sort(),
    ).toEqual(["hidden", "visible"]);
  });
  it("resolveSpatialPickTargetsToRender draws all enabled kinds", () => {
    const targets: SpatialPickTarget[] = [
      { kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
      {
        kind: "edge",
        geometryKind: "edge",
        id: "e0",
        point: [0, 0, 0],
        points: [
          [0, 0, 0],
          [1, 0, 0],
        ],
      },
    ];
    expect(resolveSpatialPickTargetsToRender(targets, { edge: false }).map(spatialPickTargetKey)).toEqual(["vertex:v0"]);
    expect(resolveSpatialPickTargetsToRender(targets, {}).map(spatialPickTargetKey).sort()).toEqual(["edge:e0", "vertex:v0"]);
  });
  it("resolveSpatialPickTargetsToRender draws factory primitives without hover reveal", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
    const solidId = Object.keys(model.solids)[0]!;
    model.objects["box-obj"] = {
      id: "box-obj" as ObjectRef,
      typology: "spatial.shape.primitive.box" as TypologyRef,
      primitives: { solid: solidId },
    };
    const targets = createSpatialPickTargets(model, defaultModelDefinitionId());
    expect(resolveSpatialPickTargetsToRender(targets, {}, new Set(), () => ({})).some((row) => row.kind === "vertex")).toBe(true);
  });
  it("revealedObjectIdsFromPickKeys expands solid and member picks to the owning object", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
    const solidId = Object.keys(model.solids)[0]!;
    const vertexId = Object.keys(model.vertices)[0]!;
    model.objects["box-obj"] = {
      id: "box-obj" as ObjectRef,
      typology: "spatial.shape.primitive.box" as TypologyRef,
      primitives: { solid: solidId },
    };
    const objectIndex = buildGeometryObjectIndex(model, defaultModelDefinitionId());
    expect([...revealedObjectIdsFromPickKeys(objectIndex, `vertex:${vertexId}`, new Set())]).toEqual(["box-obj"]);
    expect([...revealedObjectIdsFromPickKeys(objectIndex, `object:${solidId}`, new Set())]).toEqual(["box-obj"]);
  });
  it("spatialSceneColors exposes a single product-aligned palette", () => {
    resetSpatialSceneColorCache();
    const palette = spatialSceneColors();
    expect(palette.accent).toBeTruthy();
    expect(palette.construction).toBe(palette.accent);
    expect(palette.selected).toBeTruthy();
    expect(palette.groundPlane).not.toBe(palette.accent);
    resetSpatialSceneColorCache();
  });
  it("visibleSolidRefsForModelDefinition scopes building solids to visible objects only", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("solid-a")));
    applyModelDiff(model, M.boxModelDiff({ cornerA: [2, 0, 0], cornerB: [3, 1, 0], height: 1 }, solidRef("solid-b")));
    model.objects["obj-a"] = { id: "obj-a" as ObjectRef, typology: "building.building.column" as TypologyRef, primitives: { solid: "solid-a" } };
    model.objects["obj-b"] = { id: "obj-b" as ObjectRef, typology: "building.building.beam" as TypologyRef, primitives: { solid: "solid-b" } };
    model.objects["orphan"] = { id: "orphan" as ObjectRef, typology: "spatial.shape.kernel.solid" as TypologyRef, primitives: { solid: "solid-a" } };
    const flagsForId = (id: string) => model.getEntityFlags(id);
    expect([...visibleSolidRefsForModelDefinition(model, "aec.building", { flagsForId })].sort()).toEqual(["solid-a", "solid-b"]);
    model.setEntityFlag("obj-a", "hidden", true);
    expect([...visibleSolidRefsForModelDefinition(model, "aec.building", { flagsForId })].sort()).toEqual(["solid-b"]);
    model.setEntityFlag("obj-b", "hidden", true);
    expect([...visibleSolidRefsForModelDefinition(model, "aec.building", { flagsForId })]).toEqual([]);
  });
  it("buildPlanarFaceMeshTransfer shades typology surface primitives for energy models", () => {
    const model = new Model();
    const v0 = { id: "v0" as VertexRef, position: [0, 0, 0] as Vec3 };
    const v1 = { id: "v1" as VertexRef, position: [1, 0, 0] as Vec3 };
    const v2 = { id: "v2" as VertexRef, position: [0, 1, 0] as Vec3 };
    const e0 = { id: "e0" as EdgeRef, vertexIds: [v0.id, v1.id] as [VertexRef, VertexRef] };
    const e1 = { id: "e1" as EdgeRef, vertexIds: [v1.id, v2.id] as [VertexRef, VertexRef] };
    const e2 = { id: "e2" as EdgeRef, vertexIds: [v2.id, v0.id] as [VertexRef, VertexRef] };
    const wireId = "w0" as WireRef;
    const faceId = "f0" as FaceRef;
    applyModelDiff(model, {
      vertices: { added: [v0, v1, v2] },
      edges: { added: [e0, e1, e2] },
      wires: { added: [{ id: wireId, edgeIds: [e0.id, e1.id, e2.id] }] },
      faces: {
        added: [
          {
            id: faceId,
            wireIds: [wireId],
            surface: { kind: "plane", origin: [0, 0, 0], normal: [0, 0, 1] },
          },
        ],
      },
    });
    model.objects["baseplate"] = {
      id: "baseplate" as ObjectRef,
      typology: "energy.energy.baseplate" as TypologyRef,
      primitives: { surface: faceId },
    };
    const mesh = buildPlanarFaceMeshTransfer(model, faceId);
    expect(mesh).not.toBeNull();
    expect(isRenderableMeshTransfer(mesh!)).toBe(true);
    expect(mesh!.index.length).toBe(3);
    const rows = listFactoryFaceMeshesForModelDefinition(model, "aec.building.energy");
    expect(rows).toHaveLength(1);
    expect(rows[0]?.faceId).toBe(faceId);
    expect(rows[0]?.style?.color).toBeTruthy();
  });
  it("filterCommittedMeshesForModelDefinition drops meshes when all objects are hidden", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("solid-a")));
    model.objects["obj-a"] = { id: "obj-a" as ObjectRef, typology: "building.building.column" as TypologyRef, primitives: { solid: "solid-a" } };
    model.setEntityFlag("obj-a", "hidden", true);
    const mesh: MeshTransfer = {
      ...emptyMeshTransfer(),
      position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
      normal: new Float32Array([0, 0, 0, 0, 0, 0, 0, 0, 0]),
      index: new Uint32Array([0, 1, 2]),
    };
    const filtered = filterCommittedMeshesForModelDefinition(model, "aec.building", [{ solid: solidRef("solid-a"), mesh }], {
      flagsForId: (id) => model.getEntityFlags(id),
    });
    expect(filtered).toEqual([]);
  });
  it("resolveSpatialEntityFlags inherits hidden state from owning object", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("solid-a")));
    const solidId = Object.keys(model.solids)[0]!;
    const faceId = Object.keys(model.faces)[0]!;
    model.objects["obj-a"] = { id: "obj-a" as ObjectRef, typology: "building.building.column" as TypologyRef, primitives: { solid: solidId } };
    model.setEntityFlag("obj-a", "hidden", true);
    expect(resolveSpatialEntityFlags(model, "aec.building", faceId).hidden).toBe(true);
    expect(resolveSpatialEntityFlags(model, "aec.building", solidId).hidden).toBe(true);
  });
  it("typologyStyleToMaterialProps and typologyStyleCacheKey reflect resolved style", () => {
    const style = resolveTypologyStyle("structure.structure.onewayreinforcedconcreteslab");
    const props = typologyStyleToMaterialProps(style);
    expect(props.color).toBe("#8B7355");
    expect(props.opacity).toBe(0.78);
    expect(typologyStyleCacheKey(style)).toContain("hatch");
  });
  it("createTypologyStyledMaterial injects self-contained world position for patterns", () => {
    const material = createTypologyStyledMaterial({
      color: "#c0ffee",
      edgeColor: "#102030",
      opacity: 0.8,
      pattern: { kind: "hatch", direction: 30, spacing: 0.4, lineWidth: 0.02, color: "#203040" },
    });
    const shader = {
      uniforms: {},
      vertexShader: "#include <common>\nvoid main() {\nvec3 transformed = vec3(position);\n#include <worldpos_vertex>\n}",
      fragmentShader: "#include <common>\nvoid main() {\nvec3 outgoingLight = vec3(1.0);\nvec3 normal = vec3(0.0, 0.0, 1.0);\n#include <output_fragment>\n}",
    };
    material.onBeforeCompile(shader as never, {} as never);
    expect(shader.vertexShader).toContain("vec4 typologyWorldPosition = vec4(transformed, 1.0);");
    expect(shader.vertexShader).toContain("typologyWorldPosition = instanceMatrix * typologyWorldPosition;");
    expect(shader.vertexShader).not.toContain("vTypologyWorldPos = worldPosition.xyz;");
  });
  it("createSolidTypologyStyleResolver maps solids to their object typology style", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("slab-solid")));
    const solidId = Object.keys(model.solids)[0]!;
    model.objects["slab-obj"] = {
      id: "slab-obj" as ObjectRef,
      typology: "structure.structure.onewayreinforcedconcreteslab" as TypologyRef,
      primitives: { solid: solidId },
    };
    const resolveStyle = createSolidTypologyStyleResolver(model, "aec.building.structure.classic");
    const style = resolveStyle(solidId as SolidRef);
    expect(style?.pattern.kind).toBe("hatch");
    expect(style?.color).toBe("#8B7355");
  });
  it("defaultInteractionSpatialViewTheme hides the factory ground plane tint", () => {
    expect(defaultInteractionSpatialViewTheme.groundPlaneOpacity).toBe(0);
  });
  it("spatialAutoFitShouldRun keeps initial fit pane-local across mesh reloads", () => {
    expect(spatialAutoFitShouldRun("initial", "mesh-key", "mesh-key", true)).toBe(false);
    expect(spatialAutoFitShouldRun("changes", "mesh-key", "mesh-key", true)).toBe(false);
    expect(spatialAutoFitShouldRun("changes", "next-key", "mesh-key", true)).toBe(true);
  });
}
