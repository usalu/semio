/** 🖱️ The hover key, committed mesh material, selection isolation and toggle group tests. */
import type { SpatialPickTarget } from "../../../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import type { SpatialInteractionSelectionByState } from "../../../🧾️repl-model/🟦️.tsx";
import type { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ResolvedTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 🔦 Registers the hover key aliases, committed mesh materials, per state selection isolation and toggle group state tests. */
export function registerHoverMaterialsAndIsolationTests(kit: ReplTestKit): void {
  const { COMMITTED_MESH_FACE_OPACITY, M, Model, applyModelDiff, canvasHoverKeyForSelectionTarget, defaultModelDefinitionId, expect, filterSpatialPickTargets, it, replDisplayedSelectionTargets, replMergeSelectionPickInView, resolveCommittedMeshMaterialProps, solidRef, spatialPickTargetKey, spatialSceneColors, spatialToggleCheckboxState, spatialToggleGroupFill, spatialToggleGroupState } = kit;
  it("canvasHoverKeyForSelectionTarget maps primitive picks to typology object hover keys", () => {
    const model = new Model();
    const cell = solidRef("c0");
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
    model.objects["object-c0"] = {
      id: "object-c0" as ObjectRef,
      typology: "energy.energy.hull" as TypologyRef,
      primitives: { solid: String(cell) },
    };
    expect(canvasHoverKeyForSelectionTarget(model, "aec.building.energy", { kind: "solid", id: String(cell), editable: true })).toBe("object:object-c0");
    expect(canvasHoverKeyForSelectionTarget(model, "aec.building.energy", { kind: "object", id: "object-c0", editable: true })).toBe("object:object-c0");
  });
  it("resolveCommittedMeshMaterialProps resolves selection, hover, and style fallback materials", () => {
    const palette = spatialSceneColors();
    const solidId = solidRef("s0");

    const defaultProps = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, null, null, null);
    expect(defaultProps.color).toBe(palette.committed);
    expect(defaultProps.emissive).toBe(palette.committedEmissive);
    expect(defaultProps.opacity).toBe(COMMITTED_MESH_FACE_OPACITY);

    const hoveredPropsObj = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, "object:s0", null, null);
    expect(hoveredPropsObj.color).toBe(palette.hovered);
    expect(hoveredPropsObj.emissive).toBe(palette.hoveredEmissive);
    expect(hoveredPropsObj.opacity).toBe(0.28);

    const hoveredPropsSolid = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, "solid:s0", null, null);
    expect(hoveredPropsSolid.color).toBe(palette.hovered);

    const selectedProps = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, null, "object:s0", null);
    expect(selectedProps.color).toBe(palette.selected);
    expect(selectedProps.emissive).toBe(palette.selectedEmissive);
    expect(selectedProps.opacity).toBe(0.34);

    const selectedKeysProps = resolveCommittedMeshMaterialProps(undefined, undefined, solidId, null, null, new Set(["solid:s0"]));
    expect(selectedKeysProps.color).toBe(palette.selected);

    const style: ResolvedTypologyStyle = {
      color: "#ff00ff",
      edgeColor: "#00ffff",
      opacity: 0.5,
      pattern: { kind: "none", direction: 0, spacing: 0.35, lineWidth: 0.03, color: "#000000" },
    };
    const styleProps = resolveCommittedMeshMaterialProps(style, undefined, solidId, null, null, null);
    expect(styleProps.color).toBe("#ff00ff");
    expect(styleProps.opacity).toBe(0.5);
  });
  it("keeps interaction selection isolated per state", () => {
    const interactionByState: SpatialInteractionSelectionByState = {
      first_corner: [{ kind: "vertex", id: "v0", editable: true }],
      second_corner: [{ kind: "vertex", id: "v1", editable: true }],
    };
    expect(replDisplayedSelectionTargets(true, defaultModelDefinitionId(), "first_corner", {}, interactionByState)).toEqual([{ kind: "vertex", id: "v0", editable: true }]);
    expect(replMergeSelectionPickInView(true, defaultModelDefinitionId(), "second_corner", {}, interactionByState, [{ kind: "edge", id: "e0", editable: true }], { shift: true })).toEqual([
      { kind: "vertex", id: "v1", editable: true },
      { kind: "edge", id: "e0", editable: true },
    ]);
  });
  it("spatialToggleGroupState reports all, none, and partial chrome groups", () => {
    expect(spatialToggleGroupState(["a", "b"], { a: true, b: true })).toBe("all");
    expect(spatialToggleGroupState(["a", "b"], { a: false, b: false })).toBe("none");
    expect(spatialToggleGroupState(["a", "b"], { a: true, b: false })).toBe("partial");
    expect(spatialToggleGroupFill(["a", "b"], true)).toEqual({ a: true, b: true });
    expect(spatialToggleGroupFill(["a", "b"], false)).toEqual({ a: false, b: false });
    expect(spatialToggleCheckboxState("all")).toBe(true);
    expect(spatialToggleCheckboxState("none")).toBe(false);
    expect(spatialToggleCheckboxState("partial")).toBe("indeterminate");
  });
  it("filterSpatialPickTargets matches primitive geometryKind in selection accept", () => {
    const targets: SpatialPickTarget[] = [
      { kind: "face", geometryKind: "shell", id: "sh0", point: [0, 0, 0] },
      { kind: "vertex", geometryKind: "anchor", id: "a0", point: [1, 0, 0] },
    ];
    expect(filterSpatialPickTargets(targets, ["shell"], {}).map(spatialPickTargetKey)).toEqual(["face:sh0"]);
    expect(filterSpatialPickTargets(targets, ["anchor"], {}).map(spatialPickTargetKey)).toEqual(["vertex:a0"]);
  });
}
