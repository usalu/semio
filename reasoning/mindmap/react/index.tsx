// #region 🧲Header
/** @emoji 🧠 `@semio-tech/reasoning-mindmap-react` — mindmap fixture types; render via `@semio-tech/puzzle-2d-react` `graphPortMode="normal"`. */
// #endregion 🧲Header

export type {
  CameraState as MindmapCameraState,
  Puzzle2dFixtureCircleNode,
  Puzzle2dFixtureEdge as MindmapFixtureEdge,
  Puzzle2dFixtureNode as MindmapFixtureNode,
  Puzzle2dFixture as MindmapFixture,
} from "@semio-tech/puzzle-2d-react";

import type { Puzzle2dFixture } from "@semio-tech/puzzle-2d-react";

export function mindmapFixtureKindCatalogsJson(fixture: Puzzle2dFixture): string | undefined {
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
        schema: "puzzle.2d.fixture",
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
