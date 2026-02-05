// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 Ueli Saluz

import { describe, expect, it } from "vitest";
import {
  getKitDiagramShapeStrategy,
  KIT_DIAGRAM_CIRCLE_FRAME,
  KIT_DIAGRAM_COLLIDE_RADIUS,
  KIT_DIAGRAM_LONG_RECTANGLE_FRAME,
  KIT_DIAGRAM_RECTANGLE_FRAME,
  KIT_DIAGRAM_TRIANGLE_FRAME,
  resolveKitDiagramAnchorPair,
  resolveKitDiagramProximityAnchor,
} from "./sketchpad/kitSelectionHelpers";

describe("Kit Diagram Shape Strategy", () => {
  it("should map design nodes to circle snap points", () => {
    const strategy = getKitDiagramShapeStrategy("design");
    const snapPoints = strategy.getSnapPoints();
    expect(strategy.id).toBe("circle");
    expect(strategy.frame).toEqual(KIT_DIAGRAM_CIRCLE_FRAME);
    expect(snapPoints).toEqual([
      { id: "n", x: 50, y: 0, side: "top" },
      { id: "e", x: 100, y: 50, side: "right" },
      { id: "s", x: 50, y: 100, side: "bottom" },
      { id: "w", x: 0, y: 50, side: "left" },
    ]);
  });

  it("should map type nodes to rectangle snap points", () => {
    const strategy = getKitDiagramShapeStrategy("type");
    const snapPoints = strategy.getSnapPoints();
    expect(strategy.id).toBe("rectangle");
    expect(strategy.frame).toEqual(KIT_DIAGRAM_RECTANGLE_FRAME);
    expect(snapPoints).toEqual([
      { id: "n", x: 60, y: 0, side: "top" },
      { id: "e", x: 120, y: 40, side: "right" },
      { id: "s", x: 60, y: 80, side: "bottom" },
      { id: "w", x: 0, y: 40, side: "left" },
    ]);
  });

  it("should map file nodes to triangle snap points", () => {
    const strategy = getKitDiagramShapeStrategy("file");
    const snapPoints = strategy.getSnapPoints();
    expect(strategy.id).toBe("triangle");
    expect(strategy.frame).toEqual(KIT_DIAGRAM_TRIANGLE_FRAME);
    expect(snapPoints).toEqual([
      { id: "apex", x: 50, y: 0, side: "top" },
      { id: "base-left", x: 0, y: 100, side: "left" },
      { id: "base-right", x: 100, y: 100, side: "right" },
    ]);
  });

  it("should map default kinds to long-rectangle snap points", () => {
    const strategy = getKitDiagramShapeStrategy("quality");
    const snapPoints = strategy.getSnapPoints();
    expect(strategy.id).toBe("long-rectangle");
    expect(strategy.frame).toEqual(KIT_DIAGRAM_LONG_RECTANGLE_FRAME);
    expect(snapPoints).toEqual([
      { id: "n", x: 80, y: 0, side: "top" },
      { id: "e", x: 160, y: 36, side: "right" },
      { id: "s", x: 80, y: 72, side: "bottom" },
      { id: "w", x: 0, y: 36, side: "left" },
    ]);
  });

  it("should expose collision radius from the largest strategy frame", () => {
    expect(KIT_DIAGRAM_COLLIDE_RADIUS).toBe(80);
  });
});

describe("Kit Diagram Anchor Resolution", () => {
  it("should resolve east-west anchors for horizontal circle-rectangle edges", () => {
    const anchors = resolveKitDiagramAnchorPair(
      { kind: "design", position: { x: 0, y: 0 } },
      { kind: "type", position: { x: 300, y: 0 } },
    );
    expect(anchors.source.localPoint.id).toBe("e");
    expect(anchors.target.localPoint.id).toBe("w");
    expect(anchors.source.absolutePoint).toEqual({ x: 100, y: 50 });
    expect(anchors.target.absolutePoint).toEqual({ x: 300, y: 40 });
  });

  it("should resolve south-north anchors for vertical rectangle-circle edges", () => {
    const anchors = resolveKitDiagramAnchorPair(
      { kind: "type", position: { x: 0, y: 0 } },
      { kind: "design", position: { x: 0, y: 280 } },
    );
    expect(anchors.source.localPoint.id).toBe("s");
    expect(anchors.target.localPoint.id).toBe("n");
    expect(anchors.source.absolutePoint).toEqual({ x: 60, y: 80 });
    expect(anchors.target.absolutePoint).toEqual({ x: 50, y: 280 });
  });

  it("should resolve apex anchor for upward triangle connections", () => {
    const anchors = resolveKitDiagramAnchorPair(
      { kind: "file", position: { x: 220, y: 320 } },
      { kind: "design", position: { x: 220, y: 40 } },
    );
    expect(anchors.source.localPoint.id).toBe("apex");
    expect(anchors.target.localPoint.id).toBe("s");
  });

  it("should resolve proximity anchors from snap points", () => {
    const proximity = resolveKitDiagramProximityAnchor("type:1", { kind: "type", position: { x: 200, y: 100 } }, { x: 340, y: 140 });
    expect(proximity.anchor.localPoint.id).toBe("e");
    expect(proximity.anchor.absolutePoint).toEqual({ x: 320, y: 140 });
  });
});
