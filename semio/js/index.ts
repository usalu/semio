// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core domain model types, schemas and utilities for the semio platform.

// #endregion 🧲Header

// #region ⛩️Imports
// External dependency imports MUST be declared here.
import { Accessor as GltfAccessor, Buffer as GltfBuffer, Document as GltfDocument, Material as GltfMaterial, Mesh as GltfMesh, Node as GltfNode, Texture as GltfTexture, NodeIO } from "@gltf-transform/core";
import { default as adjectives } from "@semio/assets/lists/adjectives.json" with { type: "json" };
import { default as animals } from "@semio/assets/lists/animals.json" with { type: "json" };
import { ClassValue, clsx } from "clsx";
import cytoscape from "cytoscape";
import { twMerge } from "tailwind-merge";
import * as THREE from "three";
import { v7 as uuidv7 } from "uuid";
import { z } from "zod";

// #endregion ⛩️Imports

// #region 🎞️Constants
// Global constants MUST define shared numeric parameters.

/** Standard icon width in pixels.
 **/
export const ICON_WIDTH = 50;
/**
 * Numeric tolerance for floating-point comparisons.
 **/
export const TOLERANCE = 1e-5;

// #endregion 🎞️Constants

// #region 📦Utilities
// General-purpose utility functions MUST be defined here.

/**
 **/
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 **/
export const guid = () => uuidv7();
// 🎲SeededRandom provides deterministic pseudo-random number generation.
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

/**
 * Class implementing Generator behavior.
 **/
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

/**
 **/
export const normalize = (val: string | undefined | null): string => (val === undefined || val === null ? "" : val);
/**
 **/
export const round = (value: number): number => Math.round(value / TOLERANCE) * TOLERANCE;
/**
 **/
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

/**
 **/
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

/**
 **/
export const arraysEqual = <T>(a: T[] | undefined, b: T[] | undefined): boolean => {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.length === b.length && a.every((val, index) => deepEqual(val, b[index]));
};

/**
 **/
export const generateUniqueName = (baseName: string, existingNames: string[], separator: string = " "): string => {
  if (!existingNames.includes(baseName)) return baseName;
  let counter = 2;
  while (existingNames.includes(`${baseName}${separator}${counter}`)) {
    counter++;
  }
  return `${baseName}${separator}${counter}`;
};

/**
 * Zod schema for DiffStatus validation.
 **/
export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

/**
 * Enumeration of DiffStatus values.
 **/
export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}

/**
 * Converts to ThreeRotation representation.
 **/
export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);

/**
 * Converts to SemioRotation representation.
 **/
export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
/**
 * Converts to ThreeQuaternion representation.
 **/
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476);
/**
 * Converts to SemioQuaternion representation.
 **/
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476);
/**
 **/
export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z);

/**
 * Type alias for Guid.
 **/
export type Guid = string;

// #endregion 📦Utilities

// #region 🐍Entity IDs
// Entity identifier types and comparison functions MUST be defined here.

/**
 * Identifier type for Attribute entities.
 **/
export type AttributeId = { guid: Guid };
/**
 * Identifier type for Location entities.
 **/
export type LocationId = { guid: Guid };
/**
 * Identifier type for Author entities.
 **/
export type AuthorId = { guid: Guid };
/**
 * Identifier type for File entities.
 **/
export type FileId = { guid: Guid };
/**
 * Identifier type for Folder entities.
 **/
export type FolderId = { guid: Guid };
/**
 * Identifier type for Benchmark entities.
 **/
export type BenchmarkId = { guid: Guid };
/**
 * Identifier type for Quality entities.
 **/
export type QualityId = { guid: Guid };
/**
 * Identifier type for Port entities.
 **/
export type PortId = { guid: Guid };
/**
 * Identifier type for Prop entities.
 **/
export type PropId = { guid: Guid };
/**
 * Identifier type for Model entities.
 **/
export type ModelId = { guid: Guid };
/**
 * Identifier type for Connector entities.
 **/
export type ConnectorId = { guid: Guid };
/**
 * Identifier type for Type entities.
 **/
export type TypeId = { guid: Guid };
/**
 * Identifier type for Layer entities.
 **/
export type LayerId = { guid: Guid };
/**
 * Identifier type for Piece entities.
 **/
export type PieceId = { guid: Guid };
/**
 * Identifier type for Group entities.
 **/
export type GroupId = { guid: Guid };
/**
 * Identifier type for Connection entities.
 **/
export type ConnectionId = { guid: Guid };
/**
 * Identifier type for Stat entities.
 **/
export type StatId = { guid: Guid };
/**
 * Identifier type for Design entities.
 **/
export type DesignId = { guid: Guid };
/**
 * Identifier type for Kit entities.
 **/
export type KitId = { guid: Guid };
/**
 * Identifier type for Tag entities.
 **/
export type TagId = { guid: Guid };
/**
 * Identifier type for Concept entities.
 **/
export type ConceptId = { guid: Guid };

/**
 * Zod schema for validating Attribute identifiers.
 **/
export const AttributeIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Location identifiers.
 **/
export const LocationIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Author identifiers.
 **/
export const AuthorIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating File identifiers.
 **/
export const FileIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Folder identifiers.
 **/
export const FolderIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Benchmark identifiers.
 **/
export const BenchmarkIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Quality identifiers.
 **/
export const QualityIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Port identifiers.
 **/
export const PortIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Prop identifiers.
 **/
export const PropIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Model identifiers.
 **/
export const ModelIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Connector identifiers.
 **/
export const ConnectorIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Type identifiers.
 **/
export const TypeIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Layer identifiers.
 **/
export const LayerIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Piece identifiers.
 **/
export const PieceIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Group identifiers.
 **/
export const GroupIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Connection identifiers.
 **/
export const ConnectionIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Stat identifiers.
 **/
export const StatIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Design identifiers.
 **/
export const DesignIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Kit identifiers.
 **/
export const KitIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Tag identifiers.
 **/
export const TagIdSchema = z.object({ guid: z.string() });
/**
 * Zod schema for validating Concept identifiers.
 **/
export const ConceptIdSchema = z.object({ guid: z.string() });

/**
 * Factory for creating Attribute identifiers.
 **/
export const createAttributeId = (guid: Guid): AttributeId => ({ guid });
/**
 * Factory for creating Location identifiers.
 **/
export const createLocationId = (guid: Guid): LocationId => ({ guid });
/**
 * Factory for creating Author identifiers.
 **/
export const createAuthorId = (guid: Guid): AuthorId => ({ guid });
/**
 * Factory for creating File identifiers.
 **/
export const createFileId = (guid: Guid): FileId => ({ guid });
/**
 * Factory for creating Folder identifiers.
 **/
export const createFolderId = (guid: Guid): FolderId => ({ guid });
/**
 * Factory for creating Benchmark identifiers.
 **/
export const createBenchmarkId = (guid: Guid): BenchmarkId => ({ guid });
/**
 * Factory for creating Quality identifiers.
 **/
export const createQualityId = (guid: Guid): QualityId => ({ guid });
/**
 * Factory for creating Port identifiers.
 **/
export const createPortId = (guid: Guid): PortId => ({ guid });
/**
 * Factory for creating Prop identifiers.
 **/
export const createPropId = (guid: Guid): PropId => ({ guid });
/**
 * Factory for creating Model identifiers.
 **/
export const createModelId = (guid: Guid): ModelId => ({ guid });
/**
 * Factory for creating Connector identifiers.
 **/
export const createConnectorId = (guid: Guid): ConnectorId => ({ guid });
/**
 * Factory for creating Type identifiers.
 **/
export const createTypeId = (guid: Guid): TypeId => ({ guid });
/**
 * Factory for creating Layer identifiers.
 **/
export const createLayerId = (guid: Guid): LayerId => ({ guid });
/**
 * Factory for creating Piece identifiers.
 **/
export const createPieceId = (guid: Guid): PieceId => ({ guid });
/**
 * Factory for creating Group identifiers.
 **/
export const createGroupId = (guid: Guid): GroupId => ({ guid });
/**
 * Factory for creating Connection identifiers.
 **/
export const createConnectionId = (guid: Guid): ConnectionId => ({ guid });
/**
 * Factory for creating Stat identifiers.
 **/
export const createStatId = (guid: Guid): StatId => ({ guid });
/**
 * Factory for creating Design identifiers.
 **/
export const createDesignId = (guid: Guid): DesignId => ({ guid });
/**
 * Factory for creating Kit identifiers.
 **/
export const createKitId = (guid: Guid): KitId => ({ guid });
/**
 * Factory for creating Tag identifiers.
 **/
export const createTagId = (guid: Guid): TagId => ({ guid });
/**
 * Factory for creating Concept identifiers.
 **/
export const createConceptId = (guid: Guid): ConceptId => ({ guid });

/**
 * Equality check for Attribute identifiers.
 **/
export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.guid === b.guid;
/**
 * Equality check for Location identifiers.
 **/
export const areSameLocationId = (a: LocationId, b: LocationId): boolean => a.guid === b.guid;
/**
 * Equality check for Author identifiers.
 **/
export const areSameAuthorId = (a: AuthorId, b: AuthorId): boolean => a.guid === b.guid;
/**
 * Equality check for File identifiers.
 **/
export const areSameFileId = (a: FileId, b: FileId): boolean => a.guid === b.guid;
/**
 * Equality check for Folder identifiers.
 **/
export const areSameFolderId = (a: FolderId, b: FolderId): boolean => a.guid === b.guid;
/**
 * Equality check for Benchmark identifiers.
 **/
export const areSameBenchmarkId = (a: BenchmarkId, b: BenchmarkId): boolean => a.guid === b.guid;
/**
 * Equality check for Quality identifiers.
 **/
export const areSameQualityId = (a: QualityId, b: QualityId): boolean => a.guid === b.guid;
/**
 * Equality check for Port identifiers.
 **/
export const areSamePortId = (a: PortId, b: PortId): boolean => a.guid === b.guid;
/**
 * Equality check for Prop identifiers.
 **/
export const areSamePropId = (a: PropId, b: PropId): boolean => a.guid === b.guid;
/**
 * Equality check for Model identifiers.
 **/
export const areSameModelId = (a: ModelId, b: ModelId): boolean => a.guid === b.guid;
/**
 * Equality check for Connector identifiers.
 **/
export const areSameConnectorId = (a: ConnectorId, b: ConnectorId): boolean => a.guid === b.guid;
/**
 * Equality check for Type identifiers.
 **/
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.guid === b.guid;
/**
 * Equality check for Layer identifiers.
 **/
export const areSameLayerId = (a: LayerId, b: LayerId): boolean => a.guid === b.guid;
/**
 * Equality check for Piece identifiers.
 **/
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
/**
 * Equality check for Group identifiers.
 **/
export const areSameGroupId = (a: GroupId, b: GroupId): boolean => a.guid === b.guid;
/**
 * Equality check for Connection identifiers.
 **/
export const areSameConnectionId = (a: ConnectionId, b: ConnectionId): boolean => a.guid === b.guid;
/**
 * Equality check for Stat identifiers.
 **/
export const areSameStatId = (a: StatId, b: StatId): boolean => a.guid === b.guid;
/**
 * Equality check for Design identifiers.
 **/
export const areSameDesignId = (a: DesignId, b: DesignId): boolean => a.guid === b.guid;
/**
 * Equality check for Kit identifiers.
 **/
export const areSameKitId = (a: KitId, b: KitId): boolean => a.guid === b.guid;
/**
 * Equality check for Tag identifiers.
 **/
export const areSameTagId = (a: TagId, b: TagId): boolean => a.guid === b.guid;
/**
 * Equality check for Concept identifiers.
 **/
export const areSameConceptId = (a: ConceptId, b: ConceptId): boolean => a.guid === b.guid;

/**
 * Extracts the GUID from a Attribute identifier.
 **/
export const getAttributeGuid = (id: AttributeId): Guid => id.guid;
/**
 * Extracts the GUID from a Location identifier.
 **/
export const getLocationGuid = (id: LocationId): Guid => id.guid;
/**
 * Extracts the GUID from a Author identifier.
 **/
export const getAuthorGuid = (id: AuthorId): Guid => id.guid;
/**
 * Extracts the GUID from a File identifier.
 **/
export const getFileGuid = (id: FileId): Guid => id.guid;
/**
 * Extracts the GUID from a Folder identifier.
 **/
export const getFolderGuid = (id: FolderId): Guid => id.guid;
/**
 * Extracts the GUID from a Benchmark identifier.
 **/
export const getBenchmarkGuid = (id: BenchmarkId): Guid => id.guid;
/**
 * Extracts the GUID from a Quality identifier.
 **/
export const getQualityGuid = (id: QualityId): Guid => id.guid;
/**
 * Extracts the GUID from a Port identifier.
 **/
export const getPortGuid = (id: PortId): Guid => id.guid;
/**
 * Extracts the GUID from a Prop identifier.
 **/
export const getPropGuid = (id: PropId): Guid => id.guid;
/**
 * Extracts the GUID from a Model identifier.
 **/
export const getModelGuid = (id: ModelId): Guid => id.guid;
/**
 * Extracts the GUID from a Connector identifier.
 **/
export const getConnectorGuid = (id: ConnectorId): Guid => id.guid;
/**
 * Extracts the GUID from a Type identifier.
 **/
export const getTypeGuid = (id: TypeId): Guid => id.guid;
/**
 * Extracts the GUID from a Layer identifier.
 **/
export const getLayerGuid = (id: LayerId): Guid => id.guid;
/**
 * Extracts the GUID from a Piece identifier.
 **/
export const getPieceGuid = (id: PieceId): Guid => id.guid;
/**
 * Extracts the GUID from a Group identifier.
 **/
export const getGroupGuid = (id: GroupId): Guid => id.guid;
/**
 * Extracts the GUID from a Connection identifier.
 **/
export const getConnectionGuid = (id: ConnectionId): Guid => id.guid;
/**
 * Extracts the GUID from a Stat identifier.
 **/
export const getStatGuid = (id: StatId): Guid => id.guid;
/**
 * Extracts the GUID from a Design identifier.
 **/
export const getDesignGuid = (id: DesignId): Guid => id.guid;
/**
 * Extracts the GUID from a Kit identifier.
 **/
export const getKitGuid = (id: KitId): Guid => id.guid;
/**
 * Extracts the GUID from a Tag identifier.
 **/
export const getTagGuid = (id: TagId): Guid => id.guid;
/**
 * Extracts the GUID from a Concept identifier.
 **/
export const getConceptGuid = (id: ConceptId): Guid => id.guid;

// #endregion 🐍Entity IDs

// #region 🖥️Weak Entities

// #region 📺Coord
// Coord weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Coord validation.
 **/
export const CoordSchema = z.object({ u: z.number(), v: z.number() });
/**
 * Type alias for Coord.
 **/
export type Coord = z.infer<typeof CoordSchema>;
/**
 * Serializes Coord for transport.
 **/
export const serializeCoord = (coord: Coord): string => JSON.stringify(CoordSchema.parse(coord));
/**
 **/
export const deserializeCoord = (json: string): Coord => CoordSchema.parse(JSON.parse(json));

/**
 * Zod schema for Coord diff validation.
 **/
export const CoordDiffSchema = CoordSchema.partial();
/**
 * Diff type for tracking Coord changes.
 **/
export type CoordDiff = z.infer<typeof CoordDiffSchema>;
/**
 * Retrieves the CoordDiff value.
 **/
export const getCoordDiff = (before: Coord, after: Coord): CoordDiff => {
  return {
    u: after.u - before.u,
    v: after.v - before.v,
  };
};
/**
 * Diff type for tracking inverseCoord changes.
 **/
export const inverseCoordDiff = (original: Coord, appliedDiff: CoordDiff): CoordDiff => {
  const u = appliedDiff.u ?? 0;
  const v = appliedDiff.v ?? 0;
  return {
    u: original.u - u,
    v: original.v - v,
  };
};
/**
 * Diff type for tracking mergeCoord changes.
 **/
export const mergeCoordDiff = (diff1: CoordDiff, diff2: CoordDiff): CoordDiff => {
  return {
    u: (diff1.u ?? 0) + (diff2.u ?? 0),
    v: (diff1.v ?? 0) + (diff2.v ?? 0),
  };
};
/**
 * Diff type for tracking applyCoord changes.
 **/
export const applyCoordDiff = (base: Coord, diff: CoordDiff): Coord => {
  const u = diff.u ?? 0;
  const v = diff.v ?? 0;
  return {
    u: base.u + u,
    v: base.v + v,
  };
};

// #endregion 📺Coord

// #region ➡️Vec
// Vec weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Vec validation.
 **/
export const VecSchema = z.object({ u: z.number(), v: z.number() });
/**
 * Type alias for Vec.
 **/
export type Vec = z.infer<typeof VecSchema>;
/**
 * Serializes Vec for transport.
 **/
export const serializeVec = (vec: Vec): string => JSON.stringify(VecSchema.parse(vec));
/**
 **/
export const deserializeVec = (json: string): Vec => VecSchema.parse(JSON.parse(json));

/**
 * Zod schema for Vec diff validation.
 **/
export const VecDiffSchema = VecSchema.partial();
/**
 * Diff type for tracking Vec changes.
 **/
export type VecDiff = z.infer<typeof VecDiffSchema>;
/**
 * Retrieves the VecDiff value.
 **/
export const getVecDiff = (before: Vec, after: Vec): VecDiff => {
  return {
    u: after.u - before.u,
    v: after.v - before.v,
  };
};
/**
 * Diff type for tracking inverseVec changes.
 **/
export const inverseVecDiff = (original: Vec, appliedDiff: VecDiff): VecDiff => {
  const u = appliedDiff.u ?? 0;
  const v = appliedDiff.v ?? 0;
  return {
    u: original.u - u,
    v: original.v - v,
  };
};
/**
 * Diff type for tracking mergeVec changes.
 **/
export const mergeVecDiff = (diff1: VecDiff, diff2: VecDiff): VecDiff => {
  return {
    u: (diff1.u ?? 0) + (diff2.u ?? 0),
    v: (diff1.v ?? 0) + (diff2.v ?? 0),
  };
};
/**
 * Diff type for tracking applyVec changes.
 **/
export const applyVecDiff = (base: Vec, diff: VecDiff): Vec => {
  const u = diff.u ?? 0;
  const v = diff.v ?? 0;
  return {
    u: base.u + u,
    v: base.v + v,
  };
};

// #endregion ➡️Vec

// #region ✖️Point
// Point weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Point validation.
 **/
export const PointSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
/**
 * Type alias for Point.
 **/
export type Point = z.infer<typeof PointSchema>;
/**
 * Serializes Point for transport.
 **/
export const serializePoint = (point: Point): string => JSON.stringify(PointSchema.parse(point));
/**
 **/
export const deserializePoint = (json: string): Point => PointSchema.parse(JSON.parse(json));

/**
 * Zod schema for Point diff validation.
 **/
export const PointDiffSchema = PointSchema.partial();
/**
 * Diff type for tracking Point changes.
 **/
export type PointDiff = z.infer<typeof PointDiffSchema>;
/**
 * Retrieves the PointDiff value.
 **/
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};
/**
 * Diff type for tracking inversePoint changes.
 **/
export const inversePointDiff = (original: Point, appliedDiff: PointDiff): PointDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  const z = appliedDiff.z ?? 0;
  return {
    x: -x,
    y: -y,
    z: -z,
  };
};
/**
 * Diff type for tracking mergePoint changes.
 **/
export const mergePointDiff = (diff1: PointDiff, diff2: PointDiff): PointDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
    z: (diff1.z ?? 0) + (diff2.z ?? 0),
  };
};
/**
 * Diff type for tracking applyPoint changes.
 **/
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

// #endregion ✖️Point

// #region ↗️Vector
// Vector weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Vector validation.
 **/
export const VectorSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
/**
 * Type alias for Vector.
 **/
export type Vector = z.infer<typeof VectorSchema>;
/**
 * Serializes Vector for transport.
 **/
export const serializeVector = (vector: Vector): string => JSON.stringify(VectorSchema.parse(vector));
/**
 **/
export const deserializeVector = (json: string): Vector => VectorSchema.parse(JSON.parse(json));

/**
 * Zod schema for Vector diff validation.
 **/
export const VectorDiffSchema = VectorSchema.partial();
/**
 * Diff type for tracking Vector changes.
 **/
export type VectorDiff = z.infer<typeof VectorDiffSchema>;
/**
 * Retrieves the VectorDiff value.
 **/
export const getVectorDiff = (before: Vector, after: Vector): VectorDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};
/**
 * Diff type for tracking inverseVector changes.
 **/
export const inverseVectorDiff = (original: Vector, appliedDiff: VectorDiff): VectorDiff => {
  const x = appliedDiff.x ?? 0;
  const y = appliedDiff.y ?? 0;
  const z = appliedDiff.z ?? 0;
  return {
    x: -x,
    y: -y,
    z: -z,
  };
};
/**
 * Diff type for tracking mergeVector changes.
 **/
export const mergeVectorDiff = (diff1: VectorDiff, diff2: VectorDiff): VectorDiff => {
  return {
    x: (diff1.x ?? 0) + (diff2.x ?? 0),
    y: (diff1.y ?? 0) + (diff2.y ?? 0),
    z: (diff1.z ?? 0) + (diff2.z ?? 0),
  };
};
/**
 * Diff type for tracking applyVector changes.
 **/
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

// #endregion ↗️Vector

// #region ◻️Plane
// Plane weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Plane validation.
 **/
export const PlaneSchema = z.object({
  origin: PointSchema,
  xAxis: VectorSchema,
  yAxis: VectorSchema,
});
/**
 * Type alias for Plane.
 **/
export type Plane = z.infer<typeof PlaneSchema>;
/**
 * Serializes Plane for transport.
 **/
export const serializePlane = (plane: Plane): string => JSON.stringify(PlaneSchema.parse(plane));
/**
 **/
export const deserializePlane = (json: string): Plane => PlaneSchema.parse(JSON.parse(json));
/**
 **/
export const planeToMatrix = (plane: Plane): THREE.Matrix4 => {
  const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
  const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z);
  const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
  const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis).normalize();
  const orthoYAxis = new THREE.Vector3().crossVectors(zAxis, xAxis).normalize();
  const matrix = new THREE.Matrix4().makeBasis(xAxis.normalize(), orthoYAxis, zAxis).setPosition(origin);
  return matrix;
};
/**
 **/
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
 **/
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
// ◻️roundPlane rounds plane components to a specified number of decimal places.
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

/**
 * Zod schema for Plane diff validation.
 **/
export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true })
  .extend({
    origin: PointDiffSchema,
    xAxis: VectorDiffSchema,
    yAxis: VectorDiffSchema,
  })
  .partial();
/**
 * Diff type for tracking Plane changes.
 **/
export type PlaneDiff = z.infer<typeof PlaneDiffSchema>;
/**
 * Retrieves the PlaneDiff value.
 **/
export const getPlaneDiff = (before: Plane, after: Plane): PlaneDiff => {
  return {
    origin: getPointDiff(before.origin, after.origin),
    xAxis: getVectorDiff(before.xAxis, after.xAxis),
    yAxis: getVectorDiff(before.yAxis, after.yAxis),
  };
};
/**
 * Diff type for tracking inversePlane changes.
 **/
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
/**
 * Diff type for tracking mergePlane changes.
 **/
export const mergePlaneDiff = (diff1: PlaneDiff, diff2: PlaneDiff): PlaneDiff => {
  return {
    origin: diff1.origin ?? diff2.origin ?? mergePointDiff(diff1.origin!, diff2.origin!),
    xAxis: diff1.xAxis ?? diff2.xAxis ?? mergeVectorDiff(diff1.xAxis!, diff2.xAxis!),
    yAxis: diff1.yAxis ?? diff2.yAxis ?? mergeVectorDiff(diff1.yAxis!, diff2.yAxis!),
  };
};
/**
 * Diff type for tracking applyPlane changes.
 **/
export const applyPlaneDiff = (base: Plane, diff: PlaneDiff): Plane => {
  return {
    origin: diff.origin ? applyPointDiff(base.origin, diff.origin) : base.origin,
    xAxis: diff.xAxis ? applyVectorDiff(base.xAxis, diff.xAxis) : base.xAxis,
    yAxis: diff.yAxis ? applyVectorDiff(base.yAxis, diff.yAxis) : base.yAxis,
  };
};

// #endregion ◻️Plane

// #region 🎥Camera

// Camera weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Camera validation.
 **/
export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});
/**
 * Type alias for Camera.
 **/
export type Camera = z.infer<typeof CameraSchema>;
/**
 * Serializes Camera for transport.
 **/
export const serializeCamera = (camera: Camera): string => JSON.stringify(CameraSchema.parse(camera));
/**
 **/
export const deserializeCamera = (json: string): Camera => CameraSchema.parse(JSON.parse(json));

/**
 * Zod schema for Camera diff validation.
 **/
export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true })
  .extend({
    position: PointDiffSchema,
    forward: VectorDiffSchema,
    up: VectorDiffSchema,
  })
  .partial();
/**
 * Diff type for tracking Camera changes.
 **/
export type CameraDiff = z.infer<typeof CameraDiffSchema>;
/**
 * Retrieves the CameraDiff value.
 **/
export const getCameraDiff = (before: Camera, after: Camera): CameraDiff => {
  return {
    position: getPointDiff(before.position, after.position),
    forward: getVectorDiff(before.forward, after.forward),
    up: getVectorDiff(before.up, after.up),
  };
};
/**
 * Diff type for tracking inverseCamera changes.
 **/
export const inverseCameraDiff = (original: Camera, appliedDiff: CameraDiff): CameraDiff => {
  return {
    position: appliedDiff.position ? inversePointDiff(original.position, appliedDiff.position) : original.position,
    forward: appliedDiff.forward ? inverseVectorDiff(original.forward, appliedDiff.forward) : original.forward,
    up: appliedDiff.up ? inverseVectorDiff(original.up, appliedDiff.up) : original.up,
  };
};
/**
 * Diff type for tracking mergeCamera changes.
 **/
export const mergeCameraDiff = (diff1: CameraDiff, diff2: CameraDiff): CameraDiff => {
  return {
    position: diff1.position ?? diff2.position ?? mergePointDiff(diff1.position!, diff2.position!),
    forward: diff1.forward ?? diff2.forward ?? mergeVectorDiff(diff1.forward!, diff2.forward!),
    up: diff1.up ?? diff2.up ?? mergeVectorDiff(diff1.up!, diff2.up!),
  };
};
/**
 * Diff type for tracking applyCamera changes.
 *
 **/
export const applyCameraDiff = (base: Camera, diff: CameraDiff): Camera => {
  return {
    position: diff.position ? applyPointDiff(base.position, diff.position) : base.position,
    forward: diff.forward ? applyVectorDiff(base.forward, diff.forward) : base.forward,
    up: diff.up ? applyVectorDiff(base.up, diff.up) : base.up,
  };
};

// #endregion 🎥Camera

// #endregion 🖥️Weak Entities

// #region 💎Attribute
// Attribute entity types, schemas, and helper functions MUST be defined here.
// 📅DateProperty represents a date-time value as ISO string.
const DateProperty = () => z.string().optional();

/**
 * Zod schema for Attribute validation.
 **/
export const AttributeSchema = z.object({
  guid: z.string(),
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
/**
 * Type alias for Attribute.
 **/
export type Attribute = z.infer<typeof AttributeSchema>;
/**
 * Serializes Attribute for transport.
 **/
export const serializeAttribute = (attribute: Attribute): string => JSON.stringify(AttributeSchema.parse(attribute));
/**
 **/
export const deserializeAttribute = (json: string): Attribute => AttributeSchema.parse(JSON.parse(json));

/**
 * Definition of AttributeMetaSchema.
 **/
export const AttributeMetaSchema = AttributeSchema;
/**
 * Type alias for AttributeMeta.
 **/
export type AttributeMeta = z.infer<typeof AttributeMetaSchema>;
/**
 * Serializes AttributeMeta for transport.
 **/
export const serializeAttributeMeta = (attribute: AttributeMeta): string => JSON.stringify(AttributeMetaSchema.parse(attribute));
/**
 **/
export const deserializeAttributeMeta = (json: string): AttributeMeta => AttributeMetaSchema.parse(JSON.parse(json));
/**
 * Definition of AttributeShallowSchema.
 **/
export const AttributeShallowSchema = AttributeSchema;
/**
 * Type alias for AttributeShallow.
 **/
export type AttributeShallow = z.infer<typeof AttributeShallowSchema>;
/**
 * Serializes AttributeShallow for transport.
 **/
export const serializeAttributeShallow = (attribute: AttributeShallow): string => JSON.stringify(AttributeShallowSchema.parse(attribute));
/**
 **/
export const deserializeAttributeShallow = (json: string): AttributeShallow => AttributeShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Attribute diff validation.
 **/
export const AttributeDiffSchema = AttributeSchema.partial();
/**
 * Diff type for tracking Attribute changes.
 **/
export type AttributeDiff = z.infer<typeof AttributeDiffSchema>;
/**
 * Retrieves the AttributeDiff value.
 **/
export const getAttributeDiff = (before: Attribute, after: Attribute): AttributeDiff => {
  const diff: AttributeDiff = {};
  if (before.key !== after.key) diff.key = after.key;
  if (before.value !== after.value) diff.value = after.value;
  if (before.definition !== after.definition) diff.definition = after.definition;
  return diff;
};
/**
 * Diff type for tracking inverseAttribute changes.
 **/
export const inverseAttributeDiff = (original: Attribute, appliedDiff: AttributeDiff): AttributeDiff => {
  return {
    key: appliedDiff.key ? original.key : "",
    value: appliedDiff.value ? original.value : "",
    definition: appliedDiff.definition ? original.definition : "",
  };
};
/**
 * Diff type for tracking mergeAttribute changes.
 **/
export const mergeAttributeDiff = (diff1: AttributeDiff, diff2: AttributeDiff): AttributeDiff => {
  return {
    key: diff2.key ?? diff1.key,
    value: diff2.value ?? diff1.value,
    definition: diff2.definition ?? diff1.definition,
  };
};
/**
 * Diff type for tracking applyAttribute changes.
 **/
export const applyAttributeDiff = (base: Attribute, diff: AttributeDiff): Attribute => {
  return { ...base, ...diff };
};

/**
 * Zod schema for Attributes diff validation.
 **/
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(AttributeSchema).optional(),
});
/**
 * Diff type for tracking Attributes changes.
 **/
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;

// 💎getAttributesDiff computes the diff between two attribute collections.
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

/**
 * Diff type for tracking inverseAttributes changes.
 **/
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

/**
 * Diff type for tracking mergeAttributes changes.
 **/
export const mergeAttributesDiff = (first: AttributesDiff, second: AttributesDiff): AttributesDiff => {
  return { ...first, ...second };
};

/**
 * Diff type for tracking applyAttributes changes.
 **/
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

// #endregion 💎Attribute

// #region 📍Location
// Location entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Location validation.
 **/
export const LocationSchema = z.object({
  guid: z.string(),
  longitude: z.number(),
  latitude: z.number(),
  altitude: z.number().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Location.
 **/
export type Location = z.infer<typeof LocationSchema>;
/**
 * Serializes Location for transport.
 **/
export const serializeLocation = (location: Location): string => JSON.stringify(LocationSchema.parse(location));
/**
 **/
export const deserializeLocation = (json: string): Location => LocationSchema.parse(JSON.parse(json));

/**
 * Zod schema for Location diff validation.
 **/
export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Location changes.
 **/
export type LocationDiff = z.infer<typeof LocationDiffSchema>;
/**
 * Retrieves the LocationDiff value.
 **/
export const getLocationDiff = (before: Location, after: Location): LocationDiff => {
  const diff: LocationDiff = {};
  if (before.longitude !== after.longitude) diff.longitude = after.longitude - before.longitude;
  if (before.latitude !== after.latitude) diff.latitude = after.latitude - before.latitude;
  if (before.altitude !== after.altitude) diff.altitude = after.altitude !== undefined && before.altitude !== undefined ? after.altitude - before.altitude : after.altitude;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseLocation changes.
 **/
export const inverseLocationDiff = (original: Location, appliedDiff: LocationDiff): LocationDiff => {
  const inverse: LocationDiff = {};
  if (appliedDiff.longitude !== undefined) inverse.longitude = original.longitude;
  if (appliedDiff.latitude !== undefined) inverse.latitude = original.latitude;
  if (appliedDiff.altitude !== undefined) inverse.altitude = original.altitude;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeLocation changes.
 **/
export const mergeLocationDiff = (diff1: LocationDiff, diff2: LocationDiff): LocationDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyLocation changes.
 **/
export const applyLocationDiff = (base: Location, diff: LocationDiff): Location => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Location = {
    guid: base.guid,
    longitude: diff.longitude ?? base.longitude,
    latitude: diff.latitude ?? base.latitude,
    altitude: diff.altitude ?? base.altitude,
  };

  return result;
};

// #endregion 📍Location

// #region ✍️Author
// Author entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Author validation.
 **/
export const AuthorSchema = z.object({ guid: z.string(), name: z.string(), email: z.string(), attributes: z.array(AttributeSchema).optional() });
/**
 * Type alias for Author.
 **/
export type Author = z.infer<typeof AuthorSchema>;
/**
 * Serializes Author for transport.
 **/
export const serializeAuthor = (author: Author): string => JSON.stringify(AuthorSchema.parse(author));
/**
 **/
export const deserializeAuthor = (json: string): Author => AuthorSchema.parse(JSON.parse(json));

/**
 * Definition of AuthorMetaSchema.
 **/
export const AuthorMetaSchema = AuthorSchema.omit({ attributes: true });
/**
 * Type alias for AuthorMeta.
 **/
export type AuthorMeta = z.infer<typeof AuthorMetaSchema>;
/**
 * Serializes AuthorMeta for transport.
 **/
export const serializeAuthorMeta = (author: AuthorMeta): string => JSON.stringify(AuthorMetaSchema.parse(author));
/**
 **/
export const deserializeAuthorMeta = (json: string): AuthorMeta => AuthorMetaSchema.parse(JSON.parse(json));
/**
 * Definition of AuthorShallowSchema.
 **/
export const AuthorShallowSchema = AuthorSchema;
/**
 * Type alias for AuthorShallow.
 **/
export type AuthorShallow = z.infer<typeof AuthorShallowSchema>;
/**
 * Serializes AuthorShallow for transport.
 **/
export const serializeAuthorShallow = (author: AuthorShallow): string => JSON.stringify(AuthorShallowSchema.parse(author));
/**
 **/
export const deserializeAuthorShallow = (json: string): AuthorShallow => AuthorShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Author diff validation.
 **/
export const AuthorDiffSchema = AuthorSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Author changes.
 **/
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;
/**
 * Retrieves the AuthorDiff value.
 **/
export const getAuthorDiff = (before: Author, after: Author): AuthorDiff => {
  const diff: AuthorDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.email !== after.email) diff.email = after.email;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseAuthor changes.
 **/
export const inverseAuthorDiff = (original: Author, appliedDiff: AuthorDiff): AuthorDiff => {
  const inverse: AuthorDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.email !== undefined) inverse.email = original.email;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeAuthor changes.
 **/
export const mergeAuthorDiff = (diff1: AuthorDiff, diff2: AuthorDiff): AuthorDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyAuthor changes.
 **/
export const applyAuthorDiff = (base: Author, diff: AuthorDiff): Author => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Author = {
    guid: base.guid,
    name: diff.name ?? base.name,
    email: diff.email ?? base.email,
  };

  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Authors diff validation.
 **/
export const AuthorsDiffSchema = z.object({
  removed: z.array(AuthorIdSchema).optional(),
  updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(),
  added: z.array(AuthorSchema).optional(),
});
/**
 * Diff type for tracking Authors changes.
 **/
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;

// #endregion ✍️Author

// #region 📄File
// File entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for File validation.
 **/
export const FileSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  remote: z.string().optional(),
  folder: FolderIdSchema.optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  blob: z.string().optional(),
  createdAt: DateProperty(),
  createdBy: z.string().optional(),
  updatedAt: DateProperty(),
  updatedBy: z.string().optional(),
});
/**
 * Type alias for File.
 **/
export type File = z.infer<typeof FileSchema>;
/**
 * Serializes File for transport.
 **/
export const serializeFile = (file: File): string => JSON.stringify(FileSchema.parse(file));
/**
 **/
export const deserializeFile = (json: string): File => FileSchema.parse(JSON.parse(json));

/**
 * Definition of FileMetaSchema.
 **/
export const FileMetaSchema = FileSchema.omit({ blob: true });
/**
 * Type alias for FileMeta.
 **/
export type FileMeta = z.infer<typeof FileMetaSchema>;
/**
 * Serializes FileMeta for transport.
 **/
export const serializeFileMeta = (file: FileMeta): string => JSON.stringify(FileMetaSchema.parse(file));
/**
 **/
export const deserializeFileMeta = (json: string): FileMeta => FileMetaSchema.parse(JSON.parse(json));
/**
 * Definition of FileShallowSchema.
 **/
export const FileShallowSchema = FileSchema;
/**
 * Type alias for FileShallow.
 **/
export type FileShallow = z.infer<typeof FileShallowSchema>;
/**
 * Serializes FileShallow for transport.
 **/
export const serializeFileShallow = (file: FileShallow): string => JSON.stringify(FileShallowSchema.parse(file));
/**
 **/
export const deserializeFileShallow = (json: string): FileShallow => FileShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for File diff validation.
 **/
export const FileDiffSchema = FileSchema.partial();
/**
 * Diff type for tracking File changes.
 **/
export type FileDiff = z.infer<typeof FileDiffSchema>;
/**
 * Retrieves the FileDiff value.
 **/
export const getFileDiff = (before: File, after: File): FileDiff => {
  const diff: FileDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.remote !== after.remote) diff.remote = after.remote;
  if (before.size !== after.size) diff.size = after.size;
  if (before.hash !== after.hash) diff.hash = after.hash;
  if (before.blob !== after.blob) diff.blob = after.blob;
  if (before.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
  if (before.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
  if (before.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
  if (before.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
  if (before.folder?.guid !== after.folder?.guid) diff.folder = after.folder;
  return diff;
};
/**
 * Diff type for tracking inverseFile changes.
 **/
export const inverseFileDiff = (original: File, appliedDiff: FileDiff): FileDiff => {
  const inverse: FileDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.remote !== undefined) inverse.remote = original.remote;
  if (appliedDiff.size !== undefined) inverse.size = original.size;
  if (appliedDiff.hash !== undefined) inverse.hash = original.hash;
  if (appliedDiff.blob !== undefined) inverse.blob = original.blob;
  if (appliedDiff.createdAt !== undefined) inverse.createdAt = original.createdAt;
  if (appliedDiff.createdBy !== undefined) inverse.createdBy = original.createdBy;
  if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = original.updatedAt;
  if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = original.updatedBy;
  if (appliedDiff.folder !== undefined) inverse.folder = original.folder;
  return inverse;
};
/**
 * Diff type for tracking mergeFile changes.
 **/
export const mergeFileDiff = (diff1: FileDiff, diff2: FileDiff): FileDiff => {
  return { ...diff1, ...diff2 };
};
/**
 * Diff type for tracking applyFile changes.
 **/
export const applyFileDiff = (base: File, diff: FileDiff): File => {
  const result: File = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (diff.remote !== undefined || base.remote !== undefined) result.remote = diff.remote ?? base.remote;
  if (diff.size !== undefined || base.size !== undefined) result.size = diff.size ?? base.size;
  if (diff.hash !== undefined || base.hash !== undefined) result.hash = diff.hash ?? base.hash;
  if (diff.createdAt !== undefined || base.createdAt !== undefined) result.createdAt = diff.createdAt ?? base.createdAt;
  if (diff.createdBy !== undefined || base.createdBy !== undefined) result.createdBy = diff.createdBy ?? base.createdBy;
  if (diff.updatedAt !== undefined || base.updatedAt !== undefined) result.updatedAt = diff.updatedAt ?? base.updatedAt;
  if (diff.updatedBy !== undefined || base.updatedBy !== undefined) result.updatedBy = diff.updatedBy ?? base.updatedBy;
  if (diff.folder !== undefined || base.folder !== undefined) result.folder = diff.folder ?? base.folder;
  if (diff.blob !== undefined || base.blob !== undefined) result.blob = diff.blob ?? base.blob;

  return result;
};

/**
 * Zod schema for Files diff validation.
 **/
export const FilesDiffSchema = z.object({
  removed: z.array(FileIdSchema).optional(),
  updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(),
  added: z.array(FileSchema).optional(),
});
/**
 * Diff type for tracking Files changes.
 **/
export type FilesDiff = z.infer<typeof FilesDiffSchema>;

// #endregion 📄File

// #region 📁Folder
// Folder entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Folder validation.
 **/
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
/**
 * Type alias for Folder.
 **/
export type Folder = z.infer<typeof FolderSchema>;
/**
 * Serializes Folder for transport.
 **/
export const serializeFolder = (folder: Folder): string => JSON.stringify(FolderSchema.parse(folder));
/**
 **/
export const deserializeFolder = (json: string): Folder => FolderSchema.parse(JSON.parse(json));

/**
 * Definition of FolderMetaSchema.
 **/
export const FolderMetaSchema = FolderSchema.omit({ attributes: true });
/**
 * Type alias for FolderMeta.
 **/
export type FolderMeta = z.infer<typeof FolderMetaSchema>;
/**
 * Serializes FolderMeta for transport.
 **/
export const serializeFolderMeta = (folder: FolderMeta): string => JSON.stringify(FolderMetaSchema.parse(folder));
/**
 **/
export const deserializeFolderMeta = (json: string): FolderMeta => FolderMetaSchema.parse(JSON.parse(json));
/**
 * Definition of FolderShallowSchema.
 **/
export const FolderShallowSchema = FolderSchema;
/**
 * Type alias for FolderShallow.
 **/
export type FolderShallow = z.infer<typeof FolderShallowSchema>;
/**
 * Serializes FolderShallow for transport.
 **/
export const serializeFolderShallow = (folder: FolderShallow): string => JSON.stringify(FolderShallowSchema.parse(folder));
/**
 **/
export const deserializeFolderShallow = (json: string): FolderShallow => FolderShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Folder diff validation.
 **/
export const FolderDiffSchema = FolderSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Folder changes.
 **/
export type FolderDiff = z.infer<typeof FolderDiffSchema>;
/**
 * Retrieves the FolderDiff value.
 **/
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
/**
 * Diff type for tracking inverseFolder changes.
 **/
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
/**
 * Diff type for tracking mergeFolder changes.
 **/
export const mergeFolderDiff = (diff1: FolderDiff, diff2: FolderDiff): FolderDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyFolder changes.
 **/
export const applyFolderDiff = (base: Folder, diff: FolderDiff): Folder => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Folders diff validation.
 **/
export const FoldersDiffSchema = z.object({
  removed: z.array(FolderIdSchema).optional(),
  updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(),
  added: z.array(FolderSchema).optional(),
});
/**
 * Diff type for tracking Folders changes.
 **/
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;

// #endregion 📁Folder

// #region 📏Benchmark
// Benchmark entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Benchmark validation.
 **/
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
/**
 * Type alias for Benchmark.
 **/
export type Benchmark = z.infer<typeof BenchmarkSchema>;
/**
 * Serializes Benchmark for transport.
 **/
export const serializeBenchmark = (benchmark: Benchmark): string => JSON.stringify(BenchmarkSchema.parse(benchmark));
/**
 **/
export const deserializeBenchmark = (json: string): Benchmark => BenchmarkSchema.parse(JSON.parse(json));

/**
 * Zod schema for Benchmark diff validation.
 **/
export const BenchmarkDiffSchema = BenchmarkSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Benchmark changes.
 **/
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;
/**
 * Diff type for tracking applyBenchmark changes.
 **/
export const applyBenchmarkDiff = (base: Benchmark, diff: BenchmarkDiff): Benchmark => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Benchmark = {
    guid: base.guid,
    name: diff.name ?? base.name,
  };

  if (diff.icon !== undefined || base.icon !== undefined) result.icon = diff.icon ?? base.icon;
  if (diff.min !== undefined || base.min !== undefined) result.min = diff.min ?? base.min;
  if (diff.minExcluded !== undefined || base.minExcluded !== undefined) result.minExcluded = diff.minExcluded ?? base.minExcluded;
  if (diff.maxExcluded !== undefined || base.maxExcluded !== undefined) result.maxExcluded = diff.maxExcluded ?? base.maxExcluded;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};
/**
 * Retrieves the BenchmarkDiff value.
 **/
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
/**
 * Diff type for tracking inverseBenchmark changes.
 **/
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
/**
 * Diff type for tracking mergeBenchmark changes.
 **/
export const mergeBenchmarkDiff = (diff1: BenchmarkDiff, diff2: BenchmarkDiff): BenchmarkDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};

/**
 * Zod schema for Benchmarks diff validation.
 **/
export const BenchmarksDiffSchema = z.object({
  removed: z.array(BenchmarkIdSchema).optional(),
  updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(),
  added: z.array(BenchmarkSchema).optional(),
});
/**
 * Diff type for tracking Benchmarks changes.
 **/
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;
// 📏getBenchmarksDiff computes the diff between two benchmark collections.
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

// 📏inverseBenchmarksDiff inverts a benchmark diff to reverse its effect.
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
// 📏mergeBenchmarksDiff merges two benchmark diffs into one.
const mergeBenchmarksDiff = (first: BenchmarksDiff, second: BenchmarksDiff): BenchmarksDiff => {
  return { ...first, ...second };
};

// 📏applyBenchmarksDiff applies a benchmark diff to a collection.
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

// #endregion 📏Benchmark

// #region 🔬Quality
// Quality entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Quality validation.
 **/
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
/**
 * Type alias for Quality.
 **/
export type Quality = z.infer<typeof QualitySchema>;
/**
 * Serializes Quality for transport.
 **/
export const serializeQuality = (quality: Quality): string => JSON.stringify(QualitySchema.parse(quality));
/**
 **/
export const deserializeQuality = (json: string): Quality => QualitySchema.parse(JSON.parse(json));

/**
 * Definition of QualityMetaSchema.
 **/
export const QualityMetaSchema = QualitySchema.omit({ benchmarks: true, attributes: true });
/**
 * Type alias for QualityMeta.
 **/
export type QualityMeta = z.infer<typeof QualityMetaSchema>;
/**
 * Serializes QualityMeta for transport.
 **/
export const serializeQualityMeta = (quality: QualityMeta): string => JSON.stringify(QualityMetaSchema.parse(quality));
/**
 **/
export const deserializeQualityMeta = (json: string): QualityMeta => QualityMetaSchema.parse(JSON.parse(json));
/**
 * Definition of QualityShallowSchema.
 **/
export const QualityShallowSchema = QualitySchema;
/**
 * Type alias for QualityShallow.
 **/
export type QualityShallow = z.infer<typeof QualityShallowSchema>;
/**
 * Serializes QualityShallow for transport.
 **/
export const serializeQualityShallow = (quality: QualityShallow): string => JSON.stringify(QualityShallowSchema.parse(quality));
/**
 **/
export const deserializeQualityShallow = (json: string): QualityShallow => QualityShallowSchema.parse(JSON.parse(json));

/**
 **/
export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true, attributes: true }).extend({
  benchmarks: BenchmarksDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Quality changes.
 **/
export type QualityDiff = z.infer<typeof QualityDiffSchema>;
/**
 * Retrieves the QualityDiff value.
 **/
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
/**
 * Diff type for tracking inverseQuality changes.
 **/
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
/**
 * Diff type for tracking mergeQuality changes.
 **/
export const mergeQualityDiff = (diff1: QualityDiff, diff2: QualityDiff): QualityDiff => {
  return {
    ...diff1,
    ...diff2,
    benchmarks: diff1.benchmarks && diff2.benchmarks ? mergeBenchmarksDiff(diff1.benchmarks, diff2.benchmarks) : (diff2.benchmarks ?? diff1.benchmarks),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking applyQuality changes.
 **/
export const applyQualityDiff = (base: Quality, diff: QualityDiff): Quality => {
  const benchmarks = diff.benchmarks ? applyBenchmarksDiff(base.benchmarks ?? [], diff.benchmarks) : base.benchmarks;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Qualities diff validation.
 **/
export const QualitiesDiffSchema = z.object({
  removed: z.array(QualityIdSchema).optional(),
  updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(),
  added: z.array(QualitySchema).optional(),
});
export type QualitiesDiff = z.infer<typeof QualitiesDiffSchema>;

// #endregion 🔬Quality

// #region ⚓Port
// Port entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Port validation.
 **/
export const PortSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  maxChildren: z.number().int().optional(),
  compatiblePorts: z.array(PortIdSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Port.
 **/
export type Port = z.infer<typeof PortSchema>;
/**
 * Serializes Port for transport.
 **/
export const serializePort = (iface: Port): string => JSON.stringify(PortSchema.parse(iface));
/**
 **/
export const deserializePort = (json: string): Port => PortSchema.parse(JSON.parse(json));

/**
 * Definition of PortMetaSchema.
 **/
export const PortMetaSchema = PortSchema.omit({ compatiblePorts: true, attributes: true });
/**
 * Type alias for PortMeta.
 **/
export type PortMeta = z.infer<typeof PortMetaSchema>;
/**
 * Serializes PortMeta for transport.
 **/
export const serializePortMeta = (port: PortMeta): string => JSON.stringify(PortMetaSchema.parse(port));
/**
 **/
export const deserializePortMeta = (json: string): PortMeta => PortMetaSchema.parse(JSON.parse(json));
/**
 * Definition of PortShallowSchema.
 **/
export const PortShallowSchema = PortSchema;
/**
 * Type alias for PortShallow.
 **/
export type PortShallow = z.infer<typeof PortShallowSchema>;
/**
 * Serializes PortShallow for transport.
 **/
export const serializePortShallow = (port: PortShallow): string => JSON.stringify(PortShallowSchema.parse(port));
/**
 **/
export const deserializePortShallow = (json: string): PortShallow => PortShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Port diff validation.
 **/
export const PortDiffSchema = PortSchema.partial()
  .omit({ compatiblePorts: true, attributes: true })
  .extend({
    compatiblePorts: z.array(PortIdSchema).optional(),
    attributes: AttributesDiffSchema.optional(),
    description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
    maxChildren: z.number().int().nullable().optional(),
  });
/**
 * Diff type for tracking Port changes.
 **/
export type PortDiff = z.infer<typeof PortDiffSchema>;
/**
 * Retrieves the PortDiff value.
 **/
export const getPortDiff = (before: Port, after: Port): PortDiff => {
  const diff: PortDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (before.maxChildren !== after.maxChildren) diff.maxChildren = after.maxChildren ?? null;
  if (JSON.stringify(before.compatiblePorts) !== JSON.stringify(after.compatiblePorts)) diff.compatiblePorts = after.compatiblePorts;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inversePort changes.
 **/
export const inversePortDiff = (original: Port, appliedDiff: PortDiff): PortDiff => {
  const inverse: PortDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.maxChildren !== undefined) inverse.maxChildren = original.maxChildren ?? null;
  if (appliedDiff.compatiblePorts !== undefined) inverse.compatiblePorts = original.compatiblePorts;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergePort changes.
 **/
export const mergePortDiff = (diff1: PortDiff, diff2: PortDiff): PortDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking applyPort changes.
 **/
export const applyPortDiff = (base: Port, diff: PortDiff): Port => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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
  if ("maxChildren" in diff) {
    if (diff.maxChildren !== null) result.maxChildren = diff.maxChildren;
  } else if (base.maxChildren !== undefined) {
    result.maxChildren = base.maxChildren;
  }
  if (diff.compatiblePorts !== undefined || base.compatiblePorts !== undefined) result.compatiblePorts = diff.compatiblePorts ?? base.compatiblePorts;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Ports diff validation.
 **/
export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(),
  added: z.array(PortSchema).optional(),
});
/**
 * Diff type for tracking Ports changes.
 **/
export type PortsDiff = z.infer<typeof PortsDiffSchema>;
/**
 * Retrieves the PortsDiff value.
 **/
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
/**
 * Diff type for tracking inversePorts changes.
 **/
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
/**
 * Diff type for tracking mergePorts changes.
 **/
export const mergePortsDiff = (diff1: PortsDiff, diff2: PortsDiff): PortsDiff => {
  return {
    removed: [...(diff1.removed ?? []), ...(diff2.removed ?? [])],
    updated: [...(diff1.updated ?? []), ...(diff2.updated ?? [])],
    added: [...(diff1.added ?? []), ...(diff2.added ?? [])],
  };
};
/**
 * Diff type for tracking applyPorts changes.
 **/
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

/**
 **/
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

// #endregion ⚓Port

// #region 📊Prop
// Prop entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Prop validation.
 **/
export const PropSchema = z.object({
  guid: z.string(),
  quality: QualityIdSchema,
  value: z.string(),
  unit: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Prop.
 **/
export type Prop = z.infer<typeof PropSchema>;
/**
 * Serializes Prop for transport.
 **/
export const serializeProp = (prop: Prop): string => JSON.stringify(PropSchema.parse(prop));
/**
 **/
export const deserializeProp = (json: string): Prop => PropSchema.parse(JSON.parse(json));

/**
 * Definition of PropMetaSchema.
 **/
export const PropMetaSchema = PropSchema.omit({ attributes: true });
/**
 * Type alias for PropMeta.
 **/
export type PropMeta = z.infer<typeof PropMetaSchema>;
/**
 * Serializes PropMeta for transport.
 **/
export const serializePropMeta = (prop: PropMeta): string => JSON.stringify(PropMetaSchema.parse(prop));
/**
 **/
export const deserializePropMeta = (json: string): PropMeta => PropMetaSchema.parse(JSON.parse(json));
/**
 * Definition of PropShallowSchema.
 **/
export const PropShallowSchema = PropSchema;
/**
 * Type alias for PropShallow.
 **/
export type PropShallow = z.infer<typeof PropShallowSchema>;
/**
 * Serializes PropShallow for transport.
 **/
export const serializePropShallow = (prop: PropShallow): string => JSON.stringify(PropShallowSchema.parse(prop));
/**
 **/
export const deserializePropShallow = (json: string): PropShallow => PropShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Prop diff validation.
 **/
export const PropDiffSchema = PropSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Prop changes.
 **/
export type PropDiff = z.infer<typeof PropDiffSchema>;
/**
 * Retrieves the PropDiff value.
 **/
export const getPropDiff = (before: Prop, after: Prop): PropDiff => {
  const diff: PropDiff = {};
  if (before.quality?.guid !== after.quality?.guid) diff.quality = after.quality;
  if (before.value !== after.value) diff.value = after.value;
  if (before.unit !== after.unit) diff.unit = after.unit;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseProp changes.
 **/
export const inversePropDiff = (original: Prop, appliedDiff: PropDiff): PropDiff => {
  const inverse: PropDiff = {};
  if (appliedDiff.quality !== undefined) inverse.quality = original.quality;
  if (appliedDiff.value !== undefined) inverse.value = original.value;
  if (appliedDiff.unit !== undefined) inverse.unit = original.unit;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeProp changes.
 **/
export const mergePropDiff = (diff1: PropDiff, diff2: PropDiff): PropDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyProp changes.
 **/
export const applyPropDiff = (base: Prop, diff: PropDiff): Prop => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Prop = {
    guid: base.guid,
    quality: diff.quality ?? base.quality,
    value: diff.value ?? base.value,
  };

  if (diff.unit !== undefined || base.unit !== undefined) result.unit = diff.unit ?? base.unit;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Props diff validation.
 **/
export const PropsDiffSchema = z.object({
  removed: z.array(PropIdSchema).optional(),
  updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(),
  added: z.array(PropSchema).optional(),
});
/**
 * Diff type for tracking Props changes.
 **/
export type PropsDiff = z.infer<typeof PropsDiffSchema>;
// 📊getPropsDiff computes the diff between two prop collections.
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
// 📊inversePropsDiff inverts a prop diff to reverse its effect.
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
// 📊mergePropsDiff merges two prop diffs into one.
const mergePropsDiff = (first: PropsDiff, second: PropsDiff): PropsDiff => {
  return { ...first, ...second };
};
// 📊applyPropsDiff applies a prop diff to a collection.
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

// #endregion 📊Prop

// #region 🏷️Tag
// Tag entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Tag validation.
 **/
export const TagSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Tag.
 **/
export type Tag = z.infer<typeof TagSchema>;
/**
 * Serializes Tag for transport.
 **/
export const serializeTag = (tag: Tag): string => JSON.stringify(TagSchema.parse(tag));
/**
 **/
export const deserializeTag = (json: string): Tag => TagSchema.parse(JSON.parse(json));

/**
 * Definition of TagMetaSchema.
 **/
export const TagMetaSchema = TagSchema.omit({ attributes: true });
/**
 * Type alias for TagMeta.
 **/
export type TagMeta = z.infer<typeof TagMetaSchema>;
/**
 * Serializes TagMeta for transport.
 **/
export const serializeTagMeta = (tag: TagMeta): string => JSON.stringify(TagMetaSchema.parse(tag));
/**
 **/
export const deserializeTagMeta = (json: string): TagMeta => TagMetaSchema.parse(JSON.parse(json));
/**
 * Definition of TagShallowSchema.
 **/
export const TagShallowSchema = TagSchema;
/**
 * Type alias for TagShallow.
 **/
export type TagShallow = z.infer<typeof TagShallowSchema>;
/**
 * Serializes TagShallow for transport.
 **/
export const serializeTagShallow = (tag: TagShallow): string => JSON.stringify(TagShallowSchema.parse(tag));
/**
 **/
export const deserializeTagShallow = (json: string): TagShallow => TagShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Tag diff validation.
 **/
export const TagDiffSchema = TagSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Tag changes.
 **/
export type TagDiff = z.infer<typeof TagDiffSchema>;
/**
 * Retrieves the TagDiff value.
 **/
export const getTagDiff = (before: Tag, after: Tag): TagDiff => {
  const diff: TagDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseTag changes.
 **/
export const inverseTagDiff = (original: Tag, appliedDiff: TagDiff): TagDiff => {
  const inverse: TagDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeTag changes.
 **/
export const mergeTagDiff = (diff1: TagDiff, diff2: TagDiff): TagDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyTag changes.
 **/
export const applyTagDiff = (base: Tag, diff: TagDiff): Tag => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Tags diff validation.
 **/
export const TagsDiffSchema = z.object({
  removed: z.array(TagIdSchema).optional(),
  updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(),
  added: z.array(TagSchema).optional(),
});
/**
 * Diff type for tracking Tags changes.
 **/
export type TagsDiff = z.infer<typeof TagsDiffSchema>;
/**
 * Retrieves the TagsDiff value.
 **/
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
/**
 * Diff type for tracking inverseTags changes.
 **/
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
/**
 * Diff type for tracking mergeTags changes.
 **/
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
/**
 * Diff type for tracking applyTags changes.
 **/
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

/**
 * Searches for matching Tag entry.
 **/
export const findTag = (tags: Tag[], guid: string): Tag => {
  const tag = tags.find((t) => t.guid === guid);
  if (!tag) throw new Error(`Tag ${guid} not found`);
  return tag;
};

// #endregion 🏷️Tag

// #region 💡Concept
// Concept entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Concept validation.
 **/
export const ConceptSchema = z.object({
  guid: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Concept.
 **/
export type Concept = z.infer<typeof ConceptSchema>;
/**
 * Serializes Concept for transport.
 **/
export const serializeConcept = (concept: Concept): string => JSON.stringify(ConceptSchema.parse(concept));
/**
 **/
export const deserializeConcept = (json: string): Concept => ConceptSchema.parse(JSON.parse(json));

/**
 * Definition of ConceptMetaSchema.
 **/
export const ConceptMetaSchema = ConceptSchema.omit({ attributes: true });
/**
 * Type alias for ConceptMeta.
 **/
export type ConceptMeta = z.infer<typeof ConceptMetaSchema>;
/**
 * Serializes ConceptMeta for transport.
 **/
export const serializeConceptMeta = (concept: ConceptMeta): string => JSON.stringify(ConceptMetaSchema.parse(concept));
/**
 **/
export const deserializeConceptMeta = (json: string): ConceptMeta => ConceptMetaSchema.parse(JSON.parse(json));
/**
 * Definition of ConceptShallowSchema.
 **/
export const ConceptShallowSchema = ConceptSchema;
/**
 * Type alias for ConceptShallow.
 **/
export type ConceptShallow = z.infer<typeof ConceptShallowSchema>;
/**
 * Serializes ConceptShallow for transport.
 **/
export const serializeConceptShallow = (concept: ConceptShallow): string => JSON.stringify(ConceptShallowSchema.parse(concept));
/**
 **/
export const deserializeConceptShallow = (json: string): ConceptShallow => ConceptShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Concept diff validation.
 **/
export const ConceptDiffSchema = ConceptSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Concept changes.
 **/
export type ConceptDiff = z.infer<typeof ConceptDiffSchema>;
/**
 * Retrieves the ConceptDiff value.
 **/
export const getConceptDiff = (before: Concept, after: Concept): ConceptDiff => {
  const diff: ConceptDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description ?? null;
  if (before.icon !== after.icon) diff.icon = after.icon ?? null;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseConcept changes.
 **/
export const inverseConceptDiff = (original: Concept, appliedDiff: ConceptDiff): ConceptDiff => {
  const inverse: ConceptDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description ?? null;
  if (appliedDiff.icon !== undefined) inverse.icon = original.icon ?? null;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeConcept changes.
 **/
export const mergeConceptDiff = (diff1: ConceptDiff, diff2: ConceptDiff): ConceptDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyConcept changes.
 **/
export const applyConceptDiff = (base: Concept, diff: ConceptDiff): Concept => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Concepts diff validation.
 **/
export const ConceptsDiffSchema = z.object({
  removed: z.array(ConceptIdSchema).optional(),
  updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(),
  added: z.array(ConceptSchema).optional(),
});
/**
 * Diff type for tracking Concepts changes.
 **/
export type ConceptsDiff = z.infer<typeof ConceptsDiffSchema>;
/**
 * Retrieves the ConceptsDiff value.
 **/
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
/**
 * Diff type for tracking inverseConcepts changes.
 **/
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
/**
 * Diff type for tracking mergeConcepts changes.
 **/
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
/**
 * Diff type for tracking applyConcepts changes.
 **/
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

/**
 * Searches for matching Concept entry.
 **/
export const findConcept = (concepts: Concept[], guid: string): Concept => {
  const concept = concepts.find((c) => c.guid === guid);
  if (!concept) throw new Error(`Concept ${guid} not found`);
  return concept;
};

// #endregion 💡Concept

// #region 🗿Model
// Model entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Model validation.
 **/
export const ModelSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  tags: z.array(TagIdSchema).optional(),
  file: FileIdSchema,
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Model.
 **/
export type Model = z.infer<typeof ModelSchema>;
/**
 * Serializes Model for transport.
 **/
export const serializeModel = (model: Model): string => JSON.stringify(ModelSchema.parse(model));
/**
 **/
export const deserializeModel = (json: string): Model => ModelSchema.parse(JSON.parse(json));

/**
 * Definition of ModelMetaSchema.
 **/
export const ModelMetaSchema = ModelSchema.omit({ tags: true, attributes: true });
/**
 * Type alias for ModelMeta.
 **/
export type ModelMeta = z.infer<typeof ModelMetaSchema>;
/**
 * Serializes ModelMeta for transport.
 **/
export const serializeModelMeta = (model: ModelMeta): string => JSON.stringify(ModelMetaSchema.parse(model));
/**
 **/
export const deserializeModelMeta = (json: string): ModelMeta => ModelMetaSchema.parse(JSON.parse(json));
/**
 * Definition of ModelShallowSchema.
 **/
export const ModelShallowSchema = ModelSchema;
/**
 * Type alias for ModelShallow.
 **/
export type ModelShallow = z.infer<typeof ModelShallowSchema>;
/**
 * Serializes ModelShallow for transport.
 **/
export const serializeModelShallow = (model: ModelShallow): string => JSON.stringify(ModelShallowSchema.parse(model));
/**
 **/
export const deserializeModelShallow = (json: string): ModelShallow => ModelShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Model diff validation.
 **/
export const ModelDiffSchema = ModelSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Model changes.
 **/
export type ModelDiff = z.infer<typeof ModelDiffSchema>;
/**
 * Retrieves the ModelDiff value.
 **/
export const getModelDiff = (before: Model, after: Model): ModelDiff => {
  const diff: ModelDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (JSON.stringify(before.tags) !== JSON.stringify(after.tags)) diff.tags = after.tags;
  if (before.file.guid !== after.file.guid) diff.file = after.file;
  if (before.description !== after.description) diff.description = after.description;
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking inverseModel changes.
 **/
export const inverseModelDiff = (original: Model, appliedDiff: ModelDiff): ModelDiff => {
  const inverse: ModelDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.tags !== undefined) inverse.tags = original.tags;
  if (appliedDiff.file !== undefined) inverse.file = original.file;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking mergeModel changes.
 **/
export const mergeModelDiff = (diff1: ModelDiff, diff2: ModelDiff): ModelDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyModel changes.
 **/
export const applyModelDiff = (base: Model, diff: ModelDiff): Model => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Models diff validation.
 **/
export const ModelsDiffSchema = z.object({
  removed: z.array(ModelIdSchema).optional(),
  updated: z.array(z.object({ model: ModelIdSchema, diff: ModelDiffSchema })).optional(),
  added: z.array(ModelSchema).optional(),
});
export type ModelsDiff = z.infer<typeof ModelsDiffSchema>;

/**
 * Equality check for Model values.
 **/
export const areSameModel = (model: Model, other: Model): boolean => {
  const modelTagGuids = model.tags?.map((t) => t.guid) ?? [];
  const otherTagGuids = other.tags?.map((t) => t.guid) ?? [];
  return modelTagGuids.every((guid) => otherTagGuids.includes(guid));
};

/**
 * Searches for matching Model entry.
 **/
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

/**
 * Retrieves the AllTagGuidsFromModels value.
 **/
export const getAllTagGuidsFromModels = (models: Model[]): string[] => {
  const tagsSet = new Set<string>();
  models.forEach((r) => {
    toArray(r.tags).forEach((tag) => tagsSet.add(tag.guid));
  });
  return Array.from(tagsSet).sort();
};

/**
 **/
export const filterModelsByTagGuids = (models: Model[], selectedTagGuids: string[]): Model[] => {
  if (!selectedTagGuids || selectedTagGuids.length === 0) return models;
  return models.filter((r) => {
    if (!r.tags || r.tags.length === 0) return false;
    const modelTagGuids = r.tags.map((t) => t.guid);
    return selectedTagGuids.every((guid) => modelTagGuids.includes(guid));
  });
};

/**
 * Retrieves the AvailableTagGuidsForModels value.
 **/
export const getAvailableTagGuidsForModels = (models: Model[], selectedTagGuids: string[]): string[] => {
  const filteredReps = filterModelsByTagGuids(models, selectedTagGuids);
  const availableTags = getAllTagGuidsFromModels(filteredReps);
  return availableTags.filter((guid) => !selectedTagGuids.includes(guid));
};

/**
 **/
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

/**
 * Constant value for SUPPORTED_3D_EXTENSIONS.
 **/
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

/**
 * Type alias for Supported3DExtension.
 **/
export type Supported3DExtension = (typeof SUPPORTED_3D_EXTENSIONS)[number];

/**
 **/
export const isSupportedModelExtension = (filename: string): boolean => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  return SUPPORTED_3D_EXTENSIONS.includes(ext as Supported3DExtension);
};

/**
 * Interface defining ModelFileValidation structure.
 **/
export interface ModelFileValidation {
  isValid: boolean;
  warning?: string;
  extension?: string;
}

/**
 * Validates ModelFile against constraints.
 **/
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

// #endregion 🗿Model

// #region 🔌Connector
// Connector entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Connector validation.
 **/
export const ConnectorSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  t: z.number(),
  point: PointSchema,
  direction: VectorSchema,
  description: z.string().optional(),
  port: PortIdSchema.optional(),
  mandatory: z.boolean().optional(),
  maxChildren: z.number().int().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Connector.
 **/
export type Connector = z.infer<typeof ConnectorSchema>;
/**
 * Serializes Connector for transport.
 **/
export const serializeConnector = (connector: Connector): string => JSON.stringify(ConnectorSchema.parse(connector));
/**
 **/
export const deserializeConnector = (json: string): Connector => ConnectorSchema.parse(JSON.parse(json));

/**
 * Definition of ConnectorMetaSchema.
 **/
export const ConnectorMetaSchema = ConnectorSchema.omit({ props: true, attributes: true });
/**
 * Type alias for ConnectorMeta.
 **/
export type ConnectorMeta = z.infer<typeof ConnectorMetaSchema>;
/**
 * Serializes ConnectorMeta for transport.
 **/
export const serializeConnectorMeta = (connector: ConnectorMeta): string => JSON.stringify(ConnectorMetaSchema.parse(connector));
/**
 **/
export const deserializeConnectorMeta = (json: string): ConnectorMeta => ConnectorMetaSchema.parse(JSON.parse(json));
/**
 * Definition of ConnectorShallowSchema.
 **/
export const ConnectorShallowSchema = ConnectorSchema.omit({ props: true }).extend({ props: z.array(PropMetaSchema).optional() });
/**
 * Type alias for ConnectorShallow.
 **/
export type ConnectorShallow = z.infer<typeof ConnectorShallowSchema>;
/**
 * Serializes ConnectorShallow for transport.
 **/
export const serializeConnectorShallow = (connector: ConnectorShallow): string => JSON.stringify(ConnectorShallowSchema.parse(connector));
/**
 **/
export const deserializeConnectorShallow = (json: string): ConnectorShallow => ConnectorShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Connector diff validation.
 **/
export const ConnectorDiffSchema = ConnectorSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({
  point: PointDiffSchema.optional(),
  direction: VectorDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
  maxChildren: z.number().int().nullable().optional(),
});
/**
 * Diff type for tracking Connector changes.
 **/
export type ConnectorDiff = z.infer<typeof ConnectorDiffSchema>;
/**
 * Retrieves the ConnectorDiff value.
 **/
export const getConnectorDiff = (before: Connector, after: Connector): ConnectorDiff => {
  const diff: ConnectorDiff = {};
  if (before.name !== after.name) diff.name = after.name;
  if (before.description !== after.description) diff.description = after.description;
  if (before.port?.guid !== after.port?.guid) diff.port = after.port;
  if (before.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
  if (before.maxChildren !== after.maxChildren) diff.maxChildren = after.maxChildren ?? null;
  if (before.t !== after.t) diff.t = after.t;
  if (!deepEqual(before.point, after.point)) diff.point = getPointDiff(before.point, after.point);
  if (!deepEqual(before.direction, after.direction)) diff.direction = getVectorDiff(before.direction, after.direction);
  if (!deepEqual(before.props, after.props)) diff.props = getPropsDiff(before.props ?? [], after.props ?? []);
  if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
  return diff;
};
/**
 * Diff type for tracking mergeConnector changes.
 **/
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
/**
 * Diff type for tracking inverseConnector changes.
 **/
export const inverseConnectorDiff = (original: Connector, appliedDiff: ConnectorDiff): ConnectorDiff => {
  const inverse: ConnectorDiff = {};
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.port !== undefined) inverse.port = original.port;
  if (appliedDiff.mandatory !== undefined) inverse.mandatory = original.mandatory;
  if (appliedDiff.maxChildren !== undefined) inverse.maxChildren = original.maxChildren ?? null;
  if (appliedDiff.t !== undefined) inverse.t = original.t;
  if (appliedDiff.point !== undefined) inverse.point = inversePointDiff(original.point, appliedDiff.point);
  if (appliedDiff.direction !== undefined) inverse.direction = inverseVectorDiff(original.direction, appliedDiff.direction);
  if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(original.props ?? [], appliedDiff.props);
  if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking applyConnector changes.
 **/
export const applyConnectorDiff = (base: Connector, diff: ConnectorDiff): Connector => {
  const props = diff.props ? applyPropsDiff(base.props ?? [], diff.props) : base.props;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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
  if ("maxChildren" in diff) {
    if (diff.maxChildren !== null) result.maxChildren = diff.maxChildren;
  } else if (base.maxChildren !== undefined) {
    result.maxChildren = base.maxChildren;
  }
  if (props && props.length > 0) result.props = props;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Zod schema for Connectors diff validation.
 **/
export const ConnectorsDiffSchema = z.object({
  removed: z.array(ConnectorIdSchema).optional(),
  updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(),
  added: z.array(ConnectorSchema).optional(),
});
/**
 * Diff type for tracking Connectors changes.
 **/
export type ConnectorsDiff = z.infer<typeof ConnectorsDiffSchema>;
// 🔌getConnectorsDiff computes the diff between two connector collections.
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

/**
 **/
export const unifyConnectorPortsAndCompatiblePortsForTypes = (types: Type[]): TypesDiff => {
  return { updated: [] };
};

/**
 **/
export const areConnectorsCompatible = (connector: Connector, otherPort: Connector): boolean => {
  return true;
};

/**
 * Searches for matching Connector entry.
 **/
export const findConnector = (connectors: Connector[], connectorGuid: string): Connector => {
  const connector = connectors.find((p) => p.guid === connectorGuid);
  if (!connector) throw new Error(`Connector ${connectorGuid} not found in connectors`);
  return connector;
};

// #endregion 🔌Connector

// #region 🧱Type
// Type entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Type validation.
 **/
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
/**
 * Type alias for Type.
 **/
export type Type = z.infer<typeof TypeSchema>;
/**
 * Serializes Type for transport.
 **/
export const serializeType = (type: Type): string => JSON.stringify(TypeSchema.parse(type));
/**
 **/
export const deserializeType = (json: string): Type => TypeSchema.parse(JSON.parse(json));

/**
 * Definition of TypeMetaSchema.
 **/
export const TypeMetaSchema = TypeSchema.omit({ models: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
/**
 * Type alias for TypeMeta.
 **/
export type TypeMeta = z.infer<typeof TypeMetaSchema>;
/**
 * Serializes TypeMeta for transport.
 **/
export const serializeTypeMeta = (type: TypeMeta): string => JSON.stringify(TypeMetaSchema.parse(type));
/**
 **/
export const deserializeTypeMeta = (json: string): TypeMeta => TypeMetaSchema.parse(JSON.parse(json));
/**
 * Definition of TypeShallowSchema.
 **/
export const TypeShallowSchema = TypeSchema.omit({ models: true, connectors: true, props: true, attributes: true }).extend({
  models: z.array(ModelMetaSchema).optional(),
  connectors: z.array(ConnectorMetaSchema).optional(),
  props: z.array(PropMetaSchema).optional(),
  attributes: z.array(AttributeMetaSchema).optional(),
});
/**
 * Type alias for TypeShallow.
 **/
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
/**
 * Serializes TypeShallow for transport.
 **/
export const serializeTypeShallow = (type: TypeShallow): string => JSON.stringify(TypeShallowSchema.parse(type));
/**
 **/
export const deserializeTypeShallow = (json: string): TypeShallow => TypeShallowSchema.parse(JSON.parse(json));
/**
 * Zod schema for Type diff validation.
 **/
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
/**
 **/
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
/**
 * Retrieves the TypeDiff value.
 **/
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

/**
 * Diff type for tracking applyType changes.
 **/
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

/**
 * Diff type for tracking mergeType changes.
 **/
export const mergeTypeDiff = (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};

/**
 * Diff type for tracking inverseType changes.
 **/
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

/**
 * Zod schema for Types diff validation.
 **/
export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(),
  added: z.array(TypeSchema).optional(),
});
/**
 * Diff type for tracking Types changes.
 **/
export type TypesDiff = z.infer<typeof TypesDiffSchema>;

/**
 * Searches for matching ConnectorInType entry.
 **/
export const findConnectorInType = (type: Type, connectorGuid: string): Connector => findConnector(type.connectors ?? [], connectorGuid);

// #endregion 🧱Type

// #region 🎨Layer
// Layer entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Layer validation.
 **/
export const LayerSchema = z.object({
  guid: z.string(),
  path: z.string(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Layer.
 **/
export type Layer = z.infer<typeof LayerSchema>;
/**
 * Serializes Layer for transport.
 **/
export const serializeLayer = (layer: Layer): string => JSON.stringify(LayerSchema.parse(layer));
/**
 **/
export const deserializeLayer = (json: string): Layer => LayerSchema.parse(JSON.parse(json));

/**
 * Definition of LayerMetaSchema.
 **/
export const LayerMetaSchema = LayerSchema.omit({ attributes: true });
/**
 * Type alias for LayerMeta.
 **/
export type LayerMeta = z.infer<typeof LayerMetaSchema>;
/**
 * Serializes LayerMeta for transport.
 **/
export const serializeLayerMeta = (layer: LayerMeta): string => JSON.stringify(LayerMetaSchema.parse(layer));
/**
 **/
export const deserializeLayerMeta = (json: string): LayerMeta => LayerMetaSchema.parse(JSON.parse(json));
/**
 * Definition of LayerShallowSchema.
 **/
export const LayerShallowSchema = LayerSchema;
/**
 * Type alias for LayerShallow.
 **/
export type LayerShallow = z.infer<typeof LayerShallowSchema>;
/**
 * Serializes LayerShallow for transport.
 **/
export const serializeLayerShallow = (layer: LayerShallow): string => JSON.stringify(LayerShallowSchema.parse(layer));
/**
 **/
export const deserializeLayerShallow = (json: string): LayerShallow => LayerShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Layer diff validation.
 **/
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Layer changes.
 **/
export type LayerDiff = z.infer<typeof LayerDiffSchema>;

/**
 * Retrieves the LayerDiff value.
 **/
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
/**
 * Diff type for tracking inverseLayer changes.
 **/
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
/**
 * Diff type for tracking mergeLayer changes.
 **/
export const mergeLayerDiff = (diff1: LayerDiff, diff2: LayerDiff): LayerDiff => {
  return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
};
/**
 * Diff type for tracking applyLayer changes.
 **/
export const applyLayerDiff = (base: Layer, diff: LayerDiff): Layer => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Layers diff validation.
 **/
export const LayersDiffSchema = z.object({
  removed: z.array(LayerIdSchema).optional(),
  updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(),
  added: z.array(LayerSchema).optional(),
});
/**
 * Diff type for tracking Layers changes.
 **/
export type LayersDiff = z.infer<typeof LayersDiffSchema>;

// #endregion 🎨Layer

// #region 🧩Piece
// Piece entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Piece validation.
 **/
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
/**
 * Type alias for Piece.
 **/
export type Piece = z.infer<typeof PieceSchema>;
/**
 * Serializes Piece for transport.
 **/
export const serializePiece = (piece: Piece): string => JSON.stringify(PieceSchema.parse(piece));
/**
 **/
export const deserializePiece = (json: string): Piece => PieceSchema.parse(JSON.parse(json));

/**
 * Definition of PieceMetaSchema.
 **/
export const PieceMetaSchema = PieceSchema.omit({ props: true, attributes: true });
/**
 * Type alias for PieceMeta.
 **/
export type PieceMeta = z.infer<typeof PieceMetaSchema>;
/**
 * Serializes PieceMeta for transport.
 **/
export const serializePieceMeta = (piece: PieceMeta): string => JSON.stringify(PieceMetaSchema.parse(piece));
/**
 **/
export const deserializePieceMeta = (json: string): PieceMeta => PieceMetaSchema.parse(JSON.parse(json));
/**
 * Definition of PieceShallowSchema.
 **/
export const PieceShallowSchema = PieceSchema.omit({ props: true }).extend({ props: z.array(PropMetaSchema).optional() });
/**
 * Type alias for PieceShallow.
 **/
export type PieceShallow = z.infer<typeof PieceShallowSchema>;
/**
 * Serializes PieceShallow for transport.
 **/
export const serializePieceShallow = (piece: PieceShallow): string => JSON.stringify(PieceShallowSchema.parse(piece));
/**
 **/
export const deserializePieceShallow = (json: string): PieceShallow => PieceShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Piece diff validation.
 **/
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({
  plane: PlaneDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Piece changes.
 **/
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
/**
 * Retrieves the PieceDiff value.
 **/
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
/**
 * Diff type for tracking inversePiece changes.
 **/
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
/**
 * Diff type for tracking mergePiece changes.
 **/
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => {
  return {
    ...diff1,
    ...diff2,
    props: diff1.props && diff2.props ? mergePropsDiff(diff1.props, diff2.props) : (diff2.props ?? diff1.props),
    attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes),
  };
};
/**
 * Diff type for tracking applyPiece changes.
 **/
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
  const props = diff.props ? applyPropsDiff(base.props ?? [], diff.props) : base.props;
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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

/**
 * Zod schema for Pieces diff validation.
 **/
export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
/**
 * Diff type for tracking Pieces changes.
 **/
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

/**
 * Retrieves the PieceModelFileGuids value.
 **/
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

/**
 * Retrieves the PieceModelUrls value.
 **/
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
/**
 **/
export const fixPieceInDesign = (kit: Kit, designId: string, pieceId: string): DesignDiff => {
  const parentConnection = findParentConnectionForPieceInDesign(kit, designId, pieceId);
  return {
    connections: {
      removed: [{ guid: parentConnection.guid }],
    },
  };
};

/**
 **/
export const fixPiecesInDesign = (kit: Kit, designId: string, pieceIds: string[]): DesignDiff => {
  const parentConnections = pieceIds.map((pieceId) => findParentConnectionForPieceInDesign(kit, designId, pieceId));
  return {
    connections: {
      removed: parentConnections.map((c) => ({ guid: c.guid })),
    },
  };
};

/**
 **/
export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined;
  const isCenterSet = piece.center !== undefined;
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.guid} has inconsistent plane and center`);
  return isPlaneSet;
};

/**
 **/
/**
 * Searches for matching Piece entry.
 **/
export const findPiece = (pieces: Piece[], pieceGuid: string): Piece => {
  const piece = pieces.find((p) => p.guid === pieceGuid);
  if (!piece) throw new Error(`Piece ${pieceGuid} not found in pieces`);
  return piece;
};

// #endregion 🧩Piece

// #region 👥Group
// Group entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Group validation.
 **/
export const GroupSchema = z.object({
  guid: z.string(),
  pieces: z.array(PieceIdSchema),
  color: z.string().optional(),
  name: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Group.
 **/
export type Group = z.infer<typeof GroupSchema>;
/**
 * Zod schema for Group diff validation.
 **/
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Group changes.
 **/
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
/**
 * Retrieves the GroupDiff value.
 **/
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
/**
 * Diff type for tracking inverseGroup changes.
 **/
export const inverseGroupDiff = (original: Group, appliedDiff: GroupDiff): GroupDiff => {
  const inverse: GroupDiff = {};
  if (appliedDiff.pieces !== undefined) inverse.pieces = original.pieces;
  if (appliedDiff.color !== undefined) inverse.color = original.color;
  if (appliedDiff.name !== undefined) inverse.name = original.name;
  if (appliedDiff.description !== undefined) inverse.description = original.description;
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
};
/**
 * Diff type for tracking applyGroup changes.
 **/
export const applyGroupDiff = (base: Group, diff: GroupDiff): Group => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

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
/**
 * Diff type for tracking mergeGroup changes.
 **/
export const mergeGroupDiff = (diff1: GroupDiff, diff2: GroupDiff): GroupDiff => {
  return {
    ...diff1,
    ...diff2,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
};
/**
 * Zod schema for Groups diff validation.
 **/
export const GroupsDiffSchema = z.object({
  removed: z.array(GroupIdSchema).optional(),
  updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(),
  added: z.array(GroupSchema).optional(),
});
export type GroupsDiff = z.infer<typeof GroupsDiffSchema>;
/**
 * Serializes Group for transport.
 **/
export const serializeGroup = (group: Group): string => JSON.stringify(GroupSchema.parse(group));
/**
 **/
export const deserializeGroup = (json: string): Group => GroupSchema.parse(JSON.parse(json));

/**
 * Definition of GroupMetaSchema.
 **/
export const GroupMetaSchema = GroupSchema.omit({ pieces: true, attributes: true });
/**
 * Type alias for GroupMeta.
 **/
export type GroupMeta = z.infer<typeof GroupMetaSchema>;
/**
 * Serializes GroupMeta for transport.
 **/
export const serializeGroupMeta = (group: GroupMeta): string => JSON.stringify(GroupMetaSchema.parse(group));
/**
 **/
export const deserializeGroupMeta = (json: string): GroupMeta => GroupMetaSchema.parse(JSON.parse(json));
/**
 * Definition of GroupShallowSchema.
 **/
export const GroupShallowSchema = GroupSchema;
/**
 * Type alias for GroupShallow.
 **/
export type GroupShallow = z.infer<typeof GroupShallowSchema>;
/**
 * Serializes GroupShallow for transport.
 **/
export const serializeGroupShallow = (group: GroupShallow): string => JSON.stringify(GroupShallowSchema.parse(group));
/**
 **/
export const deserializeGroupShallow = (json: string): GroupShallow => GroupShallowSchema.parse(JSON.parse(json));

// #endregion 👥Group

// #region ↔️Side
// Side entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Side validation.
 **/
export const SideSchema = z.object({
  piece: PieceIdSchema,
  designPiece: PieceIdSchema.optional(),
  connector: ConnectorIdSchema.optional(),
});
/**
 * Type alias for Side.
 **/
export type Side = z.infer<typeof SideSchema>;
/**
 * Zod schema for Side diff validation.
 **/
export const SideDiffSchema = SideSchema.partial();
/**
 * Diff type for tracking Side changes.
 **/
export type SideDiff = z.infer<typeof SideDiffSchema>;
/**
 * Zod schema for validating Side identifiers.
 **/
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
/**
 * Identifier type for Side entities.
 **/
export type SideId = z.infer<typeof SideIdSchema>;
/**
 * Zod schema for Sides diff validation.
 **/
export const SidesDiffSchema = z.object({
  removed: z.array(SideIdSchema).optional(),
  updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(),
  added: z.array(SideSchema).optional(),
});
/**
 * Diff type for tracking Sides changes.
 **/
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
/**
 * Retrieves the SideDiff value.
 **/
export const getSideDiff = (before: Side, after: Side): SideDiff => {
  const diff: SideDiff = {};
  if (before.piece?.guid !== after.piece?.guid) diff.piece = after.piece;
  if (before.designPiece?.guid !== after.designPiece?.guid) diff.designPiece = after.designPiece;
  if (before.connector?.guid !== after.connector?.guid) diff.connector = after.connector;
  return diff;
};
/**
 * Diff type for tracking inverseSide changes.
 **/
export const inverseSideDiff = (original: Side, appliedDiff: SideDiff): SideDiff => {
  const inverse: SideDiff = {};
  if (appliedDiff.piece !== undefined) inverse.piece = original.piece;
  if (appliedDiff.designPiece !== undefined) inverse.designPiece = original.designPiece;
  if (appliedDiff.connector !== undefined) inverse.connector = original.connector;
  return inverse;
};
/**
 * Diff type for tracking mergeSide changes.
 **/
export const mergeSideDiff = (diff1: SideDiff, diff2: SideDiff): SideDiff => {
  return { ...diff1, ...diff2 };
};
/**
 * Diff type for tracking applySide changes.
 **/
export const applySideDiff = (base: Side, diff: SideDiff): Side => {
  const result: Side = {
    piece: diff.piece ?? base.piece,
  };

  if (diff.designPiece !== undefined || base.designPiece !== undefined) result.designPiece = diff.designPiece ?? base.designPiece;
  if (diff.connector !== undefined || base.connector !== undefined) result.connector = diff.connector ?? base.connector;

  return result;
};
/**
 * Serializes Side for transport.
 **/
export const serializeSide = (side: Side): string => JSON.stringify(SideSchema.parse(side));
/**
 **/
export const deserializeSide = (json: string): Side => SideSchema.parse(JSON.parse(json));
/**
 * Equality check for Side values.
 **/
export const areSameSide = (a: Side, b: Side): boolean => a.piece.guid === b.piece.guid && a.designPiece?.guid === b.designPiece?.guid && a.connector?.guid === b.connector?.guid;

// #endregion ↔️Side

// #region 🔗Connection
// Connection entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Connection validation.
 **/
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
/**
 * Type alias for Connection.
 **/
export type Connection = z.infer<typeof ConnectionSchema>;
/**
 * Zod schema for Connection diff validation.
 **/
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ guid: true, connected: true, connecting: true, attributes: true }).extend({
  connected: SideDiffSchema.optional(),
  connecting: SideDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Connection changes.
 **/
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
/**
 * Retrieves the ConnectionDiff value.
 **/
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

/**
 * Diff type for tracking applyConnection changes.
 **/
export const applyConnectionDiff = (base: Connection, diff: ConnectionDiff): Connection => {
  const attributes = diff.attributes ? applyAttributesDiff(base.attributes ?? [], diff.attributes) : base.attributes;

  const result: Connection = {
    guid: base.guid,
    connected: diff.connected ? applySideDiff(base.connected, diff.connected) : base.connected,
    connecting: diff.connecting ? applySideDiff(base.connecting, diff.connecting) : base.connecting,
  };

  if (diff.gap !== undefined || base.gap !== undefined) result.gap = diff.gap !== undefined && base.gap !== undefined ? base.gap + diff.gap : (diff.gap ?? base.gap);
  if (diff.shift !== undefined || base.shift !== undefined) result.shift = diff.shift !== undefined && base.shift !== undefined ? base.shift + diff.shift : (diff.shift ?? base.shift);
  if (diff.rise !== undefined || base.rise !== undefined) result.rise = diff.rise !== undefined && base.rise !== undefined ? base.rise + diff.rise : (diff.rise ?? base.rise);
  if (diff.rotation !== undefined || base.rotation !== undefined) result.rotation = diff.rotation !== undefined && base.rotation !== undefined ? base.rotation + diff.rotation : (diff.rotation ?? base.rotation);
  if (diff.turn !== undefined || base.turn !== undefined) result.turn = diff.turn !== undefined && base.turn !== undefined ? base.turn + diff.turn : (diff.turn ?? base.turn);
  if (diff.tilt !== undefined || base.tilt !== undefined) result.tilt = diff.tilt !== undefined && base.tilt !== undefined ? base.tilt + diff.tilt : (diff.tilt ?? base.tilt);
  if (diff.u !== undefined || base.u !== undefined) result.u = diff.u !== undefined && base.u !== undefined ? base.u + diff.u : (diff.u ?? base.u);
  if (diff.v !== undefined || base.v !== undefined) result.v = diff.v !== undefined && base.v !== undefined ? base.v + diff.v : (diff.v ?? base.v);
  if (diff.description !== undefined || base.description !== undefined) result.description = diff.description ?? base.description;
  if (attributes && attributes.length > 0) result.attributes = attributes;

  return result;
};

/**
 * Diff type for tracking mergeConnection changes.
 **/
export const mergeConnectionDiff = (diff1: ConnectionDiff, diff2: ConnectionDiff): ConnectionDiff => {
  return {
    ...diff1,
    ...diff2,
    connected: diff2.connected || diff1.connected,
    connecting: diff2.connecting || diff1.connecting,
    attributes: diff2.attributes || diff1.attributes,
  };
};

/**
 * Diff type for tracking inverseConnection changes.
 **/
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

/**
 * Zod schema for Connections diff validation.
 **/
export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(),
  added: z.array(ConnectionSchema).optional(),
});
/**
 * Diff type for tracking Connections changes.
 **/
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
/**
 * Serializes Connection for transport.
 **/
export const serializeConnection = (connection: Connection): string => JSON.stringify(ConnectionSchema.parse(connection));
/**
 **/
export const deserializeConnection = (json: string): Connection => ConnectionSchema.parse(JSON.parse(json));

/**
 * Definition of ConnectionMetaSchema.
 **/
export const ConnectionMetaSchema = ConnectionSchema.omit({ attributes: true });
/**
 * Type alias for ConnectionMeta.
 **/
export type ConnectionMeta = z.infer<typeof ConnectionMetaSchema>;
/**
 * Serializes ConnectionMeta for transport.
 **/
export const serializeConnectionMeta = (connection: ConnectionMeta): string => JSON.stringify(ConnectionMetaSchema.parse(connection));
/**
 **/
export const deserializeConnectionMeta = (json: string): ConnectionMeta => ConnectionMetaSchema.parse(JSON.parse(json));
/**
 * Definition of ConnectionShallowSchema.
 **/
export const ConnectionShallowSchema = ConnectionSchema;
/**
 * Type alias for ConnectionShallow.
 **/
export type ConnectionShallow = z.infer<typeof ConnectionShallowSchema>;
/**
 * Serializes ConnectionShallow for transport.
 **/
export const serializeConnectionShallow = (connection: ConnectionShallow): string => JSON.stringify(ConnectionShallowSchema.parse(connection));
/**
 **/
export const deserializeConnectionShallow = (json: string): ConnectionShallow => ConnectionShallowSchema.parse(JSON.parse(json));

/**
 * Equality check for Connection values.
 **/
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

/**
 * Searches for matching Connection entry.
 **/
export const findConnection = (connections: Connection[], connectionGuid: string): Connection => {
  const connection = connections.find((c) => c.guid === connectionGuid);
  if (!connection) throw new Error(`Connection ${connectionGuid} not found in connections`);
  return connection;
};

/**
 * Searches for matching PieceConnections entry.
 **/
export const findPieceConnections = (connections: Connection[], pieceGuid: string): Connection[] => {
  return connections.filter((c) => c.connected.piece.guid === pieceGuid || c.connecting.piece.guid === pieceGuid);
};

/**
 * Searches for matching ConnectorForPieceInConnection entry.
 **/
export const findConnectorForPieceInConnection = (type: Type, connection: Connection, pieceGuid: string): Connector | undefined => {
  const connectorGuid = connection.connected.piece.guid === pieceGuid ? connection.connected.connector?.guid : connection.connecting.connector?.guid;
  if (!connectorGuid) return undefined;
  return findConnectorInType(type, connectorGuid);
};

// #endregion 🔗Connection

// #region 📈Stat
// Stat entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Stat validation.
 **/
export const StatSchema = z.object({
  guid: z.string(),
  quality: QualityIdSchema,
  unit: z.string().optional(),
  min: z.number().optional(),
  minExcluded: z.boolean().optional(),
  max: z.number().optional(),
  maxExcluded: z.boolean().optional(),
});
/**
 * Type alias for Stat.
 **/
export type Stat = z.infer<typeof StatSchema>;
/**
 * Zod schema for Stat diff validation.
 **/
export const StatDiffSchema = StatSchema.partial();
/**
 * Diff type for tracking Stat changes.
 **/
export type StatDiff = z.infer<typeof StatDiffSchema>;
/**
 * Retrieves the StatDiff value.
 **/
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
/**
 * Diff type for tracking inverseStat changes.
 **/
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
/**
 * Diff type for tracking applyStat changes.
 **/
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
/**
 * Diff type for tracking mergeStat changes.
 **/
export const mergeStatDiff = (diff1: StatDiff, diff2: StatDiff): StatDiff => {
  return { ...diff1, ...diff2 };
};
/**
 * Zod schema for Stats diff validation.
 **/
export const StatsDiffSchema = z.object({
  removed: z.array(StatIdSchema).optional(),
  updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(),
  added: z.array(StatSchema).optional(),
});
export type StatsDiff = z.infer<typeof StatsDiffSchema>;
/**
 * Serializes Stat for transport.
 **/
export const serializeStat = (stat: Stat): string => JSON.stringify(StatSchema.parse(stat));
/**
 **/
export const deserializeStat = (json: string): Stat => StatSchema.parse(JSON.parse(json));

/**
 * Definition of StatMetaSchema.
 **/
export const StatMetaSchema = StatSchema;
/**
 * Type alias for StatMeta.
 **/
export type StatMeta = z.infer<typeof StatMetaSchema>;
/**
 * Serializes StatMeta for transport.
 **/
export const serializeStatMeta = (stat: StatMeta): string => JSON.stringify(StatMetaSchema.parse(stat));
/**
 **/
export const deserializeStatMeta = (json: string): StatMeta => StatMetaSchema.parse(JSON.parse(json));
/**
 * Definition of StatShallowSchema.
 **/
export const StatShallowSchema = StatSchema;
/**
 * Type alias for StatShallow.
 **/
export type StatShallow = z.infer<typeof StatShallowSchema>;
/**
 * Serializes StatShallow for transport.
 **/
export const serializeStatShallow = (stat: StatShallow): string => JSON.stringify(StatShallowSchema.parse(stat));
/**
 **/
export const deserializeStatShallow = (json: string): StatShallow => StatShallowSchema.parse(JSON.parse(json));

// #endregion 📈Stat

// #region 📐Design
// Design entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Design validation.
 **/
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
/**
 * Type alias for Design.
 **/
export type Design = z.infer<typeof DesignSchema>;
/**
 * Serializes Design for transport.
 **/
export const serializeDesign = (design: Design): string => JSON.stringify(DesignSchema.parse(design));
/**
 **/
export const deserializeDesign = (json: string): Design => DesignSchema.parse(JSON.parse(json));

/**
 * Definition of DesignMetaSchema.
 **/
export const DesignMetaSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true, authors: true, concepts: true });
/**
 * Type alias for DesignMeta.
 **/
export type DesignMeta = z.infer<typeof DesignMetaSchema>;
/**
 * Serializes DesignMeta for transport.
 **/
export const serializeDesignMeta = (design: DesignMeta): string => JSON.stringify(DesignMetaSchema.parse(design));
/**
 **/
export const deserializeDesignMeta = (json: string): DesignMeta => DesignMetaSchema.parse(JSON.parse(json));
/**
 * Definition of DesignShallowSchema.
 **/
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true }).extend({
  pieces: z.array(PieceMetaSchema).optional(),
  connections: z.array(ConnectionMetaSchema).optional(),
  stats: z.array(StatMetaSchema).optional(),
  props: z.array(PropMetaSchema).optional(),
  layers: z.array(LayerMetaSchema).optional(),
  groups: z.array(GroupMetaSchema).optional(),
  attributes: z.array(AttributeMetaSchema).optional(),
});
/**
 * Type alias for DesignShallow.
 **/
export type DesignShallow = z.infer<typeof DesignShallowSchema>;
/**
 * Serializes DesignShallow for transport.
 **/
export const serializeDesignShallow = (design: DesignShallow): string => JSON.stringify(DesignShallowSchema.parse(design));
/**
 **/
export const deserializeDesignShallow = (json: string): DesignShallow => DesignShallowSchema.parse(JSON.parse(json));
/**
 * Zod schema for Design diff validation.
 **/
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

/**
 * Diff type for tracking Design changes.
 **/
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
/**
 * Retrieves the DesignDiff value.
 **/
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
/**
 * Diff type for tracking mergeDesign changes.
 **/
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
/**
 * Diff type for tracking inverseDesign changes.
 **/
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

/**
 * Adds a PieceToDesignDiff element.
 **/
export const addPieceToDesignDiff = (designDiff: any, piece: Piece): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), piece],
    },
  };
};
/**
 * Replaces an existing PieceInDesignDiff element.
 **/
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

/**
 * Removes a PieceFromDesignDiff element.
 **/
export const removePieceFromDesignDiff = (designDiff: any, pieceId: string): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      removed: [...(designDiff.pieces?.removed || []), pieceId],
    },
  };
};

/**
 * Adds a PiecesToDesignDiff element.
 **/
export const addPiecesToDesignDiff = (designDiff: any, pieces: Piece[]): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), ...pieces],
    },
  };
};
/**
 * Replaces an existing PiecesInDesignDiff element.
 **/
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

/**
 * Removes a PiecesFromDesignDiff element.
 **/
export const removePiecesFromDesignDiff = (designDiff: any, pieceIds: string[]): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      removed: [...(designDiff.pieces?.removed || []), ...pieceIds],
    },
  };
};

/**
 * Adds a ConnectionToDesignDiff element.
 **/
export const addConnectionToDesignDiff = (designDiff: any, connection: Connection): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      added: [...(designDiff.connections?.added || []), connection],
    },
  };
};
/**
 * Replaces an existing ConnectionInDesignDiff element.
 **/
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
/**
 * Removes a ConnectionFromDesignDiff element.
 **/
export const removeConnectionFromDesignDiff = (designDiff: any, connectionId: { connected: { piece: string }; connecting: { piece: string } }): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), connectionId],
    },
  };
};

/**
 * Adds a ConnectionsToDesignDiff element.
 **/
export const addConnectionsToDesignDiff = (designDiff: any, connections: Connection[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      added: [...(designDiff.connections?.added || []), ...connections],
    },
  };
};
/**
 * Replaces an existing ConnectionsInDesignDiff element.
 **/
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
/**
 * Removes a ConnectionsFromDesignDiff element.
 **/
export const removeConnectionsFromDesignDiff = (designDiff: any, connectionIds: string[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), ...connectionIds],
    },
  };
};

/**
 * Diff type for tracking applyDesign changes.
 **/
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

/**
 * Creates a mixed design for visualization, annotating entities with diff status.
 * Annotate each with a semio.diffStatus attribute (unchanged/modified/removed/added).
 * Updated pieces apply non-geometric diff fields but KEEP base plane and center so
 * they render in their original location and only change color. Updated connections
 * apply the full diff. Removed entities are kept in place marked as removed.
 * Added entities are appended marked as added.
 **/
export const designWithDiff = (base: Design, diff: DesignDiff): Design => {
  const DIFF_STATUS_KEY = "semio.diffStatus";
  const setStatus = (attrs: Attribute[] | undefined, status: DiffStatus): Attribute[] => {
    const result = [...(attrs ?? [])];
    result.push({ guid: `${DIFF_STATUS_KEY}.${status}`, key: DIFF_STATUS_KEY, value: status });
    return result;
  };

  const removedPieceGuids = new Set((diff.pieces?.removed ?? []).map((r) => r.guid));
  const updatedPieceMap = new Map((diff.pieces?.updated ?? []).map((u) => [(u as any).piece.guid, u.diff]));
  const removedConnGuids = new Set((diff.connections?.removed ?? []).map((r) => r.guid));
  const updatedConnMap = new Map((diff.connections?.updated ?? []).map((u) => [(u as any).connection.guid, u.diff]));

  const resultPieces: Piece[] = (base.pieces ?? []).map((p) => {
    if (removedPieceGuids.has(p.guid)) return { ...p, attributes: setStatus(p.attributes, DiffStatus.Removed) };
    if (updatedPieceMap.has(p.guid)) {
      const applied = applyPieceDiff(p, updatedPieceMap.get(p.guid)!);
      // 📌Preserve base geometry so modified pieces stay in place and only get recolored.
      const preserved: Piece = { ...applied };
      if (p.plane !== undefined) preserved.plane = p.plane;
      else delete preserved.plane;
      if (p.center !== undefined) preserved.center = p.center;
      else delete preserved.center;
      return { ...preserved, attributes: setStatus(preserved.attributes, DiffStatus.Modified) };
    }
    return { ...p, attributes: setStatus(p.attributes, DiffStatus.Unchanged) };
  });
  for (const added of diff.pieces?.added ?? []) {
    resultPieces.push({ ...added, attributes: setStatus(added.attributes, DiffStatus.Added) });
  }

  const resultConns: Connection[] = (base.connections ?? []).map((c) => {
    if (removedConnGuids.has(c.guid)) return { ...c, attributes: setStatus(c.attributes, DiffStatus.Removed) };
    if (updatedConnMap.has(c.guid)) {
      const applied = applyConnectionDiff(c, updatedConnMap.get(c.guid)!);
      return { ...applied, attributes: setStatus(applied.attributes, DiffStatus.Modified) };
    }
    return { ...c, attributes: setStatus(c.attributes, DiffStatus.Unchanged) };
  });
  for (const added of diff.connections?.added ?? []) {
    resultConns.push({ ...added, attributes: setStatus(added.attributes, DiffStatus.Added) });
  }

  const result: Design = { ...base };
  result.pieces = resultPieces;
  result.connections = resultConns;
  return result;
};

/**
 * Zod schema for Designs diff validation.
 **/
export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(),
  added: z.array(DesignSchema).optional(),
});
/**
 * Diff type for tracking Designs changes.
 **/
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;

/**
 **/
export const mergeDesigns = (designs: Design[]): DesignDiff => {
  const pieces = designs.flatMap((d) => d.pieces ?? []);
  const connections = designs.flatMap((d) => d.connections ?? []);

  return {
    pieces: pieces.length > 0 ? { added: pieces } : undefined,
    connections: connections.length > 0 ? { added: connections } : undefined,
  };
};

/**
 **/
export const orientDesign = (plane?: Plane, center?: Coord): DesignDiff => {
  if (plane === undefined && center === undefined) {
    return {};
  }

  return {};
};

/**
 * Deletes pieces and connections from a design, returning a DesignDiff.
 * Removes stale connections referencing deleted pieces.
 * Updates pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
 **/
export const deletePiecesAndConnectionsInDesign = (kit: Kit, design: Design, pieceGuids: string[], connectionGuids: string[]): DesignDiffOperationResult => {
  const deletedPieceSet = new Set(pieceGuids);
  const connections = design.connections ?? [];

  // 🔌Find stale connections: connections referencing any deleted piece
  const staleConnectionGuids = new Set<string>();
  for (const conn of connections) {
    if (deletedPieceSet.has(conn.connected.piece.guid) || deletedPieceSet.has(conn.connecting.piece.guid)) {
      staleConnectionGuids.add(conn.guid);
    }
  }

  // 🚚All removed connections = explicit + stale
  const allRemovedConnectionGuids = new Set([...connectionGuids, ...staleConnectionGuids]);

  // 🔧Find pieces that become fixed
  const fixedPieceGuids: string[] = [];
  for (const connGuid of allRemovedConnectionGuids) {
    const conn = connections.find((c) => c.guid === connGuid);
    if (!conn) continue;
    const connectingGuid = conn.connecting.piece.guid;
    if (deletedPieceSet.has(connectingGuid)) continue;
    // ➖Check if this piece has another parent connection not in the removed set
    const hasOtherParent = connections.some((c) => c.connecting.piece.guid === connectingGuid && !allRemovedConnectionGuids.has(c.guid));
    if (!hasOtherParent && !fixedPieceGuids.includes(connectingGuid)) {
      fixedPieceGuids.push(connectingGuid);
    }
  }

  // ♻️Flatten the design to get absolute plane and center for each piece
  const flatRes = flattenDesign(kit, design.guid);
  if (!flatRes.ok) {
    return { ok: false, errors: flatRes.errors };
  }
  const flatChange = flatRes.change;
  const flatPieceMap: { [guid: string]: { plane?: Plane; center?: Coord } } = {};
  for (const piece of design.pieces ?? []) {
    if (piece.plane) flatPieceMap[piece.guid] = { plane: piece.plane, center: piece.center };
  }
  for (const update of flatChange.forward.pieces?.updated ?? []) {
    const existing = flatPieceMap[update.piece.guid] ?? {};
    if (update.diff.plane) existing.plane = update.diff.plane as Plane;
    if (update.diff.center) existing.center = update.diff.center as Coord;
    flatPieceMap[update.piece.guid] = existing;
  }

  const identityPlane: Plane = { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
  const zeroCenter: Coord = { u: 0, v: 0 };

  const diff: DesignDiff = {};

  const piecesRemoved = pieceGuids.map((guid) => ({ guid }));
  const piecesUpdated = fixedPieceGuids.map((guid) => {
    const flat = flatPieceMap[guid];
    return {
      piece: { guid },
      diff: { plane: flat?.plane ?? identityPlane, center: flat?.center ?? zeroCenter },
    };
  });
  if (piecesRemoved.length > 0 || piecesUpdated.length > 0) {
    diff.pieces = {};
    if (piecesRemoved.length > 0) diff.pieces.removed = piecesRemoved;
    if (piecesUpdated.length > 0) diff.pieces.updated = piecesUpdated;
  }

  const connectionsRemoved = [...allRemovedConnectionGuids].sort().map((guid) => ({ guid }));
  if (connectionsRemoved.length > 0) {
    diff.connections = { removed: connectionsRemoved };
  }

  return operationOk(diff, flatRes.warnings, flatRes.infos);
};

/**
 * Removes a PiecesAndConnectionsFromDesign element.
 **/
export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: string, pieceIds: string[], connectionIds: string[]): DesignOperationResult => {
  const design = findDesignInKit(kit, designId);
  const delRes = deletePiecesAndConnectionsInDesign(kit, design, pieceIds, connectionIds);
  if (!delRes.ok) {
    return { ok: false, errors: delRes.errors };
  }
  const backward = inverseDesignDiff(design, delRes.change);
  return operationOk({ forward: delRes.change, backward }, delRes.warnings, delRes.infos);
};

/**
 * Resolves {@link Type} and {@link Connector} from a kit the same way {@link flattenDesign} does.
 * Specs: Used when move needs parent connector frames from kit types.
 **/
const buildConnectorResolverFromKit = (kit: Kit): { getType: (typeGuid: string) => Type | undefined; getConnector: (type: Type | undefined, connectorGuid: string | undefined) => Connector | undefined } => {
  const typesDict: { [key: string]: Type } = {};
  (kit.types ?? []).forEach((t) => {
    typesDict[t.guid] = t;
  });
  const getType = (typeGuid: string): Type | undefined => typesDict[typeGuid];
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
  return { getType, getConnector };
};

/**
 * Parent-connector rotation and unit world axes for gap (local +Y), shift (+X), rise (+Z) before child orientation, matching {@link computeChildPlane}.
 **/
const connectionPlacementTranslationBasis = (parentConnector: Connector): { gap: THREE.Vector3; shift: THREE.Vector3; raise: THREE.Vector3; parentRotationT: THREE.Matrix4 } => {
  const parentDirection = vectorToThree(parentConnector.direction).normalize();
  const yAxis = new THREE.Vector3(0, 1, 0);
  const parentConnectorQuat = new THREE.Quaternion().setFromUnitVectors(yAxis, parentDirection);
  const parentRotationT = new THREE.Matrix4().makeRotationFromQuaternion(parentConnectorQuat);
  const gapDirection = new THREE.Vector3(0, 1, 0).applyMatrix4(parentRotationT).normalize();
  const shiftDirection = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT).normalize();
  const raiseDirection = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT).normalize();
  return { gap: gapDirection, shift: shiftDirection, raise: raiseDirection, parentRotationT };
};

// ◻️computeChildPlane computes a child plane from parent plane and connection parameters.
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

  const { gap: gapDirection, shift: shiftDirection, raise: raiseDirection, parentRotationT } = connectionPlacementTranslationBasis(parentConnector);
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
/**
 * Flattens nested Design structure.
 **/
export const flattenDesign = (kit: Kit, designId: string): DesignOperationResult => {
  const design = findDesignInKit(kit, designId);
  if (!design) {
    return operationErr([{ code: "flatten.design-not-found", message: `Design ${designId} not found in kit ${kit.name}` }]);
  }

  if (!design.pieces || design.pieces.length === 0) {
    return operationOk({ forward: {}, backward: {} }, [], [{ code: "flatten.empty-pieces", message: "No pieces to flatten; returning empty forward and backward diffs." }]);
  }

  const warnings: OperationNote[] = [];
  const infos: OperationNote[] = [];
  const placementErrors: OperationNote[] = [];

  const { getType, getConnector } = buildConnectorResolverFromKit(kit);

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
        warnings.push({
          code: "flatten.connection-skipped-missing-endpoint",
          message: `Skipping connection ${connection.guid}: source piece ${sourceId} not found in design.`,
        });
        return false;
      }
      if (!targetExists) {
        warnings.push({
          code: "flatten.connection-skipped-missing-endpoint",
          message: `Skipping connection ${connection.guid}: target piece ${targetId} not found in design.`,
        });
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
    const roots = component.nodes().filter((node) => {
      const piece = pieceMap[node.id()];
      return piece?.plane !== undefined && piece?.center !== undefined;
    });
    let rootNode = roots.length > 0 ? roots[0] : component.nodes().length > 0 ? component.nodes()[0] : undefined;
    if (!rootNode) return;
    if (roots.length === 0) {
      warnings.push({
        code: "flatten.no-fixed-piece-in-clump",
        message: `Connected pieces have no fixed root (no piece with both plane and center). Using piece ${rootNode.id()} as breadth-first root. Each connected set of pieces (clump) should include at least one fixed piece for stable, recommended layout.`,
      });
    } else if (roots.length > 1) {
      infos.push({
        code: "flatten.multiple-fixed-roots",
        message: `This clump has ${roots.length} fixed pieces; using the first (${rootNode.id()}) as breadth-first root.`,
      });
    }
    const rootPiece = pieceMap[rootNode.id()];
    if (!rootPiece || !rootPiece.guid) return;
    const updatedRootPiece = setAttributes(rootPiece, [
      { key: "semio.fixedPieceId", value: rootPiece.guid },
      { key: "semio.depth", value: "0" },
      { key: "semio.path", value: rootPiece.guid },
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

      // Keep the computed root plane/center in `pieceMap` as well.
      // Later we overwrite `flatDesign.pieces` from `pieceMap`, so without this
      // root-piece plane/center would be lost and `SemioDesign` would render
      // as diagram-only.
      pieceMap[rootNode.id()] = {
        ...(pieceMap[rootNode.id()] ?? updatedRootPiece),
        plane: rootPlane,
        center: flatDesign.pieces![rootPieceIndex].center,
      };
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
          placementErrors.push({
            code: "flatten.parent-plane-missing",
            message: `Parent piece ${parentPiece.guid} has no plane while flattening edge to child ${childPiece.guid}.`,
          });
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
          placementErrors.push({
            code: "flatten.connectors-not-found",
            message: `Connectors not found for connection between ${parentId} and ${childId}. Parent connector: ${parentConnectorGuid ?? "(default)"}, child connector: ${childConnectorGuid ?? "(default)"}.`,
          });
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

        const computedChildCenter = {
          u: round(childU),
          v: round(childV),
        };
        const childCenter = childPiece.center ?? computedChildCenter;

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
            {
              key: "semio.path",
              value: (parentPiece.attributes?.find((q) => q.key === "semio.path")?.value ?? "") + "," + childPiece.guid,
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

  const removedConnections = design.connections?.map((c) => ({ guid: c.guid })) || [];

  const forward = {
    pieces: updatedPieces.length > 0 ? { updated: updatedPieces } : undefined,
    connections: removedConnections.length > 0 ? { removed: removedConnections } : undefined,
  } as DesignDiff;

  if (piecesWithoutPlanes > 0) {
    placementErrors.push({
      code: "flatten.piece-missing-plane",
      message: `After flatten, ${piecesWithoutPlanes} piece(s) still have no plane (see prior placement messages).`,
    });
  }
  if (placementErrors.length > 0) {
    return operationErr(placementErrors);
  }

  infos.push({
    code: "flatten.summary",
    message: `Flatten removed ${removedConnections.length} connection(s); updated ${updatedPieces.length} piece record(s); ${piecesWithPlanes} piece(s) with planes.`,
  });

  const backward = inverseDesignDiff(design, forward);
  return operationOk({ forward, backward }, warnings, infos);
};

/**
 **/
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

/**
 **/
export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds: string[], clusteredDesign: Design, externalConnections: Connection[]): DesignChange => {
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
          designPiece: { guid: clusteredDesign.guid },
        },
      };
    } else if (connectingInCluster) {
      return {
        ...connection,
        connecting: {
          ...connection.connecting,
          designPiece: { guid: clusteredDesign.guid },
        },
      };
    }

    return connection;
  });

  const forward: DesignDiff = {
    pieces: {
      removed: piecesToRemove,
    },
    connections: {
      removed: connectionsToRemove,
      added: updatedExternalConnections,
    },
  };
  const backward = inverseDesignDiff(originalDesign, forward);
  return { forward, backward };
};

/**
 * Retrieves the ClusterableGroups value.
 **/
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

  const pieceGuidSet = new Set((design.pieces || []).map((piece) => piece.guid));
  const hasDesignNodes = selectedPieceIds.some((id) => !pieceGuidSet.has(id));
  const hasMultipleComponents = connectedGroups.length > 1;
  const hasLargeConnectedGroup = connectedGroups.some((group) => group.length > 1);

  if (hasDesignNodes || hasMultipleComponents || hasLargeConnectedGroup) {
    return [selectedPieceIds];
  }

  return [];
};

/**
 **/
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

/**
 * Type alias for IncludedDesignInfo.
 **/
export type IncludedDesignInfo = {
  guid: string;
  designGuid: string;
  type: "connected" | "fixed";
  center?: Coord;
  plane?: Plane;
  externalConnections?: Connection[];
};

/**
 * Retrieves the IncludedDesigns value.
 **/
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
      guid: designIdString,
      designGuid: designIdString,
      type: "connected",
      externalConnections,
    });
  });

  return includedDesigns;
};

/**
 **/
export const isPortInUse = (design: Design, pieceGuid: string, connectorGuid: string): boolean => {
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.guid === pieceGuid;
    const isPortConnected = isPieceConnected ? connection.connected.connector?.guid === connectorGuid : connection.connecting.connector?.guid === connectorGuid;
    if (isPortConnected) return true;
  }
  return false;
};

/**
 **/
export const isConnectionInDesign = (design: Design, connection: Connection): boolean => {
  return design.connections?.some((c) => areSameConnection(c, connection)) ?? false;
};

/**
 * Searches for matching PieceInDesign entry.
 **/
export const findPieceInDesign = (design: Design, pieceGuid: string): Piece => findPiece(design.pieces ?? [], pieceGuid);

/**
 * Searches for matching ConnectionInDesign entry.
 **/
export const findConnectionInDesign = (design: Design, connectionGuid: string): Connection => {
  return findConnection(design.connections ?? [], connectionGuid);
};

/**
 * Searches for matching ConnectionsInDesign entry.
 **/
export const findConnectionsInDesign = (design: Design, connectionGuids: string[]): Connection[] => {
  return connectionGuids.map((connectionGuid) => findConnectionInDesign(design, connectionGuid));
};

/**
 * Searches for matching PieceConnectionsInDesign entry.
 **/
export const findPieceConnectionsInDesign = (design: Design, pieceGuid: string): Connection[] => {
  return findPieceConnections(design.connections ?? [], pieceGuid);
};

/**
 * Searches for matching ConnectionPiecesInDesign entry.
 **/
export const findConnectionPiecesInDesign = (design: Design, connection: Connection): { connecting: Piece; connected: Piece } => {
  return {
    connected: findPieceInDesign(design, connection.connected.piece.guid),
    connecting: findPieceInDesign(design, connection.connecting.piece.guid),
  };
};

/**
 * Searches for matching StaleConnectionsInDesign entry.
 **/
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

/**
 * Computes a DesignDiff that offsets selected piece centers and adjusts orphan connections.
 * A piece's parent connection is the connection where it is the connecting (child) piece.
 **/
/**
 * Placement deltas in the **selected piece plane** frame: gap along yAxis, shift along xAxis, rise along the plane normal.
 * For connected pieces, {@link movePiecesInDesign} maps that translation into connection deltas for gap, shift, rise, rotation, turn, tilt (see {@link computeChildPlane} Jacobian step), then any leftover translation into u/v on the parent plane.
 **/
export type MoveVector = { gap: number; shift: number; rise: number };

// #region 🔖DragMoveStructuralSelection
/**
 * Shared parent graph and fixed/selection sets for {@link dragPiecesInDesign} and {@link movePiecesInDesign}.
 * Specs: "Fixed" pieces are selected pieces that never appear as the connecting (child) side of a connection.
 **/
const buildDragMoveStructuralContext = (
  design: Design,
  pieces: Design,
): {
  selectedGuids: Set<string>;
  parentMap: Map<string, { connectionGuid: string; parentGuid: string }>;
  pieceMap: Map<string, Piece>;
  fixedGuids: Set<string>;
} => {
  const selectedGuids = new Set((pieces.pieces ?? []).map((p) => p.guid));
  const parentMap = new Map<string, { connectionGuid: string; parentGuid: string }>();
  for (const c of design.connections ?? []) {
    parentMap.set(c.connecting.piece.guid, { connectionGuid: c.guid, parentGuid: c.connected.piece.guid });
  }
  const pieceMap = new Map<string, Piece>();
  for (const p of design.pieces ?? []) {
    pieceMap.set(p.guid, p);
  }
  const fixedGuids = new Set<string>();
  for (const guid of selectedGuids) {
    if (!parentMap.has(guid)) fixedGuids.add(guid);
  }
  return { selectedGuids, parentMap, pieceMap, fixedGuids };
};

/**
 * True when walking parent links finds a selected ancestor (same descendant suppression as drag).
 **/
const pieceHasSelectedAncestorInDragMoveTree = (pieceGuid: string, selectedGuids: Set<string>, parentMap: Map<string, { connectionGuid: string; parentGuid: string }>): boolean => {
  let current = pieceGuid;
  while (parentMap.has(current)) {
    const ancestor = parentMap.get(current)!.parentGuid;
    if (selectedGuids.has(ancestor)) return true;
    current = ancestor;
  }
  return false;
};
// #endregion

/**
 * World-space translation from a piece plane and placement vector (matches connection gap/shift/rise axes).
 **/
export const moveTranslationWorldFromPiecePlane = (plane: Plane, vector: MoveVector): Point => {
  const x = vectorToThree(plane.xAxis).normalize();
  const y = vectorToThree(plane.yAxis).normalize();
  const z = new THREE.Vector3().crossVectors(x, y);
  if (z.lengthSq() < 1e-12) {
    return { x: 0, y: 0, z: 0 };
  }
  z.normalize();
  const t = new THREE.Vector3().addScaledVector(y, vector.gap).addScaledVector(x, vector.shift).addScaledVector(z, vector.rise);
  return { x: t.x, y: t.y, z: t.z };
};

const identityPlaneForStructuralMove = (): Plane => ({
  origin: { x: 0, y: 0, z: 0 },
  xAxis: { x: 1, y: 0, z: 0 },
  yAxis: { x: 0, y: 1, z: 0 },
});

type ConnectionPlacementNumericKey = "gap" | "shift" | "rise" | "rotation" | "turn" | "tilt";

const CONNECTION_MOVE_JACOBIAN_KEYS: readonly ConnectionPlacementNumericKey[] = ["gap", "shift", "rise", "rotation", "turn", "tilt"];

const CONNECTION_MOVE_JACOBIAN_EPS: Record<ConnectionPlacementNumericKey, number> = {
  gap: 1e-6,
  shift: 1e-6,
  rise: 1e-6,
  rotation: 1e-4,
  turn: 1e-4,
  tilt: 1e-4,
};

const childConnectorOriginWorld = (parentPlane: Plane, parentConnector: Connector, childConnector: Connector, connection: Connection): THREE.Vector3 => {
  const plane = computeChildPlane(parentPlane, parentConnector, childConnector, connection);
  return vectorToThree(plane.origin);
};

/**
 * Minimum-norm δ with Jδ = t for 3×n Jacobian J whose columns are cols[i] = ∂origin/��param_i; δ = J��(JJ��)⁻¹t.
 **/
const solveConnectionOriginMinNorm = (cols: THREE.Vector3[], t: THREE.Vector3): number[] | undefined => {
  if (cols.length === 0) return undefined;
  const jjt = new THREE.Matrix3();
  for (let c = 0; c < 3; c++) {
    for (let r = 0; r < 3; r++) {
      let s = 0;
      for (const col of cols) s += col.getComponent(r) * col.getComponent(c);
      jjt.elements[r + c * 3] = s;
    }
  }
  jjt.elements[0] += 1e-14;
  jjt.elements[4] += 1e-14;
  jjt.elements[8] += 1e-14;
  if (Math.abs(jjt.determinant()) < 1e-22) return undefined;
  const inv = new THREE.Matrix3().copy(jjt).invert();
  if (!Number.isFinite(inv.elements[0])) return undefined;
  const u = t.clone().applyMatrix3(inv);
  return cols.map((col) => col.dot(u));
};

const connectionNumericAt = (connection: Connection, key: ConnectionPlacementNumericKey): number => {
  const v = connection[key];
  return v !== undefined && v !== null ? v : 0;
};

const connectionWithNumericDelta = (connection: Connection, key: ConnectionPlacementNumericKey, delta: number): Connection => {
  return { ...connection, [key]: connectionNumericAt(connection, key) + delta };
};

/**
 * Fallback when Jacobian is unavailable: project translation onto connector gap/shift/rise only, then u/v on parent plane.
 **/
const connectionDiffTranslationFallback = (parentPlane: Plane, parentConnector: Connector, t: THREE.Vector3): ConnectionDiff => {
  const { gap: g, shift: s, raise: r } = connectionPlacementTranslationBasis(parentConnector);
  const dgap = t.dot(g);
  const dshift = t.dot(s);
  const drise = t.dot(r);
  const res = t.clone().addScaledVector(g, -dgap).addScaledVector(s, -dshift).addScaledVector(r, -drise);
  const px = vectorToThree(parentPlane.xAxis);
  const py = vectorToThree(parentPlane.yAxis);
  const diff: ConnectionDiff = {};
  const eps = 1e-9;
  if (Math.abs(dgap) > eps) diff.gap = dgap;
  if (Math.abs(dshift) > eps) diff.shift = dshift;
  if (Math.abs(drise) > eps) diff.rise = drise;
  if (px.lengthSq() > 1e-24 && py.lengthSq() > 1e-24) {
    const pxN = px.clone().normalize();
    const pyN = py.clone().normalize();
    const du = res.dot(pxN);
    const dv = res.dot(pyN);
    if (Math.abs(du) > eps) diff.u = du;
    if (Math.abs(dv) > eps) diff.v = dv;
  }
  return diff;
};

/**
 * Converts a move vector (connecting piece plane) into connection diffs using a numerical Jacobian of {@link computeChildPlane}
 * w.r.t. gap, shift, rise, rotation, turn, tilt (degrees for angles), then puts the remaining translation into u/v on the parent plane.
 * Specs: One Gauss–Newton step; matches flatten placement when child connector exists. Falls back to translation-only basis if singular.
 **/
const connectionDiffFromStructuralMoveVector = (parentPlane: Plane, parentConnector: Connector, childConnector: Connector | undefined, connection: Connection, childPlane: Plane | undefined, vector: MoveVector): ConnectionDiff => {
  const child = childPlane ?? identityPlaneForStructuralMove();
  const tw = moveTranslationWorldFromPiecePlane(child, vector);
  const t = vectorToThree(tw);
  if (t.lengthSq() < 1e-24) return {};

  if (!childConnector) {
    return connectionDiffTranslationFallback(parentPlane, parentConnector, t);
  }

  const o0 = childConnectorOriginWorld(parentPlane, parentConnector, childConnector, connection);
  const cols: THREE.Vector3[] = [];
  for (const key of CONNECTION_MOVE_JACOBIAN_KEYS) {
    const eps = CONNECTION_MOVE_JACOBIAN_EPS[key];
    const perturbed = connectionWithNumericDelta(connection, key, eps);
    const o1 = childConnectorOriginWorld(parentPlane, parentConnector, childConnector, perturbed);
    cols.push(o1.clone().sub(o0).divideScalar(eps));
  }

  const deltas = solveConnectionOriginMinNorm(cols, t);
  const diff: ConnectionDiff = {};
  const epsOut = 1e-9;
  if (deltas) {
    CONNECTION_MOVE_JACOBIAN_KEYS.forEach((key, i) => {
      if (Math.abs(deltas[i]) > epsOut) diff[key] = deltas[i];
    });
    const pred = new THREE.Vector3();
    cols.forEach((col, i) => pred.addScaledVector(col, deltas[i]));
    const res = t.clone().sub(pred);
    const px = vectorToThree(parentPlane.xAxis);
    const py = vectorToThree(parentPlane.yAxis);
    if (px.lengthSq() > 1e-24 && py.lengthSq() > 1e-24) {
      const pxN = px.clone().normalize();
      const pyN = py.clone().normalize();
      const du = res.dot(pxN);
      const dv = res.dot(pyN);
      if (Math.abs(du) > epsOut) diff.u = du;
      if (Math.abs(dv) > epsOut) diff.v = dv;
    }
    return diff;
  }

  return connectionDiffTranslationFallback(parentPlane, parentConnector, t);
};

/**
 * Like {@link dragPiecesInDesign}: same fixed vs connected selection and descendant suppression.
 * Root movers get plane origin translation from {@link moveTranslationWorldFromPiecePlane}.
 * Connected movers need {@link buildConnectorResolverFromKit}: world delta from the child plane is split across * gap, shift, rise, rotation, turn, tilt (via Jacobian of {@link computeChildPlane}) and residual u/v on the parent plane.
 **/
export const movePiecesInDesign = (kit: Kit, design: Design, pieces: Design, vector: MoveVector): DesignDiff => {
  const { getType, getConnector } = buildConnectorResolverFromKit(kit);
  const { selectedGuids, parentMap, pieceMap, fixedGuids } = buildDragMoveStructuralContext(design, pieces);
  const pieceUpdates: { piece: { guid: string }; diff: PieceDiff }[] = [];
  for (const guid of fixedGuids) {
    const base = pieceMap.get(guid)?.plane;
    if (base === undefined) continue;
    const t = moveTranslationWorldFromPiecePlane(base, vector);
    const newPlane: Plane = {
      origin: { x: base.origin.x + t.x, y: base.origin.y + t.y, z: base.origin.z + t.z },
      xAxis: { ...base.xAxis },
      yAxis: { ...base.yAxis },
    };
    pieceUpdates.push({ piece: { guid }, diff: { plane: newPlane } });
  }
  const connectionUpdates: { connection: { guid: string }; diff: ConnectionDiff }[] = [];
  for (const guid of selectedGuids) {
    if (fixedGuids.has(guid)) continue;
    if (pieceHasSelectedAncestorInDragMoveTree(guid, selectedGuids, parentMap)) continue;
    const parent = parentMap.get(guid);
    if (!parent) continue;
    const connection = design.connections?.find((c) => c.guid === parent.connectionGuid);
    if (!connection) continue;
    const parentPiece = pieceMap.get(parent.parentGuid);
    const childPiece = pieceMap.get(guid);
    if (!parentPiece?.type?.guid || !childPiece?.type?.guid) continue;
    const parentType = getType(parentPiece.type.guid);
    const childType = getType(childPiece.type.guid);
    const parentConnector = getConnector(parentType, connection.connected.connector?.guid);
    const childConnector = getConnector(childType, connection.connecting.connector?.guid);
    if (!parentConnector) continue;
    const parentPlane = parentPiece.plane ?? identityPlaneForStructuralMove();
    const connDiff = connectionDiffFromStructuralMoveVector(parentPlane, parentConnector, childConnector, connection, childPiece.plane, vector);
    if (Object.keys(connDiff).length === 0) continue;
    connectionUpdates.push({ connection: { guid: parent.connectionGuid }, diff: connDiff });
  }
  const diff: DesignDiff = {};
  if (pieceUpdates.length > 0) diff.pieces = { updated: pieceUpdates };
  if (connectionUpdates.length > 0) diff.connections = { updated: connectionUpdates };
  return diff;
};

export const dragPiecesInDesign = (design: Design, pieces: Design, offset: Coord): DesignDiff => {
  const { selectedGuids, parentMap, pieceMap, fixedGuids } = buildDragMoveStructuralContext(design, pieces);
  const pieceUpdates: { piece: { guid: string }; diff: PieceDiff }[] = [];
  for (const guid of fixedGuids) {
    const currentCenter = pieceMap.get(guid)?.center;
    if (currentCenter !== undefined) {
      pieceUpdates.push({ piece: { guid }, diff: { center: { u: currentCenter.u + offset.u, v: currentCenter.v + offset.v } } });
    }
  }
  const connectionUpdates: { connection: { guid: string }; diff: ConnectionDiff }[] = [];
  for (const guid of selectedGuids) {
    if (fixedGuids.has(guid)) continue;
    if (pieceHasSelectedAncestorInDragMoveTree(guid, selectedGuids, parentMap)) continue;
    const parent = parentMap.get(guid);
    if (!parent) continue;
    connectionUpdates.push({ connection: { guid: parent.connectionGuid }, diff: { u: offset.u, v: offset.v } });
  }
  const diff: DesignDiff = {};
  if (pieceUpdates.length > 0) diff.pieces = { updated: pieceUpdates };
  if (connectionUpdates.length > 0) diff.connections = { updated: connectionUpdates };
  return diff;
};

/**
 * 📋Extracts selected pieces and connections from a design into a new Design (clipboard).
 * Specs: Selected pieces are classified as internal-fixed, internal-connected, or parent-piece-exclusive parent-connection-inclusive.
 * Internal pieces are copied as-is. Pp-excl-pc-incl pieces get semio.center and semio.plane attributes.
 * Non-internal connections include their external pieces marked with semio.piece.origin = "external".
 **/
export const copyDesign = (kit: Kit, design: Design, pieceGuids: string[], connectionGuids: string[]): OperationResult<Design> => {
  const selectedPieceSet = new Set(pieceGuids);
  const selectedConnectionSet = new Set(connectionGuids);

  const kitDesign = design.guid ? findDesignInKit(kit, design.guid) : undefined;
  const connections = design.connections && design.connections.length > 0 ? design.connections : (kitDesign?.connections ?? []);
  const pieces = design.pieces ?? [];

  // Build parent map: child guid -> { parentGuid, connection }
  const parentMap = new Map<string, { parentGuid: string; connection: Connection }>();
  // Build child map: parent guid -> [{ childGuid, connection }, ...]
  const childMap = new Map<string, Array<{ childGuid: string; connection: Connection }>>();
  for (const conn of connections) {
    parentMap.set(conn.connecting.piece.guid, { parentGuid: conn.connected.piece.guid, connection: conn });
    const parentGuid = conn.connected.piece.guid;
    if (!childMap.has(parentGuid)) childMap.set(parentGuid, []);
    childMap.get(parentGuid)!.push({ childGuid: conn.connecting.piece.guid, connection: conn });
  }

  // Flatten the design to get absolute planes/centers
  const flatRes = flattenDesign(kit, design.guid);
  if (!flatRes.ok) {
    return { ok: false, errors: flatRes.errors };
  }
  const flatChange = flatRes.change;
  const flatDesign = applyDesignDiff(JSON.parse(JSON.stringify(design)), flatChange.forward);
  const flatPieceMap = new Map<string, Piece>();
  for (const p of flatDesign.pieces ?? []) {
    flatPieceMap.set(p.guid, p);
  }

  const copyPieces: Piece[] = [];
  const addedPieceGuids = new Set<string>();
  const copyConnections: Connection[] = [];

  // Process selected pieces
  for (const pieceGuid of pieceGuids) {
    const piece = pieces.find((p) => p.guid === pieceGuid);
    if (!piece) continue;

    const isFixed = piece.plane !== undefined;
    const pInfo = parentMap.get(pieceGuid);
    const isConnected = pInfo !== undefined;

    let isInternalConnected = false;
    const isInternalFixed = isFixed && selectedPieceSet.has(pieceGuid);
    let isPpExclPcIncl = false;

    if (isConnected && pInfo) {
      const parentPieceSelected = selectedPieceSet.has(pInfo.parentGuid);
      const parentConnSelected = selectedConnectionSet.has(pInfo.connection.guid);
      isInternalConnected = parentPieceSelected && parentConnSelected;
      isPpExclPcIncl = !parentPieceSelected && parentConnSelected;
    }

    if (isInternalFixed || isInternalConnected) {
      copyPieces.push(JSON.parse(JSON.stringify(piece)));
      addedPieceGuids.add(pieceGuid);
    } else if (isPpExclPcIncl) {
      const copied: Piece = JSON.parse(JSON.stringify(piece));
      const flatPiece = flatPieceMap.get(pieceGuid);
      if (flatPiece) {
        const centerValue = flatPiece.center ? JSON.stringify(flatPiece.center) : JSON.stringify({ u: 0, v: 0 });
        const planeValue = flatPiece.plane ? JSON.stringify(flatPiece.plane) : JSON.stringify({ origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } });
        copied.attributes = [...(copied.attributes ?? []), { guid: "", key: "semio.center", value: centerValue }, { guid: "", key: "semio.plane", value: planeValue }];
      }
      copyPieces.push(copied);
      addedPieceGuids.add(pieceGuid);
    } else {
      // Specs: Selected piece without an internal parent edge (parent piece or parent connection unselected, and not pp-excl-pc-incl)
      // becomes a free fixed root in the clipboard at its flat absolute position. Its source descendant subtree (children
      // and their parent connections, recursively) is auto-pulled in unchanged so the subtree appears exactly as in the source.
      const copied: Piece = JSON.parse(JSON.stringify(piece));
      const flatPiece = flatPieceMap.get(pieceGuid);
      if (flatPiece) {
        if (flatPiece.center) copied.center = { u: flatPiece.center.u, v: flatPiece.center.v };
        if (flatPiece.plane) copied.plane = JSON.parse(JSON.stringify(flatPiece.plane));
        const centerValue = flatPiece.center ? JSON.stringify(flatPiece.center) : JSON.stringify({ u: 0, v: 0 });
        const planeValue = flatPiece.plane ? JSON.stringify(flatPiece.plane) : JSON.stringify({ origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } });
        copied.attributes = [...(copied.attributes ?? []), { guid: "", key: "semio.center", value: centerValue }, { guid: "", key: "semio.plane", value: planeValue }];
      }
      copyPieces.push(copied);
      addedPieceGuids.add(pieceGuid);

      const subtreeQueue: string[] = [pieceGuid];
      const subtreeVisited = new Set<string>([pieceGuid]);
      const addedConnGuids = new Set<string>(copyConnections.map((c) => c.guid));
      while (subtreeQueue.length > 0) {
        const cur = subtreeQueue.shift()!;
        const children = childMap.get(cur) ?? [];
        for (const { childGuid, connection } of children) {
          if (subtreeVisited.has(childGuid)) continue;
          subtreeVisited.add(childGuid);
          if (!addedPieceGuids.has(childGuid)) {
            const childPiece = pieces.find((p) => p.guid === childGuid);
            if (childPiece) {
              copyPieces.push(JSON.parse(JSON.stringify(childPiece)));
              addedPieceGuids.add(childGuid);
            }
          }
          if (!addedConnGuids.has(connection.guid)) {
            copyConnections.push(JSON.parse(JSON.stringify(connection)));
            addedConnGuids.add(connection.guid);
          }
          subtreeQueue.push(childGuid);
        }
      }
    }
  }

  // Process selected connections
  for (const connGuid of connectionGuids) {
    const conn = connections.find((c) => c.guid === connGuid);
    if (!conn) continue;

    const connectedGuid = conn.connected.piece.guid;
    const connectingGuid = conn.connecting.piece.guid;
    const connectedSelected = selectedPieceSet.has(connectedGuid);
    const connectingSelected = selectedPieceSet.has(connectingGuid);

    const isInternal = connectedSelected && connectingSelected;

    if (isInternal) {
      copyConnections.push(JSON.parse(JSON.stringify(conn)));
    } else {
      copyConnections.push(JSON.parse(JSON.stringify(conn)));

      const externalGuids: string[] = [];
      if (!connectedSelected) externalGuids.push(connectedGuid);
      if (!connectingSelected) externalGuids.push(connectingGuid);

      for (const extGuid of externalGuids) {
        if (!addedPieceGuids.has(extGuid)) {
          const extPiece = pieces.find((p) => p.guid === extGuid);
          if (extPiece) {
            const cloned: Piece = JSON.parse(JSON.stringify(extPiece));
            const extAttrs: Attribute[] = [...(cloned.attributes ?? []), { guid: "", key: "semio.piece.origin", value: "external" }];
            const flatExtPiece = flatPieceMap.get(extGuid);
            if (flatExtPiece) {
              const extCenterValue = flatExtPiece.center ? JSON.stringify(flatExtPiece.center) : JSON.stringify({ u: 0, v: 0 });
              extAttrs.push({ guid: "", key: "semio.center", value: extCenterValue });
            }
            cloned.attributes = extAttrs;
            copyPieces.push(cloned);
            addedPieceGuids.add(extGuid);
          }
        }
      }
    }
  }

  return operationOk({ guid: "", name: "", pieces: copyPieces, connections: copyConnections }, flatRes.warnings, [
    ...flatRes.infos,
    {
      code: "copy.summary",
      message: `Copied ${copyPieces.length} piece(s) and ${copyConnections.length} connection(s) to clipboard design.`,
    },
  ]);
};

/** Specs: Anchoring strings handled by `pasteDesign` switch; any other string falls through to the default branch (same offset as `original`). */
export const PASTE_DESIGN_ANCHORING_KINDS = ["original", "middle", "centroid", "bottomLeft", "bottomRight", "topLeft", "topRight"] as const;

export type PasteDesignAnchoringKind = (typeof PASTE_DESIGN_ANCHORING_KINDS)[number];

/**
 * 📋Pastes a copied design into a target design, returning a DesignDiff.
 * Specs: Anchoring determines the reference point within the bounding rectangle of the source.
 * External stub parents are remapped to matching target pieces (name + connector) when possible—even if the child
 * has a plane (flattened pp-excl). If rematch is impossible, fall back to center/plane from attributes then anchor/coord.
 * Other pieces with a plane alone get -anchor then +coord on diagram center.
 * Fully internal source connections keep cloned u/v when coord only affects stub-bridge remapping as above.
 * With coord, only the remapped child–stub parent bridge updates u/v: target matched parent’s diagram center minus
 * (coord + (anchor − child flat center)). Descendant internal connections keep deep-cloned u/v.
 **/

export const pasteDesign = (kit: Kit, source: Design, target: Design, anchoring: string = "bottomLeft", coord?: Coord): DesignDiff => {
  const typesMap = new Map<string, Type>();
  for (const t of kit.types ?? []) typesMap.set(t.guid, t);
  const portsMap = new Map<string, Port>();
  for (const p of kit.ports ?? []) portsMap.set(p.guid, p);

  const sourcePieces = source.pieces ?? [];
  const sourceConnections = source.connections ?? [];
  const targetPieces = target.pieces ?? [];

  // Classify source pieces
  const externalOriginGuids = new Set<string>();
  for (const piece of sourcePieces) {
    if ((piece.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")) {
      externalOriginGuids.add(piece.guid);
    }
  }

  const sourcePieceMap = new Map<string, Piece>();
  for (const p of sourcePieces) sourcePieceMap.set(p.guid, p);

  /** When the same child (`connecting`) appears in multiple clipboard edges, prefer the edge to a stub parent for rematch. */
  const sourceParentMap = new Map<string, { parentGuid: string; connection: Connection }>();
  for (const conn of sourceConnections) {
    const childGuid = conn.connecting.piece.guid;
    const parentGuid = conn.connected.piece.guid;
    const prev = sourceParentMap.get(childGuid);
    if (!prev) {
      sourceParentMap.set(childGuid, { parentGuid, connection: conn });
      continue;
    }
    const prevStub = externalOriginGuids.has(prev.parentGuid);
    const nextStub = externalOriginGuids.has(parentGuid);
    if (prevStub !== nextStub && nextStub) {
      sourceParentMap.set(childGuid, { parentGuid, connection: conn });
    }
  }

  // Compute bounding rectangle from flat centers
  const centerCoords: Coord[] = [];
  for (const piece of sourcePieces) {
    if (externalOriginGuids.has(piece.guid)) continue;
    let center: Coord | undefined = piece.center;
    if (!center) {
      const attr = (piece.attributes ?? []).find((a) => a.key === "semio.center");
      if (attr?.value) center = JSON.parse(attr.value) as Coord;
    }
    if (center) centerCoords.push(center);
  }
  if (centerCoords.length === 0) centerCoords.push({ u: 0, v: 0 });

  const minU = Math.min(...centerCoords.map((c) => c.u));
  const maxU = Math.max(...centerCoords.map((c) => c.u));
  const minV = Math.min(...centerCoords.map((c) => c.v));
  const maxV = Math.max(...centerCoords.map((c) => c.v));

  let anchor: Coord;
  switch (anchoring) {
    case "original":
      anchor = { u: 0, v: 0 };
      break;
    case "middle":
      anchor = { u: (minU + maxU) / 2, v: (minV + maxV) / 2 };
      break;
    case "centroid":
      anchor = { u: centerCoords.reduce((s, c) => s + c.u, 0) / centerCoords.length, v: centerCoords.reduce((s, c) => s + c.v, 0) / centerCoords.length };
      break;
    case "bottomLeft":
      anchor = { u: minU, v: minV };
      break;
    case "bottomRight":
      anchor = { u: maxU, v: minV };
      break;
    case "topLeft":
      anchor = { u: minU, v: maxV };
      break;
    case "topRight":
      anchor = { u: maxU, v: maxV };
      break;
    default:
      anchor = { u: 0, v: 0 };
      break;
  }

  // Build target piece maps for matching
  const targetPiecesByName = new Map<string, Piece[]>();
  for (const tp of targetPieces) {
    if (tp.name) {
      if (!targetPiecesByName.has(tp.name)) targetPiecesByName.set(tp.name, []);
      targetPiecesByName.get(tp.name)!.push(tp);
    }
  }

  // Helpers
  const arePortsCompatible = (pg1?: string, pg2?: string): boolean => {
    if (!pg1 || !pg2) return false;
    if (pg1 === pg2) return true;
    const p1 = portsMap.get(pg1);
    const p2 = portsMap.get(pg2);
    if (!p1 || !p2) return false;
    return (p1.compatiblePorts ?? []).some((cp) => cp.guid === pg2) || (p2.compatiblePorts ?? []).some((cp) => cp.guid === pg1);
  };

  const findMatchingConnector = (typeGuid: string, sourceConnector: Connector): Connector | undefined => {
    const t = typesMap.get(typeGuid);
    if (!t) return undefined;
    return (t.connectors ?? []).find((c) => {
      const nameMatch = (sourceConnector.name ?? "") !== "" && c.name === sourceConnector.name;
      const guidMatch = c.guid === sourceConnector.guid;
      if (!nameMatch && !guidMatch) return false;
      return arePortsCompatible(c.port?.guid, sourceConnector.port?.guid);
    });
  };

  /** True when the external stub parent can be replaced by a target piece with a compatible connector (remap path). */
  const canRematchExternalParentPiece = (piece: Piece, pInfo: { parentGuid: string; connection: Connection }): boolean => {
    if (!externalOriginGuids.has(pInfo.parentGuid)) return false;
    const externalParent = sourcePieceMap.get(pInfo.parentGuid);
    if (!externalParent) return false;
    const extName = externalParent.name ?? "";
    if (!extName || !targetPiecesByName.has(extName)) return false;
    const parentConn = pInfo.connection;
    const isParentConnected = parentConn.connected.piece.guid === pInfo.parentGuid;
    const parentConnectorGuid = isParentConnected ? parentConn.connected.connector?.guid : parentConn.connecting.connector?.guid;
    if (!parentConnectorGuid || !externalParent.type?.guid) return false;
    const parentType = typesMap.get(externalParent.type.guid);
    const sourceParentConnector = parentType?.connectors?.find((c) => c.guid === parentConnectorGuid);
    if (!sourceParentConnector) return false;
    const candidates = targetPiecesByName.get(extName)!;
    return candidates.some((candidate) => {
      if (!candidate.type?.guid) return false;
      return findMatchingConnector(candidate.type.guid, sourceParentConnector) !== undefined;
    });
  };

  const addedPieces: Piece[] = [];
  const addedConnections: Connection[] = [];

  // Process source pieces
  for (const piece of sourcePieces) {
    if (externalOriginGuids.has(piece.guid)) continue;

    const isFixed = piece.plane !== undefined;
    const pInfo = sourceParentMap.get(piece.guid);
    const isConnected = pInfo !== undefined;

    // Specs: If the parent is an external clipboard stub and the target has a matching named piece with a
    // compatible connector, remap the parent link first—even when this piece has a plane (flattened pp-excl).
    // Otherwise fixed pieces still get anchor/coord on center without going through rematch (e.g. parent not in target).
    if (isConnected && pInfo && externalOriginGuids.has(pInfo.parentGuid)) {
      const externalParent = sourcePieceMap.get(pInfo.parentGuid)!;
      let matched = false;

      if (canRematchExternalParentPiece(piece, pInfo)) {
        const extName = externalParent.name ?? "";
        const candidates = targetPiecesByName.get(extName)!;
        const parentConn = pInfo.connection;
        const isParentConnected = parentConn.connected.piece.guid === pInfo.parentGuid;
        const parentConnectorGuid = isParentConnected ? parentConn.connected.connector?.guid : parentConn.connecting.connector?.guid;

        let sourceParentConnector: Connector | undefined;
        if (externalParent.type?.guid) {
          const parentType = typesMap.get(externalParent.type.guid);
          if (parentType) {
            sourceParentConnector = (parentType.connectors ?? []).find((c) => c.guid === parentConnectorGuid);
          }
        }

        if (sourceParentConnector) {
          for (const candidate of candidates) {
            if (!candidate.type?.guid) continue;
            const matchingConnector = findMatchingConnector(candidate.type.guid, sourceParentConnector);
            if (matchingConnector) {
              matched = true;
              addedPieces.push(JSON.parse(JSON.stringify(piece)));

              const copiedConn: Connection = JSON.parse(JSON.stringify(parentConn));
              if (isParentConnected) {
                copiedConn.connected = { piece: { guid: candidate.guid }, connector: { guid: matchingConnector.guid } };
              } else {
                copiedConn.connecting = { piece: { guid: candidate.guid }, connector: { guid: matchingConnector.guid } };
              }

              if (coord) {
                const connectedStub = externalOriginGuids.has(parentConn.connected.piece.guid);
                const connectingStub = externalOriginGuids.has(parentConn.connecting.piece.guid);
                const connMatchesParentage =
                  (parentConn.connecting.piece.guid === piece.guid && parentConn.connected.piece.guid === pInfo.parentGuid) || (parentConn.connected.piece.guid === piece.guid && parentConn.connecting.piece.guid === pInfo.parentGuid);
                // Specs: Coord updates u/v only on this remapped stub-bridge using target matched parent center + anchor;
                // descendant internal edges are unchanged (second paste pass).
                if (connMatchesParentage && connectedStub !== connectingStub) {
                  let flatParentCenter: Coord | undefined;
                  if (candidate.center) flatParentCenter = { u: candidate.center.u, v: candidate.center.v };
                  else {
                    const candAttr = (candidate.attributes ?? []).find((a) => a.key === "semio.center");
                    if (candAttr?.value) flatParentCenter = JSON.parse(candAttr.value) as Coord;
                  }
                  if (!flatParentCenter) {
                    const epCenterAttr = (externalParent.attributes ?? []).find((a) => a.key === "semio.center");
                    if (epCenterAttr?.value) flatParentCenter = JSON.parse(epCenterAttr.value) as Coord;
                    else if (externalParent.center) flatParentCenter = externalParent.center;
                  }

                  let flatChildCenter: Coord | undefined;
                  const childCenterAttr = (piece.attributes ?? []).find((a) => a.key === "semio.center");
                  if (childCenterAttr?.value) flatChildCenter = JSON.parse(childCenterAttr.value) as Coord;
                  else if (piece.center) flatChildCenter = piece.center;

                  if (flatParentCenter && flatChildCenter) {
                    copiedConn.u = flatParentCenter.u - (coord.u + (anchor.u - flatChildCenter.u));
                    copiedConn.v = flatParentCenter.v - (coord.v + (anchor.v - flatChildCenter.v));
                  }
                }
              }

              addedConnections.push(copiedConn);
              break;
            }
          }
        }
      }

      if (!matched) {
        const copied: Piece = JSON.parse(JSON.stringify(piece));
        const attrs = piece.attributes ?? [];
        const centerAttr = attrs.find((a) => a.key === "semio.center");
        const planeAttr = attrs.find((a) => a.key === "semio.plane");
        if (centerAttr?.value) copied.center = JSON.parse(centerAttr.value);
        if (planeAttr?.value) copied.plane = JSON.parse(planeAttr.value);
        const c = copied.center ?? { u: 0, v: 0 };
        copied.center = { u: c.u - anchor.u + (coord?.u ?? 0), v: c.v - anchor.v + (coord?.v ?? 0) };
        addedPieces.push(copied);
      }
    } else if (isFixed) {
      const copied: Piece = JSON.parse(JSON.stringify(piece));
      let cu = 0;
      let cv = 0;
      if (copied.center) {
        cu = copied.center.u;
        cv = copied.center.v;
      } else {
        const centerAttr = (copied.attributes ?? []).find((a) => a.key === "semio.center");
        if (centerAttr?.value) {
          const parsed = JSON.parse(centerAttr.value) as Coord;
          cu = parsed.u;
          cv = parsed.v;
        }
      }
      copied.center = { u: cu - anchor.u + (coord?.u ?? 0), v: cv - anchor.v + (coord?.v ?? 0) };
      addedPieces.push(copied);
    } else if (isConnected && pInfo) {
      addedPieces.push(JSON.parse(JSON.stringify(piece)));
    }
  }

  // Process source connections (non-external internal connections)
  const addedPieceGuids = new Set(addedPieces.map((p) => p.guid));
  for (const conn of sourceConnections) {
    if (externalOriginGuids.has(conn.connected.piece.guid) || externalOriginGuids.has(conn.connecting.piece.guid)) continue;
    if (!addedPieceGuids.has(conn.connected.piece.guid) || !addedPieceGuids.has(conn.connecting.piece.guid)) continue;
    addedConnections.push(JSON.parse(JSON.stringify(conn)));
  }

  const diff: DesignDiff = {};
  if (addedPieces.length > 0) diff.pieces = { added: addedPieces };
  if (addedConnections.length > 0) diff.connections = { added: addedConnections };
  return diff;
};

// #endregion 📐Design

// #region ⏱️Kit
// Kit entity types, schemas, and helpers MUST be defined here.

// #region 🧬KitKind
// KitKind discriminates the five persistence/transport forms of a Kit.

/**
 * Zod schema for KitKind validation.
 *
 * Specs: Exactly five kit kinds exist:
 * - file: Self-contained JSON file (.kit.json)
 * - folder: Local folder with .semio/kit.db SQLite file and asset files
 * - archive: ZIP file packaging a FolderKit structure
 * - remote: URL-addressable kit served over HTTP(S)
 * - temporary: In-memory ephemeral kit (no persistence)
 **/
export const KitKindSchema = z.enum(["file", "folder", "archive", "remote", "temporary"]);
/**
 * Discriminator for the five kit persistence/transport forms.
 **/
export type KitKind = z.infer<typeof KitKindSchema>;
/**
 * All valid KitKind values as a readonly tuple.
 **/
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;
// #endregion 🧬KitKind

/**
 * Zod schema for Kit validation.
 **/
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
/**
 * Type alias for Kit.
 **/
export type Kit = z.infer<typeof KitSchema>;
/**
 * Serializes Kit for transport.
 **/
export const serializeKit = (kit: Kit): string => JSON.stringify(KitSchema.parse(kit));
/**
 **/
export const deserializeKit = (json: string): Kit => KitSchema.parse(JSON.parse(json, (_key, value) => (value === null ? undefined : value)));

/**
 * Definition of KitMetaSchema.
 **/
export const KitMetaSchema = KitSchema.omit({ types: true, designs: true, tags: true, concepts: true, ports: true, qualities: true, files: true, folders: true, authors: true, attributes: true });
/**
 * Type alias for KitMeta.
 **/
export type KitMeta = z.infer<typeof KitMetaSchema>;
/**
 * Serializes KitMeta for transport.
 **/
export const serializeKitMeta = (kit: KitMeta): string => JSON.stringify(KitMetaSchema.parse(kit));
/**
 **/
export const deserializeKitMeta = (json: string): KitMeta => KitMetaSchema.parse(JSON.parse(json));
/**
 * Definition of KitShallowSchema.
 **/
export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, tags: true, concepts: true, ports: true, qualities: true, files: true, folders: true, authors: true, attributes: true }).extend({
  types: z.array(TypeMetaSchema).optional(),
  designs: z.array(DesignMetaSchema).optional(),
  tags: z.array(TagMetaSchema).optional(),
  concepts: z.array(ConceptMetaSchema).optional(),
  ports: z.array(PortMetaSchema).optional(),
  qualities: z.array(QualityMetaSchema).optional(),
  files: z.array(FileMetaSchema).optional(),
  folders: z.array(FolderMetaSchema).optional(),
  authors: z.array(AuthorMetaSchema).optional(),
  attributes: z.array(AttributeMetaSchema).optional(),
});
/**
 * Type alias for KitShallow.
 **/
export type KitShallow = z.infer<typeof KitShallowSchema>;
/**
 * Serializes KitShallow for transport.
 **/
export const serializeKitShallow = (kit: KitShallow): string => JSON.stringify(KitShallowSchema.parse(kit));
/**
 **/
export const deserializeKitShallow = (json: string): KitShallow => KitShallowSchema.parse(JSON.parse(json));
/**
 * Converts a Type to TypeMeta.
 **/
export const toTypeMeta = (type: Type): TypeMeta => TypeMetaSchema.parse(type);
/**
 * Converts a Type to TypeShallow.
 **/
export const toTypeShallow = (type: Type): TypeShallow => {
  const result: any = { ...type };
  if (result.models) result.models = result.models.map((m: Model) => ModelMetaSchema.parse(m));
  if (result.connectors) result.connectors = result.connectors.map((c: Connector) => ConnectorMetaSchema.parse(c));
  if (result.props) result.props = result.props.map((p: Prop) => PropMetaSchema.parse(p));
  if (result.attributes) result.attributes = result.attributes.map((a: Attribute) => AttributeMetaSchema.parse(a));
  return TypeShallowSchema.parse(result);
};
/**
 * Converts a Design to DesignMeta.
 **/
export const toDesignMeta = (design: Design): DesignMeta => DesignMetaSchema.parse(design);
/**
 * Converts a Design to DesignShallow.
 **/
export const toDesignShallow = (design: Design): DesignShallow => {
  const result: any = { ...design };
  if (result.pieces) result.pieces = result.pieces.map((p: Piece) => PieceMetaSchema.parse(p));
  if (result.connections) result.connections = result.connections.map((c: Connection) => ConnectionMetaSchema.parse(c));
  if (result.stats) result.stats = result.stats.map((s: Stat) => StatMetaSchema.parse(s));
  if (result.props) result.props = result.props.map((p: Prop) => PropMetaSchema.parse(p));
  if (result.layers) result.layers = result.layers.map((l: Layer) => LayerMetaSchema.parse(l));
  if (result.groups) result.groups = result.groups.map((g: Group) => GroupMetaSchema.parse(g));
  if (result.attributes) result.attributes = result.attributes.map((a: Attribute) => AttributeMetaSchema.parse(a));
  return DesignShallowSchema.parse(result);
};
/**
 * Converts a Kit to KitMeta.
 **/
export const toKitMeta = (kit: Kit): KitMeta => KitMetaSchema.parse(kit);
/**
 * Converts a Kit to KitShallow.
 **/
export const toKitShallow = (kit: Kit): KitShallow => {
  const result: any = { ...kit };
  if (result.types) result.types = result.types.map((t: Type) => TypeMetaSchema.parse(t));
  if (result.designs) result.designs = result.designs.map((d: Design) => DesignMetaSchema.parse(d));
  if (result.tags) result.tags = result.tags.map((t: Tag) => TagMetaSchema.parse(t));
  if (result.concepts) result.concepts = result.concepts.map((c: Concept) => ConceptMetaSchema.parse(c));
  if (result.ports) result.ports = result.ports.map((p: Port) => PortMetaSchema.parse(p));
  if (result.qualities) result.qualities = result.qualities.map((q: Quality) => QualityMetaSchema.parse(q));
  if (result.files) result.files = result.files.map((f: File) => FileMetaSchema.parse(f));
  if (result.folders) result.folders = result.folders.map((f: Folder) => FolderMetaSchema.parse(f));
  if (result.authors) result.authors = result.authors.map((a: Author) => AuthorMetaSchema.parse(a));
  if (result.attributes) result.attributes = result.attributes.map((a: Attribute) => AttributeMetaSchema.parse(a));
  return KitShallowSchema.parse(result);
};
/**
 * Zod schema for Kit diff validation.
 **/
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
/**
 * Diff type for tracking Kit changes.
 **/
export type KitDiff = z.infer<typeof KitDiffSchema>;
// 🧬EntityIdType maps entity kind names to their ID interface types.
type EntityIdType = { guid: string };
// 🔀CollectionDiff represents added, removed, and changed items in a collection.
type CollectionDiff<K extends string, T extends { guid: string }, D> = {
  removed?: EntityIdType[];
  updated?: Array<{ [key in K]: EntityIdType } & { diff: D }>;
  added?: T[];
};
// 🔀getCollectionDiff computes the diff between two collections by key.
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
// 🔀inverseCollectionDiff inverts a collection diff to reverse its effect.
const inverseCollectionDiff = <K extends string, T extends { guid: string }, D>(entityKey: K, original: T[], appliedDiff: CollectionDiff<K, T, D>, inverseItemDiff: (original: T, appliedDiff: D) => D): CollectionDiff<K, T, D> => {
  const inverse: CollectionDiff<K, T, D> = {};
  const removedGuids = appliedDiff.removed?.map((r) => r.guid) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedGuids.includes(i.guid));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ guid: i.guid }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated
      .filter((u) => {
        const entityId = (u as any)[entityKey] as EntityIdType;
        return original.some((i) => i.guid === entityId.guid);
      })
      .map((u) => {
        const entityId = (u as any)[entityKey] as EntityIdType;
        const originalItem = original.find((i) => i.guid === entityId.guid)!;
        return { [entityKey]: entityId, diff: inverseItemDiff(originalItem, u.diff) } as { [key in K]: EntityIdType } & { diff: D };
      });
  }
  return inverse;
};
// 🔀applyCollectionDiff applies a collection diff to produce an updated collection.
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

// 🔀mergeCollectionDiff merges two collection diffs into one.
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

/**
 * Retrieves the KitDiff value.
 **/
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
/**
 * Diff type for tracking inverseKit changes.
 **/
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
/**
 * Diff type for tracking mergeKit changes.
 **/
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
/**
 * Diff type for tracking applyKit changes.
 **/
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

// #endregion ⏱️Kit

// #region 🖥️Hash
// Merkle hash functions for all entities. Each hash function computes a deterministic
// SHA-256 hex digest. Collections are hashed by sorting child hashes alphabetically.
// Field order is alphabetical by JSON field name. Missing/null fields are skipped.
// Number format: integer if no fractional part, else shortest decimal representation.

// #region 🔷SHA-256
// 🌿Pure JS SHA-256 implementation for cross-platform compatibility (Node + browser).
const _sha256K = new Uint32Array([
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
  0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
  0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
  0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]);
const _sha256H0 = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);

const sha256bytes = (data: Uint8Array): string => {
  const rr = (x: number, n: number) => (x >>> n) | (x << (32 - n));
  const bitLen = data.length * 8;
  const padLen = data.length + 1 + ((((55 - data.length) % 64) + 64) % 64) + 8;
  const padded = new Uint8Array(padLen);
  padded.set(data);
  padded[data.length] = 0x80;
  const view = new DataView(padded.buffer);
  view.setUint32(padLen - 4, bitLen, false);
  if (bitLen > 0xffffffff) view.setUint32(padLen - 8, Math.floor(bitLen / 0x100000000), false);
  const H = new Uint32Array(_sha256H0);
  const W = new Uint32Array(64);
  for (let off = 0; off < padLen; off += 64) {
    for (let i = 0; i < 16; i++) W[i] = view.getUint32(off + i * 4, false);
    for (let i = 16; i < 64; i++) {
      const s0 = rr(W[i - 15], 7) ^ rr(W[i - 15], 18) ^ (W[i - 15] >>> 3);
      const s1 = rr(W[i - 2], 17) ^ rr(W[i - 2], 19) ^ (W[i - 2] >>> 10);
      W[i] = (W[i - 16] + s0 + W[i - 7] + s1) | 0;
    }
    let [a, b, c, d, e, f, g, h] = H;
    for (let i = 0; i < 64; i++) {
      const S1 = rr(e, 6) ^ rr(e, 11) ^ rr(e, 25);
      const ch = (e & f) ^ (~e & g);
      const t1 = (h + S1 + ch + _sha256K[i] + W[i]) | 0;
      const S0 = rr(a, 2) ^ rr(a, 13) ^ rr(a, 22);
      const maj = (a & b) ^ (a & c) ^ (b & c);
      const t2 = (S0 + maj) | 0;
      h = g;
      g = f;
      f = e;
      e = (d + t1) | 0;
      d = c;
      c = b;
      b = a;
      a = (t1 + t2) | 0;
    }
    H[0] = (H[0] + a) | 0;
    H[1] = (H[1] + b) | 0;
    H[2] = (H[2] + c) | 0;
    H[3] = (H[3] + d) | 0;
    H[4] = (H[4] + e) | 0;
    H[5] = (H[5] + f) | 0;
    H[6] = (H[6] + g) | 0;
    H[7] = (H[7] + h) | 0;
  }
  return Array.from(H)
    .map((v) => (v >>> 0).toString(16).padStart(8, "0"))
    .join("");
};
// #endregion 🔷SHA-256

// #region 🌩️HashWriter
/**
 * Feeds structured data into a SHA-256 hasher for deterministic hashing.
 * Uses length-prefixed strings and type tags for unambiguous encoding.
 **/
class HashWriter {
  private parts: Uint8Array[] = [];
  private len = 0;
  private push(buf: Uint8Array) {
    this.parts.push(buf);
    this.len += buf.length;
  }
  writeString(s: string) {
    const b = new TextEncoder().encode(s);
    const lb = new Uint8Array(4);
    new DataView(lb.buffer).setUint32(0, b.length, false);
    this.push(lb);
    this.push(b);
  }
  writeNumber(n: number) {
    this.writeString(formatNumberForHash(n));
  }
  writeBool(b: boolean) {
    this.push(new Uint8Array([b ? 1 : 0]));
  }
  writeHash(h: string) {
    this.writeString(h);
  }
  writeHashList(hashes: string[]) {
    const sorted = [...hashes].sort();
    const lb = new Uint8Array(4);
    new DataView(lb.buffer).setUint32(0, sorted.length, false);
    this.push(lb);
    for (const h of sorted) this.writeString(h);
  }
  writeGuidList(guids: string[]) {
    const sorted = [...guids].sort();
    const lb = new Uint8Array(4);
    new DataView(lb.buffer).setUint32(0, sorted.length, false);
    this.push(lb);
    for (const g of sorted) this.writeString(g);
  }
  digest(): string {
    const buf = new Uint8Array(this.len);
    let off = 0;
    for (const p of this.parts) {
      buf.set(p, off);
      off += p.length;
    }
    return sha256bytes(buf);
  }
}
// #endregion 🌩️HashWriter

/**
 * Formats a number deterministically for hashing.
 * Integer values (no fractional part) are formatted without decimal point.
 * Non-integer values use shortest decimal representation.
 **/
export const formatNumberForHash = (n: number): string => {
  if (Number.isInteger(n)) return n.toString();
  return n.toString();
};

// #region 🎵Hash Value Types
/**
 * Computes SHA-256 hash of a Coord value.
 **/
export const hashCoord = (c: Coord): string => {
  const w = new HashWriter();
  w.writeString("Coord");
  w.writeString("u");
  w.writeNumber(c.u);
  w.writeString("v");
  w.writeNumber(c.v);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Vec value.
 **/
export const hashVec = (v: Vec): string => {
  const w = new HashWriter();
  w.writeString("Vec");
  w.writeString("u");
  w.writeNumber(v.u);
  w.writeString("v");
  w.writeNumber(v.v);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Point value.
 **/
export const hashPoint = (p: Point): string => {
  const w = new HashWriter();
  w.writeString("Point");
  w.writeString("x");
  w.writeNumber(p.x);
  w.writeString("y");
  w.writeNumber(p.y);
  w.writeString("z");
  w.writeNumber(p.z);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Vector value.
 **/
export const hashVector = (v: Vector): string => {
  const w = new HashWriter();
  w.writeString("Vector");
  w.writeString("x");
  w.writeNumber(v.x);
  w.writeString("y");
  w.writeNumber(v.y);
  w.writeString("z");
  w.writeNumber(v.z);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Plane value.
 **/
export const hashPlane = (p: Plane): string => {
  const w = new HashWriter();
  w.writeString("Plane");
  w.writeString("origin");
  w.writeHash(hashPoint(p.origin));
  w.writeString("xAxis");
  w.writeHash(hashVector(p.xAxis));
  w.writeString("yAxis");
  w.writeHash(hashVector(p.yAxis));
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Camera value.
 **/
export const hashCamera = (c: Camera): string => {
  const w = new HashWriter();
  w.writeString("Camera");
  w.writeString("forward");
  w.writeHash(hashVector(c.forward));
  w.writeString("position");
  w.writeHash(hashPoint(c.position));
  w.writeString("up");
  w.writeHash(hashVector(c.up));
  return w.digest();
};
// #endregion 🎵Hash Value Types

// #region 🎩Hash Entities
/**
 * Computes SHA-256 hash of an Attribute entity.
 **/
export const hashAttribute = (a: Attribute): string => {
  const w = new HashWriter();
  w.writeString("Attribute");
  if (a.definition != null) {
    w.writeString("definition");
    w.writeString(a.definition);
  }
  w.writeString("guid");
  w.writeString(a.guid);
  w.writeString("key");
  w.writeString(a.key);
  if (a.value != null) {
    w.writeString("value");
    w.writeString(a.value);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Location entity.
 **/
export const hashLocation = (l: Location): string => {
  const w = new HashWriter();
  w.writeString("Location");
  if (l.altitude != null) {
    w.writeString("altitude");
    w.writeNumber(l.altitude);
  }
  if (l.attributes && l.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(l.attributes.map(hashAttribute));
  }
  w.writeString("guid");
  w.writeString(l.guid);
  w.writeString("latitude");
  w.writeNumber(l.latitude);
  w.writeString("longitude");
  w.writeNumber(l.longitude);
  return w.digest();
};

/**
 * Computes SHA-256 hash of an Author entity.
 **/
export const hashAuthor = (a: Author): string => {
  const w = new HashWriter();
  w.writeString("Author");
  if (a.attributes && a.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(a.attributes.map(hashAttribute));
  }
  if (a.email != null && a.email !== "") {
    w.writeString("email");
    w.writeString(a.email);
  }
  w.writeString("guid");
  w.writeString(a.guid);
  w.writeString("name");
  w.writeString(a.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a File entity.
 **/
export const hashFile = (f: File): string => {
  const w = new HashWriter();
  w.writeString("File");
  if (f.blob != null) {
    w.writeString("blob");
    w.writeString(f.blob);
  }
  if (f.folder != null) {
    w.writeString("folder");
    w.writeString(f.folder.guid);
  }
  w.writeString("guid");
  w.writeString(f.guid);
  if (f.hash != null) {
    w.writeString("hash");
    w.writeString(f.hash);
  }
  w.writeString("name");
  w.writeString(f.name);
  if (f.remote != null) {
    w.writeString("remote");
    w.writeString(f.remote);
  }
  if (f.size != null) {
    w.writeString("size");
    w.writeNumber(f.size);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Folder entity.
 **/
export const hashFolder = (f: Folder): string => {
  const w = new HashWriter();
  w.writeString("Folder");
  if (f.attributes && f.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(f.attributes.map(hashAttribute));
  }
  if (f.description != null) {
    w.writeString("description");
    w.writeString(f.description);
  }
  w.writeString("guid");
  w.writeString(f.guid);
  w.writeString("name");
  w.writeString(f.name);
  if (f.parent != null) {
    w.writeString("parent");
    w.writeString(f.parent.guid);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Benchmark entity.
 **/
export const hashBenchmark = (b: Benchmark): string => {
  const w = new HashWriter();
  w.writeString("Benchmark");
  if (b.attributes && b.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(b.attributes.map(hashAttribute));
  }
  w.writeString("guid");
  w.writeString(b.guid);
  if (b.icon != null) {
    w.writeString("icon");
    w.writeString(b.icon);
  }
  if (b.max != null) {
    w.writeString("max");
    w.writeNumber(b.max);
  }
  if (b.maxExcluded != null) {
    w.writeString("maxExcluded");
    w.writeBool(b.maxExcluded);
  }
  if (b.min != null) {
    w.writeString("min");
    w.writeNumber(b.min);
  }
  if (b.minExcluded != null) {
    w.writeString("minExcluded");
    w.writeBool(b.minExcluded);
  }
  w.writeString("name");
  w.writeString(b.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Quality entity.
 **/
export const hashQuality = (q: Quality): string => {
  const w = new HashWriter();
  w.writeString("Quality");
  if (q.benchmarks && q.benchmarks.length > 0) {
    w.writeString("benchmarks");
    w.writeHashList(q.benchmarks.map(hashBenchmark));
  }
  if (q.canScale != null) {
    w.writeString("canScale");
    w.writeBool(q.canScale);
  }
  if (q.defaultImperialUnit != null) {
    w.writeString("defaultImperialUnit");
    w.writeString(q.defaultImperialUnit);
  }
  if (q.defaultSiUnit != null) {
    w.writeString("defaultSiUnit");
    w.writeString(q.defaultSiUnit);
  }
  if (q.defaultValue != null) {
    w.writeString("defaultValue");
    w.writeNumber(q.defaultValue);
  }
  if (q.description != null) {
    w.writeString("description");
    w.writeString(q.description);
  }
  if (q.formula != null) {
    w.writeString("formula");
    w.writeString(q.formula);
  }
  w.writeString("guid");
  w.writeString(q.guid);
  if (q.icon != null) {
    w.writeString("icon");
    w.writeString(q.icon);
  }
  if (q.image != null) {
    w.writeString("image");
    w.writeString(q.image);
  }
  if (q.isMaxExcluded != null) {
    w.writeString("isMaxExcluded");
    w.writeBool(q.isMaxExcluded);
  }
  if (q.isMinExcluded != null) {
    w.writeString("isMinExcluded");
    w.writeBool(q.isMinExcluded);
  }
  w.writeString("key");
  w.writeString(q.key);
  if (q.kind != null) {
    w.writeString("kind");
    w.writeNumber(q.kind);
  }
  if (q.max != null) {
    w.writeString("max");
    w.writeNumber(q.max);
  }
  if (q.min != null) {
    w.writeString("min");
    w.writeNumber(q.min);
  }
  w.writeString("name");
  w.writeString(q.name);
  if (q.unit != null) {
    w.writeString("unit");
    w.writeString(q.unit);
  }
  if (q.uri != null) {
    w.writeString("uri");
    w.writeString(q.uri);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Port entity.
 **/
export const hashPort = (p: Port): string => {
  const w = new HashWriter();
  w.writeString("Port");
  if (p.attributes && p.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(p.attributes.map(hashAttribute));
  }
  if (p.compatiblePorts && p.compatiblePorts.length > 0) {
    w.writeString("compatiblePorts");
    w.writeGuidList(p.compatiblePorts.map((cp) => cp.guid));
  }
  if (p.description != null) {
    w.writeString("description");
    w.writeString(p.description);
  }
  w.writeString("guid");
  w.writeString(p.guid);
  if (p.icon != null) {
    w.writeString("icon");
    w.writeString(p.icon);
  }
  w.writeString("name");
  w.writeString(p.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Prop entity.
 **/
export const hashProp = (p: Prop): string => {
  const w = new HashWriter();
  w.writeString("Prop");
  if (p.attributes && p.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(p.attributes.map(hashAttribute));
  }
  w.writeString("guid");
  w.writeString(p.guid);
  w.writeString("quality");
  w.writeString(p.quality.guid);
  if (p.unit != null) {
    w.writeString("unit");
    w.writeString(p.unit);
  }
  w.writeString("value");
  w.writeString(p.value);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Tag entity.
 **/
export const hashTag = (t: Tag): string => {
  const w = new HashWriter();
  w.writeString("Tag");
  if (t.attributes && t.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(t.attributes.map(hashAttribute));
  }
  if (t.description != null) {
    w.writeString("description");
    w.writeString(t.description);
  }
  w.writeString("guid");
  w.writeString(t.guid);
  if (t.icon != null) {
    w.writeString("icon");
    w.writeString(t.icon);
  }
  w.writeString("name");
  w.writeString(t.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Concept entity.
 **/
export const hashConcept = (c: Concept): string => {
  const w = new HashWriter();
  w.writeString("Concept");
  if (c.attributes && c.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(c.attributes.map(hashAttribute));
  }
  if (c.description != null) {
    w.writeString("description");
    w.writeString(c.description);
  }
  w.writeString("guid");
  w.writeString(c.guid);
  if (c.icon != null) {
    w.writeString("icon");
    w.writeString(c.icon);
  }
  w.writeString("name");
  w.writeString(c.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Model entity.
 **/
export const hashModel = (m: Model): string => {
  const w = new HashWriter();
  w.writeString("Model");
  if (m.attributes && m.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(m.attributes.map(hashAttribute));
  }
  if (m.description != null) {
    w.writeString("description");
    w.writeString(m.description);
  }
  w.writeString("file");
  w.writeString(m.file.guid);
  w.writeString("guid");
  w.writeString(m.guid);
  if (m.name != null) {
    w.writeString("name");
    w.writeString(m.name);
  }
  if (m.tags && m.tags.length > 0) {
    w.writeString("tags");
    w.writeGuidList(m.tags.map((t) => t.guid));
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Connector entity.
 **/
export const hashConnector = (c: Connector): string => {
  const w = new HashWriter();
  w.writeString("Connector");
  if (c.attributes && c.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(c.attributes.map(hashAttribute));
  }
  if (c.description != null) {
    w.writeString("description");
    w.writeString(c.description);
  }
  w.writeString("direction");
  w.writeHash(hashVector(c.direction));
  w.writeString("guid");
  w.writeString(c.guid);
  if (c.mandatory != null) {
    w.writeString("mandatory");
    w.writeBool(c.mandatory);
  }
  if (c.name != null) {
    w.writeString("name");
    w.writeString(c.name);
  }
  w.writeString("point");
  w.writeHash(hashPoint(c.point));
  if (c.port != null) {
    w.writeString("port");
    w.writeString(c.port.guid);
  }
  if (c.props && c.props.length > 0) {
    w.writeString("props");
    w.writeHashList(c.props.map(hashProp));
  }
  w.writeString("t");
  w.writeNumber(c.t);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Type entity.
 **/
export const hashType = (t: Type): string => {
  const w = new HashWriter();
  w.writeString("Type");
  if (t.attributes && t.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(t.attributes.map(hashAttribute));
  }
  if (t.authors && t.authors.length > 0) {
    w.writeString("authors");
    w.writeGuidList(t.authors.map((a) => a.guid));
  }
  if (t.concepts && t.concepts.length > 0) {
    w.writeString("concepts");
    w.writeGuidList(t.concepts.map((c) => c.guid));
  }
  if (t.connectors && t.connectors.length > 0) {
    w.writeString("connectors");
    w.writeHashList(t.connectors.map(hashConnector));
  }
  if (t.description != null) {
    w.writeString("description");
    w.writeString(t.description);
  }
  if (t.folder != null) {
    w.writeString("folder");
    w.writeString(t.folder);
  }
  w.writeString("guid");
  w.writeString(t.guid);
  if (t.icon != null) {
    w.writeString("icon");
    w.writeString(t.icon);
  }
  if (t.image != null) {
    w.writeString("image");
    w.writeString(t.image);
  }
  if (t.isAbstract != null) {
    w.writeString("isAbstract");
    w.writeBool(t.isAbstract);
  }
  if (t.location != null) {
    w.writeString("location");
    w.writeString(t.location.guid);
  }
  if (t.models && t.models.length > 0) {
    w.writeString("models");
    w.writeHashList(t.models.map(hashModel));
  }
  w.writeString("name");
  w.writeString(t.name);
  if (t.parent != null) {
    w.writeString("parent");
    w.writeString(t.parent.guid);
  }
  if (t.props && t.props.length > 0) {
    w.writeString("props");
    w.writeHashList(t.props.map(hashProp));
  }
  if (t.stock != null) {
    w.writeString("stock");
    w.writeNumber(t.stock);
  }
  if (t.unit != null) {
    w.writeString("unit");
    w.writeString(t.unit);
  }
  if (t.virtual != null) {
    w.writeString("virtual");
    w.writeBool(t.virtual);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Layer entity.
 **/
export const hashLayer = (l: Layer): string => {
  const w = new HashWriter();
  w.writeString("Layer");
  if (l.attributes && l.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(l.attributes.map(hashAttribute));
  }
  if (l.color != null) {
    w.writeString("color");
    w.writeString(l.color);
  }
  if (l.description != null) {
    w.writeString("description");
    w.writeString(l.description);
  }
  w.writeString("guid");
  w.writeString(l.guid);
  if (l.isHidden != null) {
    w.writeString("isHidden");
    w.writeBool(l.isHidden);
  }
  if (l.isLocked != null) {
    w.writeString("isLocked");
    w.writeBool(l.isLocked);
  }
  w.writeString("path");
  w.writeString(l.path);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Stat entity.
 **/
export const hashStat = (s: Stat): string => {
  const w = new HashWriter();
  w.writeString("Stat");
  w.writeString("guid");
  w.writeString(s.guid);
  if (s.max != null) {
    w.writeString("max");
    w.writeNumber(s.max);
  }
  if (s.maxExcluded != null) {
    w.writeString("maxExcluded");
    w.writeBool(s.maxExcluded);
  }
  if (s.min != null) {
    w.writeString("min");
    w.writeNumber(s.min);
  }
  if (s.minExcluded != null) {
    w.writeString("minExcluded");
    w.writeBool(s.minExcluded);
  }
  w.writeString("quality");
  w.writeString(s.quality.guid);
  if (s.unit != null) {
    w.writeString("unit");
    w.writeString(s.unit);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Group entity.
 **/
export const hashGroup = (g: Group): string => {
  const w = new HashWriter();
  w.writeString("Group");
  if (g.attributes && g.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(g.attributes.map(hashAttribute));
  }
  if (g.color != null) {
    w.writeString("color");
    w.writeString(g.color);
  }
  if (g.description != null) {
    w.writeString("description");
    w.writeString(g.description);
  }
  w.writeString("guid");
  w.writeString(g.guid);
  if (g.name != null) {
    w.writeString("name");
    w.writeString(g.name);
  }
  w.writeString("pieces");
  w.writeGuidList(g.pieces.map((p) => p.guid));
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Side value.
 **/
export const hashSide = (s: Side): string => {
  const w = new HashWriter();
  w.writeString("Side");
  if (s.connector != null) {
    w.writeString("connector");
    w.writeString(s.connector.guid);
  }
  if (s.designPiece != null) {
    w.writeString("designPiece");
    w.writeString(s.designPiece.guid);
  }
  w.writeString("piece");
  w.writeString(s.piece.guid);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Connection entity.
 **/
export const hashConnection = (c: Connection): string => {
  const w = new HashWriter();
  w.writeString("Connection");
  if (c.attributes && c.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(c.attributes.map(hashAttribute));
  }
  w.writeString("connected");
  w.writeHash(hashSide(c.connected));
  w.writeString("connecting");
  w.writeHash(hashSide(c.connecting));
  if (c.description != null) {
    w.writeString("description");
    w.writeString(c.description);
  }
  if (c.gap != null) {
    w.writeString("gap");
    w.writeNumber(c.gap);
  }
  w.writeString("guid");
  w.writeString(c.guid);
  if (c.rise != null) {
    w.writeString("rise");
    w.writeNumber(c.rise);
  }
  if (c.rotation != null) {
    w.writeString("rotation");
    w.writeNumber(c.rotation);
  }
  if (c.shift != null) {
    w.writeString("shift");
    w.writeNumber(c.shift);
  }
  if (c.tilt != null) {
    w.writeString("tilt");
    w.writeNumber(c.tilt);
  }
  if (c.turn != null) {
    w.writeString("turn");
    w.writeNumber(c.turn);
  }
  if (c.u != null) {
    w.writeString("u");
    w.writeNumber(c.u);
  }
  if (c.v != null) {
    w.writeString("v");
    w.writeNumber(c.v);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Piece entity.
 **/
export const hashPiece = (p: Piece): string => {
  const w = new HashWriter();
  w.writeString("Piece");
  if (p.attributes && p.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(p.attributes.map(hashAttribute));
  }
  if (p.center != null) {
    w.writeString("center");
    w.writeHash(hashCoord(p.center));
  }
  if (p.color != null) {
    w.writeString("color");
    w.writeString(p.color);
  }
  if (p.description != null) {
    w.writeString("description");
    w.writeString(p.description);
  }
  if (p.design != null) {
    w.writeString("design");
    w.writeString(p.design.guid);
  }
  w.writeString("guid");
  w.writeString(p.guid);
  if (p.isHidden != null) {
    w.writeString("isHidden");
    w.writeBool(p.isHidden);
  }
  if (p.isLocked != null) {
    w.writeString("isLocked");
    w.writeBool(p.isLocked);
  }
  if (p.mirrorPlane != null) {
    w.writeString("mirrorPlane");
    w.writeHash(hashPlane(p.mirrorPlane));
  }
  if (p.name != null) {
    w.writeString("name");
    w.writeString(p.name);
  }
  if (p.plane != null) {
    w.writeString("plane");
    w.writeHash(hashPlane(p.plane));
  }
  if (p.props && p.props.length > 0) {
    w.writeString("props");
    w.writeHashList(p.props.map(hashProp));
  }
  if (p.scale != null) {
    w.writeString("scale");
    w.writeNumber(p.scale);
  }
  if (p.type != null) {
    w.writeString("type");
    w.writeString(p.type.guid);
  }
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Design entity (Merkle tree).
 **/
export const hashDesign = (d: Design): string => {
  const w = new HashWriter();
  w.writeString("Design");
  if (d.activeLayer != null) {
    w.writeString("activeLayer");
    w.writeString(d.activeLayer.guid);
  }
  if (d.attributes && d.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(d.attributes.map(hashAttribute));
  }
  if (d.authors && d.authors.length > 0) {
    w.writeString("authors");
    w.writeGuidList(d.authors.map((a) => a.guid));
  }
  if (d.canMirror != null) {
    w.writeString("canMirror");
    w.writeBool(d.canMirror);
  }
  if (d.canScale != null) {
    w.writeString("canScale");
    w.writeBool(d.canScale);
  }
  if (d.concepts && d.concepts.length > 0) {
    w.writeString("concepts");
    w.writeGuidList(d.concepts.map((c) => c.guid));
  }
  if (d.connections && d.connections.length > 0) {
    w.writeString("connections");
    w.writeHashList(d.connections.map(hashConnection));
  }
  if (d.description != null) {
    w.writeString("description");
    w.writeString(d.description);
  }
  if (d.folder != null) {
    w.writeString("folder");
    w.writeString(d.folder);
  }
  if (d.groups && d.groups.length > 0) {
    w.writeString("groups");
    w.writeHashList(d.groups.map(hashGroup));
  }
  w.writeString("guid");
  w.writeString(d.guid);
  if (d.icon != null) {
    w.writeString("icon");
    w.writeString(d.icon);
  }
  if (d.image != null) {
    w.writeString("image");
    w.writeString(d.image);
  }
  if (d.isAbstract != null) {
    w.writeString("isAbstract");
    w.writeBool(d.isAbstract);
  }
  if (d.layers && d.layers.length > 0) {
    w.writeString("layers");
    w.writeHashList(d.layers.map(hashLayer));
  }
  if (d.location != null) {
    w.writeString("location");
    w.writeString(d.location.guid);
  }
  w.writeString("name");
  w.writeString(d.name);
  if (d.parent != null) {
    w.writeString("parent");
    w.writeString(d.parent.guid);
  }
  if (d.pieces && d.pieces.length > 0) {
    w.writeString("pieces");
    w.writeHashList(d.pieces.map(hashPiece));
  }
  if (d.props && d.props.length > 0) {
    w.writeString("props");
    w.writeHashList(d.props.map(hashProp));
  }
  if (d.stats && d.stats.length > 0) {
    w.writeString("stats");
    w.writeHashList(d.stats.map(hashStat));
  }
  if (d.unit != null) {
    w.writeString("unit");
    w.writeString(d.unit);
  }
  return w.digest();
};

/**
 * Computes SHA-256 Merkle hash of a Kit entity.
 * Calls hashDesign, hashType, etc. for all children.
 **/
export const hashKit = (k: Kit): string => {
  const w = new HashWriter();
  w.writeString("Kit");
  if (k.attributes && k.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(k.attributes.map(hashAttribute));
  }
  if (k.authors && k.authors.length > 0) {
    w.writeString("authors");
    w.writeHashList(k.authors.map(hashAuthor));
  }
  if (k.concepts && k.concepts.length > 0) {
    w.writeString("concepts");
    w.writeHashList(k.concepts.map(hashConcept));
  }
  if (k.description != null) {
    w.writeString("description");
    w.writeString(k.description);
  }
  if (k.designs && k.designs.length > 0) {
    w.writeString("designs");
    w.writeHashList(k.designs.map(hashDesign));
  }
  if (k.files && k.files.length > 0) {
    w.writeString("files");
    w.writeHashList(k.files.map(hashFile));
  }
  if (k.folders && k.folders.length > 0) {
    w.writeString("folders");
    w.writeHashList(k.folders.map(hashFolder));
  }
  w.writeString("guid");
  w.writeString(k.guid);
  if (k.homepage != null) {
    w.writeString("homepage");
    w.writeString(k.homepage);
  }
  if (k.icon != null) {
    w.writeString("icon");
    w.writeString(k.icon);
  }
  if (k.image != null) {
    w.writeString("image");
    w.writeString(k.image);
  }
  if (k.license != null) {
    w.writeString("license");
    w.writeString(k.license);
  }
  w.writeString("name");
  w.writeString(k.name);
  if (k.ports && k.ports.length > 0) {
    w.writeString("ports");
    w.writeHashList(k.ports.map(hashPort));
  }
  if (k.preview != null) {
    w.writeString("preview");
    w.writeString(k.preview);
  }
  if (k.qualities && k.qualities.length > 0) {
    w.writeString("qualities");
    w.writeHashList(k.qualities.map(hashQuality));
  }
  if (k.remote != null) {
    w.writeString("remote");
    w.writeString(k.remote);
  }
  if (k.tags && k.tags.length > 0) {
    w.writeString("tags");
    w.writeHashList(k.tags.map(hashTag));
  }
  if (k.types && k.types.length > 0) {
    w.writeString("types");
    w.writeHashList(k.types.map(hashType));
  }
  if (k.version != null) {
    w.writeString("version");
    w.writeString(k.version);
  }
  return w.digest();
};
// #endregion 🎩Hash Entities

// #region 🔗Hash Diffs
// Deterministic SHA-256 Merkle hash functions for all diff types.
// Null fields are marked with a single 0x00 byte. Undefined fields are skipped.

const writeNullableString = (w: HashWriter, key: string, val: string | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeString(val);
};

const writeNullableNumber = (w: HashWriter, key: string, val: number | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeNumber(val);
};

const writeNullableBool = (w: HashWriter, key: string, val: boolean | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeBool(val);
};

const writeNullableId = (w: HashWriter, key: string, val: { guid: string } | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeString(val.guid);
};

const writeNullableIdArray = (w: HashWriter, key: string, val: { guid: string }[] | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeGuidList(val.map((v) => v.guid));
};

const writeNullableHash = (w: HashWriter, key: string, val: any, hashFn: (v: any) => string) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeHash(hashFn(val));
};

const hashCollectionDiffGeneric = (tag: string, updateTag: string, entityKeyName: string, hashEntityFn: (e: any) => string, hashDiffFn: (d: any) => string, diff: { removed?: { guid: string }[]; updated?: any[]; added?: any[] }): string => {
  const w = new HashWriter();
  w.writeString(tag);
  if (diff.added && diff.added.length > 0) {
    w.writeString("added");
    w.writeHashList(diff.added.map(hashEntityFn));
  }
  if (diff.removed && diff.removed.length > 0) {
    w.writeString("removed");
    w.writeGuidList(diff.removed.map((r) => r.guid));
  }
  if (diff.updated && diff.updated.length > 0) {
    w.writeString("updated");
    const keys = [entityKeyName, "diff"].sort();
    const updateHashes = diff.updated.map((u: any) => {
      const uw = new HashWriter();
      uw.writeString(updateTag);
      for (const k of keys) {
        if (k === "diff") {
          uw.writeString("diff");
          uw.writeHash(hashDiffFn(u.diff));
        } else {
          uw.writeString(k);
          uw.writeString(u[k].guid);
        }
      }
      return uw.digest();
    });
    w.writeHashList(updateHashes);
  }
  return w.digest();
};

// #region 🐹Hash Diff Value Types

export const hashCoordDiff = (d: CoordDiff): string => {
  const w = new HashWriter();
  w.writeString("CoordDiff");
  writeNullableNumber(w, "u", d.u);
  writeNullableNumber(w, "v", d.v);
  return w.digest();
};

export const hashVecDiff = (d: VecDiff): string => {
  const w = new HashWriter();
  w.writeString("VecDiff");
  writeNullableNumber(w, "u", d.u);
  writeNullableNumber(w, "v", d.v);
  return w.digest();
};

export const hashPointDiff = (d: PointDiff): string => {
  const w = new HashWriter();
  w.writeString("PointDiff");
  writeNullableNumber(w, "x", d.x);
  writeNullableNumber(w, "y", d.y);
  writeNullableNumber(w, "z", d.z);
  return w.digest();
};

export const hashVectorDiff = (d: VectorDiff): string => {
  const w = new HashWriter();
  w.writeString("VectorDiff");
  writeNullableNumber(w, "x", d.x);
  writeNullableNumber(w, "y", d.y);
  writeNullableNumber(w, "z", d.z);
  return w.digest();
};

export const hashPlaneDiff = (d: PlaneDiff): string => {
  const w = new HashWriter();
  w.writeString("PlaneDiff");
  writeNullableHash(w, "origin", d.origin, hashPointDiff);
  writeNullableHash(w, "xAxis", d.xAxis, hashVectorDiff);
  writeNullableHash(w, "yAxis", d.yAxis, hashVectorDiff);
  return w.digest();
};

export const hashCameraDiff = (d: CameraDiff): string => {
  const w = new HashWriter();
  w.writeString("CameraDiff");
  writeNullableHash(w, "forward", d.forward, hashVectorDiff);
  writeNullableHash(w, "position", d.position, hashPointDiff);
  writeNullableHash(w, "up", d.up, hashVectorDiff);
  return w.digest();
};

// #endregion 🐹Hash Diff Value Types

// #region ⚗️Hash Diff Entities

export const hashAttributeDiff = (d: AttributeDiff): string => {
  const w = new HashWriter();
  w.writeString("AttributeDiff");
  writeNullableString(w, "definition", d.definition);
  writeNullableString(w, "key", d.key);
  writeNullableString(w, "value", d.value);
  return w.digest();
};

export const hashAttributesDiff = (d: AttributesDiff): string => hashCollectionDiffGeneric("AttributesDiff", "AttributeDiffUpdate", "attribute", hashAttribute, hashAttributeDiff, d);

export const hashLocationDiff = (d: LocationDiff): string => {
  const w = new HashWriter();
  w.writeString("LocationDiff");
  writeNullableNumber(w, "altitude", d.altitude);
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableNumber(w, "latitude", d.latitude);
  writeNullableNumber(w, "longitude", d.longitude);
  return w.digest();
};

export const hashAuthorDiff = (d: AuthorDiff): string => {
  const w = new HashWriter();
  w.writeString("AuthorDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "email", d.email);
  writeNullableString(w, "name", d.name);
  return w.digest();
};

export const hashAuthorsDiff = (d: AuthorsDiff): string => hashCollectionDiffGeneric("AuthorsDiff", "AuthorDiffUpdate", "author", hashAuthor, hashAuthorDiff, d);

export const hashFileDiff = (d: FileDiff): string => {
  const w = new HashWriter();
  w.writeString("FileDiff");
  writeNullableString(w, "blob", d.blob);
  writeNullableId(w, "folder", d.folder);
  writeNullableString(w, "hash", d.hash);
  writeNullableString(w, "name", d.name);
  writeNullableString(w, "remote", d.remote);
  writeNullableNumber(w, "size", d.size);
  return w.digest();
};

export const hashFilesDiff = (d: FilesDiff): string => hashCollectionDiffGeneric("FilesDiff", "FileDiffUpdate", "file", hashFile, hashFileDiff, d);

export const hashFolderDiff = (d: FolderDiff): string => {
  const w = new HashWriter();
  w.writeString("FolderDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "name", d.name);
  writeNullableId(w, "parent", d.parent);
  return w.digest();
};

export const hashFoldersDiff = (d: FoldersDiff): string => hashCollectionDiffGeneric("FoldersDiff", "FolderDiffUpdate", "folder", hashFolder, hashFolderDiff, d);

export const hashBenchmarkDiff = (d: BenchmarkDiff): string => {
  const w = new HashWriter();
  w.writeString("BenchmarkDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "icon", d.icon);
  writeNullableNumber(w, "max", d.max);
  writeNullableBool(w, "maxExcluded", d.maxExcluded);
  writeNullableNumber(w, "min", d.min);
  writeNullableBool(w, "minExcluded", d.minExcluded);
  writeNullableString(w, "name", d.name);
  return w.digest();
};

export const hashBenchmarksDiff = (d: BenchmarksDiff): string => hashCollectionDiffGeneric("BenchmarksDiff", "BenchmarkDiffUpdate", "benchmark", hashBenchmark, hashBenchmarkDiff, d);

export const hashQualityDiff = (d: QualityDiff): string => {
  const w = new HashWriter();
  w.writeString("QualityDiff");
  writeNullableHash(w, "benchmarks", d.benchmarks, hashBenchmarksDiff);
  writeNullableBool(w, "canScale", d.canScale);
  writeNullableString(w, "defaultImperialUnit", d.defaultImperialUnit);
  writeNullableString(w, "defaultSiUnit", d.defaultSiUnit);
  writeNullableNumber(w, "defaultValue", d.defaultValue);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "folder", d.folder);
  writeNullableString(w, "formula", d.formula);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "image", d.image);
  writeNullableBool(w, "isMaxExcluded", d.isMaxExcluded);
  writeNullableBool(w, "isMinExcluded", d.isMinExcluded);
  writeNullableString(w, "key", d.key);
  writeNullableNumber(w, "kind", d.kind);
  writeNullableNumber(w, "max", d.max);
  writeNullableNumber(w, "min", d.min);
  writeNullableString(w, "name", d.name);
  writeNullableString(w, "unit", d.unit);
  writeNullableString(w, "uri", d.uri);
  return w.digest();
};

export const hashQualitiesDiff = (d: QualitiesDiff): string => hashCollectionDiffGeneric("QualitiesDiff", "QualityDiffUpdate", "quality", hashQuality, hashQualityDiff, d);

export const hashPortDiff = (d: PortDiff): string => {
  const w = new HashWriter();
  w.writeString("PortDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableIdArray(w, "compatiblePorts", d.compatiblePorts);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "name", d.name);
  return w.digest();
};

export const hashPortsDiff = (d: PortsDiff): string => hashCollectionDiffGeneric("PortsDiff", "PortDiffUpdate", "port", hashPort, hashPortDiff, d);

export const hashPropDiff = (d: PropDiff): string => {
  const w = new HashWriter();
  w.writeString("PropDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableId(w, "quality", d.quality);
  writeNullableString(w, "unit", d.unit);
  writeNullableString(w, "value", d.value);
  return w.digest();
};

export const hashPropsDiff = (d: PropsDiff): string => hashCollectionDiffGeneric("PropsDiff", "PropDiffUpdate", "prop", hashProp, hashPropDiff, d);

export const hashTagDiff = (d: TagDiff): string => {
  const w = new HashWriter();
  w.writeString("TagDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "name", d.name);
  return w.digest();
};

export const hashTagsDiff = (d: TagsDiff): string => hashCollectionDiffGeneric("TagsDiff", "TagDiffUpdate", "tag", hashTag, hashTagDiff, d);

export const hashConceptDiff = (d: ConceptDiff): string => {
  const w = new HashWriter();
  w.writeString("ConceptDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "name", d.name);
  return w.digest();
};

export const hashConceptsDiff = (d: ConceptsDiff): string => hashCollectionDiffGeneric("ConceptsDiff", "ConceptDiffUpdate", "concept", hashConcept, hashConceptDiff, d);

export const hashModelDiff = (d: ModelDiff): string => {
  const w = new HashWriter();
  w.writeString("ModelDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableId(w, "file", d.file);
  writeNullableString(w, "name", d.name);
  writeNullableIdArray(w, "tags", d.tags);
  return w.digest();
};

export const hashModelsDiff = (d: ModelsDiff): string => hashCollectionDiffGeneric("ModelsDiff", "ModelDiffUpdate", "model", hashModel, hashModelDiff, d);

export const hashConnectorDiff = (d: ConnectorDiff): string => {
  const w = new HashWriter();
  w.writeString("ConnectorDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableHash(w, "direction", d.direction, hashVectorDiff);
  writeNullableBool(w, "mandatory", d.mandatory);
  writeNullableString(w, "name", d.name);
  writeNullableHash(w, "point", d.point, hashPointDiff);
  writeNullableId(w, "port", d.port);
  writeNullableHash(w, "props", d.props, hashPropsDiff);
  writeNullableNumber(w, "t", d.t);
  return w.digest();
};

export const hashConnectorsDiff = (d: ConnectorsDiff): string => hashCollectionDiffGeneric("ConnectorsDiff", "ConnectorDiffUpdate", "connector", hashConnector, hashConnectorDiff, d);

export const hashTypeDiff = (d: TypeDiff): string => {
  const w = new HashWriter();
  w.writeString("TypeDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableIdArray(w, "authors", d.authors);
  writeNullableIdArray(w, "concepts", d.concepts);
  writeNullableHash(w, "connectors", d.connectors, hashConnectorsDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "folder", d.folder);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "image", d.image);
  writeNullableBool(w, "isAbstract", d.isAbstract);
  writeNullableId(w, "location", d.location);
  writeNullableHash(w, "models", d.models, hashModelsDiff);
  writeNullableString(w, "name", d.name);
  writeNullableId(w, "parent", d.parent);
  writeNullableHash(w, "props", d.props, hashPropsDiff);
  writeNullableNumber(w, "stock", d.stock);
  writeNullableString(w, "unit", d.unit);
  writeNullableBool(w, "virtual", d.virtual);
  return w.digest();
};

export const hashTypesDiff = (d: TypesDiff): string => hashCollectionDiffGeneric("TypesDiff", "TypeDiffUpdate", "type", hashType, hashTypeDiff, d);

export const hashSideDiff = (d: SideDiff): string => {
  const w = new HashWriter();
  w.writeString("SideDiff");
  writeNullableId(w, "connector", d.connector);
  writeNullableId(w, "designPiece", d.designPiece);
  writeNullableId(w, "piece", d.piece);
  return w.digest();
};

export const hashLayerDiff = (d: LayerDiff): string => {
  const w = new HashWriter();
  w.writeString("LayerDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "color", d.color);
  writeNullableString(w, "description", d.description);
  writeNullableBool(w, "isHidden", d.isHidden);
  writeNullableBool(w, "isLocked", d.isLocked);
  writeNullableString(w, "path", d.path);
  return w.digest();
};

export const hashLayersDiff = (d: LayersDiff): string => hashCollectionDiffGeneric("LayersDiff", "LayerDiffUpdate", "layer", hashLayer, hashLayerDiff, d);

export const hashGroupDiff = (d: GroupDiff): string => {
  const w = new HashWriter();
  w.writeString("GroupDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "color", d.color);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "name", d.name);
  writeNullableIdArray(w, "pieces", d.pieces);
  return w.digest();
};

export const hashGroupsDiff = (d: GroupsDiff): string => hashCollectionDiffGeneric("GroupsDiff", "GroupDiffUpdate", "group", hashGroup, hashGroupDiff, d);

export const hashStatDiff = (d: StatDiff): string => {
  const w = new HashWriter();
  w.writeString("StatDiff");
  writeNullableNumber(w, "max", d.max);
  writeNullableBool(w, "maxExcluded", d.maxExcluded);
  writeNullableNumber(w, "min", d.min);
  writeNullableBool(w, "minExcluded", d.minExcluded);
  writeNullableId(w, "quality", d.quality);
  writeNullableString(w, "unit", d.unit);
  return w.digest();
};

export const hashStatsDiff = (d: StatsDiff): string => hashCollectionDiffGeneric("StatsDiff", "StatDiffUpdate", "stat", hashStat, hashStatDiff, d);

export const hashConnectionDiff = (d: ConnectionDiff): string => {
  const w = new HashWriter();
  w.writeString("ConnectionDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableHash(w, "connected", d.connected, hashSideDiff);
  writeNullableHash(w, "connecting", d.connecting, hashSideDiff);
  writeNullableString(w, "description", d.description);
  writeNullableNumber(w, "gap", d.gap);
  writeNullableNumber(w, "rise", d.rise);
  writeNullableNumber(w, "rotation", d.rotation);
  writeNullableNumber(w, "shift", d.shift);
  writeNullableNumber(w, "tilt", d.tilt);
  writeNullableNumber(w, "turn", d.turn);
  writeNullableNumber(w, "u", d.u);
  writeNullableNumber(w, "v", d.v);
  return w.digest();
};

export const hashConnectionsDiff = (d: ConnectionsDiff): string => hashCollectionDiffGeneric("ConnectionsDiff", "ConnectionDiffUpdate", "connection", hashConnection, hashConnectionDiff, d);

export const hashPieceDiff = (d: PieceDiff): string => {
  const w = new HashWriter();
  w.writeString("PieceDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableHash(w, "center", d.center, hashCoord);
  writeNullableString(w, "color", d.color);
  writeNullableString(w, "description", d.description);
  writeNullableId(w, "design", d.design);
  writeNullableBool(w, "isHidden", d.isHidden);
  writeNullableBool(w, "isLocked", d.isLocked);
  writeNullableHash(w, "mirrorPlane", d.mirrorPlane, hashPlane);
  writeNullableString(w, "name", d.name);
  writeNullableHash(w, "plane", d.plane, hashPlaneDiff);
  writeNullableHash(w, "props", d.props, hashPropsDiff);
  writeNullableNumber(w, "scale", d.scale);
  writeNullableId(w, "type", d.type);
  return w.digest();
};

export const hashPiecesDiff = (d: PiecesDiff): string => hashCollectionDiffGeneric("PiecesDiff", "PieceDiffUpdate", "piece", hashPiece, hashPieceDiff, d);

export const hashDesignDiff = (d: DesignDiff): string => {
  const w = new HashWriter();
  w.writeString("DesignDiff");
  writeNullableId(w, "activeLayer", d.activeLayer);
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableHash(w, "authors", d.authors, hashAuthorsDiff);
  writeNullableBool(w, "canMirror", d.canMirror);
  writeNullableBool(w, "canScale", d.canScale);
  writeNullableIdArray(w, "concepts", d.concepts);
  writeNullableHash(w, "connections", d.connections, hashConnectionsDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "folder", d.folder);
  writeNullableHash(w, "groups", d.groups, hashGroupsDiff);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "image", d.image);
  writeNullableBool(w, "isAbstract", d.isAbstract);
  writeNullableHash(w, "layers", d.layers, hashLayersDiff);
  writeNullableId(w, "location", d.location);
  writeNullableString(w, "name", d.name);
  writeNullableId(w, "parent", d.parent);
  writeNullableHash(w, "pieces", d.pieces, hashPiecesDiff);
  writeNullableHash(w, "props", d.props, hashPropsDiff);
  writeNullableHash(w, "stats", d.stats, hashStatsDiff);
  writeNullableString(w, "unit", d.unit);
  return w.digest();
};

export const hashDesignsDiff = (d: DesignsDiff): string => hashCollectionDiffGeneric("DesignsDiff", "DesignDiffUpdate", "design", hashDesign, hashDesignDiff, d);

export const hashKitDiff = (d: KitDiff): string => {
  const w = new HashWriter();
  w.writeString("KitDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableHash(w, "authors", d.authors, hashAuthorsDiff);
  writeNullableHash(w, "concepts", d.concepts, hashConceptsDiff);
  writeNullableString(w, "description", d.description);
  writeNullableHash(w, "designs", d.designs, hashDesignsDiff);
  writeNullableHash(w, "files", d.files, hashFilesDiff);
  writeNullableHash(w, "folders", d.folders, hashFoldersDiff);
  writeNullableString(w, "homepage", d.homepage);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "image", d.image);
  writeNullableString(w, "license", d.license);
  writeNullableString(w, "name", d.name);
  writeNullableHash(w, "ports", d.ports, hashPortsDiff);
  writeNullableString(w, "preview", d.preview);
  writeNullableHash(w, "qualities", d.qualities, hashQualitiesDiff);
  writeNullableString(w, "remote", d.remote);
  writeNullableHash(w, "tags", d.tags, hashTagsDiff);
  writeNullableHash(w, "types", d.types, hashTypesDiff);
  writeNullableString(w, "version", d.version);
  return w.digest();
};

// #endregion ⚗️Hash Diff Entities

// #endregion 🔗Hash Diffs

// #endregion 🖥️Hash

/**
 * Computes the forward and backward diffs between two design states.
 **/
export const getDesignChange = (before: Design, after: Design): DesignChange => {
  const forward = getDesignDiff(before, after);
  const backward = inverseDesignDiff(before, forward);
  return { forward, backward };
};

/**
 * Zod schema for Kits diff validation.
 **/
export const KitsDiffSchema = z.object({
  removed: z.array(KitIdSchema).optional(),
  updated: z.array(z.object({ kit: KitIdSchema, diff: KitDiffSchema })).optional(),
  added: z.array(KitSchema).optional(),
});

/**
 * Adds a TypeToKit element.
 **/
export const addTypeToKit = (kit: Kit, type: Type): KitChange => {
  const forward: KitDiff = { types: { added: [type] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing TypeInKit element.
 **/
export const setTypeInKit = (kit: Kit, type: Type): KitChange => {
  const forward: KitDiff = { types: { added: [type] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a TypeFromKit element.
 **/
export const removeTypeFromKit = (kit: Kit, typeGuid: string): KitChange => {
  const forward: KitDiff = { types: { removed: [{ guid: typeGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Adds a DesignToKit element.
 **/
export const addDesignToKit = (kit: Kit, design: Design): KitChange => {
  const forward: KitDiff = { designs: { added: [design] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing DesignInKit element.
 **/
export const setDesignInKit = (kit: Kit, design: Design): KitChange => {
  const forward: KitDiff = { designs: { added: [design] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a DesignFromKit element.
 **/
export const removeDesignFromKit = (kit: Kit, designGuid: string): KitChange => {
  const forward: KitDiff = { designs: { removed: [{ guid: designGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 **/
export const updateDesignInKit = (kit: Kit, design: Design): KitChange => {
  const forward: KitDiff = { designs: { added: [design] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Adds a PortToKit element.
 **/
export const addPortToKit = (kit: Kit, iface: Port): KitChange => {
  const forward: KitDiff = { ports: { added: [iface] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing PortInKit element.
 **/
export const setPortInKit = (kit: Kit, iface: Port): KitChange => {
  const forward: KitDiff = { ports: { added: [iface] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a PortFromKit element.
 **/
export const removePortFromKit = (kit: Kit, portGuid: string): KitChange => {
  const forward: KitDiff = { ports: { removed: [{ guid: portGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 **/
export const updatePortInKit = (kit: Kit, iface: Port): KitChange => {
  const forward: KitDiff = { ports: { added: [iface] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching FileInKit entry.
 **/
export const findFileInKit = (kit: Kit, fileGuid: string): File => {
  const file = (kit.files || []).find((f) => f.guid === fileGuid);
  if (!file) throw new Error(`File ${fileGuid} not found in kit`);
  return file;
};

/**
 * Adds a FileToKit element.
 **/
export const addFileToKit = (kit: Kit, file: File): KitChange => {
  const forward: KitDiff = { files: { added: [file] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing FileInKit element.
 **/
export const setFileInKit = (kit: Kit, file: File): KitChange => {
  const forward: KitDiff = { files: { added: [file] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a FileFromKit element.
 **/
export const removeFileFromKit = (kit: Kit, fileGuid: string): KitChange => {
  const forward: KitDiff = { files: { removed: [{ guid: fileGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Replaces an existing AttributeInKit element.
 **/
export const setAttributeInKit = (kit: Kit, attribute: Attribute): KitChange => {
  const forward: KitDiff = { attributes: { added: [attribute] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching TagInKit entry.
 **/
export const findTagInKit = (kit: Kit, tagGuid: string): Tag => {
  const tag = (kit.tags || []).find((t) => t.guid === tagGuid);
  if (!tag) throw new Error(`Tag ${tagGuid} not found in kit`);
  return tag;
};

/**
 * Adds a TagToKit element.
 **/
export const addTagToKit = (kit: Kit, tag: Tag): KitChange => {
  const forward: KitDiff = { tags: { added: [tag] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing TagInKit element.
 **/
export const setTagInKit = (kit: Kit, tag: Tag): KitChange => {
  const forward: KitDiff = { tags: { added: [tag] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a TagFromKit element.
 **/
export const removeTagFromKit = (kit: Kit, tagGuid: string): KitChange => {
  const forward: KitDiff = { tags: { removed: [{ guid: tagGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching ConceptInKit entry.
 **/
export const findConceptInKit = (kit: Kit, conceptGuid: string): Concept => {
  const concept = (kit.concepts || []).find((c) => c.guid === conceptGuid);
  if (!concept) throw new Error(`Concept ${conceptGuid} not found in kit`);
  return concept;
};

/**
 * Adds a ConceptToKit element.
 **/
export const addConceptToKit = (kit: Kit, concept: Concept): KitChange => {
  const forward: KitDiff = { concepts: { added: [concept] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Replaces an existing ConceptInKit element.
 **/
export const setConceptInKit = (kit: Kit, concept: Concept): KitChange => {
  const forward: KitDiff = { concepts: { added: [concept] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};
/**
 * Removes a ConceptFromKit element.
 **/
export const removeConceptFromKit = (kit: Kit, conceptGuid: string): KitChange => {
  const forward: KitDiff = { concepts: { removed: [{ guid: conceptGuid }] } };
  const backward = inverseKitDiff(kit, forward);
  return { forward, backward };
};

/**
 * Searches for matching ReplacableDesignsForDesignPiece entry.
 **/
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

/**
 * Equality check for Kit values.
 **/
export const areSameKit = (kitGuid: string, otherGuid: string): boolean => {
  return kitGuid === otherGuid;
};
/**
 * Checks whether SameKit condition holds.
 **/
export const hasSameKit = (kitGuid: string, otherGuids: string[]): boolean => otherGuids.some((other) => areSameKit(kitGuid, other));

/**
 * Searches for matching TypeInKit entry.
 **/
export const findTypeInKit = (kit: Kit, typeGuid: string): Type => {
  const type = kit.types?.find((t) => t.guid === typeGuid);
  if (!type) throw new Error(`Type ${typeGuid} not found in kit ${kit.name}`);
  return type;
};

/**
 * Searches for matching DesignInKit entry.
 **/
export const findDesignInKit = (kit: Kit, designGuid: string): Design => {
  const design = kit.designs?.find((d) => d.guid === designGuid);
  if (!design) throw new Error(`Design ${designGuid} not found in kit ${kit.name}`);
  return design;
};

/**
 * Glob filter with include and exclude patterns for name-based entity filtering.
 * If include is non-empty, only names matching at least one include pattern are kept.
 * Names matching any exclude pattern are always removed.
 **/
export type GlobFilter = {
  include?: string[];
  exclude?: string[];
};

/**
 * General-purpose kit filter combining design-based transitive filtering with glob-based name filtering.
 * When designGuid is set, first performs transitive design-scoped filtering.
 * Glob filters on each entity kind are applied afterwards (or directly if no designGuid).
 **/
export type KitFilter = {
  designGuid?: string;
  modelTags?: string[];
  designs?: GlobFilter;
  types?: GlobFilter;
  ports?: GlobFilter;
  files?: GlobFilter;
  tags?: GlobFilter;
  concepts?: GlobFilter;
  qualities?: GlobFilter;
  authors?: GlobFilter;
  folders?: GlobFilter;
};

/**
 * Matches a name against a glob pattern supporting * (any chars) and ? (single char). Case-insensitive.
 **/
export const globMatch = (name: string, pattern: string): boolean => {
  let regex = "^";
  for (const c of pattern) {
    if (c === "*") regex += ".*";
    else if (c === "?") regex += ".";
    else regex += c.replace(/[-/\\^$+.()|[\]{}]/g, "\\$&");
  }
  regex += "$";
  return new RegExp(regex, "i").test(name);
};

/**
 * Checks if a name passes a GlobFilter. Returns true if no filter or name matches include and not exclude.
 **/
export const matchesGlobFilter = (name: string, filter?: GlobFilter): boolean => {
  if (!filter) return true;
  const { include, exclude } = filter;
  if (include && include.length > 0 && !include.some((p) => globMatch(name, p))) return false;
  if (exclude && exclude.length > 0 && exclude.some((p) => globMatch(name, p))) return false;
  return true;
};

/**
 * Internal design-based transitive kit filtering. Produces a minimal kit subset scoped to a single design.
 **/
const filterKitByDesign = (kit: Kit, designGuid: string, modelTags?: string[]): Kit => {
  const design = findDesignInKit(kit, designGuid);

  const usedTypeGuids = new Set<string>();
  const usedDesignGuids = new Set<string>([designGuid]);
  for (const piece of design.pieces ?? []) {
    if (piece.type?.guid) usedTypeGuids.add(piece.type.guid);
    if (piece.design?.guid) usedDesignGuids.add(piece.design.guid);
  }

  const typeByGuid = new Map((kit.types ?? []).map((type) => [type.guid, type]));
  const collectAncestors = (typeGuid: string) => {
    const type = typeByGuid.get(typeGuid);
    if (!type?.parent?.guid || usedTypeGuids.has(type.parent.guid)) return;
    usedTypeGuids.add(type.parent.guid);
    collectAncestors(type.parent.guid);
  };
  for (const typeGuid of [...usedTypeGuids]) collectAncestors(typeGuid);

  const tags = modelTags;
  const resolvedTagGuids = (tags ?? []).flatMap((tagValue) => {
    const byGuid = (kit.tags ?? []).find((tag) => tag.guid === tagValue);
    if (byGuid) return [byGuid.guid];
    return (kit.tags ?? []).filter((tag) => tag.name === tagValue).map((tag) => tag.guid);
  });

  const usedPortGuids = new Set<string>();
  const usedFileGuids = new Set<string>();
  const usedTagGuids = new Set<string>();
  const usedConceptGuids = new Set<string>();
  const usedQualityGuids = new Set<string>();
  const usedAuthorGuids = new Set<string>();
  const usedFolderNames = new Set<string>();
  const selectedModels = new Map<string, Model>();

  const collectQualityFromProps = (props?: Array<{ quality?: { guid: string } }>) => {
    for (const prop of props ?? []) {
      if (prop.quality?.guid) usedQualityGuids.add(prop.quality.guid);
    }
  };

  for (const typeGuid of usedTypeGuids) {
    const type = typeByGuid.get(typeGuid);
    if (!type) continue;
    if (type.folder) usedFolderNames.add(type.folder);
    for (const connector of type.connectors ?? []) {
      if (connector.port?.guid) usedPortGuids.add(connector.port.guid);
      collectQualityFromProps(connector.props);
    }
    collectQualityFromProps(type.props);
    for (const author of type.authors ?? []) if (author.guid) usedAuthorGuids.add(author.guid);
    for (const concept of type.concepts ?? []) if (concept.guid) usedConceptGuids.add(concept.guid);
    const selectedModel = selectBestModel(type.models ?? [], resolvedTagGuids);
    if (selectedModel) {
      selectedModels.set(typeGuid, selectedModel);
      if (selectedModel.file?.guid) usedFileGuids.add(selectedModel.file.guid);
      for (const tag of selectedModel.tags ?? []) if (tag.guid) usedTagGuids.add(tag.guid);
    }
  }

  for (const piece of design.pieces ?? []) collectQualityFromProps(piece.props);
  for (const concept of design.concepts ?? []) if (concept.guid) usedConceptGuids.add(concept.guid);
  for (const author of design.authors ?? []) if (author.guid) usedAuthorGuids.add(author.guid);
  for (const portGuid of [...usedPortGuids]) {
    const port = (kit.ports ?? []).find((candidate) => candidate.guid === portGuid);
    for (const compatible of port?.compatiblePorts ?? []) if (compatible.guid) usedPortGuids.add(compatible.guid);
  }
  for (const tagGuid of resolvedTagGuids) usedTagGuids.add(tagGuid);

  return {
    guid: kit.guid,
    name: kit.name,
    version: kit.version,
    description: kit.description,
    icon: kit.icon,
    image: kit.image,
    preview: kit.preview,
    remote: kit.remote,
    homepage: kit.homepage,
    license: kit.license,
    types: (kit.types ?? [])
      .filter((type) => usedTypeGuids.has(type.guid))
      .map((type) => ({
        ...type,
        models: selectedModels.has(type.guid) ? [selectedModels.get(type.guid)!] : [],
      })),
    designs: (kit.designs ?? []).filter((candidate) => usedDesignGuids.has(candidate.guid)),
    ports: (kit.ports ?? []).filter((port) => usedPortGuids.has(port.guid)),
    files: (kit.files ?? []).filter((file) => usedFileGuids.has(file.guid)),
    tags: (kit.tags ?? []).filter((tag) => usedTagGuids.has(tag.guid)),
    concepts: (kit.concepts ?? []).filter((concept) => usedConceptGuids.has(concept.guid)),
    qualities: (kit.qualities ?? []).filter((quality) => usedQualityGuids.has(quality.guid)),
    folders: (kit.folders ?? []).filter((folder) => usedFolderNames.has(folder.name)),
    authors: (kit.authors ?? []).filter((author) => usedAuthorGuids.has(author.guid)),
    attributes: kit.attributes,
    createdAt: kit.createdAt,
    updatedAt: kit.updatedAt,
  };
};

/**
 * General-purpose kit filter. Combines optional design-based transitive filtering with glob-based name filtering.
 * When designGuid is set, first performs transitive design-scoped subset extraction.
 * Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
 **/
export const filterKit = (kit: Kit, filter: KitFilter): Kit => {
  const base = filter.designGuid ? filterKitByDesign(kit, filter.designGuid, filter.modelTags) : kit;
  const hasGlobFilters = filter.designs || filter.types || filter.ports || filter.files || filter.tags || filter.concepts || filter.qualities || filter.authors || filter.folders;
  if (!hasGlobFilters) return base;
  return {
    ...base,
    types: (base.types ?? []).filter((t) => matchesGlobFilter(t.name, filter.types)),
    designs: (base.designs ?? []).filter((d) => matchesGlobFilter(d.name, filter.designs)),
    ports: (base.ports ?? []).filter((p) => matchesGlobFilter(p.name, filter.ports)),
    files: (base.files ?? []).filter((f) => matchesGlobFilter(f.name, filter.files)),
    tags: (base.tags ?? []).filter((t) => matchesGlobFilter(t.name, filter.tags)),
    concepts: (base.concepts ?? []).filter((c) => matchesGlobFilter(c.name, filter.concepts)),
    qualities: (base.qualities ?? []).filter((q) => matchesGlobFilter(q.name, filter.qualities)),
    authors: (base.authors ?? []).filter((a) => matchesGlobFilter(a.name, filter.authors)),
    folders: (base.folders ?? []).filter((f) => matchesGlobFilter(f.name, filter.folders)),
  };
};

// #region 📻Design Family Helpers
// Design family traversal helpers MUST be defined here.

/**
 * Retrieves the PrimitiveDesign value.
 **/
export const getPrimitiveDesign = (kit: Kit, designGuid: string): Design => {
  let current = findDesignInKit(kit, designGuid);
  while (current.parent?.guid) {
    current = findDesignInKit(kit, current.parent.guid);
  }
  return current;
};

/**
 * Retrieves the DesignFamily value.
 **/
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

/**
 * Retrieves the DesignSiblings value.
 **/
export const getDesignSiblings = (kit: Kit, designGuid: string): Design[] => {
  const design = findDesignInKit(kit, designGuid);
  const parentGuid = design.parent?.guid;
  return (kit.designs || []).filter((d) => d.parent?.guid === parentGuid && d.guid !== designGuid);
};

/**
 * Retrieves the DesignChildren value.
 **/
export const getDesignChildren = (kit: Kit, designGuid: string): Design[] => {
  return (kit.designs || []).filter((d) => d.parent?.guid === designGuid);
};

/**
 * Checks if Designs belong to the same family.
 **/
export const areDesignsInSameFamily = (kit: Kit, designGuidA: string, designGuidB: string): boolean => {
  const primitiveA = getPrimitiveDesign(kit, designGuidA);
  const primitiveB = getPrimitiveDesign(kit, designGuidB);
  return primitiveA.guid === primitiveB.guid;
};

/**
 * Checks if UseDesignAsPiece action is possible.
 **/
export const canUseDesignAsPiece = (kit: Kit, containerDesignGuid: string, pieceDesignGuid: string): boolean => {
  return !areDesignsInSameFamily(kit, containerDesignGuid, pieceDesignGuid);
};

/**
 * Searches for matching SameFamilyDesignPieces entry.
 **/
export const findSameFamilyDesignPieces = (kit: Kit, designGuid: string): Piece[] => {
  const design = findDesignInKit(kit, designGuid);
  return (design.pieces || []).filter((piece) => {
    if (!piece.design?.guid) return false;
    return areDesignsInSameFamily(kit, designGuid, piece.design.guid);
  });
};

// #endregion 📻Design Family Helpers

// #region 🧊Type Family Helpers
// Type family traversal helpers MUST be defined here.

/**
 * Retrieves the PrimitiveType value.
 **/
export const getPrimitiveType = (kit: Kit, typeGuid: string): Type => {
  let current = findTypeInKit(kit, typeGuid);
  while (current.parent?.guid) {
    current = findTypeInKit(kit, current.parent.guid);
  }
  return current;
};

/**
 * Retrieves the TypeFamily value.
 **/
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

/**
 * Retrieves the TypeSiblings value.
 **/
export const getTypeSiblings = (kit: Kit, typeGuid: string): Type[] => {
  const type = findTypeInKit(kit, typeGuid);
  const parentGuid = type.parent?.guid;
  return (kit.types || []).filter((t) => t.parent?.guid === parentGuid && t.guid !== typeGuid);
};

/**
 * Retrieves the TypeChildren value.
 **/
export const getTypeChildren = (kit: Kit, typeGuid: string): Type[] => {
  return (kit.types || []).filter((t) => t.parent?.guid === typeGuid);
};

/**
 * 👨‍👩‍👧‍👦 Checks if Types belong to the same family (have same primitive type).
 **/
export const areTypesInSameFamily = (kit: Kit, typeGuidA: string, typeGuidB: string): boolean => {
  const primitiveA = getPrimitiveType(kit, typeGuidA);
  const primitiveB = getPrimitiveType(kit, typeGuidB);
  return primitiveA.guid === primitiveB.guid;
};

// #endregion 🧊Type Family Helpers

// #region 🎯OperationResult
/**
 * Human-readable note attached to an algorithm {@link OperationResult} (warning, info, or error).
 **/
export interface OperationNote {
  /** Stable machine id e.g. flatten.no-fixed-piece-in-clump */
  code?: string;
  message: string;
}

/**
 * Successful operation: produced change plus non-fatal warnings and informational notes.
 **/
export interface OperationOk<Change> {
  ok: true;
  change: Change;
  warnings: OperationNote[];
  infos: OperationNote[];
}

/**
 * Failed operation: no change; carries one or more errors.
 **/
export interface OperationErr {
  ok: false;
  errors: OperationNote[];
}

/**
 * Discriminated union returned by semio algorithms: either ok with change or failed with errors.
 **/
export type OperationResult<Change> = OperationOk<Change> | OperationErr;

/** {@link OperationResult} specialized for {@link DesignChange} (flatten, etc.). */
export type DesignOperationResult = OperationResult<DesignChange>;

/** {@link OperationResult} specialized for {@link DesignDiff}. */
export type DesignDiffOperationResult = OperationResult<DesignDiff>;

/**
 * Builds a successful {@link OperationResult}.
 **/
export const operationOk = <Change>(change: Change, warnings: OperationNote[] = [], infos: OperationNote[] = []): OperationOk<Change> => ({
  ok: true,
  change,
  warnings,
  infos,
});

/**
 * Builds a failed {@link OperationResult}.
 **/
export const operationErr = (errors: OperationNote[]): OperationErr => ({ ok: false, errors });

/**
 * Wraps a native/REST payload that may still be a bare change object into {@link DesignOperationResult}.
 **/
export const normalizeDesignFlattenResult = (raw: unknown): DesignOperationResult => {
  if (raw !== null && typeof raw === "object" && "ok" in raw) {
    return raw as DesignOperationResult;
  }
  return operationOk(raw as DesignChange, [], []);
};

/**
 * Wraps a native/REST payload that may still be a bare {@link DesignDiff} into {@link DesignDiffOperationResult}.
 **/
export const normalizeDesignDiffResult = (raw: unknown): DesignDiffOperationResult => {
  if (raw !== null && typeof raw === "object" && "ok" in raw) {
    return raw as DesignDiffOperationResult;
  }
  return operationOk(raw as DesignDiff, [], []);
};

/**
 * Wraps a native/REST payload that may still be a bare {@link Design} into {@link OperationResult}<{@link Design}>.
 **/
export const normalizeDesignCopyResult = (raw: unknown): OperationResult<Design> => {
  if (raw !== null && typeof raw === "object" && "ok" in raw) {
    return raw as OperationResult<Design>;
  }
  return operationOk(raw as Design, [], []);
};
// #endregion 🎯OperationResult

/**
 * Represents a bidirectional change between two Kit states.
 **/
export interface KitChange {
  forward: KitDiff;
  backward: KitDiff;
}
/**
 * Computes the forward and backward diffs between two kit states.
 **/
export const getKitChange = (before: Kit, after: Kit): KitChange => {
  const forward = getKitDiff(before, after);
  const backward = inverseKitDiff(before, forward);
  return { forward, backward };
};

/**
 * Represents a reversible design change with forward and backward diffs.
 **/
export interface DesignChange {
  forward: DesignDiff;
  backward: DesignDiff;
}

// #region 📦Kit Diff Validation
// Validates kit diffs before apply; optional heal trims ineffective operations.

/**
 * Outcome of {@link validateKitDiff}: errors block faithful apply, warnings flag suspicious but applicable diffs.
 **/
export interface KitDiffValidationResult {
  ok: boolean;
  errors: OperationNote[];
  warnings: OperationNote[];
  /** When `heal` was true, a copy of the diff with fixable operations removed. */
  diff?: KitDiff;
}

// KitDiffValidationCtx holds mutable state while validating a kit diff.
type KitDiffValidationCtx = {
  errors: OperationNote[];
  warnings: OperationNote[];
  heal: boolean;
  diff: KitDiff;
};

const kitDiffPush = (ctx: KitDiffValidationCtx, kind: "errors" | "warnings", code: string, message: string) => {
  ctx[kind].push({ code, message });
};

/** Generic collection diff shape used across kit, design, and type entities. */
type GuidCollDiff = {
  removed?: Array<{ guid: string }>;
  updated?: any[];
  added?: any[];
};

const collGetUpdatedId = (u: any, idKey: string): string => u?.[idKey]?.guid ?? "";

const validateGuidCollectionDiff = <TItem extends { guid: string }>(
  ctx: KitDiffValidationCtx,
  path: string,
  idKey: string,
  base: TItem[],
  raw: GuidCollDiff | undefined,
  onUpdated: (item: TItem, itemDiff: any, itemPath: string) => void,
): GuidCollDiff | undefined => {
  if (!raw) return undefined;
  const baseByGuid = new Map(base.map((i) => [i.guid, i]));
  const removedGuids = new Set((raw.removed ?? []).map((r) => r.guid));
  let healedRemoved = raw.removed ? [...raw.removed] : undefined;
  let healedUpdated = raw.updated ? [...raw.updated] : undefined;
  let healedAdded = raw.added ? [...raw.added] : undefined;

  const afterRemoveIds = new Set(base.filter((i) => !removedGuids.has(i.guid)).map((i) => i.guid));

  for (const r of raw.removed ?? []) {
    if (!baseByGuid.has(r.guid)) {
      kitDiffPush(ctx, "warnings", "kitdiff.remove.missing-target", `${path}: remove references missing ${idKey} ${r.guid}`);
      if (ctx.heal && healedRemoved) healedRemoved = healedRemoved.filter((x) => x.guid !== r.guid);
    }
  }

  const noopAddedByGuid = new Map<string, { guid: string }>();
  for (const a of raw.added ?? []) noopAddedByGuid.set(a.guid, a);

  for (const r of raw.removed ?? []) {
    const orig = baseByGuid.get(r.guid);
    const add = noopAddedByGuid.get(r.guid);
    if (orig && add && deepEqual(orig, add)) {
      kitDiffPush(ctx, "warnings", "kitdiff.cycle.noop-restore", `${path}: removed and re-added ${idKey} ${r.guid} are deeply equal (no effective change)`);
      if (ctx.heal) {
        if (healedRemoved) healedRemoved = healedRemoved.filter((x) => x.guid !== r.guid);
        if (healedAdded) healedAdded = healedAdded.filter((x) => x.guid !== r.guid);
      }
    }
  }

  const seenAdd = new Set<string>();
  for (const a of raw.added ?? []) {
    if (seenAdd.has(a.guid)) {
      kitDiffPush(ctx, "errors", "kitdiff.add.duplicate-in-diff", `${path}: duplicate added ${idKey} guid ${a.guid}`);
      if (ctx.heal && healedAdded) {
        const first = healedAdded.findIndex((x) => x.guid === a.guid);
        healedAdded = healedAdded.filter((x, i) => x.guid !== a.guid || i === first);
      }
    }
    seenAdd.add(a.guid);
    if (afterRemoveIds.has(a.guid)) {
      kitDiffPush(ctx, "errors", "kitdiff.add.duplicate-guid", `${path}: cannot add ${idKey} ${a.guid} that still exists after removes`);
      if (ctx.heal && healedAdded) healedAdded = healedAdded.filter((x) => x.guid !== a.guid);
    }
  }

  for (const u of raw.updated ?? []) {
    const gid = collGetUpdatedId(u, idKey);
    const p = `${path}.${idKey}[${gid}]`;
    if (!gid) {
      kitDiffPush(ctx, "errors", "kitdiff.update.bad-id", `${p}: missing ${idKey} id`);
      if (ctx.heal && healedUpdated) healedUpdated = healedUpdated.filter((x) => collGetUpdatedId(x, idKey) !== gid);
      continue;
    }
    if (!afterRemoveIds.has(gid)) {
      kitDiffPush(ctx, "errors", "kitdiff.update.missing-target", `${p}: update targets ${idKey} not present after removes`);
      if (ctx.heal && healedUpdated) healedUpdated = healedUpdated.filter((x) => collGetUpdatedId(x, idKey) !== gid);
      continue;
    }
    const item = baseByGuid.get(gid);
    if (!item) {
      kitDiffPush(ctx, "errors", "kitdiff.update.missing-base", `${p}: ${idKey} not found in base kit`);
      if (ctx.heal && healedUpdated) healedUpdated = healedUpdated.filter((x) => collGetUpdatedId(x, idKey) !== gid);
      continue;
    }
    onUpdated(item, u.diff, p);
  }

  if (!ctx.heal) return raw;
  const out: GuidCollDiff = {};
  if (healedRemoved && healedRemoved.length > 0) out.removed = healedRemoved;
  if (healedUpdated && healedUpdated.length > 0) out.updated = healedUpdated;
  if (healedAdded && healedAdded.length > 0) out.added = healedAdded;
  return Object.keys(out).length > 0 ? out : undefined;
};

const validateAttributesDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Attribute[], d: AttributesDiff | undefined): void => {
  validateGuidCollectionDiff(ctx, path, "attribute", base, d, (_item, _diff, _p) => { });
};

const validatePropsDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Prop[], qualities: Set<string>, d: PropsDiff | undefined): void => {
  validateGuidCollectionDiff(ctx, path, "prop", base, d, (item, diff, p) => {
    const q = (diff as PropDiff).quality?.guid ?? item.quality?.guid;
    if (q && !qualities.has(q)) kitDiffPush(ctx, "errors", "kitdiff.ref.quality-missing", `${p}: quality ${q} not in kit`);
    if ((diff as PropDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (diff as PropDiff).attributes);
  });
};

const validateModelDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Model[], files: Set<string>, d: ModelsDiff | undefined): void => {
  validateGuidCollectionDiff(ctx, path, "model", base, d, (item, diff, p) => {
    const fid = (diff as ModelDiff).file?.guid ?? item.file?.guid;
    if (fid && !files.has(fid)) kitDiffPush(ctx, "errors", "kitdiff.ref.file-missing", `${p}: model file ${fid} not in kit`);
    if ((diff as ModelDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (diff as ModelDiff).attributes);
  });
};

const validateConnectorDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Connector[], ports: Set<string>, qualities: Set<string>, d: ConnectorsDiff | undefined): void => {
  validateGuidCollectionDiff(ctx, path, "connector", base, d, (item, diff, p) => {
    const pg = (diff as ConnectorDiff).port?.guid ?? item.port?.guid;
    if (pg && !ports.has(pg)) kitDiffPush(ctx, "errors", "kitdiff.ref.port-missing", `${p}: connector port ${pg} not in kit`);
    if ((diff as ConnectorDiff).props) validatePropsDiffNested(ctx, `${p}.props`, item.props ?? [], qualities, (diff as ConnectorDiff).props);
    if ((diff as ConnectorDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (diff as ConnectorDiff).attributes);
  });
};

const validateTypeDiffNested = (
  ctx: KitDiffValidationCtx,
  path: string,
  item: Type,
  diff: TypeDiff,
  ctxRefs: { typeGuids: Set<string>; fileGuids: Set<string>; portGuids: Set<string>; conceptGuids: Set<string>; authorGuids: Set<string>; qualityGuids: Set<string> },
): void => {
  if (diff.parent?.guid) {
    if (!ctxRefs.typeGuids.has(diff.parent.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.type-parent-missing", `${path}: parent type ${diff.parent.guid} not in kit`);
    if (diff.parent.guid === item.guid) kitDiffPush(ctx, "errors", "kitdiff.ref.type-parent-self", `${path}: type cannot be its own parent`);
  }
  if (diff.models) validateModelDiffNested(ctx, `${path}.models`, item.models ?? [], ctxRefs.fileGuids, diff.models);
  if (diff.connectors) validateConnectorDiffNested(ctx, `${path}.connectors`, item.connectors ?? [], ctxRefs.portGuids, ctxRefs.qualityGuids, diff.connectors);
  if (diff.props) validatePropsDiffNested(ctx, `${path}.props`, item.props ?? [], ctxRefs.qualityGuids, diff.props);
  if (diff.attributes) validateAttributesDiffNested(ctx, `${path}.attributes`, item.attributes ?? [], diff.attributes);
  if (diff.concepts) {
    for (const c of diff.concepts ?? []) {
      if (c?.guid && !ctxRefs.conceptGuids.has(c.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.concept-missing", `${path}: concept ${c.guid} not in kit`);
    }
  }
  if (diff.authors) {
    for (const a of diff.authors ?? []) {
      if (a?.guid && !ctxRefs.authorGuids.has(a.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.author-missing", `${path}: author ${a.guid} not in kit`);
    }
  }
};

const validateBenchmarksDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Benchmark[], d: BenchmarksDiff | undefined): void => {
  validateGuidCollectionDiff(ctx, path, "benchmark", base, d, (_item, diff, p) => {
    if ((diff as BenchmarkDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, _item.attributes ?? [], (diff as BenchmarkDiff).attributes);
  });
};

const validateQualityDiffNested = (ctx: KitDiffValidationCtx, path: string, item: Quality, diff: QualityDiff): void => {
  if (diff.benchmarks) validateBenchmarksDiffNested(ctx, `${path}.benchmarks`, item.benchmarks ?? [], diff.benchmarks);
  if (diff.attributes) validateAttributesDiffNested(ctx, `${path}.attributes`, item.attributes ?? [], diff.attributes);
};

const simulatePiecesForDesign = (base: Design, d?: PiecesDiff): Piece[] => {
  if (!d) return base.pieces ?? [];
  return applyCollectionDiff("piece", base.pieces ?? [], d, applyPieceDiff);
};

const validateDesignDiffNested = (
  ctx: KitDiffValidationCtx,
  kit: Kit,
  path: string,
  design: Design,
  diff: DesignDiff,
  refs: { typeGuids: Set<string>; designGuids: Set<string>; qualityGuids: Set<string>; fileGuids: Set<string>; portGuids: Set<string>; conceptGuids: Set<string>; authorGuids: Set<string> },
): void => {
  if (diff.parent?.guid) {
    if (!refs.designGuids.has(diff.parent.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.design-parent-missing", `${path}: parent design ${diff.parent.guid} not in kit`);
    if (diff.parent.guid === design.guid) kitDiffPush(ctx, "errors", "kitdiff.ref.design-parent-self", `${path}: design cannot be its own parent`);
  }
  if (diff.concepts) {
    for (const c of diff.concepts ?? []) {
      if (c?.guid && !refs.conceptGuids.has(c.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.concept-missing", `${path}: concept ${c.guid} not in kit`);
    }
  }
  if (diff.authors !== undefined) {
    const da = diff.authors as unknown;
    if (Array.isArray(da)) {
      for (const a of da as Array<{ guid?: string }>) {
        if (a?.guid && !refs.authorGuids.has(a.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.author-missing", `${path}: author ${a.guid} not in kit`);
      }
    } else if (da !== null && typeof da === "object") {
      validateGuidCollectionDiff(ctx, `${path}.authors`, "author", kit.authors ?? [], da as GuidCollDiff, (item, adiff, p) => {
        if ((adiff as AuthorDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (adiff as AuthorDiff).attributes);
      });
    }
  }

  if (diff.pieces) {
    validateGuidCollectionDiff(ctx, `${path}.pieces`, "piece", design.pieces ?? [], diff.pieces, (item, pDiff, p) => {
      if ((pDiff as PieceDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (pDiff as PieceDiff).attributes);
      if ((pDiff as PieceDiff).props) validatePropsDiffNested(ctx, `${p}.props`, item.props ?? [], refs.qualityGuids, (pDiff as PieceDiff).props);
    });
    for (const a of diff.pieces.added ?? []) {
      const tg = a.type?.guid;
      if (tg && !refs.typeGuids.has(tg)) kitDiffPush(ctx, "errors", "kitdiff.ref.piece-type-missing", `${path}.pieces.added: type ${tg} not in kit`);
      const dg = a.design?.guid;
      if (dg && !refs.designGuids.has(dg)) kitDiffPush(ctx, "errors", "kitdiff.ref.piece-design-missing", `${path}.pieces.added: subdesign ${dg} not in kit`);
    }
  }

  const simPieces = simulatePiecesForDesign(design, diff.pieces);
  const pieceGuids = new Set(simPieces.map((p) => p.guid));

  if (diff.connections) {
    validateGuidCollectionDiff(ctx, `${path}.connections`, "connection", design.connections ?? [], diff.connections, (item, cDiff, p) => {
      if ((cDiff as ConnectionDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (cDiff as ConnectionDiff).attributes);
    });
    const checkSide = (side: Side, label: string, cpath: string) => {
      if (!pieceGuids.has(side.piece.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.connection-piece-missing", `${cpath}: ${label} piece ${side.piece.guid} not in design after piece diff`);
      if (side.designPiece?.guid && !pieceGuids.has(side.designPiece.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.connection-designpiece-missing", `${cpath}: ${label} designPiece ${side.designPiece.guid} not in design after piece diff`);
    };
    for (const a of diff.connections.added ?? []) {
      const cp = `${path}.connections.added[${a.guid}]`;
      checkSide(a.connected, "connected", cp);
      checkSide(a.connecting, "connecting", cp);
    }
    for (const u of diff.connections.updated ?? []) {
      const conn = design.connections?.find((c) => c.guid === (u as any).connection.guid);
      const merged = conn ? applyConnectionDiff(conn, u.diff as ConnectionDiff) : undefined;
      const cp = `${path}.connections.updated[${(u as any).connection.guid}]`;
      if (merged) {
        checkSide(merged.connected, "connected", cp);
        checkSide(merged.connecting, "connecting", cp);
      }
    }
  }

  if (diff.stats) {
    validateGuidCollectionDiff(ctx, `${path}.stats`, "stat", design.stats ?? [], diff.stats, (item, sdiff, p) => {
      const q = (sdiff as StatDiff).quality?.guid ?? item.quality?.guid;
      if (q && !refs.qualityGuids.has(q)) kitDiffPush(ctx, "errors", "kitdiff.ref.quality-missing", `${p}: stat quality ${q} not in kit`);
    });
  }
  if (diff.props) validatePropsDiffNested(ctx, `${path}.props`, design.props ?? [], refs.qualityGuids, diff.props);

  let simLayers = design.layers ?? [];
  if (diff.layers) {
    validateGuidCollectionDiff(ctx, `${path}.layers`, "layer", design.layers ?? [], diff.layers, (item, ldiff, p) => {
      if ((ldiff as LayerDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (ldiff as LayerDiff).attributes);
    });
    simLayers = applyCollectionDiff("layer", design.layers ?? [], diff.layers, applyLayerDiff);
  }
  const layerGuids = new Set(simLayers.map((l) => l.guid));
  const active = diff.activeLayer ?? design.activeLayer;
  if (active?.guid && !layerGuids.has(active.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.active-layer-missing", `${path}: activeLayer ${active.guid} not in layers after diff`);

  if (diff.groups) {
    validateGuidCollectionDiff(ctx, `${path}.groups`, "group", design.groups ?? [], diff.groups, (item, gdiff, p) => {
      if ((gdiff as GroupDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (gdiff as GroupDiff).attributes);
    });
    const checkGroupPieces = (g: Group, gp: string) => {
      for (const pid of g.pieces ?? []) {
        if (!pieceGuids.has(pid.guid)) kitDiffPush(ctx, "errors", "kitdiff.ref.group-piece-missing", `${gp}: piece ${pid.guid} not in design`);
      }
    };
    for (const a of diff.groups.added ?? []) checkGroupPieces(a, `${path}.groups.added[${a.guid}]`);
    for (const u of diff.groups.updated ?? []) {
      const g = design.groups?.find((x) => x.guid === (u as any).group.guid);
      if (g) {
        const ng = applyGroupDiff(g, u.diff as GroupDiff);
        checkGroupPieces(ng, `${path}.groups.updated[${(u as any).group.guid}]`);
      }
    }
  }

  if (diff.attributes) validateAttributesDiffNested(ctx, `${path}.attributes`, design.attributes ?? [], diff.attributes);
};

/**
 * Validates a {@link KitDiff} against a base {@link Kit}. Errors mean apply would skip or mis-apply operations; warnings flag redundant or suspicious edits.
 * With `heal`, returns a scrubbed diff copy with invalid operations removed where possible.
 **/
export const validateKitDiff = (kit: Kit, diff: KitDiff, heal: boolean): KitDiffValidationResult => {
  const working: KitDiff = heal ? (JSON.parse(JSON.stringify(diff)) as KitDiff) : diff;
  const ctx: KitDiffValidationCtx = { errors: [], warnings: [], heal, diff: working };

  const typeGuids = new Set((kit.types ?? []).map((t) => t.guid));
  const designGuids = new Set((kit.designs ?? []).map((d) => d.guid));
  const qualityGuids = new Set((kit.qualities ?? []).map((q) => q.guid));
  const fileGuids = new Set((kit.files ?? []).map((f) => f.guid));
  const portGuids = new Set((kit.ports ?? []).map((p) => p.guid));
  const conceptGuids = new Set((kit.concepts ?? []).map((c) => c.guid));
  const authorGuids = new Set((kit.authors ?? []).map((a) => a.guid));
  const refs = { typeGuids, designGuids, qualityGuids, fileGuids, portGuids, conceptGuids, authorGuids };

  if (ctx.diff.types) {
    ctx.diff.types = validateGuidCollectionDiff(ctx, "types", "type", kit.types ?? [], ctx.diff.types, (item, tdiff, p) => validateTypeDiffNested(ctx, p, item, tdiff as TypeDiff, refs));
  }
  if (ctx.diff.designs) {
    ctx.diff.designs = validateGuidCollectionDiff(ctx, "designs", "design", kit.designs ?? [], ctx.diff.designs, (item, ddiff, p) => validateDesignDiffNested(ctx, kit, p, item, ddiff as DesignDiff, refs));
  }
  if (ctx.diff.tags) ctx.diff.tags = validateGuidCollectionDiff(ctx, "tags", "tag", kit.tags ?? [], ctx.diff.tags, () => { });
  if (ctx.diff.concepts) ctx.diff.concepts = validateGuidCollectionDiff(ctx, "concepts", "concept", kit.concepts ?? [], ctx.diff.concepts, () => { });
  if (ctx.diff.ports) ctx.diff.ports = validateGuidCollectionDiff(ctx, "ports", "port", kit.ports ?? [], ctx.diff.ports, () => { });
  if (ctx.diff.qualities) {
    ctx.diff.qualities = validateGuidCollectionDiff(ctx, "qualities", "quality", kit.qualities ?? [], ctx.diff.qualities, (item, qdiff, p) => validateQualityDiffNested(ctx, p, item, qdiff as QualityDiff));
  }
  if (ctx.diff.files) ctx.diff.files = validateGuidCollectionDiff(ctx, "files", "file", kit.files ?? [], ctx.diff.files, () => { });
  if (ctx.diff.folders) {
    ctx.diff.folders = validateGuidCollectionDiff(ctx, "folders", "folder", kit.folders ?? [], ctx.diff.folders, (item, fdiff, p) => {
      const par = (fdiff as FolderDiff).parent?.guid ?? item.parent?.guid;
      if (par && !(kit.folders ?? []).some((f) => f.guid === par)) kitDiffPush(ctx, "errors", "kitdiff.ref.folder-parent-missing", `${p}: parent folder ${par} not in kit`);
      if ((fdiff as FolderDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (fdiff as FolderDiff).attributes);
    });
  }
  if (ctx.diff.authors) ctx.diff.authors = validateGuidCollectionDiff(ctx, "authors", "author", kit.authors ?? [], ctx.diff.authors, () => { });
  if (ctx.diff.attributes) validateAttributesDiffNested(ctx, "kit.attributes", kit.attributes ?? [], ctx.diff.attributes);

  const ok = ctx.errors.length === 0;
  return heal ? { ok, errors: ctx.errors, warnings: ctx.warnings, diff: ctx.diff } : { ok, errors: ctx.errors, warnings: ctx.warnings };
};

// #endregion 📦Kit Diff Validation

// #region 🛡️Validation

// #region 🗡️Validation Core Types

/**
 * Enumeration of EntityKind values.
 **/
export type EntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Model" | "Layer" | "Group" | "Stat";

/**
 * Interface defining DomainLocation structure.
 **/
export interface DomainLocation {
  entityKind: EntityKind;
  entityGuid?: Guid;
  field?: string;
}

/**
 * Interface defining Fix structure.
 **/
export interface Fix {
  title: string;
  diff?: KitDiff;
}

/**
 * Interface defining Problem structure.
 **/
export interface Problem {
  constraintId: string;
  message: string;
  location: DomainLocation;
  relatedGuids?: Guid[];
  fixes: Fix[];
}

/**
 * Interface defining ValidationResult structure.
 **/
export interface ValidationResult {
  problems: Problem[];
}

/**
 * Checks whether Errors condition holds.
 **/
export const hasErrors = (res: ValidationResult) => res.problems.length > 0;

// #endregion 🗡️Validation Core Types

// #region 🔍Validation Context And Engine
// Validation context construction and engine MUST be defined here.

/**
 * Interface defining ValidationContext structure.
 **/
export interface ValidationContext {
  kit: Kit;
  typesByGuid: Map<Guid, Type>;
  designsByGuid: Map<Guid, Design>;
  piecesByGuid: Map<Guid, { designGuid: Guid; piece: Piece }>;
  connectorsByTypeGuid: Map<Guid, Connector[]>;
  modelsByTypeGuid: Map<Guid, Model[]>;
}

/**
 * Constructs ValidationContext from components.
 **/
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

/**
 * Type alias for Constraint.
 **/
export type Constraint = (ctx: ValidationContext) => Problem[];

/**
 * Interface defining ValidationConfig structure.
 **/
export interface ValidationConfig {
  constraints?: Constraint[];
}

/**
 * Definition of defaultConstraints.
 **/
export let defaultConstraints: Constraint[] = [];

/**
 * Validates Kit against constraints.
 **/
export const validateKit = (kit: Kit, cfg: ValidationConfig = {}): ValidationResult => {
  const ctx = buildValidationContext(kit);
  const constraints = cfg.constraints ?? defaultConstraints;
  return { problems: constraints.flatMap((constraint) => constraint(ctx)) };
};

// #endregion 🔍Validation Context And Engine

// #region 📡Fix Helper
// Validation fix helper functions MUST be defined here.
// Validation fix helper functions MUST be defined here.

/**
 **/
export const semioMakeFix = (ctx: ValidationContext, title: string, mutate: (clone: Kit) => void): Fix => {
  const clone = JSON.parse(serializeKit(ctx.kit)) as Kit;
  mutate(clone);
  const diff = getKitDiff(ctx.kit, clone);
  return { title, diff };
};

// #endregion 📡Fix Helper

// #region 🔑GUID Update Helper
// GUID regeneration helper functions MUST be defined here.

// 🔑updateGuidEverywhere replaces an old GUID with a new GUID across all kit entities.
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

// #endregion 🔑GUID Update Helper

// #region 🔑Constraint: GUID Uniqueness
// GUID uniqueness constraint MUST be enforced here.

/**
 * Constraint validating GuidUniqueness rules.
 **/
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

// #endregion 🔑Constraint: GUID Uniqueness

// #region 🧱Constraint: Type Name Uniqueness
// Type name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating TypeNameUniqueness rules.
 **/
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

// #endregion 🧱Constraint: Type Name Uniqueness

// #region 📐Constraint: Design Name Uniqueness
// Design name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating DesignNameUniqueness rules.
 **/
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

// #endregion 📐Constraint: Design Name Uniqueness

// #region 🧩Constraint: Piece Name Uniqueness
// Piece name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating PieceNameUniqueness rules.
 **/
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

// #endregion 🧩Constraint: Piece Name Uniqueness

// #region 🔬Constraint: Quality Name Uniqueness
// Quality name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating QualityNameUniqueness rules.
 **/
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

// #endregion 🔬Constraint: Quality Name Uniqueness

// #region ⚓Constraint: Port Name Uniqueness
// Port name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating PortNameUniqueness rules.
 **/
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

// #endregion ⚓Constraint: Port Name Uniqueness

// #region 📄Constraint: File Name Uniqueness
// File name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating FileNameUniqueness rules.
 **/
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

// #endregion 📄Constraint: File Name Uniqueness

// #region 📁Constraint: Folder Name Uniqueness
// Folder name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating FolderNameUniqueness rules.
 **/
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

// #endregion 📁Constraint: Folder Name Uniqueness

// #region 🔌Constraint: Connector Name Uniqueness Within Type
// Connector name uniqueness within type constraint MUST be enforced here.

/**
 * Constraint validating ConnectorNameUniqueness rules.
 **/
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

// #endregion 🔌Constraint: Connector Name Uniqueness Within Type

// #region 🗿Constraint: Model Name Uniqueness Within Type
// Model name uniqueness within type constraint MUST be enforced here.

/**
 * Constraint validating ModelNameUniqueness rules.
 **/
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

// #endregion 🗿Constraint: Model Name Uniqueness Within Type

// #region 🎨Constraint: Layer Path Uniqueness Within Design
// Layer path uniqueness within design constraint MUST be enforced here.

/**
 * Constraint validating LayerPathUniqueness rules.
 **/
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

// #endregion 🎨Constraint: Layer Path Uniqueness Within Design

// #region 📐Constraint: Design Piece Same Family Constraint
// Design piece same family constraint MUST be enforced here.

/**
 * Constraint validating DesignPieceSameFamily rules.
 **/
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
      } catch { }
    });
  });
  return problems;
};
// 📐getPrimitiveDesignFromContext retrieves the primitive design for a piece type from validation context.
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

// #endregion 📐Constraint: Design Piece Same Family Constraint

// #region ✅Constraint Registration
// Constraint registration and default configurations MUST be defined here.

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

// #endregion ✅Constraint Registration

// #region 🌧️Validation Serialization
// Validation result serialization and deserialization MUST be defined here.

/**
 * Interface defining SerializableValidationFix structure.
 **/
export interface SerializableValidationFix {
  title: string;
  diff?: KitDiff;
}

/**
 * Interface defining SerializableProblem structure.
 **/
export interface SerializableProblem {
  constraintId: string;
  message: string;
  entityKind: string;
  entityGuid: string;
  fixes: SerializableValidationFix[];
}

/**
 * Interface defining SerializableValidationResult structure.
 **/
export interface SerializableValidationResult {
  problems: SerializableProblem[];
}

/**
 * Converts to ValidationResult representation.
 **/
export const toValidationResult = (result: ValidationResult): SerializableValidationResult => ({
  problems: result.problems.map((problem) => ({
    constraintId: problem.constraintId,
    message: problem.message,
    entityKind: problem.location?.entityKind ?? (problem as any).entityKind,
    entityGuid: problem.location?.entityGuid ?? (problem as any).entityGuid ?? "",
    fixes: problem.fixes.map((fix) => ({ title: fix.title, diff: fix.diff })),
  })),
});

/**
 * Serializes ValidationResult for transport.
 **/
export const serializeValidationResult = (result: ValidationResult): string => {
  const serializable = toValidationResult(result);
  serializable.problems.sort((a, b) => {
    const constraintCompare = a.constraintId.localeCompare(b.constraintId);
    if (constraintCompare !== 0) return constraintCompare;
    return a.entityGuid.localeCompare(b.entityGuid);
  });
  return JSON.stringify(serializable, null, 2);
};

/**
 * Parses ValidationResult from serialized input.
 **/
export const parseValidationResult = (json: string): SerializableValidationResult => JSON.parse(json);
// 🔑isGuid checks whether a string is a valid GUID format.
const isGuid = (s: string): boolean => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);

/**
 * Deep equality check for KitDiffs ignoring NewGuids entities.
 **/
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

/**
 * Deep equality check for ValidationResults entities.
 **/
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
      return fixA.title === fixB.title && areKitDiffsEqualIgnoringNewGuids(fixA.diff ?? {}, fixB.diff ?? {});
    });
  });
};

// #endregion 🌧️Validation Serialization

// #endregion 🛡️Validation

/**
 **/
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

// #region 🧿Kit Import/Export
// Kit serialization and deserialization functions MUST be defined here.

/**
 * Interface defining KitImportResult structure.
 **/
export interface KitImportResult {
  kit: Kit;
  kind?: KitKind;
  files?: Record<string, Uint8Array>;
}
// 🗄️cachedSqlJs caches the SQL.js WASM module for reuse.
let cachedSqlJs: any = null;
// 🗄️getSqlJs loads and returns the SQL.js WASM module.
export const getSqlJs = async () => {
  if (!cachedSqlJs) {
    const initSqlJs = (await import("sql.js")).default;
    try {
      const isNode = typeof process !== "undefined" && process.versions?.node;
      if (isNode) {
        const fs = await import("node:fs");
        const path = await import("path");
        const url = await import("url");
        const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
        const candidatePaths = [path.join(__dirname, "public", "sql-wasm.wasm"), path.join(__dirname, "..", "sketchpad", "public", "sql-wasm.wasm")];
        const wasmPath = candidatePaths.find((candidate) => fs.existsSync(candidate)) ?? candidatePaths[0];
        cachedSqlJs = await initSqlJs({
          locateFile: () => wasmPath,
        });
      } else {
        // Specs: Vite/Electron dev server serves `/sql-wasm.wasm` as HTML (SPA fallback) unless the asset is bundled.
        // Resolve the wasm from the hoisted `sql.js` package so `fetch` returns real WASM bytes and MIME checks pass.
        cachedSqlJs = await initSqlJs({
          locateFile: (file: string) => new URL(`../../node_modules/sql.js/dist/${file}`, import.meta.url).href,
        });
      }
    } catch (error) {
      console.error("Failed to initialize sql.js:", error);
      throw new Error("Failed to load SQLite database library.");
    }
  }
  return cachedSqlJs;
};
// buildFolderPath builds a slash-separated folder path from root to the given folder guid.
// 📁Uses proper mime type inferred from file extension.
const buildFolderPath = (kit: Kit, folderGuid: string): string => {
  const findFolder = (guid: string): Folder | undefined => (kit.folders || []).find((f) => f.guid === guid);
  const parts: string[] = [];
  let current = findFolder(folderGuid);
  while (current) {
    parts.unshift(current.name);
    current = current.parent?.guid ? findFolder(current.parent.guid) : undefined;
  }
  return parts.join("/");
};
// buildFilePath builds the full path of a kit file including its folder hierarchy.
// 🏗️Uses proper mime type inferred from file extension.
const buildFilePath = (kit: Kit, file: File): string => {
  if (file.folder?.guid) {
    const folderPath = buildFolderPath(kit, file.folder.guid);
    if (folderPath) return `${folderPath}/${file.name}`;
  }
  return file.name;
};
const bytesToUtf8 = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);
const hasZipSignature = (bytes: Uint8Array): boolean => bytes.length >= 4 && bytes[0] === 0x50 && bytes[1] === 0x4b && bytes[2] === 0x03 && bytes[3] === 0x04;
const collectKitFiles = (kit: Kit): Record<string, Uint8Array> => {
  const files: Record<string, Uint8Array> = {};
  for (const file of kit.files || []) {
    if (!file.blob) continue;
    const base64 = file.blob.startsWith("data:") ? file.blob.slice(file.blob.indexOf(",") + 1) : file.blob;
    const binary = atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
    files[buildFilePath(kit, file)] = bytes;
  }
  return files;
};
export const importFileKit = async (source: string | ArrayBuffer | Buffer | Blob): Promise<KitImportResult> => {
  let json: string;
  if (source instanceof Blob) {
    json = await source.text();
  } else if (typeof source === "string") {
    const trimmed = source.trim();
    if (trimmed.startsWith("{")) {
      json = trimmed;
    } else {
      const response = await fetch(source);
      if (!response.ok) {
        throw new Error(`Failed to fetch file kit from ${source}: ${response.statusText}`);
      }
      json = await response.text();
    }
  } else if (typeof Buffer !== "undefined" && source instanceof Buffer) {
    json = bytesToUtf8(new Uint8Array(source.buffer.slice(source.byteOffset, source.byteOffset + source.byteLength)));
  } else {
    json = bytesToUtf8(new Uint8Array(source));
  }
  return { kind: "file", kit: deserializeKit(json), files: {} };
};
export const exportFileKit = (kit: Kit): string => serializeKit(kit);
export const importArchiveKit = async (source: string | ArrayBuffer | Buffer | Blob): Promise<KitImportResult> => {
  const JSZip = (await import("jszip")).default;

  let arrayBuffer: ArrayBuffer;
  if (source instanceof Blob) {
    arrayBuffer = await source.arrayBuffer();
  } else if (typeof source === "string") {
    const response = await fetch(source);
    if (!response.ok) {
      throw new Error(`Failed to fetch archive kit from ${source}: ${response.statusText}`);
    }
    arrayBuffer = await response.arrayBuffer();
  } else if (typeof Buffer !== "undefined" && source instanceof Buffer) {
    arrayBuffer = source.buffer.slice(source.byteOffset, source.byteOffset + source.byteLength) as ArrayBuffer;
  } else {
    arrayBuffer = source as ArrayBuffer;
  }

  const zip = await JSZip.loadAsync(arrayBuffer);

  const dbFile = zip.file(".semio/kit.db");
  let kit: Kit;
  if (dbFile) {
    const dbArrayBuffer = await dbFile.async("arraybuffer");
    const SQL = await getSqlJs();
    const db = new SQL.Database(new Uint8Array(dbArrayBuffer));
    kit = await sqliteToKit(db);
    db.close();
  } else {
    const kitJsonFile = zip.file("kit.json");
    if (!kitJsonFile) {
      throw new Error("Invalid kit archive: missing .semio/kit.db or kit.json");
    }
    const kitJson = await kitJsonFile.async("string");
    kit = deserializeKit(kitJson);
  }

  const importedFiles: Record<string, Uint8Array> = {};
  const zipEntries = new Map<string, any>();
  for (const [path, zipEntry] of Object.entries(zip.files)) {
    if (!(zipEntry as any).dir && !path.startsWith(".semio/") && path !== "kit.json") {
      zipEntries.set(path, zipEntry);
    }
  }

  if (kit.files) {
    for (const file of kit.files) {
      const filePath = buildFilePath(kit, file);
      const zipEntry = zipEntries.get(filePath);
      if (zipEntry) {
        const arrayBuf = await (zipEntry as any).async("arraybuffer");
        const bytes = new Uint8Array(arrayBuf);
        importedFiles[filePath] = bytes;
        let binary = "";
        for (let i = 0; i < bytes.length; i++) binary += String.fromCharCode(bytes[i]);
        const ext = file.name.split(".").pop()?.toLowerCase() || "";
        const mimeMap: Record<string, string> = {
          stl: "model/stl",
          obj: "model/obj",
          glb: "model/gltf-binary",
          gltf: "model/gltf+json",
          "3dm": "model/vnd.3dm",
          png: "image/png",
          jpg: "image/jpeg",
          jpeg: "image/jpeg",
          svg: "image/svg+xml",
          pdf: "application/pdf",
          zip: "application/zip",
          json: "application/json",
          csv: "text/csv",
          txt: "text/plain",
        };
        const mime = mimeMap[ext] || "application/octet-stream";
        file.blob = `data:${mime};base64,${btoa(binary)}`;
      }
    }
  }

  return { kind: "archive", kit, files: importedFiles };
};
export const importRemoteKit = async (url: string): Promise<KitImportResult> => {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch remote kit from ${url}: ${response.statusText}`);
  }
  const contentType = response.headers.get("content-type")?.toLowerCase() || "";
  if (url.endsWith(".zip") || contentType.includes("zip") || contentType.includes("octet-stream")) {
    const archive = await importArchiveKit(await response.blob());
    return { ...archive, kind: "remote" };
  }
  const text = await response.text();
  if (text.trim().startsWith("{")) {
    const result = await importFileKit(text);
    return { ...result, kind: "remote" };
  }
  const archive = await importArchiveKit(new Blob([text]));
  return { ...archive, kind: "remote" };
};
export const editTemporaryKit = (kit: Kit, diff: KitDiff): Kit => applyKitDiff(kit, diff);

/**
 * Imports Kit from external source.
 **/
export const importKit = async (source: string | ArrayBuffer | Buffer | Blob): Promise<KitImportResult> => {
  if (typeof source === "string") {
    const trimmed = source.trim();
    if (trimmed.startsWith("http://") || trimmed.startsWith("https://") || trimmed.startsWith("blob:")) {
      return importRemoteKit(source);
    }
    if (trimmed.startsWith("{")) {
      return importFileKit(source);
    }
  }
  if (source instanceof Blob) {
    const header = new Uint8Array(await source.slice(0, 4).arrayBuffer());
    if (hasZipSignature(header)) {
      return importArchiveKit(source);
    }
    return importFileKit(source);
  }
  if (typeof Buffer !== "undefined" && source instanceof Buffer) {
    const header = new Uint8Array(source.buffer.slice(source.byteOffset, source.byteOffset + Math.min(source.byteLength, 4)));
    if (hasZipSignature(header)) {
      return importArchiveKit(source);
    }
  } else if (source instanceof ArrayBuffer) {
    const header = new Uint8Array(source.slice(0, 4));
    if (hasZipSignature(header)) {
      return importArchiveKit(source);
    }
  }
  try {
    return await importArchiveKit(source);
  } catch (archiveError) {
    if (typeof source !== "string") {
      throw archiveError;
    }
    return importFileKit(source);
  }
};

/**
 * Exports Kit to external format.
 **/
export const exportKit = async (kit: Kit): Promise<Blob> => {
  const JSZip = (await import("jszip")).default;

  const SQL = await getSqlJs();
  const db = new SQL.Database();

  await kitToSqlite(kit, db);

  const dbData = db.export();
  db.close();

  const zip = new JSZip();
  zip.file(".semio/kit.db", dbData);

  for (const [filePath, bytes] of Object.entries(collectKitFiles(kit))) {
    zip.file(filePath, bytes);
  }

  return await zip.generateAsync({ type: "blob" });
};

/**
 * Deep equality check for Kits entities.
 **/
export const areKitsEqual = (a: Kit, b: Kit): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined ? undefined : value);
  const normalizeNumeric = (value: number | undefined | null): number => (value === null || value === undefined ? 0 : value);
  const normalizeBoolean = (value: boolean | undefined): boolean | undefined => (value ? true : undefined);
  const floatEq = (a: number | undefined, b: number | undefined): boolean => {
    if (a === undefined && b === undefined) return true;
    if (a === undefined || b === undefined) return false;
    return Math.abs(a - b) < TOLERANCE;
  };

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
      if (!floatEq(connectorA.point.x, connectorB.point.x)) return false;
      if (!floatEq(connectorA.point.y, connectorB.point.y)) return false;
      if (!floatEq(connectorA.point.z, connectorB.point.z)) return false;
      if (!floatEq(connectorA.direction.x, connectorB.direction.x)) return false;
      if (!floatEq(connectorA.direction.y, connectorB.direction.y)) return false;
      if (!floatEq(connectorA.direction.z, connectorB.direction.z)) return false;
      if (!floatEq(connectorA.t, connectorB.t)) return false;
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
        if (!floatEq(pieceA.plane.origin.x, pieceB.plane.origin.x)) return false;
        if (!floatEq(pieceA.plane.origin.y, pieceB.plane.origin.y)) return false;
        if (!floatEq(pieceA.plane.origin.z, pieceB.plane.origin.z)) return false;
        if (!floatEq(pieceA.plane.xAxis.x, pieceB.plane.xAxis.x)) return false;
        if (!floatEq(pieceA.plane.xAxis.y, pieceB.plane.xAxis.y)) return false;
        if (!floatEq(pieceA.plane.xAxis.z, pieceB.plane.xAxis.z)) return false;
        if (!floatEq(pieceA.plane.yAxis.x, pieceB.plane.yAxis.x)) return false;
        if (!floatEq(pieceA.plane.yAxis.y, pieceB.plane.yAxis.y)) return false;
        if (!floatEq(pieceA.plane.yAxis.z, pieceB.plane.yAxis.z)) return false;
      } else if (pieceA.plane || pieceB.plane) {
        return false;
      }
      if (pieceA.center && pieceB.center) {
        if (!floatEq(pieceA.center.u, pieceB.center.u)) return false;
        if (!floatEq(pieceA.center.v, pieceB.center.v)) return false;
      } else if (pieceA.center || pieceB.center) {
        return false;
      }
      if (!floatEq(pieceA.scale, pieceB.scale)) return false;
      if (pieceA.mirrorPlane && pieceB.mirrorPlane) {
        if (!floatEq(pieceA.mirrorPlane.origin.x, pieceB.mirrorPlane.origin.x)) return false;
        if (!floatEq(pieceA.mirrorPlane.origin.y, pieceB.mirrorPlane.origin.y)) return false;
        if (!floatEq(pieceA.mirrorPlane.origin.z, pieceB.mirrorPlane.origin.z)) return false;
        if (!floatEq(pieceA.mirrorPlane.xAxis.x, pieceB.mirrorPlane.xAxis.x)) return false;
        if (!floatEq(pieceA.mirrorPlane.xAxis.y, pieceB.mirrorPlane.xAxis.y)) return false;
        if (!floatEq(pieceA.mirrorPlane.xAxis.z, pieceB.mirrorPlane.xAxis.z)) return false;
        if (!floatEq(pieceA.mirrorPlane.yAxis.x, pieceB.mirrorPlane.yAxis.x)) return false;
        if (!floatEq(pieceA.mirrorPlane.yAxis.y, pieceB.mirrorPlane.yAxis.y)) return false;
        if (!floatEq(pieceA.mirrorPlane.yAxis.z, pieceB.mirrorPlane.yAxis.z)) return false;
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
      if (!floatEq(connA.gap, connB.gap)) return false;
      if (!floatEq(connA.shift, connB.shift)) return false;
      if (!floatEq(connA.rise, connB.rise)) return false;
      if (!floatEq(connA.rotation, connB.rotation)) return false;
      if (!floatEq(connA.turn, connB.turn)) return false;
      if (!floatEq(connA.tilt, connB.tilt)) return false;
      if (!floatEq(connA.u, connB.u)) return false;
      if (!floatEq(connA.v, connB.v)) return false;
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

/**
 * Deep equality check for KitDiffs entities.
 **/
export const areKitDiffsEqual = (a: KitDiff, b: KitDiff): boolean => {
  const normalizeArray = <T>(arr: T[] | T | undefined | null): T[] => {
    if (!arr) return [];
    if (Array.isArray(arr)) return arr;
    return [arr as T];
  };
  const normalizeValue = (value: any): any => (value === null || value === "" || value === undefined ? undefined : value);
  const normalizeNumeric = (value: number | undefined | null): number => (value === null || value === undefined ? 0 : value);
  const normalizeBoolean = (value: boolean | undefined): boolean | undefined => (value ? true : undefined);
  const defaultZeroKeys = new Set(["x", "y", "z", "u", "v", "gap", "shift", "rise", "rotation", "turn", "tilt", "t"]);
  const defaultFalseKeys = new Set(["mandatory", "isHidden", "isLocked", "isAbstract", "virtual"]);
  const getComparableId = (value: unknown): string | undefined => {
    if (!value || typeof value !== "object") return undefined;
    const record = value as Record<string, any>;
    return record.guid ?? record.type?.guid ?? record.design?.guid ?? record.piece?.guid ?? record.connection?.guid ?? record.model?.guid ?? record.port?.guid ?? record.connector?.guid ?? record.prop?.guid ?? record.attribute?.guid;
  };
  const canonicalize = (value: unknown, key = ""): unknown => {
    if (value === null || value === undefined || value === "") return undefined;
    if (typeof value === "number") return defaultZeroKeys.has(key) && value === 0 ? undefined : value;
    if (typeof value === "boolean") return defaultFalseKeys.has(key) && value === false ? undefined : value;
    if (Array.isArray(value)) {
      const items = value
        .map((item) => canonicalize(item, key))
        .filter((item): item is Exclude<typeof item, undefined> => item !== undefined)
        .sort((left, right) => {
          const leftId = getComparableId(left);
          const rightId = getComparableId(right);
          return String(leftId ?? JSON.stringify(left)).localeCompare(String(rightId ?? JSON.stringify(right)));
        });
      return items.length > 0 ? items : undefined;
    }
    if (typeof value === "object") {
      const entries = Object.entries(value)
        .map(([entryKey, entryValue]) => [entryKey, canonicalize(entryValue, entryKey)] as const)
        .filter(([, entryValue]) => entryValue !== undefined)
        .sort(([leftKey], [rightKey]) => leftKey.localeCompare(rightKey));
      return entries.length > 0 ? Object.fromEntries(entries) : undefined;
    }
    return value;
  };
  if (JSON.stringify(canonicalize(a) ?? {}) === JSON.stringify(canonicalize(b) ?? {})) return true;
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
      if (normalizeValue(aa.description) !== normalizeValue(ab.description)) return false;
      if (normalizeNumeric(aa.point.x) !== normalizeNumeric(ab.point.x)) return false;
      if (normalizeNumeric(aa.point.y) !== normalizeNumeric(ab.point.y)) return false;
      if (normalizeNumeric(aa.point.z) !== normalizeNumeric(ab.point.z)) return false;
      if (normalizeNumeric(aa.direction.x) !== normalizeNumeric(ab.direction.x)) return false;
      if (normalizeNumeric(aa.direction.y) !== normalizeNumeric(ab.direction.y)) return false;
      if (normalizeNumeric(aa.direction.z) !== normalizeNumeric(ab.direction.z)) return false;
      if (normalizeNumeric(aa.t) !== normalizeNumeric(ab.t)) return false;
      if (normalizeBoolean(aa.mandatory) !== normalizeBoolean(ab.mandatory)) return false;
    }
    return true;
  };

  const areConnectorDiffEqual = (a?: ConnectorDiff, b?: ConnectorDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
    if (a.point && b.point) {
      if (normalizeNumeric(a.point.x) !== normalizeNumeric(b.point.x)) return false;
      if (normalizeNumeric(a.point.y) !== normalizeNumeric(b.point.y)) return false;
      if (normalizeNumeric(a.point.z) !== normalizeNumeric(b.point.z)) return false;
    } else if (a.point || b.point) {
      return false;
    }
    if (a.direction && b.direction) {
      if (normalizeNumeric(a.direction.x) !== normalizeNumeric(b.direction.x)) return false;
      if (normalizeNumeric(a.direction.y) !== normalizeNumeric(b.direction.y)) return false;
      if (normalizeNumeric(a.direction.z) !== normalizeNumeric(b.direction.z)) return false;
    } else if (a.direction || b.direction) {
      return false;
    }
    if (normalizeNumeric(a.t) !== normalizeNumeric(b.t)) return false;
    if (normalizeBoolean(a.mandatory) !== normalizeBoolean(b.mandatory)) return false;
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
      if (normalizeValue(aa.file?.guid) !== normalizeValue(ab.file?.guid)) return false;
      if (!arraysEqual(normalizeArray(aa.tags), normalizeArray(ab.tags))) return false;
    }
    return true;
  };

  const areModelDiffsEqual = (a?: ModelDiff, b?: ModelDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.file?.guid) !== normalizeValue(b.file?.guid)) return false;
    if (normalizeValue(a.description) !== normalizeValue(b.description)) return false;
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
    if (normalizeBoolean(a.isAbstract) !== normalizeBoolean(b.isAbstract)) return false;
    if (normalizeBoolean(a.virtual) !== normalizeBoolean(b.virtual)) return false;
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
        if (normalizeNumeric(a.plane.origin.x) !== normalizeNumeric(b.plane.origin.x)) return false;
        if (normalizeNumeric(a.plane.origin.y) !== normalizeNumeric(b.plane.origin.y)) return false;
        if (normalizeNumeric(a.plane.origin.z) !== normalizeNumeric(b.plane.origin.z)) return false;
      } else if (a.plane.origin || b.plane.origin) {
        return false;
      }
      if (a.plane.xAxis && b.plane.xAxis) {
        if (normalizeNumeric(a.plane.xAxis.x) !== normalizeNumeric(b.plane.xAxis.x)) return false;
        if (normalizeNumeric(a.plane.xAxis.y) !== normalizeNumeric(b.plane.xAxis.y)) return false;
        if (normalizeNumeric(a.plane.xAxis.z) !== normalizeNumeric(b.plane.xAxis.z)) return false;
      } else if (a.plane.xAxis || b.plane.xAxis) {
        return false;
      }
      if (a.plane.yAxis && b.plane.yAxis) {
        if (normalizeNumeric(a.plane.yAxis.x) !== normalizeNumeric(b.plane.yAxis.x)) return false;
        if (normalizeNumeric(a.plane.yAxis.y) !== normalizeNumeric(b.plane.yAxis.y)) return false;
        if (normalizeNumeric(a.plane.yAxis.z) !== normalizeNumeric(b.plane.yAxis.z)) return false;
      } else if (a.plane.yAxis || b.plane.yAxis) {
        return false;
      }
    } else if (a.plane || b.plane) {
      return false;
    }
    if (normalizeValue(a.scale) !== normalizeValue(b.scale)) return false;
    if (normalizeBoolean(a.isHidden) !== normalizeBoolean(b.isHidden)) return false;
    if (normalizeBoolean(a.isLocked) !== normalizeBoolean(b.isLocked)) return false;
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
    if (normalizeNumeric(a.gap) !== normalizeNumeric(b.gap)) return false;
    if (normalizeNumeric(a.shift) !== normalizeNumeric(b.shift)) return false;
    if (normalizeNumeric(a.rise) !== normalizeNumeric(b.rise)) return false;
    if (normalizeNumeric(a.rotation) !== normalizeNumeric(b.rotation)) return false;
    if (normalizeNumeric(a.turn) !== normalizeNumeric(b.turn)) return false;
    if (normalizeNumeric(a.tilt) !== normalizeNumeric(b.tilt)) return false;
    if (normalizeNumeric(a.u) !== normalizeNumeric(b.u)) return false;
    if (normalizeNumeric(a.v) !== normalizeNumeric(b.v)) return false;
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
// 📦sqliteToKit converts a SQLite database into a kit object.
export const sqliteToKit = async (db: any): Promise<Kit> => {
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

    const typeProps = safeExecResult("type_prop", "SELECT prop.* FROM prop JOIN type_prop ON prop.guid = type_prop.prop_guid WHERE type_prop.type_guid = ?", [typeGuid]);
    const props_value = (() => {
      const filtered = typeProps
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
    })();
    if (props_value) type.props = props_value;

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
        min: s.min_value ?? undefined,
        minExcluded: s.min_excluded ? true : undefined,
        max: s.max_value ?? undefined,
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
          kind: row.kind || undefined,
          defaultValue: row.default_value ?? undefined,
          formula: toUndefined(row.formula),
          defaultSiUnit: toUndefined(row.default_si_unit),
          defaultImperialUnit: toUndefined(row.default_imperial_unit),
          min: row.min_value ?? undefined,
          minExcluded: row.min_excluded ? true : undefined,
          max: row.max_value ?? undefined,
          maxExcluded: row.max_excluded ? true : undefined,
          canScale: row.can_scale ? true : undefined,
          uri: toUndefined(row.definition),
          benchmarks: benchmarks.map((b: any) => {
            const benchmarkAttributes = execResult("SELECT * FROM attribute WHERE benchmark_guid = ?", [b.guid]);
            return {
              guid: b.guid,
              name: b.name,
              icon: toUndefined(b.icon),
              min: b.min_value ?? undefined,
              minExcluded: b.min_excluded ? true : undefined,
              max: b.max_value ?? undefined,
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
        remote: toUndefined(row.remote_url),
        folder: row.folder_guid ? { guid: row.folder_guid } : undefined,
        size: row.size ?? undefined,
        hash: toUndefined(row.hash),
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
// 📚toArray holds the data fields for a toArray record.
const toArray = <T>(value: T | T[] | undefined): T[] => {
  if (!value) return [];
  return Array.isArray(value) ? value : [value];
};

/**
 * Constant value for KIT_SQLITE_SCHEMA.
 **/
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

CREATE TABLE type_prop (
	type_guid VARCHAR(36) NOT NULL,
	prop_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (type_guid, prop_guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid)
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

// 📦kitToSqlite converts a kit object into a SQLite database.
export const kitToSqlite = async (kit: Kit, db: any): Promise<void> => {
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
        quality.kind ?? 0,
        quality.defaultValue || null,
        quality.formula || null,
        quality.defaultSiUnit || null,
        quality.defaultImperialUnit || null,
        quality.min || null,
        quality.isMinExcluded ? 1 : null,
        quality.max || null,
        quality.isMaxExcluded ? 1 : null,
        quality.canScale ? 1 : 0,
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

    toArray(type.props).forEach((prop) => {
      const quality = toArray(kit.qualities).find((q) => q.guid === prop.quality.guid);
      const propKey = quality?.key || "";
      db.run("INSERT INTO prop (guid, key, value, unit, quality_guid) VALUES (?, ?, ?, ?, ?)", [prop.guid, propKey, prop.value, prop.unit || null, prop.quality.guid]);
      db.run("INSERT INTO type_prop (type_guid, prop_guid) VALUES (?, ?)", [type.guid, prop.guid]);
      toArray(prop.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value || null, attr.definition || null, prop.guid]);
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

// #endregion 🧿Kit Import/Export

// #region 🔩Kit Model Export
// Design model export to 3D formats (GLB, glTF, OBJ, STL, PLY, USDZ) MUST be defined here.

/**
 * Supported 3D export formats with their MIME types.
 **/
export const EXPORT_MODEL_FORMATS: Record<string, string> = {
  ".glb": "model/gltf-binary",
  ".gltf": "model/gltf+json",
  ".obj": "model/obj",
  ".stl": "model/stl",
  ".ply": "application/x-ply",
  ".usdz": "model/vnd.usdz+zip",
};

const SEMIO_TO_GLTF_BASIS = new THREE.Matrix4().set(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);

const SEMIO_TO_GLTF_BASIS_INV = SEMIO_TO_GLTF_BASIS.clone().invert();

const semioMatrixToGltfMatrix = (matrix: THREE.Matrix4): number[] => {
  const transformed = new THREE.Matrix4().multiplyMatrices(SEMIO_TO_GLTF_BASIS, matrix).multiply(SEMIO_TO_GLTF_BASIS_INV);
  return transformed.elements.slice();
};
const planeToGlbTransform = (plane: Plane): number[] => {
  return semioMatrixToGltfMatrix(planeToMatrix(plane));
};
const findMatchingModel = (kit: Kit, type: Type, tags: string[]): Model | undefined => {
  if (!type.models || type.models.length === 0) return undefined;
  const kitTags = kit.tags ?? [];
  const selectedTagGuids = tags.flatMap((tagValue) => {
    const byGuid = kitTags.find((tag) => tag.guid === tagValue);
    if (byGuid) return [byGuid.guid];
    return kitTags.filter((tag) => tag.name === tagValue).map((tag) => tag.guid);
  });
  return selectBestModel(type.models, selectedTagGuids);
};

/**
 * Decodes a base64 or data-URI blob string into a Uint8Array.
 **/
const decodeBlobToBytes = (blobStr: string): Uint8Array => {
  const base64 = blobStr.startsWith("data:") ? blobStr.slice(blobStr.indexOf(",") + 1) : blobStr;
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes;
};

const bytesToDataUri = (bytes: Uint8Array, mimeType: string): string => {
  const base64 = Buffer.from(bytes).toString("base64");
  return `data:${mimeType};base64,${base64}`;
};

const inlineJsonDocumentResources = (jsonDoc: {
  json: {
    buffers?: Array<{ uri?: string }>;
    images?: Array<{ uri?: string; mimeType?: string }>;
  };
  resources?: Record<string, Uint8Array | string>;
}) => {
  const resources = jsonDoc.resources ?? {};
  for (const buffer of jsonDoc.json.buffers ?? []) {
    if (!buffer.uri) continue;
    const resource = resources[buffer.uri];
    if (!resource) continue;
    const bytes = typeof resource === "string" ? new TextEncoder().encode(resource) : resource;
    buffer.uri = bytesToDataUri(bytes, "application/octet-stream");
  }
  for (const image of jsonDoc.json.images ?? []) {
    if (!image.uri) continue;
    const resource = resources[image.uri];
    if (!resource) continue;
    const bytes = typeof resource === "string" ? new TextEncoder().encode(resource) : resource;
    image.uri = bytesToDataUri(bytes, image.mimeType ?? "application/octet-stream");
  }
  return jsonDoc.json;
};

/**
 * Copies a texture from a source glTF document into a target document, using a cache to avoid duplicates.
 **/
const copyGltfTexture = (srcTex: GltfTexture, targetDoc: GltfDocument, textureCache: Map<GltfTexture, GltfTexture>): GltfTexture => {
  const cached = textureCache.get(srcTex);
  if (cached) return cached;
  const tex = targetDoc.createTexture(srcTex.getName());
  const img = srcTex.getImage();
  if (img) tex.setImage(new Uint8Array(img));
  tex.setMimeType(srcTex.getMimeType());
  tex.setURI(srcTex.getURI());
  textureCache.set(srcTex, tex);
  return tex;
};

/**
 * Copies a material from a source glTF document into a target document, including referenced textures.
 **/
const copyGltfMaterial = (srcMat: GltfMaterial, targetDoc: GltfDocument, textureCache: Map<GltfTexture, GltfTexture>): GltfMaterial => {
  const mat = targetDoc.createMaterial(srcMat.getName());
  mat.setBaseColorFactor(srcMat.getBaseColorFactor());
  mat.setMetallicFactor(srcMat.getMetallicFactor());
  mat.setRoughnessFactor(srcMat.getRoughnessFactor());
  mat.setEmissiveFactor(srcMat.getEmissiveFactor());
  mat.setAlphaMode(srcMat.getAlphaMode());
  mat.setAlphaCutoff(srcMat.getAlphaCutoff());
  mat.setDoubleSided(srcMat.getDoubleSided());

  const baseColorTex = srcMat.getBaseColorTexture();
  if (baseColorTex) mat.setBaseColorTexture(copyGltfTexture(baseColorTex, targetDoc, textureCache));
  const mrTex = srcMat.getMetallicRoughnessTexture();
  if (mrTex) mat.setMetallicRoughnessTexture(copyGltfTexture(mrTex, targetDoc, textureCache));
  const normalTex = srcMat.getNormalTexture();
  if (normalTex) mat.setNormalTexture(copyGltfTexture(normalTex, targetDoc, textureCache));
  const occlusionTex = srcMat.getOcclusionTexture();
  if (occlusionTex) mat.setOcclusionTexture(copyGltfTexture(occlusionTex, targetDoc, textureCache));
  const emissiveTex = srcMat.getEmissiveTexture();
  if (emissiveTex) mat.setEmissiveTexture(copyGltfTexture(emissiveTex, targetDoc, textureCache));

  return mat;
};

/**
 * Copies all meshes from a source glTF document into a target document, returning the list of copied meshes.
 **/
const copyGltfMeshes = (sourceDoc: GltfDocument, targetDoc: GltfDocument, targetBuffer: GltfBuffer, meshName?: string): GltfMesh[] => {
  const materialCache = new Map<GltfMaterial, GltfMaterial>();
  const textureCache = new Map<GltfTexture, GltfTexture>();
  const mesh = targetDoc.createMesh(meshName);

  for (const srcMesh of sourceDoc.getRoot().listMeshes()) {
    for (const srcPrim of srcMesh.listPrimitives()) {
      const prim = targetDoc.createPrimitive();

      for (const semantic of srcPrim.listSemantics()) {
        const srcAcc = srcPrim.getAttribute(semantic);
        if (!srcAcc) continue;
        const srcArray = srcAcc.getArray();
        if (!srcArray) continue;
        const acc = targetDoc
          .createAccessor(semantic)
          .setArray(srcArray.slice() as any)
          .setType(srcAcc.getType())
          .setBuffer(targetBuffer);
        if (srcAcc.getNormalized()) acc.setNormalized(true);
        prim.setAttribute(semantic, acc);
      }

      const srcIndices = srcPrim.getIndices();
      if (srcIndices) {
        const srcIdxArray = srcIndices.getArray();
        if (srcIdxArray) {
          const idxAcc = targetDoc
            .createAccessor("indices")
            .setArray(srcIdxArray.slice() as any)
            .setType(GltfAccessor.Type.SCALAR)
            .setBuffer(targetBuffer);
          prim.setIndices(idxAcc);
        }
      }

      const srcMat = srcPrim.getMaterial();
      if (srcMat) {
        let mat = materialCache.get(srcMat);
        if (!mat) {
          mat = copyGltfMaterial(srcMat, targetDoc, textureCache);
          materialCache.set(srcMat, mat);
        }
        prim.setMaterial(mat);
      }

      mesh.addPrimitive(prim);
    }
  }

  return mesh.listPrimitives().length > 0 ? [mesh] : [];
};

/**
 * Creates a unit box mesh (1x1x1 centered at origin) as a placeholder for types without models.
 **/
const createBoxMesh = (name: string, doc: GltfDocument, buffer: GltfBuffer): GltfMesh => {
  const s = 0.5;
  const positions = new Float32Array([
    -s,
    -s,
    s,
    s,
    -s,
    s,
    s,
    s,
    s,
    -s,
    s,
    s,
    -s,
    -s,
    -s,
    -s,
    s,
    -s,
    s,
    s,
    -s,
    s,
    -s,
    -s,
    -s,
    s,
    -s,
    -s,
    s,
    s,
    s,
    s,
    s,
    s,
    s,
    -s,
    -s,
    -s,
    -s,
    s,
    -s,
    -s,
    s,
    -s,
    s,
    -s,
    -s,
    s,
    s,
    -s,
    -s,
    s,
    s,
    -s,
    s,
    s,
    s,
    s,
    -s,
    s,
    -s,
    -s,
    -s,
    -s,
    -s,
    s,
    -s,
    s,
    s,
    -s,
    s,
    -s,
  ]);
  const normals = new Float32Array([
    0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0, -1, 0, 0,
  ]);
  const indices = new Uint16Array([0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23]);

  const posAcc = doc.createAccessor("POSITION").setArray(positions).setType(GltfAccessor.Type.VEC3).setBuffer(buffer);
  const normAcc = doc.createAccessor("NORMAL").setArray(normals).setType(GltfAccessor.Type.VEC3).setBuffer(buffer);
  const idxAcc = doc.createAccessor("indices").setArray(indices).setType(GltfAccessor.Type.SCALAR).setBuffer(buffer);

  const prim = doc.createPrimitive().setAttribute("POSITION", posAcc).setAttribute("NORMAL", normAcc).setIndices(idxAcc);

  return doc.createMesh(name).addPrimitive(prim);
};

/**
 * Exports the 3D model of a design to a specified format.
 * Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
 **/
export const exportDesignModel = async (kit: Kit, designId: string, format: string = ".glb", tags: string[] = [], options: Record<string, unknown> = {}): Promise<ArrayBuffer> => {
  const io = new NodeIO();
  const design = findDesignInKit(kit, designId);
  const pieces = design.pieces ?? [];
  const connections = design.connections ?? [];
  const types = kit.types ?? [];

  if (pieces.length === 0) {
    const emptyDoc = new GltfDocument();
    emptyDoc.createBuffer("main");
    emptyDoc.createScene("empty");
    const glb = await io.writeBinary(emptyDoc);
    return glb.buffer as ArrayBuffer;
  }

  const typesDict: Record<string, Type> = {};
  for (const t of types) typesDict[t.guid] = t;
  const piecesDict: Record<string, Piece> = {};
  for (const p of pieces) piecesDict[p.guid] = p;

  const adjacency: Record<string, Array<{ connection: Connection; neighborGuid: string }>> = {};
  for (const p of pieces) adjacency[p.guid] = [];
  for (const conn of connections) {
    const connectedGuid = conn.connected.piece.guid;
    const connectingGuid = conn.connecting.piece.guid;
    if (adjacency[connectedGuid]) adjacency[connectedGuid].push({ connection: conn, neighborGuid: connectingGuid });
    if (adjacency[connectingGuid]) adjacency[connectingGuid].push({ connection: conn, neighborGuid: connectedGuid });
  }

  const piecePlanes: Record<string, Plane> = {};
  const parentOf: Record<string, string> = {};
  const childrenOf: Record<string, string[]> = {};
  for (const p of pieces) childrenOf[p.guid] = [];

  const visited = new Set<string>();
  const roots: string[] = [];

  const getType = (typeGuid: string): Type | undefined => typesDict[typeGuid];
  const getConnector = (type: Type | undefined, connectorGuid: string | undefined): Connector | undefined => {
    if (!type) return undefined;
    if (!connectorGuid) return type.connectors?.[0];
    return type.connectors?.find((c) => c.guid === connectorGuid);
  };

  const queue: string[] = [];
  for (const p of pieces) {
    if (p.plane) {
      piecePlanes[p.guid] = p.plane;
      visited.add(p.guid);
      queue.push(p.guid);
      roots.push(p.guid);
    }
  }
  if (queue.length === 0 && pieces.length > 0) {
    const firstPiece = pieces[0];
    const identityPlane = matrixToPlane(new THREE.Matrix4().identity());
    piecePlanes[firstPiece.guid] = identityPlane;
    visited.add(firstPiece.guid);
    queue.push(firstPiece.guid);
    roots.push(firstPiece.guid);
  }

  while (queue.length > 0) {
    const currentGuid = queue.shift()!;
    const currentPlane = piecePlanes[currentGuid];
    for (const edge of adjacency[currentGuid] ?? []) {
      if (visited.has(edge.neighborGuid)) continue;
      const conn = edge.connection;
      const isParent = conn.connected.piece.guid === currentGuid;

      if (!isParent) continue;

      const parentGuid = currentGuid;
      const childGuid = edge.neighborGuid;
      const parentPiece = piecesDict[parentGuid];
      const childPiece = piecesDict[childGuid];
      const parentType = parentPiece.type ? getType(parentPiece.type.guid) : undefined;
      const childType = childPiece.type ? getType(childPiece.type.guid) : undefined;
      const parentConnector = getConnector(parentType, conn.connected.connector?.guid);
      const childConnector = getConnector(childType, conn.connecting.connector?.guid);

      if (parentConnector && childConnector) {
        const childPlane = computeChildPlane(currentPlane, parentConnector, childConnector, conn);
        piecePlanes[childGuid] = childPlane;
      } else {
        piecePlanes[childGuid] = currentPlane;
      }

      parentOf[childGuid] = parentGuid;
      childrenOf[parentGuid].push(childGuid);
      visited.add(childGuid);
      queue.push(childGuid);
    }
  }

  for (const p of pieces) {
    if (!visited.has(p.guid)) {
      piecePlanes[p.guid] = matrixToPlane(new THREE.Matrix4().identity());
      roots.push(p.guid);
    }
  }

  const doc = new GltfDocument();
  const buffer = doc.createBuffer("main");
  const scene = doc.createScene(design.name ?? "design");

  const typeMeshMap: Record<string, GltfMesh> = {};

  for (const piece of pieces) {
    const typeGuid = piece.type?.guid;
    if (!typeGuid || typeMeshMap[typeGuid] !== undefined) continue;

    const type = typesDict[typeGuid];
    if (!type) continue;

    const model = findMatchingModel(kit, type, tags);
    if (!model) {
      continue;
    }

    const file = kit.files?.find((f) => f.guid === model.file.guid);
    if (!file?.blob) continue;

    const fileBytes = decodeBlobToBytes(file.blob);
    const ext = file.name.split(".").pop()?.toLowerCase();

    if (ext === "glb") {
      try {
        const sourceDoc = await io.readBinary(fileBytes);
        const copiedMeshes = copyGltfMeshes(sourceDoc, doc, buffer, file.name);
        if (copiedMeshes.length > 0) {
          typeMeshMap[typeGuid] = copiedMeshes[0];
        }
      } catch { }
    }
  }

  const pieceNodeMap: Record<string, GltfNode> = {};

  const buildNode = (pieceGuid: string): GltfNode => {
    if (pieceNodeMap[pieceGuid]) return pieceNodeMap[pieceGuid];

    const piece = piecesDict[pieceGuid];
    const worldPlane = piecePlanes[pieceGuid];
    const parentGuid = parentOf[pieceGuid];
    const children = childrenOf[pieceGuid] ?? [];

    let localMatrix: number[];
    if (parentGuid && piecePlanes[parentGuid]) {
      const parentWorld = planeToMatrix(piecePlanes[parentGuid]);
      const childWorld = planeToMatrix(worldPlane);
      const invParent = parentWorld.clone().invert();
      const localMat = new THREE.Matrix4().multiplyMatrices(invParent, childWorld);
      localMatrix = semioMatrixToGltfMatrix(localMat);
    } else {
      localMatrix = planeToGlbTransform(worldPlane);
    }

    const node = doc.createNode(piece.name ?? piece.guid);
    node.setMatrix(localMatrix as any);

    const typeGuid = piece.type?.guid;
    if (typeGuid && typeMeshMap[typeGuid]) {
      node.setMesh(typeMeshMap[typeGuid]);
    }

    for (const childGuid of children) {
      node.addChild(buildNode(childGuid));
    }

    pieceNodeMap[pieceGuid] = node;
    return node;
  };

  for (const rootGuid of roots) {
    scene.addChild(buildNode(rootGuid));
  }

  if (format === ".gltf") {
    const jsonDoc = await io.writeJSON(doc);
    const encoder = new TextEncoder();
    return encoder.encode(JSON.stringify(inlineJsonDocumentResources(jsonDoc))).buffer as ArrayBuffer;
  }

  const glb = await io.writeBinary(doc);
  return glb.buffer as ArrayBuffer;
};

// #endregion 🔩Kit Model Export

// #region ❄️Geometric Insights
// Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.

/**
 * 🔷Geometric KPIs for a GLB/GLTF model in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).
 */
export interface GeometricInsights {
  boundingBoxMin?: Point;
  boundingBoxMax?: Point;
  dimensionX?: number;
  dimensionY?: number;
  dimensionZ?: number;
  characteristicLength?: number;
  footprintArea?: number;
  totalSurfaceArea?: number;
  enclosedVolume?: number;
  surfaceToVolumeRatio?: number;
  sphericity?: number;
  hullFillRatio?: number;
  aspectRatioXy?: number;
  aspectRatioXz?: number;
  aspectRatioYz?: number;
  slenderness?: number;
  centroid?: Point;
  principalAxes?: [Point, Point, Point];
  momentsOfInertia?: [number, number, number];
  vertexCount?: number;
  faceCount?: number;
  eulerCharacteristic?: number;
  genus?: number;
  isWatertight?: boolean;
  convexHullVolume?: number;
  concavityIndex?: number;
}

function glbToSemioPoint(xg: number, yg: number, _zg: number): Point {
  return { x: xg, y: -xg, z: yg };
}

function triangleArea(a: THREE.Vector3, b: THREE.Vector3, c: THREE.Vector3): number {
  return 0.5 * new THREE.Vector3().crossVectors(new THREE.Vector3(b.x - a.x, b.y - a.y, b.z - a.z), new THREE.Vector3(c.x - a.x, c.y - a.y, c.z - a.z)).length();
}

function signedTetrahedronVolume(o: THREE.Vector3, a: THREE.Vector3, b: THREE.Vector3, c: THREE.Vector3): number {
  return (1 / 6) * new THREE.Vector3().crossVectors(new THREE.Vector3(a.x - o.x, a.y - o.y, a.z - o.z), new THREE.Vector3(b.x - o.x, b.y - o.y, b.z - o.z)).dot(new THREE.Vector3(c.x - o.x, c.y - o.y, c.z - o.z));
}

/**
 * 📋Computes key performance indicators for the geometry of a GLB/GLTF model.
 */
export const getGeometricInsightsForModel = async (model: string | ArrayBuffer | Uint8Array): Promise<GeometricInsights> => {
  const io = new NodeIO();
  let doc: GltfDocument;

  if (typeof model === "string") {
    if (model.startsWith("data:")) {
      const base64 = model.slice(model.indexOf(",") + 1);
      const binary = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
      doc = await io.readBinary(binary);
    } else {
      let arrBuf: ArrayBuffer;
      const isPath = !model.startsWith("http://") && !model.startsWith("https://") && (model.endsWith(".glb") || model.endsWith(".gltf") || model.includes("/") || model.includes("\\"));
      if (typeof globalThis !== "undefined" && "process" in globalThis && typeof (globalThis as any).process?.versions?.node === "string" && isPath) {
        const { readFileSync } = await import("node:fs");
        const { dirname, join } = await import("node:path");
        const dir = dirname(model);
        if (model.endsWith(".gltf")) {
          const raw = readFileSync(model, "utf8");
          const json = JSON.parse(raw) as { buffers?: Array<{ uri?: string }>; images?: Array<{ uri?: string }> };
          const resources: Record<string, Uint8Array<ArrayBuffer>> = {};
          const addResource = (uri: string | undefined) => {
            if (!uri) return;
            if (uri.startsWith("data:")) {
              const base64 = uri.slice(uri.indexOf(",") + 1);
              resources[uri] = new Uint8Array(Buffer.from(base64, "base64"));
              return;
            }
            try {
              const binPath = join(dir, uri);
              resources[uri] = new Uint8Array(readFileSync(binPath));
            } catch {
              // skip missing external buffer
            }
          };
          for (const b of json.buffers ?? []) addResource(b.uri);
          for (const img of json.images ?? []) addResource(img.uri);
          doc = await io.readJSON({ json: json as any, resources });
        } else {
          const buf = readFileSync(model);
          arrBuf = buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength);
          doc = await io.readBinary(new Uint8Array(arrBuf));
        }
      } else {
        const res = await fetch(model);
        if (!res.ok) throw new Error(`Failed to load model: ${res.statusText}`);
        arrBuf = await res.arrayBuffer();
        const bytes = new Uint8Array(arrBuf);
        const isGlb = model.endsWith(".glb") || (bytes.length >= 4 && new TextDecoder().decode(bytes.slice(0, 4)) === "glTF");
        const base = model.replace(/\/[^/]*$/, "") || ".";
        doc = isGlb ? await io.readBinary(new Uint8Array(arrBuf)) : await io.readJSON({ json: JSON.parse(new TextDecoder().decode(new Uint8Array(arrBuf))), resources: {} });
      }
    }
  } else {
    const bytes = model instanceof Uint8Array ? model : new Uint8Array(model);
    const magic = bytes.length >= 4 ? new TextDecoder().decode(bytes.slice(0, 4)) : "";
    doc = magic === "glTF" ? await io.readBinary(bytes) : await io.readJSON({ json: JSON.parse(new TextDecoder().decode(bytes)), resources: {} });
  }

  const out: GeometricInsights = {};
  const box = new THREE.Box3();
  let totalArea = 0;
  let totalVolume = 0;
  let vertexCount = 0;
  let faceCount = 0;
  const centroidSum = { x: 0, y: 0, z: 0 };
  const origin = new THREE.Vector3(0, 0, 0);

  for (const mesh of doc.getRoot().listMeshes()) {
    for (const prim of mesh.listPrimitives()) {
      const posAcc = prim.getAttribute("POSITION");
      if (!posAcc) continue;
      const posArray = posAcc.getArray();
      if (!posArray || posArray.length < 3) continue;
      const count = posArray.length / 3;
      for (let i = 0; i < count; i++) {
        const xg = posArray[i * 3];
        const yg = posArray[i * 3 + 1];
        const zg = posArray[i * 3 + 2];
        const p = glbToSemioPoint(xg, yg, zg);
        box.expandByPoint(new THREE.Vector3(p.x, p.y, p.z));
        centroidSum.x += p.x;
        centroidSum.y += p.y;
        centroidSum.z += p.z;
      }
      vertexCount += count;
      const indices = prim.getIndices()?.getArray();
      const getVertex = (idx: number) => new THREE.Vector3(posArray[idx * 3], posArray[idx * 3 + 1], posArray[idx * 3 + 2]);
      if (indices) {
        for (let i = 0; i + 2 < indices.length; i += 3) {
          const a = getVertex(indices[i]);
          const b = getVertex(indices[i + 1]);
          const c = getVertex(indices[i + 2]);
          totalArea += triangleArea(a, b, c);
          totalVolume += signedTetrahedronVolume(origin, a, b, c);
          faceCount += 1;
        }
      } else {
        for (let i = 0; i + 2 < count; i += 3) {
          const a = getVertex(i);
          const b = getVertex(i + 1);
          const c = getVertex(i + 2);
          totalArea += triangleArea(a, b, c);
          totalVolume += signedTetrahedronVolume(origin, a, b, c);
          faceCount += 1;
        }
      }
    }
  }

  if (vertexCount === 0) return out;

  const min = box.min;
  const max = box.max;
  out.boundingBoxMin = { x: min.x, y: min.y, z: min.z };
  out.boundingBoxMax = { x: max.x, y: max.y, z: max.z };
  out.dimensionX = max.x - min.x;
  out.dimensionY = max.y - min.y;
  out.dimensionZ = max.z - min.z;
  const dimX = out.dimensionX ?? 0;
  const dimY = out.dimensionY ?? 0;
  const dimZ = out.dimensionZ ?? 0;
  out.characteristicLength = Math.cbrt(dimX * dimY * dimZ) || 0;
  out.footprintArea = dimX * dimZ;
  out.totalSurfaceArea = totalArea;
  out.vertexCount = vertexCount;
  out.faceCount = faceCount;
  out.centroid = {
    x: centroidSum.x / vertexCount,
    y: centroidSum.y / vertexCount,
    z: centroidSum.z / vertexCount,
  };
  totalVolume = Math.abs(totalVolume);
  if (totalVolume > 1e-20) {
    out.enclosedVolume = totalVolume;
    if (totalArea > 0) out.surfaceToVolumeRatio = totalArea / totalVolume;
    if (totalArea > 0) {
      const sph = (Math.PI ** (1 / 3) * (6 * totalVolume) ** (2 / 3)) / totalArea;
      out.sphericity = Math.min(1, Math.max(0, sph));
    }
  }
  if (dimY > 1e-10 && dimX > 1e-10) out.aspectRatioXy = dimX / dimY;
  if (dimZ > 1e-10 && dimX > 1e-10) out.aspectRatioXz = dimX / dimZ;
  if (dimZ > 1e-10 && dimY > 1e-10) out.aspectRatioYz = dimY / dimZ;
  const maxExtent = Math.max(dimX, dimY, dimZ);
  if (maxExtent > 1e-10 && totalArea > 0) {
    out.slenderness = maxExtent / Math.cbrt(totalArea * maxExtent);
  }
  out.eulerCharacteristic = Math.round(vertexCount - (3 * faceCount) / 2 + faceCount);
  return out;
};

// #endregion ❄️Geometric Insights

// #region 🏰KitStore
// Storage-agnostic kit store contracts MUST be defined here.
// These interfaces express what a kit store DOES, not how a specific engine stores data.
// No engine-specific primitives (map/array/doc) may appear in these contracts.

// Specs: KitStoreStatus represents the lifecycle states of a kit store.
// Providers transition through states: idle → loading → ready → saving/syncing → ready.
// Error and offline are terminal-ish states that require external resolution.

/**
 * Lifecycle status of a kit store.
 *
 * idle → loading → ready. saving/syncing are transient states
 * that return to ready. error/offline require external recovery.
 **/
export type KitStoreStatus = "idle" | "loading" | "ready" | "saving" | "syncing" | "offline" | "error";

/**
 * Synchronization state of a kit store.
 *
 * Specs: Reported as part of KitStoreSnapshot. dirty indicates
 * unsaved local changes. readonly means the store rejects mutations.
 * lastSyncedAt is ISO 8601 timestamp of the last successful persistence.
 **/
export type KitSyncState = {
  status: KitStoreStatus;
  dirty: boolean;
  readonly: boolean;
  lastSyncedAt?: string;
  error?: Error;
};

/**
 * Immutable snapshot of kit data and sync state.
 *
 * Specs: Returned by KitStore.getSnapshot(). kit is the full domain
 * Kit object. sync describes the current synchronization state.
 **/
export type KitStoreSnapshot = {
  kit: Kit;
  sync: KitSyncState;
};

/**
 * Storage-agnostic kit store contract.
 *
 * Specs: This is the boundary between the editor and storage backends.
 * semio/sketchpad depends ONLY on this interface — never on provider internals.
 * Providers (collaborative, JSON file, folder/sqlite) implement this interface
 * and live in semio/studio. The editor consumes stores by injection.
 *
 * getSnapshot() returns the current immutable state.
 * subscribe() registers a listener called on every state change.
 * transact() groups multiple mutations into one logical operation.
 * apply() merges a KitDiff into the current kit state.
 * replace() swaps the entire kit state.
 * save() persists the current state to the backend.
 * reload() re-reads state from the backend, discarding local changes.
 * dispose() releases all resources held by the store.
 **/
export interface KitStore {
  getSnapshot(): KitStoreSnapshot;
  subscribe(listener: () => void): () => void;

  transact<T>(label: string, run: () => T): T;
  apply(diff: KitDiff, meta?: { origin?: string }): void;
  replace(next: Kit, meta?: { origin?: string }): void;

  save(): Promise<void>;
  reload(): Promise<void>;
  dispose(): Promise<void> | void;
}

/**
 * Kit store with undo/redo capability.
 *
 * Specs: Extends KitStore with reversible transaction support.
 * Undo/redo semantics are provider-specific (CRDT-native, command stack, etc.)
 * but the interface is storage-agnostic. The editor uses canUndo/canRedo to
 * enable/disable UI controls without knowing the undo implementation.
 **/
export interface UndoableKitStore extends KitStore {
  canUndo(): boolean;
  canRedo(): boolean;
  undo(): void;
  redo(): void;
}

/**
 * Binary asset storage contract.
 *
 * Specs: Decouples asset storage from kit data storage.
 * Providers may store assets as blobs in IndexedDB, files on disk,
 * or references to remote URLs. The editor uses this interface
 * without knowing the storage strategy.
 **/
export interface BlobAssetStore {
  put(file: Blob, meta?: { path?: string; mimeKind?: string }): Promise<{ id: string; url?: string }>;
  get(id: string): Promise<Blob>;
  remove(id: string): Promise<void>;
}

/**
 * Fine-grained path subscription contract.
 *
 * Specs: Optional capability for stores that can optimize subscriptions
 * to specific paths within the kit data tree. The editor MAY use this
 **/
export interface ObservablePathStore {
  subscribePath(path: readonly string[], listener: () => void): () => void;
}

// #region 🖥️InMemoryKitStore
// In-memory kit store implementation for testing and fake backends.

// Specs: InMemoryKitStore implements UndoableKitStore using a plain Kit object
// in memory with a command-stack undo model. No persistence, no sync, no CRDT.
// Used for: unit tests, integration tests, fake backends, storybook.

/**
 * In-memory kit store for testing and local editing without persistence.
 *
 * Specs: Holds a Kit in memory. apply() uses applyKitDiff to merge diffs.
 * replace() swaps the Kit wholesale. transact() groups mutations.
 * Undo/redo uses a command stack of KitChange (forward+backward diffs).
 * subscribe() notifies listeners on every mutation.
 * save()/reload() are no-ops (no backend).
 **/
export class InMemoryKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus = "ready";
  private transacting: boolean = false;
  private transactionDiffs: KitDiff[] = [];

  constructor(kit: Kit) {
    this.kit = kit;
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: false,
      },
    };
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  transact<T>(label: string, run: () => T): T {
    const before = this.kit;
    this.transacting = true;
    this.transactionDiffs = [];
    try {
      const result = run();
      const after = this.kit;
      if (before !== after) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
      this.transactionDiffs = [];
    }
  }

  apply(diff: KitDiff, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = applyKitDiff(this.kit, diff);
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, this.kit);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  replace(next: Kit, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = next;
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, next);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  async save(): Promise<void> {
    this.dirty = false;
    this.notify();
  }

  async reload(): Promise<void> {
    this.notify();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
    this.undoStack = [];
    this.redoStack = [];
  }

  canUndo(): boolean {
    return this.undoStack.length > 0;
  }

  canRedo(): boolean {
    return this.redoStack.length > 0;
  }

  undo(): void {
    const change = this.undoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.backward);
    this.redoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  redo(): void {
    const change = this.redoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.forward);
    this.undoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

// #endregion 🖥️InMemoryKitStore

// #endregion 🏰KitStore

/**
 * Searches for matching PortInKit entry.
 **/
export const findPortInKit = (kit: Kit, portGuid: string): Port => {
  const iface = kit.ports?.find((i) => i.guid === portGuid);
  if (!iface) throw new Error(`Port ${portGuid} not found in kit ${kit.name}`);
  return iface;
};

/**
 * Searches for matching PieceTypeInDesign entry.
 **/
export const findPieceTypeInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Type => {
  const piece = findPieceInDesign(findDesignInKit(kit, designGuid), pieceGuid);
  if (!piece.type) throw new Error(`Piece ${pieceGuid} has no type`);
  return findTypeInKit(kit, piece.type.guid);
};

/**
 * Searches for matching ParentPieceInDesign entry.
 **/
export const findParentPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Piece => {
  const meta = piecesMetadata(kit, designGuid);
  if (!meta.ok) throw new Error(meta.errors.map((e) => e.message).join("; "));
  const parentPieceId = meta.change.get(pieceGuid)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${pieceGuid} has no parent piece`);
  return findPieceInDesign(findDesignInKit(kit, designGuid), parentPieceId);
};

/**
 * Searches for matching ParentConnectionForPieceInDesign entry.
 **/
export const findParentConnectionForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Connection => {
  const meta = piecesMetadata(kit, designGuid);
  if (!meta.ok) throw new Error(meta.errors.map((e) => e.message).join("; "));
  const parentPieceId = meta.change.get(pieceGuid)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${pieceGuid} has no parent piece and connection`);
  return findConnectionInDesign(findDesignInKit(kit, designGuid), parentPieceId);
};

/**
 * Searches for matching ChildrenPiecesInDesign entry.
 **/
export const findChildrenPiecesInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Piece[] => {
  const design = findDesignInKit(kit, designGuid);
  const meta = piecesMetadata(kit, designGuid);
  if (!meta.ok) throw new Error(meta.errors.map((e) => e.message).join("; "));
  const metadata = meta.change;
  const children: Piece[] = [];
  for (const [id, data] of Array.from(metadata)) {
    if (data.parentPieceId === pieceGuid) {
      children.push(findPieceInDesign(design, id));
    }
  }
  return children;
};

/**
 * Searches for matching UsedConnectorsByPieceInDesign entry.
 **/
export const findUsedConnectorsByPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Connector[] => {
  const design = findDesignInKit(kit, designGuid);
  const piece = findPieceInDesign(design, pieceGuid);
  if (!piece.type) return [];
  const type = findTypeInKit(kit, piece.type.guid);
  const connections = findPieceConnectionsInDesign(design, pieceGuid);
  return connections.map((c) => findConnectorForPieceInConnection(type, c, pieceGuid)).filter((p): p is Connector => p !== undefined);
};

/**
 * Searches for matching ReplacableTypesForPieceInDesign entry.
 **/
export const findReplacableTypesForPieceInDesign = (kit: Kit, designGuid: string, pieceGuid: string): Type[] => {
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
      if (!replacementType.connectors || replacementType.connectors.length === 0) return requiredConnectors.length === 0;
      return requiredConnectors.every((requiredConnector) => {
        return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
      });
    }) ?? []
  );
};

/**
 * Searches for matching ReplacableTypesForPiecesInDesign entry.
 **/
export const findReplacableTypesForPiecesInDesign = (kit: Kit, designGuid: string, pieceGuids: string[]): Type[] => {
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
      if (!replacementType.connectors || replacementType.connectors.length === 0) return externalConnections.length === 0;
      return externalConnections.every(({ requiredConnector }) => {
        return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
      });
    }) ?? []
  );
};

/**
 * Sums the values of a quality across all pieces in a design.
 * For each piece, checks piece-level props first, then falls back to type-level props.
 **/
export const sumQualityInDesign = (kit: Kit, designGuid: string, qualityGuid: string): number => {
  const design = findDesignInKit(kit, designGuid);
  let sum = 0;
  for (const piece of design.pieces ?? []) {
    const pieceProp = piece.props?.find((p) => p.quality?.guid === qualityGuid);
    if (pieceProp) {
      const val = parseFloat(pieceProp.value);
      if (!isNaN(val)) sum += val;
      continue;
    }
    if (piece.type) {
      const type = kit.types?.find((t) => t.guid === piece.type!.guid);
      if (type) {
        const typeProp = type.props?.find((p) => p.quality?.guid === qualityGuid);
        if (typeProp) {
          const val = parseFloat(typeProp.value);
          if (!isNaN(val)) sum += val;
        }
      }
    }
  }
  return sum;
};

/**
 * Per-piece placement metadata derived from a flattened design (fixed root, parent link, depth, path).
 **/
export type PiecePlacementMetadata = {
  plane: Plane;
  center: Coord;
  fixedPieceId: string;
  parentPieceId: string | null;
  depth: number;
  path: string[];
};

/**
 * Definition of piecesMetadata.
 **/
export const piecesMetadata = (kit: Kit, designGuid: string): OperationResult<Map<string, PiecePlacementMetadata>> => {
  const design = findDesignInKit(kit, designGuid);
  if (!design) {
    return operationErr([{ code: "pieces-metadata.design-not-found", message: `Design ${designGuid} not found in kit ${kit.name}` }]);
  }
  const flattenChange = flattenDesign(kit, designGuid);
  if (!flattenChange.ok) {
    return { ok: false, errors: flattenChange.errors };
  }
  const flatDesign = applyDesignDiff(design, flattenChange.change.forward);
  const fixedPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.fixedPieceId", p.guid) || p.guid);
  const parentPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.parentPieceId", null));
  const depths = flatDesign.pieces?.map((p) => parseInt(findAttributeValue(p, "semio.depth", "0")!));
  const paths = flatDesign.pieces?.map((p) => {
    const raw = findAttributeValue(p, "semio.path", p.guid);
    return raw ? raw.split(",").filter(Boolean) : [p.guid!];
  });
  return operationOk(
    new Map(
      flatDesign.pieces?.map((p, index) => [
        p.guid,
        {
          plane: p.plane!,
          center: p.center!,
          fixedPieceId: fixedPieceIds![index],
          parentPieceId: parentPieceIds![index],
          depth: depths![index],
          path: paths![index],
        },
      ]),
    ),
    flattenChange.warnings,
    flattenChange.infos,
  );
};

/**
 * Searches for matching AttributeValue entry.
 **/
export const findAttributeValue = (entity: Kit | Type | Design | Piece | Connection | Model | Connector, name: string, defaultValue?: string | null): string | null => {
  const attribute = entity.attributes?.find((q) => q.key === name);
  if (!attribute && defaultValue === undefined) throw new Error(`Attribute ${name} not found in ${entity}`);
  if (attribute?.value === undefined && defaultValue === null) return null;
  return attribute?.value ?? defaultValue ?? "";
};

// 🎨getColorForText holds the data fields for a getColorForText record.
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

/**
 * Assigns colors to PortsForTypes elements.
 **/
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

// #region 🕌File Tree Utilities
// File tree construction and traversal utilities MUST be defined here.

/**
 * Interface defining FileTreeNode structure.
 **/
export interface FileTreeNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children: FileTreeNode[];
  file?: File;
  folderGuid?: string;
  parentPath?: string;
}

/**
 * Constructs FileTree from components.
 **/
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
 * Flattens nested FileTree structure.
 **/
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

// #endregion 🕌File Tree Utilities

// #region 🧪Tests
// Vitest test suites for domain logic. MUST NOT export any symbols.
// Test code is guarded so it only executes under vitest, not in browser bundles.
// Specs: Other workspaces (e.g. @semio/ui) import this module while Vitest runs their tests; `SEMIO_JS_RUN_EMBEDDED_TESTS` is set only in @semio/js `npm test` so we do not pull @semio/sketchpad into unrelated Vitest SSR graphs.
if (
  typeof (globalThis as any).__vitest_worker__ !== "undefined" &&
  typeof process !== "undefined" &&
  process.env.SEMIO_JS_RUN_EMBEDDED_TESTS === "1"
) {
  const { beforeAll, describe, expect, it, vi } = await import("vitest");
  const { createElement } = await import("react");
  const { renderToStaticMarkup } = await import("react-dom/server");
  const ElementsBundle = await import("@elements/ui");
  const { buildControlTree, Action: UiAction } = ElementsBundle;
  type ControlDef = import("@elements/ui").ControlDef;
  const {
    DragDesign,
    DragDiffDesign,
    DragOffset,
    DragPieces,
    MoveDiffDesign,
    MoveVector,
    InvalidKit,
    InvalidKitValidation,
    MetabolismKit,
    MetabolismKitDiff,
    MetabolismKitDiffed,
    MetabolismKitDiffInverted,
    MetabolismKitFilteredNakaginCapsuleTower,
    ModelSelectionCases,
    NakaginCapsuleTowerFilteredKit,
    MetabolismMetaKit,
    MetabolismShallowKit,
    TambourMetaType,
    TambourShallowType,
    NakaginCapsuleTowerMetaDesign,
    NakaginCapsuleTowerShallowDesign,
    NakaginCapsuleTowerDeletedDesignDiff,
    NakaginCapsuleTowerDeletedSelection,
    NakaginCapsuleTowerCopySelection,
    NakaginCapsuleTowerCopyDesign,
    NakaginCapsuleTowerPasteDesignDiff,
    NakaginCapsuleTowerPasteDesign,
    NakaginCapsuleTowerPasteWithCoordDesignDiff,
    NakaginCapsuleTowerDiffDesign,
    NakaginCapsuleTowerWithDiffDesign,
    ValidateKitDiffCases,
  } = await import("@semio/assets");
  const { createFolderKitStore, createJsonFileKitStore } = await import("@semio/sketchpad");
  type KitFolderAdapter = import("@semio/sketchpad").KitFolderAdapter;
  type KitJsonFileAdapter = import("@semio/sketchpad").KitJsonFileAdapter;

  const TEST_TOLERANCE = 0.001;

  const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
    if (!p1 || !p2) return false;
    if (!p1.origin || !p2.origin || !p1.xAxis || !p2.xAxis || !p1.yAxis || !p2.yAxis) return false;
    return (
      Math.abs(p1.origin.x - p2.origin.x) < TEST_TOLERANCE &&
      Math.abs(p1.origin.y - p2.origin.y) < TEST_TOLERANCE &&
      Math.abs(p1.origin.z - p2.origin.z) < TEST_TOLERANCE &&
      Math.abs(p1.xAxis.x - p2.xAxis.x) < TEST_TOLERANCE &&
      Math.abs(p1.xAxis.y - p2.xAxis.y) < TEST_TOLERANCE &&
      Math.abs(p1.xAxis.z - p2.xAxis.z) < TEST_TOLERANCE &&
      Math.abs(p1.yAxis.x - p2.yAxis.x) < TEST_TOLERANCE &&
      Math.abs(p1.yAxis.y - p2.yAxis.y) < TEST_TOLERANCE &&
      Math.abs(p1.yAxis.z - p2.yAxis.z) < TEST_TOLERANCE
    );
  };

  const centersEqual = (c1: { u: number; v: number } | undefined, c2: { u: number; v: number } | undefined): boolean => {
    if (!c1 || !c2) return c1 === c2;
    return Math.abs(c1.u - c2.u) < TEST_TOLERANCE && Math.abs(c1.v - c2.v) < TEST_TOLERANCE;
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

  const getTestNodePaths = async () => {
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const __filename = fileURLToPath(import.meta.url);
    const __dirname = dirname(__filename);
    const EXPORT_REPORTS_DIR = resolve(__dirname, "../../reports/export-design-model");
    return { __filename, __dirname, EXPORT_REPORTS_DIR, dirname, resolve };
  };

  const writeExportReport = async (implementation: string, bytes: Uint8Array<ArrayBufferLike>) => {
    const { mkdirSync, writeFileSync } = await import("node:fs");
    const { EXPORT_REPORTS_DIR, resolve } = await getTestNodePaths();
    mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });
    const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
    writeFileSync(reportPath, bytes);
    return reportPath;
  };

  const roundSceneNumber = (value: number) => {
    const rounded = Math.round(value * 10_000) / 10_000;
    return Object.is(rounded, -0) ? 0 : rounded;
  };

  const composeNodeMatrix = (node: { matrix?: number[]; translation?: number[]; rotation?: number[]; scale?: number[] }) => {
    if (node.matrix) {
      return node.matrix.map((value) => roundSceneNumber(value));
    }
    const translation = node.translation ?? [0, 0, 0];
    const rotation = node.rotation ?? [0, 0, 0, 1];
    const scale = node.scale ?? [1, 1, 1];
    const [x, y, z, w] = rotation;
    const x2 = x + x;
    const y2 = y + y;
    const z2 = z + z;
    const xx = x * x2;
    const xy = x * y2;
    const xz = x * z2;
    const yy = y * y2;
    const yz = y * z2;
    const zz = z * z2;
    const wx = w * x2;
    const wy = w * y2;
    const wz = w * z2;
    const sx = scale[0];
    const sy = scale[1];
    const sz = scale[2];
    return [
      roundSceneNumber((1 - (yy + zz)) * sx),
      roundSceneNumber((xy + wz) * sx),
      roundSceneNumber((xz - wy) * sx),
      0,
      roundSceneNumber((xy - wz) * sy),
      roundSceneNumber((1 - (xx + zz)) * sy),
      roundSceneNumber((yz + wx) * sy),
      0,
      roundSceneNumber((xz + wy) * sz),
      roundSceneNumber((yz - wx) * sz),
      roundSceneNumber((1 - (xx + yy)) * sz),
      0,
      roundSceneNumber(translation[0]),
      roundSceneNumber(translation[1]),
      roundSceneNumber(translation[2]),
      1,
    ];
  };

  const normalizeSceneGraph = (gltfText: string) => {
    const gltf = JSON.parse(gltfText) as {
      scene?: number;
      scenes?: Array<{ nodes?: number[] }>;
      nodes?: Array<{ name?: string; children?: number[]; matrix?: number[]; mesh?: number; translation?: number[]; rotation?: number[]; scale?: number[] }>;
    };
    const nodes = gltf.nodes ?? [];
    const defaultScene = gltf.scenes?.[gltf.scene ?? 0] ?? { nodes: [] };
    const names = nodes.map((node, index) => node.name ?? `__node_${index}`);
    const parents = new Map<string, string | null>();
    for (const name of names) parents.set(name, null);
    for (let index = 0; index < nodes.length; index += 1) {
      for (const childIndex of nodes[index].children ?? []) {
        parents.set(names[childIndex], names[index]);
      }
    }
    let normalizedRoots = [...(defaultScene.nodes ?? [])].map((index) => names[index]).sort();
    let normalizedNodes = nodes
      .map((node, index) => ({
        name: names[index],
        parent: parents.get(names[index]) ?? null,
        children: [...(node.children ?? [])].map((childIndex) => names[childIndex]).sort(),
        hasMesh: node.mesh !== undefined,
        matrix: composeNodeMatrix(node),
      }))
      .sort((left, right) => left.name.localeCompare(right.name));
    const syntheticWorld = normalizedNodes.find((node) => node.name === "world");
    if (
      syntheticWorld &&
      !syntheticWorld.hasMesh &&
      syntheticWorld.parent === null &&
      syntheticWorld.children.length === 1 &&
      normalizedRoots.length === 1 &&
      normalizedRoots[0] === "world" &&
      syntheticWorld.matrix.every((value, index) => value === [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1][index])
    ) {
      const childName = syntheticWorld.children[0];
      normalizedRoots = [childName];
      normalizedNodes = normalizedNodes.filter((node) => node.name !== "world").map((node) => (node.name === childName ? { ...node, parent: null } : node));
    }
    return {
      roots: normalizedRoots,
      nodes: normalizedNodes,
    };
  };

  const runExportReportCommand = async (command: string, args: string[], cwd: string) => {
    const { execFileSync } = await import("node:child_process");
    let lastError: unknown;
    for (let attempt = 1; attempt <= 2; attempt++) {
      try {
        execFileSync(command, args, {
          cwd,
          stdio: "pipe",
        });
        return;
      } catch (error) {
        lastError = error;
        if (attempt === 2) break;
      }
    }
    throw lastError;
  };

  const parseSelfContainedGltf = async (reportText: string) => {
    const parsed = JSON.parse(reportText) as {
      buffers?: Array<{ uri?: string }>;
      images?: Array<{ uri?: string }>;
    };
    const resources: Record<string, Uint8Array<ArrayBuffer>> = {};
    const collectResource = (uri?: string) => {
      if (!uri?.startsWith("data:")) return;
      const base64 = uri.slice(uri.indexOf(",") + 1);
      resources[uri] = new Uint8Array(Buffer.from(base64, "base64"));
    };
    for (const buffer of parsed.buffers ?? []) collectResource(buffer.uri);
    for (const image of parsed.images ?? []) collectResource(image.uri);
    const io = new NodeIO();
    return io.readJSON({ json: parsed as any, resources });
  };

  const getMeshNames = (reportText: string) => {
    const parsed = JSON.parse(reportText) as { meshes?: Array<{ name?: string }> };
    return (parsed.meshes ?? []).map((mesh) => mesh.name).filter((name): name is string => Boolean(name));
  };

  describe("KitDiffValidation", () => {
    const cases = ValidateKitDiffCases as {
      tinyKit: unknown;
      cases: Array<{
        id: string;
        diff: Record<string, unknown>;
        expectOk: boolean;
        errorCodes: string[];
        warningCodes: string[];
      }>;
    };
    const tinyKit = KitSchema.parse(cases.tinyKit);
    for (const tc of cases.cases) {
      it(`asset case ${tc.id}`, () => {
        const r = validateKitDiff(tinyKit, tc.diff as KitDiff, false);
        expect(r.ok).toBe(tc.expectOk);
        const errCodes = r.errors.map((e) => e.code).filter(Boolean) as string[];
        const warnCodes = r.warnings.map((w) => w.code).filter(Boolean) as string[];
        for (const c of tc.errorCodes) {
          expect(errCodes).toContain(c);
        }
        for (const c of tc.warningCodes) {
          expect(warnCodes).toContain(c);
        }
      });
    }
    it("heal drops invalid design update", () => {
      const bad: KitDiff = {
        designs: {
          updated: [{ design: { guid: "99999999-9999-9999-9999-999999999999" }, diff: { name: "X" } }],
        },
      };
      const r = validateKitDiff(tinyKit, bad, true);
      expect(r.diff?.designs?.updated ?? []).toHaveLength(0);
    });
  });

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

  // #region 🏰Kit Filter Tests
  // Tests for filterKit MUST verify correct subset extraction with design-based and glob-based filters.

  describe("Kit/Filter/Design", () => {
    const kit = MetabolismKit as Kit;
    const expected = NakaginCapsuleTowerFilteredKit as any;
    const nakaginDesign = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent);

    it("filters kit to only contain entities related to Nakagin Capsule Tower design", () => {
      expect(nakaginDesign).toBeDefined();
      const filtered = filterKit(kit, { designGuid: nakaginDesign!.guid });

      expect(filtered.designs?.length).toBe(expected.designs.length);
      expect(filtered.types?.length).toBe(expected.types.length);
      expect(filtered.files?.length).toBe(expected.files.length);
      expect(filtered.ports?.length).toBe(expected.ports.length);
      expect(filtered.qualities?.length).toBe(expected.qualities.length);
      expect(filtered.authors?.length).toBe(expected.authors.length);

      const filteredDesign = filtered.designs?.find((d) => d.guid === nakaginDesign!.guid);
      expect(filteredDesign).toBeDefined();
      expect(filteredDesign!.pieces?.length).toBe(nakaginDesign!.pieces?.length);

      for (const expectedType of expected.types) {
        const filteredType = filtered.types?.find((t: any) => t.guid === expectedType.guid);
        expect(filteredType).toBeDefined();
        expect(filteredType!.models?.length ?? 0).toBe(expectedType.models?.length ?? 0);
      }

      for (const piece of filteredDesign!.pieces ?? []) {
        if (piece.type?.guid) {
          expect(filtered.types?.some((t) => t.guid === piece.type!.guid)).toBe(true);
        }
      }

      for (const type of filtered.types ?? []) {
        expect((type.models ?? []).length).toBeLessThanOrEqual(1);
        for (const model of type.models ?? []) {
          expect(filtered.files?.some((f) => f.guid === model.file.guid)).toBe(true);
        }
        for (const connector of type.connectors ?? []) {
          if (connector.port?.guid) {
            expect(filtered.ports?.some((p) => p.guid === connector.port!.guid)).toBe(true);
          }
        }
      }
    });

    it("preserves kit metadata", () => {
      const filtered = filterKit(kit, { designGuid: nakaginDesign!.guid });
      expect(filtered.guid).toBe(kit.guid);
      expect(filtered.name).toBe(kit.name);
      expect(filtered.version).toBe(kit.version);
    });
  });

  describe("Kit/Filter/Glob", () => {
    it("globMatch matches wildcard patterns", () => {
      expect(globMatch("Nakagin Capsule Tower", "Nakagin*")).toBe(true);
      expect(globMatch("Nakagin Capsule Tower", "*Tower")).toBe(true);
      expect(globMatch("Nakagin Capsule Tower", "*Capsule*")).toBe(true);
      expect(globMatch("Nakagin Capsule Tower", "Nakagin Capsule Tower")).toBe(true);
      expect(globMatch("Nakagin Capsule Tower", "Other*")).toBe(false);
      expect(globMatch("Wall", "W?ll")).toBe(true);
      expect(globMatch("Wall", "W??l")).toBe(true);
      expect(globMatch("Wall", "W????")).toBe(false);
    });

    it("globMatch is case-insensitive", () => {
      expect(globMatch("Wall", "wall")).toBe(true);
      expect(globMatch("wall", "WALL")).toBe(true);
      expect(globMatch("Nakagin Capsule Tower", "nakagin*")).toBe(true);
    });

    it("matchesGlobFilter with include only", () => {
      expect(matchesGlobFilter("Wall", { include: ["Wall"] })).toBe(true);
      expect(matchesGlobFilter("Column", { include: ["Wall"] })).toBe(false);
      expect(matchesGlobFilter("Wall", { include: ["W*", "C*"] })).toBe(true);
      expect(matchesGlobFilter("Column", { include: ["W*", "C*"] })).toBe(true);
      expect(matchesGlobFilter("Beam", { include: ["W*", "C*"] })).toBe(false);
    });

    it("matchesGlobFilter with exclude only", () => {
      expect(matchesGlobFilter("Wall", { exclude: ["Wall"] })).toBe(false);
      expect(matchesGlobFilter("Column", { exclude: ["Wall"] })).toBe(true);
      expect(matchesGlobFilter("Wall", { exclude: ["*all"] })).toBe(false);
    });

    it("matchesGlobFilter with include and exclude", () => {
      expect(matchesGlobFilter("Wall", { include: ["W*"], exclude: ["Wall"] })).toBe(false);
      expect(matchesGlobFilter("Window", { include: ["W*"], exclude: ["Wall"] })).toBe(true);
    });

    it("matchesGlobFilter with no filter returns true", () => {
      expect(matchesGlobFilter("anything")).toBe(true);
      expect(matchesGlobFilter("anything", undefined)).toBe(true);
    });

    it("filterKit with type glob include filters types by name", () => {
      const kit = MetabolismKit as Kit;
      const totalTypes = kit.types?.length ?? 0;
      expect(totalTypes).toBeGreaterThan(0);
      const filtered = filterKit(kit, { types: { include: ["Capsule*"] } });
      expect(filtered.types?.length ?? 0).toBeGreaterThan(0);
      expect(filtered.types?.length ?? 0).toBeLessThan(totalTypes);
      for (const t of filtered.types ?? []) {
        expect(t.name.toLowerCase().startsWith("capsule")).toBe(true);
      }
    });

    it("filterKit with type glob exclude filters out matching types", () => {
      const kit = MetabolismKit as Kit;
      const totalTypes = kit.types?.length ?? 0;
      const filtered = filterKit(kit, { types: { exclude: ["Capsule*"] } });
      expect(filtered.types?.length ?? 0).toBeLessThan(totalTypes);
      for (const t of filtered.types ?? []) {
        expect(t.name.toLowerCase().startsWith("capsule")).toBe(false);
      }
    });

    it("filterKit with design glob include filters designs by name", () => {
      const kit = MetabolismKit as Kit;
      const filtered = filterKit(kit, { designs: { include: ["Nakagin*"] } });
      expect(filtered.designs?.length ?? 0).toBeGreaterThan(0);
      for (const d of filtered.designs ?? []) {
        expect(globMatch(d.name, "Nakagin*")).toBe(true);
      }
    });

    it("filterKit with no filter returns kit unchanged", () => {
      const kit = MetabolismKit as Kit;
      const filtered = filterKit(kit, {});
      expect(filtered.types?.length).toBe(kit.types?.length);
      expect(filtered.designs?.length).toBe(kit.designs?.length);
      expect(filtered.ports?.length).toBe(kit.ports?.length);
    });

    it("filterKit combines designGuid with glob filters", () => {
      const kit = MetabolismKit as Kit;
      const nakaginDesign = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent);
      expect(nakaginDesign).toBeDefined();
      const designFiltered = filterKit(kit, { designGuid: nakaginDesign!.guid });
      const combinedFiltered = filterKit(kit, { designGuid: nakaginDesign!.guid, types: { exclude: ["Capsule*"] } });
      expect(combinedFiltered.types?.length ?? 0).toBeLessThan(designFiltered.types?.length ?? 0);
      for (const t of combinedFiltered.types ?? []) {
        expect(t.name.toLowerCase().startsWith("capsule")).toBe(false);
      }
    });
  });

  // #endregion 🏰Kit Filter Tests

  // #region 🛡️KitKind Tests
  // Tests for KitKind enum MUST verify the five kit kinds.

  describe("KitKind", () => {
    it("KitKindSchema accepts all five valid kinds", () => {
      const kinds = ["file", "folder", "archive", "remote", "temporary"] as const;
      for (const kind of kinds) {
        expect(KitKindSchema.parse(kind)).toBe(kind);
      }
    });

    it("KitKindSchema rejects invalid values", () => {
      expect(() => KitKindSchema.parse("invalid")).toThrow();
      expect(() => KitKindSchema.parse("")).toThrow();
      expect(() => KitKindSchema.parse("json")).toThrow();
      expect(() => KitKindSchema.parse("sqlite")).toThrow();
    });

    it("ALL_KIT_KINDS contains exactly five entries", () => {
      expect(ALL_KIT_KINDS).toHaveLength(5);
      expect(ALL_KIT_KINDS).toContain("file");
      expect(ALL_KIT_KINDS).toContain("folder");
      expect(ALL_KIT_KINDS).toContain("archive");
      expect(ALL_KIT_KINDS).toContain("remote");
      expect(ALL_KIT_KINDS).toContain("temporary");
    });

    it("KitKind type is assignable from literal strings", () => {
      const file: KitKind = "file";
      const folder: KitKind = "folder";
      const archive: KitKind = "archive";
      const remote: KitKind = "remote";
      const temporary: KitKind = "temporary";
      expect([file, folder, archive, remote, temporary]).toEqual(["file", "folder", "archive", "remote", "temporary"]);
    });

    it("Kit/File: roundtrips through JSON serialize/deserialize", () => {
      const kit: Kit = {
        guid: "file-kit-guid",
        name: "FileKit Test",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const json = serializeKit(kit);
      const restored = deserializeKit(json);
      expect(restored.guid).toBe(kit.guid);
      expect(restored.name).toBe(kit.name);
    });

    it("Kit/File: imports, exports and edits with file kit helpers", async () => {
      const kit: Kit = {
        guid: "file-kit-helper-guid",
        name: "FileKit Helper Test",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const exported = exportFileKit(kit);
      const imported = await importFileKit(exported);
      expect(imported.kind).toBe("file");
      expect(imported.kit.guid).toBe(kit.guid);
      const edited = editTemporaryKit(imported.kit, { name: "FileKit Helper Edited" });
      expect(edited.name).toBe("FileKit Helper Edited");
      expect(imported.kit.name).toBe("FileKit Helper Test");
    });

    it("Kit/Folder: roundtrips through SQLite via FolderKitStore adapter", async () => {
      const kit: Kit = {
        guid: "folder-kit-guid",
        name: "FolderKit Test",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
        types: [{ guid: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
      };
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(kit, db);
      const data = db.export();
      db.close();
      const db2 = new SQL.Database(new Uint8Array(data));
      const restored = await sqliteToKit(db2);
      db2.close();
      expect(restored.guid).toBe(kit.guid);
      expect(restored.name).toBe(kit.name);
      expect(restored.types).toHaveLength(1);
      expect(restored.types![0].name).toBe("Wall");
    });

    it("Kit/Archive: roundtrips through zip export/import", async () => {
      const kit: Kit = {
        guid: "archive-kit-guid",
        name: "ArchiveKit Test",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
        types: [{ guid: "at1", name: "Beam", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
      };
      const blob = await exportKit(kit);
      const result = await importKit(blob);
      expect(result.kit.guid).toBe(kit.guid);
      expect(result.kit.name).toBe(kit.name);
      expect(result.kit.types).toHaveLength(1);
      expect(result.kit.types![0].name).toBe("Beam");
    });

    it("Kit/Remote: validates remote URL field on kit", () => {
      const kit: Kit = {
        guid: "remote-kit-guid",
        name: "RemoteKit Test",
        remote: "https://example.com/metabolism.kit.json",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const parsed = KitSchema.parse(kit);
      expect(parsed.remote).toBe("https://example.com/metabolism.kit.json");
      const json = serializeKit(kit);
      const restored = deserializeKit(json);
      expect(restored.remote).toBe(kit.remote);
    });

    it("Kit/Remote: imports remote JSON and archive sources", async () => {
      const remoteJsonKit: Kit = {
        guid: "remote-json-kit-guid",
        name: "Remote JSON Kit",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const remoteArchiveKit: Kit = {
        guid: "remote-archive-kit-guid",
        name: "Remote Archive Kit",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const archiveBlob = await exportKit(remoteArchiveKit);
      const originalFetch = globalThis.fetch;
      globalThis.fetch = vi.fn(async (input: string | URL | Request) => {
        const url = String(input);
        if (url.endsWith(".kit.json")) {
          return new Response(exportFileKit(remoteJsonKit), {
            status: 200,
            headers: { "content-type": "application/json" },
          });
        }
        return new Response(await archiveBlob.arrayBuffer(), {
          status: 200,
          headers: { "content-type": "application/zip" },
        });
      }) as typeof fetch;

      try {
        const importedJson = await importRemoteKit("https://example.com/remote.kit.json");
        expect(importedJson.kind).toBe("remote");
        expect(importedJson.kit.guid).toBe(remoteJsonKit.guid);

        const importedArchive = await importRemoteKit("https://example.com/remote.kit.zip");
        expect(importedArchive.kind).toBe("remote");
        expect(importedArchive.kit.guid).toBe(remoteArchiveKit.guid);
      } finally {
        globalThis.fetch = originalFetch;
      }
    });

    it("Kit/Temporary: InMemoryKitStore roundtrip without persistence", () => {
      const kit: Kit = {
        guid: "temp-kit-guid",
        name: "TemporaryKit Test",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const store = new InMemoryKitStore(kit);
      expect(store.getSnapshot().kit.guid).toBe("temp-kit-guid");
      store.apply({ name: "Modified Temporary" });
      expect(store.getSnapshot().kit.name).toBe("Modified Temporary");
      store.undo();
      expect(store.getSnapshot().kit.name).toBe("TemporaryKit Test");
    });

    it("Kit/Temporary: editTemporaryKit applies a diff without mutating the source", () => {
      const kit: Kit = {
        guid: "temp-edit-kit-guid",
        name: "Temporary Editable Kit",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const edited = editTemporaryKit(kit, { name: "Temporary Editable Kit Edited" });
      expect(edited.name).toBe("Temporary Editable Kit Edited");
      expect(kit.name).toBe("Temporary Editable Kit");
    });
  });

  // #endregion 🛡️KitKind Tests

  // #region 🏰Kit Filter Tests
  // Tests for filterKit MUST verify correct subset extraction with design-based and glob-based filters.

  describe("Kit/Filter/Design", () => {
    const kit = MetabolismKit as Kit;
    const expected = MetabolismKitFilteredNakaginCapsuleTower as any;
    const nakaginDesign = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent);

    it("filters kit to only contain entities related to Nakagin Capsule Tower design", () => {
      expect(nakaginDesign).toBeDefined();
      const filtered = filterKit(kit, { designGuid: nakaginDesign!.guid });

      expect(filtered.designs?.length).toBe(expected.designs.length);
      expect(filtered.types?.length).toBe(expected.types.length);
      expect(filtered.files?.length).toBe(expected.files.length);
      expect(filtered.ports?.length).toBe(expected.ports.length);
      expect(filtered.qualities?.length).toBe(expected.qualities.length);
      expect(filtered.authors?.length).toBe(expected.authors.length);

      const filteredDesign = filtered.designs?.find((d) => d.guid === nakaginDesign!.guid);
      expect(filteredDesign).toBeDefined();
      expect(filteredDesign!.pieces?.length).toBe(nakaginDesign!.pieces?.length);

      for (const expectedType of expected.types) {
        const filteredType = filtered.types?.find((t: any) => t.guid === expectedType.guid);
        expect(filteredType).toBeDefined();
        expect(filteredType!.models?.length ?? 0).toBe(expectedType.models?.length ?? 0);
      }

      for (const piece of filteredDesign!.pieces ?? []) {
        if (piece.type?.guid) {
          expect(filtered.types?.some((t) => t.guid === piece.type!.guid)).toBe(true);
        }
      }

      for (const type of filtered.types ?? []) {
        for (const model of type.models ?? []) {
          expect(filtered.files?.some((f) => f.guid === model.file.guid)).toBe(true);
        }
        for (const connector of type.connectors ?? []) {
          if (connector.port?.guid) {
            expect(filtered.ports?.some((p) => p.guid === connector.port!.guid)).toBe(true);
          }
        }
      }
    });

    it("preserves kit metadata", () => {
      const filtered = filterKit(kit, { designGuid: nakaginDesign!.guid });
      expect(filtered.guid).toBe(kit.guid);
      expect(filtered.name).toBe(kit.name);
      expect(filtered.version).toBe(kit.version);
    });

    it("each type has at most one model", () => {
      const filtered = filterKit(kit, { designGuid: nakaginDesign!.guid });
      for (const type of filtered.types ?? []) {
        expect((type.models ?? []).length).toBeLessThanOrEqual(1);
      }
    });
  });

  // #endregion 🏰Kit Filter Tests

  describe("Flatten", () => {
    const kit = MetabolismKit as Kit;

    const testFlatten = (designName: string, parentName?: string) => {
      const design = findDesign(kit, designName, parentName);
      const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design.guid);
      expect(expectedDesign).toBeDefined();
      const flatOp = flattenDesign(kit, design.guid);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const flatDesign = applyDesignDiff(design, flatOp.change.forward);

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

    it("forward diff lists every connection removal by guid and apply clears connections", () => {
      const design = findDesign(kit, "Nakagin Capsule Tower");
      const origConnCount = design.connections?.length ?? 0;
      expect(origConnCount).toBeGreaterThan(0);
      const flatOp = flattenDesign(kit, design.guid);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const removed = flatOp.change.forward.connections?.removed ?? [];
      expect(removed.length).toBe(origConnCount);
      const removedSet = new Set(removed.map((r) => r.guid));
      for (const c of design.connections ?? []) {
        expect(removedSet.has(c.guid)).toBe(true);
      }
      const flatDesign = applyDesignDiff(JSON.parse(JSON.stringify(design)), flatOp.change.forward);
      expect(flatDesign.connections?.length ?? 0).toBe(0);
    });

    it("warns when a connected clump has no fixed piece and still flattens", () => {
      const floatingA: Piece = { guid: "floating-a", name: "A", type: { guid: "t1" } };
      const floatingB: Piece = { guid: "floating-b", name: "B", type: { guid: "t1" } };
      const design: Design = {
        guid: "design-float",
        name: "Float",
        unit: "mm",
        pieces: [floatingA, floatingB],
        connections: [
          {
            guid: "c-ab",
            connected: { piece: { guid: "floating-a" }, connector: { guid: "c1" } },
            connecting: { piece: { guid: "floating-b" }, connector: { guid: "c2" } },
          },
        ],
        createdAt: "2025-01-01T00:00:00.000Z",
        updatedAt: "2025-01-01T00:00:00.000Z",
      };
      const miniKit: Kit = {
        guid: "k1",
        name: "k",
        designs: [design],
        types: [
          {
            guid: "t1",
            name: "T",
            unit: "mm",
            connectors: [
              { guid: "c1", point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 1, z: 0 }, t: 0 },
              { guid: "c2", point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 1, z: 0 }, t: 0.5 },
            ],
            createdAt: "2025-01-01T00:00:00.000Z",
            updatedAt: "2025-01-01T00:00:00.000Z",
          },
        ],
        createdAt: "2025-01-01T00:00:00.000Z",
        updatedAt: "2025-01-01T00:00:00.000Z",
      };
      const op = flattenDesign(miniKit, design.guid);
      expect(op.ok).toBe(true);
      if (!op.ok) return;
      expect(op.warnings.some((w) => w.code === "flatten.no-fixed-piece-in-clump")).toBe(true);
    });
  });

  describe("Roundtrip", () => {
    describe("Metabolism", () => {
      it("Json -> Memory -> Json, Json -> Zip, Zip -> Json", async () => {
        const fs = await import("node:fs");
        const path = await import("node:path");
        const { __dirname } = await getTestNodePaths();

        const kit = MetabolismKit as unknown as Kit;
        const serializedKit = serializeKit(kit);
        const deserializedKit = deserializeKit(serializedKit);
        expect(areKitsEqual(kit, deserializedKit)).toBe(true);

        const zipPath = path.join(__dirname, "../assets/semio/metabolism.zip");
        const zipBuffer = fs.readFileSync(zipPath);
        const { kit: zipKit } = await importKit(zipBuffer);
        expect(areKitsEqual(kit, zipKit)).toBe(true);

        const exportedZip = await exportKit(kit);
        const { kit: reKit } = await importKit(exportedZip);
        expect(areKitsEqual(kit, reKit)).toBe(true);
      }, 60000);
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

      it("Plain descriptions do not create emoji validation problems", () => {
        const kit = structuredClone(MetabolismKit) as Kit;
        kit.description = "Plain kit summary";
        kit.types = (kit.types ?? []).map((entry, index) => ({
          ...entry,
          description: `Repeated plain description ${index % 2}`,
        }));

        const result = validateKit(kit);
        const emojiProblems = result.problems.filter((problem) => ["description-missing-emoji", "description-emoji-unique"].includes(problem.constraintId));

        expect(emojiProblems).toEqual([]);
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
      const change = replaceClusterWithDesign(design, ["piece-a", "piece-b"], clusteredDesign, externalConnections);
      const updatedDesign = applyDesignDiff(design, change.forward);

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

    it("Nakagin Capsule Tower flattened piece drag uses piece center diff (flat design has no connections)", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const flatOp = flattenDesign(kit, design.guid);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const flatDesign = applyDesignDiff(JSON.parse(JSON.stringify(design)), flatOp.change.forward);
      expect((flatDesign.connections ?? []).length).toBe(0);
      const pieceGuid = "9d18882e-d90b-40de-a171-47cb4564ffa6";
      const flatPiece = flatDesign.pieces!.find((p) => p.guid === pieceGuid)!;
      const pieces = { ...flatDesign, pieces: [flatPiece] } as Design;
      const offset = { u: 3, v: -1 };
      const diff = dragPiecesInDesign(flatDesign, pieces, offset);
      expect(diff.connections).toBeUndefined();
      expect(diff.pieces?.updated?.length).toBe(1);
      expect(diff.pieces!.updated![0].piece.guid).toBe(pieceGuid);
      const baseU = flatPiece.center?.u ?? 0;
      const baseV = flatPiece.center?.v ?? 0;
      expect(diff.pieces!.updated![0].diff.center?.u).toBeCloseTo(baseU + offset.u, 6);
      expect(diff.pieces!.updated![0].diff.center?.v).toBeCloseTo(baseV + offset.v, 6);
    });
  });

  describe("Move", () => {
    it("same drag fixture: roots get plane translation; connected mover gets connector-frame split (gap/shift/rise + residual u/v)", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = DragDesign as unknown as Design;
      const pieces = DragPieces as unknown as Design;
      const vector = MoveVector as { gap: number; shift: number; rise: number };
      const expectedDiff = MoveDiffDesign as any;
      const computedDiff = movePiecesInDesign(kit, design, pieces, vector);
      const computedPieceUpdates = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.guid.localeCompare(b.piece.guid));
      const expectedPieceUpdates = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.guid.localeCompare(b.piece.guid));
      expect(computedPieceUpdates.length).toBe(expectedPieceUpdates.length);
      for (let i = 0; i < computedPieceUpdates.length; i++) {
        expect(computedPieceUpdates[i].piece.guid).toBe(expectedPieceUpdates[i].piece.guid);
        const po = computedPieceUpdates[i].diff.plane?.origin;
        const eo = expectedPieceUpdates[i].diff.plane.origin;
        expect(po?.x).toBeCloseTo(eo.x, 5);
        expect(po?.y).toBeCloseTo(eo.y, 5);
        expect(po?.z).toBeCloseTo(eo.z, 5);
      }
      const computedConnUpdates = (computedDiff.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
      const expectedConnUpdates = (expectedDiff.connections?.updated ?? []).sort((a: any, b: any) => a.connection.guid.localeCompare(b.connection.guid));
      expect(computedConnUpdates.length).toBe(expectedConnUpdates.length);
      for (let i = 0; i < computedConnUpdates.length; i++) {
        expect(computedConnUpdates[i].connection.guid).toBe(expectedConnUpdates[i].connection.guid);
        const ed = expectedConnUpdates[i].diff;
        const cd = computedConnUpdates[i].diff;
        for (const key of ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"] as const) {
          if (ed[key] !== undefined) expect(cd[key]).toBeCloseTo(ed[key] as number, 8);
        }
      }
      const dragParity = dragPiecesInDesign(design, pieces, { u: vector.shift, v: vector.gap });
      const dragConn = (dragParity.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
      expect(computedConnUpdates.map((c) => c.connection.guid)).toEqual(dragConn.map((c) => c.connection.guid));
      const dragPiecesUp = (dragParity.pieces?.updated ?? []).sort((a, b) => a.piece.guid.localeCompare(b.piece.guid));
      expect(computedPieceUpdates.map((p) => p.piece.guid)).toEqual(dragPiecesUp.map((p) => p.piece.guid));
    });

    it("vertical parent connector: world move decomposes into shift, gap, rise on connection (not diagram u/v only)", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = DragDesign as unknown as Design;
      const pieces = DragPieces as unknown as Design;
      const vector = { gap: 2, shift: -1, rise: 0.5 };
      const diff = movePiecesInDesign(kit, design, pieces, vector);
      const dragParity = dragPiecesInDesign(design, pieces, { u: vector.shift, v: vector.gap });
      const moveConn = (diff.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
      const dragConn = (dragParity.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
      expect(moveConn.length).toBe(dragConn.length);
      for (let i = 0; i < moveConn.length; i++) {
        expect(moveConn[i].connection.guid).toBe(dragConn[i].connection.guid);
        expect(moveConn[i].diff.gap).toBeCloseTo(0.5, 5);
        expect(moveConn[i].diff.shift).toBeCloseTo(-1, 5);
        expect(moveConn[i].diff.rise).toBeCloseTo(-2, 5);
        for (const ang of ["rotation", "turn", "tilt"] as const) {
          const av = moveConn[i].diff[ang];
          if (av !== undefined) expect(av).toBeCloseTo(0, 3);
        }
      }
    });
  });

  describe("Delete", () => {
    it("Nakagin Capsule Tower delete third tambour and first small tower connection", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const selection = NakaginCapsuleTowerDeletedSelection as unknown as Design;
      const expectedDiff = NakaginCapsuleTowerDeletedDesignDiff as any;

      const pieceGuids = (selection.pieces ?? []).map((p) => p.guid);
      const connectionGuids = (selection.connections ?? []).map((c) => c.guid);
      const delOp = deletePiecesAndConnectionsInDesign(kit, design, pieceGuids, connectionGuids);
      expect(delOp.ok).toBe(true);
      if (!delOp.ok) return;
      const computedDiff = delOp.change;

      // 🚚Verify removed pieces
      const computedRemovedPieces = (computedDiff.pieces?.removed ?? []).sort((a, b) => a.guid.localeCompare(b.guid));
      const expectedRemovedPieces = (expectedDiff.pieces?.removed ?? []).sort((a: any, b: any) => a.guid.localeCompare(b.guid));
      expect(computedRemovedPieces.length).toBe(expectedRemovedPieces.length);
      for (let i = 0; i < computedRemovedPieces.length; i++) {
        expect(computedRemovedPieces[i].guid).toBe(expectedRemovedPieces[i].guid);
      }

      // 🔁Verify updated (fixed) pieces
      const computedUpdated = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.guid.localeCompare(b.piece.guid));
      const expectedUpdated = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.guid.localeCompare(b.piece.guid));
      expect(computedUpdated.length).toBe(expectedUpdated.length);
      for (let i = 0; i < computedUpdated.length; i++) {
        expect(computedUpdated[i].piece.guid).toBe(expectedUpdated[i].piece.guid);
        expect(computedUpdated[i].diff.plane?.origin?.x).toBeCloseTo(expectedUpdated[i].diff.plane.origin.x, 3);
        expect(computedUpdated[i].diff.plane?.origin?.y).toBeCloseTo(expectedUpdated[i].diff.plane.origin.y, 3);
        expect(computedUpdated[i].diff.plane?.origin?.z).toBeCloseTo(expectedUpdated[i].diff.plane.origin.z, 3);
        expect(computedUpdated[i].diff.center?.u).toBeCloseTo(expectedUpdated[i].diff.center.u, 3);
        expect(computedUpdated[i].diff.center?.v).toBeCloseTo(expectedUpdated[i].diff.center.v, 3);
      }

      // 🔌Verify removed connections
      const computedRemovedConns = (computedDiff.connections?.removed ?? []).sort((a, b) => a.guid.localeCompare(b.guid));
      const expectedRemovedConns = (expectedDiff.connections?.removed ?? []).sort((a: any, b: any) => a.guid.localeCompare(b.guid));
      expect(computedRemovedConns.length).toBe(expectedRemovedConns.length);
      for (let i = 0; i < computedRemovedConns.length; i++) {
        expect(computedRemovedConns[i].guid).toBe(expectedRemovedConns[i].guid);
      }
    });
  });

  // #region 📋Copy And Paste Tests
  describe("CopyAndPaste", () => {
    it("Nakagin Capsule Tower copy selected pieces and connections", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const selection = NakaginCapsuleTowerCopySelection as any;
      const expectedCopy = NakaginCapsuleTowerCopyDesign as unknown as Design;

      const pieceGuids = (selection.pieces ?? []).map((p: any) => p.guid);
      const connectionGuids = (selection.connections ?? []).map((c: any) => c.guid);
      const copyOp = copyDesign(kit, design, pieceGuids, connectionGuids);
      expect(copyOp.ok).toBe(true);
      if (!copyOp.ok) return;
      const computedCopy = copyOp.change;

      // 🧩Verify piece and connection counts
      expect((computedCopy.pieces ?? []).length).toBe((expectedCopy.pieces ?? []).length);
      expect((computedCopy.connections ?? []).length).toBe((expectedCopy.connections ?? []).length);

      // 🏷️Verify external piece has semio.piece.origin = "external" and semio.center
      const externalPieces = (computedCopy.pieces ?? []).filter((p) => (p.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external"));
      const expectedExternalPieces = (expectedCopy.pieces ?? []).filter((p) => (p.attributes ?? []).some((a: any) => a.key === "semio.piece.origin" && a.value === "external"));
      expect(externalPieces.length).toBe(expectedExternalPieces.length);
      for (const ext of externalPieces) {
        expect((ext.attributes ?? []).some((a) => a.key === "semio.center")).toBe(true);
      }

      // 📐Verify pp-excl-pc-incl pieces have semio.center and semio.plane attributes
      const ppExclPcInclPieces = (computedCopy.pieces ?? []).filter((p) => (p.attributes ?? []).some((a) => a.key === "semio.center") && !(p.attributes ?? []).some((a) => a.key === "semio.piece.origin"));
      expect(ppExclPcInclPieces.length).toBe(1);
      for (const piece of ppExclPcInclPieces) {
        expect((piece.attributes ?? []).some((a) => a.key === "semio.plane")).toBe(true);
      }
    });

    it("Nakagin Capsule Tower paste without coord", () => {
      const kit = MetabolismKit as unknown as Kit;
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const source = NakaginCapsuleTowerCopyDesign as unknown as Design;
      const expectedDiff = NakaginCapsuleTowerPasteDesignDiff as any;

      const computedDiff = pasteDesign(kit, source, pasteTarget, "original");

      // 🧩Verify added piece count
      const computedAdded = (computedDiff.pieces?.added ?? []).sort((a, b) => a.guid.localeCompare(b.guid));
      const expectedAdded = (expectedDiff.pieces?.added ?? []).sort((a: any, b: any) => a.guid.localeCompare(b.guid));
      expect(computedAdded.length).toBe(expectedAdded.length);

      // 🔌Verify added connection count
      const computedAddedConns = (computedDiff.connections?.added ?? []).sort((a, b) => a.guid.localeCompare(b.guid));
      const expectedAddedConns = (expectedDiff.connections?.added ?? []).sort((a: any, b: any) => a.guid.localeCompare(b.guid));
      expect(computedAddedConns.length).toBe(expectedAddedConns.length);

      // 🏷️Verify no external pieces in paste output
      for (const piece of computedAdded) {
        expect((piece.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(false);
      }

      // 🔗Verify connection u/v preserved
      for (let i = 0; i < computedAddedConns.length; i++) {
        expect(computedAddedConns[i].u).toBeCloseTo(expectedAddedConns[i].u, 3);
        expect(computedAddedConns[i].v).toBeCloseTo(expectedAddedConns[i].v, 3);
      }
    });

    it("Nakagin Capsule Tower paste with coord", () => {
      const kit = MetabolismKit as unknown as Kit;
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const source = NakaginCapsuleTowerCopyDesign as unknown as Design;
      const expectedDiff = NakaginCapsuleTowerPasteWithCoordDesignDiff as any;

      const computedDiff = pasteDesign(kit, source, pasteTarget, "original", { u: 10, v: 10 });

      // 🧩Verify added piece count
      const computedAdded = (computedDiff.pieces?.added ?? []).sort((a, b) => a.guid.localeCompare(b.guid));
      const expectedAdded = (expectedDiff.pieces?.added ?? []).sort((a: any, b: any) => a.guid.localeCompare(b.guid));
      expect(computedAdded.length).toBe(expectedAdded.length);

      // 🔌Verify added connection count
      const computedAddedConns = (computedDiff.connections?.added ?? []).sort((a, b) => a.guid.localeCompare(b.guid));
      const expectedAddedConns = (expectedDiff.connections?.added ?? []).sort((a: any, b: any) => a.guid.localeCompare(b.guid));
      expect(computedAddedConns.length).toBe(expectedAddedConns.length);

      // 📐Verify fixed pieces have offset centers
      for (let i = 0; i < computedAdded.length; i++) {
        if (computedAdded[i].center && expectedAdded[i].center) {
          expect(computedAdded[i].center!.u).toBeCloseTo(expectedAdded[i].center.u, 3);
          expect(computedAdded[i].center!.v).toBeCloseTo(expectedAdded[i].center.v, 3);
        }
      }

      // 🔗Verify connection u/v
      for (let i = 0; i < computedAddedConns.length; i++) {
        expect(computedAddedConns[i].u).toBeCloseTo(expectedAddedConns[i].u, 3);
        expect(computedAddedConns[i].v).toBeCloseTo(expectedAddedConns[i].v, 3);
      }
    });

    it("pasteDesign accepts every built-in anchoring string for Nakagin clipboard", () => {
      const kit = MetabolismKit as unknown as Kit;
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const source = NakaginCapsuleTowerCopyDesign as unknown as Design;
      for (const kind of PASTE_DESIGN_ANCHORING_KINDS) {
        const withoutCoord = pasteDesign(kit, source, pasteTarget, kind);
        expect((withoutCoord.pieces?.added ?? []).length).toBeGreaterThan(0);
        const withCoord = pasteDesign(kit, source, pasteTarget, kind, { u: 10, v: 10 });
        expect((withCoord.pieces?.added ?? []).length).toBe((withoutCoord.pieces?.added ?? []).length);
      }
    });

    it("Nakagin t_f5 and br_sl0 internal connection stays identical to clipboard when pasting with coord", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const flatOp = flattenDesign(kit, design.guid);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const flatDesign = applyDesignDiff(JSON.parse(JSON.stringify(design)), flatOp.change.forward);
      const t5 = "9c1ec7a2-13c2-4d23-b7bd-1efe2663d0a9";
      const br = "5feebbf8-33d9-41ad-a13a-24c271a1860b";
      const connInternal = "eb8ce9ce-091c-4495-a651-fa703748dfef";
      const connParent = "4d5ff333-d70a-43e1-8b7a-8849c8c91405";
      const copyOp2 = copyDesign(kit, flatDesign, [t5, br], [connInternal, connParent]);
      expect(copyOp2.ok).toBe(true);
      if (!copyOp2.ok) return;
      const copied = copyOp2.change;
      const srcConn = copied.connections!.find((c) => c.guid === connInternal)!;
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const withoutCoord = pasteDesign(kit, copied, pasteTarget, "original");
      const withCoord = pasteDesign(kit, copied, pasteTarget, "original", { u: 10, v: 5 });
      const connWo = withoutCoord.connections?.added?.find((c) => c.guid === connInternal);
      const connWi = withCoord.connections?.added?.find((c) => c.guid === connInternal);
      expect(connWo).toBeDefined();
      expect(connWi).toBeDefined();
      expect(connWi!.u).toBeCloseTo(srcConn.u ?? 0, 6);
      expect(connWi!.v).toBeCloseTo(srcConn.v ?? 0, 6);
      expect(connWi!.u).toBeCloseTo(connWo!.u ?? 0, 6);
      expect(connWi!.v).toBeCloseTo(connWo!.v ?? 0, 6);
    });

    it("Nakagin paste remaps t_f2–t_f1 onto target t_f1 when t_f1 is external stub only", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const flatOp3 = flattenDesign(kit, design.guid);
      expect(flatOp3.ok).toBe(true);
      if (!flatOp3.ok) return;
      const flatDesign = applyDesignDiff(JSON.parse(JSON.stringify(design)), flatOp3.change.forward);
      const sel = NakaginCapsuleTowerCopySelection as any;
      const t1 = "31be08e1-e75c-4024-86b4-c3c6d3939fbb";
      const t2t1Conn = "ddf9e0e4-40e1-4079-aa40-c86cf699788b";
      const t1ParentConn = "b1ecc6c5-722a-4814-9047-a87222bbaa4d";
      const pieceGuids = (sel.pieces as { guid: string }[]).map((p) => p.guid).filter((g: string) => g !== t1);
      const connectionGuids = (sel.connections as { guid: string }[]).map((c) => c.guid).filter((g: string) => g !== t1ParentConn);
      expect(connectionGuids).toContain(t2t1Conn);
      const copyOp3 = copyDesign(kit, flatDesign, pieceGuids, connectionGuids);
      expect(copyOp3.ok).toBe(true);
      if (!copyOp3.ok) return;
      const copied = copyOp3.change;
      const stubT1 = copied.pieces!.find((p) => p.guid === t1);
      expect(stubT1 && (stubT1.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(true);
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const diff = pasteDesign(kit, copied, pasteTarget, "original");
      const targetT1 = pasteTarget.pieces!.find((p) => p.name === "t_f1_b_c1");
      expect(targetT1).toBeDefined();
      const remapped = diff.connections?.added?.find((c) => c.guid === t2t1Conn);
      expect(remapped).toBeDefined();
      expect(remapped!.connecting.piece.guid).toBe("9d18882e-d90b-40de-a171-47cb4564ffa6");
      expect(remapped!.connected.piece.guid).toBe(targetT1!.guid);

      const t2 = "9d18882e-d90b-40de-a171-47cb4564ffa6";
      const childBelowT2Conn = "bb5449be-247b-498e-b8c8-309697ddea7b";
      const srcInternal = copied.connections!.find((c) => c.guid === childBelowT2Conn);
      expect(srcInternal).toBeDefined();
      const coord = { u: 10, v: -3.25 };
      const diffCoord = pasteDesign(kit, copied, pasteTarget, "original", coord);
      const remappedCoord = diffCoord.connections?.added?.find((c) => c.guid === t2t1Conn);
      expect(remappedCoord).toBeDefined();
      const t2Piece = copied.pieces!.find((p) => p.guid === t2)!;
      let childU = t2Piece.center?.u ?? 0;
      let childV = t2Piece.center?.v ?? 0;
      const t2cAttr = (t2Piece.attributes ?? []).find((a) => a.key === "semio.center");
      if (t2cAttr?.value) {
        const j = JSON.parse(t2cAttr.value) as Coord;
        childU = j.u;
        childV = j.v;
      }
      const parentU = targetT1!.center?.u ?? 0;
      const parentV = targetT1!.center?.v ?? 0;
      const anchor = { u: 0, v: 0 };
      expect(remappedCoord!.u).toBeCloseTo(parentU - (coord.u + (anchor.u - childU)), 6);
      expect(remappedCoord!.v).toBeCloseTo(parentV - (coord.v + (anchor.v - childV)), 6);
      const internalAfter = diffCoord.connections?.added?.find((c) => c.guid === childBelowT2Conn);
      expect(internalAfter).toBeDefined();
      expect(internalAfter!.u).toBeCloseTo(srcInternal!.u ?? 0, 6);
      expect(internalAfter!.v).toBeCloseTo(srcInternal!.v ?? 0, 6);
    });

    it("copyDesign single connected piece selected alone becomes free fixed root and auto-pulls source descendants", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const tF0BC0 = "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952";

      const sourceConns = design.connections ?? [];
      const sourcePieces = design.pieces ?? [];
      const childMap = new Map<string, Array<{ childGuid: string; connectionGuid: string }>>();
      for (const c of sourceConns) {
        const p = c.connected.piece.guid;
        if (!childMap.has(p)) childMap.set(p, []);
        childMap.get(p)!.push({ childGuid: c.connecting.piece.guid, connectionGuid: c.guid });
      }
      const expectedDescPieces = new Set<string>();
      const expectedDescConns = new Set<string>();
      const queue = [tF0BC0];
      while (queue.length > 0) {
        const cur = queue.shift()!;
        for (const { childGuid, connectionGuid } of childMap.get(cur) ?? []) {
          if (expectedDescPieces.has(childGuid)) continue;
          expectedDescPieces.add(childGuid);
          expectedDescConns.add(connectionGuid);
          queue.push(childGuid);
        }
      }
      expect(expectedDescPieces.size).toBeGreaterThan(0);

      const copyOp = copyDesign(kit, design, [tF0BC0], []);
      expect(copyOp.ok).toBe(true);
      if (!copyOp.ok) return;
      const copied = copyOp.change;
      expect((copied.pieces ?? []).length).toBe(1 + expectedDescPieces.size);
      expect((copied.connections ?? []).length).toBe(expectedDescConns.size);

      const root = copied.pieces!.find((p) => p.guid === tF0BC0)!;
      expect(root.plane).toBeDefined();
      expect(root.center).toBeDefined();
      expect((root.attributes ?? []).some((a) => a.key === "semio.center")).toBe(true);
      expect((root.attributes ?? []).some((a) => a.key === "semio.plane")).toBe(true);
      expect((root.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(false);

      for (const guid of expectedDescPieces) {
        const desc = copied.pieces!.find((p) => p.guid === guid);
        expect(desc).toBeDefined();
        const sourceDesc = sourcePieces.find((p) => p.guid === guid)!;
        expect(JSON.stringify(desc!.center ?? null)).toBe(JSON.stringify(sourceDesc.center ?? null));
        expect(JSON.stringify(desc!.plane ?? null)).toBe(JSON.stringify(sourceDesc.plane ?? null));
        expect((desc!.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(false);
      }
      for (const guid of expectedDescConns) {
        const conn = copied.connections!.find((c) => c.guid === guid);
        expect(conn).toBeDefined();
      }

      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const diff = pasteDesign(kit, copied, pasteTarget, "original");
      const added = diff.pieces?.added ?? [];
      expect(added.length).toBe(1 + expectedDescPieces.size);
      const addedRoot = added.find((p) => p.guid === tF0BC0)!;
      expect(addedRoot.plane).toBeDefined();
      expect(addedRoot.center).toBeDefined();
      expect((diff.connections?.added ?? []).length).toBe(expectedDescConns.size);

      const diffCoord = pasteDesign(kit, copied, pasteTarget, "original", { u: 7, v: -3 });
      const addedCoord = diffCoord.pieces?.added ?? [];
      const addedRootCoord = addedCoord.find((p) => p.guid === tF0BC0)!;
      expect(addedRootCoord.center!.u).toBeCloseTo(root.center!.u + 7, 6);
      expect(addedRootCoord.center!.v).toBeCloseTo(root.center!.v - 3, 6);
      const addedConnsCoord = diffCoord.connections?.added ?? [];
      for (const expConnGuid of expectedDescConns) {
        const sourceConn = copied.connections!.find((c) => c.guid === expConnGuid)!;
        const targetConn = addedConnsCoord.find((c) => c.guid === expConnGuid)!;
        expect(targetConn).toBeDefined();
        expect(targetConn.u ?? 0).toBeCloseTo(sourceConn.u ?? 0, 6);
        expect(targetConn.v ?? 0).toBeCloseTo(sourceConn.v ?? 0, 6);
      }
    });
  });
  // #endregion 📋Copy And Paste Tests

  describe("Design/WithDiff", () => {
    it("Nakagin Capsule Tower with-diff preserves old entities and annotates status", () => {
      const kit = MetabolismKit as unknown as Kit;
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const diff = NakaginCapsuleTowerDiffDesign as unknown as DesignDiff;
      const expected = NakaginCapsuleTowerWithDiffDesign as unknown as Design;
      const computed = designWithDiff(design, diff);

      expect(computed.pieces!.length).toBe(expected.pieces!.length);
      expect(computed.connections!.length).toBe(expected.connections!.length);

      const getStatus = (attrs?: Attribute[]) => (attrs ?? []).find((a) => a.key === "semio.diffStatus")?.value;

      // 🧩Verify piece status counts
      const pieceStatuses = computed.pieces!.map((p) => getStatus(p.attributes));
      expect(pieceStatuses.filter((s) => s === "unchanged").length).toBe(163);
      expect(pieceStatuses.filter((s) => s === "modified").length).toBe(7);
      expect(pieceStatuses.filter((s) => s === "removed").length).toBe(10);
      expect(pieceStatuses.filter((s) => s === "added").length).toBe(5);

      // 🔗Verify connection status counts
      const connStatuses = computed.connections!.map((c) => getStatus(c.attributes));
      expect(connStatuses.filter((s) => s === "unchanged").length).toBe(168);
      expect(connStatuses.filter((s) => s === "modified").length).toBe(1);
      expect(connStatuses.filter((s) => s === "removed").length).toBe(10);
      expect(connStatuses.filter((s) => s === "added").length).toBe(4);

      // ➖Verify removed/unchanged pieces keep their original parameters
      for (const piece of computed.pieces!) {
        if (getStatus(piece.attributes) === "removed" || getStatus(piece.attributes) === "unchanged") {
          const originalPiece = design.pieces!.find((p) => p.guid === piece.guid);
          expect(originalPiece).toBeDefined();
          expect(piece.name).toBe(originalPiece!.name);
          expect(piece.description).toBe(originalPiece!.description);
        }
      }

      // 🔧Verify modified pieces have non-geometric diff applied but keep base plane/center
      const updatedPieceMap = new Map((diff.pieces?.updated ?? []).map((u) => [(u as any).piece.guid, u.diff]));
      for (const piece of computed.pieces!) {
        if (getStatus(piece.attributes) === "modified") {
          const pieceDiff = updatedPieceMap.get(piece.guid);
          const originalPiece = design.pieces!.find((p) => p.guid === piece.guid);
          expect(originalPiece).toBeDefined();
          if (pieceDiff?.name) expect(piece.name).toBe(pieceDiff.name);
          else expect(piece.name).toBe(originalPiece!.name);
          if (pieceDiff?.description !== undefined) expect(piece.description).toBe(pieceDiff.description);
          else expect(piece.description).toBe(originalPiece!.description);
          // 📌Modified pieces MUST keep base geometry so they only get recolored, not moved.
          expect(piece.plane).toEqual(originalPiece!.plane);
          expect(piece.center).toEqual(originalPiece!.center);
        }
      }
    });

    it("modified pieces keep base plane and center even when diff specifies new geometry", () => {
      const basePiece: Piece = {
        guid: "p1",
        name: "Base",
        type: { name: "K" },
        plane: { origin: { x: 1, y: 2, z: 3 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
        center: { u: 4, v: 5 },
      };
      const base: Design = { guid: "d1", name: "D", pieces: [basePiece] };
      const newPlane: Plane = { origin: { x: 9, y: 9, z: 9 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
      const diff: DesignDiff = {
        pieces: {
          updated: [{ piece: { guid: "p1" }, diff: { name: "Renamed", plane: newPlane, center: { u: 99, v: 99 } } }],
        },
      };
      const computed = designWithDiff(base, diff);
      const piece = computed.pieces!.find((p) => p.guid === "p1")!;
      const status = (piece.attributes ?? []).find((a) => a.key === "semio.diffStatus")?.value;
      expect(status).toBe("modified");
      expect(piece.name).toBe("Renamed");
      expect(piece.plane).toEqual(basePiece.plane);
      expect(piece.center).toEqual(basePiece.center);
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

  describe("Elements Bundle", () => {
    it("sources shared element primitives directly from elements ui", () => {
      expect(UiAction).toBe(ElementsBundle.Action);
      expect(buildControlTree).toBe(ElementsBundle.buildControlTree);
      expect(ElementsBundle.LevelProvider).toBeDefined();
      expect(ElementsBundle.SectionSpecificity).toBeDefined();
    });

    it("renders an explicit TreeItem label even when an id is present", () => {
      const html = renderToStaticMarkup(
        createElement(ElementsBundle.Tree, {
          sections: [
            {
              id: "test-section",
              items: [
                {
                  id: "storybook.missing.translation.key",
                  label: createElement("span", { className: "tree-explicit-label" }, "Explicit Tree Label"),
                  icon: createElement("span", null, "∧"),
                },
              ],
            },
          ],
        }),
      );

      expect(html).toContain("Explicit Tree Label");
      expect(html).toContain("tree-explicit-label");
    });
  });

  describe("Coda Tree Descriptors", () => {
    let getOntologyNodeDescriptor: any;
    let getValidationNodeDescriptor: any;
    beforeAll(async () => {
      const mod = await import("../../coda/desktop/renderer");
      getOntologyNodeDescriptor = mod.getOntologyNodeDescriptor;
      getValidationNodeDescriptor = mod.getValidationNodeDescriptor;
    });

    it("keeps ontology fragments and validation witness/count semantics stable", () => {
      const ontologyNode: any = {
        id: "ontology-1",
        kind: "ExactCardinality",
        label: "EXACTLY 2 verbindet",
        fragment: "verbindet exactly 2 (...)",
        children: [],
      };
      const ontologyDescriptor = getOntologyNodeDescriptor(ontologyNode);
      expect(ontologyDescriptor.icon).toBe("=n");
      expect(ontologyDescriptor.primaryText).toBe("EXACTLY 2 verbindet");
      expect(ontologyDescriptor.secondaryText).toBe("verbindet exactly 2 (...)");

      const countedWitness: any = {
        id: "validation-1",
        kind: "Witness",
        label: "Geschoss_EG",
        individual: "Geschoss_EG",
        truth: "true",
        counted: true,
        summary: "counted filler 1 of 2",
        children: [],
      };
      const countedWitnessDescriptor = getValidationNodeDescriptor(countedWitness);
      expect(countedWitnessDescriptor.primaryText).toBe("Geschoss_EG");
      expect(countedWitnessDescriptor.chips).toContain("counted");
      expect(countedWitnessDescriptor.dimmed).toBe(false);

      const notMatchingWitness: any = {
        id: "validation-2",
        kind: "Witness",
        label: "Technikraum_Dach",
        individual: "Technikraum_Dach",
        truth: "unknown",
        counted: false,
        summary: "additional filler that does not satisfy the restriction",
        children: [],
      };
      const notMatchingWitnessDescriptor = getValidationNodeDescriptor(notMatchingWitness);
      expect(notMatchingWitnessDescriptor.chips).toContain("not matching");
      expect(notMatchingWitnessDescriptor.dimmed).toBe(true);

      const cardinalityNode: any = {
        id: "validation-3",
        kind: "ExactCardinality",
        label: "EXACTLY 1 in",
        fragment: "in exactly 1 (...)",
        truth: "true",
        expectedCardinality: 1,
        matchingCount: 1,
        children: [],
      };
      const cardinalityDescriptor = getValidationNodeDescriptor(cardinalityNode);
      expect(cardinalityDescriptor.icon).toBe("=n");
      expect(cardinalityDescriptor.chips).toContain("1/1");
      expect(cardinalityDescriptor.secondaryText).toBe("in exactly 1 (...)");

      const dataValueNode: any = {
        id: "validation-4",
        kind: "DataValue",
        label: "180.0",
        value: "180.0",
        datatype: "xsd:float",
        truth: "true",
        children: [],
      };
      const dataValueDescriptor = getValidationNodeDescriptor(dataValueNode);
      expect(dataValueDescriptor.primaryText).toBe("180.0");
      expect(dataValueDescriptor.chips).toContain("xsd:float");
    });
  });

  describe("Design/Quality/Sum", () => {
    const kit = MetabolismKit as Kit;
    describe("Nakagin Capsule Tower", () => {
      it("sums effective floor area to ~2349.53", () => {
        const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent);
        expect(design).toBeDefined();
        const quality = kit.qualities?.find((q) => q.name === "effective floor area");
        expect(quality).toBeDefined();
        const result = sumQualityInDesign(kit, design!.guid, quality!.guid);
        expect(Math.abs(result - 2349.53)).toBeLessThan(0.01);
      });
    });
  });

  describe("ExportDesignModel", () => {
    const kit = MetabolismKit as Kit;
    const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;

    it("exports .glb format with valid GLB header", async () => {
      const result = await exportDesignModel(kit, design.guid, ".glb");
      expect(result.byteLength).toBeGreaterThan(0);

      const view = new DataView(result);
      const magic = view.getUint32(0, true);
      expect(magic).toBe(0x46546c67);

      const version = view.getUint32(4, true);
      expect(version).toBe(2);

      const totalLength = view.getUint32(8, true);
      expect(totalLength).toBe(result.byteLength);
    });

    it("exports .gltf format as valid JSON string", async () => {
      const result = await exportDesignModel(kit, design.guid, ".gltf");
      const decoder = new TextDecoder();
      const str = decoder.decode(result);
      expect(() => JSON.parse(str)).not.toThrow();
      const parsed = JSON.parse(str);
      expect(parsed).toBeDefined();
      expect(typeof parsed).toBe("object");
    });

    it("EXPORT_MODEL_FORMATS includes .glb and .gltf", () => {
      expect(EXPORT_MODEL_FORMATS[".glb"]).toBeDefined();
      expect(EXPORT_MODEL_FORMATS[".gltf"]).toBeDefined();
    });

    it("exports identical Nakagin scene graph across implementations and writes reports", async () => {
      const { mkdirSync, readFileSync, writeFileSync } = await import("node:fs");
      const { EXPORT_REPORTS_DIR, resolve, __dirname } = await getTestNodePaths();
      mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });

      const jsResult = new Uint8Array(await exportDesignModel(kit, design.guid, ".gltf"));
      await writeExportReport("js", jsResult);

      await runExportReportCommand("uv", ["run", "pytest", "main.py", "-k", "export_scene_graph_report", "-q"], resolve(__dirname, "../py"));
      let skipGo = false;
      try {
        await runExportReportCommand("go", ["test", "./...", "-run", "TestExportDesignModelSceneGraphReport$", "-count=1"], resolve(__dirname, "../go"));
      } catch (e: any) {
        const message = String(e?.message ?? e);
        const looksLikeGoToolchainMismatch = message.includes("requires go >= 1.25.0") && message.includes("go.work lists go 1.24.0");
        if (looksLikeGoToolchainMismatch) {
          // [DEBUG] This repository's Go modules require a newer Go toolchain than the one installed in some CI/dev containers.
          // Skip the cross-implementation "go" comparison in that case; other implementations still run.
          // eslint-disable-next-line no-console
          console.warn(`[DEBUG] skipping go ExportDesignModelSceneGraphReport due to Go toolchain mismatch: ${message}`);
          skipGo = true;
        } else {
          throw e;
        }
      }
      await runExportReportCommand("cargo", ["test", "export_scene_graph_report", "--", "--nocapture"], resolve(__dirname, "../rs"));
      await runExportReportCommand(
        "dotnet",
        ["test", "Semio.Tests.csproj", "-f", "net8.0", "--filter", "FullyQualifiedName=Semio.Tests.Tests+ExportDesignModel.Nakagin_Capsule_Tower_Export_Scene_Graph_Report"],
        resolve(__dirname, "../net/Semio.Tests"),
      );

      const implementations = skipGo ? (["js", "py", "rs", "net"] as const) : (["js", "py", "go", "rs", "net"] as const);
      const normalizedByImplementation = Object.fromEntries(
        implementations.map((implementation) => {
          const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
          const reportText = readFileSync(reportPath, "utf8");
          return [implementation, normalizeSceneGraph(reportText)];
        }),
      );

      writeFileSync(resolve(EXPORT_REPORTS_DIR, "scene-graphs.json"), JSON.stringify(normalizedByImplementation, null, 2));

      const baseline = normalizedByImplementation.js;
      for (const implementation of implementations) {
        expect(normalizedByImplementation[implementation]).toEqual(baseline);
      }

      for (const implementation of implementations) {
        const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
        const reportText = readFileSync(reportPath, "utf8");
        const parsed = JSON.parse(reportText) as { buffers?: Array<{ uri?: string }>; images?: Array<{ uri?: string; bufferView?: number }> };
        for (const buffer of parsed.buffers ?? []) {
          expect(buffer.uri?.startsWith("data:")).toBe(true);
        }
        for (const image of parsed.images ?? []) {
          expect(image.uri?.startsWith("data:") ?? image.bufferView !== undefined).toBe(true);
        }
        const doc = await parseSelfContainedGltf(reportText);
        expect(doc.getRoot().listMeshes().length).toBeGreaterThan(0);
        const meshNames = getMeshNames(reportText);
        expect(meshNames.some((name) => name === "base.glb")).toBe(true);
        expect(meshNames.some((name) => /^capsule_.*\.glb$/i.test(name))).toBe(true);
      }
    }, 300000);
  });

  describe("Model/KPI", () => {
    it("getGeometricInsightsForModel(nakagin-capsule-tower.gltf) returns canonical insights and writes report", async () => {
      const fs = await import("node:fs/promises");
      const { resolve, __dirname } = await getTestNodePaths();
      const modelPath = resolve(__dirname, "../assets/semio/nakagin-capsule-tower.gltf");
      const insights = await getGeometricInsightsForModel(modelPath);
      const round6 = (x: number) => Math.round(x * 1e6) / 1e6;
      const pt = (p: { x: number; y: number; z: number } | undefined) => (p ? { x: round6(p.x), y: round6(p.y), z: round6(p.z) } : undefined);

      const reportsDir = resolve(__dirname, "../reports/model-kpi");
      await fs.mkdir(reportsDir, { recursive: true });
      const report: Record<string, unknown> = {
        aspect_ratio_xy: insights.aspectRatioXy != null ? round6(insights.aspectRatioXy) : undefined,
        aspect_ratio_xz: insights.aspectRatioXz != null ? round6(insights.aspectRatioXz) : undefined,
        aspect_ratio_yz: insights.aspectRatioYz != null ? round6(insights.aspectRatioYz) : undefined,
        bounding_box_max: pt(insights.boundingBoxMax),
        bounding_box_min: pt(insights.boundingBoxMin),
        centroid: pt(insights.centroid),
        characteristic_length: insights.characteristicLength != null ? round6(insights.characteristicLength) : undefined,
        dimension_x: insights.dimensionX != null ? round6(insights.dimensionX) : undefined,
        dimension_y: insights.dimensionY != null ? round6(insights.dimensionY) : undefined,
        dimension_z: insights.dimensionZ != null ? round6(insights.dimensionZ) : undefined,
        face_count: insights.faceCount,
        footprint_area: insights.footprintArea != null ? round6(insights.footprintArea) : undefined,
        is_watertight: insights.isWatertight ?? false,
        slenderness: insights.slenderness != null ? round6(insights.slenderness) : undefined,
        total_surface_area: insights.totalSurfaceArea != null ? round6(insights.totalSurfaceArea) : undefined,
        vertex_count: insights.vertexCount,
      };
      await fs.writeFile(resolve(reportsDir, "js.json"), JSON.stringify(report, null, 2), "utf8");

      const canonicalPath = resolve(__dirname, "../assets/semio/nakagin.kpi.model.semio.json");
      const canonical = JSON.parse(await fs.readFile(canonicalPath, "utf8"));
      const skipKeys = new Set(["centroid", "total_surface_area"]);
      for (const key of Object.keys(canonical)) {
        if (skipKeys.has(key)) continue;
        expect(report[key]).toBeDefined();
        expect(report[key]).toEqual(canonical[key]);
      }
    });
  });

  // #region 🌊InMemoryKitStore Tests
  // Contract tests for InMemoryKitStore MUST verify the full KitStore interface.

  describe("InMemoryKitStore", () => {
    const makeKit = (overrides?: Partial<Kit>): Kit => ({
      guid: "test-kit-guid",
      name: "Test Kit",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      ...overrides,
    });

    it("getSnapshot returns the initial kit and ready status", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBe("test-kit-guid");
      expect(snapshot.kit.name).toBe("Test Kit");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
      expect(snapshot.sync.readonly).toBe(false);
    });

    it("apply merges a diff and notifies subscribers", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      const diff: KitDiff = { name: "Updated Kit" };
      store.apply(diff);

      expect(store.getSnapshot().kit.name).toBe("Updated Kit");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit and notifies subscribers", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ guid: "new-guid", name: "Replaced Kit" });
      store.replace(newKit);

      expect(store.getSnapshot().kit.guid).toBe("new-guid");
      expect(store.getSnapshot().kit.name).toBe("Replaced Kit");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("subscribe returns an unsubscribe function", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      const unsub = store.subscribe(() => notified++);

      store.apply({ name: "First" });
      expect(notified).toBe(1);

      unsub();
      store.apply({ name: "Second" });
      expect(notified).toBe(1);
    });

    it("transact groups mutations into one undo entry", () => {
      const kit = makeKit({ types: [] });
      const store = new InMemoryKitStore(kit);

      store.transact("add type and rename", () => {
        store.apply({ name: "Renamed" });
        store.apply({
          types: {
            added: [{ guid: "t1", name: "Wall", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }],
          },
        });
      });

      const snap = store.getSnapshot();
      expect(snap.kit.name).toBe("Renamed");
      expect(snap.kit.types).toHaveLength(1);
      expect(store.canUndo()).toBe(true);

      store.undo();
      const undone = store.getSnapshot();
      expect(undone.kit.name).toBe("Test Kit");
      expect(undone.kit.types ?? []).toHaveLength(0);
    });

    it("undo reverses the last mutation", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");
      expect(store.canUndo()).toBe(false);
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");

      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);
    });

    it("apply after undo clears the redo stack", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "First" });
      store.apply({ name: "Second" });
      store.undo();
      expect(store.canRedo()).toBe(true);

      store.apply({ name: "Third" });
      expect(store.canRedo()).toBe(false);
      expect(store.getSnapshot().kit.name).toBe("Third");
    });

    it("save clears dirty flag", async () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
    });

    it("dispose clears all listeners and stacks", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Before dispose" });
      expect(notified).toBe(1);

      store.dispose();
      store.apply({ name: "After dispose" });
      expect(notified).toBe(1);
      expect(store.canUndo()).toBe(false);
    });

    it("multiple subscribers are all notified", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let count1 = 0;
      let count2 = 0;
      store.subscribe(() => count1++);
      store.subscribe(() => count2++);

      store.apply({ name: "Changed" });
      expect(count1).toBe(1);
      expect(count2).toBe(1);
    });

    it("undo and redo with no stack are no-ops", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");

      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Test Kit");
    });
  });

  // #endregion 🌊InMemoryKitStore Tests

  // #region ⛅JsonFileKitStore Tests
  // Contract tests for JsonFileKitStore MUST verify the full UndoableKitStore interface
  // including file I/O, save, reload, undo/redo, and external update handling.

  describe("JsonFileKitStore", () => {
    const makeKit = (overrides?: Partial<Kit>): Kit => ({
      guid: "file-kit-guid",
      name: "File Kit",
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: "2026-01-01T00:00:00.000Z",
      ...overrides,
    });

    const makeAdapter = (initialKit?: Kit): KitJsonFileAdapter & { stored: string | null } => {
      const adapter = {
        stored: initialKit ? JSON.stringify(initialKit) : null,
        async read(): Promise<string | null> {
          return adapter.stored;
        },
        async write(json: string): Promise<void> {
          adapter.stored = json;
        },
      };
      return adapter;
    };

    it("loads kit from adapter and reports ready status", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBe("file-kit-guid");
      expect(snapshot.kit.name).toBe("File Kit");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
      expect(snapshot.sync.lastSyncedAt).toBeDefined();
    });

    it("creates empty kit when adapter returns null", async () => {
      const adapter = makeAdapter();
      const store = await createJsonFileKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBeDefined();
      expect(snapshot.kit.name).toBe("New Kit");
      expect(snapshot.sync.status).toBe("ready");
    });

    it("reports error status for invalid JSON", async () => {
      const adapter = {
        stored: "not valid json {{{",
        async read() {
          return adapter.stored;
        },
        async write(json: string) {
          adapter.stored = json;
        },
      };
      const store = await createJsonFileKitStore(adapter);
      expect(store.getSnapshot().sync.status).toBe("error");
      expect(store.getSnapshot().sync.error).toBeDefined();
    });

    it("apply merges a diff and notifies subscribers", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Updated" });
      expect(store.getSnapshot().kit.name).toBe("Updated");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ guid: "new-guid", name: "Replaced" });
      store.replace(newKit);
      expect(store.getSnapshot().kit.guid).toBe("new-guid");
      expect(store.getSnapshot().kit.name).toBe("Replaced");
      expect(notified).toBe(1);
    });

    it("save writes kit JSON to adapter and clears dirty", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);

      store.apply({ name: "Saved Kit" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.getSnapshot().sync.status).toBe("ready");

      const savedKit = JSON.parse(adapter.stored!);
      expect(savedKit.name).toBe("Saved Kit");
    });

    it("reload re-reads kit from adapter and resets state", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);

      store.apply({ name: "Local Change" });
      expect(store.getSnapshot().kit.name).toBe("Local Change");

      // Simulate external file change
      adapter.stored = JSON.stringify(makeKit({ name: "External Change" }));

      await store.reload();
      expect(store.getSnapshot().kit.name).toBe("External Change");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("undo reverses the last mutation", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.apply({ name: "Changed" });
      expect(store.canUndo()).toBe(true);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("File Kit");
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.apply({ name: "Changed" });
      store.undo();
      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);
    });

    it("transact groups mutations into one undo entry", async () => {
      const kit = makeKit({ types: [] });
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.transact("batch", () => {
        store.apply({ name: "Renamed" });
        store.apply({
          types: {
            added: [{ guid: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          },
        });
      });

      expect(store.getSnapshot().kit.name).toBe("Renamed");
      expect(store.getSnapshot().kit.types).toHaveLength(1);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("File Kit");
      expect(store.getSnapshot().kit.types ?? []).toHaveLength(0);
    });

    it("subscribe returns unsubscribe function", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      const unsub = store.subscribe(() => notified++);

      store.apply({ name: "First" });
      expect(notified).toBe(1);

      unsub();
      store.apply({ name: "Second" });
      expect(notified).toBe(1);
    });

    it("dispose clears listeners and stacks", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Before" });
      expect(notified).toBe(1);

      store.dispose();
      store.apply({ name: "After" });
      expect(notified).toBe(1);
      expect(store.canUndo()).toBe(false);
    });

    it("applyExternalUpdate resets state without undo entry", async () => {
      const kit = makeKit();
      const store = await createJsonFileKitStore(makeAdapter(kit));

      store.apply({ name: "Local" });
      expect(store.canUndo()).toBe(true);

      store.applyExternalUpdate(makeKit({ name: "External" }));
      expect(store.getSnapshot().kit.name).toBe("External");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("save transitions through saving status", async () => {
      const kit = makeKit();
      const statuses: string[] = [];
      const store = await createJsonFileKitStore(makeAdapter(kit));
      store.subscribe(() => statuses.push(store.getSnapshot().sync.status));

      store.apply({ name: "Changed" });
      await store.save();

      expect(statuses).toContain("saving");
      expect(store.getSnapshot().sync.status).toBe("ready");
    });

    it("embedFileBlob inlines dropped file as data URL persisted in kit JSON", async () => {
      const fileGuid = "file-1";
      const kit = makeKit({
        files: [
          {
            guid: fileGuid,
            name: "cube.txt",
            size: 5,
            createdAt: "2026-01-01T00:00:00.000Z",
            updatedAt: "2026-01-01T00:00:00.000Z",
          },
        ],
      });
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);

      const blob = new Blob(["HELLO"], { type: "text/plain" });
      await store.embedFileBlob(fileGuid, blob);

      const fileAfter = store.getSnapshot().kit.files?.find((f) => f.guid === fileGuid);
      expect(fileAfter?.blob).toBeDefined();
      expect(fileAfter!.blob!.startsWith("data:text/plain")).toBe(true);
      expect(fileAfter!.blob).toContain("base64,");

      await store.save();
      const saved = JSON.parse(adapter.stored!);
      const persistedFile = saved.files.find((f: any) => f.guid === fileGuid);
      expect(persistedFile.blob).toBe(fileAfter!.blob);

      // Round-trip: reloading the JSON preserves the embedded blob.
      const reloaded = await createJsonFileKitStore(adapter);
      const reloadedFile = reloaded.getSnapshot().kit.files?.find((f) => f.guid === fileGuid);
      expect(reloadedFile?.blob).toBe(fileAfter!.blob);
    });

    it("addFile diff followed by embedFileBlob embeds the blob on the newly added file", async () => {
      // Simulates executeKitCommand("semio.kit.addFile", ...) → syncKitFileCommandResult → embedFileBlob.
      // Step 1: apply the addFile diff (what kitCommands["semio.kit.addFile"] returns).
      // Step 2: embedFileBlob reads the file from kit.files and applies a second diff setting blob.
      const adapter = makeAdapter(makeKit());
      const store = await createJsonFileKitStore(adapter);

      const newFileGuid = "dropped-file-guid";
      const newFile = {
        guid: newFileGuid,
        name: "drop.txt",
        size: 3,
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
      };
      store.apply({ files: { added: [newFile] } });

      const addedFile = store.getSnapshot().kit.files?.find((f) => f.guid === newFileGuid);
      expect(addedFile).toBeDefined();
      expect(addedFile?.blob).toBeUndefined();

      const blob = new Blob(["HEY"], { type: "text/plain" });
      await store.embedFileBlob(newFileGuid, blob);

      const embeddedFile = store.getSnapshot().kit.files?.find((f) => f.guid === newFileGuid);
      expect(embeddedFile?.blob).toBeDefined();
      expect(embeddedFile!.blob!.startsWith("data:text/plain")).toBe(true);
      expect(embeddedFile?.name).toBe("drop.txt");

      await store.save();
      const saved = JSON.parse(adapter.stored!);
      const persistedFile = saved.files.find((f: any) => f.guid === newFileGuid);
      expect(persistedFile.blob).toBe(embeddedFile!.blob);
      expect(persistedFile.name).toBe("drop.txt");
    });

    it("save preserves dirty flag when an apply interleaves with an in-flight adapter.write", async () => {
      // Regression: JsonFileKitStore.embedFileBlob awaits blob.arrayBuffer()
      // which yields to the event loop. If a scheduled save() fires during
      // that await, save() serializes the pre-embed kit and clears dirty
      // after adapter.write — clobbering the embed apply that ran mid-save.
      // save() MUST only clear dirty when the kit reference is unchanged, so
      // the next auto-save still runs with the embedded blob.
      let resolveFirstWrite: (() => void) | null = null;
      let writeCount = 0;
      const adapter = {
        stored: null as string | null,
        async read(): Promise<string | null> {
          return adapter.stored;
        },
        async write(json: string): Promise<void> {
          writeCount++;
          if (writeCount === 1) {
            adapter.stored = json;
            await new Promise<void>((resolve) => {
              resolveFirstWrite = resolve;
            });
          } else {
            adapter.stored = json;
          }
        },
      };
      const fileGuid = "file-race";
      adapter.stored = JSON.stringify({
        guid: "race-kit",
        name: "Race",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
        files: [
          {
            guid: fileGuid,
            name: "race.txt",
            size: 2,
            createdAt: "2026-01-01T00:00:00.000Z",
            updatedAt: "2026-01-01T00:00:00.000Z",
          },
        ],
      });
      const store = await createJsonFileKitStore(adapter);

      const savePromise = store.save();
      await Promise.resolve();

      const blob = new Blob(["HI"], { type: "text/plain" });
      await store.embedFileBlob(fileGuid, blob);

      resolveFirstWrite!();
      await savePromise;

      expect(store.getSnapshot().sync.dirty).toBe(true);
      const embeddedFile = store.getSnapshot().kit.files?.find((f) => f.guid === fileGuid);
      expect(embeddedFile?.blob).toBeDefined();
      expect(embeddedFile!.blob!.startsWith("data:text/plain")).toBe(true);

      await store.save();
      const saved = JSON.parse(adapter.stored!);
      const persistedFile = saved.files.find((f: any) => f.guid === fileGuid);
      expect(persistedFile.blob).toBe(embeddedFile!.blob);
    });

    it("embedFileBlob is a no-op when the target file is missing from the kit", async () => {
      const store = await createJsonFileKitStore(makeAdapter(makeKit()));
      const blob = new Blob(["X"], { type: "application/octet-stream" });
      await store.embedFileBlob("nonexistent", blob);
      expect(store.getSnapshot().kit.files ?? []).toHaveLength(0);
      expect(store.getSnapshot().sync.dirty).toBe(false);
    });

    it("getFileDiff and inverseFileDiff include blob for kit change metadata", () => {
      const before: File = {
        guid: "f1",
        name: "a.bin",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
      };
      const after: File = {
        ...before,
        blob: "data:application/octet-stream;base64,QUI=",
      };
      const forward = getFileDiff(before, after);
      expect(forward.blob).toBe(after.blob);
      const backward = inverseFileDiff(before, forward);
      expect(backward.blob).toBeUndefined();
    });
  });

  // #endregion ⛅JsonFileKitStore Tests

  // #region 🔊FolderKitStore Tests
  describe("FolderKitStore", () => {
    const makeKit = (overrides?: Partial<Kit>): Kit => ({
      guid: "folder-kit-guid",
      name: "Folder Kit",
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: "2026-01-01T00:00:00.000Z",
      ...overrides,
    });

    const kitToBytes = async (kit: Kit): Promise<Uint8Array> => {
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(kit, db);
      const data = db.export();
      db.close();
      return data;
    };

    const bytesToKit = async (data: Uint8Array): Promise<Kit> => {
      const SQL = await getSqlJs();
      const db = new SQL.Database(new Uint8Array(data));
      const kit = await sqliteToKit(db);
      db.close();
      return kit;
    };

    const makeAdapter = (initialKit?: Kit): KitFolderAdapter & { stored: Uint8Array | null; files: Map<string, Blob>; initPromise: Promise<void> } => {
      const adapter = {
        stored: null as Uint8Array | null,
        files: new Map<string, Blob>(),
        initPromise: Promise.resolve(),
        async readKit(): Promise<Uint8Array | null> {
          return adapter.stored;
        },
        async writeKit(data: Uint8Array): Promise<void> {
          adapter.stored = data;
        },
        async readFile(path: string): Promise<Blob | null> {
          return adapter.files.get(path) ?? null;
        },
        async writeFile(path: string, blob: Blob): Promise<void> {
          adapter.files.set(path, blob);
        },
        async deleteFile(path: string): Promise<void> {
          adapter.files.delete(path);
        },
        async listFiles(): Promise<string[]> {
          return Array.from(adapter.files.keys());
        },
      };
      if (initialKit) {
        adapter.initPromise = kitToBytes(initialKit).then((data) => {
          adapter.stored = data;
        });
      }
      return adapter;
    };

    it("loads kit from adapter and reports ready status", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBe("folder-kit-guid");
      expect(snapshot.kit.name).toBe("Folder Kit");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
    });

    it("creates empty kit when adapter returns null", async () => {
      const store = await createFolderKitStore(makeAdapter());
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.guid).toBeDefined();
      expect(snapshot.kit.name).toBe("New Kit");
      expect(snapshot.sync.status).toBe("ready");
    });

    it("reports error status for invalid SQLite data", async () => {
      const adapter = makeAdapter();
      adapter.stored = new Uint8Array([0, 1, 2, 3]);
      const store = await createFolderKitStore(adapter);
      expect(store.getSnapshot().sync.status).toBe("error");
      expect(store.getSnapshot().sync.error).toBeDefined();
    });

    it("apply merges a diff and notifies subscribers", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Updated" });
      expect(store.getSnapshot().kit.name).toBe("Updated");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ guid: "new-guid", name: "Replaced" });
      store.replace(newKit);
      expect(store.getSnapshot().kit.guid).toBe("new-guid");
      expect(store.getSnapshot().kit.name).toBe("Replaced");
      expect(notified).toBe(1);
    });

    it("save writes kit SQLite to adapter and clears dirty", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Saved Kit" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.getSnapshot().sync.status).toBe("ready");

      const savedKit = await bytesToKit(adapter.stored!);
      expect(savedKit.name).toBe("Saved Kit");
    });

    it("reload re-reads kit from adapter and resets state", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Local Change" });
      expect(store.getSnapshot().kit.name).toBe("Local Change");

      adapter.stored = await kitToBytes(makeKit({ name: "External Change" }));

      await store.reload();
      expect(store.getSnapshot().kit.name).toBe("External Change");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("undo reverses the last mutation", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Changed" });
      expect(store.canUndo()).toBe(true);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Folder Kit");
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Changed" });
      store.undo();
      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Changed");
      expect(store.canUndo()).toBe(true);
      expect(store.canRedo()).toBe(false);
    });

    it("transact groups mutations into one undo entry", async () => {
      const kit = makeKit({ types: [] });
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.transact("batch", () => {
        store.apply({ name: "Renamed" });
        store.apply({
          types: {
            added: [{ guid: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          },
        });
      });

      expect(store.getSnapshot().kit.name).toBe("Renamed");
      expect(store.getSnapshot().kit.types).toHaveLength(1);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Folder Kit");
      expect(store.getSnapshot().kit.types ?? []).toHaveLength(0);
    });

    it("subscribe returns unsubscribe function", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      const unsub = store.subscribe(() => notified++);

      store.apply({ name: "First" });
      expect(notified).toBe(1);

      unsub();
      store.apply({ name: "Second" });
      expect(notified).toBe(1);
    });

    it("dispose clears listeners and stacks", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      let notified = 0;
      store.subscribe(() => notified++);

      store.apply({ name: "Before" });
      expect(notified).toBe(1);

      store.dispose();
      store.apply({ name: "After" });
      expect(notified).toBe(1);
      expect(store.canUndo()).toBe(false);
    });

    it("applyExternalUpdate resets state without undo entry", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Local" });
      expect(store.canUndo()).toBe(true);

      store.applyExternalUpdate(makeKit({ name: "External" }));
      expect(store.getSnapshot().kit.name).toBe("External");
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.canUndo()).toBe(false);
    });

    it("writeFile and readFile roundtrip via adapter", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      const blob = new Blob(["hello world"], { type: "text/plain" });
      await store.writeFile("test.txt", blob);

      const read = await store.readFile("test.txt");
      expect(read).not.toBeNull();
      const text = await read!.text();
      expect(text).toBe("hello world");
    });

    it("deleteFile removes a stored file", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      await store.writeFile("to-delete.txt", new Blob(["data"]));
      expect(await store.readFile("to-delete.txt")).not.toBeNull();

      await store.deleteFile("to-delete.txt");
      expect(await store.readFile("to-delete.txt")).toBeNull();
    });

    it("listFiles returns all file paths", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      await store.writeFile("a.txt", new Blob(["a"]));
      await store.writeFile("b.txt", new Blob(["b"]));

      const files = await store.listFiles();
      expect(files).toContain("a.txt");
      expect(files).toContain("b.txt");
      expect(files).toHaveLength(2);
    });

    it("save transitions through saving status", async () => {
      const kit = makeKit();
      const statuses: string[] = [];
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);
      store.subscribe(() => statuses.push(store.getSnapshot().sync.status));

      store.apply({ name: "Changed" });
      await store.save();

      expect(statuses).toContain("saving");
      expect(store.getSnapshot().sync.status).toBe("ready");
    });
  });
  // #endregion 🔊FolderKitStore Tests

  // #region 🚪Open Synchronized Kit E2E Tests
  // End-to-end tests for opening synchronized kits across all three supported source kinds:
  // file (*.kit.semio.json with embedded base64 blobs), folder (.semio/kit.db + binary files on disk),
  // and remote (SessionKitStore over HTTP + WebSocket against semio/server).
  // Specs: These tests MUST verify the full open → mutate → save/sync → reload cycle using real file
  // system access or mocked server transport to guarantee the desktop/vscode/web entry points work.

  describe("Open Synchronized Kit E2E", () => {
    const { createJsonFileKitStore: makeJsonFileKitStore, createFolderKitStore: makeFolderKitStore, createSessionKitStore: makeSessionKitStore } = (async () => await import("@semio/sketchpad"))() as any;

    const loadStudio = async () => {
      const studio = await import("@semio/sketchpad");
      return studio;
    };

    const getMetabolismKitJsonPath = async (): Promise<string> => {
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const here = dirname(fileURLToPath(import.meta.url));
      return resolve(here, "../assets/semio/metabolism.kit.semio.json");
    };

    const getMetabolismFolderPath = async (): Promise<string> => {
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const here = dirname(fileURLToPath(import.meta.url));
      return resolve(here, "../assets/semio/metabolism");
    };

    const makeNodeJsonFileAdapter = async (filePath: string) => {
      const fs = await import("node:fs/promises");
      return {
        async read(): Promise<string | null> {
          try {
            return await fs.readFile(filePath, "utf-8");
          } catch {
            return null;
          }
        },
        async write(json: string): Promise<void> {
          await fs.writeFile(filePath, json, "utf-8");
        },
      };
    };

    const makeNodeFolderAdapter = async (folderPath: string) => {
      const fs = await import("node:fs/promises");
      const nodePath = await import("node:path");
      return {
        async readKit(): Promise<Uint8Array | null> {
          try {
            const buf = await fs.readFile(nodePath.join(folderPath, ".semio", "kit.db"));
            return new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
          } catch {
            return null;
          }
        },
        async writeKit(data: Uint8Array): Promise<void> {
          const dir = nodePath.join(folderPath, ".semio");
          await fs.mkdir(dir, { recursive: true });
          await fs.writeFile(nodePath.join(dir, "kit.db"), Buffer.from(data));
        },
        async readFile(rel: string): Promise<Blob | null> {
          try {
            const buf = await fs.readFile(nodePath.join(folderPath, rel));
            return new Blob([new Uint8Array(buf)]);
          } catch {
            return null;
          }
        },
        async writeFile(rel: string, blob: Blob): Promise<void> {
          const abs = nodePath.join(folderPath, rel);
          await fs.mkdir(nodePath.dirname(abs), { recursive: true });
          const ab = await blob.arrayBuffer();
          await fs.writeFile(abs, Buffer.from(ab));
        },
        async deleteFile(rel: string): Promise<void> {
          try {
            await fs.unlink(nodePath.join(folderPath, rel));
          } catch {
            /* ignore */
          }
        },
        async createDirectory(rel: string): Promise<void> {
          await fs.mkdir(nodePath.join(folderPath, rel), { recursive: true });
        },
        async moveEntry(fromRel: string, toRel: string): Promise<void> {
          const sourcePath = nodePath.join(folderPath, fromRel);
          const targetPath = nodePath.join(folderPath, toRel);
          await fs.mkdir(nodePath.dirname(targetPath), { recursive: true });
          await fs.rename(sourcePath, targetPath);
        },
        async listFiles(): Promise<string[]> {
          const out: string[] = [];
          const walk = async (dir: string, base: string) => {
            const entries = await fs.readdir(dir, { withFileTypes: true });
            for (const entry of entries) {
              if (entry.name === ".semio" || entry.name === "node_modules") continue;
              const rel = base ? `${base}/${entry.name}` : entry.name;
              if (entry.isDirectory()) await walk(nodePath.join(dir, entry.name), rel);
              else out.push(rel);
            }
          };
          try {
            await walk(folderPath, "");
          } catch {
            /* ignore */
          }
          return out;
        },
      };
    };

    describe("File Kit (JsonFileKitStore)", () => {
      it("opens metabolism.kit.semio.json with embedded blob files preserved", async () => {
        const studio = await loadStudio();
        const filePath = await getMetabolismKitJsonPath();
        const adapter = await makeNodeJsonFileAdapter(filePath);
        const store = await studio.createJsonFileKitStore(adapter);

        const snap = store.getSnapshot();
        expect(snap.sync.status).toBe("ready");
        expect(snap.kit.name).toBe("Metabolism");
        expect((snap.kit.types ?? []).length).toBeGreaterThan(0);
        expect((snap.kit.designs ?? []).length).toBeGreaterThan(0);
        expect((snap.kit.files ?? []).length).toBeGreaterThan(0);

        const filesWithBlob = (snap.kit.files ?? []).filter((f) => typeof f.blob === "string" && f.blob.length > 0);
        expect(filesWithBlob.length).toBeGreaterThan(0);
        const glb = filesWithBlob.find((f) => f.name.endsWith(".glb"));
        expect(glb).toBeDefined();
        expect(glb!.blob!.startsWith("data:")).toBe(true);
      });

      it("synchronizes apply() → save() back to the JSON file on disk", async () => {
        const fs = await import("node:fs/promises");
        const os = await import("node:os");
        const nodePath = await import("node:path");
        const srcPath = await getMetabolismKitJsonPath();
        const tmpDir = await fs.mkdtemp(nodePath.join(os.tmpdir(), "semio-file-kit-"));
        const tmpPath = nodePath.join(tmpDir, "metabolism.kit.semio.json");
        await fs.copyFile(srcPath, tmpPath);

        try {
          const studio = await loadStudio();
          const adapter = await makeNodeJsonFileAdapter(tmpPath);
          const store = await studio.createJsonFileKitStore(adapter);

          store.apply({ description: "Edited via JsonFileKitStore E2E" });
          expect(store.getSnapshot().sync.dirty).toBe(true);
          await store.save();
          expect(store.getSnapshot().sync.dirty).toBe(false);

          const rawAfter = JSON.parse(await fs.readFile(tmpPath, "utf-8"));
          expect(rawAfter.description).toBe("Edited via JsonFileKitStore E2E");
          expect(rawAfter.name).toBe("Metabolism");
          expect(Array.isArray(rawAfter.files)).toBe(true);
          expect(rawAfter.files.length).toBeGreaterThan(0);
        } finally {
          await fs.rm(tmpDir, { recursive: true, force: true });
        }
      });

      it("synchronizes type and piece edits round-trip through the JSON file", async () => {
        const fs = await import("node:fs/promises");
        const os = await import("node:os");
        const nodePath = await import("node:path");
        const tmpDir = await fs.mkdtemp(nodePath.join(os.tmpdir(), "semio-file-kit-edit-"));
        const tmpPath = nodePath.join(tmpDir, "mini.kit.semio.json");
        const initial: Kit = {
          guid: "mini-kit-guid",
          name: "Mini Kit",
          createdAt: "2026-01-01T00:00:00.000Z",
          updatedAt: "2026-01-01T00:00:00.000Z",
          types: [],
        };
        await fs.writeFile(tmpPath, JSON.stringify(initial, null, 2));

        try {
          const studio = await loadStudio();
          const adapter = await makeNodeJsonFileAdapter(tmpPath);
          const store = await studio.createJsonFileKitStore(adapter);

          store.apply({
            types: {
              added: [{ guid: "t1", name: "Column", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
            },
          });
          await store.save();

          const onDisk = JSON.parse(await fs.readFile(tmpPath, "utf-8"));
          expect(onDisk.types).toHaveLength(1);
          expect(onDisk.types[0].name).toBe("Column");

          const reopened = await studio.createJsonFileKitStore(await makeNodeJsonFileAdapter(tmpPath));
          expect(reopened.getSnapshot().kit.types?.[0]?.name).toBe("Column");
        } finally {
          await fs.rm(tmpDir, { recursive: true, force: true });
        }
      });
    });

    describe("Folder Kit (FolderKitStore)", () => {
      it("opens existing metabolism folder via .semio/kit.db without creating a new kit", async () => {
        const studio = await loadStudio();
        const folderPath = await getMetabolismFolderPath();
        const adapter = await makeNodeFolderAdapter(folderPath);
        const store = await studio.createFolderKitStore(adapter);

        const snap = store.getSnapshot();
        expect(snap.sync.status).toBe("ready");
        expect(snap.kit.guid).not.toBe("");
        expect(snap.kit.name).toBe("Metabolism");
        expect((snap.kit.types ?? []).length).toBeGreaterThan(0);
      });

      it("reads real binary files (e.g. representations/base.glb) via the folder adapter", async () => {
        const studio = await loadStudio();
        const folderPath = await getMetabolismFolderPath();
        const adapter = await makeNodeFolderAdapter(folderPath);
        const store = await studio.createFolderKitStore(adapter);

        const blob = await store.readFile("representations/base.glb");
        expect(blob).not.toBeNull();
        expect(blob!.size).toBeGreaterThan(0);

        const files = await store.listFiles();
        expect(files.some((p) => p === "representations/base.glb")).toBe(true);
      });

      it("loads types with models pointing at kit files so 3D meshes resolve", async () => {
        const studio = await loadStudio();
        const folderPath = await getMetabolismFolderPath();
        const adapter = await makeNodeFolderAdapter(folderPath);
        const store = await studio.createFolderKitStore(adapter);
        const kit = store.getSnapshot().kit;

        const typesWithModels = (kit.types ?? []).filter((t: any) => (t.models ?? []).length > 0);
        expect(typesWithModels.length).toBeGreaterThan(0);

        const fileGuidSet = new Set((kit.files ?? []).map((f: any) => f.guid));
        for (const type of typesWithModels) {
          for (const model of (type as any).models ?? []) {
            expect(model.file?.guid).toBeDefined();
            expect(fileGuidSet.has(model.file.guid)).toBe(true);
          }
        }

        const firstModel = typesWithModels[0].models?.[0];
        const firstFile = (kit.files ?? []).find((f: any) => f.guid === firstModel?.file?.guid);
        expect(firstFile).toBeDefined();
        const storagePath = (() => {
          const foldersByGuid = new Map((kit.folders ?? []).map((f: any) => [f.guid, f]));
          const segments: string[] = [firstFile!.name];
          let current = firstFile!.folder?.guid;
          while (current) {
            const folder: any = foldersByGuid.get(current);
            if (!folder) break;
            segments.unshift(folder.name);
            current = folder.parent?.guid;
          }
          return segments.join("/");
        })();
        const blob = await store.readFile(storagePath);
        expect(blob).not.toBeNull();
        expect(blob!.size).toBeGreaterThan(0);
      });

      it("synchronizes apply() → save() back to .semio/kit.db on disk", async () => {
        const fs = await import("node:fs/promises");
        const os = await import("node:os");
        const nodePath = await import("node:path");
        const tmpDir = await fs.mkdtemp(nodePath.join(os.tmpdir(), "semio-folder-kit-"));

        try {
          const studio = await loadStudio();
          const adapter = await makeNodeFolderAdapter(tmpDir);
          const initial = await studio.createFolderKitStore(adapter);
          initial.replace({
            guid: "seeded-folder-kit",
            name: "Seeded Folder Kit",
            createdAt: "2026-01-01T00:00:00.000Z",
            updatedAt: "2026-01-01T00:00:00.000Z",
            types: [{ guid: "seed-type", name: "Seed", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          });
          await initial.save();

          const kitDbPath = nodePath.join(tmpDir, ".semio", "kit.db");
          const stat = await fs.stat(kitDbPath);
          expect(stat.size).toBeGreaterThan(0);

          initial.apply({
            types: {
              added: [{ guid: "added-type", name: "Added", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
            },
          });
          await initial.save();

          const reopened = await studio.createFolderKitStore(await makeNodeFolderAdapter(tmpDir));
          const snap = reopened.getSnapshot();
          expect(snap.kit.name).toBe("Seeded Folder Kit");
          expect((snap.kit.types ?? []).map((t: any) => t.name).sort()).toEqual(["Added", "Seed"]);
        } finally {
          await fs.rm(tmpDir, { recursive: true, force: true });
        }
      });
    });

    describe("Remote Kit (SessionKitStore)", () => {
      const makeMockWebSocket = () => {
        const instances: any[] = [];
        class MockWebSocket {
          onopen: any = null;
          onmessage: any = null;
          onclose: any = null;
          onerror: any = null;
          readyState = 1;
          sent: string[] = [];
          url: string;
          constructor(url: string) {
            this.url = url;
            instances.push(this);
            setTimeout(() => this.onopen?.(), 0);
          }
          send(data: string) {
            this.sent.push(data);
          }
          close() {
            this.readyState = 3;
            this.onclose?.();
          }
          emit(event: any) {
            this.onmessage?.({ data: JSON.stringify(event) });
          }
        }
        return { MockWebSocket, instances };
      };

      it("creates a remote session, loads snapshot, and handles server events", async () => {
        const studio = await loadStudio();
        const { MockWebSocket, instances } = makeMockWebSocket();
        const originalFetch = globalThis.fetch;
        const originalWebSocket = (globalThis as any).WebSocket;
        (globalThis as any).WebSocket = MockWebSocket;

        const snapshotKit = {
          guid: "remote-session-kit",
          name: "Remote Session Kit",
          types: [],
          designs: [],
          createdAt: "2026-01-01T00:00:00.000Z",
          updatedAt: "2026-01-01T00:00:00.000Z",
        };
        globalThis.fetch = (async (input: any, init?: any) => {
          const url = String(input);
          if (url.endsWith("/sessions") && init?.method === "POST") {
            return new Response(JSON.stringify({ session_id: "session-42" }), {
              status: 200,
              headers: { "content-type": "application/json" },
            });
          }
          if (url.endsWith("/sessions/session-42/snapshot")) {
            return new Response(JSON.stringify({ kit: snapshotKit, domain_version: 1, semio_version: 0 }), {
              status: 200,
              headers: { "content-type": "application/json" },
            });
          }
          if (url.endsWith("/sessions/session-42/commands") && init?.method === "POST") {
            return new Response("{}", { status: 200, headers: { "content-type": "application/json" } });
          }
          return new Response("{}", { status: 200 });
        }) as typeof fetch;

        try {
          const store = await studio.createSessionKitStore({ serverUrl: "http://localhost:12345", kitName: "Remote Session Kit" });
          expect(store.sessionId).toBe("session-42");
          expect(store.getSnapshot().kit.name).toBe("Remote Session Kit");
          expect(store.getSnapshot().sync.status).toBe("ready");

          // Wait for mock ws to fire onopen
          await new Promise((r) => setTimeout(r, 5));
          const ws = instances[0];
          expect(ws).toBeDefined();

          // Server pushes a type creation event
          ws.emit({
            event: "DomainCommandAccepted",
            domain_version: 2,
            changes: [
              {
                op: "Created",
                entity_kind: "type",
                entity_id: "remote-type-1",
                snapshot: { name: "Remote Type", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" },
              },
            ],
          });
          const types = store.getSnapshot().kit.types ?? [];
          expect(types.some((t: any) => t.guid === "remote-type-1" && t.name === "Remote Type")).toBe(true);
          store.dispose?.();
        } finally {
          globalThis.fetch = originalFetch;
          (globalThis as any).WebSocket = originalWebSocket;
        }
      });
    });
  });
  // #endregion 🚪Open Synchronized Kit E2E Tests

  // #region 🎀Meta And Shallow Tests
  // Tests for Meta and Shallow schema parsing, conversion functions, and roundtrips.
  describe("Meta/Shallow", () => {
    describe("Kit/Meta", () => {
      it("parses metabolism.meta.kit.semio.json with KitMetaSchema", () => {
        const parsed = KitMetaSchema.parse(MetabolismMetaKit);
        expect(parsed.name).toBe("Metabolism");
        expect(parsed.guid).toBe("f042c2a4-3ba5-44b0-b22c-0ae8f568aacc");
        expect((parsed as any).types).toBeUndefined();
        expect((parsed as any).designs).toBeUndefined();
        expect((parsed as any).files).toBeUndefined();
      });
      it("toKitMeta strips collections from full kit", () => {
        const kit = MetabolismKit as unknown as Kit;
        const meta = toKitMeta(kit);
        expect(meta.name).toBe("Metabolism");
        expect((meta as any).types).toBeUndefined();
        expect((meta as any).designs).toBeUndefined();
        expect((meta as any).files).toBeUndefined();
      });
      it("roundtrips KitMeta through serialize/deserialize", () => {
        const kit = MetabolismKit as unknown as Kit;
        const meta = toKitMeta(kit);
        const serialized = serializeKitMeta(meta);
        const deserialized = deserializeKitMeta(serialized);
        expect(deserialized.name).toBe(meta.name);
        expect(deserialized.guid).toBe(meta.guid);
      });
    });

    describe("Kit/Shallow", () => {
      it("parses metabolism.shallow.kit.semio.json with KitShallowSchema", () => {
        const parsed = KitShallowSchema.parse(MetabolismShallowKit);
        expect(parsed.name).toBe("Metabolism");
        expect(parsed.types).toBeDefined();
        expect(parsed.types!.length).toBeGreaterThan(0);
        // 🏷️Shallow types should be meta (no nested collections like models)
        const firstType = parsed.types![0] as any;
        expect(firstType.models).toBeUndefined();
        expect(firstType.connectors).toBeUndefined();
      });
      it("toKitShallow converts full kit to shallow with meta children", () => {
        const kit = MetabolismKit as unknown as Kit;
        const shallow = toKitShallow(kit);
        expect(shallow.name).toBe("Metabolism");
        expect(shallow.types).toBeDefined();
        expect(shallow.types!.length).toBeGreaterThan(0);
        const firstType = shallow.types![0] as any;
        expect(firstType.models).toBeUndefined();
        expect(firstType.connectors).toBeUndefined();
      });
      it("roundtrips KitShallow through serialize/deserialize", () => {
        const kit = MetabolismKit as unknown as Kit;
        const shallow = toKitShallow(kit);
        const serialized = serializeKitShallow(shallow);
        const deserialized = deserializeKitShallow(serialized);
        expect(deserialized.name).toBe(shallow.name);
        expect(deserialized.types!.length).toBe(shallow.types!.length);
      });
    });

    describe("Type/Meta", () => {
      it("parses tambour.meta.type.semio.json with TypeMetaSchema", () => {
        const parsed = TypeMetaSchema.parse(TambourMetaType);
        expect(parsed.name).toBe("Tambour");
        expect(parsed.guid).toBe("2a6bb3e8-4adb-44a3-bc87-3314b77b40f7");
        expect((parsed as any).models).toBeUndefined();
        expect((parsed as any).connectors).toBeUndefined();
        expect((parsed as any).props).toBeUndefined();
      });
      it("toTypeMeta strips collections from full type", () => {
        const kit = MetabolismKit as unknown as Kit;
        const tambour = kit.types!.find((t: Type) => t.name === "Tambour")!;
        const meta = toTypeMeta(tambour);
        expect(meta.name).toBe("Tambour");
        expect((meta as any).models).toBeUndefined();
        expect((meta as any).connectors).toBeUndefined();
      });
    });

    describe("Type/Shallow", () => {
      it("parses tambour.shallow.type.semio.json with TypeShallowSchema", () => {
        const parsed = TypeShallowSchema.parse(TambourShallowType);
        expect(parsed.name).toBe("Tambour");
        if (parsed.models) {
          const firstModel = parsed.models[0] as any;
          expect(firstModel.tags).toBeUndefined();
        }
      });
      it("toTypeShallow converts full type to shallow with meta children", () => {
        const kit = MetabolismKit as unknown as Kit;
        const tambour = kit.types!.find((t: Type) => t.name === "Tambour")!;
        const shallow = toTypeShallow(tambour);
        expect(shallow.name).toBe("Tambour");
        if (shallow.models) {
          const firstModel = shallow.models[0] as any;
          expect(firstModel.tags).toBeUndefined();
        }
      });
    });

    describe("Design/Meta", () => {
      it("parses nakagin-capsule-tower.meta.design.semio.json with DesignMetaSchema", () => {
        const parsed = DesignMetaSchema.parse(NakaginCapsuleTowerMetaDesign);
        expect(parsed.name).toBe("Nakagin Capsule Tower");
        expect(parsed.guid).toBe("9a890dd4-0a9c-48ac-920a-9e62666465ef");
        expect((parsed as any).pieces).toBeUndefined();
        expect((parsed as any).connections).toBeUndefined();
      });
      it("toDesignMeta strips collections from full design", () => {
        const kit = MetabolismKit as unknown as Kit;
        const nct = kit.designs!.find((d: Design) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
        const meta = toDesignMeta(nct);
        expect(meta.name).toBe("Nakagin Capsule Tower");
        expect((meta as any).pieces).toBeUndefined();
        expect((meta as any).connections).toBeUndefined();
      });
    });

    describe("Design/Shallow", () => {
      it("parses nakagin-capsule-tower.shallow.design.semio.json with DesignShallowSchema", () => {
        const parsed = DesignShallowSchema.parse(NakaginCapsuleTowerShallowDesign);
        expect(parsed.name).toBe("Nakagin Capsule Tower");
        if (parsed.pieces) {
          const firstPiece = parsed.pieces[0] as any;
          expect(firstPiece.attributes).toBeUndefined();
        }
      });
      it("toDesignShallow converts full design to shallow with meta children", () => {
        const kit = MetabolismKit as unknown as Kit;
        const nct = kit.designs!.find((d: Design) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
        const shallow = toDesignShallow(nct);
        expect(shallow.name).toBe("Nakagin Capsule Tower");
        if (shallow.pieces) {
          const firstPiece = shallow.pieces[0] as any;
          expect(firstPiece.attributes).toBeUndefined();
        }
      });
    });
  });
  // #endregion 🎀Meta And Shallow Tests

  // #region 🗝️Hash Tests
  describe("Kit/Hash", () => {
    it("hashKit produces a 64-char lowercase hex string", () => {
      const kit = MetabolismKit as unknown as Kit;
      const h = hashKit(kit);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKit is deterministic (same input produces same output)", () => {
      const kitA = JSON.parse(JSON.stringify(MetabolismKit)) as Kit;
      const kitB = JSON.parse(JSON.stringify(MetabolismKit)) as Kit;
      expect(hashKit(kitA)).toBe(hashKit(kitB));
    });

    it("hashDesign produces a 64-char lowercase hex string", () => {
      const kit = MetabolismKit as unknown as Kit;
      const nct = kit.designs!.find((d: Design) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const h = hashDesign(nct);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashType produces a 64-char lowercase hex string", () => {
      const kit = MetabolismKit as unknown as Kit;
      const t = kit.types![0];
      const h = hashType(t);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("different kits produce different hashes", () => {
      const kit1 = MetabolismKit as unknown as Kit;
      const kit2 = { ...kit1, name: "Different Name" };
      expect(hashKit(kit1)).not.toBe(hashKit(kit2));
    });

    it("sha256 of empty input matches known value", () => {
      const h = sha256bytes(new Uint8Array(0));
      expect(h).toBe("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    });

    it("sha256 of 'abc' matches known value", () => {
      const h = sha256bytes(new TextEncoder().encode("abc"));
      expect(h).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    });

    it("hashPiece is deterministic", () => {
      const kit = MetabolismKit as unknown as Kit;
      const nct = kit.designs!.find((d: Design) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const piece = nct.pieces![0];
      const h1 = hashPiece(piece);
      const h2 = hashPiece(piece);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashConnection is deterministic", () => {
      const kit = MetabolismKit as unknown as Kit;
      const nct = kit.designs!.find((d: Design) => d.name === "Nakagin Capsule Tower" && !d.parent)!;
      const conn = nct.connections![0];
      const h1 = hashConnection(conn);
      const h2 = hashConnection(conn);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashConnector is deterministic", () => {
      const kit = MetabolismKit as unknown as Kit;
      const t = kit.types!.find((t: Type) => t.connectors && t.connectors.length > 0)!;
      const conn = t.connectors![0];
      const h1 = hashConnector(conn);
      const h2 = hashConnector(conn);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff is deterministic and produces valid hash", () => {
      const kit = MetabolismKit as unknown as Kit;
      const modified = { ...kit, name: "Modified Kit", description: "New description" };
      const diff = getKitDiff(kit, modified);
      const h1 = hashKitDiff(diff);
      const h2 = hashKitDiff(diff);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff produces different hashes for different diffs", () => {
      const kit = MetabolismKit as unknown as Kit;
      const mod1 = { ...kit, name: "Modified1" };
      const mod2 = { ...kit, name: "Modified2" };
      const diff1 = getKitDiff(kit, mod1);
      const diff2 = getKitDiff(kit, mod2);
      expect(hashKitDiff(diff1)).not.toBe(hashKitDiff(diff2));
    });

    it("hashKitDiff empty diff produces a consistent hash", () => {
      const kit = MetabolismKit as unknown as Kit;
      const diff = getKitDiff(kit, kit);
      const h = hashKitDiff(diff);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashAttributeDiff is deterministic", () => {
      const d: AttributeDiff = { key: "newKey", value: "newValue" };
      const h1 = hashAttributeDiff(d);
      const h2 = hashAttributeDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashCoordDiff is deterministic", () => {
      const d: CoordDiff = { u: 1.0, v: 2.0 };
      const h1 = hashCoordDiff(d);
      const h2 = hashCoordDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashTypeDiff with collection diffs is deterministic", () => {
      const kit = MetabolismKit as unknown as Kit;
      if (kit.types && kit.types.length >= 2) {
        const modified = {
          ...kit,
          types: kit.types.map((t: Type, i: number) => (i === 0 ? { ...t, description: "Updated type description" } : t)),
        };
        const diff = getKitDiff(kit, modified);
        if (diff.types) {
          const h1 = hashTypesDiff(diff.types);
          const h2 = hashTypesDiff(diff.types);
          expect(h1).toBe(h2);
          expect(h1).toMatch(/^[0-9a-f]{64}$/);
        }
      }
    });

    it("hashDesignDiff is deterministic", () => {
      const kit = MetabolismKit as unknown as Kit;
      if (kit.designs && kit.designs.length >= 1) {
        const modified = {
          ...kit,
          designs: kit.designs.map((d: Design, i: number) => (i === 0 ? { ...d, description: "Updated design" } : d)),
        };
        const diff = getKitDiff(kit, modified);
        if (diff.designs) {
          const h1 = hashDesignsDiff(diff.designs);
          const h2 = hashDesignsDiff(diff.designs);
          expect(h1).toBe(h2);
          expect(h1).toMatch(/^[0-9a-f]{64}$/);
        }
      }
    });

    it("hashPlaneDiff is deterministic", () => {
      const d: PlaneDiff = {
        origin: { x: 1.0, y: 2.0 },
        xAxis: { x: 1.0 },
      };
      const h1 = hashPlaneDiff(d);
      const h2 = hashPlaneDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashSideDiff is deterministic", () => {
      const d: SideDiff = { piece: { guid: "p1" } };
      const h1 = hashSideDiff(d);
      const h2 = hashSideDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashConnectionDiff is deterministic", () => {
      const d: ConnectionDiff = { gap: 0.5, rotation: 90 };
      const h1 = hashConnectionDiff(d);
      const h2 = hashConnectionDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashStatDiff is deterministic", () => {
      const d: StatDiff = { min: 0, max: 100, unit: "mm" };
      const h1 = hashStatDiff(d);
      const h2 = hashStatDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff with type addition produces valid hash", () => {
      const kit = MetabolismKit as unknown as Kit;
      const newType: Type = { guid: "new-type-guid", name: "NewType" };
      const modified = { ...kit, types: [...(kit.types ?? []), newType] };
      const diff = getKitDiff(kit, modified);
      const h = hashKitDiff(diff);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff matches expected canonical value", () => {
      const diff: KitDiff = { name: "updated", description: null };
      const h = hashKitDiff(diff);
      expect(h).toBe("d9ee3052111fec2e0fe08119eee6b8d5b6f5578a940f6d5c6bb1806e6e0f36a5");
    });
  });
  // #endregion 🗝️Hash Tests
  // #region 📊MaxChildren Tests
  describe("MaxChildren", () => {
    describe("Port", () => {
      it("Port schema accepts maxChildren", () => {
        const port: Port = { guid: "p1", name: "TestPort", maxChildren: 3 };
        const parsed = PortSchema.parse(port);
        expect(parsed.maxChildren).toBe(3);
      });

      it("Port schema allows omitting maxChildren", () => {
        const port: Port = { guid: "p1", name: "TestPort" };
        const parsed = PortSchema.parse(port);
        expect(parsed.maxChildren).toBeUndefined();
      });

      it("Port diff detects maxChildren change", () => {
        const before: Port = { guid: "p1", name: "TestPort", maxChildren: 1 };
        const after: Port = { guid: "p1", name: "TestPort", maxChildren: 5 };
        const diff = getPortDiff(before, after);
        expect(diff.maxChildren).toBe(5);
      });

      it("Port diff detects maxChildren removal", () => {
        const before: Port = { guid: "p1", name: "TestPort", maxChildren: 3 };
        const after: Port = { guid: "p1", name: "TestPort" };
        const diff = getPortDiff(before, after);
        expect(diff.maxChildren).toBeNull();
      });

      it("Port diff ignores unchanged maxChildren", () => {
        const before: Port = { guid: "p1", name: "TestPort", maxChildren: 2 };
        const after: Port = { guid: "p1", name: "TestPort", maxChildren: 2 };
        const diff = getPortDiff(before, after);
        expect(diff.maxChildren).toBeUndefined();
      });

      it("Port apply diff sets maxChildren", () => {
        const base: Port = { guid: "p1", name: "TestPort" };
        const diff: PortDiff = { maxChildren: 4 };
        const result = applyPortDiff(base, diff);
        expect(result.maxChildren).toBe(4);
      });

      it("Port apply diff removes maxChildren with null", () => {
        const base: Port = { guid: "p1", name: "TestPort", maxChildren: 3 };
        const diff: PortDiff = { maxChildren: null };
        const result = applyPortDiff(base, diff);
        expect(result.maxChildren).toBeUndefined();
      });

      it("Port inverse diff restores maxChildren", () => {
        const original: Port = { guid: "p1", name: "TestPort", maxChildren: 2 };
        const diff: PortDiff = { maxChildren: 5 };
        const inverse = inversePortDiff(original, diff);
        expect(inverse.maxChildren).toBe(2);
      });
    });

    describe("Connector", () => {
      it("Connector schema accepts maxChildren", () => {
        const connector: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 3 };
        const parsed = ConnectorSchema.parse(connector);
        expect(parsed.maxChildren).toBe(3);
      });

      it("Connector schema allows omitting maxChildren", () => {
        const connector: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 } };
        const parsed = ConnectorSchema.parse(connector);
        expect(parsed.maxChildren).toBeUndefined();
      });

      it("Connector diff detects maxChildren change", () => {
        const before: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 1 };
        const after: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 5 };
        const diff = getConnectorDiff(before, after);
        expect(diff.maxChildren).toBe(5);
      });

      it("Connector diff detects maxChildren removal", () => {
        const before: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 3 };
        const after: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 } };
        const diff = getConnectorDiff(before, after);
        expect(diff.maxChildren).toBeNull();
      });

      it("Connector apply diff sets maxChildren", () => {
        const base: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 } };
        const diff: ConnectorDiff = { maxChildren: 4 };
        const result = applyConnectorDiff(base, diff);
        expect(result.maxChildren).toBe(4);
      });

      it("Connector apply diff removes maxChildren with null", () => {
        const base: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 3 };
        const diff: ConnectorDiff = { maxChildren: null };
        const result = applyConnectorDiff(base, diff);
        expect(result.maxChildren).toBeUndefined();
      });

      it("Connector inverse diff restores maxChildren", () => {
        const original: Connector = { guid: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 2 };
        const diff: ConnectorDiff = { maxChildren: 5 };
        const inverse = inverseConnectorDiff(original, diff);
        expect(inverse.maxChildren).toBe(2);
      });
    });

    describe("Kit Roundtrip", () => {
      it("Kit with maxChildren roundtrips through JSON", () => {
        const kit: Kit = {
          guid: "kit-1",
          name: "TestKit",
          ports: [{ guid: "p1", name: "Port1", maxChildren: 3 }],
          types: [
            {
              guid: "t1",
              name: "Type1",
              connectors: [
                {
                  guid: "c1",
                  t: 0,
                  point: { x: 0, y: 0, z: 0 },
                  direction: { x: 0, y: 0, z: 1 },
                  maxChildren: 5,
                },
              ],
            },
          ],
        };
        const serialized = serializeKit(kit);
        const deserialized = deserializeKit(serialized);
        expect(deserialized.ports![0].maxChildren).toBe(3);
        expect(deserialized.types![0].connectors![0].maxChildren).toBe(5);
      });

      it("Kit diff captures maxChildren changes on both port and connector", () => {
        const before: Kit = {
          guid: "kit-1",
          name: "TestKit",
          ports: [{ guid: "p1", name: "Port1", maxChildren: 1 }],
          types: [
            {
              guid: "t1",
              name: "Type1",
              connectors: [
                {
                  guid: "c1",
                  t: 0,
                  point: { x: 0, y: 0, z: 0 },
                  direction: { x: 0, y: 0, z: 1 },
                  maxChildren: 1,
                },
              ],
            },
          ],
        };
        const after: Kit = {
          guid: "kit-1",
          name: "TestKit",
          ports: [{ guid: "p1", name: "Port1", maxChildren: 10 }],
          types: [
            {
              guid: "t1",
              name: "Type1",
              connectors: [
                {
                  guid: "c1",
                  t: 0,
                  point: { x: 0, y: 0, z: 0 },
                  direction: { x: 0, y: 0, z: 1 },
                  maxChildren: 20,
                },
              ],
            },
          ],
        };
        const diff = getKitDiff(before, after);
        expect(diff.ports?.updated?.[0]?.diff.maxChildren).toBe(10);
        expect(diff.types?.updated?.[0]?.diff.connectors?.updated?.[0]?.diff.maxChildren).toBe(20);
        const applied = applyKitDiff(before, diff);
        expect(applied.ports![0].maxChildren).toBe(10);
        expect(applied.types![0].connectors![0].maxChildren).toBe(20);
      });
    });
  });
  // #endregion 📊MaxChildren Tests

  // #region 🔄Transaction Undo/Redo Tests
  // Tests for the transaction state machine contract used by PlainAppStore, PlainKitDiffAppStore,
  // and the event handler factories (createKeyedTransactionHandlers, createSingleKeyTransactionHandlers).
  // Invariant: finalize merges edits via first.undo + last.do; redo is cleared on commit or recordEdit;
  //            fresh start preserves redo; abort discards current stack; undo/redo move between past/redo stacks.

  // #region 🔄Transaction State Helpers
  // Pure-function transaction state machine matching the exact behavior of the existing
  // PlainAppStore class methods and event handler factories in @semio/sketchpad.

  interface TxEdit {
    do: { value: string };
    undo: { value: string };
  }

  interface TxState {
    isTransactionActive: boolean;
    currentTransactionStack: TxEdit[];
    pastTransactionStack: TxEdit[];
    redoStack: TxEdit[];
  }

  const createTxState = (): TxState => ({
    isTransactionActive: false,
    currentTransactionStack: [],
    pastTransactionStack: [],
    redoStack: [],
  });

  const txStart = (s: TxState): TxState => {
    if (s.isTransactionActive) {
      const finalized = txCommit(s);
      return { ...finalized, isTransactionActive: true, currentTransactionStack: [] };
    }
    return { ...s, isTransactionActive: true, currentTransactionStack: [] };
  };

  const txCommit = (s: TxState): TxState => {
    if (!s.isTransactionActive) return s;
    const pastStack = [...s.pastTransactionStack];
    if (s.currentTransactionStack.length > 0) {
      const edits = s.currentTransactionStack;
      const merged: TxEdit = edits.length === 1 ? edits[0] : { do: edits[edits.length - 1].do, undo: edits[0].undo };
      pastStack.push(merged);
    }
    return { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] };
  };

  const txAbort = (s: TxState): TxState => {
    if (!s.isTransactionActive) return s;
    return { ...s, isTransactionActive: false, currentTransactionStack: [] };
  };

  const txRecordEdit = (s: TxState, edit: TxEdit): TxState => {
    if (!s.isTransactionActive) return s;
    return { ...s, currentTransactionStack: [...s.currentTransactionStack, edit], redoStack: [] };
  };

  const txUndo = (s: TxState): TxState => {
    if (s.isTransactionActive) {
      if (s.currentTransactionStack.length === 0) return s;
      const stack = [...s.currentTransactionStack];
      stack.pop();
      return { ...s, currentTransactionStack: stack };
    }
    if (s.pastTransactionStack.length === 0) return s;
    const pastStack = [...s.pastTransactionStack];
    const edit = pastStack.pop()!;
    return { ...s, pastTransactionStack: pastStack, redoStack: [...s.redoStack, edit] };
  };

  const txRedo = (s: TxState): TxState => {
    if (s.isTransactionActive) return s;
    if (s.redoStack.length === 0) return s;
    const redoStack = [...s.redoStack];
    const edit = redoStack.pop()!;
    return { ...s, pastTransactionStack: [...s.pastTransactionStack, edit], redoStack };
  };

  const mkEdit = (doVal: string, undoVal: string): TxEdit => ({
    do: { value: doVal },
    undo: { value: undoVal },
  });

  // #endregion 🔄Transaction State Helpers

  describe("Transaction Undo/Redo", () => {
    it("single commit places edit in past stack", () => {
      let s = createTxState();
      s = txStart(s);
      const edit = mkEdit("d1", "u1");
      s = txRecordEdit(s, edit);
      s = txCommit(s);
      expect(s.isTransactionActive).toBe(false);
      expect(s.pastTransactionStack).toHaveLength(1);
      expect(s.pastTransactionStack[0]).toEqual(edit);
      expect(s.currentTransactionStack).toHaveLength(0);
      expect(s.redoStack).toHaveLength(0);
    });

    it("multi-step transaction merges first.undo + last.do", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dA", "uA"));
      s = txRecordEdit(s, mkEdit("dB", "uB"));
      s = txRecordEdit(s, mkEdit("dC", "uC"));
      s = txCommit(s);
      expect(s.pastTransactionStack).toHaveLength(1);
      const merged = s.pastTransactionStack[0];
      expect(merged.do).toEqual({ value: "dC" });
      expect(merged.undo).toEqual({ value: "uA" });
    });

    it("empty transaction is ignored on commit", () => {
      let s = createTxState();
      s = txStart(s);
      s = txCommit(s);
      expect(s.pastTransactionStack).toHaveLength(0);
      expect(s.isTransactionActive).toBe(false);
    });

    it("abort discards current transaction stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txRecordEdit(s, mkEdit("d2", "u2"));
      s = txAbort(s);
      expect(s.isTransactionActive).toBe(false);
      expect(s.currentTransactionStack).toHaveLength(0);
      expect(s.pastTransactionStack).toHaveLength(0);
    });

    it("abort does not affect committed past stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txCommit(s);
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d2", "u2"));
      s = txAbort(s);
      expect(s.pastTransactionStack).toHaveLength(1);
      expect(s.pastTransactionStack[0].do).toEqual({ value: "d1" });
    });

    it("undo moves last committed transaction to redo stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txCommit(s);
      s = txUndo(s);
      expect(s.pastTransactionStack).toHaveLength(0);
      expect(s.redoStack).toHaveLength(1);
      expect(s.redoStack[0].do).toEqual({ value: "d1" });
    });

    it("redo moves last undone transaction back to past stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txCommit(s);
      s = txUndo(s);
      s = txRedo(s);
      expect(s.pastTransactionStack).toHaveLength(1);
      expect(s.redoStack).toHaveLength(0);
      expect(s.pastTransactionStack[0].do).toEqual({ value: "d1" });
    });

    it("redo invalidation: new commit clears redo stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dA", "uA"));
      s = txCommit(s);
      s = txUndo(s);
      expect(s.redoStack).toHaveLength(1);
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dB", "uB"));
      s = txCommit(s);
      expect(s.redoStack).toHaveLength(0);
      expect(s.pastTransactionStack).toHaveLength(1);
      expect(s.pastTransactionStack[0].do).toEqual({ value: "dB" });
    });

    it("redo invalidation: recording edit clears redo stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dA", "uA"));
      s = txCommit(s);
      s = txUndo(s);
      expect(s.redoStack).toHaveLength(1);
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dB", "uB"));
      expect(s.redoStack).toHaveLength(0);
    });

    it("undo boundary: undo with empty past stack is no-op", () => {
      let s = createTxState();
      const before = { ...s };
      s = txUndo(s);
      expect(s).toEqual(before);
    });

    it("redo boundary: redo with empty redo stack is no-op", () => {
      let s = createTxState();
      const before = { ...s };
      s = txRedo(s);
      expect(s).toEqual(before);
    });

    it("redo is blocked during active transaction", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dA", "uA"));
      s = txCommit(s);
      s = txUndo(s);
      expect(s.redoStack).toHaveLength(1);
      s = txStart(s);
      const sBeforeRedo = { ...s, redoStack: [...s.redoStack] };
      s = txRedo(s);
      expect(s.redoStack).toEqual(sBeforeRedo.redoStack);
    });

    it("undo inside active transaction pops from current stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txRecordEdit(s, mkEdit("d2", "u2"));
      expect(s.currentTransactionStack).toHaveLength(2);
      s = txUndo(s);
      expect(s.currentTransactionStack).toHaveLength(1);
      expect(s.currentTransactionStack[0].do).toEqual({ value: "d1" });
      s = txUndo(s);
      expect(s.currentTransactionStack).toHaveLength(0);
    });

    it("fresh start preserves redo stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dA", "uA"));
      s = txCommit(s);
      s = txUndo(s);
      expect(s.redoStack).toHaveLength(1);
      s = txStart(s);
      expect(s.redoStack).toHaveLength(1);
      expect(s.isTransactionActive).toBe(true);
    });

    it("nested start auto-finalizes previous transaction", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("dA", "uA"));
      s = txStart(s);
      expect(s.pastTransactionStack).toHaveLength(1);
      expect(s.pastTransactionStack[0].do).toEqual({ value: "dA" });
      expect(s.isTransactionActive).toBe(true);
      expect(s.currentTransactionStack).toHaveLength(0);
    });

    it("multiple undo/redo cycles are symmetric", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txCommit(s);
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d2", "u2"));
      s = txCommit(s);
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d3", "u3"));
      s = txCommit(s);
      expect(s.pastTransactionStack).toHaveLength(3);
      s = txUndo(s);
      s = txUndo(s);
      expect(s.pastTransactionStack).toHaveLength(1);
      expect(s.redoStack).toHaveLength(2);
      s = txRedo(s);
      expect(s.pastTransactionStack).toHaveLength(2);
      expect(s.redoStack).toHaveLength(1);
      s = txRedo(s);
      expect(s.pastTransactionStack).toHaveLength(3);
      expect(s.redoStack).toHaveLength(0);
    });

    it("undo all then redo all restores original stack", () => {
      let s = createTxState();
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d1", "u1"));
      s = txCommit(s);
      s = txStart(s);
      s = txRecordEdit(s, mkEdit("d2", "u2"));
      s = txCommit(s);
      const twoCommitted = s.pastTransactionStack.map((e) => ({ ...e }));
      s = txUndo(s);
      s = txUndo(s);
      expect(s.pastTransactionStack).toHaveLength(0);
      s = txRedo(s);
      s = txRedo(s);
      expect(s.pastTransactionStack).toHaveLength(2);
      expect(s.pastTransactionStack[0]).toEqual(twoCommitted[0]);
      expect(s.pastTransactionStack[1]).toEqual(twoCommitted[1]);
    });

    it("recordEdit outside active transaction is no-op", () => {
      let s = createTxState();
      const edit = mkEdit("d1", "u1");
      s = txRecordEdit(s, edit);
      expect(s.currentTransactionStack).toHaveLength(0);
      expect(s.pastTransactionStack).toHaveLength(0);
    });

    it("kit diff roundtrip: apply then inverse restores original state", () => {
      const original: Kit = {
        name: "UndoKit",
        types: [
          {
            guid: "t1",
            name: "Wall",
            description: "A wall segment",
            icon: "",
            variant: "",
          },
        ],
        designs: [],
      };
      const diff: KitDiff = {
        types: {
          added: [
            {
              guid: "t2",
              name: "Column",
              description: "A column",
              icon: "",
              variant: "",
            },
          ],
          updated: [{ type: { guid: "t1" }, diff: { description: "Modified wall" } }],
        },
      };
      const afterForward = applyKitDiff(original, diff);
      expect(afterForward.types).toHaveLength(2);
      expect(afterForward.types![0].description).toBe("Modified wall");
      const inverseDiff = inverseKitDiff(original, diff);
      const afterBackward = applyKitDiff(afterForward, inverseDiff);
      expect(afterBackward.types).toHaveLength(1);
      expect(afterBackward.types![0].description).toBe("A wall segment");
      expect(afterBackward.types![0].guid).toBe("t1");
    });
  });

  // #endregion 🔄Transaction Undo/Redo Tests
} // end vitest guard
// #endregion 🧪Tests

// #region 🏋️Benchmarks
// Performance benchmarks for kit roundtrip, diff, flatten and validation operations.
// MUST NOT be exported. MUST NOT auto-execute on import.
// Run via: npx tsx index.ts --bench

// Number of iterations per benchmark run.
// ⏱️MUST be at least 1 for meaningful timing.
const BENCH_ITERATIONS = 3;

// Runs a function multiple times, measures elapsed time and logs CSV output.
// ⏱️MUST await async functions within the iteration loop.
async function bench(name: string, fn: () => Promise<void> | void) {
  const start = performance.now();
  for (let i = 0; i < BENCH_ITERATIONS; i++) {
    await fn();
  }
  const end = performance.now();
  const durationSec = (end - start) / 1000 / BENCH_ITERATIONS;
  console.log(`${name},${durationSec.toFixed(6)}`);
}

// 🚩Runs all benchmarks. MUST only be called explicitly (e.g. via CLI flag).
async function runBenchmarks() {
  const DiffForward = (await import("@semio/assets/semio/metabolism.kit.diff.semio.json")).default;
  const DiffInverse = (await import("@semio/assets/semio/metabolism.kit.diff.inverted.semio.json")).default;
  const BenchMetabolismKit = (await import("@semio/assets/semio/metabolism.kit.semio.json")).default;
  const BenchInvalidKit = (await import("@semio/assets/semio/invalid.kit.semio.json")).default;

  const kitMetabolism = BenchMetabolismKit as unknown as Kit;
  const kitInvalid = BenchInvalidKit as unknown as Kit;
  const diffForward = DiffForward as unknown as KitDiff;
  const diffInverse = DiffInverse as unknown as KitDiff;

  const findBenchDesign = (kit: Kit, name: string, parentName?: string) => {
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

  await bench("Roundtrip/Metabolism", async () => {
    const fs = await import("fs");
    const path = await import("path");
    const zipPath = path.resolve("../assets/semio/metabolism.zip");
    const zipBuffer = fs.readFileSync(zipPath);
    const { kit } = await importKit(zipBuffer);
    await exportKit(kit);
  });

  await bench("Diff/Metabolism", () => {
    const k2 = applyKitDiff(kitMetabolism, diffForward);
    applyKitDiff(k2, diffInverse);
  });

  const d1 = findBenchDesign(kitMetabolism, "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower", () => {
    const r = flattenDesign(kitMetabolism, d1.guid);
    if (!r.ok) throw new Error(r.errors.map((e) => e.message).join("; "));
  });

  const d2 = findBenchDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower/Slanted", () => {
    const r = flattenDesign(kitMetabolism, d2.guid);
    if (!r.ok) throw new Error(r.errors.map((e) => e.message).join("; "));
  });

  const d3 = findBenchDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower/Twisted", () => {
    const r = flattenDesign(kitMetabolism, d3.guid);
    if (!r.ok) throw new Error(r.errors.map((e) => e.message).join("; "));
  });

  const d4 = findBenchDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
  await bench("Flatten Design/Nakagin Capsule Tower/Dancing", () => {
    const r = flattenDesign(kitMetabolism, d4.guid);
    if (!r.ok) throw new Error(r.errors.map((e) => e.message).join("; "));
  });

  const d5 = findBenchDesign(kitMetabolism, "Capsule Dream");
  await bench("Flatten Design/Capsule Dream", () => {
    const r = flattenDesign(kitMetabolism, d5.guid);
    if (!r.ok) throw new Error(r.errors.map((e) => e.message).join("; "));
  });

  await bench("Validation/Invalid Kit", () => {
    validateKit(kitInvalid);
  });

  await bench("Validation/Metabolism", () => {
    validateKit(kitMetabolism);
  });
}

if (typeof process !== "undefined" && process.argv?.includes("--bench")) {
  runBenchmarks();
}

// #endregion 🏋️Benchmarks
