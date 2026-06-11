// #region 🧲Header
/** @emoji 🧠 `@reasoning/mindmap/react` — mindmap fixture types; render via `@puzzle/2d/react` `graphPortMode="normal"`. */
// #endregion 🧲Header

export type {
  CameraState as MindmapCameraState,
  Puzzle2dFixtureCircleNodeV1,
  Puzzle2dFixtureEdgeV1 as MindmapFixtureEdgeV1,
  Puzzle2dFixtureNodeV1 as MindmapFixtureNodeV1,
  Puzzle2dFixtureV1 as MindmapFixtureV1,
} from "@puzzle/2d/react";

import type { Puzzle2dFixtureV1 } from "@puzzle/2d/react";

export function mindmapFixtureKindCatalogsJson(fixture: Puzzle2dFixtureV1): string | undefined {
  const catalogs = fixture.meta?.kindCatalogs;
  if (catalogs == null || typeof catalogs !== "object") {
    return undefined;
  }
  return JSON.stringify(catalogs);
}

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("mindmapFixtureKindCatalogsJson", () => {
    it("serializes meta kind catalogs", () => {
      const json = mindmapFixtureKindCatalogsJson({
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [],
        edges: [],
        meta: { kindCatalogs: { nodes: [{ id: "n1", name: "Node", color: "var(--color-light)" }] } },
      });
      expect(json).toContain("n1");
    });
  });
}
// #endregion 🧪Tests
