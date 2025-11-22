// #region Header

// semio.ts

// 2025 Ueli Saluz

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

// #endregion

// #region TODOs

// TODOs
// TODO: Conventionalize error throwing and logging

// #endregion TODOs

import { default as adjectives } from "@semio/assets/lists/adjectives.json";
import { default as animals } from "@semio/assets/lists/animals.json";
import { ClassValue, clsx } from "clsx";
import cytoscape from "cytoscape";
import { twMerge } from "tailwind-merge";
import * as THREE from "three";
import { v7 as uuidv7 } from "uuid";
import { z } from "zod";
import CONSTANTS from "./constants.json";

// #region Constants

export const ICON_WIDTH = CONSTANTS.icon.width;
export const TOLERANCE = CONSTANTS.tolerance;

// #endregion Constants

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const guid = () => uuidv7();

class SeededRandom {
  private seed: number;
  constructor(seed: number) {
    this.seed = seed % 2147483647;
    if (this.seed <= 0) this.seed += 2147483646;
  }
  next = (): number => (this.seed = (this.seed * 16807) % 2147483647);
  nextFloat = (): number => (this.next() - 1) / 2147483646;
  nextInt = (max: number): number => Math.floor(this.nextFloat() * max);
}

export class Generator {
  public static randomId(seed: number = Math.floor(Math.random() * 1000000)): string {
    const random = new SeededRandom(seed);
    let adjective = adjectives[random.nextInt(adjectives.length)];
    let animal = animals[random.nextInt(animals.length)];
    adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1);
    animal = animal.charAt(0).toUpperCase() + animal.slice(1);
    return `${adjective}${animal}${random.nextInt(1000)}`;
  }
  public static randomName(seed: number = Math.floor(Math.random() * 1000000)): string {
    const random = new SeededRandom(seed);
    let animal = animals[random.nextInt(animals.length)];
    animal = animal.charAt(0).toUpperCase() + animal.slice(1);
    return `${animal}`;
  }
}

export const normalize = (val: string | undefined | null): string => (val === undefined || val === null ? "" : val);
export const round = (value: number): number => Math.round(value / TOLERANCE) * TOLERANCE;
export const jaccard = (a: string[] | undefined, b: string[] | undefined): number => {
  if ((a === undefined && b === undefined) || (a?.length === 0 && b?.length === 0)) return 1;
  if (a === undefined || b === undefined) return 0;
  const setA = new Set(a);
  const setB = new Set(b);
  const intersection = Array.from(setA).filter((x) => setB.has(x)).length;
  const union = setA.size + setB.size - intersection;
  if (union === 0) return 0;
  return intersection / union;
};

export const deepEqual = (a: any, b: any): boolean => {
  if (a === b) return true;
  if (a == null || b == null) return a === b;
  if (typeof a !== typeof b) return false;

  if (Array.isArray(a)) {
    if (!Array.isArray(b) || a.length !== b.length) return false;
    return a.every((item, index) => deepEqual(item, b[index]));
  }

  if (typeof a === "object") {
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;
    return keysA.every((key) => keysB.includes(key) && deepEqual(a[key], b[key]));
  }

  return false;
};

export const arraysEqual = <T>(a: T[] | undefined, b: T[] | undefined): boolean => {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.length === b.length && a.every((val, index) => deepEqual(val, b[index]));
};

export const generateUniqueName = (baseName: string, existingNames: string[], separator: string = " "): string => {
  if (!existingNames.includes(baseName)) return baseName;
  let counter = 2;
  while (existingNames.includes(`${baseName}${separator}${counter}`)) {
    counter++;
  }
  return `${baseName}${separator}${counter}`;
};

export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}

// Coordinate system conversion between Semio and Three.js
// Semio: X-right, Y-forward, Z-up
// Three.js: X-right, Y-up, Z-backward
// Desired mapping: semioX->threeX, semioY->-threeZ, semioZ->threeY
// Matrix columns represent where basis vectors go:
// [1,0,0,0,  0,0,1,0,  0,-1,0,0,  0,0,0,1]
export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);
// Inverse: threeX->semioX, threeY->semioZ, threeZ->-semioY
// [1,0,0,0,  0,0,-1,0,  0,1,0,0,  0,0,0,1]
export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476);
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476);
export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z);

export type Guid = string;

// #region Entity IDs

export type AttributeId = { guid: Guid };
export type LocationId = { guid: Guid };
export type AuthorId = { guid: Guid };
export type FileId = { guid: Guid };
export type FolderId = { guid: Guid };
export type BenchmarkId = { guid: Guid };
export type QualityId = { guid: Guid };
export type InterfaceId = { guid: Guid };
export type PropId = { guid: Guid };
export type ModelId = { guid: Guid };
export type PortId = { guid: Guid };
export type TypeId = { guid: Guid };
export type LayerId = { guid: Guid };
export type PieceId = { guid: Guid };
export type GroupId = { guid: Guid };
export type ConnectionId = { guid: Guid };
export type StatId = { guid: Guid };
export type DesignId = { guid: Guid };
export type KitId = { guid: Guid };

export const AttributeIdSchema = z.object({ guid: z.string() });
export const LocationIdSchema = z.object({ guid: z.string() });
export const AuthorIdSchema = z.object({ guid: z.string() });
export const FileIdSchema = z.object({ guid: z.string() });
export const FolderIdSchema = z.object({ guid: z.string() });
export const BenchmarkIdSchema = z.object({ guid: z.string() });
export const QualityIdSchema = z.object({ guid: z.string() });
export const InterfaceIdSchema = z.object({ guid: z.string() });
export const PropIdSchema = z.object({ guid: z.string() });
export const ModelIdSchema = z.object({ guid: z.string() });
export const PortIdSchema = z.object({ guid: z.string() });
export const TypeIdSchema = z.object({ guid: z.string() });
export const LayerIdSchema = z.object({ guid: z.string() });
export const PieceIdSchema = z.object({ guid: z.string() });
export const GroupIdSchema = z.object({ guid: z.string() });
export const ConnectionIdSchema = z.object({ guid: z.string() });
export const StatIdSchema = z.object({ guid: z.string() });
export const DesignIdSchema = z.object({ guid: z.string() });
export const KitIdSchema = z.object({ guid: z.string() });

export const createAttributeId = (guid: Guid): AttributeId => ({ guid });
export const createLocationId = (guid: Guid): LocationId => ({ guid });
export const createAuthorId = (guid: Guid): AuthorId => ({ guid });
export const createFileId = (guid: Guid): FileId => ({ guid });
export const createFolderId = (guid: Guid): FolderId => ({ guid });
export const createBenchmarkId = (guid: Guid): BenchmarkId => ({ guid });
export const createQualityId = (guid: Guid): QualityId => ({ guid });
export const createInterfaceId = (guid: Guid): InterfaceId => ({ guid });
export const createPropId = (guid: Guid): PropId => ({ guid });
export const createModelId = (guid: Guid): ModelId => ({ guid });
export const createPortId = (guid: Guid): PortId => ({ guid });
export const createTypeId = (guid: Guid): TypeId => ({ guid });
export const createLayerId = (guid: Guid): LayerId => ({ guid });
export const createPieceId = (guid: Guid): PieceId => ({ guid });
export const createGroupId = (guid: Guid): GroupId => ({ guid });
export const createConnectionId = (guid: Guid): ConnectionId => ({ guid });
export const createStatId = (guid: Guid): StatId => ({ guid });
export const createDesignId = (guid: Guid): DesignId => ({ guid });
export const createKitId = (guid: Guid): KitId => ({ guid });

export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.guid === b.guid;
export const areSameLocationId = (a: LocationId, b: LocationId): boolean => a.guid === b.guid;
export const areSameAuthorId = (a: AuthorId, b: AuthorId): boolean => a.guid === b.guid;
export const areSameFileId = (a: FileId, b: FileId): boolean => a.guid === b.guid;
export const areSameFolderId = (a: FolderId, b: FolderId): boolean => a.guid === b.guid;
export const areSameBenchmarkId = (a: BenchmarkId, b: BenchmarkId): boolean => a.guid === b.guid;
export const areSameQualityId = (a: QualityId, b: QualityId): boolean => a.guid === b.guid;
export const areSameInterfaceId = (a: InterfaceId, b: InterfaceId): boolean => a.guid === b.guid;
export const areSamePropId = (a: PropId, b: PropId): boolean => a.guid === b.guid;
export const areSameModelId = (a: ModelId, b: ModelId): boolean => a.guid === b.guid;
export const areSamePortId = (a: PortId, b: PortId): boolean => a.guid === b.guid;
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.guid === b.guid;
export const areSameLayerId = (a: LayerId, b: LayerId): boolean => a.guid === b.guid;
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
export const areSameGroupId = (a: GroupId, b: GroupId): boolean => a.guid === b.guid;
export const areSameConnectionId = (a: ConnectionId, b: ConnectionId): boolean => a.guid === b.guid;
export const areSameStatId = (a: StatId, b: StatId): boolean => a.guid === b.guid;
export const areSameDesignId = (a: DesignId, b: DesignId): boolean => a.guid === b.guid;
export const areSameKitId = (a: KitId, b: KitId): boolean => a.guid === b.guid;

export const getAttributeGuid = (id: AttributeId): Guid => id.guid;
export const getLocationGuid = (id: LocationId): Guid => id.guid;
export const getAuthorGuid = (id: AuthorId): Guid => id.guid;
export const getFileGuid = (id: FileId): Guid => id.guid;
export const getFolderGuid = (id: FolderId): Guid => id.guid;
export const getBenchmarkGuid = (id: BenchmarkId): Guid => id.guid;
export const getQualityGuid = (id: QualityId): Guid => id.guid;
export const getInterfaceGuid = (id: InterfaceId): Guid => id.guid;
export const getPropGuid = (id: PropId): Guid => id.guid;
export const getModelGuid = (id: ModelId): Guid => id.guid;
export const getPortGuid = (id: PortId): Guid => id.guid;
export const getTypeGuid = (id: TypeId): Guid => id.guid;
export const getLayerGuid = (id: LayerId): Guid => id.guid;
export const getPieceGuid = (id: PieceId): Guid => id.guid;
export const getGroupGuid = (id: GroupId): Guid => id.guid;
export const getConnectionGuid = (id: ConnectionId): Guid => id.guid;
export const getStatGuid = (id: StatId): Guid => id.guid;
export const getDesignGuid = (id: DesignId): Guid => id.guid;
export const getKitGuid = (id: KitId): Guid => id.guid;

// #endregion Entity IDs

const DateProperty = () =>
  z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional();

// #region Attribute
// https://github.com/usalu/semio#-attribute-

export const AttributeSchema = z.object({
  guid: z.string(),
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
export type Attribute = z.infer<typeof AttributeSchema>;
export const serializeAttribute = (attribute: Attribute): string => JSON.stringify(AttributeSchema.parse(attribute));
export const deserializeAttribute = (json: string): Attribute => AttributeSchema.parse(JSON.parse(json));

export const AttributeDiffSchema = AttributeSchema.partial();
export type AttributeDiff = z.infer<typeof AttributeDiffSchema>;
export const getAttributeDiff = (before: Attribute, after: Attribute): AttributeDiff => {
  return { ...after };
};
export const inverseAttributeDiff = (original: Attribute, appliedDiff: AttributeDiff): AttributeDiff => {
  return {
    key: appliedDiff.key ? original.key : "",
    value: appliedDiff.value ? original.value : "",
    definition: appliedDiff.definition ? original.definition : "",
  };
};
export const mergeAttributeDiff = (diff1: AttributeDiff, diff2: AttributeDiff): AttributeDiff => {
  return {
    key: diff2.key ?? diff1.key,
    value: diff2.value ?? diff1.value,
    definition: diff2.definition ?? diff1.definition,
  };
};
export const applyAttributeDiff = (base: Attribute, diff: AttributeDiff): Attribute => {
  return { ...base, ...diff };
};

export const AttributesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: AttributeDiffSchema })).optional(),
  added: z.array(AttributeSchema).optional(),
});
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;

const getAttributesDiff = (before: Attribute[], after: Attribute[]): AttributesDiff => {
  const beforeKeys = before.map((a) => a.key);
  const afterKeys = after.map((a) => a.key);
  const removed = beforeKeys.filter((key) => !afterKeys.includes(key));
  const added = after.filter((a) => !beforeKeys.includes(a.key));
  const updated = after.filter((a) => beforeKeys.includes(a.key)).map((a) => ({ id: a.key, diff: getAttributeDiff(before.find((b) => b.key === a.key)!, a) }));
  const diff: AttributesDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

export const inverseAttributesDiff = (original: Attribute[], appliedDiff: AttributesDiff): AttributesDiff => {
  const removedKeys = appliedDiff.removed ?? [];
  const updatedKeys = appliedDiff.updated?.map((a) => a.id) ?? [];
  const addedKeys = appliedDiff.added?.map((a) => a.key) ?? [];
  return {
    removed: addedKeys,
    updated: updatedKeys
      .map((key) => {
        const orig = original.find((a) => a.key === key);
        const upd = appliedDiff.updated?.find((a) => a.id === key);
        if (!orig || !upd) return null;
        return { id: key, diff: inverseAttributeDiff(orig, upd.diff) };
      })
      .filter((item): item is { id: string; diff: AttributeDiff } => item !== null),
    added: removedKeys.map((key) => original.find((a) => a.key === key)!).filter((a) => a !== undefined),
  };
};

export const mergeAttributesDiff = (first: AttributesDiff, second: AttributesDiff): AttributesDiff => {
  return { ...first, ...second };
};

export const applyAttributesDiff = (base: Attribute[], diff: AttributesDiff): Attribute[] => {
  let result = [...base];
  if (diff.removed) {
    result = result.filter((attr) => !diff.removed!.includes(attr.key));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((attr) => attr.key === update.id);
      if (index !== -1) {
        result[index] = applyAttributeDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

// #endregion Attribute

// #region Coord (weak entity)
// https://github.com/usalu/semio#-coord-

export const CoordSchema = z.object({ u: z.number(), v: z.number() });
export type Coord = z.infer<typeof CoordSchema>;
export const serializeCoord = (coord: Coord): string => JSON.stringify(CoordSchema.parse(coord));
export const deserializeCoord = (json: string): Coord => CoordSchema.parse(JSON.parse(json));

export const CoordDiffSchema = CoordSchema.partial();
export type CoordDiff = z.infer<typeof CoordDiffSchema>;
export const getCoordDiff = (before: Coord, after: Coord): CoordDiff => {
  return {
    u: after.u - before.u,
    v: after.v - before.v,
  };
};
export const inverseCoordDiff = (original: Coord, appliedDiff: CoordDiff): CoordDiff => {
  const u = appliedDiff.u ?? 0;
  const v = appliedDiff.v ?? 0;
  return {
    u: original.u - u,
    v: original.v - v,
  };
};
export const mergeCoordDiff = (diff1: CoordDiff, diff2: CoordDiff): CoordDiff => {
  return {
    u: (diff1.u ?? 0) + (diff2.u ?? 0),
    v: (diff1.v ?? 0) + (diff2.v ?? 0),
  };
};
export const applyCoordDiff = (base: Coord, diff: CoordDiff): Coord => {
  const u = diff.u ?? 0;
  const v = diff.v ?? 0;
  return {
    u: base.u + u,
    v: base.v + v,
  };
};

// #endregion Coord

// #region Vec (weak entity)
// https://github.com/usalu/semio#-vec-

export const VecSchema = z.object({ x: z.number(), y: z.number() });
export type Vec = z.infer<typeof VecSchema>;
export const serializeVec = (vec: Vec): string => JSON.stringify(VecSchema.parse(vec));
export const deserializeVec = (json: string): Vec => VecSchema.parse(JSON.parse(json));

export const VecDiffSchema = VecSchema.partial();
export type VecDiff = z.infer<typeof VecDiffSchema>;
export const getVecDiff = (before: Vec, after: Vec): VecDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
  };
};
export const inverseVecDiff = (original: Vec, appliedDiff: VecDiff): VecDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  return {
    x: original.x - x,
    y: original.y - y,
  };
};
export const mergeVecDiff = (diff1: VecDiff, diff2: VecDiff): VecDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
  };
};
export const applyVecDiff = (base: Vec, diff: VecDiff): Vec => {
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  return {
    x: base.x + x,
    y: base.y + y,
  };
};

// #endregion Vec

// #region Point (weak entity)
// https://github.com/usalu/semio#-point-

export const PointSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
export type Point = z.infer<typeof PointSchema>;
export const serializePoint = (point: Point): string => JSON.stringify(PointSchema.parse(point));
export const deserializePoint = (json: string): Point => PointSchema.parse(JSON.parse(json));

export const PointDiffSchema = PointSchema.partial();
export type PointDiff = z.infer<typeof PointDiffSchema>;
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};
export const inversePointDiff = (original: Point, appliedDiff: PointDiff): PointDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  const z = appliedDiff.z ?? 0;
  return {
    x: original.x - x,
    y: original.y - y,
    z: original.z - z,
  };
};
export const mergePointDiff = (diff1: PointDiff, diff2: PointDiff): PointDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
    z: (diff1.z ?? 0) + (diff2.z ?? 0),
  };
};
export const applyPointDiff = (base: Point, diff: PointDiff): Point => {
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  const z = diff.z ?? 0;
  return {
    x: base.x + x,
    y: base.y + y,
    z: base.z + z,
  };
};

// #endregion Point

// #region Vector (weak entity)
// https://github.com/usalu/semio#-vector-

export const VectorSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
export type Vector = z.infer<typeof VectorSchema>;
export const serializeVector = (vector: Vector): string => JSON.stringify(VectorSchema.parse(vector));
export const deserializeVector = (json: string): Vector => VectorSchema.parse(JSON.parse(json));

export const VectorDiffSchema = VectorSchema.partial();
export type VectorDiff = z.infer<typeof VectorDiffSchema>;
export const getVectorDiff = (before: Vector, after: Vector): VectorDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};
export const inverseVectorDiff = (original: Vector, appliedDiff: VectorDiff): VectorDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  const z = appliedDiff.z ?? 0;
  return {
    x: original.x - x,
    y: original.y - y,
    z: original.z - z,
  };
};
export const mergeVectorDiff = (diff1: VectorDiff, diff2: VectorDiff): VectorDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
    z: (diff1.z ?? 0) + (diff2.z ?? 0),
  };
};
export const applyVectorDiff = (base: Vector, diff: VectorDiff): Vector => {
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  const z = diff.z ?? 0;
  return {
    x: base.x + x,
    y: base.y + y,
    z: base.z + z,
  };
};

// #endregion Vector

// #region Plane (weak entity)

// https://github.com/usalu/semio#-plane-
export const PlaneSchema = z.object({
  origin: PointSchema,
  xAxis: VectorSchema,
  yAxis: VectorSchema,
});
export type Plane = z.infer<typeof PlaneSchema>;
export const serializePlane = (plane: Plane): string => JSON.stringify(PlaneSchema.parse(plane));
export const deserializePlane = (json: string): Plane => PlaneSchema.parse(JSON.parse(json));
export const planeToMatrix = (plane: Plane): THREE.Matrix4 => {
  const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
  const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z);
  const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
  const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis).normalize();
  const orthoYAxis = new THREE.Vector3().crossVectors(zAxis, xAxis).normalize();
  const matrix = new THREE.Matrix4().makeBasis(xAxis.normalize(), orthoYAxis, zAxis).setPosition(origin);
  return matrix;
};
export const matrixToPlane = (matrix: THREE.Matrix4): Plane => {
  const origin = new THREE.Vector3();
  const xAxis = new THREE.Vector3();
  const yAxis = new THREE.Vector3();
  const zAxis = new THREE.Vector3();
  matrix.decompose(origin, new THREE.Quaternion(), new THREE.Vector3());
  matrix.extractBasis(xAxis, yAxis, zAxis);
  return {
    origin: { x: origin.x, y: origin.y, z: origin.z },
    xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
    yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
  };
};

/**
 * Calculate the average plane from multiple planes.
 * This is useful for multi-selection transforms where we need a single reference plane.
 */
export const averagePlane = (planes: Plane[]): Plane | null => {
  if (planes.length === 0) return null;
  if (planes.length === 1) return planes[0];

  // Average the origins
  const avgOrigin = planes.reduce(
    (acc, plane) => ({
      x: acc.x + plane.origin.x / planes.length,
      y: acc.y + plane.origin.y / planes.length,
      z: acc.z + plane.origin.z / planes.length,
    }),
    { x: 0, y: 0, z: 0 },
  );

  // For orientation, use the first plane's axes as the base
  // This is a simplification - a proper implementation might use quaternion averaging
  const baseXAxis = planes[0].xAxis;
  const baseYAxis = planes[0].yAxis;

  return {
    origin: avgOrigin,
    xAxis: baseXAxis,
    yAxis: baseYAxis,
  };
};
const roundPlane = (plane: Plane): Plane => ({
  origin: {
    x: round(plane.origin.x),
    y: round(plane.origin.y),
    z: round(plane.origin.z),
  },
  xAxis: {
    x: round(plane.xAxis.x),
    y: round(plane.xAxis.y),
    z: round(plane.xAxis.z),
  },
  yAxis: {
    x: round(plane.yAxis.x),
    y: round(plane.yAxis.y),
    z: round(plane.yAxis.z),
  },
});

export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true })
  .extend({
    origin: PointDiffSchema,
    xAxis: VectorDiffSchema,
    yAxis: VectorDiffSchema,
  })
  .partial();
export type PlaneDiff = z.infer<typeof PlaneDiffSchema>;
export const getPlaneDiff = (before: Plane, after: Plane): PlaneDiff => {
  return {
    origin: getPointDiff(before.origin, after.origin),
    xAxis: getVectorDiff(before.xAxis, after.xAxis),
    yAxis: getVectorDiff(before.yAxis, after.yAxis),
  };
};
export const inversePlaneDiff = (original: Plane, appliedDiff: PlaneDiff): PlaneDiff => {
  const origin = appliedDiff.origin ?? { x: 0, y: 0, z: 0 };
  const xAxis = appliedDiff.xAxis ?? { x: 0, y: 0, z: 0 };
  const yAxis = appliedDiff.yAxis ?? { x: 0, y: 0, z: 0 };
  return {
    origin: inversePointDiff(original.origin, origin),
    xAxis: inverseVectorDiff(original.xAxis, xAxis),
    yAxis: inverseVectorDiff(original.yAxis, yAxis),
  };
};
export const mergePlaneDiff = (diff1: PlaneDiff, diff2: PlaneDiff): PlaneDiff => {
  return {
    origin: diff1.origin ?? diff2.origin ?? mergePointDiff(diff1.origin!, diff2.origin!),
    xAxis: diff1.xAxis ?? diff2.xAxis ?? mergeVectorDiff(diff1.xAxis!, diff2.xAxis!),
    yAxis: diff1.yAxis ?? diff2.yAxis ?? mergeVectorDiff(diff1.yAxis!, diff2.yAxis!),
  };
};
export const applyPlaneDiff = (base: Plane, diff: PlaneDiff): Plane => {
  return {
    origin: diff.origin ? applyPointDiff(base.origin, diff.origin) : base.origin,
    xAxis: diff.xAxis ? applyVectorDiff(base.xAxis, diff.xAxis) : base.xAxis,
    yAxis: diff.yAxis ? applyVectorDiff(base.yAxis, diff.yAxis) : base.yAxis,
  };
};

// #endregion Plane

// #region Camera (weak entity)
// https://github.com/usalu/semio#-camera-

export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});
export type Camera = z.infer<typeof CameraSchema>;
export const serializeCamera = (camera: Camera): string => JSON.stringify(CameraSchema.parse(camera));
export const deserializeCamera = (json: string): Camera => CameraSchema.parse(JSON.parse(json));

export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true })
  .extend({
    position: PointDiffSchema,
    forward: VectorDiffSchema,
    up: VectorDiffSchema,
  })
  .partial();
export type CameraDiff = z.infer<typeof CameraDiffSchema>;
export const getCameraDiff = (before: Camera, after: Camera): CameraDiff => {
  return {
    position: getPointDiff(before.position, after.position),
    forward: getVectorDiff(before.forward, after.forward),
    up: getVectorDiff(before.up, after.up),
  };
};
export const inverseCameraDiff = (original: Camera, appliedDiff: CameraDiff): CameraDiff => {
  return {
    position: appliedDiff.position ? inversePointDiff(original.position, appliedDiff.position) : original.position,
    forward: appliedDiff.forward ? inverseVectorDiff(original.forward, appliedDiff.forward) : original.forward,
    up: appliedDiff.up ? inverseVectorDiff(original.up, appliedDiff.up) : original.up,
  };
};
export const mergeCameraDiff = (diff1: CameraDiff, diff2: CameraDiff): CameraDiff => {
  return {
    position: diff1.position ?? diff2.position ?? mergePointDiff(diff1.position!, diff2.position!),
    forward: diff1.forward ?? diff2.forward ?? mergeVectorDiff(diff1.forward!, diff2.forward!),
    up: diff1.up ?? diff2.up ?? mergeVectorDiff(diff1.up!, diff2.up!),
  };
};
export const applyCameraDiff = (base: Camera, diff: CameraDiff): Camera => {
  return {
    position: diff.position ? applyPointDiff(base.position, diff.position) : base.position,
    forward: diff.forward ? applyVectorDiff(base.forward, diff.forward) : base.forward,
    up: diff.up ? applyVectorDiff(base.up, diff.up) : base.up,
  };
};

// #endregion Camera

// #region Location
// https://github.com/usalu/semio#-location-

export const LocationSchema = z.object({
  guid: z.string(),
  longitude: z.number(),
  latitude: z.number(),
  altitude: z.number().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Location = z.infer<typeof LocationSchema>;
export const serializeLocation = (location: Location): string => JSON.stringify(LocationSchema.parse(location));
export const deserializeLocation = (json: string): Location => LocationSchema.parse(JSON.parse(json));

export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type LocationDiff = z.infer<typeof LocationDiffSchema>;
export const getLocationDiff = (before: Location, after: Location): LocationDiff => {
  const diff: LocationDiff = {};
  if (before.longitude !== after.longitude) diff.longitude = after.longitude - before.longitude;
  if (before.latitude !== after.latitude) diff.latitude = after.latitude - before.latitude;
  if (before.altitude !== after.altitude) diff.altitude = after.altitude !== undefined && before.altitude !== undefined ? after.altitude - before.altitude : after.altitude;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseLocationDiff = (original: Location, appliedDiff: LocationDiff): LocationDiff => {
  const inverse: LocationDiff = {};
  if (appliedDiff.longitude !== undefined) inverse.longitude = original.longitude;
  if (appliedDiff.latitude !== undefined) inverse.latitude = original.latitude;
  if (appliedDiff.altitude !== undefined) inverse.altitude = original.altitude;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeLocationDiff = (diff1: LocationDiff, diff2: LocationDiff): LocationDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyLocationDiff = (base: Location, diff: LocationDiff): Location => {
  return {
    ...base,
    longitude: diff.longitude ?? base.longitude,
    latitude: diff.latitude ?? base.latitude,
    altitude: diff.altitude ?? base.altitude,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

// #endregion Location

// #region Author
// https://github.com/usalu/semio#-author-

export const AuthorSchema = z.object({ guid: z.string(), name: z.string(), email: z.string(), attributes: z.array(AttributeSchema).optional() });
export type Author = z.infer<typeof AuthorSchema>;
export const serializeAuthor = (author: Author): string => JSON.stringify(AuthorSchema.parse(author));
export const deserializeAuthor = (json: string): Author => AuthorSchema.parse(JSON.parse(json));

export const AuthorDiffSchema = AuthorSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;
export const getAuthorDiff = (before: Author, after: Author): AuthorDiff => {
  const diff: AuthorDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.email !== after.email) diff.email = after.email;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseAuthorDiff = (original: Author, appliedDiff: AuthorDiff): AuthorDiff => {
  const inverse: AuthorDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.email !== undefined) inverse.email = original.email;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeAuthorDiff = (diff1: AuthorDiff, diff2: AuthorDiff): AuthorDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyAuthorDiff = (base: Author, diff: AuthorDiff): Author => {
  return {
    ...base,
    name: diff.name ?? base.name,
    email: diff.email ?? base.email,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const AuthorsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: AuthorDiffSchema })).optional(),
  added: z.array(AuthorSchema).optional(),
});
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;

// #endregion Author

// #region File
// https://github.com/usalu/semio#-file-

export const FileSchema = z.object({
  guid: z.string(),
  name: z.string(),
  remote: z.string().optional(),
  folder: FolderIdSchema.optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  createdAt: DateProperty(),
  createdBy: z.string().optional(),
  updatedAt: DateProperty(),
  updatedBy: z.string().optional(),
});
export type File = z.infer<typeof FileSchema>;
export const serializeFile = (file: File): string => JSON.stringify(FileSchema.parse(file));
export const deserializeFile = (json: string): File => FileSchema.parse(JSON.parse(json));

export const FileDiffSchema = FileSchema.partial();
export type FileDiff = z.infer<typeof FileDiffSchema>;
export const getFileDiff = (before: File, after: File): FileDiff => {
  const diff: FileDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.remote !== after.remote) diff.remote = after.remote;
  if (before.size !== after.size) diff.size = after.size;
  if (before.hash !== after.hash) diff.hash = after.hash;
  if (before.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
  if (before.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
  if (before.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
  if (before.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
  if (before.folder?.guid !== after.folder?.guid) diff.folder = after.folder;
  return diff;
};
export const inverseFileDiff = (original: File, appliedDiff: FileDiff): FileDiff => {
  const inverse: FileDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.remote !== undefined) inverse.remote = original.remote;
  if (appliedDiff.size !== undefined) inverse.size = original.size;
  if (appliedDiff.hash !== undefined) inverse.hash = original.hash;
  if (appliedDiff.createdAt !== undefined) inverse.createdAt = original.createdAt;
  if (appliedDiff.createdBy !== undefined) inverse.createdBy = original.createdBy;
  if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = original.updatedAt;
  if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = original.updatedBy;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder;
  return inverse;
};
export const mergeFileDiff = (diff1: FileDiff, diff2: FileDiff): FileDiff => {
  return { ...diff1, ...diff2 };
};
export const applyFileDiff = (base: File, diff: FileDiff): File => {
  return {
    ...base,
    name: diff.name ?? base.name,
    remote: diff.remote ?? base.remote,
    size: diff.size ?? base.size,
    hash: diff.hash ?? base.hash,
    createdAt: diff.createdAt ?? base.createdAt,
    createdBy: diff.createdBy ?? base.createdBy,
    updatedAt: diff.updatedAt ?? base.updatedAt,
    updatedBy: diff.updatedBy ?? base.updatedBy,
    folder: diff.folder ?? base.folder,
  };
};

export const FilesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: FileDiffSchema })).optional(),
  added: z.array(FileSchema).optional(),
});
export type FilesDiff = z.infer<typeof FilesDiffSchema>;

// #endregion File

// #region Folder

export const FolderSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: FolderIdSchema.optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  createdBy: z.string().optional(),
  updatedAt: DateProperty(),
  updatedBy: z.string().optional(),
});
export type Folder = z.infer<typeof FolderSchema>;
export const serializeFolder = (folder: Folder): string => JSON.stringify(FolderSchema.parse(folder));
export const deserializeFolder = (json: string): Folder => FolderSchema.parse(JSON.parse(json));

export const FolderDiffSchema = FolderSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type FolderDiff = z.infer<typeof FolderDiffSchema>;
export const getFolderDiff = (before: Folder, after: Folder): FolderDiff => {
  const diff: FolderDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.parent?.guid !== after.parent?.guid) diff.parent = after.parent;
  if (before.description !== after.description) diff.description = after.description;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (before.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
  if (before.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
  if (before.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
  if (before.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
  return diff;
};
export const inverseFolderDiff = (original: Folder, appliedDiff: FolderDiff): FolderDiff => {
  const inverse: FolderDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.parent !== undefined) inverse.parent = original.parent;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  if (appliedDiff.createdAt !== undefined) inverse.createdAt = original.createdAt;
  if (appliedDiff.createdBy !== undefined) inverse.createdBy = original.createdBy;
  if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = original.updatedAt;
  if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = original.updatedBy;
  return inverse;
};
export const mergeFolderDiff = (diff1: FolderDiff, diff2: FolderDiff): FolderDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyFolderDiff = (base: Folder, diff: FolderDiff): Folder => {
  return {
    ...base,
    name: diff.name ?? base.name,
    parent: diff.parent ?? base.parent,
    description: diff.description ?? base.description,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
    createdAt: diff.createdAt ?? base.createdAt,
    createdBy: diff.createdBy ?? base.createdBy,
    updatedAt: diff.updatedAt ?? base.updatedAt,
    updatedBy: diff.updatedBy ?? base.updatedBy,
  };
};

export const FoldersDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: FolderDiffSchema })).optional(),
  added: z.array(FolderSchema).optional(),
});
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;

// #endregion Folder

// #region Benchmark

// https://github.com/usalu/semio#-benchmark-
export const BenchmarkSchema = z.object({
  guid: z.string(),
  name: z.string(),
  icon: z.string().optional(),
  min: z.number().optional(),
  minExcluded: z.boolean().optional(),
  max: z.number().optional(),
  maxExcluded: z.boolean().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Benchmark = z.infer<typeof BenchmarkSchema>;
export const serializeBenchmark = (benchmark: Benchmark): string => JSON.stringify(BenchmarkSchema.parse(benchmark));
export const deserializeBenchmark = (json: string): Benchmark => BenchmarkSchema.parse(JSON.parse(json));

export const BenchmarkDiffSchema = BenchmarkSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;
export const applyBenchmarkDiff = (base: Benchmark, diff: BenchmarkDiff): Benchmark => {
  return {
    ...base,
    name: diff.name ?? base.name,
    icon: diff.icon ?? base.icon,
    min: diff.min ?? base.min,
    minExcluded: diff.minExcluded ?? base.minExcluded,
    max: diff.max ?? base.max,
    maxExcluded: diff.maxExcluded ?? base.maxExcluded,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};
export const getBenchmarkDiff = (before: Benchmark, after: Benchmark): BenchmarkDiff => {
  const diff: BenchmarkDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.min !== after.min) diff.min = after.min !== undefined && before.min !== undefined ? after.min - before.min : after.min;
  if (before.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
  if (before.max !== after.max) diff.max = after.max !== undefined && before.max !== undefined ? after.max - before.max : after.max;
  if (before.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseBenchmarkDiff = (original: Benchmark, appliedDiff: BenchmarkDiff): BenchmarkDiff => {
  const inverse: BenchmarkDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.min !== undefined) inverse.min = original.min;
  if (appliedDiff.minExcluded !== undefined) inverse.minExcluded = original.minExcluded;
  if (appliedDiff.max !== undefined) inverse.max = original.max;
  if (appliedDiff.maxExcluded !== undefined) inverse.maxExcluded = original.maxExcluded;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeBenchmarkDiff = (diff1: BenchmarkDiff, diff2: BenchmarkDiff): BenchmarkDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};

export const BenchmarksDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: BenchmarkDiffSchema })).optional(),
  added: z.array(BenchmarkSchema).optional(),
});
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;

const getBenchmarksDiff = (before: Benchmark[], after: Benchmark[]): BenchmarksDiff => {
  const beforeNames = before.map((b) => b.name);
  const afterNames = after.map((b) => b.name);
  const removed = beforeNames.filter((name) => !afterNames.includes(name));
  const added = after.filter((b) => !beforeNames.includes(b.name));
  const updated = after
    .filter((b) => beforeNames.includes(b.name))
    .map((afterBenchmark) => {
      const beforeBenchmark = before.find((b) => b.name === afterBenchmark.name)!;
      const diff = getBenchmarkDiff(beforeBenchmark, afterBenchmark);
      return { id: afterBenchmark.name, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: BenchmarksDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

const inverseBenchmarksDiff = (original: Benchmark[], appliedDiff: BenchmarksDiff): BenchmarksDiff => {
  const addedNames = appliedDiff.added?.map((b) => b.name) ?? [];
  const removedNames = appliedDiff.removed ?? [];
  const updatedNames = appliedDiff.updated?.map((u) => u.id) ?? [];
  return {
    removed: addedNames,
    added: original.filter((b) => removedNames.includes(b.name)),
    updated: updatedNames.map((name) => {
      const orig = original.find((b) => b.name === name)!;
      const upd = appliedDiff.updated?.find((u) => u.id === name)!;
      return { id: name, diff: inverseBenchmarkDiff(orig, upd.diff) };
    }),
  };
};

const mergeBenchmarksDiff = (first: BenchmarksDiff, second: BenchmarksDiff): BenchmarksDiff => {
  return { ...first, ...second };
};

const applyBenchmarksDiff = (base: Benchmark[], diff: BenchmarksDiff): Benchmark[] => {
  let result = [...base];
  if (diff.removed) {
    result = result.filter((benchmark) => !diff.removed!.includes(benchmark.name));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((benchmark) => benchmark.name === update.id);
      if (index !== -1) {
        result[index] = applyBenchmarkDiff(result[index], update.diff);
      }
    }
  }

  if (diff.added) {
    result.push(...diff.added);
  }

  return result;
};

// #endregion Benchmark

// #region QualityKind

// #endregion QualityKind

// #region Quality

// https://github.com/usalu/semio#-quality-
export const QualitySchema = z.object({
  guid: z.string(),
  key: z.string(),
  name: z.string(),
  description: z.string().optional(),
  uri: z.string().optional(),
  kind: z.number().optional(),
  folder: z.string().optional(),
  canScale: z.boolean().optional(),
  defaultSiUnit: z.string().optional(),
  defaultImperialUnit: z.string().optional(),
  min: z.number().optional(),
  isMinExcluded: z.boolean().optional(),
  max: z.number().optional(),
  isMaxExcluded: z.boolean().optional(),
  defaultValue: z.number().optional(),
  formula: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  unit: z.string().optional(),
  benchmarks: z.array(BenchmarkSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Quality = z.infer<typeof QualitySchema>;
export const serializeQuality = (quality: Quality): string => JSON.stringify(QualitySchema.parse(quality));
export const deserializeQuality = (json: string): Quality => QualitySchema.parse(JSON.parse(json));

export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true, attributes: true }).extend({
  benchmarks: BenchmarksDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type QualityDiff = z.infer<typeof QualityDiffSchema>;
export const getQualityDiff = (before: Quality, after: Quality): QualityDiff => {
  const diff: QualityDiff = {};
  if (before.key !== after.key) diff.key = after.key;
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.uri !== after.uri) diff.uri = after.uri;
  if (before.kind !== after.kind) diff.kind = after.kind !== undefined && before.kind !== undefined ? after.kind - before.kind : after.kind;
  if (before.canScale !== after.canScale) diff.canScale = after.canScale;
  if (before.defaultSiUnit !== after.defaultSiUnit) diff.defaultSiUnit = after.defaultSiUnit;
  if (before.defaultImperialUnit !== after.defaultImperialUnit) diff.defaultImperialUnit = after.defaultImperialUnit;
  if (before.min !== after.min) diff.min = after.min !== undefined && before.min !== undefined ? after.min - before.min : after.min;
  if (before.isMinExcluded !== after.isMinExcluded) diff.isMinExcluded = after.isMinExcluded;
  if (before.max !== after.max) diff.max = after.max !== undefined && before.max !== undefined ? after.max - before.max : after.max;
  if (before.isMaxExcluded !== after.isMaxExcluded) diff.isMaxExcluded = after.isMaxExcluded;
  if (before.defaultValue !== after.defaultValue) diff.defaultValue = after.defaultValue !== undefined && before.defaultValue !== undefined ? after.defaultValue - before.defaultValue : after.defaultValue;
  if (before.formula !== after.formula) diff.formula = after.formula;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.benchmarks !== after.benchmarks) diff.benchmarks = getBenchmarksDiff(before.benchmarks ?? [], after.benchmarks ?? []);
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseQualityDiff = (original: Quality, appliedDiff: QualityDiff): QualityDiff => {
  const inverse: QualityDiff = {};
  if (appliedDiff.key !== undefined) inverse.key = original.key;
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.uri !== undefined) inverse.uri = original.uri;
  if (appliedDiff.kind !== undefined) inverse.kind = original.kind;
  if (appliedDiff.canScale !== undefined) inverse.canScale = original.canScale;
  if (appliedDiff.defaultSiUnit !== undefined) inverse.defaultSiUnit = original.defaultSiUnit;
  if (appliedDiff.defaultImperialUnit !== undefined) inverse.defaultImperialUnit = original.defaultImperialUnit;
  if (appliedDiff.min !== undefined) inverse.min = original.min;
  if (appliedDiff.isMinExcluded !== undefined) inverse.isMinExcluded = original.isMinExcluded;
  if (appliedDiff.max !== undefined) inverse.max = original.max;
  if (appliedDiff.isMaxExcluded !== undefined) inverse.isMaxExcluded = original.isMaxExcluded;
  if (appliedDiff.defaultValue !== undefined) inverse.defaultValue = original.defaultValue;
  if (appliedDiff.formula !== undefined) inverse.formula = original.formula;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.image !== undefined) inverse.image = original.image;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.benchmarks !== undefined) inverse.benchmarks = inverseBenchmarksDiff(original.benchmarks ?? [], appliedDiff.benchmarks);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeQualityDiff = (diff1: QualityDiff, diff2: QualityDiff): QualityDiff => {
  return {
    ...diff1,
    ...diff2,
    benchmarks: diff1.benchmarks && diff2.benchmarks ? mergeBenchmarksDiff(diff1.benchmarks, diff2.benchmarks) : (diff2.benchmarks ?? diff1.benchmarks),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
export const applyQualityDiff = (base: Quality, diff: QualityDiff): Quality => {
  return {
    ...base,
    key: diff.key ?? base.key,
    name: diff.name ?? base.name,
    description: diff.description ?? base.description,
    uri: diff.uri ?? base.uri,
    kind: diff.kind ?? base.kind,
    canScale: diff.canScale ?? base.canScale,
    defaultSiUnit: diff.defaultSiUnit ?? base.defaultSiUnit,
    defaultImperialUnit: diff.defaultImperialUnit ?? base.defaultImperialUnit,
    min: diff.min ?? base.min,
    isMinExcluded: diff.isMinExcluded ?? base.isMinExcluded,
    max: diff.max ?? base.max,
    isMaxExcluded: diff.isMaxExcluded ?? base.isMaxExcluded,
    defaultValue: diff.defaultValue ?? base.defaultValue,
    formula: diff.formula ?? base.formula,
    icon: diff.icon ?? base.icon,
    image: diff.image ?? base.image,
    unit: diff.unit ?? base.unit,
    benchmarks: diff.benchmarks ? applyBenchmarksDiff(base.benchmarks ?? [], diff.benchmarks) : base.benchmarks,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const QualitiesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: QualityDiffSchema })).optional(),
  added: z.array(QualitySchema).optional(),
});

// #endregion Quality

// #region Interface
// https://github.com/usalu/semio#-interface-

export const InterfaceSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  compatibleInterfaces: z.array(InterfaceIdSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Interface = z.infer<typeof InterfaceSchema>;
export const serializeInterface = (iface: Interface): string => JSON.stringify(InterfaceSchema.parse(iface));
export const deserializeInterface = (json: string): Interface => InterfaceSchema.parse(JSON.parse(json));

export const InterfaceDiffSchema = InterfaceSchema.partial().omit({ compatibleInterfaces: true, attributes: true }).extend({
  compatibleInterfaces: z.array(InterfaceIdSchema).optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type InterfaceDiff = z.infer<typeof InterfaceDiffSchema>;
export const getInterfaceDiff = (before: Interface, after: Interface): InterfaceDiff => {
  const diff: InterfaceDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (JSON.stringify(before.compatibleInterfaces) !== JSON.stringify(after.compatibleInterfaces)) diff.compatibleInterfaces = after.compatibleInterfaces;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseInterfaceDiff = (original: Interface, appliedDiff: InterfaceDiff): InterfaceDiff => {
  const inverse: InterfaceDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.compatibleInterfaces !== undefined) inverse.compatibleInterfaces = original.compatibleInterfaces;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeInterfaceDiff = (diff1: InterfaceDiff, diff2: InterfaceDiff): InterfaceDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
export const applyInterfaceDiff = (base: Interface, diff: InterfaceDiff): Interface => {
  return {
    ...base,
    name: diff.name ?? base.name,
    description: diff.description ?? base.description,
    icon: diff.icon ?? base.icon,
    compatibleInterfaces: diff.compatibleInterfaces ?? base.compatibleInterfaces,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const InterfacesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: InterfaceDiffSchema })).optional(),
  added: z.array(InterfaceSchema).optional(),
});
export type InterfacesDiff = z.infer<typeof InterfacesDiffSchema>;
export const getInterfacesDiff = (before: Interface[], after: Interface[]): InterfacesDiff => {
  const diff: InterfacesDiff = {};
  const beforeGuids = new Set(before.map((i) => i.guid));
  const afterGuids = new Set(after.map((i) => i.guid));
  const removed = before.filter((i) => !afterGuids.has(i.guid)).map((i) => i.guid);
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterGuids.has(i.guid))
    .map((i) => {
      const afterInterface = after.find((a) => a.guid === i.guid)!;
      const interfaceDiff = getInterfaceDiff(i, afterInterface);
      return { id: i.guid, diff: interfaceDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeGuids.has(i.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
export const inverseInterfacesDiff = (original: Interface[], appliedDiff: InterfacesDiff): InterfacesDiff => {
  const inverse: InterfacesDiff = {};
  if (appliedDiff.removed) inverse.added = original.filter((i) => appliedDiff.removed!.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => i.guid);
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalInterface = original.find((i) => i.guid === u.id)!;
      return { id: u.id, diff: inverseInterfaceDiff(originalInterface, u.diff) };
    });
  }
  return inverse;
};
export const mergeInterfacesDiff = (diff1: InterfacesDiff, diff2: InterfacesDiff): InterfacesDiff => {
  return {
    removed: [...(diff1.removed ?? []), ...(diff2.removed ?? [])],
    updated: [...(diff1.updated ?? []), ...(diff2.updated ?? [])],
    added: [...(diff1.added ?? []), ...(diff2.added ?? [])],
  };
};
export const applyInterfacesDiff = (base: Interface[], diff: InterfacesDiff): Interface[] => {
  let result = [...base];
  if (diff.removed) {
    result = result.filter((i) => !diff.removed!.includes(i.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((i) => i.guid === update.id);
      if (index !== -1) {
        result[index] = applyInterfaceDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

export const areInterfacesCompatible = (iface1: Interface | undefined, iface2: Interface | undefined, allInterfaces: Interface[]): boolean => {
  if (!iface1 || !iface2) return true;
  if (iface1.guid === iface2.guid) return true;
  const iface1Compatible = iface1.compatibleInterfaces ?? [];
  const iface2Compatible = iface2.compatibleInterfaces ?? [];
  if (iface1Compatible.length === 0 && iface2Compatible.length === 0) return true;
  if (iface1Compatible.length === 0) return iface2Compatible.some((c) => c.guid === iface1.guid);
  if (iface2Compatible.length === 0) return iface1Compatible.some((c) => c.guid === iface2.guid);
  return iface1Compatible.some((c) => c.guid === iface2.guid) || iface2Compatible.some((c) => c.guid === iface1.guid);
};

// #endregion Interface

// #region Prop
// https://github.com/usalu/semio#-prop-

export const PropSchema = z.object({
  guid: z.string(),
  quality: QualityIdSchema,
  value: z.string(),
  unit: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Prop = z.infer<typeof PropSchema>;
export const serializeProp = (prop: Prop): string => JSON.stringify(PropSchema.parse(prop));
export const deserializeProp = (json: string): Prop => PropSchema.parse(JSON.parse(json));

export const PropDiffSchema = PropSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type PropDiff = z.infer<typeof PropDiffSchema>;
export const getPropDiff = (before: Prop, after: Prop): PropDiff => {
  const diff: PropDiff = {};
  if (before.quality.guid !== after.quality.guid) diff.quality = after.quality;
  if (before.value !== after.value) diff.value = after.value;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inversePropDiff = (original: Prop, appliedDiff: PropDiff): PropDiff => {
  const inverse: PropDiff = {};
  if (appliedDiff.quality !== undefined) inverse.quality = original.quality;
  if (appliedDiff.value !== undefined) inverse.value = original.value;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergePropDiff = (diff1: PropDiff, diff2: PropDiff): PropDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyPropDiff = (base: Prop, diff: PropDiff): Prop => {
  return {
    ...base,
    quality: diff.quality ?? base.quality,
    value: diff.value ?? base.value,
    unit: diff.unit ?? base.unit,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const PropsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: PropDiffSchema })).optional(),
  added: z.array(PropSchema).optional(),
});
export type PropsDiff = z.infer<typeof PropsDiffSchema>;

const getPropsDiff = (before: Prop[], after: Prop[]): PropsDiff => {
  const beforeKeys = before.map((p) => p.quality.guid);
  const afterKeys = after.map((p) => p.quality.guid);
  const removed = beforeKeys.filter((key) => !afterKeys.includes(key));
  const added = after.filter((p) => !beforeKeys.includes(p.quality.guid));
  const updated = after
    .filter((p) => beforeKeys.includes(p.quality.guid))
    .map((afterProp) => {
      const beforeProp = before.find((p) => p.quality.guid === afterProp.quality.guid)!;
      const diff = getPropDiff(beforeProp, afterProp);
      return { id: afterProp.quality.guid, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: PropsDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

const inversePropsDiff = (original: Prop[], appliedDiff: PropsDiff): PropsDiff => {
  const addedKeys = appliedDiff.added?.map((p) => p.quality.guid) ?? [];
  const removedKeys = appliedDiff.removed ?? [];
  const updatedKeys = appliedDiff.updated?.map((u) => u.id) ?? [];
  return {
    removed: addedKeys,
    added: original.filter((p) => removedKeys.includes(p.quality.guid)),
    updated: updatedKeys.map((key) => {
      const orig = original.find((p) => p.quality.guid === key)!;
      const upd = appliedDiff.updated?.find((u) => u.id === key)!;
      return { id: key, diff: inversePropDiff(orig, upd.diff) };
    }),
  };
};

const mergePropsDiff = (first: PropsDiff, second: PropsDiff): PropsDiff => {
  return { ...first, ...second };
};

const applyPropsDiff = (base: Prop[], diff: PropsDiff): Prop[] => {
  let result = [...base];
  if (diff.removed) {
    result = result.filter((prop) => !diff.removed!.includes(prop.quality.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((prop) => prop.quality.guid === update.id);
      if (index !== -1) {
        result[index] = applyPropDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

// #endregion Prop

// #region Model
// https://github.com/usalu/semio#-model-

export const ModelSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  tags: z.array(z.string()).optional(),
  file: z.string(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Model = z.infer<typeof ModelSchema>;
export const serializeModel = (model: Model): string => JSON.stringify(ModelSchema.parse(model));
export const deserializeModel = (json: string): Model => ModelSchema.parse(JSON.parse(json));

export const ModelDiffSchema = ModelSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type ModelDiff = z.infer<typeof ModelDiffSchema>;
export const getModelDiff = (before: Model, after: Model): ModelDiff => {
  const diff: ModelDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (JSON.stringify(before.tags) !== JSON.stringify(after.tags)) diff.tags = after.tags;
  if (before.file !== after.file) diff.file = after.file;
  if (before.description !== after.description) diff.description = after.description;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseModelDiff = (original: Model, appliedDiff: ModelDiff): ModelDiff => {
  const inverse: ModelDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.tags !== undefined) inverse.tags = original.tags;
  if (appliedDiff.file !== undefined) inverse.file = original.file;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeModelDiff = (diff1: ModelDiff, diff2: ModelDiff): ModelDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyModelDiff = (base: Model, diff: ModelDiff): Model => {
  return {
    ...base,
    name: diff.name ?? base.name,
    tags: diff.tags ?? base.tags,
    file: diff.file ?? base.file,
    description: diff.description ?? base.description,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const ModelsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: ModelDiffSchema })).optional(),
  added: z.array(ModelSchema).optional(),
});

export const areSameModel = (model: Model, other: Model): boolean => {
  return model.tags?.every((tag) => other.tags?.includes(tag)) ?? true;
};

export const findModel = (models: Model[], tags: string[]): Model => {
  const indices = models.map((r) => jaccard(r.tags, tags));
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return models[maxIndexIndex];
};

export const getAllTagsFromModels = (models: Model[]): string[] => {
  const tagsSet = new Set<string>();
  models.forEach((r) => {
    toArray(r.tags).forEach((tag) => tagsSet.add(tag));
  });
  return Array.from(tagsSet).sort();
};

export const filterModelsByTags = (models: Model[], selectedTags: string[]): Model[] => {
  if (!selectedTags || selectedTags.length === 0) return models;
  return models.filter((r) => {
    if (!r.tags || r.tags.length === 0) return false;
    return selectedTags.every((tag) => r.tags?.includes(tag));
  });
};

export const getAvailableTagsForModels = (models: Model[], selectedTags: string[]): string[] => {
  const filteredReps = filterModelsByTags(models, selectedTags);
  const availableTags = getAllTagsFromModels(filteredReps);
  return availableTags.filter((tag) => !selectedTags.includes(tag));
};

export const selectBestModel = (models: Model[], selectedTags: string[]): Model | undefined => {
  if (models.length === 0) return undefined;
  if (selectedTags.length === 0) {
    const defaultRep = models.find((r) => !r.tags || r.tags.length === 0);
    return defaultRep ?? models[0];
  }
  const filteredReps = filterModelsByTags(models, selectedTags);
  if (filteredReps.length === 0) return undefined;
  return findModel(filteredReps, selectedTags);
};

// #endregion Model

// #region Port
// https://github.com/usalu/semio#-port-

export const PortSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  t: z.number(),
  point: PointSchema,
  direction: VectorSchema,
  description: z.string().optional(),
  interface: InterfaceIdSchema.optional(),
  mandatory: z.boolean().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Port = z.infer<typeof PortSchema>;
export const serializePort = (port: Port): string => JSON.stringify(PortSchema.parse(port));
export const deserializePort = (json: string): Port => PortSchema.parse(JSON.parse(json));

export const PortDiffSchema = PortSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({
  point: PointDiffSchema.optional(),
  direction: VectorDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type PortDiff = z.infer<typeof PortDiffSchema>;
export const getPortDiff = (before: Port, after: Port): PortDiff => {
  const diff: PortDiff = {};
  if (before.guid !== after.guid) diff.guid = after.guid;
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.interface?.guid !== after.interface?.guid) diff.interface = after.interface;
  if (before.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
  if (before.t !== after.t) diff.t = after.t - before.t;
  if (before.point !== after.point) diff.point = getPointDiff(before.point, after.point);
  if (before.direction !== after.direction) diff.direction = getVectorDiff(before.direction, after.direction);
  if (before.props !== after.props) diff.props = getPropsDiff(before.props ?? [], after.props ?? []);
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const mergePortDiff = (diff1: PortDiff, diff2: PortDiff): PortDiff => {
  return {
    ...diff1,
    ...diff2,
    point: diff2.point ?? diff1.point,
    direction: diff2.direction ?? diff1.direction,
    props: diff1.props && diff2.props ? mergePropsDiff(diff1.props, diff2.props) : (diff2.props ?? diff1.props),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
export const inversePortDiff = (original: Port, appliedDiff: PortDiff): PortDiff => {
  const inverse: PortDiff = {};
  if (appliedDiff.guid !== undefined) inverse.guid = original.guid;
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.interface !== undefined) inverse.interface = original.interface;
  if (appliedDiff.mandatory !== undefined) inverse.mandatory = original.mandatory;
  if (appliedDiff.t !== undefined) inverse.t = original.t;
  if (appliedDiff.point !== undefined) inverse.point = inversePointDiff(original.point, appliedDiff.point);
  if (appliedDiff.direction !== undefined) inverse.direction = inverseVectorDiff(original.direction, appliedDiff.direction);
  if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(original.props ?? [], appliedDiff.props);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const applyPortDiff = (base: Port, diff: PortDiff): Port => {
  return {
    ...base,
    guid: diff.guid ?? base.guid,
    name: diff.name ?? base.name,
    description: diff.description ?? base.description,
    interface: diff.interface ?? base.interface,
    mandatory: diff.mandatory ?? base.mandatory,
    t: diff.t ?? base.t,
    point: diff.point ? applyPointDiff(base.point, diff.point) : base.point,
    direction: diff.direction ? applyVectorDiff(base.direction, diff.direction) : base.direction,
    props: diff.props ? applyPropsDiff(base.props ?? [], diff.props) : base.props,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const PortsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: PortDiffSchema })).optional(),
  added: z.array(PortSchema).optional(),
});

const getPortsDiff = (before: Port[], after: Port[]): { removed?: string[]; updated?: { id: string; diff: PortDiff }[]; added?: Port[] } => {
  const beforeGuids = before.map((p) => p.guid);
  const afterGuids = after.map((p) => p.guid);
  const removed = beforeGuids.filter((guid) => !afterGuids.includes(guid));
  const added = after.filter((p) => !beforeGuids.includes(p.guid));
  const updated = after
    .filter((p) => beforeGuids.includes(p.guid))
    .map((afterPort) => {
      const beforePort = before.find((p) => p.guid === afterPort.guid)!;
      const diff = getPortDiff(beforePort, afterPort);
      return { id: afterPort.guid, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: { removed?: string[]; updated?: { id: string; diff: PortDiff }[]; added?: Port[] } = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

// FIXME: Disabled - uses old Port schema where interface was string and compatibleInterfaces existed on Port
// Now: port.interface is {guid: string}, compatibility is on Interface entity
export const unifyPortInterfacesAndCompatibleInterfacesForTypes = (types: Type[]): TypesDiff => {
  return { updated: [] };
  /*
  const allInterfaces = new Set<string>();
  for (const type of types) {
    for (const port of type.ports || []) {
      if (port.interface && port.interface !== "") allInterfaces.add(port.interface);
      for (const compatibleInterface of port.compatibleInterfaces || []) {
        if (compatibleInterface && compatibleInterface !== "") allInterfaces.add(compatibleInterface);
      }
    }
  }

  // Union-Find data structure
  const parent = new Map<string, string>();
  const rank = new Map<string, number>();

  // Initialize each interface as its own parent
  for (const interface_ of Array.from(allInterfaces)) {
    parent.set(interface_, interface_);
    rank.set(interface_, 0);
  }

  // Find with path compression
  const find = (interface_: string): string => {
    if (parent.get(interface_) !== interface_) parent.set(interface_, find(parent.get(interface_)!));
    return parent.get(interface_)!;
  };

  // Union by rank
  const union = (interface1: string, interface2: string): void => {
    const root1 = find(interface1);
    const root2 = find(interface2);

    if (root1 === root2) return;

    const rank1 = rank.get(root1)!;
    const rank2 = rank.get(root2)!;

    if (rank1 < rank2) {
      parent.set(root1, root2);
    } else if (rank1 > rank2) {
      parent.set(root2, root1);
    } else {
      parent.set(root2, root1);
      rank.set(root1, rank1 + 1);
    }
  };

  // Build compatibility groups by examining all ports
  for (const type of types) {
    for (const port of type.ports || []) {
      const portInterface = port.interface;
      const compatibleInterfaces = port.compatibleInterfaces || [];

      if (portInterface && portInterface !== "") {
        // Union port's interface with all its compatible interfaces
        for (const compatibleInterface of compatibleInterfaces) {
          if (compatibleInterface && compatibleInterface !== "") {
            union(portInterface, compatibleInterface);
          }
        }
      }

      // Also union all compatible interfaces with each other
      for (let i = 0; i < compatibleInterfaces.length; i++) {
        for (let j = i + 1; j < compatibleInterfaces.length; j++) {
          const interface1 = compatibleInterfaces[i];
          const interface2 = compatibleInterfaces[j];
          if (interface1 && interface1 !== "" && interface2 && interface2 !== "") {
            union(interface1, interface2);
          }
        }
      }
    }
  }

  // Create mapping from any interface to its representative
  const interfaceToRepresentative = new Map<string, string>();
  for (const interface_ of Array.from(allInterfaces)) {
    interfaceToRepresentative.set(interface_, find(interface_));
  }

  // Update all types with unified port interfaces
  const updated: { id: string; diff: TypeDiff }[] = [];

  for (const type of types) {
    const updatedPorts = type.ports?.map((port) => {
      const portInterface = port.interface;
      const compatibleInterfaces = port.compatibleInterfaces || [];

      // Determine the representative interface for this port
      let representative: string | undefined;

      if (portInterface && portInterface !== "") {
        representative = interfaceToRepresentative.get(portInterface);
      } else if (compatibleInterfaces.length > 0) {
        // If no interface but has compatible interfaces, use the first one's representative
        const firstCompatible = compatibleInterfaces.find((f) => f && f !== "");
        if (firstCompatible) {
          representative = interfaceToRepresentative.get(firstCompatible);
        }
      }

      if (representative) {
        return {
          ...port,
          interface: representative,
          compatibleInterfaces: [representative],
        };
      } else {
        // No interface information, keep as is
        return port;
      }
    });

    if (updatedPorts) {
      const portsDiff = getPortsDiff(type.ports ?? [], updatedPorts);
      updated.push({
        id: type.guid,
        diff: {
          ports: portsDiff,
        },
      });
    }
  }

  return { updated };
  */
};
// FIXME: Disabled - uses old Port schema
export const arePortsCompatible = (port: Port, otherPort: Port): boolean => {
  return true; // Compatibility now handled by Interface entity
  /*
  const normalizedPortInterface = normalize(port.interface);
  const normalizedOtherPortInterface = normalize(otherPort.interface);
  if (normalizedPortInterface === "" || normalizedOtherPortInterface === "") return true;
  return (port.compatibleInterfaces ?? []).includes(normalizedOtherPortInterface) || (otherPort.compatibleInterfaces ?? []).includes(normalizedPortInterface);
  */
};

export const findPort = (ports: Port[], portGuid: string): Port => {
  const port = ports.find((p) => p.guid === portGuid);
  if (!port) throw new Error(`Port ${portGuid} not found in ports`);
  return port;
};

// #endregion Port

// #region Type
// https://github.com/usalu/semio#-type-
export const TypeSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: TypeIdSchema.optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  models: z.array(ModelSchema).optional(),
  ports: z.array(PortSchema).optional(),
  props: z.array(PropSchema).optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(z.string()).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Type = z.infer<typeof TypeSchema>;
export const serializeType = (type: Type): string => JSON.stringify(TypeSchema.parse(type));
export const deserializeType = (json: string): Type => TypeSchema.parse(JSON.parse(json));

export const TypeShallowSchema = TypeSchema.omit({ models: true, ports: true }).extend({
  models: z.array(z.string()).optional(),
  ports: z.array(z.string()).optional(),
});
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
export const serializeTypeShallow = (type: TypeShallow): string => JSON.stringify(TypeShallowSchema.parse(type));
export const deserializeTypeShallow = (json: string): TypeShallow => TypeShallowSchema.parse(JSON.parse(json));
export const TypeDiffSchema = TypeSchema.partial().omit({ models: true, ports: true, props: true, attributes: true }).extend({
  models: ModelsDiffSchema.optional(),
  ports: PortsDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
export const getTypeDiff = (before: Type, after: Type): TypeDiff => {
  // TODO: Implement full Type diff logic
  return {};
};

export const applyTypeDiff = (base: Type, diff: TypeDiff): Type => {
  // TODO: Implement full Type apply diff logic including ports, models, props
  return base;
};

export const mergeTypeDiff = (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => {
  // TODO: Implement full Type merge diff logic
  return { ...diff1, ...diff2 };
};

export const inverseTypeDiff = (original: Type, appliedDiff: TypeDiff): TypeDiff => {
  // TODO: Implement full Type inverse diff logic
  return {};
};

export const TypesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: TypeDiffSchema })).optional(),
  added: z.array(TypeSchema).optional(),
});
export type TypesDiff = z.infer<typeof TypesDiffSchema>;

export const findPortInType = (type: Type, portGuid: string): Port => findPort(type.ports ?? [], portGuid);

// #endregion Type

// #region Layer
// https://github.com/usalu/semio#-layer-

export const LayerSchema = z.object({
  path: z.string(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Layer = z.infer<typeof LayerSchema>;
export const serializeLayer = (layer: Layer): string => JSON.stringify(LayerSchema.parse(layer));
export const deserializeLayer = (json: string): Layer => LayerSchema.parse(JSON.parse(json));

export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type LayerDiff = z.infer<typeof LayerDiffSchema>;

export const getLayerDiff = (before: Layer, after: Layer): LayerDiff => {
  const diff: LayerDiff = {};
  if (before.path !== after.path) diff.path = after.path;
  if (before.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
  if (before.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
  if (before.color !== after.color) diff.color = after.color;
  if (before.description !== after.description) diff.description = after.description;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseLayerDiff = (original: Layer, appliedDiff: LayerDiff): LayerDiff => {
  const inverse: LayerDiff = {};
  if (appliedDiff.path !== undefined) inverse.path = original.path;
  if (appliedDiff.isHidden !== undefined) inverse.isHidden = original.isHidden;
  if (appliedDiff.isLocked !== undefined) inverse.isLocked = original.isLocked;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeLayerDiff = (diff1: LayerDiff, diff2: LayerDiff): LayerDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyLayerDiff = (base: Layer, diff: LayerDiff): Layer => {
  return {
    ...base,
    path: diff.path ?? base.path,
    isHidden: diff.isHidden ?? base.isHidden,
    isLocked: diff.isLocked ?? base.isLocked,
    color: diff.color ?? base.color,
    description: diff.description ?? base.description,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const LayersDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: LayerDiffSchema })).optional(),
  added: z.array(LayerSchema).optional(),
});
export type LayersDiff = z.infer<typeof LayersDiffSchema>;

// #endregion Layer

// #region Piece
// https://github.com/usalu/semio#-piece-

export const PieceSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  type: TypeIdSchema.optional(),
  design: DesignIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: CoordSchema.optional(),
  scale: z.number().optional(),
  mirrorPlane: PlaneSchema.optional(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Piece = z.infer<typeof PieceSchema>;
export const serializePiece = (piece: Piece): string => JSON.stringify(PieceSchema.parse(piece));
export const deserializePiece = (json: string): Piece => PieceSchema.parse(JSON.parse(json));

export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, attributes: true }).extend({
  plane: PlaneDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export const getPieceDiff = (before: Piece, after: Piece): PieceDiff => {
  const diff: PieceDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.type?.guid !== after.type?.guid) diff.type = after.type;
  if (before.design?.guid !== after.design?.guid) diff.design = after.design;
  if (before.plane !== after.plane) diff.plane = after.plane ? getPlaneDiff(before.plane ?? { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } }, after.plane) : undefined;
  if (before.center !== after.center) diff.center = after.center;
  if (before.scale !== after.scale) diff.scale = after.scale;
  if (before.mirrorPlane !== after.mirrorPlane) diff.mirrorPlane = after.mirrorPlane;
  if (before.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
  if (before.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
  if (before.color !== after.color) diff.color = after.color;
  if (before.description !== after.description) diff.description = after.description;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inversePieceDiff = (original: Piece, appliedDiff: PieceDiff): PieceDiff => {
  const inverse: PieceDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.type !== undefined) inverse.type = original.type;
  if (appliedDiff.design !== undefined) inverse.design = original.design;
  if (appliedDiff.plane !== undefined) inverse.plane = original.plane;
  if (appliedDiff.center !== undefined) inverse.center = original.center;
  if (appliedDiff.scale !== undefined) inverse.scale = original.scale;
  if (appliedDiff.mirrorPlane !== undefined) inverse.mirrorPlane = original.mirrorPlane;
  if (appliedDiff.isHidden !== undefined) inverse.isHidden = original.isHidden;
  if (appliedDiff.isLocked !== undefined) inverse.isLocked = original.isLocked;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
export const applyPieceDiff = (base: Piece, diff: PieceDiff): Piece => {
  let newPlane = base.plane;
  if (diff.plane) {
    const diffPlane = diff.plane as any;
    if (diffPlane.origin && diffPlane.xAxis && diffPlane.yAxis) {
      newPlane = diffPlane as Plane;
    } else {
      newPlane = applyPlaneDiff(base.plane ?? { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } }, diff.plane);
    }
  }
  return {
    ...base,
    name: diff.name ?? base.name,
    type: diff.type ?? base.type,
    design: diff.design ?? base.design,
    plane: newPlane,
    center: diff.center ?? base.center,
    scale: diff.scale ?? base.scale,
    mirrorPlane: diff.mirrorPlane ?? base.mirrorPlane,
    isHidden: diff.isHidden ?? base.isHidden,
    isLocked: diff.isLocked ?? base.isLocked,
    color: diff.color ?? base.color,
    description: diff.description ?? base.description,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const PiecesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

/**
 * 🔗 Returns a map of piece ids to model file guids for the given design and types.
 * @param design - The design with the pieces to get the model file guids for.
 * @param types - The types of the pieces with the models.
 * @returns A map of piece ids to model file guids.
 */
export const getPieceModelFileGuids = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const modelFileGuids = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type!.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.models) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no models`);
    const model = findModel(type.models, tags);
    modelFileGuids.set(p.guid, model.file);
  });
  return modelFileGuids;
};

/**
 * 🔗 Returns a map of piece ids to model urls for the given design, types, and files.
 * @param design - The design with the pieces to get the model urls for.
 * @param types - The types of the pieces with the models.
 * @param files - The files in the kit to resolve urls from.
 * @param getFileUrl - Function to get the url for a file (from file provider).
 * @returns A map of piece ids to model urls.
 */
export const getPieceModelUrls = (design: Design, types: Type[], files: File[], getFileUrl: (fileGuid: string) => string, tags: string[] = []): Map<string, string> => {
  const modelUrls = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type!.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.models) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no models`);
    const model = findModel(type.models, tags);
    const file = files.find((f) => f.guid === model.file);
    if (!file) throw new Error(`File ${model.file} for model ${model.guid} not found`);
    modelUrls.set(p.guid, getFileUrl(file.guid));
  });
  return modelUrls;
};
export const fixPieceInDesign = (kit: Kit, designId: string, pieceId: string): DesignDiff => {
  const parentConnection = findParentConnectionForPieceInDesign(kit, designId, pieceId);
  return {
    connections: {
      removed: [
        {
          connected: { piece: parentConnection.connected.piece.guid },
          connecting: { piece: parentConnection.connecting.piece.guid },
        },
      ],
    },
  };
};

export const fixPiecesInDesign = (kit: Kit, designId: string, pieceIds: string[]): DesignDiff => {
  const parentConnections = pieceIds.map((pieceId) => findParentConnectionForPieceInDesign(kit, designId, pieceId));
  return {
    connections: {
      removed: parentConnections.map((c) => ({
        connected: { piece: c.connected.piece.guid },
        connecting: { piece: c.connecting.piece.guid },
      })),
    },
  };
};

export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined;
  const isCenterSet = piece.center !== undefined;
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.guid} has inconsistent plane and center`);
  return isPlaneSet;
};

export const findPiece = (pieces: Piece[], pieceGuid: string): Piece => {
  const piece = pieces.find((p) => p.guid === pieceGuid);
  if (!piece) throw new Error(`Piece ${pieceGuid} not found in pieces`);
  return piece;
};

// #endregion Piece

// #region Group
// https://github.com/usalu/semio#-group-

export const GroupSchema = z.object({
  pieces: z.array(PieceIdSchema),
  color: z.string().optional(),
  name: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Group = z.infer<typeof GroupSchema>;
export const GroupDiffSchema = GroupSchema.partial();
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
export const GroupsDiffSchema = z.object({
  removed: z.array(z.array(z.string())).optional(),
  updated: z.array(z.object({ id: z.array(z.string()), diff: GroupDiffSchema })).optional(),
  added: z.array(GroupSchema).optional(),
});
export const serializeGroup = (group: Group): string => JSON.stringify(GroupSchema.parse(group));
export const deserializeGroup = (json: string): Group => GroupSchema.parse(JSON.parse(json));

// #endregion Group

// #region Side
// https://github.com/usalu/semio#-side-

export const SideSchema = z.object({
  piece: PieceIdSchema,
  designPiece: PieceIdSchema.optional(),
  port: PortIdSchema,
});
export type Side = z.infer<typeof SideSchema>;
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = z.infer<typeof SideDiffSchema>;
export const SidesDiffSchema = z.object({
  removed: z.array(z.object({ piece: z.string(), designPiece: z.string().optional(), port: z.string() })).optional(),
  updated: z.array(z.object({ id: z.object({ piece: z.string(), designPiece: z.string().optional(), port: z.string() }), diff: SideDiffSchema })).optional(),
  added: z.array(SideSchema).optional(),
});
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
export const getSideDiff = (before: Side, after: Side): SideDiff => {
  const diff: SideDiff = {};
  if (before.piece.guid !== after.piece.guid) diff.piece = after.piece;
  if (before.designPiece?.guid !== after.designPiece?.guid) diff.designPiece = after.designPiece;
  if (before.port.guid !== after.port.guid) diff.port = after.port;
  return diff;
};
export const inverseSideDiff = (original: Side, appliedDiff: SideDiff): SideDiff => {
  const inverse: SideDiff = {};
  if (appliedDiff.piece !== undefined) inverse.piece = original.piece;
  if (appliedDiff.designPiece !== undefined) inverse.designPiece = original.designPiece;
  if (appliedDiff.port !== undefined) inverse.port = original.port;
  return inverse;
};
export const mergeSideDiff = (diff1: SideDiff, diff2: SideDiff): SideDiff => {
  return { ...diff1, ...diff2 };
};
export const applySideDiff = (base: Side, diff: SideDiff): Side => {
  return {
    ...base,
    ...diff,
  };
};
export const serializeSide = (side: Side): string => JSON.stringify(SideSchema.parse(side));
export const deserializeSide = (json: string): Side => SideSchema.parse(JSON.parse(json));
export const areSameSide = (a: Side, b: Side): boolean => a.piece.guid === b.piece.guid && a.designPiece?.guid === b.designPiece?.guid && a.port.guid === b.port.guid;

// #endregion Side

// #region Connection

// https://github.com/usalu/semio#-connection-
export const ConnectionSchema = z.object({
  guid: z.string(),
  connected: SideSchema,
  connecting: SideSchema,
  gap: z.number().optional(),
  shift: z.number().optional(),
  rise: z.number().optional(),
  rotation: z.number().optional(),
  turn: z.number().optional(),
  tilt: z.number().optional(),
  x: z.number().optional(),
  y: z.number().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Connection = z.infer<typeof ConnectionSchema>;
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ guid: true, connected: true, connecting: true, attributes: true }).extend({
  connected: SideDiffSchema.optional(),
  connecting: SideDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
export const getConnectionDiff = (before: Connection, after: Connection): ConnectionDiff => {
  const diff: ConnectionDiff = {};
  if (before.connected !== after.connected) diff.connected = getSideDiff(before.connected, after.connected);
  if (before.connecting !== after.connecting) diff.connecting = getSideDiff(before.connecting, after.connecting);
  if (before.gap !== after.gap) diff.gap = after.gap !== undefined && before.gap !== undefined ? after.gap - before.gap : after.gap;
  if (before.shift !== after.shift) diff.shift = after.shift !== undefined && before.shift !== undefined ? after.shift - before.shift : after.shift;
  if (before.rise !== after.rise) diff.rise = after.rise !== undefined && before.rise !== undefined ? after.rise - before.rise : after.rise;
  if (before.rotation !== after.rotation) diff.rotation = after.rotation !== undefined && before.rotation !== undefined ? after.rotation - before.rotation : after.rotation;
  if (before.turn !== after.turn) diff.turn = after.turn !== undefined && before.turn !== undefined ? after.turn - before.turn : after.turn;
  if (before.tilt !== after.tilt) diff.tilt = after.tilt !== undefined && before.tilt !== undefined ? after.tilt - before.tilt : after.tilt;
  if (before.x !== after.x) diff.x = after.x !== undefined && before.x !== undefined ? after.x - before.x : after.x;
  if (before.y !== after.y) diff.y = after.y !== undefined && before.y !== undefined ? after.y - before.y : after.y;
  if (before.description !== after.description) diff.description = after.description;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};

export const applyConnectionDiff = (base: Connection, diff: ConnectionDiff): Connection => {
  return {
    ...base,
    ...diff,
    connected: diff.connected ? applySideDiff(base.connected, diff.connected) : base.connected,
    connecting: diff.connecting ? applySideDiff(base.connecting, diff.connecting) : base.connecting,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const mergeConnectionDiff = (diff1: ConnectionDiff, diff2: ConnectionDiff): ConnectionDiff => {
  return {
    ...diff1,
    ...diff2,
    connected: diff2.connected || diff1.connected,
    connecting: diff2.connecting || diff1.connecting,
    attributes: diff2.attributes || diff1.attributes,
  };
};

export const inverseConnectionDiff = (original: Connection, appliedDiff: ConnectionDiff): ConnectionDiff => {
  const inverse: ConnectionDiff = {};
  if (appliedDiff.connected !== undefined) inverse.connected = inverseSideDiff(original.connected, appliedDiff.connected);
  if (appliedDiff.connecting !== undefined) inverse.connecting = inverseSideDiff(original.connecting, appliedDiff.connecting);
  if (appliedDiff.gap !== undefined) inverse.gap = original.gap !== undefined && appliedDiff.gap !== undefined ? -appliedDiff.gap : original.gap;
  if (appliedDiff.shift !== undefined) inverse.shift = original.shift !== undefined && appliedDiff.shift !== undefined ? -appliedDiff.shift : original.shift;
  if (appliedDiff.rise !== undefined) inverse.rise = original.rise !== undefined && appliedDiff.rise !== undefined ? -appliedDiff.rise : original.rise;
  if (appliedDiff.rotation !== undefined) inverse.rotation = original.rotation !== undefined && appliedDiff.rotation !== undefined ? -appliedDiff.rotation : original.rotation;
  if (appliedDiff.turn !== undefined) inverse.turn = original.turn !== undefined && appliedDiff.turn !== undefined ? -appliedDiff.turn : original.turn;
  if (appliedDiff.tilt !== undefined) inverse.tilt = original.tilt !== undefined && appliedDiff.tilt !== undefined ? -appliedDiff.tilt : original.tilt;
  if (appliedDiff.x !== undefined) inverse.x = original.x !== undefined && appliedDiff.x !== undefined ? -appliedDiff.x : original.x;
  if (appliedDiff.y !== undefined) inverse.y = original.y !== undefined && appliedDiff.y !== undefined ? -appliedDiff.y : original.y;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = getAttributesDiff(appliedDiff.attributes ? applyAttributesDiff([], appliedDiff.attributes) : [], original.attributes ?? []);
  return inverse;
};

export const ConnectionsDiffSchema = z.object({
  removed: z.array(z.object({ connected: z.object({ piece: z.string() }), connecting: z.object({ piece: z.string() }) })).optional(),
  updated: z.array(z.object({ id: z.object({ connected: z.object({ piece: z.string() }), connecting: z.object({ piece: z.string() }) }), diff: ConnectionDiffSchema })).optional(),
  added: z.array(ConnectionSchema).optional(),
});
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export const serializeConnection = (connection: Connection): string => JSON.stringify(ConnectionSchema.parse(connection));
export const deserializeConnection = (json: string): Connection => ConnectionSchema.parse(JSON.parse(json));

export const areSameConnection = (connection: Connection | ConnectionDiff, other: Connection | ConnectionDiff, strict: boolean = false): boolean => {
  const getConnectedPieceId = (conn: typeof connection) => ("connected" in conn && conn.connected && "piece" in conn.connected ? (typeof conn.connected.piece === "string" ? conn.connected.piece : conn.connected.piece?.guid ?? "") : "");
  const getConnectingPieceId = (conn: typeof connection) => ("connecting" in conn && conn.connecting && "piece" in conn.connecting ? (typeof conn.connecting.piece === "string" ? conn.connecting.piece : conn.connecting.piece?.guid ?? "") : "");

  const connectedPiece1 = getConnectedPieceId(connection);
  const connectingPiece1 = getConnectingPieceId(connection);
  const connectedPiece2 = getConnectedPieceId(other);
  const connectingPiece2 = getConnectingPieceId(other);

  const isExactMatch = connectingPiece1 === connectingPiece2 && connectedPiece1 === connectedPiece2;
  if (strict) return isExactMatch;
  const isSwappedMatch = connectingPiece1 === connectedPiece2 && connectedPiece1 === connectingPiece2;
  return isExactMatch || isSwappedMatch;
};

export const findConnection = (connections: Connection[], connectionGuid: string): Connection => {
  const connection = connections.find((c) => c.guid === connectionGuid);
  if (!connection) throw new Error(`Connection ${connectionGuid} not found in connections`);
  return connection;
};

export const findPieceConnections = (connections: Connection[], pieceGuid: string): Connection[] => {
  return connections.filter((c) => c.connected.piece.guid === pieceGuid || c.connecting.piece.guid === pieceGuid);
};

export const findPortForPieceInConnection = (type: Type, connection: Connection, pieceGuid: string): Port => {
  const portGuid = connection.connected.piece.guid === pieceGuid ? connection.connected.port.guid : connection.connecting.port.guid;
  return findPortInType(type, portGuid);
};

// #endregion Connection

// #region Stat
// https://github.com/usalu/semio#-stat-

export const StatSchema = z.object({
  guid: z.string(),
  quality: QualityIdSchema,
  unit: z.string().optional(),
  min: z.number().optional(),
  minExcluded: z.boolean().optional(),
  max: z.number().optional(),
  maxExcluded: z.boolean().optional(),
});
export type Stat = z.infer<typeof StatSchema>;
export const StatDiffSchema = StatSchema.partial();
export type StatDiff = z.infer<typeof StatDiffSchema>;
export const StatsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: StatDiffSchema })).optional(),
  added: z.array(StatSchema).optional(),
});
export const serializeStat = (stat: Stat): string => JSON.stringify(StatSchema.parse(stat));
export const deserializeStat = (json: string): Stat => StatSchema.parse(JSON.parse(json));

// #endregion Stat

// #region Design
// https://github.com/usalu/semio#-design-

export const DesignSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: DesignIdSchema.optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  pieces: z.array(PieceSchema).optional(),
  connections: z.array(ConnectionSchema).optional(),
  stats: z.array(StatSchema).optional(),
  props: z.array(PropSchema).optional(),
  layers: z.array(LayerSchema).optional(),
  activeLayer: LayerIdSchema.optional(),
  groups: z.array(GroupSchema).optional(),
  canScale: z.boolean().optional(),
  canMirror: z.boolean().optional(),
  unit: z.string().optional(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(z.string()).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type Design = z.infer<typeof DesignSchema>;
export const serializeDesign = (design: Design): string => JSON.stringify(DesignSchema.parse(design));
export const deserializeDesign = (json: string): Design => DesignSchema.parse(JSON.parse(json));

export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true }).extend({
  pieces: z.array(z.string()).optional(),
  connections: z.array(z.string()).optional(),
  stats: z.array(z.string()).optional(),
});

export type DesignShallow = z.infer<typeof DesignShallowSchema>;
export const serializeDesignShallow = (design: DesignShallow): string => JSON.stringify(DesignShallowSchema.parse(design));
export const deserializeDesignShallow = (json: string): DesignShallow => DesignShallowSchema.parse(JSON.parse(json));
export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).partial().extend({
  pieces: PiecesDiffSchema.optional(),
  connections: ConnectionsDiffSchema.optional(),
  stats: StatsDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  layers: LayersDiffSchema.optional(),
  groups: GroupsDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});

export type DesignDiff = z.infer<typeof DesignDiffSchema>;
export const getDesignDiff = (before: Design, after: Design): DesignDiff => {
  // TODO: Implement full Design diff logic
  return {};
};
export const mergeDesignDiff = (diff1: DesignDiff, diff2: DesignDiff): DesignDiff => {
  // TODO: Implement full Design merge diff logic
  return { ...diff1, ...diff2 };
};
export const inverseDesignDiff = (original: Design, appliedDiff: DesignDiff): DesignDiff => {
  // TODO: Implement full Design inverse diff logic
  return {};
};

export const addPieceToDesignDiff = (designDiff: any, piece: Piece): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), piece],
    },
  };
};
export const setPieceInDesignDiff = (designDiff: any, pieceDiff: { id_: string; diff: PieceDiff }): any => {
  const existingIndex = (designDiff.pieces?.updated || []).findIndex((p: { id_: string; diff: PieceDiff }) => p.id_ === pieceDiff.id_);
  const updated = [...(designDiff.pieces?.updated || [])];
  if (existingIndex >= 0) {
    updated[existingIndex] = pieceDiff;
  } else {
    updated.push(pieceDiff);
  }
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } };
};

export const removePieceFromDesignDiff = (designDiff: any, pieceId: string): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      removed: [...(designDiff.pieces?.removed || []), pieceId],
    },
  };
};

export const addPiecesToDesignDiff = (designDiff: any, pieces: Piece[]): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), ...pieces],
    },
  };
};
export const setPiecesInDesignDiff = (designDiff: any, pieceDiffs: { id_: string; diff: PieceDiff }[]): any => {
  const updated = [...(designDiff.pieces?.updated || [])];
  pieceDiffs.forEach((pieceDiff: { id_: string; diff: PieceDiff }) => {
    const existingIndex = updated.findIndex((p: { id_: string; diff: PieceDiff }) => p.id_ === pieceDiff.id_);
    if (existingIndex >= 0) {
      updated[existingIndex] = pieceDiff;
    } else {
      updated.push(pieceDiff);
    }
  });
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } };
};

export const removePiecesFromDesignDiff = (designDiff: any, pieceIds: string[]): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      removed: [...(designDiff.pieces?.removed || []), ...pieceIds],
    },
  };
};

export const addConnectionToDesignDiff = (designDiff: any, connection: Connection): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      added: [...(designDiff.connections?.added || []), connection],
    },
  };
};
export const setConnectionInDesignDiff = (designDiff: any, connectionDiff: ConnectionDiff): any => {
  const existingIndex = (designDiff.connections?.updated || []).findIndex((c: ConnectionDiff) => areSameConnection(c, connectionDiff));
  const updated = [...(designDiff.connections?.updated || [])];
  if (existingIndex >= 0) {
    updated[existingIndex] = connectionDiff;
  } else {
    updated.push(connectionDiff);
  }
  return { ...designDiff, connections: { ...designDiff.connections, updated } };
};
export const removeConnectionFromDesignDiff = (designDiff: any, connectionId: { connected: { piece: string }; connecting: { piece: string } }): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), connectionId],
    },
  };
};

export const addConnectionsToDesignDiff = (designDiff: any, connections: Connection[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      added: [...(designDiff.connections?.added || []), ...connections],
    },
  };
};
export const setConnectionsInDesignDiff = (designDiff: any, connectionDiffs: ConnectionDiff[]): any => {
  const updated = [...(designDiff.connections?.updated || [])];
  connectionDiffs.forEach((connectionDiff: ConnectionDiff) => {
    const existingIndex = updated.findIndex((c: ConnectionDiff) => areSameConnection(c, connectionDiff));
    if (existingIndex >= 0) {
      updated[existingIndex] = connectionDiff;
    } else {
      updated.push(connectionDiff);
    }
  });
  return { ...designDiff, connections: { ...designDiff.connections, updated } };
};
export const removeConnectionsFromDesignDiff = (designDiff: any, connectionIds: string[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), ...connectionIds],
    },
  };
};

export const applyDesignDiff = (base: Design, diff: DesignDiff): Design => {
  let result = { ...base };
  if (diff.name !== undefined) result.name = diff.name;
  if (diff.parent !== undefined) result.parent = diff.parent;
  if (diff.isAbstract !== undefined) result.isAbstract = diff.isAbstract;
  if (diff.folder !== undefined) result.folder = diff.folder;
  if (diff.canScale !== undefined) result.canScale = diff.canScale;
  if (diff.canMirror !== undefined) result.canMirror = diff.canMirror;
  if (diff.unit !== undefined) result.unit = diff.unit;
  if (diff.location !== undefined) result.location = diff.location;
  if (diff.icon !== undefined) result.icon = diff.icon;
  if (diff.image !== undefined) result.image = diff.image;
  if (diff.description !== undefined) result.description = diff.description;
  if (diff.createdAt !== undefined) result.createdAt = diff.createdAt;
  if (diff.updatedAt !== undefined) result.updatedAt = diff.updatedAt;
  if (diff.pieces) {
    let pieces = [...(result.pieces ?? [])];
    if (diff.pieces.removed) {
      pieces = pieces.filter((p) => !diff.pieces!.removed!.includes(p.guid));
    }
    if (diff.pieces.updated) {
      pieces = pieces.map((piece) => {
        const update = diff.pieces!.updated!.find((u) => u.id === piece.guid);
        return update ? applyPieceDiff(piece, update.diff) : piece;
      });
    }
    if (diff.pieces.added) {
      pieces.push(...diff.pieces.added);
    }
    result.pieces = pieces;
  }
  if (diff.connections) {
    let connections = [...(result.connections ?? [])];
    if (diff.connections.removed) {
      connections = connections.filter((c) => !diff.connections!.removed!.some((removed) => areSameConnection(c, removed as any)));
    }
    if (diff.connections.updated) {
      connections = connections.map((connection) => {
        const update = diff.connections!.updated!.find((u) => areSameConnection(connection, u.id as any));
        return update ? applyConnectionDiff(connection, update.diff) : connection;
      });
    }
    if (diff.connections.added) {
      connections.push(...diff.connections.added);
    }
    result.connections = connections;
  }
  if (diff.attributes) {
    result.attributes = applyAttributesDiff(result.attributes ?? [], diff.attributes);
  }
  return result;
};

export const DesignsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: DesignDiffSchema })).optional(),
  added: z.array(DesignSchema).optional(),
});
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;

export const mergeDesigns = (designs: Design[]): DesignDiff => {
  const pieces = designs.flatMap((d) => d.pieces ?? []);
  const connections = designs.flatMap((d) => d.connections ?? []);

  return {
    pieces: pieces.length > 0 ? { added: pieces } : undefined,
    connections: connections.length > 0 ? { added: connections } : undefined,
  };
};

export const orientDesign = (plane?: Plane, center?: Coord): DesignDiff => {
  if (plane === undefined && center === undefined) {
    return {};
  }

  // This function would need the current design state to determine which pieces are fixed
  // For now, return an empty diff as this function needs additional context
  // In practice, this would be used with the current design state
  return {};
};

export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: string, pieceIds: string[], connectionIds: string[]): DesignDiff => {
  const design = kit.designs?.find((d) => d.guid === designId);
  const removedConnections = connectionIds
    .map((connId) => {
      const conn = design?.connections?.find((c) => c.guid === connId);
      if (!conn) return null;
      return { connected: { piece: conn.connected.piece.guid }, connecting: { piece: conn.connecting.piece.guid } };
    })
    .filter((c): c is { connected: { piece: string }; connecting: { piece: string } } => c !== null);

  return {
    pieces: {
      removed: pieceIds,
    },
    connections: {
      removed: removedConnections,
    },
  };
};

const computeChildPlane = (parentPlane: Plane, parentPort: Port, childPort: Port, connection: Connection): Plane => {
  const parentMatrix = planeToMatrix(parentPlane);
  const parentPoint = vectorToThree(parentPort.point);
  const parentDirection = vectorToThree(parentPort.direction).normalize();
  const childPoint = vectorToThree(childPort.point);
  const childDirection = vectorToThree(childPort.direction).normalize();

  const { gap, shift, rise, rotation, turn, tilt } = connection;
  const rotationRad = THREE.MathUtils.degToRad(rotation ?? 0);
  const turnRad = THREE.MathUtils.degToRad(turn ?? 0);
  const tiltRad = THREE.MathUtils.degToRad(tilt ?? 0);

  const reverseChildDirection = childDirection.clone().negate();

  let alignQuat: THREE.Quaternion;
  if (new THREE.Vector3().crossVectors(parentDirection, reverseChildDirection).length() < 0.01) {
    // Parallel vectors
    // Idea taken from: // https://github.com/dfki-ric/pytransform3d/blob/143943b028fc776adfc6939b1d7c2c6edeaa2d90/pytransform3d/rotations/_utils.py#L253
    if (Math.abs(parentDirection.z) < TOLERANCE) {
      alignQuat = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 0, 1), Math.PI); // 180* around z axis
    } else {
      // 180* around cross product of z and parentDirection
      const axis = new THREE.Vector3(0, 0, 1).cross(parentDirection).normalize();
      alignQuat = new THREE.Quaternion().setFromAxisAngle(axis, Math.PI);
    }
  } else {
    alignQuat = new THREE.Quaternion().setFromUnitVectors(reverseChildDirection, parentDirection);
  }

  const directionT = new THREE.Matrix4().makeRotationFromQuaternion(alignQuat);

  const yAxis = new THREE.Vector3(0, 1, 0);
  const parentPortQuat = new THREE.Quaternion().setFromUnitVectors(yAxis, parentDirection);
  const parentRotationT = new THREE.Matrix4().makeRotationFromQuaternion(parentPortQuat);

  const gapDirection = new THREE.Vector3(0, 1, 0).applyMatrix4(parentRotationT);
  const shiftDirection = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT);
  const raiseDirection = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT);
  const turnAxis = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT);
  const tiltAxis = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT);

  let orientationT = directionT.clone();

  const rotateT = new THREE.Matrix4().makeRotationAxis(parentDirection, -rotationRad);
  orientationT.premultiply(rotateT);

  turnAxis.applyMatrix4(rotateT);
  tiltAxis.applyMatrix4(rotateT);

  const turnT = new THREE.Matrix4().makeRotationAxis(turnAxis, turnRad);
  orientationT.premultiply(turnT);

  const tiltT = new THREE.Matrix4().makeRotationAxis(tiltAxis, tiltRad);
  orientationT.premultiply(tiltT);

  const centerChildT = new THREE.Matrix4().makeTranslation(-childPoint.x, -childPoint.y, -childPoint.z);
  let transform = new THREE.Matrix4().multiplyMatrices(orientationT, centerChildT);

  const gapTransform = new THREE.Matrix4().makeTranslation(gapDirection.x * (gap ?? 0), gapDirection.y * (gap ?? 0), gapDirection.z * (gap ?? 0));
  const shiftTransform = new THREE.Matrix4().makeTranslation(shiftDirection.x * (shift ?? 0), shiftDirection.y * (shift ?? 0), shiftDirection.z * (shift ?? 0));
  const raiseTransform = new THREE.Matrix4().makeTranslation(raiseDirection.x * (rise ?? 0), raiseDirection.y * (rise ?? 0), raiseDirection.z * (rise ?? 0));

  const translationT = raiseTransform.clone().multiply(shiftTransform).multiply(gapTransform);
  transform.premultiply(translationT);
  const moveToParentT = new THREE.Matrix4().makeTranslation(parentPoint.x, parentPoint.y, parentPoint.z);
  transform.premultiply(moveToParentT);
  const finalMatrix = new THREE.Matrix4().multiplyMatrices(parentMatrix, transform);

  return matrixToPlane(finalMatrix);
};
export const flattenDesign = (kit: Kit, designId: string): DesignDiff => {
  const design = findDesignInKit(kit, designId);
  if (!design) {
    throw new Error(`Design ${designId} not found in kit ${kit.name}`);
  }
  const types = kit.types ?? [];

  if (!design.pieces || design.pieces.length === 0) return {};

  const typesDict: { [key: string]: Type } = {};
  types.forEach((t) => {
    typesDict[t.guid] = t;
  });
  const getType = (typeGuid: string): Type | undefined => {
    return typesDict[typeGuid];
  };
  const getPort = (type: Type | undefined, portGuid: string | undefined): Port | undefined => {
    if (!type) return undefined;

    // If no port GUID specified, return first port
    if (!portGuid) {
      if (type.ports && type.ports.length > 0) {
        return type.ports[0];
      }
      // Check parent
      if (type.parent?.guid) {
        const parentType = getType(type.parent.guid);
        return getPort(parentType, portGuid);
      }
      return undefined;
    }

    // Port GUID specified - try to find it in type hierarchy
    if (type.ports && type.ports.length > 0) {
      const port = type.ports.find((p) => p.guid === portGuid);
      if (port) return port;
    }

    // Not found in current type, check parent
    if (type.parent?.guid) {
      const parentType = getType(type.parent.guid);
      const port = getPort(parentType, portGuid);
      if (port) return port;
    }

    // Port GUID specified but not found anywhere in hierarchy - fall back to first port
    if (type.ports && type.ports.length > 0) {
      return type.ports[0];
    }

    return undefined;
  };

  const flatDesign: Design = JSON.parse(JSON.stringify(design));
  if (!flatDesign.pieces) flatDesign.pieces = [];

  const piecePlanes: { [pieceGuid: string]: Plane } = {};
  const pieceMap: { [pieceGuid: string]: Piece } = {};
  flatDesign.pieces!.forEach((p) => {
    if (p.guid) pieceMap[p.guid] = p;
  });


  const filteredConnections = flatDesign.connections?.filter((connection) => {
    const sourceId = connection.connected.piece.guid;
    const targetId = connection.connecting.piece.guid;
    const sourceExists = pieceMap[sourceId];
    const targetExists = pieceMap[targetId];
    if (!sourceExists) {
      console.warn(`[ORIGIN] flattenDesign: Skipping connection ${connection.guid} - source piece ${sourceId} not found`);
      return false;
    }
    if (!targetExists) {
      console.warn(`[ORIGIN] flattenDesign: Skipping connection ${connection.guid} - target piece ${targetId} not found`);
      return false;
    }
    return true;
  }) || [];


  const cy = cytoscape({
    elements: {
      nodes: flatDesign.pieces!.map((piece) => ({
        data: { id: piece.guid, label: piece.guid },
      })),
      edges: filteredConnections.map((connection, index) => {
        const sourceId = connection.connected.piece.guid;
        const targetId = connection.connecting.piece.guid;
        return {
          data: {
            id: connection.guid,
            source: sourceId,
            target: targetId,
            connectionData: connection,
          },
        };
      }),
    } as any,
    headless: true,
  });

  const components = cy.elements().components();
  let isFirstRoot = true;

  // Helper to add or update attributes on a piece
  const setAttributes = (piece: Piece, newAttrs: { key: string; value?: string; definition?: string }[]): Piece => {
    const existingAttrs = piece.attributes || [];
    const updatedAttrs = [...existingAttrs];
    newAttrs.forEach((newAttr) => {
      const existingIndex = updatedAttrs.findIndex((a) => a.key === newAttr.key);
      if (existingIndex >= 0) {
        updatedAttrs[existingIndex] = { ...updatedAttrs[existingIndex], ...newAttr, guid: updatedAttrs[existingIndex].guid };
      } else {
        updatedAttrs.push({ guid: guid(), ...newAttr });
      }
    });
    return { ...piece, attributes: updatedAttrs };
  };

  components.forEach((component) => {
    let roots = component.nodes().filter((node) => {
      const piece = pieceMap[node.id()];
      return piece?.plane !== undefined;
    });
    let rootNode = roots.length > 0 ? roots[0] : component.nodes().length > 0 ? component.nodes()[0] : undefined;
    if (!rootNode) return;
    const rootPiece = pieceMap[rootNode.id()];
    if (!rootPiece || !rootPiece.guid) return;
    const updatedRootPiece = setAttributes(rootPiece, [
      { key: "semio.fixedPieceId", value: rootPiece.guid },
      { key: "semio.depth", value: "0" },
    ]);
    pieceMap[rootNode.id()] = updatedRootPiece;
    let rootPlane: Plane;
    if (rootPiece.plane) {
      rootPlane = rootPiece.plane;
    } else if (isFirstRoot) {
      const identityMatrix = new THREE.Matrix4().identity();
      rootPlane = matrixToPlane(identityMatrix);
      isFirstRoot = false;
    } else {
      console.warn(`Root piece ${rootPiece.guid} has no defined plane and is not the first root. Defaulting to identity plane.`);
      const identityMatrix = new THREE.Matrix4().identity();
      rootPlane = matrixToPlane(identityMatrix);
    }

    piecePlanes[rootPiece.guid] = rootPlane;
    const rootPieceIndex = flatDesign.pieces!.findIndex((p) => p.guid === rootPiece.guid);
    if (rootPieceIndex !== -1) {
      flatDesign.pieces![rootPieceIndex].plane = rootPlane;
      // Ensure root piece has a center (default to origin if not set)
      if (!flatDesign.pieces![rootPieceIndex].center) {
        flatDesign.pieces![rootPieceIndex].center = { u: 0, v: 0 };
      }
    }

    let visitCount = 0;
    let skipCount = 0;
    const bfs = component.bfs({
      roots: `#${rootNode.id()}`,
      visit: (v, e, u, i, depth) => {
        if (!e) return;
        visitCount++;
        const edgeData = e.data();
        const connection: Connection | undefined = edgeData.connectionData;
        if (!connection) {
          skipCount++;
          return;
        }
        const parentNode = u;
        const childNode = v;
        const parentId = parentNode.id();
        const childId = childNode.id();
        const parentPiece = pieceMap[parentId];
        const childPiece = pieceMap[childId];
        if (!parentPiece || !childPiece || !parentPiece.guid || !childPiece.guid) {
          skipCount++;
          return;
        }
        if (piecePlanes[childPiece.guid]) return;
        const parentPlane = piecePlanes[parentPiece.guid];
        if (!parentPlane) {
          console.error(`Error during flatten: Parent piece ${parentPiece.guid} plane not found.`);
          skipCount++;
          return;
        }
        const parentSide = connection.connected.piece.guid === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.guid === childId ? connection.connecting : connection.connected;
        const parentType = parentPiece.type ? getType(parentPiece.type.guid) : undefined;
        const childType = childPiece.type ? getType(childPiece.type.guid) : undefined;

        // Get ports - use recursive parent type lookup via getPort
        // If no explicit port GUID, getPort will return the first available port
        const parentPortGuid = parentSide.port?.guid;
        const childPortGuid = childSide.port?.guid;
        const parentPort = getPort(parentType, parentPortGuid);
        const childPort = getPort(childType, childPortGuid);

        if (!parentPort || !childPort) {
          console.error(`Error during flatten: Ports not found for connection between ${parentId} and ${childId}. Parent Port: ${parentPortGuid}, Child Port: ${childPortGuid}`);
          skipCount++;
          return;
        }
        const childPlane = roundPlane(computeChildPlane(parentPlane, parentPort, childPort, connection));
        piecePlanes[childPiece.guid] = childPlane;

        // Ensure parent has a center (default to origin if not set)
        const parentCenter = parentPiece.center || { u: 0, v: 0 };

        const childCenter = {
          u: round(parentCenter.u + (connection.x ?? 0)),
          v: round(parentCenter.v + (connection.y ?? 0)),
        };

        const flatChildPiece: Piece = setAttributes(
          {
            ...childPiece,
            plane: childPlane,
            center: childCenter,
          },
          [
            {
              key: "semio.fixedPieceId",
              value: parentPiece.attributes?.find((q) => q.key === "semio.fixedPieceId")?.value ?? "",
            },
            {
              key: "semio.parentPieceId",
              value: parentPiece.guid,
            },
            {
              key: "semio.depth",
              value: depth.toString(),
            },
          ],
        );
        pieceMap[childId] = flatChildPiece;
      },
      directed: false,
    });
  });
  flatDesign.pieces = flatDesign.pieces?.map((p) => pieceMap[p.guid ?? ""]);
  flatDesign.connections = [];

  // Return the diff between original design and flattened design
  let piecesWithPlanes = 0;
  let piecesWithoutPlanes = 0;
  const updatedPieces = flatDesign.pieces
    ?.map((flatPiece) => {
      if (flatPiece.plane) piecesWithPlanes++;
      else piecesWithoutPlanes++;

      const originalPiece = design.pieces?.find((p) => p.guid === flatPiece.guid);
      if (!originalPiece) return null;

      // Build piece diff for pieces that changed
      const pieceDiff: PieceDiff = {};
      // Always include plane and center from flatPiece (these are what flattenDesign computed)
      if (flatPiece.plane) pieceDiff.plane = flatPiece.plane;
      if (flatPiece.center) pieceDiff.center = flatPiece.center;
      if (JSON.stringify(flatPiece.attributes) !== JSON.stringify(originalPiece.attributes)) {
        pieceDiff.attributes = getAttributesDiff(originalPiece.attributes ?? [], flatPiece.attributes ?? []);
      }

      // Only return diff if there are changes
      if (Object.keys(pieceDiff).length === 0) return null;

      return {
        id: flatPiece.guid,
        diff: pieceDiff,
      };
    })
    .filter((update) => update !== null) as Array<{ id: string; diff: PieceDiff }>;


  const removedConnections = design.connections?.map((c) => ({ connected: { piece: c.connected.piece.guid }, connecting: { piece: c.connecting.piece.guid } })) || [];

  return {
    pieces: updatedPieces.length > 0 ? { updated: updatedPieces } : undefined,
    connections: removedConnections.length > 0 ? { removed: removedConnections } : undefined,
  } as DesignDiff;
};

/**
 * Creates a clustered design from a cluster of pieces and connections
 * @param originalDesign - The original design containing the pieces to cluster
 * @param clusterPieceIds - The IDs of pieces to include in the clustered design
 * @param designName - Name for the new design
 * @returns Object containing the clustered design and external connections
 */
export const createClusteredDesign = (originalDesign: Design, clusterPieceIds: string[], designName: string): { clusteredDesign: Design; externalConnections: Connection[] } => {
  // Validate inputs
  if (!originalDesign.pieces || originalDesign.pieces.length === 0) {
    throw new Error("Original design has no pieces to cluster");
  }
  if (!clusterPieceIds || clusterPieceIds.length === 0) {
    throw new Error("No piece IDs provided for clustering");
  }

  // Extract clustered pieces and their connections
  const clusteredPieces = (originalDesign.pieces || []).filter((piece) => clusterPieceIds.includes(piece.guid));

  if (clusteredPieces.length === 0) {
    throw new Error("No pieces found matching the provided IDs");
  }

  // Find internal connections (both pieces in cluster)
  const internalConnections = (originalDesign.connections || []).filter((connection) => clusterPieceIds.includes(connection.connected.piece.guid) && clusterPieceIds.includes(connection.connecting.piece.guid));

  // Find external connections (one piece in cluster, one outside)
  const externalConnections = (originalDesign.connections || []).filter((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);
    return connectedInCluster !== connectingInCluster; // XOR - exactly one is in cluster
  });

  // Create the clustered design
  const clusteredDesign: Design = {
    guid: guid(),
    name: designName,
    unit: originalDesign.unit,
    description: `Clustered design with ${clusteredPieces.length} pieces`,
    pieces: clusteredPieces,
    connections: internalConnections,
    createdAt: new Date(),
    updatedAt: new Date(),
  };

  return { clusteredDesign, externalConnections };
};

/**
 * Replaces clustered pieces with direct design references in connections
 * @param originalDesign - The original design
 * @param clusterPieceIds - IDs of pieces to remove and cluster
 * @param clusteredDesign - The clustered design to include
 * @param externalConnections - External connections to update
 * @returns Updated design with clustered pieces removed and direct design references
 */
export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds: string[], clusteredDesign: Design, externalConnections: Connection[]): DesignDiff => {
  // Remove clustered pieces
  const piecesToRemove = clusterPieceIds;

  // Remove all connections involving clustered pieces
  const connectionsToRemove = (originalDesign.connections || [])
    .filter((connection) => {
      const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
      const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);
      return connectedInCluster || connectingInCluster;
    })
    .map((c) => ({ connected: { piece: c.connected.piece.guid }, connecting: { piece: c.connecting.piece.guid } }));

  // Update external connections to use direct design references
  const updatedExternalConnections = externalConnections.map((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);

    if (connectedInCluster) {
      // Keep original piece guid but add designPiece to reference the nested design
      return {
        ...connection,
        connected: {
          ...connection.connected,
          designPiece: { guid: connection.connected.piece.guid }, // Reference to the piece within nested design
        },
      };
    } else if (connectingInCluster) {
      // Keep original piece guid but add designPiece to reference the nested design
      return {
        ...connection,
        connecting: {
          ...connection.connecting,
          designPiece: { guid: connection.connecting.piece.guid }, // Reference to the piece within nested design
        },
      };
    }

    return connection;
  });

  return {
    pieces: {
      removed: piecesToRemove,
    },
    connections: {
      removed: connectionsToRemove,
      added: updatedExternalConnections,
    },
  };
};

/**
 * Expands design pieces by replacing them with their constituent pieces and connections
 * @param design - The design to expand
 * @param kit - The kit containing type information
 * @returns Design with design pieces expanded
 */
export const getClusterableGroups = (design: Design, selectedPieceIds: string[]): string[][] => {
  if (selectedPieceIds.length < 2) return []; // Need at least 2 items to cluster

  // Build adjacency map from all connections
  const adjacencyMap = new Map<string, Set<string>>();
  (design.connections || []).forEach((connection) => {
    const sourceId = connection.connecting.piece.guid;
    const targetId = connection.connected.piece.guid;

    if (!adjacencyMap.has(sourceId)) adjacencyMap.set(sourceId, new Set());
    if (!adjacencyMap.has(targetId)) adjacencyMap.set(targetId, new Set());

    adjacencyMap.get(sourceId)!.add(targetId);
    adjacencyMap.get(targetId)!.add(sourceId);
  });

  // Find connected components using DFS
  const visited = new Set<string>();
  const connectedGroups: string[][] = [];

  const dfs = (pieceId: string, currentGroup: string[]) => {
    if (visited.has(pieceId)) return;
    visited.add(pieceId);
    currentGroup.push(pieceId);

    const neighbors = adjacencyMap.get(pieceId) || new Set();
    for (const neighbor of Array.from(neighbors)) {
      if (selectedPieceIds.includes(neighbor) && !visited.has(neighbor)) {
        dfs(neighbor, currentGroup);
      }
    }
  };

  // First, find all connected components
  for (const pieceId of selectedPieceIds) {
    if (!visited.has(pieceId)) {
      const group: string[] = [];
      dfs(pieceId, group);
      connectedGroups.push(group);
    }
  }

  // If we have multiple connected components OR design nodes in selection,
  // allow clustering the entire selection as one group
  const hasDesignNodes = selectedPieceIds.some((id) => id.startsWith("design-"));
  const hasMultipleComponents = connectedGroups.length > 1;
  const hasLargeConnectedGroup = connectedGroups.some((group) => group.length > 1);

  if (hasDesignNodes || hasMultipleComponents || hasLargeConnectedGroup) {
    // Return all selected pieces as one clusterable group
    return [selectedPieceIds];
  }

  return [];
};

export const expandDesignPieces = (design: Design, kit: Kit): Design => {
  // Check if there are any connections with designPiece (indicating clustered pieces)
  const hasDesignConnections = design.connections?.some((conn) => conn.connected.designPiece || conn.connecting.designPiece);
  if (!hasDesignConnections) {
    return design; // No design connections to expand
  }

  let expandedDesign = { ...design };

  // Find all unique designIds referenced in connections
  const designIds = new Set<string>();
  toArray(design.connections).forEach((conn) => {
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.guid);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.guid);
  });

  if (designIds.size === 0) {
    return expandedDesign; // No design references found
  }

  // For each referenced design, expand it
  for (const designName of Array.from(designIds)) {
    // Find the design in the kit
    const referencedDesign = findDesignInKit(kit, designName);
    if (!referencedDesign) continue;

    // Recursively expand the referenced design first
    const expandedReferencedDesign = expandDesignPieces(referencedDesign, kit);

    // For design connections, use the original pieces and connections without namespacing
    const transformedPieces = (expandedReferencedDesign.pieces || []).map((piece) => ({
      ...piece,
      center: piece.center || { u: 0, v: 0 },
    }));

    const transformedConnections = expandedReferencedDesign.connections || [];

    const updatedExternalConnections = (expandedDesign.connections || []).map((connection) => {
      if (connection.connected.designPiece?.guid === designName) {
        return {
          ...connection,
          connected: {
            ...connection.connected,
            designPiece: undefined,
          },
        };
      }

      if (connection.connecting.designPiece?.guid === designName) {
        // Use the original piece ID directly (no namespacing)
        return {
          ...connection,
          connecting: {
            ...connection.connecting,
            designPiece: undefined, // Remove designPiece since we've expanded
          },
        };
      }

      return connection;
    });

    // Add expanded pieces and update connections
    expandedDesign = {
      ...expandedDesign,
      pieces: [...(expandedDesign.pieces || []), ...transformedPieces],
      connections: [...updatedExternalConnections, ...transformedConnections],
    };
  }

  return expandedDesign;
};

export type IncludedDesignInfo = {
  guid: string;
  designGuid: string;
  type: "connected" | "fixed";
  center?: Coord;
  plane?: Plane;
  externalConnections?: Connection[];
};

export const getIncludedDesigns = (design: Design): IncludedDesignInfo[] => {
  const includedDesigns: IncludedDesignInfo[] = [];

  // Get designs from external connections (clustered designs)
  const designIds = new Set<string>();
  toArray(design.connections).forEach((conn: Connection) => {
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.guid);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.guid);
  });

  // Add connected designs
  Array.from(designIds).forEach((designIdString) => {
    const externalConnections =
      design.connections?.filter((connection: Connection) => {
        const connectedToDesign = connection.connected.designPiece?.guid === designIdString;
        const connectingToDesign = connection.connecting.designPiece?.guid === designIdString;
        return connectedToDesign || connectingToDesign;
      }) ?? [];

    includedDesigns.push({
      guid: guid(),
      designGuid: designIdString,
      type: "connected",
      externalConnections,
    });
  });

  return includedDesigns;
};

export const isPortInUse = (design: Design, pieceGuid: string, portGuid: string): boolean => {
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.guid === pieceGuid;
    const isPortConnected = isPieceConnected ? connection.connected.port.guid === portGuid : connection.connecting.port.guid === portGuid;
    if (isPortConnected) return true;
  }
  return false;
};

export const isConnectionInDesign = (design: Design, connection: Connection): boolean => {
  return design.connections?.some((c) => areSameConnection(c, connection)) ?? false;
};

export const findPieceInDesign = (design: Design, pieceGuid: string): Piece => findPiece(design.pieces ?? [], pieceGuid);

export const findConnectionInDesign = (design: Design, connectionGuid: string): Connection => {
  return findConnection(design.connections ?? [], connectionGuid);
};

export const findConnectionsInDesign = (design: Design, connectionGuids: string[]): Connection[] => {
  return connectionGuids.map((connectionGuid) => findConnectionInDesign(design, connectionGuid));
};

export const findPieceConnectionsInDesign = (design: Design, pieceGuid: string): Connection[] => {
  return findPieceConnections(design.connections ?? [], pieceGuid);
};

export const findConnectionPiecesInDesign = (design: Design, connection: Connection): { connecting: Piece; connected: Piece } => {
  return {
    connected: findPieceInDesign(design, connection.connected.piece.guid),
    connecting: findPieceInDesign(design, connection.connecting.piece.guid),
  };
};

export const findStaleConnectionsInDesign = (design: Design): Connection[] => {
  return (
    design.connections?.filter((c) => {
      try {
        findPieceInDesign(design, c.connected.piece.guid);
        findPieceInDesign(design, c.connecting.piece.guid);
        return false;
      } catch (e) {
        return true;
      }
    }) ?? []
  );
};

// #endregion Design

// #region Kit

// https://github.com/usalu/semio#-kit-
export const KitSchema = z.object({
  guid: z.string(),
  name: z.string(),
  version: z.string().optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  interfaces: z.array(InterfaceSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  files: z.array(FileSchema).optional(),
  folders: z.array(FolderSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
  remote: z.string().optional(),
  homepage: z.string().optional(),
  license: z.string().optional(),
  preview: z.string().optional(),
  concepts: z.array(z.string()).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type Kit = z.infer<typeof KitSchema>;
export const serializeKit = (kit: Kit): string => JSON.stringify(KitSchema.parse(kit));
export const deserializeKit = (json: string): Kit => KitSchema.parse(JSON.parse(json));

export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, interfaces: true, qualities: true, folders: true, authors: true }).extend({
  types: z.array(z.string()).optional(),
  designs: z.array(z.string()).optional(),
  interfaces: z.array(z.string()).optional(),
  qualities: z.array(z.string()).optional(),
  folders: z.array(z.string()).optional(),
  authors: z.array(z.string()).optional(),
});
export type KitShallow = z.infer<typeof KitShallowSchema>;
export const serializeKitShallow = (kit: KitShallow): string => JSON.stringify(KitShallowSchema.parse(kit));
export const deserializeKitShallow = (json: string): KitShallow => KitShallowSchema.parse(JSON.parse(json));
export const KitDiffSchema = KitSchema.partial().omit({ types: true, designs: true, interfaces: true, qualities: true, authors: true, files: true, folders: true, attributes: true }).extend({
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  interfaces: InterfacesDiffSchema.optional(),
  qualities: QualitiesDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  files: FilesDiffSchema.optional(),
  folders: FoldersDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type KitDiff = z.infer<typeof KitDiffSchema>;
const getCollectionDiff = <T extends { guid: string }, D>(
  before: T[],
  after: T[],
  getItemDiff: (before: T, after: T) => D
): { removed?: string[]; updated?: { id: string; diff: D }[]; added?: T[] } => {
  const diff: { removed?: string[]; updated?: { id: string; diff: D }[]; added?: T[] } = {};
  const beforeGuids = new Set(before.map((i) => i.guid));
  const afterGuids = new Set(after.map((i) => i.guid));
  const removed = before.filter((i) => !afterGuids.has(i.guid)).map((i) => i.guid);
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterGuids.has(i.guid))
    .map((i) => {
      const afterItem = after.find((a) => a.guid === i.guid)!;
      const itemDiff = getItemDiff(i, afterItem);
      return { id: i.guid, diff: itemDiff };
    })
    .filter((u) => Object.keys(u.diff as any).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeGuids.has(i.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};

const inverseCollectionDiff = <T extends { guid: string }, D>(
  original: T[],
  appliedDiff: { removed?: string[]; updated?: { id: string; diff: D }[]; added?: T[] },
  inverseItemDiff: (original: T, appliedDiff: D) => D
): { removed?: string[]; updated?: { id: string; diff: D }[]; added?: T[] } => {
  const inverse: { removed?: string[]; updated?: { id: string; diff: D }[]; added?: T[] } = {};
  if (appliedDiff.removed) inverse.added = original.filter((i) => appliedDiff.removed!.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => i.guid);
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalItem = original.find((i) => i.guid === u.id)!;
      return { id: u.id, diff: inverseItemDiff(originalItem, u.diff) };
    });
  }
  return inverse;
};

const applyCollectionDiff = <T extends { guid: string }, D>(
  base: T[],
  diff: { removed?: string[]; updated?: { id: string; diff: D }[]; added?: T[] } | undefined,
  applyItemDiff: (base: T, diff: D) => T
): T[] => {
  if (!diff) return base;
  let result = [...base];
  if (diff.removed) {
    result = result.filter((i) => !diff.removed!.includes(i.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((i) => i.guid === update.id);
      if (index !== -1) {
        result[index] = applyItemDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

export const getKitDiff = (before: Kit, after: Kit): KitDiff => {
  const diff: KitDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.version !== after.version) diff.version = after.version;
  if (before.description !== after.description) diff.description = after.description;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.remote !== after.remote) diff.remote = after.remote;
  if (before.homepage !== after.homepage) diff.homepage = after.homepage;
  if (before.license !== after.license) diff.license = after.license;
  if (before.preview !== after.preview) diff.preview = after.preview;
  if (!arraysEqual(before.concepts, after.concepts)) diff.concepts = after.concepts;
  const typesDiff = getCollectionDiff(before.types ?? [], after.types ?? [], getTypeDiff);
  if (Object.keys(typesDiff).length > 0) diff.types = typesDiff;
  const designsDiff = getCollectionDiff(before.designs ?? [], after.designs ?? [], getDesignDiff);
  if (Object.keys(designsDiff).length > 0) diff.designs = designsDiff;
  const interfacesDiff = getInterfacesDiff(before.interfaces ?? [], after.interfaces ?? []);
  if (Object.keys(interfacesDiff).length > 0) diff.interfaces = interfacesDiff;
  const qualitiesDiff = getCollectionDiff(before.qualities ?? [], after.qualities ?? [], getQualityDiff);
  if (Object.keys(qualitiesDiff).length > 0) diff.qualities = qualitiesDiff;
  const filesDiff = getCollectionDiff(before.files ?? [], after.files ?? [], getFileDiff);
  if (Object.keys(filesDiff).length > 0) diff.files = filesDiff;
  const foldersDiff = getCollectionDiff(before.folders ?? [], after.folders ?? [], getFolderDiff);
  if (Object.keys(foldersDiff).length > 0) diff.folders = foldersDiff;
  const authorsDiff = getCollectionDiff(before.authors ?? [], after.authors ?? [], getAuthorDiff);
  if (Object.keys(authorsDiff).length > 0) diff.authors = authorsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
export const inverseKitDiff = (original: Kit, appliedDiff: KitDiff): KitDiff => {
  const inverse: KitDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.version !== undefined) inverse.version = original.version;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.image !== undefined) inverse.image = original.image;
  if (appliedDiff.remote !== undefined) inverse.remote = original.remote;
  if (appliedDiff.homepage !== undefined) inverse.homepage = original.homepage;
  if (appliedDiff.license !== undefined) inverse.license = original.license;
  if (appliedDiff.preview !== undefined) inverse.preview = original.preview;
  if (appliedDiff.concepts !== undefined) inverse.concepts = original.concepts;
  if (appliedDiff.types) inverse.types = inverseCollectionDiff(original.types ?? [], appliedDiff.types, inverseTypeDiff);
  if (appliedDiff.designs) inverse.designs = inverseCollectionDiff(original.designs ?? [], appliedDiff.designs, inverseDesignDiff);
  if (appliedDiff.interfaces) inverse.interfaces = inverseInterfacesDiff(original.interfaces ?? [], appliedDiff.interfaces);
  if (appliedDiff.qualities) inverse.qualities = inverseCollectionDiff(original.qualities ?? [], appliedDiff.qualities, inverseQualityDiff);
  if (appliedDiff.files) inverse.files = inverseCollectionDiff(original.files ?? [], appliedDiff.files, inverseFileDiff);
  if (appliedDiff.folders) inverse.folders = inverseCollectionDiff(original.folders ?? [], appliedDiff.folders, inverseFolderDiff);
  if (appliedDiff.authors) inverse.authors = inverseCollectionDiff(original.authors ?? [], appliedDiff.authors, inverseAuthorDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeKitDiff = (diff1: KitDiff, diff2: KitDiff): KitDiff => {
  return {
    ...diff1,
    ...diff2,
    types: diff1.types || diff2.types ? mergeInterfacesDiff(diff1.types ?? {}, diff2.types ?? {}) as any : undefined,
    designs: diff1.designs || diff2.designs ? mergeInterfacesDiff(diff1.designs ?? {}, diff2.designs ?? {}) as any : undefined,
    interfaces: diff1.interfaces || diff2.interfaces ? mergeInterfacesDiff(diff1.interfaces ?? {}, diff2.interfaces ?? {}) : undefined,
    qualities: diff1.qualities || diff2.qualities ? mergeInterfacesDiff(diff1.qualities ?? {}, diff2.qualities ?? {}) as any : undefined,
    files: diff1.files || diff2.files ? mergeInterfacesDiff(diff1.files ?? {}, diff2.files ?? {}) as any : undefined,
    folders: diff1.folders || diff2.folders ? mergeInterfacesDiff(diff1.folders ?? {}, diff2.folders ?? {}) as any : undefined,
    authors: diff1.authors || diff2.authors ? mergeInterfacesDiff(diff1.authors ?? {}, diff2.authors ?? {}) as any : undefined,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
export const applyKitDiff = (base: Kit, diff: KitDiff): Kit => {
  const result: Kit = {
    ...base,
    guid: diff.guid ?? base.guid,
    name: diff.name ?? base.name,
    version: diff.version ?? base.version,
    description: diff.description ?? base.description,
    icon: diff.icon ?? base.icon,
    image: diff.image ?? base.image,
    remote: diff.remote ?? base.remote,
    homepage: diff.homepage ?? base.homepage,
    license: diff.license ?? base.license,
    preview: diff.preview ?? base.preview,
    concepts: diff.concepts ?? base.concepts,
    createdAt: base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
    types: applyCollectionDiff(base.types ?? [], diff.types, applyTypeDiff),
    designs: applyCollectionDiff(base.designs ?? [], diff.designs, applyDesignDiff),
    interfaces: applyInterfacesDiff(base.interfaces ?? [], diff.interfaces ?? {}),
    qualities: applyCollectionDiff(base.qualities ?? [], diff.qualities, applyQualityDiff),
    files: applyCollectionDiff(base.files ?? [], diff.files, applyFileDiff),
    folders: applyCollectionDiff(base.folders ?? [], diff.folders, applyFolderDiff),
    authors: applyCollectionDiff(base.authors ?? [], diff.authors, applyAuthorDiff),
    attributes: applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {}),
  };
  return result;
};

export const KitsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: KitDiffSchema })).optional(),
  added: z.array(KitSchema).optional(),
});

export const addTypeToKit = (type: Type): KitDiff => ({
  types: {
    added: [type],
  },
});
export const setTypeInKit = (type: Type): KitDiff => ({
  types: {
    added: [type],
  },
});
export const removeTypeFromKit = (typeGuid: string): KitDiff => ({
  types: { removed: [typeGuid] },
});

export const addDesignToKit = (design: Design): KitDiff => ({
  designs: {
    added: [design],
  },
});
export const setDesignInKit = (design: Design): KitDiff => ({
  designs: {
    added: [design],
  },
});
export const removeDesignFromKit = (designGuid: string): KitDiff => {
  return {
    designs: {
      removed: [designGuid],
    },
  };
};

export const updateDesignInKit = (design: Design): KitDiff => ({
  designs: {
    added: [design],
  },
});

export const addInterfaceToKit = (iface: Interface): KitDiff => ({
  interfaces: {
    added: [iface],
  },
});
export const setInterfaceInKit = (iface: Interface): KitDiff => ({
  interfaces: {
    added: [iface],
  },
});
export const removeInterfaceFromKit = (interfaceGuid: string): KitDiff => ({
  interfaces: { removed: [interfaceGuid] },
});
export const updateInterfaceInKit = (iface: Interface): KitDiff => ({
  interfaces: {
    added: [iface],
  },
});

export const findFileInKit = (kit: Kit, fileGuid: string): File => {
  const file = (kit.files || []).find((f) => f.guid === fileGuid);
  if (!file) throw new Error(`File ${fileGuid} not found in kit`);
  return file;
};

export const addFileToKit = (file: File): KitDiff => ({ files: { added: [file] } });
export const setFileInKit = (file: File): KitDiff => ({ files: { added: [file] } });
export const removeFileFromKit = (fileGuid: string): KitDiff => ({
  files: { removed: [fileGuid] },
});

export const setAttributeInKit = (attribute: Attribute): KitDiff => ({
  attributes: { added: [attribute] },
});

export const findReplacableDesignsForDesignPiece = (kit: Kit, currentDesignGuid: string, designPiece: Piece): Design[] => {
  if (!designPiece.design) return [];

  const allDesigns = kit.designs || [];
  const currentDesign = findDesignInKit(kit, designPiece.design.guid);

  return allDesigns.filter((design) => {
    if (design.guid === currentDesign.guid) return false;
    if (design.isAbstract) return false;
    return true;
  });
};

export const areSameKit = (kitGuid: string, otherGuid: string): boolean => {
  return kitGuid === otherGuid;
};
export const hasSameKit = (kitGuid: string, otherGuids: string[]): boolean => otherGuids.some((other) => areSameKit(kitGuid, other));

export const findTypeInKit = (kit: Kit, typeGuid: string): Type => {
  const type = kit.types?.find((t) => t.guid === typeGuid);
  if (!type) throw new Error(`Type ${typeGuid} not found in kit ${kit.name}`);
  return type;
};

export const findDesignInKit = (kit: Kit, designGuid: string): Design => {
  const design = kit.designs?.find((d) => d.guid === designGuid);
  if (!design) throw new Error(`Design ${designGuid} not found in kit ${kit.name}`);
  return design;
};

export const findInterfaceInKit = (kit: Kit, interfaceGuid: string): Interface => {
  const iface = kit.interfaces?.find((i) => i.guid === interfaceGuid);
  if (!iface) throw new Error(`Interface ${interfaceGuid} not found in kit ${kit.name}`);
  return iface;
};

export const findPieceTypeInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Type => {
  const piece = findPieceInDesign(findDesignInKit(kit, designGuid), pieceGuid);
  if (!piece.type) throw new Error(`Piece ${pieceGuid} has no type`);
  return findTypeInKit(kit, piece.type.guid);
};

export const findParentPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Piece => {
  const parentPieceId = piecesMetadata(kit, designGuid).get(pieceGuid)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${pieceGuid} has no parent piece`);
  return findPieceInDesign(findDesignInKit(kit, designGuid), parentPieceId);
};

export const findParentConnectionForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Connection => {
  const parentPieceId = piecesMetadata(kit, designGuid).get(pieceGuid)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${pieceGuid} has no parent piece and connection`);
  return findConnectionInDesign(findDesignInKit(kit, designGuid), parentPieceId);
};

export const findChildrenPiecesInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Piece[] => {
  const design = findDesignInKit(kit, designGuid);
  const metadata = piecesMetadata(kit, designGuid);
  const children: Piece[] = [];
  for (const [id, data] of Array.from(metadata)) {
    if (data.parentPieceId === pieceGuid) {
      children.push(findPieceInDesign(design, id));
    }
  }
  return children;
};

export const findUsedPortsByPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Port[] => {
  const design = findDesignInKit(kit, designGuid);
  const piece = findPieceInDesign(design, pieceGuid);
  if (!piece.type) return [];
  const type = findTypeInKit(kit, piece.type.guid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  return connections.map((c) => findPortForPieceInConnection(type, c, pieceGuid));
};

export const findReplacableTypesForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string, variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designGuid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  const requiredPorts: Port[] = [];
  for (const connection of connections) {
    try {
      const otherPieceId = connection.connected.piece.guid === pieceGuid ? connection.connecting.piece.guid : connection.connected.piece.guid;
      const otherPiece = findPieceInDesign(design, otherPieceId);
      if (!otherPiece.type) continue;
      const otherType = findTypeInKit(kit, otherPiece.type.guid);
      const otherPortId = connection.connected.piece.guid === pieceGuid ? connection.connecting.port.guid : connection.connected.port.guid;
      const otherPort = findPortInType(otherType, otherPortId || "");
      requiredPorts.push(otherPort);
    } catch (error) {
      continue;
    }
  }
  return (
    kit.types?.filter((replacementType) => {
      if (replacementType.isAbstract) return false;
      if (variants !== undefined && !variants.includes(replacementType.parent?.guid ?? "")) return false;
      if (!replacementType.ports || replacementType.ports.length === 0) return requiredPorts.length === 0;
      return requiredPorts.every((requiredPort) => {
        return replacementType.ports!.some((replacementPort) => arePortsCompatible(replacementPort, requiredPort));
      });
    }) ?? []
  );
};

export const findReplacableTypesForPiecesInDesign = (kit: Kit, designGuid: string, pieceGuids: string[], variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designGuid);
  const pieces = pieceGuids.map((id) => findPieceInDesign(design, id));
  const externalConnections: Array<{
    connection: Connection;
    requiredPort: Port;
  }> = [];
  for (const piece of pieces) {
    const connections = findPieceConnectionsInDesign(design, piece.guid);
    for (const connection of connections) {
      const otherPieceId = connection.connected.piece.guid === piece.guid ? connection.connecting.piece.guid : connection.connected.piece.guid;
      if (!pieceGuids.includes(otherPieceId)) {
        try {
          const otherPiece = findPieceInDesign(design, otherPieceId);
          if (!otherPiece.type) continue;
          const otherType = findTypeInKit(kit, otherPiece.type.guid);
          const otherPortId = connection.connected.piece.guid === piece.guid ? connection.connecting.port.guid : connection.connected.port.guid;
          const otherPort = findPortInType(otherType, otherPortId || "");
          externalConnections.push({ connection, requiredPort: otherPort });
        } catch (error) {
          continue;
        }
      }
    }
  }
  return (
    kit.types?.filter((replacementType) => {
      if (replacementType.isAbstract) return false;
      if (variants !== undefined && !variants.includes(replacementType.parent?.guid ?? "")) return false;
      if (!replacementType.ports || replacementType.ports.length === 0) return externalConnections.length === 0;
      return externalConnections.every(({ requiredPort }) => {
        return replacementType.ports!.some((replacementPort) => arePortsCompatible(replacementPort, requiredPort));
      });
    }) ?? []
  );
};

export const piecesMetadata = (
  kit: Kit,
  designGuid: string,
): Map<
  string,
  {
    plane: Plane;
    center: Coord;
    fixedPieceId: string;
    parentPieceId: string | null;
    depth: number;
  }
> => {
  const design = findDesignInKit(kit, designGuid);
  if (!design) {
    throw new Error(`Design ${designGuid} not found in kit ${kit.name}`);
  }
  const flattenDiff = flattenDesign(kit, designGuid);
  const flatDesign = applyDesignDiff(design, flattenDiff);
  const fixedPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.fixedPieceId", p.guid) || p.guid);
  const parentPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.parentPieceId", null));
  const depths = flatDesign.pieces?.map((p) => parseInt(findAttributeValue(p, "semio.depth", "0")!));
  return new Map(
    flatDesign.pieces?.map((p, index) => [
      p.guid,
      {
        plane: p.plane!,
        center: p.center!,
        fixedPieceId: fixedPieceIds![index],
        parentPieceId: parentPieceIds![index],
        depth: depths![index],
      },
    ]),
  );
};

export const findAttributeValue = (entity: Kit | Type | Design | Piece | Connection | Model | Port, name: string, defaultValue?: string | null): string | null => {
  const attribute = entity.attributes?.find((q) => q.key === name);
  if (!attribute && defaultValue === undefined) throw new Error(`Attribute ${name} not found in ${entity}`);
  if (attribute?.value === undefined && defaultValue === null) return null;
  return attribute?.value ?? defaultValue ?? "";
};

const getColorForText = (text?: string): string => {
  if (!text || text === "") return "var(--foreground)";

  // Create a simple hash from the interface string
  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }

  // Generate color variations based on accent and status semantics
  const baseColors = [
    {
      base: "var(--accent)",
      variations: [
        "color-mix(in srgb, var(--accent) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--accent) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--accent) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--accent) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--accent-secondary)",
      variations: [
        "color-mix(in srgb, var(--accent-secondary) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--accent-secondary) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--accent-secondary) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--accent-secondary) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--accent-tertiary)",
      variations: [
        "color-mix(in srgb, var(--accent-tertiary) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--accent-tertiary) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--accent-tertiary) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--accent-tertiary) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--status-success)",
      variations: [
        "color-mix(in srgb, var(--status-success) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--status-success) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--status-success) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--status-success) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--status-warning)",
      variations: [
        "color-mix(in srgb, var(--status-warning) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--status-warning) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--status-warning) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--status-warning) 45%, var(--foreground) 55%)",
      ],
    },
    {
      base: "var(--status-info)",
      variations: [
        "color-mix(in srgb, var(--status-info) 85%, var(--base) 15%)",
        "color-mix(in srgb, var(--status-info) 70%, var(--base) 30%)",
        "color-mix(in srgb, var(--status-info) 60%, var(--foreground) 40%)",
        "color-mix(in srgb, var(--status-info) 45%, var(--foreground) 55%)",
      ],
    },
  ];

  const colorSetIndex = Math.abs(hash) % baseColors.length;
  const variationIndex = Math.abs(Math.floor(hash / baseColors.length)) % baseColors[colorSetIndex].variations.length;

  return baseColors[colorSetIndex].variations[variationIndex];
};

export const colorPortsForTypes = (types: Type[]): TypesDiff => {
  const updated: { id: string; diff: TypeDiff }[] = [];

  for (const type of types) {
    const updatedPorts = (type.ports || []).map((port) => ({
      ...port,
      attributes: [
        ...(port.attributes || []),
        {
          guid: guid(),
          key: "semio.color",
          value: getColorForText(port.interface?.guid),
        },
      ],
    }));

    updated.push({
      id: type.guid,
      diff: {
        ports: { added: updatedPorts },
      },
    });
  }

  return { updated };
};

// Helper function to parse design guid from design piece variant
export const parseDesignIdFromVariant = (variant: string): string => {
  return variant.split("-")[0];
};

// #region File Tree Utilities

export interface FileTreeNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children: FileTreeNode[];
  file?: File;
  folderGuid?: string;
  parentPath?: string;
}

export const buildFileTree = (folders: Folder[], files: File[]): FileTreeNode[] => {
  const folderChildren = new Map<string | undefined, Folder[]>();
  folders.forEach((folder) => {
    const parent = folder.parent?.guid;
    if (!folderChildren.has(parent)) folderChildren.set(parent, []);
    folderChildren.get(parent)!.push(folder);
  });

  const filesByFolder = new Map<string | undefined, File[]>();
  files.forEach((file) => {
    const folder = file.folder?.guid;
    if (!filesByFolder.has(folder)) filesByFolder.set(folder, []);
    filesByFolder.get(folder)!.push(file);
  });

  const sortFolders = (items?: Folder[]): Folder[] => {
    return (items || []).slice().sort((a, b) => a.name.localeCompare(b.name));
  };

  const sortFiles = (items?: File[]): File[] => {
    return (items || []).slice().sort((a, b) => a.name.localeCompare(b.name));
  };

  const buildNodes = (parentGuid?: string, parentPath?: string): FileTreeNode[] => {
    const children: FileTreeNode[] = [];
    const childFolders = sortFolders(folderChildren.get(parentGuid));
    childFolders.forEach((folder) => {
      const nodePath = folder.guid;
      children.push({
        name: folder.name,
        path: nodePath,
        parentPath,
        isDirectory: true,
        folderGuid: folder.guid,
        children: buildNodes(folder.guid, nodePath),
      });
    });
    const childFiles = sortFiles(filesByFolder.get(parentGuid));
    childFiles.forEach((file) => {
      children.push({
        name: file.name,
        path: file.guid,
        parentPath,
        isDirectory: false,
        children: [],
        file,
      });
    });
    return children;
  };

  return buildNodes(undefined, undefined);
};

/**
 * Flattens the file tree respecting expansion state.
 */
export const flattenFileTree = (nodes: FileTreeNode[], level: number = 0, expandedPaths: Set<string> = new Set()): Array<FileTreeNode & { level: number; isExpanded: boolean }> => {
  const result: Array<FileTreeNode & { level: number; isExpanded: boolean }> = [];

  nodes.forEach((node) => {
    const isExpanded = expandedPaths.has(`file-${node.path}`);
    result.push({ ...node, level, isExpanded });

    if (node.isDirectory && isExpanded && node.children.length > 0) {
      result.push(...flattenFileTree(node.children, level + 1, expandedPaths));
    }
  });

  return result;
};

// #endregion File Tree Utilities

// File utility functions
export const createFileFromDataUri = (name: string, dataUri: string): File => {
  const sizeMatch = dataUri.match(/data:([^;]+)(;base64)?,(.+)/);
  let size = 0;
  if (sizeMatch) {
    const data = sizeMatch[3];
    if (sizeMatch[2] === ";base64") {
      size = Math.floor(data.length * 0.75);
    } else {
      size = data.length;
    }
  }

  // Simple hash calculation (not cryptographically secure, but sufficient for tracking)
  let hash = 0;
  for (let i = 0; i < dataUri.length; i++) {
    const char = dataUri.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }

  return {
    guid: guid(),
    name,
    size,
    hash: hash.toString(36),
    createdAt: new Date(),
    updatedAt: new Date(),
  };
};

// #region Kit Import/Export

export interface KitImportResult {
  kit: Kit;
  files: Map<string, Blob>;
}

/**
 * Import a kit from a URL (remote HTTP/HTTPS) or ArrayBuffer/Buffer
 * Fetches the archive, extracts it, reads the SQLite database, and returns the kit and files.
 */
export const importKit = async (source: string | ArrayBuffer | Buffer): Promise<KitImportResult> => {
  const JSZip = (await import("jszip")).default;
  const initSqlJs = (await import("sql.js")).default;

  let arrayBuffer: ArrayBuffer;
  if (typeof source === "string") {
    const response = await fetch(source);
    if (!response.ok) {
      throw new Error(`Failed to fetch kit from ${source}: ${response.statusText}`);
    }
    arrayBuffer = await response.arrayBuffer();
  } else if (source instanceof Buffer) {
    arrayBuffer = source.buffer.slice(source.byteOffset, source.byteOffset + source.byteLength) as ArrayBuffer;
  } else {
    arrayBuffer = source as ArrayBuffer;
  }

  const zip = await JSZip.loadAsync(arrayBuffer);

  const dbFile = zip.file(".semio/kit.db");
  if (!dbFile) {
    throw new Error("Invalid kit archive: missing .semio/kit.db");
  }

  const dbArrayBuffer = await dbFile.async("arraybuffer");
  const SQL = await initSqlJs();
  const db = new SQL.Database(new Uint8Array(dbArrayBuffer));

  const kit = await sqliteToKit(db);

  const files = new Map<string, Blob>();
  for (const [path, zipEntry] of Object.entries(zip.files)) {
    if (!zipEntry.dir && !path.startsWith(".semio/")) {
      const blob = await zipEntry.async("blob");
      files.set(path, blob);
    }
  }

  db.close();

  return { kit, files };
};

/**
 * Export a kit to a zip blob
 * Creates a .semio/kit.db SQLite database and bundles it with all files into a zip.
 * If the kit is the Metabolism kit, also includes all files from examples/metabolism (excluding .semio folder).
 */
export const exportKit = async (kit: Kit, files: Map<string, Blob>): Promise<Blob> => {
  const JSZip = (await import("jszip")).default;
  const initSqlJs = (await import("sql.js")).default;

  const SQL = await initSqlJs();
  const db = new SQL.Database();

  await kitToSqlite(kit, db);

  const dbData = db.export();
  db.close();

  const zip = new JSZip();
  zip.file(".semio/kit.db", dbData);

  // Add provided files
  for (const [path, blob] of files.entries()) {
    zip.file(path, blob);
  }

  // If this is the Metabolism kit, add all files from examples/metabolism (excluding .semio)
  // Note: This requires the files to be provided in the files Map parameter
  // In the future, this could be enhanced to fetch files from a known location

  return await zip.generateAsync({ type: "blob" });
};

/**
 * Deep equality check for kits - recursively compares all properties including nested entities
 */
export const areKitsEqual = (a: Kit, b: Kit): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined) ? undefined : value;

  const areAttributesEqual = (a?: Attribute[], b?: Attribute[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const attrA of arrA) {
      const attrB = arrB.find(x => x.guid === attrA.guid);
      if (!attrB) return false;
      if (attrA.key !== attrB.key) return false;
      if (normalizeValue(attrA.value) !== normalizeValue(attrB.value)) return false;
      if (normalizeValue(attrA.definition) !== normalizeValue(attrB.definition)) return false;
    }
    return true;
  };

  const arePortsEqual = (a?: Port[], b?: Port[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const portA of arrA) {
      const portB = arrB.find(x => x.guid === portA.guid);
      if (!portB) return false;
      if (normalizeValue(portA.name) !== normalizeValue(portB.name)) return false;
      if (portA.point.x !== portB.point.x) return false;
      if (portA.point.y !== portB.point.y) return false;
      if (portA.point.z !== portB.point.z) return false;
      if (portA.direction.x !== portB.direction.x) return false;
      if (portA.direction.y !== portB.direction.y) return false;
      if (portA.direction.z !== portB.direction.z) return false;
      if (portA.t !== portB.t) return false;
      if (portA.mandatory !== portB.mandatory) return false;
      if (!areAttributesEqual(portA.attributes, portB.attributes)) return false;
    }
    return true;
  };

  const areModelsEqual = (a?: Model[], b?: Model[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const modelA of arrA) {
      const modelB = arrB.find(x => x.guid === modelA.guid);
      if (!modelB) return false;
      if (normalizeValue(modelA.name) !== normalizeValue(modelB.name)) return false;
      if (modelA.file !== modelB.file) return false;
      if (!arraysEqual(normalizeArray(modelA.tags), normalizeArray(modelB.tags))) return false;
      if (!areAttributesEqual(modelA.attributes, modelB.attributes)) return false;
    }
    return true;
  };

  const areTypesEqual = (a?: Type[], b?: Type[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const typeA of arrA) {
      const typeB = arrB.find(t => {
        if (t.guid !== typeA.guid) return false;
        if (!t.parent && !typeA.parent) return true;
        if (!t.parent || !typeA.parent) return false;
        return areSameTypeId(t.parent, typeA.parent);
      });
      if (!typeB) return false;
      if (typeA.name !== typeB.name) return false;
      if (normalizeValue(typeA.description) !== normalizeValue(typeB.description)) return false;
      if (normalizeValue(typeA.icon) !== normalizeValue(typeB.icon)) return false;
      if (normalizeValue(typeA.image) !== normalizeValue(typeB.image)) return false;
      if (!arraysEqual(normalizeArray(typeA.concepts), normalizeArray(typeB.concepts))) return false;
      if (!areModelsEqual(typeA.models, typeB.models)) return false;
      if (!arePortsEqual(typeA.ports, typeB.ports)) return false;
      if (!areAttributesEqual(typeA.attributes, typeB.attributes)) return false;
    }
    return true;
  };

  const arePiecesEqual = (a?: Piece[], b?: Piece[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const pieceA of arrA) {
      const pieceB = arrB.find(x => x.guid === pieceA.guid);
      if (!pieceB) return false;
      if (normalizeValue(pieceA.name) !== normalizeValue(pieceB.name)) return false;
      if (pieceA.type?.guid !== pieceB.type?.guid) return false;
      if (pieceA.design?.guid !== pieceB.design?.guid) return false;
      if (pieceA.plane && pieceB.plane) {
        if (pieceA.plane.origin.x !== pieceB.plane.origin.x) return false;
        if (pieceA.plane.origin.y !== pieceB.plane.origin.y) return false;
        if (pieceA.plane.origin.z !== pieceB.plane.origin.z) return false;
        if (pieceA.plane.xAxis.x !== pieceB.plane.xAxis.x) return false;
        if (pieceA.plane.xAxis.y !== pieceB.plane.xAxis.y) return false;
        if (pieceA.plane.xAxis.z !== pieceB.plane.xAxis.z) return false;
        if (pieceA.plane.yAxis.x !== pieceB.plane.yAxis.x) return false;
        if (pieceA.plane.yAxis.y !== pieceB.plane.yAxis.y) return false;
        if (pieceA.plane.yAxis.z !== pieceB.plane.yAxis.z) return false;
      } else if (pieceA.plane || pieceB.plane) {
        return false;
      }
      if (pieceA.center && pieceB.center) {
        if (pieceA.center.u !== pieceB.center.u) return false;
        if (pieceA.center.v !== pieceB.center.v) return false;
      } else if (pieceA.center || pieceB.center) {
        return false;
      }
      if (pieceA.scale !== pieceB.scale) return false;
      if (pieceA.mirrorPlane && pieceB.mirrorPlane) {
        if (pieceA.mirrorPlane.origin.x !== pieceB.mirrorPlane.origin.x) return false;
        if (pieceA.mirrorPlane.origin.y !== pieceB.mirrorPlane.origin.y) return false;
        if (pieceA.mirrorPlane.origin.z !== pieceB.mirrorPlane.origin.z) return false;
        if (pieceA.mirrorPlane.xAxis.x !== pieceB.mirrorPlane.xAxis.x) return false;
        if (pieceA.mirrorPlane.xAxis.y !== pieceB.mirrorPlane.xAxis.y) return false;
        if (pieceA.mirrorPlane.xAxis.z !== pieceB.mirrorPlane.xAxis.z) return false;
        if (pieceA.mirrorPlane.yAxis.x !== pieceB.mirrorPlane.yAxis.x) return false;
        if (pieceA.mirrorPlane.yAxis.y !== pieceB.mirrorPlane.yAxis.y) return false;
        if (pieceA.mirrorPlane.yAxis.z !== pieceB.mirrorPlane.yAxis.z) return false;
      } else if (pieceA.mirrorPlane || pieceB.mirrorPlane) {
        return false;
      }
      if (pieceA.isHidden !== pieceB.isHidden) return false;
      if (pieceA.isLocked !== pieceB.isLocked) return false;
      if (normalizeValue(pieceA.color) !== normalizeValue(pieceB.color)) return false;
      if (normalizeValue(pieceA.description) !== normalizeValue(pieceB.description)) return false;
      if (!areAttributesEqual(pieceA.attributes, pieceB.attributes)) return false;
    }
    return true;
  };

  const areConnectionsEqual = (a?: Connection[], b?: Connection[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const connA of arrA) {
      const connB = arrB.find(x => x.guid === connA.guid);
      if (!connB) return false;
      if (connA.connected.piece.guid !== connB.connected.piece.guid) return false;
      if (connA.connected.designPiece?.guid !== connB.connected.designPiece?.guid) return false;
      if (connA.connected.port.guid !== connB.connected.port.guid) return false;
      if (connA.connecting.piece.guid !== connB.connecting.piece.guid) return false;
      if (connA.connecting.designPiece?.guid !== connB.connecting.designPiece?.guid) return false;
      if (connA.connecting.port.guid !== connB.connecting.port.guid) return false;
      if (connA.gap !== connB.gap) return false;
      if (connA.shift !== connB.shift) return false;
      if (connA.rise !== connB.rise) return false;
      if (connA.rotation !== connB.rotation) return false;
      if (connA.turn !== connB.turn) return false;
      if (connA.tilt !== connB.tilt) return false;
      if (connA.x !== connB.x) return false;
      if (connA.y !== connB.y) return false;
      if (normalizeValue(connA.description) !== normalizeValue(connB.description)) return false;
      if (!areAttributesEqual(connA.attributes, connB.attributes)) return false;
    }
    return true;
  };

  const areDesignsEqual = (a?: Design[], b?: Design[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const designA of arrA) {
      const designB = arrB.find(d => {
        if (d.guid !== designA.guid) return false;
        if (!d.parent && !designA.parent) return true;
        if (!d.parent || !designA.parent) return false;
        return areSameDesignId(d.parent, designA.parent);
      });
      if (!designB) return false;
      if (designA.name !== designB.name) return false;
      if (normalizeValue(designA.description) !== normalizeValue(designB.description)) return false;
      if (normalizeValue(designA.icon) !== normalizeValue(designB.icon)) return false;
      if (normalizeValue(designA.image) !== normalizeValue(designB.image)) return false;
      if (!arraysEqual(normalizeArray(designA.concepts), normalizeArray(designB.concepts))) return false;
      if (!arePiecesEqual(designA.pieces, designB.pieces)) return false;
      if (!areConnectionsEqual(designA.connections, designB.connections)) return false;
      if (!areAttributesEqual(designA.attributes, designB.attributes)) return false;
    }
    return true;
  };

  const areInterfacesEqual = (a?: Interface[], b?: Interface[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const ifaceA of arrA) {
      const ifaceB = arrB.find(x => x.guid === ifaceA.guid);
      if (!ifaceB) return false;
      if (ifaceA.name !== ifaceB.name) return false;
      if (normalizeValue(ifaceA.description) !== normalizeValue(ifaceB.description)) return false;
      if (!areAttributesEqual(ifaceA.attributes, ifaceB.attributes)) return false;
    }
    return true;
  };

  const areQualitiesEqual = (a?: Quality[], b?: Quality[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const qualA of arrA) {
      const qualB = arrB.find(x => x.guid === qualA.guid);
      if (!qualB) return false;
      if (qualA.key !== qualB.key) return false;
      if (qualA.name !== qualB.name) return false;
      if (!areAttributesEqual(qualA.attributes, qualB.attributes)) return false;
    }
    return true;
  };

  const areFilesEqual = (a?: File[], b?: File[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const fileA of arrA) {
      const fileB = arrB.find(x => x.guid === fileA.guid);
      if (!fileB) return false;
      if (fileA.name !== fileB.name) return false;
    }
    return true;
  };

  const areFoldersEqual = (a?: Folder[], b?: Folder[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const folderA of arrA) {
      const folderB = arrB.find(x => x.guid === folderA.guid);
      if (!folderB) return false;
      if (folderA.name !== folderB.name) return false;
      if (!areAttributesEqual(folderA.attributes, folderB.attributes)) return false;
    }
    return true;
  };

  const areAuthorsEqual = (a?: Author[], b?: Author[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const authorA of arrA) {
      const authorB = arrB.find(x => x.guid === authorA.guid);
      if (!authorB) return false;
      if (authorA.name !== authorB.name) return false;
      if (normalizeValue(authorA.email) !== normalizeValue(authorB.email)) return false;
      if (!areAttributesEqual(authorA.attributes, authorB.attributes)) return false;
    }
    return true;
  };

  // Top-level kit properties
  if (a.guid !== b.guid) return false;
  if (a.name !== b.name) return false;
  if (normalizeValue(a.version) !== normalizeValue(b.version)) return false;
  if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
  if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
  if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
  if (normalizeValue(a.preview) !== normalizeValue(b.preview)) return false;
  if (normalizeValue(a.remote) !== normalizeValue(b.remote)) return false;
  if (normalizeValue(a.homepage) !== normalizeValue(b.homepage)) return false;
  if (normalizeValue(a.license) !== normalizeValue(b.license)) return false;

  if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
  if (!areTypesEqual(a.types, b.types)) return false;
  if (!areDesignsEqual(a.designs, b.designs)) return false;
  if (!areInterfacesEqual(a.interfaces, b.interfaces)) return false;
  if (!areQualitiesEqual(a.qualities, b.qualities)) return false;
  if (!areFilesEqual(a.files, b.files)) return false;
  if (!areFoldersEqual(a.folders, b.folders)) return false;
  if (!areAuthorsEqual(a.authors, b.authors)) return false;
  if (!areAttributesEqual(a.attributes, b.attributes)) return false;

  return true;
};

/**
 * Convert SQLite database to Kit JSON structure
 */
const sqliteToKit = async (db: any): Promise<Kit> => {
  const execResult = (query: string, params?: any[]): any[] => {
    const stmt = db.prepare(query);
    if (params) {
      stmt.bind(params);
    }
    const result: any[] = [];
    while (stmt.step()) {
      const row = stmt.getAsObject();
      result.push(row);
    }
    stmt.free();
    return result;
  };

  const kitRows = execResult("SELECT * FROM kit LIMIT 1");
  if (kitRows.length === 0) {
    throw new Error("No kit found in database");
  }
  const kitRow = kitRows[0];

  const toUndefined = (value: any): any => (value === null || value === "") ? undefined : value;
  const mapOrUndefined = <T, R>(arr: T[], mapper: (item: T) => R): R[] | undefined =>
    arr.length > 0 ? arr.map(mapper) : undefined;

  const kit: Kit = {
    guid: kitRow.guid || kitRow.uri || guid(),
    name: kitRow.name || "Unnamed Kit",
    version: kitRow.version || "0.0.0",
    description: toUndefined(kitRow.description),
    icon: toUndefined(kitRow.icon),
    image: toUndefined(kitRow.image),
    preview: toUndefined(kitRow.preview),
    remote: toUndefined(kitRow.remote),
    homepage: toUndefined(kitRow.homepage),
    license: toUndefined(kitRow.license),
    createdAt: new Date(kitRow.created),
    updatedAt: new Date(kitRow.updated),
  };

  const types = execResult("SELECT * FROM type WHERE kit_guid = ?", [kit.guid]);
  kit.types = mapOrUndefined(types, (row: any) => {
    const typeGuid = row.guid || String(row.id);
    const models = execResult("SELECT * FROM model WHERE type_guid = ?", [typeGuid]);
    const ports = execResult("SELECT * FROM port WHERE type_guid = ?", [typeGuid]);
    const typeAttributes = execResult("SELECT * FROM attribute WHERE type_guid = ?", [typeGuid]);
    const typeConcepts = execResult("SELECT * FROM type_concept WHERE type_guid = ?", [typeGuid]);

    return {
      guid: typeGuid,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      image: toUndefined(row.image),
      parent: row.parent_guid ? { guid: row.parent_guid } : (row.parent_id ? { guid: String(row.parent_id) } : undefined),
      virtual: Boolean(row.virtual),
      unit: toUndefined(row.unit),
      stock: row.stock,
      createdAt: new Date(row.created),
      updatedAt: new Date(row.updated),
      models: mapOrUndefined(models, (m: any) => {
        const modelTags = execResult("SELECT tag FROM model_tag WHERE model_guid = ?", [m.guid]);
        const modelAttributes = execResult("SELECT * FROM attribute WHERE model_guid = ?", [m.guid]);
        return {
          guid: m.guid,
          file: m.file,
          name: toUndefined(m.name),
          description: toUndefined(m.description),
          tags: modelTags.map((t: any) => t.tag),
          attributes: mapOrUndefined(modelAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      ports: mapOrUndefined(ports, (p: any) => {
        const portProps = execResult("SELECT * FROM prop WHERE port_guid = ?", [p.guid]);
        const portAttributes = execResult("SELECT * FROM attribute WHERE port_guid = ?", [p.guid]);
        return {
          guid: p.guid,
          name: toUndefined(p.name),
          point: { x: p.point_x, y: p.point_y, z: p.point_z },
          direction: { x: p.direction_x, y: p.direction_y, z: p.direction_z },
          t: p.t,
          mandatory: Boolean(p.mandatory),
          interface: p.interface_guid ? { guid: p.interface_guid } : undefined,
          description: toUndefined(p.description),
          props: portProps.map((pr: any) => {
            const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
            if (!pr.quality_guid) return null;
            return {
              guid: pr.guid,
              value: String(pr.value),
              unit: toUndefined(pr.unit),
              quality: { guid: pr.quality_guid },
              attributes: mapOrUndefined(propAttributes, (a: any) => ({
                guid: a.guid,
                key: a.key,
                value: toUndefined(a.value),
                definition: toUndefined(a.definition),
              })),
            };
          }).filter((p: any): p is NonNullable<typeof p> => p !== null),
          attributes: mapOrUndefined(portAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      attributes: mapOrUndefined(typeAttributes, (a: any) => ({
        guid: a.guid,
        key: a.key,
        value: toUndefined(a.value),
        definition: toUndefined(a.definition),
      })),
      concepts: typeConcepts.length > 0 ? typeConcepts.map((c: any) => c.concept) : undefined,
    };
  });

  const designs = execResult("SELECT * FROM design WHERE kit_guid = ?", [kit.guid]);
  kit.designs = mapOrUndefined(designs, (row: any) => {
    const designGuid = row.guid || String(row.id);
    const pieces = execResult("SELECT * FROM piece WHERE design_guid = ?", [designGuid]);
    const connections = execResult("SELECT * FROM connection WHERE design_guid = ?", [designGuid]);
    const layers = execResult("SELECT * FROM layer WHERE design_guid = ?", [designGuid]);
    const groups = execResult("SELECT * FROM \"group\" WHERE design_guid = ?", [designGuid]);
    const stats = execResult("SELECT * FROM stat WHERE design_guid = ?", [designGuid]);
    const designAttributes = execResult("SELECT * FROM attribute WHERE design_guid = ?", [designGuid]);
    const designConcepts = execResult("SELECT * FROM design_concept WHERE design_guid = ?", [designGuid]);
    const designProps = execResult("SELECT * FROM design_prop WHERE design_guid = ?", [designGuid]);
    const designAuthors = execResult("SELECT * FROM design_author WHERE design_guid = ? ORDER BY rank ASC", [designGuid]);

    return {
      guid: designGuid,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      image: toUndefined(row.image),
      parent: row.parent_guid ? { guid: row.parent_guid } : (row.parent_id ? { guid: String(row.parent_id) } : undefined),
      unit: toUndefined(row.unit),
      isAbstract: row.is_abstract ? true : undefined,
      folder: toUndefined(row.folder),
      canScale: row.can_scale ? true : undefined,
      canMirror: row.can_mirror ? true : undefined,
      createdAt: new Date(row.created),
      updatedAt: new Date(row.updated),
      activeLayer: row.active_layer_guid ? { guid: row.active_layer_guid } : undefined,
      props: mapOrUndefined(designProps, (dp: any) => ({
        guid: guid(),
        quality: { guid: dp.quality_guid },
        value: String(dp.value),
        unit: toUndefined(dp.unit),
      })),
      authors: mapOrUndefined(designAuthors, (da: any) => da.author_guid),
      pieces: pieces.map((p: any) => {
        const pieceProps = execResult("SELECT prop.* FROM prop JOIN piece_prop ON prop.guid = piece_prop.prop_guid WHERE piece_prop.piece_guid = ?", [p.guid]);
        const pieceAttributes = execResult("SELECT * FROM attribute WHERE piece_guid = ?", [p.guid]);
        return {
          guid: p.guid,
          name: toUndefined(p.name),
          type: p.type_guid ? { guid: p.type_guid } : undefined,
          design: p.design_guid_ref ? { guid: p.design_guid_ref } : undefined,
          plane: p.plane_origin_x !== null ? {
            origin: { x: p.plane_origin_x, y: p.plane_origin_y, z: p.plane_origin_z },
            xAxis: { x: p.plane_x_axis_x, y: p.plane_x_axis_y, z: p.plane_x_axis_z },
            yAxis: { x: p.plane_y_axis_x, y: p.plane_y_axis_y, z: p.plane_y_axis_z },
          } : undefined,
          center: p.center_u !== null || p.center_v !== null ? { u: p.center_u, v: p.center_v } : undefined,
          scale: p.scale !== null ? p.scale : undefined,
          mirrorPlane: p.mirror_plane_origin_x !== null ? {
            origin: { x: p.mirror_plane_origin_x, y: p.mirror_plane_origin_y, z: p.mirror_plane_origin_z },
            xAxis: { x: p.mirror_plane_x_axis_x, y: p.mirror_plane_x_axis_y, z: p.mirror_plane_x_axis_z },
            yAxis: { x: p.mirror_plane_y_axis_x, y: p.mirror_plane_y_axis_y, z: p.mirror_plane_y_axis_z },
          } : undefined,
          isHidden: Boolean(p.is_hidden),
          isLocked: Boolean(p.is_locked),
          color: toUndefined(p.color),
          description: toUndefined(p.description),
          props: mapOrUndefined(pieceProps, (pr: any) => {
            const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
            return {
              guid: pr.guid,
              key: pr.key,
              value: pr.value,
              unit: toUndefined(pr.unit),
              quality: pr.quality_guid ? { guid: pr.quality_guid } : undefined,
              attributes: mapOrUndefined(propAttributes, (a: any) => ({
                guid: a.guid,
                key: a.key,
                value: toUndefined(a.value),
                definition: toUndefined(a.definition),
              })),
            };
          }),
          attributes: mapOrUndefined(pieceAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      connections: connections.map((c: any) => {
        const connectionAttributes = execResult("SELECT * FROM attribute WHERE connection_guid = ?", [c.guid]);
        return {
          guid: c.guid,
          connected: {
            piece: { guid: c.connected_piece_guid },
            designPiece: c.connected_design_piece_guid ? { guid: c.connected_design_piece_guid } : undefined,
            port: { guid: c.connected_port_guid },
          },
          connecting: {
            piece: { guid: c.connecting_piece_guid },
            designPiece: c.connecting_design_piece_guid ? { guid: c.connecting_design_piece_guid } : undefined,
            port: { guid: c.connecting_port_guid },
          },
          gap: c.gap || 0,
          shift: c.shift || 0,
          rise: c.rise || 0,
          rotation: c.rotation || 0,
          turn: c.turn || 0,
          tilt: c.tilt || 0,
          x: c.x !== null ? c.x : undefined,
          y: c.y !== null ? c.y : undefined,
          description: toUndefined(c.description),
          attributes: mapOrUndefined(connectionAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      layers: layers.map((l: any) => {
        const layerAttributes = execResult("SELECT * FROM attribute WHERE layer_guid = ?", [l.guid]);
        return {
          guid: l.guid,
          path: l.path,
          isHidden: Boolean(l.is_hidden),
          isLocked: Boolean(l.is_locked),
          color: toUndefined(l.color),
          description: toUndefined(l.description),
          attributes: mapOrUndefined(layerAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      groups: groups.map((g: any) => {
        const groupPieces = execResult("SELECT piece_guid FROM group_piece WHERE group_guid = ?", [g.guid]);
        const groupAttributes = execResult("SELECT * FROM attribute WHERE group_guid = ?", [g.guid]);
        return {
          guid: g.guid,
          name: toUndefined(g.name),
          color: toUndefined(g.color),
          description: toUndefined(g.description),
          pieces: groupPieces.map((gp: any) => ({ guid: gp.piece_guid })),
          attributes: mapOrUndefined(groupAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      stats: stats.map((s: any) => ({
        guid: s.guid,
        quality: { guid: s.quality_guid },
        min: s.min_value,
        minExcluded: s.min_excluded ? true : undefined,
        max: s.max_value,
        maxExcluded: s.max_excluded ? true : undefined,
        unit: toUndefined(s.unit),
      })),
      attributes: mapOrUndefined(designAttributes, (a: any) => ({
        guid: a.guid,
        key: a.key,
        value: toUndefined(a.value),
        definition: toUndefined(a.definition),
      })),
      concepts: designConcepts.length > 0 ? designConcepts.map((c: any) => c.concept) : undefined,
    };
  });

  // Load interfaces
  const interfaces = execResult("SELECT * FROM interface WHERE kit_guid = ?", [kit.guid]);
  kit.interfaces = mapOrUndefined(interfaces, (row: any) => {
    const compatibleInterfaces = execResult("SELECT compatible_interface_guid FROM interface_compatibility WHERE interface_guid = ?", [row.guid]);
    const interfaceAttributes = execResult("SELECT * FROM attribute WHERE interface_guid = ?", [row.guid]);
    return {
      guid: row.guid,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      compatible: compatibleInterfaces.map((ci: any) => ({ guid: ci.compatible_interface_guid })),
      attributes: mapOrUndefined(interfaceAttributes, (a: any) => ({
        guid: a.guid,
        key: a.key,
        value: toUndefined(a.value),
        definition: toUndefined(a.definition),
      })),
    };
  });

  // Load qualities
  const qualities = execResult("SELECT * FROM quality WHERE kit_guid = ?", [kit.guid]);
  kit.qualities = qualities.length > 0 ? qualities.map((row: any) => {
    const benchmarks = execResult("SELECT * FROM benchmark WHERE quality_guid = ?", [row.guid]);
    const qualityAttributes = execResult("SELECT * FROM attribute WHERE quality_guid = ?", [row.guid]);
    return {
      guid: row.guid,
      key: row.key,
      name: row.name,
      kind: row.kind,
      default: row.default_value,
      formula: toUndefined(row.formula),
      defaultSiUnit: toUndefined(row.default_si_unit),
      defaultImperialUnit: toUndefined(row.default_imperial_unit),
      min: row.min_value,
      minExcluded: Boolean(row.min_excluded),
      max: row.max_value,
      maxExcluded: Boolean(row.max_excluded),
      canScale: Boolean(row.can_scale),
      uri: toUndefined(row.definition),
      benchmarks: benchmarks.map((b: any) => {
        const benchmarkAttributes = execResult("SELECT * FROM attribute WHERE benchmark_guid = ?", [b.guid]);
        return {
          guid: b.guid,
          name: b.name,
          icon: toUndefined(b.icon),
          min: b.min_value,
          minExcluded: Boolean(b.min_excluded),
          max: b.max_value,
          maxExcluded: Boolean(b.max_excluded),
          attributes: mapOrUndefined(benchmarkAttributes, (a: any) => ({
            guid: a.guid,
            key: a.key,
            value: toUndefined(a.value),
            definition: toUndefined(a.definition),
          })),
        };
      }),
      attributes: mapOrUndefined(qualityAttributes, (a: any) => ({
        guid: a.guid,
        key: a.key,
        value: toUndefined(a.value),
        definition: toUndefined(a.definition),
      })),
    };
  }) : undefined;

  // Load files
  const files = execResult("SELECT * FROM file WHERE kit_guid = ?", [kit.guid]);
  kit.files = files.length > 0 ? files.map((row: any) => ({
    guid: row.guid,
    name: row.name,
    remote: toUndefined(row.remote_url),
    folder: row.folder_guid ? { guid: row.folder_guid } : undefined,
    size: row.size,
    hash: row.hash,
    createdAt: row.created ? new Date(row.created) : undefined,
    updatedAt: row.updated ? new Date(row.updated) : undefined,
  })) : undefined;

  // Load folders
  const folders = execResult("SELECT * FROM folder WHERE kit_guid = ?", [kit.guid]);
  kit.folders = mapOrUndefined(folders, (row: any) => ({
    guid: row.guid,
    name: row.name,
    parent: row.parent_guid ? { guid: row.parent_guid } : undefined,
    createdAt: row.created ? new Date(row.created) : undefined,
    updatedAt: row.updated ? new Date(row.updated) : undefined,
  }));

  // Load authors
  const authors = execResult("SELECT * FROM author WHERE kit_guid = ?", [kit.guid]);
  kit.authors = authors.length > 0 ? authors.map((row: any) => ({
    guid: row.guid,
    name: row.name,
    email: toUndefined(row.email),
  })) : undefined;

  // Load concepts
  const concepts = execResult("SELECT * FROM concept WHERE kit_guid = ?", [kit.guid]);
  kit.concepts = concepts.map((row: any) => row.value);

  // Load kit attributes
  const kitAttributes = execResult("SELECT * FROM attribute WHERE kit_guid = ?", [kit.guid]);
  kit.attributes = mapOrUndefined(kitAttributes, (a: any) => ({
    guid: a.guid,
    key: a.key,
    value: toUndefined(a.value),
    definition: toUndefined(a.definition),
  }));

  return kit;
};

/**
 * Convert Kit JSON structure to SQLite database with complete schema
 */
const toArray = <T>(value: T | T[] | undefined): T[] => {
  if (!value) return [];
  return Array.isArray(value) ? value : [value];
};

const kitToSqlite = async (kit: Kit, db: any): Promise<void> => {

  const SCHEMA = `
CREATE TABLE semio (
	release VARCHAR NOT NULL,
	engine VARCHAR NOT NULL,
	created DATETIME NOT NULL,
	PRIMARY KEY (release)
);

CREATE TABLE kit (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	version VARCHAR(64),
	description TEXT,
	icon TEXT,
	image TEXT,
	preview TEXT,
	remote TEXT,
	homepage TEXT,
	license TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	PRIMARY KEY (guid)
);

CREATE TABLE quality (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	name VARCHAR(256) NOT NULL,
	kind INTEGER NOT NULL,
	default_value FLOAT,
	formula TEXT,
	default_si_unit VARCHAR(64),
	default_imperial_unit VARCHAR(64),
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	can_scale BOOLEAN NOT NULL DEFAULT 0,
	definition TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE benchmark (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	icon TEXT,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	definition TEXT,
	quality_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE interface (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE interface_compatibility (
	interface_guid VARCHAR(36) NOT NULL,
	compatible_interface_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (interface_guid, compatible_interface_guid),
	FOREIGN KEY(interface_guid) REFERENCES interface (guid),
	FOREIGN KEY(compatible_interface_guid) REFERENCES interface (guid)
);

CREATE TABLE folder (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(parent_guid) REFERENCES folder (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE file (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	folder_guid VARCHAR(36),
	size INTEGER,
	hash VARCHAR(128),
	remote_url TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(folder_guid) REFERENCES folder (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE author (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	email VARCHAR(256),
	kit_guid VARCHAR(36),
	type_guid VARCHAR(36),
	design_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	is_abstract BOOLEAN NOT NULL DEFAULT 0,
	folder VARCHAR(256),
	stock INTEGER,
	virtual BOOLEAN NOT NULL DEFAULT 0,
	unit VARCHAR(64),
	location_guid VARCHAR(36),
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, kit_guid, parent_guid),
	FOREIGN KEY(parent_guid) REFERENCES type (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE model (
	guid VARCHAR(36) NOT NULL,
	file VARCHAR(256) NOT NULL,
	name VARCHAR(256),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE model_tag (
	model_guid VARCHAR(36) NOT NULL,
	tag VARCHAR(128) NOT NULL,
	PRIMARY KEY (model_guid, tag),
	FOREIGN KEY(model_guid) REFERENCES model (guid)
);

CREATE TABLE prop (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	quality_guid VARCHAR(36),
	port_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE port (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	point_x FLOAT NOT NULL,
	point_y FLOAT NOT NULL,
	point_z FLOAT NOT NULL,
	direction_x FLOAT NOT NULL,
	direction_y FLOAT NOT NULL,
	direction_z FLOAT NOT NULL,
	t FLOAT NOT NULL,
	mandatory BOOLEAN NOT NULL DEFAULT 0,
	interface_guid VARCHAR(36),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, type_guid),
	FOREIGN KEY(interface_guid) REFERENCES interface (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE design (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	variant VARCHAR(256),
	view_center_u FLOAT,
	view_center_v FLOAT,
	view_zoom FLOAT,
	unit VARCHAR(64),
	location_guid VARCHAR(36),
	active_layer_guid VARCHAR(36),
	is_abstract BOOLEAN,
	folder VARCHAR(256),
	can_scale BOOLEAN,
	can_mirror BOOLEAN,
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, kit_guid, parent_guid),
	FOREIGN KEY(parent_guid) REFERENCES design (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE design_prop (
	design_guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	PRIMARY KEY (design_guid, quality_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE design_author (
	design_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (design_guid, author_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
);

CREATE TABLE layer (
	guid VARCHAR(36) NOT NULL,
	path VARCHAR(512) NOT NULL,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE piece (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	type_guid VARCHAR(36),
	design_guid_ref VARCHAR(36),
	plane_origin_x FLOAT,
	plane_origin_y FLOAT,
	plane_origin_z FLOAT,
	plane_x_axis_x FLOAT,
	plane_x_axis_y FLOAT,
	plane_x_axis_z FLOAT,
	plane_y_axis_x FLOAT,
	plane_y_axis_y FLOAT,
	plane_y_axis_z FLOAT,
	center_u FLOAT,
	center_v FLOAT,
	scale FLOAT,
	mirror_plane_origin_x FLOAT,
	mirror_plane_origin_y FLOAT,
	mirror_plane_origin_z FLOAT,
	mirror_plane_x_axis_x FLOAT,
	mirror_plane_x_axis_y FLOAT,
	mirror_plane_x_axis_z FLOAT,
	mirror_plane_y_axis_x FLOAT,
	mirror_plane_y_axis_y FLOAT,
	mirror_plane_y_axis_z FLOAT,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(design_guid_ref) REFERENCES design (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE piece_prop (
	piece_guid VARCHAR(36) NOT NULL,
	prop_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (piece_guid, prop_guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid)
);

CREATE TABLE "group" (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE group_piece (
	group_guid VARCHAR(36) NOT NULL,
	piece_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (group_guid, piece_guid),
	FOREIGN KEY(group_guid) REFERENCES "group" (guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid)
);

CREATE TABLE connection (
	guid VARCHAR(36) NOT NULL,
	connected_piece_guid VARCHAR(36) NOT NULL,
	connected_design_piece_guid VARCHAR(36),
	connected_port_guid VARCHAR(36) NOT NULL,
	connecting_piece_guid VARCHAR(36) NOT NULL,
	connecting_design_piece_guid VARCHAR(36),
	connecting_port_guid VARCHAR(36) NOT NULL,
	gap FLOAT NOT NULL DEFAULT 0,
	shift FLOAT NOT NULL DEFAULT 0,
	rise FLOAT NOT NULL DEFAULT 0,
	rotation FLOAT NOT NULL DEFAULT 0,
	turn FLOAT NOT NULL DEFAULT 0,
	tilt FLOAT NOT NULL DEFAULT 0,
	x FLOAT,
	y FLOAT,
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	CHECK (connecting_piece_guid != connected_piece_guid),
	FOREIGN KEY(connected_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connected_port_guid) REFERENCES port (guid),
	FOREIGN KEY(connecting_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connecting_port_guid) REFERENCES port (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE stat (
	guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	unit VARCHAR(64),
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE concept (
	kit_guid VARCHAR(36) NOT NULL,
	value VARCHAR(256) NOT NULL,
	PRIMARY KEY (kit_guid, value),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type_concept (
	type_guid VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (type_guid, concept)
);

CREATE TABLE design_concept (
	design_guid VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (design_guid, concept)
);

CREATE TABLE attribute (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(256) NOT NULL,
	value TEXT,
	definition TEXT,
	quality_guid VARCHAR(36),
	benchmark_guid VARCHAR(36),
	interface_guid VARCHAR(36),
	folder_guid VARCHAR(36),
	file_guid VARCHAR(36),
	author_guid VARCHAR(36),
	model_guid VARCHAR(36),
	prop_guid VARCHAR(36),
	port_guid VARCHAR(36),
	type_guid VARCHAR(36),
	layer_guid VARCHAR(36),
	piece_guid VARCHAR(36),
	group_guid VARCHAR(36),
	connection_guid VARCHAR(36),
	stat_guid VARCHAR(36),
	design_guid VARCHAR(36),
	kit_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid),
	FOREIGN KEY(benchmark_guid) REFERENCES benchmark (guid),
	FOREIGN KEY(interface_guid) REFERENCES interface (guid),
	FOREIGN KEY(folder_guid) REFERENCES folder (guid),
	FOREIGN KEY(file_guid) REFERENCES file (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(layer_guid) REFERENCES layer (guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(group_guid) REFERENCES "group" (guid),
	FOREIGN KEY(connection_guid) REFERENCES connection (guid),
	FOREIGN KEY(stat_guid) REFERENCES stat (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);
  `;

  // Execute schema using exec for multiple statements
  db.exec(SCHEMA);

  const toISOString = (date: Date | string | undefined): string => {
    if (!date) return new Date().toISOString();
    if (typeof date === "string") return date;
    return date.toISOString();
  };

  db.run("INSERT INTO semio (release, engine, created) VALUES (?, ?, ?)", ["1.0.0", "js", new Date().toISOString()]);

  db.run(
    "INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    [
      kit.guid,
      kit.name,
      kit.version || null,
      kit.description || null,
      kit.icon || null,
      kit.image || null,
      kit.preview || null,
      kit.remote || null,
      kit.homepage || null,
      kit.license || null,
      toISOString(kit.createdAt),
      toISOString(kit.updatedAt),
    ]
  );

  toArray(kit.concepts).forEach((concept) => {
    db.run("INSERT INTO concept (kit_guid, value) VALUES (?, ?)", [kit.guid, concept]);
  });

  toArray(kit.attributes).forEach((attr) => {
    db.run("INSERT INTO attribute (guid, key, value, definition, kit_guid) VALUES (?, ?, ?, ?, ?)", [
      attr.guid,
      attr.key,
      attr.value || null,
      attr.definition || null,
      kit.guid,
    ]);
  });

  toArray(kit.interfaces).forEach((iface) => {
    db.run("INSERT INTO interface (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [
      iface.guid,
      iface.name,
      iface.description || null,
      iface.icon || null,
      kit.guid,
    ]);

    toArray(iface.compatibleInterfaces).forEach((compat) => {
      db.run("INSERT INTO interface_compatibility (interface_guid, compatible_interface_guid) VALUES (?, ?)", [
        iface.guid,
        compat.guid,
      ]);
    });

    toArray(iface.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, interface_guid) VALUES (?, ?, ?, ?, ?)", [
        attr.guid,
        attr.key,
        attr.value || null,
        attr.definition || null,
        iface.guid,
      ]);
    });
  });

  toArray(kit.qualities).forEach((quality) => {
    db.run(
      "INSERT INTO quality (guid, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      [
        quality.guid,
        quality.key,
        quality.name,
        quality.kind,
        quality.defaultValue || null,
        quality.formula || null,
        quality.defaultSiUnit || null,
        quality.defaultImperialUnit || null,
        quality.min || null,
        quality.isMinExcluded ? 1 : 0,
        quality.max || null,
        quality.isMaxExcluded ? 1 : 0,
        quality.canScale ? 1 : 0,
        quality.uri || null,
        kit.guid,
      ]
    );

    toArray(quality.benchmarks).forEach((benchmark) => {
      db.run(
        "INSERT INTO benchmark (guid, name, icon, min_value, min_excluded, max_value, max_excluded, quality_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        [
          benchmark.guid,
          benchmark.name,
          benchmark.icon || null,
          benchmark.min || null,
          benchmark.minExcluded ? 1 : 0,
          benchmark.max || null,
          benchmark.maxExcluded ? 1 : 0,
          quality.guid,
        ]
      );

      toArray(benchmark.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, benchmark_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          benchmark.guid,
        ]);
      });
    });

    toArray(quality.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, quality_guid) VALUES (?, ?, ?, ?, ?)", [
        attr.guid,
        attr.key,
        attr.value || null,
        attr.definition || null,
        quality.guid,
      ]);
    });
  });

  toArray(kit.folders).forEach((folder) => {
    db.run("INSERT INTO folder (guid, name, parent_guid, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?)", [
      folder.guid,
      folder.name,
      folder.parent?.guid || null,
      toISOString(folder.createdAt),
      toISOString(folder.updatedAt),
      kit.guid,
    ]);
  });

  toArray(kit.files).forEach((file) => {
    db.run("INSERT INTO file (guid, name, folder_guid, size, hash, remote_url, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      file.guid,
      file.name,
      file.folder?.guid || null,
      file.size || null,
      file.hash || null,
      file.remote || null,
      toISOString(file.createdAt),
      toISOString(file.updatedAt),
      kit.guid,
    ]);
  });

  toArray(kit.authors).forEach((author) => {
    db.run("INSERT INTO author (guid, name, email, kit_guid) VALUES (?, ?, ?, ?)", [
      author.guid,
      author.name,
      author.email || null,
      kit.guid,
    ]);
  });

  toArray(kit.types).forEach((type) => {
    db.run(
      "INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      [
        type.guid,
        type.name,
        type.parent?.guid || null,
        type.isAbstract ? 1 : 0,
        type.folder || null,
        type.stock || null,
        type.virtual ? 1 : 0,
        type.unit || null,
        type.description || null,
        type.icon || null,
        type.image || null,
        toISOString(type.createdAt),
        toISOString(type.updatedAt),
        kit.guid,
      ]
    );

    toArray(type.concepts).forEach((concept) => {
      db.run("INSERT INTO type_concept (type_guid, concept) VALUES (?, ?)", [type.guid, concept]);
    });

    toArray(type.models).forEach((model) => {
      db.run("INSERT INTO model (guid, file, name, description, type_guid) VALUES (?, ?, ?, ?, ?)", [
        model.guid,
        model.file,
        model.name || null,
        model.description || null,
        type.guid,
      ]);

      toArray(model.tags).forEach((tag) => {
        db.run("INSERT INTO model_tag (model_guid, tag) VALUES (?, ?)", [model.guid, tag]);
      });

      toArray(model.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, model_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          model.guid,
        ]);
      });
    });

    toArray(type.ports).forEach((port) => {
      db.run(
        "INSERT INTO port (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, interface_guid, description, type_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          port.guid,
          port.name || null,
          port.point.x,
          port.point.y,
          port.point.z,
          port.direction.x,
          port.direction.y,
          port.direction.z,
          port.t,
          port.mandatory ? 1 : 0,
          port.interface?.guid || null,
          port.description || null,
          type.guid,
        ]
      );

      toArray(port.props).forEach((prop) => {
        db.run("INSERT INTO prop (guid, value, unit, quality_guid, port_guid) VALUES (?, ?, ?, ?, ?)", [
          prop.guid,
          prop.value,
          prop.unit || null,
          prop.quality.guid,
          port.guid,
        ]); toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [
            attr.guid,
            attr.key,
            attr.value || null,
            attr.definition || null,
            prop.guid,
          ]);
        });
      });

      toArray(port.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, port_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          port.guid,
        ]);
      });
    });

    toArray(type.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, type_guid) VALUES (?, ?, ?, ?, ?)", [
        attr.guid,
        attr.key,
        attr.value || null,
        attr.definition || null,
        type.guid,
      ]);
    });
  });

  toArray(kit.designs).forEach((design) => {
    db.run(
      "INSERT INTO design (guid, name, parent_guid, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      [
        design.guid,
        design.name,
        design.parent?.guid || null,
        design.unit || null,
        design.isAbstract ? 1 : null,
        design.folder || null,
        design.canScale ? 1 : null,
        design.canMirror ? 1 : null,
        design.description || null,
        design.icon || null,
        design.image || null,
        toISOString(design.createdAt),
        toISOString(design.updatedAt),
        kit.guid,
      ]
    );

    toArray(design.concepts).forEach((concept) => {
      db.run("INSERT INTO design_concept (design_guid, concept) VALUES (?, ?)", [design.guid, concept]);
    });

    toArray(design.props).forEach((prop) => {
      db.run("INSERT INTO design_prop (design_guid, quality_guid, value, unit) VALUES (?, ?, ?, ?)", [
        design.guid,
        prop.quality.guid,
        parseFloat(prop.value),
        prop.unit || null,
      ]);
    });

    toArray(design.authors).forEach((authorId, index) => {
      db.run("INSERT INTO design_author (design_guid, author_guid, rank) VALUES (?, ?, ?)", [
        design.guid,
        typeof authorId === 'object' ? authorId.guid : authorId,
        index,
      ]);
    });

    toArray(design.layers).forEach((layer) => {
      const layerGuid = guid();
      db.run("INSERT INTO layer (guid, path, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?)", [
        layerGuid,
        layer.path,
        layer.isHidden ? 1 : 0,
        layer.isLocked ? 1 : 0,
        layer.color || null,
        layer.description || null,
        design.guid,
      ]);

      toArray(layer.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, layer_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          layerGuid,
        ]);
      });
    });

    toArray(design.pieces).forEach((piece) => {
      db.run(
        "INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z, mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z, mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          piece.guid,
          piece.name || null,
          piece.type?.guid || null,
          piece.design?.guid || null,
          piece.plane?.origin.x !== undefined ? piece.plane.origin.x : null,
          piece.plane?.origin.y !== undefined ? piece.plane.origin.y : null,
          piece.plane?.origin.z !== undefined ? piece.plane.origin.z : null,
          piece.plane?.xAxis.x !== undefined ? piece.plane.xAxis.x : null,
          piece.plane?.xAxis.y !== undefined ? piece.plane.xAxis.y : null,
          piece.plane?.xAxis.z !== undefined ? piece.plane.xAxis.z : null,
          piece.plane?.yAxis.x !== undefined ? piece.plane.yAxis.x : null,
          piece.plane?.yAxis.y !== undefined ? piece.plane.yAxis.y : null,
          piece.plane?.yAxis.z !== undefined ? piece.plane.yAxis.z : null,
          piece.center?.u !== undefined ? piece.center.u : null,
          piece.center?.v !== undefined ? piece.center.v : null,
          piece.scale !== undefined ? piece.scale : null,
          piece.mirrorPlane?.origin.x !== undefined ? piece.mirrorPlane.origin.x : null,
          piece.mirrorPlane?.origin.y !== undefined ? piece.mirrorPlane.origin.y : null,
          piece.mirrorPlane?.origin.z !== undefined ? piece.mirrorPlane.origin.z : null,
          piece.mirrorPlane?.xAxis.x !== undefined ? piece.mirrorPlane.xAxis.x : null,
          piece.mirrorPlane?.xAxis.y !== undefined ? piece.mirrorPlane.xAxis.y : null,
          piece.mirrorPlane?.xAxis.z !== undefined ? piece.mirrorPlane.xAxis.z : null,
          piece.mirrorPlane?.yAxis.x !== undefined ? piece.mirrorPlane.yAxis.x : null,
          piece.mirrorPlane?.yAxis.y !== undefined ? piece.mirrorPlane.yAxis.y : null,
          piece.mirrorPlane?.yAxis.z !== undefined ? piece.mirrorPlane.yAxis.z : null,
          piece.isHidden ? 1 : 0,
          piece.isLocked ? 1 : 0,
          piece.color || null,
          piece.description || null,
          design.guid,
        ]
      );

      // Piece.props not in schema

      toArray(piece.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, piece_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          piece.guid,
        ]);
      });
    });

    toArray(design.groups).forEach((group) => {
      const groupGuid = guid();
      db.run("INSERT INTO \"group\" (guid, name, color, description, design_guid) VALUES (?, ?, ?, ?, ?)", [
        groupGuid,
        group.name || null,
        group.color || null,
        group.description || null,
        design.guid,
      ]);

      toArray(group.pieces).forEach((piece) => {
        db.run("INSERT INTO group_piece (group_guid, piece_guid) VALUES (?, ?)", [groupGuid, piece.guid]);
      });

      toArray(group.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, group_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          groupGuid,
        ]);
      });
    });

    toArray(design.connections).forEach((connection) => {
      if (!connection.connected?.piece || !connection.connecting?.piece || !connection.connected?.port || !connection.connecting?.port) {
        return;
      }
      db.run(
        "INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_port_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_port_guid, gap, shift, rise, rotation, turn, tilt, x, y, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          connection.guid,
          connection.connected.piece.guid,
          connection.connected.designPiece?.guid || null,
          connection.connected.port.guid,
          connection.connecting.piece.guid,
          connection.connecting.designPiece?.guid || null,
          connection.connecting.port.guid,
          connection.gap || 0,
          connection.shift || 0,
          connection.rise || 0,
          connection.rotation || 0,
          connection.turn || 0,
          connection.tilt || 0,
          connection.x !== undefined ? connection.x : null,
          connection.y !== undefined ? connection.y : null,
          connection.description || null,
          design.guid,
        ]
      );

      toArray(connection.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, connection_guid) VALUES (?, ?, ?, ?, ?)", [
          attr.guid,
          attr.key,
          attr.value || null,
          attr.definition || null,
          connection.guid,
        ]);
      });
    });

    toArray(design.stats).forEach((stat) => {
      db.run("INSERT INTO stat (guid, quality_guid, min_value, min_excluded, max_value, max_excluded, unit, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
        stat.guid,
        stat.quality.guid,
        stat.min || null,
        stat.minExcluded ? 1 : null,
        stat.max || null,
        stat.maxExcluded ? 1 : null,
        stat.unit || null,
        design.guid,
      ]);

      // Stat.attributes not in schema
    });

    toArray(design.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, design_guid) VALUES (?, ?, ?, ?, ?)", [
        attr.guid,
        attr.key,
        attr.value || null,
        attr.definition || null,
        design.guid,
      ]);
    });
  });
};

// #endregion Kit Import/Export

// #endregion Kit

