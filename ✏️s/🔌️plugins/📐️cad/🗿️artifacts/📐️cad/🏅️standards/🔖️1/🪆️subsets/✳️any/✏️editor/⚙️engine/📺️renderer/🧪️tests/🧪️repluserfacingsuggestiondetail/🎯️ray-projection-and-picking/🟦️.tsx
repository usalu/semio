/** 🎯️ The CAD axes, ray projection, ground pick and geometry pick target tests. */
import type { VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { SpatialPickTarget } from "../../../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import type { InteractionSnapshot } from "../../../../🗿️artifact/📨️interaction-model/🟦️.ts";
import type { Vec3 } from "@semio-tech/cad-engine-ts/registry";
import type { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 📍 Registers the CAD axes, host picking states, ray projections, ground pick planes and geometry pick target tests. */
export function registerRayProjectionAndPickingTests(kit: ReplTestKit): void {
  const { CAD_WORLD_FORWARD, CAD_WORLD_UP, M, Model, THREE, applyCadWorldCoordinateSystem, applyModelDiff, createSpatialPickEvent, createSpatialPickTargets, defaultModelDefinitionId, expect, filterSpatialPickTargetsForActiveView, interactionSpatialGroundPickPlaneEnabled, it, loadSpatialInteraction, mergeInteractionSpatial, projectRayToVerticalZLine, projectRayToYzPlaneAtX, replHostGeometryPickingEnabled, solidRef, spatialPickTargetKey } = kit;
  it("uses CAD Y-forward and Z-up instead of Three.js Y-up defaults", () => {
    const camera = new THREE.PerspectiveCamera();
    const scene = new THREE.Scene();
    applyCadWorldCoordinateSystem(camera, scene);
    expect(CAD_WORLD_FORWARD).toEqual([0, 1, 0]);
    expect(camera.up.toArray()).toEqual(CAD_WORLD_UP);
    expect(scene.up.toArray()).toEqual(CAD_WORLD_UP);
  });
  it("replHostGeometryPickingEnabled follows pickDisabledStates while session is active", () => {
    const spec = loadSpatialInteraction("primitive.box");
    expect(spec).not.toBeNull();
    expect(replHostGeometryPickingEnabled("primitive.box", spec!, "first_corner")).toBe(false);
    expect(replHostGeometryPickingEnabled("primitive.box", spec!, "ready")).toBe(true);
    expect(replHostGeometryPickingEnabled("primitive.box", spec!, "committed")).toBe(true);
    expect(replHostGeometryPickingEnabled("primitive.box", spec!, "idle")).toBe(true);
    expect(replHostGeometryPickingEnabled("", spec!, "first_corner")).toBe(true);
  });
  it("surface.extrudeCrv enables host curve picking and disables ground plane during select_curves_to_extrude", () => {
    const spec = loadSpatialInteraction("surface.extrudeCrv");
    expect(spec).not.toBeNull();
    expect(replHostGeometryPickingEnabled("surface.extrudeCrv", spec!, "select_curves_to_extrude")).toBe(true);
    const snapshot = {
      state: "select_curves_to_extrude",
      spatialInteraction: mergeInteractionSpatial(spec!),
    } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
    expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(false);
  });
  it("surface.extrudeCrv enables ground pick during extrusion_distance for click finalize", () => {
    const spec = loadSpatialInteraction("surface.extrudeCrv");
    expect(spec).not.toBeNull();
    const snapshot = {
      state: "extrusion_distance",
      spatialInteraction: mergeInteractionSpatial(spec!),
    } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
    expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(true);
  });
  it("projectRayToVerticalZLine locks XY to origin and allows negative Z", () => {
    const origin: Vec3 = [2, 3, 1];
    const ray = new THREE.Ray(new THREE.Vector3(2, 3, -8), new THREE.Vector3(0, 0, 1));
    const vertical = projectRayToVerticalZLine(ray, origin);
    expect(vertical[0]).toBeCloseTo(2, 4);
    expect(vertical[1]).toBeCloseTo(3, 4);
    expect(vertical[2]).toBeCloseTo(-8, 4);
    const oblique = new THREE.Ray(new THREE.Vector3(0, 0, 3), new THREE.Vector3(1, 0, -1.5).normalize());
    const point = projectRayToVerticalZLine(oblique, origin);
    expect(point[0]).toBeCloseTo(2, 4);
    expect(point[2]).toBeLessThan(1);
  });
  it("projectRayToYzPlaneAtX intersects height-drag wall plane", () => {
    const planeX = 4.06;
    const ray = new THREE.Ray(new THREE.Vector3(0, 0, 5), new THREE.Vector3(1, 0, -0.2).normalize());
    const point = projectRayToYzPlaneAtX(ray, planeX);
    expect(point).not.toBeNull();
    expect(point![0]).toBeCloseTo(planeX, 4);
    expect(point![2]).toBeLessThan(5);
  });
  it("enables spatial ground pick plane during rubber-band states regardless of host selection accept", () => {
    const snapshot = {
      state: "first_corner",
      spatialInteraction: {
        spatialGroundPick: true,
        pickDisabledStates: ["idle", "ready", "committed"],
        groundPointerMoveStates: ["first_corner"],
        heightDragStates: [],
        verticalRodStates: [],
        heightConfirmState: null,
        lengthEntry: [],
        scalarEntry: [],
      },
    } satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
    expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true)).toBe(true);
    expect(interactionSpatialGroundPickPlaneEnabled(snapshot, false)).toBe(false);
  });
  it("creates snap and selection metadata for geometry targets", () => {
    const model = new Model();
    model.vertices.v0 = { id: "v0" as VertexRef, position: [1, 2, 3] };
    const targets = createSpatialPickTargets(model);
    expect(targets).toEqual([{ kind: "vertex", geometryKind: "vertex", id: "v0", point: [1, 2, 3], typologyId: undefined }]);
    expect(createSpatialPickEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
      kind: "pointer.down",
      point: [9, 9, 9],
      modifiers: { shift: true },
      snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
      selection: { kind: "vertex", id: "v0" },
    });
  });
  it("adds typology object picks for non-shape model definitions", async () => {
    const model = new Model();
    const cell = solidRef("c0");
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
    model.objects["object-c0"] = {
      id: "object-c0" as ObjectRef,
      typology: "energy.energy.hull" as TypologyRef,
      primitives: { solid: String(cell) },
    };
    const activeModelDefinitionId = "aec.building.energy";
    const editTargets = createSpatialPickTargets(model, defaultModelDefinitionId());
    const objectTargets = createSpatialPickTargets(model, activeModelDefinitionId);
    expect(editTargets.some((t) => t.kind === "vertex")).toBe(true);
    expect(objectTargets.some((t) => t.kind === "object" && !t.geometryKind)).toBe(true);
    expect(objectTargets.some((t) => t.geometryKind === "vertex")).toBe(true);
    const structureTargets = createSpatialPickTargets(model, "aec.building.structure");
    expect(structureTargets.some((t) => t.kind === "face")).toBe(true);
    expect(structureTargets.some((t) => t.kind === "object")).toBe(true);
  });
  it("filterSpatialPickTargetsForActiveView scopes by model definition entity kinds", () => {
    const targets: SpatialPickTarget[] = [
      { kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
      { kind: "face", geometryKind: "face", id: "f0", point: [0.5, 0.5, 0.5] },
      { kind: "object", id: "energy.energy.hull", point: [0.5, 0.5, 0.5] },
    ];
    expect(filterSpatialPickTargetsForActiveView(targets, defaultModelDefinitionId()).map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0"]);
    expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.energy").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0", "object:energy.energy.hull"]);
    expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.structure").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0", "object:energy.energy.hull"]);
  });
}
