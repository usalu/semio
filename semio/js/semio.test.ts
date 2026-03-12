// #region 🔖Header
// [👤semio📚js🥼semiotest](semiorepo://p/u/semio/b/l/js/f/semio.test.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

import { InvalidKit, InvalidKitValidation, MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted, DragDesign, DragPieces, DragOffset, DragDiffDesign, ModelSelectionCases } from "@semio/assets";
import { describe, expect, it } from "vitest";
import { detectStorybookLaunchKind, isStorybookIndexPayload, readLaunchKind } from "./dev";
import { buildControlTree, ControlDef } from "./sketchpad/elements";
import {
  applyDesignDiff,
  applyKitDiff,
  areKitDiffsEqual,
  areKitsEqual,
  areValidationResultsEqual,
  createClusteredDesign,
  Design,
  deserializeKit,
  dragPiecesInDesign,
  exportKit,
  flattenDesign,
  getIncludedDesigns,
  getKitChange,
  getKitDiff,
  hasErrors,
  importKit,
  inverseKitDiff,
  Kit,
  Model,
  Plane,
  replaceClusterWithDesign,
  selectBestModel,
  serializeKit,
  validateKit,
  ValidationResult,
} from "./semio";
import { applySelectionComposition, getNextPanelVisibilityFromToggle, resolveSelectionCompositionKind, ToolKind } from "./sketchpad/shared";

const TOLERANCE = 0.001;

const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
  if (!p1 || !p2) return false;
  if (!p1.origin || !p2.origin || !p1.xAxis || !p2.xAxis || !p1.yAxis || !p2.yAxis) return false;
  return (
    Math.abs(p1.origin.x - p2.origin.x) < TOLERANCE &&
    Math.abs(p1.origin.y - p2.origin.y) < TOLERANCE &&
    Math.abs(p1.origin.z - p2.origin.z) < TOLERANCE &&
    Math.abs(p1.xAxis.x - p2.xAxis.x) < TOLERANCE &&
    Math.abs(p1.xAxis.y - p2.xAxis.y) < TOLERANCE &&
    Math.abs(p1.xAxis.z - p2.xAxis.z) < TOLERANCE &&
    Math.abs(p1.yAxis.x - p2.yAxis.x) < TOLERANCE &&
    Math.abs(p1.yAxis.y - p2.yAxis.y) < TOLERANCE &&
    Math.abs(p1.yAxis.z - p2.yAxis.z) < TOLERANCE
  );
};

const centersEqual = (c1: { u: number; v: number } | undefined, c2: { u: number; v: number } | undefined): boolean => {
  if (!c1 || !c2) return c1 === c2;
  return Math.abs(c1.u - c2.u) < TOLERANCE && Math.abs(c1.v - c2.v) < TOLERANCE;
};

const findDesign = (kit: Kit, name: string, parentName?: string) => {
  let parentGuid: string | undefined;
  if (parentName) {
    const p = kit.designs?.find((d) => d.name === parentName);
    if (!p) throw new Error(`Parent ${parentName} not found`);
    parentGuid = p.guid;
  }
  const d = kit.designs?.find((d) => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
  if (!d) throw new Error(`Design ${name} not found`);
  return d;
};

describe("Change", () => {
  describe("Metabolism", () => {
    const kitOriginal = { ...(MetabolismKit as any), designs: (MetabolismKit as any).designs?.filter((d: any) => !d.parent) };
    const kitDiff = MetabolismKitDiff as any;
    const kitDiffInverted = MetabolismKitDiffInverted as any;
    const kitDiffed = MetabolismKitDiffed as any;

    it("Kit + Change.Forward = DiffedKit & DiffedKit + Change.Backward = Kit", () => {
      const change = getKitChange(kitOriginal, kitDiffed);
      const computedDiff = getKitDiff(kitOriginal, kitDiffed);
      expect(areKitDiffsEqual(computedDiff, kitDiff)).toBe(true);
      const computedInverseDiff = inverseKitDiff(kitOriginal, change.forward);
      expect(areKitDiffsEqual(computedInverseDiff, kitDiffInverted)).toBe(true);
      expect(areKitDiffsEqual(change.forward, kitDiff)).toBe(true);
      expect(areKitDiffsEqual(change.backward, kitDiffInverted)).toBe(true);
      const appliedForward = applyKitDiff(kitOriginal, change.forward);
      expect(areKitsEqual(appliedForward, kitDiffed)).toBe(true);
      const appliedInverse = applyKitDiff(kitDiffed, change.backward);
      expect(areKitsEqual(appliedInverse, kitOriginal)).toBe(true);
    });

    describe("Design/Model", () => {
      it("selectBestModel uses tag filtering + modified jaccard and matches shared semio asset cases", () => {
        const payload = ModelSelectionCases as {
          cases: Array<{
            name: string;
            selectedTagGuids: string[];
            expectedGuid: string | null;
            models: Array<{ guid: string; fileGuid: string; tagGuids: string[] }>;
          }>;
        };
        payload.cases.forEach((testCase) => {
          const models: Model[] = testCase.models.map((model) => ({
            guid: model.guid,
            file: { guid: model.fileGuid },
            tags: model.tagGuids.map((guid) => ({ guid })),
          }));
          const selected = selectBestModel(models, testCase.selectedTagGuids);
          expect(selected?.guid ?? null).toBe(testCase.expectedGuid);
        });
      });
    });
  });
});

describe("Flatten", () => {
  const kit = MetabolismKit as Kit;

  const testFlatten = (designName: string, parentName?: string) => {
    const design = findDesign(kit, designName, parentName);
    const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design.guid);
    expect(expectedDesign).toBeDefined();
    const flatDesignDiff = flattenDesign(kit, design.guid);
    const flatDesign = applyDesignDiff(design, flatDesignDiff);

    flatDesign!.pieces?.forEach((p) => {
      const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
      expect(expectedPiece).toBeDefined();
      expect(p.plane).toBeDefined();
      expect(p.center).toBeDefined();
      expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
      expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
    });
  };

  describe("Nakagin Capsule Tower", () => {
    it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
      testFlatten("Nakagin Capsule Tower");
    });
    describe("Slanted", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Slanted", "Nakagin Capsule Tower");
      });
    });
    describe("Twisted", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Twisted", "Nakagin Capsule Tower");
      });
    });
    describe("Dancing", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Dancing", "Nakagin Capsule Tower");
      });
    });
  });

  describe("Capsule Dream", () => {
    it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
      testFlatten("Capsule Dream");
    });
  });
});

describe("Roundtrip", () => {
  describe("Metabolism", () => {
    it("Json -> Memory -> Json, Json -> Zip, Zip -> Json", async () => {
      const fs = await import("node:fs");
      const path = await import("node:path");

      const kit = MetabolismKit as unknown as Kit;
      const serializedKit = serializeKit(kit);
      const deserializedKit = deserializeKit(serializedKit);
      expect(areKitsEqual(kit, deserializedKit)).toBe(true);

      const zipPath = path.join(__dirname, "../assets/semio/metabolism.zip");
      const zipBuffer = fs.readFileSync(zipPath);
      const { kit: zipKit } = await importKit(zipBuffer.buffer);
      expect(areKitsEqual(kit, zipKit)).toBe(true);

      const exportedZip = await exportKit(kit);
      const { kit: reKit } = await importKit(exportedZip);
      expect(areKitsEqual(kit, reKit)).toBe(true);
    });
  });
});

describe("Validation", () => {
  describe("Metabolism", () => {
    it("Metabolism Kit -> Validate = Empty report", () => {
      const validKit = MetabolismKit as unknown as Kit;
      expect(hasErrors(validateKit(validKit))).toBe(false);
    });
  });

  describe("Invalid", () => {
    it("Invalid Kit -> Validate = Invalid Report", () => {
      const invalidKit = InvalidKit as unknown as Kit;
      const result = validateKit(invalidKit);
      const expected = InvalidKitValidation as unknown as ValidationResult;
      expect(areValidationResultsEqual(result, expected)).toBe(true);
    });
  });
});

describe("Cluster", () => {
  it("Cluster replacement uses design-guid designPiece and yields included design entry", () => {
    const design = {
      guid: "design-root",
      name: "Root",
      pieces: [
        { guid: "piece-a", type: { guid: "type-a" } },
        { guid: "piece-b", type: { guid: "type-b" } },
        { guid: "piece-c", type: { guid: "type-c" } },
      ],
      connections: [
        {
          guid: "conn-ab",
          connecting: { piece: { guid: "piece-a" } },
          connected: { piece: { guid: "piece-b" } },
        },
        {
          guid: "conn-bc",
          connecting: { piece: { guid: "piece-b" } },
          connected: { piece: { guid: "piece-c" } },
        },
      ],
      createdAt: "2025-01-01T00:00:00.000Z",
      updatedAt: "2025-01-01T00:00:00.000Z",
    } as Design;

    const { clusteredDesign, externalConnections } = createClusteredDesign(design, ["piece-a", "piece-b"], "Cluster");
    const diff = replaceClusterWithDesign(design, ["piece-a", "piece-b"], clusteredDesign, externalConnections);
    const updatedDesign = applyDesignDiff(design, diff);

    const clusterConnection = updatedDesign.connections?.find((c) => c.guid === "conn-bc");
    expect(clusterConnection?.connecting.designPiece?.guid).toBe(clusteredDesign.guid);
    expect(clusterConnection?.connected.designPiece?.guid).toBeUndefined();

    const included = getIncludedDesigns(updatedDesign);
    expect(included.length).toBe(1);
    expect(included[0].guid).toBe(clusteredDesign.guid);
    expect(included[0].designGuid).toBe(clusteredDesign.guid);
  });
});

describe("Drag", () => {
  it("Design + Pieces + Offset = DiffDesign", () => {
    const design = DragDesign as unknown as Design;
    const pieces = DragPieces as unknown as Design;
    const offset = DragOffset as { u: number; v: number };
    const expectedDiff = DragDiffDesign as any;
    const computedDiff = dragPiecesInDesign(design, pieces, offset);
    const computedPieceUpdates = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.guid.localeCompare(b.piece.guid));
    const expectedPieceUpdates = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.guid.localeCompare(b.piece.guid));
    expect(computedPieceUpdates.length).toBe(expectedPieceUpdates.length);
    for (let i = 0; i < computedPieceUpdates.length; i++) {
      expect(computedPieceUpdates[i].piece.guid).toBe(expectedPieceUpdates[i].piece.guid);
      expect(computedPieceUpdates[i].diff.center?.u).toBe(expectedPieceUpdates[i].diff.center.u);
      expect(computedPieceUpdates[i].diff.center?.v).toBe(expectedPieceUpdates[i].diff.center.v);
    }
    const computedConnUpdates = (computedDiff.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
    const expectedConnUpdates = (expectedDiff.connections?.updated ?? []).sort((a: any, b: any) => a.connection.guid.localeCompare(b.connection.guid));
    expect(computedConnUpdates.length).toBe(expectedConnUpdates.length);
    for (let i = 0; i < computedConnUpdates.length; i++) {
      expect(computedConnUpdates[i].connection.guid).toBe(expectedConnUpdates[i].connection.guid);
      expect(computedConnUpdates[i].diff.u).toBe(expectedConnUpdates[i].diff.u);
      expect(computedConnUpdates[i].diff.v).toBe(expectedConnUpdates[i].diff.v);
    }
  });
});

describe("Sketchpad Selection Composition", () => {
  it("applies replace/additive/subtractive/intersect and resolves mode from tools/modifiers", () => {
    expect(applySelectionComposition(["a", "b", "a"], ["c", "c", "b"], "replace")).toEqual(["c", "b"]);
    expect(applySelectionComposition(["a", "b"], ["b", "c", "c"], "additive")).toEqual(["a", "b", "c"]);
    expect(applySelectionComposition(["a", "b", "c", "b"], ["b", "x"], "subtractive")).toEqual(["a", "c"]);
    expect(applySelectionComposition(["a", "b", "c"], ["c", "a", "x"], "intersect")).toEqual(["a", "c"]);
    expect(applySelectionComposition(["a", "b"], [], "replace")).toEqual([]);
    expect(applySelectionComposition(["a", "b"], [], "additive")).toEqual(["a", "b"]);
    expect(applySelectionComposition(["a", "b"], [], "subtractive")).toEqual(["a", "b"]);
    expect(applySelectionComposition(["a", "b"], [], "intersect")).toEqual([]);
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL)).toBe("replace");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_ADDITIVE)).toBe("additive");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_SUBTRACTIVE)).toBe("subtractive");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_INTERSECT)).toBe("intersect");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, { shiftKey: true })).toBe("additive");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, { ctrlKey: true })).toBe("subtractive");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, { altKey: true })).toBe("subtractive");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, { metaKey: true })).toBe("subtractive");
    expect(resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, { shiftKey: true, ctrlKey: true })).toBe("intersect");
  });
});

describe("Sketchpad ControlTree", () => {
  it("builds nested folders from paths and applies case-insensitive filter on leaf keys", () => {
    const controls: ControlDef[] = [
      { path: "Transform/Position/X", controlKind: "number", value: 1, onChange: () => { } },
      { path: "Transform/Position/Y", controlKind: "number", value: 2, onChange: () => { } },
      { path: "Appearance/Material/roughness", controlKind: "slider", value: 0.5, onChange: () => { } },
    ];
    const folderSettings = {
      Transform: { path: "Transform", order: 2 },
      "Appearance/Material": { path: "Appearance/Material", order: 1, collapsed: true },
    };
    const fullTree = buildControlTree(controls, "", folderSettings);
    expect(Object.keys(fullTree)).toEqual(["Transform", "Appearance"]);
    expect(fullTree.Transform.kind).toBe("folder");
    expect(fullTree.Transform.order).toBe(2);
    expect(fullTree.Transform.children?.Position.kind).toBe("folder");
    expect(fullTree.Transform.children?.Position.children?.X.kind).toBe("control");
    expect(fullTree.Appearance.children?.Material.order).toBe(1);
    const filteredTree = buildControlTree(controls, "rouGH", folderSettings);
    expect(Object.keys(filteredTree)).toEqual(["Appearance"]);
    expect(filteredTree.Appearance.children?.Material.children?.roughness.kind).toBe("control");
    expect(filteredTree.Appearance.children?.Material.children?.roughness.path).toBe("Appearance/Material/roughness");
  });
});

describe("Sketchpad Panel Visibility", () => {
  it("keeps chat, settings, and property tabs mutually exclusive when toggled on", () => {
    const initialVisibility = {
      rightSidePanel: false,
      chat: false,
      settings: false,
      leftSidePanel: false,
    };
    const propertyVisible = getNextPanelVisibilityFromToggle(initialVisibility, "rightSidePanel");
    expect(propertyVisible.rightSidePanel).toBe(true);
    expect(propertyVisible.chat).toBe(false);
    expect(propertyVisible.settings).toBe(false);
    const chatVisible = getNextPanelVisibilityFromToggle(propertyVisible, "chat");
    expect(chatVisible.rightSidePanel).toBe(false);
    expect(chatVisible.chat).toBe(true);
    expect(chatVisible.settings).toBe(false);
    const settingsVisible = getNextPanelVisibilityFromToggle(chatVisible, "settings");
    expect(settingsVisible.rightSidePanel).toBe(false);
    expect(settingsVisible.chat).toBe(false);
    expect(settingsVisible.settings).toBe(true);
    const settingsHidden = getNextPanelVisibilityFromToggle(settingsVisible, "settings");
    expect(settingsHidden.rightSidePanel).toBe(false);
    expect(settingsHidden.chat).toBe(false);
    expect(settingsHidden.settings).toBe(false);
    const leftVisible = getNextPanelVisibilityFromToggle(settingsVisible, "leftSidePanel");
    expect(leftVisible.leftSidePanel).toBe(true);
    expect(leftVisible.settings).toBe(true);
  });
});

describe("JS Dev Launcher", () => {
  it("classifies storybook launches as start, reuse, or fail and parses the wrapper inputs", async () => {
    await expect(detectStorybookLaunchKind(async () => true, async () => false)).resolves.toBe("start");
    await expect(detectStorybookLaunchKind(async () => false, async () => true)).resolves.toBe("reuse");
    await expect(detectStorybookLaunchKind(async () => false, async () => false)).resolves.toBe("fail");
    expect(isStorybookIndexPayload("{\"v\":5,\"entries\":{}}")).toBe(true);
    expect(isStorybookIndexPayload("{\"hello\":\"world\"}")).toBe(false);
    expect(readLaunchKind(["storybook"])).toBe("storybook");
    expect(readLaunchKind([])).toBe("workspace");
  });
});
