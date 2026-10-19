/** 🎚️ The repl chrome toggles and the pick target filter tests. */
import type { AnchorRef, EdgeRef, ShellRef, VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { SpatialRendererSelectionByModel } from "../../../🧾️repl-model/🟦️.tsx";
import type { DisplayModel } from "../../../../🎬️actions/🖼️display-model/🟦️.ts";
import type { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 🔀 Registers the chrome toggle seeds, footprint preview, primitive and typology toggle filters and selection scope tests. */
export function registerChromeTogglesAndPickFilterTests(kit: ReplTestKit): void {
  const { M, Model, applyModelDiff, createSpatialPickTargets, defaultInteractionReplChromeState, defaultModelDefinitionId, expect, filterFootprintBoxPreviewDisplayItems, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, historyEntryArchivesBoxFootprint, it, modelDefinitionPickTargetKinds, modelDefinitionTypologyIds, pinnedPickTargetKeys, replDisplayedSelectionTargets, replMergeSelectionPickInView, replRendererSelectionTargets, solidRef, spatialHoverKeyAliases, spatialHoverKeysMatch, spatialTypologyToggleLabel } = kit;
  it("defaultInteractionReplChromeState seeds typology and primitive toggles by default", () => {
    const chrome = defaultInteractionReplChromeState();
    expect(chrome.activeModelDefinitionId).toBe(defaultModelDefinitionId());
    expect(chrome.filterTypologyToggles["spatial.shape.primitive.box"]).toBe(true);
    expect(chrome.filterPrimitiveToggles.vertex).toBe(true);
    expect(chrome.filterPrimitiveToggles.solid).toBe(true);
  });
  it("filterFootprintBoxPreviewDisplayItems removes box-preview when model has solids", () => {
    const model = new Model();
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
    const display: DisplayModel = {
      items: [
        { kind: "box-preview", id: "preview-committed", params: { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 } },
        { kind: "point", id: "p0", params: { position: [0, 0, 0] } },
      ],
    };
    const filtered = filterFootprintBoxPreviewDisplayItems(display, model);
    expect(filtered.items.some((item) => item.kind === "box-preview")).toBe(false);
    expect(filtered.items.some((item) => item.kind === "point")).toBe(true);
  });
  it("historyEntryArchivesBoxFootprint skips transform and measure interactions", () => {
    expect(historyEntryArchivesBoxFootprint("transform.move")).toBe(false);
    expect(historyEntryArchivesBoxFootprint("transform.copy")).toBe(false);
    expect(historyEntryArchivesBoxFootprint("measure.vertexDistance")).toBe(false);
    expect(historyEntryArchivesBoxFootprint("primitive.box")).toBe(true);
  });
  it("filterSpatialPickTargetsForPrimitiveToggles hides primitive picks by kind", () => {
    const model = new Model();
    model.vertices.v0 = { id: "v0" as VertexRef, position: [0, 0, 0] };
    model.edges.e0 = { id: "e0" as EdgeRef, vertexIds: ["v0" as VertexRef, "v0" as VertexRef], curve: { kind: "line" } };
    const targets = createSpatialPickTargets(model);
    const visible = filterSpatialPickTargetsForPrimitiveToggles(targets, { vertex: false });
    expect(visible.some((row) => row.geometryKind === "vertex")).toBe(false);
    expect(visible.some((row) => row.geometryKind === "edge")).toBe(true);
  });
  it("spatialTypologyToggleLabel uses typology label pascal case", () => {
    expect(spatialTypologyToggleLabel("energy.energy.baseplate", "Base Plate")).toBe("BasePlate");
    expect(spatialTypologyToggleLabel("spatial.shape.primitive.box", "Box")).toBe("Box");
  });
  it("filterSpatialPickTargetsForTypologyToggles hides typology object picks", async () => {
    const model = new Model();
    const cell = solidRef("c0");
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
    model.objects["hull"] = {
      id: "hull" as ObjectRef,
      typology: "energy.energy.hull" as TypologyRef,
      primitives: { solid: String(cell) },
    };
    const targets = createSpatialPickTargets(model, "aec.building.energy");
    const typologyIds = modelDefinitionTypologyIds("aec.building.energy");
    const visible = filterSpatialPickTargetsForTypologyToggles(targets, { "energy.energy.hull": false }, typologyIds);
    expect(visible.some((row) => row.typologyId === "energy.energy.hull")).toBe(false);
  });
  it("scopes displayed selection to activeModelDefinitionId", () => {
    const rendererByModel: SpatialRendererSelectionByModel = {
      [defaultModelDefinitionId()]: [{ kind: "face", id: "f0", editable: true }],
      "aec.building.energy": [
        { kind: "face", id: "f0", editable: true },
        { kind: "object", id: "o0", editable: false },
      ],
    };
    expect(replDisplayedSelectionTargets(false, defaultModelDefinitionId(), "idle", rendererByModel, {})).toEqual([{ kind: "face", id: "f0", editable: true }]);
    expect(replDisplayedSelectionTargets(false, "aec.building.energy", "idle", rendererByModel, {})).toEqual([
      { kind: "face", id: "f0", editable: true },
      { kind: "object", id: "o0", editable: false },
    ]);
    expect(replDisplayedSelectionTargets(false, "aec.building.structure", "idle", rendererByModel, {})).toEqual([]);
  });
  it("creates anchor and shell pick targets for spatial.shape geometry", () => {
    const model = new Model();
    model.anchors["a0"] = { id: "a0" as AnchorRef, position: [0, 0, 0], attachment: { kind: "vertex", id: "v0" as VertexRef } };
    model.vertices["v0"] = { id: "v0" as VertexRef, position: [0, 0, 0] };
    const cell = solidRef("c0");
    applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
    const faceId = Object.values(model.faces)[0]!.id;
    model.shells["sh0"] = { id: "sh0" as ShellRef, faceIds: [faceId] };
    const targets = createSpatialPickTargets(model, defaultModelDefinitionId());
    expect(targets.some((t) => t.geometryKind === "anchor")).toBe(true);
    expect(targets.some((t) => t.geometryKind === "shell")).toBe(true);
  });
  it("modelDefinitionPickTargetKinds maps primitive entity kinds to pick toggles", () => {
    expect([...modelDefinitionPickTargetKinds(defaultModelDefinitionId())].sort()).toEqual(["edge", "face", "object", "vertex"]);
    expect([...modelDefinitionPickTargetKinds("aec.building.structure")].sort()).toEqual(["edge", "face", "object", "vertex"]);
  });
  it("merges picks within active model definition without clearing other models", () => {
    const rendererByModel: SpatialRendererSelectionByModel = {
      [defaultModelDefinitionId()]: [{ kind: "wire", id: "w0", editable: true }],
      "aec.building.energy": [{ kind: "object", id: "o0", editable: false }],
    };
    expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [{ kind: "wire", id: "w1", editable: true }], {})).toEqual([{ kind: "wire", id: "w1", editable: true }]);
    expect(replRendererSelectionTargets(rendererByModel, "aec.building.energy")).toEqual([{ kind: "object", id: "o0", editable: false }]);
  });
  it("clears selection on empty background pick in default modifier mode", () => {
    const rendererByModel: SpatialRendererSelectionByModel = {
      [defaultModelDefinitionId()]: [{ kind: "wire", id: "w0", editable: true }],
    };
    expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [], {})).toEqual([]);
    expect(replMergeSelectionPickInView(false, defaultModelDefinitionId(), "idle", rendererByModel, {}, [], { shift: true })).toEqual([{ kind: "wire", id: "w0", editable: true }]);
  });
  it("maps selection target keys to pick target keys for highlights", () => {
    const keys = pinnedPickTargetKeys(new Set(["shell:sh0" as string]));
    expect(keys.has("shell:sh0")).toBe(true);
    expect(keys.has("face:sh0")).toBe(true);
  });
  it("spatialHoverKeyAliases links object and solid geometry pick keys", () => {
    expect(spatialHoverKeyAliases("solid:foo").has("object:foo")).toBe(true);
    expect(spatialHoverKeyAliases("object:foo").has("solid:foo")).toBe(true);
    expect(spatialHoverKeysMatch("object:foo", "solid:foo")).toBe(true);
  });
}
