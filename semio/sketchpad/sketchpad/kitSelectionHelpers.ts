// #region Header

// js/semio/sketchpad/kitSelectionHelpers.ts

// SPDX-License-Identifier: LGPL-3.0-or-later

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { ICON_WIDTH } from "@semio/js/semio";
import type { KitAppSelection } from "./Kit";

// #endregion Imports

// #region Types

/**
 * Helper type to extract array element type from a selection dimension
 */
export type SelectionValue<K extends keyof KitAppSelection> = NonNullable<KitAppSelection[K]> extends (infer T)[] ? T : never;

// #endregion Types

// #region Generic Utilities

/**
 * Adds a value to a selection dimension without clearing other dimensions.
 * @param selection - Current selection object
 * @param key - Dimension key (e.g., "types", "designs")
 * @param value - Value to add (e.g., guid)
 * @returns New selection object with value added
 * @example
 * const newSelection = addToSelection(
 *   { types: ["guid1"] },
 *   "types",
 *   "guid2"
 * );
 * // Result: { types: ["guid1", "guid2"] }
 */
export function addToSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];

  if (currentArray.includes(value)) {
    return selection;
  }

  return {
    ...selection,
    [key]: [...currentArray, value],
  };
}

/**
 * Removes a value from a selection dimension without affecting other dimensions.
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to remove
 * @returns New selection object with value removed
 * @example
 * const newSelection = removeFromSelection(
 *   { types: ["guid1", "guid2"], designs: ["guid3"] },
 *   "types",
 *   "guid2"
 * );
 * // Result: { types: ["guid1"], designs: ["guid3"] }
 */
export function removeFromSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  const newArray = currentArray.filter((v) => v !== value);

  if (newArray.length === 0) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }

  return {
    ...selection,
    [key]: newArray,
  };
}

/**
 * Toggles a value in a selection dimension (add if missing, remove if present).
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to toggle
 * @returns New selection object with value toggled
 * @example
 * toggleInSelection({ types: ["guid1"] }, "types", "guid2")
 * // => { types: ["guid1", "guid2"] }
 * toggleInSelection({ types: ["guid1", "guid2"] }, "types", "guid2")
 * // => { types: ["guid1"] }
 */
export function toggleInSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];

  if (currentArray.includes(value)) {
    return removeFromSelection(selection, key, value);
  } else {
    return addToSelection(selection, key, value);
  }
}

/**
 * Replaces an entire selection dimension without affecting other dimensions.
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param values - New values for the dimension (undefined to clear)
 * @returns New selection object with dimension replaced
 * @example
 * replaceSelectionDimension(
 *   { types: ["guid1"], designs: ["guid2"] },
 *   "types",
 *   ["guid3", "guid4"]
 * );
 * // Result: { types: ["guid3", "guid4"], designs: ["guid2"] }
 */
export function replaceSelectionDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, values: KitAppSelection[K] | undefined): KitAppSelection {
  if (!values || (Array.isArray(values) && values.length === 0)) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }

  return {
    ...selection,
    [key]: values,
  };
}

/**
 * Clears a single selection dimension without affecting others.
 * @param selection - Current selection object
 * @param key - Dimension key to clear
 * @returns New selection object with dimension cleared
 * @example
 * clearSelectionDimension({ types: ["guid1"], designs: ["guid2"] }, "types")
 * // Result: { designs: ["guid2"] }
 */
export function clearSelectionDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K): KitAppSelection {
  const { [key]: _, ...rest } = selection;
  return rest;
}

/**
 * Clears all selection dimensions.
 * @returns Empty selection object
 * @example
 * clearSelection()
 * // Result: {}
 */
export function clearSelection(): KitAppSelection {
  return {};
}

/**
 * Selects all items in a dimension (replaces existing selection for that dimension).
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param allValues - All available values for the dimension
 * @returns New selection object with all values selected
 * @example
 * selectAllInDimension({ types: ["guid1"] }, "types", ["guid1", "guid2", "guid3"])
 * // Result: { types: ["guid1", "guid2", "guid3"] }
 */
export function selectAllInDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, allValues: SelectionValue<K>[]): KitAppSelection {
  return replaceSelectionDimension(selection, key, allValues as KitAppSelection[K]);
}

/**
 * Checks if a value is selected in a dimension.
 * @param selection - Current selection object
 * @param key - Dimension key
 * @param value - Value to check
 * @returns True if value is selected
 */
export function isSelected<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): boolean {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  return currentArray.includes(value);
}

// #endregion Generic Utilities

// #region Kit Diagram Geometry

export type KitDiagramNodeKind = "type" | "design" | "quality" | "port" | "tag" | "concept" | "file" | "folder" | "author";
export type KitDiagramShapeId = "circle" | "rectangle" | "triangle" | "long-rectangle";
export type KitDiagramSnapSide = "top" | "right" | "bottom" | "left";

export interface KitDiagramFrame {
  width: number;
  height: number;
}

export interface KitDiagramPoint {
  x: number;
  y: number;
}

export interface KitDiagramSnapPoint extends KitDiagramPoint {
  id: string;
  side: KitDiagramSnapSide;
}

export interface KitDiagramShapeRenderPayload {
  className?: string;
  style?: Record<string, string | number>;
}

export interface KitDiagramShapeStrategy {
  id: KitDiagramShapeId;
  frame: KitDiagramFrame;
  getRenderPayload: () => KitDiagramShapeRenderPayload;
  getSnapPoints: (frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint[];
  resolveNearestPoint: (targetVector: KitDiagramPoint, frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint;
}

export interface KitDiagramResolvedAnchor {
  strategyId: KitDiagramShapeId;
  frame: KitDiagramFrame;
  localPoint: KitDiagramSnapPoint;
  absolutePoint: KitDiagramPoint;
  center: KitDiagramPoint;
}

export interface KitDiagramNodeGeometryInput {
  kind: KitDiagramNodeKind;
  position: KitDiagramPoint;
  frame?: Partial<KitDiagramFrame>;
}

export interface KitDiagramResolvedAnchorPair {
  source: KitDiagramResolvedAnchor;
  target: KitDiagramResolvedAnchor;
}

export interface KitDiagramProximityAnchor {
  nodeId: string;
  distance: number;
  anchor: KitDiagramResolvedAnchor;
}

export const KIT_DIAGRAM_NODE_SCALE = 2;
export const KIT_DIAGRAM_BASE_SIZE = ICON_WIDTH * KIT_DIAGRAM_NODE_SCALE;
export const KIT_DIAGRAM_CIRCLE_FRAME: KitDiagramFrame = { width: KIT_DIAGRAM_BASE_SIZE, height: KIT_DIAGRAM_BASE_SIZE };
export const KIT_DIAGRAM_RECTANGLE_FRAME: KitDiagramFrame = { width: Math.round(KIT_DIAGRAM_BASE_SIZE * 1.2), height: Math.round(KIT_DIAGRAM_BASE_SIZE * 0.8) };
export const KIT_DIAGRAM_TRIANGLE_FRAME: KitDiagramFrame = { width: KIT_DIAGRAM_BASE_SIZE, height: KIT_DIAGRAM_BASE_SIZE };
export const KIT_DIAGRAM_LONG_RECTANGLE_FRAME: KitDiagramFrame = { width: Math.round(KIT_DIAGRAM_BASE_SIZE * 1.6), height: Math.round(KIT_DIAGRAM_BASE_SIZE * 0.72) };
export const KIT_DIAGRAM_COLLIDE_RADIUS =
  Math.max(
    KIT_DIAGRAM_CIRCLE_FRAME.width,
    KIT_DIAGRAM_CIRCLE_FRAME.height,
    KIT_DIAGRAM_RECTANGLE_FRAME.width,
    KIT_DIAGRAM_RECTANGLE_FRAME.height,
    KIT_DIAGRAM_TRIANGLE_FRAME.width,
    KIT_DIAGRAM_TRIANGLE_FRAME.height,
    KIT_DIAGRAM_LONG_RECTANGLE_FRAME.width,
    KIT_DIAGRAM_LONG_RECTANGLE_FRAME.height,
  ) / 2;

export const normalizeKitDiagramFrame = (frame?: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramFrame => {
  const width = frame?.width ?? fallback.width;
  const height = frame?.height ?? fallback.height;
  return {
    width: Number.isFinite(width) && width > 0 ? width : fallback.width,
    height: Number.isFinite(height) && height > 0 ? height : fallback.height,
  };
};

export const kitDiagramCenter = (frame: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramPoint => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  return { x: normalizedFrame.width / 2, y: normalizedFrame.height / 2 };
};

export const kitDiagramVector = (from: KitDiagramPoint, to: KitDiagramPoint): KitDiagramPoint => ({ x: to.x - from.x, y: to.y - from.y });
export const kitDiagramVectorLength = (vector: KitDiagramPoint): number => Math.hypot(vector.x, vector.y);
export const kitDiagramNormalizeVector = (vector: KitDiagramPoint): KitDiagramPoint => {
  const length = kitDiagramVectorLength(vector);
  if (length === 0) return { x: 0, y: 0 };
  return { x: vector.x / length, y: vector.y / length };
};
export const kitDiagramDot = (a: KitDiagramPoint, b: KitDiagramPoint): number => a.x * b.x + a.y * b.y;
export const kitDiagramDistanceSquared = (a: KitDiagramPoint, b: KitDiagramPoint): number => {
  const dx = a.x - b.x;
  const dy = a.y - b.y;
  return dx * dx + dy * dy;
};
export const kitDiagramToAbsolutePoint = (origin: KitDiagramPoint, localPoint: KitDiagramPoint): KitDiagramPoint => ({
  x: origin.x + localPoint.x,
  y: origin.y + localPoint.y,
});
export const kitDiagramInferSnapSide = (point: KitDiagramPoint, frame: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramSnapSide => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  const center = kitDiagramCenter(normalizedFrame, fallback);
  const dx = point.x - center.x;
  const dy = point.y - center.y;
  if (Math.abs(dx) > Math.abs(dy)) {
    return dx >= 0 ? "right" : "left";
  }
  return dy >= 0 ? "bottom" : "top";
};

const createCircleSnapPoints = (frame?: Partial<KitDiagramFrame>): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, KIT_DIAGRAM_CIRCLE_FRAME);
  const center = kitDiagramCenter(normalizedFrame, KIT_DIAGRAM_CIRCLE_FRAME);
  return [
    { id: "n", x: center.x, y: 0, side: "top" },
    { id: "e", x: normalizedFrame.width, y: center.y, side: "right" },
    { id: "s", x: center.x, y: normalizedFrame.height, side: "bottom" },
    { id: "w", x: 0, y: center.y, side: "left" },
  ];
};

const createRectangleSnapPoints = (frame?: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_RECTANGLE_FRAME): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  const center = kitDiagramCenter(normalizedFrame, fallback);
  return [
    { id: "n", x: center.x, y: 0, side: "top" },
    { id: "e", x: normalizedFrame.width, y: center.y, side: "right" },
    { id: "s", x: center.x, y: normalizedFrame.height, side: "bottom" },
    { id: "w", x: 0, y: center.y, side: "left" },
  ];
};

const createTriangleSnapPoints = (frame?: Partial<KitDiagramFrame>): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, KIT_DIAGRAM_TRIANGLE_FRAME);
  return [
    { id: "apex", x: normalizedFrame.width / 2, y: 0, side: "top" },
    { id: "base-left", x: 0, y: normalizedFrame.height, side: "left" },
    { id: "base-right", x: normalizedFrame.width, y: normalizedFrame.height, side: "right" },
  ];
};

const rankSnapPointsByVector = (points: KitDiagramSnapPoint[], frame: Partial<KitDiagramFrame>, targetVector: KitDiagramPoint, fallback: KitDiagramFrame): Array<{ point: KitDiagramSnapPoint; alignment: number; orthogonal: number }> => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  const center = kitDiagramCenter(normalizedFrame, fallback);
  const normalizedTarget = kitDiagramNormalizeVector(targetVector);
  return points
    .map((point) => {
      const fromCenter = kitDiagramVector(center, point);
      const normalizedDirection = kitDiagramNormalizeVector(fromCenter);
      const alignment = kitDiagramDot(normalizedDirection, normalizedTarget);
      const projection = kitDiagramDot(fromCenter, normalizedTarget);
      const projectedPoint = {
        x: normalizedTarget.x * projection,
        y: normalizedTarget.y * projection,
      };
      const orthogonal = kitDiagramVectorLength({
        x: fromCenter.x - projectedPoint.x,
        y: fromCenter.y - projectedPoint.y,
      });
      return { point, alignment, orthogonal };
    })
    .sort((a, b) => {
      if (b.alignment !== a.alignment) return b.alignment - a.alignment;
      if (a.orthogonal !== b.orthogonal) return a.orthogonal - b.orthogonal;
      return a.point.id.localeCompare(b.point.id);
    });
};

export const resolveNearestKitDiagramSnapPoint = (points: KitDiagramSnapPoint[], frame: Partial<KitDiagramFrame>, targetVector: KitDiagramPoint, fallback: KitDiagramFrame): KitDiagramSnapPoint => {
  if (points.length === 0) {
    const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
    const center = kitDiagramCenter(normalizedFrame, fallback);
    return { id: "center", ...center, side: kitDiagramInferSnapSide(center, normalizedFrame, fallback) };
  }
  const ranked = rankSnapPointsByVector(points, frame, targetVector, fallback);
  return ranked[0]?.point ?? points[0];
};

const createStrategy = (id: KitDiagramShapeId, frame: KitDiagramFrame, getSnapPoints: (frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint[], renderPayload: KitDiagramShapeRenderPayload): KitDiagramShapeStrategy => ({
  id,
  frame,
  getRenderPayload: () => renderPayload,
  getSnapPoints: (frameOverride?: Partial<KitDiagramFrame>) => getSnapPoints(frameOverride ?? frame),
  resolveNearestPoint: (targetVector: KitDiagramPoint, frameOverride?: Partial<KitDiagramFrame>) => {
    const resolvedFrame = normalizeKitDiagramFrame(frameOverride, frame);
    const points = getSnapPoints(resolvedFrame);
    return resolveNearestKitDiagramSnapPoint(points, resolvedFrame, targetVector, frame);
  },
});

export const kitDiagramCircleStrategy = createStrategy("circle", KIT_DIAGRAM_CIRCLE_FRAME, createCircleSnapPoints, {});
export const kitDiagramRectangleStrategy = createStrategy("rectangle", KIT_DIAGRAM_RECTANGLE_FRAME, createRectangleSnapPoints, {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
});
export const kitDiagramTriangleStrategy = createStrategy("triangle", KIT_DIAGRAM_TRIANGLE_FRAME, createTriangleSnapPoints, {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
  style: { clipPath: "polygon(50% 0%, 0% 100%, 100% 100%)" },
});
export const kitDiagramLongRectangleStrategy = createStrategy("long-rectangle", KIT_DIAGRAM_LONG_RECTANGLE_FRAME, (frame) => createRectangleSnapPoints(frame, KIT_DIAGRAM_LONG_RECTANGLE_FRAME), {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
});

export const KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY = kitDiagramLongRectangleStrategy;

export const KIT_DIAGRAM_SHAPE_STRATEGY_REGISTRY: Record<KitDiagramNodeKind, KitDiagramShapeStrategy> = {
  design: kitDiagramCircleStrategy,
  type: kitDiagramRectangleStrategy,
  file: kitDiagramTriangleStrategy,
  quality: kitDiagramLongRectangleStrategy,
  port: kitDiagramLongRectangleStrategy,
  tag: kitDiagramLongRectangleStrategy,
  concept: kitDiagramLongRectangleStrategy,
  folder: kitDiagramLongRectangleStrategy,
  author: kitDiagramLongRectangleStrategy,
};

export const getKitDiagramShapeStrategy = (kind: KitDiagramNodeKind): KitDiagramShapeStrategy => KIT_DIAGRAM_SHAPE_STRATEGY_REGISTRY[kind] ?? KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY;

export const getKitDiagramNodeFrameForKind = (kind: KitDiagramNodeKind, override?: Partial<KitDiagramFrame>): KitDiagramFrame => normalizeKitDiagramFrame(override, getKitDiagramShapeStrategy(kind).frame);

export const resolveKitDiagramAnchorPair = (sourceNode: KitDiagramNodeGeometryInput, targetNode: KitDiagramNodeGeometryInput): KitDiagramResolvedAnchorPair => {
  const sourceStrategy = getKitDiagramShapeStrategy(sourceNode.kind);
  const targetStrategy = getKitDiagramShapeStrategy(targetNode.kind);
  const sourceFrame = normalizeKitDiagramFrame(sourceNode.frame, sourceStrategy.frame);
  const targetFrame = normalizeKitDiagramFrame(targetNode.frame, targetStrategy.frame);
  const sourceCenterLocal = kitDiagramCenter(sourceFrame, sourceStrategy.frame);
  const targetCenterLocal = kitDiagramCenter(targetFrame, targetStrategy.frame);
  const sourceCenterAbsolute = kitDiagramToAbsolutePoint(sourceNode.position, sourceCenterLocal);
  const targetCenterAbsolute = kitDiagramToAbsolutePoint(targetNode.position, targetCenterLocal);
  const direction = kitDiagramVector(sourceCenterAbsolute, targetCenterAbsolute);
  const reverseDirection = { x: -direction.x, y: -direction.y };
  const sourceRanked = rankSnapPointsByVector(sourceStrategy.getSnapPoints(sourceFrame), sourceFrame, direction, sourceStrategy.frame);
  const targetRanked = rankSnapPointsByVector(targetStrategy.getSnapPoints(targetFrame), targetFrame, reverseDirection, targetStrategy.frame);
  const sourceCandidates = sourceRanked.slice(0, Math.min(3, sourceRanked.length));
  const targetCandidates = targetRanked.slice(0, Math.min(3, targetRanked.length));
  let best:
    | {
      score: number;
      sourcePoint: KitDiagramSnapPoint;
      targetPoint: KitDiagramSnapPoint;
    }
    | undefined;

  for (const sourceCandidate of sourceCandidates) {
    for (const targetCandidate of targetCandidates) {
      const sourceAbsolute = kitDiagramToAbsolutePoint(sourceNode.position, sourceCandidate.point);
      const targetAbsolute = kitDiagramToAbsolutePoint(targetNode.position, targetCandidate.point);
      const distanceScore = kitDiagramDistanceSquared(sourceAbsolute, targetAbsolute);
      const alignmentScore = sourceCandidate.alignment + targetCandidate.alignment;
      const score = distanceScore - alignmentScore * (sourceFrame.width + targetFrame.width) * 24;
      if (!best || score < best.score) {
        best = {
          score,
          sourcePoint: sourceCandidate.point,
          targetPoint: targetCandidate.point,
        };
      }
    }
  }

  const sourcePoint = best?.sourcePoint ?? sourceStrategy.resolveNearestPoint(direction, sourceFrame);
  const targetPoint = best?.targetPoint ?? targetStrategy.resolveNearestPoint(reverseDirection, targetFrame);

  return {
    source: {
      strategyId: sourceStrategy.id,
      frame: sourceFrame,
      localPoint: sourcePoint,
      absolutePoint: kitDiagramToAbsolutePoint(sourceNode.position, sourcePoint),
      center: sourceCenterAbsolute,
    },
    target: {
      strategyId: targetStrategy.id,
      frame: targetFrame,
      localPoint: targetPoint,
      absolutePoint: kitDiagramToAbsolutePoint(targetNode.position, targetPoint),
      center: targetCenterAbsolute,
    },
  };
};

export const resolveKitDiagramProximityAnchor = (nodeId: string, node: KitDiagramNodeGeometryInput, targetPoint: KitDiagramPoint): KitDiagramProximityAnchor => {
  const strategy = getKitDiagramShapeStrategy(node.kind);
  const frame = normalizeKitDiagramFrame(node.frame, strategy.frame);
  const points = strategy.getSnapPoints(frame);
  const bestPoint = points.reduce(
    (best, point) => {
      const absolutePoint = kitDiagramToAbsolutePoint(node.position, point);
      const distance = Math.sqrt(kitDiagramDistanceSquared(absolutePoint, targetPoint));
      if (!best || distance < best.distance) {
        return { point, absolutePoint, distance };
      }
      return best;
    },
    null as null | { point: KitDiagramSnapPoint; absolutePoint: KitDiagramPoint; distance: number },
  );
  const resolvedPoint = bestPoint?.point ?? strategy.resolveNearestPoint(kitDiagramVector(kitDiagramCenter(frame, strategy.frame), targetPoint), frame);
  const resolvedAbsolutePoint = bestPoint?.absolutePoint ?? kitDiagramToAbsolutePoint(node.position, resolvedPoint);
  return {
    nodeId,
    distance: bestPoint?.distance ?? Math.sqrt(kitDiagramDistanceSquared(resolvedAbsolutePoint, targetPoint)),
    anchor: {
      strategyId: strategy.id,
      frame,
      localPoint: resolvedPoint,
      absolutePoint: resolvedAbsolutePoint,
      center: kitDiagramToAbsolutePoint(node.position, kitDiagramCenter(frame, strategy.frame)),
    },
  };
};

// #endregion Kit Diagram Geometry
