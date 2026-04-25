// @ts-nocheck
// #region ­ƒº▓Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core domain representation types, schemas and utilities for the semio platform.

// #endregion ­ƒº▓Header

// #region Ôø®´©ÅImports
// External dependency imports MUST be declared here.
import { default as adjectives } from "@semio/assets/lists/adjectives.json" with { type: "json" };
import { default as animals } from "@semio/assets/lists/animals.json" with { type: "json" };
import * as Comlink from "comlink";
import { describe, expect, it } from "vitest";
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

/** Coalesce optional arrays for iteration (replaces removed `toArray` helper). */
export const toArray = <T>(xs: readonly T[] | T[] | null | undefined): T[] => (xs == null ? [] : [...xs]);

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
 * Type alias for Id.
 **/
export type Id = string;

// #region 🔧Semio
// Semio utility class delegates to WASM for domain operations.

/**
 * 🔧Semio utility class with static methods delegating to WASM.
 */
export class Semio {
  private static _wasm: any;
  private static async wasm() {
    if (!Semio._wasm) Semio._wasm = await import("@semio/rs-wasm");
    return Semio._wasm;
  }
  static async normalizeName(s: string): Promise<string> { return (await Semio.wasm()).semioNormalizeName(s); }
  static async round(value: number, decimals: number): Promise<number> { return (await Semio.wasm()).semioRound(value, decimals); }
  static async generateId(): Promise<string> { return (await Semio.wasm()).generateId(); }
  static async kitFromJson(s: string): Promise<any> { return (await Semio.wasm()).kitFromJson(s); }
  static async kitToJson(value: any): Promise<any> { return (await Semio.wasm()).kitToJson(value); }
  static async kitValidate(value: any): Promise<any> { return (await Semio.wasm()).kitValidate(value); }
  static async kitsAreEqual(a: any, b: any): Promise<any> { return (await Semio.wasm()).kitsAreEqual(a, b); }
  static async flattenDesign(kit: any, designId: string): Promise<any> { return (await Semio.wasm()).flattenDesign(kit, designId); }
}

// #endregion 🔧Semio

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
 * Identifier type for Kit entities.
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
 * Zod schema for validating Kit identifiers.
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
 * Factory for creating Kit identifiers.
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
 * Equality check for Kit identifiers.
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
 * Extracts the ID from a Kit identifier.
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
  static fromPlain(plain: VecPlain): Vec {
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
  static fromPlain(plain: PointPlain): Point {
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
  static fromPlain(plain: VectorPlain): Vector {
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
  static fromPlain(plain: PlanePlain): Plane {
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
  static fromPlain(plain: CameraPlain): Camera {
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
  static fromPlain(plain: AttributePlain): Attribute {
    return new Attribute(plain);
  }
  static createId(id: string): AttributeId {
    return { id };
  }
  static areSameId(a: AttributeId, b: AttributeId): boolean {
    return a.id === b.id;
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
 * Definition of AttributeMetadataDtoSchema.
 **/
export const AttributeMetadataDtoSchema = AttributeSchema;
/**
 * Type alias for AttributeMetadataDto.
 **/
export type AttributeMetadataDto = z.infer<typeof AttributeMetadataDtoSchema>;
/**
 * Serializes AttributeMetadataDto for transport.
 **/
export const serializeAttributeMetadataDto = (attribute: AttributeMetadataDto): string => JSON.stringify(AttributeMetadataDtoSchema.parse(attribute));
/**
 **/
export const deserializeAttributeMetadataDto = (json: string): AttributeMetadataDto => AttributeMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: LocationPlain): Location {
    return new Location(plain);
  }
  static createId(id: string): LocationId {
    return { id };
  }
  static areSameId(a: LocationId, b: LocationId): boolean {
    return a.id === b.id;
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
  static fromPlain(plain: AuthorPlain): Author {
    return new Author(plain);
  }
  static createId(id: string): AuthorId {
    return { id };
  }
  static areSameId(a: AuthorId, b: AuthorId): boolean {
    return a.id === b.id;
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
 * Definition of AuthorMetadataDtoSchema.
 **/
export const AuthorMetadataDtoSchema = AuthorSchema.omit({ attributes: true });
/**
 * Type alias for AuthorMetadataDto.
 **/
export type AuthorMetadataDto = z.infer<typeof AuthorMetadataDtoSchema>;
/**
 * Serializes AuthorMetadataDto for transport.
 **/
export const serializeAuthorMetadataDto = (author: AuthorMetadataDto): string => JSON.stringify(AuthorMetadataDtoSchema.parse(author));
/**
 **/
export const deserializeAuthorMetadataDto = (json: string): AuthorMetadataDto => AuthorMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: FilePlain): File {
    return new File(plain);
  }
  static createId(id: string): FileId {
    return { id };
  }
  static areSameId(a: FileId, b: FileId): boolean {
    return a.id === b.id;
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
 * Definition of FileMetadataDtoSchema.
 **/
export const FileMetadataDtoSchema = FileSchema.omit({ blob: true });
/**
 * Type alias for FileMetadataDto.
 **/
export type FileMetadataDto = z.infer<typeof FileMetadataDtoSchema>;
/**
 * Serializes FileMetadataDto for transport.
 **/
export const serializeFileMetadataDto = (file: FileMetadataDto): string => JSON.stringify(FileMetadataDtoSchema.parse(file));
/**
 **/
export const deserializeFileMetadataDto = (json: string): FileMetadataDto => FileMetadataDtoSchema.parse(JSON.parse(json));
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
 * Definition of FolderMetadataDtoSchema.
 **/
export const FolderMetadataDtoSchema = FolderSchema.omit({ attributes: true });
/**
 * Type alias for FolderMetadataDto.
 **/
export type FolderMetadataDto = z.infer<typeof FolderMetadataDtoSchema>;
/**
 * Serializes FolderMetadataDto for transport.
 **/
export const serializeFolderMetadataDto = (folder: FolderMetadataDto): string => JSON.stringify(FolderMetadataDtoSchema.parse(folder));
/**
 **/
export const deserializeFolderMetadataDto = (json: string): FolderMetadataDto => FolderMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: QualityPlain): Quality {
    return new Quality(plain);
  }
  static createId(id: string): QualityId {
    return { id };
  }
  static areSameId(a: QualityId, b: QualityId): boolean {
    return a.id === b.id;
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
 * Definition of QualityMetadataDtoSchema.
 **/
export const QualityMetadataDtoSchema = QualitySchema.omit({ benchmarks: true, attributes: true });
/**
 * Type alias for QualityMetadataDto.
 **/
export type QualityMetadataDto = z.infer<typeof QualityMetadataDtoSchema>;
/**
 * Serializes QualityMetadataDto for transport.
 **/
export const serializeQualityMetadataDto = (quality: QualityMetadataDto): string => JSON.stringify(QualityMetadataDtoSchema.parse(quality));
/**
 **/
export const deserializeQualityMetadataDto = (json: string): QualityMetadataDto => QualityMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: PortPlain): Port {
    return new Port(plain);
  }
  static createId(id: string): PortId {
    return { id };
  }
  static areSameId(a: PortId, b: PortId): boolean {
    return a.id === b.id;
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
 * Definition of PortMetadataDtoSchema.
 **/
export const PortMetadataDtoSchema = PortSchema.omit({ compatiblePorts: true, attributes: true });
/**
 * Type alias for PortMetadataDto.
 **/
export type PortMetadataDto = z.infer<typeof PortMetadataDtoSchema>;
/**
 * Serializes PortMetadataDto for transport.
 **/
export const serializePortMetadataDto = (port: PortMetadataDto): string => JSON.stringify(PortMetadataDtoSchema.parse(port));
/**
 **/
export const deserializePortMetadataDto = (json: string): PortMetadataDto => PortMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: FamilyPlain): Family {
    return new Family(plain);
  }
  static createId(id: string): FamilyId {
    return { id };
  }
  static areSameId(a: FamilyId, b: FamilyId): boolean {
    return a.id === b.id;
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
 * Definition of FamilyMetadataDtoSchema.
 **/
export const FamilyMetadataDtoSchema = FamilySchema.omit({ ports: true, attributes: true });
/**
 * Type alias for FamilyMetadataDto.
 **/
export type FamilyMetadataDto = z.infer<typeof FamilyMetadataDtoSchema>;
/**
 * Serializes FamilyMetadataDto for transport.
 **/
export const serializeFamilyMetadataDto = (family: FamilyMetadataDto): string => JSON.stringify(FamilyMetadataDtoSchema.parse(family));
/**
 **/
export const deserializeFamilyMetadataDto = (json: string): FamilyMetadataDto => FamilyMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: PropPlain): Prop {
    return new Prop(plain);
  }
  static createId(id: string): PropId {
    return { id };
  }
  static areSameId(a: PropId, b: PropId): boolean {
    return a.id === b.id;
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
 * Definition of PropMetadataDtoSchema.
 **/
export const PropMetadataDtoSchema = PropSchema.omit({ attributes: true });
/**
 * Type alias for PropMetadataDto.
 **/
export type PropMetadataDto = z.infer<typeof PropMetadataDtoSchema>;
/**
 * Serializes PropMetadataDto for transport.
 **/
export const serializePropMetadataDto = (prop: PropMetadataDto): string => JSON.stringify(PropMetadataDtoSchema.parse(prop));
/**
 **/
export const deserializePropMetadataDto = (json: string): PropMetadataDto => PropMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: TagPlain): Tag {
    return new Tag(plain);
  }
  static createId(id: string): TagId {
    return { id };
  }
  static areSameId(a: TagId, b: TagId): boolean {
    return a.id === b.id;
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
 * Definition of TagMetadataDtoSchema.
 **/
export const TagMetadataDtoSchema = TagSchema.omit({ attributes: true });
/**
 * Type alias for TagMetadataDto.
 **/
export type TagMetadataDto = z.infer<typeof TagMetadataDtoSchema>;
/**
 * Serializes TagMetadataDto for transport.
 **/
export const serializeTagMetadataDto = (tag: TagMetadataDto): string => JSON.stringify(TagMetadataDtoSchema.parse(tag));
/**
 **/
export const deserializeTagMetadataDto = (json: string): TagMetadataDto => TagMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: ConceptPlain): Concept {
    return new Concept(plain);
  }
  static createId(id: string): ConceptId {
    return { id };
  }
  static areSameId(a: ConceptId, b: ConceptId): boolean {
    return a.id === b.id;
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
 * Definition of ConceptMetadataDtoSchema.
 **/
export const ConceptMetadataDtoSchema = ConceptSchema.omit({ attributes: true });
/**
 * Type alias for ConceptMetadataDto.
 **/
export type ConceptMetadataDto = z.infer<typeof ConceptMetadataDtoSchema>;
/**
 * Serializes ConceptMetadataDto for transport.
 **/
export const serializeConceptMetadataDto = (concept: ConceptMetadataDto): string => JSON.stringify(ConceptMetadataDtoSchema.parse(concept));
/**
 **/
export const deserializeConceptMetadataDto = (json: string): ConceptMetadataDto => ConceptMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: RepresentationPlain): Representation {
    return new Representation(plain);
  }
  static createId(id: string): RepresentationId {
    return { id };
  }
  static areSameId(a: RepresentationId, b: RepresentationId): boolean {
    return a.id === b.id;
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
 * Definition of RepresentationMetadataDtoSchema.
 **/
export const RepresentationMetadataDtoSchema = RepresentationSchema.omit({ tags: true, attributes: true });
/**
 * Type alias for RepresentationMetadataDto.
 **/
export type RepresentationMetadataDto = z.infer<typeof RepresentationMetadataDtoSchema>;
/**
 * Serializes RepresentationMetadataDto for transport.
 **/
export const serializeRepresentationMetadataDto = (representation: RepresentationMetadataDto): string => JSON.stringify(RepresentationMetadataDtoSchema.parse(representation));
/**
 **/
export const deserializeRepresentationMetadataDto = (json: string): RepresentationMetadataDto => RepresentationMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: ConnectorPlain): Connector {
    return new Connector(plain);
  }
  static createId(id: string): ConnectorId {
    return { id };
  }
  static areSameId(a: ConnectorId, b: ConnectorId): boolean {
    return a.id === b.id;
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
 * Definition of ConnectorMetadataDtoSchema.
 **/
export const ConnectorMetadataDtoSchema = ConnectorSchema.omit({ props: true, attributes: true });
/**
 * Type alias for ConnectorMetadataDto.
 **/
export type ConnectorMetadataDto = z.infer<typeof ConnectorMetadataDtoSchema>;
/**
 * Serializes ConnectorMetadataDto for transport.
 **/
export const serializeConnectorMetadataDto = (connector: ConnectorMetadataDto): string => JSON.stringify(ConnectorMetadataDtoSchema.parse(connector));
/**
 **/
export const deserializeConnectorMetadataDto = (json: string): ConnectorMetadataDto => ConnectorMetadataDtoSchema.parse(JSON.parse(json));
/**
 * Definition of ConnectorShallowSchema.
 **/
export const ConnectorShallowSchema = ConnectorSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
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

  constructor(plain: TypePlain) {
    const p = TypeSchema.parse(plain);
    Object.assign(this, p);
    this.representations = p.representations?.map((m) => new Representation(m));
    this.connectors = p.connectors?.map((c) => new Connector(c));
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(plain: TypePlain): Type {
    return new Type(plain);
  }

  findConnector(connectorId: string): Connector | undefined {
    return this.connectors?.find((c) => c.id === connectorId);
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Type {
    return Type.fromPlain(TypeSchema.parse(JSON.parse(json)));
  }

  toPlain(): TypePlain {
    return TypeSchema.parse({ ...(this as unknown as TypePlain) });
  }

  toMeta(): TypeMetadataDto {
    return TypeMetadataDtoSchema.parse(this.toPlain());
  }

  toShallow(): TypeShallow {
    const plain = this.toPlain();
    return TypeShallowSchema.parse({
      ...plain,
      representations: this.representations?.map((m) => RepresentationMetadataDtoSchema.parse(m.toPlain())),
      connectors: this.connectors?.map((c) => ConnectorMetadataDtoSchema.parse(c.toPlain())),
      props: this.props?.map((p) => PropMetadataDtoSchema.parse(p.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetadataDtoSchema.parse(a.toPlain())),
    });
  }

  static createId(id: string): TypeId {
    return { id };
  }

  static areSameId(a: TypeId, b: TypeId): boolean {
    return a.id === b.id;
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
 * Definition of TypeMetadataDtoSchema.
 **/
export const TypeMetadataDtoSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
/**
 * Type alias for TypeMetadataDto.
 **/
export type TypeMetadataDto = z.infer<typeof TypeMetadataDtoSchema>;
/**
 * Serializes TypeMetadataDto for transport.
 **/
export const serializeTypeMetadataDto = (type: TypeMetadataDto): string => JSON.stringify(TypeMetadataDtoSchema.parse(type));
/**
 **/
export const deserializeTypeMetadataDto = (json: string): TypeMetadataDto => TypeMetadataDtoSchema.parse(JSON.parse(json));
/**
 * Definition of TypeShallowSchema.
 **/
export const TypeShallowSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true }).extend({
  representations: z.array(RepresentationMetadataDtoSchema).optional(),
  connectors: z.array(ConnectorMetadataDtoSchema).optional(),
  props: z.array(PropMetadataDtoSchema).optional(),
  attributes: z.array(AttributeMetadataDtoSchema).optional(),
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
  static fromPlain(plain: LayerPlain): Layer {
    return new Layer(plain);
  }
  toPlain(): LayerPlain {
    return LayerSchema.parse(this as unknown as LayerPlain);
  }
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  static deserialize(json: string): Layer {
    return new Layer(LayerSchema.parse(JSON.parse(json)));
  }
  static createId(id: string): LayerId {
    return { id };
  }
  static areSameId(a: LayerId, b: LayerId): boolean {
    return a.id === b.id;
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
 * Definition of LayerMetadataDtoSchema.
 **/
export const LayerMetadataDtoSchema = LayerSchema.omit({ attributes: true });
/**
 * Type alias for LayerMetadataDto.
 **/
export type LayerMetadataDto = z.infer<typeof LayerMetadataDtoSchema>;
/**
 * Serializes LayerMetadataDto for transport.
 **/
export const serializeLayerMetadataDto = (layer: LayerMetadataDto): string => JSON.stringify(LayerMetadataDtoSchema.parse(layer));
/**
 **/
export const deserializeLayerMetadataDto = (json: string): LayerMetadataDto => LayerMetadataDtoSchema.parse(JSON.parse(json));
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
  type?: TypeId;
  design?: DesignId;
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

  constructor(plain: PiecePlain) {
    const p = PieceSchema.parse(plain);
    Object.assign(this, p);
    this.plane = p.plane ? new Plane(p.plane) : undefined;
    this.center = p.center ? new Coordinate(p.center) : undefined;
    this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined;
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(plain: PiecePlain): Piece {
    return new Piece(plain);
  }

  wireTypeId(): TypeId | undefined {
    return this.type;
  }

  wireDesignAsPieceId(): DesignId | undefined {
    return this.design;
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Piece {
    return new Piece(PieceSchema.parse(JSON.parse(json)));
  }

  toPlain(): PiecePlain {
    return PieceSchema.parse({
      ...(this as unknown as PiecePlain),
      type: this.wireTypeId(),
      design: this.wireDesignAsPieceId(),
    });
  }

  toMeta(): PieceMetadataDto {
    return PieceMetadataDtoSchema.parse(this.toPlain());
  }

  toShallow(): PieceShallow {
    const plain = this.toPlain();
    return PieceShallowSchema.parse(plain);
  }

  static createId(id: string): PieceId {
    return { id };
  }

  static areSameId(a: PieceId, b: PieceId): boolean {
    return a.id === b.id;
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
  const g = piece.wireTypeId()?.id;
  return g ? getType(g) : undefined;
};

/**
 * Definition of PieceMetadataDtoSchema.
 **/
export const PieceMetadataDtoSchema = PieceSchema.omit({ props: true, attributes: true });
/**
 * Type alias for PieceMetadataDto.
 **/
export type PieceMetadataDto = z.infer<typeof PieceMetadataDtoSchema>;
/**
 * Serializes PieceMetadataDto for transport.
 **/
export const serializePieceMetadataDto = (piece: PieceMetadataDto): string => JSON.stringify(PieceMetadataDtoSchema.parse(piece));
/**
 **/
export const deserializePieceMetadataDto = (json: string): PieceMetadataDto => PieceMetadataDtoSchema.parse(JSON.parse(json));
/**
 * Definition of PieceShallowSchema.
 **/
export const PieceShallowSchema = PieceSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
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
  static fromPlain(plain: GroupPlain): Group {
    return new Group(plain);
  }
  static createId(id: string): GroupId {
    return { id };
  }
  static areSameId(a: GroupId, b: GroupId): boolean {
    return a.id === b.id;
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
 * Definition of GroupMetadataDtoSchema.
 **/
export const GroupMetadataDtoSchema = GroupSchema.omit({ pieces: true, attributes: true });
/**
 * Type alias for GroupMetadataDto.
 **/
export type GroupMetadataDto = z.infer<typeof GroupMetadataDtoSchema>;
/**
 * Serializes GroupMetadataDto for transport.
 **/
export const serializeGroupMetadataDto = (group: GroupMetadataDto): string => JSON.stringify(GroupMetadataDtoSchema.parse(group));
/**
 **/
export const deserializeGroupMetadataDto = (json: string): GroupMetadataDto => GroupMetadataDtoSchema.parse(JSON.parse(json));
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

  constructor(plain: SidePlain) {
    const p = SideSchema.parse(plain);
    this.#pieceId = p.piece.id;
    this.#designPieceId = p.designPiece?.id;
    this.#connectorId = p.connector?.id;
  }

  /** Piece endpoint as an id-bearing object (no live resolution). */
  get piece(): PieceId {
    return { id: this.#pieceId };
  }

  /** Optional nested design-piece reference. */
  get designPiece(): PieceId | undefined {
    if (!this.#designPieceId) return undefined;
    return { id: this.#designPieceId };
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
  static deserialize(json: string): Side {
    return new Side(SideSchema.parse(JSON.parse(json)));
  }

  static from(plain: SidePlain): Side {
    return new Side(plain);
  }

  static fromPlain(plain: SidePlain): Side {
    return new Side(plain);
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

  constructor(plain: ConnectionPlain) {
    const p = ConnectionSchema.parse(plain);
    Object.assign(this, p);
    this.connected = new Side(p.connected);
    this.connecting = new Side(p.connecting);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
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
  static deserialize(json: string): Connection {
    return new Connection(ConnectionSchema.parse(JSON.parse(json)));
  }

  static from(plain: ConnectionPlain): Connection {
    return new Connection(plain);
  }

  static fromPlain(plain: ConnectionPlain): Connection {
    return new Connection(plain);
  }

  static createId(id: string): ConnectionId {
    return { id };
  }

  static areSameId(a: ConnectionId, b: ConnectionId): boolean {
    return a.id === b.id;
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
 * Definition of ConnectionMetadataDtoSchema.
 **/
export const ConnectionMetadataDtoSchema = ConnectionSchema.omit({ attributes: true });
/**
 * Type alias for ConnectionMetadataDto.
 **/
export type ConnectionMetadataDto = z.infer<typeof ConnectionMetadataDtoSchema>;
/**
 * Serializes ConnectionMetadataDto for transport.
 **/
export const serializeConnectionMetadataDto = (connection: ConnectionMetadataDto): string => JSON.stringify(ConnectionMetadataDtoSchema.parse(connection));
/**
 **/
export const deserializeConnectionMetadataDto = (json: string): ConnectionMetadataDto => ConnectionMetadataDtoSchema.parse(JSON.parse(json));
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
  static fromPlain(plain: StatPlain): Stat {
    return new Stat(plain);
  }
  static createId(id: string): StatId {
    return { id };
  }
  static areSameId(a: StatId, b: StatId): boolean {
    return a.id === b.id;
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
 * Definition of StatMetadataDtoSchema.
 **/
export const StatMetadataDtoSchema = StatSchema;
/**
 * Type alias for StatMetadataDto.
 **/
export type StatMetadataDto = z.infer<typeof StatMetadataDtoSchema>;
/**
 * Serializes StatMetadataDto for transport.
 **/
export const serializeStatMetadataDto = (stat: StatMetadataDto): string => JSON.stringify(StatMetadataDtoSchema.parse(stat));
/**
 **/
export const deserializeStatMetadataDto = (json: string): StatMetadataDto => StatMetadataDtoSchema.parse(JSON.parse(json));
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
  createdAt!: string;
  updatedAt!: string;

  constructor(plain: DesignPlain | Design) {
    const wire: DesignPlain = plain instanceof Design ? plain.toPlain() : plain;
    const p = DesignSchema.parse(wire);
    const { connections: _wcon, pieces: _wp, ...rest } = p;
    Object.assign(this, rest);
    this.pieces = p.pieces?.map((x) => new Piece(x));
    this._connections = p.connections?.map((x) => new Connection(x));
    this.stats = p.stats?.map((x) => new Stat(x));
    this.props = p.props?.map((x) => new Prop(x));
    this.layers = p.layers?.map((x) => new Layer(x));
    this.groups = p.groups?.map((x) => new Group(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(plain: DesignPlain): Design {
    return new Design(plain);
  }

  findPiece(lookup: string | { name: string }): Piece | undefined {
    const key = typeof lookup === "string" ? lookup : lookup.name;
    const byId = this.pieces?.find((p) => p.id === key);
    if (byId) return byId;
    return this.pieces?.find((p) => p.name === key);
  }

  requirePiece(lookup: string | { name: string }): Piece {
    const piece = this.findPiece(lookup);
    const label = typeof lookup === "string" ? lookup : lookup.name;
    if (!piece) throw new Error(`Piece ${label} not found in design ${this.name}`);
    return piece;
  }

  findConnection(connectionId: string): Connection | undefined {
    return this._connections?.find((c) => c.id === connectionId);
  }

  requireConnection(connectionId: string): Connection {
    return findConnection(this._connections ?? [], connectionId);
  }

  getPieces(): readonly Piece[] {
    return this.pieces ?? [];
  }

  getConnections(): readonly Connection[] {
    return this._connections ?? [];
  }

  connections(): readonly Connection[] {
    return this.getConnections();
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

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Design {
    return new Design(DesignSchema.parse(JSON.parse(json)));
  }

  toMeta(): DesignMetadataDto {
    return DesignMetadataDtoSchema.parse(this.toPlain());
  }

  toShallow(): DesignShallow {
    const plain = this.toPlain();
    return DesignShallowSchema.parse({
      ...plain,
      pieces: this.pieces?.map((p) => PieceMetadataDtoSchema.parse(p.toPlain())),
      connections: this._connections?.map((c) => ConnectionMetadataDtoSchema.parse(c.toPlain())),
      stats: this.stats?.map((s) => StatMetadataDtoSchema.parse(s.toPlain())),
      props: this.props?.map((p) => PropMetadataDtoSchema.parse(p.toPlain())),
      layers: this.layers?.map((l) => LayerMetadataDtoSchema.parse(l.toPlain())),
      groups: this.groups?.map((g) => GroupMetadataDtoSchema.parse(g.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetadataDtoSchema.parse(a.toPlain())),
    });
  }

  static createId(id: string): DesignId {
    return { id };
  }

  static areSameId(a: DesignId, b: DesignId): boolean {
    return a.id === b.id;
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
 * Definition of DesignMetadataDtoSchema.
 **/
export const DesignMetadataDtoSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true, authors: true, concepts: true });
/**
 * Type alias for DesignMetadataDto.
 **/
export type DesignMetadataDto = z.infer<typeof DesignMetadataDtoSchema>;
/**
 * Serializes DesignMetadataDto for transport.
 **/
export const serializeDesignMetadataDto = (design: DesignMetadataDto): string => JSON.stringify(DesignMetadataDtoSchema.parse(design));
/**
 **/
export const deserializeDesignMetadataDto = (json: string): DesignMetadataDto => DesignMetadataDtoSchema.parse(JSON.parse(json));
/**
 * Definition of DesignShallowSchema.
 **/
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true }).extend({
  pieces: z.array(PieceMetadataDtoSchema).optional(),
  connections: z.array(ConnectionMetadataDtoSchema).optional(),
  stats: z.array(StatMetadataDtoSchema).optional(),
  props: z.array(PropMetadataDtoSchema).optional(),
  layers: z.array(LayerMetadataDtoSchema).optional(),
  groups: z.array(GroupMetadataDtoSchema).optional(),
  attributes: z.array(AttributeMetadataDtoSchema).optional(),
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

export const PASTE_DESIGN_ANCHORING_KINDS = ["original", "middle", "centroid", "bottomLeft", "bottomRight", "topLeft", "topRight"] as const;

export type PasteDesignAnchoringKind = (typeof PASTE_DESIGN_ANCHORING_KINDS)[number];

// #endregion ­ƒôÉDesign

// #region ÔÅ▒´©ÅKit
// Kit entity types, schemas, and helpers MUST be defined here.

// #region ­ƒº¼KitKind
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
 * Zod schema for Kit validation.
 **/
export const KitFullDtoSchema = z.object({
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
export type KitFullDto = z.infer<typeof KitFullDtoSchema>;

// #region KitEntity
/**
 * Thin {@link KitFullDto} view: serialization + plain DTOs only. Domain mutations use {@link KitStoreClient} (WASM).
 */
export class Kit {
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

  constructor(data: KitFullDto) {
    const p = KitFullDtoSchema.parse(data);
    Object.assign(this, p);
    this.types = p.types?.map((t) => new Type(t));
    this.designs = p.designs?.map((d) => new Design(d));
    this.tags = p.tags?.map((t) => new Tag(t));
    this.concepts = p.concepts?.map((c) => new Concept(c));
    this.families = p.families?.map((f) => new Family(f));
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.files = p.files?.map((f) => new File(f));
    this.folders = p.folders?.map((f) => new Folder(f));
    this.authors = p.authors?.map((a) => new Author(a));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(data: KitFullDto): Kit {
    return new Kit(data);
  }

  toPlain(): KitFullDto {
    return KitFullDtoSchema.parse({
      ...(this as unknown as KitFullDto),
      types: this.types?.map((t) => t.toPlain()),
      designs: this.designs?.map((d) => d.toPlain()),
      tags: this.tags?.map((t) => t.toPlain()),
      concepts: this.concepts?.map((c) => c.toPlain()),
      families: this.families?.map((f) => f.toPlain()),
      qualities: this.qualities?.map((q) => q.toPlain()),
      files: this.files?.map((f) => f.toPlain()),
      folders: this.folders?.map((f) => f.toPlain()),
      authors: this.authors?.map((a) => a.toPlain()),
      attributes: this.attributes?.map((a) => a.toPlain()),
    });
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Kit {
    return Kit.fromPlain(KitFullDtoSchema.parse(JSON.parse(json)));
  }

  toJSON(): KitFullDto {
    return this.toPlain();
  }

  static createId(id: string): KitId {
    return { id };
  }

  static areSameId(a: KitId, b: KitId): boolean {
    return a.id === b.id;
  }
}

/**
 * Wire DTO or thin {@link Kit} instance.
 */
export type KitLike = Kit | KitFullDto;
// #endregion KitEntity

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
 * Specs: Returned by KitStore.getSnapshot(). kit is the in-memory
 * {@link Kit} (or DTO) snapshot. sync describes the current synchronization state.
 **/
export type KitStoreSnapshot = {
  kit: Kit;
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
  apply(diff: unknown, meta?: { origin?: string }): void;
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
export type BackboneConfig =
  | { dev: { path: string } }
  | { local: { folder: string } }
  | { remote: { url: string; sessionId: string } };

/** JSON shape for [`semio::kit_backbone_wire::ConflictResolution`] (unit variants use `null` payload like `newSession`). */
export type ConflictResolution = { dropWip: null } | { forceOverwriteBackbone: null };

/** Payload inside `KitStoreCommandResult::BackboneStatus` (`tip` is checkpoint id when present). */
export type BackboneStatusDto = {
  attached: boolean;
  kind?: string | null;
  tip?: string | null;
};

/** Row from `KitStoreCommandResult::ListConflicts` (`items` entry). */
export type KitConflict = {
  id: string;
  wipCheckpoint: unknown;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
};

function parseKitStoreBackboneStatusResult(raw: unknown): BackboneStatusDto {
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

function parseKitStoreListConflictsResult(raw: unknown): KitConflict[] {
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
  attachBackbone(cfg: BackboneConfig): Promise<SetResult>;
  detachBackbone(): Promise<SetResult>;
  backboneStatus(): Promise<BackboneStatusDto>;
  listConflicts(): Promise<KitConflict[]>;
  resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult>;
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

  async attachBackbone(cfg: BackboneConfig): Promise<SetResult> {
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

  async backboneStatus(): Promise<BackboneStatusDto> {
    const r = await this.execute({ backboneStatus: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreBackboneStatusResult(r.result);
  }

  async listConflicts(): Promise<KitConflict[]> {
    const r = await this.execute({ listConflicts: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreListConflictsResult(r.result);
  }

  async resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> {
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

  async attachBackbone(cfg: BackboneConfig): Promise<SetResult> {
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

  async backboneStatus(): Promise<BackboneStatusDto> {
    const r = await this.execute({ backboneStatus: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreBackboneStatusResult(r.result);
  }

  async listConflicts(): Promise<KitConflict[]> {
    const r = await this.execute({ listConflicts: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitStoreListConflictsResult(r.result);
  }

  async resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> {
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
  const dto = JSON.parse(JSON.stringify(opts.initialKit)) as KitFullDto;
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
  ) { }

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
  ) { }

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
  ) { }

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
  constructor(readonly gql: KitGraphqlHandle) { }

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

async function importWasmModule(specifier: string) {
  if (specifier === "@semio/rs-wasm") {
    return import("@semio/rs-wasm");
  }
  return import(/* @vite-ignore */ specifier);
}

function settle(p: Promise<any>): Promise<any> {
  return p.catch((e: any) => ({ ok: false, error: { kind: "Internal", message: String(e) } }));
}

export class KitWorkerApi {
  private handle: any = null;
  private eventListeners = new Map<number, (ev: unknown) => void>();
  private nextEventListenerId = 0;
  private eventGqlStarted = false;

  private gql(): KitGraphqlHandle {
    if (!this.handle) throw new Error("KitStoreHandle not initialized");
    return { execute: (requestJson: string, onMessage: (line: string) => void) => this.handle.execute(requestJson, onMessage) };
  }
  private asExecuteRead(): KitExecuteRead {
    return { executeRead: (batch) => kitGraphqlExecuteRead(this.gql(), batch) };
  }
  private requireHandle(): any {
    if (!this.handle) throw new Error("KitStoreHandle not initialized");
    return this.handle;
  }

  async init(wasmSpecifier: string, dto: unknown) {
    const mod = await importWasmModule(wasmSpecifier);
    if (typeof mod.default === "function") await mod.default();
    if (typeof mod.boot === "function") mod.boot();
    this.handle = mod.KitStoreHandle.create(dto as any);
  }
  snapshot() { return this.requireHandle().snapshot(); }
  setField(kind: string, id: string, field: string, value: unknown) {
    this.requireHandle();
    return settle((async () => { try { const cmds = this.handle.changeKitCommandsForFieldPatch(kind, id, field, value); await this.handle.executeChangeKitCommands(cmds); return { ok: true }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown) {
    this.requireHandle();
    return settle((async () => { try { const cmds = this.handle.changeKitCommandsForAddChild(parentKind, parentId, childKind, dto); await this.handle.executeChangeKitCommands(cmds); return { ok: true }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string) {
    this.requireHandle();
    return settle((async () => { try { const cmds = this.handle.changeKitCommandsForRemoveChild(parentKind, parentId, childKind, childId); await this.handle.executeChangeKitCommands(cmds); return { ok: true }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
  applyDesignDiff(designId: string, diff: unknown) { this.requireHandle(); return settle(Promise.resolve(this.handle.applyDesignDiff(designId, diff))); }
  applyKitDiff(diff: unknown) { this.requireHandle(); return settle(Promise.resolve(this.handle.applyKitDiff(diff))); }
  clusterPieces(designId: string, pieceIds: string[], clusterName: string) { this.requireHandle(); return settle(Promise.resolve(this.handle.clusterPieces(designId, pieceIds, clusterName))); }
  dragPieces(designId: string, pieceIds: string[], du: number, dv: number) { this.requireHandle(); return settle(Promise.resolve(this.handle.dragPieces(designId, pieceIds, du, dv))); }
  movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number) { this.requireHandle(); return settle(Promise.resolve(this.handle.movePieces(designId, pieceIds, gap, shift, rise))); }
  fixPieces(designId: string, pieceIds: string[]) { this.requireHandle(); return settle(Promise.resolve(this.handle.fixPieces(designId, pieceIds))); }
  flattenDesign(designId: string) { this.requireHandle(); return settle(Promise.resolve(this.handle.flattenDesign(designId))); }
  expandDesign(parentDesignId: string, nestedDesignId: string) { this.requireHandle(); return settle(Promise.resolve(this.handle.expandDesign(parentDesignId, nestedDesignId))); }
  deleteConnection(designId: string, connectionId: string) { this.requireHandle(); return settle(Promise.resolve(this.handle.deleteConnection(designId, connectionId))); }
  changePieceType(designId: string, pieceId: string, newTypeId: string) { this.requireHandle(); return settle(Promise.resolve(this.handle.changePieceType(designId, pieceId, newTypeId))); }
  pasteDesignSelection(designId: string, selection: unknown, plane: unknown) { this.requireHandle(); return settle(Promise.resolve(this.handle.pasteDesignSelection(designId, selection, plane))); }
  createHangingPieces(designId: string, typeIds: string[], plane: unknown) { this.requireHandle(); return settle(Promise.resolve(this.handle.createHangingPieces(designId, typeIds, plane))); }
  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) { this.requireHandle(); return settle(Promise.resolve(this.handle.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort))); }
  createFixedPiece(designId: string, typeId: string, plane: unknown) { this.requireHandle(); return settle(Promise.resolve(this.handle.createFixedPiece(designId, typeId, plane))); }
  getPiecesMetadata(designId: string) { this.requireHandle(); return settle(kitGraphqlKitDesignPiecesMetadata(this.gql(), designId)); }
  getPieces(designId: string) {
    this.requireHandle();
    return settle((async () => { const out = await readKitDesign(this.asExecuteRead(), designId, { readDesignPiecesFullCommand: null }); if (!("readDesignPiecesFullCommand" in out) || out.readDesignPiecesFullCommand == null) throw new Error("readDesignPiecesFullCommand: missing output"); return out.readDesignPiecesFullCommand.pieces; })());
  }
  getConnections(designId: string) {
    this.requireHandle();
    return settle((async () => { const out = await readKitDesign(this.asExecuteRead(), designId, { readDesignConnectionsFullCommand: null }); if (!("readDesignConnectionsFullCommand" in out) || out.readDesignConnectionsFullCommand == null) throw new Error("readDesignConnectionsFullCommand: missing output"); return out.readDesignConnectionsFullCommand.connections; })());
  }
  getDesigns() {
    this.requireHandle();
    return settle((async () => { const out = await readKit(this.asExecuteRead(), { readKitDesignsShallowCommand: null }); if (!("readKitDesignsShallowCommand" in out) || out.readKitDesignsShallowCommand == null) throw new Error("readKitDesignsShallowCommand: missing output"); return out.readKitDesignsShallowCommand.designs; })());
  }
  getTypes() {
    this.requireHandle();
    return settle((async () => { const out = await readKit(this.asExecuteRead(), { readKitTypesShallowCommand: null }); if (!("readKitTypesShallowCommand" in out) || out.readKitTypesShallowCommand == null) throw new Error("readKitTypesShallowCommand: missing output"); return out.readKitTypesShallowCommand.types; })());
  }
  getAuthors() {
    this.requireHandle();
    return settle((async () => { const out = await readKit(this.asExecuteRead(), { readKitAuthorsShallowCommand: null }); if (!("readKitAuthorsShallowCommand" in out) || out.readKitAuthorsShallowCommand == null) throw new Error("readKitAuthorsShallowCommand: missing output"); return out.readKitAuthorsShallowCommand.authors; })());
  }
  getKitMetadata() {
    this.requireHandle();
    return settle((async () => { const out = await readKit(this.asExecuteRead(), { readKitMetadataCommand: null }); if (!("readKitMetadataCommand" in out) || out.readKitMetadataCommand == null) throw new Error("readKitMetadataCommand: missing output"); return out.readKitMetadataCommand.metadata; })());
  }
  graphqlExecute(requestJson: string, onMessage: (line: string) => void) { this.requireHandle(); return this.handle.execute(requestJson, onMessage); }
  undo() { this.requireHandle(); return settle(Promise.resolve(this.handle.undo())); }
  redo() { this.requireHandle(); return settle(Promise.resolve(this.handle.redo())); }
  canUndo() { this.requireHandle(); return this.handle.canUndo(); }
  canRedo() { this.requireHandle(); return this.handle.canRedo(); }
  subscribe(cb: (ev: unknown) => void) {
    this.requireHandle();
    const proxy = Comlink.proxy(cb);
    const id = this.nextEventListenerId++;
    const forward = (payload: unknown) => { try { proxy(payload); } catch { /* ignore */ } };
    this.eventListeners.set(id, forward);
    if (!this.eventGqlStarted) {
      this.eventGqlStarted = true;
      kitGraphqlSubscribeLoop(this.gql(), (payload) => { for (const fn of this.eventListeners.values()) fn(payload); });
    }
    return () => { this.eventListeners.delete(id); if (this.eventListeners.size === 0) this.eventGqlStarted = false; };
  }
  async execute(cmd: unknown) {
    this.requireHandle();
    try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), cmd); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; }
  }
  async executeRead(cmds: ReadCommandBatch) { this.requireHandle(); return await kitGraphqlExecuteRead(this.gql(), cmds); }
  vcsState() { return this.requireHandle().vcsState(); }
  theKitDto() { return this.requireHandle().theKitDto(); }
  materializeAt(at: unknown) { return this.requireHandle().materializeAt(at); }
  attachBackbone(config: unknown) {
    this.requireHandle();
    return settle((async () => { try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), { attachBackbone: { config } }); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
  detachBackbone() {
    this.requireHandle();
    return settle((async () => { try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), { detachBackbone: null }); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
  async backboneStatus() {
    this.requireHandle();
    try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), { backboneStatus: null }); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; }
  }
  async listConflicts() {
    this.requireHandle();
    try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), { listConflicts: null }); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; }
  }
  resolveConflict(conflictId: string, resolution: unknown) {
    this.requireHandle();
    return settle((async () => { try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), { resolveConflict: { id: conflictId, strategy: resolution } }); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
  syncNow() {
    this.requireHandle();
    return settle((async () => { try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), { syncNow: null }); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } })());
  }
}

/** Singleton instance for backwards compatibility. */
export const kitWorkerApi = new KitWorkerApi();

/** Call in a Web Worker context to expose `kitWorkerApi` via Comlink. */
export function bootKitWorker() {
  Comlink.expose(kitWorkerApi);
}

//#endregion 🧵KitWorker

// #region 🧪EmbeddedTests

if (process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  describe("semio-js thin client", () => {
    it("round-trips an empty kit through KitSchema", () => {
      const dto = {
        id: "kit-embedded-1",
        name: "Embedded",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
      };
      const k = Kit.fromPlain(dto);
      expect(k.toPlain().id).toBe("kit-embedded-1");
    });
  });
}
// #endregion 🧪EmbeddedTests

