// #region 🔖Header

// 💻 semio/js/semio.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

// #region 🔖TODOs

// TODOs
// TODO: Conventionalize error throwing and logging

// #endregion 🔖TODOs

import { ClassValue, clsx } from "clsx";
import cytoscape from "cytoscape";
import { default as adjectives } from "@semio/assets/lists/adjectives.json";
import { default as animals } from "@semio/assets/lists/animals.json";
import { twMerge } from "tailwind-merge";
import * as THREE from "three";
import { v7 as uuidv7 } from "uuid";
import { z } from "zod";
import CONSTANTS from "./constants.json";

// #region 🔖Constants

export const ICON_WIDTH = CONSTANTS.icon.width;
export const TOLERANCE = CONSTANTS.tolerance;

// #endregion 🔖Constants

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

  if (a == null && b == null) return true;
  if (a == null || b == null) return false;
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

export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);

export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476);
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476);
export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z);

export type Guid = string;

// #region 🔖Entity IDs

export type AttributeId = { guid: Guid };
export type LocationId = { guid: Guid };
export type AuthorId = { guid: Guid };
export type FileId = { guid: Guid };
export type FolderId = { guid: Guid };
export type BenchmarkId = { guid: Guid };
export type QualityId = { guid: Guid };
export type PortId = { guid: Guid };
export type PropId = { guid: Guid };
export type ModelId = { guid: Guid };
export type ConnectorId = { guid: Guid };
export type TypeId = { guid: Guid };
export type LayerId = { guid: Guid };
export type PieceId = { guid: Guid };
export type GroupId = { guid: Guid };
export type ConnectionId = { guid: Guid };
export type StatId = { guid: Guid };
export type DesignId = { guid: Guid };
export type KitId = { guid: Guid };
export type TagId = { guid: Guid };
export type ConceptId = { guid: Guid };

export const AttributeIdSchema = z.object({ guid: z.string() });
export const LocationIdSchema = z.object({ guid: z.string() });
export const AuthorIdSchema = z.object({ guid: z.string() });
export const FileIdSchema = z.object({ guid: z.string() });
export const FolderIdSchema = z.object({ guid: z.string() });
export const BenchmarkIdSchema = z.object({ guid: z.string() });
export const QualityIdSchema = z.object({ guid: z.string() });
export const PortIdSchema = z.object({ guid: z.string() });
export const PropIdSchema = z.object({ guid: z.string() });
export const ModelIdSchema = z.object({ guid: z.string() });
export const ConnectorIdSchema = z.object({ guid: z.string() });
export const TypeIdSchema = z.object({ guid: z.string() });
export const LayerIdSchema = z.object({ guid: z.string() });
export const PieceIdSchema = z.object({ guid: z.string() });
export const GroupIdSchema = z.object({ guid: z.string() });
export const ConnectionIdSchema = z.object({ guid: z.string() });
export const StatIdSchema = z.object({ guid: z.string() });
export const DesignIdSchema = z.object({ guid: z.string() });
export const KitIdSchema = z.object({ guid: z.string() });
export const TagIdSchema = z.object({ guid: z.string() });
export const ConceptIdSchema = z.object({ guid: z.string() });

export const createAttributeId = (guid: Guid): AttributeId => ({ guid });
export const createLocationId = (guid: Guid): LocationId => ({ guid });
export const createAuthorId = (guid: Guid): AuthorId => ({ guid });
export const createFileId = (guid: Guid): FileId => ({ guid });
export const createFolderId = (guid: Guid): FolderId => ({ guid });
export const createBenchmarkId = (guid: Guid): BenchmarkId => ({ guid });
export const createQualityId = (guid: Guid): QualityId => ({ guid });
export const createPortId = (guid: Guid): PortId => ({ guid });
export const createPropId = (guid: Guid): PropId => ({ guid });
export const createModelId = (guid: Guid): ModelId => ({ guid });
export const createConnectorId = (guid: Guid): ConnectorId => ({ guid });
export const createTypeId = (guid: Guid): TypeId => ({ guid });
export const createLayerId = (guid: Guid): LayerId => ({ guid });
export const createPieceId = (guid: Guid): PieceId => ({ guid });
export const createGroupId = (guid: Guid): GroupId => ({ guid });
export const createConnectionId = (guid: Guid): ConnectionId => ({ guid });
export const createStatId = (guid: Guid): StatId => ({ guid });
export const createDesignId = (guid: Guid): DesignId => ({ guid });
export const createKitId = (guid: Guid): KitId => ({ guid });
export const createTagId = (guid: Guid): TagId => ({ guid });
export const createConceptId = (guid: Guid): ConceptId => ({ guid });

export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.guid === b.guid;
export const areSameLocationId = (a: LocationId, b: LocationId): boolean => a.guid === b.guid;
export const areSameAuthorId = (a: AuthorId, b: AuthorId): boolean => a.guid === b.guid;
export const areSameFileId = (a: FileId, b: FileId): boolean => a.guid === b.guid;
export const areSameFolderId = (a: FolderId, b: FolderId): boolean => a.guid === b.guid;
export const areSameBenchmarkId = (a: BenchmarkId, b: BenchmarkId): boolean => a.guid === b.guid;
export const areSameQualityId = (a: QualityId, b: QualityId): boolean => a.guid === b.guid;
export const areSamePortId = (a: PortId, b: PortId): boolean => a.guid === b.guid;
export const areSamePropId = (a: PropId, b: PropId): boolean => a.guid === b.guid;
export const areSameModelId = (a: ModelId, b: ModelId): boolean => a.guid === b.guid;
export const areSameConnectorId = (a: ConnectorId, b: ConnectorId): boolean => a.guid === b.guid;
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.guid === b.guid;
export const areSameLayerId = (a: LayerId, b: LayerId): boolean => a.guid === b.guid;
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
export const areSameGroupId = (a: GroupId, b: GroupId): boolean => a.guid === b.guid;
export const areSameConnectionId = (a: ConnectionId, b: ConnectionId): boolean => a.guid === b.guid;
export const areSameStatId = (a: StatId, b: StatId): boolean => a.guid === b.guid;
export const areSameDesignId = (a: DesignId, b: DesignId): boolean => a.guid === b.guid;
export const areSameKitId = (a: KitId, b: KitId): boolean => a.guid === b.guid;
export const areSameTagId = (a: TagId, b: TagId): boolean => a.guid === b.guid;
export const areSameConceptId = (a: ConceptId, b: ConceptId): boolean => a.guid === b.guid;

export const getAttributeGuid = (id: AttributeId): Guid => id.guid;
export const getLocationGuid = (id: LocationId): Guid => id.guid;
export const getAuthorGuid = (id: AuthorId): Guid => id.guid;
export const getFileGuid = (id: FileId): Guid => id.guid;
export const getFolderGuid = (id: FolderId): Guid => id.guid;
export const getBenchmarkGuid = (id: BenchmarkId): Guid => id.guid;
export const getQualityGuid = (id: QualityId): Guid => id.guid;
export const getPortGuid = (id: PortId): Guid => id.guid;
export const getPropGuid = (id: PropId): Guid => id.guid;
export const getModelGuid = (id: ModelId): Guid => id.guid;
export const getConnectorGuid = (id: ConnectorId): Guid => id.guid;
export const getTypeGuid = (id: TypeId): Guid => id.guid;
export const getLayerGuid = (id: LayerId): Guid => id.guid;
export const getPieceGuid = (id: PieceId): Guid => id.guid;
export const getGroupGuid = (id: GroupId): Guid => id.guid;
export const getConnectionGuid = (id: ConnectionId): Guid => id.guid;
export const getStatGuid = (id: StatId): Guid => id.guid;
export const getDesignGuid = (id: DesignId): Guid => id.guid;
export const getKitGuid = (id: KitId): Guid => id.guid;
export const getTagGuid = (id: TagId): Guid => id.guid;
export const getConceptGuid = (id: ConceptId): Guid => id.guid;

// #endregion 🔖Entity IDs

const DateProperty = () => z.string().optional();

// #region 🔖Attribute

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
  const diff: AttributeDiff = {};
  if (before.key !== after.key) diff.key = after.key;
  if (before.value !== after.value) diff.value = after.value;
  if (before.definition !== after.definition) diff.definition = after.definition;
  return diff;
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
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(AttributeSchema).optional(),
});
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;

const getAttributesDiff = (before: Attribute[], after: Attribute[]): AttributesDiff => {
  const beforeGuids = new Set(before.map((a) => a.guid));
  const afterGuids = new Set(after.map((a) => a.guid));
  const removed = before.filter((a) => !afterGuids.has(a.guid)).map((a) => ({ guid: a.guid }));
  const added = after.filter((a) => !beforeGuids.has(a.guid));
  const updated = after
    .filter((a) => beforeGuids.has(a.guid))
    .map((a) => ({ attribute: { guid: a.guid }, diff: getAttributeDiff(before.find((b) => b.guid === a.guid)!, a) }))
    .filter((u) => Object.keys(u.diff).length > 0);
  const diff: AttributesDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

export const inverseAttributesDiff = (original: Attribute[], appliedDiff: AttributesDiff): AttributesDiff => {
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  const updatedGuids = appliedDiff.updated?.map((a) => a.attribute.guid) ?? [];
  const addedGuids = appliedDiff.added?.map((a) => a.guid) ?? [];
  return {
    removed: addedGuids.map((guid) => ({ guid })),
    updated: updatedGuids
      .map((guid) => {
        const orig = original.find((a) => a.guid === guid);
        const upd = appliedDiff.updated?.find((a) => a.attribute.guid === guid);
        if (!orig || !upd) return null;
        return { attribute: { guid }, diff: inverseAttributeDiff(orig, upd.diff) };
      })
      .filter((item): item is { attribute: AttributeId; diff: AttributeDiff } => item !== null),
    added: removedGuids.map((guid) => original.find((a) => a.guid === guid)!).filter((a) => a !== undefined),
  };
};

export const mergeAttributesDiff = (first: AttributesDiff, second: AttributesDiff): AttributesDiff => {
  return { ...first, ...second };
};

export const applyAttributesDiff = (base: Attribute[], diff: AttributesDiff): Attribute[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((attr) => !removedGuids.has(attr.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((attr) => attr.guid === update.attribute.guid);
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

// #endregion 🔖Attribute

// #region 🔖Coord (weak entity)

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

// #endregion 🔖Coord (weak entity)

// #region 🔖Vec (weak entity)

export const VecSchema = z.object({ u: z.number(), v: z.number() });
export type Vec = z.infer<typeof VecSchema>;
export const serializeVec = (vec: Vec): string => JSON.stringify(VecSchema.parse(vec));
export const deserializeVec = (json: string): Vec => VecSchema.parse(JSON.parse(json));

export const VecDiffSchema = VecSchema.partial();
export type VecDiff = z.infer<typeof VecDiffSchema>;
export const getVecDiff = (before: Vec, after: Vec): VecDiff => {
  return {
    u: after.u - before.u,
    v: after.v - before.v,
  };
};
export const inverseVecDiff = (original: Vec, appliedDiff: VecDiff): VecDiff => {
  const u = appliedDiff.u ?? 0;
  const v = appliedDiff.v ?? 0;
  return {
    u: original.u - u,
    v: original.v - v,
  };
};
export const mergeVecDiff = (diff1: VecDiff, diff2: VecDiff): VecDiff => {
  return {
    u: (diff1.u ?? 0) + (diff2.u ?? 0),
    v: (diff1.v ?? 0) + (diff2.v ?? 0),
  };
};
export const applyVecDiff = (base: Vec, diff: VecDiff): Vec => {
  const u = diff.u ?? 0;
  const v = diff.v ?? 0;
  return {
    u: base.u + u,
    v: base.v + v,
  };
};

// #endregion 🔖Vec (weak entity)

// #region 🔖Point (weak entity)

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

// #endregion 🔖Point (weak entity)

// #region 🔖Vector (weak entity)

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

// #endregion 🔖Vector (weak entity)

// #region 🔖Plane (weak entity)

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

export const averagePlane = (planes: Plane[]): Plane | null => {
  if (planes.length === 0) return null;
  if (planes.length === 1) return planes[0];

  const avgOrigin = planes.reduce(
    (acc, plane) => ({
      x: acc.x + plane.origin.x / planes.length,
      y: acc.y + plane.origin.y / planes.length,
      z: acc.z + plane.origin.z / planes.length,
    }),
    { x: 0, y: 0, z: 0 },
  );

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

// #endregion 🔖Plane (weak entity)

// #region 🔖Camera (weak entity)

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

// #endregion 🔖Camera (weak entity)

// #region 🔖Location

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
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Location = {
    guid: base.guid,
    longitude: diff.longitude ?? base.longitude,
    latitude: diff.latitude ?? base.latitude,
    altitude: diff.altitude ?? base.altitude,
  };

  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

// #endregion 🔖Location

// #region 🔖Author

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
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Author = {
    guid: base.guid,
    name: diff.name ?? base.name,
    email: diff.email ?? base.email,
  };

  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const AuthorsDiffSchema = z.object({
  removed: z.array(AuthorIdSchema).optional(),
  updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(),
  added: z.array(AuthorSchema).optional(),
});
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;

// #endregion 🔖Author

// #region 🔖File

export const FileSchema = z.object({
  guid: z.string(),
  name: z.string(),
  mime: z.string().optional(),
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
  if (before.mime !== after.mime) diff.mime = after.mime;
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
  if (appliedDiff.mime !== undefined) inverse.mime = original.mime;
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
  const result: File = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.mime !== undefined || base.mime !== undefined) result.mime = diff.mime ?? base.mime;
  if (diff.remote !== undefined || base.remote !== undefined) result.remote = diff.remote ?? base.remote;
  if (diff.size !== undefined || base.size !== undefined) result.size = diff.size ?? base.size;
  if (diff.hash !== undefined || base.hash !== undefined) result.hash = diff.hash ?? base.hash;
  if (diff.createdAt !== undefined || base.createdAt !== undefined) result.createdAt = diff.createdAt ?? base.createdAt;
  if (diff.createdBy !== undefined || base.createdBy !== undefined) result.createdBy = diff.createdBy ?? base.createdBy;
  if (diff.updatedAt !== undefined || base.updatedAt !== undefined) result.updatedAt = diff.updatedAt ?? base.updatedAt;
  if (diff.updatedBy !== undefined || base.updatedBy !== undefined) result.updatedBy = diff.updatedBy ?? base.updatedBy;
  if (diff.folder !== undefined || base.folder !== undefined) result.folder = diff.folder ?? base.folder;

  return result;
};

export const FilesDiffSchema = z.object({
  removed: z.array(FileIdSchema).optional(),
  updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(),
  added: z.array(FileSchema).optional(),
});
export type FilesDiff = z.infer<typeof FilesDiffSchema>;

// #endregion 🔖File

// #region 🔖Folder

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
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Folder = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.parent !== undefined || base.parent !== undefined) result.parent = diff.parent ?? base.parent;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;
  if (diff.createdAt !== undefined || base.createdAt !== undefined) result.createdAt = diff.createdAt ?? base.createdAt;
  if (diff.createdBy !== undefined || base.createdBy !== undefined) result.createdBy = diff.createdBy ?? base.createdBy;
  if (diff.updatedAt !== undefined || base.updatedAt !== undefined) result.updatedAt = diff.updatedAt ?? base.updatedAt;
  if (diff.updatedBy !== undefined || base.updatedBy !== undefined) result.updatedBy = diff.updatedBy ?? base.updatedBy;

  return result;
};

export const FoldersDiffSchema = z.object({
  removed: z.array(FolderIdSchema).optional(),
  updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(),
  added: z.array(FolderSchema).optional(),
});
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;

// #endregion 🔖Folder

// #region 🔖Benchmark

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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Benchmark = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.icon !== undefined || base.icon !== undefined) result.icon = diff.icon ?? base.icon;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.minExcluded !== undefined || base.minExcluded !== undefined) result.minExcluded = diff.minExcluded ?? base.minExcluded;
  if (diff.max !== undefined || base.max !== undefined) result.max = diff.max ?? base.max;
  if (diff.maxExcluded !== undefined || base.maxExcluded !== undefined) result.maxExcluded = diff.maxExcluded ?? base.maxExcluded;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};
export const getBenchmarkDiff = (before: Benchmark, after: Benchmark): BenchmarkDiff => {
  const diff: BenchmarkDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.min !== after.min) diff.min = after.min !== undefined && before.min !== undefined ? after.min - before.min : after.min;
  if (before.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
  if (before.max !== after.max) diff.max = after.max !== undefined && before.max !== undefined ? after.max - before.max : after.max;
  if (before.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  removed: z.array(BenchmarkIdSchema).optional(),
  updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(),
  added: z.array(BenchmarkSchema).optional(),
});
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;

const getBenchmarksDiff = (before: Benchmark[], after: Benchmark[]): BenchmarksDiff => {
  const beforeGuids = new Set(before.map((b) => b.guid));
  const afterGuids = new Set(after.map((b) => b.guid));
  const removed = before.filter((b) => !afterGuids.has(b.guid)).map((b) => ({ guid: b.guid }));
  const added = after.filter((b) => !beforeGuids.has(b.guid));
  const updated = after
    .filter((b) => beforeGuids.has(b.guid))
    .map((afterBenchmark) => {
      const beforeBenchmark = before.find((b) => b.guid === afterBenchmark.guid)!;
      const diff = getBenchmarkDiff(beforeBenchmark, afterBenchmark);
      return { benchmark: { guid: afterBenchmark.guid }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: BenchmarksDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

const inverseBenchmarksDiff = (original: Benchmark[], appliedDiff: BenchmarksDiff): BenchmarksDiff => {
  const addedGuids = appliedDiff.added?.map((b) => b.guid) ?? [];
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  const updatedGuids = appliedDiff.updated?.map((u) => u.benchmark.guid) ?? [];
  return {
    removed: addedGuids.map((guid) => ({ guid })),
    added: original.filter((b) => removedGuids.includes(b.guid)),
    updated: updatedGuids.map((guid) => {
      const orig = original.find((b) => b.guid === guid)!;
      const upd = appliedDiff.updated?.find((u) => u.benchmark.guid === guid)!;
      return { benchmark: { guid }, diff: inverseBenchmarkDiff(orig, upd.diff) };
    }),
  };
};

const mergeBenchmarksDiff = (first: BenchmarksDiff, second: BenchmarksDiff): BenchmarksDiff => {
  return { ...first, ...second };
};

const applyBenchmarksDiff = (base: Benchmark[], diff: BenchmarksDiff): Benchmark[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((benchmark) => !removedGuids.has(benchmark.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((benchmark) => benchmark.guid === update.benchmark.guid);
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

// #endregion 🔖Benchmark

// #region 🔖Quality

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
  if (!deepEqual(before.benchmarks, after.benchmarks)) diff.benchmarks = getBenchmarksDiff(before.benchmarks ?? [], after.benchmarks ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const benchmarks = diff.benchmarks ? applyBenchmarksDiff(base.benchmarks ?? [], diff.benchmarks) : undefined;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Quality = {
    guid: base.guid,
    key: diff.key ?? base.key,
    name: diff.name ?? base.name,
  };

  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (diff.uri !== undefined || base.uri !== undefined) result.uri = diff.uri ?? base.uri;
  if (diff.kind !== undefined || base.kind !== undefined) result.kind = diff.kind ?? base.kind;
  if (diff.folder !== undefined || base.folder !== undefined) result.folder = diff.folder ?? base.folder;
  if (diff.canScale !== undefined || base.canScale !== undefined) result.canScale = diff.canScale ?? base.canScale;
  if (diff.defaultSiUnit !== undefined || base.defaultSiUnit !== undefined) result.defaultSiUnit = diff.defaultSiUnit ?? base.defaultSiUnit;
  if (diff.defaultImperialUnit !== undefined || base.defaultImperialUnit !== undefined) result.defaultImperialUnit = diff.defaultImperialUnit ?? base.defaultImperialUnit;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.isMinExcluded !== undefined || base.isMinExcluded !== undefined) result.isMinExcluded = diff.isMinExcluded ?? base.isMinExcluded;
  if (diff.max !== undefined || base.max !== undefined) result.max = diff.max ?? base.max;
  if (diff.isMaxExcluded !== undefined || base.isMaxExcluded !== undefined) result.isMaxExcluded = diff.isMaxExcluded ?? base.isMaxExcluded;
  if (diff.defaultValue !== undefined || base.defaultValue !== undefined) result.defaultValue = diff.defaultValue ?? base.defaultValue;
  if (diff.formula !== undefined || base.formula !== undefined) result.formula = diff.formula ?? base.formula;
  if (diff.icon !== undefined || base.icon !== undefined) result.icon = diff.icon ?? base.icon;
  if (diff.image !== undefined || base.image !== undefined) result.image = diff.image ?? base.image;
  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (benchmarks && benchmarks.length > 0) result.benchmarks = benchmarks;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const QualitiesDiffSchema = z.object({
  removed: z.array(QualityIdSchema).optional(),
  updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(),
  added: z.array(QualitySchema).optional(),
});

// #endregion 🔖Quality

// #region 🔖Port

export const PortSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  compatiblePorts: z.array(PortIdSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Port = z.infer<typeof PortSchema>;
export const serializePort = (iface: Port): string => JSON.stringify(PortSchema.parse(iface));
export const deserializePort = (json: string): Port => PortSchema.parse(JSON.parse(json));

export const PortDiffSchema = PortSchema.partial()
  .omit({ compatiblePorts: true, attributes: true })
  .extend({
    compatiblePorts: z.array(PortIdSchema).optional(),
    attributes: AttributesDiffSchema.optional(),
    description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
  });
export type PortDiff = z.infer<typeof PortDiffSchema>;
export const getPortDiff = (before: Port, after: Port): PortDiff => {
  const diff: PortDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (JSON.stringify(before.compatiblePorts) !== JSON.stringify(after.compatiblePorts)) diff.compatiblePorts = after.compatiblePorts;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inversePortDiff = (original: Port, appliedDiff: PortDiff): PortDiff => {
  const inverse: PortDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.compatiblePorts !== undefined) inverse.compatiblePorts = original.compatiblePorts;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergePortDiff = (diff1: PortDiff, diff2: PortDiff): PortDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
export const applyPortDiff = (base: Port, diff: PortDiff): Port => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Port = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if ("description" in diff) {
    if (diff.description !== null) result.description = diff.description;
  } else if (base.description !== undefined) {
    result.description = base.description;
  }
  if ("icon" in diff) {
    if (diff.icon !== null) result.icon = diff.icon;
  } else if (base.icon !== undefined) {
    result.icon = base.icon;
  }
  if (diff.compatiblePorts !== undefined || base.compatiblePorts !== undefined) result.compatiblePorts = diff.compatiblePorts ?? base.compatiblePorts;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(),
  added: z.array(PortSchema).optional(),
});
export type PortsDiff = z.infer<typeof PortsDiffSchema>;
export const getPortsDiff = (before: Port[], after: Port[]): PortsDiff => {
  const diff: PortsDiff = {};
  const beforeGuids = new Set(before.map((i) => i.guid));
  const afterGuids = new Set(after.map((i) => i.guid));
  const removed = before.filter((i) => !afterGuids.has(i.guid)).map((i) => ({ guid: i.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterGuids.has(i.guid))
    .map((i) => {
      const afterPort = after.find((a) => a.guid === i.guid)!;
      const portDiff = getPortDiff(i, afterPort);
      return { port: { guid: i.guid }, diff: portDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeGuids.has(i.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
export const inversePortsDiff = (original: Port[], appliedDiff: PortsDiff): PortsDiff => {
  const inverse: PortsDiff = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedGuids.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ guid: i.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalPort = original.find((i) => i.guid === u.port.guid)!;
      return { port: { guid: u.port.guid }, diff: inversePortDiff(originalPort, u.diff) };
    });
  }
  return inverse;
};
export const mergePortsDiff = (diff1: PortsDiff, diff2: PortsDiff): PortsDiff => {
  return {
    removed: [...(diff1.removed ?? []), ...(diff2.removed ?? [])],
    updated: [...(diff1.updated ?? []), ...(diff2.updated ?? [])],
    added: [...(diff1.added ?? []), ...(diff2.added ?? [])],
  };
};
export const applyPortsDiff = (base: Port[], diff: PortsDiff): Port[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((i) => !removedGuids.has(i.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((i) => i.guid === update.port.guid);
      if (index !== -1) {
        result[index] = applyPortDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

export const arePortsCompatible = (iface1: Port | undefined, iface2: Port | undefined, allPorts: Port[]): boolean => {
  if (!iface1 || !iface2) return true;
  if (iface1.guid === iface2.guid) return true;
  const iface1Compatible = iface1.compatiblePorts ?? [];
  const iface2Compatible = iface2.compatiblePorts ?? [];
  if (iface1Compatible.length === 0 && iface2Compatible.length === 0) return true;
  if (iface1Compatible.length === 0) return iface2Compatible.some((c) => c.guid === iface1.guid);
  if (iface2Compatible.length === 0) return iface1Compatible.some((c) => c.guid === iface2.guid);
  return iface1Compatible.some((c) => c.guid === iface2.guid) || iface2Compatible.some((c) => c.guid === iface1.guid);
};

// #endregion 🔖Port

// #region 🔖Prop

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
  if (before.quality?.guid !== after.quality?.guid) diff.quality = after.quality;
  if (before.value !== after.value) diff.value = after.value;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Prop = {
    guid: base.guid,
    quality: diff.quality ?? base.quality,
    value: diff.value ?? base.value,
  };

  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const PropsDiffSchema = z.object({
  removed: z.array(PropIdSchema).optional(),
  updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(),
  added: z.array(PropSchema).optional(),
});
export type PropsDiff = z.infer<typeof PropsDiffSchema>;

const getPropsDiff = (before: Prop[], after: Prop[]): PropsDiff => {
  const beforeGuids = new Set(before.map((p) => p.guid));
  const afterGuids = new Set(after.map((p) => p.guid));
  const removed = before.filter((p) => !afterGuids.has(p.guid)).map((p) => ({ guid: p.guid }));
  const added = after.filter((p) => !beforeGuids.has(p.guid));
  const updated = after
    .filter((p) => beforeGuids.has(p.guid))
    .map((afterProp) => {
      const beforeProp = before.find((p) => p.guid === afterProp.guid)!;
      const diff = getPropDiff(beforeProp, afterProp);
      return { prop: { guid: afterProp.guid }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: PropsDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

const inversePropsDiff = (original: Prop[], appliedDiff: PropsDiff): PropsDiff => {
  const addedGuids = appliedDiff.added?.map((p) => p.guid) ?? [];
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  const updatedGuids = appliedDiff.updated?.map((u) => u.prop.guid) ?? [];
  return {
    removed: addedGuids.map((guid) => ({ guid })),
    added: original.filter((p) => removedGuids.includes(p.guid)),
    updated: updatedGuids.map((guid) => {
      const orig = original.find((p) => p.guid === guid)!;
      const upd = appliedDiff.updated?.find((u) => u.prop.guid === guid)!;
      return { prop: { guid }, diff: inversePropDiff(orig, upd.diff) };
    }),
  };
};

const mergePropsDiff = (first: PropsDiff, second: PropsDiff): PropsDiff => {
  return { ...first, ...second };
};

const applyPropsDiff = (base: Prop[], diff: PropsDiff): Prop[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((prop) => !removedGuids.has(prop.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((prop) => prop.guid === update.prop.guid);
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

// #endregion 🔖Prop

// #region 🔖Tag

export const TagSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Tag = z.infer<typeof TagSchema>;
export const serializeTag = (tag: Tag): string => JSON.stringify(TagSchema.parse(tag));
export const deserializeTag = (json: string): Tag => TagSchema.parse(JSON.parse(json));

export const TagDiffSchema = TagSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
export type TagDiff = z.infer<typeof TagDiffSchema>;
export const getTagDiff = (before: Tag, after: Tag): TagDiff => {
  const diff: TagDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseTagDiff = (original: Tag, appliedDiff: TagDiff): TagDiff => {
  const inverse: TagDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeTagDiff = (diff1: TagDiff, diff2: TagDiff): TagDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyTagDiff = (base: Tag, diff: TagDiff): Tag => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Tag = {
    guid: base.guid,
    name: "name" in diff && diff.name !== undefined ? diff.name : base.name,
  };

  if ("description" in diff) {
    const value = diff.description ?? undefined;
    if (value !== undefined) result.description = value;
  } else if (base.description !== undefined) {
    result.description = base.description;
  }
  if ("icon" in diff) {
    const value = diff.icon ?? undefined;
    if (value !== undefined) result.icon = value;
  } else if (base.icon !== undefined) {
    result.icon = base.icon;
  }
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const TagsDiffSchema = z.object({
  removed: z.array(TagIdSchema).optional(),
  updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(),
  added: z.array(TagSchema).optional(),
});
export type TagsDiff = z.infer<typeof TagsDiffSchema>;
export const getTagsDiff = (before: Tag[], after: Tag[]): TagsDiff => {
  const diff: TagsDiff = {};
  const beforeGuids = new Set(before.map((t) => t.guid));
  const afterGuids = new Set(after.map((t) => t.guid));
  const removed = before.filter((t) => !afterGuids.has(t.guid)).map((t) => ({ guid: t.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((t) => afterGuids.has(t.guid))
    .map((t) => {
      const afterTag = after.find((a) => a.guid === t.guid)!;
      const tagDiff = getTagDiff(t, afterTag);
      return { tag: { guid: t.guid }, diff: tagDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((t) => !beforeGuids.has(t.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
export const inverseTagsDiff = (original: Tag[], appliedDiff: TagsDiff): TagsDiff => {
  const inverse: TagsDiff = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((t) => removedGuids.includes(t.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((t) => ({ guid: t.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalTag = original.find((t) => t.guid === u.tag.guid)!;
      return { tag: { guid: u.tag.guid }, diff: inverseTagDiff(originalTag, u.diff) };
    });
  }
  return inverse;
};
export const mergeTagsDiff = (diff1: TagsDiff, diff2: TagsDiff): TagsDiff => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [u.tag.guid, u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [u.tag.guid, u.diff]));
  const allGuids = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allGuids).map((guid) => ({
    tag: { guid },
    diff: mergeTagDiff(updated1Map.get(guid) ?? {}, updated2Map.get(guid) ?? {}),
  }));
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
};
export const applyTagsDiff = (base: Tag[], diff: TagsDiff): Tag[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((t) => !removedGuids.has(t.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((t) => t.guid === update.tag.guid);
      if (index !== -1) {
        result[index] = applyTagDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

export const findTag = (tags: Tag[], guid: string): Tag => {
  const tag = tags.find((t) => t.guid === guid);
  if (!tag) throw new Error(`Tag ${guid} not found`);
  return tag;
};

// #endregion 🔖Tag

// #region 🔖Concept

export const ConceptSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Concept = z.infer<typeof ConceptSchema>;
export const serializeConcept = (concept: Concept): string => JSON.stringify(ConceptSchema.parse(concept));
export const deserializeConcept = (json: string): Concept => ConceptSchema.parse(JSON.parse(json));

export const ConceptDiffSchema = ConceptSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
export type ConceptDiff = z.infer<typeof ConceptDiffSchema>;
export const getConceptDiff = (before: Concept, after: Concept): ConceptDiff => {
  const diff: ConceptDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const inverseConceptDiff = (original: Concept, appliedDiff: ConceptDiff): ConceptDiff => {
  const inverse: ConceptDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeConceptDiff = (diff1: ConceptDiff, diff2: ConceptDiff): ConceptDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
export const applyConceptDiff = (base: Concept, diff: ConceptDiff): Concept => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Concept = {
    guid: base.guid,
    name: "name" in diff && diff.name !== undefined ? diff.name : base.name,
  };

  if ("description" in diff) {
    const value = diff.description ?? undefined;
    if (value !== undefined) result.description = value;
  } else if (base.description !== undefined) {
    result.description = base.description;
  }
  if ("icon" in diff) {
    const value = diff.icon ?? undefined;
    if (value !== undefined) result.icon = value;
  } else if (base.icon !== undefined) {
    result.icon = base.icon;
  }
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const ConceptsDiffSchema = z.object({
  removed: z.array(ConceptIdSchema).optional(),
  updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(),
  added: z.array(ConceptSchema).optional(),
});
export type ConceptsDiff = z.infer<typeof ConceptsDiffSchema>;
export const getConceptsDiff = (before: Concept[], after: Concept[]): ConceptsDiff => {
  const diff: ConceptsDiff = {};
  const beforeGuids = new Set(before.map((c) => c.guid));
  const afterGuids = new Set(after.map((c) => c.guid));
  const removed = before.filter((c) => !afterGuids.has(c.guid)).map((c) => ({ guid: c.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((c) => afterGuids.has(c.guid))
    .map((c) => {
      const afterConcept = after.find((a) => a.guid === c.guid)!;
      const conceptDiff = getConceptDiff(c, afterConcept);
      return { concept: { guid: c.guid }, diff: conceptDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((c) => !beforeGuids.has(c.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};
export const inverseConceptsDiff = (original: Concept[], appliedDiff: ConceptsDiff): ConceptsDiff => {
  const inverse: ConceptsDiff = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((c) => removedGuids.includes(c.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((c) => ({ guid: c.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalConcept = original.find((c) => c.guid === u.concept.guid)!;
      return { concept: { guid: u.concept.guid }, diff: inverseConceptDiff(originalConcept, u.diff) };
    });
  }
  return inverse;
};
export const mergeConceptsDiff = (diff1: ConceptsDiff, diff2: ConceptsDiff): ConceptsDiff => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [u.concept.guid, u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [u.concept.guid, u.diff]));
  const allGuids = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allGuids).map((guid) => ({
    concept: { guid },
    diff: mergeConceptDiff(updated1Map.get(guid) ?? {}, updated2Map.get(guid) ?? {}),
  }));
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
};
export const applyConceptsDiff = (base: Concept[], diff: ConceptsDiff): Concept[] => {
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((c) => !removedGuids.has(c.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const index = result.findIndex((c) => c.guid === update.concept.guid);
      if (index !== -1) {
        result[index] = applyConceptDiff(result[index], update.diff);
      }
    }
  }
  if (diff.added) {
    result.push(...diff.added);
  }
  return result;
};

export const findConcept = (concepts: Concept[], guid: string): Concept => {
  const concept = concepts.find((c) => c.guid === guid);
  if (!concept) throw new Error(`Concept ${guid} not found`);
  return concept;
};

// #endregion 🔖Concept

// #region 🔖Model

export const ModelSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  tags: z.array(TagIdSchema).optional(),
  file: FileIdSchema,
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
  if (before.file.guid !== after.file.guid) diff.file = after.file;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Model = {
    guid: base.guid,
    file: diff.file ?? base.file,
  };

  if (diff.name !== undefined || base.name !== undefined) result.name = diff.name ?? base.name;
  if (diff.tags !== undefined || base.tags !== undefined) result.tags = diff.tags ?? base.tags;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const ModelsDiffSchema = z.object({
  removed: z.array(ModelIdSchema).optional(),
  updated: z.array(z.object({ model: ModelIdSchema, diff: ModelDiffSchema })).optional(),
  added: z.array(ModelSchema).optional(),
});

export const areSameModel = (model: Model, other: Model): boolean => {
  const modelTagGuids = model.tags?.map((t) => t.guid) ?? [];
  const otherTagGuids = other.tags?.map((t) => t.guid) ?? [];
  return modelTagGuids.every((guid) => otherTagGuids.includes(guid));
};

export const findModel = (models: Model[], tagGuids: string[]): Model => {
  const indices = models.map((r) =>
    jaccard(
      r.tags?.map((t) => t.guid),
      tagGuids,
    ),
  );
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return models[maxIndexIndex];
};

export const getAllTagGuidsFromModels = (models: Model[]): string[] => {
  const tagsSet = new Set<string>();
  models.forEach((r) => {
    toArray(r.tags).forEach((tag) => tagsSet.add(tag.guid));
  });
  return Array.from(tagsSet).sort();
};

export const filterModelsByTagGuids = (models: Model[], selectedTagGuids: string[]): Model[] => {
  if (!selectedTagGuids || selectedTagGuids.length === 0) return models;
  return models.filter((r) => {
    if (!r.tags || r.tags.length === 0) return false;
    const modelTagGuids = r.tags.map((t) => t.guid);
    return selectedTagGuids.every((guid) => modelTagGuids.includes(guid));
  });
};

export const getAvailableTagGuidsForModels = (models: Model[], selectedTagGuids: string[]): string[] => {
  const filteredReps = filterModelsByTagGuids(models, selectedTagGuids);
  const availableTags = getAllTagGuidsFromModels(filteredReps);
  return availableTags.filter((guid) => !selectedTagGuids.includes(guid));
};

export const selectBestModel = (models: Model[], selectedTagGuids: string[]): Model | undefined => {
  if (models.length === 0) return undefined;
  if (selectedTagGuids.length === 0) {
    const defaultRep = models.find((r) => !r.tags || r.tags.length === 0);
    return defaultRep ?? models[0];
  }
  const filteredReps = filterModelsByTagGuids(models, selectedTagGuids);
  if (filteredReps.length === 0) return undefined;
  return findModel(filteredReps, selectedTagGuids);
};

export const SUPPORTED_3D_EXTENSIONS = [
  "gltf",
  "glb",

  "fbx",

  "obj",

  "dae",

  "3ds",

  "stl",

  "ply",

  "usdz",

  "vrm",

  "ifc",

  "3mf",

  "amf",

  "bvh",

  "drc",

  "ktx2",

  "ldr",
  "mpd",

  "json",

  "pmd",
  "pmx",
  "vmd",

  "pcd",

  "pdb",

  "svg",

  "tilt",

  "vox",

  "wrl",

  "xyz",
] as const;

export type Supported3DExtension = (typeof SUPPORTED_3D_EXTENSIONS)[number];

export const isSupportedModelExtension = (filename: string): boolean => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  return SUPPORTED_3D_EXTENSIONS.includes(ext as Supported3DExtension);
};

export interface ModelFileValidation {
  isValid: boolean;
  warning?: string;
  extension?: string;
}

export const validateModelFile = (filename: string): ModelFileValidation => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  if (!ext) {
    return { isValid: false, warning: "File has no extension" };
  }
  if (!isSupportedModelExtension(filename)) {
    return {
      isValid: true,
      warning: `File extension '.${ext}' is not a common 3D format. Supported: ${SUPPORTED_3D_EXTENSIONS.slice(0, 5).join(", ")}...`,
      extension: ext,
    };
  }
  return { isValid: true, extension: ext };
};

// #endregion 🔖Model

// #region 🔖Connector

export const ConnectorSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  t: z.number(),
  point: PointSchema,
  direction: VectorSchema,
  description: z.string().optional(),
  port: PortIdSchema.optional(),
  mandatory: z.boolean().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Connector = z.infer<typeof ConnectorSchema>;
export const serializeConnector = (connector: Connector): string => JSON.stringify(ConnectorSchema.parse(connector));
export const deserializeConnector = (json: string): Connector => ConnectorSchema.parse(JSON.parse(json));

export const ConnectorDiffSchema = ConnectorSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({
  point: PointDiffSchema.optional(),
  direction: VectorDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type ConnectorDiff = z.infer<typeof ConnectorDiffSchema>;
export const getConnectorDiff = (before: Connector, after: Connector): ConnectorDiff => {
  const diff: ConnectorDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.port?.guid !== after.port?.guid) diff.port = after.port;
  if (before.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
  if (before.t !== after.t) diff.t = after.t;
  if (!deepEqual(before.point, after.point)) diff.point = getPointDiff(before.point, after.point);
  if (!deepEqual(before.direction, after.direction)) diff.direction = getVectorDiff(before.direction, after.direction);
  if (!deepEqual(before.props, after.props)) diff.props = getPropsDiff(before.props ?? [], after.props ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
export const mergeConnectorDiff = (diff1: ConnectorDiff, diff2: ConnectorDiff): ConnectorDiff => {
  return {
    ...diff1,
    ...diff2,
    point: diff2.point ?? diff1.point,
    direction: diff2.direction ?? diff1.direction,
    props: diff1.props && diff2.props ? mergePropsDiff(diff1.props, diff2.props) : (diff2.props ?? diff1.props),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
export const inverseConnectorDiff = (original: Connector, appliedDiff: ConnectorDiff): ConnectorDiff => {
  const inverse: ConnectorDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.port !== undefined) inverse.port = original.port;
  if (appliedDiff.mandatory !== undefined) inverse.mandatory = original.mandatory;
  if (appliedDiff.t !== undefined) inverse.t = original.t;
  if (appliedDiff.point !== undefined) inverse.point = inversePointDiff(original.point, appliedDiff.point);
  if (appliedDiff.direction !== undefined) inverse.direction = inverseVectorDiff(original.direction, appliedDiff.direction);
  if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(original.props ?? [], appliedDiff.props);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const applyConnectorDiff = (base: Connector, diff: ConnectorDiff): Connector => {
  const props = diff.props ? applyPropsDiff(base.props ?? [], diff.props) : undefined;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Connector = {
    guid: base.guid,
    t: diff.t ?? base.t,
    point: diff.point ? applyPointDiff(base.point, diff.point) : base.point,
    direction: diff.direction ? applyVectorDiff(base.direction, diff.direction) : base.direction,
  };

  if (diff.name !== undefined || base.name !== undefined) result.name = diff.name ?? base.name;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (diff.port !== undefined || base.port !== undefined) result.port = diff.port ?? base.port;
  if (diff.mandatory !== undefined || base.mandatory !== undefined) result.mandatory = diff.mandatory ?? base.mandatory;
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const ConnectorsDiffSchema = z.object({
  removed: z.array(ConnectorIdSchema).optional(),
  updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(),
  added: z.array(ConnectorSchema).optional(),
});
export type ConnectorsDiff = z.infer<typeof ConnectorsDiffSchema>;

const getConnectorsDiff = (before: Connector[], after: Connector[]): ConnectorsDiff => {
  const beforeGuids = new Set(before.map((p) => p.guid));
  const afterGuids = new Set(after.map((p) => p.guid));
  const removed = before.filter((p) => !afterGuids.has(p.guid)).map((p) => ({ guid: p.guid }));
  const added = after.filter((p) => !beforeGuids.has(p.guid));
  const updated = after
    .filter((p) => beforeGuids.has(p.guid))
    .map((afterPort) => {
      const beforePort = before.find((p) => p.guid === afterPort.guid)!;
      const diff = getConnectorDiff(beforePort, afterPort);
      return { connector: { guid: afterPort.guid }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: ConnectorsDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

export const unifyConnectorPortsAndCompatiblePortsForTypes = (types: Type[]): TypesDiff => {
  return { updated: [] };
};

export const areConnectorsCompatible = (connector: Connector, otherPort: Connector): boolean => {
  return true;
};

export const findConnector = (connectors: Connector[], connectorGuid: string): Connector => {
  const connector = connectors.find((p) => p.guid === connectorGuid);
  if (!connector) throw new Error(`Connector ${connectorGuid} not found in connectors`);
  return connector;
};

// #endregion 🔖Connector

// #region 🔖Type

export const TypeSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: TypeIdSchema.optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  models: z.array(ModelSchema).optional(),
  connectors: z.array(ConnectorSchema).optional(),
  props: z.array(PropSchema).optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Type = z.infer<typeof TypeSchema>;
export const serializeType = (type: Type): string => JSON.stringify(TypeSchema.parse(type));
export const deserializeType = (json: string): Type => TypeSchema.parse(JSON.parse(json));

export const TypeShallowSchema = TypeSchema.omit({ models: true, connectors: true }).extend({
  models: z.array(z.string()).optional(),
  connectors: z.array(z.string()).optional(),
});
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
export const serializeTypeShallow = (type: TypeShallow): string => JSON.stringify(TypeShallowSchema.parse(type));
export const deserializeTypeShallow = (json: string): TypeShallow => TypeShallowSchema.parse(JSON.parse(json));
export const TypeDiffSchema = TypeSchema.partial()
  .omit({ models: true, connectors: true, props: true, attributes: true })
  .extend({
    models: ModelsDiffSchema.optional(),
    connectors: ConnectorsDiffSchema.optional(),
    props: PropsDiffSchema.optional(),
    attributes: AttributesDiffSchema.optional(),
    description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
    image: z.string().nullable().optional(),
    location: LocationIdSchema.nullable().optional(),
    folder: z.string().nullable().optional(),
    concepts: z.array(ConceptIdSchema).nullable().optional(),
    authors: z.array(AuthorIdSchema).nullable().optional(),
    parent: TypeIdSchema.nullable().optional(),
  });
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
export const getTypeDiff = (before: Type, after: Type): TypeDiff => {
  const diff: TypeDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.parent?.guid !== after.parent?.guid) diff.parent = after.parent;
  if (before.isAbstract !== after.isAbstract) diff.isAbstract = after.isAbstract;
  if (before.folder !== after.folder) diff.folder = after.folder;
  if (before.stock !== after.stock) diff.stock = after.stock;
  if (before.virtual !== after.virtual) diff.virtual = after.virtual;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.location?.guid !== after.location?.guid) diff.location = after.location;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.description !== after.description) diff.description = after.description;
  if (!arraysEqual(before.authors, after.authors)) diff.authors = after.authors;
  if (!arraysEqual(before.concepts, after.concepts)) diff.concepts = after.concepts;
  const modelsDiff = getCollectionDiff("model", before.models ?? [], after.models ?? [], getModelDiff);
  if (Object.keys(modelsDiff).length > 0) diff.models = modelsDiff;
  const connectorsDiff = getCollectionDiff("connector", before.connectors ?? [], after.connectors ?? [], getConnectorDiff);
  if (Object.keys(connectorsDiff).length > 0) diff.connectors = connectorsDiff;
  const propsDiff = getCollectionDiff("prop", before.props ?? [], after.props ?? [], getPropDiff);
  if (Object.keys(propsDiff).length > 0) diff.props = propsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};

export const applyTypeDiff = (base: Type, diff: TypeDiff): Type => {
  const models = diff.models || base.models ? applyCollectionDiff("model", base.models ?? [], diff.models, applyModelDiff) : undefined;
  const connectors = diff.connectors || base.connectors ? applyCollectionDiff("connector", base.connectors ?? [], diff.connectors, applyConnectorDiff) : undefined;
  const props = diff.props || base.props ? applyCollectionDiff("prop", base.props ?? [], diff.props, applyPropDiff) : undefined;
  const attributes = diff.attributes || base.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {}) : undefined;

  const result: Type = {
    guid: base.guid,
    name: diff.name ?? base.name,
    isAbstract: diff.isAbstract ?? base.isAbstract,
    createdAt: diff.createdAt ?? base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
  };

  if (diff.parent !== undefined ? (diff.parent ?? undefined) : base.parent) result.parent = diff.parent !== undefined ? (diff.parent ?? undefined) : base.parent;
  if (diff.folder !== undefined ? (diff.folder ?? undefined) : base.folder) result.folder = diff.folder !== undefined ? (diff.folder ?? undefined) : base.folder;
  if (diff.stock !== undefined ? diff.stock : base.stock) result.stock = diff.stock !== undefined ? diff.stock : base.stock;
  if (diff.virtual ?? base.virtual) result.virtual = diff.virtual ?? base.virtual;
  if (diff.unit !== undefined ? diff.unit : base.unit) result.unit = diff.unit !== undefined ? diff.unit : base.unit;
  if (diff.location !== undefined ? (diff.location ?? undefined) : base.location) result.location = diff.location !== undefined ? (diff.location ?? undefined) : base.location;
  if (diff.icon !== undefined ? (diff.icon ?? undefined) : base.icon) result.icon = diff.icon !== undefined ? (diff.icon ?? undefined) : base.icon;
  if (diff.image !== undefined ? (diff.image ?? undefined) : base.image) result.image = diff.image !== undefined ? (diff.image ?? undefined) : base.image;
  if (diff.description !== undefined ? (diff.description ?? undefined) : base.description) result.description = diff.description !== undefined ? (diff.description ?? undefined) : base.description;
  if (diff.authors !== undefined ? (diff.authors ?? undefined) : base.authors) result.authors = diff.authors !== undefined ? (diff.authors ?? undefined) : base.authors;
  if (diff.concepts !== undefined ? (diff.concepts ?? undefined) : base.concepts) result.concepts = diff.concepts !== undefined ? (diff.concepts ?? undefined) : base.concepts;

  if (models && models.length > 0) result.models = models;
  if (connectors && connectors.length > 0) result.connectors = connectors;
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const mergeTypeDiff = (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};

export const inverseTypeDiff = (original: Type, appliedDiff: TypeDiff): TypeDiff => {
  const inverse: TypeDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.parent !== undefined) inverse.parent = original.parent ?? null;
  if (appliedDiff.isAbstract !== undefined) inverse.isAbstract = original.isAbstract;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder ?? null;
  if (appliedDiff.stock !== undefined) inverse.stock = original.stock;
  if (appliedDiff.virtual !== undefined) inverse.virtual = original.virtual;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.location !== undefined) inverse.location = original.location ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.image !== undefined) inverse.image = original.image ?? null;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.authors !== undefined) inverse.authors = original.authors ?? null;
  if (appliedDiff.concepts !== undefined) inverse.concepts = original.concepts ?? null;
  if (appliedDiff.models) inverse.models = inverseCollectionDiff("model", original.models ?? [], appliedDiff.models, inverseModelDiff);
  if (appliedDiff.connectors) inverse.connectors = inverseCollectionDiff("connector", original.connectors ?? [], appliedDiff.connectors, inverseConnectorDiff);
  if (appliedDiff.props) inverse.props = inverseCollectionDiff("prop", original.props ?? [], appliedDiff.props, inversePropDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};

export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(),
  added: z.array(TypeSchema).optional(),
});
export type TypesDiff = z.infer<typeof TypesDiffSchema>;

export const findConnectorInType = (type: Type, connectorGuid: string): Connector => findConnector(type.connectors ?? [], connectorGuid);

// #endregion 🔖Type

// #region 🔖Layer

export const LayerSchema = z.object({
  guid: z.string(),
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
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Layer = {
    guid: base.guid,
    path: diff.path ?? base.path,
  };

  if (diff.isHidden !== undefined || base.isHidden !== undefined) result.isHidden = diff.isHidden ?? base.isHidden;
  if (diff.isLocked !== undefined || base.isLocked !== undefined) result.isLocked = diff.isLocked ?? base.isLocked;
  if (diff.color !== undefined || base.color !== undefined) result.color = diff.color ?? base.color;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const LayersDiffSchema = z.object({
  removed: z.array(LayerIdSchema).optional(),
  updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(),
  added: z.array(LayerSchema).optional(),
});
export type LayersDiff = z.infer<typeof LayersDiffSchema>;

// #endregion 🔖Layer

// #region 🔖Piece

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
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Piece = z.infer<typeof PieceSchema>;
export const serializePiece = (piece: Piece): string => JSON.stringify(PieceSchema.parse(piece));
export const deserializePiece = (json: string): Piece => PieceSchema.parse(JSON.parse(json));

export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({
  plane: PlaneDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export const getPieceDiff = (before: Piece, after: Piece): PieceDiff => {
  const diff: PieceDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.type?.guid !== after.type?.guid) diff.type = after.type;
  if (before.design?.guid !== after.design?.guid) diff.design = after.design;
  if (!deepEqual(before.plane, after.plane)) diff.plane = after.plane ? getPlaneDiff(before.plane ?? { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } }, after.plane) : undefined;
  if (!deepEqual(before.center, after.center)) diff.center = after.center;
  if (before.scale !== after.scale) diff.scale = after.scale;
  if (!deepEqual(before.mirrorPlane, after.mirrorPlane)) diff.mirrorPlane = after.mirrorPlane;
  if (before.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
  if (before.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
  if (before.color !== after.color) diff.color = after.color;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.props, after.props)) diff.props = getPropsDiff(before.props ?? [], after.props ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
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
  if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(original.props ?? [], appliedDiff.props);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => {
  return {
    ...diff1,
    ...diff2,
    props: diff1.props && diff2.props ? mergePropsDiff(diff1.props, diff2.props) : (diff2.props ?? diff1.props),
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
  const props = diff.props ? applyPropsDiff(base.props ?? [], diff.props) : undefined;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Piece = {
    guid: base.guid,
    name: diff.name ?? base.name,
    type: diff.type ?? base.type,
  };

  if (diff.design !== undefined || base.design !== undefined) result.design = diff.design ?? base.design;
  if (newPlane) result.plane = newPlane;
  if (diff.center !== undefined || base.center !== undefined) result.center = diff.center ?? base.center;
  if (diff.scale !== undefined || base.scale !== undefined) result.scale = diff.scale ?? base.scale;
  if (diff.mirrorPlane !== undefined || base.mirrorPlane !== undefined) result.mirrorPlane = diff.mirrorPlane ?? base.mirrorPlane;
  if (diff.isHidden !== undefined || base.isHidden !== undefined) result.isHidden = diff.isHidden ?? base.isHidden;
  if (diff.isLocked !== undefined || base.isLocked !== undefined) result.isLocked = diff.isLocked ?? base.isLocked;
  if (diff.color !== undefined || base.color !== undefined) result.color = diff.color ?? base.color;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

export const getPieceModelFileGuids = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const modelFileGuids = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type!.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.models) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no models`);
    const model = findModel(type.models, tags);
    modelFileGuids.set(p.guid, model.file.guid);
  });
  return modelFileGuids;
};

export const getPieceModelUrls = (design: Design, types: Type[], files: File[], getFileUrl: (fileGuid: string) => string, tags: string[] = []): Map<string, string> => {
  const modelUrls = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.guid === p.type!.guid);
    if (!type) throw new Error(`Type ${p.type.guid} for piece ${p.guid} not found`);
    if (!type.models) throw new Error(`Type ${p.type.guid} for piece ${p.guid} has no models`);
    const model = findModel(type.models, tags);
    const file = files.find((f) => f.guid === model.file.guid);
    if (!file) throw new Error(`File ${model.file.guid} for model ${model.guid} not found`);
    modelUrls.set(p.guid, getFileUrl(file.guid));
  });
  return modelUrls;
};
export const fixPieceInDesign = (kit: Kit, designId: string, pieceId: string): DesignDiff => {
  const parentConnection = findParentConnectionForPieceInDesign(kit, designId, pieceId);
  return {
    connections: {
      removed: [{ guid: parentConnection.guid }],
    },
  };
};

export const fixPiecesInDesign = (kit: Kit, designId: string, pieceIds: string[]): DesignDiff => {
  const parentConnections = pieceIds.map((pieceId) => findParentConnectionForPieceInDesign(kit, designId, pieceId));
  return {
    connections: {
      removed: parentConnections.map((c) => ({ guid: c.guid })),
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

// #endregion 🔖Piece

// #region 🔖Group

export const GroupSchema = z.object({
  guid: z.string(),
  pieces: z.array(PieceIdSchema),
  color: z.string().optional(),
  name: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Group = z.infer<typeof GroupSchema>;
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
export const getGroupDiff = (before: Group, after: Group): GroupDiff => {
  const diff: GroupDiff = {};
  if (!arraysEqual(before.pieces, after.pieces)) diff.pieces = after.pieces;
  if (before.color !== after.color) diff.color = after.color;
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
export const inverseGroupDiff = (original: Group, appliedDiff: GroupDiff): GroupDiff => {
  const inverse: GroupDiff = {};
  if (appliedDiff.pieces !== undefined) inverse.pieces = original.pieces;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const applyGroupDiff = (base: Group, diff: GroupDiff): Group => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Group = {
    guid: base.guid,
    pieces: diff.pieces ?? base.pieces,
  };

  if (diff.color !== undefined || base.color !== undefined) result.color = diff.color ?? base.color;
  if (diff.name !== undefined || base.name !== undefined) result.name = diff.name ?? base.name;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};
export const mergeGroupDiff = (diff1: GroupDiff, diff2: GroupDiff): GroupDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
export const GroupsDiffSchema = z.object({
  removed: z.array(GroupIdSchema).optional(),
  updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(),
  added: z.array(GroupSchema).optional(),
});
export const serializeGroup = (group: Group): string => JSON.stringify(GroupSchema.parse(group));
export const deserializeGroup = (json: string): Group => GroupSchema.parse(JSON.parse(json));

// #endregion 🔖Group

// #region 🔖Side

export const SideSchema = z.object({
  piece: PieceIdSchema,
  designPiece: PieceIdSchema.optional(),
  connector: ConnectorIdSchema.optional(),
});
export type Side = z.infer<typeof SideSchema>;
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = z.infer<typeof SideDiffSchema>;
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SideId = z.infer<typeof SideIdSchema>;
export const SidesDiffSchema = z.object({
  removed: z.array(SideIdSchema).optional(),
  updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(),
  added: z.array(SideSchema).optional(),
});
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
export const getSideDiff = (before: Side, after: Side): SideDiff => {
  const diff: SideDiff = {};
  if (before.piece?.guid !== after.piece?.guid) diff.piece = after.piece;
  if (before.designPiece?.guid !== after.designPiece?.guid) diff.designPiece = after.designPiece;
  if (before.connector?.guid !== after.connector?.guid) diff.connector = after.connector;
  return diff;
};
export const inverseSideDiff = (original: Side, appliedDiff: SideDiff): SideDiff => {
  const inverse: SideDiff = {};
  if (appliedDiff.piece !== undefined) inverse.piece = original.piece;
  if (appliedDiff.designPiece !== undefined) inverse.designPiece = original.designPiece;
  if (appliedDiff.connector !== undefined) inverse.connector = original.connector;
  return inverse;
};
export const mergeSideDiff = (diff1: SideDiff, diff2: SideDiff): SideDiff => {
  return { ...diff1, ...diff2 };
};
export const applySideDiff = (base: Side, diff: SideDiff): Side => {
  const result: Side = {
    piece: diff.piece ?? base.piece,
  };

  if (diff.designPiece !== undefined || base.designPiece !== undefined) result.designPiece = diff.designPiece ?? base.designPiece;
  if (diff.connector !== undefined || base.connector !== undefined) result.connector = diff.connector ?? base.connector;

  return result;
};
export const serializeSide = (side: Side): string => JSON.stringify(SideSchema.parse(side));
export const deserializeSide = (json: string): Side => SideSchema.parse(JSON.parse(json));
export const areSameSide = (a: Side, b: Side): boolean => a.piece.guid === b.piece.guid && a.designPiece?.guid === b.designPiece?.guid && a.connector?.guid === b.connector?.guid;

// #endregion 🔖Side

// #region 🔖Connection

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
  u: z.number().optional(),
  v: z.number().optional(),
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
  if (!deepEqual(before.connected, after.connected)) diff.connected = getSideDiff(before.connected, after.connected);
  if (!deepEqual(before.connecting, after.connecting)) diff.connecting = getSideDiff(before.connecting, after.connecting);
  if (before.gap !== after.gap) diff.gap = after.gap !== undefined && before.gap !== undefined ? after.gap - before.gap : after.gap;
  if (before.shift !== after.shift) diff.shift = after.shift !== undefined && before.shift !== undefined ? after.shift - before.shift : after.shift;
  if (before.rise !== after.rise) diff.rise = after.rise !== undefined && before.rise !== undefined ? after.rise - before.rise : after.rise;
  if (before.rotation !== after.rotation) diff.rotation = after.rotation !== undefined && before.rotation !== undefined ? after.rotation - before.rotation : after.rotation;
  if (before.turn !== after.turn) diff.turn = after.turn !== undefined && before.turn !== undefined ? after.turn - before.turn : after.turn;
  if (before.tilt !== after.tilt) diff.tilt = after.tilt !== undefined && before.tilt !== undefined ? after.tilt - before.tilt : after.tilt;
  if (before.u !== after.u) diff.u = after.u !== undefined && before.u !== undefined ? after.u - before.u : after.u;
  if (before.v !== after.v) diff.v = after.v !== undefined && before.v !== undefined ? after.v - before.v : after.v;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};

export const applyConnectionDiff = (base: Connection, diff: ConnectionDiff): Connection => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : undefined;

  const result: Connection = {
    guid: base.guid,
    connected: diff.connected ? applySideDiff(base.connected, diff.connected) : base.connected,
    connecting: diff.connecting ? applySideDiff(base.connecting, diff.connecting) : base.connecting,
  };

  if (diff.gap !== undefined || base.gap !== undefined) result.gap = diff.gap ?? base.gap;
  if (diff.shift !== undefined || base.shift !== undefined) result.shift = diff.shift ?? base.shift;
  if (diff.rise !== undefined || base.rise !== undefined) result.rise = diff.rise ?? base.rise;
  if (diff.rotation !== undefined || base.rotation !== undefined) result.rotation = diff.rotation ?? base.rotation;
  if (diff.turn !== undefined || base.turn !== undefined) result.turn = diff.turn ?? base.turn;
  if (diff.tilt !== undefined || base.tilt !== undefined) result.tilt = diff.tilt ?? base.tilt;
  if (diff.u !== undefined || base.u !== undefined) result.u = diff.u ?? base.u;
  if (diff.v !== undefined || base.v !== undefined) result.v = diff.v ?? base.v;
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
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
  if (appliedDiff.u !== undefined) inverse.u = original.u !== undefined && appliedDiff.u !== undefined ? -appliedDiff.u : original.u;
  if (appliedDiff.v !== undefined) inverse.v = original.v !== undefined && appliedDiff.v !== undefined ? -appliedDiff.v : original.v;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = getAttributesDiff(appliedDiff.attributes ? applyAttributesDiff([], appliedDiff.attributes) : [], original.attributes ?? []);
  return inverse;
};

export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(),
  added: z.array(ConnectionSchema).optional(),
});
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export const serializeConnection = (connection: Connection): string => JSON.stringify(ConnectionSchema.parse(connection));
export const deserializeConnection = (json: string): Connection => ConnectionSchema.parse(JSON.parse(json));

export const areSameConnection = (connection: Connection | ConnectionDiff, other: Connection | ConnectionDiff, strict: boolean = false): boolean => {
  const getConnectedPieceId = (conn: typeof connection) => ("connected" in conn && conn.connected && "piece" in conn.connected ? (typeof conn.connected.piece === "string" ? conn.connected.piece : (conn.connected.piece?.guid ?? "")) : "");
  const getConnectingPieceId = (conn: typeof connection) => ("connecting" in conn && conn.connecting && "piece" in conn.connecting ? (typeof conn.connecting.piece === "string" ? conn.connecting.piece : (conn.connecting.piece?.guid ?? "")) : "");

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

export const findConnectorForPieceInConnection = (type: Type, connection: Connection, pieceGuid: string): Connector | undefined => {
  const connectorGuid = connection.connected.piece.guid === pieceGuid ? connection.connected.connector?.guid : connection.connecting.connector?.guid;
  if (!connectorGuid) return undefined;
  return findConnectorInType(type, connectorGuid);
};

// #endregion 🔖Connection

// #region 🔖Stat

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
export const getStatDiff = (before: Stat, after: Stat): StatDiff => {
  const diff: StatDiff = {};
  if (before.quality?.guid !== after.quality?.guid) diff.quality = after.quality;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.min !== after.min) diff.min = after.min;
  if (before.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
  if (before.max !== after.max) diff.max = after.max;
  if (before.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
  return diff;
};
export const inverseStatDiff = (original: Stat, appliedDiff: StatDiff): StatDiff => {
  const inverse: StatDiff = {};
  if (appliedDiff.quality !== undefined) inverse.quality = original.quality;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.min !== undefined) inverse.min = original.min;
  if (appliedDiff.minExcluded !== undefined) inverse.minExcluded = original.minExcluded;
  if (appliedDiff.max !== undefined) inverse.max = original.max;
  if (appliedDiff.maxExcluded !== undefined) inverse.maxExcluded = original.maxExcluded;
  return inverse;
};
export const applyStatDiff = (base: Stat, diff: StatDiff): Stat => {
  const result: Stat = {
    guid: base.guid,
    quality: diff.quality ?? base.quality,
  };

  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.minExcluded !== undefined || base.minExcluded !== undefined) result.minExcluded = diff.minExcluded ?? base.minExcluded;
  if (diff.max !== undefined || base.max !== undefined) result.max = diff.max ?? base.max;
  if (diff.maxExcluded !== undefined || base.maxExcluded !== undefined) result.maxExcluded = diff.maxExcluded ?? base.maxExcluded;

  return result;
};
export const mergeStatDiff = (diff1: StatDiff, diff2: StatDiff): StatDiff => {
  return { ...diff1, ...diff2 };
};
export const StatsDiffSchema = z.object({
  removed: z.array(StatIdSchema).optional(),
  updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(),
  added: z.array(StatSchema).optional(),
});
export const serializeStat = (stat: Stat): string => JSON.stringify(StatSchema.parse(stat));
export const deserializeStat = (json: string): Stat => StatSchema.parse(JSON.parse(json));

// #endregion 🔖Stat

// #region 🔖Design

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
  concepts: z.array(ConceptIdSchema).optional(),
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
  const diff: DesignDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.parent?.guid !== after.parent?.guid) diff.parent = after.parent;
  if (before.isAbstract !== after.isAbstract) diff.isAbstract = after.isAbstract;
  if (before.folder !== after.folder) diff.folder = after.folder;
  if (before.canScale !== after.canScale) diff.canScale = after.canScale;
  if (before.canMirror !== after.canMirror) diff.canMirror = after.canMirror;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (before.activeLayer?.guid !== after.activeLayer?.guid) diff.activeLayer = after.activeLayer;
  if (before.location?.guid !== after.location?.guid) diff.location = after.location;
  if (before.icon !== after.icon) diff.icon = after.icon;
  if (before.image !== after.image) diff.image = after.image;
  if (before.description !== after.description) diff.description = after.description;
  if (!arraysEqual(before.authors, after.authors)) diff.authors = after.authors as any;
  if (!arraysEqual(before.concepts, after.concepts)) diff.concepts = after.concepts;
  const piecesDiff = getCollectionDiff("piece", before.pieces ?? [], after.pieces ?? [], getPieceDiff);
  if (Object.keys(piecesDiff).length > 0) diff.pieces = piecesDiff;
  const connectionsDiff = getCollectionDiff("connection", before.connections ?? [], after.connections ?? [], getConnectionDiff);
  if (Object.keys(connectionsDiff).length > 0) diff.connections = connectionsDiff;
  const statsDiff = getCollectionDiff("stat", before.stats ?? [], after.stats ?? [], getStatDiff);
  if (Object.keys(statsDiff).length > 0) diff.stats = statsDiff;
  const propsDiff = getCollectionDiff("prop", before.props ?? [], after.props ?? [], getPropDiff);
  if (Object.keys(propsDiff).length > 0) diff.props = propsDiff;
  const layersDiff = getCollectionDiff("layer", before.layers ?? [], after.layers ?? [], getLayerDiff);
  if (Object.keys(layersDiff).length > 0) diff.layers = layersDiff;
  const groupsDiff = getCollectionDiff("group", before.groups ?? [], after.groups ?? [], getGroupDiff);
  if (Object.keys(groupsDiff).length > 0) diff.groups = groupsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
export const mergeDesignDiff = (diff1: DesignDiff, diff2: DesignDiff): DesignDiff => {
  return {
    ...diff1,
    ...diff2,
    pieces: diff1.pieces || diff2.pieces ? mergeCollectionDiff("piece", diff1.pieces ?? {}, diff2.pieces ?? {}, mergePieceDiff) : undefined,
    connections: diff1.connections || diff2.connections ? mergeCollectionDiff("connection", diff1.connections ?? {}, diff2.connections ?? {}, mergeConnectionDiff) : undefined,
    stats: diff1.stats || diff2.stats ? mergeCollectionDiff("stat", diff1.stats ?? {}, diff2.stats ?? {}, mergeStatDiff) : undefined,
    props: diff1.props || diff2.props ? mergeCollectionDiff("prop", diff1.props ?? {}, diff2.props ?? {}, mergePropDiff) : undefined,
    layers: diff1.layers || diff2.layers ? mergeCollectionDiff("layer", diff1.layers ?? {}, diff2.layers ?? {}, mergeLayerDiff) : undefined,
    groups: diff1.groups || diff2.groups ? mergeCollectionDiff("group", diff1.groups ?? {}, diff2.groups ?? {}, mergeGroupDiff) : undefined,
    authors: diff2.authors ?? diff1.authors,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
export const inverseDesignDiff = (original: Design, appliedDiff: DesignDiff): DesignDiff => {
  const inverse: DesignDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.parent !== undefined) inverse.parent = original.parent;
  if (appliedDiff.isAbstract !== undefined) inverse.isAbstract = original.isAbstract;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder;
  if (appliedDiff.canScale !== undefined) inverse.canScale = original.canScale;
  if (appliedDiff.canMirror !== undefined) inverse.canMirror = original.canMirror;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.activeLayer !== undefined) inverse.activeLayer = original.activeLayer;
  if (appliedDiff.location !== undefined) inverse.location = original.location;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
  if (appliedDiff.image !== undefined) inverse.image = original.image;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.authors !== undefined) inverse.authors = original.authors as any;
  if (appliedDiff.concepts !== undefined) inverse.concepts = original.concepts;
  if (appliedDiff.pieces) inverse.pieces = inverseCollectionDiff("piece", original.pieces ?? [], appliedDiff.pieces, inversePieceDiff);
  if (appliedDiff.connections) inverse.connections = inverseCollectionDiff("connection", original.connections ?? [], appliedDiff.connections, inverseConnectionDiff);
  if (appliedDiff.stats) inverse.stats = inverseCollectionDiff("stat", original.stats ?? [], appliedDiff.stats, inverseStatDiff);
  if (appliedDiff.props) inverse.props = inverseCollectionDiff("prop", original.props ?? [], appliedDiff.props, inversePropDiff);
  if (appliedDiff.layers) inverse.layers = inverseCollectionDiff("layer", original.layers ?? [], appliedDiff.layers, inverseLayerDiff);
  if (appliedDiff.groups) inverse.groups = inverseCollectionDiff("group", original.groups ?? [], appliedDiff.groups, inverseGroupDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
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
  const pieces = diff.pieces || base.pieces ? applyCollectionDiff("piece", base.pieces ?? [], diff.pieces, applyPieceDiff) : undefined;
  const connections = diff.connections || base.connections ? applyCollectionDiff("connection", base.connections ?? [], diff.connections, applyConnectionDiff) : undefined;
  const stats = diff.stats || base.stats ? applyCollectionDiff("stat", base.stats ?? [], diff.stats, applyStatDiff) : undefined;
  const props = diff.props || base.props ? applyCollectionDiff("prop", base.props ?? [], diff.props, applyPropDiff) : undefined;
  const layers = diff.layers || base.layers ? applyCollectionDiff("layer", base.layers ?? [], diff.layers, applyLayerDiff) : undefined;
  const groups = diff.groups || base.groups ? applyCollectionDiff("group", base.groups ?? [], diff.groups, applyGroupDiff) : undefined;
  const attributes = diff.attributes || base.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {}) : undefined;

  const result: Design = {
    guid: base.guid,
    name: diff.name ?? base.name,
    isAbstract: diff.isAbstract ?? base.isAbstract,
    createdAt: diff.createdAt ?? base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
  };

  if (diff.parent !== undefined ? diff.parent : base.parent) result.parent = diff.parent !== undefined ? diff.parent : base.parent;
  if (diff.folder ?? base.folder) result.folder = diff.folder ?? base.folder;
  if (diff.canScale ?? base.canScale) result.canScale = diff.canScale ?? base.canScale;
  if (diff.canMirror ?? base.canMirror) result.canMirror = diff.canMirror ?? base.canMirror;
  if (diff.unit !== undefined ? diff.unit : base.unit) result.unit = diff.unit !== undefined ? diff.unit : base.unit;
  if (diff.activeLayer !== undefined ? diff.activeLayer : base.activeLayer) result.activeLayer = diff.activeLayer !== undefined ? diff.activeLayer : base.activeLayer;
  if (diff.location !== undefined ? diff.location : base.location) result.location = diff.location !== undefined ? diff.location : base.location;
  if (diff.icon !== undefined ? diff.icon : base.icon) result.icon = diff.icon !== undefined ? diff.icon : base.icon;
  if (diff.image !== undefined ? diff.image : base.image) result.image = diff.image !== undefined ? diff.image : base.image;
  if (diff.description !== undefined ? diff.description : base.description) result.description = diff.description !== undefined ? diff.description : base.description;
  if (diff.authors !== undefined ? (diff.authors as any) : base.authors) result.authors = diff.authors !== undefined ? (diff.authors as any) : base.authors;
  if (diff.concepts !== undefined ? diff.concepts : base.concepts) result.concepts = diff.concepts !== undefined ? diff.concepts : base.concepts;

  if (pieces && pieces.length > 0) result.pieces = pieces;
  if (connections && connections.length > 0) result.connections = connections;
  if (stats && stats.length > 0) result.stats = stats;
  if (props && props.length > 0) result.props = props;
  if (layers && layers.length > 0) result.layers = layers;
  if (groups && groups.length > 0) result.groups = groups;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(),
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

  return {};
};

export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: string, pieceIds: string[], connectionIds: string[]): DesignDiff => {
  return {
    pieces: {
      removed: pieceIds.map((guid) => ({ guid })),
    },
    connections: {
      removed: connectionIds.map((guid) => ({ guid })),
    },
  };
};

const computeChildPlane = (parentPlane: Plane, parentConnector: Connector, childConnector: Connector, connection: Connection): Plane => {
  const parentMatrix = planeToMatrix(parentPlane);
  const parentPoint = vectorToThree(parentConnector.point);
  const parentDirection = vectorToThree(parentConnector.direction).normalize();
  const childPoint = vectorToThree(childConnector.point);
  const childDirection = vectorToThree(childConnector.direction).normalize();

  const { gap, shift, rise, rotation, turn, tilt } = connection;
  const rotationRad = THREE.MathUtils.degToRad(rotation ?? 0);
  const turnRad = THREE.MathUtils.degToRad(turn ?? 0);
  const tiltRad = THREE.MathUtils.degToRad(tilt ?? 0);

  const reverseChildDirection = childDirection.clone().negate();

  let alignQuat: THREE.Quaternion;
  if (new THREE.Vector3().crossVectors(parentDirection, reverseChildDirection).length() < 0.01) {
    if (Math.abs(parentDirection.z) < TOLERANCE) {
      alignQuat = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 0, 1), Math.PI);
    } else {
      const axis = new THREE.Vector3(0, 0, 1).cross(parentDirection).normalize();
      alignQuat = new THREE.Quaternion().setFromAxisAngle(axis, Math.PI);
    }
  } else {
    alignQuat = new THREE.Quaternion().setFromUnitVectors(reverseChildDirection, parentDirection);
  }

  const directionT = new THREE.Matrix4().makeRotationFromQuaternion(alignQuat);

  const yAxis = new THREE.Vector3(0, 1, 0);
  const parentConnectorQuat = new THREE.Quaternion().setFromUnitVectors(yAxis, parentDirection);
  const parentRotationT = new THREE.Matrix4().makeRotationFromQuaternion(parentConnectorQuat);

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
  const getConnector = (type: Type | undefined, connectorGuid: string | undefined): Connector | undefined => {
    if (!type) return undefined;

    if (!connectorGuid) {
      if (type.connectors && type.connectors.length > 0) {
        return type.connectors[0];
      }

      if (type.parent?.guid) {
        const parentType = getType(type.parent.guid);
        return getConnector(parentType, connectorGuid);
      }
      return undefined;
    }

    if (type.connectors && type.connectors.length > 0) {
      const connector = type.connectors.find((p) => p.guid === connectorGuid);
      if (connector) return connector;
    }

    if (type.parent?.guid) {
      const parentType = getType(type.parent.guid);
      const connector = getConnector(parentType, connectorGuid);
      if (connector) return connector;
    }

    if (type.connectors && type.connectors.length > 0) {
      return type.connectors[0];
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

  const filteredConnections =
    flatDesign.connections?.filter((connection) => {
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
    } else {
      const identityMatrix = new THREE.Matrix4().identity();
      rootPlane = matrixToPlane(identityMatrix);
    }

    piecePlanes[rootPiece.guid] = rootPlane;
    const rootPieceIndex = flatDesign.pieces!.findIndex((p) => p.guid === rootPiece.guid);
    if (rootPieceIndex !== -1) {
      flatDesign.pieces![rootPieceIndex].plane = rootPlane;

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

        const parentConnectorGuid = parentSide.connector?.guid;
        const childConnectorGuid = childSide.connector?.guid;
        const parentConnector = getConnector(parentType, parentConnectorGuid);
        const childConnector = getConnector(childType, childConnectorGuid);

        if (!parentConnector || !childConnector) {
          console.error(`Error during flatten: Connectors not found for connection between ${parentId} and ${childId}. Parent Connector: ${parentConnectorGuid}, Child Connector: ${childConnectorGuid}`);
          skipCount++;
          return;
        }
        const childPlane = roundPlane(computeChildPlane(parentPlane, parentConnector, childConnector, connection));
        piecePlanes[childPiece.guid] = childPlane;

        const radius = 2.697;
        const verticalVExtra = 1.0;
        const horizontalScale = 3.0633;
        const parentCenter = parentPiece.center || { u: 0, v: 0 };
        const connectionU = connection.u ?? 0;
        const connectionV = connection.v ?? 0;

        let childU: number;
        let childV: number;

        if (parentCenter.u === 0 && parentCenter.v === 0) {
          const angle = 2 * Math.PI * parentConnector.t;
          childU = radius * Math.sin(angle);
          childV = radius * Math.cos(angle);
        } else {
          const isVerticalConnection = Math.abs(parentConnector.direction?.z ?? 0) > 0.5;

          if (isVerticalConnection) {
            childU = parentCenter.u + connectionU;
            childV = parentCenter.v + connectionV + verticalVExtra;
          } else {
            childU = parentCenter.u + connectionU * horizontalScale;
            childV = parentCenter.v + connectionV * horizontalScale;
          }
        }

        const childCenter = {
          u: round(childU),
          v: round(childV),
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

  let piecesWithPlanes = 0;
  let piecesWithoutPlanes = 0;
  const updatedPieces = flatDesign.pieces
    ?.map((flatPiece) => {
      if (flatPiece.plane) piecesWithPlanes++;
      else piecesWithoutPlanes++;

      const originalPiece = design.pieces?.find((p) => p.guid === flatPiece.guid);
      if (!originalPiece) return null;

      const pieceDiff: PieceDiff = {};

      if (flatPiece.plane && JSON.stringify(flatPiece.plane) !== JSON.stringify(originalPiece.plane)) {
        pieceDiff.plane = flatPiece.plane;
      }

      if (flatPiece.center && JSON.stringify(flatPiece.center) !== JSON.stringify(originalPiece.center)) {
        pieceDiff.center = flatPiece.center;
      }
      if (JSON.stringify(flatPiece.attributes) !== JSON.stringify(originalPiece.attributes)) {
        pieceDiff.attributes = getAttributesDiff(originalPiece.attributes ?? [], flatPiece.attributes ?? []);
      }

      if (Object.keys(pieceDiff).length === 0) return null;

      return {
        piece: { guid: flatPiece.guid },
        diff: pieceDiff,
      };
    })
    .filter((update) => update !== null) as Array<{ piece: PieceId; diff: PieceDiff }>;

  const removedConnections = design.connections?.map((c) => ({ connected: { piece: c.connected.piece.guid }, connecting: { piece: c.connecting.piece.guid } })) || [];

  return {
    pieces: updatedPieces.length > 0 ? { updated: updatedPieces } : undefined,
    connections: removedConnections.length > 0 ? { removed: removedConnections } : undefined,
  } as DesignDiff;
};

export const createClusteredDesign = (originalDesign: Design, clusterPieceIds: string[], designName: string): { clusteredDesign: Design; externalConnections: Connection[] } => {
  if (!originalDesign.pieces || originalDesign.pieces.length === 0) {
    throw new Error("Original design has no pieces to cluster");
  }
  if (!clusterPieceIds || clusterPieceIds.length === 0) {
    throw new Error("No piece IDs provided for clustering");
  }

  const clusteredPieces = (originalDesign.pieces || []).filter((piece) => clusterPieceIds.includes(piece.guid));

  if (clusteredPieces.length === 0) {
    throw new Error("No pieces found matching the provided IDs");
  }

  const internalConnections = (originalDesign.connections || []).filter((connection) => clusterPieceIds.includes(connection.connected.piece.guid) && clusterPieceIds.includes(connection.connecting.piece.guid));

  const externalConnections = (originalDesign.connections || []).filter((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);
    return connectedInCluster !== connectingInCluster;
  });

  const clusteredDesign: Design = {
    guid: guid(),
    name: designName,
    unit: originalDesign.unit,
    description: `Clustered design with ${clusteredPieces.length} pieces`,
    pieces: clusteredPieces,
    connections: internalConnections,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  return { clusteredDesign, externalConnections };
};

export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds: string[], clusteredDesign: Design, externalConnections: Connection[]): DesignDiff => {
  const piecesToRemove = clusterPieceIds.map((guid) => ({ guid }));

  const connectionsToRemove = (originalDesign.connections || [])
    .filter((connection) => {
      const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
      const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);
      return connectedInCluster || connectingInCluster;
    })
    .map((c) => ({ guid: c.guid }));

  const updatedExternalConnections = externalConnections.map((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.guid);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.guid);

    if (connectedInCluster) {
      return {
        ...connection,
        connected: {
          ...connection.connected,
          designPiece: { guid: connection.connected.piece.guid },
        },
      };
    } else if (connectingInCluster) {
      return {
        ...connection,
        connecting: {
          ...connection.connecting,
          designPiece: { guid: connection.connecting.piece.guid },
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

export const getClusterableGroups = (design: Design, selectedPieceIds: string[]): string[][] => {
  if (selectedPieceIds.length < 2) return [];

  const adjacencyMap = new Map<string, Set<string>>();
  (design.connections || []).forEach((connection) => {
    const sourceId = connection.connecting.piece.guid;
    const targetId = connection.connected.piece.guid;

    if (!adjacencyMap.has(sourceId)) adjacencyMap.set(sourceId, new Set());
    if (!adjacencyMap.has(targetId)) adjacencyMap.set(targetId, new Set());

    adjacencyMap.get(sourceId)!.add(targetId);
    adjacencyMap.get(targetId)!.add(sourceId);
  });

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

  for (const pieceId of selectedPieceIds) {
    if (!visited.has(pieceId)) {
      const group: string[] = [];
      dfs(pieceId, group);
      connectedGroups.push(group);
    }
  }

  const hasDesignNodes = selectedPieceIds.some((id) => id.startsWith("design-"));
  const hasMultipleComponents = connectedGroups.length > 1;
  const hasLargeConnectedGroup = connectedGroups.some((group) => group.length > 1);

  if (hasDesignNodes || hasMultipleComponents || hasLargeConnectedGroup) {
    return [selectedPieceIds];
  }

  return [];
};

export const expandDesignPieces = (design: Design, kit: Kit): Design => {
  const hasDesignConnections = design.connections?.some((conn) => conn.connected.designPiece || conn.connecting.designPiece);
  if (!hasDesignConnections) {
    return design;
  }

  let expandedDesign = { ...design };

  const designIds = new Set<string>();
  toArray(design.connections).forEach((conn) => {
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.guid);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.guid);
  });

  if (designIds.size === 0) {
    return expandedDesign;
  }

  for (const designName of Array.from(designIds)) {
    const referencedDesign = findDesignInKit(kit, designName);
    if (!referencedDesign) continue;

    const expandedReferencedDesign = expandDesignPieces(referencedDesign, kit);

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
        return {
          ...connection,
          connecting: {
            ...connection.connecting,
            designPiece: undefined,
          },
        };
      }

      return connection;
    });

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

  const designIds = new Set<string>();
  toArray(design.connections).forEach((conn: Connection) => {
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.guid);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.guid);
  });

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

export const isPortInUse = (design: Design, pieceGuid: string, connectorGuid: string): boolean => {
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.guid === pieceGuid;
    const isPortConnected = isPieceConnected ? connection.connected.connector?.guid === connectorGuid : connection.connecting.connector?.guid === connectorGuid;
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

// #endregion 🔖Design

// #region 🔖Kit

export const KitSchema = z.object({
  guid: z.string(),
  name: z.string(),
  version: z.string().optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  tags: z.array(TagSchema).optional(),
  concepts: z.array(ConceptSchema).optional(),
  ports: z.array(PortSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  files: z.array(FileSchema).optional(),
  folders: z.array(FolderSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
  remote: z.string().optional(),
  homepage: z.string().optional(),
  license: z.string().optional(),
  preview: z.string().optional(),
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

export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, tags: true, concepts: true, ports: true, qualities: true, folders: true, authors: true }).extend({
  types: z.array(z.string()).optional(),
  designs: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  concepts: z.array(z.string()).optional(),
  ports: z.array(z.string()).optional(),
  qualities: z.array(z.string()).optional(),
  folders: z.array(z.string()).optional(),
  authors: z.array(z.string()).optional(),
});
export type KitShallow = z.infer<typeof KitShallowSchema>;
export const serializeKitShallow = (kit: KitShallow): string => JSON.stringify(KitShallowSchema.parse(kit));
export const deserializeKitShallow = (json: string): KitShallow => KitShallowSchema.parse(JSON.parse(json));
export const KitDiffSchema = KitSchema.partial().omit({ types: true, designs: true, tags: true, concepts: true, ports: true, qualities: true, authors: true, files: true, folders: true, attributes: true }).extend({
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  tags: TagsDiffSchema.optional(),
  concepts: ConceptsDiffSchema.optional(),
  ports: PortsDiffSchema.optional(),
  qualities: QualitiesDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  files: FilesDiffSchema.optional(),
  folders: FoldersDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
  image: z.string().nullable().optional(),
  remote: z.string().nullable().optional(),
  homepage: z.string().nullable().optional(),
  license: z.string().nullable().optional(),
  preview: z.string().nullable().optional(),
});
export type KitDiff = z.infer<typeof KitDiffSchema>;
type EntityIdType = { guid: string };
type CollectionDiff<K extends string, T extends { guid: string }, D> = {
  removed?: EntityIdType[];
  updated?: Array<{ [key in K]: EntityIdType } & { diff: D }>;
  added?: T[];
};

const getCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, before: T[], after: T[], getItemDiff: (before: T, after: T) => D): CollectionDiff<K, T, D> => {
  const diff: CollectionDiff<K, T, D> = {};
  const beforeGuids = new Set(before.map((i) => i.guid));
  const afterGuids = new Set(after.map((i) => i.guid));
  const removed = before.filter((i) => !afterGuids.has(i.guid)).map((i) => ({ guid: i.guid }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterGuids.has(i.guid))
    .map((i) => {
      const afterItem = after.find((a) => a.guid === i.guid)!;
      const itemDiff = getItemDiff(i, afterItem);
      return { [entityKey]: { guid: i.guid }, diff: itemDiff } as { [key in K]: EntityIdType } & { diff: D };
    })
    .filter((u) => Object.keys(u.diff as any).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeGuids.has(i.guid));
  if (added.length > 0) diff.added = added;
  return diff;
};

const inverseCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, original: T[], appliedDiff: CollectionDiff<K, T, D>, inverseItemDiff: (original: T, appliedDiff: D) => D): CollectionDiff<K, T, D> => {
  const inverse: CollectionDiff<K, T, D> = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedGuids.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ guid: i.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const entityId = (u as any)[entityKey] as EntityIdType;
      const originalItem = original.find((i) => i.guid === entityId.guid)!;
      return { [entityKey]: entityId, diff: inverseItemDiff(originalItem, u.diff) } as { [key in K]: EntityIdType } & { diff: D };
    });
  }
  return inverse;
};

const applyCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, base: T[], diff: CollectionDiff<K, T, D> | undefined, applyItemDiff: (base: T, diff: D) => T): T[] => {
  if (!diff) return base;
  let result = [...base];
  if (diff.removed) {
    const removedGuids = new Set(diff.removed.map((r) => r.guid));
    result = result.filter((i) => !removedGuids.has(i.guid));
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const entityId = (update as any)[entityKey] as EntityIdType;
      const index = result.findIndex((i) => i.guid === entityId.guid);
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

const mergeCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, diff1: CollectionDiff<K, T, D>, diff2: CollectionDiff<K, T, D>, mergeItemDiff: (diff1: D, diff2: D) => D): CollectionDiff<K, T, D> => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const getEntityGuid = (u: any) => (u[entityKey] as EntityIdType).guid;
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [getEntityGuid(u), u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [getEntityGuid(u), u.diff]));
  const allGuids = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allGuids).map((guid) => ({
    [entityKey]: { guid },
    diff: mergeItemDiff(updated1Map.get(guid) ?? ({} as D), updated2Map.get(guid) ?? ({} as D)),
  })) as Array<{ [key in K]: EntityIdType } & { diff: D }>;
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
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
  const typesDiff = getCollectionDiff("type", before.types ?? [], after.types ?? [], getTypeDiff);
  if (Object.keys(typesDiff).length > 0) diff.types = typesDiff;
  const designsDiff = getCollectionDiff("design", before.designs ?? [], after.designs ?? [], getDesignDiff);
  if (Object.keys(designsDiff).length > 0) diff.designs = designsDiff;
  const tagsDiff = getTagsDiff(before.tags ?? [], after.tags ?? []);
  if (Object.keys(tagsDiff).length > 0) diff.tags = tagsDiff;
  const conceptsDiff = getConceptsDiff(before.concepts ?? [], after.concepts ?? []);
  if (Object.keys(conceptsDiff).length > 0) diff.concepts = conceptsDiff;
  const portsDiff = getPortsDiff(before.ports ?? [], after.ports ?? []);
  if (Object.keys(portsDiff).length > 0) diff.ports = portsDiff;
  const qualitiesDiff = getCollectionDiff("quality", before.qualities ?? [], after.qualities ?? [], getQualityDiff);
  if (Object.keys(qualitiesDiff).length > 0) diff.qualities = qualitiesDiff;
  const filesDiff = getCollectionDiff("file", before.files ?? [], after.files ?? [], getFileDiff);
  if (Object.keys(filesDiff).length > 0) diff.files = filesDiff;
  const foldersDiff = getCollectionDiff("folder", before.folders ?? [], after.folders ?? [], getFolderDiff);
  if (Object.keys(foldersDiff).length > 0) diff.folders = foldersDiff;
  const authorsDiff = getCollectionDiff("author", before.authors ?? [], after.authors ?? [], getAuthorDiff);
  if (Object.keys(authorsDiff).length > 0) diff.authors = authorsDiff;
  const attributesDiff = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
  return diff;
};
export const inverseKitDiff = (original: Kit, appliedDiff: KitDiff): KitDiff => {
  const inverse: KitDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.version !== undefined) inverse.version = original.version;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.image !== undefined) inverse.image = original.image ?? null;
  if (appliedDiff.remote !== undefined) inverse.remote = original.remote ?? null;
  if (appliedDiff.homepage !== undefined) inverse.homepage = original.homepage ?? null;
  if (appliedDiff.license !== undefined) inverse.license = original.license ?? null;
  if (appliedDiff.preview !== undefined) inverse.preview = original.preview ?? null;
  if (appliedDiff.types) inverse.types = inverseCollectionDiff("type", original.types ?? [], appliedDiff.types, inverseTypeDiff);
  if (appliedDiff.designs) inverse.designs = inverseCollectionDiff("design", original.designs ?? [], appliedDiff.designs, inverseDesignDiff);
  if (appliedDiff.tags) inverse.tags = inverseTagsDiff(original.tags ?? [], appliedDiff.tags);
  if (appliedDiff.concepts) inverse.concepts = inverseConceptsDiff(original.concepts ?? [], appliedDiff.concepts);
  if (appliedDiff.ports) inverse.ports = inversePortsDiff(original.ports ?? [], appliedDiff.ports);
  if (appliedDiff.qualities) inverse.qualities = inverseCollectionDiff("quality", original.qualities ?? [], appliedDiff.qualities, inverseQualityDiff);
  if (appliedDiff.files) inverse.files = inverseCollectionDiff("file", original.files ?? [], appliedDiff.files, inverseFileDiff);
  if (appliedDiff.folders) inverse.folders = inverseCollectionDiff("folder", original.folders ?? [], appliedDiff.folders, inverseFolderDiff);
  if (appliedDiff.authors) inverse.authors = inverseCollectionDiff("author", original.authors ?? [], appliedDiff.authors, inverseAuthorDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
export const mergeKitDiff = (diff1: KitDiff, diff2: KitDiff): KitDiff => {
  const mergeSimpleDiff = <D>(d1: D, d2: D): D => ({ ...d1, ...d2 });
  return {
    ...diff1,
    ...diff2,
    types: diff1.types || diff2.types ? mergeCollectionDiff("type", diff1.types ?? {}, diff2.types ?? {}, mergeTypeDiff) : undefined,
    designs: diff1.designs || diff2.designs ? mergeCollectionDiff("design", diff1.designs ?? {}, diff2.designs ?? {}, mergeDesignDiff) : undefined,
    tags: diff1.tags || diff2.tags ? mergeTagsDiff(diff1.tags ?? {}, diff2.tags ?? {}) : undefined,
    concepts: diff1.concepts || diff2.concepts ? mergeConceptsDiff(diff1.concepts ?? {}, diff2.concepts ?? {}) : undefined,
    ports: diff1.ports || diff2.ports ? mergePortsDiff(diff1.ports ?? {}, diff2.ports ?? {}) : undefined,
    qualities: diff1.qualities || diff2.qualities ? mergeCollectionDiff("quality", diff1.qualities ?? {}, diff2.qualities ?? {}, mergeQualityDiff) : undefined,
    files: diff1.files || diff2.files ? mergeCollectionDiff("file", diff1.files ?? {}, diff2.files ?? {}, mergeSimpleDiff) : undefined,
    folders: diff1.folders || diff2.folders ? mergeCollectionDiff("folder", diff1.folders ?? {}, diff2.folders ?? {}, mergeSimpleDiff) : undefined,
    authors: diff1.authors || diff2.authors ? mergeCollectionDiff("author", diff1.authors ?? {}, diff2.authors ?? {}, mergeSimpleDiff) : undefined,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
export const applyKitDiff = (base: Kit, diff: KitDiff): Kit => {
  const result: any = {
    guid: base.guid,
    name: "name" in diff ? diff.name! : base.name,
    version: "version" in diff ? diff.version! : base.version,
    createdAt: base.createdAt,
    updatedAt: diff.updatedAt ?? base.updatedAt,
  };

  const optionalScalars = ["description", "icon", "image", "remote", "homepage", "license", "preview"] as const;
  for (const key of optionalScalars) {
    if (key in diff) {
      const value = diff[key] ?? undefined;
      if (value !== undefined) result[key] = value;
    } else if (key in base && base[key] !== undefined) {
      result[key] = base[key];
    }
  }

  if (diff.types || base.types) {
    const types = applyCollectionDiff("type", base.types ?? [], diff.types, applyTypeDiff);
    if (types.length > 0) result.types = types;
  }
  if (diff.designs || base.designs) {
    const designs = applyCollectionDiff("design", base.designs ?? [], diff.designs, applyDesignDiff);
    if (designs.length > 0) result.designs = designs;
  }
  if (diff.tags || base.tags) {
    const tags = applyTagsDiff(base.tags ?? [], diff.tags ?? {});
    if (tags.length > 0) result.tags = tags;
  }
  if (diff.concepts || base.concepts) {
    const concepts = applyConceptsDiff(base.concepts ?? [], diff.concepts ?? {});
    if (concepts.length > 0) result.concepts = concepts;
  }
  if (diff.ports || base.ports) {
    const ports = applyPortsDiff(base.ports ?? [], diff.ports ?? {});
    if (ports.length > 0) result.ports = ports;
  }
  if (diff.qualities || base.qualities) {
    const qualities = applyCollectionDiff("quality", base.qualities ?? [], diff.qualities, applyQualityDiff);
    if (qualities.length > 0) result.qualities = qualities;
  }
  if (diff.files || base.files) {
    const files = applyCollectionDiff("file", base.files ?? [], diff.files, applyFileDiff);
    if (files.length > 0) result.files = files;
  }
  if (diff.folders || base.folders) {
    const folders = applyCollectionDiff("folder", base.folders ?? [], diff.folders, applyFolderDiff);
    if (folders.length > 0) result.folders = folders;
  }
  if (diff.authors || base.authors) {
    const authors = applyCollectionDiff("author", base.authors ?? [], diff.authors, applyAuthorDiff);
    if (authors.length > 0) result.authors = authors;
  }
  if (diff.attributes || base.attributes) {
    const attributes = applyAttributesDiff(base.attributes ?? [], diff.attributes ?? {});
    if (attributes.length > 0) result.attributes = attributes;
  }

  return result as Kit;
};

export const KitsDiffSchema = z.object({
  removed: z.array(KitIdSchema).optional(),
  updated: z.array(z.object({ kit: KitIdSchema, diff: KitDiffSchema })).optional(),
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
  types: { removed: [{ guid: typeGuid }] },
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
      removed: [{ guid: designGuid }],
    },
  };
};

export const updateDesignInKit = (design: Design): KitDiff => ({
  designs: {
    added: [design],
  },
});

export const addPortToKit = (iface: Port): KitDiff => ({
  ports: {
    added: [iface],
  },
});
export const setPortInKit = (iface: Port): KitDiff => ({
  ports: {
    added: [iface],
  },
});
export const removePortFromKit = (portGuid: string): KitDiff => ({
  ports: { removed: [{ guid: portGuid }] },
});
export const updatePortInKit = (iface: Port): KitDiff => ({
  ports: {
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
  files: { removed: [{ guid: fileGuid }] },
});

export const setAttributeInKit = (attribute: Attribute): KitDiff => ({
  attributes: { added: [attribute] },
});

export const findTagInKit = (kit: Kit, tagGuid: string): Tag => {
  const tag = (kit.tags || []).find((t) => t.guid === tagGuid);
  if (!tag) throw new Error(`Tag ${tagGuid} not found in kit`);
  return tag;
};

export const addTagToKit = (tag: Tag): KitDiff => ({ tags: { added: [tag] } });
export const setTagInKit = (tag: Tag): KitDiff => ({ tags: { added: [tag] } });
export const removeTagFromKit = (tagGuid: string): KitDiff => ({
  tags: { removed: [{ guid: tagGuid }] },
});

export const findConceptInKit = (kit: Kit, conceptGuid: string): Concept => {
  const concept = (kit.concepts || []).find((c) => c.guid === conceptGuid);
  if (!concept) throw new Error(`Concept ${conceptGuid} not found in kit`);
  return concept;
};

export const addConceptToKit = (concept: Concept): KitDiff => ({ concepts: { added: [concept] } });
export const setConceptInKit = (concept: Concept): KitDiff => ({ concepts: { added: [concept] } });
export const removeConceptFromKit = (conceptGuid: string): KitDiff => ({
  concepts: { removed: [{ guid: conceptGuid }] },
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

// #region 🔖Design Family Helpers

export const getPrimitiveDesign = (kit: Kit, designGuid: string): Design => {
  let current = findDesignInKit(kit, designGuid);
  while (current.parent?.guid) {
    current = findDesignInKit(kit, current.parent.guid);
  }
  return current;
};

export const getDesignFamily = (kit: Kit, designGuid: string): Design[] => {
  const primitive = getPrimitiveDesign(kit, designGuid);
  const family: Design[] = [];
  const collectDescendants = (parentGuid: string) => {
    const parent = findDesignInKit(kit, parentGuid);
    family.push(parent);
    const children = (kit.designs || []).filter((d) => d.parent?.guid === parentGuid);
    children.forEach((child) => collectDescendants(child.guid));
  };
  collectDescendants(primitive.guid);
  return family;
};

export const getDesignSiblings = (kit: Kit, designGuid: string): Design[] => {
  const design = findDesignInKit(kit, designGuid);
  const parentGuid = design.parent?.guid;
  return (kit.designs || []).filter((d) => d.parent?.guid === parentGuid && d.guid !== designGuid);
};

export const getDesignChildren = (kit: Kit, designGuid: string): Design[] => {
  return (kit.designs || []).filter((d) => d.parent?.guid === designGuid);
};

export const areDesignsInSameFamily = (kit: Kit, designGuidA: string, designGuidB: string): boolean => {
  const primitiveA = getPrimitiveDesign(kit, designGuidA);
  const primitiveB = getPrimitiveDesign(kit, designGuidB);
  return primitiveA.guid === primitiveB.guid;
};

export const canUseDesignAsPiece = (kit: Kit, containerDesignGuid: string, pieceDesignGuid: string): boolean => {
  return !areDesignsInSameFamily(kit, containerDesignGuid, pieceDesignGuid);
};

export const findSameFamilyDesignPieces = (kit: Kit, designGuid: string): Piece[] => {
  const design = findDesignInKit(kit, designGuid);
  return (design.pieces || []).filter((piece) => {
    if (!piece.design?.guid) return false;
    return areDesignsInSameFamily(kit, designGuid, piece.design.guid);
  });
};

// #endregion 🔖Design Family Helpers

// #region 🔖Type Family Helpers

export const getPrimitiveType = (kit: Kit, typeGuid: string): Type => {
  let current = findTypeInKit(kit, typeGuid);
  while (current.parent?.guid) {
    current = findTypeInKit(kit, current.parent.guid);
  }
  return current;
};

export const getTypeFamily = (kit: Kit, typeGuid: string): Type[] => {
  const primitive = getPrimitiveType(kit, typeGuid);
  const family: Type[] = [];
  const collectDescendants = (parentGuid: string) => {
    const parent = findTypeInKit(kit, parentGuid);
    family.push(parent);
    const children = (kit.types || []).filter((t) => t.parent?.guid === parentGuid);
    children.forEach((child) => collectDescendants(child.guid));
  };
  collectDescendants(primitive.guid);
  return family;
};

export const getTypeSiblings = (kit: Kit, typeGuid: string): Type[] => {
  const type = findTypeInKit(kit, typeGuid);
  const parentGuid = type.parent?.guid;
  return (kit.types || []).filter((t) => t.parent?.guid === parentGuid && t.guid !== typeGuid);
};

export const getTypeChildren = (kit: Kit, typeGuid: string): Type[] => {
  return (kit.types || []).filter((t) => t.parent?.guid === typeGuid);
};

export const areTypesInSameFamily = (kit: Kit, typeGuidA: string, typeGuidB: string): boolean => {
  const primitiveA = getPrimitiveType(kit, typeGuidA);
  const primitiveB = getPrimitiveType(kit, typeGuidB);
  return primitiveA.guid === primitiveB.guid;
};

// #endregion 🔖Type Family Helpers

export const findPortInKit = (kit: Kit, portGuid: string): Port => {
  const iface = kit.ports?.find((i) => i.guid === portGuid);
  if (!iface) throw new Error(`Port ${portGuid} not found in kit ${kit.name}`);
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

export const findUsedConnectorsByPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Connector[] => {
  const design = findDesignInKit(kit, designGuid);
  const piece = findPieceInDesign(design, pieceGuid);
  if (!piece.type) return [];
  const type = findTypeInKit(kit, piece.type.guid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  return connections.map((c) => findConnectorForPieceInConnection(type, c, pieceGuid)).filter((p): p is Connector => p !== undefined);
};

export const findReplacableTypesForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string, variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designGuid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  const requiredConnectors: Connector[] = [];
  for (const connection of connections) {
    try {
      const otherPieceId = connection.connected.piece.guid === pieceGuid ? connection.connecting.piece.guid : connection.connected.piece.guid;
      const otherPiece = findPieceInDesign(design, otherPieceId);
      if (!otherPiece.type) continue;
      const otherType = findTypeInKit(kit, otherPiece.type.guid);
      const otherPortId = connection.connected.piece.guid === pieceGuid ? connection.connecting.connector?.guid : connection.connected.connector?.guid;
      const otherPort = findConnectorInType(otherType, otherPortId || "");
      requiredConnectors.push(otherPort);
    } catch (error) {
      continue;
    }
  }
  return (
    kit.types?.filter((replacementType) => {
      if (replacementType.isAbstract) return false;
      if (variants !== undefined && !variants.includes(replacementType.parent?.guid ?? "")) return false;
      if (!replacementType.connectors || replacementType.connectors.length === 0) return requiredConnectors.length === 0;
      return requiredConnectors.every((requiredConnector) => {
        return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
      });
    }) ?? []
  );
};

export const findReplacableTypesForPiecesInDesign = (kit: Kit, designGuid: string, pieceGuids: string[], variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designGuid);
  const pieces = pieceGuids.map((id) => findPieceInDesign(design, id));
  const externalConnections: Array<{
    connection: Connection;
    requiredConnector: Connector;
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
          const otherPortId = connection.connected.piece.guid === piece.guid ? connection.connecting.connector?.guid : connection.connected.connector?.guid;
          const otherPort = findConnectorInType(otherType, otherPortId || "");
          externalConnections.push({ connection, requiredConnector: otherPort });
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
      if (!replacementType.connectors || replacementType.connectors.length === 0) return externalConnections.length === 0;
      return externalConnections.every(({ requiredConnector }) => {
        return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
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

export const findAttributeValue = (entity: Kit | Type | Design | Piece | Connection | Model | Connector, name: string, defaultValue?: string | null): string | null => {
  const attribute = entity.attributes?.find((q) => q.key === name);
  if (!attribute && defaultValue === undefined) throw new Error(`Attribute ${name} not found in ${entity}`);
  if (attribute?.value === undefined && defaultValue === null) return null;
  return attribute?.value ?? defaultValue ?? "";
};

const getColorForText = (text?: string): string => {
  if (!text || text === "") return "var(--foreground)";

  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }

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
  const updated: { type: TypeId; diff: TypeDiff }[] = [];

  for (const type of types) {
    const updatedConnectors = (type.connectors || []).map((connector) => ({
      ...connector,
      attributes: [
        ...(connector.attributes || []),
        {
          guid: guid(),
          key: "semio.color",
          value: getColorForText(connector.port?.guid),
        },
      ],
    }));

    updated.push({
      type: { guid: type.guid },
      diff: {
        connectors: { added: updatedConnectors },
      },
    });
  }

  return { updated };
};

export const parseDesignIdFromVariant = (variant: string): string => {
  return variant.split("-")[0];
};

// #region 🔖File Tree Utilities

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

// #endregion 🔖File Tree Utilities

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

  let hash = 0;
  for (let i = 0; i < dataUri.length; i++) {
    const char = dataUri.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }

  return {
    guid: guid(),
    name,
    size,
    hash: hash.toString(36),
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };
};

// #region 🔖Kit Import/Export

export interface KitImportResult {
  kit: Kit;
  files: Map<string, Blob>;
}

let cachedSqlJs: any = null;
const getSqlJs = async () => {
  if (!cachedSqlJs) {
    const initSqlJs = (await import("sql.js")).default;
    try {
      const isNode = typeof process !== "undefined" && process.versions?.node;
      if (isNode) {
        const path = await import("path");
        const url = await import("url");
        const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
        cachedSqlJs = await initSqlJs({
          locateFile: (file: string) => path.join(__dirname, "public", file),
        });
      } else {
        cachedSqlJs = await initSqlJs({
          locateFile: (file: string) => `/${file}`,
        });
      }
    } catch (error) {
      console.error("Failed to initialize sql.js:", error);
      throw new Error("Failed to load SQLite database library.");
    }
  }
  return cachedSqlJs;
};

export const importKit = async (source: string | ArrayBuffer | Buffer | Blob): Promise<KitImportResult> => {
  const JSZip = (await import("jszip")).default;

  let arrayBuffer: ArrayBuffer;
  if (source instanceof Blob) {
    arrayBuffer = await source.arrayBuffer();
  } else if (typeof source === "string") {
    if (source.startsWith("blob:")) {
      const response = await fetch(source);
      if (!response.ok) {
        throw new Error(`Failed to fetch kit from blob URL: ${response.statusText}`);
      }
      arrayBuffer = await response.arrayBuffer();
    } else {
      const response = await fetch(source);
      if (!response.ok) {
        throw new Error(`Failed to fetch kit from ${source}: ${response.statusText}`);
      }
      arrayBuffer = await response.arrayBuffer();
    }
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
  const SQL = await getSqlJs();
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

export const exportKit = async (kit: Kit, files: Map<string, Blob>): Promise<Blob> => {
  const JSZip = (await import("jszip")).default;

  const SQL = await getSqlJs();
  const db = new SQL.Database();

  await kitToSqlite(kit, db);

  const dbData = db.export();
  db.close();

  const zip = new JSZip();
  zip.file(".semio/kit.db", dbData);

  for (const [path, blob] of files.entries()) {
    const arrayBuffer = await blob.arrayBuffer();
    zip.file(path, new Uint8Array(arrayBuffer));
  }

  return await zip.generateAsync({ type: "blob" });
};

export const areKitsEqual = (a: Kit, b: Kit): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined ? undefined : value);
  const normalizeBoolean = (value: boolean | undefined): boolean | undefined => (value ? true : undefined);

  const areAttributesEqual = (a?: Attribute[], b?: Attribute[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const attrA of arrA) {
      const attrB = arrB.find((x) => x.guid === attrA.guid);
      if (!attrB) return false;
      if (attrA.key !== attrB.key) return false;
      if (normalizeValue(attrA.value) !== normalizeValue(attrB.value)) return false;
      if (normalizeValue(attrA.definition) !== normalizeValue(attrB.definition)) return false;
    }
    return true;
  };

  const arePropsEqual = (a?: Prop[], b?: Prop[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const propA of arrA) {
      const propB = arrB.find((x) => x.guid === propA.guid);
      if (!propB) return false;
      if (propA.quality.guid !== propB.quality.guid) return false;
      if (propA.value !== propB.value) return false;
      if (normalizeValue(propA.unit) !== normalizeValue(propB.unit)) return false;
      if (!areAttributesEqual(propA.attributes, propB.attributes)) return false;
    }
    return true;
  };

  const areConnectorsEqual = (a?: Connector[], b?: Connector[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const connectorA of arrA) {
      const connectorB = arrB.find((x) => x.guid === connectorA.guid);
      if (!connectorB) return false;
      if (normalizeValue(connectorA.name) !== normalizeValue(connectorB.name)) return false;
      if (connectorA.point.x !== connectorB.point.x) return false;
      if (connectorA.point.y !== connectorB.point.y) return false;
      if (connectorA.point.z !== connectorB.point.z) return false;
      if (connectorA.direction.x !== connectorB.direction.x) return false;
      if (connectorA.direction.y !== connectorB.direction.y) return false;
      if (connectorA.direction.z !== connectorB.direction.z) return false;
      if (connectorA.t !== connectorB.t) return false;
      if (normalizeBoolean(connectorA.mandatory) !== normalizeBoolean(connectorB.mandatory)) return false;
      if (normalizeValue(connectorA.port?.guid) !== normalizeValue(connectorB.port?.guid)) return false;
      if (!arePropsEqual(connectorA.props, connectorB.props)) return false;
      if (!areAttributesEqual(connectorA.attributes, connectorB.attributes)) return false;
    }
    return true;
  };

  const areModelsEqual = (a?: Model[], b?: Model[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const modelA of arrA) {
      const modelB = arrB.find((x) => x.guid === modelA.guid);
      if (!modelB) return false;
      if (normalizeValue(modelA.name) !== normalizeValue(modelB.name)) return false;
      if (modelA.file.guid !== modelB.file.guid) return false;

      const tagsA = normalizeArray(modelA.tags).map((t) => (typeof t === "object" ? t.guid : t));
      const tagsB = normalizeArray(modelB.tags).map((t) => (typeof t === "object" ? t.guid : t));
      if (tagsA.length !== tagsB.length || !tagsA.every((g) => tagsB.includes(g))) return false;
      if (!areAttributesEqual(modelA.attributes, modelB.attributes)) return false;
    }
    return true;
  };

  const areTypesEqual = (a?: Type[], b?: Type[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const typeA of arrA) {
      const typeB = arrB.find((t) => {
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
      if (normalizeValue(typeA.folder) !== normalizeValue(typeB.folder)) return false;
      if (normalizeValue(typeA.unit) !== normalizeValue(typeB.unit)) return false;
      if (typeA.stock !== typeB.stock) return false;
      if (normalizeBoolean(typeA.isAbstract) !== normalizeBoolean(typeB.isAbstract)) return false;
      if (normalizeBoolean(typeA.virtual) !== normalizeBoolean(typeB.virtual)) return false;
      if (normalizeValue(typeA.location?.guid) !== normalizeValue(typeB.location?.guid)) return false;
      if (!arraysEqual(normalizeArray(typeA.concepts), normalizeArray(typeB.concepts))) return false;
      if (!arraysEqual(normalizeArray(typeA.authors?.map((a) => a.guid)), normalizeArray(typeB.authors?.map((a) => a.guid)))) return false;
      if (!arePropsEqual(typeA.props, typeB.props)) return false;
      if (!areModelsEqual(typeA.models, typeB.models)) return false;
      if (!areConnectorsEqual(typeA.connectors, typeB.connectors)) return false;
      if (!areAttributesEqual(typeA.attributes, typeB.attributes)) return false;
    }
    return true;
  };

  const arePiecesEqual = (a?: Piece[], b?: Piece[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const pieceA of arrA) {
      const pieceB = arrB.find((x) => x.guid === pieceA.guid);
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
      if (normalizeBoolean(pieceA.isHidden) !== normalizeBoolean(pieceB.isHidden)) return false;
      if (normalizeBoolean(pieceA.isLocked) !== normalizeBoolean(pieceB.isLocked)) return false;
      if (normalizeValue(pieceA.color) !== normalizeValue(pieceB.color)) return false;
      if (normalizeValue(pieceA.description) !== normalizeValue(pieceB.description)) return false;
      if (!arePropsEqual(pieceA.props, pieceB.props)) return false;
      if (!areAttributesEqual(pieceA.attributes, pieceB.attributes)) return false;
    }
    return true;
  };

  const areConnectionsEqual = (a?: Connection[], b?: Connection[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const connA of arrA) {
      const connB = arrB.find((x) => x.guid === connA.guid);
      if (!connB) return false;
      if (connA.connected.piece.guid !== connB.connected.piece.guid) return false;
      if (normalizeValue(connA.connected.designPiece?.guid) !== normalizeValue(connB.connected.designPiece?.guid)) return false;
      if (normalizeValue(connA.connected.connector?.guid) !== normalizeValue(connB.connected.connector?.guid)) return false;
      if (connA.connecting.piece.guid !== connB.connecting.piece.guid) return false;
      if (normalizeValue(connA.connecting.designPiece?.guid) !== normalizeValue(connB.connecting.designPiece?.guid)) return false;
      if (normalizeValue(connA.connecting.connector?.guid) !== normalizeValue(connB.connecting.connector?.guid)) return false;
      if (connA.gap !== connB.gap) return false;
      if (connA.shift !== connB.shift) return false;
      if (connA.rise !== connB.rise) return false;
      if (connA.rotation !== connB.rotation) return false;
      if (connA.turn !== connB.turn) return false;
      if (connA.tilt !== connB.tilt) return false;
      if (connA.u !== connB.u) return false;
      if (connA.v !== connB.v) return false;
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
      const designB = arrB.find((d) => {
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

  const arePortsEqual = (a?: Port[], b?: Port[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const ifaceA of arrA) {
      const ifaceB = arrB.find((x) => x.guid === ifaceA.guid);
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
      const qualB = arrB.find((x) => x.guid === qualA.guid);
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
      const fileB = arrB.find((x) => x.guid === fileA.guid);
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
      const folderB = arrB.find((x) => x.guid === folderA.guid);
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
      const authorB = arrB.find((x) => x.guid === authorA.guid);
      if (!authorB) return false;
      if (authorA.name !== authorB.name) return false;
      if (normalizeValue(authorA.email) !== normalizeValue(authorB.email)) return false;
      if (!areAttributesEqual(authorA.attributes, authorB.attributes)) return false;
    }
    return true;
  };

  const areConceptsEqual = (a?: Concept[], b?: Concept[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const conceptA of arrA) {
      const conceptB = arrB.find((x) => x.guid === conceptA.guid);
      if (!conceptB) return false;
      if (conceptA.name !== conceptB.name) return false;
      if (normalizeValue(conceptA.description) !== normalizeValue(conceptB.description)) return false;
      if (normalizeValue(conceptA.icon) !== normalizeValue(conceptB.icon)) return false;
    }
    return true;
  };

  const areTagsEqual = (a?: Tag[], b?: Tag[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const tagA of arrA) {
      const tagB = arrB.find((x) => x.guid === tagA.guid);
      if (!tagB) return false;
      if (tagA.name !== tagB.name) return false;
      if (normalizeValue(tagA.description) !== normalizeValue(tagB.description)) return false;
      if (normalizeValue(tagA.icon) !== normalizeValue(tagB.icon)) return false;
    }
    return true;
  };

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

  if (!areConceptsEqual(a.concepts, b.concepts)) return false;
  if (!areTagsEqual(a.tags, b.tags)) return false;
  if (!areTypesEqual(a.types, b.types)) return false;
  if (!areDesignsEqual(a.designs, b.designs)) return false;
  if (!arePortsEqual(a.ports, b.ports)) return false;
  if (!areQualitiesEqual(a.qualities, b.qualities)) return false;
  if (!areFilesEqual(a.files, b.files)) return false;
  if (!areFoldersEqual(a.folders, b.folders)) return false;
  if (!areAuthorsEqual(a.authors, b.authors)) return false;
  if (!areAttributesEqual(a.attributes, b.attributes)) return false;

  return true;
};

export const areKitDiffsEqual = (a: KitDiff, b: KitDiff): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined ? undefined : value);
  const normalizeBoolean = (value: boolean | undefined): boolean | undefined => (value ? true : undefined);
  const areRemovedArraysEqual = (a?: { guid: string }[], b?: { guid: string }[]): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (a.length !== b.length) return false;
    const aGuids = new Set(a.map((x) => x.guid));
    const bGuids = new Set(b.map((x) => x.guid));
    for (const guid of aGuids) {
      if (!bGuids.has(guid)) return false;
    }
    return true;
  };

  const areAttributesDiffsEqual = (a?: AttributesDiff, b?: AttributesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.attribute.guid === ua.attribute.guid);
      if (!ub) return false;
      if (!areAttributeDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.key !== ab.key) return false;
      if (normalizeValue(aa.value) !== normalizeValue(ab.value)) return false;
      if (normalizeValue(aa.definition) !== normalizeValue(ab.definition)) return false;
    }
    return true;
  };

  const areAttributeDiffsEqual = (a?: AttributeDiff, b?: AttributeDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.key) !== normalizeValue(b.key)) return false;
    if (normalizeValue(a.value) !== normalizeValue(b.value)) return false;
    if (normalizeValue(a.definition) !== normalizeValue(b.definition)) return false;
    return true;
  };

  const arePropsDiffsEqual = (a?: PropsDiff, b?: PropsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.prop.guid === ua.prop.guid);
      if (!ub) return false;
      if (!arePropDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.quality.guid !== ab.quality.guid) return false;
      if (aa.value !== ab.value) return false;
      if (normalizeValue(aa.unit) !== normalizeValue(ab.unit)) return false;
    }
    return true;
  };

  const arePropDiffsEqual = (a?: PropDiff, b?: PropDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.value) !== normalizeValue(b.value)) return false;
    if (normalizeValue(a.unit) !== normalizeValue(b.unit)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areConnectorsDiffsEqual = (a?: z.infer<typeof ConnectorsDiffSchema>, b?: z.infer<typeof ConnectorsDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.connector.guid === ua.connector.guid);
      if (!ub) return false;
      if (!areConnectorDiffEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (aa.point.x !== ab.point.x) return false;
      if (aa.point.y !== ab.point.y) return false;
      if (aa.point.z !== ab.point.z) return false;
      if (aa.direction.x !== ab.direction.x) return false;
      if (aa.direction.y !== ab.direction.y) return false;
      if (aa.direction.z !== ab.direction.z) return false;
      if (aa.t !== ab.t) return false;
      if (normalizeBoolean(aa.mandatory) !== normalizeBoolean(ab.mandatory)) return false;
    }
    return true;
  };

  const areConnectorDiffEqual = (a?: ConnectorDiff, b?: ConnectorDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (a.point && b.point) {
      if (normalizeValue(a.point.x) !== normalizeValue(b.point.x)) return false;
      if (normalizeValue(a.point.y) !== normalizeValue(b.point.y)) return false;
      if (normalizeValue(a.point.z) !== normalizeValue(b.point.z)) return false;
    } else if (a.point || b.point) {
      return false;
    }
    if (a.direction && b.direction) {
      if (normalizeValue(a.direction.x) !== normalizeValue(b.direction.x)) return false;
      if (normalizeValue(a.direction.y) !== normalizeValue(b.direction.y)) return false;
      if (normalizeValue(a.direction.z) !== normalizeValue(b.direction.z)) return false;
    } else if (a.direction || b.direction) {
      return false;
    }
    if (normalizeValue(a.t) !== normalizeValue(b.t)) return false;
    if (normalizeValue(a.mandatory) !== normalizeValue(b.mandatory)) return false;
    if (normalizeValue(a.port?.guid) !== normalizeValue(b.port?.guid)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areModelsDiffsEqual = (a?: z.infer<typeof ModelsDiffSchema>, b?: z.infer<typeof ModelsDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.model.guid === ua.model.guid);
      if (!ub) return false;
      if (!areModelDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (aa.file !== ab.file) return false;
      if (!arraysEqual(normalizeArray(aa.tags), normalizeArray(ab.tags))) return false;
    }
    return true;
  };

  const areModelDiffsEqual = (a?: ModelDiff, b?: ModelDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.file) !== normalizeValue(b.file)) return false;
    if (a.tags && b.tags) {
      if (!arraysEqual(normalizeArray(a.tags), normalizeArray(b.tags))) return false;
    } else if (a.tags || b.tags) {
      return false;
    }
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areTypesDiffsEqual = (a?: TypesDiff, b?: TypesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.type.guid === ua.type.guid);
      if (!ub) return false;
      if (!areTypeDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areTypeDiffsEqual = (a?: TypeDiff, b?: TypeDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
    if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
    if (normalizeValue(a.folder) !== normalizeValue(b.folder)) return false;
    if (normalizeValue(a.unit) !== normalizeValue(b.unit)) return false;
    if (normalizeValue(a.stock) !== normalizeValue(b.stock)) return false;
    if (normalizeValue(a.isAbstract) !== normalizeValue(b.isAbstract)) return false;
    if (normalizeValue(a.virtual) !== normalizeValue(b.virtual)) return false;
    if (normalizeValue(a.location?.guid) !== normalizeValue(b.location?.guid)) return false;
    if (a.concepts && b.concepts) {
      if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
    } else if (a.concepts || b.concepts) {
      return false;
    }
    if (!areModelsDiffsEqual(a.models, b.models)) return false;
    if (!areConnectorsDiffsEqual(a.connectors, b.connectors)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const arePiecesDiffsEqual = (a?: PiecesDiff, b?: PiecesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.piece.guid === ua.piece.guid);
      if (!ub) return false;
      if (!arePieceDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (aa.type?.guid !== ab.type?.guid) return false;
      if (aa.design?.guid !== ab.design?.guid) return false;
    }
    return true;
  };

  const arePieceDiffsEqual = (a?: PieceDiff, b?: PieceDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.type?.guid) !== normalizeValue(b.type?.guid)) return false;
    if (normalizeValue(a.design?.guid) !== normalizeValue(b.design?.guid)) return false;
    if (a.plane && b.plane) {
      if (a.plane.origin && b.plane.origin) {
        if (normalizeValue(a.plane.origin.x) !== normalizeValue(b.plane.origin.x)) return false;
        if (normalizeValue(a.plane.origin.y) !== normalizeValue(b.plane.origin.y)) return false;
        if (normalizeValue(a.plane.origin.z) !== normalizeValue(b.plane.origin.z)) return false;
      } else if (a.plane.origin || b.plane.origin) {
        return false;
      }
      if (a.plane.xAxis && b.plane.xAxis) {
        if (normalizeValue(a.plane.xAxis.x) !== normalizeValue(b.plane.xAxis.x)) return false;
        if (normalizeValue(a.plane.xAxis.y) !== normalizeValue(b.plane.xAxis.y)) return false;
        if (normalizeValue(a.plane.xAxis.z) !== normalizeValue(b.plane.xAxis.z)) return false;
      } else if (a.plane.xAxis || b.plane.xAxis) {
        return false;
      }
      if (a.plane.yAxis && b.plane.yAxis) {
        if (normalizeValue(a.plane.yAxis.x) !== normalizeValue(b.plane.yAxis.x)) return false;
        if (normalizeValue(a.plane.yAxis.y) !== normalizeValue(b.plane.yAxis.y)) return false;
        if (normalizeValue(a.plane.yAxis.z) !== normalizeValue(b.plane.yAxis.z)) return false;
      } else if (a.plane.yAxis || b.plane.yAxis) {
        return false;
      }
    } else if (a.plane || b.plane) {
      return false;
    }
    if (normalizeValue(a.scale) !== normalizeValue(b.scale)) return false;
    if (normalizeValue(a.isHidden) !== normalizeValue(b.isHidden)) return false;
    if (normalizeValue(a.isLocked) !== normalizeValue(b.isLocked)) return false;
    if (normalizeValue(a.color) !== normalizeValue(b.color)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areConnectionsDiffsEqual = (a?: ConnectionsDiff, b?: ConnectionsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.connection.guid === ua.connection.guid);
      if (!ub) return false;
      if (!areConnectionDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.connected.piece.guid !== ab.connected.piece.guid) return false;
      if (aa.connecting.piece.guid !== ab.connecting.piece.guid) return false;
    }
    return true;
  };

  const areConnectionDiffsEqual = (a?: ConnectionDiff, b?: ConnectionDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.gap) !== normalizeValue(b.gap)) return false;
    if (normalizeValue(a.shift) !== normalizeValue(b.shift)) return false;
    if (normalizeValue(a.rise) !== normalizeValue(b.rise)) return false;
    if (normalizeValue(a.rotation) !== normalizeValue(b.rotation)) return false;
    if (normalizeValue(a.turn) !== normalizeValue(b.turn)) return false;
    if (normalizeValue(a.tilt) !== normalizeValue(b.tilt)) return false;
    if (normalizeValue(a.u) !== normalizeValue(b.u)) return false;
    if (normalizeValue(a.v) !== normalizeValue(b.v)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areDesignsDiffsEqual = (a?: DesignsDiff, b?: DesignsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.design.guid === ua.design.guid);
      if (!ub) return false;
      if (!areDesignDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areDesignDiffsEqual = (a?: DesignDiff, b?: DesignDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
    if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
    if (a.concepts && b.concepts) {
      if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
    } else if (a.concepts || b.concepts) {
      return false;
    }
    if (!arePiecesDiffsEqual(a.pieces, b.pieces)) return false;
    if (!areConnectionsDiffsEqual(a.connections, b.connections)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const arePortsDiffsEqual = (a?: PortsDiff, b?: PortsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.port.guid === ua.port.guid);
      if (!ub) return false;
      if (!arePortDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const arePortDiffsEqual = (a?: PortDiff, b?: PortDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areQualitiesDiffsEqual = (a?: z.infer<typeof QualitiesDiffSchema>, b?: z.infer<typeof QualitiesDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.quality.guid === ua.quality.guid);
      if (!ub) return false;
      if (!areQualityDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.key !== ab.key) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areQualityDiffsEqual = (a?: QualityDiff, b?: QualityDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.key) !== normalizeValue(b.key)) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areFilesDiffsEqual = (a?: FilesDiff, b?: FilesDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.file.guid === ua.file.guid);
      if (!ub) return false;
      if (!areFileDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areFileDiffsEqual = (a?: FileDiff, b?: FileDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    return true;
  };

  const areFoldersDiffsEqual = (a?: FoldersDiff, b?: FoldersDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.folder.guid === ua.folder.guid);
      if (!ub) return false;
      if (!areFolderDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areFolderDiffsEqual = (a?: FolderDiff, b?: FolderDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areAuthorsDiffsEqual = (a?: AuthorsDiff, b?: AuthorsDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.author.guid === ua.author.guid);
      if (!ub) return false;
      if (!areAuthorDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.guid === aa.guid);
      if (!ab) return false;
      if (aa.name !== ab.name) return false;
    }
    return true;
  };

  const areAuthorDiffsEqual = (a?: AuthorDiff, b?: AuthorDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.email) !== normalizeValue(b.email)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
  if (normalizeValue(a.version) !== normalizeValue(b.version)) return false;
  if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
  if (normalizeValue(a.icon) !== normalizeValue(b.icon)) return false;
  if (normalizeValue(a.image) !== normalizeValue(b.image)) return false;
  if (normalizeValue(a.preview) !== normalizeValue(b.preview)) return false;
  if (normalizeValue(a.remote) !== normalizeValue(b.remote)) return false;
  if (normalizeValue(a.homepage) !== normalizeValue(b.homepage)) return false;
  if (normalizeValue(a.license) !== normalizeValue(b.license)) return false;

  if (a.concepts && b.concepts) {
    if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
  } else if (a.concepts || b.concepts) {
    return false;
  }
  if (!areTypesDiffsEqual(a.types, b.types)) return false;
  if (!areDesignsDiffsEqual(a.designs, b.designs)) return false;
  if (!arePortsDiffsEqual(a.ports, b.ports)) return false;
  if (!areQualitiesDiffsEqual(a.qualities, b.qualities)) return false;
  if (!areFilesDiffsEqual(a.files, b.files)) return false;
  if (!areFoldersDiffsEqual(a.folders, b.folders)) return false;
  if (!areAuthorsDiffsEqual(a.authors, b.authors)) return false;
  if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;

  return true;
};

const sqliteToKit = async (db: any): Promise<Kit> => {
  const existingTables = new Set<string>();
  const tableStmt = db.prepare("SELECT name FROM sqlite_master WHERE type='table'");
  while (tableStmt.step()) {
    existingTables.add(tableStmt.getAsObject().name as string);
  }
  tableStmt.free();

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

  const safeExecResult = (tableName: string, query: string, params?: any[]): any[] => {
    if (!existingTables.has(tableName)) {
      return [];
    }
    return execResult(query, params);
  };

  const kitRows = execResult("SELECT * FROM kit LIMIT 1");
  if (kitRows.length === 0) {
    throw new Error("No kit found in database");
  }
  const kitRow = kitRows[0];

  const toUndefined = (value: any): any => (value === null || value === "" ? undefined : value);
  const buildAttribute = (a: any): any => {
    const attr: any = { guid: a.guid, key: a.key };
    const value = toUndefined(a.value);
    const definition = toUndefined(a.definition);
    if (value !== undefined) attr.value = value;
    if (definition !== undefined) attr.definition = definition;
    return attr;
  };
  const mapOrUndefined = <T, R>(arr: T[], mapper: (item: T) => R): R[] | undefined => (arr.length > 0 ? arr.map(mapper) : undefined);

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
    createdAt: kitRow.created,
    updatedAt: kitRow.updated,
  };

  const types = execResult("SELECT * FROM type WHERE kit_guid = ?", [kit.guid]);
  kit.types = mapOrUndefined(types, (row: any) => {
    const typeGuid = row.guid || String(row.id);
    const models = execResult("SELECT * FROM model WHERE type_guid = ?", [typeGuid]);
    const connectors = execResult("SELECT * FROM connector WHERE type_guid = ?", [typeGuid]);
    const typeAttributes = execResult("SELECT * FROM attribute WHERE type_guid = ?", [typeGuid]);
    const typeConcepts = execResult("SELECT * FROM type_concept WHERE type_guid = ?", [typeGuid]);
    const typeAuthors = execResult("SELECT * FROM type_author WHERE type_guid = ? ORDER BY rank", [typeGuid]);

    const type: any = {
      guid: typeGuid,
      name: row.name,
      createdAt: row.created,
      updatedAt: row.updated,
    };
    if (row.is_abstract) type.isAbstract = true;
    const folder = toUndefined(row.folder);
    if (folder !== undefined) type.folder = folder;
    const description = toUndefined(row.description);
    if (description !== undefined) type.description = description;
    const icon = toUndefined(row.icon);
    if (icon !== undefined) type.icon = icon;
    const image = toUndefined(row.image);
    if (image !== undefined) type.image = image;
    if (row.parent_guid || row.parent_id) type.parent = { guid: row.parent_guid || String(row.parent_id) };
    if (row.virtual) type.virtual = true;
    const unit = toUndefined(row.unit);
    if (unit !== undefined) type.unit = unit;
    if (row.stock !== null && row.stock !== undefined) type.stock = row.stock;
    if (row.location_guid) type.location = { guid: row.location_guid };

    const concepts = mapOrUndefined(typeConcepts, (c: any) => c.concept);
    if (concepts) type.concepts = concepts;

    const authors = mapOrUndefined(typeAuthors, (ta: any) => ({ guid: ta.author_guid }));
    if (authors) type.authors = authors;

    const models_value = mapOrUndefined(models, (m: any) => {
      const modelTags = execResult("SELECT tag_guid FROM model_tag WHERE model_guid = ?", [m.guid]);
      const modelAttributes = execResult("SELECT * FROM attribute WHERE model_guid = ?", [m.guid]);
      return {
        guid: m.guid,
        file: { guid: m.file_guid },
        name: toUndefined(m.name),
        description: toUndefined(m.description),
        tags: modelTags.map((t: any) => ({ guid: t.tag_guid })),
        attributes: mapOrUndefined(modelAttributes, buildAttribute),
      };
    });
    if (models_value) type.models = models_value;

    const connectors_value = mapOrUndefined(connectors, (p: any) => {
      const connectorProps = execResult("SELECT * FROM prop WHERE connector_guid = ?", [p.guid]);
      const connectorAttributes = execResult("SELECT * FROM attribute WHERE connector_guid = ?", [p.guid]);

      const connector: any = {
        guid: p.guid,
        point: { x: p.point_x, y: p.point_y, z: p.point_z },
        direction: { x: p.direction_x, y: p.direction_y, z: p.direction_z },
        t: p.t,
      };

      if (p.name) connector.name = p.name;
      if (p.mandatory) connector.mandatory = true;
      if (p.port_guid) connector.port = { guid: p.port_guid };
      if (p.description) connector.description = p.description;

      const props_value = connectorProps
        .map((pr: any) => {
          const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
          if (!pr.quality_guid) return null;
          return {
            guid: pr.guid,
            value: String(pr.value),
            unit: toUndefined(pr.unit),
            quality: { guid: pr.quality_guid },
            attributes: mapOrUndefined(propAttributes, buildAttribute),
          };
        })
        .filter((p: any): p is NonNullable<typeof p> => p !== null);
      if (props_value && props_value.length > 0) connector.props = props_value;

      const attributes_value = mapOrUndefined(connectorAttributes, buildAttribute);
      if (attributes_value) connector.attributes = attributes_value;

      return connector;
    });
    if (connectors_value) type.connectors = connectors_value;

    const attributes_value = mapOrUndefined(typeAttributes, buildAttribute);
    if (attributes_value) type.attributes = attributes_value;

    return type;
  });

  const designs = execResult("SELECT * FROM design WHERE kit_guid = ?", [kit.guid]);
  kit.designs = mapOrUndefined(designs, (row: any) => {
    const designGuid = row.guid || String(row.id);
    const pieces = execResult("SELECT * FROM piece WHERE design_guid = ?", [designGuid]);
    const connections = execResult("SELECT * FROM connection WHERE design_guid = ?", [designGuid]);
    const layers = execResult("SELECT * FROM layer WHERE design_guid = ?", [designGuid]);
    const groups = execResult('SELECT * FROM "group" WHERE design_guid = ?', [designGuid]);
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
      parent: row.parent_guid ? { guid: row.parent_guid } : row.parent_id ? { guid: String(row.parent_id) } : undefined,
      unit: toUndefined(row.unit),
      isAbstract: row.is_abstract ? true : undefined,
      folder: toUndefined(row.folder),
      canScale: row.can_scale ? true : undefined,
      canMirror: row.can_mirror ? true : undefined,
      createdAt: row.created,
      updatedAt: row.updated,
      activeLayer: row.active_layer_guid ? { guid: row.active_layer_guid } : undefined,
      props: mapOrUndefined(designProps, (dp: any) => ({
        guid: dp.guid,
        quality: { guid: dp.quality_guid },
        value: String(dp.value),
        unit: toUndefined(dp.unit),
      })),
      authors: mapOrUndefined(designAuthors, (da: any) => ({ guid: da.author_guid })),
      pieces: pieces.map((p: any) => {
        const pieceProps = execResult("SELECT prop.* FROM prop JOIN piece_prop ON prop.guid = piece_prop.prop_guid WHERE piece_prop.piece_guid = ?", [p.guid]);
        const pieceAttributes = execResult("SELECT * FROM attribute WHERE piece_guid = ?", [p.guid]);
        return {
          guid: p.guid,
          name: toUndefined(p.name),
          type: p.type_guid ? { guid: p.type_guid } : undefined,
          design: p.design_guid_ref ? { guid: p.design_guid_ref } : undefined,
          plane:
            p.plane_origin_x !== null
              ? {
                origin: { x: p.plane_origin_x, y: p.plane_origin_y, z: p.plane_origin_z },
                xAxis: { x: p.plane_x_axis_x, y: p.plane_x_axis_y, z: p.plane_x_axis_z },
                yAxis: { x: p.plane_y_axis_x, y: p.plane_y_axis_y, z: p.plane_y_axis_z },
              }
              : undefined,
          center: p.center_u !== null || p.center_v !== null ? { u: p.center_u, v: p.center_v } : undefined,
          scale: p.scale !== null ? p.scale : undefined,
          mirrorPlane:
            p.mirror_plane_origin_x !== null
              ? {
                origin: { x: p.mirror_plane_origin_x, y: p.mirror_plane_origin_y, z: p.mirror_plane_origin_z },
                xAxis: { x: p.mirror_plane_x_axis_x, y: p.mirror_plane_x_axis_y, z: p.mirror_plane_x_axis_z },
                yAxis: { x: p.mirror_plane_y_axis_x, y: p.mirror_plane_y_axis_y, z: p.mirror_plane_y_axis_z },
              }
              : undefined,
          isHidden: p.is_hidden ? true : undefined,
          isLocked: p.is_locked ? true : undefined,
          color: toUndefined(p.color),
          description: toUndefined(p.description),
          props: (() => {
            const filtered = pieceProps
              .map((pr: any) => {
                const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
                if (!pr.quality_guid) return null;
                return {
                  guid: pr.guid,
                  value: String(pr.value),
                  unit: toUndefined(pr.unit),
                  quality: { guid: pr.quality_guid },
                  attributes: mapOrUndefined(propAttributes, buildAttribute),
                };
              })
              .filter((p: any): p is NonNullable<typeof p> => p !== null);
            return filtered.length > 0 ? filtered : undefined;
          })(),
          attributes: mapOrUndefined(pieceAttributes, buildAttribute),
        };
      }),
      connections: connections.map((c: any) => {
        const connectionAttributes = execResult("SELECT * FROM attribute WHERE connection_guid = ?", [c.guid]);
        return {
          guid: c.guid,
          connected: {
            piece: { guid: c.connected_piece_guid },
            designPiece: c.connected_design_piece_guid ? { guid: c.connected_design_piece_guid } : undefined,
            connector: { guid: c.connected_connector_guid },
          },
          connecting: {
            piece: { guid: c.connecting_piece_guid },
            designPiece: c.connecting_design_piece_guid ? { guid: c.connecting_design_piece_guid } : undefined,
            connector: { guid: c.connecting_connector_guid },
          },
          gap: c.gap || 0,
          shift: c.shift || 0,
          rise: c.rise || 0,
          rotation: c.rotation || 0,
          turn: c.turn || 0,
          tilt: c.tilt || 0,
          u: c.u !== null ? c.u : undefined,
          v: c.v !== null ? c.v : undefined,
          description: toUndefined(c.description),
          attributes: mapOrUndefined(connectionAttributes, buildAttribute),
        };
      }),
      layers: layers.map((l: any) => {
        const layerAttributes = execResult("SELECT * FROM attribute WHERE layer_guid = ?", [l.guid]);
        return {
          guid: l.guid,
          path: l.path,
          isHidden: l.is_hidden ? true : undefined,
          isLocked: l.is_locked ? true : undefined,
          color: toUndefined(l.color),
          description: toUndefined(l.description),
          attributes: mapOrUndefined(layerAttributes, buildAttribute),
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
          attributes: mapOrUndefined(groupAttributes, buildAttribute),
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
      attributes: mapOrUndefined(designAttributes, buildAttribute),
      concepts: designConcepts.length > 0 ? designConcepts.map((c: any) => c.concept) : undefined,
    };
  });

  const ports = execResult("SELECT * FROM port WHERE kit_guid = ?", [kit.guid]);
  kit.ports = mapOrUndefined(ports, (row: any) => {
    const compatiblePorts = execResult("SELECT compatible_port_guid FROM port_compatibility WHERE port_guid = ?", [row.guid]);
    const portAttributes = execResult("SELECT * FROM attribute WHERE port_guid = ?", [row.guid]);
    return {
      guid: row.guid,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      compatiblePorts: compatiblePorts.length > 0 ? compatiblePorts.map((ci: any) => ({ guid: ci.compatible_port_guid })) : undefined,
      attributes: mapOrUndefined(portAttributes, buildAttribute),
    };
  });

  const tags = safeExecResult("tag", "SELECT * FROM tag WHERE kit_guid = ?", [kit.guid]);
  kit.tags = mapOrUndefined(tags, (row: any) => ({
    guid: row.guid,
    name: row.name,
    description: toUndefined(row.description),
    icon: toUndefined(row.icon),
  }));

  const qualities = execResult("SELECT * FROM quality WHERE kit_guid = ?", [kit.guid]);
  kit.qualities =
    qualities.length > 0
      ? qualities.map((row: any) => {
        const benchmarks = execResult("SELECT * FROM benchmark WHERE quality_guid = ?", [row.guid]);
        const qualityAttributes = execResult("SELECT * FROM attribute WHERE quality_guid = ?", [row.guid]);
        return {
          guid: row.guid,
          key: row.key,
          name: row.name,
          kind: row.kind,
          defaultValue: row.default_value,
          formula: toUndefined(row.formula),
          defaultSiUnit: toUndefined(row.default_si_unit),
          defaultImperialUnit: toUndefined(row.default_imperial_unit),
          min: row.min_value,
          minExcluded: row.min_excluded ? true : undefined,
          max: row.max_value,
          maxExcluded: row.max_excluded ? true : undefined,
          canScale: row.can_scale ? true : undefined,
          uri: toUndefined(row.definition),
          benchmarks: benchmarks.map((b: any) => {
            const benchmarkAttributes = execResult("SELECT * FROM attribute WHERE benchmark_guid = ?", [b.guid]);
            return {
              guid: b.guid,
              name: b.name,
              icon: toUndefined(b.icon),
              min: b.min_value,
              minExcluded: b.min_excluded ? true : undefined,
              max: b.max_value,
              maxExcluded: b.max_excluded ? true : undefined,
              attributes: mapOrUndefined(benchmarkAttributes, buildAttribute),
            };
          }),
          attributes: mapOrUndefined(qualityAttributes, buildAttribute),
        };
      })
      : undefined;

  const files = execResult("SELECT * FROM file WHERE kit_guid = ?", [kit.guid]);
  kit.files =
    files.length > 0
      ? files.map((row: any) => ({
        guid: row.guid,
        name: row.name,
        mime: toUndefined(row.mime),
        remote: toUndefined(row.remote_url),
        folder: row.folder_guid ? { guid: row.folder_guid } : undefined,
        size: row.size,
        hash: row.hash,
        createdAt: row.created,
        updatedAt: row.updated,
      }))
      : undefined;

  const folders = execResult("SELECT * FROM folder WHERE kit_guid = ?", [kit.guid]);
  kit.folders = mapOrUndefined(folders, (row: any) => ({
    guid: row.guid,
    name: row.name,
    parent: row.parent_guid ? { guid: row.parent_guid } : undefined,
    createdAt: row.created,
    updatedAt: row.updated,
  }));

  const authors = execResult("SELECT * FROM author WHERE kit_guid = ?", [kit.guid]);
  kit.authors =
    authors.length > 0
      ? authors.map((row: any) => ({
        guid: row.guid,
        name: row.name,
        email: toUndefined(row.email),
      }))
      : undefined;

  const concepts = execResult("SELECT * FROM concept WHERE kit_guid = ?", [kit.guid]);
  kit.concepts = mapOrUndefined(concepts, (row: any) => ({
    guid: row.guid,
    name: row.name,
    description: toUndefined(row.description),
    icon: toUndefined(row.icon),
  }));

  const kitAttributes = execResult("SELECT * FROM attribute WHERE kit_guid = ?", [kit.guid]);
  kit.attributes = mapOrUndefined(kitAttributes, buildAttribute);

  return kit;
};

const toArray = <T>(value: T | T[] | undefined): T[] => {
  if (!value) return [];
  return Array.isArray(value) ? value : [value];
};

export const KIT_SQLITE_SCHEMA = `
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

CREATE TABLE port (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE port_compatibility (
	port_guid VARCHAR(36) NOT NULL,
	compatible_port_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (port_guid, compatible_port_guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(compatible_port_guid) REFERENCES port (guid)
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
	mime VARCHAR(128),
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

CREATE TABLE tag (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
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
	file_guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(file_guid) REFERENCES file (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE model_tag (
	model_guid VARCHAR(36) NOT NULL,
	tag_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (model_guid, tag_guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(tag_guid) REFERENCES tag (guid)
);

CREATE TABLE prop (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	quality_guid VARCHAR(36),
	connector_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE connector (
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
	port_guid VARCHAR(36),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	UNIQUE (guid, type_guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
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
	guid VARCHAR(36) NOT NULL,
	design_guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	PRIMARY KEY (guid),
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
	connected_connector_guid VARCHAR(36) NOT NULL,
	connecting_piece_guid VARCHAR(36) NOT NULL,
	connecting_design_piece_guid VARCHAR(36),
	connecting_connector_guid VARCHAR(36) NOT NULL,
	gap FLOAT NOT NULL DEFAULT 0,
	shift FLOAT NOT NULL DEFAULT 0,
	rise FLOAT NOT NULL DEFAULT 0,
	rotation FLOAT NOT NULL DEFAULT 0,
	turn FLOAT NOT NULL DEFAULT 0,
	tilt FLOAT NOT NULL DEFAULT 0,
	u FLOAT,
	v FLOAT,
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	CHECK (connecting_piece_guid != connected_piece_guid),
	FOREIGN KEY(connected_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connected_connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(connecting_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connecting_connector_guid) REFERENCES connector (guid),
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
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type_concept (
	type_guid VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (type_guid, concept)
);

CREATE TABLE type_author (
	type_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (type_guid, author_guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
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
	port_guid VARCHAR(36),
	folder_guid VARCHAR(36),
	file_guid VARCHAR(36),
	author_guid VARCHAR(36),
	model_guid VARCHAR(36),
	prop_guid VARCHAR(36),
	connector_guid VARCHAR(36),
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
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(folder_guid) REFERENCES folder (guid),
	FOREIGN KEY(file_guid) REFERENCES file (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid),
	FOREIGN KEY(connector_guid) REFERENCES connector (guid),
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

const kitToSqlite = async (kit: Kit, db: any): Promise<void> => {
  db.exec(KIT_SQLITE_SCHEMA);

  const toISOString = (date: Date | string | undefined): string => {
    if (!date) return new Date().toISOString();
    if (typeof date === "string") return date;
    return date.toISOString();
  };

  db.run("INSERT INTO semio (release, engine, created) VALUES (?, ?, ?)", ["1.0.0", "js", new Date().toISOString()]);

  db.run("INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
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
  ]);

  toArray(kit.concepts).forEach((concept) => {
    if (typeof concept === "object") {
      db.run("INSERT INTO concept (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [concept.guid, concept.name, concept.description || null, concept.icon || null, kit.guid]);
    } else {
      db.run("INSERT INTO concept (guid, name, kit_guid) VALUES (?, ?, ?)", [guid(), concept, kit.guid]);
    }
  });

  toArray(kit.attributes).forEach((attr) => {
    db.run("INSERT INTO attribute (guid, key, value, definition, kit_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, kit.guid]);
  });

  toArray(kit.ports).forEach((iface) => {
    db.run("INSERT INTO port (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [iface.guid, iface.name, iface.description || null, iface.icon || null, kit.guid]);

    toArray(iface.compatiblePorts).forEach((compat) => {
      db.run("INSERT INTO port_compatibility (port_guid, compatible_port_guid) VALUES (?, ?)", [iface.guid, compat.guid]);
    });

    toArray(iface.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, port_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, iface.guid]);
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
        quality.isMinExcluded ? 1 : null,
        quality.max || null,
        quality.isMaxExcluded ? 1 : null,
        quality.canScale ? 1 : null,
        quality.uri || null,
        kit.guid,
      ],
    );

    toArray(quality.benchmarks).forEach((benchmark) => {
      db.run("INSERT INTO benchmark (guid, name, icon, min_value, min_excluded, max_value, max_excluded, quality_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
        benchmark.guid,
        benchmark.name,
        benchmark.icon || null,
        benchmark.min || null,
        benchmark.minExcluded ? 1 : null,
        benchmark.max || null,
        benchmark.maxExcluded ? 1 : null,
        quality.guid,
      ]);

      toArray(benchmark.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, benchmark_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, benchmark.guid]);
      });
    });

    toArray(quality.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, quality_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, quality.guid]);
    });
  });

  toArray(kit.folders).forEach((folder) => {
    db.run("INSERT INTO folder (guid, name, parent_guid, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?)", [folder.guid, folder.name, folder.parent?.guid || null, toISOString(folder.createdAt), toISOString(folder.updatedAt), kit.guid]);
  });

  toArray(kit.files).forEach((file) => {
    db.run("INSERT INTO file (guid, name, mime, folder_guid, size, hash, remote_url, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      file.guid,
      file.name,
      file.mime || null,
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
    db.run("INSERT INTO author (guid, name, email, kit_guid) VALUES (?, ?, ?, ?)", [author.guid, author.name, author.email || null, kit.guid]);
  });

  toArray(kit.tags).forEach((tag) => {
    db.run("INSERT INTO tag (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?)", [tag.guid, tag.name, tag.description || null, tag.icon || null, kit.guid]);
  });

  toArray(kit.types).forEach((type) => {
    db.run("INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
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
    ]);

    toArray(type.concepts).forEach((concept) => {
      db.run("INSERT INTO type_concept (type_guid, concept) VALUES (?, ?)", [type.guid, concept]);
    });

    toArray(type.authors).forEach((authorId, index) => {
      db.run("INSERT INTO type_author (type_guid, author_guid, rank) VALUES (?, ?, ?)", [type.guid, typeof authorId === "object" ? authorId.guid : authorId, index]);
    });

    toArray(type.models).forEach((model) => {
      db.run("INSERT INTO model (guid, file_guid, name, description, type_guid) VALUES (?, ?, ?, ?, ?)", [model.guid, model.file.guid, model.name || null, model.description || null, type.guid]);

      toArray(model.tags).forEach((tag) => {
        db.run("INSERT INTO model_tag (model_guid, tag_guid) VALUES (?, ?)", [model.guid, typeof tag === "object" ? tag.guid : tag]);
      });

      toArray(model.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, model_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, model.guid]);
      });
    });

    toArray(type.connectors).forEach((connector) => {
      db.run("INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
        connector.guid,
        connector.name || null,
        connector.point.x,
        connector.point.y,
        connector.point.z,
        connector.direction.x,
        connector.direction.y,
        connector.direction.z,
        connector.t,
        connector.mandatory ? 1 : 0,
        connector.port?.guid || null,
        connector.description || null,
        type.guid,
      ]);

      toArray(connector.props).forEach((prop) => {
        const quality = toArray(kit.qualities).find((q) => q.guid === prop.quality.guid);
        const propKey = quality?.key || "";
        db.run("INSERT INTO prop (guid, key, value, unit, quality_guid, connector_guid) VALUES (?, ?, ?, ?, ?, ?)", [prop.guid, propKey, prop.value, prop.unit || null, prop.quality.guid, connector.guid]);
        toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, prop.guid]);
        });
      });

      toArray(connector.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, connector_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, connector.guid]);
      });
    });

    toArray(type.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, type_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, type.guid]);
    });
  });

  toArray(kit.designs).forEach((design) => {
    db.run("INSERT INTO design (guid, name, parent_guid, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
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
    ]);

    toArray(design.concepts).forEach((concept) => {
      db.run("INSERT INTO design_concept (design_guid, concept) VALUES (?, ?)", [design.guid, concept]);
    });

    toArray(design.props).forEach((prop) => {
      db.run("INSERT INTO design_prop (guid, design_guid, quality_guid, value, unit) VALUES (?, ?, ?, ?, ?)", [prop.guid, design.guid, prop.quality.guid, parseFloat(prop.value), prop.unit || null]);
    });

    toArray(design.authors).forEach((authorId, index) => {
      db.run("INSERT INTO design_author (design_guid, author_guid, rank) VALUES (?, ?, ?)", [design.guid, typeof authorId === "object" ? authorId.guid : authorId, index]);
    });

    toArray(design.layers).forEach((layer) => {
      db.run("INSERT INTO layer (guid, path, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?)", [
        layer.guid,
        layer.path,
        layer.isHidden ? 1 : 0,
        layer.isLocked ? 1 : 0,
        layer.color || null,
        layer.description || null,
        design.guid,
      ]);

      toArray(layer.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, layer_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, layer.guid]);
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
        ],
      );

      toArray(piece.props).forEach((prop) => {
        const quality = toArray(kit.qualities).find((q) => q.guid === prop.quality.guid);
        const propKey = quality?.key || "";
        db.run("INSERT INTO prop (guid, key, value, unit, quality_guid) VALUES (?, ?, ?, ?, ?)", [prop.guid, propKey, prop.value, prop.unit || null, prop.quality.guid]);
        db.run("INSERT INTO piece_prop (piece_guid, prop_guid) VALUES (?, ?)", [piece.guid, prop.guid]);
        toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, prop.guid]);
        });
      });

      toArray(piece.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, piece_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, piece.guid]);
      });
    });

    toArray(design.groups).forEach((group) => {
      db.run('INSERT INTO "group" (guid, name, color, description, design_guid) VALUES (?, ?, ?, ?, ?)', [group.guid, group.name || null, group.color || null, group.description || null, design.guid]);

      toArray(group.pieces).forEach((piece) => {
        db.run("INSERT INTO group_piece (group_guid, piece_guid) VALUES (?, ?)", [group.guid, piece.guid]);
      });

      toArray(group.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, group_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, group.guid]);
      });
    });

    toArray(design.connections).forEach((connection) => {
      if (!connection.guid || !connection.connected?.piece?.guid || !connection.connecting?.piece?.guid || !connection.connected?.connector?.guid || !connection.connecting?.connector?.guid) {
        return;
      }

      db.run(
        "INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          connection.guid,
          connection.connected.piece.guid,
          connection.connected.designPiece?.guid || null,
          connection.connected.connector.guid,
          connection.connecting.piece.guid,
          connection.connecting.designPiece?.guid || null,
          connection.connecting.connector.guid,
          connection.gap || 0,
          connection.shift || 0,
          connection.rise || 0,
          connection.rotation || 0,
          connection.turn || 0,
          connection.tilt || 0,
          connection.u !== undefined ? connection.u : null,
          connection.v !== undefined ? connection.v : null,
          connection.description || null,
          design.guid,
        ],
      );

      toArray(connection.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, connection_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, connection.guid]);
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

    });

    toArray(design.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (guid, key, value, definition, design_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, design.guid]);
    });
  });
};

// #endregion 🔖Kit Import/Export

// #region 🔖Validation

// #region 🔖Validation core types

export type EntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Model" | "Layer" | "Group" | "Stat";

export interface DomainLocation {
  entityKind: EntityKind;
  entityGuid?: Guid;
  field?: string;
}

export interface Fix {
  title: string;
  diff: KitDiff;
}

export interface Problem {
  constraintId: string;
  message: string;
  location: DomainLocation;
  relatedGuids?: Guid[];
  fixes: Fix[];
}

export interface ValidationResult {
  problems: Problem[];
}

export const hasErrors = (res: ValidationResult) => res.problems.length > 0;

// #endregion 🔖Validation core types

// #region 🔖Validation context & engine

export interface ValidationContext {
  kit: Kit;
  typesByGuid: Map<Guid, Type>;
  designsByGuid: Map<Guid, Design>;
  piecesByGuid: Map<Guid, { designGuid: Guid; piece: Piece }>;
  connectorsByTypeGuid: Map<Guid, Connector[]>;
  modelsByTypeGuid: Map<Guid, Model[]>;
}

export const buildValidationContext = (kit: Kit): ValidationContext => {
  const typesByGuid = new Map<Guid, Type>();
  const designsByGuid = new Map<Guid, Design>();
  const piecesByGuid = new Map<Guid, { designGuid: Guid; piece: Piece }>();
  const connectorsByTypeGuid = new Map<Guid, Connector[]>();
  const modelsByTypeGuid = new Map<Guid, Model[]>();
  toArray(kit.types).forEach((t) => {
    typesByGuid.set(t.guid, t);
    connectorsByTypeGuid.set(t.guid, toArray(t.connectors));
    modelsByTypeGuid.set(t.guid, toArray(t.models));
  });
  toArray(kit.designs).forEach((d) => {
    designsByGuid.set(d.guid, d);
    toArray(d.pieces).forEach((p) => piecesByGuid.set(p.guid, { designGuid: d.guid, piece: p }));
  });
  return { kit, typesByGuid, designsByGuid, piecesByGuid, connectorsByTypeGuid, modelsByTypeGuid };
};

export type Constraint = (ctx: ValidationContext) => Problem[];

export interface ValidationConfig {
  constraints?: Constraint[];
}

export let defaultConstraints: Constraint[] = [];

export const validateKit = (kit: Kit, cfg: ValidationConfig = {}): ValidationResult => {
  const ctx = buildValidationContext(kit);
  const constraints = cfg.constraints ?? defaultConstraints;
  return { problems: constraints.flatMap((constraint) => constraint(ctx)) };
};

// #endregion 🔖Validation context & engine

// #region 🔖Fix helper

export const semioMakeFix = (ctx: ValidationContext, title: string, mutate: (clone: Kit) => void): Fix => {
  const clone = JSON.parse(serializeKit(ctx.kit)) as Kit;
  mutate(clone);
  const diff = getKitDiff(ctx.kit, clone);
  return { title, diff };
};

// #endregion 🔖Fix helper

// #region 🔖GUID update helper

const updateGuidEverywhere = (kit: Kit, oldGuid: Guid, newGuid: Guid): void => {
  const update = (obj: any) => {
    if (!obj || typeof obj !== "object") return;
    if (obj.guid === oldGuid) obj.guid = newGuid;
    if (obj.parent?.guid === oldGuid) obj.parent = createTypeId(newGuid);
    if (obj.type?.guid === oldGuid) obj.type = createTypeId(newGuid);
    if (obj.design?.guid === oldGuid) obj.design = createDesignId(newGuid);
    if (obj.port?.guid === oldGuid) obj.port = createPortId(newGuid);
    if (obj.quality?.guid === oldGuid) obj.quality = createQualityId(newGuid);
    if (obj.piece?.guid === oldGuid) obj.piece = createPieceId(newGuid);
    if (obj.designPiece?.guid === oldGuid) obj.designPiece = createPieceId(newGuid);
    if (obj.connector?.guid === oldGuid) obj.connector = createConnectorId(newGuid);
    if (Array.isArray(obj.compatiblePorts)) {
      obj.compatiblePorts = obj.compatiblePorts.map((iid: PortId) => (iid.guid === oldGuid ? createPortId(newGuid) : iid));
    }
    if (Array.isArray(obj.pieces)) {
      obj.pieces = obj.pieces.map((p: PieceId) => (p.guid === oldGuid ? createPieceId(newGuid) : p));
    }
    for (const key in obj) {
      if (Array.isArray(obj[key])) {
        obj[key].forEach(update);
      } else if (typeof obj[key] === "object") {
        update(obj[key]);
      }
    }
  };
  update(kit);
};

// #endregion 🔖GUID update helper

// #region 🔖Constraint: GUID uniqueness

export const semioGuidUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const seen = new Map<Guid, EntityKind>();
  const check = (entityKind: EntityKind, entityGuid: Guid) => {
    const existing = seen.get(entityGuid);
    if (!existing) {
      seen.set(entityGuid, entityKind);
      return;
    }
    const problem: Problem = {
      constraintId: "guid-unique",
      message: `Duplicate GUID "${entityGuid}". First occurrence kept.`,
      location: { entityKind, entityGuid, field: "guid" },
      relatedGuids: [entityGuid],
      fixes: [
        semioMakeFix(ctx, "Regenerate GUID", (clone) => {
          const newGuid = guid();
          updateGuidEverywhere(clone, entityGuid, newGuid);
        }),
      ],
    };
    problems.push(problem);
  };
  check("Kit", ctx.kit.guid);
  toArray(ctx.kit.types).forEach((t) => check("Type", t.guid));
  toArray(ctx.kit.designs).forEach((d) => {
    check("Design", d.guid);
    toArray(d.pieces).forEach((p) => check("Piece", p.guid));
    toArray(d.connections).forEach((c) => check("Connection", c.guid));
    toArray(d.stats).forEach((s) => check("Stat", s.guid));
  });
  toArray(ctx.kit.qualities).forEach((q) => check("Quality", q.guid));
  toArray(ctx.kit.ports).forEach((i) => check("Port", i.guid));
  toArray(ctx.kit.files).forEach((f) => check("File", f.guid));
  toArray(ctx.kit.folders).forEach((f) => check("Folder", f.guid));
  return problems;
};

// #endregion 🔖Constraint: GUID uniqueness

// #region 🔖Constraint: Type name uniqueness

export const semioTypeNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Guid | undefined, Type[]>();
  toArray(ctx.kit.types).forEach((t) => {
    const pid = t.parent?.guid as Guid | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(t);
  });
  for (const [parentGuid, siblings] of byParent) {
    const names = new Map<string, Type[]>();
    siblings.forEach((t) => {
      const name = t.name ?? "";
      if (!names.has(name)) names.set(name, []);
      names.get(name)!.push(t);
    });
    for (const [name, group] of names) {
      if (group.length <= 1) continue;
      const [first, ...rest] = group;
      const siblingNames = siblings.map((s) => s.name ?? "");
      rest.forEach((type) => {
        const fix = semioMakeFix(ctx, `Rename "${name}"`, (clone) => {
          const ct = toArray(clone.types).find((x) => x.guid === type.guid);
          if (!ct) return;
          const newName = generateUniqueName(name, siblingNames);
          ct.name = newName;
        });
        problems.push({
          constraintId: "type-name-unique",
          message: `Duplicate type name "${name}" among siblings.`,
          location: { entityKind: "Type", entityGuid: type.guid, field: "name" },
          relatedGuids: group.map((t) => t.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Type name uniqueness

// #region 🔖Constraint: Design name uniqueness

export const semioDesignNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Guid | undefined, Design[]>();
  toArray(ctx.kit.designs).forEach((d) => {
    const pid = d.parent?.guid as Guid | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(d);
  });
  for (const [parentGuid, siblings] of byParent) {
    const names = new Map<string, Design[]>();
    siblings.forEach((d) => {
      const name = d.name ?? "";
      if (!names.has(name)) names.set(name, []);
      names.get(name)!.push(d);
    });
    for (const [name, group] of names) {
      if (group.length <= 1) continue;
      const [first, ...rest] = group;
      const siblingNames = siblings.map((s) => s.name ?? "");
      rest.forEach((design) => {
        const fix = semioMakeFix(ctx, `Rename "${name}"`, (clone) => {
          const cd = toArray(clone.designs).find((x) => x.guid === design.guid);
          if (!cd) return;
          const newName = generateUniqueName(name, siblingNames);
          cd.name = newName;
        });
        problems.push({
          constraintId: "design-name-unique",
          message: `Duplicate design name "${name}" among siblings.`,
          location: { entityKind: "Design", entityGuid: design.guid, field: "name" },
          relatedGuids: group.map((d) => d.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Design name uniqueness

// #region 🔖Constraint: Piece name uniqueness

export const semioPieceNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const pieces = toArray(design.pieces);
    if (pieces.length === 0) return;
    const nameMap = new Map<string, Piece[]>();
    pieces.forEach((p) => {
      const n = p.name ?? "";
      if (!nameMap.has(n)) nameMap.set(n, []);
      nameMap.get(n)!.push(p);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = pieces.map((p) => p.name ?? "");
      rest.forEach((piece) => {
        const fix = semioMakeFix(ctx, `Rename piece "${name}"`, (clone) => {
          const cd = toArray(clone.designs).find((d) => d.guid === design.guid);
          if (!cd) return;
          const cpieces = toArray(cd.pieces);
          const cp = cpieces.find((p) => p.guid === piece.guid);
          if (!cp) return;
          cp.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "piece-name-unique",
          message: `Duplicate piece name "${name}" inside design "${design.name}".`,
          location: { entityKind: "Piece", entityGuid: piece.guid, field: "name" },
          relatedGuids: list.map((p) => p.guid),
          fixes: [fix],
        });
      });
    }
  });
  return problems;
};

// #endregion 🔖Constraint: Piece name uniqueness

// #region 🔖Constraint: Quality name uniqueness

export const semioQualityNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const qualities = toArray(ctx.kit.qualities);
  const nameMap = new Map<string, Quality[]>();
  qualities.forEach((q) => {
    const name = q.name ?? "";
    if (!nameMap.has(name)) nameMap.set(name, []);
    nameMap.get(name)!.push(q);
  });
  for (const [name, list] of nameMap) {
    if (list.length <= 1) continue;
    const [first, ...rest] = list;
    const allNames = qualities.map((q) => q.name ?? "");
    rest.forEach((quality) => {
      const fix = semioMakeFix(ctx, `Rename quality "${name}"`, (clone) => {
        const cq = toArray(clone.qualities).find((q) => q.guid === quality.guid);
        if (!cq) return;
        cq.name = generateUniqueName(name, allNames);
      });
      problems.push({
        constraintId: "quality-name-unique",
        message: `Duplicate quality name "${name}".`,
        location: { entityKind: "Quality", entityGuid: quality.guid, field: "name" },
        relatedGuids: list.map((q) => q.guid),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion 🔖Constraint: Quality name uniqueness

// #region 🔖Constraint: Port name uniqueness

export const semioPortNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const ports = toArray(ctx.kit.ports);
  const nameMap = new Map<string, Port[]>();
  ports.forEach((i) => {
    const name = i.name ?? "";
    if (!nameMap.has(name)) nameMap.set(name, []);
    nameMap.get(name)!.push(i);
  });
  for (const [name, list] of nameMap) {
    if (list.length <= 1) continue;
    const [first, ...rest] = list;
    const allNames = ports.map((i) => i.name ?? "");
    rest.forEach((iface) => {
      const fix = semioMakeFix(ctx, `Rename port "${name}"`, (clone) => {
        const ci = toArray(clone.ports).find((i) => i.guid === iface.guid);
        if (!ci) return;
        ci.name = generateUniqueName(name, allNames);
      });
      problems.push({
        constraintId: "port-name-unique",
        message: `Duplicate port name "${name}".`,
        location: { entityKind: "Port", entityGuid: iface.guid, field: "name" },
        relatedGuids: list.map((i) => i.guid),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion 🔖Constraint: Port name uniqueness

// #region 🔖Constraint: File name uniqueness

export const semioFileNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const files = toArray(ctx.kit.files);
  const nameMap = new Map<string, File[]>();
  files.forEach((f) => {
    const name = f.name ?? "";
    if (!nameMap.has(name)) nameMap.set(name, []);
    nameMap.get(name)!.push(f);
  });
  for (const [name, list] of nameMap) {
    if (list.length <= 1) continue;
    const [first, ...rest] = list;
    const allNames = files.map((f) => f.name ?? "");
    rest.forEach((file) => {
      const fix = semioMakeFix(ctx, `Rename file "${name}"`, (clone) => {
        const cf = toArray(clone.files).find((f) => f.guid === file.guid);
        if (!cf) return;
        cf.name = generateUniqueName(name, allNames);
      });
      problems.push({
        constraintId: "file-name-unique",
        message: `Duplicate file name "${name}".`,
        location: { entityKind: "File", entityGuid: file.guid, field: "name" },
        relatedGuids: list.map((f) => f.guid),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion 🔖Constraint: File name uniqueness

// #region 🔖Constraint: Folder name uniqueness

export const semioFolderNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Guid | undefined, Folder[]>();
  const folders = toArray(ctx.kit.folders);
  folders.forEach((f) => {
    const pid = f.parent?.guid as Guid | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(f);
  });
  for (const [parentGuid, siblings] of byParent) {
    const nameMap = new Map<string, Folder[]>();
    siblings.forEach((f) => {
      const name = f.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(f);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = siblings.map((f) => f.name ?? "");
      rest.forEach((folder) => {
        const fix = semioMakeFix(ctx, `Rename folder "${name}"`, (clone) => {
          const cf = toArray(clone.folders).find((f) => f.guid === folder.guid);
          if (!cf) return;
          cf.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "folder-name-unique",
          message: `Duplicate folder name "${name}" among siblings.`,
          location: { entityKind: "Folder", entityGuid: folder.guid, field: "name" },
          relatedGuids: list.map((f) => f.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Folder name uniqueness

// #region 🔖Constraint: Connector name uniqueness within type

export const semioConnectorNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  for (const [typeGuid, connectors] of ctx.connectorsByTypeGuid) {
    if (connectors.length === 0) continue;
    const nameMap = new Map<string, Connector[]>();
    connectors.forEach((p) => {
      const name = p.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(p);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = connectors.map((p) => p.name ?? "");
      const type = ctx.typesByGuid.get(typeGuid);
      rest.forEach((connector) => {
        const fix = semioMakeFix(ctx, `Rename connector "${name}"`, (clone) => {
          const ct = toArray(clone.types).find((t) => t.guid === typeGuid);
          if (!ct) return;
          const cconnectors = toArray(ct.connectors);
          const cp = cconnectors.find((p) => p.guid === connector.guid);
          if (!cp) return;
          cp.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "connector-name-unique",
          message: `Duplicate connector name "${name}" inside type "${type?.name}".`,
          location: { entityKind: "Connector", entityGuid: connector.guid, field: "name" },
          relatedGuids: list.map((p) => p.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Connector name uniqueness within type

// #region 🔖Constraint: Model name uniqueness within type

export const semioModelNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  for (const [typeGuid, models] of ctx.modelsByTypeGuid) {
    if (models.length === 0) continue;
    const nameMap = new Map<string, Model[]>();
    models.forEach((m) => {
      const name = m.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(m);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = models.map((m) => m.name ?? "");
      const type = ctx.typesByGuid.get(typeGuid);
      rest.forEach((model) => {
        const fix = semioMakeFix(ctx, `Rename model "${name}"`, (clone) => {
          const ct = toArray(clone.types).find((t) => t.guid === typeGuid);
          if (!ct) return;
          const cmodels = toArray(ct.models);
          const cm = cmodels.find((m) => m.guid === model.guid);
          if (!cm) return;
          cm.name = generateUniqueName(name, allNames);
        });
        problems.push({
          constraintId: "model-name-unique",
          message: `Duplicate model name "${name}" inside type "${type?.name}".`,
          location: { entityKind: "Model", entityGuid: model.guid, field: "name" },
          relatedGuids: list.map((m) => m.guid),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion 🔖Constraint: Model name uniqueness within type

// #region 🔖Constraint: Layer path uniqueness within design

export const semioLayerPathUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const layers = toArray(design.layers);
    if (layers.length === 0) return;
    const pathMap = new Map<string, Layer[]>();
    layers.forEach((l) => {
      const path = l.path ?? "";
      if (!pathMap.has(path)) pathMap.set(path, []);
      pathMap.get(path)!.push(l);
    });
    for (const [path, list] of pathMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allPaths = layers.map((l) => l.path ?? "");
      rest.forEach((layer) => {
        const fix = semioMakeFix(ctx, `Rename layer "${path}"`, (clone) => {
          const cd = toArray(clone.designs).find((d) => d.guid === design.guid);
          if (!cd) return;
          const clayers = toArray(cd.layers);
          const cl = clayers.find((l) => l.path === layer.path);
          if (!cl) return;
          cl.path = generateUniqueName(path, allPaths);
        });
        problems.push({
          constraintId: "layer-path-unique",
          message: `Duplicate layer path "${path}" inside design "${design.name}".`,
          location: { entityKind: "Layer", entityGuid: layer.guid, field: "path" },
          fixes: [fix],
        });
      });
    }
  });
  return problems;
};

// #endregion 🔖Constraint: Layer path uniqueness within design

// #region 🔖Constraint: Design piece same family constraint

export const semioDesignPieceSameFamilyConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const pieces = toArray(design.pieces);
    pieces.forEach((piece) => {
      if (!piece.design?.guid) return;
      try {
        const pieceDesign = ctx.designsByGuid.get(piece.design.guid);
        if (!pieceDesign) return;

        const containerPrimitive = getPrimitiveDesignFromContext(ctx, design.guid);
        const piecePrimitive = getPrimitiveDesignFromContext(ctx, piece.design.guid);

        if (containerPrimitive === piecePrimitive) {
          const fix = semioMakeFix(ctx, `Remove design piece "${piece.name || piece.guid}"`, (clone) => {
            const cd = toArray(clone.designs).find((d) => d.guid === design.guid);
            if (!cd) return;
            cd.pieces = toArray(cd.pieces).filter((p) => p.guid !== piece.guid);

            cd.connections = toArray(cd.connections).filter((c) => c.connected.piece.guid !== piece.guid && c.connecting.piece.guid !== piece.guid);
          });
          problems.push({
            constraintId: "design-piece-same-family",
            message: `Design piece "${piece.name || piece.guid}" references design "${pieceDesign.name}" which is in the same design family as container design "${design.name}". A design cannot contain design pieces from the same family.`,
            location: { entityKind: "Piece", entityGuid: piece.guid, field: "design" },
            relatedGuids: [piece.guid, design.guid, pieceDesign.guid],
            fixes: [fix],
          });
        }
      } catch {
      }
    });
  });
  return problems;
};

const getPrimitiveDesignFromContext = (ctx: ValidationContext, designGuid: string): string => {
  let currentGuid = designGuid;
  let interactions = 0;
  const maxIterations = 1000;
  while (interactions < maxIterations) {
    const design = ctx.designsByGuid.get(currentGuid);
    if (!design || !design.parent?.guid) return currentGuid;
    currentGuid = design.parent.guid;
    interactions++;
  }
  return currentGuid;
};

// #endregion 🔖Constraint: Design piece same family constraint

// #region 🔖Constraint registration

defaultConstraints = [
  semioGuidUniquenessConstraint,
  semioTypeNameUniquenessConstraint,
  semioDesignNameUniquenessConstraint,
  semioPieceNameUniquenessConstraint,
  semioQualityNameUniquenessConstraint,
  semioPortNameUniquenessConstraint,
  semioFileNameUniquenessConstraint,
  semioFolderNameUniquenessConstraint,
  semioConnectorNameUniquenessConstraint,
  semioModelNameUniquenessConstraint,
  semioLayerPathUniquenessConstraint,
  semioDesignPieceSameFamilyConstraint,
];

// #endregion 🔖Constraint registration

// #region 🔖Validation serialization

export interface SerializableValidationFix {
  title: string;
  diff: KitDiff;
}

export interface SerializableProblem {
  constraintId: string;
  message: string;
  entityKind: string;
  entityGuid: string;
  fixes: SerializableValidationFix[];
}

export interface SerializableValidationResult {
  problems: SerializableProblem[];
}

export const toValidationResult = (result: ValidationResult): SerializableValidationResult => ({
  problems: result.problems.map((problem) => ({
    constraintId: problem.constraintId,
    message: problem.message,
    entityKind: problem.location.entityKind,
    entityGuid: problem.location.entityGuid ?? "",
    fixes: problem.fixes.map((fix) => ({ title: fix.title, diff: fix.diff })),
  })),
});

export const serializeValidationResult = (result: ValidationResult): string => {
  const serializable = toValidationResult(result);
  serializable.problems.sort((a, b) => {
    const constraintCompare = a.constraintId.localeCompare(b.constraintId);
    if (constraintCompare !== 0) return constraintCompare;
    return a.entityGuid.localeCompare(b.entityGuid);
  });
  return JSON.stringify(serializable, null, 2);
};

export const parseValidationResult = (json: string): SerializableValidationResult => JSON.parse(json);

const isGuid = (s: string): boolean => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);

export const areKitDiffsEqualIgnoringNewGuids = (a: KitDiff, b: KitDiff): boolean => {
  const normalize = (obj: unknown): unknown => {
    if (obj === null || obj === undefined) return obj;
    if (typeof obj === "string" && isGuid(obj)) return "<GUID>";
    if (Array.isArray(obj)) return obj.map(normalize);
    if (typeof obj === "object") {
      const result: Record<string, unknown> = {};
      for (const [k, v] of Object.entries(obj)) result[k] = normalize(v);
      return result;
    }
    return obj;
  };
  return JSON.stringify(normalize(a)) === JSON.stringify(normalize(b));
};

export const areValidationResultsEqual = (a: ValidationResult, b: ValidationResult): boolean => {
  const serializableA = toValidationResult(a);
  const serializableB = toValidationResult(b);
  if (serializableA.problems.length !== serializableB.problems.length) return false;
  const sortProblems = (problems: SerializableProblem[]) =>
    [...problems].sort((x, y) => {
      const constraintCompare = x.constraintId.localeCompare(y.constraintId);
      if (constraintCompare !== 0) return constraintCompare;
      return x.entityGuid.localeCompare(y.entityGuid);
    });
  const sortedA = sortProblems(serializableA.problems);
  const sortedB = sortProblems(serializableB.problems);
  return sortedA.every((problemA, i) => {
    const problemB = sortedB[i];
    if (problemA.constraintId !== problemB.constraintId || problemA.message !== problemB.message || problemA.entityKind !== problemB.entityKind || problemA.entityGuid !== problemB.entityGuid) return false;
    if (problemA.fixes.length !== problemB.fixes.length) return false;
    return problemA.fixes.every((fixA, j) => {
      const fixB = problemB.fixes[j];
      return fixA.title === fixB.title && areKitDiffsEqualIgnoringNewGuids(fixA.diff, fixB.diff);
    });
  });
};

// #endregion 🔖Validation serialization

// #endregion 🔖Validation

// #endregion 🔖Kit
