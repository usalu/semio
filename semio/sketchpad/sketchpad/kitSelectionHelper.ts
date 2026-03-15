// #region 🔖Header
// [👤semio📚js🗃️sketchpad💻kitselectionhelper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts)

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

// Geometry and selection utilities for kit diagram interactions.

// #endregion 🔖Header

// #region Imports
// [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Imports)
// Imports MUST include icon width constant and kit selection types.

import { ICON_WIDTH } from "@semio/js/semio";
import type { KitAppSelection } from "./Kit";

// #endregion Imports

// #region Types
// [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖types](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Types)
// Types MUST define selection value extraction for KitAppSelection dimensions.

/**
 * Extracts the element type from an array-valued KitAppSelection dimension.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖types🛠️selectionvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Types/d/i/SelectionValue)
 **/
export type SelectionValue<K extends keyof KitAppSelection> = NonNullable<KitAppSelection[K]> extends (infer T)[] ? T : never;

// #endregion Types

// #region Generic Utilities
// [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities)
// Generic Utilities MUST provide immutable selection manipulation functions.

/**
 * Adds a value to the specified selection dimension array.
 * MUST return the original selection if the value is already present.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️addtoselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/addToSelection)
 **/
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
 * Removes a value from the specified selection dimension array.
 * MUST remove the dimension key entirely when the array becomes empty.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️removefromselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/removeFromSelection)
 **/
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
 * Toggles a value in the specified selection dimension array.
 * MUST add the value if absent or remove it if present.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️toggleinselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/toggleInSelection)
 **/
export function toggleInSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];

  if (currentArray.includes(value)) {
    return removeFromSelection(selection, key, value);
  } else {
    return addToSelection(selection, key, value);
  }
}

/**
 * Replaces an entire selection dimension with the given values.
 * MUST remove the dimension key when values are undefined or empty.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️replaceselectiondimension](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/replaceSelectionDimension)
 **/
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
 * Removes an entire dimension from the selection.
 * MUST return a new selection object without the specified key.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️clearselectiondimension](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/clearSelectionDimension)
 **/
export function clearSelectionDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K): KitAppSelection {
  const { [key]: _, ...rest } = selection;
  return rest;
}

/**
 * Returns an empty selection with all dimensions cleared.
 * MUST return a new empty KitAppSelection object.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️clearselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/clearSelection)
 **/
export function clearSelection(): KitAppSelection {
  return {};
}

/**
 * Replaces a selection dimension with all available values.
 * MUST delegate to replaceSelectionDimension with the full value list.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️selectallindimension](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/selectAllInDimension)
 **/
export function selectAllInDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, allValues: SelectionValue<K>[]): KitAppSelection {
  return replaceSelectionDimension(selection, key, allValues as KitAppSelection[K]);
}

/**
 * Checks whether a value is present in the specified selection dimension.
 * MUST return false when the dimension is undefined or empty.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖genericutilities🛠️isselected](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Generic%20Utilities/d/i/isSelected)
 **/
export function isSelected<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): boolean {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  return currentArray.includes(value);
}

// #endregion Generic Utilities

// #region Kit Diagram Geometry
// [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry)
// Kit Diagram Geometry MUST provide geometry primitives, shape strategies, and anchor resolution.

/**
 * Union of diagram node kind identifiers mapped to shape strategies.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramnodekind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramNodeKind)
 **/
export type KitDiagramNodeKind = "type" | "design" | "quality" | "port" | "tag" | "concept" | "file" | "folder" | "author";
/**
 * Union of supported diagram shape identifiers.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramshapeid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramShapeId)
 **/
export type KitDiagramShapeId = "circle" | "rectangle" | "triangle" | "long-rectangle";
/**
 * Union of cardinal snap sides for anchor point placement.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramsnapside](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramSnapSide)
 **/
export type KitDiagramSnapSide = "top" | "right" | "bottom" | "left";

/**
 * Width and height dimensions of a diagram node frame.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramframe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramFrame)
 **/
export interface KitDiagramFrame {
  width: number;
  height: number;
}

/**
 * Two-dimensional coordinate point in diagram space.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagrampoint](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramPoint)
 **/
export interface KitDiagramPoint {
  x: number;
  y: number;
}

/**
 * Named snap point on a shape boundary with directional side.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramsnappoint](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramSnapPoint)
 **/
export interface KitDiagramSnapPoint extends KitDiagramPoint {
  id: string;
  side: KitDiagramSnapSide;
}

/**
 * Optional CSS class and style overrides for shape rendering.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramshaperenderpayload](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramShapeRenderPayload)
 **/
export interface KitDiagramShapeRenderPayload {
  className?: string;
  style?: Record<string, string | number>;
}

/**
 * Shape strategy providing frame, snap points, and nearest-point resolution.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramshapestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramShapeStrategy)
 **/
export interface KitDiagramShapeStrategy {
  id: KitDiagramShapeId;
  frame: KitDiagramFrame;
  getRenderPayload: () => KitDiagramShapeRenderPayload;
  getSnapPoints: (frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint[];
  resolveNearestPoint: (targetVector: KitDiagramPoint, frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint;
}

/**
 * Fully resolved anchor with local and absolute positions on a shape.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramresolvedanchor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramResolvedAnchor)
 **/
export interface KitDiagramResolvedAnchor {
  strategyId: KitDiagramShapeId;
  frame: KitDiagramFrame;
  localPoint: KitDiagramSnapPoint;
  absolutePoint: KitDiagramPoint;
  center: KitDiagramPoint;
}

/**
 * Input parameters for computing diagram node geometry.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramnodegeometryinput](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramNodeGeometryInput)
 **/
export interface KitDiagramNodeGeometryInput {
  kind: KitDiagramNodeKind;
  position: KitDiagramPoint;
  frame?: Partial<KitDiagramFrame>;
}

/**
 * Pair of resolved anchors for source and target endpoints of a connection.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramresolvedanchorpair](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramResolvedAnchorPair)
 **/
export interface KitDiagramResolvedAnchorPair {
  source: KitDiagramResolvedAnchor;
  target: KitDiagramResolvedAnchor;
}

/**
 * Proximity-based anchor result with distance from a target point.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🛠️kitdiagramproximityanchor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KitDiagramProximityAnchor)
 **/
export interface KitDiagramProximityAnchor {
  nodeId: string;
  distance: number;
  anchor: KitDiagramResolvedAnchor;
}

/**
 * Scale multiplier applied to icon width for diagram node sizing.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramnodescale](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_NODE_SCALE)
 **/
export const KIT_DIAGRAM_NODE_SCALE = 2;
/**
 * Base pixel size for diagram nodes derived from icon width and scale.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagrambasesize](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_BASE_SIZE)
 **/
export const KIT_DIAGRAM_BASE_SIZE = ICON_WIDTH * KIT_DIAGRAM_NODE_SCALE;
/**
 * Default frame dimensions for circle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramcircleframe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_CIRCLE_FRAME)
 **/
export const KIT_DIAGRAM_CIRCLE_FRAME: KitDiagramFrame = { width: KIT_DIAGRAM_BASE_SIZE, height: KIT_DIAGRAM_BASE_SIZE };
/**
 * Default frame dimensions for rectangle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramrectangleframe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_RECTANGLE_FRAME)
 **/
export const KIT_DIAGRAM_RECTANGLE_FRAME: KitDiagramFrame = { width: Math.round(KIT_DIAGRAM_BASE_SIZE * 1.2), height: Math.round(KIT_DIAGRAM_BASE_SIZE * 0.8) };
/**
 * Default frame dimensions for triangle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramtriangleframe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_TRIANGLE_FRAME)
 **/
export const KIT_DIAGRAM_TRIANGLE_FRAME: KitDiagramFrame = { width: KIT_DIAGRAM_BASE_SIZE, height: KIT_DIAGRAM_BASE_SIZE };
/**
 * Default frame dimensions for long-rectangle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramlongrectangleframe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_LONG_RECTANGLE_FRAME)
 **/
export const KIT_DIAGRAM_LONG_RECTANGLE_FRAME: KitDiagramFrame = { width: Math.round(KIT_DIAGRAM_BASE_SIZE * 1.6), height: Math.round(KIT_DIAGRAM_BASE_SIZE * 0.72) };
/**
 * Half of the largest frame dimension used as collision radius for force layout.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramcollideradius](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_COLLIDE_RADIUS)
 **/
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

/**
 * Validates and normalizes a partial frame to a complete frame with positive dimensions.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨normalizekitdiagramframe](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/normalizeKitDiagramFrame)
 **/
export const normalizeKitDiagramFrame = (frame?: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramFrame => {
  const width = frame?.width ?? fallback.width;
  const height = frame?.height ?? fallback.height;
  return {
    width: Number.isFinite(width) && width > 0 ? width : fallback.width,
    height: Number.isFinite(height) && height > 0 ? height : fallback.height,
  };
};

/**
 * Computes the center point of a diagram frame.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramcenter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramCenter)
 **/
export const kitDiagramCenter = (frame: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramPoint => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  return { x: normalizedFrame.width / 2, y: normalizedFrame.height / 2 };
};

/**
 * Computes the direction vector from one point to another.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramvector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramVector)
 **/
export const kitDiagramVector = (from: KitDiagramPoint, to: KitDiagramPoint): KitDiagramPoint => ({ x: to.x - from.x, y: to.y - from.y });
/**
 * Computes the Euclidean length of a vector.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramvectorlength](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramVectorLength)
 **/
export const kitDiagramVectorLength = (vector: KitDiagramPoint): number => Math.hypot(vector.x, vector.y);
/**
 * Returns a unit-length vector in the same direction or zero vector if length is zero.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramnormalizevector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramNormalizeVector)
 **/
export const kitDiagramNormalizeVector = (vector: KitDiagramPoint): KitDiagramPoint => {
  const length = kitDiagramVectorLength(vector);
  if (length === 0) return { x: 0, y: 0 };
  return { x: vector.x / length, y: vector.y / length };
};
/**
 * Computes the dot product of two vectors.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramdot](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramDot)
 **/
export const kitDiagramDot = (a: KitDiagramPoint, b: KitDiagramPoint): number => a.x * b.x + a.y * b.y;
/**
 * Computes the squared Euclidean distance between two points.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramdistancesquared](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramDistanceSquared)
 **/
export const kitDiagramDistanceSquared = (a: KitDiagramPoint, b: KitDiagramPoint): number => {
  const dx = a.x - b.x;
  const dy = a.y - b.y;
  return dx * dx + dy * dy;
};
/**
 * Translates a local point to absolute coordinates by adding an origin offset.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramtoabsolutepoint](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramToAbsolutePoint)
 **/
export const kitDiagramToAbsolutePoint = (origin: KitDiagramPoint, localPoint: KitDiagramPoint): KitDiagramPoint => ({
  x: origin.x + localPoint.x,
  y: origin.y + localPoint.y,
});
/**
 * Infers the cardinal snap side of a point relative to the frame center.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagraminfersnapside](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramInferSnapSide)
 **/
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

/**
* [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨createcirclesnappoints](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/createCircleSnapPoints)
* createCircleSnapPoints holds the data fields for a createCircleSnapPoints record.
**/
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

/**
* [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨createrectanglesnappoints](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/createRectangleSnapPoints)
* createRectangleSnapPoints holds the data fields for a createRectangleSnapPoints record.
**/
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

/**
* [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨createtrianglesnappoints](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/createTriangleSnapPoints)
* createTriangleSnapPoints holds the data fields for a createTriangleSnapPoints record.
**/
const createTriangleSnapPoints = (frame?: Partial<KitDiagramFrame>): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, KIT_DIAGRAM_TRIANGLE_FRAME);
  return [
    { id: "apex", x: normalizedFrame.width / 2, y: 0, side: "top" },
    { id: "base-left", x: 0, y: normalizedFrame.height, side: "left" },
    { id: "base-right", x: normalizedFrame.width, y: normalizedFrame.height, side: "right" },
  ];
};

/** rankSnapPointsByVector holds the data fields for a rankSnapPointsByVector record.
// [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨ranksnappointsbyvector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/rankSnapPointsByVector)
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨ranksnappointsbyvector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/rankSnapPointsByVector)
 **/
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

/**
 * Selects the snap point best aligned with a target vector direction.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨resolvenearestkitdiagramsnappoint](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/resolveNearestKitDiagramSnapPoint)
 **/
export const resolveNearestKitDiagramSnapPoint = (points: KitDiagramSnapPoint[], frame: Partial<KitDiagramFrame>, targetVector: KitDiagramPoint, fallback: KitDiagramFrame): KitDiagramSnapPoint => {
  if (points.length === 0) {
    const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
    const center = kitDiagramCenter(normalizedFrame, fallback);
    return { id: "center", ...center, side: kitDiagramInferSnapSide(center, normalizedFrame, fallback) };
  }
  const ranked = rankSnapPointsByVector(points, frame, targetVector, fallback);
  return ranked[0]?.point ?? points[0];
};

/**
* [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨createstrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/createStrategy)
* createStrategy holds the data fields for a createStrategy record.
**/
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

/**
 * Shape strategy for circle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramcirclestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramCircleStrategy)
 **/
export const kitDiagramCircleStrategy = createStrategy("circle", KIT_DIAGRAM_CIRCLE_FRAME, createCircleSnapPoints, {});
/**
 * Shape strategy for rectangle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramrectanglestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramRectangleStrategy)
 **/
export const kitDiagramRectangleStrategy = createStrategy("rectangle", KIT_DIAGRAM_RECTANGLE_FRAME, createRectangleSnapPoints, {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
});
/**
 * Shape strategy for triangle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramtrianglestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramTriangleStrategy)
 **/
export const kitDiagramTriangleStrategy = createStrategy("triangle", KIT_DIAGRAM_TRIANGLE_FRAME, createTriangleSnapPoints, {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
  style: { clipPath: "polygon(50% 0%, 0% 100%, 100% 100%)" },
});
/**
 * Shape strategy for long-rectangle-shaped diagram nodes.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramlongrectanglestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/kitDiagramLongRectangleStrategy)
 **/
export const kitDiagramLongRectangleStrategy = createStrategy("long-rectangle", KIT_DIAGRAM_LONG_RECTANGLE_FRAME, (frame) => createRectangleSnapPoints(frame, KIT_DIAGRAM_LONG_RECTANGLE_FRAME), {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
});

/**
 * Fallback shape strategy used when no kind-specific strategy is registered.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramdefaultshapestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY)
 **/
export const KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY = kitDiagramLongRectangleStrategy;

/**
 * Registry mapping each node kind to its associated shape strategy.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨kitdiagramshapestrategyregistry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/KIT_DIAGRAM_SHAPE_STRATEGY_REGISTRY)
 **/
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

/**
 * Looks up the shape strategy for a given node kind with fallback to default.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨getkitdiagramshapestrategy](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/getKitDiagramShapeStrategy)
 **/
export const getKitDiagramShapeStrategy = (kind: KitDiagramNodeKind): KitDiagramShapeStrategy => KIT_DIAGRAM_SHAPE_STRATEGY_REGISTRY[kind] ?? KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY;

/**
 * Returns the normalized frame dimensions for a given node kind with optional override.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨getkitdiagramnodeframeforkind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/getKitDiagramNodeFrameForKind)
 **/
export const getKitDiagramNodeFrameForKind = (kind: KitDiagramNodeKind, override?: Partial<KitDiagramFrame>): KitDiagramFrame => normalizeKitDiagramFrame(override, getKitDiagramShapeStrategy(kind).frame);

/**
 * Resolves the optimal anchor pair between two diagram nodes for edge routing.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨resolvekitdiagramanchorpair](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/resolveKitDiagramAnchorPair)
 **/
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

/**
 * Finds the closest snap point on a node to a given target point for proximity-based connections.
 * [👤semio📚js🗃️sketchpad💻kitselectionhelper🔖kitdiagramgeometry🪨resolvekitdiagramproximityanchor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/kitSelectionHelper.ts/s/Kit%20Diagram%20Geometry/d/i/resolveKitDiagramProximityAnchor)
 **/
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
