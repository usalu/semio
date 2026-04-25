// @ts-nocheck
// #region ­ƒº▓Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core domain representation types, schemas and utilities for the semio platform.

// #endregion ­ƒº▓Header

// #region Ôø®´©ÅImports
// External dependency imports MUST be declared here.
import { Accessor as GltfAccessor, Buffer as GltfBuffer, Document as GltfDocument, Material as GltfMaterial, Mesh as GltfMesh, Node as GltfNode, Texture as GltfTexture, NodeIO } from "@gltf-transform/core";
import { default as adjectives } from "@semio/assets/lists/adjectives.json" with { type: "json" };
import { default as animals } from "@semio/assets/lists/animals.json" with { type: "json" };
import { ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import * as THREE from "three";
import { v7 as uuidv7 } from "uuid";
import { z } from "zod";
// #endregion Ôø®´©ÅImports

// #region ­ƒÄ×´©ÅConstants
// Global constants MUST define shared numeric parameters.

/** Standard icon width in pixels.
 **/
export const ICON_WIDTH = 50;
/**
 * Numeric tolerance for floating-point comparisons.
 **/
export const TOLERANCE = 1e-5;

// #endregion ­ƒÄ×´©ÅConstants

// #region ­ƒôªUtilities
// General-purpose utility functions MUST be defined here.

/**
 **/
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 **/
export const id = () => uuidv7();
// ­ƒÄ▓SeededRandom provides deterministic pseudo-random number generation.
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
 * Type alias for Id.
 **/
export type Id = string;

// #endregion ­ƒôªUtilities

// #region ­ƒÉìEntity IDs
// Entity identifier types and comparison functions MUST be defined here.

/**
 * Identifier type for Attribute entities.
 **/
export type AttributeId = { id: Id };
/**
 * Identifier type for Location entities.
 **/
export type LocationId = { id: Id };
/**
 * Identifier type for Author entities.
 **/
export type AuthorId = { id: Id };
/**
 * Identifier type for File entities.
 **/
export type FileId = { id: Id };
/**
 * Identifier type for Folder entities.
 **/
export type FolderId = { id: Id };
/**
 * Identifier type for Benchmark entities.
 **/
export type BenchmarkId = { id: Id };
/**
 * Identifier type for Quality entities.
 **/
export type QualityId = { id: Id };
/**
 * Identifier type for Port entities.
 **/
export type PortId = { id: Id };
/**
 * Identifier type for Prop entities.
 **/
export type PropId = { id: Id };
/**
 * Identifier type for Representation entities.
 **/
export type RepresentationId = { id: Id };
/**
 * Identifier type for Connector entities.
 **/
export type ConnectorId = { id: Id };
/**
 * Identifier type for Type entities.
 **/
export type TypeId = { id: Id };
/**
 * Identifier type for Layer entities.
 **/
export type LayerId = { id: Id };
/**
 * Identifier type for Piece entities.
 **/
export type PieceId = { id: Id };
/**
 * Identifier type for Group entities.
 **/
export type GroupId = { id: Id };
/**
 * Identifier type for Connection entities.
 **/
export type ConnectionId = { id: Id };
/**
 * Identifier type for Stat entities.
 **/
export type StatId = { id: Id };
/**
 * Identifier type for Design entities.
 **/
export type DesignId = { id: Id };
/**
 * Identifier type for KitImpl entities.
 **/
export type KitId = { id: Id };
/**
 * Identifier type for Tag entities.
 **/
export type TagId = { id: Id };
/**
 * Identifier type for Concept entities.
 **/
export type ConceptId = { id: Id };
/**
 * Identifier type for Family entities.
 **/
export type FamilyId = { id: Id };

/**
 * Zod schema for validating Attribute identifiers.
 **/
export const AttributeIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Location identifiers.
 **/
export const LocationIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Author identifiers.
 **/
export const AuthorIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating File identifiers.
 **/
export const FileIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Folder identifiers.
 **/
export const FolderIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Benchmark identifiers.
 **/
export const BenchmarkIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Quality identifiers.
 **/
export const QualityIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Port identifiers.
 **/
export const PortIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Prop identifiers.
 **/
export const PropIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Representation identifiers.
 **/
export const RepresentationIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Connector identifiers.
 **/
export const ConnectorIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Type identifiers.
 **/
export const TypeIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Layer identifiers.
 **/
export const LayerIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Piece identifiers.
 **/
export const PieceIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Group identifiers.
 **/
export const GroupIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Connection identifiers.
 **/
export const ConnectionIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Stat identifiers.
 **/
export const StatIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Design identifiers.
 **/
export const DesignIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating KitImpl identifiers.
 **/
export const KitIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Tag identifiers.
 **/
export const TagIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Concept identifiers.
 **/
export const ConceptIdSchema = z.object({ id: z.string() });
/**
 * Zod schema for validating Family identifiers.
 **/
export const FamilyIdSchema = z.object({ id: z.string() });

/**
 * Factory for creating Attribute identifiers.
 **/
export const createAttributeId = (id: Id): AttributeId => ({ id });
/**
 * Factory for creating Location identifiers.
 **/
export const createLocationId = (id: Id): LocationId => ({ id });
/**
 * Factory for creating Author identifiers.
 **/
export const createAuthorId = (id: Id): AuthorId => ({ id });
/**
 * Factory for creating File identifiers.
 **/
export const createFileId = (id: Id): FileId => ({ id });
/**
 * Factory for creating Folder identifiers.
 **/
export const createFolderId = (id: Id): FolderId => ({ id });
/**
 * Factory for creating Benchmark identifiers.
 **/
export const createBenchmarkId = (id: Id): BenchmarkId => ({ id });
/**
 * Factory for creating Quality identifiers.
 **/
export const createQualityId = (id: Id): QualityId => ({ id });
/**
 * Factory for creating Port identifiers.
 **/
export const createPortId = (id: Id): PortId => ({ id });
/**
 * Factory for creating Prop identifiers.
 **/
export const createPropId = (id: Id): PropId => ({ id });
/**
 * Factory for creating Representation identifiers.
 **/
export const createRepresentationId = (id: Id): RepresentationId => ({ id });
/**
 * Factory for creating Connector identifiers.
 **/
export const createConnectorId = (id: Id): ConnectorId => ({ id });
/**
 * Factory for creating Type identifiers.
 **/
export const createTypeId = (id: Id): TypeId => ({ id });
/**
 * Factory for creating Layer identifiers.
 **/
export const createLayerId = (id: Id): LayerId => ({ id });
/**
 * Factory for creating Piece identifiers.
 **/
export const createPieceId = (id: Id): PieceId => ({ id });
/**
 * Factory for creating Group identifiers.
 **/
export const createGroupId = (id: Id): GroupId => ({ id });
/**
 * Factory for creating Connection identifiers.
 **/
export const createConnectionId = (id: Id): ConnectionId => ({ id });
/**
 * Factory for creating Stat identifiers.
 **/
export const createStatId = (id: Id): StatId => ({ id });
/**
 * Factory for creating Design identifiers.
 **/
export const createDesignId = (id: Id): DesignId => ({ id });
/**
 * Factory for creating KitImpl identifiers.
 **/
export const createKitId = (id: Id): KitId => ({ id });
/**
 * Factory for creating Tag identifiers.
 **/
export const createTagId = (id: Id): TagId => ({ id });
/**
 * Factory for creating Concept identifiers.
 **/
export const createConceptId = (id: Id): ConceptId => ({ id });
/**
 * Factory for creating Family identifiers.
 **/
export const createFamilyId = (id: Id): FamilyId => ({ id });

/**
 * Equality check for Attribute identifiers.
 **/
export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.id === b.id;
/**
 * Equality check for Location identifiers.
 **/
export const areSameLocationId = (a: LocationId, b: LocationId): boolean => a.id === b.id;
/**
 * Equality check for Author identifiers.
 **/
export const areSameAuthorId = (a: AuthorId, b: AuthorId): boolean => a.id === b.id;
/**
 * Equality check for File identifiers.
 **/
export const areSameFileId = (a: FileId, b: FileId): boolean => a.id === b.id;
/**
 * Equality check for Folder identifiers.
 **/
export const areSameFolderId = (a: FolderId, b: FolderId): boolean => a.id === b.id;
/**
 * Equality check for Benchmark identifiers.
 **/
export const areSameBenchmarkId = (a: BenchmarkId, b: BenchmarkId): boolean => a.id === b.id;
/**
 * Equality check for Quality identifiers.
 **/
export const areSameQualityId = (a: QualityId, b: QualityId): boolean => a.id === b.id;
/**
 * Equality check for Port identifiers.
 **/
export const areSamePortId = (a: PortId, b: PortId): boolean => a.id === b.id;
/**
 * Equality check for Prop identifiers.
 **/
export const areSamePropId = (a: PropId, b: PropId): boolean => a.id === b.id;
/**
 * Equality check for Representation identifiers.
 **/
export const areSameRepresentationId = (a: RepresentationId, b: RepresentationId): boolean => a.id === b.id;
/**
 * Equality check for Connector identifiers.
 **/
export const areSameConnectorId = (a: ConnectorId, b: ConnectorId): boolean => a.id === b.id;
/**
 * Equality check for Type identifiers.
 **/
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.id === b.id;
/**
 * Equality check for Layer identifiers.
 **/
export const areSameLayerId = (a: LayerId, b: LayerId): boolean => a.id === b.id;
/**
 * Equality check for Piece identifiers.
 **/
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.id === b.id;
/**
 * Equality check for Group identifiers.
 **/
export const areSameGroupId = (a: GroupId, b: GroupId): boolean => a.id === b.id;
/**
 * Equality check for Connection identifiers.
 **/
export const areSameConnectionId = (a: ConnectionId, b: ConnectionId): boolean => a.id === b.id;
/**
 * Equality check for Stat identifiers.
 **/
export const areSameStatId = (a: StatId, b: StatId): boolean => a.id === b.id;
/**
 * Equality check for Design identifiers.
 **/
export const areSameDesignId = (a: DesignId, b: DesignId): boolean => a.id === b.id;
/**
 * Equality check for KitImpl identifiers.
 **/
export const areSameKitId = (a: KitId, b: KitId): boolean => a.id === b.id;
/**
 * Equality check for Tag identifiers.
 **/
export const areSameTagId = (a: TagId, b: TagId): boolean => a.id === b.id;
/**
 * Equality check for Concept identifiers.
 **/
export const areSameConceptId = (a: ConceptId, b: ConceptId): boolean => a.id === b.id;
/**
 * Equality check for Family identifiers.
 **/
export const areSameFamilyId = (a: FamilyId, b: FamilyId): boolean => a.id === b.id;

/**
 * Extracts the ID from a Attribute identifier.
 **/
export const getAttributeId = (id: AttributeId): Id => id.id;
/**
 * Extracts the ID from a Location identifier.
 **/
export const getLocationId = (id: LocationId): Id => id.id;
/**
 * Extracts the ID from a Author identifier.
 **/
export const getAuthorId = (id: AuthorId): Id => id.id;
/**
 * Extracts the ID from a File identifier.
 **/
export const getFileId = (id: FileId): Id => id.id;
/**
 * Extracts the ID from a Folder identifier.
 **/
export const getFolderId = (id: FolderId): Id => id.id;
/**
 * Extracts the ID from a Benchmark identifier.
 **/
export const getBenchmarkId = (id: BenchmarkId): Id => id.id;
/**
 * Extracts the ID from a Quality identifier.
 **/
export const getQualityId = (id: QualityId): Id => id.id;
/**
 * Extracts the ID from a Port identifier.
 **/
export const getPortId = (id: PortId): Id => id.id;
/**
 * Extracts the ID from a Prop identifier.
 **/
export const getPropId = (id: PropId): Id => id.id;
/**
 * Extracts the ID from a Representation identifier.
 **/
export const getRepresentationId = (id: RepresentationId): Id => id.id;
/**
 * Extracts the ID from a Connector identifier.
 **/
export const getConnectorId = (id: ConnectorId): Id => id.id;
/**
 * Extracts the ID from a Type identifier.
 **/
export const getTypeId = (id: TypeId): Id => id.id;
/**
 * Extracts the ID from a Layer identifier.
 **/
export const getLayerId = (id: LayerId): Id => id.id;
/**
 * Extracts the ID from a Piece identifier.
 **/
export const getPieceId = (id: PieceId): Id => id.id;
/**
 * Extracts the ID from a Group identifier.
 **/
export const getGroupId = (id: GroupId): Id => id.id;
/**
 * Extracts the ID from a Connection identifier.
 **/
export const getConnectionId = (id: ConnectionId): Id => id.id;
/**
 * Extracts the ID from a Stat identifier.
 **/
export const getStatId = (id: StatId): Id => id.id;
/**
 * Extracts the ID from a Design identifier.
 **/
export const getDesignId = (id: DesignId): Id => id.id;
/**
 * Extracts the ID from a KitImpl identifier.
 **/
export const getKitId = (id: KitId): Id => id.id;
/**
 * Extracts the ID from a Tag identifier.
 **/
export const getTagId = (id: TagId): Id => id.id;
/**
 * Extracts the ID from a Concept identifier.
 **/
export const getConceptId = (id: ConceptId): Id => id.id;
/**
 * Extracts the ID from a Family identifier.
 **/
export const getFamilyId = (id: FamilyId): Id => id.id;

// #endregion ­ƒÉìEntity IDs

// #region ­ƒûÑ´©ÅWeak Entities

// #region ­ƒô║Coordinate
// Coordinate weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Coordinate validation.
 **/
export const CoordinateSchema = z.object({ u: z.number(), v: z.number() });
/**
 * Type alias for Coordinate.
 **/
export type CoordinatePlain = z.infer<typeof CoordinateSchema>;
export class Coordinate implements CoordinatePlain {
  u!: number;
  v!: number;
  constructor(plain: CoordinatePlain) {
    Object.assign(this, CoordinateSchema.parse(plain));
  }
  static from(plain: CoordinatePlain): Coordinate {
    return new Coordinate(plain);
  }
  toPlain(): CoordinatePlain {
    return CoordinateSchema.parse(this as unknown as CoordinatePlain);
  }
  /** ­ƒô║Serialize this coordinate for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒô║Deserialize a coordinate from transport JSON. */
  static deserialize(json: string): Coordinate {
    return new Coordinate(CoordinateSchema.parse(JSON.parse(json)));
  }
  /** ­ƒô║Compute a coordinate delta from this coordinate to another coordinate. */
  diffTo(after: Coordinate): CoordinateDiff {
    return { u: after.u - this.u, v: after.v - this.v };
  }
  /** ­ƒô║Build the reverse coordinate delta for an already-applied delta. */
  inverseDiff(appliedDiff: CoordinateDiff): CoordinateDiff {
    return { u: this.u - (appliedDiff.u ?? 0), v: this.v - (appliedDiff.v ?? 0) };
  }
  /** ­ƒô║Merge two coordinate deltas. */
  static mergeDiff(first: CoordinateDiff, second: CoordinateDiff): CoordinateDiff {
    return { u: (first.u ?? 0) + (second.u ?? 0), v: (first.v ?? 0) + (second.v ?? 0) };
  }
  /** ­ƒô║Apply a coordinate delta to this coordinate. */
  applyDiff(diff: CoordinateDiff): void {
    if (diff.u !== undefined) this.u += diff.u;
    if (diff.v !== undefined) this.v += diff.v;
  }
}
/**
 * Serializes Coordinate for transport.
 **/
export const serializeCoordinate = (coordinate: Coordinate): string => coordinate.serialize();
/**
 **/
export const deserializeCoordinate = (json: string): Coordinate => Coordinate.deserialize(json);

/**
 * Zod schema for Coordinate diff validation.
 **/
export const CoordinateDiffSchema = CoordinateSchema.partial();
/**
 * Diff type for tracking Coordinate changes.
 **/
export type CoordinateDiff = z.infer<typeof CoordinateDiffSchema>;
/**
 * Retrieves the CoordinateDiff value.
 **/
export const getCoordinateDiff = (before: Coordinate, after: Coordinate): CoordinateDiff => {
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseCoordinate changes.
 **/
export const inverseCoordinateDiff = (original: Coordinate, appliedDiff: CoordinateDiff): CoordinateDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeCoordinate changes.
 **/
export const mergeCoordinateDiff = (diff1: CoordinateDiff, diff2: CoordinateDiff): CoordinateDiff => {
  return Coordinate.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyCoordinate changes.
 **/
export const applyCoordinateDiff = (target: Coordinate, diff: CoordinateDiff): void => {
  target.applyDiff(diff);
};

// #endregion ­ƒô║Coordinate

// #region Ô×í´©ÅVec
// Vec weak entity types and schemas MUST be defined here.

/**
 * Zod schema for Vec validation.
 **/
export const VecSchema = z.object({ u: z.number(), v: z.number() });
/**
 * Type alias for Vec.
 **/
export type VecPlain = z.infer<typeof VecSchema>;
export class Vec implements VecPlain {
  u!: number;
  v!: number;
  constructor(plain: VecPlain) {
    Object.assign(this, VecSchema.parse(plain));
  }
  static from(plain: VecPlain): Vec {
    return new Vec(plain);
  }
  toPlain(): VecPlain {
    return VecSchema.parse(this as unknown as VecPlain);
  }
  /** Ô×í´©ÅSerialize this vector coordinate for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** Ô×í´©ÅDeserialize a vector coordinate from transport JSON. */
  static deserialize(json: string): Vec {
    return new Vec(VecSchema.parse(JSON.parse(json)));
  }
  /** Ô×í´©ÅCompute a vector-coordinate delta. */
  diffTo(after: Vec): VecDiff {
    return { u: after.u - this.u, v: after.v - this.v };
  }
  /** Ô×í´©ÅBuild the reverse vector-coordinate delta for an already-applied delta. */
  inverseDiff(appliedDiff: VecDiff): VecDiff {
    return { u: this.u - (appliedDiff.u ?? 0), v: this.v - (appliedDiff.v ?? 0) };
  }
  /** Ô×í´©ÅMerge two vector-coordinate deltas. */
  static mergeDiff(first: VecDiff, second: VecDiff): VecDiff {
    return { u: (first.u ?? 0) + (second.u ?? 0), v: (first.v ?? 0) + (second.v ?? 0) };
  }
  /** Ô×í´©ÅApply a vector-coordinate delta to this vector coordinate. */
  applyDiff(diff: VecDiff): void {
    if (diff.u !== undefined) this.u += diff.u;
    if (diff.v !== undefined) this.v += diff.v;
  }
}
/**
 * Serializes Vec for transport.
 **/
export const serializeVec = (vec: Vec): string => vec.serialize();
/**
 **/
export const deserializeVec = (json: string): Vec => Vec.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseVec changes.
 **/
export const inverseVecDiff = (original: Vec, appliedDiff: VecDiff): VecDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeVec changes.
 **/
export const mergeVecDiff = (diff1: VecDiff, diff2: VecDiff): VecDiff => {
  return Vec.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyVec changes.
 **/
export const applyVecDiff = (target: Vec, diff: VecDiff): void => {
  target.applyDiff(diff);
};

// #endregion Ô×í´©ÅVec

// #region Ô£û´©ÅPoint
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
export type PointPlain = z.infer<typeof PointSchema>;
export class Point implements PointPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: PointPlain) {
    Object.assign(this, PointSchema.parse(plain));
  }
  static from(plain: PointPlain): Point {
    return new Point(plain);
  }
  toPlain(): PointPlain {
    return PointSchema.parse(this as unknown as PointPlain);
  }
  /** Ô£û´©ÅSerialize this point for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** Ô£û´©ÅDeserialize a point from transport JSON. */
  static deserialize(json: string): Point {
    return new Point(PointSchema.parse(JSON.parse(json)));
  }
  /** Ô£û´©ÅCompute a point delta from this point to another point. */
  diffTo(after: Point): PointDiff {
    return { x: after.x - this.x, y: after.y - this.y, z: after.z - this.z };
  }
  /** Ô£û´©ÅBuild the reverse point delta for an already-applied delta. */
  inverseDiff(appliedDiff: PointDiff): PointDiff {
    return { x: -(appliedDiff.x ?? 0), y: -(appliedDiff.y ?? 0), z: -(appliedDiff.z ?? 0) };
  }
  /** Ô£û´©ÅMerge two point deltas. */
  static mergeDiff(first: PointDiff, second: PointDiff): PointDiff {
    return { x: (first.x ?? 0) + (second.x ?? 0), y: (first.y ?? 0) + (second.y ?? 0), z: (first.z ?? 0) + (second.z ?? 0) };
  }
  /** Ô£û´©ÅApply a point delta to this point. */
  applyDiff(diff: PointDiff): void {
    if (diff.x !== undefined) this.x += diff.x;
    if (diff.y !== undefined) this.y += diff.y;
    if (diff.z !== undefined) this.z += diff.z;
  }
}
/**
 * Serializes Point for transport.
 **/
export const serializePoint = (point: Point): string => point.serialize();
/**
 **/
export const deserializePoint = (json: string): Point => Point.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inversePoint changes.
 **/
export const inversePointDiff = (original: Point, appliedDiff: PointDiff): PointDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergePoint changes.
 **/
export const mergePointDiff = (diff1: PointDiff, diff2: PointDiff): PointDiff => {
  return Point.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyPoint changes.
 **/
export const applyPointDiff = (target: Point, diff: PointDiff): void => {
  target.applyDiff(diff);
};

// #endregion Ô£û´©ÅPoint

// #region Ôåù´©ÅVector
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
export type VectorPlain = z.infer<typeof VectorSchema>;
export class Vector implements VectorPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: VectorPlain) {
    Object.assign(this, VectorSchema.parse(plain));
  }
  static from(plain: VectorPlain): Vector {
    return new Vector(plain);
  }
  toPlain(): VectorPlain {
    return VectorSchema.parse(this as unknown as VectorPlain);
  }
  /** Ôåù´©ÅSerialize this vector for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** Ôåù´©ÅDeserialize a vector from transport JSON. */
  static deserialize(json: string): Vector {
    return new Vector(VectorSchema.parse(JSON.parse(json)));
  }
  /** Ôåù´©ÅCompute a vector delta from this vector to another vector. */
  diffTo(after: Vector): VectorDiff {
    return { x: after.x - this.x, y: after.y - this.y, z: after.z - this.z };
  }
  /** Ôåù´©ÅBuild the reverse vector delta for an already-applied delta. */
  inverseDiff(appliedDiff: VectorDiff): VectorDiff {
    return { x: -(appliedDiff.x ?? 0), y: -(appliedDiff.y ?? 0), z: -(appliedDiff.z ?? 0) };
  }
  /** Ôåù´©ÅMerge two vector deltas. */
  static mergeDiff(first: VectorDiff, second: VectorDiff): VectorDiff {
    return { x: (first.x ?? 0) + (second.x ?? 0), y: (first.y ?? 0) + (second.y ?? 0), z: (first.z ?? 0) + (second.z ?? 0) };
  }
  /** Ôåù´©ÅApply a vector delta to this vector. */
  applyDiff(diff: VectorDiff): void {
    if (diff.x !== undefined) this.x += diff.x;
    if (diff.y !== undefined) this.y += diff.y;
    if (diff.z !== undefined) this.z += diff.z;
  }
}
/**
 * Serializes Vector for transport.
 **/
export const serializeVector = (vector: Vector): string => vector.serialize();
/**
 **/
export const deserializeVector = (json: string): Vector => Vector.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseVector changes.
 **/
export const inverseVectorDiff = (original: Vector, appliedDiff: VectorDiff): VectorDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeVector changes.
 **/
export const mergeVectorDiff = (diff1: VectorDiff, diff2: VectorDiff): VectorDiff => {
  return Vector.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyVector changes.
 **/
export const applyVectorDiff = (target: Vector, diff: VectorDiff): void => {
  target.applyDiff(diff);
};

// #endregion Ôåù´©ÅVector

// #region Ôù╗´©ÅPlane
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
export type PlanePlain = z.infer<typeof PlaneSchema>;
export class Plane implements PlanePlain {
  origin!: Point;
  xAxis!: Vector;
  yAxis!: Vector;
  constructor(plain: PlanePlain) {
    const p = PlaneSchema.parse(plain);
    this.origin = new Point(p.origin);
    this.xAxis = new Vector(p.xAxis);
    this.yAxis = new Vector(p.yAxis);
  }
  static from(plain: PlanePlain): Plane {
    return new Plane(plain);
  }
  toPlain(): PlanePlain {
    return PlaneSchema.parse(this as unknown as PlanePlain);
  }
  /** Ôù╗´©ÅSerialize this plane for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** Ôù╗´©ÅDeserialize a plane from transport JSON. */
  static deserialize(json: string): Plane {
    return new Plane(PlaneSchema.parse(JSON.parse(json)));
  }
  /** Ôù╗´©ÅConvert this plane to a three.js matrix. */
  toMatrix(): THREE.Matrix4 {
    const origin = new THREE.Vector3(this.origin.x, this.origin.y, this.origin.z);
    const xAxis = new THREE.Vector3(this.xAxis.x, this.xAxis.y, this.xAxis.z);
    const yAxis = new THREE.Vector3(this.yAxis.x, this.yAxis.y, this.yAxis.z);
    const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis).normalize();
    const orthoYAxis = new THREE.Vector3().crossVectors(zAxis, xAxis).normalize();
    return new THREE.Matrix4().makeBasis(xAxis.normalize(), orthoYAxis, zAxis).setPosition(origin);
  }
  /** Ôù╗´©ÅCreate a plane from a three.js matrix. */
  static fromMatrix(matrix: THREE.Matrix4): Plane {
    const origin = new THREE.Vector3();
    const xAxis = new THREE.Vector3();
    const yAxis = new THREE.Vector3();
    const zAxis = new THREE.Vector3();
    matrix.decompose(origin, new THREE.Quaternion(), new THREE.Vector3());
    matrix.extractBasis(xAxis, yAxis, zAxis);
    return new Plane({
      origin: { x: origin.x, y: origin.y, z: origin.z },
      xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
      yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
    });
  }
  /** Ôù╗´©ÅAverage this plane with other planes while preserving the first orientation. */
  averageWith(planes: Plane[]): Plane {
    return Plane.average([this, ...planes]) ?? this;
  }
  /** Ôù╗´©ÅAverage a plane collection. */
  static average(planes: Plane[]): Plane | null {
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
    return new Plane({ origin: avgOrigin, xAxis: planes[0].xAxis, yAxis: planes[0].yAxis });
  }
  /** Ôù╗´©ÅRound plane components to tolerance. */
  rounded(): Plane {
    return new Plane({
      origin: { x: round(this.origin.x), y: round(this.origin.y), z: round(this.origin.z) },
      xAxis: { x: round(this.xAxis.x), y: round(this.xAxis.y), z: round(this.xAxis.z) },
      yAxis: { x: round(this.yAxis.x), y: round(this.yAxis.y), z: round(this.yAxis.z) },
    });
  }
  /** Ôù╗´©ÅCompute a plane delta from this plane to another plane. */
  diffTo(after: Plane): PlaneDiff {
    return { origin: this.origin.diffTo(after.origin), xAxis: this.xAxis.diffTo(after.xAxis), yAxis: this.yAxis.diffTo(after.yAxis) };
  }
  /** Ôù╗´©ÅBuild the reverse plane delta for an already-applied delta. */
  inverseDiff(appliedDiff: PlaneDiff): PlaneDiff {
    return {
      origin: this.origin.inverseDiff(appliedDiff.origin ?? { x: 0, y: 0, z: 0 }),
      xAxis: this.xAxis.inverseDiff(appliedDiff.xAxis ?? { x: 0, y: 0, z: 0 }),
      yAxis: this.yAxis.inverseDiff(appliedDiff.yAxis ?? { x: 0, y: 0, z: 0 }),
    };
  }
  /** Ôù╗´©ÅMerge two plane deltas. */
  static mergeDiff(first: PlaneDiff, second: PlaneDiff): PlaneDiff {
    return {
      origin: first.origin && second.origin ? Point.mergeDiff(first.origin, second.origin) : (second.origin ?? first.origin),
      xAxis: first.xAxis && second.xAxis ? Vector.mergeDiff(first.xAxis, second.xAxis) : (second.xAxis ?? first.xAxis),
      yAxis: first.yAxis && second.yAxis ? Vector.mergeDiff(first.yAxis, second.yAxis) : (second.yAxis ?? first.yAxis),
    };
  }
  /** Ôù╗´©ÅApply a plane delta to this plane. */
  applyDiff(diff: PlaneDiff): void {
    if (diff.origin) this.origin.applyDiff(diff.origin);
    if (diff.xAxis) this.xAxis.applyDiff(diff.xAxis);
    if (diff.yAxis) this.yAxis.applyDiff(diff.yAxis);
  }
}
/**
 * Serializes Plane for transport.
 **/
export const serializePlane = (plane: Plane): string => plane.serialize();
/**
 **/
export const deserializePlane = (json: string): Plane => Plane.deserialize(json);
/**
 **/
export const planeToMatrix = (plane: Plane): THREE.Matrix4 => {
  return plane.toMatrix();
};
/**
 **/
export const matrixToPlane = (matrix: THREE.Matrix4): Plane => {
  return Plane.fromMatrix(matrix);
};

/**
 **/
export const averagePlane = (planes: Plane[]): Plane | null => {
  return Plane.average(planes);
};
// Ôù╗´©ÅroundPlane rounds plane components to a specified number of decimal places.
const roundPlane = (plane: Plane): Plane => plane.rounded();

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inversePlane changes.
 **/
export const inversePlaneDiff = (original: Plane, appliedDiff: PlaneDiff): PlaneDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergePlane changes.
 **/
export const mergePlaneDiff = (diff1: PlaneDiff, diff2: PlaneDiff): PlaneDiff => {
  return Plane.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyPlane changes.
 **/
export const applyPlaneDiff = (target: Plane, diff: PlaneDiff): void => {
  target.applyDiff(diff);
};

// #endregion Ôù╗´©ÅPlane

// #region ­ƒÄÑCamera

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
export type CameraPlain = z.infer<typeof CameraSchema>;
export class Camera implements CameraPlain {
  position!: Point;
  forward!: Vector;
  up!: Vector;
  constructor(plain: CameraPlain) {
    const p = CameraSchema.parse(plain);
    this.position = new Point(p.position);
    this.forward = new Vector(p.forward);
    this.up = new Vector(p.up);
  }
  static from(plain: CameraPlain): Camera {
    return new Camera(plain);
  }
  /** ­ƒôªSerialize this camera for wire transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒº¡Deserialize a wire camera into a stateful instance. */
  static deserialize(json: string): Camera {
    return new Camera(CameraSchema.parse(JSON.parse(json)));
  }
  toPlain(): CameraPlain {
    return CameraSchema.parse(this as unknown as CameraPlain);
  }
  /** ­ƒº«Compute the additive camera delta from this camera to another camera. */
  diffTo(after: Camera): CameraDiff {
    return {
      position: this.position.diffTo(after.position),
      forward: this.forward.diffTo(after.forward),
      up: this.up.diffTo(after.up),
    };
  }
  /** Ôå®´©ÅCompute the inverse additive camera delta for an already-applied delta. */
  inverseDiff(appliedDiff: CameraDiff): CameraDiff {
    return {
      position: appliedDiff.position ? this.position.inverseDiff(appliedDiff.position) : this.position,
      forward: appliedDiff.forward ? this.forward.inverseDiff(appliedDiff.forward) : this.forward,
      up: appliedDiff.up ? this.up.inverseDiff(appliedDiff.up) : this.up,
    };
  }
  /** ­ƒº¼Merge two additive camera deltas. */
  static mergeDiff(first: CameraDiff, second: CameraDiff): CameraDiff {
    return {
      position: first.position && second.position ? Point.mergeDiff(first.position, second.position) : (second.position ?? first.position),
      forward: first.forward && second.forward ? Vector.mergeDiff(first.forward, second.forward) : (second.forward ?? first.forward),
      up: first.up && second.up ? Vector.mergeDiff(first.up, second.up) : (second.up ?? first.up),
    };
  }
  /** Ô£ì´©ÅApply an additive camera delta in place. */
  applyDiff(diff: CameraDiff): void {
    if (diff.position) this.position.applyDiff(diff.position);
    if (diff.forward) this.forward.applyDiff(diff.forward);
    if (diff.up) this.up.applyDiff(diff.up);
  }
}
/**
 * Serializes Camera for transport.
 **/
export const serializeCamera = (camera: Camera): string => camera.serialize();
/**
 **/
export const deserializeCamera = (json: string): Camera => Camera.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseCamera changes.
 **/
export const inverseCameraDiff = (original: Camera, appliedDiff: CameraDiff): CameraDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeCamera changes.
 **/
export const mergeCameraDiff = (diff1: CameraDiff, diff2: CameraDiff): CameraDiff => {
  return Camera.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyCamera changes.
 *
 **/
export const applyCameraDiff = (target: Camera, diff: CameraDiff): void => {
  target.applyDiff(diff);
};

// #endregion ­ƒÄÑCamera

// #endregion ­ƒûÑ´©ÅWeak Entities

// #region ­ƒÆÄAttribute
// Attribute entity types, schemas, and helper functions MUST be defined here.
// ­ƒôàDateProperty represents a date-time value as ISO string.
const DateProperty = () => z.string().optional();

/**
 * Zod schema for Attribute validation.
 **/
export const AttributeSchema = z.object({
  id: z.string(),
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
/**
 * Type alias for Attribute.
 **/
export type AttributePlain = z.infer<typeof AttributeSchema>;
export class Attribute implements AttributePlain {
  id!: string;
  key!: string;
  value?: string;
  definition?: string;
  constructor(plain: AttributePlain) {
    Object.assign(this, AttributeSchema.parse(plain));
  }
  static from(plain: AttributePlain): Attribute {
    return new Attribute(plain);
  }
  toPlain(): AttributePlain {
    return AttributeSchema.parse(this as unknown as AttributePlain);
  }
  /** ­ƒôªSerialize this attribute for wire transport. */
  toJson(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒº¡Deserialize a wire attribute into a stateful instance. */
  static fromJson(json: string): Attribute {
    return new Attribute(AttributeSchema.parse(JSON.parse(json)));
  }
  /** ­ƒº«Compute the replacement-style attribute delta from this attribute to another attribute. */
  diffTo(after: Attribute): AttributeDiff {
    const diff: AttributeDiff = {};
    if (this.key !== after.key) diff.key = after.key;
    if (this.value !== after.value) diff.value = after.value;
    if (this.definition !== after.definition) diff.definition = after.definition;
    return diff;
  }
  /** Ôå®´©ÅCompute the inverse replacement-style attribute delta for an already-applied delta. */
  inverseDiff(appliedDiff: AttributeDiff): AttributeDiff {
    return {
      key: appliedDiff.key ? this.key : "",
      value: appliedDiff.value ? this.value : "",
      definition: appliedDiff.definition ? this.definition : "",
    };
  }
  /** ­ƒº¼Merge two replacement-style attribute deltas. */
  static mergeDiff(first: AttributeDiff, second: AttributeDiff): AttributeDiff {
    return {
      key: second.key ?? first.key,
      value: second.value ?? first.value,
      definition: second.definition ?? first.definition,
    };
  }
  /** Ô£ì´©ÅApply a replacement-style attribute delta in place. */
  applyDiff(diff: AttributeDiff): void {
    if (diff.key !== undefined) this.key = diff.key;
    if (diff.value !== undefined) this.value = diff.value;
    if (diff.definition !== undefined) this.definition = diff.definition;
  }
}
/**
 * Serializes Attribute for transport.
 **/
export const serializeAttribute = (attribute: Attribute): string => attribute.toJson();
/**
 **/
export const deserializeAttribute = (json: string): Attribute => Attribute.fromJson(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseAttribute changes.
 **/
export const inverseAttributeDiff = (original: Attribute, appliedDiff: AttributeDiff): AttributeDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeAttribute changes.
 **/
export const mergeAttributeDiff = (diff1: AttributeDiff, diff2: AttributeDiff): AttributeDiff => {
  return Attribute.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyAttribute changes.
 **/
export const applyAttributeDiff = (target: Attribute, diff: AttributeDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Attributes diff validation.
 **/
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Attributes changes.
 **/
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;

// ­ƒÆÄgetAttributesDiff computes the diff between two attribute collections.
const getAttributesDiff = (before: Attribute[], after: Attribute[]): AttributesDiff => {
  const beforeIds = new Set(before.map((a) => a.id));
  const afterIds = new Set(after.map((a) => a.id));
  const removed = before.filter((a) => !afterIds.has(a.id)).map((a) => ({ id: a.id }));
  const added = after.filter((a) => !beforeIds.has(a.id));
  const updated = after
    .filter((a) => beforeIds.has(a.id))
    .map((a) => ({ attribute: { id: a.id }, diff: getAttributeDiff(before.find((b) => b.id === a.id)!, a) }))
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
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  const updatedIds = appliedDiff.updated?.map((a) => a.attribute.id) ?? [];
  const addedIds = appliedDiff.added?.map((a) => a.id) ?? [];
  return {
    removed: addedIds.map((id) => ({ id })),
    updated: updatedIds
      .map((id) => {
        const orig = original.find((a) => a.id === id);
        const upd = appliedDiff.updated?.find((a) => a.attribute.id === id);
        if (!orig || !upd) return null;
        return { attribute: { id }, diff: inverseAttributeDiff(orig, upd.diff) };
      })
      .filter((item): item is { attribute: AttributeId; diff: AttributeDiff } => item !== null),
    added: removedIds.map((id) => original.find((a) => a.id === id)!).filter((a) => a !== undefined),
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
export const applyAttributesDiff = (items: Attribute[], diff: AttributesDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const item = items.find((attr) => attr.id === update.attribute.id);
      if (item) applyAttributeDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map((a) => new Attribute(a as AttributePlain)));
  }
};

// #endregion ­ƒÆÄAttribute

// #region ­ƒôìLocation
// Location entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Location validation.
 **/
export const LocationSchema = z.object({
  id: z.string(),
  longitude: z.number(),
  latitude: z.number(),
  altitude: z.number().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Location.
 **/
export type LocationPlain = z.infer<typeof LocationSchema>;
export class Location implements LocationPlain {
  id!: string;
  longitude!: number;
  latitude!: number;
  altitude?: number;
  attributes?: Attribute[];
  constructor(plain: LocationPlain) {
    const p = LocationSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: LocationPlain): Location {
    return new Location(plain);
  }
  toPlain(): LocationPlain {
    return LocationSchema.parse(this as unknown as LocationPlain);
  }
  /** ­ƒôìSerialize this location for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒôìDeserialize a location from transport JSON. */
  static deserialize(json: string): Location {
    return new Location(LocationSchema.parse(JSON.parse(json)));
  }
  /** ­ƒôìCompute a location delta from this location to another location. */
  diffTo(after: Location): LocationDiff {
    const diff: LocationDiff = {};
    if (this.longitude !== after.longitude) diff.longitude = after.longitude - this.longitude;
    if (this.latitude !== after.latitude) diff.latitude = after.latitude - this.latitude;
    if (this.altitude !== after.altitude) diff.altitude = after.altitude !== undefined && this.altitude !== undefined ? after.altitude - this.altitude : after.altitude;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒôìBuild the reverse location delta for an already-applied delta. */
  inverseDiff(appliedDiff: LocationDiff): LocationDiff {
    const inverse: LocationDiff = {};
    if (appliedDiff.longitude !== undefined) inverse.longitude = this.longitude;
    if (appliedDiff.latitude !== undefined) inverse.latitude = this.latitude;
    if (appliedDiff.altitude !== undefined) inverse.altitude = this.altitude;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒôìMerge two location deltas. */
  static mergeDiff(first: LocationDiff, second: LocationDiff): LocationDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒôìApply a location delta to this location. */
  applyDiff(diff: LocationDiff): void {
    if (diff.longitude !== undefined) this.longitude = diff.longitude;
    if (diff.latitude !== undefined) this.latitude = diff.latitude;
    if (diff.altitude !== undefined) this.altitude = diff.altitude;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Location for transport.
 **/
export const serializeLocation = (location: Location): string => location.serialize();
/**
 **/
export const deserializeLocation = (json: string): Location => Location.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseLocation changes.
 **/
export const inverseLocationDiff = (original: Location, appliedDiff: LocationDiff): LocationDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeLocation changes.
 **/
export const mergeLocationDiff = (diff1: LocationDiff, diff2: LocationDiff): LocationDiff => {
  return Location.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyLocation changes.
 **/
export const applyLocationDiff = (target: Location, diff: LocationDiff): void => {
  target.applyDiff(diff);
};

// #endregion ­ƒôìLocation

// #region Ô£ì´©ÅAuthor
// Author entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Author validation.
 **/
export const AuthorSchema = z.object({ id: z.string(), name: z.string(), email: z.string(), attributes: z.array(AttributeSchema).optional() });
/**
 * Type alias for Author.
 **/
export type AuthorPlain = z.infer<typeof AuthorSchema>;
export class Author implements AuthorPlain {
  id!: string;
  name!: string;
  email!: string;
  attributes?: Attribute[];
  constructor(plain: AuthorPlain) {
    const p = AuthorSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: AuthorPlain): Author {
    return new Author(plain);
  }
  toPlain(): AuthorPlain {
    return AuthorSchema.parse(this as unknown as AuthorPlain);
  }
  /** Ô£ì´©ÅSerialize this author for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** Ô£ì´©ÅDeserialize an author from transport JSON. */
  static deserialize(json: string): Author {
    return new Author(AuthorSchema.parse(JSON.parse(json)));
  }
  /** Ô£ì´©ÅCompute an author delta from this author to another author. */
  diffTo(after: Author): AuthorDiff {
    const diff: AuthorDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.email !== after.email) diff.email = after.email;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** Ô£ì´©ÅBuild the reverse author delta for an already-applied delta. */
  inverseDiff(appliedDiff: AuthorDiff): AuthorDiff {
    const inverse: AuthorDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.email !== undefined) inverse.email = this.email;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** Ô£ì´©ÅMerge two author deltas. */
  static mergeDiff(first: AuthorDiff, second: AuthorDiff): AuthorDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** Ô£ì´©ÅApply an author delta to this author. */
  applyDiff(diff: AuthorDiff): void {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.email !== undefined) this.email = diff.email;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Author for transport.
 **/
export const serializeAuthor = (author: Author): string => author.serialize();
/**
 **/
export const deserializeAuthor = (json: string): Author => Author.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseAuthor changes.
 **/
export const inverseAuthorDiff = (original: Author, appliedDiff: AuthorDiff): AuthorDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeAuthor changes.
 **/
export const mergeAuthorDiff = (diff1: AuthorDiff, diff2: AuthorDiff): AuthorDiff => {
  return Author.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyAuthor changes.
 **/
export const applyAuthorDiff = (target: Author, diff: AuthorDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Authors diff validation.
 **/
export const AuthorsDiffSchema = z.object({
  removed: z.array(AuthorIdSchema).optional(),
  updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Authors changes.
 **/
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;

// #endregion Ô£ì´©ÅAuthor

// #region ­ƒôäFile
// File entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for File validation.
 **/
export const FileSchema = z.object({
  id: z.string(),
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
export type FilePlain = z.infer<typeof FileSchema>;
export class File implements FilePlain {
  id!: string;
  name!: string;
  description?: string;
  remote?: string;
  folder?: FolderId;
  size?: number;
  hash?: string;
  blob?: string;
  createdAt?: string;
  createdBy?: string;
  updatedAt?: string;
  updatedBy?: string;
  constructor(plain: FilePlain) {
    Object.assign(this, FileSchema.parse(plain));
  }
  static from(plain: FilePlain): File {
    return new File(plain);
  }
  toPlain(): FilePlain {
    return FileSchema.parse(this as unknown as FilePlain);
  }
  /** ­ƒôäSerialize this file for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒôäDeserialize a file from transport JSON. */
  static deserialize(json: string): File {
    return new File(FileSchema.parse(JSON.parse(json)));
  }
  /** ­ƒôäCompute a file delta from this file to another file. */
  diffTo(after: File): FileDiff {
    const diff: FileDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description;
    if (this.remote !== after.remote) diff.remote = after.remote;
    if (this.size !== after.size) diff.size = after.size;
    if (this.hash !== after.hash) diff.hash = after.hash;
    if (this.blob !== after.blob) diff.blob = after.blob;
    if (this.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
    if (this.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
    if (this.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
    if (this.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
    if (this.folder?.id !== after.folder?.id) diff.folder = after.folder;
    return diff;
  }
  /** ­ƒôäBuild the reverse file delta for an already-applied delta. */
  inverseDiff(appliedDiff: FileDiff): FileDiff {
    const inverse: FileDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.remote !== undefined) inverse.remote = this.remote;
    if (appliedDiff.size !== undefined) inverse.size = this.size;
    if (appliedDiff.hash !== undefined) inverse.hash = this.hash;
    if (appliedDiff.blob !== undefined) inverse.blob = this.blob;
    if (appliedDiff.createdAt !== undefined) inverse.createdAt = this.createdAt;
    if (appliedDiff.createdBy !== undefined) inverse.createdBy = this.createdBy;
    if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = this.updatedAt;
    if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = this.updatedBy;
    if (appliedDiff.folder !== undefined) inverse.folder = this.folder;
    return inverse;
  }
  /** ­ƒôäMerge two file deltas. */
  static mergeDiff(first: FileDiff, second: FileDiff): FileDiff {
    return { ...first, ...second };
  }
  /** ­ƒôäApply a file delta to this file. */
  applyDiff(diff: FileDiff): void {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.remote !== undefined) this.remote = diff.remote;
    if (diff.size !== undefined) this.size = diff.size;
    if (diff.hash !== undefined) this.hash = diff.hash;
    if (diff.createdAt !== undefined) this.createdAt = diff.createdAt;
    if (diff.createdBy !== undefined) this.createdBy = diff.createdBy;
    if (diff.updatedAt !== undefined) this.updatedAt = diff.updatedAt;
    if (diff.updatedBy !== undefined) this.updatedBy = diff.updatedBy;
    if (diff.folder !== undefined) this.folder = diff.folder;
    if (diff.blob !== undefined) this.blob = diff.blob;
  }
}
/**
 * Serializes File for transport.
 **/
export const serializeFile = (file: File): string => file.serialize();
/**
 **/
export const deserializeFile = (json: string): File => File.deserialize(json);

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
  const b = before instanceof File ? before : new File(before);
  return b.diffTo(after);
};
/**
 * Diff type for tracking inverseFile changes.
 **/
export const inverseFileDiff = (original: File, appliedDiff: FileDiff): FileDiff => {
  const o = original instanceof File ? original : new File(original);
  return o.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeFile changes.
 **/
export const mergeFileDiff = (diff1: FileDiff, diff2: FileDiff): FileDiff => {
  return File.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyFile changes.
 **/
export const applyFileDiff = (target: File, diff: FileDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Files diff validation.
 **/
export const FilesDiffSchema = z.object({
  removed: z.array(FileIdSchema).optional(),
  updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Files changes.
 **/
export type FilesDiff = z.infer<typeof FilesDiffSchema>;

// #endregion ­ƒôäFile

// #region ­ƒôüFolder
// Folder entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Folder validation.
 **/
export const FolderSchema = z.object({
  id: z.string(),
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
export type FolderPlain = z.infer<typeof FolderSchema>;
export class Folder implements FolderPlain {
  id!: string;
  name!: string;
  parent?: FolderId;
  description?: string;
  attributes?: Attribute[];
  createdAt?: string;
  createdBy?: string;
  updatedAt?: string;
  updatedBy?: string;
  constructor(plain: FolderPlain) {
    const p = FolderSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: FolderPlain): Folder {
    return new Folder(plain);
  }
  toPlain(): FolderPlain {
    return FolderSchema.parse(this as unknown as FolderPlain);
  }
  /** ­ƒôüSerialize this folder for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒôüDeserialize a folder from transport JSON. */
  static deserialize(json: string): Folder {
    return new Folder(FolderSchema.parse(JSON.parse(json)));
  }
  /** ­ƒôüCompute a folder delta from this folder to another folder. */
  diffTo(after: Folder): FolderDiff {
    const diff: FolderDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.parent?.id !== after.parent?.id) diff.parent = after.parent;
    if (this.description !== after.description) diff.description = after.description;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    if (this.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
    if (this.createdBy !== after.createdBy) diff.createdBy = after.createdBy;
    if (this.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
    if (this.updatedBy !== after.updatedBy) diff.updatedBy = after.updatedBy;
    return diff;
  }
  /** ­ƒôüBuild the reverse folder delta for an already-applied delta. */
  inverseDiff(appliedDiff: FolderDiff): FolderDiff {
    const inverse: FolderDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.parent !== undefined) inverse.parent = this.parent;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    if (appliedDiff.createdAt !== undefined) inverse.createdAt = this.createdAt;
    if (appliedDiff.createdBy !== undefined) inverse.createdBy = this.createdBy;
    if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = this.updatedAt;
    if (appliedDiff.updatedBy !== undefined) inverse.updatedBy = this.updatedBy;
    return inverse;
  }
  /** ­ƒôüMerge two folder deltas. */
  static mergeDiff(first: FolderDiff, second: FolderDiff): FolderDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒôüApply a folder delta to this folder. */
  applyDiff(diff: FolderDiff): void {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.parent !== undefined) this.parent = diff.parent;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.createdAt !== undefined) this.createdAt = diff.createdAt;
    if (diff.createdBy !== undefined) this.createdBy = diff.createdBy;
    if (diff.updatedAt !== undefined) this.updatedAt = diff.updatedAt;
    if (diff.updatedBy !== undefined) this.updatedBy = diff.updatedBy;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Folder for transport.
 **/
export const serializeFolder = (folder: Folder): string => folder.serialize();
/**
 **/
export const deserializeFolder = (json: string): Folder => Folder.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseFolder changes.
 **/
export const inverseFolderDiff = (original: Folder, appliedDiff: FolderDiff): FolderDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeFolder changes.
 **/
export const mergeFolderDiff = (diff1: FolderDiff, diff2: FolderDiff): FolderDiff => {
  return Folder.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyFolder changes.
 **/
export const applyFolderDiff = (target: Folder, diff: FolderDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Folders diff validation.
 **/
export const FoldersDiffSchema = z.object({
  removed: z.array(FolderIdSchema).optional(),
  updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Folders changes.
 **/
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;

// #endregion ­ƒôüFolder


// #region ­ƒôÅBenchmark
// Benchmark entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Benchmark validation.
 **/
export const BenchmarkSchema = z.object({
  id: z.string(),
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
export type BenchmarkPlain = z.infer<typeof BenchmarkSchema>;
export class Benchmark implements BenchmarkPlain {
  id!: string;
  name!: string;
  icon?: string;
  min?: number;
  minExcluded?: boolean;
  max?: number;
  maxExcluded?: boolean;
  attributes?: Attribute[];
  constructor(plain: BenchmarkPlain) {
    const p = BenchmarkSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: BenchmarkPlain): Benchmark {
    return new Benchmark(plain);
  }
  toPlain(): BenchmarkPlain {
    return BenchmarkSchema.parse(this as unknown as BenchmarkPlain);
  }
  diffTo(other: Benchmark): BenchmarkDiff {
    const before = this;
    const after = other;
    const diff: BenchmarkDiff = {};
    if (before.name !== after.name) diff.name = after.name;
    if (before.icon !== after.icon) diff.icon = after.icon;
    if (before.min !== after.min) diff.min = after.min !== undefined && before.min !== undefined ? after.min - before.min : after.min;
    if (before.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
    if (before.max !== after.max) diff.max = after.max !== undefined && before.max !== undefined ? after.max - before.max : after.max;
    if (before.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
    if (!deepEqual(before.attributes, after.attributes)) diff.attributes = getAttributesDiff(before.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  inverseDiff(appliedDiff: BenchmarkDiff): BenchmarkDiff {
    const original = this;
    const inverse: BenchmarkDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = original.name;
    if (appliedDiff.icon !== undefined) inverse.icon = original.icon;
    if (appliedDiff.min !== undefined) inverse.min = original.min;
    if (appliedDiff.minExcluded !== undefined) inverse.minExcluded = original.minExcluded;
    if (appliedDiff.max !== undefined) inverse.max = original.max;
    if (appliedDiff.maxExcluded !== undefined) inverse.maxExcluded = original.maxExcluded;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  static mergeDiff(diff1: BenchmarkDiff, diff2: BenchmarkDiff): BenchmarkDiff {
    return { ...diff1, ...diff2, attributes: diff1.attributes && diff2.attributes ? mergeAttributesDiff(diff1.attributes, diff2.attributes) : (diff2.attributes ?? diff1.attributes) };
  }
  applyDiff(diff: BenchmarkDiff): void {
    const target = this;
    if (diff.name !== undefined) target.name = diff.name;
    if (diff.icon !== undefined) target.icon = diff.icon;
    if (diff.min !== undefined) target.min = diff.min;
    if (diff.minExcluded !== undefined) target.minExcluded = diff.minExcluded;
    if (diff.max !== undefined) target.max = diff.max;
    if (diff.maxExcluded !== undefined) target.maxExcluded = diff.maxExcluded;
    if (diff.attributes) {
      if (!target.attributes) target.attributes = [];
      applyAttributesDiff(target.attributes, diff.attributes);
    }
  }
  toJson(): string {
    return JSON.stringify(this.toPlain());
  }
  static fromJson(json: string): Benchmark {
    return new Benchmark(BenchmarkSchema.parse(JSON.parse(json)));
  }
}
/**
 * Serializes Benchmark for transport.
 **/
export const serializeBenchmark = (benchmark: Benchmark): string => benchmark.toJson();
/**
 **/
export const deserializeBenchmark = (json: string): Benchmark => Benchmark.fromJson(json);

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
export const applyBenchmarkDiff = (target: Benchmark, diff: BenchmarkDiff): void => {
  target.applyDiff(diff);
};
/**
 * Retrieves the BenchmarkDiff value.
 **/
export const getBenchmarkDiff = (before: Benchmark, after: Benchmark): BenchmarkDiff => {
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseBenchmark changes.
 **/
export const inverseBenchmarkDiff = (original: Benchmark, appliedDiff: BenchmarkDiff): BenchmarkDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeBenchmark changes.
 **/
export const mergeBenchmarkDiff = (diff1: BenchmarkDiff, diff2: BenchmarkDiff): BenchmarkDiff => {
  return Benchmark.mergeDiff(diff1, diff2);
};

/**
 * Zod schema for Benchmarks diff validation.
 **/
export const BenchmarksDiffSchema = z.object({
  removed: z.array(BenchmarkIdSchema).optional(),
  updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Benchmarks changes.
 **/
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;
// ­ƒôÅgetBenchmarksDiff computes the diff between two benchmark collections.
const getBenchmarksDiff = (before: Benchmark[], after: Benchmark[]): BenchmarksDiff => {
  const beforeIds = new Set(before.map((b) => b.id));
  const afterIds = new Set(after.map((b) => b.id));
  const removed = before.filter((b) => !afterIds.has(b.id)).map((b) => ({ id: b.id }));
  const added = after.filter((b) => !beforeIds.has(b.id));
  const updated = after
    .filter((b) => beforeIds.has(b.id))
    .map((afterBenchmark) => {
      const beforeBenchmark = before.find((b) => b.id === afterBenchmark.id)!;
      const diff = getBenchmarkDiff(beforeBenchmark, afterBenchmark);
      return { benchmark: { id: afterBenchmark.id }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: BenchmarksDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};

// ­ƒôÅinverseBenchmarksDiff inverts a benchmark diff to reverse its effect.
const inverseBenchmarksDiff = (original: Benchmark[], appliedDiff: BenchmarksDiff): BenchmarksDiff => {
  const addedIds = appliedDiff.added?.map((b) => b.id) ?? [];
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  const updatedIds = appliedDiff.updated?.map((u) => u.benchmark.id) ?? [];
  return {
    removed: addedIds.map((id) => ({ id })),
    added: original.filter((b) => removedIds.includes(b.id)),
    updated: updatedIds.map((id) => {
      const orig = original.find((b) => b.id === id)!;
      const upd = appliedDiff.updated?.find((u) => u.benchmark.id === id)!;
      return { benchmark: { id }, diff: inverseBenchmarkDiff(orig, upd.diff) };
    }),
  };
};
// ­ƒôÅmergeBenchmarksDiff merges two benchmark diffs into one.
const mergeBenchmarksDiff = (first: BenchmarksDiff, second: BenchmarksDiff): BenchmarksDiff => {
  return { ...first, ...second };
};

// ­ƒôÅapplyBenchmarksDiff applies a benchmark diff to a collection.
const applyBenchmarksDiff = (items: Benchmark[], diff: BenchmarksDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const item = items.find((b) => b.id === update.benchmark.id);
      if (item) applyBenchmarkDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map((b) => new Benchmark(b as BenchmarkPlain)));
  }
};

// #endregion ­ƒôÅBenchmark

// #region ­ƒö¼Quality
// Quality entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Quality validation.
 **/
export const QualitySchema = z.object({
  id: z.string(),
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
export type QualityPlain = z.infer<typeof QualitySchema>;
export class Quality implements QualityPlain {
  id!: string;
  key!: string;
  name!: string;
  description?: string;
  uri?: string;
  kind?: number;
  folder?: string;
  canScale?: boolean;
  defaultSiUnit?: string;
  defaultImperialUnit?: string;
  min?: number;
  isMinExcluded?: boolean;
  max?: number;
  isMaxExcluded?: boolean;
  defaultValue?: number;
  formula?: string;
  icon?: string;
  image?: string;
  unit?: string;
  benchmarks?: Benchmark[];
  attributes?: Attribute[];
  constructor(plain: QualityPlain) {
    const p = QualitySchema.parse(plain);
    Object.assign(this, p);
    this.benchmarks = p.benchmarks?.map((b) => new Benchmark(b));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: QualityPlain): Quality {
    return new Quality(plain);
  }
  toPlain(): QualityPlain {
    return QualitySchema.parse(this as unknown as QualityPlain);
  }
  /** ­ƒö¼Serialize this quality for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒö¼Deserialize a quality from transport JSON. */
  static deserialize(json: string): Quality {
    return new Quality(QualitySchema.parse(JSON.parse(json)));
  }
  /** ­ƒö¼Compute a quality delta from this quality to another quality. */
  diffTo(after: Quality): QualityDiff {
    const diff: QualityDiff = {};
    if (this.key !== after.key) diff.key = after.key;
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description;
    if (this.uri !== after.uri) diff.uri = after.uri;
    if (this.kind !== after.kind) diff.kind = after.kind !== undefined && this.kind !== undefined ? after.kind - this.kind : after.kind;
    if (this.canScale !== after.canScale) diff.canScale = after.canScale;
    if (this.defaultSiUnit !== after.defaultSiUnit) diff.defaultSiUnit = after.defaultSiUnit;
    if (this.defaultImperialUnit !== after.defaultImperialUnit) diff.defaultImperialUnit = after.defaultImperialUnit;
    if (this.min !== after.min) diff.min = after.min !== undefined && this.min !== undefined ? after.min - this.min : after.min;
    if (this.isMinExcluded !== after.isMinExcluded) diff.isMinExcluded = after.isMinExcluded;
    if (this.max !== after.max) diff.max = after.max !== undefined && this.max !== undefined ? after.max - this.max : after.max;
    if (this.isMaxExcluded !== after.isMaxExcluded) diff.isMaxExcluded = after.isMaxExcluded;
    if (this.defaultValue !== after.defaultValue) diff.defaultValue = after.defaultValue !== undefined && this.defaultValue !== undefined ? after.defaultValue - this.defaultValue : after.defaultValue;
    if (this.formula !== after.formula) diff.formula = after.formula;
    if (this.icon !== after.icon) diff.icon = after.icon;
    if (this.image !== after.image) diff.image = after.image;
    if (this.unit !== after.unit) diff.unit = after.unit;
    if (!deepEqual(this.benchmarks, after.benchmarks)) diff.benchmarks = getBenchmarksDiff(this.benchmarks ?? [], after.benchmarks ?? []);
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒö¼Build the reverse quality delta for an already-applied delta. */
  inverseDiff(appliedDiff: QualityDiff): QualityDiff {
    const inverse: QualityDiff = {};
    if (appliedDiff.key !== undefined) inverse.key = this.key;
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.uri !== undefined) inverse.uri = this.uri;
    if (appliedDiff.kind !== undefined) inverse.kind = this.kind;
    if (appliedDiff.canScale !== undefined) inverse.canScale = this.canScale;
    if (appliedDiff.defaultSiUnit !== undefined) inverse.defaultSiUnit = this.defaultSiUnit;
    if (appliedDiff.defaultImperialUnit !== undefined) inverse.defaultImperialUnit = this.defaultImperialUnit;
    if (appliedDiff.min !== undefined) inverse.min = this.min;
    if (appliedDiff.isMinExcluded !== undefined) inverse.isMinExcluded = this.isMinExcluded;
    if (appliedDiff.max !== undefined) inverse.max = this.max;
    if (appliedDiff.isMaxExcluded !== undefined) inverse.isMaxExcluded = this.isMaxExcluded;
    if (appliedDiff.defaultValue !== undefined) inverse.defaultValue = this.defaultValue;
    if (appliedDiff.formula !== undefined) inverse.formula = this.formula;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon;
    if (appliedDiff.image !== undefined) inverse.image = this.image;
    if (appliedDiff.unit !== undefined) inverse.unit = this.unit;
    if (appliedDiff.benchmarks !== undefined) inverse.benchmarks = inverseBenchmarksDiff(this.benchmarks ?? [], appliedDiff.benchmarks);
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒö¼Merge two quality deltas. */
  static mergeDiff(first: QualityDiff, second: QualityDiff): QualityDiff {
    return {
      ...first,
      ...second,
      benchmarks: first.benchmarks && second.benchmarks ? mergeBenchmarksDiff(first.benchmarks, second.benchmarks) : (second.benchmarks ?? first.benchmarks),
      attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes),
    };
  }
  /** ­ƒö¼Apply a quality delta to this quality. */
  applyDiff(diff: QualityDiff): void {
    if (diff.key !== undefined) this.key = diff.key;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.uri !== undefined) this.uri = diff.uri;
    if (diff.kind !== undefined) this.kind = diff.kind;
    if (diff.folder !== undefined) this.folder = diff.folder;
    if (diff.canScale !== undefined) this.canScale = diff.canScale;
    if (diff.defaultSiUnit !== undefined) this.defaultSiUnit = diff.defaultSiUnit;
    if (diff.defaultImperialUnit !== undefined) this.defaultImperialUnit = diff.defaultImperialUnit;
    if (diff.min !== undefined) this.min = diff.min;
    if (diff.isMinExcluded !== undefined) this.isMinExcluded = diff.isMinExcluded;
    if (diff.max !== undefined) this.max = diff.max;
    if (diff.isMaxExcluded !== undefined) this.isMaxExcluded = diff.isMaxExcluded;
    if (diff.defaultValue !== undefined) this.defaultValue = diff.defaultValue;
    if (diff.formula !== undefined) this.formula = diff.formula;
    if (diff.icon !== undefined) this.icon = diff.icon;
    if (diff.image !== undefined) this.image = diff.image;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.benchmarks) {
      if (!this.benchmarks) this.benchmarks = [];
      applyBenchmarksDiff(this.benchmarks, diff.benchmarks);
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Quality for transport.
 **/
export const serializeQuality = (quality: Quality): string => quality.serialize();
/**
 **/
export const deserializeQuality = (json: string): Quality => Quality.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseQuality changes.
 **/
export const inverseQualityDiff = (original: Quality, appliedDiff: QualityDiff): QualityDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeQuality changes.
 **/
export const mergeQualityDiff = (diff1: QualityDiff, diff2: QualityDiff): QualityDiff => {
  return Quality.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyQuality changes.
 **/
export const applyQualityDiff = (target: Quality, diff: QualityDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Qualities diff validation.
 **/
export const QualitiesDiffSchema = z.object({
  removed: z.array(QualityIdSchema).optional(),
  updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type QualitiesDiff = z.infer<typeof QualitiesDiffSchema>;

// #endregion ­ƒö¼Quality

// #region ÔÜôPort
// Port entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Port validation.
 **/
export const PortSchema = z.object({
  id: z.string(),
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
export type PortPlain = z.infer<typeof PortSchema>;
export class Port implements PortPlain {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  maxChildren?: number;
  compatiblePorts?: PortId[];
  attributes?: Attribute[];
  constructor(plain: PortPlain) {
    const p = PortSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: PortPlain): Port {
    return new Port(plain);
  }
  toPlain(): PortPlain {
    return PortSchema.parse(this as unknown as PortPlain);
  }
  /** ÔÜôSerialize this port for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ÔÜôDeserialize a port from transport JSON. */
  static deserialize(json: string): Port {
    return new Port(PortSchema.parse(JSON.parse(json)));
  }
  /** ÔÜôCompute a port delta from this port to another port. */
  diffTo(after: Port): PortDiff {
    const diff: PortDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description ?? null;
    if (this.icon !== after.icon) diff.icon = after.icon ?? null;
    if (this.maxChildren !== after.maxChildren) diff.maxChildren = after.maxChildren ?? null;
    if (JSON.stringify(this.compatiblePorts) !== JSON.stringify(after.compatiblePorts)) diff.compatiblePorts = after.compatiblePorts;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ÔÜôBuild the reverse port delta for an already-applied delta. */
  inverseDiff(appliedDiff: PortDiff): PortDiff {
    const inverse: PortDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description ?? null;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon ?? null;
    if (appliedDiff.maxChildren !== undefined) inverse.maxChildren = this.maxChildren ?? null;
    if (appliedDiff.compatiblePorts !== undefined) inverse.compatiblePorts = this.compatiblePorts;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ÔÜôMerge two port deltas. */
  static mergeDiff(first: PortDiff, second: PortDiff): PortDiff {
    return {
      ...first,
      ...second,
      attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes),
    };
  }
  /** ÔÜôApply a port delta to this port. */
  applyDiff(diff: PortDiff): void {
    if (diff.name !== undefined) this.name = diff.name;
    if ("description" in diff) {
      this.description = diff.description !== null ? diff.description : undefined;
    }
    if ("icon" in diff) {
      this.icon = diff.icon !== null ? diff.icon : undefined;
    }
    if ("maxChildren" in diff) {
      this.maxChildren = diff.maxChildren !== null ? diff.maxChildren : undefined;
    }
    if (diff.compatiblePorts !== undefined) this.compatiblePorts = diff.compatiblePorts;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Port for transport.
 **/
export const serializePort = (iface: Port): string => iface.serialize();
/**
 **/
export const deserializePort = (json: string): Port => Port.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inversePort changes.
 **/
export const inversePortDiff = (original: Port, appliedDiff: PortDiff): PortDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergePort changes.
 **/
export const mergePortDiff = (diff1: PortDiff, diff2: PortDiff): PortDiff => {
  return Port.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyPort changes.
 **/
export const applyPortDiff = (target: Port, diff: PortDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Ports diff validation.
 **/
export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
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
  const beforeIds = new Set(before.map((i) => i.id));
  const afterIds = new Set(after.map((i) => i.id));
  const removed = before.filter((i) => !afterIds.has(i.id)).map((i) => ({ id: i.id }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterIds.has(i.id))
    .map((i) => {
      const afterPort = after.find((a) => a.id === i.id)!;
      const portDiff = getPortDiff(i, afterPort);
      return { port: { id: i.id }, diff: portDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeIds.has(i.id));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Diff type for tracking inversePorts changes.
 **/
export const inversePortsDiff = (original: Port[], appliedDiff: PortsDiff): PortsDiff => {
  const inverse: PortsDiff = {};
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedIds.includes(i.id));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ id: i.id }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalPort = original.find((i) => i.id === u.port.id)!;
      return { port: { id: u.port.id }, diff: inversePortDiff(originalPort, u.diff) };
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
export const applyPortsDiff = (items: Port[], diff: PortsDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const item = items.find((i) => i.id === update.port.id);
      if (item) applyPortDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map((x) => new Port(x as PortPlain)));
  }
};

/**
 **/
export const arePortsCompatible = (iface1: Port | undefined, iface2: Port | undefined, allPorts: Port[]): boolean => {
  if (!iface1 || !iface2) return true;
  if (iface1.id === iface2.id) return true;
  const iface1Compatible = iface1.compatiblePorts ?? [];
  const iface2Compatible = iface2.compatiblePorts ?? [];
  if (iface1Compatible.length === 0 && iface2Compatible.length === 0) return true;
  if (iface1Compatible.length === 0) return iface2Compatible.some((c) => c.id === iface1.id);
  if (iface2Compatible.length === 0) return iface1Compatible.some((c) => c.id === iface2.id);
  return iface1Compatible.some((c) => c.id === iface2.id) || iface2Compatible.some((c) => c.id === iface1.id);
};

export const getKitPorts = (kit: { families?: Array<{ ports?: Port[] }> } | undefined | null): Port[] => (kit?.families ?? []).flatMap((family) => family.ports ?? []);

export const findKitPortFamily = (kit: { families?: Array<{ id: string; ports?: Array<{ id: string }> }> } | undefined | null, portId: string): { id: string; ports?: Array<{ id: string }> } | undefined =>
  (kit?.families ?? []).find((family) => (family.ports ?? []).some((port) => port.id === portId));

// #endregion ÔÜôPort

// #region ´┐¢Family
// Family entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Family validation.
 **/
export const FamilySchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  ports: z.array(PortSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Family.
 **/
export type FamilyPlain = z.infer<typeof FamilySchema>;
export class Family implements FamilyPlain {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  ports?: Port[];
  attributes?: Attribute[];
  constructor(plain: FamilyPlain) {
    const p = FamilySchema.parse(plain);
    Object.assign(this, p);
    this.ports = p.ports?.map((x) => new Port(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: FamilyPlain): Family {
    return new Family(plain);
  }
  toPlain(): FamilyPlain {
    return FamilySchema.parse(this as unknown as FamilyPlain);
  }
  /** ­ƒæ¬Serialize this family for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒæ¬Deserialize a family from transport JSON. */
  static deserialize(json: string): Family {
    return new Family(FamilySchema.parse(JSON.parse(json)));
  }
  /** ­ƒæ¬Compute a family delta from this family to another family. */
  diffTo(after: Family): FamilyDiff {
    const diff: FamilyDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description ?? null;
    if (this.icon !== after.icon) diff.icon = after.icon ?? null;
    if (!deepEqual(this.ports, after.ports)) diff.ports = getPortsDiff(this.ports ?? [], after.ports ?? []);
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒæ¬Build the reverse family delta for an already-applied delta. */
  inverseDiff(appliedDiff: FamilyDiff): FamilyDiff {
    const inverse: FamilyDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description ?? null;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon ?? null;
    if (appliedDiff.ports !== undefined) inverse.ports = inversePortsDiff(this.ports ?? [], appliedDiff.ports);
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒæ¬Merge two family deltas. */
  static mergeDiff(first: FamilyDiff, second: FamilyDiff): FamilyDiff {
    return {
      ...first,
      ...second,
      ports: first.ports && second.ports ? mergePortsDiff(first.ports, second.ports) : (second.ports ?? first.ports),
      attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes),
    };
  }
  /** ­ƒæ¬Apply a family delta to this family. */
  applyDiff(diff: FamilyDiff): void {
    if (diff.name !== undefined) this.name = diff.name;
    if ("description" in diff) {
      this.description = diff.description !== null ? diff.description : undefined;
    }
    if ("icon" in diff) {
      this.icon = diff.icon !== null ? diff.icon : undefined;
    }
    if (diff.ports) {
      if (!this.ports) this.ports = [];
      applyPortsDiff(this.ports, diff.ports);
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Family for transport.
 **/
export const serializeFamily = (family: Family): string => family.serialize();
/**
 **/
export const deserializeFamily = (json: string): Family => Family.deserialize(json);

/**
 * Definition of FamilyMetaSchema.
 **/
export const FamilyMetaSchema = FamilySchema.omit({ ports: true, attributes: true });
/**
 * Type alias for FamilyMeta.
 **/
export type FamilyMeta = z.infer<typeof FamilyMetaSchema>;
/**
 * Serializes FamilyMeta for transport.
 **/
export const serializeFamilyMeta = (family: FamilyMeta): string => JSON.stringify(FamilyMetaSchema.parse(family));
/**
 **/
export const deserializeFamilyMeta = (json: string): FamilyMeta => FamilyMetaSchema.parse(JSON.parse(json));
/**
 * Definition of FamilyShallowSchema.
 **/
export const FamilyShallowSchema = FamilySchema;
/**
 * Type alias for FamilyShallow.
 **/
export type FamilyShallow = z.infer<typeof FamilyShallowSchema>;
/**
 * Serializes FamilyShallow for transport.
 **/
export const serializeFamilyShallow = (family: FamilyShallow): string => JSON.stringify(FamilyShallowSchema.parse(family));
/**
 **/
export const deserializeFamilyShallow = (json: string): FamilyShallow => FamilyShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Family diff validation.
 **/
export const FamilyDiffSchema = FamilySchema.partial().omit({ ports: true, attributes: true }).extend({
  ports: PortsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
  icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Family changes.
 **/
export type FamilyDiff = z.infer<typeof FamilyDiffSchema>;
/**
 * Retrieves the FamilyDiff value.
 **/
export const getFamilyDiff = (before: Family, after: Family): FamilyDiff => {
  return before.diffTo(after);
};
/**
 * Inverse of FamilyDiff.
 **/
export const inverseFamilyDiff = (original: Family, appliedDiff: FamilyDiff): FamilyDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Merge two FamilyDiffs.
 **/
export const mergeFamilyDiff = (diff1: FamilyDiff, diff2: FamilyDiff): FamilyDiff => {
  return Family.mergeDiff(diff1, diff2);
};
/**
 * Apply a FamilyDiff.
 **/
export const applyFamilyDiff = (family: Family, diff: FamilyDiff): void => {
  family.applyDiff(diff);
};

/**
 * Zod schema for Families collection diff.
 **/
export const FamiliesDiffSchema = z.object({
  removed: z.array(FamilyIdSchema).optional(),
  updated: z.array(z.object({ family: FamilyIdSchema, diff: FamilyDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Families changes.
 **/
export type FamiliesDiff = z.infer<typeof FamiliesDiffSchema>;
/**
 * Retrieves the FamiliesDiff value.
 **/
export const getFamiliesDiff = (before: Family[], after: Family[]): FamiliesDiff => {
  const diff: FamiliesDiff = {};
  const beforeIds = new Set(before.map((i) => i.id));
  const afterIds = new Set(after.map((i) => i.id));
  const removed = before.filter((i) => !afterIds.has(i.id)).map((i) => ({ id: i.id }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterIds.has(i.id))
    .map((i) => {
      const afterFamily = after.find((a) => a.id === i.id)!;
      const familyDiff = getFamilyDiff(i, afterFamily);
      return { family: { id: i.id }, diff: familyDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeIds.has(i.id));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Inverse of FamiliesDiff.
 **/
export const inverseFamiliesDiff = (original: Family[], appliedDiff: FamiliesDiff): FamiliesDiff => {
  const inverse: FamiliesDiff = {};
  if (appliedDiff.removed) inverse.added = original.filter((i) => appliedDiff.removed!.some((r) => r.id === i.id));
  if (appliedDiff.added) inverse.removed = (appliedDiff.added as Family[]).map((i) => ({ id: i.id }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated
      .filter((u) => original.some((i) => i.id === u.family.id))
      .map((u) => {
        const orig = original.find((i) => i.id === u.family.id)!;
        return { family: { id: u.family.id }, diff: inverseFamilyDiff(orig, u.diff) };
      });
  }
  return inverse;
};
/**
 * Merge two FamiliesDiffs.
 **/
export const mergeFamiliesDiff = (diff1: FamiliesDiff, diff2: FamiliesDiff): FamiliesDiff => {
  return {
    removed: [...(diff1.removed ?? []), ...(diff2.removed ?? [])].length > 0 ? [...(diff1.removed ?? []), ...(diff2.removed ?? [])] : undefined,
    updated: [...(diff1.updated ?? []), ...(diff2.updated ?? [])].length > 0 ? [...(diff1.updated ?? []), ...(diff2.updated ?? [])] : undefined,
    added: [...(diff1.added ?? []), ...(diff2.added ?? [])].length > 0 ? [...(diff1.added ?? []), ...(diff2.added ?? [])] : undefined,
  };
};
/**
 * Apply a FamiliesDiff to a collection.
 **/
export const applyFamiliesDiff = (families: Family[], diff: FamiliesDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = families.length - 1; i >= 0; i--) {
      if (removedIds.has(families[i].id)) families.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const u of diff.updated) {
      const family = families.find((i) => i.id === u.family.id);
      if (family) applyFamilyDiff(family, u.diff);
    }
  }
  if (diff.added) {
    for (const a of diff.added) {
      families.push(new Family(FamilySchema.parse(a)));
    }
  }
};

// #endregion ­ƒæ¬Family

// #region ´┐¢­ƒôèProp
// Prop entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Prop validation.
 **/
export const PropSchema = z.object({
  id: z.string(),
  quality: QualityIdSchema,
  value: z.string(),
  unit: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Prop.
 **/
export type PropPlain = z.infer<typeof PropSchema>;
export class Prop implements PropPlain {
  id!: string;
  quality!: QualityId;
  value!: string;
  unit?: string;
  attributes?: Attribute[];
  constructor(plain: PropPlain) {
    const p = PropSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: PropPlain): Prop {
    return new Prop(plain);
  }
  toPlain(): PropPlain {
    return PropSchema.parse(this as unknown as PropPlain);
  }
  /** ­ƒôèSerialize this prop for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒôèDeserialize a prop from transport JSON. */
  static deserialize(json: string): Prop {
    return new Prop(PropSchema.parse(JSON.parse(json)));
  }
  /** ­ƒôèCompute a prop delta from this prop to another prop. */
  diffTo(after: Prop): PropDiff {
    const diff: PropDiff = {};
    if (this.quality?.id !== after.quality?.id) diff.quality = after.quality;
    if (this.value !== after.value) diff.value = after.value;
    if (this.unit !== after.unit) diff.unit = after.unit;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒôèBuild the reverse prop delta for an already-applied delta. */
  inverseDiff(appliedDiff: PropDiff): PropDiff {
    const inverse: PropDiff = {};
    if (appliedDiff.quality !== undefined) inverse.quality = this.quality;
    if (appliedDiff.value !== undefined) inverse.value = this.value;
    if (appliedDiff.unit !== undefined) inverse.unit = this.unit;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒôèMerge two prop deltas. */
  static mergeDiff(first: PropDiff, second: PropDiff): PropDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒôèApply a prop delta to this prop. */
  applyDiff(diff: PropDiff): void {
    if (diff.quality !== undefined) this.quality = diff.quality;
    if (diff.value !== undefined) this.value = diff.value;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Prop for transport.
 **/
export const serializeProp = (prop: Prop): string => prop.serialize();
/**
 **/
export const deserializeProp = (json: string): Prop => Prop.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseProp changes.
 **/
export const inversePropDiff = (original: Prop, appliedDiff: PropDiff): PropDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeProp changes.
 **/
export const mergePropDiff = (diff1: PropDiff, diff2: PropDiff): PropDiff => {
  return Prop.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyProp changes.
 **/
export const applyPropDiff = (target: Prop, diff: PropDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Props diff validation.
 **/
export const PropsDiffSchema = z.object({
  removed: z.array(PropIdSchema).optional(),
  updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Props changes.
 **/
export type PropsDiff = z.infer<typeof PropsDiffSchema>;
// ­ƒôègetPropsDiff computes the diff between two prop collections.
const getPropsDiff = (before: Prop[], after: Prop[]): PropsDiff => {
  const beforeIds = new Set(before.map((p) => p.id));
  const afterIds = new Set(after.map((p) => p.id));
  const removed = before.filter((p) => !afterIds.has(p.id)).map((p) => ({ id: p.id }));
  const added = after.filter((p) => !beforeIds.has(p.id));
  const updated = after
    .filter((p) => beforeIds.has(p.id))
    .map((afterProp) => {
      const beforeProp = before.find((p) => p.id === afterProp.id)!;
      const diff = getPropDiff(beforeProp, afterProp);
      return { prop: { id: afterProp.id }, diff };
    })
    .filter((update) => Object.keys(update.diff).length > 0);
  const diff: PropsDiff = {};
  if (removed.length > 0) diff.removed = removed;
  if (updated.length > 0) diff.updated = updated;
  if (added.length > 0) diff.added = added;
  return diff;
};
// ­ƒôèinversePropsDiff inverts a prop diff to reverse its effect.
const inversePropsDiff = (original: Prop[], appliedDiff: PropsDiff): PropsDiff => {
  const addedIds = appliedDiff.added?.map((p) => p.id) ?? [];
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  const updatedIds = appliedDiff.updated?.map((u) => u.prop.id) ?? [];
  return {
    removed: addedIds.map((id) => ({ id })),
    added: original.filter((p) => removedIds.includes(p.id)),
    updated: updatedIds.map((id) => {
      const orig = original.find((p) => p.id === id)!;
      const upd = appliedDiff.updated?.find((u) => u.prop.id === id)!;
      return { prop: { id }, diff: inversePropDiff(orig, upd.diff) };
    }),
  };
};
// ­ƒôèmergePropsDiff merges two prop diffs into one.
const mergePropsDiff = (first: PropsDiff, second: PropsDiff): PropsDiff => {
  return { ...first, ...second };
};
// ­ƒôèapplyPropsDiff applies a prop diff to a collection.
const applyPropsDiff = (items: Prop[], diff: PropsDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const item = items.find((p) => p.id === update.prop.id);
      if (item) applyPropDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map((x) => new Prop(x as PropPlain)));
  }
};

// #endregion ­ƒôèProp

// #region ­ƒÅÀ´©ÅTag
// Tag entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Tag validation.
 **/
export const TagSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Tag.
 **/
export type TagPlain = z.infer<typeof TagSchema>;
export class Tag implements TagPlain {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  attributes?: Attribute[];
  constructor(plain: TagPlain) {
    const p = TagSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: TagPlain): Tag {
    return new Tag(plain);
  }
  toPlain(): TagPlain {
    return TagSchema.parse(this as unknown as TagPlain);
  }
  /** ­ƒÅÀ´©ÅSerialize this tag for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒÅÀ´©ÅDeserialize a tag from transport JSON. */
  static deserialize(json: string): Tag {
    return new Tag(TagSchema.parse(JSON.parse(json)));
  }
  /** ­ƒÅÀ´©ÅCompute a tag delta from this tag to another tag. */
  diffTo(after: Tag): TagDiff {
    const diff: TagDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description ?? null;
    if (this.icon !== after.icon) diff.icon = after.icon ?? null;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒÅÀ´©ÅBuild the reverse tag delta for an already-applied delta. */
  inverseDiff(appliedDiff: TagDiff): TagDiff {
    const inverse: TagDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description ?? null;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon ?? null;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒÅÀ´©ÅMerge two tag deltas. */
  static mergeDiff(first: TagDiff, second: TagDiff): TagDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒÅÀ´©ÅApply a tag delta to this tag. */
  applyDiff(diff: TagDiff): void {
    if ("name" in diff && diff.name !== undefined) this.name = diff.name;
    if ("description" in diff) {
      this.description = diff.description ?? undefined;
    }
    if ("icon" in diff) {
      this.icon = diff.icon ?? undefined;
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Tag for transport.
 **/
export const serializeTag = (tag: Tag): string => tag.serialize();
/**
 **/
export const deserializeTag = (json: string): Tag => Tag.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseTag changes.
 **/
export const inverseTagDiff = (original: Tag, appliedDiff: TagDiff): TagDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeTag changes.
 **/
export const mergeTagDiff = (diff1: TagDiff, diff2: TagDiff): TagDiff => {
  return Tag.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyTag changes.
 **/
export const applyTagDiff = (target: Tag, diff: TagDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Tags diff validation.
 **/
export const TagsDiffSchema = z.object({
  removed: z.array(TagIdSchema).optional(),
  updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
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
  const beforeIds = new Set(before.map((t) => t.id));
  const afterIds = new Set(after.map((t) => t.id));
  const removed = before.filter((t) => !afterIds.has(t.id)).map((t) => ({ id: t.id }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((t) => afterIds.has(t.id))
    .map((t) => {
      const afterTag = after.find((a) => a.id === t.id)!;
      const tagDiff = getTagDiff(t, afterTag);
      return { tag: { id: t.id }, diff: tagDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((t) => !beforeIds.has(t.id));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Diff type for tracking inverseTags changes.
 **/
export const inverseTagsDiff = (original: Tag[], appliedDiff: TagsDiff): TagsDiff => {
  const inverse: TagsDiff = {};
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((t) => removedIds.includes(t.id));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((t) => ({ id: t.id }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalTag = original.find((t) => t.id === u.tag.id)!;
      return { tag: { id: u.tag.id }, diff: inverseTagDiff(originalTag, u.diff) };
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
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [u.tag.id, u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [u.tag.id, u.diff]));
  const allIds = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allIds).map((id) => ({
    tag: { id },
    diff: mergeTagDiff(updated1Map.get(id) ?? {}, updated2Map.get(id) ?? {}),
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
export const applyTagsDiff = (items: Tag[], diff: TagsDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const item = items.find((t) => t.id === update.tag.id);
      if (item) applyTagDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map((t) => new Tag(t as TagPlain)));
  }
};

/**
 * Searches for matching Tag entry.
 **/
export const findTag = (tags: Tag[], id: string): Tag => {
  const tag = tags.find((t) => t.id === id);
  if (!tag) throw new Error(`Tag ${id} not found`);
  return tag;
};

// #endregion ­ƒÅÀ´©ÅTag

// #region ­ƒÆíConcept
// Concept entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Concept validation.
 **/
export const ConceptSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Concept.
 **/
export type ConceptPlain = z.infer<typeof ConceptSchema>;
export class Concept implements ConceptPlain {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  attributes?: Attribute[];
  constructor(plain: ConceptPlain) {
    const p = ConceptSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: ConceptPlain): Concept {
    return new Concept(plain);
  }
  toPlain(): ConceptPlain {
    return ConceptSchema.parse(this as unknown as ConceptPlain);
  }
  /** ­ƒÆíSerialize this concept for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒÆíDeserialize a concept from transport JSON. */
  static deserialize(json: string): Concept {
    return new Concept(ConceptSchema.parse(JSON.parse(json)));
  }
  /** ­ƒÆíCompute a concept delta from this concept to another concept. */
  diffTo(after: Concept): ConceptDiff {
    const diff: ConceptDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description ?? null;
    if (this.icon !== after.icon) diff.icon = after.icon ?? null;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒÆíBuild the reverse concept delta for an already-applied delta. */
  inverseDiff(appliedDiff: ConceptDiff): ConceptDiff {
    const inverse: ConceptDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description ?? null;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon ?? null;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒÆíMerge two concept deltas. */
  static mergeDiff(first: ConceptDiff, second: ConceptDiff): ConceptDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒÆíApply a concept delta to this concept. */
  applyDiff(diff: ConceptDiff): void {
    if ("name" in diff && diff.name !== undefined) this.name = diff.name;
    if ("description" in diff) {
      this.description = diff.description ?? undefined;
    }
    if ("icon" in diff) {
      this.icon = diff.icon ?? undefined;
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Concept for transport.
 **/
export const serializeConcept = (concept: Concept): string => concept.serialize();
/**
 **/
export const deserializeConcept = (json: string): Concept => Concept.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseConcept changes.
 **/
export const inverseConceptDiff = (original: Concept, appliedDiff: ConceptDiff): ConceptDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeConcept changes.
 **/
export const mergeConceptDiff = (diff1: ConceptDiff, diff2: ConceptDiff): ConceptDiff => {
  return Concept.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyConcept changes.
 **/
export const applyConceptDiff = (target: Concept, diff: ConceptDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Concepts diff validation.
 **/
export const ConceptsDiffSchema = z.object({
  removed: z.array(ConceptIdSchema).optional(),
  updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
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
  const beforeIds = new Set(before.map((c) => c.id));
  const afterIds = new Set(after.map((c) => c.id));
  const removed = before.filter((c) => !afterIds.has(c.id)).map((c) => ({ id: c.id }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((c) => afterIds.has(c.id))
    .map((c) => {
      const afterConcept = after.find((a) => a.id === c.id)!;
      const conceptDiff = getConceptDiff(c, afterConcept);
      return { concept: { id: c.id }, diff: conceptDiff };
    })
    .filter((u) => Object.keys(u.diff).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((c) => !beforeIds.has(c.id));
  if (added.length > 0) diff.added = added;
  return diff;
};
/**
 * Diff type for tracking inverseConcepts changes.
 **/
export const inverseConceptsDiff = (original: Concept[], appliedDiff: ConceptsDiff): ConceptsDiff => {
  const inverse: ConceptsDiff = {};
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((c) => removedIds.includes(c.id));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((c) => ({ id: c.id }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated.map((u) => {
      const originalConcept = original.find((c) => c.id === u.concept.id)!;
      return { concept: { id: u.concept.id }, diff: inverseConceptDiff(originalConcept, u.diff) };
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
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [u.concept.id, u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [u.concept.id, u.diff]));
  const allIds = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allIds).map((id) => ({
    concept: { id },
    diff: mergeConceptDiff(updated1Map.get(id) ?? {}, updated2Map.get(id) ?? {}),
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
export const applyConceptsDiff = (items: Concept[], diff: ConceptsDiff): void => {
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const item = items.find((c) => c.id === update.concept.id);
      if (item) applyConceptDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map((c) => new Concept(c as ConceptPlain)));
  }
};

/**
 * Searches for matching Concept entry.
 **/
export const findConcept = (concepts: Concept[], id: string): Concept => {
  const concept = concepts.find((c) => c.id === id);
  if (!concept) throw new Error(`Concept ${id} not found`);
  return concept;
};

// #endregion ­ƒÆíConcept

// #region ­ƒù┐Representation
// Representation entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Representation validation.
 **/
export const RepresentationSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  tags: z.array(TagIdSchema).optional(),
  file: FileIdSchema,
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Representation.
 **/
export type RepresentationPlain = z.infer<typeof RepresentationSchema>;
export class Representation implements RepresentationPlain {
  id!: string;
  name?: string;
  tags?: TagId[];
  file!: FileId;
  description?: string;
  attributes?: Attribute[];
  constructor(plain: RepresentationPlain) {
    const p = RepresentationSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: RepresentationPlain): Representation {
    return new Representation(plain);
  }
  toPlain(): RepresentationPlain {
    return RepresentationSchema.parse(this as unknown as RepresentationPlain);
  }
  /** ­ƒù┐Serialize this representation for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒù┐Deserialize a representation from transport JSON. */
  static deserialize(json: string): Representation {
    return new Representation(RepresentationSchema.parse(JSON.parse(json)));
  }
  /** ­ƒù┐Compute a representation delta from this representation to another representation. */
  diffTo(after: Representation): RepresentationDiff {
    const diff: RepresentationDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (JSON.stringify(this.tags) !== JSON.stringify(after.tags)) diff.tags = after.tags;
    if (this.file.id !== after.file.id) diff.file = after.file;
    if (this.description !== after.description) diff.description = after.description;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒù┐Build the reverse representation delta for an already-applied delta. */
  inverseDiff(appliedDiff: RepresentationDiff): RepresentationDiff {
    const inverse: RepresentationDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.tags !== undefined) inverse.tags = this.tags;
    if (appliedDiff.file !== undefined) inverse.file = this.file;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒù┐Merge two representation deltas. */
  static mergeDiff(first: RepresentationDiff, second: RepresentationDiff): RepresentationDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒù┐Apply a representation delta to this representation. */
  applyDiff(diff: RepresentationDiff): void {
    if (diff.file !== undefined) this.file = diff.file;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.tags !== undefined) this.tags = diff.tags;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Representation for transport.
 **/
export const serializeRepresentation = (representation: Representation): string => representation.serialize();
/**
 **/
export const deserializeRepresentation = (json: string): Representation => Representation.deserialize(json);

/**
 * Definition of RepresentationMetaSchema.
 **/
export const RepresentationMetaSchema = RepresentationSchema.omit({ tags: true, attributes: true });
/**
 * Type alias for RepresentationMeta.
 **/
export type RepresentationMeta = z.infer<typeof RepresentationMetaSchema>;
/**
 * Serializes RepresentationMeta for transport.
 **/
export const serializeRepresentationMeta = (representation: RepresentationMeta): string => JSON.stringify(RepresentationMetaSchema.parse(representation));
/**
 **/
export const deserializeRepresentationMeta = (json: string): RepresentationMeta => RepresentationMetaSchema.parse(JSON.parse(json));
/**
 * Definition of RepresentationShallowSchema.
 **/
export const RepresentationShallowSchema = RepresentationSchema;
/**
 * Type alias for RepresentationShallow.
 **/
export type RepresentationShallow = z.infer<typeof RepresentationShallowSchema>;
/**
 * Serializes RepresentationShallow for transport.
 **/
export const serializeRepresentationShallow = (representation: RepresentationShallow): string => JSON.stringify(RepresentationShallowSchema.parse(representation));
/**
 **/
export const deserializeRepresentationShallow = (json: string): RepresentationShallow => RepresentationShallowSchema.parse(JSON.parse(json));

/**
 * Zod schema for Representation diff validation.
 **/
export const RepresentationDiffSchema = RepresentationSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Representation changes.
 **/
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;
/**
 * Retrieves the RepresentationDiff value.
 **/
export const getRepresentationDiff = (before: Representation, after: Representation): RepresentationDiff => {
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseRepresentation changes.
 **/
export const inverseRepresentationDiff = (original: Representation, appliedDiff: RepresentationDiff): RepresentationDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeRepresentation changes.
 **/
export const mergeRepresentationDiff = (diff1: RepresentationDiff, diff2: RepresentationDiff): RepresentationDiff => {
  return Representation.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyRepresentation changes.
 **/
export const applyRepresentationDiff = (target: Representation, diff: RepresentationDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Representations diff validation.
 **/
export const RepresentationsDiffSchema = z.object({
  removed: z.array(RepresentationIdSchema).optional(),
  updated: z.array(z.object({ representation: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type RepresentationsDiff = z.infer<typeof RepresentationsDiffSchema>;

/**
 * Equality check for Representation values.
 **/
export const areSameRepresentation = (representation: Representation, other: Representation): boolean => {
  const representationTagIds = representation.tags?.map((t) => t.id) ?? [];
  const otherTagIds = other.tags?.map((t) => t.id) ?? [];
  return representationTagIds.every((id) => otherTagIds.includes(id));
};

/**
 * Searches for matching Representation entry.
 **/
export const findRepresentation = (representations: Representation[], tagIds: string[]): Representation => {
  const indices = representations.map((r) =>
    jaccard(
      r.tags?.map((t) => t.id),
      tagIds,
    ),
  );
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return representations[maxIndexIndex];
};

/**
 * Retrieves the AllTagIdsFromRepresentations value.
 **/
export const getAllTagIdsFromRepresentations = (representations: Representation[]): string[] => {
  const tagsSet = new Set<string>();
  representations.forEach((r) => {
    toArray(r.tags).forEach((tag) => tagsSet.add(tag.id));
  });
  return Array.from(tagsSet).sort();
};

/**
 **/
export const filterRepresentationsByTagIds = (representations: Representation[], selectedTagIds: string[]): Representation[] => {
  if (!selectedTagIds || selectedTagIds.length === 0) return representations;
  return representations.filter((r) => {
    if (!r.tags || r.tags.length === 0) return false;
    const representationTagIds = r.tags.map((t) => t.id);
    return selectedTagIds.every((id) => representationTagIds.includes(id));
  });
};

/**
 * Retrieves the AvailableTagIdsForRepresentations value.
 **/
export const getAvailableTagIdsForRepresentations = (representations: Representation[], selectedTagIds: string[]): string[] => {
  const filteredReps = filterRepresentationsByTagIds(representations, selectedTagIds);
  const availableTags = getAllTagIdsFromRepresentations(filteredReps);
  return availableTags.filter((id) => !selectedTagIds.includes(id));
};

/**
 **/
export const selectBestRepresentation = (representations: Representation[], selectedTagIds: string[]): Representation | undefined => {
  if (representations.length === 0) return undefined;
  if (selectedTagIds.length === 0) {
    const defaultRep = representations.find((r) => !r.tags || r.tags.length === 0);
    return defaultRep ?? representations[0];
  }
  const filteredReps = filterRepresentationsByTagIds(representations, selectedTagIds);
  if (filteredReps.length === 0) return undefined;
  return findRepresentation(filteredReps, selectedTagIds);
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
export const isSupportedRepresentationExtension = (filename: string): boolean => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  return SUPPORTED_3D_EXTENSIONS.includes(ext as Supported3DExtension);
};

/**
 * Interface defining RepresentationFileValidation structure.
 **/
export interface RepresentationFileValidation {
  isValid: boolean;
  warning?: string;
  extension?: string;
}

/**
 * Validates RepresentationFile against constraints.
 **/
export const validateRepresentationFile = (filename: string): RepresentationFileValidation => {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  if (!ext) {
    return { isValid: false, warning: "File has no extension" };
  }
  if (!isSupportedRepresentationExtension(filename)) {
    return {
      isValid: true,
      warning: `File extension '.${ext}' is not a common 3D format. Supported: ${SUPPORTED_3D_EXTENSIONS.slice(0, 5).join(", ")}...`,
      extension: ext,
    };
  }
  return { isValid: true, extension: ext };
};

// #endregion ­ƒù┐Representation

// #region ­ƒöîConnector
// Connector entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Connector validation.
 **/
export const ConnectorSchema = z.object({
  id: z.string(),
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
export type ConnectorPlain = z.infer<typeof ConnectorSchema>;
export class Connector implements ConnectorPlain {
  id!: string;
  name?: string;
  t!: number;
  point!: Point;
  direction!: Vector;
  description?: string;
  port?: PortId;
  mandatory?: boolean;
  maxChildren?: number;
  props?: Prop[];
  attributes?: Attribute[];
  constructor(plain: ConnectorPlain) {
    const p = ConnectorSchema.parse(plain);
    Object.assign(this, p);
    this.point = new Point(p.point);
    this.direction = new Vector(p.direction);
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: ConnectorPlain): Connector {
    return new Connector(plain);
  }
  toPlain(): ConnectorPlain {
    return ConnectorSchema.parse(this as unknown as ConnectorPlain);
  }
  /** ­ƒöîSerialize this connector for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒöîDeserialize a connector from transport JSON. */
  static deserialize(json: string): Connector {
    return new Connector(ConnectorSchema.parse(JSON.parse(json)));
  }
  /** ­ƒöîCompute a connector delta from this connector to another connector. */
  diffTo(after: Connector): ConnectorDiff {
    const diff: ConnectorDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description;
    if (this.port?.id !== after.port?.id) diff.port = after.port;
    if (this.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
    if (this.maxChildren !== after.maxChildren) diff.maxChildren = after.maxChildren ?? null;
    if (this.t !== after.t) diff.t = after.t;
    if (!deepEqual(this.point, after.point)) diff.point = this.point.diffTo(after.point);
    if (!deepEqual(this.direction, after.direction)) diff.direction = this.direction.diffTo(after.direction);
    if (!deepEqual(this.props, after.props)) diff.props = getPropsDiff(this.props ?? [], after.props ?? []);
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒöîBuild the reverse connector delta for an already-applied delta. */
  inverseDiff(appliedDiff: ConnectorDiff): ConnectorDiff {
    const inverse: ConnectorDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.port !== undefined) inverse.port = this.port;
    if (appliedDiff.mandatory !== undefined) inverse.mandatory = this.mandatory;
    if (appliedDiff.maxChildren !== undefined) inverse.maxChildren = this.maxChildren ?? null;
    if (appliedDiff.t !== undefined) inverse.t = this.t;
    if (appliedDiff.point !== undefined) inverse.point = this.point.inverseDiff(appliedDiff.point);
    if (appliedDiff.direction !== undefined) inverse.direction = this.direction.inverseDiff(appliedDiff.direction);
    if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(this.props ?? [], appliedDiff.props);
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒöîMerge two connector deltas. */
  static mergeDiff(first: ConnectorDiff, second: ConnectorDiff): ConnectorDiff {
    return {
      ...first,
      ...second,
      point: second.point ?? first.point,
      direction: second.direction ?? first.direction,
      props: first.props && second.props ? mergePropsDiff(first.props, second.props) : (second.props ?? first.props),
      attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes),
    };
  }
  /** ­ƒöîApply a connector delta to this connector. */
  applyDiff(diff: ConnectorDiff): void {
    if (diff.t !== undefined) this.t = diff.t;
    if (diff.point) this.point.applyDiff(diff.point);
    if (diff.direction) this.direction.applyDiff(diff.direction);
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.port !== undefined) this.port = diff.port;
    if (diff.mandatory !== undefined) this.mandatory = diff.mandatory;
    if ("maxChildren" in diff) {
      this.maxChildren = diff.maxChildren !== null ? diff.maxChildren : undefined;
    }
    if (diff.props) {
      if (!this.props) this.props = [];
      applyPropsDiff(this.props, diff.props);
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Connector for transport.
 **/
export const serializeConnector = (connector: Connector): string => connector.serialize();
/**
 **/
export const deserializeConnector = (json: string): Connector => Connector.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking mergeConnector changes.
 **/
export const mergeConnectorDiff = (diff1: ConnectorDiff, diff2: ConnectorDiff): ConnectorDiff => {
  return Connector.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking inverseConnector changes.
 **/
export const inverseConnectorDiff = (original: Connector, appliedDiff: ConnectorDiff): ConnectorDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking applyConnector changes.
 **/
export const applyConnectorDiff = (target: Connector, diff: ConnectorDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Connectors diff validation.
 **/
export const ConnectorsDiffSchema = z.object({
  removed: z.array(ConnectorIdSchema).optional(),
  updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Connectors changes.
 **/
export type ConnectorsDiff = z.infer<typeof ConnectorsDiffSchema>;
// ­ƒöîgetConnectorsDiff computes the diff between two connector collections.
const getConnectorsDiff = (before: Connector[], after: Connector[]): ConnectorsDiff => {
  const beforeIds = new Set(before.map((p) => p.id));
  const afterIds = new Set(after.map((p) => p.id));
  const removed = before.filter((p) => !afterIds.has(p.id)).map((p) => ({ id: p.id }));
  const added = after.filter((p) => !beforeIds.has(p.id));
  const updated = after
    .filter((p) => beforeIds.has(p.id))
    .map((afterPort) => {
      const beforePort = before.find((p) => p.id === afterPort.id)!;
      const diff = getConnectorDiff(beforePort, afterPort);
      return { connector: { id: afterPort.id }, diff };
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
export const findConnector = (connectors: Connector[], connectorId: string): Connector => {
  const connector = connectors.find((p) => p.id === connectorId);
  if (!connector) throw new Error(`Connector ${connectorId} not found in connectors`);
  return connector;
};

// #endregion ­ƒöîConnector

// #region ­ƒº▒Type
// Type entity types, schemas, and helpers MUST be defined here.

/** Lifecycle for tombstones and collaborative conflict detection. */
export type EntityLifecycle = "active" | "deleted";

/**
 * Zod schema for Type validation.
 **/
export const TypeSchema = z.object({
  id: z.string(),
  name: z.string(),
  families: z.array(FamilyIdSchema).optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  representations: z.array(RepresentationSchema).optional(),
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
  lifecycle: z.enum(["active", "deleted"]).optional(),
  deletedByUserId: z.string().optional(),
  deletedByDisplayName: z.string().optional(),
  deletedAt: z.string().optional(),
  deletedInChangeId: z.string().optional(),
});
/**
 * Type alias for Type.
 **/
export type TypePlain = z.infer<typeof TypeSchema>;
export class Type {
  id!: string;
  name!: string;
  families?: FamilyId[];
  isAbstract?: boolean;
  folder?: string;
  representations?: Representation[];
  connectors?: Connector[];
  props?: Prop[];
  stock?: number;
  virtual?: boolean;
  unit?: string;
  createdAt?: string;
  updatedAt?: string;
  location?: LocationId;
  authors?: AuthorId[];
  concepts?: ConceptId[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  lifecycle?: EntityLifecycle;
  deletedByUserId?: string;
  deletedByDisplayName?: string;
  deletedAt?: string;
  deletedInChangeId?: string;

  /** True private field ÔÇö avoids enumerable kit Ôåö types cycles in diffs and deep equality. */
  #kit?: KitImpl;

  constructor(plain: TypePlain, kit?: KitImpl) {
    const p = TypeSchema.parse(plain);
    Object.assign(this, p);
    this.representations = p.representations?.map((m) => new Representation(m));
    this.connectors = p.connectors?.map((c) => new Connector(c));
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
    if (kit !== undefined && !(kit instanceof KitImpl)) throw new Error("Type must be wired to a KitImpl class instance");
    this.#kit = kit;
  }

  static from(plain: TypePlain, kit?: KitImpl): Type {
    return new Type(plain, kit);
  }

  /** Pick a representative representation for the given tag ids (UI / scene helpers). */
  static pickBestRepresentation(representations: Representation[], tagIds: string[]): Representation | undefined {
    return selectBestRepresentation(representations, tagIds);
  }

  // #region Ô£Å´©ÅMethods
  /**
   * ­ƒôøRename this type via the kit graph pipeline
   */
  rename(newName: string): KitGraphChange {
    if (!this.#kit) throw new Error("Type not attached to a KitImpl");
    const diff: KitDiff = {
      types: {
        updated: [
          {
            type: { id: this.id },
            diff: { name: newName },
          },
        ],
      },
    };
    return this.#kit._applyDiff(diff);
  }

  /** Semantic restore of a tombstoned type (first-class command). */
  restore(opts?: KitChangeOptions): KitGraphChange {
    if (!this.#kit) throw new Error("Type not attached to a KitImpl");
    return this.#kit.restoreType(this, opts);
  }

  /**
   * ­ƒùæ´©ÅDelete this type via the kit graph pipeline
   */
  delete(opts?: KitChangeOptions): KitGraphChange {
    if (!this.#kit) throw new Error("Type not attached to a KitImpl");
    return this.#kit.removeType(this, opts);
  }

  /**
   * ­ƒöìFind connector by id
   */
  findConnector(connectorId: string): Connector | undefined {
    return this.connectors?.find((c) => c.id === connectorId);
  }

  getKit(): KitImpl | undefined {
    return this.#kit;
  }

  getTypeFamily(): Type[] {
    if (!this.#kit) throw new Error("Type not attached to a KitImpl");
    return asKitInstance(this.#kit).getTypeFamilyFor(this.id);
  }

  isInSameFamilyAsType(otherTypeId: string): boolean {
    if (!this.#kit) throw new Error("Type not attached to a KitImpl");
    return asKitInstance(this.#kit).areTypesInSameFamily(this.id, otherTypeId);
  }
  // #endregion Ô£Å´©ÅMethods

  /** ­ƒôªSerialize this type for wire transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  /** ­ƒº¡Deserialize a wire type into a stateful instance. */
  static deserialize(json: string, kit?: KitImpl): Type {
    return new Type(TypeSchema.parse(JSON.parse(json)), kit);
  }

  toPlain(): TypePlain {
    return TypeSchema.parse({
      ...(this as unknown as TypePlain),
    });
  }

  /** ­ƒ¬¬Project this type into its metadata wire shape. */
  toMeta(): TypeMeta {
    return TypeMetaSchema.parse(this.toPlain());
  }

  /** ­ƒº¥Project this type into its shallow wire shape. */
  toShallow(): TypeShallow {
    const plain = this.toPlain();
    return TypeShallowSchema.parse({
      ...plain,
      representations: this.representations?.map((m) => RepresentationMetaSchema.parse(m.toPlain())),
      connectors: this.connectors?.map((c) => ConnectorMetaSchema.parse(c.toPlain())),
      props: this.props?.map((p) => PropMetaSchema.parse(p.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetaSchema.parse(a.toPlain())),
    });
  }

  /** ­ƒº▒Compute a type delta from this type to another type. */
  diffTo(after: Type): TypeDiff {
    const diff: TypeDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (!arraysEqual(this.families, after.families)) diff.families = after.families ?? null;
    if (this.isAbstract !== after.isAbstract) diff.isAbstract = after.isAbstract;
    if (this.folder !== after.folder) diff.folder = after.folder ?? null;
    if (this.stock !== after.stock) diff.stock = after.stock;
    if (this.virtual !== after.virtual) diff.virtual = after.virtual;
    if (this.unit !== after.unit) diff.unit = after.unit;
    if (this.createdAt !== after.createdAt) diff.createdAt = after.createdAt;
    if (this.updatedAt !== after.updatedAt) diff.updatedAt = after.updatedAt;
    if (this.location?.id !== after.location?.id) diff.location = after.location ?? null;
    if (this.icon !== after.icon) diff.icon = after.icon ?? null;
    if (this.image !== after.image) diff.image = after.image ?? null;
    if (this.description !== after.description) diff.description = after.description ?? null;
    if ((this.lifecycle ?? "active") !== (after.lifecycle ?? "active")) diff.lifecycle = after.lifecycle ?? "active";
    if (this.deletedByUserId !== after.deletedByUserId) diff.deletedByUserId = after.deletedByUserId ?? null;
    if (this.deletedByDisplayName !== after.deletedByDisplayName) diff.deletedByDisplayName = after.deletedByDisplayName ?? null;
    if (this.deletedAt !== after.deletedAt) diff.deletedAt = after.deletedAt ?? null;
    if (this.deletedInChangeId !== after.deletedInChangeId) diff.deletedInChangeId = after.deletedInChangeId ?? null;
    if (JSON.stringify(this.authors) !== JSON.stringify(after.authors)) diff.authors = after.authors ?? null;
    if (JSON.stringify(this.concepts) !== JSON.stringify(after.concepts)) diff.concepts = after.concepts ?? null;
    if (!deepEqual(this.representations, after.representations)) diff.representations = getCollectionDiff("representation", this.representations ?? [], after.representations ?? [], getRepresentationDiff);
    if (!deepEqual(this.connectors, after.connectors)) diff.connectors = getCollectionDiff("connector", this.connectors ?? [], after.connectors ?? [], getConnectorDiff);
    if (!deepEqual(this.props, after.props)) diff.props = getPropsDiff(this.props ?? [], after.props ?? []);
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }

  /** ­ƒº▒Build the reverse type delta for an already-applied delta. */
  inverseDiff(appliedDiff: TypeDiff): TypeDiff {
    const inverse: TypeDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.families !== undefined) inverse.families = this.families ?? null;
    if (appliedDiff.isAbstract !== undefined) inverse.isAbstract = this.isAbstract;
    if (appliedDiff.folder !== undefined) inverse.folder = this.folder ?? null;
    if (appliedDiff.stock !== undefined) inverse.stock = this.stock;
    if (appliedDiff.virtual !== undefined) inverse.virtual = this.virtual;
    if (appliedDiff.unit !== undefined) inverse.unit = this.unit;
    if (appliedDiff.createdAt !== undefined) inverse.createdAt = this.createdAt;
    if (appliedDiff.updatedAt !== undefined) inverse.updatedAt = this.updatedAt;
    if (appliedDiff.location !== undefined) inverse.location = this.location ?? null;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon ?? null;
    if (appliedDiff.image !== undefined) inverse.image = this.image ?? null;
    if (appliedDiff.description !== undefined) inverse.description = this.description ?? null;
    if (appliedDiff.lifecycle !== undefined) inverse.lifecycle = this.lifecycle ?? "active";
    if (appliedDiff.deletedByUserId !== undefined) inverse.deletedByUserId = this.deletedByUserId ?? null;
    if (appliedDiff.deletedByDisplayName !== undefined) inverse.deletedByDisplayName = this.deletedByDisplayName ?? null;
    if (appliedDiff.deletedAt !== undefined) inverse.deletedAt = this.deletedAt ?? null;
    if (appliedDiff.deletedInChangeId !== undefined) inverse.deletedInChangeId = this.deletedInChangeId ?? null;
    if (appliedDiff.authors !== undefined) inverse.authors = this.authors ?? null;
    if (appliedDiff.concepts !== undefined) inverse.concepts = this.concepts ?? null;
    if (appliedDiff.representations) inverse.representations = inverseCollectionDiff("representation", this.representations ?? [], appliedDiff.representations, inverseRepresentationDiff);
    if (appliedDiff.connectors) inverse.connectors = inverseCollectionDiff("connector", this.connectors ?? [], appliedDiff.connectors, inverseConnectorDiff);
    if (appliedDiff.props) inverse.props = inversePropsDiff(this.props ?? [], appliedDiff.props);
    if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }

  /** ­ƒº▒Merge two type deltas. */
  static mergeDiff(first: TypeDiff, second: TypeDiff): TypeDiff {
    return {
      ...first,
      ...second,
      attributes: first.attributes || second.attributes ? mergeAttributesDiff(first.attributes ?? {}, second.attributes ?? {}) : undefined,
    };
  }

  /** ­ƒº▒Apply a type delta to this type. */
  applyDiff(diff: TypeDiff): void {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.families !== undefined) this.families = diff.families ?? undefined;
    if (diff.isAbstract !== undefined) this.isAbstract = diff.isAbstract;
    if (diff.createdAt !== undefined) this.createdAt = diff.createdAt;
    if (diff.updatedAt !== undefined) this.updatedAt = diff.updatedAt;
    if (diff.folder !== undefined) this.folder = diff.folder ?? undefined;
    if (diff.stock !== undefined) this.stock = diff.stock;
    if (diff.virtual !== undefined) this.virtual = diff.virtual;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.location !== undefined) this.location = diff.location ?? undefined;
    if (diff.icon !== undefined) this.icon = diff.icon ?? undefined;
    if (diff.image !== undefined) this.image = diff.image ?? undefined;
    if (diff.description !== undefined) this.description = diff.description ?? undefined;
    if (diff.lifecycle !== undefined) this.lifecycle = diff.lifecycle;
    if (diff.deletedByUserId !== undefined) this.deletedByUserId = diff.deletedByUserId ?? undefined;
    if (diff.deletedByDisplayName !== undefined) this.deletedByDisplayName = diff.deletedByDisplayName ?? undefined;
    if (diff.deletedAt !== undefined) this.deletedAt = diff.deletedAt ?? undefined;
    if (diff.deletedInChangeId !== undefined) this.deletedInChangeId = diff.deletedInChangeId ?? undefined;
    if (diff.authors !== undefined) this.authors = diff.authors ?? undefined;
    if (diff.concepts !== undefined) this.concepts = diff.concepts ?? undefined;
    if (diff.representations) {
      if (!this.representations) this.representations = [];
      applyCollectionDiff("representation", this.representations, diff.representations, applyRepresentationDiff, (raw) => new Representation(raw as RepresentationPlain));
    }
    if (diff.connectors) {
      if (!this.connectors) this.connectors = [];
      applyCollectionDiff("connector", this.connectors, diff.connectors, applyConnectorDiff, (raw) => new Connector(raw as ConnectorPlain));
    }
    if (diff.props) {
      if (!this.props) this.props = [];
      applyCollectionDiff("prop", this.props, diff.props, applyPropDiff, (raw) => new Prop(raw as PropPlain));
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Type for transport.
 **/
export const serializeType = (type: Type): string => type.serialize();
/**
 **/
export const deserializeType = (json: string): Type => Type.deserialize(json);

/**
 * Definition of TypeMetaSchema.
 **/
export const TypeMetaSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
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
export const TypeShallowSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true }).extend({
  representations: z.array(RepresentationMetaSchema).optional(),
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
  .omit({ representations: true, connectors: true, props: true, attributes: true })
  .extend({
    representations: RepresentationsDiffSchema.optional(),
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
    families: z.array(FamilyIdSchema).nullable().optional(),
    lifecycle: z.enum(["active", "deleted"]).optional(),
    deletedByUserId: z.string().nullable().optional(),
    deletedByDisplayName: z.string().nullable().optional(),
    deletedAt: z.string().nullable().optional(),
    deletedInChangeId: z.string().nullable().optional(),
  });
/**
 **/
export type TypeDiff = z.infer<typeof TypeDiffSchema>;

/**
 * Retrieves the TypeDiff value.
 **/
export const getTypeDiff = (before: Type, after: Type): TypeDiff => {
  return before.diffTo(after);
};

/**
 * Diff type for tracking applyType changes.
 **/
export const applyTypeDiff = (target: Type, diff: TypeDiff): void => {
  target.applyDiff(diff);
};

/**
 * Diff type for tracking mergeType changes.
 **/
export const mergeTypeDiff = (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => {
  return Type.mergeDiff(diff1, diff2);
};

/**
 * Diff type for tracking inverseType changes.
 **/
export const inverseTypeDiff = (original: Type, appliedDiff: TypeDiff): TypeDiff => {
  return original.inverseDiff(appliedDiff);
};

/**
 * Zod schema for Types diff validation.
 **/
export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Types changes.
 **/
export type TypesDiff = z.infer<typeof TypesDiffSchema>;

/**
 * Searches for matching ConnectorInType entry.
 **/
export const findConnectorInType = (type: Type, connectorId: string): Connector => findConnector(type.connectors ?? [], connectorId);

// #endregion ­ƒº▒Type

// #region ­ƒÄ¿Layer
// Layer entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Layer validation.
 **/
export const LayerSchema = z.object({
  id: z.string(),
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
export type LayerPlain = z.infer<typeof LayerSchema>;
export class Layer implements LayerPlain {
  id!: string;
  path!: string;
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
  description?: string;
  attributes?: Attribute[];
  constructor(plain: LayerPlain) {
    const p = LayerSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: LayerPlain): Layer {
    return new Layer(plain);
  }
  toPlain(): LayerPlain {
    return LayerSchema.parse(this as unknown as LayerPlain);
  }
  /** ­ƒÄ¿Serialize this layer for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒÄ¿Deserialize a layer from transport JSON. */
  static deserialize(json: string): Layer {
    return new Layer(LayerSchema.parse(JSON.parse(json)));
  }
  /** ­ƒÄ¿Compute a layer delta from this layer to another layer. */
  diffTo(after: Layer): LayerDiff {
    const diff: LayerDiff = {};
    if (this.path !== after.path) diff.path = after.path;
    if (this.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
    if (this.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
    if (this.color !== after.color) diff.color = after.color;
    if (this.description !== after.description) diff.description = after.description;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }
  /** ­ƒÄ¿Build the reverse layer delta for an already-applied delta. */
  inverseDiff(appliedDiff: LayerDiff): LayerDiff {
    const inverse: LayerDiff = {};
    if (appliedDiff.path !== undefined) inverse.path = this.path;
    if (appliedDiff.isHidden !== undefined) inverse.isHidden = this.isHidden;
    if (appliedDiff.isLocked !== undefined) inverse.isLocked = this.isLocked;
    if (appliedDiff.color !== undefined) inverse.color = this.color;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒÄ¿Merge two layer deltas. */
  static mergeDiff(first: LayerDiff, second: LayerDiff): LayerDiff {
    return { ...first, ...second, attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes) };
  }
  /** ­ƒÄ¿Apply a layer delta to this layer. */
  applyDiff(diff: LayerDiff): void {
    if (diff.path !== undefined) this.path = diff.path;
    if (diff.isHidden !== undefined) this.isHidden = diff.isHidden;
    if (diff.isLocked !== undefined) this.isLocked = diff.isLocked;
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
/**
 * Serializes Layer for transport.
 **/
export const serializeLayer = (layer: Layer): string => layer.serialize();
/**
 **/
export const deserializeLayer = (json: string): Layer => Layer.deserialize(json);

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseLayer changes.
 **/
export const inverseLayerDiff = (original: Layer, appliedDiff: LayerDiff): LayerDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergeLayer changes.
 **/
export const mergeLayerDiff = (diff1: LayerDiff, diff2: LayerDiff): LayerDiff => {
  return Layer.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyLayer changes.
 **/
export const applyLayerDiff = (target: Layer, diff: LayerDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Layers diff validation.
 **/
export const LayersDiffSchema = z.object({
  removed: z.array(LayerIdSchema).optional(),
  updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Layers changes.
 **/
export type LayersDiff = z.infer<typeof LayersDiffSchema>;

// #endregion ­ƒÄ¿Layer

// #region ­ƒº®Piece
// Piece entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Piece validation.
 **/
export const PieceSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  type: TypeIdSchema.optional(),
  design: DesignIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: CoordinateSchema.optional(),
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
export type PiecePlain = z.infer<typeof PieceSchema>;
export class Piece {
  id!: string;
  name?: string;
  plane?: Plane;
  center?: Coordinate;
  scale?: number;
  mirrorPlane?: Plane;
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
  description?: string;
  props?: Prop[];
  attributes?: Attribute[];

  /** Wire ids; resolved objects live in {@link Piece.type} / {@link Piece.design}. */
  #typeId?: string;
  #designAsPieceId?: string;
  #typeObj?: Type;
  #designAsPieceObj?: Design;

  /** Not enumerable ÔÇö avoids JSON / diff cycles (piece Ôåö design Ôåö kit). */
  #hostDesign?: Design;
  #hostKit?: KitImpl;

  /** Semio {@link Type} for this piece (kit graph pointer, not an id blob). */
  get type(): Type | undefined {
    if (this.#typeObj !== undefined) return this.#typeObj;
    const g = this.#typeId;
    if (!g) return undefined;
    const k = this.#resolveKit();
    if (!k) return undefined;
    this.#typeObj = k.findType(g);
    return this.#typeObj;
  }

  /** Nested design when this piece is a design-reference (kit graph pointer). */
  get design(): Design | undefined {
    if (this.#designAsPieceObj !== undefined) return this.#designAsPieceObj;
    const g = this.#designAsPieceId;
    if (!g) return undefined;
    const k = this.#resolveKit();
    if (!k) return undefined;
    this.#designAsPieceObj = k.findDesign(g);
    return this.#designAsPieceObj;
  }

  #resolveKit(): KitImpl | undefined {
    return this.#hostKit ?? this.#hostDesign?.getKit();
  }

  constructor(plain: PiecePlain, hostDesign?: Design, hostKit?: KitImpl) {
    const p = PieceSchema.parse(plain);
    this.#typeId = p.type?.id;
    this.#designAsPieceId = p.design?.id;
    const { type: _pt, design: _pd, ...rest } = p;
    Object.assign(this, rest);
    this.plane = p.plane ? new Plane(p.plane) : undefined;
    this.center = p.center ? new Coordinate(p.center) : undefined;
    this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined;
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
    this.#hostDesign = hostDesign;
    const wireKit = hostKit ?? hostDesign?.getKit();
    if (wireKit !== undefined && !(wireKit instanceof KitImpl)) throw new Error("Piece must be wired to a KitImpl class instance");
    this.#hostKit = wireKit;
    const k = this.#resolveKit();
    if (k) {
      if (this.#typeId) this.#typeObj = k.findType(this.#typeId);
      if (this.#designAsPieceId) this.#designAsPieceObj = k.findDesign(this.#designAsPieceId);
    }
  }

  /** Type id for diffs / JSON (no object pointer on the wire). */
  wireTypeId(): TypeId | undefined {
    const t = this.type;
    if (t) return { id: t.id };
    return this.#typeId ? { id: this.#typeId } : undefined;
  }

  /** Nested design id for diffs / JSON. */
  wireDesignAsPieceId(): DesignId | undefined {
    const d = this.design;
    if (d) return { id: d.id };
    return this.#designAsPieceId ? { id: this.#designAsPieceId } : undefined;
  }

  syncTypeFromWire(id: TypeId | null | undefined): void {
    this.#typeId = id?.id;
    this.#typeObj = undefined;
    const k = this.#resolveKit();
    if (id && k) this.#typeObj = k.findType(id.id);
  }

  syncDesignAsPieceFromWire(id: DesignId | null | undefined): void {
    this.#designAsPieceId = id?.id;
    this.#designAsPieceObj = undefined;
    const k = this.#resolveKit();
    if (id && k) this.#designAsPieceObj = k.findDesign(id.id);
  }

  /**
   * Lazy flattened placement: fills the kit geometry cache via {@link KitImpl.ensureFlattenGeometryCache} (no persist diff).
   */
  flatPlane(): Plane | undefined {
    const design = this.#hostDesign;
    const kit = this.#resolveKit();
    if (!design || !kit) return undefined;
    kit.ensureFlattenGeometryCache(design.id);
    return kit.getFlattenMerkleCache(design.id)?.[this.id]?.plane;
  }

  /**
   * Lazy flattened 2D center in host-design space (see {@link flatPlane}; no persist diff).
   */
  flatCenter(): Coordinate | undefined {
    const design = this.#hostDesign;
    const kit = this.#resolveKit();
    if (!design || !kit) return undefined;
    kit.ensureFlattenGeometryCache(design.id);
    return kit.getFlattenMerkleCache(design.id)?.[this.id]?.center;
  }

  getHostDesign(): Design | undefined {
    return this.#hostDesign;
  }

  /**
   * Deletes this piece and stale connections, and fixes child pieces (see {@link Design.deletePiecesAndConnectionsDiff}).
   */
  delete(opts?: KitChangeOptions): KitGraphChange {
    const design = this.#hostDesign;
    const kit = this.#hostKit ?? design?.getKit();
    if (!design || !kit) throw new Error("Piece not attached to a Design/KitImpl");
    return design.deletePieces(this, opts);
  }

  /** Compatible replacement {@link Type}s for this piece (connector-aware), via the host design. */
  alternativeTypes(): Type[] {
    const design = this.#hostDesign;
    if (!design) throw new Error("Piece not attached to a Design");
    return design.findReplacableTypesForPiece(this.id);
  }

  /** Change this pieceÔÇÖs type (validated kit pipeline; respects active/open transaction). */
  changeType(type: Type, opts?: KitChangeOptions): KitGraphChange {
    const design = this.#hostDesign;
    const kit = this.#hostKit ?? design?.getKit();
    if (!design || !kit) throw new Error("Piece not attached to a Design/KitImpl");
    const diff: KitDiff = {
      designs: {
        updated: [
          {
            design: { id: design.id },
            diff: {
              pieces: {
                updated: [{ piece: { id: this.id }, diff: { type: { id: type.id } } }],
              },
            },
          },
        ],
      },
    };
    return kit._applyDiff(diff, opts ?? {});
  }

  static from(plain: PiecePlain, hostDesign?: Design, hostKit?: KitImpl): Piece {
    return new Piece(plain, hostDesign, hostKit);
  }

  /** ­ƒº«Compute the replacement-style piece delta from this piece to another piece. */
  diffTo(after: Piece): PieceDiff {
    const beforeTypeId = this.wireTypeId();
    const afterTypeId = after.wireTypeId();
    const beforeDesignAsPieceId = this.wireDesignAsPieceId();
    const afterDesignAsPieceId = after.wireDesignAsPieceId();
    const diff: PieceDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (beforeTypeId?.id !== afterTypeId?.id) diff.type = afterTypeId;
    if (beforeDesignAsPieceId?.id !== afterDesignAsPieceId?.id) diff.design = afterDesignAsPieceId;
    if (!deepEqual(this.plane, after.plane)) diff.plane = after.plane ? thisPlaneOrIdentity(this.plane).diffTo(after.plane) : undefined;
    if (!deepEqual(this.center, after.center)) diff.center = after.center;
    if (this.scale !== after.scale) diff.scale = after.scale;
    if (!deepEqual(this.mirrorPlane, after.mirrorPlane)) diff.mirrorPlane = after.mirrorPlane;
    if (this.isHidden !== after.isHidden) diff.isHidden = after.isHidden;
    if (this.isLocked !== after.isLocked) diff.isLocked = after.isLocked;
    if (this.color !== after.color) diff.color = after.color;
    if (this.description !== after.description) diff.description = after.description;
    if (!deepEqual(this.props, after.props)) diff.props = getPropsDiff(this.props ?? [], after.props ?? []);
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }

  /** Ôå®´©ÅCompute the inverse piece delta for an already-applied delta. */
  inverseDiff(appliedDiff: PieceDiff): PieceDiff {
    const inverse: PieceDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.type !== undefined) inverse.type = this.wireTypeId();
    if (appliedDiff.design !== undefined) inverse.design = this.wireDesignAsPieceId();
    if (appliedDiff.plane !== undefined) inverse.plane = this.plane;
    if (appliedDiff.center !== undefined) inverse.center = this.center;
    if (appliedDiff.scale !== undefined) inverse.scale = this.scale;
    if (appliedDiff.mirrorPlane !== undefined) inverse.mirrorPlane = this.mirrorPlane;
    if (appliedDiff.isHidden !== undefined) inverse.isHidden = this.isHidden;
    if (appliedDiff.isLocked !== undefined) inverse.isLocked = this.isLocked;
    if (appliedDiff.color !== undefined) inverse.color = this.color;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.props !== undefined) inverse.props = inversePropsDiff(this.props ?? [], appliedDiff.props);
    if (appliedDiff.attributes !== undefined) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }

  /** ­ƒº«Merge two piece deltas. */
  static mergeDiff(first: PieceDiff, second: PieceDiff): PieceDiff {
    return {
      ...first,
      ...second,
      props: first.props && second.props ? mergePropsDiff(first.props, second.props) : (second.props ?? first.props),
      attributes: first.attributes && second.attributes ? mergeAttributesDiff(first.attributes, second.attributes) : (second.attributes ?? first.attributes),
    };
  }

  /** Ô£ì´©ÅApply a piece delta in place. */
  applyDiff(diff: PieceDiff): void {
    if (diff.plane) {
      const diffPlane = diff.plane as any;
      const looksLikeFullPlane = diffPlane.origin && diffPlane.xAxis && diffPlane.yAxis && typeof diffPlane.origin.x === "number" && typeof diffPlane.xAxis.x === "number" && typeof diffPlane.yAxis.x === "number";
      if (looksLikeFullPlane) {
        this.plane = new Plane(PlaneSchema.parse(diffPlane));
      } else {
        if (!this.plane)
          this.plane = new Plane({
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 },
          });
        this.plane.applyDiff(diff.plane);
      }
    }
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.type !== undefined) this.syncTypeFromWire(diff.type);
    if (diff.design !== undefined) this.syncDesignAsPieceFromWire(diff.design);
    if (diff.center !== undefined) this.center = diff.center instanceof Coordinate ? diff.center : new Coordinate(CoordinateSchema.parse(diff.center as CoordinatePlain));
    if (diff.scale !== undefined) this.scale = diff.scale;
    if (diff.mirrorPlane !== undefined) this.mirrorPlane = diff.mirrorPlane instanceof Plane ? diff.mirrorPlane : new Plane(PlaneSchema.parse(diff.mirrorPlane as PlanePlain));
    if (diff.isHidden !== undefined) this.isHidden = diff.isHidden;
    if (diff.isLocked !== undefined) this.isLocked = diff.isLocked;
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.props) {
      if (!this.props) this.props = [];
      applyPropsDiff(this.props, diff.props);
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }

  /** ­ƒôªSerialize this piece for wire transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  /** ­ƒº¡Deserialize a wire piece into a stateful instance. */
  static deserialize(json: string, hostDesign?: Design, hostKit?: KitImpl): Piece {
    return new Piece(PieceSchema.parse(JSON.parse(json)), hostDesign, hostKit);
  }

  toPlain(): PiecePlain {
    return PieceSchema.parse({
      ...(this as unknown as PiecePlain),
      type: this.wireTypeId(),
      design: this.wireDesignAsPieceId(),
    });
  }
}
/**
 * Serializes Piece for transport.
 **/
export const serializePiece = (piece: Piece): string => piece.serialize();
/**
 **/
export const deserializePiece = (json: string): Piece => Piece.deserialize(json);

/** Flatten helpers may run on detached {@link Piece} copies (no kit); resolve {@link Type} via id map when needed. */
const resolvePieceTypeForFlatten = (piece: Piece, getType: (typeId: string) => Type | undefined): Type | undefined => {
  const direct = piece.type;
  if (direct && Array.isArray((direct as Type).connectors)) return direct;
  const g = typeof piece.wireTypeId === "function" ? piece.wireTypeId()?.id : (piece as any).type?.id;
  return g ? getType(g) : undefined;
};

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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inversePiece changes.
 **/
export const inversePieceDiff = (original: Piece, appliedDiff: PieceDiff): PieceDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking mergePiece changes.
 **/
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => {
  return Piece.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking applyPiece changes.
 **/
export const applyPieceDiff = (target: Piece, diff: PieceDiff): void => {
  target.applyDiff(diff);
};

/**
 * Zod schema for Pieces diff validation.
 **/
export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Pieces changes.
 **/
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

/**
 * Retrieves the PieceRepresentationFileIds value.
 **/
export const getPieceRepresentationFileIds = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const representationFileIds = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.id === p.type!.id);
    if (!type) throw new Error(`Type ${p.type.id} for piece ${p.id} not found`);
    if (!type.representations) throw new Error(`Type ${p.type.id} for piece ${p.id} has no representations`);
    const representation = findRepresentation(type.representations, tags);
    representationFileIds.set(p.id, representation.file.id);
  });
  return representationFileIds;
};

/**
 * Retrieves the PieceRepresentationUrls value.
 **/
export const getPieceRepresentationUrls = (design: Design, types: Type[], files: File[], getFileUrl: (fileId: string) => string, tags: string[] = []): Map<string, string> => {
  const representationUrls = new Map<string, string>();
  toArray(design.pieces).forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.id === p.type!.id);
    if (!type) throw new Error(`Type ${p.type.id} for piece ${p.id} not found`);
    if (!type.representations) throw new Error(`Type ${p.type.id} for piece ${p.id} has no representations`);
    const representation = findRepresentation(type.representations, tags);
    const file = files.find((f) => f.id === representation.file.id);
    if (!file) throw new Error(`File ${representation.file.id} for representation ${representation.id} not found`);
    representationUrls.set(p.id, getFileUrl(file.id));
  });
  return representationUrls;
};
/**
 **/
export const fixPieceInDesign = (kit: KitLike, designId: string, pieceId: string): DesignDiff => asKitInstance(kit).fixPieceInDesignDiff(designId, pieceId);

/**
 **/
/**
 **/
export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined;
  const isCenterSet = piece.center !== undefined;
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.id} has inconsistent plane and center`);
  return isPlaneSet;
};

/**
 **/
/**
 * Searches for matching Piece entry.
 **/
export const findPiece = (pieces: Piece[], pieceId: string): Piece => {
  const piece = pieces.find((p) => p.id === pieceId);
  if (!piece) throw new Error(`Piece ${pieceId} not found in pieces`);
  return piece;
};

// #endregion ­ƒº®Piece

// #region ­ƒæÑGroup
// Group entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Group validation.
 **/
export const GroupSchema = z.object({
  id: z.string(),
  pieces: z.array(PieceIdSchema),
  color: z.string().optional(),
  name: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
/**
 * Type alias for Group.
 **/
export type GroupPlain = z.infer<typeof GroupSchema>;
export class Group implements GroupPlain {
  id!: string;
  pieces!: PieceId[];
  color?: string;
  name?: string;
  description?: string;
  attributes?: Attribute[];
  constructor(plain: GroupPlain) {
    const p = GroupSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(plain: GroupPlain): Group {
    return new Group(plain);
  }
  toPlain(): GroupPlain {
    return GroupSchema.parse(this as unknown as GroupPlain);
  }
  /** ­ƒæÑSerialize this group for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒæÑDeserialize a group from transport JSON. */
  static deserialize(json: string): Group {
    return new Group(GroupSchema.parse(JSON.parse(json)));
  }
  /** ­ƒæÑCompute a group delta from this group to another group. */
  diffTo(after: Group): GroupDiff {
    const diff: GroupDiff = {};
    if (!arraysEqual(this.pieces, after.pieces)) diff.pieces = after.pieces;
    if (this.color !== after.color) diff.color = after.color;
    if (this.name !== after.name) diff.name = after.name;
    if (this.description !== after.description) diff.description = after.description;
    const attributesDiff = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
    return diff;
  }
  /** ­ƒæÑBuild the reverse group delta for an already-applied delta. */
  inverseDiff(appliedDiff: GroupDiff): GroupDiff {
    const inverse: GroupDiff = {};
    if (appliedDiff.pieces !== undefined) inverse.pieces = this.pieces;
    if (appliedDiff.color !== undefined) inverse.color = this.color;
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }
  /** ­ƒæÑMerge two group deltas. */
  static mergeDiff(first: GroupDiff, second: GroupDiff): GroupDiff {
    return {
      ...first,
      ...second,
      attributes: first.attributes || second.attributes ? mergeAttributesDiff(first.attributes ?? {}, second.attributes ?? {}) : undefined,
    };
  }
  /** ­ƒæÑApply a group delta to this group. */
  applyDiff(diff: GroupDiff): void {
    if (diff.pieces !== undefined) this.pieces = diff.pieces;
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }
}
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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseGroup changes.
 **/
export const inverseGroupDiff = (original: Group, appliedDiff: GroupDiff): GroupDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking applyGroup changes.
 **/
export const applyGroupDiff = (target: Group, diff: GroupDiff): void => {
  target.applyDiff(diff);
};
/**
 * Diff type for tracking mergeGroup changes.
 **/
export const mergeGroupDiff = (diff1: GroupDiff, diff2: GroupDiff): GroupDiff => {
  return Group.mergeDiff(diff1, diff2);
};
/**
 * Zod schema for Groups diff validation.
 **/
export const GroupsDiffSchema = z.object({
  removed: z.array(GroupIdSchema).optional(),
  updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type GroupsDiff = z.infer<typeof GroupsDiffSchema>;
/**
 * Serializes Group for transport.
 **/
export const serializeGroup = (group: Group): string => group.serialize();
/**
 **/
export const deserializeGroup = (json: string): Group => Group.deserialize(json);

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

// #endregion ­ƒæÑGroup

// #region Ôåö´©ÅSide
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
export type SidePlain = z.infer<typeof SideSchema>;
export class Side {
  #pieceId!: string;
  #designPieceId?: string;
  #connectorId?: string;
  /** Owning design for resolving {@link Side.piece} / {@link Side.designPiece}. */
  #hostDesign?: Design;

  constructor(plain: SidePlain, hostDesign?: Design) {
    const p = SideSchema.parse(plain);
    this.#hostDesign = hostDesign;
    this.#pieceId = p.piece.id;
    this.#designPieceId = p.designPiece?.id;
    this.#connectorId = p.connector?.id;
  }

  /** Live piece endpoint (requires host design). */
  get piece(): Piece {
    const d = this.#hostDesign;
    if (!d) throw new Error("Side: missing host design (attach via Design connections or pass host into constructor)");
    const pc = d.findPiece(this.#pieceId);
    if (!pc) throw new Error(`Side: piece ${this.#pieceId} not found in design ${d.id}`);
    return pc;
  }

  /** Optional nested design-piece reference in the same design. */
  get designPiece(): Piece | undefined {
    if (!this.#designPieceId) return undefined;
    return this.#hostDesign?.findPiece(this.#designPieceId);
  }

  /** Wire connector id for export and diffs. */
  get connector(): ConnectorId | undefined {
    return this.#connectorId !== undefined ? { id: this.#connectorId } : undefined;
  }

  wirePieceId(): PieceId {
    return { id: this.#pieceId };
  }

  wireDesignPieceId(): PieceId | undefined {
    return this.#designPieceId ? { id: this.#designPieceId } : undefined;
  }

  syncPieceFromWire(id: PieceId | null | undefined): void {
    if (id?.id !== undefined) this.#pieceId = id.id;
  }

  syncDesignPieceFromWire(id: PieceId | null | undefined): void {
    this.#designPieceId = id?.id;
  }

  syncConnectorFromWire(id: ConnectorId | null | undefined): void {
    this.#connectorId = id?.id;
  }

  bindHostDesign(design: Design): void {
    this.#hostDesign = design;
  }

  /** Ôåö´©ÅCompute a side endpoint delta from this side to another side. */
  diffTo(after: Side): SideDiff {
    const diff: SideDiff = {};
    if (this.wirePieceId().id !== after.wirePieceId().id) diff.piece = after.wirePieceId();
    const beforeDesignPieceId = this.wireDesignPieceId()?.id;
    const afterDesignPieceId = after.wireDesignPieceId()?.id;
    if (beforeDesignPieceId !== afterDesignPieceId) diff.designPiece = after.wireDesignPieceId();
    if (this.connector?.id !== after.connector?.id) diff.connector = after.connector;
    return diff;
  }

  /** Ôåö´©ÅBuild the reverse side delta for an already-applied delta. */
  inverseDiff(appliedDiff: SideDiff): SideDiff {
    const inverse: SideDiff = {};
    if (appliedDiff.piece !== undefined) inverse.piece = this.wirePieceId();
    if (appliedDiff.designPiece !== undefined) inverse.designPiece = this.wireDesignPieceId();
    if (appliedDiff.connector !== undefined) inverse.connector = this.connector;
    return inverse;
  }

  /** Ôåö´©ÅApply a side delta to this side endpoint. */
  applyDiff(diff: SideDiff): void {
    if (diff.piece !== undefined) this.syncPieceFromWire(diff.piece);
    if (diff.designPiece !== undefined) this.syncDesignPieceFromWire(diff.designPiece);
    if (diff.connector !== undefined) this.syncConnectorFromWire(diff.connector);
  }

  /** Ôåö´©ÅSerialize this side endpoint for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  /** Ôåö´©ÅDeserialize a side endpoint from transport JSON. */
  static deserialize(json: string, hostDesign?: Design): Side {
    return new Side(SideSchema.parse(JSON.parse(json)), hostDesign);
  }

  static from(plain: SidePlain, hostDesign?: Design): Side {
    return new Side(plain, hostDesign);
  }

  toPlain(): SidePlain {
    return SideSchema.parse({
      piece: { id: this.#pieceId },
      designPiece: this.#designPieceId ? { id: this.#designPieceId } : undefined,
      connector: this.#connectorId ? { id: this.#connectorId } : undefined,
    });
  }
}
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
export type SideIdPlain = z.infer<typeof SideIdSchema>;
export class SideId implements SideIdPlain {
  piece!: PieceId;
  designPiece?: PieceId;
  connector?: ConnectorId;
  constructor(plain: SideIdPlain) {
    Object.assign(this, SideIdSchema.parse(plain));
  }
  static from(plain: SideIdPlain): SideId {
    return new SideId(plain);
  }
  toPlain(): SideIdPlain {
    return SideIdSchema.parse(this as unknown as SideIdPlain);
  }
}
/**
 * Zod schema for Sides diff validation.
 **/
export const SidesDiffSchema = z.object({
  removed: z.array(SideIdSchema).optional(),
  updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Sides changes.
 **/
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
/**
 * Retrieves the SideDiff value.
 **/
export const getSideDiff = (before: Side, after: Side): SideDiff => {
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseSide changes.
 **/
export const inverseSideDiff = (original: Side, appliedDiff: SideDiff): SideDiff => {
  return original.inverseDiff(appliedDiff);
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
export const applySideDiff = (target: Side, diff: SideDiff): void => {
  target.applyDiff(diff);
};
/**
 * Serializes Side for transport.
 **/
export const serializeSide = (side: Side): string => side.serialize();
/**
 **/
export const deserializeSide = (json: string): Side => Side.deserialize(json);
/**
 * Equality check for Side values.
 **/
export const areSameSide = (a: Side, b: Side): boolean => a.wirePieceId().id === b.wirePieceId().id && a.wireDesignPieceId()?.id === b.wireDesignPieceId()?.id && a.connector?.id === b.connector?.id;

// #endregion Ôåö´©ÅSide

// #region ­ƒöùConnection
// Connection entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Connection validation.
 **/
export const ConnectionSchema = z.object({
  id: z.string(),
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
export type ConnectionPlain = z.infer<typeof ConnectionSchema>;
export class Connection implements ConnectionPlain {
  id!: string;
  connected!: Side;
  connecting!: Side;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  u?: number;
  v?: number;
  description?: string;
  attributes?: Attribute[];
  #hostDesign?: Design;

  constructor(plain: ConnectionPlain, hostDesign?: Design) {
    const p = ConnectionSchema.parse(plain);
    Object.assign(this, p);
    this.#hostDesign = hostDesign;
    this.connected = new Side(p.connected, hostDesign);
    this.connecting = new Side(p.connecting, hostDesign);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  /** Design that owns this connection (used to resolve {@link Side.piece}). */
  getHostDesign(): Design | undefined {
    return this.#hostDesign;
  }

  rebindToDesign(design: Design): void {
    this.#hostDesign = design;
    this.connected.bindHostDesign(design);
    this.connecting.bindHostDesign(design);
  }

  /** ­ƒöùCompute a connection delta from this connection to another connection. */
  diffTo(after: Connection): ConnectionDiff {
    const diff: ConnectionDiff = {};
    if (!deepEqual(this.connected.toPlain(), after.connected.toPlain())) diff.connected = this.connected.diffTo(after.connected);
    if (!deepEqual(this.connecting.toPlain(), after.connecting.toPlain())) diff.connecting = this.connecting.diffTo(after.connecting);
    if (this.gap !== after.gap) diff.gap = after.gap !== undefined && this.gap !== undefined ? after.gap - this.gap : after.gap;
    if (this.shift !== after.shift) diff.shift = after.shift !== undefined && this.shift !== undefined ? after.shift - this.shift : after.shift;
    if (this.rise !== after.rise) diff.rise = after.rise !== undefined && this.rise !== undefined ? after.rise - this.rise : after.rise;
    if (this.rotation !== after.rotation) diff.rotation = after.rotation !== undefined && this.rotation !== undefined ? after.rotation - this.rotation : after.rotation;
    if (this.turn !== after.turn) diff.turn = after.turn !== undefined && this.turn !== undefined ? after.turn - this.turn : after.turn;
    if (this.tilt !== after.tilt) diff.tilt = after.tilt !== undefined && this.tilt !== undefined ? after.tilt - this.tilt : after.tilt;
    if (this.u !== after.u) diff.u = after.u !== undefined && this.u !== undefined ? after.u - this.u : after.u;
    if (this.v !== after.v) diff.v = after.v !== undefined && this.v !== undefined ? after.v - this.v : after.v;
    if (this.description !== after.description) diff.description = after.description;
    if (!deepEqual(this.attributes, after.attributes)) diff.attributes = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    return diff;
  }

  /** ­ƒöùBuild the reverse connection delta for an already-applied delta. */
  inverseDiff(appliedDiff: ConnectionDiff): ConnectionDiff {
    const inverse: ConnectionDiff = {};
    if (appliedDiff.connected !== undefined) inverse.connected = this.connected.inverseDiff(appliedDiff.connected);
    if (appliedDiff.connecting !== undefined) inverse.connecting = this.connecting.inverseDiff(appliedDiff.connecting);
    if (appliedDiff.gap !== undefined) inverse.gap = this.gap !== undefined && appliedDiff.gap !== undefined ? -appliedDiff.gap : this.gap;
    if (appliedDiff.shift !== undefined) inverse.shift = this.shift !== undefined && appliedDiff.shift !== undefined ? -appliedDiff.shift : this.shift;
    if (appliedDiff.rise !== undefined) inverse.rise = this.rise !== undefined && appliedDiff.rise !== undefined ? -appliedDiff.rise : this.rise;
    if (appliedDiff.rotation !== undefined) inverse.rotation = this.rotation !== undefined && appliedDiff.rotation !== undefined ? -appliedDiff.rotation : this.rotation;
    if (appliedDiff.turn !== undefined) inverse.turn = this.turn !== undefined && appliedDiff.turn !== undefined ? -appliedDiff.turn : this.turn;
    if (appliedDiff.tilt !== undefined) inverse.tilt = this.tilt !== undefined && appliedDiff.tilt !== undefined ? -appliedDiff.tilt : this.tilt;
    if (appliedDiff.u !== undefined) inverse.u = this.u !== undefined && appliedDiff.u !== undefined ? -appliedDiff.u : this.u;
    if (appliedDiff.v !== undefined) inverse.v = this.v !== undefined && appliedDiff.v !== undefined ? -appliedDiff.v : this.v;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.attributes !== undefined) {
      const appliedAttrs: Attribute[] = [];
      if (appliedDiff.attributes) applyAttributesDiff(appliedAttrs, appliedDiff.attributes);
      inverse.attributes = getAttributesDiff(appliedAttrs, this.attributes ?? []);
    }
    return inverse;
  }

  /** ­ƒöùApply a connection delta to this connection. */
  applyDiff(diff: ConnectionDiff): void {
    if (diff.connected) this.connected.applyDiff(diff.connected);
    if (diff.connecting) this.connecting.applyDiff(diff.connecting);
    if (diff.gap !== undefined) this.gap = this.gap !== undefined ? this.gap + diff.gap : diff.gap;
    if (diff.shift !== undefined) this.shift = this.shift !== undefined ? this.shift + diff.shift : diff.shift;
    if (diff.rise !== undefined) this.rise = this.rise !== undefined ? this.rise + diff.rise : diff.rise;
    if (diff.rotation !== undefined) this.rotation = this.rotation !== undefined ? this.rotation + diff.rotation : diff.rotation;
    if (diff.turn !== undefined) this.turn = this.turn !== undefined ? this.turn + diff.turn : diff.turn;
    if (diff.tilt !== undefined) this.tilt = this.tilt !== undefined ? this.tilt + diff.tilt : diff.tilt;
    if (diff.u !== undefined) this.u = this.u !== undefined ? this.u + diff.u : diff.u;
    if (diff.v !== undefined) this.v = this.v !== undefined ? this.v + diff.v : diff.v;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }

  /** ­ƒöùSerialize this connection for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  /** ­ƒöùDeserialize a connection from transport JSON. */
  static deserialize(json: string, hostDesign?: Design): Connection {
    return new Connection(ConnectionSchema.parse(JSON.parse(json)), hostDesign);
  }

  static from(plain: ConnectionPlain, hostDesign?: Design): Connection {
    return new Connection(plain, hostDesign);
  }

  toPlain(): ConnectionPlain {
    return ConnectionSchema.parse({
      id: this.id,
      connected: this.connected.toPlain(),
      connecting: this.connecting.toPlain(),
      gap: this.gap,
      shift: this.shift,
      rise: this.rise,
      rotation: this.rotation,
      turn: this.turn,
      tilt: this.tilt,
      u: this.u,
      v: this.v,
      description: this.description,
      attributes: this.attributes?.map((a) => a.toPlain()),
    } as unknown as ConnectionPlain);
  }
}
/**
 * Zod schema for Connection diff validation.
 **/
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ id: true, connected: true, connecting: true, attributes: true }).extend({
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
  return before.diffTo(after);
};

/**
 * Diff type for tracking applyConnection changes.
 **/
export const applyConnectionDiff = (target: Connection, diff: ConnectionDiff): void => {
  target.applyDiff(diff);
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
  return original.inverseDiff(appliedDiff);
};

/**
 * Zod schema for Connections diff validation.
 **/
export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Connections changes.
 **/
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
/**
 * Serializes Connection for transport.
 **/
export const serializeConnection = (connection: Connection): string => connection.serialize();
/**
 **/
export const deserializeConnection = (json: string): Connection => Connection.deserialize(json);

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
  const getConnectedPieceId = (conn: typeof connection) => ("connected" in conn && conn.connected && "piece" in conn.connected ? (typeof conn.connected.piece === "string" ? conn.connected.piece : (conn.connected.piece?.id ?? "")) : "");
  const getConnectingPieceId = (conn: typeof connection) => ("connecting" in conn && conn.connecting && "piece" in conn.connecting ? (typeof conn.connecting.piece === "string" ? conn.connecting.piece : (conn.connecting.piece?.id ?? "")) : "");

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
export const findConnection = (connections: Connection[], connectionId: string): Connection => {
  const connection = connections.find((c) => c.id === connectionId);
  if (!connection) throw new Error(`Connection ${connectionId} not found in connections`);
  return connection;
};

/**
 * Searches for matching PieceConnections entry.
 **/
export const findPieceConnections = (connections: Connection[], pieceId: string): Connection[] => {
  return connections.filter((c) => c.connected.piece.id === pieceId || c.connecting.piece.id === pieceId);
};

/**
 * Searches for matching ConnectorForPieceInConnection entry.
 **/
export const findConnectorForPieceInConnection = (type: Type, connection: Connection, pieceId: string): Connector | undefined => {
  const connectorId = connection.connected.piece.id === pieceId ? connection.connected.connector?.id : connection.connecting.connector?.id;
  if (!connectorId) return undefined;
  return findConnectorInType(type, connectorId);
};

// #endregion ­ƒöùConnection

// #region ­ƒôêStat
// Stat entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Stat validation.
 **/
export const StatSchema = z.object({
  id: z.string(),
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
export type StatPlain = z.infer<typeof StatSchema>;
export class Stat implements StatPlain {
  id!: string;
  quality!: QualityId;
  unit?: string;
  min?: number;
  minExcluded?: boolean;
  max?: number;
  maxExcluded?: boolean;
  constructor(plain: StatPlain) {
    Object.assign(this, StatSchema.parse(plain));
  }
  static from(plain: StatPlain): Stat {
    return new Stat(plain);
  }
  toPlain(): StatPlain {
    return StatSchema.parse(this as unknown as StatPlain);
  }
  /** ­ƒôêSerialize this stat for transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  /** ­ƒôêDeserialize a stat from transport JSON. */
  static deserialize(json: string): Stat {
    return new Stat(StatSchema.parse(JSON.parse(json)));
  }
  /** ­ƒôêCompute a stat delta from this stat to another stat. */
  diffTo(after: Stat): StatDiff {
    const diff: StatDiff = {};
    if (this.quality?.id !== after.quality?.id) diff.quality = after.quality;
    if (this.unit !== after.unit) diff.unit = after.unit;
    if (this.min !== after.min) diff.min = after.min;
    if (this.minExcluded !== after.minExcluded) diff.minExcluded = after.minExcluded;
    if (this.max !== after.max) diff.max = after.max;
    if (this.maxExcluded !== after.maxExcluded) diff.maxExcluded = after.maxExcluded;
    return diff;
  }
  /** ­ƒôêBuild the reverse stat delta for an already-applied delta. */
  inverseDiff(appliedDiff: StatDiff): StatDiff {
    const inverse: StatDiff = {};
    if (appliedDiff.quality !== undefined) inverse.quality = this.quality;
    if (appliedDiff.unit !== undefined) inverse.unit = this.unit;
    if (appliedDiff.min !== undefined) inverse.min = this.min;
    if (appliedDiff.minExcluded !== undefined) inverse.minExcluded = this.minExcluded;
    if (appliedDiff.max !== undefined) inverse.max = this.max;
    if (appliedDiff.maxExcluded !== undefined) inverse.maxExcluded = this.maxExcluded;
    return inverse;
  }
  /** ­ƒôêMerge two stat deltas. */
  static mergeDiff(first: StatDiff, second: StatDiff): StatDiff {
    return { ...first, ...second };
  }
  /** ­ƒôêApply a stat delta to this stat. */
  applyDiff(diff: StatDiff): void {
    if (diff.quality !== undefined) this.quality = diff.quality;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.min !== undefined) this.min = diff.min;
    if (diff.minExcluded !== undefined) this.minExcluded = diff.minExcluded;
    if (diff.max !== undefined) this.max = diff.max;
    if (diff.maxExcluded !== undefined) this.maxExcluded = diff.maxExcluded;
  }
}
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
  return before.diffTo(after);
};
/**
 * Diff type for tracking inverseStat changes.
 **/
export const inverseStatDiff = (original: Stat, appliedDiff: StatDiff): StatDiff => {
  return original.inverseDiff(appliedDiff);
};
/**
 * Diff type for tracking applyStat changes.
 **/
export const applyStatDiff = (target: Stat, diff: StatDiff): void => {
  target.applyDiff(diff);
};
/**
 * Diff type for tracking mergeStat changes.
 **/
export const mergeStatDiff = (diff1: StatDiff, diff2: StatDiff): StatDiff => {
  return Stat.mergeDiff(diff1, diff2);
};
/**
 * Zod schema for Stats diff validation.
 **/
export const StatsDiffSchema = z.object({
  removed: z.array(StatIdSchema).optional(),
  updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type StatsDiff = z.infer<typeof StatsDiffSchema>;
/**
 * Serializes Stat for transport.
 **/
export const serializeStat = (stat: Stat): string => stat.serialize();
/**
 **/
export const deserializeStat = (json: string): Stat => Stat.deserialize(json);

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

// #endregion ­ƒôêStat

// #region ­ƒôÉDesign
// Design entity types, schemas, and helpers MUST be defined here.

/**
 * Zod schema for Design validation.
 **/
export const DesignSchema = z.object({
  id: z.string(),
  name: z.string(),
  families: z.array(FamilyIdSchema).optional(),
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
export type DesignPlain = z.infer<typeof DesignSchema>;
export class Design {
  id!: string;
  name!: string;
  families?: FamilyId[];
  isAbstract?: boolean;
  folder?: string;
  pieces?: Piece[];
  /** @internal Graph storage; use {@link connections}() for the spec OO snapshot. */
  _connections?: Connection[];
  stats?: Stat[];
  props?: Prop[];
  layers?: Layer[];
  activeLayer?: LayerId;
  groups?: Group[];
  canScale?: boolean;
  canMirror?: boolean;
  unit?: string;
  location?: LocationId;
  authors?: AuthorId[];
  concepts?: ConceptId[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt?: string;
  updatedAt?: string;

  /** True private field ÔÇö avoids enumerable kit Ôåö designs cycles in diffs and deep equality. */
  #kit?: KitImpl;

  constructor(plain: DesignPlain | Design, kit?: KitImpl) {
    const wire: DesignPlain = plain instanceof Design ? plain.toPlain() : plain;
    const p = DesignSchema.parse(wire);
    if (kit !== undefined && !(kit instanceof KitImpl)) throw new Error("Design must be wired to a KitImpl class instance");
    const { connections: _wireConnections, pieces: _wirePieces, ...rest } = p;
    Object.assign(this, rest);
    this.#kit = kit;
    this.pieces = p.pieces?.map((x) => new Piece(x, this, kit));
    this._connections = p.connections?.map((x) => new Connection(x, this));
    this.stats = p.stats?.map((x) => new Stat(x));
    this.props = p.props?.map((x) => new Prop(x));
    this.layers = p.layers?.map((x) => new Layer(x));
    this.groups = p.groups?.map((x) => new Group(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static from(plain: DesignPlain, kit?: KitImpl): Design {
    return new Design(plain, kit);
  }

  /**
   * Non-mutating preview of applying a diff (clone wire state, then {@link applyDiff}).
   */
  static previewWithDiff(design: Design, diff: DesignDiff): Design {
    const raw = JSON.parse(JSON.stringify(design.toPlain()), (_k, v) => (v === null ? undefined : v)) as DesignPlain;
    const copy = new Design(raw, design.getKit());
    copy.applyDiff(diff);
    return copy;
  }

  // #region Ô£Å´©ÅMethods
  /**
   * ­ƒôøRename this design via the kit graph pipeline
   */
  rename(newName: string): KitGraphChange {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    const diff: KitDiff = {
      designs: {
        updated: [
          {
            design: { id: this.id },
            diff: { name: newName },
          },
        ],
      },
    };
    return this.#kit._applyDiff(diff);
  }

  /**
   * ­ƒùæ´©ÅRemove this design from the kit graph (validated pipeline).
   */
  delete(opts?: KitChangeOptions): KitGraphChange {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return this.#kit.removeDesign(this, opts);
  }

  getKit(): KitImpl | undefined {
    return this.#kit;
  }

  /**
   * Applies a {@link DesignDiff} in place (mutates this designÔÇÖs collections and scalars).
   */
  applyDiff(diff: DesignDiff): this {
    applyDesignDiffCore(this, diff);
    return this;
  }

  /** ­ƒôÉCompute a design delta from this design to another design. */
  diffTo(after: Design): DesignDiff {
    const diff: DesignDiff = {};
    if (this.name !== after.name) diff.name = after.name;
    if (!arraysEqual(this.families, after.families)) diff.families = after.families ?? null;
    if (this.isAbstract !== after.isAbstract) diff.isAbstract = after.isAbstract;
    if (this.folder !== after.folder) diff.folder = after.folder;
    if (this.canScale !== after.canScale) diff.canScale = after.canScale;
    if (this.canMirror !== after.canMirror) diff.canMirror = after.canMirror;
    if (this.unit !== after.unit) diff.unit = after.unit;
    if (this.activeLayer?.id !== after.activeLayer?.id) diff.activeLayer = after.activeLayer;
    if (this.location?.id !== after.location?.id) diff.location = after.location;
    if (this.icon !== after.icon) diff.icon = after.icon;
    if (this.image !== after.image) diff.image = after.image;
    if (this.description !== after.description) diff.description = after.description;
    if (!arraysEqual(this.authors, after.authors)) diff.authors = after.authors as any;
    if (!arraysEqual(this.concepts, after.concepts)) diff.concepts = after.concepts;
    const piecesDiff = getCollectionDiff("piece", this.pieces ?? [], after.pieces ?? [], getPieceDiff);
    if (Object.keys(piecesDiff).length > 0) diff.pieces = piecesDiff;
    const connectionsDiff = getCollectionDiff("connection", this._connections ?? [], after._connections ?? [], getConnectionDiff);
    if (Object.keys(connectionsDiff).length > 0) diff.connections = connectionsDiff;
    const statsDiff = getCollectionDiff("stat", this.stats ?? [], after.stats ?? [], getStatDiff);
    if (Object.keys(statsDiff).length > 0) diff.stats = statsDiff;
    const propsDiff = getCollectionDiff("prop", this.props ?? [], after.props ?? [], getPropDiff);
    if (Object.keys(propsDiff).length > 0) diff.props = propsDiff;
    const layersDiff = getCollectionDiff("layer", this.layers ?? [], after.layers ?? [], getLayerDiff);
    if (Object.keys(layersDiff).length > 0) diff.layers = layersDiff;
    const groupsDiff = getCollectionDiff("group", this.groups ?? [], after.groups ?? [], getGroupDiff);
    if (Object.keys(groupsDiff).length > 0) diff.groups = groupsDiff;
    const attributesDiff = getAttributesDiff(this.attributes ?? [], after.attributes ?? []);
    if (Object.keys(attributesDiff).length > 0) diff.attributes = attributesDiff;
    return diff;
  }

  /** ­ƒôÉBuild the reverse design delta for an already-applied delta. */
  inverseDiff(appliedDiff: DesignDiff): DesignDiff {
    const inverse: DesignDiff = {};
    if (appliedDiff.name !== undefined) inverse.name = this.name;
    if (appliedDiff.families !== undefined) inverse.families = this.families ?? null;
    if (appliedDiff.isAbstract !== undefined) inverse.isAbstract = this.isAbstract;
    if (appliedDiff.folder !== undefined) inverse.folder = this.folder;
    if (appliedDiff.canScale !== undefined) inverse.canScale = this.canScale;
    if (appliedDiff.canMirror !== undefined) inverse.canMirror = this.canMirror;
    if (appliedDiff.unit !== undefined) inverse.unit = this.unit;
    if (appliedDiff.activeLayer !== undefined) inverse.activeLayer = this.activeLayer;
    if (appliedDiff.location !== undefined) inverse.location = this.location;
    if (appliedDiff.icon !== undefined) inverse.icon = this.icon;
    if (appliedDiff.image !== undefined) inverse.image = this.image;
    if (appliedDiff.description !== undefined) inverse.description = this.description;
    if (appliedDiff.authors !== undefined) inverse.authors = this.authors as any;
    if (appliedDiff.concepts !== undefined) inverse.concepts = this.concepts;
    if (appliedDiff.pieces) inverse.pieces = inverseCollectionDiff("piece", this.pieces ?? [], appliedDiff.pieces, inversePieceDiff);
    if (appliedDiff.connections) inverse.connections = inverseCollectionDiff("connection", this._connections ?? [], appliedDiff.connections, inverseConnectionDiff);
    if (appliedDiff.stats) inverse.stats = inverseCollectionDiff("stat", this.stats ?? [], appliedDiff.stats, inverseStatDiff);
    if (appliedDiff.props) inverse.props = inverseCollectionDiff("prop", this.props ?? [], appliedDiff.props, inversePropDiff);
    if (appliedDiff.layers) inverse.layers = inverseCollectionDiff("layer", this.layers ?? [], appliedDiff.layers, inverseLayerDiff);
    if (appliedDiff.groups) inverse.groups = inverseCollectionDiff("group", this.groups ?? [], appliedDiff.groups, inverseGroupDiff);
    if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(this.attributes ?? [], appliedDiff.attributes);
    return inverse;
  }

  /** ­ƒôÉMerge two design deltas. */
  static mergeDiff(first: DesignDiff, second: DesignDiff): DesignDiff {
    return {
      ...first,
      ...second,
      pieces: first.pieces || second.pieces ? mergeCollectionDiff("piece", first.pieces ?? {}, second.pieces ?? {}, mergePieceDiff) : undefined,
      connections: first.connections || second.connections ? mergeCollectionDiff("connection", first.connections ?? {}, second.connections ?? {}, mergeConnectionDiff) : undefined,
      stats: first.stats || second.stats ? mergeCollectionDiff("stat", first.stats ?? {}, second.stats ?? {}, mergeStatDiff) : undefined,
      props: first.props || second.props ? mergeCollectionDiff("prop", first.props ?? {}, second.props ?? {}, mergePropDiff) : undefined,
      layers: first.layers || second.layers ? mergeCollectionDiff("layer", first.layers ?? {}, second.layers ?? {}, mergeLayerDiff) : undefined,
      groups: first.groups || second.groups ? mergeCollectionDiff("group", first.groups ?? {}, second.groups ?? {}, mergeGroupDiff) : undefined,
      authors: second.authors ?? first.authors,
      attributes: first.attributes || second.attributes ? mergeAttributesDiff(first.attributes ?? {}, second.attributes ?? {}) : undefined,
    };
  }

  /**
   * ­ƒùæ´©ÅDelete pieces from this design (and stale connections; fixes children that become fixed).
   */
  deletePieces(pieces: Piece | readonly Piece[], opts?: KitChangeOptions): KitGraphChange {
    const list = (Array.isArray(pieces) ? pieces : [pieces]) as Piece[];
    if (list.length === 0) throw new Error("deletePieces: pass at least one Piece");
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    const pieceIds = list.map((p) => p.id);
    const result = this.deletePiecesAndConnectionsDiff(pieceIds, []);
    if (!result.ok || !result.diff) {
      throw new Error(`Delete pieces failed: ${result.errors.map((e) => e.message).join("; ")}`);
    }
    const kitDiff: KitDiff = {
      designs: {
        updated: [{ design: { id: this.id }, diff: result.diff }],
      },
    };
    return this.#kit._applyDiff(kitDiff, opts ?? {});
  }

  /**
   * Persist a full flatten to the kit (forward {@link DesignDiff}, removes connections): the rare explicit layout commit.
   * For rendering / hit-testing, use {@link Piece.flatPlane}, {@link Piece.flatCenter}, {@link KitImpl.ensureFlattenGeometryCache}, or {@link KitImpl.piecesMetadataFor} instead ÔÇö those update only the in-memory geometry cache without building a persist diff.
   */
  flatten(opts?: KitChangeOptions): KitGraphChange {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    const op = this.runFlattenOptimized();
    if (!op.ok || !op.diff) {
      throw new Error(`flatten failed: ${op.errors.map((e) => e.message).join("; ")}`);
    }
    const kitDiff: KitDiff = {
      designs: {
        updated: [{ design: { id: this.id }, diff: op.diff.forward }],
      },
    };
    return this.#kit._applyDiff(kitDiff, opts ?? {});
  }

  /**
   * Computes a {@link DesignDiff} that removes pieces and connections, prunes stale links, and fixes pieces that become fixed (flattened plane/center). Does not mutate the kit.
   */
  deletePiecesAndConnectionsDiff(pieceIds: Id[], connectionIds: Id[]): DesignDiffOperationResult {
    const kit = this.#kit;
    if (!kit) return operationErr([{ message: "Design not attached to a KitImpl" }]);
    const deletedPieceSet = new Set(pieceIds);
    const connections = this._connections ?? [];

    const staleConnectionIds = new Set<string>();
    for (const conn of connections) {
      if (deletedPieceSet.has(conn.connected.piece.id) || deletedPieceSet.has(conn.connecting.piece.id)) {
        staleConnectionIds.add(conn.id);
      }
    }

    const allRemovedConnectionIds = new Set([...connectionIds, ...staleConnectionIds]);

    const fixedPieceIds: string[] = [];
    for (const connId of allRemovedConnectionIds) {
      const conn = connections.find((c) => c.id === connId);
      if (!conn) continue;
      const connectingId = conn.connecting.piece.id;
      if (deletedPieceSet.has(connectingId)) continue;
      const hasOtherParent = connections.some((c) => c.connecting.piece.id === connectingId && !allRemovedConnectionIds.has(c.id));
      if (!hasOtherParent && !fixedPieceIds.includes(connectingId)) {
        fixedPieceIds.push(connectingId);
      }
    }

    const flatRes = flattenDesignOptimizedForKit(kit, this.id);
    if (!flatRes.ok) {
      return operationErr(flatRes.errors);
    }
    const flatChange = flatRes.diff!;
    const flatPieceMap: { [id: string]: { plane?: Plane; center?: Coordinate } } = {};
    for (const piece of this.pieces ?? []) {
      if (piece.plane) flatPieceMap[piece.id] = { plane: piece.plane, center: piece.center };
    }
    for (const update of flatChange.forward.pieces?.updated ?? []) {
      const existing = flatPieceMap[update.piece.id] ?? {};
      if (update.diff.plane) existing.plane = update.diff.plane as Plane;
      if (update.diff.center) existing.center = update.diff.center as Coordinate;
      flatPieceMap[update.piece.id] = existing;
    }

    const identityPlane = new Plane({ origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } });
    const zeroCenter = new Coordinate({ u: 0, v: 0 });

    const diff: DesignDiff = {};

    const piecesRemoved = pieceIds.map((id) => ({ id }));
    const piecesUpdated = fixedPieceIds.map((id) => {
      const flat = flatPieceMap[id];
      return {
        piece: { id },
        diff: { plane: flat?.plane ?? identityPlane, center: flat?.center ?? zeroCenter },
      };
    });
    if (piecesRemoved.length > 0 || piecesUpdated.length > 0) {
      diff.pieces = {};
      if (piecesRemoved.length > 0) diff.pieces.removed = piecesRemoved;
      if (piecesUpdated.length > 0) diff.pieces.updated = piecesUpdated;
    }

    const connectionsRemoved = [...allRemovedConnectionIds].sort().map((id) => ({ id }));
    if (connectionsRemoved.length > 0) {
      diff.connections = { removed: connectionsRemoved };
    }

    return operationOk(diff, flatRes.warnings, flatRes.infos);
  }

  /**
   * ­ƒöìFind piece by id or by {@link Piece.name} (object form matches the spec `findPiece(name=ÔÇª)` style).
   */
  findPiece(lookup: string | { name: string }): Piece | undefined {
    const key = typeof lookup === "string" ? lookup : lookup.name;
    const byId = this.pieces?.find((p) => p.id === key);
    if (byId) return byId;
    return this.pieces?.find((p) => p.name === key);
  }

  /**
   * Resolves a piece by id/name or throws (same as {@link findPieceInDesign}).
   */
  requirePiece(lookup: string | { name: string }): Piece {
    const piece = this.findPiece(lookup);
    const label = typeof lookup === "string" ? lookup : lookup.name;
    if (!piece) throw new Error(`Piece ${label} not found in design ${this.name}`);
    return piece;
  }

  /**
   * ­ƒöìFind connection by id
   */
  findConnection(connectionId: string): Connection | undefined {
    return this._connections?.find((c) => c.id === connectionId);
  }

  /**
   * Resolves a connection by id or throws (same as {@link findConnectionInDesign}).
   */
  requireConnection(connectionId: string): Connection {
    return findConnection(this._connections ?? [], connectionId);
  }

  /**
   * ­ƒôïGet all pieces
   */
  getPieces(): readonly Piece[] {
    return this.pieces ?? [];
  }

  /**
   * ­ƒôïGet all connections
   */
  getConnections(): readonly Connection[] {
    return this._connections ?? [];
  }

  /**
   * Snapshot of connections on this design (OO spec); same array as {@link getConnections}.
   * After async or queued graph work, call again for an up-to-date view.
   */
  connections(): readonly Connection[] {
    return this.getConnections();
  }

  getDesignFamily(): Design[] {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).getDesignFamilyFor(this.id);
  }

  isInSameFamilyAsDesign(otherDesignId: string): boolean {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).areDesignsInSameFamily(this.id, otherDesignId);
  }

  canUseDesignAsPieceIn(containerDesignId: string): boolean {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).canUseDesignAsPiece(containerDesignId, this.id);
  }

  findSameFamilyDesignPiecesHere(): Piece[] {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findSameFamilyDesignPiecesIn(this.id);
  }

  runFlatten(): DesignOperationResult {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).runFlattenDesign(this.id);
  }

  runFlattenOptimized(): DesignOperationResult {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).flattenDesignMerkle(this.id);
  }

  previewRemovePiecesAndConnections(pieceIds: string[], connectionIds: string[]): DesignOperationResult {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).previewRemovePiecesAndConnections(this.id, pieceIds, connectionIds);
  }

  fixPieceDiff(pieceId: string): DesignDiff {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).fixPieceInDesignDiff(this.id, pieceId);
  }

  fixPiecesDiff(pieceIds: string[]): DesignDiff {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).fixPiecesInDesignDiff(this.id, pieceIds);
  }

  movePieces(pieces: Design, vector: MoveVector): DesignDiff {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).movePiecesInDesignOp(this, pieces, vector);
  }

  copyToClipboard(pieceIds: string[], connectionIds: string[]): OperationResult<Design> {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).copyDesignOp(this, pieceIds, connectionIds);
  }

  pasteFrom(source: Design, anchoring: string = "bottomLeft", coordinate?: Coordinate): DesignDiff {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).pasteDesignOp(source, this, anchoring, coordinate);
  }

  piecesMetadata(): OperationResult<Map<string, PiecePlacementMetadata>> {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).piecesMetadataFor(this.id);
  }

  piecesMetadataCached(cache?: { [pieceId: string]: FlatMerkleCacheEntry }): {
    result: OperationResult<Map<string, PiecePlacementMetadata>>;
    cache: { [pieceId: string]: FlatMerkleCacheEntry };
  } {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).piecesMetadataCachedFor(this.id, cache);
  }

  findPieceType(pieceId: string): Type {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findPieceTypeInDesign(this.id, pieceId);
  }

  findParentPiece(pieceId: string): Piece {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findParentPieceInDesign(this.id, pieceId);
  }

  findParentConnectionForPiece(pieceId: string): Connection {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findParentConnectionForPieceInDesign(this.id, pieceId);
  }

  findChildrenPieces(pieceId: string): Piece[] {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findChildrenPiecesInDesign(this.id, pieceId);
  }

  findUsedConnectorsByPiece(pieceId: string): Connector[] {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findUsedConnectorsByPieceInDesign(this.id, pieceId);
  }

  findReplacableTypesForPiece(pieceId: string): Type[] {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findReplacableTypesForPieceInDesign(this.id, pieceId);
  }

  findReplacableTypesForPieces(pieceIds: string[]): Type[] {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).findReplacableTypesForPiecesInDesign(this.id, pieceIds);
  }

  sumQuality(qualityId: string): number {
    if (!this.#kit) throw new Error("Design not attached to a KitImpl");
    return asKitInstance(this.#kit).sumQualityInDesign(this.id, qualityId);
  }
  // #endregion Ô£Å´©ÅMethods

  /**
   * Ensures the kit's flatten geometry cache is populated for this design (no persist diff).
   * Batch this before many {@link Piece.flatPlane} / {@link Piece.flatCenter} calls.
   */
  ensureFlattenMerkleCache(): void {
    const k = this.#kit;
    if (!k) return;
    asKitInstance(k).ensureFlattenGeometryCache(this.id);
  }

  toPlain(): DesignPlain {
    return DesignSchema.parse({
      ...(this as unknown as DesignPlain),
      pieces: this.pieces?.map((x) => x.toPlain()),
      connections: this._connections?.map((x) => x.toPlain()),
      stats: this.stats?.map((x) => x.toPlain()),
      props: this.props?.map((x) => x.toPlain()),
      layers: this.layers?.map((x) => x.toPlain()),
      groups: this.groups?.map((x) => x.toPlain()),
      attributes: this.attributes?.map((x) => x.toPlain()),
    });
  }

  /** ­ƒôªSerialize this design for wire transport. */
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  /** ­ƒº¡Deserialize a wire design into a stateful instance. */
  static deserialize(json: string, kit?: KitImpl): Design {
    return new Design(DesignSchema.parse(JSON.parse(json)), kit);
  }

  /** ­ƒ¬¬Project this design into its metadata wire shape. */
  toMeta(): DesignMeta {
    return DesignMetaSchema.parse(this.toPlain());
  }

  /** ­ƒº¥Project this design into its shallow wire shape. */
  toShallow(): DesignShallow {
    const plain = this.toPlain();
    return DesignShallowSchema.parse({
      ...plain,
      pieces: this.pieces?.map((p) => PieceMetaSchema.parse(p.toPlain())),
      connections: this._connections?.map((c) => ConnectionMetaSchema.parse(c.toPlain())),
      stats: this.stats?.map((s) => StatMetaSchema.parse(s.toPlain())),
      props: this.props?.map((p) => PropMetaSchema.parse(p.toPlain())),
      layers: this.layers?.map((l) => LayerMetaSchema.parse(l.toPlain())),
      groups: this.groups?.map((g) => GroupMetaSchema.parse(g.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetaSchema.parse(a.toPlain())),
    });
  }
}
/**
 * Serializes Design for transport.
 **/
export const serializeDesign = (design: Design): string => design.serialize();
/**
 **/
export const deserializeDesign = (json: string): Design => Design.deserialize(json);

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
 * Deep duplicate of a design diff (e.g. tests that strip or mutate entries).
 **/
export const duplicateDesignDiffForIsolation = (diff: DesignDiff): DesignDiff => DesignDiffSchema.parse(JSON.parse(JSON.stringify(diff)));

/**
 * Retrieves the DesignDiff value.
 **/
export const getDesignDiff = (before: Design, after: Design): DesignDiff => {
  return before.diffTo(after);
};
/**
 * Diff type for tracking mergeDesign changes.
 **/
export const mergeDesignDiff = (diff1: DesignDiff, diff2: DesignDiff): DesignDiff => {
  return Design.mergeDiff(diff1, diff2);
};
/**
 * Diff type for tracking inverseDesign changes.
 **/
export const inverseDesignDiff = (original: Design, appliedDiff: DesignDiff): DesignDiff => {
  return original.inverseDiff(appliedDiff);
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

// #region ­ƒºÀLocal detach (no structuredClone)
// Algorithms that must not mutate the live kit graph allocate detached copies via these helpers.

const stripNullsJsonClone = <T>(x: T): T => JSON.parse(JSON.stringify(x), (_k, v) => (v === null ? undefined : v));

const detachPieceForLocalMutation = (p: Piece | PiecePlain): Piece => {
  const source = p as any;
  const plain = PieceSchema.parse(
    stripNullsJsonClone((typeof (p as Piece).toPlain === "function" ? (p as Piece).toPlain() : p) as unknown) as unknown,
  );
  const type = plain.type ?? (source.type?.id ? { id: source.type.id } : undefined);
  const design = plain.design ?? (source.design?.id ? { id: source.design.id } : undefined);
  const plane = source.plane
    ? {
      origin: { ...source.plane.origin },
      xAxis: { ...source.plane.xAxis },
      yAxis: { ...source.plane.yAxis },
    }
    : undefined;
  const mirrorPlane = source.mirrorPlane
    ? {
      origin: { ...source.mirrorPlane.origin },
      xAxis: { ...source.mirrorPlane.xAxis },
      yAxis: { ...source.mirrorPlane.yAxis },
    }
    : undefined;
  const attributesPlain = source.attributes?.map((a: any) =>
    typeof a.toPlain === "function" ? a.toPlain() : AttributeSchema.parse(stripNullsJsonClone(a) as AttributePlain),
  );
  return new Piece({
    ...plain,
    type,
    design,
    plane,
    center: source.center ? { ...source.center } : undefined,
    mirrorPlane,
    props: source.props?.map((x: any) => ({ ...PropSchema.parse(stripNullsJsonClone(x) as PropPlain) })),
    attributes: attributesPlain,
  });
};

const detachConnectionForLocalMutation = (c: Connection): Connection => new Connection(ConnectionSchema.parse(stripNullsJsonClone(c.toPlain()) as unknown), c.getHostDesign());

const detachDesignForLocalMutation = (d: Design): Design =>
  new Design(
    {
      ...DesignSchema.parse(stripNullsJsonClone(d.toPlain()) as unknown),
      pieces: d.pieces?.map((p) => detachPieceForLocalMutation(p).toPlain()),
      connections: d._connections?.map((c) => detachConnectionForLocalMutation(c).toPlain()),
      stats: d.stats?.map((s) => ({ ...StatSchema.parse(stripNullsJsonClone(s) as StatPlain) })),
      props: d.props?.map((x) => ({ ...PropSchema.parse(stripNullsJsonClone(x) as PropPlain) })),
      layers: d.layers?.map((l) => ({
        ...LayerSchema.parse(stripNullsJsonClone(l) as LayerPlain),
        attributes: l.attributes?.map((a) => ({ ...AttributeSchema.parse(stripNullsJsonClone(a) as AttributePlain) })),
      })),
      groups: d.groups?.map((g) => ({
        ...GroupSchema.parse(stripNullsJsonClone(g) as GroupPlain),
        pieces: g.pieces?.map((pid) => ({ ...pid })),
        attributes: g.attributes?.map((a) => ({ ...AttributeSchema.parse(stripNullsJsonClone(a) as AttributePlain) })),
      })),
      attributes: d.attributes?.map((a) => ({ ...AttributeSchema.parse(stripNullsJsonClone(a) as AttributePlain) })),
    },
    d.getKit(),
  );
// #endregion ­ƒºÀLocal detach

/**
 * Creates a mixed design for visualization, annotating entities with diff status.
 * Annotate each with a semio.diffStatus attribute (unchanged/modified/removed/added).
 * Updated pieces apply non-geometric diff fields but KEEP base plane and center so
 * they render in their original location and only change color. Updated connections
 * apply the full diff. Removed entities are kept in place marked as removed.
 * Added entities are appended marked as added.
 **/
export const designWithDiff = (base: Design, diff: DesignDiff): Design => {
  const baseDesign = base instanceof Design ? base : new Design(DesignSchema.parse(stripNullsJsonClone(base) as unknown));
  const DIFF_STATUS_KEY = "semio.diffStatus";
  const setStatus = (attrs: Attribute[] | undefined, status: DiffStatus): AttributePlain[] => {
    const result = [...(attrs ?? [])].map((a) => (typeof a.toPlain === "function" ? a.toPlain() : a));
    result.push(new Attribute({ id: `${DIFF_STATUS_KEY}.${status}`, key: DIFF_STATUS_KEY, value: status }).toPlain());
    return result;
  };

  const removedPieceIds = new Set((diff.pieces?.removed ?? []).map((r) => r.id));
  const updatedPieceMap = new Map((diff.pieces?.updated ?? []).map((u) => [(u as any).piece.id, u.diff]));
  const removedConnIds = new Set((diff.connections?.removed ?? []).map((r) => r.id));
  const updatedConnMap = new Map((diff.connections?.updated ?? []).map((u) => [(u as any).connection.id, u.diff]));

  const resultPieces: Piece[] = (baseDesign.pieces ?? []).map((p) => {
    if (removedPieceIds.has(p.id)) {
      return new Piece({ ...p.toPlain(), attributes: setStatus(p.attributes, DiffStatus.Removed) });
    }
    if (updatedPieceMap.has(p.id)) {
      const applied = detachPieceForLocalMutation(p);
      applyPieceDiff(applied, updatedPieceMap.get(p.id)!);
      const preserved = { ...applied.toPlain() };
      if (p.plane !== undefined) preserved.plane = PlaneSchema.parse(p.plane as unknown);
      else delete preserved.plane;
      if (p.center !== undefined) preserved.center = CoordinateSchema.parse(p.center as unknown);
      else delete preserved.center;
      preserved.attributes = setStatus(applied.attributes, DiffStatus.Modified);
      return new Piece(preserved);
    }
    return new Piece({ ...p.toPlain(), attributes: setStatus(p.attributes, DiffStatus.Unchanged) });
  });
  for (const added of diff.pieces?.added ?? []) {
    const raw = added as PiecePlain;
    const attrsForStatus = raw.attributes?.map((a) => new Attribute(a)) ?? undefined;
    resultPieces.push(
      new Piece({
        ...raw,
        attributes: setStatus(attrsForStatus, DiffStatus.Added),
      }),
    );
  }

  const resultConns: Connection[] = (baseDesign._connections ?? []).map((c) => {
    if (removedConnIds.has(c.id)) {
      return new Connection({ ...c.toPlain(), attributes: setStatus(c.attributes, DiffStatus.Removed) });
    }
    if (updatedConnMap.has(c.id)) {
      const applied = detachConnectionForLocalMutation(c);
      applyConnectionDiff(applied, updatedConnMap.get(c.id)!);
      return new Connection({ ...applied.toPlain(), attributes: setStatus(applied.attributes, DiffStatus.Modified) });
    }
    return new Connection({ ...c.toPlain(), attributes: setStatus(c.attributes, DiffStatus.Unchanged) });
  });
  for (const added of diff.connections?.added ?? []) {
    const raw = added as ConnectionPlain;
    const attrsForStatus = raw.attributes?.map((a) => new Attribute(a)) ?? undefined;
    resultConns.push(
      new Connection({
        ...raw,
        attributes: setStatus(attrsForStatus, DiffStatus.Added),
      }),
    );
  }

  return new Design(
    DesignSchema.parse({
      ...DesignSchema.parse(baseDesign.toPlain()),
      pieces: resultPieces.map((x) => x.toPlain()),
      connections: resultConns.map((x) => x.toPlain()),
    }),
  );
};

/**
 * Zod schema for Designs diff validation.
 **/
export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
/**
 * Diff type for tracking Designs changes.
 **/
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;

/**
 **/
export const mergeDesigns = (designs: Design[]): DesignDiff => {
  const pieces = designs.flatMap((d) => d.pieces ?? []);
  const connections = designs.flatMap((d) => d._connections ?? []);

  return {
    pieces: pieces.length > 0 ? { added: pieces } : undefined,
    connections: connections.length > 0 ? { added: connections } : undefined,
  };
};

/**
 **/
export const orientDesign = (plane?: Plane, center?: Coordinate): DesignDiff => {
  if (plane === undefined && center === undefined) {
    return {};
  }

  return {};
};

/**
 * Deletes pieces and connections from a design, returning a DesignDiff.
 * Prefer {@link Design.deletePiecesAndConnectionsDiff} / {@link Design.deletePieces}.
 * @deprecated Parameter `kit` is ignored; the design's host kit is used.
 **/
export const deletePiecesAndConnectionsInDesign = (kit: KitImpl, design: Design, pieceIds: string[], connectionIds: string[]): DesignDiffOperationResult => {
  void kit;
  const d = design instanceof Design ? design : new Design(design as any);
  return d.deletePiecesAndConnectionsDiff(pieceIds, connectionIds);
};

/** @see {@link KitImpl.removePiecesAndConnectionsFromDesignOp} */
export const removePiecesAndConnectionsFromDesign = (kit: KitLike, designId: string, pieceIds: string[], connectionIds: string[]): DesignOperationResult => asKitInstance(kit).removePiecesAndConnectionsFromDesignOp(designId, pieceIds, connectionIds);

/**
 * Parent-connector rotation and unit world axes for gap (local +Y), shift (+X), rise (+Z) before child orientation, matching {@link computeChildPlane}.
 * Used by flatten / move paths that need connector frames from kit types (see {@link KitImpl.buildConnectorResolver}).
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

// Ôù╗´©ÅcomputeChildPlane computes a child plane from parent plane and connection parameters.
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

// #subregion ­ƒº¡Flatten placement walk
/** ­ƒöûSingle undirected adjacency entry for flatten BFS (matches Go FlattenDesign traversal order). */
type FlattenAdjacencyEntry = { neighborId: string; connection: Connection };

const flattenPlaneCenterTol = 1e-4;
const flattenPlanesDiffer = (a?: Plane, b?: Plane): boolean => {
  if (a == null && b == null) return false;
  if (a == null || b == null) return true;
  return (
    Math.abs(a.origin.x - b.origin.x) >= flattenPlaneCenterTol ||
    Math.abs(a.origin.y - b.origin.y) >= flattenPlaneCenterTol ||
    Math.abs(a.origin.z - b.origin.z) >= flattenPlaneCenterTol ||
    Math.abs(a.xAxis.x - b.xAxis.x) >= flattenPlaneCenterTol ||
    Math.abs(a.xAxis.y - b.xAxis.y) >= flattenPlaneCenterTol ||
    Math.abs(a.xAxis.z - b.xAxis.z) >= flattenPlaneCenterTol ||
    Math.abs(a.yAxis.x - b.yAxis.x) >= flattenPlaneCenterTol ||
    Math.abs(a.yAxis.y - b.yAxis.y) >= flattenPlaneCenterTol ||
    Math.abs(a.yAxis.z - b.yAxis.z) >= flattenPlaneCenterTol
  );
};
const flattenCentersDiffer = (a?: Coordinate, b?: Coordinate): boolean => {
  if (a == null && b == null) return false;
  if (a == null || b == null) return true;
  return Math.abs(a.u - b.u) >= flattenPlaneCenterTol || Math.abs(a.v - b.v) >= flattenPlaneCenterTol;
};

/** Same plane as {@link matrixToPlane} on an identity matrix; avoids per-call THREE allocations in flatten. */
const FLATTEN_IDENTITY_PLANE: Plane = new Plane({
  origin: { x: 0, y: 0, z: 0 },
  xAxis: { x: 1, y: 0, z: 0 },
  yAxis: { x: 0, y: 1, z: 0 },
});

const buildFlattenPieceAdjacency = (pieces: Piece[], connections: Connection[]): { pieceMap: { [id: string]: Piece }; adjacency: Map<string, FlattenAdjacencyEntry[]> } => {
  const pieceMap: { [id: string]: Piece } = {};
  for (const p of pieces) {
    if (p.id) pieceMap[p.id] = p;
  }
  const adjacency = new Map<string, FlattenAdjacencyEntry[]>();
  for (const connection of connections) {
    const sourceId = connection.connected.piece.id;
    const targetId = connection.connecting.piece.id;
    if (!pieceMap[sourceId] || !pieceMap[targetId]) continue;
    const a = adjacency.get(sourceId);
    if (a) a.push({ neighborId: targetId, connection });
    else adjacency.set(sourceId, [{ neighborId: targetId, connection }]);
    const b = adjacency.get(targetId);
    if (b) b.push({ neighborId: sourceId, connection });
    else adjacency.set(targetId, [{ neighborId: sourceId, connection }]);
  }
  return { pieceMap, adjacency };
};

const collectUndirectedComponentIds = (startId: string, adjacency: Map<string, FlattenAdjacencyEntry[]>): Set<string> => {
  const comp = new Set<string>();
  const stack: string[] = [startId];
  comp.add(startId);
  while (stack.length) {
    const u = stack.pop()!;
    for (const { neighborId } of adjacency.get(u) ?? []) {
      if (!comp.has(neighborId)) {
        comp.add(neighborId);
        stack.push(neighborId);
      }
    }
  }
  return comp;
};

type FlattenEdgeVisit = {
  parentId: string;
  childId: string;
  connection: Connection;
  depth: number;
  parentPiece: Piece;
  childPiece: Piece;
};

/** ­ƒöûBreadth-first placement walk: one BFS tree per connected component; root = first fixed (plane+center) piece in design.pieces order, else earliest piece in that order (matches .NET QuickGraph connected-component ordering). */
const flattenPlacementWalkDesignOrderRoots = (
  pieceMap: { [id: string]: Piece },
  adjacency: Map<string, FlattenAdjacencyEntry[]>,
  pieces: Piece[],
  handlers: {
    onComponentDiscovered?: (component: Set<string>, rootId: string, pieceMap: { [id: string]: Piece }) => void;
    initRoot?: (rootId: string, rootPiece: Piece) => void;
    onTreeEdge?: (ev: FlattenEdgeVisit) => void;
  },
): void => {
  const pieceIndexById = new Map<string, number>();
  pieces.forEach((p, i) => {
    if (p.id) pieceIndexById.set(p.id, i);
  });
  const processed = new Set<string>();

  for (const p of pieces) {
    const seedId = p.id;
    if (!seedId || processed.has(seedId)) continue;
    const component = collectUndirectedComponentIds(seedId, adjacency);
    const sortedIds = [...component].sort((a, b) => (pieceIndexById.get(a) ?? 0) - (pieceIndexById.get(b) ?? 0));
    for (const id of component) processed.add(id);

    const fixedSorted = sortedIds.filter((id) => {
      const piece = pieceMap[id];
      return piece?.plane !== undefined && piece?.center !== undefined;
    });
    const rootId = fixedSorted.length > 0 ? fixedSorted[0] : sortedIds[0];
    handlers.onComponentDiscovered?.(component, rootId, pieceMap);

    const visited = new Set<string>();
    const queue: string[] = [rootId];
    visited.add(rootId);
    handlers.initRoot?.(rootId, pieceMap[rootId]);

    const depthById = new Map<string, number>();
    depthById.set(rootId, 0);

    while (queue.length) {
      const currentId = queue.shift()!;
      const depth = depthById.get(currentId) ?? 0;
      const parentPiece = pieceMap[currentId];
      if (!parentPiece) continue;

      for (const { neighborId, connection } of adjacency.get(currentId) ?? []) {
        if (visited.has(neighborId)) continue;
        visited.add(neighborId);
        depthById.set(neighborId, depth + 1);
        const childPiece = pieceMap[neighborId];
        if (!childPiece) continue;
        handlers.onTreeEdge?.({
          parentId: currentId,
          childId: neighborId,
          connection,
          depth: depth + 1,
          parentPiece,
          childPiece,
        });
        queue.push(neighborId);
      }
    }
  }
};
// #endregion ­ƒº¡Flatten placement walk

// #region ­ƒî│Flatten Merkle Hashes
/**
 * Per-piece merkle hash pair used to cache flattenDesign results and skip recomputation when inputs are unchanged.
 **/
export type FlatMerkleHashes = { planeHash: string; centerHash: string };

const hashPlaneRoot = (id: string, plane: Plane | undefined): string => {
  const w = new HashWriter();
  if (!plane) {
    w.writeString("plane.root.identity");
    w.writeString(id);
    return w.digest();
  }
  w.writeString("plane.root");
  w.writeString(id);
  w.writeNumber(plane.origin?.x ?? 0);
  w.writeNumber(plane.origin?.y ?? 0);
  w.writeNumber(plane.origin?.z ?? 0);
  w.writeNumber(plane.xAxis?.x ?? 0);
  w.writeNumber(plane.xAxis?.y ?? 0);
  w.writeNumber(plane.xAxis?.z ?? 0);
  w.writeNumber(plane.yAxis?.x ?? 0);
  w.writeNumber(plane.yAxis?.y ?? 0);
  w.writeNumber(plane.yAxis?.z ?? 0);
  return w.digest();
};

const hashPlaneChain = (parentHash: string, parentConnector: Connector, childConnector: Connector, connection: Connection): string => {
  const w = new HashWriter();
  w.writeString("plane.chain");
  w.writeHash(parentHash);
  w.writeNumber(parentConnector.point?.x ?? 0);
  w.writeNumber(parentConnector.point?.y ?? 0);
  w.writeNumber(parentConnector.point?.z ?? 0);
  w.writeNumber(parentConnector.direction?.x ?? 0);
  w.writeNumber(parentConnector.direction?.y ?? 0);
  w.writeNumber(parentConnector.direction?.z ?? 0);
  w.writeNumber(childConnector.point?.x ?? 0);
  w.writeNumber(childConnector.point?.y ?? 0);
  w.writeNumber(childConnector.point?.z ?? 0);
  w.writeNumber(childConnector.direction?.x ?? 0);
  w.writeNumber(childConnector.direction?.y ?? 0);
  w.writeNumber(childConnector.direction?.z ?? 0);
  w.writeNumber(connection.gap ?? 0);
  w.writeNumber(connection.shift ?? 0);
  w.writeNumber(connection.rise ?? 0);
  w.writeNumber(connection.rotation ?? 0);
  w.writeNumber(connection.turn ?? 0);
  w.writeNumber(connection.tilt ?? 0);
  return w.digest();
};

const hashCenterRoot = (id: string, center: Coordinate | undefined): string => {
  const w = new HashWriter();
  if (!center) {
    w.writeString("center.root.identity");
    w.writeString(id);
    return w.digest();
  }
  w.writeString("center.root");
  w.writeString(id);
  w.writeNumber(center.u ?? 0);
  w.writeNumber(center.v ?? 0);
  return w.digest();
};

const hashCenterChain = (parentHash: string, parentConnector: Connector, connection: Connection): string => {
  const w = new HashWriter();
  w.writeString("center.chain");
  w.writeHash(parentHash);
  w.writeNumber(parentConnector.direction?.z ?? 0);
  w.writeNumber(parentConnector.t ?? 0);
  w.writeNumber(connection.u ?? 0);
  w.writeNumber(connection.v ?? 0);
  return w.digest();
};

/**
 * ­ƒºáFlatMerkleCacheEntry bundles a piece's merkle hashes with its cached plane/center/flat piece so incremental flatten calls can reuse unchanged values without redoing the matrix math or attribute bookkeeping.
 **/
export type FlatMerkleCacheEntry = {
  planeHash: string;
  centerHash: string;
  plane?: Plane;
  center?: Coordinate;
  flatPiece?: Piece;
};

// #endregion ­ƒî│Flatten Merkle Hashes

/**
 * Retrieves the ClusterableGroups value.
 **/
export const getClusterableGroups = (design: Design, selectedPieceIds: string[]): string[][] => {
  if (selectedPieceIds.length < 2) return [];

  const adjacencyMap = new Map<string, Set<string>>();
  (design._connections || []).forEach((connection) => {
    const sourceId = connection.connecting.piece.id;
    const targetId = connection.connected.piece.id;

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

  const pieceIdSet = new Set((design.pieces || []).map((piece) => piece.id));
  const hasDesignNodes = selectedPieceIds.some((id) => !pieceIdSet.has(id));
  const hasMultipleComponents = connectedGroups.length > 1;
  const hasLargeConnectedGroup = connectedGroups.some((group) => group.length > 1);

  if (hasDesignNodes || hasMultipleComponents || hasLargeConnectedGroup) {
    return [selectedPieceIds];
  }

  return [];
};

/**
 * Type alias for IncludedDesignInfo.
 **/
export type IncludedDesignInfo = {
  id: string;
  designId: string;
  type: "connected" | "fixed";
  center?: Coordinate;
  plane?: Plane;
  externalConnections?: Connection[];
};

/**
 * Retrieves the IncludedDesigns value.
 **/
export const getIncludedDesigns = (design: Design): IncludedDesignInfo[] => {
  const includedDesigns: IncludedDesignInfo[] = [];

  const designIds = new Set<string>();
  toArray(design._connections).forEach((conn: Connection) => {
    const cStub = conn.connected.wireDesignPieceId()?.id;
    const gStub = conn.connecting.wireDesignPieceId()?.id;
    if (cStub) designIds.add(cStub);
    if (gStub) designIds.add(gStub);
  });

  Array.from(designIds).forEach((designIdString) => {
    const externalConnections =
      design._connections?.filter((connection: Connection) => {
        const connectedToDesign = connection.connected.wireDesignPieceId()?.id === designIdString;
        const connectingToDesign = connection.connecting.wireDesignPieceId()?.id === designIdString;
        return connectedToDesign || connectingToDesign;
      }) ?? [];

    includedDesigns.push({
      id: designIdString,
      designId: designIdString,
      type: "connected",
      externalConnections,
    });
  });

  return includedDesigns;
};

/**
 **/
export const isPortInUse = (design: Design, pieceId: string, connectorId: string): boolean => {
  const connections = findPieceConnectionsInDesign(design, pieceId);
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.id === pieceId;
    const isPortConnected = isPieceConnected ? connection.connected.connector?.id === connectorId : connection.connecting.connector?.id === connectorId;
    if (isPortConnected) return true;
  }
  return false;
};

/**
 **/
export const isConnectionInDesign = (design: Design, connection: Connection): boolean => {
  return design._connections?.some((c) => areSameConnection(c, connection)) ?? false;
};

/**
 * Searches for matching PieceInDesign entry.
 **/
export const findPieceInDesign = (design: Design, pieceId: string): Piece => design.requirePiece(pieceId);

/**
 * Resolves a design by id on a kit snapshot (plain {@link Kit} / {@link KitImpl} data).
 */
export const findDesignInKit = (kit: { designs?: Design[] } | null | undefined, designId: string): Design | undefined =>
  kit?.designs?.find((d) => d.id === designId);

/**
 * Resolves a type by id on a kit snapshot.
 */
export const findTypeInKit = (kit: { types?: Type[] } | null | undefined, typeId: string): Type | undefined => kit?.types?.find((t) => t.id === typeId);

/**
 * Searches for matching ConnectionInDesign entry.
 **/
export const findConnectionInDesign = (design: Design, connectionId: string): Connection => design.requireConnection(connectionId);

/**
 * Searches for matching ConnectionsInDesign entry.
 **/
export const findConnectionsInDesign = (design: Design, connectionIds: string[]): Connection[] => {
  return connectionIds.map((connectionId) => findConnectionInDesign(design, connectionId));
};

/**
 * Searches for matching PieceConnectionsInDesign entry.
 **/
export const findPieceConnectionsInDesign = (design: Design, pieceId: string): Connection[] => {
  return findPieceConnections(design._connections ?? [], pieceId);
};

/**
 * Searches for matching ConnectionPiecesInDesign entry.
 **/
export const findConnectionPiecesInDesign = (design: Design, connection: Connection): { connecting: Piece; connected: Piece } => {
  return {
    connected: findPieceInDesign(design, connection.connected.piece.id),
    connecting: findPieceInDesign(design, connection.connecting.piece.id),
  };
};

/**
 * Searches for matching StaleConnectionsInDesign entry.
 **/
export const findStaleConnectionsInDesign = (design: Design): Connection[] => {
  return (
    design._connections?.filter((c) => {
      try {
        findPieceInDesign(design, c.connected.piece.id);
        findPieceInDesign(design, c.connecting.piece.id);
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

// #region ­ƒöûDragMoveStructuralSelection
/**
 * Shared parent graph and fixed/selection sets for {@link dragPiecesInDesign} and {@link movePiecesInDesign}.
 * Specs: "Fixed" pieces are selected pieces that never appear as the connecting (child) side of a connection.
 **/
const buildDragMoveStructuralContext = (
  design: Design,
  pieces: Design,
): {
  selectedIds: Set<string>;
  parentMap: Map<string, { connectionId: string; parentId: string }>;
  pieceMap: Map<string, Piece>;
  fixedIds: Set<string>;
} => {
  const selectedIds = new Set((pieces.pieces ?? []).map((p) => p.id));
  const parentMap = new Map<string, { connectionId: string; parentId: string }>();
  for (const c of design._connections ?? []) {
    parentMap.set(c.connecting.piece.id, { connectionId: c.id, parentId: c.connected.piece.id });
  }
  const pieceMap = new Map<string, Piece>();
  for (const p of design.pieces ?? []) {
    pieceMap.set(p.id, p);
  }
  const fixedIds = new Set<string>();
  for (const id of selectedIds) {
    if (!parentMap.has(id)) fixedIds.add(id);
  }
  return { selectedIds, parentMap, pieceMap, fixedIds };
};

/**
 * True when walking parent links finds a selected ancestor (same descendant suppression as drag).
 **/
const pieceHasSelectedAncestorInDragMoveTree = (pieceId: string, selectedIds: Set<string>, parentMap: Map<string, { connectionId: string; parentId: string }>): boolean => {
  let current = pieceId;
  while (parentMap.has(current)) {
    const ancestor = parentMap.get(current)!.parentId;
    if (selectedIds.has(ancestor)) return true;
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
 * Minimum-norm ╬┤ with J╬┤ = t for 3├ùn Jacobian J whose columns are cols[i] = Ôêéorigin/Ôêéparam_i; ╬┤ = JßÁÇ(JJßÁÇ)Ôü╗┬╣t.
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
 * Specs: One GaussÔÇôNewton step; matches flatten placement when child connector exists. Falls back to translation-only basis if singular.
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
 * ­ƒôïExtracts selected pieces and connections from a design into a new Design (clipboard).
 * Specs: Selected pieces are classified as internal-fixed, internal-connected, or parent-piece-exclusive parent-connection-inclusive.
 * Internal pieces are copied as-is. Pp-excl-pc-incl pieces get semio.center and semio.plane attributes.
 * Non-internal connections include their external pieces marked with semio.piece.origin = "external".
 **/
export const copyDesign = (kit: KitLike, design: Design, pieceIds: string[], connectionIds: string[]): OperationResult<Design> => asKitInstance(kit).copyDesignOp(design, pieceIds, connectionIds);

/** Specs: Anchoring strings handled by `pasteDesign` switch; any other string falls through to the default branch (same offset as `original`). */
export const PASTE_DESIGN_ANCHORING_KINDS = ["original", "middle", "centroid", "bottomLeft", "bottomRight", "topLeft", "topRight"] as const;

export type PasteDesignAnchoringKind = (typeof PASTE_DESIGN_ANCHORING_KINDS)[number];

/**
 * ­ƒôïPastes a copied design into a target design, returning a DesignDiff.
 * Specs: Anchoring determines the reference point within the bounding rectangle of the source.
 * External stub parents are remapped to matching target pieces (name + connector) when possibleÔÇöeven if the child
 * has a plane (flattened pp-excl). If rematch is impossible, fall back to center/plane from attributes then anchor/coordinate.
 * Other pieces with a plane alone get -anchor then +coordinate on diagram center.
 * Fully internal source connections keep cloned u/v when coordinate only affects stub-bridge remapping as above.
 * With coordinate, only the remapped childÔÇôstub parent bridge updates u/v: target matched parentÔÇÖs diagram center minus
 * (coordinate + (anchor ÔêÆ child flat center)). Descendant internal connections keep deep-cloned u/v.
 **/
export const pasteDesign = (kit: KitLike, source: Design, target: Design, anchoring: string = "bottomLeft", coordinate?: Coordinate): DesignDiff => asKitInstance(kit).pasteDesignOp(source, target, anchoring, coordinate);

// #endregion ­ƒôÉDesign

// #region ÔÅ▒´©ÅKitImpl
// KitImpl entity types, schemas, and helpers MUST be defined here.

// #region ­ƒº¼KitKind
// KitKind discriminates the five persistence/transport forms of a KitImpl.

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
export const KitKindSchema = z.enum(["dev", "local", "archive", "remote", "transport"]);
/**
 * Discriminator for the five kit persistence/transport forms.
 **/
export type KitKind = z.infer<typeof KitKindSchema>;
/**
 * All valid KitKind values as a readonly tuple.
 **/
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;
// #endregion ­ƒº¼KitKind

/**
 * Zod schema for KitImpl validation.
 **/
export const KitSchema = z.object({
  id: z.string(),
  name: z.string(),
  version: z.string().optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  tags: z.array(TagSchema).optional(),
  concepts: z.array(ConceptSchema).optional(),
  families: z.array(FamilySchema).optional(),
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
 * Plain JSON shape of a kit (Zod-validated).
 **/
export type KitData = z.infer<typeof KitSchema>;

/**
 * Live {@link KitImpl} or wire {@link KitData}. Resolve to a class instance with {@link asKitInstance}; entity constructors accept only {@link KitImpl}.
 */
export type KitLike = KitImpl | KitData;

// #region ­ƒöûValidationState
/** Last validation outcome for a conflicted {@link KitImpl}; same shape as {@link KitDiffValidationResult} without apply metadata. */
export type ValidationState = KitDiffValidationResult;
// #endregion ­ƒöûValidationState

// #region ­ƒöîBackbone Interface
/**
 * ­ƒöîBackbone handles KitImpl change persistence and synchronization.
 * Implementations support Dev (file), Local (folder), and Remote (hub) backends.
 */
export interface Backbone {
  changed(change: KitGraphChange): Promise<void>;
  /**
   * Optional wiring for inbound sync: the backbone calls `onInboundDiff` with remote/foreign diffs;
   * the kit applies them through the same validation pipeline (no echo to `changed` by default).
   */
  attach?(kit: KitImpl, onInboundDiff: (diff: KitDiff) => void): void | Promise<void>;
}

/**
 * ­ƒºáDevKitBackbone persists KitImpl changes to a single JSON file.
 */
export class DevKitBackbone implements Backbone {
  constructor(private filePath: string) { }
  async changed(_change: KitGraphChange): Promise<void> {
    void this.filePath;
  }
}

/**
 * ­ƒôüLocalKitBackbone persists KitImpl to folder structure with assets.
 */
export class LocalKitBackbone implements Backbone {
  constructor(private folderPath: string) { }
  async changed(_change: KitGraphChange): Promise<void> {
    void this.folderPath;
  }
}

/**
 * ­ƒîÉRemoteKitBackbone syncs KitImpl changes to remote hub via WebSocket.
 */
export class RemoteKitBackbone implements Backbone {
  constructor(private websocketUrl: string) { }
  async changed(_change: KitGraphChange): Promise<void> {
    void this.websocketUrl;
  }
}
// #endregion ­ƒöîBackbone Interface

/**
 * ­ƒöäKitGraphChange represents a bidirectional change to KitImpl state.
 */
export interface KitGraphChange {
  forward: KitDiff;
  backward: KitDiff;
  /** Result of {@link validateKitGraphDiff} for {@link forward} (before apply). */
  validation: KitDiffValidationResult;
  preconditions?: ChangePrecondition[];
}

export type ConcurrentDeleteConflict = {
  id: string;
  entityKind: "Type";
  entityId: string;
  localInteractionId: string;
  localPendingChanges: KitGraphChange[];
  deletedByUserId?: string;
  deletedByDisplayName?: string;
  deletedAt?: string;
  deleteChangeId?: string;
  proposedResolutions: readonly ConcurrentDeleteProposedResolution[];
};

/** Options for {@link KitImpl._applyDiff} (internal / kit-store pipeline). Prefer semantic entity methods for domain edits. */
export type KitChangeOptions = {
  origin?: string;
  /** Record this step on an open transaction (see {@link KitImpl.beginTransaction}). */
  transactionId?: string;
  /** When false, do not enqueue {@link Backbone.changed}. Default true. */
  notifyBackbone?: boolean;
  /** When true, do not push to finalized local history (see {@link KitImpl.undo}). Default false. */
  skipGlobalHistory?: boolean;
  /** Inbound backbone: committed external change ÔÇö not part of local history; clears redo. */
  inboundCommitted?: boolean;
  inboundActor?: { changeId?: string; actorId?: string; actorDisplayName?: string };
};

export type ConflictKind = "LocalChange" | "TxUndo" | "TxRedo" | "TxAbort" | "HistoryUndo" | "HistoryRedo" | "BackboneChange";

export type Conflict = {
  id: string;
  kind: ConflictKind;
  txId?: string;
  proposedDiff?: KitDiff;
  proposedChange?: KitGraphChange;
  validationReport: KitDiffValidationResult;
  createdAt: string;
};

export type KitPhase = "ready" | "frozen";

export type HistoryInfo = {
  pastCount: number;
  futureCount: number;
  revision: number;
  auditLength: number;
};

export type TransactionStatus = "open" | "finalized" | "aborted";

export type InteractionWorkspaceStatus = "clean" | "conflicted";

export type TransactionView = {
  id: string;
  status: TransactionStatus;
  label?: string;
  workspaceStatus?: InteractionWorkspaceStatus;
  conflicts?: readonly ConcurrentDeleteConflict[];
};

// #region ­ƒ¬¬KitEntity wire DTOs & ledger KitChange
/** Plain string identifiers on the {@link KitEntity} surface (contrast with object-shaped {@link TypeId}). */
export type KitEntityUUID = string;
export type KitEntityTypeId = string;
export type KitEntityDesignId = string;
export type KitEntityPieceId = string;
export type InteractionId = string;
export type ChangeId = string;

/** Optimistic precondition on an interaction step (see collaboration representation). */
export type ChangePrecondition = {
  entityKind: "Type" | string;
  entityId: string;
  expectedLifecycle: EntityLifecycle;
  expectedVersionHash: string;
};

export type ConcurrentDeleteProposedResolution = "discardLocalChanges" | "restoreEntityAndReplayLocalChanges";

export interface KitWireType {
  id: KitEntityTypeId;
  name: string;
}

export interface KitWireDesign {
  id: KitEntityDesignId;
  name: string;
}

/** Narrow kit view for {@link KitEntity#import} / {@link KitEntity#export}; full graph uses {@link KitData}. */
export interface KitWire {
  uuid: KitEntityUUID;
  name: string;
  types: KitWireType[];
  designs: KitWireDesign[];
}

export const KitWireDtoSchema = z.object({
  uuid: z.string(),
  name: z.string(),
  types: z.array(z.any()),
  designs: z.array(z.any()),
});
export type KitDTO = z.infer<typeof KitWireDtoSchema>;

export interface KitSelection {
  types: KitEntityTypeId[];
  designs: KitEntityDesignId[];
}

export interface KitInteraction {
  uuid: InteractionId;
  label: string;
  selection: KitSelection;
}

export const KitInteractionWireSchema = z.object({
  uuid: z.string(),
  label: z.string(),
  selection: z.object({
    types: z.array(z.string()),
    designs: z.array(z.string()),
  }),
});
export type KitInteractionDTO = z.infer<typeof KitInteractionWireSchema>;

export type InteractionStatus = TransactionStatus;

export type ChangeOrigin = "local-interaction" | "local-finalize" | "local-history-undo" | "local-history-redo" | "backbone";

export interface ValidationMessage {
  code: string;
  path?: string[];
  message: string;
}

export interface ValidationReport {
  infos: ValidationMessage[];
  warnings: ValidationMessage[];
  errors: ValidationMessage[];
}

/**
 * Ledger/backbone change record ({@link KitBackbone#submitCommittedChange}).
 * Reversible graph bundles used while editing are {@link KitGraphChange}.
 */
export interface KitChange {
  id: ChangeId;
  origin: ChangeOrigin;
  interactionId?: InteractionId;
  baseRevision: number;
  revision: number;
  diff: KitDiff;
  inverse: KitDiff;
  report: ValidationReport;
  createdAt: string;
  metadata?: Record<string, string>;
  actorId?: string;
  actorDisplayName?: string;
  affectedEntities?: ReadonlyArray<{ kind: string; id: string }>;
}

export interface HistoryEntry {
  change: KitChange;
}

export interface InteractionSession extends KitInteraction {
  status: InteractionStatus;
  done: KitChange[];
  undone: KitChange[];
  netForward: KitDiff;
  netBackward: KitDiff;
  baseRevision: number;
  touched: Set<string>;
}

export interface BackboneSink {
  changed(change: KitChange): void;
  failed(error: unknown): void;
}

export interface KitBackbone {
  readonly kind: "local" | "dev" | "remote";
  open(input: { kitId: KitEntityUUID; sink: BackboneSink; options?: unknown }): Promise<void>;
  close(): Promise<void>;
  importSnapshot(dto: KitDTO): Promise<void>;
  exportSnapshot(): Promise<KitDTO>;
  submitCommittedChange(change: KitChange): Promise<void>;
}

// #endregion ­ƒ¬¬KitEntity wire DTOs & ledger KitChange

type KitAuditEntry = {
  revision: number;
  tag: string;
  change?: KitGraphChange;
};

type KitRuntimeTransaction = {
  label?: string;
  status: TransactionStatus;
  startPlain: KitData;
  done: KitGraphChange[];
  undone: KitGraphChange[];
  netForward: KitDiff;
  netBackward: KitDiff;
  baseRevision: number;
  touchedEntities: Set<string>;
  touchedVersions: Map<string, string>;
  workspaceStatus: InteractionWorkspaceStatus;
  conflicts: ConcurrentDeleteConflict[];
};

/**
 * Starts named open transactions ({@link KitImpl.beginTransaction}); returns uuid-v7 ids from {@link id}.
 */
export class KitTransactionsCoordinator {
  constructor(private readonly host: KitImpl) { }

  start(label?: string): string {
    return this.host.beginTransaction(label).id;
  }
}

/**
 * Undo/redo within the kitÔÇÖs {@link KitImpl.setActiveTransaction active} open transaction.
 */
export class KitActiveTransactionSurface {
  constructor(private readonly host: KitImpl) { }

  undo(): void {
    this.host.undoWithinTransaction(this.host.requireActiveTransactionId());
  }

  redo(): void {
    this.host.redoWithinTransaction(this.host.requireActiveTransactionId());
  }

  get canUndo(): boolean {
    const id = this.host.activeTransactionId;
    return !!id && this.host.canUndoWithinTransaction(id);
  }

  get canRedo(): boolean {
    const id = this.host.activeTransactionId;
    return !!id && this.host.canRedoWithinTransaction(id);
  }
}

/**
 * Single-threaded transactional kit runtime: live state, provisional open transactions, finalized history undo/redo,
 * backbone sync for committed changes only. Raw {@link KitDiff} application is internal ({@link KitImpl._applyDiff}).
 **/
export class KitImpl {
  id!: string;
  name!: string;
  version?: string;
  types?: Type[];
  designs?: Design[];
  tags?: Tag[];
  concepts?: Concept[];
  families?: Family[];
  qualities?: Quality[];
  files?: File[];
  folders?: Folder[];
  authors?: Author[];
  remote?: string;
  homepage?: string;
  license?: string;
  preview?: string;
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;

  /** Namespaced mutations and lookups; prefer `kit.ops.types.add(ÔÇª)` over removed module-level helpers. */
  readonly ops: KitOps;

  /** `transactions.start(label)` ÔåÆ open tx id (uuid-v7); use with {@link setActiveTransaction}. */
  readonly transactions!: KitTransactionsCoordinator;

  /** Undo/redo steps on the {@link activeTransactionId} stack (open transaction only). */
  readonly transaction!: KitActiveTransactionSurface;

  // #region ­ƒöûPrivate State
  private backbone?: Backbone;
  private validationState: ValidationState = { ok: true, errors: [], warnings: [], infos: [] };
  #phase: KitPhase = "ready";
  #conflict?: Conflict;
  #conflicted = false;
  private strictMode: boolean = false;
  #revision = 0;
  #auditLog: KitAuditEntry[] = [];
  #openTransactions = new Map<string, KitRuntimeTransaction>();
  /** Finalized local transactions only (public {@link KitImpl.undo} / {@link KitImpl.redo}). */
  #historyDone: KitGraphChange[] = [];
  #historyUndone: KitGraphChange[] = [];
  #flattenMerkleByDesign = new Map<string, { [pieceId: string]: FlatMerkleCacheEntry }>();
  #listeners: Set<() => void> = new Set();
  /** Outbound backbone notifications (non-blocking; flushed on a microtask). */
  #backboneOutbound: KitGraphChange[] = [];
  #backboneOutboundFrozen: KitGraphChange[] = [];
  #backboneFlushScheduled = false;
  #deferredInboundQueue: KitDiff[] = [];
  /** When set, {@link _applyDiff} records steps on this open transaction unless `transactionId` is passed explicitly. */
  #activeTransactionId?: string;
  /**
   * Backbone-synced committed kit (no open interaction overlays).
   * The live graph is the effective view: {@link #reprojectEffectiveView} = committed + composed {@link #openTransactions} nets.
   */
  #committedPlain!: KitData;
  // #endregion ­ƒöûPrivate State

  /**
   * Applies a {@link KitDiff} in place with no validation ÔÇö undo/redo replay, document hydration, and tests only.
   * Domain edits must use semantic methods / {@link KitImpl._applyDiff} (internal pipeline).
   * Not allowed while interactions are open (would break committed vs overlay invariant).
   */
  replayChangeUnchecked(diff: KitDiff): void {
    if (this.#openTransactions.size > 0) {
      throw new Error("replayChangeUnchecked is not allowed while interactions are open; abort or finalize them first.");
    }
    this.#applyRawKitDiff(diff);
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(this)));
  }

  #applyRawKitDiff(diff: KitDiff): void {
    if ("name" in diff) this.name = diff.name!;
    if ("version" in diff) this.version = diff.version!;
    if (diff.updatedAt !== undefined) this.updatedAt = diff.updatedAt;

    const optionalScalars = ["description", "icon", "image", "remote", "homepage", "license", "preview"] as const;
    for (const key of optionalScalars) {
      if (key in diff) {
        (this as any)[key] = diff[key] ?? undefined;
      }
    }

    if (diff.types) {
      if (!this.types) this.types = [];
      applyCollectionDiff("type", this.types, diff.types, applyTypeDiff, (raw) => new Type(raw as TypePlain, this));
    }
    if (diff.designs) {
      if (!this.designs) this.designs = [];
      applyCollectionDiff("design", this.designs, diff.designs, applyDesignDiffCore, (raw) => new Design(raw as DesignPlain, this));
    }
    if (diff.tags) {
      if (!this.tags) this.tags = [];
      applyTagsDiff(this.tags, diff.tags);
    }
    if (diff.concepts) {
      if (!this.concepts) this.concepts = [];
      applyConceptsDiff(this.concepts, diff.concepts);
    }
    if (diff.families) {
      if (!this.families) this.families = [];
      applyFamiliesDiff(this.families, diff.families);
    }
    if (diff.qualities) {
      if (!this.qualities) this.qualities = [];
      applyCollectionDiff("quality", this.qualities, diff.qualities, applyQualityDiff, (raw) => new Quality(raw as QualityPlain));
    }
    if (diff.files) {
      if (!this.files) this.files = [];
      applyCollectionDiff("file", this.files, diff.files, applyFileDiff, (raw) => new File(raw as FilePlain));
    }
    if (diff.folders) {
      if (!this.folders) this.folders = [];
      applyCollectionDiff("folder", this.folders, diff.folders, applyFolderDiff, (raw) => new Folder(raw as FolderPlain));
    }
    if (diff.authors) {
      if (!this.authors) this.authors = [];
      applyCollectionDiff("author", this.authors, diff.authors, applyAuthorDiff, (raw) => new Author(raw as AuthorPlain));
    }
    if (diff.attributes) {
      if (!this.attributes) this.attributes = [];
      applyAttributesDiff(this.attributes, diff.attributes);
    }
  }

  /** Inverse diff for this kit's state *before* `appliedDiff` is applied. */
  inverseDiffFromPreApplyState(appliedDiff: KitDiff): KitDiff {
    return inverseKitGraphDiff(this, appliedDiff);
  }

  static mergeGraphDiffs(diff1: KitDiff, diff2: KitDiff): KitDiff {
    return mergeKitGraphDiff(diff1, diff2);
  }

  static changeBetween(before: KitLike, after: KitLike): KitGraphChange {
    const b = asKitInstance(before);
    const a = asKitInstance(after);
    const forward = computeKitGraphDiffBetween(b, a);
    const backward = inverseKitGraphDiff(b, forward);
    const validation = validateKitGraphDiff(b, forward, false);
    return { forward, backward, validation };
  }

  validateGraphDiff(diff: KitDiff, heal: boolean = false): KitDiffValidationResult {
    return validateKitGraphDiff(this, diff, heal);
  }

  /** Deep clone of a diff (isolation for parallel apply experiments). */
  static cloneGraphDiff(diff: KitDiff): KitDiff {
    return KitDiffSchema.parse(JSON.parse(JSON.stringify(diff)));
  }

  /**
   * Empty in-memory kit (single live graph). Prefer {@link Kit}() for the spec entry point.
   */
  static open(backbone?: Backbone): KitImpl {
    const now = new Date().toISOString();
    return new KitImpl(
      KitSchema.parse({
        id: id(),
        name: "Kit",
        version: "0",
        types: [],
        designs: [],
        tags: [],
        concepts: [],
        families: [],
        qualities: [],
        files: [],
        folders: [],
        authors: [],
        attributes: [],
        createdAt: now,
        updatedAt: now,
      }),
      backbone,
    );
  }

  /** Coerce wire {@link KitData} or pass through a live {@link KitImpl}. */
  static ensure(like: KitLike): KitImpl {
    return like instanceof KitImpl ? like : new KitImpl(KitSchema.parse(stripNullsJsonClone(like) as unknown));
  }

  /** Semio world basis ÔåÆ three.js scene root (same transform as {@link toThreeRotation}). */
  static semioToThreeRootBasis(): THREE.Matrix4 {
    return toThreeRotation();
  }

  #scheduleBackboneNotify(change: KitGraphChange): void {
    if (!this.backbone) return;
    if (this.#phase === "frozen") {
      this.#backboneOutboundFrozen.push(change);
      return;
    }
    this.#backboneOutbound.push(change);
    this.#flushBackboneOutboundSoon();
  }

  constructor(plain: KitData, backbone?: Backbone) {
    const p = KitSchema.parse(plain);
    Object.assign(this, p);
    this.types = p.types?.map((t) => new Type(t, this));
    this.designs = p.designs?.map((d) => new Design(d, this));
    this.tags = p.tags?.map((t) => new Tag(t));
    this.concepts = p.concepts?.map((c) => new Concept(c));
    this.families = p.families?.map((x) => new Family(x));
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.files = p.files?.map((f) => new File(f));
    this.folders = p.folders?.map((f) => new Folder(f));
    this.authors = p.authors?.map((a) => new Author(a));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
    this.backbone = backbone;
    if (backbone?.attach) {
      queueMicrotask(() => {
        try {
          const maybe = backbone.attach!(this, (inbound) => {
            if (this.#phase === "frozen") {
              this.#deferredInboundQueue.push(inbound);
              return;
            }
            this._applyDiff(inbound, { notifyBackbone: false, skipGlobalHistory: true, inboundCommitted: true });
          });
          void Promise.resolve(maybe).catch((err) => console.error("Backbone attach error:", err));
        } catch (err) {
          console.error("Backbone attach error:", err);
        }
      });
    }
    this.ops = new KitOps(this);
    this.transactions = new KitTransactionsCoordinator(this);
    this.transaction = new KitActiveTransactionSurface(this);
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(this)));
  }

  /** Committed (synced) snapshot without interaction overlays. */
  getCommittedPlain(): KitData {
    return KitSchema.parse(stripNullsJsonClone(this.#committedPlain));
  }

  /** Align committed snapshot with the current effective graph when no interactions are open. */
  syncCommittedPlainFromGraph(): void {
    if (this.#openTransactions.size > 0) {
      throw new Error("syncCommittedPlainFromGraph requires all interactions to be closed.");
    }
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(this)));
  }

  get activeTransactionId(): string | undefined {
    return this.#activeTransactionId;
  }

  /**
   * Select which open transaction receives domain edits when {@link KitChangeOptions.transactionId} is omitted on {@link _applyDiff}.
   */
  setActiveTransaction(transactionId: string): void {
    if (!this.#openTransactions.has(transactionId)) {
      throw new Error(`Transaction ${transactionId} is not open on this kit.`);
    }
    this.#activeTransactionId = transactionId;
  }

  clearActiveTransaction(): void {
    this.#activeTransactionId = undefined;
  }

  /** Clears {@link activeTransactionId} (alias for {@link clearActiveTransaction}). */
  unsetActiveTransaction(): void {
    this.clearActiveTransaction();
  }

  requireActiveTransactionId(): string {
    if (!this.#activeTransactionId) {
      throw new Error("No active transaction; call setActiveTransaction after transactions.start.");
    }
    if (!this.#openTransactions.has(this.#activeTransactionId)) {
      throw new Error(`Active transaction ${this.#activeTransactionId} is not open.`);
    }
    return this.#activeTransactionId;
  }

  /**
   * Load kit JSON from a folder (`kit.json`) or a direct `.json` path (Node.js). Replaces the live graph in place.
   */
  async importLocal(folderPath: string): Promise<void> {
    const isNode = typeof process !== "undefined" && process.versions?.node;
    if (!isNode) {
      throw new Error("Kit.importLocal requires Node.js; in the browser use importKit(blobOrUrl).");
    }
    const fs = await import("node:fs/promises");
    const path = await import("node:path");
    const resolved = folderPath.endsWith(".json") ? folderPath : path.join(folderPath, "kit.json");
    const text = await fs.readFile(resolved, "utf8");
    const { kit: loaded } = await importFileKit(text);
    const diff = computeKitGraphDiffBetween(this, loaded);
    this.#openTransactions.clear();
    this.#activeTransactionId = undefined;
    this.replayChangeUnchecked(diff);
    this.#historyDone.length = 0;
    this.#historyUndone.length = 0;
    this.#flattenMerkleByDesign.clear();
    this.#revision++;
    this.#phase = "ready";
    this.#conflicted = false;
    this.#conflict = undefined;
    this.validationState = { ok: true, errors: [], warnings: [], infos: [] };
    this.notify();
  }

  /**
   * Builds a full persist {@link DesignDiff} (forward/backward) and updates the merkle flatten cache.
   * Use {@link KitImpl.ensureFlattenGeometryCache} when you only need resolved plane/center for rendering or {@link Piece.flatPlane}.
   */
  flattenDesignMerkle(designId: string) {
    const prev = this.#flattenMerkleByDesign.get(designId);
    const { result, cache } = this.#flattenDesignCached(designId, prev);
    this.#flattenMerkleByDesign.set(designId, cache);
    return result;
  }

  /** Latest merkle flatten cache for a design (after {@link flattenDesignMerkle}). */
  getFlattenMerkleCache(designId: string): { [pieceId: string]: FlatMerkleCacheEntry } | undefined {
    return this.#flattenMerkleByDesign.get(designId);
  }

  /** Clears all flatten caches (call after design topology changes if consumers rely on fresh geometry). */
  invalidateFlattenMerkleCaches(): void {
    this.#flattenMerkleByDesign.clear();
  }

  /** Per-piece merkle hashes for flatten cache identity (same chain as incremental flatten). */
  #computeFlatMerkleHashes(designId: string): { [pieceId: string]: FlatMerkleHashes } {
    const design = this.requireDesign(designId);
    const pieces = design.pieces ?? [];
    if (pieces.length === 0) return {};
    const { getType, getConnector } = this.buildConnectorResolver();
    const connections = (design._connections ?? []).filter((c) => pieces.some((p) => p.id === c.connected.piece.id) && pieces.some((p) => p.id === c.connecting.piece.id));
    const planeHashes: { [id: string]: string } = {};
    const centerHashes: { [id: string]: string } = {};
    const { pieceMap, adjacency } = buildFlattenPieceAdjacency(pieces, connections);
    flattenPlacementWalkDesignOrderRoots(pieceMap, adjacency, pieces, {
      initRoot: (rootId, rootPiece) => {
        if (!rootPiece.id) return;
        planeHashes[rootPiece.id] = hashPlaneRoot(rootPiece.id, rootPiece.plane);
        centerHashes[rootPiece.id] = hashCenterRoot(rootPiece.id, rootPiece.center);
      },
      onTreeEdge: ({ parentId, childId, connection, parentPiece, childPiece }) => {
        if (!parentPiece.id || !childPiece.id) return;
        const parentPlaneHash = planeHashes[parentPiece.id];
        const parentCenterHash = centerHashes[parentPiece.id];
        if (!parentPlaneHash || !parentCenterHash) return;
        const parentSide = connection.connected.piece.id === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.id === childId ? connection.connecting : connection.connected;
        const parentType = resolvePieceTypeForFlatten(parentPiece, getType);
        const childType = resolvePieceTypeForFlatten(childPiece, getType);
        const parentConnector = getConnector(parentType, parentSide.connector?.id);
        const childConnector = getConnector(childType, childSide.connector?.id);
        if (!parentConnector || !childConnector) return;
        planeHashes[childPiece.id] = hashPlaneChain(parentPlaneHash, parentConnector, childConnector, connection);
        centerHashes[childPiece.id] = hashCenterChain(parentCenterHash, parentConnector, connection);
      },
    });
    const result: { [id: string]: FlatMerkleHashes } = {};
    for (const id of Object.keys(planeHashes)) {
      result[id] = { planeHash: planeHashes[id], centerHash: centerHashes[id] };
    }
    return result;
  }

  /** Uncached flatten implementation for {@link KitImpl.runFlattenDesign}. */
  #flattenDesignUncached(designId: string): DesignOperationResult {
    const design = this.findDesign(designId);
    if (!design) {
      return operationErr([{ code: "flatten.design-not-found", message: `Design ${designId} not found in kit ${this.name}` }]);
    }

    if (!design.pieces || design.pieces.length === 0) {
      return operationOk({ forward: {}, backward: {} }, [], [{ code: "flatten.empty-pieces", message: "No pieces to flatten; returning empty forward and backward diffs." }]);
    }

    const warnings: OperationNote[] = [];
    const infos: OperationNote[] = [];
    const placementErrors: OperationNote[] = [];

    const { getType, getConnector } = this.buildConnectorResolver();

    const flatPieces: Piece[] = design.pieces.map(
      (p) =>
        new Piece({
          ...PieceSchema.parse(stripNullsJsonClone(p.toPlain()) as unknown),
          attributes: p.attributes?.map((a) => a.toPlain()),
        }),
    );
    const flatDesign = new Design({
      ...DesignSchema.parse(stripNullsJsonClone(design.toPlain()) as unknown),
      pieces: flatPieces.map((x) => x.toPlain()),
      connections: design._connections?.map((c) => ConnectionSchema.parse(stripNullsJsonClone(c.toPlain()) as unknown)),
    });

    const piecePlanes: { [pieceId: string]: Plane } = {};

    const setAttributes = (piece: Piece, newAttrs: { key: string; value?: string; definition?: string }[]): Piece => {
      const existingAttrs = piece.attributes || [];
      const updatedAttrs = [...existingAttrs];
      newAttrs.forEach((newAttr) => {
        const existingIndex = updatedAttrs.findIndex((a) => a.key === newAttr.key);
        if (existingIndex >= 0) {
          updatedAttrs[existingIndex] = new Attribute({
            ...updatedAttrs[existingIndex].toPlain(),
            ...newAttr,
            id: updatedAttrs[existingIndex].id,
          });
        } else {
          updatedAttrs.push(new Attribute({ id: id(), ...newAttr }));
        }
      });
      return new Piece({ ...piece.toPlain(), attributes: updatedAttrs.map((a) => a.toPlain()) });
    };

    const filteredConnections =
      flatDesign._connections?.filter((connection) => {
        const sourceId = connection.connected.piece.id;
        const targetId = connection.connecting.piece.id;
        const sourceExists = flatPieces.some((x) => x.id === sourceId);
        const targetExists = flatPieces.some((x) => x.id === targetId);
        if (!sourceExists) {
          warnings.push({
            code: "flatten.connection-skipped-missing-endpoint",
            message: `Skipping connection ${connection.id}: source piece ${sourceId} not found in design.`,
          });
          return false;
        }
        if (!targetExists) {
          warnings.push({
            code: "flatten.connection-skipped-missing-endpoint",
            message: `Skipping connection ${connection.id}: target piece ${targetId} not found in design.`,
          });
          return false;
        }
        return true;
      }) || [];

    const { pieceMap, adjacency } = buildFlattenPieceAdjacency(flatPieces, filteredConnections);
    flattenPlacementWalkDesignOrderRoots(pieceMap, adjacency, flatPieces, {
      onComponentDiscovered: (component, rootId, pm) => {
        const fixedInDesignOrder = flatPieces.map((fp) => fp.id).filter((g): g is string => Boolean(g && component.has(g) && pm[g]?.plane !== undefined && pm[g]?.center !== undefined));
        if (fixedInDesignOrder.length === 0) {
          warnings.push({
            code: "flatten.no-fixed-piece-in-clump",
            message: `Connected pieces have no fixed root (no piece with both plane and center). Using piece ${rootId} as breadth-first root. Each connected set of pieces (clump) should include at least one fixed piece for stable, recommended layout.`,
          });
        } else if (fixedInDesignOrder.length > 1) {
          infos.push({
            code: "flatten.multiple-fixed-roots",
            message: `This clump has ${fixedInDesignOrder.length} fixed pieces; using the first (${rootId}) as breadth-first root.`,
          });
        }
      },
      initRoot: (rootId, rootPiece) => {
        if (!rootPiece.id) return;
        const updatedRootPiece = setAttributes(rootPiece, [
          { key: "semio.fixedPieceId", value: rootPiece.id },
          { key: "semio.depth", value: "0" },
          { key: "semio.path", value: rootPiece.id },
        ]);
        pieceMap[rootId] = updatedRootPiece;
        let rootPlane: Plane;
        if (rootPiece.plane) {
          rootPlane = rootPiece.plane;
        } else {
          rootPlane = FLATTEN_IDENTITY_PLANE;
        }

        piecePlanes[rootPiece.id] = rootPlane;
        const rootPieceIndex = flatDesign.pieces!.findIndex((p) => p.id === rootPiece.id);
        if (rootPieceIndex !== -1) {
          flatDesign.pieces![rootPieceIndex].plane = rootPlane;

          if (!flatDesign.pieces![rootPieceIndex].center) {
            flatDesign.pieces![rootPieceIndex].center = new Coordinate({ u: 0, v: 0 });
          }

          const cur = pieceMap[rootId] ?? updatedRootPiece;
          pieceMap[rootId] = new Piece({
            ...PieceSchema.parse(stripNullsJsonClone(cur.toPlain()) as unknown),
            plane: PlaneSchema.parse(rootPlane as unknown),
            center: CoordinateSchema.parse(flatDesign.pieces![rootPieceIndex].center as unknown),
          });
        }
      },
      onTreeEdge: ({ parentId, childId, connection, depth, parentPiece, childPiece }) => {
        if (!parentPiece.id || !childPiece.id) return;
        const parentPlane = piecePlanes[parentPiece.id];
        if (!parentPlane) {
          placementErrors.push({
            code: "flatten.parent-plane-missing",
            message: `Parent piece ${parentPiece.id} has no plane while flattening edge to child ${childPiece.id}.`,
          });
          return;
        }
        const parentSide = connection.connected.piece.id === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.id === childId ? connection.connecting : connection.connected;
        const parentType = resolvePieceTypeForFlatten(parentPiece, getType);
        const childType = resolvePieceTypeForFlatten(childPiece, getType);

        const parentConnectorId = parentSide.connector?.id;
        const childConnectorId = childSide.connector?.id;
        const parentConnector = getConnector(parentType, parentConnectorId);
        const childConnector = getConnector(childType, childConnectorId);

        if (!parentConnector || !childConnector) {
          placementErrors.push({
            code: "flatten.connectors-not-found",
            message: `Connectors not found for connection between ${parentId} and ${childId}. Parent connector: ${parentConnectorId ?? "(default)"}, child connector: ${childConnectorId ?? "(default)"}.`,
          });
          return;
        }
        const childPlane = roundPlane(computeChildPlane(parentPlane, parentConnector, childConnector, connection));
        piecePlanes[childPiece.id] = childPlane;

        const radius = 2.697;
        const verticalVExtra = 1.0;
        const horizontalScale = 3.0633;
        const parentCenter = parentPiece.center || new Coordinate({ u: 0, v: 0 });
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

        const computedChildCenter = new Coordinate({
          u: round(childU),
          v: round(childV),
        });
        const childCenter: Coordinate = childPiece.center ?? computedChildCenter;

        const flatChildPiece: Piece = setAttributes(
          new Piece({
            ...PieceSchema.parse(stripNullsJsonClone(childPiece.toPlain()) as unknown),
            plane: PlaneSchema.parse(childPlane as unknown),
            center: CoordinateSchema.parse((childPiece.center ?? computedChildCenter) as unknown),
          }),
          [
            {
              key: "semio.fixedPieceId",
              value: parentPiece.attributes?.find((q) => q.key === "semio.fixedPieceId")?.value ?? "",
            },
            {
              key: "semio.parentPieceId",
              value: parentPiece.id,
            },
            {
              key: "semio.depth",
              value: depth.toString(),
            },
            {
              key: "semio.path",
              value: (parentPiece.attributes?.find((q) => q.key === "semio.path")?.value ?? "") + "," + childPiece.id,
            },
          ],
        );
        pieceMap[childId] = flatChildPiece;
      },
    });
    flatDesign.pieces = flatDesign.pieces?.map((p) => pieceMap[p.id ?? ""]);
    flatDesign._connections = [];

    let piecesWithPlanes = 0;
    let piecesWithoutPlanes = 0;
    const updatedPieces = flatDesign.pieces
      ?.map((flatPiece) => {
        if (flatPiece.plane) piecesWithPlanes++;
        else piecesWithoutPlanes++;

        const originalPiece = design.pieces?.find((p) => p.id === flatPiece.id);
        if (!originalPiece) return null;

        const pieceDiff: PieceDiff = {};

        if (flatPiece.plane && flattenPlanesDiffer(flatPiece.plane, originalPiece.plane)) {
          pieceDiff.plane = flatPiece.plane;
        }

        if (flatPiece.center && flattenCentersDiffer(flatPiece.center, originalPiece.center)) {
          pieceDiff.center = flatPiece.center;
        }
        if (!deepEqual(flatPiece.attributes, originalPiece.attributes)) {
          pieceDiff.attributes = getAttributesDiff(originalPiece.attributes ?? [], flatPiece.attributes ?? []);
        }

        if (Object.keys(pieceDiff).length === 0) return null;

        return {
          piece: { id: flatPiece.id },
          diff: pieceDiff,
        };
      })
      .filter((update) => update !== null) as Array<{ piece: PieceId; diff: PieceDiff }>;

    const removedConnections = design._connections?.map((c) => ({ id: c.id })) || [];

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
  }

  /**
   * Resolves flattened plane/center per piece into {@link KitImpl.getFlattenMerkleCache}.
   * Does not build a persist {@link DesignDiff}. Use {@link Design.flatten} / {@link KitImpl.flattenDesignMerkle} to commit layout to the kit.
   */
  ensureFlattenGeometryCache(designId: string): void {
    const design = this.findDesign(designId);
    if (!design?.pieces?.length) return;
    const prev = this.#flattenMerkleByDesign.get(designId);
    const walk = this.#runFlattenPlacementWalk(design, prev);
    this.#flattenMerkleByDesign.set(designId, walk.nextCache);
  }

  /** Shared placement walk for {@link KitImpl.ensureFlattenGeometryCache} and flatten-to-diff ({@link KitImpl.flattenDesignMerkle}). */
  #runFlattenPlacementWalk(
    design: Design,
    cache?: { [pieceId: string]: FlatMerkleCacheEntry },
  ): {
    flatPieces: Piece[];
    nextCache: { [id: string]: FlatMerkleCacheEntry };
    warnings: OperationNote[];
    infos: OperationNote[];
    placementErrors: OperationNote[];
  } {
    const warnings: OperationNote[] = [];
    const infos: OperationNote[] = [];
    const placementErrors: OperationNote[] = [];
    const { getType, getConnector } = this.buildConnectorResolver();

    const pieces = design.pieces;
    const flatPieces: Piece[] = pieces.map((p) => detachPieceForLocalMutation(p));
    const filteredConnections = (design._connections ?? []).filter((connection) => {
      const sourceId = connection.connected.piece.id;
      const targetId = connection.connecting.piece.id;
      const sourceExists = flatPieces.some((x) => x.id === sourceId);
      const targetExists = flatPieces.some((x) => x.id === targetId);
      if (!sourceExists) {
        warnings.push({ code: "flatten.connection-skipped-missing-endpoint", message: `Skipping connection ${connection.id}: source piece ${sourceId} not found in design.` });
        return false;
      }
      if (!targetExists) {
        warnings.push({ code: "flatten.connection-skipped-missing-endpoint", message: `Skipping connection ${connection.id}: target piece ${targetId} not found in design.` });
        return false;
      }
      return true;
    });

    const { pieceMap, adjacency } = buildFlattenPieceAdjacency(flatPieces, filteredConnections);
    const piecePlanes: { [id: string]: Plane } = {};
    const planeHashes: { [id: string]: string } = {};
    const centerHashes: { [id: string]: string } = {};
    const nextCache: { [id: string]: FlatMerkleCacheEntry } = {};

    const setAttributes = (piece: Piece, newAttrs: { key: string; value?: string; definition?: string }[]): Piece => {
      const existingAttrs = piece.attributes || [];
      const updatedAttrs: Attribute[] = [...existingAttrs];
      newAttrs.forEach((newAttr) => {
        const existingIndex = updatedAttrs.findIndex((a) => a.key === newAttr.key);
        if (existingIndex >= 0) {
          updatedAttrs[existingIndex] = new Attribute({ ...updatedAttrs[existingIndex].toPlain(), ...newAttr, id: updatedAttrs[existingIndex].id });
        } else {
          updatedAttrs.push(new Attribute({ id: id(), ...newAttr }));
        }
      });
      return new Piece(
        PieceSchema.parse({
          ...piece.toPlain(),
          attributes: updatedAttrs.map((a) => a.toPlain()),
        }),
        design,
        this,
      );
    };

    flattenPlacementWalkDesignOrderRoots(pieceMap, adjacency, flatPieces, {
      onComponentDiscovered: (component, rootId, pm) => {
        const fixedInDesignOrder = flatPieces.map((fp) => fp.id).filter((g): g is string => Boolean(g && component.has(g) && pm[g]?.plane !== undefined && pm[g]?.center !== undefined));
        if (fixedInDesignOrder.length === 0) {
          warnings.push({
            code: "flatten.no-fixed-piece-in-clump",
            message: `Connected pieces have no fixed root (no piece with both plane and center). Using piece ${rootId} as breadth-first root. Each connected set of pieces (clump) should include at least one fixed piece for stable, recommended layout.`,
          });
        } else if (fixedInDesignOrder.length > 1) {
          infos.push({ code: "flatten.multiple-fixed-roots", message: `This clump has ${fixedInDesignOrder.length} fixed pieces; using the first (${rootId}) as breadth-first root.` });
        }
      },
      initRoot: (rootId, rootPiece) => {
        if (!rootPiece.id) return;
        const planeHash = hashPlaneRoot(rootPiece.id, rootPiece.plane);
        const centerHash = hashCenterRoot(rootPiece.id, rootPiece.center);
        planeHashes[rootId] = planeHash;
        centerHashes[rootId] = centerHash;

        const cached = cache?.[rootId];
        const planeMatches = !!cached && cached.planeHash === planeHash;
        const centerMatches = !!cached && cached.centerHash === centerHash;

        let flatPiece: Piece;
        let rootPlane: Plane;
        let rootCenter: Coordinate;
        if (planeMatches && centerMatches && cached?.flatPiece) {
          flatPiece = cached.flatPiece;
          rootPlane = cached.plane ?? flatPiece.plane!;
          rootCenter = cached.center ?? flatPiece.center!;
        } else {
          rootPlane = planeMatches && cached?.plane ? cached.plane : (rootPiece.plane ?? matrixToPlane(new THREE.Matrix4().identity()));
          rootCenter = centerMatches && cached?.center ? cached.center : (rootPiece.center ?? new Coordinate({ u: 0, v: 0 }));
          const mergedRoot = new Piece(
            PieceSchema.parse({
              ...rootPiece.toPlain(),
              plane: rootPlane.toPlain(),
              center: rootCenter.toPlain(),
            }),
            design,
            this,
          );
          flatPiece = setAttributes(mergedRoot, [
            { key: "semio.fixedPieceId", value: rootPiece.id },
            { key: "semio.depth", value: "0" },
            { key: "semio.path", value: rootPiece.id },
          ]);
        }

        piecePlanes[rootId] = rootPlane;
        pieceMap[rootId] = flatPiece;
        const rootIdx = flatPieces.findIndex((p) => p.id === rootId);
        if (rootIdx !== -1) flatPieces[rootIdx] = flatPiece;
        nextCache[rootId] = { planeHash, centerHash, plane: rootPlane, center: rootCenter, flatPiece };
      },
      onTreeEdge: ({ parentId, childId, connection, depth, parentPiece, childPiece }) => {
        if (!parentPiece.id || !childPiece.id) return;
        const parentPlane = piecePlanes[parentPiece.id];
        const parentPlaneHash = planeHashes[parentPiece.id];
        const parentCenterHash = centerHashes[parentPiece.id];
        if (!parentPlane || !parentPlaneHash || !parentCenterHash) {
          placementErrors.push({ code: "flatten.parent-plane-missing", message: `Parent piece ${parentPiece.id} has no plane while flattening edge to child ${childPiece.id}.` });
          return;
        }
        const parentSide = connection.connected.piece.id === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.id === childId ? connection.connecting : connection.connected;
        const parentType = resolvePieceTypeForFlatten(parentPiece, getType);
        const childType = resolvePieceTypeForFlatten(childPiece, getType);
        const parentConnector = getConnector(parentType, parentSide.connector?.id);
        const childConnector = getConnector(childType, childSide.connector?.id);
        if (!parentConnector || !childConnector) {
          placementErrors.push({
            code: "flatten.connectors-not-found",
            message: `Connectors not found for connection between ${parentId} and ${childId}. Parent connector: ${parentSide.connector?.id ?? "(default)"}, child connector: ${childSide.connector?.id ?? "(default)"}.`,
          });
          return;
        }
        const planeHash = hashPlaneChain(parentPlaneHash, parentConnector, childConnector, connection);
        const centerHash = hashCenterChain(parentCenterHash, parentConnector, connection);
        planeHashes[childId] = planeHash;
        centerHashes[childId] = centerHash;

        const cached = cache?.[childId];
        const planeMatches = !!cached && cached.planeHash === planeHash;
        const centerMatches = !!cached && cached.centerHash === centerHash;

        let flatChildPiece: Piece;
        let childPlane: Plane;
        let childCenter: Coordinate;
        if (planeMatches && centerMatches && cached?.flatPiece) {
          flatChildPiece = cached.flatPiece;
          childPlane = cached.plane ?? flatChildPiece.plane!;
          childCenter = cached.center ?? flatChildPiece.center!;
        } else {
          childPlane = planeMatches && cached?.plane ? cached.plane : roundPlane(computeChildPlane(parentPlane, parentConnector, childConnector, connection));
          if (centerMatches && cached?.center) {
            childCenter = cached.center;
          } else {
            const radius = 2.697;
            const verticalVExtra = 1.0;
            const horizontalScale = 3.0633;
            const parentFlatCenter = parentPiece.center ?? new Coordinate({ u: 0, v: 0 });
            const connectionU = connection.u ?? 0;
            const connectionV = connection.v ?? 0;
            let childU: number;
            let childV: number;
            if (parentFlatCenter.u === 0 && parentFlatCenter.v === 0) {
              const angle = 2 * Math.PI * parentConnector.t;
              childU = radius * Math.sin(angle);
              childV = radius * Math.cos(angle);
            } else {
              const isVerticalConnection = Math.abs(parentConnector.direction?.z ?? 0) > 0.5;
              if (isVerticalConnection) {
                childU = parentFlatCenter.u + connectionU;
                childV = parentFlatCenter.v + connectionV + verticalVExtra;
              } else {
                childU = parentFlatCenter.u + connectionU * horizontalScale;
                childV = parentFlatCenter.v + connectionV * horizontalScale;
              }
            }
            const computedChildCenter = new Coordinate({ u: round(childU), v: round(childV) });
            childCenter = childPiece.center ?? computedChildCenter;
          }
          const mergedChild = new Piece(
            PieceSchema.parse({
              ...childPiece.toPlain(),
              plane: childPlane.toPlain(),
              center: childCenter.toPlain(),
            }),
            design,
            this,
          );
          flatChildPiece = setAttributes(mergedChild, [
            { key: "semio.fixedPieceId", value: parentPiece.attributes?.find((q) => q.key === "semio.fixedPieceId")?.value ?? "" },
            { key: "semio.parentPieceId", value: parentPiece.id },
            { key: "semio.depth", value: depth.toString() },
            { key: "semio.path", value: (parentPiece.attributes?.find((q) => q.key === "semio.path")?.value ?? "") + "," + childPiece.id },
          ]);
        }

        piecePlanes[childId] = childPlane;
        pieceMap[childId] = flatChildPiece;
        const childIdx = flatPieces.findIndex((p) => p.id === childId);
        if (childIdx !== -1) flatPieces[childIdx] = flatChildPiece;
        nextCache[childId] = { planeHash, centerHash, plane: childPlane, center: childCenter, flatPiece: flatChildPiece };
      },
    });

    return { flatPieces, nextCache, warnings, infos, placementErrors };
  }

  /**
   * Incremental flatten with optional merkle cache; computes forward/backward {@link DesignDiff} for persistence (see {@link Design.flatten}).
   */
  #flattenDesignCached(designId: string, cache?: { [pieceId: string]: FlatMerkleCacheEntry }): { result: DesignOperationResult; cache: { [pieceId: string]: FlatMerkleCacheEntry } } {
    const design = this.findDesign(designId);
    if (!design) {
      return {
        result: operationErr([{ code: "flatten.design-not-found", message: `Design ${designId} not found in kit ${this.name}` }]),
        cache: {},
      };
    }
    if (!design.pieces || design.pieces.length === 0) {
      return {
        result: operationOk({ forward: {}, backward: {} }, [], [{ code: "flatten.empty-pieces", message: "No pieces to flatten; returning empty forward and backward diffs." }]),
        cache: {},
      };
    }

    const walk = this.#runFlattenPlacementWalk(design, cache);
    const { flatPieces, nextCache, warnings, infos, placementErrors } = walk;
    const pieces = design.pieces;

    let piecesWithPlanes = 0;
    let piecesWithoutPlanes = 0;
    const updatedPieces = flatPieces
      .map((flatPiece) => {
        if (flatPiece.plane) piecesWithPlanes++;
        else piecesWithoutPlanes++;
        const originalPiece = pieces.find((p) => p.id === flatPiece.id);
        if (!originalPiece) return null;
        const pieceDiff: PieceDiff = {};
        if (flatPiece.plane && flattenPlanesDiffer(flatPiece.plane, originalPiece.plane)) pieceDiff.plane = flatPiece.plane;
        if (flatPiece.center && flattenCentersDiffer(flatPiece.center, originalPiece.center)) pieceDiff.center = flatPiece.center;
        if (!deepEqual(flatPiece.attributes, originalPiece.attributes)) pieceDiff.attributes = getAttributesDiff(originalPiece.attributes ?? [], flatPiece.attributes ?? []);
        if (Object.keys(pieceDiff).length === 0) return null;
        return { piece: { id: flatPiece.id }, diff: pieceDiff };
      })
      .filter((u) => u !== null) as Array<{ piece: PieceId; diff: PieceDiff }>;

    const removedConnections = (design._connections ?? []).map((c) => ({ id: c.id }));
    const forward = {
      pieces: updatedPieces.length > 0 ? { updated: updatedPieces } : undefined,
      connections: removedConnections.length > 0 ? { removed: removedConnections } : undefined,
    } as DesignDiff;

    if (piecesWithoutPlanes > 0) {
      placementErrors.push({ code: "flatten.piece-missing-plane", message: `After flatten, ${piecesWithoutPlanes} piece(s) still have no plane (see prior placement messages).` });
    }
    if (placementErrors.length > 0) {
      return { result: operationErr(placementErrors), cache: nextCache };
    }

    infos.push({ code: "flatten.summary", message: `Flatten removed ${removedConnections.length} connection(s); updated ${updatedPieces.length} piece record(s); ${piecesWithPlanes} piece(s) with planes.` });
    const backward = inverseDesignDiff(design, forward);
    return { result: operationOk({ forward, backward }, warnings, infos), cache: nextCache };
  }
  /** @see {@link copyDesign} */
  #copyDesignClipboard(design: Design, pieceIds: string[], connectionIds: string[]): OperationResult<Design> {
    const selectedPieceSet = new Set(pieceIds);
    const selectedConnectionSet = new Set(connectionIds);

    const kitDesign = design.id ? this.findDesign(design.id) : undefined;
    const connections = design._connections && design._connections.length > 0 ? design._connections : (kitDesign?._connections ?? []);
    const pieces = design.pieces ?? [];

    const parentMap = new Map<string, { parentId: string; connection: Connection }>();
    const childMap = new Map<string, Array<{ childId: string; connection: Connection }>>();
    for (const conn of connections) {
      parentMap.set(conn.connecting.piece.id, { parentId: conn.connected.piece.id, connection: conn });
      const parentId = conn.connected.piece.id;
      if (!childMap.has(parentId)) childMap.set(parentId, []);
      childMap.get(parentId)!.push({ childId: conn.connecting.piece.id, connection: conn });
    }

    const flatRes = this.flattenDesignMerkle(design.id);
    if (!flatRes.ok) {
      return operationErr(flatRes.errors);
    }
    const flatChange = flatRes.diff!;
    const flatDesign = detachDesignForLocalMutation(design);
    flatDesign.applyDiff(flatChange.forward);
    const flatPieceMap = new Map<string, Piece>();
    for (const p of flatDesign.pieces ?? []) {
      flatPieceMap.set(p.id, p);
    }

    const copyPieces: Piece[] = [];
    const addedPieceIds = new Set<string>();
    const copyConnections: Connection[] = [];

    for (const pieceId of pieceIds) {
      const piece = pieces.find((p) => p.id === pieceId);
      if (!piece) continue;

      const isFixed = piece.plane !== undefined;
      const pInfo = parentMap.get(pieceId);
      const isConnected = pInfo !== undefined;

      const isInternalFixed = isFixed && selectedPieceSet.has(pieceId);
      let isPpExclPcIncl = false;
      let isInternalConnected = false;

      if (isConnected && pInfo) {
        const parentPieceSelected = selectedPieceSet.has(pInfo.parentId);
        const parentConnSelected = selectedConnectionSet.has(pInfo.connection.id);
        isInternalConnected = parentPieceSelected && parentConnSelected;
        isPpExclPcIncl = !parentPieceSelected && parentConnSelected;
      }

      if (isInternalFixed || isInternalConnected) {
        copyPieces.push(detachPieceForLocalMutation(piece));
        addedPieceIds.add(pieceId);
      } else if (isPpExclPcIncl) {
        const copied: Piece = detachPieceForLocalMutation(piece);
        const flatPiece = flatPieceMap.get(pieceId);
        if (flatPiece) {
          const centerValue = flatPiece.center ? JSON.stringify(flatPiece.center) : JSON.stringify({ u: 0, v: 0 });
          const planeValue = flatPiece.plane ? JSON.stringify(flatPiece.plane) : JSON.stringify({ origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } });
          copied.attributes = [...(copied.attributes ?? []), { id: "", key: "semio.center", value: centerValue }, { id: "", key: "semio.plane", value: planeValue }];
        }
        copyPieces.push(copied);
        addedPieceIds.add(pieceId);
      } else {
        const copied: Piece = detachPieceForLocalMutation(piece);
        const flatPiece = flatPieceMap.get(pieceId);
        if (flatPiece) {
          if (flatPiece.center) copied.center = { u: flatPiece.center.u, v: flatPiece.center.v };
          if (flatPiece.plane)
            copied.plane = {
              origin: { ...flatPiece.plane.origin },
              xAxis: { ...flatPiece.plane.xAxis },
              yAxis: { ...flatPiece.plane.yAxis },
            };
          const centerValue = flatPiece.center ? JSON.stringify(flatPiece.center) : JSON.stringify({ u: 0, v: 0 });
          const planeValue = flatPiece.plane ? JSON.stringify(flatPiece.plane) : JSON.stringify({ origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } });
          copied.attributes = [...(copied.attributes ?? []), { id: "", key: "semio.center", value: centerValue }, { id: "", key: "semio.plane", value: planeValue }];
        }
        copyPieces.push(copied);
        addedPieceIds.add(pieceId);

        const subtreeQueue: string[] = [pieceId];
        const subtreeVisited = new Set<string>([pieceId]);
        const addedConnIds = new Set<string>(copyConnections.map((c) => c.id));
        while (subtreeQueue.length > 0) {
          const cur = subtreeQueue.shift()!;
          const children = childMap.get(cur) ?? [];
          for (const { childId, connection } of children) {
            if (subtreeVisited.has(childId)) continue;
            subtreeVisited.add(childId);
            if (!addedPieceIds.has(childId)) {
              const childPiece = pieces.find((p) => p.id === childId);
              if (childPiece) {
                copyPieces.push(detachPieceForLocalMutation(childPiece));
                addedPieceIds.add(childId);
              }
            }
            if (!addedConnIds.has(connection.id)) {
              copyConnections.push(detachConnectionForLocalMutation(connection));
              addedConnIds.add(connection.id);
            }
            subtreeQueue.push(childId);
          }
        }
      }
    }

    for (const connId of connectionIds) {
      const conn = connections.find((c) => c.id === connId);
      if (!conn) continue;

      const connectedId = conn.connected.piece.id;
      const connectingId = conn.connecting.piece.id;
      const connectedSelected = selectedPieceSet.has(connectedId);
      const connectingSelected = selectedPieceSet.has(connectingId);

      const isInternal = connectedSelected && connectingSelected;

      if (isInternal) {
        copyConnections.push(detachConnectionForLocalMutation(conn));
      } else {
        copyConnections.push(detachConnectionForLocalMutation(conn));

        const externalIds: string[] = [];
        if (!connectedSelected) externalIds.push(connectedId);
        if (!connectingSelected) externalIds.push(connectingId);

        for (const extId of externalIds) {
          if (!addedPieceIds.has(extId)) {
            const extPiece = pieces.find((p) => p.id === extId);
            if (extPiece) {
              const cloned: Piece = detachPieceForLocalMutation(extPiece);
              const extAttrs: Attribute[] = [...(cloned.attributes ?? []), { id: "", key: "semio.piece.origin", value: "external" }];
              const flatExtPiece = flatPieceMap.get(extId);
              if (flatExtPiece) {
                const extCenterValue = flatExtPiece.center ? JSON.stringify(flatExtPiece.center) : JSON.stringify({ u: 0, v: 0 });
                extAttrs.push({ id: "", key: "semio.center", value: extCenterValue });
              }
              cloned.attributes = extAttrs;
              copyPieces.push(cloned);
              addedPieceIds.add(extId);
            }
          }
        }
      }
    }

    return operationOk({ id: "", name: "", pieces: copyPieces, connections: copyConnections }, flatRes.warnings, [
      ...flatRes.infos,
      {
        code: "copy.summary",
        message: `Copied ${copyPieces.length} piece(s) and ${copyConnections.length} connection(s) to clipboard design.`,
      },
    ]);
  }

  /** @see {@link pasteDesign} */
  #pasteDesign(source: Design, target: Design, anchoring: string = "bottomLeft", coordinate?: Coordinate): DesignDiff {
    const typesMap = new Map<string, Type>();
    for (const t of this.types ?? []) typesMap.set(t.id, t);
    const portsMap = new Map<string, Port>();
    for (const p of (this.families ?? []).flatMap((f) => f.ports ?? [])) portsMap.set(p.id, p);

    const sourcePieces = source.pieces ?? [];
    const sourceConnections = source._connections ?? [];
    const targetPieces = target.pieces ?? [];

    const externalOriginIds = new Set<string>();
    for (const piece of sourcePieces) {
      if ((piece.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")) {
        externalOriginIds.add(piece.id);
      }
    }

    const sourcePieceMap = new Map<string, Piece>();
    for (const p of sourcePieces) sourcePieceMap.set(p.id, p);

    const sourceParentMap = new Map<string, { parentId: string; connection: Connection }>();
    for (const conn of sourceConnections) {
      const childId = conn.connecting.piece.id;
      const parentId = conn.connected.piece.id;
      const prev = sourceParentMap.get(childId);
      if (!prev) {
        sourceParentMap.set(childId, { parentId, connection: conn });
        continue;
      }
      const prevStub = externalOriginIds.has(prev.parentId);
      const nextStub = externalOriginIds.has(parentId);
      if (prevStub !== nextStub && nextStub) {
        sourceParentMap.set(childId, { parentId, connection: conn });
      }
    }

    const centerCoordinates: Coordinate[] = [];
    for (const piece of sourcePieces) {
      if (externalOriginIds.has(piece.id)) continue;
      let center: Coordinate | undefined = piece.center;
      if (!center) {
        const attr = (piece.attributes ?? []).find((a) => a.key === "semio.center");
        if (attr?.value) center = JSON.parse(attr.value) as Coordinate;
      }
      if (center) centerCoordinates.push(center);
    }
    if (centerCoordinates.length === 0) centerCoordinates.push({ u: 0, v: 0 });

    const minU = Math.min(...centerCoordinates.map((c) => c.u));
    const maxU = Math.max(...centerCoordinates.map((c) => c.u));
    const minV = Math.min(...centerCoordinates.map((c) => c.v));
    const maxV = Math.max(...centerCoordinates.map((c) => c.v));

    let anchor: Coordinate;
    switch (anchoring) {
      case "original":
        anchor = { u: 0, v: 0 };
        break;
      case "middle":
        anchor = { u: (minU + maxU) / 2, v: (minV + maxV) / 2 };
        break;
      case "centroid":
        anchor = { u: centerCoordinates.reduce((s, c) => s + c.u, 0) / centerCoordinates.length, v: centerCoordinates.reduce((s, c) => s + c.v, 0) / centerCoordinates.length };
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

    const targetPiecesByName = new Map<string, Piece[]>();
    for (const tp of targetPieces) {
      if (tp.name) {
        if (!targetPiecesByName.has(tp.name)) targetPiecesByName.set(tp.name, []);
        targetPiecesByName.get(tp.name)!.push(tp);
      }
    }

    const arePortsCompatible = (pg1?: string, pg2?: string): boolean => {
      if (!pg1 || !pg2) return false;
      if (pg1 === pg2) return true;
      const p1 = portsMap.get(pg1);
      const p2 = portsMap.get(pg2);
      if (!p1 || !p2) return false;
      return (p1.compatiblePorts ?? []).some((cp) => cp.id === pg2) || (p2.compatiblePorts ?? []).some((cp) => cp.id === pg1);
    };

    const findMatchingConnector = (typeId: string, sourceConnector: Connector): Connector | undefined => {
      const t = typesMap.get(typeId);
      if (!t) return undefined;
      return (t.connectors ?? []).find((c) => {
        const nameMatch = (sourceConnector.name ?? "") !== "" && c.name === sourceConnector.name;
        const idMatch = c.id === sourceConnector.id;
        if (!nameMatch && !idMatch) return false;
        return arePortsCompatible(c.port?.id, sourceConnector.port?.id);
      });
    };

    const canRematchExternalParentPiece = (piece: Piece, pInfo: { parentId: string; connection: Connection }): boolean => {
      if (!externalOriginIds.has(pInfo.parentId)) return false;
      const externalParent = sourcePieceMap.get(pInfo.parentId);
      if (!externalParent) return false;
      const extName = externalParent.name ?? "";
      if (!extName || !targetPiecesByName.has(extName)) return false;
      const parentConn = pInfo.connection;
      const isParentConnected = parentConn.connected.piece.id === pInfo.parentId;
      const parentConnectorId = isParentConnected ? parentConn.connected.connector?.id : parentConn.connecting.connector?.id;
      if (!parentConnectorId || !externalParent.type?.id) return false;
      const parentType = typesMap.get(externalParent.type.id);
      const sourceParentConnector = parentType?.connectors?.find((c) => c.id === parentConnectorId);
      if (!sourceParentConnector) return false;
      const candidates = targetPiecesByName.get(extName)!;
      return candidates.some((candidate) => {
        if (!candidate.type?.id) return false;
        return findMatchingConnector(candidate.type.id, sourceParentConnector) !== undefined;
      });
    };

    const addedPieces: Piece[] = [];
    const addedConnections: Connection[] = [];

    for (const piece of sourcePieces) {
      if (externalOriginIds.has(piece.id)) continue;

      const isFixed = piece.plane !== undefined;
      const pInfo = sourceParentMap.get(piece.id);
      const isConnected = pInfo !== undefined;

      if (isConnected && pInfo && externalOriginIds.has(pInfo.parentId)) {
        const externalParent = sourcePieceMap.get(pInfo.parentId)!;
        let matched = false;

        if (canRematchExternalParentPiece(piece, pInfo)) {
          const extName = externalParent.name ?? "";
          const candidates = targetPiecesByName.get(extName)!;
          const parentConn = pInfo.connection;
          const isParentConnected = parentConn.connected.piece.id === pInfo.parentId;
          const parentConnectorId = isParentConnected ? parentConn.connected.connector?.id : parentConn.connecting.connector?.id;

          let sourceParentConnector: Connector | undefined;
          if (externalParent.type?.id) {
            const parentType = typesMap.get(externalParent.type.id);
            if (parentType) {
              sourceParentConnector = (parentType.connectors ?? []).find((c) => c.id === parentConnectorId);
            }
          }

          if (sourceParentConnector) {
            for (const candidate of candidates) {
              if (!candidate.type?.id) continue;
              const matchingConnector = findMatchingConnector(candidate.type.id, sourceParentConnector);
              if (matchingConnector) {
                matched = true;
                addedPieces.push(detachPieceForLocalMutation(piece));

                const copiedConn: Connection = detachConnectionForLocalMutation(parentConn);
                if (isParentConnected) {
                  copiedConn.connected = { piece: { id: candidate.id }, connector: { id: matchingConnector.id } };
                } else {
                  copiedConn.connecting = { piece: { id: candidate.id }, connector: { id: matchingConnector.id } };
                }

                if (coordinate) {
                  const connectedStub = externalOriginIds.has(parentConn.connected.piece.id);
                  const connectingStub = externalOriginIds.has(parentConn.connecting.piece.id);
                  const connMatchesParentage =
                    (parentConn.connecting.piece.id === piece.id && parentConn.connected.piece.id === pInfo.parentId) || (parentConn.connected.piece.id === piece.id && parentConn.connecting.piece.id === pInfo.parentId);
                  if (connMatchesParentage && connectedStub !== connectingStub) {
                    let flatParentCenter: Coordinate | undefined;
                    if (candidate.center) flatParentCenter = { u: candidate.center.u, v: candidate.center.v };
                    else {
                      const candAttr = (candidate.attributes ?? []).find((a) => a.key === "semio.center");
                      if (candAttr?.value) flatParentCenter = JSON.parse(candAttr.value) as Coordinate;
                    }
                    if (!flatParentCenter) {
                      const epCenterAttr = (externalParent.attributes ?? []).find((a) => a.key === "semio.center");
                      if (epCenterAttr?.value) flatParentCenter = JSON.parse(epCenterAttr.value) as Coordinate;
                      else if (externalParent.center) flatParentCenter = externalParent.center;
                    }

                    let flatChildCenter: Coordinate | undefined;
                    const childCenterAttr = (piece.attributes ?? []).find((a) => a.key === "semio.center");
                    if (childCenterAttr?.value) flatChildCenter = JSON.parse(childCenterAttr.value) as Coordinate;
                    else if (piece.center) flatChildCenter = piece.center;

                    if (flatParentCenter && flatChildCenter) {
                      copiedConn.u = flatParentCenter.u - (coordinate.u + (anchor.u - flatChildCenter.u));
                      copiedConn.v = flatParentCenter.v - (coordinate.v + (anchor.v - flatChildCenter.v));
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
          const copied: Piece = detachPieceForLocalMutation(piece);
          const attrs = piece.attributes ?? [];
          const centerAttr = attrs.find((a) => a.key === "semio.center");
          const planeAttr = attrs.find((a) => a.key === "semio.plane");
          if (centerAttr?.value) copied.center = JSON.parse(centerAttr.value);
          if (planeAttr?.value) copied.plane = JSON.parse(planeAttr.value);
          const c = copied.center ?? { u: 0, v: 0 };
          copied.center = { u: c.u - anchor.u + (coordinate?.u ?? 0), v: c.v - anchor.v + (coordinate?.v ?? 0) };
          addedPieces.push(copied);
        }
      } else if (isFixed) {
        const copied: Piece = detachPieceForLocalMutation(piece);
        let cu = 0;
        let cv = 0;
        if (copied.center) {
          cu = copied.center.u;
          cv = copied.center.v;
        } else {
          const centerAttr = (copied.attributes ?? []).find((a) => a.key === "semio.center");
          if (centerAttr?.value) {
            const parsed = JSON.parse(centerAttr.value) as Coordinate;
            cu = parsed.u;
            cv = parsed.v;
          }
        }
        copied.center = { u: cu - anchor.u + (coordinate?.u ?? 0), v: cv - anchor.v + (coordinate?.v ?? 0) };
        addedPieces.push(copied);
      } else if (isConnected && pInfo) {
        addedPieces.push(detachPieceForLocalMutation(piece));
      }
    }

    const addedPieceIds = new Set(addedPieces.map((p) => p.id));
    for (const conn of sourceConnections) {
      if (externalOriginIds.has(conn.connected.piece.id) || externalOriginIds.has(conn.connecting.piece.id)) continue;
      if (!addedPieceIds.has(conn.connected.piece.id) || !addedPieceIds.has(conn.connecting.piece.id)) continue;
      addedConnections.push(detachConnectionForLocalMutation(conn));
    }

    const diff: DesignDiff = {};
    if (addedPieces.length > 0) diff.pieces = { added: addedPieces };
    if (addedConnections.length > 0) diff.connections = { added: addedConnections };
    return diff;
  }

  #appendAudit(tag: string, change?: KitGraphChange): void {
    this.#auditLog.push({ revision: this.#revision, tag, change });
  }

  #freezeConflict(kind: ConflictKind, report: KitDiffValidationResult, extra?: { txId?: string; diff?: KitDiff }): void {
    this.#phase = "frozen";
    this.#conflicted = true;
    this.validationState = report;
    this.#conflict = {
      id: id(),
      kind,
      txId: extra?.txId,
      proposedDiff: extra?.diff,
      validationReport: report,
      createdAt: new Date().toISOString(),
    };
  }

  #invalidateCachesTouchedByDiff(diff: KitDiff): void {
    if (diff.designs) {
      for (const x of diff.designs.added ?? []) this.#flattenMerkleByDesign.delete(x.id);
      for (const x of diff.designs.removed ?? []) this.#flattenMerkleByDesign.delete(x.id);
      for (const u of diff.designs.updated ?? []) {
        if (u.design?.id) this.#flattenMerkleByDesign.delete(u.design.id);
      }
    }
  }

  #entityVersionHashFor(entityId: string): string {
    for (const d of this.designs ?? []) {
      const piece = d.pieces?.find((p) => p.id === entityId);
      if (piece) return hashPiece(piece);
      const conn = d._connections?.find((c) => c.id === entityId);
      if (conn) return hashConnection(conn);
    }
    const ty = this.findType(entityId);
    if (ty) return hashType(ty);
    return "";
  }

  #normalizeInboundTypeRemovalsToTombstones(diff: KitDiff, meta?: { deletedByUserId?: string; deletedByDisplayName?: string; deletedInChangeId?: string }): KitDiff {
    if (!diff.types?.removed?.length) return diff;
    const now = new Date().toISOString();
    const extraUpdated =
      diff.types.removed?.map((r) => ({
        type: { id: r.id },
        diff: {
          lifecycle: "deleted" as const,
          deletedAt: now,
          deletedByUserId: meta?.deletedByUserId,
          deletedByDisplayName: meta?.deletedByDisplayName,
          deletedInChangeId: meta?.deletedInChangeId,
        },
      })) ?? [];
    const prevUpdated = diff.types.updated ?? [];
    const { removed: _r, ...restTypes } = diff.types;
    const next: KitDiff = {
      ...diff,
      types: {
        ...restTypes,
        updated: [...prevUpdated, ...extraUpdated],
      },
    };
    delete (next.types as { removed?: unknown }).removed;
    if (next.types && Object.keys(next.types).length === 0) delete next.types;
    return next;
  }

  #extractTypesDeletedByDiff(diff: KitDiff): Set<string> {
    const out = new Set<string>();
    if (diff.types?.updated) {
      for (const u of diff.types.updated) {
        const ld = u.diff as TypeDiff;
        if (ld.lifecycle === "deleted") out.add(u.type.id);
      }
    }
    if (diff.types?.removed) {
      for (const r of diff.types.removed) out.add(r.id);
    }
    return out;
  }

  #detectConcurrentDeleteConflicts(appliedDiff: KitDiff, inbound?: { changeId?: string; actorId?: string; actorDisplayName?: string }): void {
    const deleted = this.#extractTypesDeletedByDiff(appliedDiff);
    if (deleted.size === 0) return;
    for (const typeId of deleted) {
      const t = this.findType(typeId);
      if (!t || (t.lifecycle ?? "active") !== "deleted") continue;
      for (const [txId, tx] of this.#openTransactions) {
        if (tx.status !== "open" || !tx.touchedEntities.has(typeId)) continue;
        if (tx.conflicts.some((c) => c.entityId === typeId)) continue;
        const c: ConcurrentDeleteConflict = {
          id: id(),
          entityKind: "Type",
          entityId: typeId,
          localInteractionId: txId,
          localPendingChanges: [...tx.done],
          deletedByUserId: inbound?.actorId,
          deletedByDisplayName: inbound?.actorDisplayName,
          deletedAt: t.deletedAt,
          deleteChangeId: inbound?.changeId ?? t.deletedInChangeId,
          proposedResolutions: ["discardLocalChanges", "restoreEntityAndReplayLocalChanges"],
        };
        tx.conflicts.push(c);
        tx.workspaceStatus = "conflicted";
      }
    }
  }

  #flushBackboneOutboundSoon(): void {
    const bb = this.backbone;
    if (!bb || this.#phase === "frozen") return;
    if (this.#backboneFlushScheduled) return;
    this.#backboneFlushScheduled = true;
    queueMicrotask(() => {
      this.#backboneFlushScheduled = false;
      const batch = this.#backboneOutbound.splice(0, this.#backboneOutbound.length);
      for (const ch of batch) {
        void bb.changed(ch).catch((err) => console.error("Backbone sync error:", err));
      }
    });
  }

  /** Replace live graph from a scratch {@link KitImpl} (same scalar + entity wiring as constructor). */
  #adoptGraphFrom(source: KitImpl): void {
    const p = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(source)));
    this.id = p.id;
    this.name = p.name;
    this.version = p.version;
    this.remote = p.remote;
    this.homepage = p.homepage;
    this.license = p.license;
    this.preview = p.preview;
    this.icon = p.icon;
    this.image = p.image;
    this.description = p.description;
    this.createdAt = p.createdAt;
    this.updatedAt = p.updatedAt;
    this.types = p.types?.map((t) => new Type(t, this));
    this.designs = p.designs?.map((d) => new Design(d, this));
    this.tags = p.tags?.map((t) => new Tag(t));
    this.concepts = p.concepts?.map((c) => new Concept(c));
    this.families = p.families?.map((x) => new Family(x));
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.files = p.files?.map((f) => new File(f));
    this.folders = p.folders?.map((f) => new Folder(f));
    this.authors = p.authors?.map((a) => new Author(a));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
    this.#flattenMerkleByDesign.clear();
  }

  /**
   * Effective kit graph = {@link #committedPlain} + each open interactionÔÇÖs squashed net forward (Map insertion order).
   */
  #reprojectEffectiveView(): void {
    const base = KitSchema.parse(stripNullsJsonClone(this.#committedPlain));
    const shell = new KitImpl(base, undefined);
    for (const [, tx] of this.#openTransactions) {
      if (tx.status !== "open" || tx.done.length === 0) continue;
      const nf = DiffComposer.normalize(tx.netForward);
      if (nf && Object.keys(nf).length > 0) shell.replayChangeUnchecked(nf);
    }
    this.#adoptGraphFrom(shell);
  }

  /**
   * Opens a named transaction context (multiple may be open). Steps use {@link KitImpl._applyDiff}(ÔÇª, { transactionId: tx.id }).
   */
  beginTransaction(label?: string): Transaction {
    if (this.#phase === "frozen" || this.#conflicted) {
      throw new Error("KitImpl has unresolved validation conflicts; call resolveConflict() before starting a transaction.");
    }
    const id = id();
    this.#openTransactions.set(id, {
      label,
      status: "open",
      startPlain: KitSchema.parse(stripNullsJsonClone(this.#committedPlain)),
      done: [],
      undone: [],
      netForward: {},
      netBackward: {},
      baseRevision: this.#revision,
      touchedEntities: new Set(),
      touchedVersions: new Map(),
      workspaceStatus: "clean",
      conflicts: [],
    });
    return new Transaction(this, id, label);
  }

  /** @deprecated Prefer {@link KitImpl.beginTransaction}. */
  startTransaction(): string {
    return this.beginTransaction().id;
  }

  _getTransactionStatus(id: string): TransactionStatus {
    return this.#openTransactions.get(id)?.status ?? "finalized";
  }

  _transactionFinalize(transactionId: string): KitGraphChange | undefined {
    const tx = this.#openTransactions.get(transactionId);
    if (!tx) throw new Error(`Unknown transaction ${transactionId}`);
    if (this.#phase === "frozen" || this.#conflicted) {
      throw new Error("KitImpl is conflicted; call resolveConflict() before finalizing a transaction.");
    }
    if (tx.workspaceStatus === "conflicted") {
      throw new Error("Resolve concurrent delete conflicts on this interaction before finalizing.");
    }
    if (tx.done.length === 0) {
      tx.status = "finalized";
      this.#openTransactions.delete(transactionId);
      if (this.#activeTransactionId === transactionId) this.#activeTransactionId = undefined;
      this.#reprojectEffectiveView();
      this.notify();
      return undefined;
    }
    const sk = new KitImpl(KitSchema.parse(stripNullsJsonClone(this.#committedPlain)), undefined);
    const validation = validateKitGraphDiff(sk, tx.netForward, false);
    if (!validation.ok || validation.errors.length > 0) {
      this.#freezeConflict("LocalChange", validation, { txId: transactionId, diff: tx.netForward });
      throw new Error(`Transaction finalize validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
    }
    if (this.strictMode && validation.warnings.length > 0) {
      this.#freezeConflict("LocalChange", validation, { txId: transactionId, diff: tx.netForward });
      throw new Error(`Transaction finalize warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
    }
    const diffToApply = validation.diff ?? tx.netForward;
    const backward = sk.inverseDiffFromPreApplyState(diffToApply);
    sk.replayChangeUnchecked(diffToApply);
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(sk)));
    const squashed: KitGraphChange = { forward: diffToApply, backward, validation };
    tx.status = "finalized";
    this.#openTransactions.delete(transactionId);
    if (this.#activeTransactionId === transactionId) this.#activeTransactionId = undefined;
    this.#historyDone.push(squashed);
    this.#historyUndone.length = 0;
    this.#appendAudit("TransactionFinalized", squashed);
    this.#scheduleBackboneNotify(squashed);
    this.#reprojectEffectiveView();
    this.notify();
    return squashed;
  }

  _transactionAbort(transactionId: string): void {
    const tx = this.#openTransactions.get(transactionId);
    if (!tx) throw new Error(`Unknown transaction ${transactionId}`);
    if (this.#phase === "frozen" || this.#conflicted) {
      throw new Error("KitImpl is conflicted; call resolveConflict() before aborting a transaction.");
    }
    tx.status = "aborted";
    this.#openTransactions.delete(transactionId);
    if (this.#activeTransactionId === transactionId) this.#activeTransactionId = undefined;
    this.#reprojectEffectiveView();
    this.#appendAudit("TransactionAborted");
    this.validationState = { ok: true, errors: [], warnings: [], infos: [] };
    this.notify();
  }

  _transactionUndo(transactionId: string): void {
    const tx = this.#openTransactions.get(transactionId);
    if (!tx || tx.done.length === 0) return;
    if (this.#phase === "frozen" || this.#conflicted) throw new Error("KitImpl is conflicted.");
    const ch = tx.done.pop()!;
    tx.undone.push(ch);
    recomputeTxNet(tx);
    this.#reprojectEffectiveView();
    this.#revision++;
    this.#appendAudit("TxUndoApplied", ch);
    this.notify();
  }

  _transactionRedo(transactionId: string): void {
    const tx = this.#openTransactions.get(transactionId);
    if (!tx || tx.undone.length === 0) return;
    if (this.#phase === "frozen" || this.#conflicted) throw new Error("KitImpl is conflicted.");
    const ch = tx.undone.pop()!;
    const validation = validateKitGraphDiff(this, ch.forward, false);
    if (!validation.ok || validation.errors.length > 0) {
      tx.undone.push(ch);
      this.#freezeConflict("TxRedo", validation, { txId: transactionId, diff: ch.forward });
      throw new Error(`Transaction redo validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
    }
    if (this.strictMode && validation.warnings.length > 0) {
      tx.undone.push(ch);
      this.#freezeConflict("TxRedo", validation, { txId: transactionId, diff: ch.forward });
      throw new Error(`Transaction redo warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
    }
    const diffToApply = validation.diff ?? ch.forward;
    const backward = this.inverseDiffFromPreApplyState(diffToApply);
    const ch2: KitGraphChange = { forward: diffToApply, backward, validation, preconditions: ch.preconditions };
    tx.done.push(ch2);
    recomputeTxNet(tx);
    this.#reprojectEffectiveView();
    this.#revision++;
    this.#appendAudit("TxRedoApplied", ch2);
    this.notify();
  }

  abortTransaction(transactionId: string): void {
    this._transactionAbort(transactionId);
  }

  finalizeTransaction(transactionId: string): KitGraphChange | undefined {
    return this._transactionFinalize(transactionId);
  }

  undoWithinTransaction(transactionId: string): void {
    this._transactionUndo(transactionId);
  }

  redoWithinTransaction(transactionId: string): void {
    this._transactionRedo(transactionId);
  }

  canUndoWithinTransaction(transactionId: string): boolean {
    return (this.#openTransactions.get(transactionId)?.done.length ?? 0) > 0;
  }

  canRedoWithinTransaction(transactionId: string): boolean {
    return (this.#openTransactions.get(transactionId)?.undone.length ?? 0) > 0;
  }

  getOpenTransactions(): TransactionView[] {
    const out: TransactionView[] = [];
    for (const [id, tx] of this.#openTransactions) {
      out.push({
        id,
        status: tx.status,
        label: tx.label,
        workspaceStatus: tx.workspaceStatus,
        conflicts: tx.conflicts.length > 0 ? [...tx.conflicts] : undefined,
      });
    }
    return out;
  }

  resolveConcurrentDeleteConflict(interactionId: string, conflictId: string, resolution: ConcurrentDeleteProposedResolution): void {
    const tx = this.#openTransactions.get(interactionId);
    if (!tx) throw new Error(`Unknown interaction ${interactionId}`);
    const c = tx.conflicts.find((x) => x.id === conflictId);
    if (!c) throw new Error(`Unknown conflict ${conflictId}`);
    if (resolution === "discardLocalChanges") {
      const touch = c.entityId;
      tx.done = tx.done.filter((ch) => !collectEntityIdsFromKitDiff(ch.forward).has(touch));
      tx.undone.length = 0;
      tx.conflicts = tx.conflicts.filter((x) => x.id !== conflictId);
      if (tx.conflicts.length === 0) tx.workspaceStatus = "clean";
      recomputeTxNet(tx);
      this.#reprojectEffectiveView();
      this.#revision++;
      this.notify();
      return;
    }
    const restoreDiff: KitDiff = {
      types: {
        updated: [
          {
            type: { id: c.entityId },
            diff: {
              lifecycle: "active",
              deletedAt: null,
              deletedByUserId: null,
              deletedByDisplayName: null,
              deletedInChangeId: null,
            },
          },
        ],
      },
    };
    const validation = validateKitGraphDiff(this, restoreDiff, false);
    if (!validation.ok || validation.errors.length > 0) {
      throw new Error(`RestoreType validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
    }
    const diffToApply = validation.diff ?? restoreDiff;
    const backward = this.inverseDiffFromPreApplyState(diffToApply);
    const restoreChange: KitGraphChange = {
      forward: diffToApply,
      backward,
      validation,
      preconditions: [],
    };
    tx.done.unshift(restoreChange);
    tx.undone.length = 0;
    tx.conflicts = tx.conflicts.filter((x) => x.id !== conflictId);
    if (tx.conflicts.length === 0) tx.workspaceStatus = "clean";
    recomputeTxNet(tx);
    this.#reprojectEffectiveView();
    this.#revision++;
    this.#invalidateCachesTouchedByDiff(diffToApply);
    this.notify();
  }

  getHistoryInfo(): HistoryInfo {
    return {
      pastCount: this.#historyDone.length,
      futureCount: this.#historyUndone.length,
      revision: this.#revision,
      auditLength: this.#auditLog.length,
    };
  }

  getConflict(): Conflict | undefined {
    return this.#conflict;
  }

  get kitPhase(): KitPhase {
    return this.#phase;
  }

  /** Undo last finalized local change (open transactions may still be in progress; they do not block this). */
  undo(): void {
    if (this.#conflicted) throw new Error("KitImpl is conflicted.");
    const ch = this.#historyDone.pop();
    if (!ch) return;
    const sk = new KitImpl(KitSchema.parse(stripNullsJsonClone(this.#committedPlain)), undefined);
    const validation = validateKitGraphDiff(sk, ch.backward, false);
    if (!validation.ok || validation.errors.length > 0) {
      this.#historyDone.push(ch);
      this.#freezeConflict("HistoryUndo", validation, { diff: ch.backward });
      throw new Error(`History undo validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
    }
    if (this.strictMode && validation.warnings.length > 0) {
      this.#historyDone.push(ch);
      this.#freezeConflict("HistoryUndo", validation, { diff: ch.backward });
      throw new Error(`History undo warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
    }
    const diffBack = validation.diff ?? ch.backward;
    sk.replayChangeUnchecked(diffBack);
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(sk)));
    this.#revision++;
    this.#invalidateCachesTouchedByDiff(diffBack);
    this.#historyUndone.push(ch);
    const outbound: KitGraphChange = { forward: diffBack, backward: ch.forward, validation };
    this.#scheduleBackboneNotify(outbound);
    this.#appendAudit("HistoryUndoApplied", ch);
    this.#reprojectEffectiveView();
    this.notify();
  }

  redo(): void {
    if (this.#conflicted) throw new Error("KitImpl is conflicted.");
    const ch = this.#historyUndone.pop();
    if (!ch) return;
    const sk = new KitImpl(KitSchema.parse(stripNullsJsonClone(this.#committedPlain)), undefined);
    const validation = validateKitGraphDiff(sk, ch.forward, false);
    if (!validation.ok || validation.errors.length > 0) {
      this.#historyUndone.push(ch);
      this.#freezeConflict("HistoryRedo", validation, { diff: ch.forward });
      throw new Error(`History redo validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
    }
    if (this.strictMode && validation.warnings.length > 0) {
      this.#historyUndone.push(ch);
      this.#freezeConflict("HistoryRedo", validation, { diff: ch.forward });
      throw new Error(`History redo warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
    }
    const diffToApply = validation.diff ?? ch.forward;
    const backward2 = sk.inverseDiffFromPreApplyState(diffToApply);
    sk.replayChangeUnchecked(diffToApply);
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(sk)));
    this.#revision++;
    this.#invalidateCachesTouchedByDiff(diffToApply);
    const ch2: KitGraphChange = { forward: diffToApply, backward: backward2, validation };
    this.#historyDone.push(ch2);
    this.#scheduleBackboneNotify(ch2);
    this.#appendAudit("HistoryRedoApplied", ch2);
    this.#reprojectEffectiveView();
    this.notify();
  }

  canUndo(): boolean {
    return this.#historyDone.length > 0;
  }

  canRedo(): boolean {
    return this.#historyUndone.length > 0;
  }

  /**
   * Runs `fn` with a new transaction id; finalizes on success or aborts on throw.
   */
  transactFinalized<T>(fn: (transactionId: string) => T): T {
    const id = this.beginTransaction().id;
    this.setActiveTransaction(id);
    try {
      const out = fn(id);
      this.finalizeTransaction(id);
      return out;
    } catch (err) {
      if (this.#openTransactions.has(id)) {
        this.abortTransaction(id);
      }
      throw err;
    }
  }

  /**
   * Internal validated diff pipeline ({@link KitDiff} is not a public mutation primitive).
   * Used by semantic entity methods, {@link InMemoryKitStore}, and backbone inbound wiring.
   */
  _applyDiff(diff: KitDiff, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff(diff, opts ?? {});
  }

  #applyDiff(diff: KitDiff, opts: KitChangeOptions): KitGraphChange {
    if (this.#phase === "frozen" || this.#conflicted) {
      throw new Error("KitImpl has unresolved validation conflicts; call resolveConflict() before applying further changes.");
    }

    const rawIn = opts.inboundCommitted
      ? this.#normalizeInboundTypeRemovalsToTombstones(diff, {
        deletedByUserId: opts.inboundActor?.actorId,
        deletedByDisplayName: opts.inboundActor?.actorDisplayName,
        deletedInChangeId: opts.inboundActor?.changeId,
      })
      : diff;
    const normalized = DiffComposer.normalize(rawIn);

    if (opts.inboundCommitted) {
      const sk = new KitImpl(KitSchema.parse(stripNullsJsonClone(this.#committedPlain)), undefined);
      const validation = validateKitGraphDiff(sk, normalized, false);
      if (!validation.ok || validation.errors.length > 0) {
        this.#freezeConflict("LocalChange", validation, { diff: normalized });
        throw new Error(`KitImpl validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
      }
      if (this.strictMode && validation.warnings.length > 0) {
        this.#freezeConflict("LocalChange", validation, { diff: normalized });
        throw new Error(`KitImpl validation warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
      }
      const diffToApply = validation.diff ?? normalized;
      const backward = sk.inverseDiffFromPreApplyState(diffToApply);
      sk.replayChangeUnchecked(diffToApply);
      this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(sk)));
      const change: KitGraphChange = { forward: diffToApply, backward, validation };
      this.#reprojectEffectiveView();
      this.#detectConcurrentDeleteConflicts(diffToApply, opts.inboundActor);
      this.#revision++;
      this.#invalidateCachesTouchedByDiff(diffToApply);
      this.#historyUndone.length = 0;
      this.#appendAudit("BackboneInbound", change);
      this.#phase = "ready";
      this.#conflicted = false;
      this.#conflict = undefined;
      this.validationState = { ok: true, errors: [], warnings: [], infos: [] };
      this.notify();
      return change;
    }

    const resolvedTxId = opts.transactionId ?? this.#activeTransactionId;
    if (resolvedTxId && this.#activeTransactionId && this.#activeTransactionId !== resolvedTxId) {
      throw new Error("transactionId does not match setActiveTransaction; align active transaction with the interaction receiving this change.");
    }

    if (resolvedTxId) {
      const validation = validateKitGraphDiff(this, normalized, false);
      if (!validation.ok || validation.errors.length > 0) {
        this.#freezeConflict("LocalChange", validation, { diff: normalized });
        throw new Error(`KitImpl validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
      }
      if (this.strictMode && validation.warnings.length > 0) {
        this.#freezeConflict("LocalChange", validation, { diff: normalized });
        throw new Error(`KitImpl validation warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
      }
      const diffToApply = validation.diff ?? normalized;
      const backward = this.inverseDiffFromPreApplyState(diffToApply);
      const preconditions: ChangePrecondition[] = [];
      for (const g of collectEntityIdsFromKitDiff(diffToApply)) {
        const t = this.findType(g);
        if (t) {
          preconditions.push({
            entityKind: "Type",
            entityId: g,
            expectedLifecycle: (t.lifecycle ?? "active") as EntityLifecycle,
            expectedVersionHash: this.#entityVersionHashFor(g),
          });
        }
      }
      const change: KitGraphChange = { forward: diffToApply, backward, validation, preconditions: preconditions.length > 0 ? preconditions : undefined };
      const tx = this.#openTransactions.get(resolvedTxId);
      if (!tx) throw new Error(`Unknown transaction ${resolvedTxId}`);
      if (tx.workspaceStatus === "conflicted") {
        throw new Error("Interaction workspace has unresolved concurrent delete conflicts; resolve them before editing.");
      }
      tx.done.push(change);
      tx.undone.length = 0;
      recomputeTxNet(tx);
      for (const g of collectEntityIdsFromKitDiff(change.forward)) {
        tx.touchedEntities.add(g);
        if (!tx.touchedVersions.has(g)) tx.touchedVersions.set(g, this.#entityVersionHashFor(g));
      }
      this.#reprojectEffectiveView();
      this.#revision++;
      this.#invalidateCachesTouchedByDiff(diffToApply);
      this.#appendAudit("TxStep", change);
      this.#phase = "ready";
      this.#conflicted = false;
      this.#conflict = undefined;
      this.validationState = { ok: true, errors: [], warnings: [], infos: [] };
      this.notify();
      return change;
    }

    if (this.#openTransactions.size > 0) {
      throw new Error("Cannot apply committed changes while interaction workspaces are open; finalize or abort them first.");
    }

    const validation = validateKitGraphDiff(this, normalized, false);
    if (!validation.ok || validation.errors.length > 0) {
      this.#freezeConflict("LocalChange", validation, { diff: normalized });
      throw new Error(`KitImpl validation failed: ${validation.errors.map((e) => e.message).join("; ")}`);
    }
    if (this.strictMode && validation.warnings.length > 0) {
      this.#freezeConflict("LocalChange", validation, { diff: normalized });
      throw new Error(`KitImpl validation warnings (strict): ${validation.warnings.map((e) => e.message).join("; ")}`);
    }
    const diffToApply = validation.diff ?? normalized;
    const backward = this.inverseDiffFromPreApplyState(diffToApply);
    this.#applyRawKitDiff(diffToApply);
    this.#committedPlain = KitSchema.parse(stripNullsJsonClone(kitGraphToPlainData(this)));
    const change: KitGraphChange = { forward: diffToApply, backward, validation };
    this.#revision++;
    this.#invalidateCachesTouchedByDiff(diffToApply);
    if (!opts.skipGlobalHistory) {
      this.#historyDone.push(change);
      this.#historyUndone.length = 0;
      this.#appendAudit("CommittedLocal", change);
    } else {
      this.#appendAudit("CommittedNoHistory", change);
    }
    const notifyBackbone = opts.notifyBackbone !== false;
    if (notifyBackbone) {
      this.#scheduleBackboneNotify(change);
    }
    this.#phase = "ready";
    this.#conflicted = false;
    this.#conflict = undefined;
    this.validationState = { ok: true, errors: [], warnings: [], infos: [] };
    this.notify();
    return change;
  }

  /**
   * Unfreezes after {@link Conflict}; flushes deferred backbone outbound and applies queued inbound diffs.
   */
  resolveConflict(): void {
    this.#phase = "ready";
    this.#conflicted = false;
    this.#conflict = undefined;
    this.validationState = { ok: true, errors: [], warnings: [], infos: [] };
    const pendingOut = this.#backboneOutboundFrozen.splice(0);
    this.#backboneOutbound.push(...pendingOut);
    this.#flushBackboneOutboundSoon();
    const inbound = this.#deferredInboundQueue.splice(0);
    for (const d of inbound) {
      this._applyDiff(d, { notifyBackbone: false, skipGlobalHistory: true, inboundCommitted: true });
    }
  }

  /**
   * ­ƒöìValidation - check constraints before applying diff.
   */
  private validate(diff: KitDiff): KitDiffValidationResult {
    return validateKitGraphDiff(this, diff, false);
  }

  /**
   * ­ƒô©Snapshot as plain data for serialization.
   */
  toData(): KitData {
    return kitGraphToPlainData(this);
  }

  /**
   * ­ƒô©JSON.stringify hook ÔÇô returns plain data without circular refs.
   */
  toJSON(): KitData {
    return kitGraphToPlainData(this);
  }

  /** ­ƒôªSerialize this kit for wire transport. */
  serialize(): string {
    return JSON.stringify(this.toData());
  }

  /** ­ƒº¡Deserialize a wire kit into a stateful kit graph. */
  static deserialize(json: string, backbone?: Backbone): KitImpl {
    return new KitImpl(KitSchema.parse(JSON.parse(json, (_key, value) => (value === null ? undefined : value))), backbone);
  }

  /** ­ƒº¼Create an isolated stateful copy of this kit graph. */
  duplicateForIsolation(): KitImpl {
    return KitImpl.deserialize(this.serialize());
  }

  /** ­ƒ¬¬Project this kit into its metadata wire shape. */
  toMeta(): KitMeta {
    return KitMetaSchema.parse(this.toData());
  }

  /** ­ƒº¥Project this kit into its shallow wire shape. */
  toShallow(): KitShallow {
    return KitShallowSchema.parse({
      ...this.toData(),
      types: this.types?.map((t) => t.toMeta()),
      designs: this.designs?.map((d) => d.toMeta()),
      tags: this.tags?.map((t) => TagMetaSchema.parse(t.toPlain())),
      concepts: this.concepts?.map((c) => ConceptMetaSchema.parse(c.toPlain())),
      families: this.families?.map((f) => f.toPlain()),
      qualities: this.qualities?.map((q) => QualityMetaSchema.parse(q.toPlain())),
      files: this.files?.map((f) => FileMetaSchema.parse(f.toPlain())),
      folders: this.folders?.map((f) => FolderMetaSchema.parse(f.toPlain())),
      authors: this.authors?.map((a) => AuthorMetaSchema.parse(a.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetaSchema.parse(a.toPlain())),
    });
  }

  /**
   * Validates and wraps a plain kit data object.
   */
  static fromData(data: KitData, backbone?: Backbone): KitImpl {
    return new KitImpl(data, backbone);
  }

  /**
   * ­ƒôïSubscribe to changes.
   */
  subscribe(listener: () => void): () => void {
    this.#listeners.add(listener);
    return () => this.#listeners.delete(listener);
  }

  private notify(): void {
    for (const listener of this.#listeners) listener();
  }

  // #region ­ƒöìFinders
  findType(id: Id): Type | undefined {
    return this.types?.find((t) => t.id === id);
  }

  findDesign(lookup: Id | { name: string }): Design | undefined {
    if (typeof lookup === "object" && lookup !== null && "name" in lookup) {
      const { name } = lookup;
      return this.designs?.find((d) => d.name === name);
    }
    const byId = this.designs?.find((d) => d.id === lookup);
    if (byId) return byId;
    return this.designs?.find((d) => d.name === lookup);
  }

  findPiece(designId: Id, pieceId: Id): Piece | undefined {
    const design = this.findDesign(designId);
    return design?.pieces?.find((p) => p.id === pieceId);
  }

  findConnection(designId: Id, connectionId: Id): Connection | undefined {
    const design = this.findDesign(designId);
    return design?._connections?.find((c) => c.id === connectionId);
  }

  requireType(typeId: string): Type {
    const t = this.findType(typeId);
    if (!t) throw new Error(`Type ${typeId} not found in kit ${this.name}`);
    return t;
  }

  requireDesign(designId: string): Design {
    const d = this.findDesign(designId);
    if (!d) throw new Error(`Design ${designId} not found in kit ${this.name}`);
    return d;
  }

  requireFile(fileId: string): File {
    const file = (this.files || []).find((f) => f.id === fileId);
    if (!file) throw new Error(`File ${fileId} not found in kit`);
    return file;
  }

  requireTag(tagId: string): Tag {
    const tag = (this.tags || []).find((t) => t.id === tagId);
    if (!tag) throw new Error(`Tag ${tagId} not found in kit`);
    return tag;
  }

  requireConcept(conceptId: string): Concept {
    const concept = (this.concepts || []).find((c) => c.id === conceptId);
    if (!concept) throw new Error(`Concept ${conceptId} not found in kit`);
    return concept;
  }

  requirePort(portId: string): Port {
    const iface = (this.families ?? []).flatMap((f) => f.ports ?? []).find((i) => i.id === portId);
    if (!iface) throw new Error(`Port ${portId} not found in kit ${this.name}`);
    return iface;
  }

  requireFamily(familyId: string): Family {
    const family = this.families?.find((f) => f.id === familyId);
    if (!family) throw new Error(`Family ${familyId} not found in kit ${this.name}`);
    return family;
  }

  /**
   * Designs that can replace a design-reference on a piece (non-abstract, not the piece's current design).
   */
  findReplacableDesignsForDesignPiece(_currentDesignId: string, designPiece: Piece): Design[] {
    void _currentDesignId;
    if (!designPiece.design) return [];
    const allDesigns = this.designs || [];
    const currentDesign = this.requireDesign(designPiece.design.id);
    return allDesigns.filter((design) => {
      if (design.id === currentDesign.id) return false;
      if (design.isAbstract) return false;
      return true;
    });
  }
  // #endregion ­ƒöìFinders

  // #region ­ƒº░KitImpl graph CRUD (validated {@link KitImpl._applyDiff})
  addType(type: Type, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ types: { added: [type] } }, opts ?? {});
  }
  setType(type: Type, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ types: { added: [type] } }, opts ?? {});
  }
  removeType(
    type: Type,
    opts?: KitChangeOptions & {
      deletedByUserId?: string;
      deletedByDisplayName?: string;
      deletedInChangeId?: string;
    },
  ): KitGraphChange {
    const now = new Date().toISOString();
    return this.#applyDiff(
      {
        types: {
          updated: [
            {
              type: { id: type.id },
              diff: {
                lifecycle: "deleted",
                deletedAt: now,
                deletedByUserId: opts?.deletedByUserId,
                deletedByDisplayName: opts?.deletedByDisplayName,
                deletedInChangeId: opts?.deletedInChangeId,
              },
            },
          ],
        },
      },
      opts ?? {},
    );
  }

  restoreType(type: Type, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff(
      {
        types: {
          updated: [
            {
              type: { id: type.id },
              diff: {
                lifecycle: "active",
                deletedAt: null,
                deletedByUserId: null,
                deletedByDisplayName: null,
                deletedInChangeId: null,
              },
            },
          ],
        },
      },
      opts ?? {},
    );
  }

  addDesign(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ designs: { added: [design] } }, opts ?? {});
  }
  setDesign(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ designs: { added: [design] } }, opts ?? {});
  }
  updateDesign(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ designs: { added: [design] } }, opts ?? {});
  }
  removeDesign(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ designs: { removed: [{ id: design.id }] } }, opts ?? {});
  }

  addFamily(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ families: { added: [family] } }, opts ?? {});
  }
  setFamily(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ families: { added: [family] } }, opts ?? {});
  }
  updateFamily(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ families: { added: [family] } }, opts ?? {});
  }
  removeFamily(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ families: { removed: [{ id: family.id }] } }, opts ?? {});
  }

  addFile(file: File, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ files: { added: [file] } }, opts ?? {});
  }
  setFile(file: File, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ files: { added: [file] } }, opts ?? {});
  }
  removeFile(file: File, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ files: { removed: [{ id: file.id }] } }, opts ?? {});
  }

  setAttribute(attribute: Attribute, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ attributes: { added: [attribute] } }, opts ?? {});
  }

  addTag(tag: Tag, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ tags: { added: [tag] } }, opts ?? {});
  }
  setTag(tag: Tag, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ tags: { added: [tag] } }, opts ?? {});
  }
  removeTag(tag: Tag, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ tags: { removed: [{ id: tag.id }] } }, opts ?? {});
  }

  addConcept(concept: Concept, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ concepts: { added: [concept] } }, opts ?? {});
  }
  setConcept(concept: Concept, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ concepts: { added: [concept] } }, opts ?? {});
  }
  removeConcept(concept: Concept, opts?: KitChangeOptions): KitGraphChange {
    return this.#applyDiff({ concepts: { removed: [{ id: concept.id }] } }, opts ?? {});
  }
  // #endregion ­ƒº░KitImpl graph CRUD (validated {@link KitImpl._applyDiff})

  // #region ­ƒôÉKitImpl queries & algorithms (see module exports for plain-kit fallbacks)
  filter(filter: KitFilter): KitImpl {
    const hasGlobFilters = !!(filter.designs || filter.types || filter.families || filter.files || filter.tags || filter.concepts || filter.qualities || filter.authors || filter.folders);
    if (!filter.designId && !hasGlobFilters) return this;

    const baseData: KitData = filter.designId ? filterKitByDesign(this, filter.designId, filter.representationTags) : this.toData();

    if (!hasGlobFilters) {
      return new KitImpl(KitSchema.parse(stripNullsJsonClone(baseData)));
    }

    const filtered: KitData = {
      ...baseData,
      types: (baseData.types ?? []).filter((t) => matchesGlobFilter(t.name, filter.types)),
      designs: (baseData.designs ?? []).filter((d) => matchesGlobFilter(d.name, filter.designs)),
      families: (baseData.families ?? []).filter((f) => matchesGlobFilter(f.name, filter.families)),
      files: (baseData.files ?? []).filter((f) => matchesGlobFilter(f.name, filter.files)),
      tags: (baseData.tags ?? []).filter((t) => matchesGlobFilter(t.name, filter.tags)),
      concepts: (baseData.concepts ?? []).filter((c) => matchesGlobFilter(c.name, filter.concepts)),
      qualities: (baseData.qualities ?? []).filter((q) => matchesGlobFilter(q.name, filter.qualities)),
      authors: (baseData.authors ?? []).filter((a) => matchesGlobFilter(a.name, filter.authors)),
      folders: (baseData.folders ?? []).filter((f) => matchesGlobFilter(f.name, filter.folders)),
    };
    return new KitImpl(KitSchema.parse(stripNullsJsonClone(filtered)));
  }

  getPrimitiveDesignFor(designId: string): Design {
    return this.requireDesign(designId);
  }

  getDesignFamilyFor(designId: string): Design[] {
    const design = this.requireDesign(designId);
    const designFamilies = design.families ?? [];
    if (designFamilies.length === 0) return [design];
    return (this.designs || []).filter((d) => {
      const df = d.families ?? [];
      return df.some((f) => designFamilies.some((df2) => df2.id === f.id));
    });
  }

  areDesignsInSameFamily(designIdA: string, designIdB: string): boolean {
    const a = this.requireDesign(designIdA);
    const b = this.requireDesign(designIdB);
    const familiesA = a.families ?? [];
    const familiesB = b.families ?? [];
    if (familiesA.length === 0 && familiesB.length === 0) return a.id === b.id;
    return familiesA.some((f) => familiesB.some((fb) => fb.id === f.id));
  }

  canUseDesignAsPiece(containerDesignId: string, pieceDesignId: string): boolean {
    return !this.areDesignsInSameFamily(containerDesignId, pieceDesignId);
  }

  findSameFamilyDesignPiecesIn(designId: string): Piece[] {
    const design = this.requireDesign(designId);
    return (design.pieces || []).filter((piece) => {
      if (!piece.design?.id) return false;
      return this.areDesignsInSameFamily(designId, piece.design.id);
    });
  }

  getPrimitiveTypeFor(typeId: string): Type {
    return this.requireType(typeId);
  }

  getTypeFamilyFor(typeId: string): Type[] {
    const type = this.requireType(typeId);
    const typeFamilies = type.families ?? [];
    if (typeFamilies.length === 0) return [type];
    return (this.types || []).filter((t) => {
      const tf = t.families ?? [];
      return tf.some((f) => typeFamilies.some((tf2) => tf2.id === f.id));
    });
  }

  areTypesInSameFamily(typeIdA: string, typeIdB: string): boolean {
    const a = this.requireType(typeIdA);
    const b = this.requireType(typeIdB);
    const familiesA = a.families ?? [];
    const familiesB = b.families ?? [];
    if (familiesA.length === 0 && familiesB.length === 0) return a.id === b.id;
    return familiesA.some((f) => familiesB.some((fb) => fb.id === f.id));
  }

  createClusteredDesignFromDesign(originalDesign: Design, clusterPieceIds: string[], designName: string): { clusteredDesign: Design; externalConnections: Connection[] } {
    const host = asKitInstance(this);
    const source =
      originalDesign instanceof Design
        ? originalDesign
        : new Design(DesignSchema.parse(stripNullsJsonClone(originalDesign) as unknown), host);
    if (!source.pieces || source.pieces.length === 0) {
      throw new Error("Original design has no pieces to cluster");
    }
    if (!clusterPieceIds || clusterPieceIds.length === 0) {
      throw new Error("No piece IDs provided for clustering");
    }
    const clusteredPieces = (source.pieces || []).filter((piece) => clusterPieceIds.includes(piece.id));
    if (clusteredPieces.length === 0) {
      throw new Error("No pieces found matching the provided IDs");
    }
    const internalConnections = (source._connections || []).filter(
      (connection) => clusterPieceIds.includes(connection.connected.piece.id) && clusterPieceIds.includes(connection.connecting.piece.id),
    );
    const externalConnections = (source._connections || []).filter((connection) => {
      const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.id);
      const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.id);
      return connectedInCluster !== connectingInCluster;
    });
    const pieceRow = (p: Piece | PiecePlain): PiecePlain =>
      typeof (p as Piece).toPlain === "function" ? (p as Piece).toPlain() : PieceSchema.parse(stripNullsJsonClone(p) as unknown);
    const connectionRow = (c: Connection | ConnectionPlain): ConnectionPlain =>
      typeof (c as Connection).toPlain === "function" ? (c as Connection).toPlain() : ConnectionSchema.parse(stripNullsJsonClone(c) as unknown);
    const hostForNested = typeof source.getKit === "function" ? source.getKit()! : host;
    const clusteredDesign = new Design(
      {
        id: id(),
        name: designName,
        unit: source.unit,
        description: `Clustered design with ${clusteredPieces.length} pieces`,
        pieces: clusteredPieces.map((p) => pieceRow(p as Piece | PiecePlain)),
        connections: internalConnections.map((c) => connectionRow(c as Connection | ConnectionPlain)),
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
      hostForNested,
    );
    return { clusteredDesign, externalConnections };
  }

  replaceClusterWithDesignChange(originalDesign: Design, clusterPieceIds: string[], clusteredDesign: Design, externalConnections: Connection[]): DesignChange {
    const host = asKitInstance(this);
    const orig =
      originalDesign instanceof Design
        ? originalDesign
        : new Design(DesignSchema.parse(stripNullsJsonClone(originalDesign) as unknown), host);
    const piecesToRemove = clusterPieceIds.map((g) => ({ id: g }));
    const connectionsToRemove = (orig._connections || [])
      .filter((connection) => {
        const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.id);
        const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.id);
        return connectedInCluster || connectingInCluster;
      })
      .map((c) => ({ id: c.id }));
    const updatedExternalConnections = externalConnections.map((connection) => {
      const base =
        connection instanceof Connection
          ? connection.toPlain()
          : ConnectionSchema.parse(stripNullsJsonClone(connection) as unknown);
      const connectedInCluster = clusterPieceIds.includes(base.connected.piece.id);
      const connectingInCluster = clusterPieceIds.includes(base.connecting.piece.id);
      if (connectedInCluster) {
        return { ...base, connected: { ...base.connected, designPiece: { id: clusteredDesign.id } } };
      } else if (connectingInCluster) {
        return { ...base, connecting: { ...base.connecting, designPiece: { id: clusteredDesign.id } } };
      }
      return base;
    });
    const forward: DesignDiff = {
      pieces: { removed: piecesToRemove },
      connections: { removed: connectionsToRemove, added: updatedExternalConnections },
    };
    const backward = inverseDesignDiff(orig, forward);
    return { forward, backward };
  }

  expandDesignPiecesFrom(design: Design): Design {
    const hasDesignConnections = design._connections?.some((conn) => conn.connected.designPiece || conn.connecting.designPiece);
    if (!hasDesignConnections) {
      return design;
    }
    let expandedDesign: Design = design;
    const designIds = new Set<string>();
    toArray(design._connections).forEach((conn) => {
      if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.id);
      if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.id);
    });
    if (designIds.size === 0) {
      return expandedDesign;
    }
    const kit = asKitInstance(this);
    for (const designName of Array.from(designIds)) {
      const referencedDesign = kit.findDesign(designName);
      if (!referencedDesign) continue;
      const expandedReferencedDesign = this.expandDesignPiecesFrom(referencedDesign);
      const transformedPieces: Piece[] = (expandedReferencedDesign.pieces || []).map(
        (piece) =>
          new Piece(
            PieceSchema.parse({
              ...piece.toPlain(),
              center: piece.center ? piece.center.toPlain() : { u: 0, v: 0 },
            }),
            expandedDesign,
            kit,
          ),
      );
      const transformedConnections = expandedReferencedDesign._connections || [];
      const updatedExternalConnections = (expandedDesign._connections || []).map((connection) => {
        if (connection.connected.designPiece?.id === designName) {
          return { ...connection, connected: { ...connection.connected, designPiece: undefined } };
        }
        if (connection.connecting.designPiece?.id === designName) {
          return { ...connection, connecting: { ...connection.connecting, designPiece: undefined } };
        }
        return connection;
      });
      expandedDesign = new Design(
        DesignSchema.parse({
          ...(expandedDesign as unknown as DesignPlain),
          pieces: [...(expandedDesign.pieces || []).map((p) => p.toPlain()), ...transformedPieces.map((p) => p.toPlain())],
          connections: [...updatedExternalConnections.map((c) => c.toPlain()), ...transformedConnections.map((c) => c.toPlain())],
        }),
        kit,
      );
    }
    return expandedDesign;
  }

  dragPiecesInDesignDiff(design: Design, pieces: Design, offset: Coordinate): DesignDiff {
    const { selectedIds, parentMap, pieceMap, fixedIds } = buildDragMoveStructuralContext(design, pieces);
    const pieceUpdates: { piece: { id: string }; diff: PieceDiff }[] = [];
    for (const g of fixedIds) {
      const currentCenter = pieceMap.get(g)?.center;
      if (currentCenter !== undefined) {
        pieceUpdates.push({ piece: { id: g }, diff: { center: { u: currentCenter.u + offset.u, v: currentCenter.v + offset.v } } });
      }
    }
    const connectionUpdates: { connection: { id: string }; diff: ConnectionDiff }[] = [];
    for (const g of selectedIds) {
      if (fixedIds.has(g)) continue;
      if (pieceHasSelectedAncestorInDragMoveTree(g, selectedIds, parentMap)) continue;
      const parent = parentMap.get(g);
      if (!parent) continue;
      connectionUpdates.push({ connection: { id: parent.connectionId }, diff: { u: offset.u, v: offset.v } });
    }
    const diff: DesignDiff = {};
    if (pieceUpdates.length > 0) diff.pieces = { updated: pieceUpdates };
    if (connectionUpdates.length > 0) diff.connections = { updated: connectionUpdates };
    return diff;
  }

  findReplaceableTypesInDesignsForPiecesInDesignOp(
    design: Design,
    designs: Design[],
    types: Type[],
    ports: Port[],
    selection: { pieces: string[] },
  ): { types: string[]; designs: string[] } {
    const selectedPieceSet = new Set(selection.pieces);
    const pieces = design.pieces ?? [];
    const connections = design._connections ?? [];
    const pieceMap = new Map<string, Piece>();
    for (const piece of pieces) pieceMap.set(piece.id, piece);
    const portMap = new Map<string, Port>();
    for (const p of ports) portMap.set(p.id, p);
    const typeMap = new Map<string, Type>();
    for (const t of types) typeMap.set(t.id, t);
    const checkPortCompatibility = (candidatePortId: string, requiredPortId: string): boolean => {
      if (!candidatePortId || !requiredPortId) return false;
      if (candidatePortId === requiredPortId) return true;
      const candidatePort = portMap.get(candidatePortId);
      const requiredPort = portMap.get(requiredPortId);
      if (!candidatePort || !requiredPort) return false;
      return (
        (candidatePort.compatiblePorts ?? []).some((compatiblePort) => compatiblePort.id === requiredPortId) ||
        (requiredPort.compatiblePorts ?? []).some((compatiblePort) => compatiblePort.id === candidatePortId)
      );
    };
    const getConnectorPortId = (typeId: string | undefined, connectorId: string | undefined): string => {
      if (!typeId || !connectorId) return "";
      const type = typeMap.get(typeId);
      const connector = type?.connectors?.find((candidateConnector) => candidateConnector.id === connectorId);
      return connector?.port?.id ?? "";
    };
    const getOwnRequirementPortIds = (pieceId: string): string[] => {
      const piece = pieceMap.get(pieceId);
      const type = piece?.type?.id ? typeMap.get(piece.type.id) : undefined;
      return (type?.connectors ?? []).map((connector) => connector.port?.id ?? "");
    };
    const getBoundaryRequirementPortIds = (): string[] => {
      const requirementPortIds: string[] = [];
      for (const connection of connections) {
        const connectedSelected = selectedPieceSet.has(connection.connected.piece.id);
        const connectingSelected = selectedPieceSet.has(connection.connecting.piece.id);
        if (connectedSelected === connectingSelected) continue;
        const otherSide = connectedSelected ? connection.connecting : connection.connected;
        const otherPiece = pieceMap.get(otherSide.piece.id);
        requirementPortIds.push(getConnectorPortId(otherPiece?.type?.id, otherSide.connector?.id));
      }
      return requirementPortIds;
    };
    const getSelectionOwnRequirementPortIds = (): string[] => selection.pieces.flatMap((pieceId) => getOwnRequirementPortIds(pieceId));
    const canSatisfyRequirements = (requiredPortIds: string[], availablePortIds: string[]): boolean => {
      if (requiredPortIds.length === 0) return true;
      if (availablePortIds.length < requiredPortIds.length) return false;
      const requirementOptions = requiredPortIds
        .map((requiredPortId) => ({
          connectorIndexes: availablePortIds.flatMap((availablePortId, connectorIndex) =>
            checkPortCompatibility(availablePortId, requiredPortId) ? [connectorIndex] : [],
          ),
        }))
        .sort((leftRequirement, rightRequirement) => leftRequirement.connectorIndexes.length - rightRequirement.connectorIndexes.length);
      if (requirementOptions.some((requirementOption) => requirementOption.connectorIndexes.length === 0)) return false;
      const usedConnectorIndexes = new Array(availablePortIds.length).fill(false);
      const matchRequirement = (requirementOptionIndex: number): boolean => {
        if (requirementOptionIndex >= requirementOptions.length) return true;
        for (const connectorIndex of requirementOptions[requirementOptionIndex].connectorIndexes) {
          if (usedConnectorIndexes[connectorIndex]) continue;
          usedConnectorIndexes[connectorIndex] = true;
          if (matchRequirement(requirementOptionIndex + 1)) return true;
          usedConnectorIndexes[connectorIndex] = false;
        }
        return false;
      };
      return matchRequirement(0);
    };
    const candidateTypeAvailablePortIds = (candidateType: Type): string[] => (candidateType.connectors ?? []).map((connector) => connector.port?.id ?? "");
    const candidateDesignAvailablePortIds = (candidateDesign: Design): string[] => {
      const consumedConnectorKeys = new Set<string>();
      for (const connection of candidateDesign._connections ?? []) {
        for (const side of [connection.connected, connection.connecting]) {
          if (side.piece.id && side.connector?.id) consumedConnectorKeys.add(`${side.piece.id}::${side.connector.id}`);
        }
      }
      const availablePortIds: string[] = [];
      for (const piece of candidateDesign.pieces ?? []) {
        const type = piece.type?.id ? typeMap.get(piece.type.id) : undefined;
        for (const connector of type?.connectors ?? []) {
          if (consumedConnectorKeys.has(`${piece.id}::${connector.id}`)) continue;
          availablePortIds.push(connector.port?.id ?? "");
        }
      }
      return availablePortIds;
    };
    if (selection.pieces.length === 0) {
      return {
        types: types.filter((candidateType) => candidateTypeAvailablePortIds(candidateType).length === 0).map((candidateType) => candidateType.id),
        designs: designs.filter((candidateDesign) => candidateDesignAvailablePortIds(candidateDesign).length === 0).map((candidateDesign) => candidateDesign.id),
      };
    }
    const requiredPortIds = (() => {
      const boundaryRequirementPortIds = getBoundaryRequirementPortIds();
      return boundaryRequirementPortIds.length > 0 ? boundaryRequirementPortIds : getSelectionOwnRequirementPortIds();
    })();
    const isValidCandidate = (availablePortIds: string[]): boolean => canSatisfyRequirements(requiredPortIds, availablePortIds);
    return {
      types: types.filter((candidateType) => isValidCandidate(candidateTypeAvailablePortIds(candidateType))).map((candidateType) => candidateType.id),
      designs: designs.filter((candidateDesign) => isValidCandidate(candidateDesignAvailablePortIds(candidateDesign))).map((candidateDesign) => candidateDesign.id),
    };
  }

  /**
   * Resolves {@link Type} and {@link Connector} like {@link flattenDesign} / move / copy paths.
   */
  buildConnectorResolver(): { getType: (typeId: string) => Type | undefined; getConnector: (type: Type | undefined, connectorId: string | undefined) => Connector | undefined } {
    const typesDict: { [key: string]: Type } = {};
    (this.types ?? []).forEach((t) => {
      typesDict[t.id] = t;
    });
    const getType = (typeId: string): Type | undefined => typesDict[typeId];
    const getConnector = (type: Type | undefined, connectorId: string | undefined): Connector | undefined => {
      if (!type) return undefined;

      if (!connectorId) {
        if (type.connectors && type.connectors.length > 0) {
          return type.connectors[0];
        }
        return undefined;
      }

      if (type.connectors && type.connectors.length > 0) {
        const connector = type.connectors.find((p) => p.id === connectorId);
        if (connector) return connector;
      }

      if (type.connectors && type.connectors.length > 0) {
        return type.connectors[0];
      }

      return undefined;
    };
    return { getType, getConnector };
  }

  /** Full flatten (no merkle cache); see {@link flattenDesign}. */
  runFlattenDesign(designId: string): DesignOperationResult {
    return this.#flattenDesignUncached(designId);
  }

  /** Flatten using this kitÔÇÖs merkle cache ({@link KitImpl.flattenDesignMerkle}). */
  runFlattenDesignOptimized(designId: string): DesignOperationResult {
    return this.flattenDesignMerkle(designId);
  }

  getFlatMerkleHashes(designId: string): { [pieceId: string]: FlatMerkleHashes } {
    return this.#computeFlatMerkleHashes(designId);
  }

  /**
   * Computes a remove diff (forward/backward design change) without committing to the kit graph.
   * See {@link removePiecesAndConnectionsFromDesign}.
   */
  previewRemovePiecesAndConnections(designId: string, pieceIds: string[], connectionIds: string[]): DesignOperationResult {
    return this.#removePiecesAndConnectionsOperation(designId, pieceIds, connectionIds);
  }

  removePiecesAndConnectionsFromDesignOp(designId: string, pieceIds: string[], connectionIds: string[]): DesignOperationResult {
    return this.#removePiecesAndConnectionsOperation(designId, pieceIds, connectionIds);
  }

  #removePiecesAndConnectionsOperation(designId: string, pieceIds: string[], connectionIds: string[]): DesignOperationResult {
    const design = this.requireDesign(designId);
    const delRes = design.deletePiecesAndConnectionsDiff(pieceIds, connectionIds);
    if (!delRes.ok) {
      return operationErr(delRes.errors);
    }
    const forward = delRes.diff!;
    const backward = inverseDesignDiff(design, forward);
    return operationOk({ forward, backward }, delRes.warnings, delRes.infos);
  }

  fixPieceInDesignDiff(designId: string, pieceId: string): DesignDiff {
    const parentConnection = this.findParentConnectionForPieceInDesign(designId, pieceId);
    return {
      connections: {
        removed: [{ id: parentConnection.id }],
      },
    };
  }

  fixPiecesInDesignDiff(designId: string, pieceIds: string[]): DesignDiff {
    const parentConnections = pieceIds.map((pieceId) => this.findParentConnectionForPieceInDesign(designId, pieceId));
    return {
      connections: {
        removed: parentConnections.map((c) => ({ id: c.id })),
      },
    };
  }

  deletePiecesAndConnectionsInDesignOp(design: Design, pieceIds: string[], connectionIds: string[]): DesignDiffOperationResult {
    return design.deletePiecesAndConnectionsDiff(pieceIds, connectionIds);
  }

  movePiecesInDesignOp(design: Design, pieces: Design, vector: MoveVector): DesignDiff {
    const { getType, getConnector } = this.buildConnectorResolver();
    const { selectedIds, parentMap, pieceMap, fixedIds } = buildDragMoveStructuralContext(design, pieces);
    const pieceUpdates: { piece: { id: string }; diff: PieceDiff }[] = [];
    for (const id of fixedIds) {
      const base = pieceMap.get(id)?.plane;
      if (base === undefined) continue;
      const t = moveTranslationWorldFromPiecePlane(base, vector);
      const newPlane: Plane = new Plane({
        origin: { x: base.origin.x + t.x, y: base.origin.y + t.y, z: base.origin.z + t.z },
        xAxis: VectorSchema.parse(base.xAxis as unknown),
        yAxis: VectorSchema.parse(base.yAxis as unknown),
      });
      pieceUpdates.push({ piece: { id }, diff: { plane: newPlane } });
    }
    const connectionUpdates: { connection: { id: string }; diff: ConnectionDiff }[] = [];
    for (const id of selectedIds) {
      if (fixedIds.has(id)) continue;
      if (pieceHasSelectedAncestorInDragMoveTree(id, selectedIds, parentMap)) continue;
      const parent = parentMap.get(id);
      if (!parent) continue;
      const connection = design._connections?.find((c) => c.id === parent.connectionId);
      if (!connection) continue;
      const parentPiece = pieceMap.get(parent.parentId);
      const childPiece = pieceMap.get(id);
      if (!parentPiece || !childPiece) continue;
      if (!(typeof parentPiece.wireTypeId === "function" ? parentPiece.wireTypeId()?.id : (parentPiece as any).type?.id) || !(typeof childPiece.wireTypeId === "function" ? childPiece.wireTypeId()?.id : (childPiece as any).type?.id)) continue;
      const parentType = resolvePieceTypeForFlatten(parentPiece, getType);
      const childType = resolvePieceTypeForFlatten(childPiece, getType);
      const parentConnector = getConnector(parentType, connection.connected.connector?.id);
      const childConnector = getConnector(childType, connection.connecting.connector?.id);
      if (!parentConnector) continue;
      const parentPlane = parentPiece.plane ?? identityPlaneForStructuralMove();
      const connDiff = connectionDiffFromStructuralMoveVector(parentPlane, parentConnector, childConnector, connection, childPiece.plane, vector);
      if (Object.keys(connDiff).length === 0) continue;
      connectionUpdates.push({ connection: { id: parent.connectionId }, diff: connDiff });
    }
    const diff: DesignDiff = {};
    if (pieceUpdates.length > 0) diff.pieces = { updated: pieceUpdates };
    if (connectionUpdates.length > 0) diff.connections = { updated: connectionUpdates };
    return diff;
  }

  copyDesignOp(design: Design, pieceIds: string[], connectionIds: string[]): OperationResult<Design> {
    return this.#copyDesignClipboard(design, pieceIds, connectionIds);
  }

  pasteDesignOp(source: Design, target: Design, anchoring: string = "bottomLeft", coordinate?: Coordinate): DesignDiff {
    return this.#pasteDesign(source, target, anchoring, coordinate);
  }

  piecesMetadataFor(designId: string): OperationResult<Map<string, PiecePlacementMetadata>> {
    const design = this.findDesign(designId);
    if (!design) {
      return operationErr([{ code: "pieces-metadata.design-not-found", message: `Design ${designId} not found in kit ${this.name}` }]);
    }
    if (!design.pieces?.length) {
      return operationOk(new Map(), [], []);
    }
    const prev = this.#flattenMerkleByDesign.get(designId);
    const walk = this.#runFlattenPlacementWalk(design, prev);
    this.#flattenMerkleByDesign.set(designId, walk.nextCache);
    if (walk.placementErrors.length > 0) {
      return operationErr(walk.placementErrors);
    }
    const metadata = new Map<string, PiecePlacementMetadata>();
    for (const piece of design.pieces) {
      if (!piece.id) continue;
      const entry = walk.nextCache[piece.id];
      const fp = entry?.flatPiece;
      if (!entry?.plane || !entry?.center || !fp) continue;
      const rawPath = findAttributeValue(fp, "semio.path", piece.id);
      metadata.set(piece.id, {
        plane: entry.plane,
        center: entry.center,
        fixedPieceId: findAttributeValue(fp, "semio.fixedPieceId", piece.id) || piece.id,
        parentPieceId: findAttributeValue(fp, "semio.parentPieceId", null),
        depth: parseInt(findAttributeValue(fp, "semio.depth", "0")!),
        path: rawPath ? rawPath.split(",").filter(Boolean) : [piece.id],
      });
    }
    return operationOk(metadata, walk.warnings, walk.infos);
  }

  piecesMetadataCachedFor(designId: string, cache?: { [pieceId: string]: FlatMerkleCacheEntry }): { result: OperationResult<Map<string, PiecePlacementMetadata>>; cache: { [pieceId: string]: FlatMerkleCacheEntry } } {
    const design = this.findDesign(designId);
    if (!design) {
      return {
        result: operationErr([{ code: "pieces-metadata.design-not-found", message: `Design ${designId} not found in kit ${this.name}` }]),
        cache: {},
      };
    }
    if (!design.pieces?.length) {
      return {
        result: operationOk(new Map(), [], []),
        cache: {},
      };
    }
    const walk = this.#runFlattenPlacementWalk(design, cache);
    this.#flattenMerkleByDesign.set(designId, walk.nextCache);
    if (walk.placementErrors.length > 0) {
      return { result: operationErr(walk.placementErrors), cache: walk.nextCache };
    }
    const metadata = new Map<string, PiecePlacementMetadata>();
    for (const piece of design.pieces) {
      if (!piece.id) continue;
      const entry = walk.nextCache[piece.id];
      const fp = entry?.flatPiece;
      if (!entry?.plane || !entry?.center || !fp) continue;
      const rawPath = findAttributeValue(fp, "semio.path", piece.id);
      metadata.set(piece.id, {
        plane: entry.plane,
        center: entry.center,
        fixedPieceId: findAttributeValue(fp, "semio.fixedPieceId", piece.id) || piece.id,
        parentPieceId: findAttributeValue(fp, "semio.parentPieceId", null),
        depth: parseInt(findAttributeValue(fp, "semio.depth", "0")!),
        path: rawPath ? rawPath.split(",").filter(Boolean) : [piece.id],
      });
    }
    return {
      result: operationOk(metadata, walk.warnings, walk.infos),
      cache: walk.nextCache,
    };
  }

  flattenDesignCachedOp(designId: string, cache?: { [pieceId: string]: FlatMerkleCacheEntry }): { result: DesignOperationResult; cache: { [pieceId: string]: FlatMerkleCacheEntry } } {
    return this.#flattenDesignCached(designId, cache);
  }

  findPieceTypeInDesign(designId: string, pieceId: string): Type {
    const piece = findPieceInDesign(this.requireDesign(designId), pieceId);
    if (!piece.type) throw new Error(`Piece ${pieceId} has no type`);
    return this.requireType(piece.type.id);
  }

  findParentPieceInDesign(designId: string, pieceId: string): Piece {
    const meta = this.piecesMetadataFor(designId);
    if (!meta.ok) throw new Error(meta.errors.map((e) => e.message).join("; "));
    const parentPieceId = meta.diff.get(pieceId)?.parentPieceId;
    if (!parentPieceId) throw new Error(`Piece ${pieceId} has no parent piece`);
    return findPieceInDesign(this.requireDesign(designId), parentPieceId);
  }

  findParentConnectionForPieceInDesign(designId: string, pieceId: string): Connection {
    const meta = this.piecesMetadataFor(designId);
    if (!meta.ok) throw new Error(meta.errors.map((e) => e.message).join("; "));
    const parentPieceId = meta.diff.get(pieceId)?.parentPieceId;
    if (!parentPieceId) throw new Error(`Piece ${pieceId} has no parent piece and connection`);
    const design = this.requireDesign(designId);
    const incident = findPieceConnectionsInDesign(design, pieceId);
    const parentConnection = incident.find((c) => {
      const other = c.connected.piece.id === pieceId ? c.connecting.piece.id : c.connected.piece.id;
      return other === parentPieceId;
    });
    if (!parentConnection) {
      throw new Error(`No connection found from piece ${pieceId} to parent piece ${parentPieceId}`);
    }
    return parentConnection;
  }

  findChildrenPiecesInDesign(designId: string, pieceId: string): Piece[] {
    const design = this.requireDesign(designId);
    const meta = this.piecesMetadataFor(designId);
    if (!meta.ok) throw new Error(meta.errors.map((e) => e.message).join("; "));
    const metadata = meta.diff;
    const children: Piece[] = [];
    for (const [id, data] of Array.from(metadata)) {
      if (data.parentPieceId === pieceId) {
        children.push(findPieceInDesign(design, id));
      }
    }
    return children;
  }

  findUsedConnectorsByPieceInDesign(designId: string, pieceId: string): Connector[] {
    const design = this.requireDesign(designId);
    const piece = findPieceInDesign(design, pieceId);
    if (!piece.type) return [];
    const type = this.requireType(piece.type.id);
    const connections = findPieceConnectionsInDesign(design, pieceId);
    return connections.map((c) => findConnectorForPieceInConnection(type, c, pieceId)).filter((p): p is Connector => p !== undefined);
  }

  findReplacableTypesForPieceInDesign(designId: string, pieceId: string): Type[] {
    const design = this.requireDesign(designId);
    const connections = findPieceConnectionsInDesign(design, pieceId);
    const requiredConnectors: Connector[] = [];
    for (const connection of connections) {
      try {
        const otherPieceId = connection.connected.piece.id === pieceId ? connection.connecting.piece.id : connection.connected.piece.id;
        const otherPiece = findPieceInDesign(design, otherPieceId);
        if (!otherPiece.type) continue;
        const otherType = this.requireType(otherPiece.type.id);
        const otherPortId = connection.connected.piece.id === pieceId ? connection.connecting.connector?.id : connection.connected.connector?.id;
        const otherPort = findConnectorInType(otherType, otherPortId || "");
        requiredConnectors.push(otherPort);
      } catch {
        continue;
      }
    }
    return (
      this.types?.filter((replacementType) => {
        if (replacementType.isAbstract) return false;
        if (!replacementType.connectors || replacementType.connectors.length === 0) return requiredConnectors.length === 0;
        return requiredConnectors.every((requiredConnector) => {
          return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
        });
      }) ?? []
    );
  }

  findReplacableTypesForPiecesInDesign(designId: string, pieceIds: string[]): Type[] {
    const design = this.requireDesign(designId);
    const pieces = pieceIds.map((id) => findPieceInDesign(design, id));
    const externalConnections: Array<{
      connection: Connection;
      requiredConnector: Connector;
    }> = [];
    for (const piece of pieces) {
      const connections = findPieceConnectionsInDesign(design, piece.id);
      for (const connection of connections) {
        const otherPieceId = connection.connected.piece.id === piece.id ? connection.connecting.piece.id : connection.connected.piece.id;
        if (!pieceIds.includes(otherPieceId)) {
          try {
            const otherPiece = findPieceInDesign(design, otherPieceId);
            if (!otherPiece.type) continue;
            const otherType = this.requireType(otherPiece.type.id);
            const otherPortId = connection.connected.piece.id === piece.id ? connection.connecting.connector?.id : connection.connected.connector?.id;
            const otherPort = findConnectorInType(otherType, otherPortId || "");
            externalConnections.push({ connection, requiredConnector: otherPort });
          } catch {
            continue;
          }
        }
      }
    }
    return (
      this.types?.filter((replacementType) => {
        if (replacementType.isAbstract) return false;
        if (!replacementType.connectors || replacementType.connectors.length === 0) return externalConnections.length === 0;
        return externalConnections.every(({ requiredConnector }) => {
          return replacementType.connectors!.some((replacementConnector) => areConnectorsCompatible(replacementConnector, requiredConnector));
        });
      }) ?? []
    );
  }

  sumQualityInDesign(designId: string, qualityId: string): number {
    const design = this.requireDesign(designId);
    let sum = 0;
    for (const piece of design.pieces ?? []) {
      const pieceProp = piece.props?.find((p) => p.quality?.id === qualityId);
      if (pieceProp) {
        const val = parseFloat(pieceProp.value);
        if (!isNaN(val)) sum += val;
        continue;
      }
      if (piece.type) {
        const type = this.types?.find((t) => t.id === piece.type!.id);
        if (type) {
          const typeProp = type.props?.find((p) => p.quality?.id === qualityId);
          if (typeProp) {
            const val = parseFloat(typeProp.value);
            if (!isNaN(val)) sum += val;
          }
        }
      }
    }
    return sum;
  }
  // #endregion ­ƒôÉKitImpl queries & algorithms

  // #region ­ƒöùGetters
  getDiff(other: KitImpl): KitDiff {
    return computeKitGraphDiffBetween(this, asKitInstance(other));
  }

  getHash(): string {
    return JSON.stringify(this.toData())
      .split("")
      .reduce((a, b) => {
        a = (a << 5) - a + b.charCodeAt(0);
        return a & a;
      }, 0)
      .toString(36);
  }

  get isConflicted(): boolean {
    return this.#conflicted;
  }

  get validation(): ValidationState {
    return {
      ok: this.validationState.ok,
      errors: [...this.validationState.errors],
      warnings: [...this.validationState.warnings],
      infos: [...this.validationState.infos],
      ...(this.validationState.diff !== undefined ? { diff: this.validationState.diff } : {}),
    };
  }

  setStrictMode(strict: boolean): void {
    this.strictMode = strict;
  }
  // #endregion ­ƒöùGetters
}

function _isKitDataShape(x: unknown): x is KitData {
  return typeof x === "object" && x !== null && "id" in x && typeof (x as KitData).id === "string";
}

type KitCallable = (this: unknown, arg0?: KitData | Backbone, arg1?: Backbone) => InstanceType<typeof KitImpl>;

/**
 * Primary kit factory from the object representation spec: `Kit()` opens an empty graph; `Kit(plainData)` hydrates; `new Kit(plain, backbone)` is supported. Static methods match {@link KitImpl}.
 */
const _kitFactory: KitCallable = function (this: unknown, arg0?: KitData | Backbone, arg1?: Backbone): InstanceType<typeof KitImpl> {
  if (new.target !== undefined) {
    if (!_isKitDataShape(arg0)) {
      throw new Error("new Kit() requires kit data; use Kit() for an empty in-memory kit.");
    }
    return new KitImpl(arg0, arg1);
  }
  if (arg0 !== undefined && _isKitDataShape(arg0)) {
    return new KitImpl(arg0, arg1 as Backbone | undefined);
  }
  return KitImpl.open(arg0 as Backbone | undefined);
};

export const Kit = _kitFactory as KitCallable & typeof KitImpl;
for (const key of Object.getOwnPropertyNames(KitImpl)) {
  if (key === "prototype") continue;
  const d = Object.getOwnPropertyDescriptor(KitImpl, key);
  if (d) Object.defineProperty(Kit, key, d);
}
Kit.prototype = KitImpl.prototype;

/** Live kit graph instance (returned by {@link Kit}, {@link KitImpl.open}, etc.). */
export type Kit = InstanceType<typeof KitImpl>;

/**
 * Object-oriented entry for type rows on a {@link KitImpl} (replaces removed `*TypeToKit` / `findTypeInKit` helpers).
 */
export class KitTypesOps {
  constructor(private readonly kit: KitImpl) { }

  add(type: Type, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.addType(type, opts);
  }

  set(type: Type, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setType(type, opts);
  }

  remove(type: Type, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.removeType(type, opts);
  }

  find(id: Id): Type | undefined {
    return this.kit.findType(id);
  }

  require(id: string): Type {
    return this.kit.requireType(id);
  }
}

/**
 * Object-oriented entry for design rows on a {@link KitImpl}.
 */
export class KitDesignsOps {
  constructor(private readonly kit: KitImpl) { }

  add(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.addDesign(design, opts);
  }

  set(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setDesign(design, opts);
  }

  update(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.updateDesign(design, opts);
  }

  remove(design: Design, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.removeDesign(design, opts);
  }

  find(id: Id): Design | undefined {
    return this.kit.findDesign(id);
  }

  require(id: string): Design {
    return this.kit.requireDesign(id);
  }

  replacableForPiece(currentDesignId: string, designPiece: Piece): Design[] {
    return this.kit.findReplacableDesignsForDesignPiece(currentDesignId, designPiece);
  }
}

/**
 * Object-oriented entry for port rows on a {@link KitImpl}.
 */
export class KitFamiliesOps {
  constructor(private readonly kit: KitImpl) { }

  add(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.addFamily(family, opts);
  }

  set(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setFamily(family, opts);
  }

  update(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.updateFamily(family, opts);
  }

  remove(family: Family, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.removeFamily(family, opts);
  }

  require(id: string): Family {
    return this.kit.requireFamily(id);
  }
}

/**
 * Object-oriented entry for file rows on a {@link KitImpl}.
 */
export class KitFilesOps {
  constructor(private readonly kit: KitImpl) { }

  add(file: File, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.addFile(file, opts);
  }

  set(file: File, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setFile(file, opts);
  }

  remove(file: File, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.removeFile(file, opts);
  }

  require(id: string): File {
    return this.kit.requireFile(id);
  }
}

/**
 * Object-oriented entry for tag rows on a {@link KitImpl}.
 */
export class KitTagsOps {
  constructor(private readonly kit: KitImpl) { }

  add(tag: Tag, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.addTag(tag, opts);
  }

  set(tag: Tag, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setTag(tag, opts);
  }

  remove(tag: Tag, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.removeTag(tag, opts);
  }

  require(id: string): Tag {
    return this.kit.requireTag(id);
  }
}

/**
 * Object-oriented entry for concept rows on a {@link KitImpl}.
 */
export class KitConceptsOps {
  constructor(private readonly kit: KitImpl) { }

  add(concept: Concept, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.addConcept(concept, opts);
  }

  set(concept: Concept, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setConcept(concept, opts);
  }

  remove(concept: Concept, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.removeConcept(concept, opts);
  }

  require(id: string): Concept {
    return this.kit.requireConcept(id);
  }
}

/**
 * Object-oriented entry for top-level kit attributes.
 */
export class KitAttributesOps {
  constructor(private readonly kit: KitImpl) { }

  set(attribute: Attribute, opts?: KitChangeOptions): KitGraphChange {
    return this.kit.setAttribute(attribute, opts);
  }
}

/**
 * Bundles namespace objects for validated kit graph edits.
 */
export class KitOps {
  readonly types: KitTypesOps;
  readonly designs: KitDesignsOps;
  readonly families: KitFamiliesOps;
  readonly files: KitFilesOps;
  readonly tags: KitTagsOps;
  readonly concepts: KitConceptsOps;
  readonly attributes: KitAttributesOps;

  constructor(kit: KitImpl) {
    this.types = new KitTypesOps(kit);
    this.designs = new KitDesignsOps(kit);
    this.families = new KitFamiliesOps(kit);
    this.files = new KitFilesOps(kit);
    this.tags = new KitTagsOps(kit);
    this.concepts = new KitConceptsOps(kit);
    this.attributes = new KitAttributesOps(kit);
  }
}

/**
 * Open transactional editing context on a {@link KitImpl}. Provisional steps apply live; finalize commits history + backbone.
 */
export class Transaction {
  constructor(
    private readonly host: KitImpl,
    readonly id: string,
    readonly label?: string,
  ) { }

  get status(): TransactionStatus {
    return this.host._getTransactionStatus(this.id);
  }

  finalize(): KitGraphChange | undefined {
    return this.host._transactionFinalize(this.id);
  }

  abort(): void {
    this.host._transactionAbort(this.id);
  }

  undo(): void {
    this.host._transactionUndo(this.id);
  }

  redo(): void {
    this.host._transactionRedo(this.id);
  }
}

/** OO snapshot for I/O: only valid on a live {@link KitImpl} (nested {@link toPlain}). */
const kitGraphToPlainData = (kit: KitImpl): KitData => {
  if (!(kit instanceof KitImpl)) throw new Error("kitGraphToPlainData requires a KitImpl class instance");
  return KitSchema.parse({
    id: kit.id,
    name: kit.name,
    version: kit.version,
    types: kit.types?.map((t) => t.toPlain()),
    designs: kit.designs?.map((d) => d.toPlain()),
    tags: kit.tags?.map((t) => t.toPlain()),
    concepts: kit.concepts?.map((c) => c.toPlain()),
    families: kit.families?.map((f) => f.toPlain()),
    qualities: kit.qualities?.map((q) => q.toPlain()),
    files: kit.files?.map((f) => f.toPlain()),
    folders: kit.folders?.map((f) => f.toPlain()),
    authors: kit.authors?.map((a) => a.toPlain()),
    remote: kit.remote,
    homepage: kit.homepage,
    license: kit.license,
    preview: kit.preview,
    icon: kit.icon,
    image: kit.image,
    description: kit.description,
    attributes: kit.attributes?.map((a) => a.toPlain()),
    createdAt: kit.createdAt,
    updatedAt: kit.updatedAt,
  } as KitData);
};

/** Wire / {@link KitData} ÔåÆ live {@link KitImpl}; identity on class instances. */
export const asKitInstance = (kit: KitLike): KitImpl => (kit instanceof KitImpl ? kit : new KitImpl(KitSchema.parse(stripNullsJsonClone(kit) as unknown)));

/** Merkle-cached flatten on a resolved {@link KitImpl} instance (accepts wire data via {@link asKitInstance}). */
export function flattenDesignOptimizedForKit(kit: KitLike, designId: string): DesignOperationResult {
  return asKitInstance(kit).flattenDesignMerkle(designId);
}

function requireKit(k: KitImpl): KitImpl {
  if (!(k instanceof KitImpl)) throw new Error("Expected a KitImpl class instance");
  return k;
}

/**
 * Serializes KitImpl for transport.
 **/
export const serializeKit = (kit: KitLike): string => JSON.stringify(kitGraphToPlainData(asKitInstance(kit)));
/**
 **/
export const deserializeKit = (json: string): KitImpl => new KitImpl(KitSchema.parse(JSON.parse(json, (_key, value) => (value === null ? undefined : value))));

/**
 * Round-trips a kit through JSON so tests (or callers) can mutate without touching shared fixtures.
 **/
export const duplicateKitForIsolation = (kit: KitImpl): KitImpl => deserializeKit(serializeKit(kit));

/**
 * ­ƒôÉComputes the diff between two kits.
 */
export const getKitDiff = (before: KitLike, after: KitLike): KitDiff => asKitInstance(before).getDiff(asKitInstance(after));

/**
 * ­ƒöäComputes the inverse of an applied diff relative to the original kit.
 */
export const inverseKitDiff = (original: KitLike, appliedDiff: KitDiff): KitDiff => inverseKitGraphDiff(asKitInstance(original), appliedDiff);

/**
 * ­ƒöÇMerges two kit diffs into one.
 */
export const mergeKitDiff = (diff1: KitDiff, diff2: KitDiff): KitDiff => mergeKitGraphDiff(diff1, diff2);

/**
 * ­ƒöäComputes the full change (forward + backward + validation) between two kits.
 */
export const getKitChange = (before: KitImpl, after: KitImpl): KitGraphChange => KitImpl.changeBetween(before, after);

// #region ­ƒº®KitEntity (synchronized kit facade)

export interface SemanticCommand {
  readonly type: string;
}

export class FlattenDesignCommand implements SemanticCommand {
  readonly type = "design.flatten";
  constructor(public readonly designId: KitEntityDesignId) { }
}

export class DeletePieceCommand implements SemanticCommand {
  readonly type = "piece.delete";
  constructor(public readonly pieceId: KitEntityPieceId) { }
}

export class ChangePieceTypeCommand implements SemanticCommand {
  readonly type = "piece.changeType";
  constructor(
    public readonly pieceId: KitEntityPieceId,
    public readonly nextTypeId: KitEntityTypeId,
  ) { }
}

function validationReportFromGraph(v: KitDiffValidationResult): ValidationReport {
  return {
    infos: (v.infos ?? []).map((n) => ({ code: n.code ?? "info", message: n.message ?? "" })),
    warnings: (v.warnings ?? []).map((n) => ({ code: n.code ?? "warning", message: n.message ?? "" })),
    errors: (v.errors ?? []).map((n) => ({ code: n.code ?? "error", message: n.message ?? "" })),
  };
}

function graphValidationFromLedgerReport(r: ValidationReport): KitDiffValidationResult {
  return {
    ok: r.errors.length === 0,
    errors: r.errors.map((e) => ({ code: e.code, message: e.message })),
    warnings: r.warnings.map((e) => ({ code: e.code, message: e.message })),
    infos: r.infos.map((e) => ({ code: e.code, message: e.message })),
  };
}

export function ledgerKitChangeFromGraph(graph: KitGraphChange, origin: ChangeOrigin, revision: number, baseRevision: number, interactionId?: InteractionId, metadata?: Record<string, string>): KitChange {
  return {
    id: id(),
    origin,
    interactionId,
    baseRevision,
    revision,
    diff: graph.forward,
    inverse: graph.backward,
    report: validationReportFromGraph(graph.validation),
    createdAt: new Date().toISOString(),
    metadata,
  };
}

function graphKitChangeFromLedger(c: KitChange): KitGraphChange {
  return {
    forward: c.diff,
    backward: c.inverse,
    validation: graphValidationFromLedgerReport(c.report),
  };
}

/** @alias {@link id} ÔÇö uuid v7 strings for {@link KitEntity} interactions. */
export { id as uuidv7 };

export function emptyKitWireDto(): KitDTO {
  const now = new Date().toISOString();
  return {
    uuid: id(),
    name: "Untitled Kit",
    types: [],
    designs: [],
    version: "0",
    tags: [],
    concepts: [],
    families: [],
    qualities: [],
    files: [],
    folders: [],
    authors: [],
    attributes: [],
    createdAt: now,
    updatedAt: now,
  } as unknown as KitDTO;
}

function kitDataFromWireDto(dto: KitDTO): KitData {
  const d = dto as Record<string, unknown>;
  const merged = {
    ...d,
    id: (d.uuid as string) ?? (d.id as string) ?? id(),
    types: d.types ?? [],
    designs: d.designs ?? [],
    tags: d.tags ?? [],
    concepts: d.concepts ?? [],
    families: d.families ?? [],
    qualities: d.qualities ?? [],
    files: d.files ?? [],
    folders: d.folders ?? [],
    authors: d.authors ?? [],
    attributes: d.attributes ?? [],
    version: d.version ?? "0",
    createdAt: d.createdAt ?? new Date().toISOString(),
    updatedAt: d.updatedAt ?? new Date().toISOString(),
  };
  delete (merged as { uuid?: string }).uuid;
  return KitSchema.parse(merged as unknown);
}

function kitWireProjectionFromImpl(k: KitImpl): KitWire {
  const data = k.toData();
  return {
    uuid: data.id,
    name: data.name,
    types: (data.types ?? []).map((t) => ({ id: t.id, name: t.name })),
    designs: (data.designs ?? []).map((d) => ({ id: d.id, name: d.name ?? "" })),
  };
}

export function emptyLedgerDiff(): KitDiff {
  return {};
}

export function kitEntityDiffIsBlocking(report: ValidationReport): boolean {
  return report.errors.length > 0;
}

export function validateKitEntityDiff(kit: KitEntity, diff: KitDiff): ValidationReport {
  return validationReportFromGraph(kit._inner.validateGraphDiff(diff, false));
}

export function normalizeLedgerDiff(diff: KitDiff): KitDiff {
  return DiffComposer.normalize(diff);
}

export function composeLedgerDiffs(a: KitDiff, b: KitDiff): KitDiff {
  return mergeKitDiff(a, b);
}

export function invertLedgerDiff(kit: KitEntity, diff: KitDiff): KitDiff {
  return kit._inner.inverseDiffFromPreApplyState(diff);
}

export function squashLedgerChangesForward(changes: KitChange[]): KitDiff {
  return changes.reduce((acc, x) => mergeKitDiff(acc, x.diff), emptyLedgerDiff());
}

export function squashLedgerChangesBackward(changes: KitChange[]): KitDiff {
  return changes.reduceRight((acc, x) => mergeKitDiff(acc, x.inverse), emptyLedgerDiff());
}

export function expandSemanticCommandToDiff(kit: KitEntity, command: SemanticCommand): KitDiff {
  switch (command.type) {
    case "design.flatten": {
      const c = command as FlattenDesignCommand;
      const design = kit._inner.findDesign(c.designId);
      if (!design) throw new Error(`Design ${c.designId} not found`);
      const op = design.runFlattenOptimized();
      if (!op.ok || !op.diff) {
        throw new Error(`flatten failed: ${op.errors.map((e) => e.message).join("; ")}`);
      }
      return {
        designs: {
          updated: [{ design: { id: design.id }, diff: op.diff.forward }],
        },
      };
    }
    case "piece.delete": {
      const c = command as DeletePieceCommand;
      const { design } = kit._findDesignHostingPiece(c.pieceId);
      const res = deletePiecesAndConnectionsInDesign(kit._inner, design, [c.pieceId], []);
      if (!res.ok || !res.diff) {
        throw new Error(`delete piece failed: ${res.errors.map((e) => e.message).join("; ")}`);
      }
      return {
        designs: {
          updated: [{ design: { id: design.id }, diff: res.diff }],
        },
      };
    }
    case "piece.changeType": {
      const c = command as ChangePieceTypeCommand;
      const { design } = kit._findDesignHostingPiece(c.pieceId);
      return {
        designs: {
          updated: [
            {
              design: { id: design.id },
              diff: {
                pieces: {
                  updated: [{ piece: { id: c.pieceId }, diff: { type: { id: c.nextTypeId } } }],
                },
              },
            },
          ],
        },
      };
    }
    default:
      throw new Error(`Unknown command: ${(command as SemanticCommand).type}`);
  }
}

export function applyLedgerDiffToKitEntity(kit: KitEntity, diff: KitDiff): void {
  kit._inner.replayChangeUnchecked(diff);
}

class KitBackboneBridge implements Backbone {
  private owner: KitEntity | undefined;

  wire(owner: KitEntity): void {
    this.owner = owner;
  }

  async changed(change: KitGraphChange): Promise<void> {
    const bb = this.owner?._peekKitBackbone();
    if (!bb) return;
    const impl = this.owner!._inner;
    const rev = impl.getHistoryInfo().revision;
    await bb.submitCommittedChange(ledgerKitChangeFromGraph(change, "local-finalize", rev + 1, rev, impl.activeTransactionId, { channel: "commit" }));
  }
}

export function createLocalBackbone(input: { path: string }): KitBackbone {
  return {
    kind: "local",
    async open() {
      void input.path;
    },
    async close() { },
    async importSnapshot() { },
    async exportSnapshot() {
      return emptyKitWireDto();
    },
    async submitCommittedChange() { },
  };
}

export function createDevBackbone(input: { jsonFilePath: string }): KitBackbone {
  return {
    kind: "dev",
    async open() {
      void input.jsonFilePath;
    },
    async close() { },
    async importSnapshot() { },
    async exportSnapshot() {
      return emptyKitWireDto();
    },
    async submitCommittedChange() { },
  };
}

export function createRemoteBackbone(input: { url: string; token?: string }): KitBackbone {
  return {
    kind: "remote",
    async open() {
      void input.url;
      void input.token;
    },
    async close() { },
    async importSnapshot() { },
    async exportSnapshot() {
      return emptyKitWireDto();
    },
    async submitCommittedChange() { },
  };
}

export class KitInteractionEntity implements KitInteraction {
  constructor(
    public uuid: InteractionId,
    public label: string,
    public selection: KitSelection = { types: [], designs: [] },
  ) { }
}

export class KitEntityType {
  constructor(
    private readonly _host: KitEntity,
    private readonly _type: Type,
  ) { }

  get id(): KitEntityTypeId {
    return this._type.id;
  }

  get name(): string {
    return this._type.name;
  }
}

export class KitEntityPiece {
  constructor(
    private readonly _host: KitEntity,
    private readonly _piece: Piece,
  ) { }

  get id(): KitEntityPieceId {
    return this._piece.id;
  }

  get name(): string {
    return this._piece.name ?? "";
  }

  delete(): this {
    this._host._applySemanticCommand(new DeletePieceCommand(this.id));
    return this;
  }

  changeType(nextType: KitEntityType): this {
    this._host._applySemanticCommand(new ChangePieceTypeCommand(this.id, nextType.id));
    return this;
  }
}

export class KitEntityDesign {
  constructor(
    private readonly _host: KitEntity,
    private readonly _design: Design,
  ) { }

  get id(): KitEntityDesignId {
    return this._design.id;
  }

  get name(): string {
    return this._design.name ?? "";
  }

  flatten(): this {
    this._host._applySemanticCommand(new FlattenDesignCommand(this.id));
    return this;
  }

  findPiece(where: { id?: KitEntityPieceId; name?: string }): KitEntityPiece {
    const pieceId = this._host._findPieceIdInDesign(this.id, where);
    return this._host._pieceEntityById(pieceId);
  }
}

export class KitInteractionsApi {
  constructor(private readonly _kit: KitEntity) { }

  start(label: string): InteractionId {
    return this._kit._inner.beginTransaction(label).id;
  }

  setActive(id: InteractionId): this {
    this._kit._inner.setActiveTransaction(id);
    return this;
  }

  unsetActive(): this {
    this._kit._inner.clearActiveTransaction();
    return this;
  }

  finalize(id: InteractionId): this {
    this._kit._inner.finalizeTransaction(id);
    if (this._kit._inner.activeTransactionId === id) {
      this._kit._inner.clearActiveTransaction();
    }
    return this;
  }

  abort(id: InteractionId): this {
    this._kit._inner.abortTransaction(id);
    if (this._kit._inner.activeTransactionId === id) {
      this._kit._inner.clearActiveTransaction();
    }
    return this;
  }

  undo(id?: InteractionId): this {
    const resolvedId = id ?? this._kit._inner.activeTransactionId;
    if (!resolvedId) throw new Error("No active interaction");
    this._kit._inner.undoWithinTransaction(resolvedId);
    return this;
  }

  redo(id?: InteractionId): this {
    const resolvedId = id ?? this._kit._inner.activeTransactionId;
    if (!resolvedId) throw new Error("No active interaction");
    this._kit._inner.redoWithinTransaction(resolvedId);
    return this;
  }

  list(): KitInteractionEntity[] {
    return this._kit._inner.getOpenTransactions().map((x) => new KitInteractionEntity(x.id, x.label ?? "", { types: [], designs: [] }));
  }
}

export class KitEntityIndexes {
  readonly typesById = new Map<KitEntityTypeId, Type>();
  readonly designsById = new Map<KitEntityDesignId, Design>();
  readonly piecesById = new Map<KitEntityPieceId, Piece>();

  rebuild(entity: KitEntity): void {
    this.typesById.clear();
    this.designsById.clear();
    this.piecesById.clear();
    for (const t of entity._inner.types ?? []) {
      this.typesById.set(t.id, t);
    }
    for (const d of entity._inner.designs ?? []) {
      this.designsById.set(d.id, d);
      for (const p of d.pieces ?? []) {
        this.piecesById.set(p.id, p);
      }
    }
  }

  findPieceIdInDesignByName(designId: KitEntityDesignId, name: string): KitEntityPieceId {
    const design = this.designsById.get(designId);
    const hit = design?.pieces?.find((p) => p.name === name);
    if (!hit) throw new Error(`Piece named "${name}" not found in design ${designId}`);
    return hit.id;
  }
}

export class KitEntityCaches {
  rebuild(entity: KitEntity, indexes: KitEntityIndexes): void {
    void entity;
    void indexes;
  }

  invalidateByDiff(_diff: KitDiff, _indexes: KitEntityIndexes, _entity: KitEntity): void {
    void _diff;
    void _indexes;
    void _entity;
  }
}

export interface SynchronizedKit extends KitWire {
  interactions: KitInteractionsApi;
  importKit(kit: KitWire): Promise<this>;
  exportWire(): Promise<KitWire>;
  open(options?: unknown): Promise<this>;
  close(): Promise<void>;
  setActiveInteraction(id: InteractionId): this;
  unsetActiveInteraction(): this;
  undo(): this;
  redo(): this;
}

export class KitEntity implements SynchronizedKit {
  public readonly interactions = new KitInteractionsApi(this);

  private readonly _bridge = new KitBackboneBridge();
  private _kitBackbone?: KitBackbone;

  private readonly _indexes = new KitEntityIndexes();
  private readonly _caches = new KitEntityCaches();

  private readonly _typeEntities = new Map<KitEntityTypeId, KitEntityType>();
  private readonly _designEntities = new Map<KitEntityDesignId, KitEntityDesign>();
  private readonly _pieceEntities = new Map<KitEntityPieceId, KitEntityPiece>();

  #inner: KitImpl;

  private readonly _seenLedgerIds = new Set<ChangeId>();
  private readonly _backboneSink: BackboneSink;

  constructor(input: { dto: KitDTO; backbone?: KitBackbone }) {
    this._kitBackbone = input.backbone;
    this._bridge.wire(this);
    this.#inner = new KitImpl(kitDataFromWireDto(input.dto), this._bridge);
    this._indexes.rebuild(this);
    this._caches.rebuild(this, this._indexes);

    this._backboneSink = {
      changed: (change) => this._onKitBackboneInbound(change),
      failed: (error) => console.error(error),
    };
  }

  get uuid(): KitEntityUUID {
    return this.#inner.id;
  }

  get name(): string {
    return this.#inner.name;
  }

  get types(): KitEntityType[] {
    return (this.#inner.types ?? []).map((t) => this._typeEntity(t));
  }

  get designs(): KitEntityDesign[] {
    return (this.#inner.designs ?? []).map((d) => this._designEntity(d));
  }

  /** @internal */
  get _inner(): KitImpl {
    return this.#inner;
  }

  /** @internal */
  _peekKitBackbone(): KitBackbone | undefined {
    return this._kitBackbone;
  }

  static async create(input: { dto?: KitDTO; backbone?: KitBackbone; openOptions?: unknown }): Promise<KitEntity> {
    const dto = input.dto ?? (input.backbone ? await input.backbone.exportSnapshot() : emptyKitWireDto());
    const kit = new KitEntity({ dto, backbone: input.backbone });
    if (input.backbone) {
      await kit.open(input.openOptions);
    }
    return kit;
  }

  async open(options?: unknown): Promise<this> {
    if (!this._kitBackbone) return this;
    await this._kitBackbone.open({ kitId: this.uuid, sink: this._backboneSink, options });
    return this;
  }

  async close(): Promise<void> {
    if (!this._kitBackbone) return;
    await this._kitBackbone.close();
  }

  async importKit(kit: KitWire): Promise<this> {
    this._assertKitEntityReady();
    const data = kitDataFromWireDto(kit as KitDTO);
    this.#inner = new KitImpl(data, this._bridge);
    this._clearKitEntityCaches();
    this._indexes.rebuild(this);
    this._caches.rebuild(this, this._indexes);
    if (this._kitBackbone) {
      await this._kitBackbone.importSnapshot(kit as KitDTO);
    }
    return this;
  }

  async exportWire(): Promise<KitWire> {
    if (this._kitBackbone) {
      return structuredClone(await this._kitBackbone.exportSnapshot()) as KitWire;
    }
    return kitWireProjectionFromImpl(this.#inner);
  }

  findDesign(where: { id?: KitEntityDesignId; name?: string }): KitEntityDesign {
    const designId = this._findDesignId(where);
    return this._designEntityById(designId);
  }

  findType(where: { id?: KitEntityTypeId; name?: string }): KitEntityType {
    const typeId = this._findTypeId(where);
    return this._typeEntityById(typeId);
  }

  setActiveInteraction(id: InteractionId): this {
    this.interactions.setActive(id);
    return this;
  }

  unsetActiveInteraction(): this {
    this.interactions.unsetActive();
    return this;
  }

  undo(): this {
    if (this.#inner.activeTransactionId) {
      this.#inner.undoWithinTransaction(this.#inner.activeTransactionId);
      return this;
    }
    if (this.#inner.getOpenTransactions().length > 0) {
      throw new Error("History undo requires no open interactions");
    }
    this.#inner.undo();
    return this;
  }

  redo(): this {
    if (this.#inner.activeTransactionId) {
      this.#inner.redoWithinTransaction(this.#inner.activeTransactionId);
      return this;
    }
    if (this.#inner.getOpenTransactions().length > 0) {
      throw new Error("History redo requires no open interactions");
    }
    this.#inner.redo();
    return this;
  }

  resolveKitEntityConflict(_resolution: { kind: "discard" | "accept-warnings" | "force-apply" }): this {
    void _resolution;
    this.#inner.resolveConflict();
    return this;
  }

  _applySemanticCommand(command: SemanticCommand): void {
    this._assertKitEntityReady();
    this.#inner.requireActiveTransactionId();
    const diff = normalizeLedgerDiff(expandSemanticCommandToDiff(this, command));
    this.#inner._applyDiff(diff, {});
    this._indexes.rebuild(this);
    this._caches.invalidateByDiff(diff, this._indexes, this);
  }

  _findDesignId(where: { id?: KitEntityDesignId; name?: string }): KitEntityDesignId {
    if (where.id) return where.id;
    const hit = this.#inner.designs?.find((x) => x.name === where.name);
    if (!hit) throw new Error("Design not found");
    return hit.id;
  }

  _findTypeId(where: { id?: KitEntityTypeId; name?: string }): KitEntityTypeId {
    if (where.id) return where.id;
    const hit = this.#inner.types?.find((x) => x.name === where.name);
    if (!hit) throw new Error("Type not found");
    return hit.id;
  }

  _findPieceIdInDesign(designId: KitEntityDesignId, where: { id?: KitEntityPieceId; name?: string }): KitEntityPieceId {
    if (where.id) return where.id;
    if (!where.name) throw new Error("findPiece requires id or name");
    return this._indexes.findPieceIdInDesignByName(designId, where.name);
  }

  _findDesignHostingPiece(pieceId: KitEntityPieceId): { design: Design; piece: Piece } {
    this._indexes.rebuild(this);
    const piece = this._indexes.piecesById.get(pieceId);
    if (!piece) throw new Error(`Piece ${pieceId} not found`);
    const design = this.#inner.designs?.find((d) => d.pieces?.some((p) => p.id === pieceId));
    if (!design) throw new Error(`No design contains piece ${pieceId}`);
    return { design, piece };
  }

  _typeEntity(t: Type): KitEntityType {
    let e = this._typeEntities.get(t.id);
    if (!e) {
      e = new KitEntityType(this, t);
      this._typeEntities.set(t.id, e);
    }
    return e;
  }

  _typeEntityById(id: KitEntityTypeId): KitEntityType {
    const t = this.#inner.findType(id);
    if (!t) throw new Error(`Type ${id} not found`);
    return this._typeEntity(t);
  }

  _designEntity(d: Design): KitEntityDesign {
    let e = this._designEntities.get(d.id);
    if (!e) {
      e = new KitEntityDesign(this, d);
      this._designEntities.set(d.id, e);
    }
    return e;
  }

  _designEntityById(id: KitEntityDesignId): KitEntityDesign {
    const d = this.#inner.findDesign(id);
    if (!d) throw new Error(`Design ${id} not found`);
    return this._designEntity(d);
  }

  _pieceEntityById(pieceId: KitEntityPieceId): KitEntityPiece {
    this._indexes.rebuild(this);
    const piece = this._indexes.piecesById.get(pieceId);
    if (!piece) throw new Error(`Piece ${pieceId} not found`);
    let e = this._pieceEntities.get(pieceId);
    if (!e) {
      e = new KitEntityPiece(this, piece);
      this._pieceEntities.set(pieceId, e);
    }
    return e;
  }

  private _onKitBackboneInbound(change: KitChange): void {
    if (this._seenLedgerIds.has(change.id)) return;
    const graph = graphKitChangeFromLedger(change);
    const normalized = DiffComposer.normalize(graph.forward);
    const v = validateKitGraphDiff(this.#inner, normalized, false);
    if (!v.ok || v.errors.length > 0) {
      console.error("Inbound backbone change failed validation", v.errors);
      return;
    }
    const diffToApply = v.diff ?? normalized;
    this.#inner._applyDiff(diffToApply, {
      notifyBackbone: false,
      skipGlobalHistory: true,
      inboundCommitted: true,
      inboundActor: { changeId: change.id, actorId: change.actorId, actorDisplayName: change.actorDisplayName },
    });
    this._seenLedgerIds.add(change.id);
    this._indexes.rebuild(this);
  }

  private _clearKitEntityCaches(): void {
    this._typeEntities.clear();
    this._designEntities.clear();
    this._pieceEntities.clear();
  }

  private _assertKitEntityReady(): void {
    if (this.#inner.kitPhase === "frozen") {
      const c = this.#inner.getConflict();
      throw new Error(c?.validationReport?.errors?.[0]?.message ?? "Kit is frozen");
    }
  }
}

// #endregion ­ƒº®KitEntity (synchronized kit facade)

/**
 * Applies `diff` to `kit` in place (no validation). Prefer semantic methods or {@link KitImpl._applyDiff} for validated edits.
 */
export const applyKitDiff = (kit: KitLike, diff: KitDiff): KitImpl => {
  const source = asKitInstance(kit);
  const clone = duplicateKitForIsolation(source);
  clone.replayChangeUnchecked(diff);
  return clone;
};

/**
 * Definition of KitMetaSchema.
 **/
export const KitMetaSchema = KitSchema.omit({ types: true, designs: true, tags: true, concepts: true, families: true, qualities: true, files: true, folders: true, authors: true, attributes: true });
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
export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, tags: true, concepts: true, families: true, qualities: true, files: true, folders: true, authors: true, attributes: true }).extend({
  types: z.array(TypeMetaSchema).optional(),
  designs: z.array(DesignMetaSchema).optional(),
  tags: z.array(TagMetaSchema).optional(),
  concepts: z.array(ConceptMetaSchema).optional(),
  families: z.array(FamilySchema).optional(),
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
  if (result.representations) result.representations = result.representations.map((m: Representation) => RepresentationMetaSchema.parse(m));
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
 * Converts a KitImpl to KitMeta.
 **/
export const toKitMeta = (kit: KitImpl): KitMeta => KitMetaSchema.parse(kit);
/**
 * Converts a KitImpl to KitShallow.
 **/
export const toKitShallow = (kit: KitImpl): KitShallow => {
  const result: any = { ...kit };
  if (result.types) result.types = result.types.map((t: Type) => TypeMetaSchema.parse(t));
  if (result.designs) result.designs = result.designs.map((d: Design) => DesignMetaSchema.parse(d));
  if (result.tags) result.tags = result.tags.map((t: Tag) => TagMetaSchema.parse(t));
  if (result.concepts) result.concepts = result.concepts.map((c: Concept) => ConceptMetaSchema.parse(c));
  if (result.families) result.families = result.families.map((f: Family) => FamilySchema.parse(f));
  if (result.qualities) result.qualities = result.qualities.map((q: Quality) => QualityMetaSchema.parse(q));
  if (result.files) result.files = result.files.map((f: File) => FileMetaSchema.parse(f));
  if (result.folders) result.folders = result.folders.map((f: Folder) => FolderMetaSchema.parse(f));
  if (result.authors) result.authors = result.authors.map((a: Author) => AuthorMetaSchema.parse(a));
  if (result.attributes) result.attributes = result.attributes.map((a: Attribute) => AttributeMetaSchema.parse(a));
  return KitShallowSchema.parse(result);
};
/**
 * Zod schema for KitImpl diff validation.
 **/
export const KitDiffSchema = KitSchema.partial().omit({ types: true, designs: true, tags: true, concepts: true, families: true, qualities: true, authors: true, files: true, folders: true, attributes: true }).extend({
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  tags: TagsDiffSchema.optional(),
  concepts: ConceptsDiffSchema.optional(),
  families: FamiliesDiffSchema.optional(),
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
 * Diff type for tracking KitImpl changes.
 **/
export type KitDiff = z.infer<typeof KitDiffSchema>;

/**
 * Deep duplicate of a kit diff for tests / isolated apply simulation.
 **/

// ­ƒº¼EntityIdType maps entity kind names to their ID interface types.
type EntityIdType = { id: string };
// ­ƒöÇCollectionDiff represents added, removed, and changed items in a collection.
// `added` is intentionally `unknown[]`: Zod-inferred kit/design diffs carry plain JSON shapes;
// `applyCollectionDiff` hydrates entries via `hydrateAdded`.
type CollectionDiff<K extends string, T extends { id: string }, D> = {
  removed?: EntityIdType[];
  updated?: Array<{ [key in K]: EntityIdType } & { diff: D }>;
  added?: unknown[];
};
// ­ƒöÇgetCollectionDiff computes the diff between two collections by key.
const getCollectionDiff = <K extends string, T extends { id: string }, D>(entityKey: K, before: T[], after: T[], getItemDiff: (before: T, after: T) => D): CollectionDiff<K, T, D> => {
  const diff: CollectionDiff<K, T, D> = {};
  const beforeIds = new Set(before.map((i) => i.id));
  const afterIds = new Set(after.map((i) => i.id));
  const removed = before.filter((i) => !afterIds.has(i.id)).map((i) => ({ id: i.id }));
  if (removed.length > 0) diff.removed = removed;
  const updated = before
    .filter((i) => afterIds.has(i.id))
    .map((i) => {
      const afterItem = after.find((a) => a.id === i.id)!;
      const itemDiff = getItemDiff(i, afterItem);
      return { [entityKey]: { id: i.id }, diff: itemDiff } as { [key in K]: EntityIdType } & { diff: D };
    })
    .filter((u) => Object.keys(u.diff as any).length > 0);
  if (updated.length > 0) diff.updated = updated;
  const added = after.filter((i) => !beforeIds.has(i.id));
  if (added.length > 0) diff.added = added;
  return diff;
};
// ­ƒöÇinverseCollectionDiff inverts a collection diff to reverse its effect.
const inverseCollectionDiff = <K extends string, T extends { id: string }, D>(entityKey: K, original: T[], appliedDiff: CollectionDiff<K, T, D>, inverseItemDiff: (original: T, appliedDiff: D) => D): CollectionDiff<K, T, D> => {
  const inverse: CollectionDiff<K, T, D> = {};
  const removedIds = appliedDiff.removed?.map((r) => r.id) ?? [];
  if (appliedDiff.removed) inverse.added = original.filter((i) => removedIds.includes(i.id));
  if (appliedDiff.added) inverse.removed = appliedDiff.added.map((i) => ({ id: (i as T).id }));
  if (appliedDiff.updated) {
    inverse.updated = appliedDiff.updated
      .filter((u) => {
        const entityId = (u as any)[entityKey] as EntityIdType;
        return original.some((i) => i.id === entityId.id);
      })
      .map((u) => {
        const entityId = (u as any)[entityKey] as EntityIdType;
        const originalItem = original.find((i) => i.id === entityId.id)!;
        return { [entityKey]: entityId, diff: inverseItemDiff(originalItem, u.diff) } as { [key in K]: EntityIdType } & { diff: D };
      });
  }
  return inverse;
};
// ­ƒöÇapplyCollectionDiff applies a collection diff to produce an updated collection.
const applyCollectionDiff = <K extends string, T extends { id: string }, D>(entityKey: K, items: T[], diff: CollectionDiff<K, T, D> | undefined, applyItemDiff: (target: T, diff: D) => void, hydrateAdded: (raw: unknown) => T): void => {
  if (!diff) return;
  if (diff.removed) {
    const removedIds = new Set(diff.removed.map((r) => r.id));
    for (let i = items.length - 1; i >= 0; i--) {
      if (removedIds.has(items[i].id)) items.splice(i, 1);
    }
  }
  if (diff.updated) {
    for (const update of diff.updated) {
      const entityId = (update as any)[entityKey] as EntityIdType;
      const item = items.find((i) => i.id === entityId.id);
      if (item) applyItemDiff(item, update.diff);
    }
  }
  if (diff.added) {
    items.push(...diff.added.map(hydrateAdded));
  }
};

/** Applies a design diff in place (kit graph + local algorithms). */
function applyDesignDiffCore(target: Design, diff: DesignDiff): void {
  if (diff.name !== undefined) target.name = diff.name;
  if (diff.families !== undefined) target.families = diff.families ?? undefined;
  if (diff.isAbstract !== undefined) target.isAbstract = diff.isAbstract;
  if (diff.createdAt !== undefined) target.createdAt = diff.createdAt;
  if (diff.updatedAt !== undefined) target.updatedAt = diff.updatedAt;
  if (diff.folder !== undefined) target.folder = diff.folder;
  if (diff.canScale !== undefined) target.canScale = diff.canScale;
  if (diff.canMirror !== undefined) target.canMirror = diff.canMirror;
  if (diff.unit !== undefined) target.unit = diff.unit;
  if (diff.activeLayer !== undefined) target.activeLayer = diff.activeLayer;
  if (diff.location !== undefined) target.location = diff.location;
  if (diff.icon !== undefined) target.icon = diff.icon;
  if (diff.image !== undefined) target.image = diff.image;
  if (diff.description !== undefined) target.description = diff.description;
  if (diff.authors !== undefined) target.authors = diff.authors as any;
  if (diff.concepts !== undefined) target.concepts = diff.concepts;
  if (diff.pieces) {
    if (!target.pieces) target.pieces = [];
    applyCollectionDiff("piece", target.pieces, diff.pieces, applyPieceDiff, (raw) => new Piece(raw as PiecePlain, target, target.getKit()));
  }
  if (diff.connections) {
    if (!target._connections) target._connections = [];
    applyCollectionDiff("connection", target._connections, diff.connections, applyConnectionDiff, (raw) => new Connection(raw as ConnectionPlain, target));
  }
  if (diff.stats) {
    if (!target.stats) target.stats = [];
    applyCollectionDiff("stat", target.stats, diff.stats, applyStatDiff, (raw) => new Stat(raw as StatPlain));
  }
  if (diff.props) {
    if (!target.props) target.props = [];
    applyCollectionDiff("prop", target.props, diff.props, applyPropDiff, (raw) => new Prop(raw as PropPlain));
  }
  if (diff.layers) {
    if (!target.layers) target.layers = [];
    applyCollectionDiff("layer", target.layers, diff.layers, applyLayerDiff, (raw) => new Layer(raw as LayerPlain));
  }
  if (diff.groups) {
    if (!target.groups) target.groups = [];
    applyCollectionDiff("group", target.groups, diff.groups, applyGroupDiff, (raw) => new Group(raw as GroupPlain));
  }
  if (diff.attributes) {
    if (!target.attributes) target.attributes = [];
    applyAttributesDiff(target.attributes, diff.attributes);
  }
}

// ­ƒöÇmergeCollectionDiff merges two collection diffs into one.
const mergeCollectionDiff = <K extends string, T extends { id: string }, D>(entityKey: K, diff1: CollectionDiff<K, T, D>, diff2: CollectionDiff<K, T, D>, mergeItemDiff: (diff1: D, diff2: D) => D): CollectionDiff<K, T, D> => {
  const removed = [...(diff1.removed ?? []), ...(diff2.removed ?? [])];
  const added = [...(diff1.added ?? []), ...(diff2.added ?? [])];
  const getEntityId = (u: any) => (u[entityKey] as EntityIdType).id;
  const updated1Map = new Map((diff1.updated ?? []).map((u) => [getEntityId(u), u.diff]));
  const updated2Map = new Map((diff2.updated ?? []).map((u) => [getEntityId(u), u.diff]));
  const allIds = new Set([...updated1Map.keys(), ...updated2Map.keys()]);
  const updated = Array.from(allIds).map((id) => ({
    [entityKey]: { id },
    diff: mergeItemDiff(updated1Map.get(id) ?? ({} as D), updated2Map.get(id) ?? ({} as D)),
  })) as Array<{ [key in K]: EntityIdType } & { diff: D }>;
  return {
    removed: removed.length > 0 ? removed : undefined,
    updated: updated.length > 0 ? updated : undefined,
    added: added.length > 0 ? added : undefined,
  };
};

const typesActiveForStructuralDiff = (types: Type[] | undefined): Type[] => (types ?? []).filter((t) => (t.lifecycle ?? "active") !== "deleted");

/**
 * Computes the structural diff from `before` to `after` (both kit graphs).
 */
function computeKitGraphDiffBetween(before: KitImpl, after: KitImpl): KitDiff {
  before = asKitInstance(before);
  after = asKitInstance(after);
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
  const typesDiff = getCollectionDiff("type", typesActiveForStructuralDiff(before.types), typesActiveForStructuralDiff(after.types), getTypeDiff);
  if (Object.keys(typesDiff).length > 0) diff.types = typesDiff;
  const designsDiff = getCollectionDiff("design", before.designs ?? [], after.designs ?? [], getDesignDiff);
  if (Object.keys(designsDiff).length > 0) diff.designs = designsDiff;
  const tagsDiff = getTagsDiff(before.tags ?? [], after.tags ?? []);
  if (Object.keys(tagsDiff).length > 0) diff.tags = tagsDiff;
  const conceptsDiff = getConceptsDiff(before.concepts ?? [], after.concepts ?? []);
  if (Object.keys(conceptsDiff).length > 0) diff.concepts = conceptsDiff;
  const familiesDiff = getFamiliesDiff(before.families ?? [], after.families ?? []);
  if (Object.keys(familiesDiff).length > 0) diff.families = familiesDiff;
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
}

function inverseKitGraphDiff(original: KitImpl, appliedDiff: KitDiff): KitDiff {
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
  if (appliedDiff.families) inverse.families = inverseFamiliesDiff(original.families ?? [], appliedDiff.families);
  if (appliedDiff.qualities) inverse.qualities = inverseCollectionDiff("quality", original.qualities ?? [], appliedDiff.qualities, inverseQualityDiff);
  if (appliedDiff.files) inverse.files = inverseCollectionDiff("file", original.files ?? [], appliedDiff.files, inverseFileDiff);
  if (appliedDiff.folders) inverse.folders = inverseCollectionDiff("folder", original.folders ?? [], appliedDiff.folders, inverseFolderDiff);
  if (appliedDiff.authors) inverse.authors = inverseCollectionDiff("author", original.authors ?? [], appliedDiff.authors, inverseAuthorDiff);
  if (appliedDiff.attributes) inverse.attributes = inverseAttributesDiff(original.attributes ?? [], appliedDiff.attributes);
  return inverse;
}

function mergeKitGraphDiff(diff1: KitDiff, diff2: KitDiff): KitDiff {
  const mergeSimpleDiff = <D>(d1: D, d2: D): D => ({ ...d1, ...d2 });
  return {
    ...diff1,
    ...diff2,
    types: diff1.types || diff2.types ? mergeCollectionDiff("type", diff1.types ?? {}, diff2.types ?? {}, mergeTypeDiff) : undefined,
    designs: diff1.designs || diff2.designs ? mergeCollectionDiff("design", diff1.designs ?? {}, diff2.designs ?? {}, mergeDesignDiff) : undefined,
    tags: diff1.tags || diff2.tags ? mergeTagsDiff(diff1.tags ?? {}, diff2.tags ?? {}) : undefined,
    concepts: diff1.concepts || diff2.concepts ? mergeConceptsDiff(diff1.concepts ?? {}, diff2.concepts ?? {}) : undefined,
    families: diff1.families || diff2.families ? mergeFamiliesDiff(diff1.families ?? {}, diff2.families ?? {}) : undefined,
    qualities: diff1.qualities || diff2.qualities ? mergeCollectionDiff("quality", diff1.qualities ?? {}, diff2.qualities ?? {}, mergeQualityDiff) : undefined,
    files: diff1.files || diff2.files ? mergeCollectionDiff("file", diff1.files ?? {}, diff2.files ?? {}, mergeSimpleDiff) : undefined,
    folders: diff1.folders || diff2.folders ? mergeCollectionDiff("folder", diff1.folders ?? {}, diff2.folders ?? {}, mergeSimpleDiff) : undefined,
    authors: diff1.authors || diff2.authors ? mergeCollectionDiff("author", diff1.authors ?? {}, diff2.authors ?? {}, mergeSimpleDiff) : undefined,
    attributes: diff1.attributes || diff2.attributes ? mergeAttributesDiff(diff1.attributes ?? {}, diff2.attributes ?? {}) : undefined,
  };
}

function collectEntityIdsFromKitDiff(diff: KitDiff): Set<string> {
  const out = new Set<string>();
  const add = (g: string | undefined) => {
    if (g) out.add(g);
  };
  if (diff.types) {
    for (const x of diff.types.added ?? []) add(x.id);
    for (const x of diff.types.removed ?? []) add(x.id);
    for (const u of diff.types.updated ?? []) add(u.type?.id);
  }
  if (diff.designs) {
    const d = diff.designs;
    for (const x of d.added ?? []) add(x.id);
    for (const x of d.removed ?? []) add(x.id);
    for (const u of d.updated ?? []) {
      add(u.design?.id);
      const pd = u.diff?.pieces;
      if (pd) {
        for (const p of pd.added ?? []) add(p.id);
        for (const p of pd.removed ?? []) add(p.id);
        for (const pu of pd.updated ?? []) add(pu.piece?.id);
      }
      const cd = u.diff?.connections;
      if (cd) {
        for (const c of cd.added ?? []) add(c.id);
        for (const c of cd.removed ?? []) add(c.id);
        for (const cu of cd.updated ?? []) add(cu.connection?.id);
      }
    }
  }
  return out;
}

function recomputeTxNet(tx: KitRuntimeTransaction): void {
  let nf: KitDiff = {};
  let nb: KitDiff = {};
  for (const ch of tx.done) {
    nf = mergeKitGraphDiff(nf, ch.forward);
    nb = mergeKitGraphDiff(ch.backward, nb);
  }
  tx.netForward = nf;
  tx.netBackward = nb;
  tx.touchedEntities.clear();
  for (const ch of tx.done) {
    for (const g of collectEntityIdsFromKitDiff(ch.forward)) tx.touchedEntities.add(g);
  }
}

/** Deterministic diff composition for transactional net forward/backward squashing. */
export class DiffComposer {
  static compose(a: KitDiff, b: KitDiff): KitDiff {
    return mergeKitGraphDiff(a, b);
  }
  static normalize(diff: KitDiff): KitDiff {
    return KitImpl.cloneGraphDiff(diff);
  }
  static touchedEntities(diff: KitDiff): Set<string> {
    return collectEntityIdsFromKitDiff(diff);
  }
}

/** Semantic command labels ÔÇö each maps to one deterministic KitDiff expansion (cross-language parity). */
export type SemioCommandKind = "DeletePiece" | "MovePiece" | "RenamePiece" | "ReconnectConnection" | "DeletePiecesCascade" | "NormalizeStaleConnections";

/**
 * Mutable kit document: holds a single KitImpl graph and applies diffs in place.
 * Prefer passing this (or the underlying `KitImpl`) by reference instead of cloning.
 **/
export class KitDocument {
  constructor(public readonly root: KitImpl) { }
  apply(diff: KitDiff): void {
    this.root.replayChangeUnchecked(diff);
  }
  /** Computes the diff from another kit snapshot to this document's current state. */
  diffSince(other: KitImpl): KitDiff {
    return asKitInstance(other).getDiff(this.root);
  }
}

// #endregion ÔÅ▒´©ÅKitImpl

// #region ­ƒûÑ´©ÅHash
// Merkle hash functions for all entities. Each hash function computes a deterministic
// SHA-256 hex digest. Collections are hashed by sorting child hashes alphabetically.
// Field order is alphabetical by JSON field name. Missing/null fields are skipped.
// Number format: integer if no fractional part, else shortest decimal representation.

// #region ­ƒöÀSHA-256
// ­ƒî┐Pure JS SHA-256 implementation for cross-platform compatibility (Node + browser).
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
// #endregion ­ƒöÀSHA-256

// #region ­ƒî®´©ÅHashWriter
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
  writeIdList(ids: string[]) {
    const sorted = [...ids].sort();
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
// #endregion ­ƒî®´©ÅHashWriter

/**
 * Formats a number deterministically for hashing.
 * Integer values (no fractional part) are formatted without decimal point.
 * Non-integer values use shortest decimal representation.
 **/
export const formatNumberForHash = (n: number): string => {
  if (Number.isInteger(n)) return n.toString();
  return n.toString();
};

// #region ­ƒÄÁHash Value Types
/**
 * Computes SHA-256 hash of a Coordinate value.
 **/
export const hashCoordinate = (c: Coordinate): string => {
  const w = new HashWriter();
  w.writeString("Coordinate");
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
// #endregion ­ƒÄÁHash Value Types

// #region ­ƒÄ®Hash Entities
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
  w.writeString("id");
  w.writeString(a.id);
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
  w.writeString("id");
  w.writeString(l.id);
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
  w.writeString("id");
  w.writeString(a.id);
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
    w.writeString(f.folder.id);
  }
  w.writeString("id");
  w.writeString(f.id);
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
  w.writeString("id");
  w.writeString(f.id);
  w.writeString("name");
  w.writeString(f.name);
  if (f.parent != null) {
    w.writeString("parent");
    w.writeString(f.parent.id);
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
  w.writeString("id");
  w.writeString(b.id);
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
  w.writeString("id");
  w.writeString(q.id);
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
    w.writeIdList(p.compatiblePorts.map((cp) => cp.id));
  }
  if (p.description != null) {
    w.writeString("description");
    w.writeString(p.description);
  }
  w.writeString("id");
  w.writeString(p.id);
  if (p.icon != null) {
    w.writeString("icon");
    w.writeString(p.icon);
  }
  w.writeString("name");
  w.writeString(p.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Family entity.
 **/
export const hashFamily = (f: Family): string => {
  const w = new HashWriter();
  w.writeString("Family");
  if (f.attributes && f.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(f.attributes.map(hashAttribute));
  }
  if (f.description != null) {
    w.writeString("description");
    w.writeString(f.description);
  }
  w.writeString("id");
  w.writeString(f.id);
  if (f.icon != null) {
    w.writeString("icon");
    w.writeString(f.icon);
  }
  w.writeString("name");
  w.writeString(f.name);
  if (f.ports && f.ports.length > 0) {
    w.writeString("ports");
    w.writeHashList(f.ports.map(hashPort));
  }
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
  w.writeString("id");
  w.writeString(p.id);
  w.writeString("quality");
  w.writeString(p.quality.id);
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
  w.writeString("id");
  w.writeString(t.id);
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
  w.writeString("id");
  w.writeString(c.id);
  if (c.icon != null) {
    w.writeString("icon");
    w.writeString(c.icon);
  }
  w.writeString("name");
  w.writeString(c.name);
  return w.digest();
};

/**
 * Computes SHA-256 hash of a Representation entity.
 **/
export const hashRepresentation = (m: Representation): string => {
  const w = new HashWriter();
  w.writeString("Representation");
  if (m.attributes && m.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(m.attributes.map(hashAttribute));
  }
  if (m.description != null) {
    w.writeString("description");
    w.writeString(m.description);
  }
  w.writeString("file");
  w.writeString(m.file.id);
  w.writeString("id");
  w.writeString(m.id);
  if (m.name != null) {
    w.writeString("name");
    w.writeString(m.name);
  }
  if (m.tags && m.tags.length > 0) {
    w.writeString("tags");
    w.writeIdList(m.tags.map((t) => t.id));
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
  w.writeString("id");
  w.writeString(c.id);
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
    w.writeString(c.port.id);
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
    w.writeIdList(t.authors.map((a) => a.id));
  }
  if (t.concepts && t.concepts.length > 0) {
    w.writeString("concepts");
    w.writeIdList(t.concepts.map((c) => c.id));
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
  w.writeString("id");
  w.writeString(t.id);
  if (t.lifecycle === "deleted") {
    w.writeString("lifecycle");
    w.writeString("deleted");
    if (t.deletedByUserId != null) {
      w.writeString("deletedByUserId");
      w.writeString(t.deletedByUserId);
    }
    if (t.deletedByDisplayName != null) {
      w.writeString("deletedByDisplayName");
      w.writeString(t.deletedByDisplayName);
    }
    if (t.deletedAt != null) {
      w.writeString("deletedAt");
      w.writeString(t.deletedAt);
    }
    if (t.deletedInChangeId != null) {
      w.writeString("deletedInChangeId");
      w.writeString(t.deletedInChangeId);
    }
  }
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
    w.writeString(t.location.id);
  }
  if (t.representations && t.representations.length > 0) {
    w.writeString("representations");
    w.writeHashList(t.representations.map(hashRepresentation));
  }
  w.writeString("name");
  w.writeString(t.name);
  if (t.families && t.families.length > 0) {
    w.writeString("families");
    w.writeIdList(t.families.map((f) => f.id));
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
  w.writeString("id");
  w.writeString(l.id);
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
  w.writeString("id");
  w.writeString(s.id);
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
  w.writeString(s.quality.id);
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
  w.writeString("id");
  w.writeString(g.id);
  if (g.name != null) {
    w.writeString("name");
    w.writeString(g.name);
  }
  w.writeString("pieces");
  w.writeIdList(g.pieces.map((p) => p.id));
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
    w.writeString(s.connector.id);
  }
  if (s.designPiece != null) {
    w.writeString("designPiece");
    w.writeString(s.designPiece.id);
  }
  w.writeString("piece");
  w.writeString(s.piece.id);
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
  w.writeString("id");
  w.writeString(c.id);
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
    w.writeHash(hashCoordinate(p.center));
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
    w.writeString(p.design.id);
  }
  w.writeString("id");
  w.writeString(p.id);
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
    w.writeString(p.type.id);
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
    w.writeString(d.activeLayer.id);
  }
  if (d.attributes && d.attributes.length > 0) {
    w.writeString("attributes");
    w.writeHashList(d.attributes.map(hashAttribute));
  }
  if (d.authors && d.authors.length > 0) {
    w.writeString("authors");
    w.writeIdList(d.authors.map((a) => a.id));
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
    w.writeIdList(d.concepts.map((c) => c.id));
  }
  if (d._connections && d._connections.length > 0) {
    w.writeString("connections");
    w.writeHashList(d._connections.map(hashConnection));
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
  w.writeString("id");
  w.writeString(d.id);
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
    w.writeString(d.location.id);
  }
  w.writeString("name");
  w.writeString(d.name);
  if (d.families && d.families.length > 0) {
    w.writeString("families");
    w.writeIdList(d.families.map((f) => f.id));
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
 * Computes SHA-256 Merkle hash of a KitImpl entity.
 * Calls hashDesign, hashType, etc. for all children.
 **/
export const hashKit = (k: KitImpl): string => {
  const w = new HashWriter();
  w.writeString("KitImpl");
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
  w.writeString("id");
  w.writeString(k.id);
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
  if (k.families && k.families.length > 0) {
    w.writeString("families");
    w.writeHashList(k.families.map(hashFamily));
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
// #endregion ­ƒÄ®Hash Entities

// #region ­ƒöùHash Diffs
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

const writeNullableId = (w: HashWriter, key: string, val: { id: string } | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeString(val.id);
};

const writeNullableIdArray = (w: HashWriter, key: string, val: { id: string }[] | null | undefined) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeIdList(val.map((v) => v.id));
};

const writeNullableHash = (w: HashWriter, key: string, val: any, hashFn: (v: any) => string) => {
  if (val === undefined) return;
  w.writeString(key);
  if (val === null) w.writeBool(false);
  else w.writeHash(hashFn(val));
};

const hashCollectionDiffGeneric = (tag: string, updateTag: string, entityKeyName: string, hashEntityFn: (e: any) => string, hashDiffFn: (d: any) => string, diff: { removed?: { id: string }[]; updated?: any[]; added?: any[] }): string => {
  const w = new HashWriter();
  w.writeString(tag);
  if (diff.added && diff.added.length > 0) {
    w.writeString("added");
    w.writeHashList(diff.added.map(hashEntityFn));
  }
  if (diff.removed && diff.removed.length > 0) {
    w.writeString("removed");
    w.writeIdList(diff.removed.map((r) => r.id));
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
          uw.writeString(u[k].id);
        }
      }
      return uw.digest();
    });
    w.writeHashList(updateHashes);
  }
  return w.digest();
};

// #region ­ƒÉ╣Hash Diff Value Types

export const hashCoordinateDiff = (d: CoordinateDiff): string => {
  const w = new HashWriter();
  w.writeString("CoordinateDiff");
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

// #endregion ­ƒÉ╣Hash Diff Value Types

// #region ÔÜù´©ÅHash Diff Entities

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

export const hashFamilyDiff = (d: FamilyDiff): string => {
  const w = new HashWriter();
  w.writeString("FamilyDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "name", d.name);
  writeNullableHash(w, "ports", d.ports, hashPortsDiff);
  return w.digest();
};

export const hashFamiliesDiff = (d: FamiliesDiff): string => hashCollectionDiffGeneric("FamiliesDiff", "FamilyDiffUpdate", "family", hashFamily, hashFamilyDiff, d);

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

export const hashRepresentationDiff = (d: RepresentationDiff): string => {
  const w = new HashWriter();
  w.writeString("RepresentationDiff");
  writeNullableHash(w, "attributes", d.attributes, hashAttributesDiff);
  writeNullableString(w, "description", d.description);
  writeNullableId(w, "file", d.file);
  writeNullableString(w, "name", d.name);
  writeNullableIdArray(w, "tags", d.tags);
  return w.digest();
};

export const hashRepresentationsDiff = (d: RepresentationsDiff): string => hashCollectionDiffGeneric("RepresentationsDiff", "RepresentationDiffUpdate", "representation", hashRepresentation, hashRepresentationDiff, d);

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
  writeNullableHash(w, "representations", d.representations, hashRepresentationsDiff);
  writeNullableString(w, "name", d.name);
  if (d.families !== undefined) {
    w.writeString("families");
    if (d.families === null) {
      w.writeString("null");
    } else {
      w.writeIdList(d.families.map((f) => f.id));
    }
  }
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
  writeNullableHash(w, "center", d.center, hashCoordinate);
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
  writeNullableHash(w, "connections", d._connections, hashConnectionsDiff);
  writeNullableString(w, "description", d.description);
  writeNullableString(w, "folder", d.folder);
  writeNullableHash(w, "groups", d.groups, hashGroupsDiff);
  writeNullableString(w, "icon", d.icon);
  writeNullableString(w, "image", d.image);
  writeNullableBool(w, "isAbstract", d.isAbstract);
  writeNullableHash(w, "layers", d.layers, hashLayersDiff);
  writeNullableId(w, "location", d.location);
  writeNullableString(w, "name", d.name);
  if (d.families !== undefined) {
    w.writeString("families");
    if (d.families === null) {
      w.writeString("null");
    } else {
      w.writeIdList(d.families.map((f) => f.id));
    }
  }
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
  writeNullableHash(w, "families", d.families, hashFamiliesDiff);
  writeNullableString(w, "preview", d.preview);
  writeNullableHash(w, "qualities", d.qualities, hashQualitiesDiff);
  writeNullableString(w, "remote", d.remote);
  writeNullableHash(w, "tags", d.tags, hashTagsDiff);
  writeNullableHash(w, "types", d.types, hashTypesDiff);
  writeNullableString(w, "version", d.version);
  return w.digest();
};

// #endregion ÔÜù´©ÅHash Diff Entities

// #endregion ­ƒöùHash Diffs

// #endregion ­ƒûÑ´©ÅHash

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
  added: z.array(z.any()).optional(),
});

// KitImpl graph mutations and entity resolution: use `kit.ops.*` (see {@link KitOps}) or entity methods such as {@link Type.delete}.

/**
 * Equality check for KitImpl values.
 **/
export const areSameKit = (kitId: string, otherId: string): boolean => {
  return kitId === otherId;
};
/**
 * Checks whether SameKit condition holds.
 **/
export const hasSameKit = (kitId: string, otherIds: string[]): boolean => otherIds.some((other) => areSameKit(kitId, other));

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
 * When designId is set, first performs transitive design-scoped filtering.
 * Glob filters on each entity kind are applied afterwards (or directly if no designId).
 **/
export type KitFilter = {
  designId?: string;
  representationTags?: string[];
  designs?: GlobFilter;
  types?: GlobFilter;
  families?: GlobFilter;
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
const filterKitByDesign = (kit: KitLike, designId: string, representationTags?: string[]): KitData => {
  const k = asKitInstance(kit);
  const design = k.requireDesign(designId);

  const usedTypeIds = new Set<string>();
  const usedDesignIds = new Set<string>([designId]);
  for (const piece of design.pieces ?? []) {
    if (piece.type?.id) usedTypeIds.add(piece.type.id);
    if (piece.design?.id) usedDesignIds.add(piece.design.id);
  }

  const typeById = new Map((k.types ?? []).map((type) => [type.id, type]));
  const collectFamilyTypes = (typeId: string) => {
    const type = typeById.get(typeId);
    if (!type) return;
    const families = type.families ?? [];
    if (families.length === 0) return;
    for (const [id, t] of typeById) {
      if (usedTypeIds.has(id)) continue;
      const tf = t.families ?? [];
      if (tf.some((f) => families.some((fam) => fam.id === f.id))) {
        usedTypeIds.add(id);
      }
    }
  };
  for (const typeId of [...usedTypeIds]) collectFamilyTypes(typeId);

  const tags = representationTags;
  const resolvedTagIds = (tags ?? []).flatMap((tagValue) => {
    const byId = (k.tags ?? []).find((tag) => tag.id === tagValue);
    if (byId) return [byId.id];
    return (k.tags ?? []).filter((tag) => tag.name === tagValue).map((tag) => tag.id);
  });

  const usedFamilyIds = new Set<string>();
  const usedFileIds = new Set<string>();
  const usedTagIds = new Set<string>();
  const usedConceptIds = new Set<string>();
  const usedQualityIds = new Set<string>();
  const usedAuthorIds = new Set<string>();
  const usedFolderNames = new Set<string>();
  const selectedRepresentations = new Map<string, Representation>();

  const collectQualityFromProps = (props?: Array<{ quality?: { id: string } }>) => {
    for (const prop of props ?? []) {
      if (prop.quality?.id) usedQualityIds.add(prop.quality.id);
    }
  };

  for (const typeId of usedTypeIds) {
    const type = typeById.get(typeId);
    if (!type) continue;
    if (type.folder) usedFolderNames.add(type.folder);
    for (const connector of type.connectors ?? []) {
      if (connector.port?.id) {
        const family = (k.families ?? []).find((f) => (f.ports ?? []).some((p) => p.id === connector.port!.id));
        if (family) usedFamilyIds.add(family.id);
      }
      collectQualityFromProps(connector.props);
    }
    collectQualityFromProps(type.props);
    for (const author of type.authors ?? []) if (author.id) usedAuthorIds.add(author.id);
    for (const concept of type.concepts ?? []) if (concept.id) usedConceptIds.add(concept.id);
    const selectedRepresentation = selectBestRepresentation(type.representations ?? [], resolvedTagIds);
    if (selectedRepresentation) {
      selectedRepresentations.set(typeId, selectedRepresentation);
      if (selectedRepresentation.file?.id) usedFileIds.add(selectedRepresentation.file.id);
      for (const tag of selectedRepresentation.tags ?? []) if (tag.id) usedTagIds.add(tag.id);
    }
  }

  for (const piece of design.pieces ?? []) collectQualityFromProps(piece.props);
  for (const concept of design.concepts ?? []) if (concept.id) usedConceptIds.add(concept.id);
  for (const author of design.authors ?? []) if (author.id) usedAuthorIds.add(author.id);
  for (const familyId of [...usedFamilyIds]) {
    const family = (k.families ?? []).find((f) => f.id === familyId);
    if (!family) continue;
    for (const port of family.ports ?? []) {
      for (const compatible of port.compatiblePorts ?? []) {
        if (compatible.id) {
          const compatFamily = (k.families ?? []).find((f) => (f.ports ?? []).some((p) => p.id === compatible.id));
          if (compatFamily) usedFamilyIds.add(compatFamily.id);
        }
      }
    }
  }
  for (const tagId of resolvedTagIds) usedTagIds.add(tagId);

  return {
    id: k.id,
    name: k.name,
    version: k.version,
    description: k.description,
    icon: k.icon,
    image: k.image,
    preview: k.preview,
    remote: k.remote,
    homepage: k.homepage,
    license: k.license,
    types: (k.types ?? [])
      .filter((type) => usedTypeIds.has(type.id))
      .map((type) => ({
        ...type,
        representations: selectedRepresentations.has(type.id) ? [selectedRepresentations.get(type.id)!] : [],
      })),
    designs: (k.designs ?? []).filter((candidate) => usedDesignIds.has(candidate.id)).map((d) => d.toPlain()),
    families: (k.families ?? []).filter((family) => usedFamilyIds.has(family.id)),
    files: (k.files ?? []).filter((file) => usedFileIds.has(file.id)),
    tags: (k.tags ?? []).filter((tag) => usedTagIds.has(tag.id)),
    concepts: (k.concepts ?? []).filter((concept) => usedConceptIds.has(concept.id)),
    qualities: (k.qualities ?? []).filter((quality) => usedQualityIds.has(quality.id)),
    folders: (k.folders ?? []).filter((folder) => usedFolderNames.has(folder.name)),
    authors: (k.authors ?? []).filter((author) => usedAuthorIds.has(author.id)),
    attributes: k.attributes,
    createdAt: k.createdAt,
    updatedAt: k.updatedAt,
  };
};

/**
 * General-purpose kit filter. Combines optional design-based transitive filtering with glob-based name filtering.
 * When designId is set, first performs transitive design-scoped subset extraction.
 * Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
 **/
export const filterKit = (kit: KitLike, filter: KitFilter): KitImpl => asKitInstance(kit).filter(filter);

// #region ­ƒô╗Design Family Helpers
// Design family traversal helpers MUST be defined here.

/**
 * ­ƒô╗ Retrieves the DesignFamily: all designs sharing any family name with the given design.
 **/
export const getDesignFamily = (kit: KitLike, designId: string): Design[] => asKitInstance(kit).getDesignFamilyFor(designId);

/**
 * ­ƒô╗ Checks if Designs belong to the same family (share any family name).
 **/
export const areDesignsInSameFamily = (kit: KitLike, designIdA: string, designIdB: string): boolean => asKitInstance(kit).areDesignsInSameFamily(designIdA, designIdB);

/**
 * Checks if UseDesignAsPiece action is possible.
 **/
export const canUseDesignAsPiece = (kit: KitLike, containerDesignId: string, pieceDesignId: string): boolean => asKitInstance(kit).canUseDesignAsPiece(containerDesignId, pieceDesignId);

/**
 * Searches for matching SameFamilyDesignPieces entry.
 **/
export const findSameFamilyDesignPieces = (kit: KitLike, designId: string): Piece[] => asKitInstance(kit).findSameFamilyDesignPiecesIn(designId);

// #endregion ­ƒô╗Design Family Helpers

// #region ­ƒºèType Family Helpers
// Type family traversal helpers MUST be defined here.

/**
 * ­ƒºè Retrieves the TypeFamily: all types sharing any family name with the given type.
 **/
export const getTypeFamily = (kit: KitLike, typeId: string): Type[] => asKitInstance(kit).getTypeFamilyFor(typeId);

/**
 * ­ƒæ¿ÔÇì­ƒæ®ÔÇì­ƒæºÔÇì­ƒæª Checks if Types belong to the same family (share any family name).
 **/
export const areTypesInSameFamily = (kit: KitLike, typeIdA: string, typeIdB: string): boolean => asKitInstance(kit).areTypesInSameFamily(typeIdA, typeIdB);

// #endregion ­ƒºèType Family Helpers

// #region ­ƒÄ»SemioReport
/**
 * Human-readable note attached to a {@link SemioReport} (warning, info, or error).
 **/
export interface OperationNote {
  /** Stable machine id e.g. flatten.no-fixed-piece-in-clump */
  code?: string;
  message: string;
}

/**
 * ­ƒôïCanonical semio algorithm output: always exposes diff, warnings, infos, and errors (tool-friendly).
 * When `ok` is true, `diff` is non-null; when false, `diff` is null and `errors` is non-empty.
 **/
export interface SemioReport<TDiff = unknown> {
  ok: boolean;
  diff: TDiff | null;
  warnings: OperationNote[];
  infos: OperationNote[];
  errors: OperationNote[];
}

/** @deprecated Use {@link SemioReport}; kept as alias for existing type names. */
export type OperationResult<TDiff> = SemioReport<TDiff>;

export type DesignOperationResult = SemioReport<DesignChange>;
export type DesignDiffOperationResult = SemioReport<DesignDiff>;

/**
 * Successful report: `diff` set, `errors` empty.
 **/
export const operationOk = <TDiff>(diff: TDiff, warnings: OperationNote[] = [], infos: OperationNote[] = []): SemioReport<TDiff> => ({
  ok: true,
  diff,
  warnings,
  infos,
  errors: [],
});

/**
 * Failed report: `diff` null, `errors` populated; warnings/infos empty unless caller merges.
 **/
export const operationErr = <TDiff = unknown>(errors: OperationNote[]): SemioReport<TDiff> => ({
  ok: false,
  diff: null,
  warnings: [],
  infos: [],
  errors,
});

/**
 * Wraps a native/REST payload into {@link DesignOperationResult} (accepts `diff` or alternate `change` for the design payload).
 **/
export const normalizeDesignFlattenResult = (raw: unknown): DesignOperationResult => {
  if (raw !== null && typeof raw === "object" && "ok" in raw) {
    const o = raw as Record<string, unknown>;
    if ("diff" in o) return raw as DesignOperationResult;
    if ("change" in o) {
      return {
        ok: !!o.ok,
        diff: (o.change ?? null) as DesignChange | null,
        warnings: (o.warnings as OperationNote[]) ?? [],
        infos: (o.infos as OperationNote[]) ?? [],
        errors: (o.errors as OperationNote[]) ?? [],
      };
    }
    if (o.ok === false && "errors" in o) {
      return operationErr<DesignChange>(o.errors as OperationNote[]);
    }
  }
  return operationOk(raw as DesignChange, [], []);
};

/**
 * Wraps a native/REST payload into {@link DesignDiffOperationResult}.
 **/
export const normalizeDesignDiffResult = (raw: unknown): DesignDiffOperationResult => {
  if (raw !== null && typeof raw === "object" && "ok" in raw) {
    const o = raw as Record<string, unknown>;
    if ("diff" in o) return raw as DesignDiffOperationResult;
    if ("change" in o) {
      return {
        ok: !!o.ok,
        diff: (o.change ?? null) as DesignDiff | null,
        warnings: (o.warnings as OperationNote[]) ?? [],
        infos: (o.infos as OperationNote[]) ?? [],
        errors: (o.errors as OperationNote[]) ?? [],
      };
    }
    if (o.ok === false && "errors" in o) {
      return operationErr<DesignDiff>(o.errors as OperationNote[]);
    }
  }
  return operationOk(raw as DesignDiff, [], []);
};

/**
 * Wraps a native/REST payload into {@link SemioReport}<{@link Design}>.
 **/
export const normalizeDesignCopyResult = (raw: unknown): SemioReport<Design> => {
  if (raw !== null && typeof raw === "object" && "ok" in raw) {
    const o = raw as Record<string, unknown>;
    if ("diff" in o) return raw as SemioReport<Design>;
    if ("change" in o) {
      return {
        ok: !!o.ok,
        diff: (o.change ?? null) as Design | null,
        warnings: (o.warnings as OperationNote[]) ?? [],
        infos: (o.infos as OperationNote[]) ?? [],
        errors: (o.errors as OperationNote[]) ?? [],
      };
    }
    if (o.ok === false && "errors" in o) {
      return operationErr<Design>(o.errors as OperationNote[]);
    }
  }
  return operationOk(raw as Design, [], []);
};
// #endregion ­ƒÄ»SemioReport

/** One undo step, or a transaction batch (undo applies `backward` in reverse order). */
export type KitUndoEntry = KitGraphChange | { batch: KitGraphChange[] };

/**
 * Computes the forward and backward diffs between two kit states.
 **/
/**
 * Represents a reversible design change with forward and backward diffs.
 **/
export interface DesignChange {
  forward: DesignDiff;
  backward: DesignDiff;
}

// #region ­ƒôªKitImpl Diff Validation
// Validates kit diffs before apply; optional heal trims ineffective operations.

/**
 * Outcome of {@link validateKitGraphDiff}: errors block faithful apply, warnings flag suspicious but applicable diffs.
 **/
export interface KitDiffValidationResult {
  ok: boolean;
  errors: OperationNote[];
  warnings: OperationNote[];
  infos: OperationNote[];
  /** When `heal` was true, the same diff reference after in-place healing. */
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
type IdCollDiff = {
  removed?: Array<{ id: string }>;
  updated?: any[];
  added?: any[];
};

const collGetUpdatedId = (u: any, idKey: string): string => u?.[idKey]?.id ?? "";

const validateIdCollectionDiff = <TItem extends { id: string }>(
  ctx: KitDiffValidationCtx,
  path: string,
  idKey: string,
  base: TItem[],
  raw: IdCollDiff | undefined,
  onUpdated: (item: TItem, itemDiff: any, itemPath: string) => void,
): IdCollDiff | undefined => {
  if (!raw) return undefined;
  const baseById = new Map(base.map((i) => [i.id, i]));
  const removedIds = new Set((raw.removed ?? []).map((r) => r.id));
  let healedRemoved = raw.removed ? [...raw.removed] : undefined;
  let healedUpdated = raw.updated ? [...raw.updated] : undefined;
  let healedAdded = raw.added ? [...raw.added] : undefined;

  const afterRemoveIds = new Set(base.filter((i) => !removedIds.has(i.id)).map((i) => i.id));

  for (const r of raw.removed ?? []) {
    if (!baseById.has(r.id)) {
      kitDiffPush(ctx, "warnings", "kitdiff.remove.missing-target", `${path}: remove references missing ${idKey} ${r.id}`);
      if (ctx.heal && healedRemoved) healedRemoved = healedRemoved.filter((x) => x.id !== r.id);
    }
  }

  const noopAddedById = new Map<string, { id: string }>();
  for (const a of raw.added ?? []) noopAddedById.set(a.id, a);

  const jsonNormForDiffCompare = (x: unknown) => JSON.parse(JSON.stringify(x), (_k, v) => (v === null ? undefined : v));
  for (const r of raw.removed ?? []) {
    const orig = baseById.get(r.id);
    const add = noopAddedById.get(r.id);
    if (orig && add && deepEqual(jsonNormForDiffCompare(orig), jsonNormForDiffCompare(add))) {
      kitDiffPush(ctx, "warnings", "kitdiff.cycle.noop-restore", `${path}: removed and re-added ${idKey} ${r.id} are deeply equal (no effective change)`);
      if (ctx.heal) {
        if (healedRemoved) healedRemoved = healedRemoved.filter((x) => x.id !== r.id);
        if (healedAdded) healedAdded = healedAdded.filter((x) => x.id !== r.id);
      }
    }
  }

  const seenAdd = new Set<string>();
  for (const a of raw.added ?? []) {
    if (seenAdd.has(a.id)) {
      kitDiffPush(ctx, "errors", "kitdiff.add.duplicate-in-diff", `${path}: duplicate added ${idKey} id ${a.id}`);
      if (ctx.heal && healedAdded) {
        const first = healedAdded.findIndex((x) => x.id === a.id);
        healedAdded = healedAdded.filter((x, i) => x.id !== a.id || i === first);
      }
    }
    seenAdd.add(a.id);
    if (afterRemoveIds.has(a.id)) {
      kitDiffPush(ctx, "errors", "kitdiff.add.duplicate-id", `${path}: cannot add ${idKey} ${a.id} that still exists after removes`);
      if (ctx.heal && healedAdded) healedAdded = healedAdded.filter((x) => x.id !== a.id);
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
    const item = baseById.get(gid);
    if (!item) {
      kitDiffPush(ctx, "errors", "kitdiff.update.missing-base", `${p}: ${idKey} not found in base kit`);
      if (ctx.heal && healedUpdated) healedUpdated = healedUpdated.filter((x) => collGetUpdatedId(x, idKey) !== gid);
      continue;
    }
    onUpdated(item, u.diff, p);
  }

  if (!ctx.heal) return raw;
  const out: IdCollDiff = {};
  if (healedRemoved && healedRemoved.length > 0) out.removed = healedRemoved;
  if (healedUpdated && healedUpdated.length > 0) out.updated = healedUpdated;
  if (healedAdded && healedAdded.length > 0) out.added = healedAdded;
  return Object.keys(out).length > 0 ? out : undefined;
};

const validateAttributesDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Attribute[], d: AttributesDiff | undefined): void => {
  validateIdCollectionDiff(ctx, path, "attribute", base, d, (_item, _diff, _p) => { });
};

const validatePropsDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Prop[], qualities: Set<string>, d: PropsDiff | undefined): void => {
  validateIdCollectionDiff(ctx, path, "prop", base, d, (item, diff, p) => {
    const q = (diff as PropDiff).quality?.id ?? item.quality?.id;
    if (q && !qualities.has(q)) kitDiffPush(ctx, "errors", "kitdiff.ref.quality-missing", `${p}: quality ${q} not in kit`);
    if ((diff as PropDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (diff as PropDiff).attributes);
  });
};

const validateRepresentationDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Representation[], files: Set<string>, d: RepresentationsDiff | undefined): void => {
  validateIdCollectionDiff(ctx, path, "representation", base, d, (item, diff, p) => {
    const fid = (diff as RepresentationDiff).file?.id ?? item.file?.id;
    if (fid && !files.has(fid)) kitDiffPush(ctx, "errors", "kitdiff.ref.file-missing", `${p}: representation file ${fid} not in kit`);
    if ((diff as RepresentationDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (diff as RepresentationDiff).attributes);
  });
};

const validateConnectorDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Connector[], ports: Set<string>, qualities: Set<string>, d: ConnectorsDiff | undefined): void => {
  validateIdCollectionDiff(ctx, path, "connector", base, d, (item, diff, p) => {
    const pg = (diff as ConnectorDiff).port?.id ?? item.port?.id;
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
  ctxRefs: { typeIds: Set<string>; fileIds: Set<string>; portIds: Set<string>; conceptIds: Set<string>; authorIds: Set<string>; qualityIds: Set<string> },
): void => {
  if (diff.representations) validateRepresentationDiffNested(ctx, `${path}.representations`, item.representations ?? [], ctxRefs.fileIds, diff.representations);
  if (diff.connectors) validateConnectorDiffNested(ctx, `${path}.connectors`, item.connectors ?? [], ctxRefs.portIds, ctxRefs.qualityIds, diff.connectors);
  if (diff.props) validatePropsDiffNested(ctx, `${path}.props`, item.props ?? [], ctxRefs.qualityIds, diff.props);
  if (diff.attributes) validateAttributesDiffNested(ctx, `${path}.attributes`, item.attributes ?? [], diff.attributes);
  if (diff.concepts) {
    for (const c of diff.concepts ?? []) {
      if (c?.id && !ctxRefs.conceptIds.has(c.id)) kitDiffPush(ctx, "errors", "kitdiff.ref.concept-missing", `${path}: concept ${c.id} not in kit`);
    }
  }
  if (diff.authors) {
    for (const a of diff.authors ?? []) {
      if (a?.id && !ctxRefs.authorIds.has(a.id)) kitDiffPush(ctx, "errors", "kitdiff.ref.author-missing", `${path}: author ${a.id} not in kit`);
    }
  }
};

const validateBenchmarksDiffNested = (ctx: KitDiffValidationCtx, path: string, base: Benchmark[], d: BenchmarksDiff | undefined): void => {
  validateIdCollectionDiff(ctx, path, "benchmark", base, d, (_item, diff, p) => {
    if ((diff as BenchmarkDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, _item.attributes ?? [], (diff as BenchmarkDiff).attributes);
  });
};

const validateQualityDiffNested = (ctx: KitDiffValidationCtx, path: string, item: Quality, diff: QualityDiff): void => {
  if (diff.benchmarks) validateBenchmarksDiffNested(ctx, `${path}.benchmarks`, item.benchmarks ?? [], diff.benchmarks);
  if (diff.attributes) validateAttributesDiffNested(ctx, `${path}.attributes`, item.attributes ?? [], diff.attributes);
};

/** Piece IDs after applying a pieces diff, without cloning piece objects. */
const simulatePieceIdSetForDesign = (base: Design, d?: PiecesDiff): Set<string> => {
  const ids = new Set((base.pieces ?? []).map((p) => p.id));
  if (!d) return ids;
  for (const r of d.removed ?? []) ids.delete(r.id);
  for (const a of d.added ?? []) ids.add(a.id);
  return ids;
};

/** Layer IDs after applying a layers diff, without cloning layer objects. */
const simulateLayerIdSetForDesign = (base: Design, d?: LayersDiff): Set<string> => {
  const ids = new Set((base.layers ?? []).map((l) => l.id));
  if (!d) return ids;
  for (const r of d.removed ?? []) ids.delete(r.id);
  for (const a of d.added ?? []) ids.add(a.id);
  return ids;
};

const previewConnectionSidesAfterDiff = (conn: Connection, d: ConnectionDiff, hostDesign: Design): { connected: Side; connecting: Side } => {
  const connected = new Side(conn.connected.toPlain(), hostDesign);
  const connecting = new Side(conn.connecting.toPlain(), hostDesign);
  if (d.connected) applySideDiff(connected, d.connected);
  if (d.connecting) applySideDiff(connecting, d.connecting);
  return { connected, connecting };
};

const validateDesignDiffNested = (
  ctx: KitDiffValidationCtx,
  kit: KitImpl,
  path: string,
  design: Design,
  diff: DesignDiff,
  refs: { typeIds: Set<string>; designIds: Set<string>; qualityIds: Set<string>; fileIds: Set<string>; portIds: Set<string>; conceptIds: Set<string>; authorIds: Set<string> },
): void => {
  if (diff.concepts) {
    for (const c of diff.concepts ?? []) {
      if (c?.id && !refs.conceptIds.has(c.id)) kitDiffPush(ctx, "errors", "kitdiff.ref.concept-missing", `${path}: concept ${c.id} not in kit`);
    }
  }
  if (diff.authors !== undefined) {
    const da = diff.authors as unknown;
    if (Array.isArray(da)) {
      for (const a of da as Array<{ id?: string }>) {
        if (a?.id && !refs.authorIds.has(a.id)) kitDiffPush(ctx, "errors", "kitdiff.ref.author-missing", `${path}: author ${a.id} not in kit`);
      }
    } else if (da !== null && typeof da === "object") {
      validateIdCollectionDiff(ctx, `${path}.authors`, "author", kit.authors ?? [], da as IdCollDiff, (item, adiff, p) => {
        if ((adiff as AuthorDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (adiff as AuthorDiff).attributes);
      });
    }
  }

  if (diff.pieces) {
    validateIdCollectionDiff(ctx, `${path}.pieces`, "piece", design.pieces ?? [], diff.pieces, (item, pDiff, p) => {
      if ((pDiff as PieceDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (pDiff as PieceDiff).attributes);
      if ((pDiff as PieceDiff).props) validatePropsDiffNested(ctx, `${p}.props`, item.props ?? [], refs.qualityIds, (pDiff as PieceDiff).props);
    });
    for (const a of diff.pieces.added ?? []) {
      const tg = a.type?.id;
      if (tg && !refs.typeIds.has(tg)) kitDiffPush(ctx, "errors", "kitdiff.ref.piece-type-missing", `${path}.pieces.added: type ${tg} not in kit`);
      const dg = a.design?.id;
      if (dg && !refs.designIds.has(dg)) kitDiffPush(ctx, "errors", "kitdiff.ref.piece-design-missing", `${path}.pieces.added: subdesign ${dg} not in kit`);
    }
  }

  const pieceIds = simulatePieceIdSetForDesign(design, diff.pieces);

  if (diff.connections) {
    validateIdCollectionDiff(ctx, `${path}.connections`, "connection", design._connections ?? [], diff.connections, (item, cDiff, p) => {
      if ((cDiff as ConnectionDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (cDiff as ConnectionDiff).attributes);
    });
    const checkSide = (side: Side | SidePlain, label: string, cpath: string) => {
      const pieceId = side instanceof Side ? side.wirePieceId().id : side.piece.id;
      const designPieceId = side instanceof Side ? side.designPiece?.id : side.designPiece?.id;
      if (!pieceIds.has(pieceId)) kitDiffPush(ctx, "errors", "kitdiff.ref.connection-piece-missing", `${cpath}: ${label} piece ${pieceId} not in design after piece diff`);
      if (designPieceId && !pieceIds.has(designPieceId)) kitDiffPush(ctx, "errors", "kitdiff.ref.connection-designpiece-missing", `${cpath}: ${label} designPiece ${designPieceId} not in design after piece diff`);
    };
    for (const a of diff.connections.added ?? []) {
      const cp = `${path}.connections.added[${a.id}]`;
      checkSide(a.connected, "connected", cp);
      checkSide(a.connecting, "connecting", cp);
    }
    for (const u of diff.connections.updated ?? []) {
      const conn = design._connections?.find((c) => c.id === (u as any).connection.id);
      const cp = `${path}.connections.updated[${(u as any).connection.id}]`;
      if (conn) {
        const { connected, connecting } = previewConnectionSidesAfterDiff(conn, u.diff as ConnectionDiff, design);
        checkSide(connected, "connected", cp);
        checkSide(connecting, "connecting", cp);
      }
    }
  }

  if (diff.stats) {
    validateIdCollectionDiff(ctx, `${path}.stats`, "stat", design.stats ?? [], diff.stats, (item, sdiff, p) => {
      const q = (sdiff as StatDiff).quality?.id ?? item.quality?.id;
      if (q && !refs.qualityIds.has(q)) kitDiffPush(ctx, "errors", "kitdiff.ref.quality-missing", `${p}: stat quality ${q} not in kit`);
    });
  }
  if (diff.props) validatePropsDiffNested(ctx, `${path}.props`, design.props ?? [], refs.qualityIds, diff.props);

  if (diff.layers) {
    validateIdCollectionDiff(ctx, `${path}.layers`, "layer", design.layers ?? [], diff.layers, (item, ldiff, p) => {
      if ((ldiff as LayerDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (ldiff as LayerDiff).attributes);
    });
  }
  const layerIds = simulateLayerIdSetForDesign(design, diff.layers);
  const active = diff.activeLayer ?? design.activeLayer;
  if (active?.id && !layerIds.has(active.id)) kitDiffPush(ctx, "errors", "kitdiff.ref.active-layer-missing", `${path}: activeLayer ${active.id} not in layers after diff`);

  if (diff.groups) {
    validateIdCollectionDiff(ctx, `${path}.groups`, "group", design.groups ?? [], diff.groups, (item, gdiff, p) => {
      if ((gdiff as GroupDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (gdiff as GroupDiff).attributes);
    });
    const checkGroupPieces = (g: Group, gp: string) => {
      for (const pid of g.pieces ?? []) {
        if (!pieceIds.has(pid.id)) kitDiffPush(ctx, "errors", "kitdiff.ref.group-piece-missing", `${gp}: piece ${pid.id} not in design`);
      }
    };
    for (const a of diff.groups.added ?? []) checkGroupPieces(a, `${path}.groups.added[${a.id}]`);
    for (const u of diff.groups.updated ?? []) {
      const g = design.groups?.find((x) => x.id === (u as any).group.id);
      if (g) {
        const gd = u.diff as GroupDiff;
        const virtual: Group = { ...g, pieces: gd.pieces !== undefined ? gd.pieces : g.pieces };
        checkGroupPieces(virtual, `${path}.groups.updated[${(u as any).group.id}]`);
      }
    }
  }

  if (diff.attributes) validateAttributesDiffNested(ctx, `${path}.attributes`, design.attributes ?? [], diff.attributes);
};

/**
 * Validates a {@link KitDiff} against a base {@link KitImpl}. Errors mean apply would skip or mis-apply operations; warnings flag redundant or suspicious edits.
 * With `heal`, trims invalid operations on the provided `diff` **in place** (same object reference) and returns it in `result.diff`.
 **/
function validateKitGraphDiff(kit: KitImpl, diff: KitDiff, heal: boolean): KitDiffValidationResult {
  const working: KitDiff = diff;
  const ctx: KitDiffValidationCtx = { errors: [], warnings: [], heal, diff: working };

  const typeIds = new Set((kit.types ?? []).map((t) => t.id));
  const designIds = new Set((kit.designs ?? []).map((d) => d.id));
  const qualityIds = new Set((kit.qualities ?? []).map((q) => q.id));
  const fileIds = new Set((kit.files ?? []).map((f) => f.id));
  const portIds = new Set((kit.families ?? []).flatMap((f) => f.ports ?? []).map((p) => p.id));
  const familyIds = new Set((kit.families ?? []).map((f) => f.id));
  const conceptIds = new Set((kit.concepts ?? []).map((c) => c.id));
  const authorIds = new Set((kit.authors ?? []).map((a) => a.id));
  const refs = { typeIds, designIds, qualityIds, fileIds, portIds, conceptIds, authorIds };

  if (ctx.diff.types) {
    ctx.diff.types = validateIdCollectionDiff(ctx, "types", "type", kit.types ?? [], ctx.diff.types, (item, tdiff, p) => validateTypeDiffNested(ctx, p, item, tdiff as TypeDiff, refs));
  }
  if (ctx.diff.designs) {
    ctx.diff.designs = validateIdCollectionDiff(ctx, "designs", "design", kit.designs ?? [], ctx.diff.designs, (item, ddiff, p) => validateDesignDiffNested(ctx, kit, p, item, ddiff as DesignDiff, refs));
  }
  if (ctx.diff.tags) ctx.diff.tags = validateIdCollectionDiff(ctx, "tags", "tag", kit.tags ?? [], ctx.diff.tags, () => { });
  if (ctx.diff.concepts) ctx.diff.concepts = validateIdCollectionDiff(ctx, "concepts", "concept", kit.concepts ?? [], ctx.diff.concepts, () => { });
  if (ctx.diff.families) ctx.diff.families = validateIdCollectionDiff(ctx, "families", "family", kit.families ?? [], ctx.diff.families, () => { });
  if (ctx.diff.qualities) {
    ctx.diff.qualities = validateIdCollectionDiff(ctx, "qualities", "quality", kit.qualities ?? [], ctx.diff.qualities, (item, qdiff, p) => validateQualityDiffNested(ctx, p, item, qdiff as QualityDiff));
  }
  if (ctx.diff.files) ctx.diff.files = validateIdCollectionDiff(ctx, "files", "file", kit.files ?? [], ctx.diff.files, () => { });
  if (ctx.diff.folders) {
    ctx.diff.folders = validateIdCollectionDiff(ctx, "folders", "folder", kit.folders ?? [], ctx.diff.folders, (item, fdiff, p) => {
      const par = (fdiff as FolderDiff).parent?.id ?? item.parent?.id;
      if (par && !(kit.folders ?? []).some((f) => f.id === par)) kitDiffPush(ctx, "errors", "kitdiff.ref.folder-parent-missing", `${p}: parent folder ${par} not in kit`);
      if ((fdiff as FolderDiff).attributes) validateAttributesDiffNested(ctx, `${p}.attributes`, item.attributes ?? [], (fdiff as FolderDiff).attributes);
    });
  }
  if (ctx.diff.authors) ctx.diff.authors = validateIdCollectionDiff(ctx, "authors", "author", kit.authors ?? [], ctx.diff.authors, () => { });
  if (ctx.diff.attributes) validateAttributesDiffNested(ctx, "kit.attributes", kit.attributes ?? [], ctx.diff.attributes);

  const ok = ctx.errors.length === 0;
  return heal ? { ok, errors: ctx.errors, warnings: ctx.warnings, infos: [], diff: ctx.diff } : { ok, errors: ctx.errors, warnings: ctx.warnings, infos: [] };
}

// #endregion ­ƒôªKitImpl Diff Validation

// #region ­ƒøí´©ÅValidation

// #region ­ƒùí´©ÅValidation Core Types

/**
 * Enumeration of EntityKind values.
 **/
export type EntityKind = "KitImpl" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Representation" | "Layer" | "Group" | "Stat";

/**
 * Interface defining DomainLocation structure.
 **/
export interface DomainLocation {
  entityKind: EntityKind;
  entityId?: Id;
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
  relatedIds?: Id[];
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

// #endregion ­ƒùí´©ÅValidation Core Types

// #region ­ƒöìValidation Context And Engine
// Validation context construction and engine MUST be defined here.

/**
 * Interface defining ValidationContext structure.
 **/
export interface ValidationContext {
  kit: KitImpl;
  typesById: Map<Id, Type>;
  designsById: Map<Id, Design>;
  piecesById: Map<Id, { designId: Id; piece: Piece }>;
  connectorsByTypeId: Map<Id, Connector[]>;
  representationsByTypeId: Map<Id, Representation[]>;
}

/**
 * Constructs ValidationContext from components.
 **/
export const buildValidationContext = (kit: KitLike): ValidationContext => {
  const k = asKitInstance(kit);
  const typesById = new Map<Id, Type>();
  const designsById = new Map<Id, Design>();
  const piecesById = new Map<Id, { designId: Id; piece: Piece }>();
  const connectorsByTypeId = new Map<Id, Connector[]>();
  const representationsByTypeId = new Map<Id, Representation[]>();
  toArray(k.types).forEach((t) => {
    typesById.set(t.id, t);
    connectorsByTypeId.set(t.id, toArray(t.connectors));
    representationsByTypeId.set(t.id, toArray(t.representations));
  });
  toArray(k.designs).forEach((d) => {
    designsById.set(d.id, d);
    toArray(d.pieces).forEach((p) => piecesById.set(p.id, { designId: d.id, piece: p }));
  });
  return { kit: k, typesById, designsById, piecesById, connectorsByTypeId, representationsByTypeId };
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
 * Validates KitImpl against constraints.
 **/
export const validateKit = (kit: KitLike, cfg: ValidationConfig = {}): ValidationResult => {
  const ctx = buildValidationContext(kit);
  const constraints = cfg.constraints ?? defaultConstraints;
  return { problems: constraints.flatMap((constraint) => constraint(ctx)) };
};

// #endregion ­ƒöìValidation Context And Engine

// #region ­ƒôíFix Helper
// Validation fix helper functions MUST be defined here.
// Validation fix helper functions MUST be defined here.

/**
 **/
export const semioMakeFix = (ctx: ValidationContext, title: string, buildDiff: (kit: KitImpl) => KitDiff): Fix => ({
  title,
  diff: buildDiff(ctx.kit),
});

// #endregion ­ƒôíFix Helper

// #region ­ƒöæConstraint: ID Uniqueness
// ID uniqueness constraint MUST be enforced here.

/**
 * Constraint validating IdUniqueness rules.
 **/
export const semioIdUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const seen = new Map<Id, EntityKind>();
  const check = (entityKind: EntityKind, entityId: Id) => {
    const existing = seen.get(entityId);
    if (!existing) {
      seen.set(entityId, entityKind);
      return;
    }
    const problem: Problem = {
      constraintId: "id-unique",
      message: `Duplicate ID "${entityId}". Entity IDs are immutable; resolve by removing or replacing the duplicate entity (first occurrence kept).`,
      location: { entityKind, entityId, field: "id" },
      relatedIds: [entityId],
      fixes: [],
    };
    problems.push(problem);
  };
  check("KitImpl", ctx.kit.id);
  toArray(ctx.kit.types).forEach((t) => check("Type", t.id));
  toArray(ctx.kit.designs).forEach((d) => {
    check("Design", d.id);
    toArray(d.pieces).forEach((p) => check("Piece", p.id));
    toArray(d._connections).forEach((c) => check("Connection", c.id));
    toArray(d.stats).forEach((s) => check("Stat", s.id));
  });
  toArray(ctx.kit.qualities).forEach((q) => check("Quality", q.id));
  toArray(ctx.kit.families).forEach((f) => {
    check("Family", f.id);
    toArray(f.ports).forEach((i) => check("Port", i.id));
  });
  toArray(ctx.kit.files).forEach((f) => check("File", f.id));
  toArray(ctx.kit.folders).forEach((f) => check("Folder", f.id));
  return problems;
};

// #endregion ­ƒöæConstraint: ID Uniqueness

// #region ­ƒº▒Constraint: Type Name Uniqueness
// Type name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating TypeNameUniqueness rules.
 **/
export const semioTypeNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const groups = new Map<string, Type[]>();
  toArray(ctx.kit.types).forEach((t) => {
    if ((t.lifecycle ?? "active") === "deleted") return;
    const name = t.name ?? "";
    const familiesKey = JSON.stringify([...(t.families ?? [])].map((f) => f.id).sort());
    const key = `${name}\0${familiesKey}`;
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key)!.push(t);
  });
  const allNames = Array.from(
    new Set(
      toArray(ctx.kit.types)
        .filter((t) => (t.lifecycle ?? "active") !== "deleted")
        .map((t) => t.name ?? ""),
    ),
  );
  for (const [_key, group] of groups) {
    if (group.length <= 1) continue;
    const name = group[0].name ?? "";
    const [first, ...rest] = group;
    rest.forEach((type) => {
      const fix = semioMakeFix(ctx, `Rename "${name}"`, () => ({
        types: {
          updated: [{ type: { id: type.id }, diff: { name: generateUniqueName(name, allNames) } }],
        },
      }));
      problems.push({
        constraintId: "type-name-unique",
        message: `Duplicate type name "${name}".`,
        location: { entityKind: "Type", entityId: type.id, field: "name" },
        relatedIds: group.map((t) => t.id),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion ­ƒº▒Constraint: Type Name Uniqueness

// #region ­ƒôÉConstraint: Design Name Uniqueness
// Design name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating DesignNameUniqueness rules.
 **/
export const semioDesignNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const groups = new Map<string, Design[]>();
  toArray(ctx.kit.designs).forEach((d) => {
    const name = d.name ?? "";
    const familiesKey = JSON.stringify([...(d.families ?? [])].map((f) => f.id).sort());
    const key = `${name}\0${familiesKey}`;
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key)!.push(d);
  });
  const allNames = Array.from(new Set(toArray(ctx.kit.designs).map((d) => d.name ?? "")));
  for (const [_key, group] of groups) {
    if (group.length <= 1) continue;
    const name = group[0].name ?? "";
    const [first, ...rest] = group;
    rest.forEach((design) => {
      const fix = semioMakeFix(ctx, `Rename "${name}"`, () => ({
        designs: {
          updated: [{ design: { id: design.id }, diff: { name: generateUniqueName(name, allNames) } }],
        },
      }));
      problems.push({
        constraintId: "design-name-unique",
        message: `Duplicate design name "${name}".`,
        location: { entityKind: "Design", entityId: design.id, field: "name" },
        relatedIds: group.map((d) => d.id),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion ­ƒôÉConstraint: Design Name Uniqueness

// #region ­ƒº®Constraint: Piece Name Uniqueness
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
        const fix = semioMakeFix(ctx, `Rename piece "${name}"`, () => ({
          designs: {
            updated: [
              {
                design: { id: design.id },
                diff: {
                  pieces: {
                    updated: [{ piece: { id: piece.id }, diff: { name: generateUniqueName(name, allNames) } }],
                  },
                },
              },
            ],
          },
        }));
        problems.push({
          constraintId: "piece-name-unique",
          message: `Duplicate piece name "${name}" inside design "${design.name}".`,
          location: { entityKind: "Piece", entityId: piece.id, field: "name" },
          relatedIds: list.map((p) => p.id),
          fixes: [fix],
        });
      });
    }
  });
  return problems;
};

// #endregion ­ƒº®Constraint: Piece Name Uniqueness

// #region ­ƒö¼Constraint: Quality Name Uniqueness
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
      const fix = semioMakeFix(ctx, `Rename quality "${name}"`, () => ({
        qualities: {
          updated: [{ quality: { id: quality.id }, diff: { name: generateUniqueName(name, allNames) } }],
        },
      }));
      problems.push({
        constraintId: "quality-name-unique",
        message: `Duplicate quality name "${name}".`,
        location: { entityKind: "Quality", entityId: quality.id, field: "name" },
        relatedIds: list.map((q) => q.id),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion ­ƒö¼Constraint: Quality Name Uniqueness

// #region ÔÜôConstraint: Port Name Uniqueness
// Port name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating PortNameUniqueness rules.
 **/
export const semioPortNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const ports = (ctx.kit.families ?? []).flatMap((f) => toArray(f.ports));
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
      const familyOfPort = (ctx.kit.families ?? []).find((f) => (f.ports ?? []).some((p) => p.id === iface.id));
      const fix = semioMakeFix(ctx, `Rename port "${name}"`, () => ({
        families: familyOfPort
          ? {
            updated: [{ family: { id: familyOfPort.id }, diff: { ports: { updated: [{ port: { id: iface.id }, diff: { name: generateUniqueName(name, allNames) } }] } } }],
          }
          : undefined,
      }));
      problems.push({
        constraintId: "port-name-unique",
        message: `Duplicate port name "${name}".`,
        location: { entityKind: "Port", entityId: iface.id, field: "name" },
        relatedIds: list.map((i) => i.id),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion ÔÜôConstraint: Port Name Uniqueness

// #region ­ƒôäConstraint: File Name Uniqueness
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
      const fix = semioMakeFix(ctx, `Rename file "${name}"`, () => ({
        files: {
          updated: [{ file: { id: file.id }, diff: { name: generateUniqueName(name, allNames) } }],
        },
      }));
      problems.push({
        constraintId: "file-name-unique",
        message: `Duplicate file name "${name}".`,
        location: { entityKind: "File", entityId: file.id, field: "name" },
        relatedIds: list.map((f) => f.id),
        fixes: [fix],
      });
    });
  }
  return problems;
};

// #endregion ­ƒôäConstraint: File Name Uniqueness

// #region ­ƒôüConstraint: Folder Name Uniqueness
// Folder name uniqueness constraint MUST be enforced here.

/**
 * Constraint validating FolderNameUniqueness rules.
 **/
export const semioFolderNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const byParent = new Map<Id | undefined, Folder[]>();
  const folders = toArray(ctx.kit.folders);
  folders.forEach((f) => {
    const pid = f.parent?.id as Id | undefined;
    if (!byParent.has(pid)) byParent.set(pid, []);
    byParent.get(pid)!.push(f);
  });
  for (const [parentId, siblings] of byParent) {
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
        const fix = semioMakeFix(ctx, `Rename folder "${name}"`, () => ({
          folders: {
            updated: [{ folder: { id: folder.id }, diff: { name: generateUniqueName(name, allNames) } }],
          },
        }));
        problems.push({
          constraintId: "folder-name-unique",
          message: `Duplicate folder name "${name}" among siblings.`,
          location: { entityKind: "Folder", entityId: folder.id, field: "name" },
          relatedIds: list.map((f) => f.id),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion ­ƒôüConstraint: Folder Name Uniqueness

// #region ­ƒöîConstraint: Connector Name Uniqueness Within Type
// Connector name uniqueness within type constraint MUST be enforced here.

/**
 * Constraint validating ConnectorNameUniqueness rules.
 **/
export const semioConnectorNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  for (const [typeId, connectors] of ctx.connectorsByTypeId) {
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
      const type = ctx.typesById.get(typeId);
      rest.forEach((connector) => {
        const fix = semioMakeFix(ctx, `Rename connector "${name}"`, () => ({
          types: {
            updated: [
              {
                type: { id: typeId },
                diff: {
                  connectors: {
                    updated: [{ connector: { id: connector.id }, diff: { name: generateUniqueName(name, allNames) } }],
                  },
                },
              },
            ],
          },
        }));
        problems.push({
          constraintId: "connector-name-unique",
          message: `Duplicate connector name "${name}" inside type "${type?.name}".`,
          location: { entityKind: "Connector", entityId: connector.id, field: "name" },
          relatedIds: list.map((p) => p.id),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion ­ƒöîConstraint: Connector Name Uniqueness Within Type

// #region ­ƒù┐Constraint: Representation Name Uniqueness Within Type
// Representation name uniqueness within type constraint MUST be enforced here.

/**
 * Constraint validating RepresentationNameUniqueness rules.
 **/
export const semioRepresentationNameUniquenessConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  for (const [typeId, representations] of ctx.representationsByTypeId) {
    if (representations.length === 0) continue;
    const nameMap = new Map<string, Representation[]>();
    representations.forEach((m) => {
      const name = m.name ?? "";
      if (!nameMap.has(name)) nameMap.set(name, []);
      nameMap.get(name)!.push(m);
    });
    for (const [name, list] of nameMap) {
      if (list.length <= 1) continue;
      const [first, ...rest] = list;
      const allNames = representations.map((m) => m.name ?? "");
      const type = ctx.typesById.get(typeId);
      rest.forEach((representation) => {
        const fix = semioMakeFix(ctx, `Rename representation "${name}"`, () => ({
          types: {
            updated: [
              {
                type: { id: typeId },
                diff: {
                  representations: {
                    updated: [{ representation: { id: representation.id }, diff: { name: generateUniqueName(name, allNames) } }],
                  },
                },
              },
            ],
          },
        }));
        problems.push({
          constraintId: "representation-name-unique",
          message: `Duplicate representation name "${name}" inside type "${type?.name}".`,
          location: { entityKind: "Representation", entityId: representation.id, field: "name" },
          relatedIds: list.map((m) => m.id),
          fixes: [fix],
        });
      });
    }
  }
  return problems;
};

// #endregion ­ƒù┐Constraint: Representation Name Uniqueness Within Type

// #region ­ƒÄ¿Constraint: Layer Path Uniqueness Within Design
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
        const fix = semioMakeFix(ctx, `Rename layer "${path}"`, () => ({
          designs: {
            updated: [
              {
                design: { id: design.id },
                diff: {
                  layers: {
                    updated: [{ layer: { id: layer.id }, diff: { path: generateUniqueName(path, allPaths) } }],
                  },
                },
              },
            ],
          },
        }));
        problems.push({
          constraintId: "layer-path-unique",
          message: `Duplicate layer path "${path}" inside design "${design.name}".`,
          location: { entityKind: "Layer", entityId: layer.id, field: "path" },
          fixes: [fix],
        });
      });
    }
  });
  return problems;
};

// #endregion ­ƒÄ¿Constraint: Layer Path Uniqueness Within Design

// #region ­ƒôÉConstraint: Design Piece Same Family Constraint
// Design piece same family constraint MUST be enforced here.

/**
 * Constraint validating DesignPieceSameFamily rules.
 **/
export const semioDesignPieceSameFamilyConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  toArray(ctx.kit.designs).forEach((design) => {
    const pieces = toArray(design.pieces);
    pieces.forEach((piece) => {
      if (!piece.design?.id) return;
      try {
        const pieceDesign = ctx.designsById.get(piece.design.id);
        if (!pieceDesign) return;

        const containerFamilies = design.families ?? [];
        const pieceFamilies = pieceDesign.families ?? [];
        const sameFamily = containerFamilies.length === 0 && pieceFamilies.length === 0 ? design.id === pieceDesign.id : containerFamilies.some((f) => pieceFamilies.some((pf) => pf.id === f.id));

        if (sameFamily) {
          const conns = toArray(design._connections);
          const removedConnIds = conns.filter((c) => c.connected.piece.id === piece.id || c.connecting.piece.id === piece.id).map((c) => ({ id: c.id }));
          const fix = semioMakeFix(ctx, `Remove design piece "${piece.name || piece.id}"`, () => ({
            designs: {
              updated: [
                {
                  design: { id: design.id },
                  diff: {
                    pieces: { removed: [{ id: piece.id }] },
                    ...(removedConnIds.length > 0 ? { connections: { removed: removedConnIds } } : {}),
                  },
                },
              ],
            },
          }));
          problems.push({
            constraintId: "design-piece-same-family",
            message: `Design piece "${piece.name || piece.id}" references design "${pieceDesign.name}" which is in the same design family as container design "${design.name}". A design cannot contain design pieces from the same family.`,
            location: { entityKind: "Piece", entityId: piece.id, field: "design" },
            relatedIds: [piece.id, design.id, pieceDesign.id],
            fixes: [fix],
          });
        }
      } catch { }
    });
  });
  return problems;
};

// #endregion ­ƒôÉConstraint: Design Piece Same Family Constraint

// #region Ô£àConstraint Registration
// Constraint registration and default configurations MUST be defined here.

defaultConstraints = [
  semioIdUniquenessConstraint,
  semioTypeNameUniquenessConstraint,
  semioDesignNameUniquenessConstraint,
  semioPieceNameUniquenessConstraint,
  semioQualityNameUniquenessConstraint,
  semioPortNameUniquenessConstraint,
  semioFileNameUniquenessConstraint,
  semioFolderNameUniquenessConstraint,
  semioConnectorNameUniquenessConstraint,
  semioRepresentationNameUniquenessConstraint,
  semioLayerPathUniquenessConstraint,
  semioDesignPieceSameFamilyConstraint,
];

// #endregion Ô£àConstraint Registration

// #region ­ƒîº´©ÅValidation Serialization
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
  entityId: string;
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
    entityId: problem.location?.entityId ?? (problem as any).entityId ?? "",
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
    return a.entityId.localeCompare(b.entityId);
  });
  return JSON.stringify(serializable, null, 2);
};

/**
 * Parses ValidationResult from serialized input.
 **/
export const parseValidationResult = (json: string): SerializableValidationResult => JSON.parse(json);
// ­ƒöæisId checks whether a string is a valid ID format.
const isId = (s: string): boolean => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(s);

/**
 * Deep equality check for KitDiffs ignoring NewIds entities.
 **/
export const areKitDiffsEqualIgnoringNewIds = (a: KitDiff, b: KitDiff): boolean => {
  const normalize = (obj: unknown): unknown => {
    if (obj === null || obj === undefined) return obj;
    if (typeof obj === "string" && isId(obj)) return "<ID>";
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
      return x.entityId.localeCompare(y.entityId);
    });
  const sortedA = sortProblems(serializableA.problems);
  const sortedB = sortProblems(serializableB.problems);
  return sortedA.every((problemA, i) => {
    const problemB = sortedB[i];
    if (problemA.constraintId !== problemB.constraintId || problemA.message !== problemB.message || problemA.entityKind !== problemB.entityKind || problemA.entityId !== problemB.entityId) return false;
    if (problemA.fixes.length !== problemB.fixes.length) return false;
    return problemA.fixes.every((fixA, j) => {
      const fixB = problemB.fixes[j];
      return fixA.title === fixB.title && areKitDiffsEqualIgnoringNewIds(fixA.diff ?? {}, fixB.diff ?? {});
    });
  });
};

// #endregion ­ƒîº´©ÅValidation Serialization

// #endregion ­ƒøí´©ÅValidation

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
    id: id(),
    name,
    size,
    hash: hash.toString(36),
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };
};

// #region ­ƒº┐KitImpl Import/Export
// KitImpl serialization and deserialization functions MUST be defined here.

/**
 * Interface defining KitImportResult structure.
 **/
export interface KitImportResult {
  kit: KitImpl;
  kind?: KitKind;
  files?: Record<string, Uint8Array>;
}
// ­ƒùä´©ÅcachedSqlJs caches the SQL.js WASM module for reuse.
let cachedSqlJs: any = null;
// ­ƒùä´©ÅgetSqlJs loads and returns the SQL.js WASM module.
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
        const candidatePaths = [path.join(__dirname, "public", "sql-wasm.wasm"), path.join(__dirname, "..", "sketchpad", "public", "sql-wasm.wasm"), path.join(__dirname, "..", "..", "node_modules", "sql.js", "dist", "sql-wasm.wasm")];
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
// buildFolderPath builds a slash-separated folder path from root to the given folder id.
// ­ƒôüUses proper mime type inferred from file extension.
const buildFolderPath = (kit: KitImpl, folderId: string): string => {
  const findFolder = (id: string): Folder | undefined => (kit.folders || []).find((f) => f.id === id);
  const parts: string[] = [];
  let current = findFolder(folderId);
  while (current) {
    parts.unshift(current.name);
    current = current.parent?.id ? findFolder(current.parent.id) : undefined;
  }
  return parts.join("/");
};
// buildFilePath builds the full path of a kit file including its folder hierarchy.
// ­ƒÅù´©ÅUses proper mime type inferred from file extension.
const buildFilePath = (kit: KitImpl, file: File): string => {
  if (file.folder?.id) {
    const folderPath = buildFolderPath(kit, file.folder.id);
    if (folderPath) return `${folderPath}/${file.name}`;
  }
  return file.name;
};
const bytesToUtf8 = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);
const hasZipSignature = (bytes: Uint8Array): boolean => bytes.length >= 4 && bytes[0] === 0x50 && bytes[1] === 0x4b && bytes[2] === 0x03 && bytes[3] === 0x04;
const collectKitFiles = (kit: KitImpl): Record<string, Uint8Array> => {
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
  return { kind: "dev", kit: deserializeKit(json), files: {} };
};
export const exportFileKit = (kit: KitImpl): string => serializeKit(kit);
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
  let kit: KitImpl;
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
          stl: "representation/stl",
          obj: "representation/obj",
          glb: "representation/gltf-binary",
          gltf: "representation/gltf+json",
          "3dm": "representation/vnd.3dm",
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
/**
 * Applies `diff` to `kit` in place and returns the same kit reference.
 * Callers that need an immutable snapshot should copy explicitly (e.g. serialize/deserialize).
 **/
export const editTemporaryKit = (kit: KitLike, diff: KitDiff): KitImpl => {
  const k = asKitInstance(kit);
  k.replayChangeUnchecked(diff);
  return k;
};

// #region ­ƒÅÀ´©ÅKitImpl Kind Classes
// Typed KitImpl wrappers scoped by KitKind. Each class carries a `kind` discriminator and
// wraps a plain KitImpl value. SyncKit is an interface for kits that support bi-directional sync.

/**
 * ­ƒÜÜ Transport kit ÔÇô ephemeral, in-memory only, never persisted.
 **/
export class TransportKit {
  readonly kind = "transport" as const;
  constructor(public kit: KitImpl) { }
}

/**
 * ­ƒôª Archive kit ÔÇô read-only snapshot loaded from an archive file.
 **/
export class ArchiveKit {
  readonly kind = "archive" as const;
  constructor(public kit: KitImpl) { }
}

/**
 * ­ƒöä SyncKit ÔÇô interface for kits that support bi-directional sync.
 **/
export interface SyncKit {
  readonly kind: KitKind;
  kit: KitImpl;
  apply(diff: KitDiff): void;
}

/**
 * ­ƒÆ╗ Dev kit ÔÇô local dev workspace backed by a file-system directory.
 **/
export class DevKit implements SyncKit {
  readonly kind = "dev" as const;
  constructor(public kit: KitImpl) { }
  apply(diff: KitDiff): void {
    applyKitDiff(this.kit, diff);
  }
}

/**
 * ­ƒôü Local kit ÔÇô local SQLite-backed kit.
 **/
export class LocalKit implements SyncKit {
  readonly kind = "local" as const;
  constructor(public kit: KitImpl) { }
  apply(diff: KitDiff): void {
    applyKitDiff(this.kit, diff);
  }
}

/**
 * ­ƒîÉ Remote kit ÔÇô kit synced with a remote server.
 **/
export class RemoteKit implements SyncKit {
  readonly kind = "remote" as const;
  constructor(public kit: KitImpl) { }
  apply(diff: KitDiff): void {
    applyKitDiff(this.kit, diff);
  }
}
// #endregion ­ƒÅÀ´©ÅKitImpl Kind Classes

/**
 * Imports KitImpl from external source.
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
 * Exports KitImpl to external format.
 **/
export const exportKit = async (kit: KitImpl): Promise<Blob> => {
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
export const areKitsEqual = (a: KitImpl, b: KitImpl): boolean => {
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
      const attrB = arrB.find((x) => x.id === attrA.id);
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
      const propB = arrB.find((x) => x.id === propA.id);
      if (!propB) return false;
      if (propA.quality.id !== propB.quality.id) return false;
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
      const connectorB = arrB.find((x) => x.id === connectorA.id);
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
      if (normalizeValue(connectorA.port?.id) !== normalizeValue(connectorB.port?.id)) return false;
      if (!arePropsEqual(connectorA.props, connectorB.props)) return false;
      if (!areAttributesEqual(connectorA.attributes, connectorB.attributes)) return false;
    }
    return true;
  };

  const areRepresentationsEqual = (a?: Representation[], b?: Representation[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const representationA of arrA) {
      const representationB = arrB.find((x) => x.id === representationA.id);
      if (!representationB) return false;
      if (normalizeValue(representationA.name) !== normalizeValue(representationB.name)) return false;
      if (representationA.file.id !== representationB.file.id) return false;

      const tagsA = normalizeArray(representationA.tags).map((t) => (typeof t === "object" ? t.id : t));
      const tagsB = normalizeArray(representationB.tags).map((t) => (typeof t === "object" ? t.id : t));
      if (tagsA.length !== tagsB.length || !tagsA.every((g) => tagsB.includes(g))) return false;
      if (!areAttributesEqual(representationA.attributes, representationB.attributes)) return false;
    }
    return true;
  };

  const areTypesEqual = (a?: Type[], b?: Type[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const typeA of arrA) {
      const typeB = arrB.find((t) => {
        if (t.id !== typeA.id) return false;
        const familiesA = typeA.families ?? [];
        const familiesB = t.families ?? [];
        if (familiesA.length !== familiesB.length) return false;
        return familiesA.every((fA) => familiesB.some((fB) => (typeof fA === "string" ? fA === fB : fA.id === (typeof fB === "string" ? fB : fB.id))));
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
      if (normalizeValue(typeA.location?.id) !== normalizeValue(typeB.location?.id)) return false;
      if (!arraysEqual(normalizeArray(typeA.concepts), normalizeArray(typeB.concepts))) return false;
      if (!arraysEqual(normalizeArray(typeA.authors?.map((a) => a.id)), normalizeArray(typeB.authors?.map((a) => a.id)))) return false;
      if (!arePropsEqual(typeA.props, typeB.props)) return false;
      if (!areRepresentationsEqual(typeA.representations, typeB.representations)) return false;
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
      const pieceB = arrB.find((x) => x.id === pieceA.id);
      if (!pieceB) return false;
      if (normalizeValue(pieceA.name) !== normalizeValue(pieceB.name)) return false;
      if (pieceA.type?.id !== pieceB.type?.id) return false;
      if (pieceA.design?.id !== pieceB.design?.id) return false;
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
      const connB = arrB.find((x) => x.id === connA.id);
      if (!connB) return false;
      if (connA.connected.piece.id !== connB.connected.piece.id) return false;
      if (normalizeValue(connA.connected.designPiece?.id) !== normalizeValue(connB.connected.designPiece?.id)) return false;
      if (normalizeValue(connA.connected.connector?.id) !== normalizeValue(connB.connected.connector?.id)) return false;
      if (connA.connecting.piece.id !== connB.connecting.piece.id) return false;
      if (normalizeValue(connA.connecting.designPiece?.id) !== normalizeValue(connB.connecting.designPiece?.id)) return false;
      if (normalizeValue(connA.connecting.connector?.id) !== normalizeValue(connB.connecting.connector?.id)) return false;
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
        if (d.id !== designA.id) return false;
        const familiesA = designA.families ?? [];
        const familiesB = d.families ?? [];
        if (familiesA.length !== familiesB.length) return false;
        return familiesA.every((fA) => familiesB.some((fB) => (typeof fA === "string" ? fA === fB : fA.id === (typeof fB === "string" ? fB : fB.id))));
      });
      if (!designB) return false;
      if (designA.name !== designB.name) return false;
      if (normalizeValue(designA.description) !== normalizeValue(designB.description)) return false;
      if (normalizeValue(designA.icon) !== normalizeValue(designB.icon)) return false;
      if (normalizeValue(designA.image) !== normalizeValue(designB.image)) return false;
      if (!arraysEqual(normalizeArray(designA.concepts), normalizeArray(designB.concepts))) return false;
      if (!arePiecesEqual(designA.pieces, designB.pieces)) return false;
      if (!areConnectionsEqual(designA._connections, designB._connections)) return false;
      if (!areAttributesEqual(designA.attributes, designB.attributes)) return false;
    }
    return true;
  };

  const arePortsEqual = (a?: Port[], b?: Port[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const ifaceA of arrA) {
      const ifaceB = arrB.find((x) => x.id === ifaceA.id);
      if (!ifaceB) return false;
      if (ifaceA.name !== ifaceB.name) return false;
      if (normalizeValue(ifaceA.description) !== normalizeValue(ifaceB.description)) return false;
      if (!areAttributesEqual(ifaceA.attributes, ifaceB.attributes)) return false;
    }
    return true;
  };

  const areFamiliesEqual = (a?: Family[], b?: Family[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const famA of arrA) {
      const famB = arrB.find((x) => x.id === famA.id);
      if (!famB) return false;
      if (famA.name !== famB.name) return false;
      if (normalizeValue(famA.description) !== normalizeValue(famB.description)) return false;
      if (normalizeValue(famA.icon) !== normalizeValue(famB.icon)) return false;
      if (!arePortsEqual(famA.ports, famB.ports)) return false;
      if (!areAttributesEqual(famA.attributes, famB.attributes)) return false;
    }
    return true;
  };

  const areQualitiesEqual = (a?: Quality[], b?: Quality[]): boolean => {
    const arrA = normalizeArray(a);
    const arrB = normalizeArray(b);
    if (arrA.length !== arrB.length) return false;
    for (const qualA of arrA) {
      const qualB = arrB.find((x) => x.id === qualA.id);
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
      const fileB = arrB.find((x) => x.id === fileA.id);
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
      const folderB = arrB.find((x) => x.id === folderA.id);
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
      const authorB = arrB.find((x) => x.id === authorA.id);
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
      const conceptB = arrB.find((x) => x.id === conceptA.id);
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
      const tagB = arrB.find((x) => x.id === tagA.id);
      if (!tagB) return false;
      if (tagA.name !== tagB.name) return false;
      if (normalizeValue(tagA.description) !== normalizeValue(tagB.description)) return false;
      if (normalizeValue(tagA.icon) !== normalizeValue(tagB.icon)) return false;
    }
    return true;
  };

  if (a.id !== b.id) return false;
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
  if (!areFamiliesEqual(a.families, b.families)) return false;
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
    return record.id ?? record.type?.id ?? record.design?.id ?? record.piece?.id ?? record.connection?.id ?? record.representation?.id ?? record.port?.id ?? record.connector?.id ?? record.prop?.id ?? record.attribute?.id;
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
  const areRemovedArraysEqual = (a?: { id: string }[], b?: { id: string }[]): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (a.length !== b.length) return false;
    const aIds = new Set(a.map((x) => x.id));
    const bIds = new Set(b.map((x) => x.id));
    for (const id of aIds) {
      if (!bIds.has(id)) return false;
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
      const ub = updatedB.find((x) => x.attribute.id === ua.attribute.id);
      if (!ub) return false;
      if (!areAttributeDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
      const ub = updatedB.find((x) => x.prop.id === ua.prop.id);
      if (!ub) return false;
      if (!arePropDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
      if (!ab) return false;
      if (aa.quality.id !== ab.quality.id) return false;
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
      const ub = updatedB.find((x) => x.connector.id === ua.connector.id);
      if (!ub) return false;
      if (!areConnectorDiffEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
    if (normalizeValue(a.port?.id) !== normalizeValue(b.port?.id)) return false;
    if (!arePropsDiffsEqual(a.props, b.props)) return false;
    if (!areAttributesDiffsEqual(a.attributes, b.attributes)) return false;
    return true;
  };

  const areRepresentationsDiffsEqual = (a?: z.infer<typeof RepresentationsDiffSchema>, b?: z.infer<typeof RepresentationsDiffSchema>): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (!areRemovedArraysEqual(a.removed, b.removed)) return false;
    const updatedA = normalizeArray(a.updated);
    const updatedB = normalizeArray(b.updated);
    if (updatedA.length !== updatedB.length) return false;
    for (const ua of updatedA) {
      const ub = updatedB.find((x) => x.representation.id === ua.representation.id);
      if (!ub) return false;
      if (!areRepresentationDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (normalizeValue(aa.file?.id) !== normalizeValue(ab.file?.id)) return false;
      if (!arraysEqual(normalizeArray(aa.tags), normalizeArray(ab.tags))) return false;
    }
    return true;
  };

  const areRepresentationDiffsEqual = (a?: RepresentationDiff, b?: RepresentationDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.file?.id) !== normalizeValue(b.file?.id)) return false;
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
      const ub = updatedB.find((x) => x.type.id === ua.type.id);
      if (!ub) return false;
      if (!areTypeDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
    if (normalizeValue(a.location?.id) !== normalizeValue(b.location?.id)) return false;
    if (a.concepts && b.concepts) {
      if (!arraysEqual(normalizeArray(a.concepts), normalizeArray(b.concepts))) return false;
    } else if (a.concepts || b.concepts) {
      return false;
    }
    if (!areRepresentationsDiffsEqual(a.representations, b.representations)) return false;
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
      const ub = updatedB.find((x) => x.piece.id === ua.piece.id);
      if (!ub) return false;
      if (!arePieceDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
      if (!ab) return false;
      if (normalizeValue(aa.name) !== normalizeValue(ab.name)) return false;
      if (aa.type?.id !== ab.type?.id) return false;
      if (aa.design?.id !== ab.design?.id) return false;
    }
    return true;
  };

  const arePieceDiffsEqual = (a?: PieceDiff, b?: PieceDiff): boolean => {
    if (!a && !b) return true;
    if (!a || !b) return false;
    if (normalizeValue(a.name) !== normalizeValue(b.name)) return false;
    if (normalizeValue(a.type?.id) !== normalizeValue(b.type?.id)) return false;
    if (normalizeValue(a.design?.id) !== normalizeValue(b.design?.id)) return false;
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
      const ub = updatedB.find((x) => x.connection.id === ua.connection.id);
      if (!ub) return false;
      if (!areConnectionDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
      if (!ab) return false;
      if (aa.connected.piece.id !== ab.connected.piece.id) return false;
      if (aa.connecting.piece.id !== ab.connecting.piece.id) return false;
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
      const ub = updatedB.find((x) => x.design.id === ua.design.id);
      if (!ub) return false;
      if (!areDesignDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
      const ub = updatedB.find((x) => x.port.id === ua.port.id);
      if (!ub) return false;
      if (!arePortDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
      const ub = updatedB.find((x) => x.quality.id === ua.quality.id);
      if (!ub) return false;
      if (!areQualityDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
      const ub = updatedB.find((x) => x.file.id === ua.file.id);
      if (!ub) return false;
      if (!areFileDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
      const ub = updatedB.find((x) => x.folder.id === ua.folder.id);
      if (!ub) return false;
      if (!areFolderDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
      const ub = updatedB.find((x) => x.author.id === ua.author.id);
      if (!ub) return false;
      if (!areAuthorDiffsEqual(ua.diff, ub.diff)) return false;
    }
    const addedA = normalizeArray(a.added);
    const addedB = normalizeArray(b.added);
    if (addedA.length !== addedB.length) return false;
    for (const aa of addedA) {
      const ab = addedB.find((x) => x.id === aa.id);
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
// ­ƒôªsqliteToKit converts a SQLite database into a kit object.
export const sqliteToKit = async (db: any): Promise<KitImpl> => {
  const existingTables = new Set<string>();
  const existingColumns = new Map<string, Set<string>>();
  const tableStmt = db.prepare("SELECT name FROM sqlite_master WHERE type='table'");
  while (tableStmt.step()) {
    const tableName = tableStmt.getAsObject().name as string;
    existingTables.add(tableName);
  }
  tableStmt.free();

  for (const tableName of existingTables) {
    const columns = new Set<string>();
    const columnStmt = db.prepare(`PRAGMA table_info("${String(tableName).replace(/"/g, '""')}")`);
    while (columnStmt.step()) {
      const row = columnStmt.getAsObject();
      if (typeof row.name === "string") columns.add(row.name);
    }
    columnStmt.free();
    existingColumns.set(tableName, columns);
  }

  const quoteIdentifier = (value: string): string => `"${String(value).replace(/"/g, '""')}"`;
  const normalizeRow = (row: any): any => {
    const normalized: any = { ...row };
    for (const [key, value] of Object.entries(row)) {
      if (key === "guid" && normalized.id === undefined) {
        normalized.id = value;
        continue;
      }
      if (key.endsWith("_guid_ref")) {
        const alias = `${key.slice(0, -9)}_id_ref`;
        if (normalized[alias] === undefined) normalized[alias] = value;
        continue;
      }
      if (key.endsWith("_guid")) {
        const alias = `${key.slice(0, -5)}_id`;
        if (normalized[alias] === undefined) normalized[alias] = value;
      }
    }
    return normalized;
  };
  const pickColumn = (tableName: string, ...candidates: string[]): string => {
    const columns = existingColumns.get(tableName);
    for (const candidate of candidates) {
      if (columns?.has(candidate)) return candidate;
    }
    throw new Error(`Missing expected column on ${tableName}: ${candidates.join(", ")}`);
  };

  const execResult = (query: string, params?: any[]): any[] => {
    const stmt = db.prepare(query);
    if (params) {
      stmt.bind(params);
    }
    const result: any[] = [];
    while (stmt.step()) {
      const row = normalizeRow(stmt.getAsObject());
      result.push(row);
    }
    stmt.free();
    return result;
  };

  const selectAll = (tableName: string, options?: { columns: string[]; value: any; orderBy?: string }): any[] => {
    let query = `SELECT * FROM ${quoteIdentifier(tableName)}`;
    const params: any[] = [];
    if (options) {
      const columnName = pickColumn(tableName, ...options.columns);
      query += ` WHERE ${quoteIdentifier(columnName)} = ?`;
      params.push(options.value);
      if (options.orderBy) query += ` ORDER BY ${options.orderBy}`;
    }
    return execResult(query, params);
  };

  const safeSelectAll = (tableName: string, options?: { columns: string[]; value: any; orderBy?: string }): any[] => {
    if (!existingTables.has(tableName)) {
      return [];
    }
    return selectAll(tableName, options);
  };
  const selectById = (tableName: string, entityId: any): any | null => selectAll(tableName, { columns: ["id", "guid"], value: entityId })[0] ?? null;

  const kitRows = selectAll("kit");
  if (kitRows.length === 0) {
    throw new Error("No kit found in database");
  }
  const kitRow = kitRows[0];

  const toUndefined = (value: any): any => (value === null || value === "" ? undefined : value);
  const buildAttribute = (a: any): any => {
    const attr: any = { id: a.id, key: a.key };
    const value = toUndefined(a.value);
    const definition = toUndefined(a.definition);
    if (value !== undefined) attr.value = value;
    if (definition !== undefined) attr.definition = definition;
    return attr;
  };
  const mapOrUndefined = <T, R>(arr: T[], mapper: (item: T) => R): R[] | undefined => (arr.length > 0 ? arr.map(mapper) : undefined);

  const kit: KitImpl = {
    id: kitRow.id || kitRow.uri || id(),
    name: kitRow.name || "Unnamed KitImpl",
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

  const familyRows = selectAll("family", { columns: ["kit_id", "kit_guid"], value: kit.id });
  const normalizeEntityRefs = (raw: any): Array<{ id: string }> | undefined => {
    if (raw === undefined || raw === null || raw === "") return undefined;
    const parsed = typeof raw === "string" ? JSON.parse(raw) : raw;
    if (!Array.isArray(parsed)) return undefined;
    const refs = parsed
      .map((entry: any) => {
        if (typeof entry === "string") return { id: entry };
        if (entry && typeof entry.id === "string") return { id: entry.id };
        if (entry && typeof entry.guid === "string") return { id: entry.guid };
        return null;
      })
      .filter((entry: any): entry is { id: string } => entry !== null);
    return refs.length > 0 ? refs : undefined;
  };

  const types = selectAll("type", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.types = mapOrUndefined(types, (row: any) => {
    const typeId = row.id || String(row.id);
    const representations = selectAll("representation", { columns: ["type_id", "type_guid"], value: typeId });
    const connectors = selectAll("connector", { columns: ["type_id", "type_guid"], value: typeId });
    const typeAttributes = selectAll("attribute", { columns: ["type_id", "type_guid"], value: typeId });
    const typeConcepts = selectAll("type_concept", { columns: ["type_id", "type_guid"], value: typeId });
    const typeAuthors = selectAll("type_author", { columns: ["type_id", "type_guid"], value: typeId, orderBy: "rank" });

    const type: any = {
      id: typeId,
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
    const families = normalizeEntityRefs(toUndefined(row.families));
    if (families) type.families = families;
    if (row.virtual) type.virtual = true;
    const unit = toUndefined(row.unit);
    if (unit !== undefined) type.unit = unit;
    if (row.stock !== null && row.stock !== undefined) type.stock = row.stock;
    if (row.location_id) type.location = { id: row.location_id };

    const concepts = mapOrUndefined(typeConcepts, (c: any) => c.concept);
    if (concepts) type.concepts = concepts;

    const authors = mapOrUndefined(typeAuthors, (ta: any) => ({ id: ta.author_id }));
    if (authors) type.authors = authors;

    const representations_value = mapOrUndefined(representations, (m: any) => {
      const representationTags = selectAll("representation_tag", { columns: ["representation_id", "representation_guid"], value: m.id });
      const representationAttributes = selectAll("attribute", { columns: ["representation_id", "representation_guid"], value: m.id });
      return {
        id: m.id,
        file: { id: m.file_id },
        name: toUndefined(m.name),
        description: toUndefined(m.description),
        tags: representationTags.map((t: any) => ({ id: t.tag_id })),
        attributes: mapOrUndefined(representationAttributes, buildAttribute),
      };
    });
    if (representations_value) type.representations = representations_value;

    const connectors_value = mapOrUndefined(connectors, (p: any) => {
      const connectorProps = selectAll("prop", { columns: ["connector_id", "connector_guid"], value: p.id });
      const connectorAttributes = selectAll("attribute", { columns: ["connector_id", "connector_guid"], value: p.id });

      const connector: any = {
        id: p.id,
        point: { x: p.point_x, y: p.point_y, z: p.point_z },
        direction: { x: p.direction_x, y: p.direction_y, z: p.direction_z },
        t: p.t,
      };

      if (p.name) connector.name = p.name;
      if (p.mandatory) connector.mandatory = true;
      if (p.port_id) connector.port = { id: p.port_id };
      if (p.description) connector.description = p.description;

      const props_value = connectorProps
        .map((pr: any) => {
          const propAttributes = selectAll("attribute", { columns: ["prop_id", "prop_guid"], value: pr.id });
          if (!pr.quality_id) return null;
          return {
            id: pr.id,
            value: String(pr.value),
            unit: toUndefined(pr.unit),
            quality: { id: pr.quality_id },
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

    const typeProps = safeSelectAll("type_prop", { columns: ["type_id", "type_guid"], value: typeId })
      .map((link: any) => selectById("prop", link.prop_id))
      .filter((prop: any): prop is NonNullable<typeof prop> => prop !== null);
    const props_value = (() => {
      const filtered = typeProps
        .map((pr: any) => {
          const propAttributes = selectAll("attribute", { columns: ["prop_id", "prop_guid"], value: pr.id });
          if (!pr.quality_id) return null;
          return {
            id: pr.id,
            value: String(pr.value),
            unit: toUndefined(pr.unit),
            quality: { id: pr.quality_id },
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

  const designs = selectAll("design", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.designs = mapOrUndefined(designs, (row: any) => {
    const designId = row.id || String(row.id);
    const pieces = selectAll("piece", { columns: ["design_id", "design_guid"], value: designId });
    const connections = selectAll("connection", { columns: ["design_id", "design_guid"], value: designId });
    const layers = selectAll("layer", { columns: ["design_id", "design_guid"], value: designId });
    const groups = selectAll("group", { columns: ["design_id", "design_guid"], value: designId });
    const stats = selectAll("stat", { columns: ["design_id", "design_guid"], value: designId });
    const designAttributes = selectAll("attribute", { columns: ["design_id", "design_guid"], value: designId });
    const designConcepts = selectAll("design_concept", { columns: ["design_id", "design_guid"], value: designId });
    const designProps = selectAll("design_prop", { columns: ["design_id", "design_guid"], value: designId });
    const designAuthors = selectAll("design_author", { columns: ["design_id", "design_guid"], value: designId, orderBy: "rank ASC" });

    return {
      id: designId,
      name: row.name,
      description: toUndefined(row.description),
      icon: toUndefined(row.icon),
      image: toUndefined(row.image),
      families: normalizeEntityRefs(toUndefined(row.families)),
      unit: toUndefined(row.unit),
      isAbstract: row.is_abstract ? true : undefined,
      folder: toUndefined(row.folder),
      canScale: row.can_scale ? true : undefined,
      canMirror: row.can_mirror ? true : undefined,
      createdAt: row.created,
      updatedAt: row.updated,
      activeLayer: row.active_layer_id ? { id: row.active_layer_id } : undefined,
      props: mapOrUndefined(designProps, (dp: any) => ({
        id: dp.id,
        quality: { id: dp.quality_id },
        value: String(dp.value),
        unit: toUndefined(dp.unit),
      })),
      authors: mapOrUndefined(designAuthors, (da: any) => ({ id: da.author_id })),
      pieces: pieces.map((p: any) => {
        const pieceProps = safeSelectAll("piece_prop", { columns: ["piece_id", "piece_guid"], value: p.id })
          .map((link: any) => selectById("prop", link.prop_id))
          .filter((prop: any): prop is NonNullable<typeof prop> => prop !== null);
        const pieceAttributes = selectAll("attribute", { columns: ["piece_id", "piece_guid"], value: p.id });
        return {
          id: p.id,
          name: toUndefined(p.name),
          type: p.type_id ? { id: p.type_id } : undefined,
          design: p.design_id_ref ? { id: p.design_id_ref } : undefined,
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
                const propAttributes = execResult("SELECT * FROM attribute WHERE prop_id = ?", [pr.id]);
                if (!pr.quality_id) return null;
                return {
                  id: pr.id,
                  value: String(pr.value),
                  unit: toUndefined(pr.unit),
                  quality: { id: pr.quality_id },
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
        const connectionAttributes = selectAll("attribute", { columns: ["connection_id", "connection_guid"], value: c.id });
        return {
          id: c.id,
          connected: {
            piece: { id: c.connected_piece_id },
            designPiece: c.connected_design_piece_id ? { id: c.connected_design_piece_id } : undefined,
            connector: { id: c.connected_connector_id },
          },
          connecting: {
            piece: { id: c.connecting_piece_id },
            designPiece: c.connecting_design_piece_id ? { id: c.connecting_design_piece_id } : undefined,
            connector: { id: c.connecting_connector_id },
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
        const layerAttributes = selectAll("attribute", { columns: ["layer_id", "layer_guid"], value: l.id });
        return {
          id: l.id,
          path: l.path,
          isHidden: l.is_hidden ? true : undefined,
          isLocked: l.is_locked ? true : undefined,
          color: toUndefined(l.color),
          description: toUndefined(l.description),
          attributes: mapOrUndefined(layerAttributes, buildAttribute),
        };
      }),
      groups: groups.map((g: any) => {
        const groupPieces = selectAll("group_piece", { columns: ["group_id", "group_guid"], value: g.id });
        const groupAttributes = selectAll("attribute", { columns: ["group_id", "group_guid"], value: g.id });
        return {
          id: g.id,
          name: toUndefined(g.name),
          color: toUndefined(g.color),
          description: toUndefined(g.description),
          pieces: groupPieces.map((gp: any) => ({ id: gp.piece_id })),
          attributes: mapOrUndefined(groupAttributes, buildAttribute),
        };
      }),
      stats: stats.map((s: any) => ({
        id: s.id,
        quality: { id: s.quality_id },
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

  kit.families = mapOrUndefined(familyRows, (fRow: any) => {
    const familyAttributes = selectAll("attribute", { columns: ["family_id", "family_guid"], value: fRow.id });
    const ports = selectAll("port", { columns: ["family_id", "family_guid"], value: fRow.id });
    return {
      id: fRow.id,
      name: fRow.name,
      description: toUndefined(fRow.description),
      icon: toUndefined(fRow.icon),
      ports: mapOrUndefined(ports, (row: any) => {
        const compatiblePorts = selectAll("port_compatibility", { columns: ["port_id", "port_guid"], value: row.id });
        const portAttributes = selectAll("attribute", { columns: ["port_id", "port_guid"], value: row.id });
        return {
          id: row.id,
          name: row.name,
          description: toUndefined(row.description),
          icon: toUndefined(row.icon),
          compatiblePorts: compatiblePorts.length > 0 ? compatiblePorts.map((ci: any) => ({ id: ci.compatible_port_id })) : undefined,
          attributes: mapOrUndefined(portAttributes, buildAttribute),
        };
      }),
      attributes: mapOrUndefined(familyAttributes, buildAttribute),
    };
  });

  const tags = safeSelectAll("tag", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.tags = mapOrUndefined(tags, (row: any) => ({
    id: row.id,
    name: row.name,
    description: toUndefined(row.description),
    icon: toUndefined(row.icon),
  }));

  const qualities = selectAll("quality", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.qualities =
    qualities.length > 0
      ? qualities.map((row: any) => {
        const benchmarks = selectAll("benchmark", { columns: ["quality_id", "quality_guid"], value: row.id });
        const qualityAttributes = selectAll("attribute", { columns: ["quality_id", "quality_guid"], value: row.id });
        return {
          id: row.id,
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
            const benchmarkAttributes = selectAll("attribute", { columns: ["benchmark_id", "benchmark_guid"], value: b.id });
            return {
              id: b.id,
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

  const files = selectAll("file", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.files =
    files.length > 0
      ? files.map((row: any) => ({
        id: row.id,
        name: row.name,
        remote: toUndefined(row.remote_url),
        folder: row.folder_id ? { id: row.folder_id } : undefined,
        size: row.size ?? undefined,
        hash: toUndefined(row.hash),
        createdAt: row.created,
        updatedAt: row.updated,
      }))
      : undefined;

  const folders = selectAll("folder", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.folders = mapOrUndefined(folders, (row: any) => ({
    id: row.id,
    name: row.name,
    parent: row.parent_id ? { id: row.parent_id } : undefined,
    createdAt: row.created,
    updatedAt: row.updated,
  }));

  const authors = selectAll("author", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.authors =
    authors.length > 0
      ? authors.map((row: any) => ({
        id: row.id,
        name: row.name,
        email: toUndefined(row.email),
      }))
      : undefined;

  const concepts = selectAll("concept", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.concepts = mapOrUndefined(concepts, (row: any) => ({
    id: row.id,
    name: row.name,
    description: toUndefined(row.description),
    icon: toUndefined(row.icon),
  }));

  const kitAttributes = selectAll("attribute", { columns: ["kit_id", "kit_guid"], value: kit.id });
  kit.attributes = mapOrUndefined(kitAttributes, buildAttribute);

  return asKitInstance(kit);
};
// ­ƒôÜtoArray holds the data fields for a toArray record.
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
	id VARCHAR(36) NOT NULL,
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
	PRIMARY KEY (id)
);

CREATE TABLE quality (
	id VARCHAR(36) NOT NULL,
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
	kit_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE benchmark (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	icon TEXT,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	definition TEXT,
	quality_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(quality_id) REFERENCES quality (id)
);

CREATE TABLE family (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE port (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	family_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(family_id) REFERENCES family (id)
);

CREATE TABLE port_compatibility (
	port_id VARCHAR(36) NOT NULL,
	compatible_port_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (port_id, compatible_port_id),
	FOREIGN KEY(port_id) REFERENCES port (id),
	FOREIGN KEY(compatible_port_id) REFERENCES port (id)
);

CREATE TABLE folder (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_id VARCHAR(36),
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(parent_id) REFERENCES folder (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE file (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	folder_id VARCHAR(36),
	size INTEGER,
	hash VARCHAR(128),
	remote_url TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(folder_id) REFERENCES folder (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE author (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	email VARCHAR(256),
	kit_id VARCHAR(36),
	type_id VARCHAR(36),
	design_id VARCHAR(36),
	PRIMARY KEY (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE tag (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE type (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	families TEXT,
	is_abstract BOOLEAN NOT NULL DEFAULT 0,
	folder VARCHAR(256),
	stock INTEGER,
	virtual BOOLEAN NOT NULL DEFAULT 0,
	unit VARCHAR(64),
	location_id VARCHAR(36),
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_id VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (id, kit_id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE representation (
	id VARCHAR(36) NOT NULL,
	file_id VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	description TEXT,
	type_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(file_id) REFERENCES file (id),
	FOREIGN KEY(type_id) REFERENCES type (id)
);

CREATE TABLE representation_tag (
	representation_id VARCHAR(36) NOT NULL,
	tag_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (representation_id, tag_id),
	FOREIGN KEY(representation_id) REFERENCES representation (id),
	FOREIGN KEY(tag_id) REFERENCES tag (id)
);

CREATE TABLE prop (
	id VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	quality_id VARCHAR(36),
	connector_id VARCHAR(36),
	PRIMARY KEY (id),
	FOREIGN KEY(quality_id) REFERENCES quality (id)
);

CREATE TABLE type_prop (
	type_id VARCHAR(36) NOT NULL,
	prop_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (type_id, prop_id),
	FOREIGN KEY(type_id) REFERENCES type (id),
	FOREIGN KEY(prop_id) REFERENCES prop (id)
);

CREATE TABLE connector (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	point_x FLOAT NOT NULL,
	point_y FLOAT NOT NULL,
	point_z FLOAT NOT NULL,
	direction_x FLOAT NOT NULL,
	direction_y FLOAT NOT NULL,
	direction_z FLOAT NOT NULL,
	t FLOAT NOT NULL,
	mandatory BOOLEAN NOT NULL DEFAULT 0,
	port_id VARCHAR(36),
	description TEXT,
	type_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	UNIQUE (id, type_id),
	FOREIGN KEY(port_id) REFERENCES port (id),
	FOREIGN KEY(type_id) REFERENCES type (id)
);

CREATE TABLE design (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	families TEXT,
	variant VARCHAR(256),
	view_center_u FLOAT,
	view_center_v FLOAT,
	view_zoom FLOAT,
	unit VARCHAR(64),
	location_id VARCHAR(36),
	active_layer_id VARCHAR(36),
	is_abstract BOOLEAN,
	folder VARCHAR(256),
	can_scale BOOLEAN,
	can_mirror BOOLEAN,
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_id VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (id, kit_id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE design_prop (
	id VARCHAR(36) NOT NULL,
	design_id VARCHAR(36) NOT NULL,
	quality_id VARCHAR(36) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	PRIMARY KEY (id),
	FOREIGN KEY(design_id) REFERENCES design (id),
	FOREIGN KEY(quality_id) REFERENCES quality (id)
);

CREATE TABLE design_author (
	design_id VARCHAR(36) NOT NULL,
	author_id VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (design_id, author_id),
	FOREIGN KEY(design_id) REFERENCES design (id),
	FOREIGN KEY(author_id) REFERENCES author (id)
);

CREATE TABLE layer (
	id VARCHAR(36) NOT NULL,
	path VARCHAR(512) NOT NULL,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(design_id) REFERENCES design (id)
);

CREATE TABLE piece (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	type_id VARCHAR(36),
	design_id_ref VARCHAR(36),
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
	design_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(type_id) REFERENCES type (id),
	FOREIGN KEY(design_id_ref) REFERENCES design (id),
	FOREIGN KEY(design_id) REFERENCES design (id)
);

CREATE TABLE piece_prop (
	piece_id VARCHAR(36) NOT NULL,
	prop_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (piece_id, prop_id),
	FOREIGN KEY(piece_id) REFERENCES piece (id),
	FOREIGN KEY(prop_id) REFERENCES prop (id)
);

CREATE TABLE "group" (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	color VARCHAR(32),
	description TEXT,
	design_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(design_id) REFERENCES design (id)
);

CREATE TABLE group_piece (
	group_id VARCHAR(36) NOT NULL,
	piece_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (group_id, piece_id),
	FOREIGN KEY(group_id) REFERENCES "group" (id),
	FOREIGN KEY(piece_id) REFERENCES piece (id)
);

CREATE TABLE connection (
	id VARCHAR(36) NOT NULL,
	connected_piece_id VARCHAR(36) NOT NULL,
	connected_design_piece_id VARCHAR(36),
	connected_connector_id VARCHAR(36) NOT NULL,
	connecting_piece_id VARCHAR(36) NOT NULL,
	connecting_design_piece_id VARCHAR(36),
	connecting_connector_id VARCHAR(36) NOT NULL,
	gap FLOAT NOT NULL DEFAULT 0,
	shift FLOAT NOT NULL DEFAULT 0,
	rise FLOAT NOT NULL DEFAULT 0,
	rotation FLOAT NOT NULL DEFAULT 0,
	turn FLOAT NOT NULL DEFAULT 0,
	tilt FLOAT NOT NULL DEFAULT 0,
	u FLOAT,
	v FLOAT,
	description TEXT,
	design_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	CHECK (connecting_piece_id != connected_piece_id),
	FOREIGN KEY(connected_piece_id) REFERENCES piece (id),
	FOREIGN KEY(connected_connector_id) REFERENCES connector (id),
	FOREIGN KEY(connecting_piece_id) REFERENCES piece (id),
	FOREIGN KEY(connecting_connector_id) REFERENCES connector (id),
	FOREIGN KEY(design_id) REFERENCES design (id)
);

CREATE TABLE stat (
	id VARCHAR(36) NOT NULL,
	quality_id VARCHAR(36) NOT NULL,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	unit VARCHAR(64),
	design_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(quality_id) REFERENCES quality (id),
	FOREIGN KEY(design_id) REFERENCES design (id)
);

CREATE TABLE concept (
	id VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_id VARCHAR(36) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);

CREATE TABLE type_concept (
	type_id VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (type_id, concept)
);

CREATE TABLE type_author (
	type_id VARCHAR(36) NOT NULL,
	author_id VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (type_id, author_id),
	FOREIGN KEY(type_id) REFERENCES type (id),
	FOREIGN KEY(author_id) REFERENCES author (id)
);

CREATE TABLE design_concept (
	design_id VARCHAR(36) NOT NULL,
	concept VARCHAR(256) NOT NULL,
	PRIMARY KEY (design_id, concept)
);

CREATE TABLE attribute (
	id VARCHAR(36) NOT NULL,
	key VARCHAR(256) NOT NULL,
	value TEXT,
	definition TEXT,
	quality_id VARCHAR(36),
	benchmark_id VARCHAR(36),
	family_id VARCHAR(36),
	port_id VARCHAR(36),
	folder_id VARCHAR(36),
	file_id VARCHAR(36),
	author_id VARCHAR(36),
	representation_id VARCHAR(36),
	prop_id VARCHAR(36),
	connector_id VARCHAR(36),
	type_id VARCHAR(36),
	layer_id VARCHAR(36),
	piece_id VARCHAR(36),
	group_id VARCHAR(36),
	connection_id VARCHAR(36),
	stat_id VARCHAR(36),
	design_id VARCHAR(36),
	kit_id VARCHAR(36),
	PRIMARY KEY (id),
	FOREIGN KEY(quality_id) REFERENCES quality (id),
	FOREIGN KEY(benchmark_id) REFERENCES benchmark (id),
	FOREIGN KEY(family_id) REFERENCES family (id),
	FOREIGN KEY(port_id) REFERENCES port (id),
	FOREIGN KEY(folder_id) REFERENCES folder (id),
	FOREIGN KEY(file_id) REFERENCES file (id),
	FOREIGN KEY(author_id) REFERENCES author (id),
	FOREIGN KEY(representation_id) REFERENCES representation (id),
	FOREIGN KEY(prop_id) REFERENCES prop (id),
	FOREIGN KEY(connector_id) REFERENCES connector (id),
	FOREIGN KEY(type_id) REFERENCES type (id),
	FOREIGN KEY(layer_id) REFERENCES layer (id),
	FOREIGN KEY(piece_id) REFERENCES piece (id),
	FOREIGN KEY(group_id) REFERENCES "group" (id),
	FOREIGN KEY(connection_id) REFERENCES connection (id),
	FOREIGN KEY(stat_id) REFERENCES stat (id),
	FOREIGN KEY(design_id) REFERENCES design (id),
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
`;

// ­ƒôªkitToSqlite converts a kit object into a SQLite database.
export const kitToSqlite = async (kit: KitImpl, db: any): Promise<void> => {
  db.exec(KIT_SQLITE_SCHEMA);

  const toISOString = (date: Date | string | undefined): string => {
    if (!date) return new Date().toISOString();
    if (typeof date === "string") return date;
    return date.toISOString();
  };

  db.run("INSERT INTO semio (release, engine, created) VALUES (?, ?, ?)", ["1.0.0", "js", new Date().toISOString()]);

  db.run("INSERT INTO kit (id, name, version, description, icon, image, preview, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
    kit.id,
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
      db.run("INSERT INTO concept (id, name, description, icon, kit_id) VALUES (?, ?, ?, ?, ?)", [concept.id, concept.name, concept.description || null, concept.icon || null, kit.id]);
    } else {
      db.run("INSERT INTO concept (id, name, kit_id) VALUES (?, ?, ?)", [id(), concept, kit.id]);
    }
  });

  toArray(kit.attributes).forEach((attr) => {
    db.run("INSERT INTO attribute (id, key, value, definition, kit_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, kit.id]);
  });

  toArray(kit.families).forEach((family) => {
    db.run("INSERT INTO family (id, name, description, icon, kit_id) VALUES (?, ?, ?, ?, ?)", [family.id, family.name, family.description || null, family.icon || null, kit.id]);

    toArray(family.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (id, key, value, definition, family_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, family.id]);
    });

    toArray(family.ports).forEach((iface) => {
      db.run("INSERT INTO port (id, name, description, icon, family_id) VALUES (?, ?, ?, ?, ?)", [iface.id, iface.name, iface.description || null, iface.icon || null, family.id]);

      toArray(iface.compatiblePorts).forEach((compat) => {
        db.run("INSERT INTO port_compatibility (port_id, compatible_port_id) VALUES (?, ?)", [iface.id, compat.id]);
      });

      toArray(iface.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, port_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, iface.id]);
      });
    });
  });

  toArray(kit.qualities).forEach((quality) => {
    db.run(
      "INSERT INTO quality (id, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      [
        quality.id,
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
        kit.id,
      ],
    );

    toArray(quality.benchmarks).forEach((benchmark) => {
      db.run("INSERT INTO benchmark (id, name, icon, min_value, min_excluded, max_value, max_excluded, quality_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
        benchmark.id,
        benchmark.name,
        benchmark.icon || null,
        benchmark.min || null,
        benchmark.minExcluded ? 1 : null,
        benchmark.max || null,
        benchmark.maxExcluded ? 1 : null,
        quality.id,
      ]);

      toArray(benchmark.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, benchmark_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, benchmark.id]);
      });
    });

    toArray(quality.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (id, key, value, definition, quality_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, quality.id]);
    });
  });

  toArray(kit.folders).forEach((folder) => {
    db.run("INSERT INTO folder (id, name, parent_id, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?)", [folder.id, folder.name, folder.parent?.id || null, toISOString(folder.createdAt), toISOString(folder.updatedAt), kit.id]);
  });

  toArray(kit.files).forEach((file) => {
    db.run("INSERT INTO file (id, name, folder_id, size, hash, remote_url, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      file.id,
      file.name,
      file.folder?.id || null,
      file.size || null,
      file.hash || null,
      file.remote || null,
      toISOString(file.createdAt),
      toISOString(file.updatedAt),
      kit.id,
    ]);
  });

  toArray(kit.authors).forEach((author) => {
    db.run("INSERT INTO author (id, name, email, kit_id) VALUES (?, ?, ?, ?)", [author.id, author.name, author.email || null, kit.id]);
  });

  toArray(kit.tags).forEach((tag) => {
    db.run("INSERT INTO tag (id, name, description, icon, kit_id) VALUES (?, ?, ?, ?, ?)", [tag.id, tag.name, tag.description || null, tag.icon || null, kit.id]);
  });

  toArray(kit.types).forEach((type) => {
    db.run("INSERT INTO type (id, name, families, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      type.id,
      type.name,
      type.families && type.families.length > 0 ? JSON.stringify(type.families) : null,
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
      kit.id,
    ]);

    toArray(type.concepts).forEach((concept) => {
      db.run("INSERT INTO type_concept (type_id, concept) VALUES (?, ?)", [type.id, concept]);
    });

    toArray(type.authors).forEach((authorId, index) => {
      db.run("INSERT INTO type_author (type_id, author_id, rank) VALUES (?, ?, ?)", [type.id, typeof authorId === "object" ? authorId.id : authorId, index]);
    });

    toArray(type.representations).forEach((representation) => {
      db.run("INSERT INTO representation (id, file_id, name, description, type_id) VALUES (?, ?, ?, ?, ?)", [representation.id, representation.file.id, representation.name || null, representation.description || null, type.id]);

      toArray(representation.tags).forEach((tag) => {
        db.run("INSERT INTO representation_tag (representation_id, tag_id) VALUES (?, ?)", [representation.id, typeof tag === "object" ? tag.id : tag]);
      });

      toArray(representation.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, representation_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, representation.id]);
      });
    });

    toArray(type.connectors).forEach((connector) => {
      db.run("INSERT INTO connector (id, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_id, description, type_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
        connector.id,
        connector.name || null,
        connector.point.x,
        connector.point.y,
        connector.point.z,
        connector.direction.x,
        connector.direction.y,
        connector.direction.z,
        connector.t,
        connector.mandatory ? 1 : 0,
        connector.port?.id || null,
        connector.description || null,
        type.id,
      ]);

      toArray(connector.props).forEach((prop) => {
        const quality = toArray(kit.qualities).find((q) => q.id === prop.quality.id);
        const propKey = quality?.key || "";
        db.run("INSERT INTO prop (id, key, value, unit, quality_id, connector_id) VALUES (?, ?, ?, ?, ?, ?)", [prop.id, propKey, prop.value, prop.unit || null, prop.quality.id, connector.id]);
        toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (id, key, value, definition, prop_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, prop.id]);
        });
      });

      toArray(connector.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, connector_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, connector.id]);
      });
    });

    toArray(type.props).forEach((prop) => {
      const quality = toArray(kit.qualities).find((q) => q.id === prop.quality.id);
      const propKey = quality?.key || "";
      db.run("INSERT INTO prop (id, key, value, unit, quality_id) VALUES (?, ?, ?, ?, ?)", [prop.id, propKey, prop.value, prop.unit || null, prop.quality.id]);
      db.run("INSERT INTO type_prop (type_id, prop_id) VALUES (?, ?)", [type.id, prop.id]);
      toArray(prop.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, prop_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, prop.id]);
      });
    });

    toArray(type.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (id, key, value, definition, type_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, type.id]);
    });
  });

  toArray(kit.designs).forEach((design) => {
    db.run("INSERT INTO design (id, name, families, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [
      design.id,
      design.name,
      design.families && design.families.length > 0 ? JSON.stringify(design.families) : null,
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
      kit.id,
    ]);

    toArray(design.concepts).forEach((concept) => {
      db.run("INSERT INTO design_concept (design_id, concept) VALUES (?, ?)", [design.id, concept]);
    });

    toArray(design.props).forEach((prop) => {
      db.run("INSERT INTO design_prop (id, design_id, quality_id, value, unit) VALUES (?, ?, ?, ?, ?)", [prop.id, design.id, prop.quality.id, parseFloat(prop.value), prop.unit || null]);
    });

    toArray(design.authors).forEach((authorId, index) => {
      db.run("INSERT INTO design_author (design_id, author_id, rank) VALUES (?, ?, ?)", [design.id, typeof authorId === "object" ? authorId.id : authorId, index]);
    });

    toArray(design.layers).forEach((layer) => {
      db.run("INSERT INTO layer (id, path, is_hidden, is_locked, color, description, design_id) VALUES (?, ?, ?, ?, ?, ?, ?)", [
        layer.id,
        layer.path,
        layer.isHidden ? 1 : 0,
        layer.isLocked ? 1 : 0,
        layer.color || null,
        layer.description || null,
        design.id,
      ]);

      toArray(layer.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, layer_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, layer.id]);
      });
    });

    toArray(design.pieces).forEach((piece) => {
      db.run(
        "INSERT INTO piece (id, name, type_id, design_id_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z, mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z, mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z, is_hidden, is_locked, color, description, design_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          piece.id,
          piece.name || null,
          piece.type?.id || null,
          piece.design?.id || null,
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
          design.id,
        ],
      );

      toArray(piece.props).forEach((prop) => {
        const quality = toArray(kit.qualities).find((q) => q.id === prop.quality.id);
        const propKey = quality?.key || "";
        db.run("INSERT INTO prop (id, key, value, unit, quality_id) VALUES (?, ?, ?, ?, ?)", [prop.id, propKey, prop.value, prop.unit || null, prop.quality.id]);
        db.run("INSERT INTO piece_prop (piece_id, prop_id) VALUES (?, ?)", [piece.id, prop.id]);
        toArray(prop.attributes).forEach((attr) => {
          db.run("INSERT INTO attribute (id, key, value, definition, prop_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, prop.id]);
        });
      });

      toArray(piece.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, piece_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, piece.id]);
      });
    });

    toArray(design.groups).forEach((group) => {
      db.run('INSERT INTO "group" (id, name, color, description, design_id) VALUES (?, ?, ?, ?, ?)', [group.id, group.name || null, group.color || null, group.description || null, design.id]);

      toArray(group.pieces).forEach((piece) => {
        db.run("INSERT INTO group_piece (group_id, piece_id) VALUES (?, ?)", [group.id, piece.id]);
      });

      toArray(group.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, group_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, group.id]);
      });
    });

    toArray(design._connections).forEach((connection) => {
      if (!connection.id || !connection.connected?.piece?.id || !connection.connecting?.piece?.id || !connection.connected?.connector?.id || !connection.connecting?.connector?.id) {
        return;
      }

      db.run(
        "INSERT INTO connection (id, connected_piece_id, connected_design_piece_id, connected_connector_id, connecting_piece_id, connecting_design_piece_id, connecting_connector_id, gap, shift, rise, rotation, turn, tilt, u, v, description, design_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
          connection.id,
          connection.connected.piece.id,
          connection.connected.designPiece?.id || null,
          connection.connected.connector.id,
          connection.connecting.piece.id,
          connection.connecting.designPiece?.id || null,
          connection.connecting.connector.id,
          connection.gap || 0,
          connection.shift || 0,
          connection.rise || 0,
          connection.rotation || 0,
          connection.turn || 0,
          connection.tilt || 0,
          connection.u !== undefined ? connection.u : null,
          connection.v !== undefined ? connection.v : null,
          connection.description || null,
          design.id,
        ],
      );

      toArray(connection.attributes).forEach((attr) => {
        db.run("INSERT INTO attribute (id, key, value, definition, connection_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, connection.id]);
      });
    });

    toArray(design.stats).forEach((stat) => {
      db.run("INSERT INTO stat (id, quality_id, min_value, min_excluded, max_value, max_excluded, unit, design_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", [
        stat.id,
        stat.quality.id,
        stat.min || null,
        stat.minExcluded ? 1 : null,
        stat.max || null,
        stat.maxExcluded ? 1 : null,
        stat.unit || null,
        design.id,
      ]);
    });

    toArray(design.attributes).forEach((attr) => {
      db.run("INSERT INTO attribute (id, key, value, definition, design_id) VALUES (?, ?, ?, ?, ?)", [attr.id, attr.key, attr.value || null, attr.definition || null, design.id]);
    });
  });
};

// #endregion ­ƒº┐KitImpl Import/Export

// #region ­ƒö®KitImpl Representation Export
// Design representation export to 3D formats (GLB, glTF, OBJ, STL, PLY, USDZ) MUST be defined here.

/**
 * Supported 3D export formats with their MIME types.
 **/
export const EXPORT_REPRESENTATION_FORMATS: Record<string, string> = {
  ".glb": "representation/gltf-binary",
  ".gltf": "representation/gltf+json",
  ".obj": "representation/obj",
  ".stl": "representation/stl",
  ".ply": "application/x-ply",
  ".usdz": "representation/vnd.usdz+zip",
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
const findMatchingRepresentation = (kit: KitImpl, type: Type, tags: string[]): Representation | undefined => {
  if (!type.representations || type.representations.length === 0) return undefined;
  const kitTags = kit.tags ?? [];
  const selectedTagIds = tags.flatMap((tagValue) => {
    const byId = kitTags.find((tag) => tag.id === tagValue);
    if (byId) return [byId.id];
    return kitTags.filter((tag) => tag.name === tagValue).map((tag) => tag.id);
  });
  return selectBestRepresentation(type.representations, selectedTagIds);
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
 * Creates a unit box mesh (1x1x1 centered at origin) as a placeholder for types without representations.
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
 * Exports the 3D representation of a design to a specified format.
 * Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
 **/
export const exportDesignRepresentation = async (kit: KitImpl, designId: string, format: string = ".glb", tags: string[] = [], options: Record<string, unknown> = {}): Promise<ArrayBuffer> => {
  const io = new NodeIO();
  const design = kit.requireDesign(designId);
  const pieces = design.pieces ?? [];
  const connections = design._connections ?? [];
  const types = kit.types ?? [];

  if (pieces.length === 0) {
    const emptyDoc = new GltfDocument();
    emptyDoc.createBuffer("main");
    emptyDoc.createScene("empty");
    const glb = await io.writeBinary(emptyDoc);
    return glb.buffer as ArrayBuffer;
  }

  const typesDict: Record<string, Type> = {};
  for (const t of types) typesDict[t.id] = t;
  const piecesDict: Record<string, Piece> = {};
  for (const p of pieces) piecesDict[p.id] = p;

  const adjacency: Record<string, Array<{ connection: Connection; neighborId: string }>> = {};
  for (const p of pieces) adjacency[p.id] = [];
  for (const conn of connections) {
    const connectedId = conn.connected.piece.id;
    const connectingId = conn.connecting.piece.id;
    if (adjacency[connectedId]) adjacency[connectedId].push({ connection: conn, neighborId: connectingId });
    if (adjacency[connectingId]) adjacency[connectingId].push({ connection: conn, neighborId: connectedId });
  }

  const piecePlanes: Record<string, Plane> = {};
  const parentOf: Record<string, string> = {};
  const childrenOf: Record<string, string[]> = {};
  for (const p of pieces) childrenOf[p.id] = [];

  const visited = new Set<string>();
  const roots: string[] = [];

  const getType = (typeId: string): Type | undefined => typesDict[typeId];
  const getConnector = (type: Type | undefined, connectorId: string | undefined): Connector | undefined => {
    if (!type) return undefined;
    if (!connectorId) return type.connectors?.[0];
    return type.connectors?.find((c) => c.id === connectorId);
  };

  const queue: string[] = [];
  for (const p of pieces) {
    if (p.plane) {
      piecePlanes[p.id] = p.plane;
      visited.add(p.id);
      queue.push(p.id);
      roots.push(p.id);
    }
  }
  if (queue.length === 0 && pieces.length > 0) {
    const firstPiece = pieces[0];
    const identityPlane = matrixToPlane(new THREE.Matrix4().identity());
    piecePlanes[firstPiece.id] = identityPlane;
    visited.add(firstPiece.id);
    queue.push(firstPiece.id);
    roots.push(firstPiece.id);
  }

  while (queue.length > 0) {
    const currentId = queue.shift()!;
    const currentPlane = piecePlanes[currentId];
    for (const edge of adjacency[currentId] ?? []) {
      if (visited.has(edge.neighborId)) continue;
      const conn = edge.connection;
      const isParent = conn.connected.piece.id === currentId;

      if (!isParent) continue;

      const parentId = currentId;
      const childId = edge.neighborId;
      const parentPiece = piecesDict[parentId];
      const childPiece = piecesDict[childId];
      const parentType = resolvePieceTypeForFlatten(parentPiece, getType);
      const childType = resolvePieceTypeForFlatten(childPiece, getType);
      const parentConnector = getConnector(parentType, conn.connected.connector?.id);
      const childConnector = getConnector(childType, conn.connecting.connector?.id);

      if (parentConnector && childConnector) {
        const childPlane = computeChildPlane(currentPlane, parentConnector, childConnector, conn);
        piecePlanes[childId] = childPlane;
      } else {
        piecePlanes[childId] = currentPlane;
      }

      parentOf[childId] = parentId;
      childrenOf[parentId].push(childId);
      visited.add(childId);
      queue.push(childId);
    }
  }

  for (const p of pieces) {
    if (!visited.has(p.id)) {
      piecePlanes[p.id] = matrixToPlane(new THREE.Matrix4().identity());
      roots.push(p.id);
    }
  }

  const doc = new GltfDocument();
  const buffer = doc.createBuffer("main");
  const scene = doc.createScene(design.name ?? "design");

  const typeMeshMap: Record<string, GltfMesh> = {};

  for (const piece of pieces) {
    const typeId = piece.type?.id;
    if (!typeId || typeMeshMap[typeId] !== undefined) continue;

    const type = typesDict[typeId];
    if (!type) continue;

    const representation = findMatchingRepresentation(kit, type, tags);
    if (!representation) {
      continue;
    }

    const file = kit.files?.find((f) => f.id === representation.file.id);
    if (!file?.blob) continue;

    const fileBytes = decodeBlobToBytes(file.blob);
    const ext = file.name.split(".").pop()?.toLowerCase();

    if (ext === "glb") {
      try {
        const sourceDoc = await io.readBinary(fileBytes);
        const copiedMeshes = copyGltfMeshes(sourceDoc, doc, buffer, file.name);
        if (copiedMeshes.length > 0) {
          typeMeshMap[typeId] = copiedMeshes[0];
        }
      } catch { }
    }
  }

  const pieceNodeMap: Record<string, GltfNode> = {};

  const buildNode = (pieceId: string): GltfNode => {
    if (pieceNodeMap[pieceId]) return pieceNodeMap[pieceId];

    const piece = piecesDict[pieceId];
    const worldPlane = piecePlanes[pieceId];
    const parentId = parentOf[pieceId];
    const children = childrenOf[pieceId] ?? [];

    let localMatrix: number[];
    if (parentId && piecePlanes[parentId]) {
      const parentWorld = planeToMatrix(piecePlanes[parentId]);
      const childWorld = planeToMatrix(worldPlane);
      const invParent = parentWorld.clone().invert();
      const localMat = new THREE.Matrix4().multiplyMatrices(invParent, childWorld);
      localMatrix = semioMatrixToGltfMatrix(localMat);
    } else {
      localMatrix = planeToGlbTransform(worldPlane);
    }

    const node = doc.createNode(piece.name ?? piece.id);
    node.setMatrix(localMatrix as any);

    const typeId = piece.type?.id;
    if (typeId && typeMeshMap[typeId]) {
      node.setMesh(typeMeshMap[typeId]);
    }

    for (const childId of children) {
      node.addChild(buildNode(childId));
    }

    pieceNodeMap[pieceId] = node;
    return node;
  };

  for (const rootId of roots) {
    scene.addChild(buildNode(rootId));
  }

  if (format === ".gltf") {
    const jsonDoc = await io.writeJSON(doc);
    const encoder = new TextEncoder();
    return encoder.encode(JSON.stringify(inlineJsonDocumentResources(jsonDoc))).buffer as ArrayBuffer;
  }

  const glb = await io.writeBinary(doc);
  return glb.buffer as ArrayBuffer;
};

// #endregion ­ƒö®KitImpl Representation Export

// #region ÔØä´©ÅGeometric Insights
// Key performance indicators for GLB/GLTF representation geometry. Representation MUST be glb/gltf.

/**
 * ­ƒöÀGeometric KPIs for a GLB/GLTF representation in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).
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
 * ­ƒôïComputes key performance indicators for the geometry of a GLB/GLTF representation.
 */
export const getGeometricInsightsForRepresentation = async (representation: string | ArrayBuffer | Uint8Array): Promise<GeometricInsights> => {
  const io = new NodeIO();
  let doc: GltfDocument;

  if (typeof representation === "string") {
    if (representation.startsWith("data:")) {
      const base64 = representation.slice(representation.indexOf(",") + 1);
      const binary = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
      doc = await io.readBinary(binary);
    } else {
      let arrBuf: ArrayBuffer;
      const isPath = !representation.startsWith("http://") && !representation.startsWith("https://") && (representation.endsWith(".glb") || representation.endsWith(".gltf") || representation.includes("/") || representation.includes("\\"));
      if (typeof globalThis !== "undefined" && "process" in globalThis && typeof (globalThis as any).process?.versions?.node === "string" && isPath) {
        const { readFileSync } = await import("node:fs");
        const { dirname, join } = await import("node:path");
        const dir = dirname(representation);
        if (representation.endsWith(".gltf")) {
          const raw = readFileSync(representation, "utf8");
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
          const buf = readFileSync(representation);
          arrBuf = buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength);
          doc = await io.readBinary(new Uint8Array(arrBuf));
        }
      } else {
        const res = await fetch(representation);
        if (!res.ok) throw new Error(`Failed to load representation: ${res.statusText}`);
        arrBuf = await res.arrayBuffer();
        const bytes = new Uint8Array(arrBuf);
        const isGlb = representation.endsWith(".glb") || (bytes.length >= 4 && new TextDecoder().decode(bytes.slice(0, 4)) === "glTF");
        const base = representation.replace(/\/[^/]*$/, "") || ".";
        doc = isGlb ? await io.readBinary(new Uint8Array(arrBuf)) : await io.readJSON({ json: JSON.parse(new TextDecoder().decode(new Uint8Array(arrBuf))), resources: {} });
      }
    }
  } else {
    const bytes = representation instanceof Uint8Array ? representation : new Uint8Array(representation);
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

// #endregion ÔØä´©ÅGeometric Insights

// #region ­ƒÅ░KitStore
// Storage-agnostic kit store contracts MUST be defined here.
// These interfaces express what a kit store DOES, not how a specific engine stores data.
// No engine-specific primitives (map/array/doc) may appear in these contracts.

// Specs: KitStoreStatus represents the lifecycle states of a kit store.
// Providers transition through states: idle ÔåÆ loading ÔåÆ ready ÔåÆ saving/syncing ÔåÆ ready.
// Error and offline are terminal-ish states that require external resolution.

/**
 * Lifecycle status of a kit store.
 *
 * idle ÔåÆ loading ÔåÆ ready. saving/syncing are transient states
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
 * KitImpl object. sync describes the current synchronization state.
 **/
export type KitStoreSnapshot = {
  kit: KitImpl;
  sync: KitSyncState;
};

/**
 * Storage-agnostic kit store contract.
 *
 * Specs: This is the boundary between the editor and storage backends.
 * semio/sketchpad depends ONLY on this interface ÔÇö never on provider internals.
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
  replace(next: KitImpl, meta?: { origin?: string }): void;

  save(): Promise<void>;
  reload(): Promise<void>;
  dispose(): Promise<void> | void;
}

/**
 * KitImpl store with undo/redo capability.
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

// #region 🌐 KitStorePipeline
// Worker-hosted WASM kit store client, structured [`SetResult`] / [`WriteStatus`] types, and JSON fallback for Node/tests.

/** Wire-format rejection kinds from Rust [`SetError`]. */
export type SetErrorKind =
  | "IllegalName"
  | "NameTooLong"
  | "InvalidUrl"
  | "InvalidValue"
  | "DuplicateId"
  | "NotFound"
  | "CyclicReference"
  | "PortFamilyMismatch"
  | "Readonly"
  | "Disposed"
  | "Timeout"
  | "LockPoisoned"
  | "Internal"
  | "NotSupported";

export type SetError = {
  kind: SetErrorKind;
  message: string;
  field?: string;
  entity?: { kind: string; id: string };
};

export type SetResult = { ok: true } | { ok: false; error: SetError };

/** Result of [`KitStoreHandle.execute`] / [`KitStoreCommand`] (success payload is the serde `KitStoreCommandResult`). */
export type KitStoreExecuteResult = { ok: true; result: unknown } | { ok: false; error: SetError };

/**
 * JSON shape for [`semio::kit_backbone_wire::BackboneConfig`] (externally tagged, camelCase variant keys).
 * Pass as `config` inside `{ attachBackbone: { config } }`.
 */
export type KitStoreWireBackboneConfig =
  | { dev: { path: string } }
  | { local: { folder: string } }
  | { remote: { url: string; sessionId: string } };

/** JSON shape for [`semio::kit_backbone_wire::ConflictResolution`] (unit variants use `null` payload like `newSession`). */
export type KitStoreWireConflictResolution = { dropWip: null } | { forceOverwriteBackbone: null };

/** Payload inside `KitStoreCommandResult::BackboneStatus` (`tip` is checkpoint id when present). */
export type KitStoreWireBackboneStatus = {
  attached: boolean;
  kind?: string | null;
  tip?: string | null;
};

/** Row from `KitStoreCommandResult::ListConflicts` (`items` entry). */
export type KitStoreWireKitConflict = {
  id: string;
  wipCheckpoint: unknown;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
};

function parseKitStoreBackboneStatusResult(raw: unknown): KitStoreWireBackboneStatus {
  if (raw == null || typeof raw !== "object") throw new Error("backboneStatus: unexpected result");
  const o = raw as Record<string, unknown>;
  const inner = o.backboneStatus as Record<string, unknown> | undefined;
  if (!inner || typeof inner !== "object") throw new Error("backboneStatus: missing backboneStatus field");
  return {
    attached: Boolean(inner.attached),
    kind: inner.kind != null ? String(inner.kind) : null,
    tip: inner.tip != null && inner.tip !== "" ? String(inner.tip) : null,
  };
}

function parseKitStoreListConflictsResult(raw: unknown): KitStoreWireKitConflict[] {
  if (raw == null || typeof raw !== "object") throw new Error("listConflicts: unexpected result");
  const o = raw as Record<string, unknown>;
  const inner = o.listConflicts as { items?: unknown[] } | undefined;
  if (!inner || !Array.isArray(inner.items)) throw new Error("listConflicts: missing listConflicts.items");
  return inner.items.map((row) => {
    if (row == null || typeof row !== "object") throw new Error("listConflicts: invalid row");
    const r = row as Record<string, unknown>;
    return {
      id: String(r.id ?? ""),
      wipCheckpoint: r.wipCheckpoint,
      backboneTip: r.backboneTip != null ? String(r.backboneTip) : null,
      reason: String(r.reason ?? ""),
      createdAt: String(r.createdAt ?? ""),
    };
  });
}

export type WriteStatus =
  | { kind: "idle"; pending: 0; lastError?: undefined }
  | { kind: "pending"; pending: number; lastError?: SetError }
  | { kind: "error"; pending: 0; lastError: SetError }
  | { kind: "readonly"; pending: 0 };

export type HookTriad<T> = readonly [
  T,
  (next: T | ((prev: T) => T)) => Promise<SetResult>,
  WriteStatus,
];

export function normalizeRustSetError(raw: any): SetError {
  if (!raw || typeof raw !== "object") {
    return { kind: "Internal", message: "invalid error payload" };
  }
  const kind = typeof raw.kind === "string" ? (raw.kind as SetErrorKind) : "Internal";
  const message = typeof raw.message === "string" ? raw.message : JSON.stringify(raw);
  return { kind, message };
}

/** 🪤 Normalize wasm-thrown kit command errors (strings from `JsValue::from_str`) into [`SetError`]. */
export function normalizeWasmThrownKitError(err: unknown): SetError {
  const message = String(err).replace(/^Error:\s*/, "").trim();
  const lower = message.toLowerCase();
  if (lower.includes("illegal name") || lower.includes("cannot be empty")) {
    return { kind: "IllegalName", message };
  }
  if (lower.includes("name too long") || (lower.includes("exceeds") && lower.includes("char"))) {
    return { kind: "NameTooLong", message };
  }
  return { kind: "Internal", message };
}

export function settleSetPromise(p: Promise<unknown>): Promise<SetResult> {
  return p.then((v: any) => {
    if (v && typeof v === "object" && v.ok === true) return { ok: true } as const;
    if (v && typeof v === "object" && v.ok === false && v.error) {
      return { ok: false, error: normalizeRustSetError(v.error) } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "unexpected setField result" } } as const;
  });
}

/** Boundary contract consumed by [`@semio/react`] and sketchpad. */
export interface KitStoreClient {
  getDto(): any;
  getSnapshot(): Promise<any>;
  setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult>;
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult>;
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult>;
  applyDesignDiff(designId: string, diff: unknown): Promise<SetResult>;
  applyKitDiff(diff: unknown): Promise<SetResult>;
  clusterPieces(designId: string, pieceIds: string[], clusterName: string): Promise<SetResult>;
  dragPieces(designId: string, pieceIds: string[], du: number, dv: number): Promise<SetResult>;
  movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number): Promise<SetResult>;
  fixPieces(designId: string, pieceIds: string[]): Promise<SetResult>;
  flattenDesign(designId: string): Promise<SetResult>;
  expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult>;
  deleteConnection(designId: string, connectionId: string): Promise<SetResult>;
  changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult>;
  pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult>;
  createHangingPieces(designId: string, typeIds: string[], plane: unknown): Promise<SetResult>;
  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult>;
  createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult>;
  getPiecesMetadata(designId: string): Promise<any>;
  getPieces(designId: string): Promise<any>;
  getConnections(designId: string): Promise<any>;
  getDesigns(): Promise<any>;
  getTypes(): Promise<any>;
  getAuthors(): Promise<any>;
  getKitMetadata(): Promise<any>;
  undo(): Promise<SetResult>;
  redo(): Promise<SetResult>;
  canUndo(): Promise<boolean>;
  canRedo(): Promise<boolean>;
  subscribe(cb: (ev: any) => void): () => void;
  dispose(): void;

  execute(cmd: unknown): Promise<KitStoreExecuteResult>;
  /** Field `ReadKitCommand` batch via the same GraphQL `execute` stream as `kitGraphqlExecuteRead`. */
  executeRead(commands: ReadCommandBatch): Promise<ReadCommandBatchResult>;
  /** GraphQL `execute` stream handle (WASM `KitStoreHandle::execute`); all live reads use this. */
  kitGraphql(): KitGraphqlHandle;
  vcsState(): Promise<any>;
  theKitDto(): Promise<any>;
  materializeAt(id: string): Promise<any>;
  attachBackbone(cfg: KitStoreWireBackboneConfig): Promise<SetResult>;
  detachBackbone(): Promise<SetResult>;
  backboneStatus(): Promise<KitStoreWireBackboneStatus>;
  listConflicts(): Promise<KitStoreWireKitConflict[]>;
  resolveConflict(id: string, strategy: KitStoreWireConflictResolution): Promise<SetResult>;
  syncNow(): Promise<SetResult>;
}

export type CreateKitStoreClientOptions = {
  initialKit: KitLike;
  /** Vite/consumer should `resolve.alias` this to the wasm-bindgen `*.js` entry. */
  wasmSpecifier?: string;
  timeoutMs?: number;
  /** Use in-process JSON mirror (no Worker). Defaults to true when [`Worker`] is undefined. */
  forceFallback?: boolean;
  workerFactory?: () => Worker;
};

const KIT_NAME_MAX = 512;

function validateRequiredName(raw: string, label: string): SetResult {
  const t = raw.trim();
  if (!t) return { ok: false, error: { kind: "IllegalName", message: `${label} cannot be empty` } };
  if (t.length > KIT_NAME_MAX) return { ok: false, error: { kind: "NameTooLong", message: `${label} exceeds ${KIT_NAME_MAX} chars` } };
  return { ok: true } as const;
}

function validateOptionalDisplayName(name: string | null | undefined, label: string): SetResult {
  if (name == null) return { ok: true } as const;
  const t = String(name).trim();
  if (!t) return { ok: false, error: { kind: "IllegalName", message: `${label} cannot be empty` } };
  if (t.length > KIT_NAME_MAX) return { ok: false, error: { kind: "NameTooLong", message: `${label} exceeds ${KIT_NAME_MAX} chars` } };
  return { ok: true } as const;
}

/** In-process mirror of a subset of [`KitStore::set_field_rpc`] for Node/tests. */
export class FallbackKitStoreClient implements KitStoreClient {
  private handle: any;
  private listeners: Set<(ev: any) => void> = new Set();
  private cached: any;
  private timeoutMs: number;
  private subscribed = false;
  private gqlUnsub: (() => void) | undefined;

  constructor(handle: any, cachedDto: any, timeoutMs: number) {
    this.handle = handle;
    this.cached = cachedDto;
    this.timeoutMs = timeoutMs;
  }

  private gql(): KitGraphqlHandle {
    return {
      execute: (requestJson: string, onMessage: (line: string) => void) => this.handle.execute(requestJson, onMessage),
    };
  }

  kitGraphql(): KitGraphqlHandle {
    return this.gql();
  }

  getDto() {
    return this.cached;
  }

  async getSnapshot() {
    try {
      this.cached = await withTimeout(Promise.resolve(this.handle.snapshot()), this.timeoutMs, "snapshot timeout");
    } catch {
      /* keep cached */
    }
    return this.cached;
  }

  subscribe(cb: (ev: any) => void): () => void {
    this.listeners.add(cb);
    if (!this.subscribed) {
      this.subscribed = true;
      this.gqlUnsub = kitGraphqlSubscribeLoop(this.gql(), (ev: any) => {
        for (const listener of this.listeners) {
          try {
            listener(ev);
          } catch {
            /* ignore */
          }
        }
      });
    }
    return () => {
      this.listeners.delete(cb);
      if (this.listeners.size === 0) {
        this.gqlUnsub?.();
        this.gqlUnsub = undefined;
        this.subscribed = false;
      }
    };
  }

  dispose() {
    this.listeners.clear();
    if (typeof this.handle?.free === "function") {
      try {
        this.handle.free();
      } catch {
        /* ignore */
      }
    }
  }

  private async settleMutateAndRefresh(raw: Promise<unknown> | unknown): Promise<SetResult> {
    try {
      const got = await withTimeout(Promise.resolve(raw), this.timeoutMs, "timeout");
      const result = await settleSetPromise(Promise.resolve(got));
      if (result.ok) {
        await this.getSnapshot();
      }
      return result;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      (async () => {
        try {
          const cmds = this.handle.changeKitCommandsForFieldPatch(kind, id, field, value);
          await this.handle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: normalizeWasmThrownKitError(e) };
        }
      })(),
    );
  }

  async addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      (async () => {
        try {
          const cmds = this.handle.changeKitCommandsForAddChild(parentKind, parentId, childKind, dto);
          await this.handle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: normalizeWasmThrownKitError(e) };
        }
      })(),
    );
  }

  async removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      (async () => {
        try {
          const cmds = this.handle.changeKitCommandsForRemoveChild(parentKind, parentId, childKind, childId);
          await this.handle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: normalizeWasmThrownKitError(e) };
        }
      })(),
    );
  }

  async applyDesignDiff(designId: string, diff: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.applyDesignDiff(designId, diff));
  }

  async applyKitDiff(diff: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.applyKitDiff(diff));
  }

  async clusterPieces(designId: string, pieceIds: string[], clusterName: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.clusterPieces(designId, pieceIds, clusterName));
  }

  async dragPieces(designId: string, pieceIds: string[], du: number, dv: number): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.dragPieces(designId, pieceIds, du, dv));
  }

  async movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.movePieces(designId, pieceIds, gap, shift, rise));
  }

  async fixPieces(designId: string, pieceIds: string[]): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.fixPieces(designId, pieceIds));
  }

  async flattenDesign(designId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.flattenDesign(designId));
  }

  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.expandDesign(parentDesignId, nestedDesignId));
  }

  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.deleteConnection(designId, connectionId));
  }

  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.changePieceType(designId, pieceId, newTypeId));
  }

  async pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.pasteDesignSelection(designId, selection, plane));
  }

  async createHangingPieces(designId: string, typeIds: string[], plane: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.createHangingPieces(designId, typeIds, plane));
  }

  async createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort));
  }

  async createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.createFixedPiece(designId, typeId, plane));
  }

  async undo(): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.undo());
  }

  async redo(): Promise<SetResult> {
    return this.settleMutateAndRefresh(this.handle.redo());
  }

  async canUndo(): Promise<boolean> {
    try {
      return Boolean(await withTimeout(Promise.resolve(this.handle.canUndo()), this.timeoutMs, "timeout"));
    } catch {
      return false;
    }
  }

  async canRedo(): Promise<boolean> {
    try {
      return Boolean(await withTimeout(Promise.resolve(this.handle.canRedo()), this.timeoutMs, "timeout"));
    } catch {
      return false;
    }
  }

  private unwrapQuery(raw: any) {
    if (raw && typeof raw === "object" && raw.ok === false && raw.error) {
      throw new Error(typeof raw.error?.message === "string" ? raw.error.message : JSON.stringify(raw.error));
    }
    return raw;
  }

  async getPiecesMetadata(designId: string) {
    const raw = await withTimeout(kitGraphqlKitDesignPiecesMetadata(this.gql(), designId), this.timeoutMs, "timeout");
    return this.unwrapQuery(raw);
  }

  async getPieces(designId: string) {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKitDesign(this, designId, { readDesignPiecesFullCommand: null });
          if (!("readDesignPiecesFullCommand" in out) || out.readDesignPiecesFullCommand == null) {
            throw new Error("readDesignPiecesFullCommand: missing output");
          }
          return out.readDesignPiecesFullCommand.pieces;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getConnections(designId: string) {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKitDesign(this, designId, { readDesignConnectionsFullCommand: null });
          if (!("readDesignConnectionsFullCommand" in out) || out.readDesignConnectionsFullCommand == null) {
            throw new Error("readDesignConnectionsFullCommand: missing output");
          }
          return out.readDesignConnectionsFullCommand.connections;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getDesigns() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this, { readKitDesignsShallowCommand: null });
          if (!("readKitDesignsShallowCommand" in out) || out.readKitDesignsShallowCommand == null) {
            throw new Error("readKitDesignsShallowCommand: missing output");
          }
          return out.readKitDesignsShallowCommand.designs;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getTypes() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this, { readKitTypesShallowCommand: null });
          if (!("readKitTypesShallowCommand" in out) || out.readKitTypesShallowCommand == null) {
            throw new Error("readKitTypesShallowCommand: missing output");
          }
          return out.readKitTypesShallowCommand.types;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getAuthors() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this, { readKitAuthorsShallowCommand: null });
          if (!("readKitAuthorsShallowCommand" in out) || out.readKitAuthorsShallowCommand == null) {
            throw new Error("readKitAuthorsShallowCommand: missing output");
          }
          return out.readKitAuthorsShallowCommand.authors;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getKitMetadata() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this, { readKitMetadataCommand: null });
          if (!("readKitMetadataCommand" in out) || out.readKitMetadataCommand == null) {
            throw new Error("readKitMetadataCommand: missing output");
          }
          return out.readKitMetadataCommand.metadata;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async executeRead(cmds: ReadCommandBatch): Promise<ReadCommandBatchResult> {
    return await withTimeout(kitGraphqlExecuteRead(this.gql(), cmds), this.timeoutMs, "timeout");
  }

  async execute(cmd: unknown): Promise<KitStoreExecuteResult> {
    try {
      const result = await withTimeout(kitGraphqlExecuteStoreCommand(this.gql(), cmd), this.timeoutMs, "timeout");
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  async vcsState(): Promise<any> {
    return await withTimeout(Promise.resolve(this.handle.vcsState()), this.timeoutMs, "timeout");
  }

  async theKitDto(): Promise<any> {
    return await withTimeout(Promise.resolve(this.handle.theKitDto()), this.timeoutMs, "timeout");
  }

  async materializeAt(id: string): Promise<any> {
    const at = id.trim() === "" ? undefined : id;
    return await withTimeout(Promise.resolve(this.handle.materializeAt(at)), this.timeoutMs, "timeout");
  }

  async attachBackbone(cfg: KitStoreWireBackboneConfig): Promise<SetResult> {
    const r = await this.execute({ attachBackbone: { config: cfg } });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.attachBackbone as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      await this.getSnapshot();
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "attachBackbone: unexpected result" } };
  }

  async detachBackbone(): Promise<SetResult> {
    const r = await this.execute({ detachBackbone: null });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.detachBackbone as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      await this.getSnapshot();
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "detachBackbone: unexpected result" } };
  }

  async backboneStatus(): Promise<KitStoreWireBackboneStatus> {
    const r = await this.execute({ backboneStatus: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreBackboneStatusResult(r.result);
  }

  async listConflicts(): Promise<KitStoreWireKitConflict[]> {
    const r = await this.execute({ listConflicts: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreListConflictsResult(r.result);
  }

  async resolveConflict(id: string, strategy: KitStoreWireConflictResolution): Promise<SetResult> {
    const r = await this.execute({ resolveConflict: { id, strategy } });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.resolveConflict as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      await this.getSnapshot();
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "resolveConflict: unexpected result" } };
  }

  async syncNow(): Promise<SetResult> {
    const r = await this.execute({ syncNow: null });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.syncNow as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      await this.getSnapshot();
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "syncNow: unexpected result" } };
  }
}

function withTimeout<T>(p: Promise<T>, ms: number, label: string): Promise<T> {
  if (!ms || ms <= 0) return p;
  return new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(label)), ms);
    p.then(
      (v) => {
        clearTimeout(t);
        resolve(v);
      },
      (e) => {
        clearTimeout(t);
        reject(e);
      },
    );
  });
}

/** Comlink-backed client; falls back if worker fails to boot. */
export class WorkerKitStoreClient implements KitStoreClient {
  private worker: Worker;
  private api: any;
  private listeners: Set<(ev: any) => void> = new Set();
  private cached: any;
  private timeoutMs: number;
  private workerGqlSubStarted = false;

  constructor(worker: Worker, api: any, cachedDto: any, timeoutMs: number) {
    this.worker = worker;
    this.api = api;
    this.cached = cachedDto;
    this.timeoutMs = timeoutMs;
  }

  kitGraphql(): KitGraphqlHandle {
    return {
      execute: async (requestJson: string, onMessage: (line: string) => void) => {
        const Comlink = await import("comlink");
        await this.api.graphqlExecute(requestJson, Comlink.proxy(onMessage));
      },
    };
  }

  getDto() {
    return this.cached;
  }

  async getSnapshot() {
    try {
      this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "snapshot timeout");
    } catch {
      /* keep cached */
    }
    return this.cached;
  }

  subscribe(cb: (ev: any) => void): () => void {
    this.listeners.add(cb);
    if (!this.workerGqlSubStarted) {
      this.workerGqlSubStarted = true;
      void import("comlink").then((Comlink) => {
        void this.api.subscribe(Comlink.proxy((ev: any) => {
          for (const l of this.listeners) {
            try {
              l(ev);
            } catch {
              /* ignore */
            }
          }
        }));
      });
    }
    return () => {
      this.listeners.delete(cb);
      if (this.listeners.size === 0) this.workerGqlSubStarted = false;
    };
  }

  dispose() {
    this.listeners.clear();
    this.worker.terminate();
  }

  async setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.setField(kind, id, field, value), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.addChild(parentKind, parentId, childKind, dto), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.removeChild(parentKind, parentId, childKind, childId), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async applyDesignDiff(designId: string, diff: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.applyDesignDiff(designId, diff), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async applyKitDiff(diff: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.applyKitDiff(diff), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async clusterPieces(designId: string, pieceIds: string[], clusterName: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.clusterPieces(designId, pieceIds, clusterName), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async dragPieces(designId: string, pieceIds: string[], du: number, dv: number): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.dragPieces(designId, pieceIds, du, dv), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.movePieces(designId, pieceIds, gap, shift, rise), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async fixPieces(designId: string, pieceIds: string[]): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.fixPieces(designId, pieceIds), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async flattenDesign(designId: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.flattenDesign(designId), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.expandDesign(parentDesignId, nestedDesignId), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.deleteConnection(designId, connectionId), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.changePieceType(designId, pieceId, newTypeId), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.pasteDesignSelection(designId, selection, plane), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async createHangingPieces(designId: string, typeIds: string[], plane: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.createHangingPieces(designId, typeIds, plane), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> {
    try {
      const raw = await withTimeout(this.api.createFixedPiece(designId, typeId, plane), this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(raw));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  private async settleMutateAndRefresh(raw: Promise<unknown>): Promise<SetResult> {
    try {
      const got = await withTimeout(raw, this.timeoutMs, "timeout");
      const r = await settleSetPromise(Promise.resolve(got));
      if (r.ok) {
        try {
          this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
        } catch {
          /* ignore */
        }
      }
      return r;
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async undo(): Promise<SetResult> {
    return this.settleMutateAndRefresh(Promise.resolve(this.api.undo()));
  }

  async redo(): Promise<SetResult> {
    return this.settleMutateAndRefresh(Promise.resolve(this.api.redo()));
  }

  async canUndo(): Promise<boolean> {
    try {
      return await withTimeout(this.api.canUndo(), this.timeoutMs, "timeout");
    } catch {
      return false;
    }
  }

  async canRedo(): Promise<boolean> {
    try {
      return await withTimeout(this.api.canRedo(), this.timeoutMs, "timeout");
    } catch {
      return false;
    }
  }

  private unwrapQuery(raw: any) {
    if (raw && typeof raw === "object" && raw.ok === false && raw.error) {
      throw new Error(typeof raw.error?.message === "string" ? raw.error.message : JSON.stringify(raw.error));
    }
    return raw;
  }

  async getPiecesMetadata(designId: string) {
    return this.unwrapQuery(
      await withTimeout(kitGraphqlKitDesignPiecesMetadata(this.kitGraphql(), designId), this.timeoutMs, "timeout"),
    );
  }

  private asKitExecuteReadClient(): KitExecuteRead {
    return { executeRead: (batch) => kitGraphqlExecuteRead(this.kitGraphql(), batch) };
  }

  async getPieces(designId: string) {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKitDesign(this.asKitExecuteReadClient(), designId, { readDesignPiecesFullCommand: null });
          if (!("readDesignPiecesFullCommand" in out) || out.readDesignPiecesFullCommand == null) {
            throw new Error("readDesignPiecesFullCommand: missing output");
          }
          return out.readDesignPiecesFullCommand.pieces;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getConnections(designId: string) {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKitDesign(this.asKitExecuteReadClient(), designId, { readDesignConnectionsFullCommand: null });
          if (!("readDesignConnectionsFullCommand" in out) || out.readDesignConnectionsFullCommand == null) {
            throw new Error("readDesignConnectionsFullCommand: missing output");
          }
          return out.readDesignConnectionsFullCommand.connections;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getDesigns() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this.asKitExecuteReadClient(), { readKitDesignsShallowCommand: null });
          if (!("readKitDesignsShallowCommand" in out) || out.readKitDesignsShallowCommand == null) {
            throw new Error("readKitDesignsShallowCommand: missing output");
          }
          return out.readKitDesignsShallowCommand.designs;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getTypes() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this.asKitExecuteReadClient(), { readKitTypesShallowCommand: null });
          if (!("readKitTypesShallowCommand" in out) || out.readKitTypesShallowCommand == null) {
            throw new Error("readKitTypesShallowCommand: missing output");
          }
          return out.readKitTypesShallowCommand.types;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getAuthors() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this.asKitExecuteReadClient(), { readKitAuthorsShallowCommand: null });
          if (!("readKitAuthorsShallowCommand" in out) || out.readKitAuthorsShallowCommand == null) {
            throw new Error("readKitAuthorsShallowCommand: missing output");
          }
          return out.readKitAuthorsShallowCommand.authors;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async getKitMetadata() {
    return this.unwrapQuery(
      await withTimeout(
        (async () => {
          const out = await readKit(this.asKitExecuteReadClient(), { readKitMetadataCommand: null });
          if (!("readKitMetadataCommand" in out) || out.readKitMetadataCommand == null) {
            throw new Error("readKitMetadataCommand: missing output");
          }
          return out.readKitMetadataCommand.metadata;
        })(),
        this.timeoutMs,
        "timeout",
      ),
    );
  }

  async executeRead(cmds: ReadCommandBatch): Promise<ReadCommandBatchResult> {
    return await withTimeout(this.api.executeRead(cmds) as Promise<ReadCommandBatchResult>, this.timeoutMs, "timeout");
  }

  async execute(cmd: unknown): Promise<KitStoreExecuteResult> {
    try {
      return await withTimeout(this.api.execute(cmd), this.timeoutMs, "timeout");
    } catch {
      return { ok: false, error: { kind: "Timeout", message: "timeout" } };
    }
  }

  async vcsState(): Promise<any> {
    return await withTimeout(this.api.vcsState(), this.timeoutMs, "timeout");
  }

  async theKitDto(): Promise<any> {
    return await withTimeout(this.api.theKitDto(), this.timeoutMs, "timeout");
  }

  async materializeAt(id: string): Promise<any> {
    const at = id.trim() === "" ? undefined : id;
    return await withTimeout(this.api.materializeAt(at), this.timeoutMs, "timeout");
  }

  async attachBackbone(cfg: KitStoreWireBackboneConfig): Promise<SetResult> {
    const r = await this.execute({ attachBackbone: { config: cfg } });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.attachBackbone as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      try {
        this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
      } catch {
        /* ignore */
      }
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "attachBackbone: unexpected result" } };
  }

  async detachBackbone(): Promise<SetResult> {
    const r = await this.execute({ detachBackbone: null });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.detachBackbone as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      try {
        this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
      } catch {
        /* ignore */
      }
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "detachBackbone: unexpected result" } };
  }

  async backboneStatus(): Promise<KitStoreWireBackboneStatus> {
    const r = await this.execute({ backboneStatus: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreBackboneStatusResult(r.result);
  }

  async listConflicts(): Promise<KitStoreWireKitConflict[]> {
    const r = await this.execute({ listConflicts: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreListConflictsResult(r.result);
  }

  async resolveConflict(id: string, strategy: KitStoreWireConflictResolution): Promise<SetResult> {
    const r = await this.execute({ resolveConflict: { id, strategy } });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.resolveConflict as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      try {
        this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
      } catch {
        /* ignore */
      }
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "resolveConflict: unexpected result" } };
  }

  async syncNow(): Promise<SetResult> {
    const r = await this.execute({ syncNow: null });
    if (!r.ok) return r;
    const o = r.result as Record<string, unknown>;
    const inner = o.syncNow as { ok?: boolean } | undefined;
    if (inner?.ok === true) {
      try {
        this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout");
      } catch {
        /* ignore */
      }
      return { ok: true } as const;
    }
    return { ok: false, error: { kind: "Internal", message: "syncNow: unexpected result" } };
  }
}

/** Single-flight wasm `default()` + `boot()` per specifier (re-entrant `createKitStoreClient` must not re-init). */
const semioWasmInitBySpecifier = new Map<string, Promise<void>>();

async function ensureSemioWasmInitialized(wasmSpecifier: string, mod: any, tryNodeFsWasm: boolean): Promise<void> {
  let flight = semioWasmInitBySpecifier.get(wasmSpecifier);
  if (!flight) {
    flight = (async () => {
      if (typeof mod.default !== "function") return;
      if (tryNodeFsWasm) {
        try {
          const fs = await import("node:fs/promises");
          const { fileURLToPath } = await import("node:url");
          const wasmPath = fileURLToPath(new URL("../rs/pkg/semio_bg.wasm", import.meta.url));
          const wasmBytes = await fs.readFile(wasmPath);
          await mod.default({ module_or_path: wasmBytes });
          if (typeof mod.boot === "function") mod.boot();
          return;
        } catch {
          /* fall through to fetch/init */
        }
      }
      await mod.default();
      if (typeof mod.boot === "function") mod.boot();
    })();
    semioWasmInitBySpecifier.set(wasmSpecifier, flight);
  }
  await flight;
}

/** Hosts should map this import to the wasm-bindgen JS glue (see sketchpad Vite config). */
export async function createKitStoreClient(opts: CreateKitStoreClientOptions): Promise<KitStoreClient> {
  // JSON round-trip: wasm bindgen deserializer expects plain objects; structuredClone can preserve prototypes that break `Reflect.get` during `from_value`.
  const dto = JSON.parse(JSON.stringify(asKitInstance(opts.initialKit).toJSON())) as ReturnType<KitImpl["toJSON"]>;
  const wasmSpecifier =
    opts.wasmSpecifier ??
    (globalThis as any).__SEMIO_WASM_SPECIFIER__ ??
    "@semio/rs-wasm";
  const timeoutMs = opts.timeoutMs ?? 60_000;
  const isNodeRuntime = (typeof process !== "undefined" && !!process.versions?.node) || (typeof navigator !== "undefined" && /jsdom/i.test(navigator.userAgent ?? ""));
  const useFallback = opts.forceFallback === true || typeof Worker === "undefined" || isNodeRuntime;

  const importWasmModule = async (specifier: string) => {
    if (specifier === "@semio/rs-wasm") {
      return import("@semio/rs-wasm");
    }
    return import(/* @vite-ignore */ specifier);
  };

  if (useFallback) {
    const mod = await importWasmModule(wasmSpecifier);
    await ensureSemioWasmInitialized(wasmSpecifier, mod, isNodeRuntime);
    return new FallbackKitStoreClient(mod.KitStoreHandle.create(dto), dto, timeoutMs);
  }
  try {
    const Comlink = await import("comlink");
    const worker =
      opts.workerFactory?.() ??
      new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
    const api = Comlink.wrap(worker);
    await api.init(wasmSpecifier, dto);
    return new WorkerKitStoreClient(worker, api, dto, timeoutMs);
  } catch {
    const mod = await importWasmModule(wasmSpecifier);
    await ensureSemioWasmInitialized(wasmSpecifier, mod, isNodeRuntime);
    return new FallbackKitStoreClient(mod.KitStoreHandle.create(dto), dto, timeoutMs);
  }
}

// #endregion 🌐 KitStorePipeline
//#region 📖ReadCommandTypes
/**
 * Read command + output types (serde **externally tagged**, `camelCase` keys).
 * @generated by gen_read_command_types.py from ../rs/read_module.rs
 */

/** `{"id": string}` in JSON; all `*IdDto` use this shape. */
export type IdDto = { readonly id: string };

/** One row of `readDesignFlattenMap` (camelCase fields). */
export type DesignFlattenMapEntryDto = {
  readonly pieceId: string;
  readonly plane: unknown;
  readonly center: unknown;
};

export type ReadAttributeCommand =
  | { readonly readAttributeFullCommand: null }
  | { readonly readAttributeShallowCommand: null }
  | { readonly readAttributeMetadataCommand: null }
  | { readonly readAttributeIdCommand: null }
  | { readonly readAttributeKeyCommand: null }
  | { readonly readAttributeValueCommand: null }
  | { readonly readAttributeDefinitionCommand: null }

export type ReadAuthorCommand =
  | { readonly readAuthorFullCommand: null }
  | { readonly readAuthorShallowCommand: null }
  | { readonly readAuthorMetadataCommand: null }
  | { readonly readAuthorIdCommand: null }
  | { readonly readAuthorNameCommand: null }
  | { readonly readAuthorEmailCommand: null }
  | { readonly readAuthorRoleCommand: null }
  | { readonly readAuthorRankCommand: null }

export type ReadBenchmarkCommand =
  | { readonly readBenchmarkFullCommand: null }
  | { readonly readBenchmarkShallowCommand: null }
  | { readonly readBenchmarkMetadataCommand: null }
  | { readonly readBenchmarkIdCommand: null }
  | { readonly readBenchmarkNameCommand: null }
  | { readonly readBenchmarkMinCommand: null }
  | { readonly readBenchmarkMaxCommand: null }
  | { readonly readBenchmarkMinExcludedCommand: null }
  | { readonly readBenchmarkMaxExcludedCommand: null }

export type ReadConceptCommand =
  | { readonly readConceptFullCommand: null }
  | { readonly readConceptShallowCommand: null }
  | { readonly readConceptMetadataCommand: null }
  | { readonly readConceptIdCommand: null }
  | { readonly readConceptNameCommand: null }
  | { readonly readConceptDescriptionCommand: null }
  | { readonly readConceptOrderCommand: null }

export type ReadConnectionCommand =
  | { readonly readConnectionFullCommand: null }
  | { readonly readConnectionShallowCommand: null }
  | { readonly readConnectionMetadataCommand: null }
  | { readonly readConnectionIdCommand: null }
  | { readonly readConnectionConnectedSideMetadataCommand: null }
  | { readonly readConnectionConnectingSideMetadataCommand: null }
  | { readonly readConnectionConnectedSideFullCommand: null }
  | { readonly readConnectionConnectingSideFullCommand: null }
  | { readonly readConnectionGapCommand: null }
  | { readonly readConnectionShiftCommand: null }
  | { readonly readConnectionRiseCommand: null }
  | { readonly readConnectionRotationCommand: null }
  | { readonly readConnectionTurnCommand: null }
  | { readonly readConnectionTiltCommand: null }
  | { readonly readConnectionUCommand: null }
  | { readonly readConnectionVCommand: null }
  | { readonly readConnectionDescriptionCommand: null }
  | { readonly readConnectionAttributesFullCommand: null }
  | { readonly readConnectionAttributesShallowCommand: null }
  | { readonly readConnectionChildPlaneMatrixCommand: null }
  | { readonly readConnectionFlatSidesForChildCommand: { readonly childPieceId: IdDto } }
  | { readonly readConnectionAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }
  | { readonly readConnectionConnectedSideCommands: { readonly commands: ReadonlyArray<ReadSideCommand> } }
  | { readonly readConnectionConnectingSideCommands: { readonly commands: ReadonlyArray<ReadSideCommand> } }

export type ReadConnectorCommand =
  | { readonly readConnectorFullCommand: null }
  | { readonly readConnectorShallowCommand: null }
  | { readonly readConnectorMetadataCommand: null }
  | { readonly readConnectorIdCommand: null }
  | { readonly readConnectorCodeCommand: null }
  | { readonly readConnectorDescriptionCommand: null }
  | { readonly readConnectorPortIdCommand: null }
  | { readonly readConnectorQualitiesFullCommand: null }
  | { readonly readConnectorQualitiesShallowCommand: null }
  | { readonly readConnectorAttributesFullCommand: null }
  | { readonly readConnectorAttributesShallowCommand: null }
  | { readonly readConnectorQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } }
  | { readonly readConnectorAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadDesignCommand =
  | { readonly readDesignFullCommand: null }
  | { readonly readDesignShallowCommand: null }
  | { readonly readDesignMetadataCommand: null }
  | { readonly readDesignIdCommand: null }
  | { readonly readDesignNameCommand: null }
  | { readonly readDesignDescriptionCommand: null }
  | { readonly readDesignIconCommand: null }
  | { readonly readDesignImageCommand: null }
  | { readonly readDesignLocationCommand: null }
  | { readonly readDesignUnitCommand: null }
  | { readonly readDesignCreatedCommand: null }
  | { readonly readDesignUpdatedCommand: null }
  | { readonly readDesignKitCommand: null }
  | { readonly readDesignFamiliesCommand: null }
  | { readonly readDesignPiecesFullCommand: null }
  | { readonly readDesignPiecesShallowCommand: null }
  | { readonly readDesignConnectionsFullCommand: null }
  | { readonly readDesignConnectionsShallowCommand: null }
  | { readonly readDesignLayersFullCommand: null }
  | { readonly readDesignLayersShallowCommand: null }
  | { readonly readDesignGroupsFullCommand: null }
  | { readonly readDesignGroupsShallowCommand: null }
  | { readonly readDesignAuthorsFullCommand: null }
  | { readonly readDesignAuthorsShallowCommand: null }
  | { readonly readDesignConceptsFullCommand: null }
  | { readonly readDesignConceptsShallowCommand: null }
  | { readonly readDesignTagsFullCommand: null }
  | { readonly readDesignTagsShallowCommand: null }
  | { readonly readDesignQualitiesFullCommand: null }
  | { readonly readDesignQualitiesShallowCommand: null }
  | { readonly readDesignPropsFullCommand: null }
  | { readonly readDesignPropsShallowCommand: null }
  | { readonly readDesignAttributesFullCommand: null }
  | { readonly readDesignAttributesShallowCommand: null }
  | { readonly readDesignStatsFullCommand: null }
  | { readonly readDesignStatsShallowCommand: null }
  | { readonly readDesignFlattenMapCommand: null }
  | { readonly readDesignClusterableGroupsCommand: { readonly selection: ReadonlyArray<IdDto> } }
  | { readonly readDesignIncludedDesignsCommand: null }
  | { readonly readDesignReplaceableCatalogCommand: { readonly selection: ReadonlyArray<IdDto> } }
  | { readonly readDesignIncludedDesignIdsCommand: null }
  | { readonly readDesignQualitySumCommand: { readonly qualityId: IdDto } }
  | { readonly readDesignFamilyCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFamilyCommand> } }
  | { readonly readDesignPieceCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPieceCommand> } }
  | { readonly readDesignConnectionCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConnectionCommand> } }
  | { readonly readDesignLayerCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadLayerCommand> } }
  | { readonly readDesignGroupCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadGroupCommand> } }
  | { readonly readDesignAuthorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAuthorCommand> } }
  | { readonly readDesignConceptCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConceptCommand> } }
  | { readonly readDesignTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } }
  | { readonly readDesignQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } }
  | { readonly readDesignPropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } }
  | { readonly readDesignAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }
  | { readonly readDesignStatCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadStatCommand> } }

export type ReadFamilyCommand =
  | { readonly readFamilyFullCommand: null }
  | { readonly readFamilyShallowCommand: null }
  | { readonly readFamilyMetadataCommand: null }
  | { readonly readFamilyIdCommand: null }
  | { readonly readFamilyNameCommand: null }
  | { readonly readFamilyDescriptionCommand: null }
  | { readonly readFamilyIconCommand: null }
  | { readonly readFamilyPortsFullCommand: null }
  | { readonly readFamilyPortsShallowCommand: null }
  | { readonly readFamilyAttributesFullCommand: null }
  | { readonly readFamilyAttributesShallowCommand: null }
  | { readonly readFamilyPortCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPortCommand> } }
  | { readonly readFamilyAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadFileCommand =
  | { readonly readFileFullCommand: null }
  | { readonly readFileShallowCommand: null }
  | { readonly readFileMetadataCommand: null }
  | { readonly readFileIdCommand: null }
  | { readonly readFileUrlCommand: null }
  | { readonly readFileMimeCommand: null }
  | { readonly readFileSizeCommand: null }
  | { readonly readFileHashCommand: null }
  | { readonly readFileDescriptionCommand: null }
  | { readonly readFileCreatedCommand: null }
  | { readonly readFileUpdatedCommand: null }

export type ReadFolderCommand =
  | { readonly readFolderFullCommand: null }
  | { readonly readFolderShallowCommand: null }
  | { readonly readFolderMetadataCommand: null }
  | { readonly readFolderIdCommand: null }
  | { readonly readFolderPathCommand: null }
  | { readonly readFolderDescriptionCommand: null }

export type ReadGroupCommand =
  | { readonly readGroupFullCommand: null }
  | { readonly readGroupShallowCommand: null }
  | { readonly readGroupMetadataCommand: null }
  | { readonly readGroupIdCommand: null }
  | { readonly readGroupNameCommand: null }
  | { readonly readGroupDescriptionCommand: null }
  | { readonly readGroupColorCommand: null }
  | { readonly readGroupIconCommand: null }
  | { readonly readGroupPiecesCommand: null }

export type ReadKitCommand =
  | { readonly readKitFullCommand: null }
  | { readonly readKitShallowCommand: null }
  | { readonly readKitMetadataCommand: null }
  | { readonly readKitIdCommand: null }
  | { readonly readKitNameCommand: null }
  | { readonly readKitDescriptionCommand: null }
  | { readonly readKitIconCommand: null }
  | { readonly readKitImageCommand: null }
  | { readonly readKitPreviewCommand: null }
  | { readonly readKitRemoteCommand: null }
  | { readonly readKitHomepageCommand: null }
  | { readonly readKitLicenseCommand: null }
  | { readonly readKitUriCommand: null }
  | { readonly readKitCreatedCommand: null }
  | { readonly readKitUpdatedCommand: null }
  | { readonly readKitTypesFullCommand: null }
  | { readonly readKitTypesShallowCommand: null }
  | { readonly readKitTypeIdsCommand: null }
  | { readonly readKitTypesMetadataCommand: null }
  | { readonly readKitDesignsFullCommand: null }
  | { readonly readKitDesignsShallowCommand: null }
  | { readonly readKitDesignIdsCommand: null }
  | { readonly readKitDesignsMetadataCommand: null }
  | { readonly readKitFilesFullCommand: null }
  | { readonly readKitFilesShallowCommand: null }
  | { readonly readKitFoldersFullCommand: null }
  | { readonly readKitFoldersShallowCommand: null }
  | { readonly readKitLocationsFullCommand: null }
  | { readonly readKitLocationsShallowCommand: null }
  | { readonly readKitFamiliesFullCommand: null }
  | { readonly readKitFamiliesShallowCommand: null }
  | { readonly readKitPortsFullCommand: null }
  | { readonly readKitAuthorsFullCommand: null }
  | { readonly readKitAuthorsShallowCommand: null }
  | { readonly readKitConceptsFullCommand: null }
  | { readonly readKitConceptsShallowCommand: null }
  | { readonly readKitTagsFullCommand: null }
  | { readonly readKitTagsShallowCommand: null }
  | { readonly readKitQualitiesFullCommand: null }
  | { readonly readKitQualitiesShallowCommand: null }
  | { readonly readKitPropsFullCommand: null }
  | { readonly readKitPropsShallowCommand: null }
  | { readonly readKitAttributesFullCommand: null }
  | { readonly readKitAttributesShallowCommand: null }
  | { readonly readKitColoredConnectorsCommand: null }
  | { readonly readKitTypeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTypeCommand> } }
  | { readonly readKitDesignCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadDesignCommand> } }
  | { readonly readKitFileCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFileCommand> } }
  | { readonly readKitFolderCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFolderCommand> } }
  | { readonly readKitLocationCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadLocationCommand> } }
  | { readonly readKitFamilyCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFamilyCommand> } }
  | { readonly readKitPortCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPortCommand> } }
  | { readonly readKitAuthorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAuthorCommand> } }
  | { readonly readKitConceptCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConceptCommand> } }
  | { readonly readKitTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } }
  | { readonly readKitQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } }
  | { readonly readKitPropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } }
  | { readonly readKitAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadLayerCommand =
  | { readonly readLayerFullCommand: null }
  | { readonly readLayerShallowCommand: null }
  | { readonly readLayerMetadataCommand: null }
  | { readonly readLayerIdCommand: null }
  | { readonly readLayerNameCommand: null }
  | { readonly readLayerDescriptionCommand: null }
  | { readonly readLayerColorCommand: null }
  | { readonly readLayerOrderCommand: null }
  | { readonly readLayerVisibleCommand: null }
  | { readonly readLayerLockedCommand: null }

export type ReadLocationCommand =
  | { readonly readLocationFullCommand: null }
  | { readonly readLocationShallowCommand: null }
  | { readonly readLocationMetadataCommand: null }
  | { readonly readLocationIdCommand: null }
  | { readonly readLocationLongitudeCommand: null }
  | { readonly readLocationLatitudeCommand: null }
  | { readonly readLocationAltitudeCommand: null }
  | { readonly readLocationAttributesFullCommand: null }
  | { readonly readLocationAttributesShallowCommand: null }
  | { readonly readLocationAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadPieceCommand =
  | { readonly readPieceFullCommand: null }
  | { readonly readPieceShallowCommand: null }
  | { readonly readPieceMetadataCommand: null }
  | { readonly readPieceIdCommand: null }
  | { readonly readPieceNameCommand: null }
  | { readonly readPieceDescriptionCommand: null }
  | { readonly readPiecePlaneCommand: null }
  | { readonly readPieceCenterCommand: null }
  | { readonly readPieceScaleCommand: null }
  | { readonly readPieceMirrorPlaneCommand: null }
  | { readonly readPieceHiddenCommand: null }
  | { readonly readPieceLockedCommand: null }
  | { readonly readPieceColorCommand: null }
  | { readonly readPieceTypeCommand: null }
  | { readonly readPieceDesignCommand: null }
  | { readonly readPiecePropsFullCommand: null }
  | { readonly readPiecePropsShallowCommand: null }
  | { readonly readPieceAttributesFullCommand: null }
  | { readonly readPieceAttributesShallowCommand: null }
  | { readonly readPieceFlatPlaneCommand: null }
  | { readonly readPieceFlatCenterCommand: null }
  | { readonly readPieceFlatPoseCommand: null }
  | { readonly readPiecePathCommand: null }
  | { readonly readPieceParentPieceIdCommand: null }
  | { readonly readPieceParentConnectionIdCommand: null }
  | { readonly readPieceParentConnectionFullCommand: null }
  | { readonly readPieceParentDesignIdCommand: null }
  | { readonly readPieceFixedCommand: null }
  | { readonly readPieceConnectedCommand: null }
  | { readonly readPieceAlternativesCommand: null }
  | { readonly readPieceAlternativeTypesCommand: null }
  | { readonly readPieceAlternativeDesignsCommand: null }
  | { readonly readPiecePropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } }
  | { readonly readPieceAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadPortCommand =
  | { readonly readPortFullCommand: null }
  | { readonly readPortShallowCommand: null }
  | { readonly readPortMetadataCommand: null }
  | { readonly readPortIdCommand: null }
  | { readonly readPortNameCommand: null }
  | { readonly readPortDescriptionCommand: null }
  | { readonly readPortIconCommand: null }
  | { readonly readPortCompatibleFamiliesCommand: null }
  | { readonly readPortMandatoryCommand: null }
  | { readonly readPortTCommand: null }
  | { readonly readPortPointCommand: null }
  | { readonly readPortDirectionCommand: null }
  | { readonly readPortCompatiblePortsCommand: null }
  | { readonly readPortQualitiesFullCommand: null }
  | { readonly readPortQualitiesShallowCommand: null }
  | { readonly readPortAttributesFullCommand: null }
  | { readonly readPortAttributesShallowCommand: null }
  | { readonly readPortQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } }
  | { readonly readPortAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadPropCommand =
  | { readonly readPropFullCommand: null }
  | { readonly readPropShallowCommand: null }
  | { readonly readPropIdCommand: null }
  | { readonly readPropKeyCommand: null }
  | { readonly readPropValueCommand: null }
  | { readonly readPropUnitCommand: null }
  | { readonly readPropQualityIdCommand: null }

export type ReadQualityCommand =
  | { readonly readQualityFullCommand: null }
  | { readonly readQualityShallowCommand: null }
  | { readonly readQualityMetadataCommand: null }
  | { readonly readQualityIdCommand: null }
  | { readonly readQualityKeyCommand: null }
  | { readonly readQualityValueCommand: null }
  | { readonly readQualityUnitCommand: null }
  | { readonly readQualityDefinitionCommand: null }
  | { readonly readQualityDescriptionCommand: null }
  | { readonly readQualityBenchmarksFullCommand: null }
  | { readonly readQualityBenchmarksShallowCommand: null }
  | { readonly readQualityBenchmarkCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadBenchmarkCommand> } }

export type ReadRepresentationCommand =
  | { readonly readRepresentationFullCommand: null }
  | { readonly readRepresentationShallowCommand: null }
  | { readonly readRepresentationMetadataCommand: null }
  | { readonly readRepresentationIdCommand: null }
  | { readonly readRepresentationUrlCommand: null }
  | { readonly readRepresentationDescriptionCommand: null }
  | { readonly readRepresentationFileIdCommand: null }
  | { readonly readRepresentationTagsFullCommand: null }
  | { readonly readRepresentationTagsShallowCommand: null }
  | { readonly readRepresentationQualitiesFullCommand: null }
  | { readonly readRepresentationQualitiesShallowCommand: null }
  | { readonly readRepresentationAttributesFullCommand: null }
  | { readonly readRepresentationAttributesShallowCommand: null }
  | { readonly readRepresentationTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } }
  | { readonly readRepresentationQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } }
  | { readonly readRepresentationAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadSideCommand =
  | { readonly readSideFullCommand: null }
  | { readonly readSideShallowCommand: null }
  | { readonly readSideMetadataCommand: null }
  | { readonly readSideIdCommand: null }
  | { readonly readSidePieceIdCommand: null }
  | { readonly readSidePortIdCommand: null }
  | { readonly readSideDesignPieceIdCommand: null }

export type ReadStatCommand =
  | { readonly readStatFullCommand: null }
  | { readonly readStatShallowCommand: null }
  | { readonly readStatMetadataCommand: null }
  | { readonly readStatIdCommand: null }
  | { readonly readStatKeyCommand: null }
  | { readonly readStatValueCommand: null }
  | { readonly readStatUnitCommand: null }
  | { readonly readStatDescriptionCommand: null }

export type ReadTagCommand =
  | { readonly readTagFullCommand: null }
  | { readonly readTagShallowCommand: null }
  | { readonly readTagMetadataCommand: null }
  | { readonly readTagIdCommand: null }
  | { readonly readTagNameCommand: null }
  | { readonly readTagOrderCommand: null }

export type ReadTypeCommand =
  | { readonly readTypeFullCommand: null }
  | { readonly readTypeShallowCommand: null }
  | { readonly readTypeMetadataCommand: null }
  | { readonly readTypeIdCommand: null }
  | { readonly readTypeNameCommand: null }
  | { readonly readTypeDescriptionCommand: null }
  | { readonly readTypeIconCommand: null }
  | { readonly readTypeImageCommand: null }
  | { readonly readTypeStockCommand: null }
  | { readonly readTypeVirtualCommand: null }
  | { readonly readTypeUnitCommand: null }
  | { readonly readTypeLocationCommand: null }
  | { readonly readTypeCreatedCommand: null }
  | { readonly readTypeUpdatedCommand: null }
  | { readonly readTypeFamiliesCommand: null }
  | { readonly readTypeConnectorsFullCommand: null }
  | { readonly readTypeConnectorsShallowCommand: null }
  | { readonly readTypeRepresentationsFullCommand: null }
  | { readonly readTypeRepresentationsShallowCommand: null }
  | { readonly readTypeAuthorsFullCommand: null }
  | { readonly readTypeAuthorsShallowCommand: null }
  | { readonly readTypeConceptsFullCommand: null }
  | { readonly readTypeConceptsShallowCommand: null }
  | { readonly readTypeTagsFullCommand: null }
  | { readonly readTypeTagsShallowCommand: null }
  | { readonly readTypeQualitiesFullCommand: null }
  | { readonly readTypeQualitiesShallowCommand: null }
  | { readonly readTypePropsFullCommand: null }
  | { readonly readTypePropsShallowCommand: null }
  | { readonly readTypeAttributesFullCommand: null }
  | { readonly readTypeAttributesShallowCommand: null }
  | { readonly readTypePortsFullCommand: null }
  | { readonly readTypeConnectorForPortIdCommand: { readonly portId: IdDto } }
  | { readonly readTypeBestRepresentationCommand: { readonly tagIds: ReadonlyArray<string> } }
  | { readonly readTypeFamilyCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFamilyCommand> } }
  | { readonly readTypeConnectorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConnectorCommand> } }
  | { readonly readTypeRepresentationCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadRepresentationCommand> } }
  | { readonly readTypePortCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPortCommand> } }
  | { readonly readTypeAuthorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAuthorCommand> } }
  | { readonly readTypeConceptCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConceptCommand> } }
  | { readonly readTypeTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } }
  | { readonly readTypeQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } }
  | { readonly readTypePropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } }
  | { readonly readTypeAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } }

export type ReadAttributeCommandOutput =
  | { readonly readAttributeFullCommand: { readonly attribute: unknown } }
  | { readonly readAttributeShallowCommand: { readonly attribute: unknown } }
  | { readonly readAttributeMetadataCommand: { readonly metadata: unknown } }
  | { readonly readAttributeIdCommand: { readonly id: IdDto } }
  | { readonly readAttributeKeyCommand: { readonly key: string } }
  | { readonly readAttributeValueCommand: { readonly value: string } }
  | { readonly readAttributeDefinitionCommand: { readonly definition: (string | null | undefined) } }

export type ReadAuthorCommandOutput =
  | { readonly readAuthorFullCommand: { readonly author: unknown } }
  | { readonly readAuthorShallowCommand: { readonly author: unknown } }
  | { readonly readAuthorMetadataCommand: { readonly metadata: unknown } }
  | { readonly readAuthorIdCommand: { readonly id: IdDto } }
  | { readonly readAuthorNameCommand: { readonly name: string } }
  | { readonly readAuthorEmailCommand: { readonly email: string } }
  | { readonly readAuthorRoleCommand: { readonly role: (string | null | undefined) } }
  | { readonly readAuthorRankCommand: { readonly rank: (number | null | undefined) } }

export type ReadBenchmarkCommandOutput =
  | { readonly readBenchmarkFullCommand: { readonly benchmark: unknown } }
  | { readonly readBenchmarkShallowCommand: { readonly benchmark: unknown } }
  | { readonly readBenchmarkMetadataCommand: { readonly metadata: unknown } }
  | { readonly readBenchmarkIdCommand: { readonly id: IdDto } }
  | { readonly readBenchmarkNameCommand: { readonly name: string } }
  | { readonly readBenchmarkMinCommand: { readonly min: (number | null | undefined) } }
  | { readonly readBenchmarkMaxCommand: { readonly max: (number | null | undefined) } }
  | { readonly readBenchmarkMinExcludedCommand: { readonly minExcluded: (boolean | null | undefined) } }
  | { readonly readBenchmarkMaxExcludedCommand: { readonly maxExcluded: (boolean | null | undefined) } }

export type ReadConceptCommandOutput =
  | { readonly readConceptFullCommand: { readonly concept: unknown } }
  | { readonly readConceptShallowCommand: { readonly concept: unknown } }
  | { readonly readConceptMetadataCommand: { readonly metadata: unknown } }
  | { readonly readConceptIdCommand: { readonly id: IdDto } }
  | { readonly readConceptNameCommand: { readonly name: string } }
  | { readonly readConceptDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readConceptOrderCommand: { readonly order: (number | null | undefined) } }

export type ReadConnectionCommandOutput =
  | { readonly readConnectionFullCommand: { readonly dto: unknown } }
  | { readonly readConnectionShallowCommand: { readonly dto: unknown } }
  | { readonly readConnectionMetadataCommand: { readonly metadata: unknown } }
  | { readonly readConnectionIdCommand: { readonly id: IdDto } }
  | { readonly readConnectionConnectedSideMetadataCommand: { readonly side: unknown } }
  | { readonly readConnectionConnectingSideMetadataCommand: { readonly side: unknown } }
  | { readonly readConnectionConnectedSideFullCommand: { readonly side: unknown } }
  | { readonly readConnectionConnectingSideFullCommand: { readonly side: unknown } }
  | { readonly readConnectionGapCommand: { readonly gap: (number | null | undefined) } }
  | { readonly readConnectionShiftCommand: { readonly shift: (number | null | undefined) } }
  | { readonly readConnectionRiseCommand: { readonly rise: (number | null | undefined) } }
  | { readonly readConnectionRotationCommand: { readonly rotation: (number | null | undefined) } }
  | { readonly readConnectionTurnCommand: { readonly turn: (number | null | undefined) } }
  | { readonly readConnectionTiltCommand: { readonly tilt: (number | null | undefined) } }
  | { readonly readConnectionUCommand: { readonly u: (number | null | undefined) } }
  | { readonly readConnectionVCommand: { readonly v: (number | null | undefined) } }
  | { readonly readConnectionDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readConnectionAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readConnectionAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readConnectionChildPlaneMatrixCommand: { readonly matrix: unknown } }
  | { readonly readConnectionFlatSidesForChildCommand: { readonly connected: unknown; readonly connecting: unknown } }
  | { readonly readConnectionAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }
  | { readonly readConnectionConnectedSideCommands: { readonly results: ReadonlyArray<ReadSideCommandOutput> } }
  | { readonly readConnectionConnectingSideCommands: { readonly results: ReadonlyArray<ReadSideCommandOutput> } }

export type ReadConnectorCommandOutput =
  | { readonly readConnectorFullCommand: { readonly connector: unknown } }
  | { readonly readConnectorShallowCommand: { readonly connector: unknown } }
  | { readonly readConnectorMetadataCommand: { readonly metadata: unknown } }
  | { readonly readConnectorIdCommand: { readonly id: IdDto } }
  | { readonly readConnectorCodeCommand: { readonly code: string } }
  | { readonly readConnectorDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readConnectorPortIdCommand: { readonly port: (IdDto | null | undefined) } }
  | { readonly readConnectorQualitiesFullCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readConnectorQualitiesShallowCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readConnectorAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readConnectorAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readConnectorQualityCommands: { readonly results: ReadonlyArray<ReadQualityCommandOutput> } }
  | { readonly readConnectorAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadDesignCommandOutput =
  | { readonly readDesignFullCommand: { readonly dto: unknown } }
  | { readonly readDesignShallowCommand: { readonly dto: unknown } }
  | { readonly readDesignMetadataCommand: { readonly metadata: unknown } }
  | { readonly readDesignIdCommand: { readonly id: IdDto } }
  | { readonly readDesignNameCommand: { readonly name: string } }
  | { readonly readDesignDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readDesignIconCommand: { readonly icon: (string | null | undefined) } }
  | { readonly readDesignImageCommand: { readonly image: (string | null | undefined) } }
  | { readonly readDesignLocationCommand: { readonly location: (IdDto | null | undefined) } }
  | { readonly readDesignUnitCommand: { readonly unit: (string | null | undefined) } }
  | { readonly readDesignCreatedCommand: { readonly created: (string | null | undefined) } }
  | { readonly readDesignUpdatedCommand: { readonly updated: (string | null | undefined) } }
  | { readonly readDesignKitCommand: { readonly kit: (IdDto | null | undefined) } }
  | { readonly readDesignFamiliesCommand: { readonly families: ReadonlyArray<IdDto> } }
  | { readonly readDesignPiecesFullCommand: { readonly pieces: ReadonlyArray<unknown> } }
  | { readonly readDesignPiecesShallowCommand: { readonly pieces: ReadonlyArray<unknown> } }
  | { readonly readDesignConnectionsFullCommand: { readonly connections: ReadonlyArray<unknown> } }
  | { readonly readDesignConnectionsShallowCommand: { readonly connections: ReadonlyArray<unknown> } }
  | { readonly readDesignLayersFullCommand: { readonly layers: ReadonlyArray<unknown> } }
  | { readonly readDesignLayersShallowCommand: { readonly layers: ReadonlyArray<unknown> } }
  | { readonly readDesignGroupsFullCommand: { readonly groups: ReadonlyArray<unknown> } }
  | { readonly readDesignGroupsShallowCommand: { readonly groups: ReadonlyArray<unknown> } }
  | { readonly readDesignAuthorsFullCommand: { readonly authors: ReadonlyArray<unknown> } }
  | { readonly readDesignAuthorsShallowCommand: { readonly authors: ReadonlyArray<unknown> } }
  | { readonly readDesignConceptsFullCommand: { readonly concepts: ReadonlyArray<unknown> } }
  | { readonly readDesignConceptsShallowCommand: { readonly concepts: ReadonlyArray<unknown> } }
  | { readonly readDesignTagsFullCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readDesignTagsShallowCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readDesignQualitiesFullCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readDesignQualitiesShallowCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readDesignPropsFullCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readDesignPropsShallowCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readDesignAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readDesignAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readDesignStatsFullCommand: { readonly stats: ReadonlyArray<unknown> } }
  | { readonly readDesignStatsShallowCommand: { readonly stats: ReadonlyArray<unknown> } }
  | { readonly readDesignFlattenMapCommand: { readonly entries: ReadonlyArray<unknown> } }
  | { readonly readDesignClusterableGroupsCommand: { readonly groups: ReadonlyArray<ReadonlyArray<IdDto>> } }
  | { readonly readDesignIncludedDesignsCommand: { readonly designs: ReadonlyArray<unknown> } }
  | {
    readonly readDesignReplaceableCatalogCommand: {
      readonly types: ReadonlyArray<IdDto>;
      readonly designs: ReadonlyArray<IdDto>;
    };
  }
  | { readonly readDesignIncludedDesignIdsCommand: { readonly designIds: ReadonlyArray<IdDto> } }
  | { readonly readDesignQualitySumCommand: { readonly sum: number } }
  | { readonly readDesignFamilyCommands: { readonly results: ReadonlyArray<ReadFamilyCommandOutput> } }
  | { readonly readDesignPieceCommands: { readonly results: ReadonlyArray<ReadPieceCommandOutput> } }
  | { readonly readDesignConnectionCommands: { readonly results: ReadonlyArray<ReadConnectionCommandOutput> } }
  | { readonly readDesignLayerCommands: { readonly results: ReadonlyArray<ReadLayerCommandOutput> } }
  | { readonly readDesignGroupCommands: { readonly results: ReadonlyArray<ReadGroupCommandOutput> } }
  | { readonly readDesignAuthorCommands: { readonly results: ReadonlyArray<ReadAuthorCommandOutput> } }
  | { readonly readDesignConceptCommands: { readonly results: ReadonlyArray<ReadConceptCommandOutput> } }
  | { readonly readDesignTagCommands: { readonly results: ReadonlyArray<ReadTagCommandOutput> } }
  | { readonly readDesignQualityCommands: { readonly results: ReadonlyArray<ReadQualityCommandOutput> } }
  | { readonly readDesignPropCommands: { readonly results: ReadonlyArray<ReadPropCommandOutput> } }
  | { readonly readDesignAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }
  | { readonly readDesignStatCommands: { readonly results: ReadonlyArray<ReadStatCommandOutput> } }

export type ReadFamilyCommandOutput =
  | { readonly readFamilyFullCommand: { readonly family: unknown } }
  | { readonly readFamilyShallowCommand: { readonly family: unknown } }
  | { readonly readFamilyMetadataCommand: { readonly metadata: unknown } }
  | { readonly readFamilyIdCommand: { readonly id: IdDto } }
  | { readonly readFamilyNameCommand: { readonly name: string } }
  | { readonly readFamilyDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readFamilyIconCommand: { readonly icon: (string | null | undefined) } }
  | { readonly readFamilyPortsFullCommand: { readonly ports: ReadonlyArray<unknown> } }
  | { readonly readFamilyPortsShallowCommand: { readonly ports: ReadonlyArray<unknown> } }
  | { readonly readFamilyAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readFamilyAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readFamilyPortCommands: { readonly results: ReadonlyArray<ReadPortCommandOutput> } }
  | { readonly readFamilyAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadFileCommandOutput =
  | { readonly readFileFullCommand: { readonly file: unknown } }
  | { readonly readFileShallowCommand: { readonly file: unknown } }
  | { readonly readFileMetadataCommand: { readonly metadata: unknown } }
  | { readonly readFileIdCommand: { readonly id: IdDto } }
  | { readonly readFileUrlCommand: { readonly url: string } }
  | { readonly readFileMimeCommand: { readonly mime: (string | null | undefined) } }
  | { readonly readFileSizeCommand: { readonly size: (number | null | undefined) } }
  | { readonly readFileHashCommand: { readonly hash: (string | null | undefined) } }
  | { readonly readFileDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readFileCreatedCommand: { readonly created: (string | null | undefined) } }
  | { readonly readFileUpdatedCommand: { readonly updated: (string | null | undefined) } }

export type ReadFolderCommandOutput =
  | { readonly readFolderFullCommand: { readonly folder: unknown } }
  | { readonly readFolderShallowCommand: { readonly folder: unknown } }
  | { readonly readFolderMetadataCommand: { readonly metadata: unknown } }
  | { readonly readFolderIdCommand: { readonly id: IdDto } }
  | { readonly readFolderPathCommand: { readonly path: string } }
  | { readonly readFolderDescriptionCommand: { readonly description: (string | null | undefined) } }

export type ReadGroupCommandOutput =
  | { readonly readGroupFullCommand: { readonly group: unknown } }
  | { readonly readGroupShallowCommand: { readonly group: unknown } }
  | { readonly readGroupMetadataCommand: { readonly metadata: unknown } }
  | { readonly readGroupIdCommand: { readonly id: IdDto } }
  | { readonly readGroupNameCommand: { readonly name: string } }
  | { readonly readGroupDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readGroupColorCommand: { readonly color: (string | null | undefined) } }
  | { readonly readGroupIconCommand: { readonly icon: (string | null | undefined) } }
  | { readonly readGroupPiecesCommand: { readonly pieces: ReadonlyArray<IdDto> } }

export type ReadKitCommandOutput =
  | { readonly readKitFullCommand: { readonly kit: unknown } }
  | { readonly readKitShallowCommand: { readonly kit: unknown } }
  | { readonly readKitMetadataCommand: { readonly metadata: unknown } }
  | { readonly readKitIdCommand: { readonly id: IdDto } }
  | { readonly readKitNameCommand: { readonly name: string } }
  | { readonly readKitDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readKitIconCommand: { readonly icon: (string | null | undefined) } }
  | { readonly readKitImageCommand: { readonly image: (string | null | undefined) } }
  | { readonly readKitPreviewCommand: { readonly preview: (string | null | undefined) } }
  | { readonly readKitRemoteCommand: { readonly remote: (string | null | undefined) } }
  | { readonly readKitHomepageCommand: { readonly homepage: (string | null | undefined) } }
  | { readonly readKitLicenseCommand: { readonly license: (string | null | undefined) } }
  | { readonly readKitUriCommand: { readonly uri: (string | null | undefined) } }
  | { readonly readKitCreatedCommand: { readonly created: (string | null | undefined) } }
  | { readonly readKitUpdatedCommand: { readonly updated: (string | null | undefined) } }
  | { readonly readKitTypesFullCommand: { readonly types: ReadonlyArray<unknown> } }
  | { readonly readKitTypesShallowCommand: { readonly types: ReadonlyArray<unknown> } }
  | { readonly readKitTypeIdsCommand: { readonly typeIds: ReadonlyArray<IdDto> } }
  | { readonly readKitTypesMetadataCommand: { readonly types: ReadonlyArray<unknown> } }
  | { readonly readKitDesignsFullCommand: { readonly designs: ReadonlyArray<unknown> } }
  | { readonly readKitDesignsShallowCommand: { readonly designs: ReadonlyArray<unknown> } }
  | { readonly readKitDesignIdsCommand: { readonly designIds: ReadonlyArray<IdDto> } }
  | { readonly readKitDesignsMetadataCommand: { readonly designs: ReadonlyArray<unknown> } }
  | { readonly readKitFilesFullCommand: { readonly files: ReadonlyArray<unknown> } }
  | { readonly readKitFilesShallowCommand: { readonly files: ReadonlyArray<unknown> } }
  | { readonly readKitFoldersFullCommand: { readonly folders: ReadonlyArray<unknown> } }
  | { readonly readKitFoldersShallowCommand: { readonly folders: ReadonlyArray<unknown> } }
  | { readonly readKitLocationsFullCommand: { readonly locations: ReadonlyArray<unknown> } }
  | { readonly readKitLocationsShallowCommand: { readonly locations: ReadonlyArray<unknown> } }
  | { readonly readKitFamiliesFullCommand: { readonly families: ReadonlyArray<unknown> } }
  | { readonly readKitFamiliesShallowCommand: { readonly families: ReadonlyArray<unknown> } }
  | { readonly readKitPortsFullCommand: { readonly ports: ReadonlyArray<unknown> } }
  | { readonly readKitAuthorsFullCommand: { readonly authors: ReadonlyArray<unknown> } }
  | { readonly readKitAuthorsShallowCommand: { readonly authors: ReadonlyArray<unknown> } }
  | { readonly readKitConceptsFullCommand: { readonly concepts: ReadonlyArray<unknown> } }
  | { readonly readKitConceptsShallowCommand: { readonly concepts: ReadonlyArray<unknown> } }
  | { readonly readKitTagsFullCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readKitTagsShallowCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readKitQualitiesFullCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readKitQualitiesShallowCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readKitPropsFullCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readKitPropsShallowCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readKitAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readKitAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readKitColoredConnectorsCommand: { readonly rows: ReadonlyArray<unknown> } }
  | { readonly readKitTypeCommands: { readonly results: ReadonlyArray<ReadTypeCommandOutput> } }
  | { readonly readKitDesignCommands: { readonly results: ReadonlyArray<ReadDesignCommandOutput> } }
  | { readonly readKitFileCommands: { readonly results: ReadonlyArray<ReadFileCommandOutput> } }
  | { readonly readKitFolderCommands: { readonly results: ReadonlyArray<ReadFolderCommandOutput> } }
  | { readonly readKitLocationCommands: { readonly results: ReadonlyArray<ReadLocationCommandOutput> } }
  | { readonly readKitFamilyCommands: { readonly results: ReadonlyArray<ReadFamilyCommandOutput> } }
  | { readonly readKitPortCommands: { readonly results: ReadonlyArray<ReadPortCommandOutput> } }
  | { readonly readKitAuthorCommands: { readonly results: ReadonlyArray<ReadAuthorCommandOutput> } }
  | { readonly readKitConceptCommands: { readonly results: ReadonlyArray<ReadConceptCommandOutput> } }
  | { readonly readKitTagCommands: { readonly results: ReadonlyArray<ReadTagCommandOutput> } }
  | { readonly readKitQualityCommands: { readonly results: ReadonlyArray<ReadQualityCommandOutput> } }
  | { readonly readKitPropCommands: { readonly results: ReadonlyArray<ReadPropCommandOutput> } }
  | { readonly readKitAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadLayerCommandOutput =
  | { readonly readLayerFullCommand: { readonly layer: unknown } }
  | { readonly readLayerShallowCommand: { readonly layer: unknown } }
  | { readonly readLayerMetadataCommand: { readonly metadata: unknown } }
  | { readonly readLayerIdCommand: { readonly id: IdDto } }
  | { readonly readLayerNameCommand: { readonly name: string } }
  | { readonly readLayerDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readLayerColorCommand: { readonly color: (string | null | undefined) } }
  | { readonly readLayerOrderCommand: { readonly order: (number | null | undefined) } }
  | { readonly readLayerVisibleCommand: { readonly visible: (boolean | null | undefined) } }
  | { readonly readLayerLockedCommand: { readonly locked: (boolean | null | undefined) } }

export type ReadLocationCommandOutput =
  | { readonly readLocationFullCommand: { readonly location: unknown } }
  | { readonly readLocationShallowCommand: { readonly location: unknown } }
  | { readonly readLocationMetadataCommand: { readonly metadata: unknown } }
  | { readonly readLocationIdCommand: { readonly id: IdDto } }
  | { readonly readLocationLongitudeCommand: { readonly longitude: number } }
  | { readonly readLocationLatitudeCommand: { readonly latitude: number } }
  | { readonly readLocationAltitudeCommand: { readonly altitude: (number | null | undefined) } }
  | { readonly readLocationAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readLocationAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readLocationAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadPieceCommandOutput =
  | { readonly readPieceFullCommand: { readonly dto: unknown } }
  | { readonly readPieceShallowCommand: { readonly dto: unknown } }
  | { readonly readPieceMetadataCommand: { readonly metadata: unknown } }
  | { readonly readPieceIdCommand: { readonly id: IdDto } }
  | { readonly readPieceNameCommand: { readonly name: (string | null | undefined) } }
  | { readonly readPieceDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readPiecePlaneCommand: { readonly plane: (unknown | null | undefined) } }
  | { readonly readPieceCenterCommand: { readonly center: (unknown | null | undefined) } }
  | { readonly readPieceScaleCommand: { readonly scale: (number | null | undefined) } }
  | { readonly readPieceMirrorPlaneCommand: { readonly mirrorPlane: (unknown | null | undefined) } }
  | { readonly readPieceHiddenCommand: { readonly hidden: (boolean | null | undefined) } }
  | { readonly readPieceLockedCommand: { readonly locked: (boolean | null | undefined) } }
  | { readonly readPieceColorCommand: { readonly color: (string | null | undefined) } }
  | { readonly readPieceTypeCommand: { readonly type: (IdDto | null | undefined) } }
  | { readonly readPieceDesignCommand: { readonly design: (IdDto | null | undefined) } }
  | { readonly readPiecePropsFullCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readPiecePropsShallowCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readPieceAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readPieceAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readPieceFlatPlaneCommand: { readonly flatPlane: unknown } }
  | { readonly readPieceFlatCenterCommand: { readonly flatCenter: unknown } }
  | { readonly readPieceFlatPoseCommand: { readonly flatPose: unknown } }
  | { readonly readPiecePathCommand: { readonly path: ReadonlyArray<IdDto> } }
  | { readonly readPieceParentPieceIdCommand: { readonly parentPiece: (IdDto | null | undefined) } }
  | { readonly readPieceParentConnectionIdCommand: { readonly parentConnection: (IdDto | null | undefined) } }
  | { readonly readPieceParentConnectionFullCommand: { readonly connection: unknown } }
  | { readonly readPieceParentDesignIdCommand: { readonly parentDesign: IdDto } }
  | { readonly readPieceFixedCommand: { readonly fixed: unknown } }
  | { readonly readPieceConnectedCommand: { readonly connected: unknown } }
  | { readonly readPieceAlternativesCommand: { readonly alternatives: unknown } }
  | { readonly readPieceAlternativeTypesCommand: { readonly types: ReadonlyArray<IdDto> } }
  | { readonly readPieceAlternativeDesignsCommand: { readonly designs: ReadonlyArray<IdDto> } }
  | { readonly readPiecePropCommands: { readonly results: ReadonlyArray<ReadPropCommandOutput> } }
  | { readonly readPieceAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadPortCommandOutput =
  | { readonly readPortFullCommand: { readonly port: unknown } }
  | { readonly readPortShallowCommand: { readonly port: unknown } }
  | { readonly readPortMetadataCommand: { readonly metadata: unknown } }
  | { readonly readPortIdCommand: { readonly id: IdDto } }
  | { readonly readPortNameCommand: { readonly name: string } }
  | { readonly readPortDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readPortIconCommand: { readonly icon: (string | null | undefined) } }
  | { readonly readPortCompatibleFamiliesCommand: { readonly families: ReadonlyArray<IdDto> } }
  | { readonly readPortMandatoryCommand: { readonly mandatory: (boolean | null | undefined) } }
  | { readonly readPortTCommand: { readonly t: (number | null | undefined) } }
  | { readonly readPortPointCommand: { readonly point: (unknown | null | undefined) } }
  | { readonly readPortDirectionCommand: { readonly direction: (unknown | null | undefined) } }
  | { readonly readPortCompatiblePortsCommand: { readonly compatiblePorts: ReadonlyArray<IdDto> } }
  | { readonly readPortQualitiesFullCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readPortQualitiesShallowCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readPortAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readPortAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readPortQualityCommands: { readonly results: ReadonlyArray<ReadQualityCommandOutput> } }
  | { readonly readPortAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadPropCommandOutput =
  | { readonly readPropFullCommand: { readonly prop: unknown } }
  | { readonly readPropShallowCommand: { readonly prop: unknown } }
  | { readonly readPropIdCommand: { readonly id: IdDto } }
  | { readonly readPropKeyCommand: { readonly key: string } }
  | { readonly readPropValueCommand: { readonly value: string } }
  | { readonly readPropUnitCommand: { readonly unit: (string | null | undefined) } }
  | { readonly readPropQualityIdCommand: { readonly quality: (IdDto | null | undefined) } }

export type ReadQualityCommandOutput =
  | { readonly readQualityFullCommand: { readonly quality: unknown } }
  | { readonly readQualityShallowCommand: { readonly quality: unknown } }
  | { readonly readQualityMetadataCommand: { readonly metadata: unknown } }
  | { readonly readQualityIdCommand: { readonly id: IdDto } }
  | { readonly readQualityKeyCommand: { readonly key: string } }
  | { readonly readQualityValueCommand: { readonly value: (string | null | undefined) } }
  | { readonly readQualityUnitCommand: { readonly unit: (string | null | undefined) } }
  | { readonly readQualityDefinitionCommand: { readonly definition: (string | null | undefined) } }
  | { readonly readQualityDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readQualityBenchmarksFullCommand: { readonly benchmarks: ReadonlyArray<unknown> } }
  | { readonly readQualityBenchmarksShallowCommand: { readonly benchmarks: ReadonlyArray<unknown> } }
  | { readonly readQualityBenchmarkCommands: { readonly results: ReadonlyArray<ReadBenchmarkCommandOutput> } }

export type ReadRepresentationCommandOutput =
  | { readonly readRepresentationFullCommand: { readonly representation: unknown } }
  | { readonly readRepresentationShallowCommand: { readonly representation: unknown } }
  | { readonly readRepresentationMetadataCommand: { readonly metadata: unknown } }
  | { readonly readRepresentationIdCommand: { readonly id: IdDto } }
  | { readonly readRepresentationUrlCommand: { readonly url: string } }
  | { readonly readRepresentationDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readRepresentationFileIdCommand: { readonly file: (IdDto | null | undefined) } }
  | { readonly readRepresentationTagsFullCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readRepresentationTagsShallowCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readRepresentationQualitiesFullCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readRepresentationQualitiesShallowCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readRepresentationAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readRepresentationAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readRepresentationTagCommands: { readonly results: ReadonlyArray<ReadTagCommandOutput> } }
  | { readonly readRepresentationQualityCommands: { readonly results: ReadonlyArray<ReadQualityCommandOutput> } }
  | { readonly readRepresentationAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

export type ReadSideCommandOutput =
  | { readonly readSideFullCommand: { readonly side: unknown } }
  | { readonly readSideShallowCommand: { readonly side: unknown } }
  | { readonly readSideMetadataCommand: { readonly side: unknown } }
  | { readonly readSideIdCommand: { readonly id: IdDto } }
  | { readonly readSidePieceIdCommand: { readonly piece: IdDto } }
  | { readonly readSidePortIdCommand: { readonly port: (IdDto | null | undefined) } }
  | { readonly readSideDesignPieceIdCommand: { readonly designPiece: (IdDto | null | undefined) } }

export type ReadStatCommandOutput =
  | { readonly readStatFullCommand: { readonly stat: unknown } }
  | { readonly readStatShallowCommand: { readonly stat: unknown } }
  | { readonly readStatMetadataCommand: { readonly metadata: unknown } }
  | { readonly readStatIdCommand: { readonly id: IdDto } }
  | { readonly readStatKeyCommand: { readonly key: string } }
  | { readonly readStatValueCommand: { readonly value: string } }
  | { readonly readStatUnitCommand: { readonly unit: (string | null | undefined) } }
  | { readonly readStatDescriptionCommand: { readonly description: (string | null | undefined) } }

export type ReadTagCommandOutput =
  | { readonly readTagFullCommand: { readonly tag: unknown } }
  | { readonly readTagShallowCommand: { readonly tag: unknown } }
  | { readonly readTagMetadataCommand: { readonly metadata: unknown } }
  | { readonly readTagIdCommand: { readonly id: IdDto } }
  | { readonly readTagNameCommand: { readonly name: string } }
  | { readonly readTagOrderCommand: { readonly order: (number | null | undefined) } }

export type ReadTypeCommandOutput =
  | { readonly readTypeFullCommand: { readonly dto: unknown } }
  | { readonly readTypeShallowCommand: { readonly dto: unknown } }
  | { readonly readTypeMetadataCommand: { readonly metadata: unknown } }
  | { readonly readTypeIdCommand: { readonly id: IdDto } }
  | { readonly readTypeNameCommand: { readonly name: string } }
  | { readonly readTypeDescriptionCommand: { readonly description: (string | null | undefined) } }
  | { readonly readTypeIconCommand: { readonly icon: (string | null | undefined) } }
  | { readonly readTypeImageCommand: { readonly image: (string | null | undefined) } }
  | { readonly readTypeStockCommand: { readonly stock: (number | null | undefined) } }
  | { readonly readTypeVirtualCommand: { readonly virtual: (boolean | null | undefined) } }
  | { readonly readTypeUnitCommand: { readonly unit: (string | null | undefined) } }
  | { readonly readTypeLocationCommand: { readonly location: (IdDto | null | undefined) } }
  | { readonly readTypeCreatedCommand: { readonly created: (string | null | undefined) } }
  | { readonly readTypeUpdatedCommand: { readonly updated: (string | null | undefined) } }
  | { readonly readTypeFamiliesCommand: { readonly families: ReadonlyArray<IdDto> } }
  | { readonly readTypeConnectorsFullCommand: { readonly connectors: ReadonlyArray<unknown> } }
  | { readonly readTypeConnectorsShallowCommand: { readonly connectors: ReadonlyArray<unknown> } }
  | { readonly readTypeRepresentationsFullCommand: { readonly representations: ReadonlyArray<unknown> } }
  | { readonly readTypeRepresentationsShallowCommand: { readonly representations: ReadonlyArray<unknown> } }
  | { readonly readTypeAuthorsFullCommand: { readonly authors: ReadonlyArray<unknown> } }
  | { readonly readTypeAuthorsShallowCommand: { readonly authors: ReadonlyArray<unknown> } }
  | { readonly readTypeConceptsFullCommand: { readonly concepts: ReadonlyArray<unknown> } }
  | { readonly readTypeConceptsShallowCommand: { readonly concepts: ReadonlyArray<unknown> } }
  | { readonly readTypeTagsFullCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readTypeTagsShallowCommand: { readonly tags: ReadonlyArray<unknown> } }
  | { readonly readTypeQualitiesFullCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readTypeQualitiesShallowCommand: { readonly qualities: ReadonlyArray<unknown> } }
  | { readonly readTypePropsFullCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readTypePropsShallowCommand: { readonly props: ReadonlyArray<unknown> } }
  | { readonly readTypeAttributesFullCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readTypeAttributesShallowCommand: { readonly attributes: ReadonlyArray<unknown> } }
  | { readonly readTypePortsFullCommand: { readonly ports: ReadonlyArray<unknown> } }
  | { readonly readTypeConnectorForPortIdCommand: { readonly connector: (unknown | null | undefined) } }
  | { readonly readTypeBestRepresentationCommand: { readonly representation: (unknown | null | undefined) } }
  | { readonly readTypeFamilyCommands: { readonly results: ReadonlyArray<ReadFamilyCommandOutput> } }
  | { readonly readTypeConnectorCommands: { readonly results: ReadonlyArray<ReadConnectorCommandOutput> } }
  | { readonly readTypeRepresentationCommands: { readonly results: ReadonlyArray<ReadRepresentationCommandOutput> } }
  | { readonly readTypePortCommands: { readonly results: ReadonlyArray<ReadPortCommandOutput> } }
  | { readonly readTypeAuthorCommands: { readonly results: ReadonlyArray<ReadAuthorCommandOutput> } }
  | { readonly readTypeConceptCommands: { readonly results: ReadonlyArray<ReadConceptCommandOutput> } }
  | { readonly readTypeTagCommands: { readonly results: ReadonlyArray<ReadTagCommandOutput> } }
  | { readonly readTypeQualityCommands: { readonly results: ReadonlyArray<ReadQualityCommandOutput> } }
  | { readonly readTypePropCommands: { readonly results: ReadonlyArray<ReadPropCommandOutput> } }
  | { readonly readTypeAttributeCommands: { readonly results: ReadonlyArray<ReadAttributeCommandOutput> } }

/** Externally-tagged `ReadKitCommand` JSON keys (camelCase). */
export const ALL_READ_KIT_COMMAND_KEYS = ['readKitFullCommand', 'readKitShallowCommand', 'readKitMetadataCommand', 'readKitIdCommand', 'readKitNameCommand', 'readKitDescriptionCommand', 'readKitIconCommand', 'readKitImageCommand', 'readKitPreviewCommand', 'readKitRemoteCommand', 'readKitHomepageCommand', 'readKitLicenseCommand', 'readKitUriCommand', 'readKitCreatedCommand', 'readKitUpdatedCommand', 'readKitTypesFullCommand', 'readKitTypesShallowCommand', 'readKitTypeIdsCommand', 'readKitTypesMetadataCommand', 'readKitDesignsFullCommand', 'readKitDesignsShallowCommand', 'readKitDesignIdsCommand', 'readKitDesignsMetadataCommand', 'readKitFilesFullCommand', 'readKitFilesShallowCommand', 'readKitFoldersFullCommand', 'readKitFoldersShallowCommand', 'readKitLocationsFullCommand', 'readKitLocationsShallowCommand', 'readKitFamiliesFullCommand', 'readKitFamiliesShallowCommand', 'readKitPortsFullCommand', 'readKitAuthorsFullCommand', 'readKitAuthorsShallowCommand', 'readKitConceptsFullCommand', 'readKitConceptsShallowCommand', 'readKitTagsFullCommand', 'readKitTagsShallowCommand', 'readKitQualitiesFullCommand', 'readKitQualitiesShallowCommand', 'readKitPropsFullCommand', 'readKitPropsShallowCommand', 'readKitAttributesFullCommand', 'readKitAttributesShallowCommand', 'readKitColoredConnectorsCommand', 'readKitTypeCommands', 'readKitDesignCommands', 'readKitFileCommands', 'readKitFolderCommands', 'readKitLocationCommands', 'readKitFamilyCommands', 'readKitPortCommands', 'readKitAuthorCommands', 'readKitConceptCommands', 'readKitTagCommands', 'readKitQualityCommands', 'readKitPropCommands', 'readKitAttributeCommands'] as const;
export type AllReadKitCommandKey = (typeof ALL_READ_KIT_COMMAND_KEYS)[number];

export type ReadRootCommand = ReadKitCommand;
export type ReadCommandBatch = ReadonlyArray<ReadKitCommand>;
export type ReadCommandBatchResult = ReadonlyArray<ReadKitCommandOutput>;

//#endregion 📖ReadCommandTypes

//#region 🔖KitGraphqlWire
/**
 * Live kit reads use GraphQL fields on `kitStore` / `designForId` / `pieceForId` (no `readKitCommands` batch).
 * VCS uses typed root mutations; shape of each result is the same tagged JSON as `KitStoreCommandResult`.
 */

/** WASM [`KitStoreHandle::execute`] shape: streams JSON-stringified GraphQL responses. */
export type KitGraphqlHandle = {
  execute(requestJson: string, onMessage: (line: string) => void): Promise<void>;
};

export async function kitGraphqlRun(
  handle: KitGraphqlHandle,
  body: { query: string; variables?: Record<string, unknown>; operationName?: string },
): Promise<unknown[]> {
  const out: unknown[] = [];
  await handle.execute(JSON.stringify(body), (line: string) => {
    out.push(JSON.parse(line));
  });
  return out;
}

export function kitGraphqlFirstData(msgs: unknown[]): Record<string, unknown> {
  for (const m of msgs) {
    if (m == null || typeof m !== "object") continue;
    const r = m as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
    if (Array.isArray(r.errors) && r.errors.length > 0) {
      throw new Error(r.errors[0]?.message ?? "GraphQL error");
    }
    if (r.data != null && typeof r.data === "object") {
      return r.data as Record<string, unknown>;
    }
  }
  throw new Error("kitGraphql: no data in response");
}

function storePayload(cmd: unknown): { tag: string; value: unknown } {
  if (cmd == null || typeof cmd !== "object" || Array.isArray(cmd)) {
    throw new Error("kit store command: expected object");
  }
  const o = cmd as Record<string, unknown>;
  const keys = Object.keys(o);
  if (keys.length !== 1) {
    throw new Error("kit store command: expected a single tagged variant");
  }
  const tag = keys[0]!;
  return { tag, value: o[tag] };
}

/** Maps `KitStoreCommand` JSON to typed root mutations; returns the tagged `KitStoreCommandResult` JSON. */
export async function kitGraphqlExecuteStoreCommand(handle: KitGraphqlHandle, cmd: unknown): Promise<unknown> {
  const { tag, value } = storePayload(cmd);
  const data = await kitGraphqlRun(handle, (() => {
    switch (tag) {
      case "newSession":
        return { query: `mutation { newSession }` };
      case "endSession": {
        const id = (value as { id?: string } | null)?.id;
        if (typeof id !== "string") throw new Error("endSession: need id");
        return { query: `mutation($id: String!) { endSession(id: $id) }`, variables: { id } };
      }
      case "newAlternative": {
        const v = value as { fromCheckpoint?: string | null; name: string } | null;
        if (v == null || typeof v.name !== "string") throw new Error("newAlternative");
        return {
          query: `mutation($fromCheckpoint: String, $name: String!) { newAlternative(fromCheckpoint: $fromCheckpoint, name: $name) }`,
          variables: { fromCheckpoint: v.fromCheckpoint ?? null, name: v.name },
        };
      }
      case "executeSessionCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        const id = v?.id;
        const sc = v?.commands;
        if (typeof id !== "string" || !Array.isArray(sc)) throw new Error("executeSessionCommands");
        return {
          query: `mutation($sessionId: String!, $sessionCommands: [JSON!]!) { executeSessionCommands(sessionId: $sessionId, sessionCommands: $sessionCommands) }`,
          variables: { sessionId: id, sessionCommands: sc },
        };
      }
      case "executeKitCheckpointCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        if (typeof v?.id !== "string" || !Array.isArray(v?.commands)) throw new Error("executeKitCheckpointCommands");
        return {
          query: `mutation($checkpointId: String!, $commands: [JSON!]!) { executeKitCheckpointCommands(checkpointId: $checkpointId, commands: $commands) }`,
          variables: { checkpointId: v.id, commands: v.commands },
        };
      }
      case "executeKitAlternativeCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        if (typeof v?.id !== "string" || !Array.isArray(v?.commands)) throw new Error("executeKitAlternativeCommands");
        return {
          query: `mutation($alternativeId: String!, $commands: [JSON!]!) { executeKitAlternativeCommands(alternativeId: $alternativeId, commands: $commands) }`,
          variables: { alternativeId: v.id, commands: v.commands },
        };
      }
      case "attachBackbone": {
        const cfg = (value as { config?: unknown } | null)?.config;
        return {
          query: `mutation($config: JSON!) { attachBackbone(config: $config) }`,
          variables: { config: cfg },
        };
      }
      case "detachBackbone":
        return { query: `mutation { detachBackbone }` };
      case "setActiveCheckpoint": {
        const id = (value as { id?: string | null } | null)?.id ?? null;
        return {
          query: `mutation($id: String) { setActiveCheckpoint(id: $id) }`,
          variables: { id },
        };
      }
      case "listConflicts":
        return { query: `mutation { listConflicts }` };
      case "resolveConflict": {
        const v = value as { id?: string; strategy?: unknown } | null;
        if (typeof v?.id !== "string") throw new Error("resolveConflict");
        return {
          query: `mutation($id: String!, $strategy: JSON!) { resolveConflict(id: $id, strategy: $strategy) }`,
          variables: { id: v.id, strategy: v.strategy },
        };
      }
      case "backboneStatus":
        return { query: `mutation { backboneStatus }` };
      case "syncNow":
        return { query: `mutation { syncNow }` };
      case "batch": {
        const cmds = (value as { commands?: unknown[] } | null)?.commands;
        if (!Array.isArray(cmds)) throw new Error("batch.commands");
        return { query: `mutation($commands: [JSON!]!) { kitStoreBatch(commands: $commands) }`, variables: { commands: cmds } };
      }
      default:
        throw new Error(`[DEBUG] kitGraphqlExecuteStoreCommand: unhandled ${tag}`);
    }
  })());
  const root = kitGraphqlFirstData(data);
  const op = Object.keys(root)[0];
  if (op === undefined) throw new Error("kitGraphql: empty mutation data");
  return root[op];
}

/** Fan-out kit events from `subscription { eventStream }`; cancel stops invoking `sink` (underlying stream may continue). */
export function kitGraphqlSubscribeLoop(handle: KitGraphqlHandle, sink: (payload: unknown) => void): () => void {
  let cancelled = false;
  void handle
    .execute(JSON.stringify({ query: "subscription { eventStream }" }), (line: string) => {
      if (cancelled) return;
      try {
        const msg = JSON.parse(line) as { data?: { eventStream?: unknown } | null; errors?: unknown[] };
        if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return;
        if (msg.data && "eventStream" in msg.data && msg.data.eventStream !== undefined) {
          sink(msg.data.eventStream);
        }
      } catch {
        /* ignore */
      }
    })
    .catch(() => { });
  return () => {
    cancelled = true;
  };
}

/**
 * 🌐 Design flatten placement map JSON (`piecesMetadataJson`) — used when no `readDesign*Metadata` read variant exists yet.
 */
export async function kitGraphqlKitDesignPiecesMetadata(handle: KitGraphqlHandle, designId: string): Promise<Record<string, unknown>> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, {
      query: `query($id: String!) { kitStore { designForId(id: $id) { piecesMetadataJson } } }`,
      variables: { id: designId },
    }),
  ) as { kitStore?: { designForId?: { piecesMetadataJson?: unknown } | null } };
  const v = root.kitStore?.designForId?.piecesMetadataJson;
  if (v && typeof v === "object" && !Array.isArray(v)) {
    return v as Record<string, unknown>;
  }
  return {};
}

/** @emoji 📌 GraphQL `Json` → array (catalog shallow rows). */
function kitGraphqlCatalogJsonArray(v: unknown): unknown[] {
  if (Array.isArray(v)) {
    return v;
  }
  if (typeof v === "string") {
    try {
      const p = JSON.parse(v) as unknown;
      return Array.isArray(p) ? p : [];
    } catch {
      return [];
    }
  }
  return [];
}

/** @emoji 📌 `designsShallowJson` — same contract as legacy wasm `getDesigns`. */
export async function kitGraphqlKitDesignsShallow(handle: KitGraphqlHandle): Promise<unknown[]> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, { query: `query { kitStore { designsShallowJson } }` }),
  ) as { kitStore?: { designsShallowJson?: unknown } };
  return kitGraphqlCatalogJsonArray(root.kitStore?.designsShallowJson);
}

/** @emoji 📌 `typesShallowJson` — same as wasm `getTypes`. */
export async function kitGraphqlKitTypesShallow(handle: KitGraphqlHandle): Promise<unknown[]> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, { query: `query { kitStore { typesShallowJson } }` }),
  ) as { kitStore?: { typesShallowJson?: unknown } };
  return kitGraphqlCatalogJsonArray(root.kitStore?.typesShallowJson);
}

/** @emoji 📌 `authorsShallowJson` — same as wasm `getAuthors`. */
export async function kitGraphqlKitAuthorsShallow(handle: KitGraphqlHandle): Promise<unknown[]> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, { query: `query { kitStore { authorsShallowJson } }` }),
  ) as { kitStore?: { authorsShallowJson?: unknown } };
  return kitGraphqlCatalogJsonArray(root.kitStore?.authorsShallowJson);
}

/** @emoji 📌 `kitMetadataJson` — same as wasm `getKitMetadata`. */
export async function kitGraphqlKitMetadataJson(handle: KitGraphqlHandle): Promise<unknown> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, { query: `query { kitStore { kitMetadataJson } }` }),
  ) as { kitStore?: { kitMetadataJson?: unknown } };
  return root.kitStore?.kitMetadataJson;
}

/** @emoji 📌 `piecesFullJson` for a design — same as wasm `getPieces`. */
export async function kitGraphqlKitDesignPiecesFull(handle: KitGraphqlHandle, designId: string): Promise<unknown[]> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, {
      query: `query($id: String!) { kitStore { designForId(id: $id) { piecesFullJson } } }`,
      variables: { id: designId },
    }),
  ) as { kitStore?: { designForId?: { piecesFullJson?: unknown } | null } };
  return kitGraphqlCatalogJsonArray(root.kitStore?.designForId?.piecesFullJson);
}

/** @emoji 📌 `connectionsFullJson` for a design — same as wasm `getConnections`. */
export async function kitGraphqlKitDesignConnectionsFull(handle: KitGraphqlHandle, designId: string): Promise<unknown[]> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, {
      query: `query($id: String!) { kitStore { designForId(id: $id) { connectionsFullJson } } }`,
      variables: { id: designId },
    }),
  ) as { kitStore?: { designForId?: { connectionsFullJson?: unknown } | null } };
  return kitGraphqlCatalogJsonArray(root.kitStore?.designForId?.connectionsFullJson);
}

//#endregion 🔖KitGraphqlWire

//#region 🔖KitGraphLive

/** @emoji 📌 `{ id }` DTO helper for GraphQL variables. */
export function idDto(id: string): IdDto {
  return { id };
}

function kitGraphqlJsonToReadonlyArray(v: unknown): ReadonlyArray<unknown> {
  if (Array.isArray(v)) return v;
  if (v == null) return [];
  if (typeof v === "string") {
    try {
      const p = JSON.parse(v) as unknown;
      return Array.isArray(p) ? p : [];
    } catch {
      return [];
    }
  }
  return [];
}

/** Maps one `ReadKitCommand` to field-based GraphQL (no read-command batch API). */
export async function kitGraphqlMapReadCommand(handle: KitGraphqlHandle, c: ReadKitCommand): Promise<ReadKitCommandOutput> {
  if ("readKitTypeIdsCommand" in c && c.readKitTypeIdsCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { typeIds } }` }),
    ) as { kitStore?: { typeIds?: string[] } };
    const typeIds = d.kitStore?.typeIds;
    if (!Array.isArray(typeIds)) throw new Error("typeIds");
    return { readKitTypeIdsCommand: { typeIds: typeIds.map((id) => idDto(id)) } };
  }
  if ("readKitTypesMetadataCommand" in c && c.readKitTypesMetadataCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { typesMetadata } }` }),
    ) as { kitStore?: { typesMetadata?: unknown } };
    return { readKitTypesMetadataCommand: { types: d.kitStore?.typesMetadata as any } };
  }
  if ("readKitDesignIdsCommand" in c && c.readKitDesignIdsCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { designIds } }` }),
    ) as { kitStore?: { designIds?: string[] } };
    const designIds = d.kitStore?.designIds;
    if (!Array.isArray(designIds)) throw new Error("designIds");
    return { readKitDesignIdsCommand: { designIds: designIds.map((id) => idDto(id)) } };
  }
  if ("readKitDesignsMetadataCommand" in c && c.readKitDesignsMetadataCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { designsMetadata } }` }),
    ) as { kitStore?: { designsMetadata?: unknown } };
    return { readKitDesignsMetadataCommand: { designs: d.kitStore?.designsMetadata as any } };
  }
  if ("readKitColoredConnectorsCommand" in c && c.readKitColoredConnectorsCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { coloredConnectors } }` }),
    ) as { kitStore?: { coloredConnectors?: unknown } };
    return { readKitColoredConnectorsCommand: { rows: d.kitStore?.coloredConnectors as any } };
  }
  if ("readKitNameCommand" in c && c.readKitNameCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { name } }` }),
    ) as { kitStore?: { name?: string } };
    if (d.kitStore?.name == null) throw new Error("kit name");
    return { readKitNameCommand: { name: d.kitStore.name } };
  }
  if ("readKitMetadataCommand" in c && c.readKitMetadataCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { kitMetadataJson } }` }),
    ) as { kitStore?: { kitMetadataJson?: unknown } };
    return { readKitMetadataCommand: { metadata: d.kitStore?.kitMetadataJson } };
  }
  if ("readKitTypesShallowCommand" in c && c.readKitTypesShallowCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { typesShallowJson } }` }),
    ) as { kitStore?: { typesShallowJson?: unknown } };
    return { readKitTypesShallowCommand: { types: kitGraphqlJsonToReadonlyArray(d.kitStore?.typesShallowJson) } };
  }
  if ("readKitDesignsShallowCommand" in c && c.readKitDesignsShallowCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { designsShallowJson } }` }),
    ) as { kitStore?: { designsShallowJson?: unknown } };
    return { readKitDesignsShallowCommand: { designs: kitGraphqlJsonToReadonlyArray(d.kitStore?.designsShallowJson) } };
  }
  if ("readKitAuthorsShallowCommand" in c && c.readKitAuthorsShallowCommand === null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { authorsShallowJson } }` }),
    ) as { kitStore?: { authorsShallowJson?: unknown } };
    return { readKitAuthorsShallowCommand: { authors: kitGraphqlJsonToReadonlyArray(d.kitStore?.authorsShallowJson) } };
  }
  if ("readKitDesignCommands" in c && c.readKitDesignCommands) {
    const { id, commands } = c.readKitDesignCommands;
    const out: ReadDesignCommandOutput[] = [];
    for (const sub of commands) {
      out.push(await kitGraphqlMapDesignRead(handle, id.id, sub));
    }
    return { readKitDesignCommands: { results: out } };
  }
  if ("readKitTypeCommands" in c && c.readKitTypeCommands) {
    const { id, commands } = c.readKitTypeCommands;
    const out: ReadTypeCommandOutput[] = [];
    for (const sub of commands) {
      out.push(await kitGraphqlMapTypeRead(handle, id.id, sub));
    }
    return { readKitTypeCommands: { results: out } };
  }
  throw new Error(`[DEBUG] kitGraphql: unsupported read command ${Object.keys(c).join(",")}`);
}

async function kitGraphqlMapDesignRead(
  handle: KitGraphqlHandle,
  designId: string,
  cmd: ReadDesignCommand,
): Promise<ReadDesignCommandOutput> {
  if ("readDesignClusterableGroupsCommand" in cmd && cmd.readDesignClusterableGroupsCommand) {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { clusterableGroups(selection: $sel) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, {
        query: q,
        variables: { id: designId, sel: cmd.readDesignClusterableGroupsCommand.selection.map((x) => x.id) },
      }),
    ) as {
      kitStore?: { designForId?: { clusterableGroups?: string[][] } | null };
    };
    const g = d.kitStore?.designForId?.clusterableGroups;
    if (!Array.isArray(g)) throw new Error("clusterableGroups");
    return { readDesignClusterableGroupsCommand: { groups: g.map((row) => row.map((id) => idDto(id))) } };
  }
  if ("readDesignQualitySumCommand" in cmd && cmd.readDesignQualitySumCommand) {
    const qid = cmd.readDesignQualitySumCommand.qualityId.id;
    const q = `query($id: String!, $q: String!) { kitStore { designForId(id: $id) { qualitySum(qualityId: $q) } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId, q: qid } })) as {
      kitStore?: { designForId?: { qualitySum?: number } | null };
    };
    const s = d.kitStore?.designForId?.qualitySum;
    if (typeof s !== "number") throw new Error("qualitySum");
    return { readDesignQualitySumCommand: { sum: s } };
  }
  if ("readDesignReplaceableCatalogCommand" in cmd && cmd.readDesignReplaceableCatalogCommand) {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { replaceableCatalog(selection: $sel) { typeIds designIds } } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, {
        query: q,
        variables: { id: designId, sel: cmd.readDesignReplaceableCatalogCommand.selection.map((x) => x.id) },
      }),
    ) as { kitStore?: { designForId?: { replaceableCatalog?: { typeIds: string[]; designIds: string[] } } | null } };
    const rc = d.kitStore?.designForId?.replaceableCatalog;
    if (rc == null) throw new Error("replaceableCatalog");
    return {
      readDesignReplaceableCatalogCommand: {
        types: rc.typeIds.map((t) => idDto(t)),
        designs: rc.designIds.map((x) => idDto(x)),
      },
    };
  }
  if ("readDesignIncludedDesignsCommand" in cmd && cmd.readDesignIncludedDesignsCommand === null) {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { includedDesigns } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId } })) as {
      kitStore?: { designForId?: { includedDesigns?: unknown } | null };
    };
    return { readDesignIncludedDesignsCommand: { designs: d.kitStore?.designForId?.includedDesigns as any } };
  }
  if ("readDesignIncludedDesignIdsCommand" in cmd && cmd.readDesignIncludedDesignIdsCommand === null) {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { includedDesignIds } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId } })) as {
      kitStore?: { designForId?: { includedDesignIds?: string[] } | null };
    };
    const ids = d.kitStore?.designForId?.includedDesignIds;
    if (!Array.isArray(ids)) throw new Error("includedDesignIds");
    return { readDesignIncludedDesignIdsCommand: { designIds: ids.map((x) => idDto(x)) } };
  }
  if ("readDesignPiecesFullCommand" in cmd && cmd.readDesignPiecesFullCommand === null) {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { piecesFullJson } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId } })) as {
      kitStore?: { designForId?: { piecesFullJson?: unknown } | null } | null;
    };
    return {
      readDesignPiecesFullCommand: { pieces: kitGraphqlJsonToReadonlyArray(d.kitStore?.designForId?.piecesFullJson) },
    };
  }
  if ("readDesignConnectionsFullCommand" in cmd && cmd.readDesignConnectionsFullCommand === null) {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { connectionsFullJson } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId } })) as {
      kitStore?: { designForId?: { connectionsFullJson?: unknown } | null } | null;
    };
    return {
      readDesignConnectionsFullCommand: {
        connections: kitGraphqlJsonToReadonlyArray(d.kitStore?.designForId?.connectionsFullJson),
      },
    };
  }
  if ("readDesignPieceCommands" in cmd && cmd.readDesignPieceCommands) {
    const { id, commands } = cmd.readDesignPieceCommands;
    const results: ReadPieceCommandOutput[] = [];
    for (const pc of commands) {
      results.push(await kitGraphqlMapPieceRead(handle, designId, id.id, pc));
    }
    return { readDesignPieceCommands: { results } };
  }
  throw new Error(`[DEBUG] kitGraphqlMapDesignRead: ${Object.keys(cmd).join(",")}`);
}

async function kitGraphqlMapPieceRead(
  handle: KitGraphqlHandle,
  designId: string,
  pieceId: string,
  cmd: ReadPieceCommand,
): Promise<ReadPieceCommandOutput> {
  if ("readPieceFlatPlaneCommand" in cmd && cmd.readPieceFlatPlaneCommand === null) {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatPlane } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { d: designId, p: pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { flatPlane?: unknown } | null } | null };
    };
    return { readPieceFlatPlaneCommand: { flatPlane: d.kitStore?.designForId?.pieceForId?.flatPlane as any } };
  }
  if ("readPieceFlatCenterCommand" in cmd && cmd.readPieceFlatCenterCommand === null) {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatCenter } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { d: designId, p: pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { flatCenter?: unknown } | null } | null };
    };
    return { readPieceFlatCenterCommand: { flatCenter: d.kitStore?.designForId?.pieceForId?.flatCenter as any } };
  }
  if ("readPieceParentConnectionFullCommand" in cmd && cmd.readPieceParentConnectionFullCommand === null) {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { parentConnectionFull } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { d: designId, p: pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { parentConnectionFull?: unknown } | null } | null };
    };
    return {
      readPieceParentConnectionFullCommand: {
        connection: d.kitStore?.designForId?.pieceForId?.parentConnectionFull as any,
      },
    };
  }
  throw new Error(`[DEBUG] kitGraphqlMapPieceRead: ${Object.keys(cmd).join(",")}`);
}

async function kitGraphqlMapTypeRead(handle: KitGraphqlHandle, typeId: string, cmd: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
  if ("readTypeBestRepresentationCommand" in cmd && cmd.readTypeBestRepresentationCommand) {
    const tags = cmd.readTypeBestRepresentationCommand.tagIds;
    const q = `query($id: String!, $tags: [String!]!) { kitStore { typeForId(id: $id) { bestRepresentation(tagIds: $tags) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: q, variables: { id: typeId, tags: [...tags] } }),
    ) as { kitStore?: { typeForId?: { bestRepresentation?: unknown } | null } };
    return { readTypeBestRepresentationCommand: { representation: d.kitStore?.typeForId?.bestRepresentation as any } };
  }
  throw new Error(`[DEBUG] kitGraphqlMapTypeRead: ${Object.keys(cmd).join(",")}`);
}

export async function kitGraphqlExecuteRead(handle: KitGraphqlHandle, batch: ReadCommandBatch): Promise<ReadCommandBatchResult> {
  const out: ReadKitCommandOutput[] = [];
  for (const c of batch) {
    out.push(await kitGraphqlMapReadCommand(handle, c));
  }
  return out;
}

/** Any client exposing `executeRead` (e.g. [`FallbackKitStoreClient`]). */
export type KitExecuteRead = {
  executeRead(commands: ReadCommandBatch): Promise<ReadCommandBatchResult>;
};

function assertSingleReadResult(results: ReadCommandBatchResult): ReadKitCommandOutput {
  if (results.length !== 1) {
    throw new Error(`read batch: expected 1 result, got ${results.length}`);
  }
  return results[0]!;
}

export async function readKit(client: KitExecuteRead, command: ReadKitCommand): Promise<ReadKitCommandOutput> {
  return assertSingleReadResult(await client.executeRead([command]));
}

export async function readKitDesign(
  client: KitExecuteRead,
  designId: string,
  command: ReadDesignCommand,
): Promise<ReadDesignCommandOutput> {
  const out = await readKit(client, {
    readKitDesignCommands: {
      id: idDto(designId),
      commands: [command],
    },
  });
  if (!("readKitDesignCommands" in out) || out.readKitDesignCommands == null) {
    throw new Error("read path: expected readKitDesignCommands");
  }
  return out.readKitDesignCommands.results[0]!;
}

export async function readKitDesignPiece(
  client: KitExecuteRead,
  designId: string,
  pieceId: string,
  command: ReadPieceCommand,
): Promise<ReadPieceCommandOutput> {
  const d0 = await readKitDesign(client, designId, {
    readDesignPieceCommands: {
      id: idDto(pieceId),
      commands: [command],
    },
  });
  if (!("readDesignPieceCommands" in d0) || d0.readDesignPieceCommands == null) {
    throw new Error("read path: expected readDesignPieceCommands");
  }
  return d0.readDesignPieceCommands.results[0]!;
}

export async function readKitType(client: KitExecuteRead, typeId: string, command: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
  const out = await readKit(client, {
    readKitTypeCommands: {
      id: idDto(typeId),
      commands: [command],
    },
  });
  if (!("readKitTypeCommands" in out) || out.readKitTypeCommands == null) {
    throw new Error("read path: expected readKitTypeCommands");
  }
  return out.readKitTypeCommands.results[0]!;
}

/** Piece-scoped live reads via `kitStore.designForId.pieceForId` fields. */
export class LivePieceView {
  constructor(
    private readonly gql: KitGraphqlHandle,
    readonly designId: string,
    readonly pieceId: string,
  ) {}

  async readFlatPlane(): Promise<unknown> {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatPlane } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { d: this.designId, p: this.pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { flatPlane?: unknown } | null } | null };
    };
    return d.kitStore?.designForId?.pieceForId?.flatPlane;
  }

  async readFlatCenter(): Promise<unknown> {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatCenter } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { d: this.designId, p: this.pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { flatCenter?: unknown } | null } | null };
    };
    return d.kitStore?.designForId?.pieceForId?.flatCenter;
  }

  async readParentConnectionFull(): Promise<unknown | null | undefined> {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { parentConnectionFull } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { d: this.designId, p: this.pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { parentConnectionFull?: unknown } | null } | null };
    };
    return d.kitStore?.designForId?.pieceForId?.parentConnectionFull;
  }
}

/** Design-scoped live reads via `kitStore.designForId` fields. */
export class LiveDesignView {
  constructor(
    private readonly gql: KitGraphqlHandle,
    readonly designId: string,
  ) {}

  async readClusterableGroups(selection: ReadonlyArray<string>): Promise<ReadonlyArray<ReadonlyArray<IdDto>>> {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { clusterableGroups(selection: $sel) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId, sel: [...selection] } }),
    ) as { kitStore?: { designForId?: { clusterableGroups?: string[][] } | null } };
    const g = d.kitStore?.designForId?.clusterableGroups;
    if (!Array.isArray(g)) throw new Error("clusterableGroups");
    return g.map((row) => row.map((id) => idDto(id)));
  }

  async readIncludedDesigns(): Promise<ReadonlyArray<unknown>> {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { includedDesigns } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId } })) as {
      kitStore?: { designForId?: { includedDesigns?: unknown } | null };
    };
    const v = d.kitStore?.designForId?.includedDesigns;
    return Array.isArray(v) ? v : [];
  }

  async readQualitySum(qualityId: string): Promise<number> {
    const q = `query($id: String!, $q: String!) { kitStore { designForId(id: $id) { qualitySum(qualityId: $q) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId, q: qualityId } }),
    ) as { kitStore?: { designForId?: { qualitySum?: number } | null } };
    const s = d.kitStore?.designForId?.qualitySum;
    if (typeof s !== "number") throw new Error("qualitySum");
    return s;
  }

  async readReplaceableCatalog(selection: ReadonlyArray<string>): Promise<{ types: string[]; designs: string[] }> {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { replaceableCatalog(selection: $sel) { typeIds designIds } } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId, sel: [...selection] } }),
    ) as { kitStore?: { designForId?: { replaceableCatalog?: { typeIds: string[]; designIds: string[] } } | null } };
    const rc = d.kitStore?.designForId?.replaceableCatalog;
    if (rc == null) throw new Error("replaceableCatalog");
    return { types: rc.typeIds, designs: rc.designIds };
  }

  async readIncludedDesignIds(): Promise<string[]> {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { includedDesignIds } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId } })) as {
      kitStore?: { designForId?: { includedDesignIds?: string[] } | null };
    };
    const ids = d.kitStore?.designForId?.includedDesignIds;
    if (!Array.isArray(ids)) throw new Error("includedDesignIds");
    return ids;
  }
}

/** Type-scoped live reads via `kitStore.typeForId` fields. */
export class LiveTypeView {
  constructor(
    private readonly gql: KitGraphqlHandle,
    readonly typeId: string,
  ) {}

  async readBestRepresentation(tagIds: ReadonlyArray<string>): Promise<unknown | null | undefined> {
    const q = `query($id: String!, $tags: [String!]!) { kitStore { typeForId(id: $id) { bestRepresentation(tagIds: $tags) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.typeId, tags: [...tagIds] } }),
    ) as { kitStore?: { typeForId?: { bestRepresentation?: unknown } | null } };
    return d.kitStore?.typeForId?.bestRepresentation;
  }
}

/**
 * Root of live read facades: pass {@link KitStoreClient.kitGraphql} (WASM GraphQL `execute` stream).
 */
export class LiveKitRoot {
  constructor(readonly gql: KitGraphqlHandle) {}

  piece(designId: string, pieceId: string): LivePieceView {
    return new LivePieceView(this.gql, designId, pieceId);
  }

  design(designId: string): LiveDesignView {
    return new LiveDesignView(this.gql, designId);
  }

  type(typeId: string): LiveTypeView {
    return new LiveTypeView(this.gql, typeId);
  }

  async readTypeIds(): Promise<readonly string[]> {
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { typeIds } }` })) as { kitStore?: { typeIds?: string[] } };
    const typeIds = d.kitStore?.typeIds;
    if (!Array.isArray(typeIds)) throw new Error("typeIds");
    return typeIds;
  }

  async readTypesMetadata(): Promise<ReadonlyArray<unknown>> {
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { typesMetadata } }` })) as {
      kitStore?: { typesMetadata?: unknown };
    };
    return kitGraphqlCatalogJsonArray(d.kitStore?.typesMetadata);
  }

  async readDesignIds(): Promise<readonly string[]> {
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { designIds } }` })) as { kitStore?: { designIds?: string[] } };
    const designIds = d.kitStore?.designIds;
    if (!Array.isArray(designIds)) throw new Error("designIds");
    return designIds;
  }

  async readDesignsMetadata(): Promise<ReadonlyArray<unknown>> {
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { designsMetadata } }` })) as {
      kitStore?: { designsMetadata?: unknown };
    };
    return kitGraphqlCatalogJsonArray(d.kitStore?.designsMetadata);
  }

  async readColoredConnectors(): Promise<ReadonlyArray<unknown>> {
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { coloredConnectors } }` })) as {
      kitStore?: { coloredConnectors?: unknown };
    };
    return kitGraphqlCatalogJsonArray(d.kitStore?.coloredConnectors);
  }
}

//#endregion 🔖KitGraphLive

//#region 🧵KitWorker
// Web Worker API: loads the semio WASM module (host-configured), hosts [`KitStoreHandle`], exposes RPC via Comlink.

let kitWorkerHandle: any = null;
const kitEventListeners = new Map<number, (ev: unknown) => void>();
let nextKitEventListenerId = 0;
let kitEventGqlStarted = false;

function kitWorkerGqlHandle(): KitGraphqlHandle {
  if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
  return {
    execute: (requestJson: string, onMessage: (line: string) => void) => kitWorkerHandle.execute(requestJson, onMessage),
  };
}

function kitWorkerAsExecuteRead(): KitExecuteRead {
  return { executeRead: (batch) => kitGraphqlExecuteRead(kitWorkerGqlHandle(), batch) };
}

async function importWasmModule(specifier: string) {
  if (specifier === "@semio/rs-wasm") {
    return import("@semio/rs-wasm");
  }
  return import(/* @vite-ignore */ specifier);
}

function settle(p: Promise<any>): Promise<any> {
  return p.catch((e: any) => ({ ok: false, error: { kind: "Internal", message: String(e) } }));
}

export const kitWorkerApi = {
  async init(wasmSpecifier: string, dto: unknown) {
    const mod = await importWasmModule(wasmSpecifier);
    if (typeof mod.default === "function") {
      await mod.default();
    }
    if (typeof mod.boot === "function") {
      mod.boot();
    }
    const { KitStoreHandle } = mod;
    kitWorkerHandle = KitStoreHandle.create(dto as any);
  },
  snapshot() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.snapshot();
  },
  setField(kind: string, id: string, field: string, value: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const cmds = kitWorkerHandle.changeKitCommandsForFieldPatch(kind, id, field, value);
          await kitWorkerHandle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const cmds = kitWorkerHandle.changeKitCommandsForAddChild(parentKind, parentId, childKind, dto);
          await kitWorkerHandle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const cmds = kitWorkerHandle.changeKitCommandsForRemoveChild(parentKind, parentId, childKind, childId);
          await kitWorkerHandle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
  applyDesignDiff(designId: string, diff: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.applyDesignDiff(designId, diff)));
  },
  applyKitDiff(diff: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.applyKitDiff(diff)));
  },
  clusterPieces(designId: string, pieceIds: string[], clusterName: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.clusterPieces(designId, pieceIds, clusterName)));
  },
  dragPieces(designId: string, pieceIds: string[], du: number, dv: number) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.dragPieces(designId, pieceIds, du, dv)));
  },
  movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.movePieces(designId, pieceIds, gap, shift, rise)));
  },
  fixPieces(designId: string, pieceIds: string[]) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.fixPieces(designId, pieceIds)));
  },
  flattenDesign(designId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.flattenDesign(designId)));
  },
  expandDesign(parentDesignId: string, nestedDesignId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.expandDesign(parentDesignId, nestedDesignId)));
  },
  deleteConnection(designId: string, connectionId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.deleteConnection(designId, connectionId)));
  },
  changePieceType(designId: string, pieceId: string, newTypeId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.changePieceType(designId, pieceId, newTypeId)));
  },
  pasteDesignSelection(designId: string, selection: unknown, plane: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.pasteDesignSelection(designId, selection, plane)));
  },
  createHangingPieces(designId: string, typeIds: string[], plane: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.createHangingPieces(designId, typeIds, plane)));
  },
  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort)));
  },
  createFixedPiece(designId: string, typeId: string, plane: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.createFixedPiece(designId, typeId, plane)));
  },
  getPiecesMetadata(designId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlKitDesignPiecesMetadata(kitWorkerGqlHandle(), designId));
  },
  getPieces(designId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        const out = await readKitDesign(kitWorkerAsExecuteRead(), designId, { readDesignPiecesFullCommand: null });
        if (!("readDesignPiecesFullCommand" in out) || out.readDesignPiecesFullCommand == null) {
          throw new Error("readDesignPiecesFullCommand: missing output");
        }
        return out.readDesignPiecesFullCommand.pieces;
      })(),
    );
  },
  getConnections(designId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        const out = await readKitDesign(kitWorkerAsExecuteRead(), designId, { readDesignConnectionsFullCommand: null });
        if (!("readDesignConnectionsFullCommand" in out) || out.readDesignConnectionsFullCommand == null) {
          throw new Error("readDesignConnectionsFullCommand: missing output");
        }
        return out.readDesignConnectionsFullCommand.connections;
      })(),
    );
  },
  getDesigns() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        const out = await readKit(kitWorkerAsExecuteRead(), { readKitDesignsShallowCommand: null });
        if (!("readKitDesignsShallowCommand" in out) || out.readKitDesignsShallowCommand == null) {
          throw new Error("readKitDesignsShallowCommand: missing output");
        }
        return out.readKitDesignsShallowCommand.designs;
      })(),
    );
  },
  getTypes() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        const out = await readKit(kitWorkerAsExecuteRead(), { readKitTypesShallowCommand: null });
        if (!("readKitTypesShallowCommand" in out) || out.readKitTypesShallowCommand == null) {
          throw new Error("readKitTypesShallowCommand: missing output");
        }
        return out.readKitTypesShallowCommand.types;
      })(),
    );
  },
  getAuthors() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        const out = await readKit(kitWorkerAsExecuteRead(), { readKitAuthorsShallowCommand: null });
        if (!("readKitAuthorsShallowCommand" in out) || out.readKitAuthorsShallowCommand == null) {
          throw new Error("readKitAuthorsShallowCommand: missing output");
        }
        return out.readKitAuthorsShallowCommand.authors;
      })(),
    );
  },
  getKitMetadata() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        const out = await readKit(kitWorkerAsExecuteRead(), { readKitMetadataCommand: null });
        if (!("readKitMetadataCommand" in out) || out.readKitMetadataCommand == null) {
          throw new Error("readKitMetadataCommand: missing output");
        }
        return out.readKitMetadataCommand.metadata;
      })(),
    );
  },
  graphqlExecute(requestJson: string, onMessage: (line: string) => void) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.execute(requestJson, onMessage);
  },
  undo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.undo()));
  },
  redo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(kitWorkerHandle.redo()));
  },
  canUndo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.canUndo();
  },
  canRedo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.canRedo();
  },
  subscribe(cb: (ev: unknown) => void) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    const proxy = Comlink.proxy(cb);
    const id = nextKitEventListenerId++;
    const forward = (payload: unknown) => {
      try {
        proxy(payload);
      } catch {
        /* ignore */
      }
    };
    kitEventListeners.set(id, forward);
    if (!kitEventGqlStarted) {
      kitEventGqlStarted = true;
      kitGraphqlSubscribeLoop(kitWorkerGqlHandle(), (payload) => {
        for (const fn of kitEventListeners.values()) fn(payload);
      });
    }
    return () => {
      kitEventListeners.delete(id);
      if (kitEventListeners.size === 0) kitEventGqlStarted = false;
    };
  },

  async execute(cmd: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    try {
      const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), cmd);
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  },

  async executeRead(cmds: ReadCommandBatch) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return await kitGraphqlExecuteRead(kitWorkerGqlHandle(), cmds);
  },

  vcsState() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.vcsState();
  },

  theKitDto() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.theKitDto();
  },

  materializeAt(at: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return kitWorkerHandle.materializeAt(at);
  },

  attachBackbone(config: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), { attachBackbone: { config } });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },

  detachBackbone() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), { detachBackbone: null });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },

  async backboneStatus() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    try {
      const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), { backboneStatus: null });
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  },

  async listConflicts() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    try {
      const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), { listConflicts: null });
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  },

  resolveConflict(conflictId: string, resolution: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), {
            resolveConflict: { id: conflictId, strategy: resolution },
          });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },

  syncNow() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(kitWorkerGqlHandle(), { syncNow: null });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
};

/** Call in a Web Worker context to expose `kitWorkerApi` via Comlink. */
export function bootKitWorker() {
  Comlink.expose(kitWorkerApi);
}

//#endregion 🧵KitWorker

