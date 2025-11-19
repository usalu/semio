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
export type PropId = { guid: Guid };
export type RepresentationId = { guid: Guid };
export type PortId = { guid: Guid };
export type TypeId = { guid: Guid };
export type LayerId = { guid: Guid };
export type PieceId = { guid: Guid };
export type GroupId = { guid: Guid };
export type SideId = { guid: Guid };
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
export const PropIdSchema = z.object({ guid: z.string() });
export const RepresentationIdSchema = z.object({ guid: z.string() });
export const PortIdSchema = z.object({ guid: z.string() });
export const TypeIdSchema = z.object({ guid: z.string() });
export const LayerIdSchema = z.object({ guid: z.string() });
export const PieceIdSchema = z.object({ guid: z.string() });
export const GroupIdSchema = z.object({ guid: z.string() });
export const SideIdSchema = z.object({ guid: z.string() });
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
export const createPropId = (guid: Guid): PropId => ({ guid });
export const createRepresentationId = (guid: Guid): RepresentationId => ({ guid });
export const createPortId = (guid: Guid): PortId => ({ guid });
export const createTypeId = (guid: Guid): TypeId => ({ guid });
export const createLayerId = (guid: Guid): LayerId => ({ guid });
export const createPieceId = (guid: Guid): PieceId => ({ guid });
export const createGroupId = (guid: Guid): GroupId => ({ guid });
export const createSideId = (guid: Guid): SideId => ({ guid });
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
export const areSamePropId = (a: PropId, b: PropId): boolean => a.guid === b.guid;
export const areSameRepresentationId = (a: RepresentationId, b: RepresentationId): boolean => a.guid === b.guid;
export const areSamePortId = (a: PortId, b: PortId): boolean => a.guid === b.guid;
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.guid === b.guid;
export const areSameLayerId = (a: LayerId, b: LayerId): boolean => a.guid === b.guid;
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
export const areSameGroupId = (a: GroupId, b: GroupId): boolean => a.guid === b.guid;
export const areSameSideId = (a: SideId, b: SideId): boolean => a.guid === b.guid;
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
export const getPropGuid = (id: PropId): Guid => id.guid;
export const getRepresentationGuid = (id: RepresentationId): Guid => id.guid;
export const getPortGuid = (id: PortId): Guid => id.guid;
export const getTypeGuid = (id: TypeId): Guid => id.guid;
export const getLayerGuid = (id: LayerId): Guid => id.guid;
export const getPieceGuid = (id: PieceId): Guid => id.guid;
export const getGroupGuid = (id: GroupId): Guid => id.guid;
export const getSideGuid = (id: SideId): Guid => id.guid;
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
  return { removed, updated, added };
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
  return { removed, added, updated };
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
  return { removed, added, updated };
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

// #region Representation
// https://github.com/usalu/semio#-representation-

export const RepresentationSchema = z.object({
  guid: z.string(),
  tags: z.array(z.string()).optional(),
  file: z.string(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Representation = z.infer<typeof RepresentationSchema>;
export const serializeRepresentation = (representation: Representation): string => JSON.stringify(RepresentationSchema.parse(representation));
export const deserializeRepresentation = (json: string): Representation => RepresentationSchema.parse(JSON.parse(json));

export const RepresentationDiffSchema = RepresentationSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;
export const getRepresentationDiff = (before: Representation, after: Representation): RepresentationDiff => {
  const diff: RepresentationDiff = {};
  if (JSON.stringify(before.tags) !== JSON.stringify(after.tags)) diff.tags = after.tags;
  if (before.file !== after.file) diff.file = after.file;
  if (before.description !== after.description) diff.description = after.description;
  if (before.attributes !== after.attributes) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseRepresentationDiff = (original: Representation, appliedDiff: RepresentationDiff): RepresentationDiff => {
  const inverse: RepresentationDiff = {};
  if (appliedDiff.tags !== undefined) inverse.tags = original.tags;
  if (appliedDiff.file !== undefined) inverse.file = original.file;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeRepresentationDiff = (diff1: RepresentationDiff, diff2: RepresentationDiff): RepresentationDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyRepresentationDiff = (base: Representation, diff: RepresentationDiff): Representation => {
  return {
    ...base,
    tags: diff.tags ?? base.tags,
    file: diff.file ?? base.file,
    description: diff.description ?? base.description,
    attributes: diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes,
  };
};

export const RepresentationsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: RepresentationDiffSchema })).optional(),
  added: z.array(RepresentationSchema).optional(),
});

export const areSameRepresentation = (representation: Representation, other: Representation): boolean => {
  return representation.tags?.every((tag) => other.tags?.includes(tag)) ?? true;
};

export const findRepresentation = (representations: Representation[], tags: string[]): Representation => {
  const indices = representations.map((r) => jaccard(r.tags, tags));
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return representations[maxIndexIndex];
};

export const getAllTagsFromRepresentations = (representations: Representation[]): string[] => {
  const tagsSet = new Set<string>();
  representations.forEach((r) => {
    r.tags?.forEach((tag) => tagsSet.add(tag));
  });
  return Array.from(tagsSet).sort();
};

export const filterRepresentationsByTags = (representations: Representation[], selectedTags: string[]): Representation[] => {
  if (!selectedTags || selectedTags.length === 0) return representations;
  return representations.filter((r) => {
    if (!r.tags || r.tags.length === 0) return false;
    return selectedTags.every((tag) => r.tags?.includes(tag));
  });
};

export const getAvailableTagsForRepresentations = (representations: Representation[], selectedTags: string[]): string[] => {
  const filteredReps = filterRepresentationsByTags(representations, selectedTags);
  const availableTags = getAllTagsFromRepresentations(filteredReps);
  return availableTags.filter((tag) => !selectedTags.includes(tag));
};

export const selectBestRepresentation = (representations: Representation[], selectedTags: string[]): Representation | undefined => {
  if (representations.length === 0) return undefined;
  if (selectedTags.length === 0) {
    const defaultRep = representations.find((r) => !r.tags || r.tags.length === 0);
    return defaultRep ?? representations[0];
  }
  const filteredReps = filterRepresentationsByTags(representations, selectedTags);
  if (filteredReps.length === 0) return undefined;
  return findRepresentation(filteredReps, selectedTags);
};

// #endregion Representation

// #region Port
// https://github.com/usalu/semio#-port-

export const PortSchema = z.object({
  guid: z.string(),
  t: z.number(),
  point: PointSchema,
  direction: VectorSchema,
  description: z.string().optional(),
  family: z.string().optional(),
  mandatory: z.boolean().optional(),
  compatibleFamilies: z.array(z.string()).optional(),
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
  if (before.description !== after.description) diff.description = after.description;
  if (before.family !== after.family) diff.family = after.family;
  if (before.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
  if (before.t !== after.t) diff.t = after.t - before.t;
  if (JSON.stringify(before.compatibleFamilies) !== JSON.stringify(after.compatibleFamilies)) diff.compatibleFamilies = after.compatibleFamilies;
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
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.family !== undefined) inverse.family = original.family;
  if (appliedDiff.mandatory !== undefined) inverse.mandatory = original.mandatory;
  if (appliedDiff.t !== undefined) inverse.t = original.t;
  if (appliedDiff.compatibleFamilies !== undefined) inverse.compatibleFamilies = original.compatibleFamilies;
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
    description: diff.description ?? base.description,
    family: diff.family ?? base.family,
    mandatory: diff.mandatory ?? base.mandatory,
    t: diff.t ?? base.t,
    compatibleFamilies: diff.compatibleFamilies ?? base.compatibleFamilies,
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
  return { removed, added, updated };
};

export const unifyPortFamiliesAndCompatibleFamiliesForTypes = (types: Type[]): TypesDiff => {
  const allFamilies = new Set<string>();
  for (const type of types) {
    for (const port of type.ports || []) {
      if (port.family && port.family !== "") allFamilies.add(port.family);
      for (const compatibleFamily of port.compatibleFamilies || []) {
        if (compatibleFamily && compatibleFamily !== "") allFamilies.add(compatibleFamily);
      }
    }
  }

  // Union-Find data structure
  const parent = new Map<string, string>();
  const rank = new Map<string, number>();

  // Initialize each family as its own parent
  for (const family of Array.from(allFamilies)) {
    parent.set(family, family);
    rank.set(family, 0);
  }

  // Find with path compression
  const find = (family: string): string => {
    if (parent.get(family) !== family) parent.set(family, find(parent.get(family)!));
    return parent.get(family)!;
  };

  // Union by rank
  const union = (family1: string, family2: string): void => {
    const root1 = find(family1);
    const root2 = find(family2);

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
      const portFamily = port.family;
      const compatibleFamilies = port.compatibleFamilies || [];

      if (portFamily && portFamily !== "") {
        // Union port's family with all its compatible families
        for (const compatibleFamily of compatibleFamilies) {
          if (compatibleFamily && compatibleFamily !== "") {
            union(portFamily, compatibleFamily);
          }
        }
      }

      // Also union all compatible families with each other
      for (let i = 0; i < compatibleFamilies.length; i++) {
        for (let j = i + 1; j < compatibleFamilies.length; j++) {
          const family1 = compatibleFamilies[i];
          const family2 = compatibleFamilies[j];
          if (family1 && family1 !== "" && family2 && family2 !== "") {
            union(family1, family2);
          }
        }
      }
    }
  }

  // Create mapping from any family to its representative
  const familyToRepresentative = new Map<string, string>();
  for (const family of Array.from(allFamilies)) {
    familyToRepresentative.set(family, find(family));
  }

  // Update all types with unified port families
  const updated: { id: string; diff: TypeDiff }[] = [];

  for (const type of types) {
    const updatedPorts = type.ports?.map((port) => {
      const portFamily = port.family;
      const compatibleFamilies = port.compatibleFamilies || [];

      // Determine the representative family for this port
      let representative: string | undefined;

      if (portFamily && portFamily !== "") {
        representative = familyToRepresentative.get(portFamily);
      } else if (compatibleFamilies.length > 0) {
        // If no family but has compatible families, use the first one's representative
        const firstCompatible = compatibleFamilies.find((f) => f && f !== "");
        if (firstCompatible) {
          representative = familyToRepresentative.get(firstCompatible);
        }
      }

      if (representative) {
        return {
          ...port,
          family: representative,
          compatibleFamilies: [representative],
        };
      } else {
        // No family information, keep as is
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
};
export const arePortsCompatible = (port: Port, otherPort: Port): boolean => {
  const normalizedPortFamily = normalize(port.family);
  const normalizedOtherPortFamily = normalize(otherPort.family);
  if (normalizedPortFamily === "" || normalizedOtherPortFamily === "") return true;
  return (port.compatibleFamilies ?? []).includes(normalizedOtherPortFamily) || (otherPort.compatibleFamilies ?? []).includes(normalizedPortFamily);
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
  parent: z.string().optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  representations: z.array(RepresentationSchema).optional(),
  ports: z.array(PortSchema).optional(),
  props: z.array(PropSchema).optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
  location: LocationSchema.optional(),
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

export const TypeShallowSchema = TypeSchema.omit({ representations: true, ports: true }).extend({
  representations: z.array(z.string()).optional(),
  ports: z.array(z.string()).optional(),
});
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
export const serializeTypeShallow = (type: TypeShallow): string => JSON.stringify(TypeShallowSchema.parse(type));
export const deserializeTypeShallow = (json: string): TypeShallow => TypeShallowSchema.parse(JSON.parse(json));
export const TypeDiffSchema = TypeSchema.partial().omit({ representations: true, ports: true, props: true, attributes: true }).extend({
  representations: RepresentationsDiffSchema.optional(),
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
  // TODO: Implement full Type apply diff logic including ports, representations, props
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
  // TODO: Implement full Piece diff logic
  return {};
};
export const inversePieceDiff = (original: Piece, appliedDiff: PieceDiff): PieceDiff => {
  // TODO: Implement full Piece inverse diff logic
  return {};
};
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => {
  // TODO: Implement full Piece merge diff logic
  return { ...diff1, ...diff2 };
};
export const applyPieceDiff = (base: Piece, diff: PieceDiff): Piece => {
  // TODO: Implement full Piece apply diff logic
  return base;
};

export const PiecesDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(z.object({ id: z.string(), diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

/**
 * 🔗 Returns a map of piece ids to representation file guids for the given design and types.
 * @param design - The design with the pieces to get the representation file guids for.
 * @param types - The types of the pieces with the representations.
 * @returns A map of piece ids to representation file guids.
 */
export const getPieceRepresentationFileGuids = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const representationFileGuids = new Map<string, string>();
  design.pieces?.forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.representations) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no representations`);
    const representation = findRepresentation(type.representations, tags);
    representationFileGuids.set(p.guid, representation.file);
  });
  return representationFileGuids;
};

/**
 * 🔗 Returns a map of piece ids to representation urls for the given design, types, and files.
 * @param design - The design with the pieces to get the representation urls for.
 * @param types - The types of the pieces with the representations.
 * @param files - The files in the kit to resolve urls from.
 * @param getFileUrl - Function to get the url for a file (from file provider).
 * @returns A map of piece ids to representation urls.
 */
export const getPieceRepresentationUrls = (design: Design, types: Type[], files: File[], getFileUrl: (fileGuid: string) => string, tags: string[] = []): Map<string, string> => {
  const representationUrls = new Map<string, string>();
  design.pieces?.forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.representations) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no representations`);
    const representation = findRepresentation(type.representations, tags);
    const file = files.find((f) => f.guid === representation.file);
    if (!file) throw new Error(`File ${representation.file} for representation ${representation.guid} not found`);
    representationUrls.set(p.guid, getFileUrl(file.guid));
  });
  return representationUrls;
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
  guid: z.string(),
  piece: PieceIdSchema,
  designPiece: PieceIdSchema.optional(),
  port: PortIdSchema,
});
export type Side = z.infer<typeof SideSchema>;
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = z.infer<typeof SideDiffSchema>;
export const SidesDiffSchema = z.object({
  removed: z.array(z.object({ piece: z.string(), designPiece: z.string().optional() })).optional(),
  updated: z.array(z.object({ id: z.object({ piece: z.string(), designPiece: z.string().optional() }), diff: SideDiffSchema })).optional(),
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
  parent: z.string().optional(),
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
  location: LocationSchema.optional(),
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
  // TODO: Implement full Design apply diff logic
  return base;
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

  let expandedDesign = expandDesignPieces(design, kit);

  if (!expandedDesign.pieces || expandedDesign.pieces.length === 0) return {};

  const typesDict: { [key: string]: Type } = {};
  types.forEach((t) => {
    typesDict[t.guid] = t;
  });
  const getType = (typeGuid: string): Type | undefined => {
    return typesDict[typeGuid];
  };
  const getPort = (type: Type | undefined, portGuid: string | undefined): Port | undefined => {
    if (!type?.ports) return undefined;
    return portGuid ? type.ports.find((p) => p.guid === portGuid) : type.ports[0];
  };

  const flatDesign: Design = JSON.parse(JSON.stringify(expandedDesign));
  if (!flatDesign.pieces) flatDesign.pieces = [];

  const piecePlanes: { [pieceGuid: string]: Plane } = {};
  const pieceMap: { [pieceGuid: string]: Piece } = {};
  flatDesign.pieces!.forEach((p) => {
    if (p.guid) pieceMap[p.guid] = p;
  });

  console.log(`[ORIGIN] flattenDesign: Starting with ${flatDesign.pieces?.length || 0} pieces and ${flatDesign.connections?.length || 0} connections`);
  console.log(
    `[ORIGIN] flattenDesign: Piece GUIDs in array:`,
    flatDesign.pieces?.map((p) => p.guid),
  );
  console.log(`[ORIGIN] flattenDesign: Piece GUIDs in map:`, Object.keys(pieceMap));
  if (flatDesign.connections && flatDesign.connections.length > 0) {
    console.log(
      `[ORIGIN] flattenDesign: Connection details:`,
      flatDesign.connections.map((c) => ({
        guid: c.guid,
        connected: c.connected.piece.guid,
        connecting: c.connecting.piece.guid,
      })),
    );
  }

  const cy = cytoscape({
    elements: {
      nodes: flatDesign.pieces!.map((piece) => ({
        data: { id: piece.guid, label: piece.guid },
      })),
      edges: flatDesign.connections
        ?.filter((connection) => {
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
        })
        .map((connection, index) => {
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

    const bfs = cy.elements().bfs({
      roots: `#${rootNode.id()}`,
      visit: (v, e, u, i, depth) => {
        if (!e) return;
        const edgeData = e.data();
        const connection: Connection | undefined = edgeData.connectionData;
        if (!connection) return;
        const parentNode = u;
        const childNode = v;
        const parentId = parentNode.id();
        const childId = childNode.id();
        const parentPiece = pieceMap[parentId];
        const childPiece = pieceMap[childId];
        if (!parentPiece || !childPiece || !parentPiece.guid || !childPiece.guid) return;
        if (piecePlanes[childPiece.guid]) return;
        const parentPlane = piecePlanes[parentPiece.guid];
        if (!parentPlane) {
          console.error(`Error during flatten: Parent piece ${parentPiece.guid} plane not found.`);
          return;
        }
        const parentSide = connection.connected.piece.guid === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.guid === childId ? connection.connecting : connection.connected;
        const parentType = parentPiece.type ? getType(parentPiece.type.guid) : undefined;
        const childType = childPiece.type ? getType(childPiece.type.guid) : undefined;
        const parentPort = getPort(parentType, parentSide.port.guid);
        const childPort = getPort(childType, childSide.port.guid);
        if (!parentPort || !childPort) {
          console.error(`Error during flatten: Ports not found for connection between ${parentId} and ${childId}. Parent Port: ${parentSide.port.guid}, Child Port: ${childSide.port.guid}`);
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

  // Return the diff between original expanded design and flattened design
  const updatedPieces = flatDesign.pieces
    ?.map((flatPiece) => {
      const originalPiece = expandedDesign.pieces?.find((p) => p.guid === flatPiece.guid);
      if (!originalPiece) return null;

      // Build piece diff for pieces that changed
      const pieceDiff: PieceDiff = {};
      if (flatPiece.plane !== originalPiece.plane) pieceDiff.plane = flatPiece.plane;
      if (flatPiece.center !== originalPiece.center) pieceDiff.center = flatPiece.center;
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

  const removedConnections = expandedDesign.connections?.map((c) => ({ connected: { piece: c.connected.piece.guid }, connecting: { piece: c.connecting.piece.guid } })) || [];

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
  design.connections?.forEach((conn) => {
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
  design.connections?.forEach((conn: Connection) => {
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

export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, qualities: true, folders: true, authors: true }).extend({
  types: z.array(z.string()).optional(),
  designs: z.array(z.string()).optional(),
  qualities: z.array(z.string()).optional(),
  folders: z.array(z.string()).optional(),
  authors: z.array(z.string()).optional(),
});
export type KitShallow = z.infer<typeof KitShallowSchema>;
export const serializeKitShallow = (kit: KitShallow): string => JSON.stringify(KitShallowSchema.parse(kit));
export const deserializeKitShallow = (json: string): KitShallow => KitShallowSchema.parse(JSON.parse(json));
export const KitDiffSchema = KitSchema.partial().omit({ types: true, designs: true, qualities: true, authors: true, files: true, folders: true }).extend({
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  qualities: QualitiesDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  files: FilesDiffSchema.optional(),
  folders: FoldersDiffSchema.optional(),
});
export type KitDiff = z.infer<typeof KitDiffSchema>;
export const getKitDiff = (before: Kit, after: Kit): KitDiff => {
  // TODO: Implement full Kit diff logic
  return {};
};
export const inverseKitDiff = (original: Kit, appliedDiff: KitDiff): KitDiff => {
  // TODO: Implement full Kit inverse diff logic
  return {};
};
export const mergeKitDiff = (diff1: KitDiff, diff2: KitDiff): KitDiff => {
  // TODO: Implement full Kit merge diff logic
  return { ...diff1, ...diff2 };
};
export const applyKitDiff = (base: Kit, diff: KitDiff): Kit => {
  // TODO: Implement full Kit apply diff logic
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
    concepts: diff.concepts ?? base.concepts,
    attributes: diff.attributes ?? base.attributes,
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
  attributes: [attribute],
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
      if (variants !== undefined && !variants.includes(replacementType.parent ?? "")) return false;
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
      if (variants !== undefined && !variants.includes(replacementType.parent ?? "")) return false;
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

export const findAttributeValue = (entity: Kit | Type | Design | Piece | Connection | Representation | Port, name: string, defaultValue?: string | null): string | null => {
  const attribute = entity.attributes?.find((q) => q.key === name);
  if (!attribute && defaultValue === undefined) throw new Error(`Attribute ${name} not found in ${entity}`);
  if (attribute?.value === undefined && defaultValue === null) return null;
  return attribute?.value ?? defaultValue ?? "";
};

const getColorForText = (text?: string): string => {
  if (!text || text === "") return "var(--foreground)";

  // Create a simple hash from the family string
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
          value: getColorForText(port.family),
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

// #endregion Kit
