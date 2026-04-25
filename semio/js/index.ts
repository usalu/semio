// @ts-nocheck
// #region ­ƒº▓Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core domain representation types, schemas and utilities for the semio platform.

// #endregion ­ƒº▓Header

// #region Imports
// External dependency imports MUST be declared here.
import { z } from "zod";
// #endregion Imports

// #region Constants
// Global constants MUST define shared numeric parameters.

/** Standard icon width in pixels.
 **/
export const ICON_WIDTH = 50;
/**
 * Numeric tolerance for floating-point comparisons.
 **/
export const TOLERANCE = 1e-5;

// #endregion Constants

// #region Utilities
// Removed: toArray, SeededRandom, Generator, round, jaccard, deepEqual, arraysEqual — domain logic moved to semio/rs (Requirements 1.3, 3.5)

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

// #region Semio
// Semio utility class delegates to WASM for domain operations.

/**
 * Semio utility class with static methods delegating to WASM.
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

// #endregion Semio

// #endregion Utilities

// #region Entity IDs
// Entity identifier types and comparison functions MUST be defined here.

export type AttributeId = { id: Id };
export type LocationId = { id: Id };
export type AuthorId = { id: Id };
export type FileId = { id: Id };
export type FolderId = { id: Id };
export type BenchmarkId = { id: Id };
export type QualityId = { id: Id };
export type PortId = { id: Id };
export type PropId = { id: Id };
export type RepresentationId = { id: Id };
export type ConnectorId = { id: Id };
export type TypeId = { id: Id };
export type LayerId = { id: Id };
export type PieceId = { id: Id };
export type GroupId = { id: Id };
export type ConnectionId = { id: Id };
export type StatId = { id: Id };
export type DesignId = { id: Id };
export type KitId = { id: Id };
export type TagId = { id: Id };
export type ConceptId = { id: Id };
export type FamilyId = { id: Id };

export const AttributeIdSchema = z.object({ id: z.string() });
export const LocationIdSchema = z.object({ id: z.string() });
export const AuthorIdSchema = z.object({ id: z.string() });
export const FileIdSchema = z.object({ id: z.string() });
export const FolderIdSchema = z.object({ id: z.string() });
export const BenchmarkIdSchema = z.object({ id: z.string() });
export const QualityIdSchema = z.object({ id: z.string() });
export const PortIdSchema = z.object({ id: z.string() });
export const PropIdSchema = z.object({ id: z.string() });
export const RepresentationIdSchema = z.object({ id: z.string() });
export const ConnectorIdSchema = z.object({ id: z.string() });
export const TypeIdSchema = z.object({ id: z.string() });
export const LayerIdSchema = z.object({ id: z.string() });
export const PieceIdSchema = z.object({ id: z.string() });
export const GroupIdSchema = z.object({ id: z.string() });
export const ConnectionIdSchema = z.object({ id: z.string() });
export const StatIdSchema = z.object({ id: z.string() });
export const DesignIdSchema = z.object({ id: z.string() });
export const KitIdSchema = z.object({ id: z.string() });
export const TagIdSchema = z.object({ id: z.string() });
export const ConceptIdSchema = z.object({ id: z.string() });
export const FamilyIdSchema = z.object({ id: z.string() });

// Removed: All free create*Id, areSame*Id, get*Id functions — use Entity.createId/areSameId static methods (Requirement 3.2)

// #endregion Entity IDs

// #region Weak Entities

// #region Coordinate
export const CoordinateSchema = z.object({ u: z.number(), v: z.number() });
export type CoordinatePlain = z.infer<typeof CoordinateSchema>;
export class Coordinate implements CoordinatePlain {
  u!: number;
  v!: number;
  constructor(plain: CoordinatePlain) {
    Object.assign(this, CoordinateSchema.parse(plain));
  }
  static from(plain: CoordinatePlain): Coordinate { return new Coordinate(plain); }
  toPlain(): CoordinatePlain { return CoordinateSchema.parse(this as unknown as CoordinatePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Coordinate { return new Coordinate(CoordinateSchema.parse(JSON.parse(json))); }
}
export const CoordinateDiffSchema = CoordinateSchema.partial();
export type CoordinateDiff = z.infer<typeof CoordinateDiffSchema>;
// #endregion Coordinate

// #region Vec
export const VecSchema = z.object({ u: z.number(), v: z.number() });
export type VecPlain = z.infer<typeof VecSchema>;
export class Vec implements VecPlain {
  u!: number;
  v!: number;
  constructor(plain: VecPlain) { Object.assign(this, VecSchema.parse(plain)); }
  static from(plain: VecPlain): Vec { return new Vec(plain); }
  static fromPlain(plain: VecPlain): Vec { return new Vec(plain); }
  toPlain(): VecPlain { return VecSchema.parse(this as unknown as VecPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Vec { return new Vec(VecSchema.parse(JSON.parse(json))); }
}
export const VecDiffSchema = VecSchema.partial();
export type VecDiff = z.infer<typeof VecDiffSchema>;
// #endregion Vec

// #region Point
export const PointSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type PointPlain = z.infer<typeof PointSchema>;
export class Point implements PointPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: PointPlain) { Object.assign(this, PointSchema.parse(plain)); }
  static from(plain: PointPlain): Point { return new Point(plain); }
  static fromPlain(plain: PointPlain): Point { return new Point(plain); }
  toPlain(): PointPlain { return PointSchema.parse(this as unknown as PointPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Point { return new Point(PointSchema.parse(JSON.parse(json))); }
}
export const PointDiffSchema = PointSchema.partial();
export type PointDiff = z.infer<typeof PointDiffSchema>;
// #endregion Point

// #region Vector
export const VectorSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type VectorPlain = z.infer<typeof VectorSchema>;
export class Vector implements VectorPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: VectorPlain) { Object.assign(this, VectorSchema.parse(plain)); }
  static from(plain: VectorPlain): Vector { return new Vector(plain); }
  static fromPlain(plain: VectorPlain): Vector { return new Vector(plain); }
  toPlain(): VectorPlain { return VectorSchema.parse(this as unknown as VectorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Vector { return new Vector(VectorSchema.parse(JSON.parse(json))); }
}
export const VectorDiffSchema = VectorSchema.partial();
export type VectorDiff = z.infer<typeof VectorDiffSchema>;
// #endregion Vector

// #region Plane
export const PlaneSchema = z.object({ origin: PointSchema, xAxis: VectorSchema, yAxis: VectorSchema });
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
  static from(plain: PlanePlain): Plane { return new Plane(plain); }
  static fromPlain(plain: PlanePlain): Plane { return new Plane(plain); }
  toPlain(): PlanePlain { return PlaneSchema.parse(this as unknown as PlanePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Plane { return new Plane(PlaneSchema.parse(JSON.parse(json))); }
  // Removed: averageWith, average, rounded — geometry computation moved to semio/rs (Requirement 1.14)
}
export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true })
  .extend({ origin: PointDiffSchema, xAxis: VectorDiffSchema, yAxis: VectorDiffSchema }).partial();
export type PlaneDiff = z.infer<typeof PlaneDiffSchema>;
// #endregion Plane

// #region Camera
export const CameraSchema = z.object({ position: PointSchema, forward: VectorSchema, up: VectorSchema });
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
  static from(plain: CameraPlain): Camera { return new Camera(plain); }
  static fromPlain(plain: CameraPlain): Camera { return new Camera(plain); }
  toPlain(): CameraPlain { return CameraSchema.parse(this as unknown as CameraPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Camera { return new Camera(CameraSchema.parse(JSON.parse(json))); }
}
export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true })
  .extend({ position: PointDiffSchema, forward: VectorDiffSchema, up: VectorDiffSchema }).partial();
export type CameraDiff = z.infer<typeof CameraDiffSchema>;
// #endregion Camera

// #endregion Weak Entities

// #region Attribute
const DateProperty = () => z.string().optional();
export const AttributeSchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), definition: z.string().optional() });
export type AttributePlain = z.infer<typeof AttributeSchema>;
export class Attribute implements AttributePlain {
  id!: string; key!: string; value?: string; definition?: string;
  constructor(plain: AttributePlain) { Object.assign(this, AttributeSchema.parse(plain)); }
  static from(plain: AttributePlain): Attribute { return new Attribute(plain); }
  static fromPlain(plain: AttributePlain): Attribute { return new Attribute(plain); }
  static createId(id: string): AttributeId { return { id }; }
  static areSameId(a: AttributeId, b: AttributeId): boolean { return a.id === b.id; }
  toPlain(): AttributePlain { return AttributeSchema.parse(this as unknown as AttributePlain); }
  toJson(): string { return JSON.stringify(this.toPlain()); }
  static fromJson(json: string): Attribute { return new Attribute(AttributeSchema.parse(JSON.parse(json))); }
}
export const AttributeMetadataDtoSchema = AttributeSchema;
export type AttributeMetadataDto = z.infer<typeof AttributeMetadataDtoSchema>;
export const AttributeShallowSchema = AttributeSchema;
export type AttributeShallow = z.infer<typeof AttributeShallowSchema>;
export const AttributeDiffSchema = AttributeSchema.partial();
export type AttributeDiff = z.infer<typeof AttributeDiffSchema>;
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;
// #endregion Attribute

// #region Location
export const LocationSchema = z.object({ id: z.string(), longitude: z.number().optional(), latitude: z.number().optional(), altitude: z.number().optional(), attributes: z.array(AttributeSchema).optional() });
export type LocationPlain = z.infer<typeof LocationSchema>;
export class Location implements LocationPlain {
  id!: string; longitude?: number; latitude?: number; altitude?: number; attributes?: Attribute[];
  constructor(plain: LocationPlain) { const p = LocationSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: LocationPlain): Location { return new Location(plain); }
  static fromPlain(plain: LocationPlain): Location { return new Location(plain); }
  static createId(id: string): LocationId { return { id }; }
  static areSameId(a: LocationId, b: LocationId): boolean { return a.id === b.id; }
  toPlain(): LocationPlain { return LocationSchema.parse(this as unknown as LocationPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Location { return new Location(LocationSchema.parse(JSON.parse(json))); }
}
export const LocationMetadataDtoSchema = LocationSchema;
export type LocationMetadataDto = z.infer<typeof LocationMetadataDtoSchema>;
export const LocationShallowSchema = LocationSchema;
export type LocationShallow = z.infer<typeof LocationShallowSchema>;
export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LocationDiff = z.infer<typeof LocationDiffSchema>;
// #endregion Location

// #region Author
export const AuthorSchema = z.object({ id: z.string(), name: z.string(), email: z.string().optional(), role: z.string().optional(), rank: z.number().optional() });
export type AuthorPlain = z.infer<typeof AuthorSchema>;
export class Author implements AuthorPlain {
  id!: string; name!: string; email?: string; role?: string; rank?: number;
  constructor(plain: AuthorPlain) { Object.assign(this, AuthorSchema.parse(plain)); }
  static from(plain: AuthorPlain): Author { return new Author(plain); }
  static fromPlain(plain: AuthorPlain): Author { return new Author(plain); }
  static createId(id: string): AuthorId { return { id }; }
  static areSameId(a: AuthorId, b: AuthorId): boolean { return a.id === b.id; }
  toPlain(): AuthorPlain { return AuthorSchema.parse(this as unknown as AuthorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Author { return new Author(AuthorSchema.parse(JSON.parse(json))); }
}
export const AuthorMetadataDtoSchema = AuthorSchema;
export type AuthorMetadataDto = z.infer<typeof AuthorMetadataDtoSchema>;
export const AuthorShallowSchema = AuthorSchema;
export type AuthorShallow = z.infer<typeof AuthorShallowSchema>;
export const AuthorDiffSchema = AuthorSchema.partial();
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;
export const AuthorsDiffSchema = z.object({ removed: z.array(AuthorIdSchema).optional(), updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;
// #endregion Author

// #region File
export const FileSchema = z.object({ id: z.string(), url: z.string().optional(), mime: z.string().optional(), size: z.number().optional(), hash: z.string().optional(), description: z.string().optional(), createdAt: DateProperty(), updatedAt: DateProperty() });
export type FilePlain = z.infer<typeof FileSchema>;
export class File implements FilePlain {
  id!: string; url?: string; mime?: string; size?: number; hash?: string; description?: string; createdAt?: string; updatedAt?: string;
  constructor(plain: FilePlain) { Object.assign(this, FileSchema.parse(plain)); }
  static from(plain: FilePlain): File { return new File(plain); }
  static fromPlain(plain: FilePlain): File { return new File(plain); }
  static createId(id: string): FileId { return { id }; }
  static areSameId(a: FileId, b: FileId): boolean { return a.id === b.id; }
  toPlain(): FilePlain { return FileSchema.parse(this as unknown as FilePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): File { return new File(FileSchema.parse(JSON.parse(json))); }
}
export const FileMetadataDtoSchema = FileSchema;
export type FileMetadataDto = z.infer<typeof FileMetadataDtoSchema>;
export const FileShallowSchema = FileSchema;
export type FileShallow = z.infer<typeof FileShallowSchema>;
export const FileDiffSchema = FileSchema.partial();
export type FileDiff = z.infer<typeof FileDiffSchema>;
export const FilesDiffSchema = z.object({ removed: z.array(FileIdSchema).optional(), updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FilesDiff = z.infer<typeof FilesDiffSchema>;
// #endregion File

// #region Folder
export const FolderSchema = z.object({ id: z.string(), path: z.string(), description: z.string().optional() });
export type FolderPlain = z.infer<typeof FolderSchema>;
export class Folder implements FolderPlain {
  id!: string; path!: string; description?: string;
  constructor(plain: FolderPlain) { Object.assign(this, FolderSchema.parse(plain)); }
  static from(plain: FolderPlain): Folder { return new Folder(plain); }
  static fromPlain(plain: FolderPlain): Folder { return new Folder(plain); }
  static createId(id: string): FolderId { return { id }; }
  static areSameId(a: FolderId, b: FolderId): boolean { return a.id === b.id; }
  toPlain(): FolderPlain { return FolderSchema.parse(this as unknown as FolderPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Folder { return new Folder(FolderSchema.parse(JSON.parse(json))); }
}
export const FolderMetadataDtoSchema = FolderSchema;
export type FolderMetadataDto = z.infer<typeof FolderMetadataDtoSchema>;
export const FolderShallowSchema = FolderSchema;
export type FolderShallow = z.infer<typeof FolderShallowSchema>;
export const FolderDiffSchema = FolderSchema.partial();
export type FolderDiff = z.infer<typeof FolderDiffSchema>;
export const FoldersDiffSchema = z.object({ removed: z.array(FolderIdSchema).optional(), updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;
// #endregion Folder

// #region Benchmark
export const BenchmarkSchema = z.object({ id: z.string(), name: z.string(), min: z.number().optional(), max: z.number().optional(), minExcluded: z.boolean().optional(), maxExcluded: z.boolean().optional() });
export type BenchmarkPlain = z.infer<typeof BenchmarkSchema>;
export class Benchmark implements BenchmarkPlain {
  id!: string; name!: string; min?: number; max?: number; minExcluded?: boolean; maxExcluded?: boolean;
  constructor(plain: BenchmarkPlain) { Object.assign(this, BenchmarkSchema.parse(plain)); }
  static from(plain: BenchmarkPlain): Benchmark { return new Benchmark(plain); }
  static fromPlain(plain: BenchmarkPlain): Benchmark { return new Benchmark(plain); }
  static createId(id: string): BenchmarkId { return { id }; }
  static areSameId(a: BenchmarkId, b: BenchmarkId): boolean { return a.id === b.id; }
  toPlain(): BenchmarkPlain { return BenchmarkSchema.parse(this as unknown as BenchmarkPlain); }
  toJson(): string { return JSON.stringify(this.toPlain()); }
  static fromJson(json: string): Benchmark { return new Benchmark(BenchmarkSchema.parse(JSON.parse(json))); }
}
export const BenchmarkMetadataDtoSchema = BenchmarkSchema;
export type BenchmarkMetadataDto = z.infer<typeof BenchmarkMetadataDtoSchema>;
export const BenchmarkShallowSchema = BenchmarkSchema;
export type BenchmarkShallow = z.infer<typeof BenchmarkShallowSchema>;
export const BenchmarkDiffSchema = BenchmarkSchema.partial();
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;
export const BenchmarksDiffSchema = z.object({ removed: z.array(BenchmarkIdSchema).optional(), updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;
// #endregion Benchmark

// #region Quality
export const QualitySchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), unit: z.string().optional(), definition: z.string().optional(), description: z.string().optional(), benchmarks: z.array(BenchmarkSchema).optional() });
export type QualityPlain = z.infer<typeof QualitySchema>;
export class Quality implements QualityPlain {
  id!: string; key!: string; value?: string; unit?: string; definition?: string; description?: string; benchmarks?: Benchmark[];
  constructor(plain: QualityPlain) { const p = QualitySchema.parse(plain); Object.assign(this, p); this.benchmarks = p.benchmarks?.map((b) => new Benchmark(b)); }
  static from(plain: QualityPlain): Quality { return new Quality(plain); }
  static fromPlain(plain: QualityPlain): Quality { return new Quality(plain); }
  static createId(id: string): QualityId { return { id }; }
  static areSameId(a: QualityId, b: QualityId): boolean { return a.id === b.id; }
  toPlain(): QualityPlain { return QualitySchema.parse(this as unknown as QualityPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Quality { return new Quality(QualitySchema.parse(JSON.parse(json))); }
}
export const QualityMetadataDtoSchema = QualitySchema.omit({ benchmarks: true });
export type QualityMetadataDto = z.infer<typeof QualityMetadataDtoSchema>;
export const QualityShallowSchema = QualitySchema;
export type QualityShallow = z.infer<typeof QualityShallowSchema>;
export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true }).extend({ benchmarks: BenchmarksDiffSchema.optional() });
export type QualityDiff = z.infer<typeof QualityDiffSchema>;
export const QualitiesDiffSchema = z.object({ removed: z.array(QualityIdSchema).optional(), updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type QualitiesDiff = z.infer<typeof QualitiesDiffSchema>;
// #endregion Quality

// #region Port
export const PortSchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), icon: z.string().optional(), compatibleFamilies: z.array(FamilyIdSchema).optional(), mandatory: z.boolean().optional(), t: z.number().optional(), point: PointSchema.optional(), direction: VectorSchema.optional(), compatiblePorts: z.array(PortIdSchema).optional(), qualities: z.array(QualitySchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type PortPlain = z.infer<typeof PortSchema>;
export class Port implements PortPlain {
  id!: string; name!: string; description?: string; icon?: string; compatibleFamilies?: FamilyId[]; mandatory?: boolean; t?: number; point?: Point; direction?: Vector; compatiblePorts?: PortId[]; qualities?: Quality[]; attributes?: Attribute[];
  constructor(plain: PortPlain) { const p = PortSchema.parse(plain); Object.assign(this, p); this.point = p.point ? new Point(p.point) : undefined; this.direction = p.direction ? new Vector(p.direction) : undefined; this.qualities = p.qualities?.map((q) => new Quality(q)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: PortPlain): Port { return new Port(plain); }
  static fromPlain(plain: PortPlain): Port { return new Port(plain); }
  static createId(id: string): PortId { return { id }; }
  static areSameId(a: PortId, b: PortId): boolean { return a.id === b.id; }
  toPlain(): PortPlain { return PortSchema.parse(this as unknown as PortPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Port { return new Port(PortSchema.parse(JSON.parse(json))); }
}
export const PortMetadataDtoSchema = PortSchema.omit({ qualities: true, attributes: true });
export type PortMetadataDto = z.infer<typeof PortMetadataDtoSchema>;
export const PortShallowSchema = PortSchema;
export type PortShallow = z.infer<typeof PortShallowSchema>;
export const PortDiffSchema = PortSchema.partial().omit({ qualities: true, attributes: true }).extend({ qualities: QualitiesDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PortDiff = z.infer<typeof PortDiffSchema>;
export const PortsDiffSchema = z.object({ removed: z.array(PortIdSchema).optional(), updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PortsDiff = z.infer<typeof PortsDiffSchema>;
// #endregion Port

// #region Family
export const FamilySchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), icon: z.string().optional(), ports: z.array(PortSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type FamilyPlain = z.infer<typeof FamilySchema>;
export class Family implements FamilyPlain {
  id!: string; name!: string; description?: string; icon?: string; ports?: Port[]; attributes?: Attribute[];
  constructor(plain: FamilyPlain) { const p = FamilySchema.parse(plain); Object.assign(this, p); this.ports = p.ports?.map((x) => new Port(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: FamilyPlain): Family { return new Family(plain); }
  static fromPlain(plain: FamilyPlain): Family { return new Family(plain); }
  static createId(id: string): FamilyId { return { id }; }
  static areSameId(a: FamilyId, b: FamilyId): boolean { return a.id === b.id; }
  toPlain(): FamilyPlain { return FamilySchema.parse(this as unknown as FamilyPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Family { return new Family(FamilySchema.parse(JSON.parse(json))); }
}
export const FamilyMetadataDtoSchema = FamilySchema.omit({ ports: true, attributes: true });
export type FamilyMetadataDto = z.infer<typeof FamilyMetadataDtoSchema>;
export const FamilyShallowSchema = FamilySchema;
export type FamilyShallow = z.infer<typeof FamilyShallowSchema>;
export const FamilyDiffSchema = FamilySchema.partial().omit({ ports: true, attributes: true }).extend({ ports: PortsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type FamilyDiff = z.infer<typeof FamilyDiffSchema>;
export const FamiliesDiffSchema = z.object({ removed: z.array(FamilyIdSchema).optional(), updated: z.array(z.object({ family: FamilyIdSchema, diff: FamilyDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FamiliesDiff = z.infer<typeof FamiliesDiffSchema>;
// #endregion Family

// #region Prop
export const PropSchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), unit: z.string().optional(), quality: QualityIdSchema.optional() });
export type PropPlain = z.infer<typeof PropSchema>;
export class Prop implements PropPlain {
  id!: string; key!: string; value?: string; unit?: string; quality?: QualityId;
  constructor(plain: PropPlain) { Object.assign(this, PropSchema.parse(plain)); }
  static from(plain: PropPlain): Prop { return new Prop(plain); }
  static fromPlain(plain: PropPlain): Prop { return new Prop(plain); }
  static createId(id: string): PropId { return { id }; }
  static areSameId(a: PropId, b: PropId): boolean { return a.id === b.id; }
  toPlain(): PropPlain { return PropSchema.parse(this as unknown as PropPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Prop { return new Prop(PropSchema.parse(JSON.parse(json))); }
}
export const PropMetadataDtoSchema = PropSchema;
export type PropMetadataDto = z.infer<typeof PropMetadataDtoSchema>;
export const PropShallowSchema = PropSchema;
export type PropShallow = z.infer<typeof PropShallowSchema>;
export const PropDiffSchema = PropSchema.partial();
export type PropDiff = z.infer<typeof PropDiffSchema>;
export const PropsDiffSchema = z.object({ removed: z.array(PropIdSchema).optional(), updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PropsDiff = z.infer<typeof PropsDiffSchema>;
// #endregion Prop

// #region Tag
export const TagSchema = z.object({ id: z.string(), name: z.string(), order: z.number().optional() });
export type TagPlain = z.infer<typeof TagSchema>;
export class Tag implements TagPlain {
  id!: string; name!: string; order?: number;
  constructor(plain: TagPlain) { Object.assign(this, TagSchema.parse(plain)); }
  static from(plain: TagPlain): Tag { return new Tag(plain); }
  static fromPlain(plain: TagPlain): Tag { return new Tag(plain); }
  static createId(id: string): TagId { return { id }; }
  static areSameId(a: TagId, b: TagId): boolean { return a.id === b.id; }
  toPlain(): TagPlain { return TagSchema.parse(this as unknown as TagPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Tag { return new Tag(TagSchema.parse(JSON.parse(json))); }
}
export const TagMetadataDtoSchema = TagSchema;
export type TagMetadataDto = z.infer<typeof TagMetadataDtoSchema>;
export const TagShallowSchema = TagSchema;
export type TagShallow = z.infer<typeof TagShallowSchema>;
export const TagDiffSchema = TagSchema.partial();
export type TagDiff = z.infer<typeof TagDiffSchema>;
export const TagsDiffSchema = z.object({ removed: z.array(TagIdSchema).optional(), updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TagsDiff = z.infer<typeof TagsDiffSchema>;
// #endregion Tag

// #region Concept
export const ConceptSchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), order: z.number().optional() });
export type ConceptPlain = z.infer<typeof ConceptSchema>;
export class Concept implements ConceptPlain {
  id!: string; name!: string; description?: string; order?: number;
  constructor(plain: ConceptPlain) { Object.assign(this, ConceptSchema.parse(plain)); }
  static from(plain: ConceptPlain): Concept { return new Concept(plain); }
  static fromPlain(plain: ConceptPlain): Concept { return new Concept(plain); }
  static createId(id: string): ConceptId { return { id }; }
  static areSameId(a: ConceptId, b: ConceptId): boolean { return a.id === b.id; }
  toPlain(): ConceptPlain { return ConceptSchema.parse(this as unknown as ConceptPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Concept { return new Concept(ConceptSchema.parse(JSON.parse(json))); }
}
export const ConceptMetadataDtoSchema = ConceptSchema;
export type ConceptMetadataDto = z.infer<typeof ConceptMetadataDtoSchema>;
export const ConceptShallowSchema = ConceptSchema;
export type ConceptShallow = z.infer<typeof ConceptShallowSchema>;
export const ConceptDiffSchema = ConceptSchema.partial();
export type ConceptDiff = z.infer<typeof ConceptDiffSchema>;
export const ConceptsDiffSchema = z.object({ removed: z.array(ConceptIdSchema).optional(), updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConceptsDiff = z.infer<typeof ConceptsDiffSchema>;
// #endregion Concept

// #region Representation
export const RepresentationSchema = z.object({ id: z.string(), name: z.string().optional(), tags: z.array(TagIdSchema).optional(), file: FileIdSchema, description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type RepresentationPlain = z.infer<typeof RepresentationSchema>;
export class Representation implements RepresentationPlain {
  id!: string; name?: string; tags?: TagId[]; file!: FileId; description?: string; attributes?: Attribute[];
  constructor(plain: RepresentationPlain) { const p = RepresentationSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: RepresentationPlain): Representation { return new Representation(plain); }
  static fromPlain(plain: RepresentationPlain): Representation { return new Representation(plain); }
  static createId(id: string): RepresentationId { return { id }; }
  static areSameId(a: RepresentationId, b: RepresentationId): boolean { return a.id === b.id; }
  toPlain(): RepresentationPlain { return RepresentationSchema.parse(this as unknown as RepresentationPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Representation { return new Representation(RepresentationSchema.parse(JSON.parse(json))); }
}
export const RepresentationMetadataDtoSchema = RepresentationSchema.omit({ tags: true, attributes: true });
export type RepresentationMetadataDto = z.infer<typeof RepresentationMetadataDtoSchema>;
export const RepresentationShallowSchema = RepresentationSchema;
export type RepresentationShallow = z.infer<typeof RepresentationShallowSchema>;
export const RepresentationDiffSchema = RepresentationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;
export const RepresentationsDiffSchema = z.object({ removed: z.array(RepresentationIdSchema).optional(), updated: z.array(z.object({ representation: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type RepresentationsDiff = z.infer<typeof RepresentationsDiffSchema>;
// Removed: selectBestRepresentation, filterRepresentationsByTagIds, getAvailableTagIdsForRepresentations, getAllTagIdsFromRepresentations, findRepresentation, areSameRepresentation, SUPPORTED_3D_EXTENSIONS, isSupportedRepresentationExtension, validateRepresentationFile, RepresentationFileValidation — representation selection logic moved to semio/rs (Requirement 1.3)
// #endregion Representation

// #region Connector
export const ConnectorSchema = z.object({ id: z.string(), name: z.string().optional(), t: z.number(), point: PointSchema, direction: VectorSchema, description: z.string().optional(), port: PortIdSchema.optional(), mandatory: z.boolean().optional(), maxChildren: z.number().int().optional(), props: z.array(PropSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type ConnectorPlain = z.infer<typeof ConnectorSchema>;
export class Connector implements ConnectorPlain {
  id!: string; name?: string; t!: number; point!: Point; direction!: Vector; description?: string; port?: PortId; mandatory?: boolean; maxChildren?: number; props?: Prop[]; attributes?: Attribute[];
  constructor(plain: ConnectorPlain) { const p = ConnectorSchema.parse(plain); Object.assign(this, p); this.point = new Point(p.point); this.direction = new Vector(p.direction); this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: ConnectorPlain): Connector { return new Connector(plain); }
  static fromPlain(plain: ConnectorPlain): Connector { return new Connector(plain); }
  static createId(id: string): ConnectorId { return { id }; }
  static areSameId(a: ConnectorId, b: ConnectorId): boolean { return a.id === b.id; }
  toPlain(): ConnectorPlain { return ConnectorSchema.parse(this as unknown as ConnectorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Connector { return new Connector(ConnectorSchema.parse(JSON.parse(json))); }
}
export const ConnectorMetadataDtoSchema = ConnectorSchema.omit({ props: true, attributes: true });
export type ConnectorMetadataDto = z.infer<typeof ConnectorMetadataDtoSchema>;
export const ConnectorShallowSchema = ConnectorSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type ConnectorShallow = z.infer<typeof ConnectorShallowSchema>;
export const ConnectorDiffSchema = ConnectorSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({ point: PointDiffSchema.optional(), direction: VectorDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), maxChildren: z.number().int().nullable().optional() });
export type ConnectorDiff = z.infer<typeof ConnectorDiffSchema>;
export const ConnectorsDiffSchema = z.object({ removed: z.array(ConnectorIdSchema).optional(), updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectorsDiff = z.infer<typeof ConnectorsDiffSchema>;
// Removed: areConnectorsCompatible, unifyConnectorPortsAndCompatiblePortsForTypes, findConnector, findConnectorInType — connector compatibility moved to semio/rs (Requirement 1.5)
// #endregion Connector

// #region Type
export type EntityLifecycle = "active" | "deleted";
export const TypeSchema = z.object({ id: z.string(), name: z.string(), families: z.array(FamilyIdSchema).optional(), isAbstract: z.boolean().optional(), folder: z.string().optional(), representations: z.array(RepresentationSchema).optional(), connectors: z.array(ConnectorSchema).optional(), props: z.array(PropSchema).optional(), stock: z.number().optional(), virtual: z.boolean().optional(), unit: z.string().optional(), createdAt: DateProperty(), updatedAt: DateProperty(), location: LocationIdSchema.optional(), authors: z.array(AuthorIdSchema).optional(), concepts: z.array(ConceptIdSchema).optional(), icon: z.string().optional(), image: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional(), lifecycle: z.enum(["active", "deleted"]).optional(), deletedByUserId: z.string().optional(), deletedByDisplayName: z.string().optional(), deletedAt: z.string().optional(), deletedInChangeId: z.string().optional() });
export type TypePlain = z.infer<typeof TypeSchema>;
export class Type {
  id!: string; name!: string; families?: FamilyId[]; isAbstract?: boolean; folder?: string; representations?: Representation[]; connectors?: Connector[]; props?: Prop[]; stock?: number; virtual?: boolean; unit?: string; createdAt?: string; updatedAt?: string; location?: LocationId; authors?: AuthorId[]; concepts?: ConceptId[]; icon?: string; image?: string; description?: string; attributes?: Attribute[]; lifecycle?: EntityLifecycle; deletedByUserId?: string; deletedByDisplayName?: string; deletedAt?: string; deletedInChangeId?: string;
  constructor(plain: TypePlain) { const p = TypeSchema.parse(plain); Object.assign(this, p); this.representations = p.representations?.map((m) => new Representation(m)); this.connectors = p.connectors?.map((c) => new Connector(c)); this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static fromPlain(plain: TypePlain): Type { return new Type(plain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Type { return Type.fromPlain(TypeSchema.parse(JSON.parse(json))); }
  toPlain(): TypePlain { return TypeSchema.parse({ ...(this as unknown as TypePlain) }); }
  static createId(id: string): TypeId { return { id }; }
  static areSameId(a: TypeId, b: TypeId): boolean { return a.id === b.id; }
}
export const TypeMetadataDtoSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
export type TypeMetadataDto = z.infer<typeof TypeMetadataDtoSchema>;
export const TypeShallowSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true }).extend({ representations: z.array(RepresentationMetadataDtoSchema).optional(), connectors: z.array(ConnectorMetadataDtoSchema).optional(), props: z.array(PropMetadataDtoSchema).optional(), attributes: z.array(AttributeMetadataDtoSchema).optional() });
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
export const TypeDiffSchema = TypeSchema.partial().omit({ representations: true, connectors: true, props: true, attributes: true }).extend({ representations: RepresentationsDiffSchema.optional(), connectors: ConnectorsDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), description: z.string().nullable().optional(), icon: z.string().nullable().optional(), image: z.string().nullable().optional(), location: LocationIdSchema.nullable().optional(), folder: z.string().nullable().optional(), concepts: z.array(ConceptIdSchema).nullable().optional(), authors: z.array(AuthorIdSchema).nullable().optional(), families: z.array(FamilyIdSchema).nullable().optional(), lifecycle: z.enum(["active", "deleted"]).optional(), deletedByUserId: z.string().nullable().optional(), deletedByDisplayName: z.string().nullable().optional(), deletedAt: z.string().nullable().optional(), deletedInChangeId: z.string().nullable().optional() });
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
export const TypesDiffSchema = z.object({ removed: z.array(TypeIdSchema).optional(), updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TypesDiff = z.infer<typeof TypesDiffSchema>;
// #endregion Type

// #region Layer
export const LayerSchema = z.object({ id: z.string(), path: z.string(), isHidden: z.boolean().optional(), isLocked: z.boolean().optional(), color: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type LayerPlain = z.infer<typeof LayerSchema>;
export class Layer implements LayerPlain {
  id!: string; path!: string; isHidden?: boolean; isLocked?: boolean; color?: string; description?: string; attributes?: Attribute[];
  constructor(plain: LayerPlain) { const p = LayerSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: LayerPlain): Layer { return new Layer(plain); }
  static fromPlain(plain: LayerPlain): Layer { return new Layer(plain); }
  static createId(id: string): LayerId { return { id }; }
  static areSameId(a: LayerId, b: LayerId): boolean { return a.id === b.id; }
  toPlain(): LayerPlain { return LayerSchema.parse(this as unknown as LayerPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Layer { return new Layer(LayerSchema.parse(JSON.parse(json))); }
}
export const LayerMetadataDtoSchema = LayerSchema.omit({ attributes: true });
export type LayerMetadataDto = z.infer<typeof LayerMetadataDtoSchema>;
export const LayerShallowSchema = LayerSchema;
export type LayerShallow = z.infer<typeof LayerShallowSchema>;
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LayerDiff = z.infer<typeof LayerDiffSchema>;
export const LayersDiffSchema = z.object({ removed: z.array(LayerIdSchema).optional(), updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type LayersDiff = z.infer<typeof LayersDiffSchema>;
// #endregion Layer

// #region Piece
export const PieceSchema = z.object({ id: z.string(), name: z.string().optional(), type: TypeIdSchema.optional(), design: DesignIdSchema.optional(), plane: PlaneSchema.optional(), center: CoordinateSchema.optional(), scale: z.number().optional(), mirrorPlane: PlaneSchema.optional(), isHidden: z.boolean().optional(), isLocked: z.boolean().optional(), color: z.string().optional(), description: z.string().optional(), props: z.array(PropSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type PiecePlain = z.infer<typeof PieceSchema>;
export class Piece {
  id!: string; name?: string; type?: TypeId; design?: DesignId; plane?: Plane; center?: Coordinate; scale?: number; mirrorPlane?: Plane; isHidden?: boolean; isLocked?: boolean; color?: string; description?: string; props?: Prop[]; attributes?: Attribute[];
  constructor(plain: PiecePlain) { const p = PieceSchema.parse(plain); Object.assign(this, p); this.plane = p.plane ? new Plane(p.plane) : undefined; this.center = p.center ? new Coordinate(p.center) : undefined; this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined; this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static fromPlain(plain: PiecePlain): Piece { return new Piece(plain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Piece { return new Piece(PieceSchema.parse(JSON.parse(json))); }
  toPlain(): PiecePlain { return PieceSchema.parse(this as unknown as PiecePlain); }
  static createId(id: string): PieceId { return { id }; }
  static areSameId(a: PieceId, b: PieceId): boolean { return a.id === b.id; }
}
export const PieceMetadataDtoSchema = PieceSchema.omit({ props: true, attributes: true });
export type PieceMetadataDto = z.infer<typeof PieceMetadataDtoSchema>;
export const PieceShallowSchema = PieceSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type PieceShallow = z.infer<typeof PieceShallowSchema>;
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({ plane: PlaneDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export const PiecesDiffSchema = z.object({ removed: z.array(PieceIdSchema).optional(), updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;
// Removed: isFixedPiece, findPiece, findPieceConnections, findConnectorForPieceInConnection, getPieceRepresentationFileIds, getPieceRepresentationUrls, resolvePieceTypeForFlatten — domain logic moved to semio/rs
// #endregion Piece

// #region Group
export const GroupSchema = z.object({ id: z.string(), pieces: z.array(PieceIdSchema), color: z.string().optional(), name: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type GroupPlain = z.infer<typeof GroupSchema>;
export class Group implements GroupPlain {
  id!: string; pieces!: PieceId[]; color?: string; name?: string; description?: string; attributes?: Attribute[];
  constructor(plain: GroupPlain) { const p = GroupSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: GroupPlain): Group { return new Group(plain); }
  static fromPlain(plain: GroupPlain): Group { return new Group(plain); }
  static createId(id: string): GroupId { return { id }; }
  static areSameId(a: GroupId, b: GroupId): boolean { return a.id === b.id; }
  toPlain(): GroupPlain { return GroupSchema.parse(this as unknown as GroupPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Group { return new Group(GroupSchema.parse(JSON.parse(json))); }
}
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
export const GroupsDiffSchema = z.object({ removed: z.array(GroupIdSchema).optional(), updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type GroupsDiff = z.infer<typeof GroupsDiffSchema>;
export const GroupMetadataDtoSchema = GroupSchema.omit({ pieces: true, attributes: true });
export type GroupMetadataDto = z.infer<typeof GroupMetadataDtoSchema>;
export const GroupShallowSchema = GroupSchema;
export type GroupShallow = z.infer<typeof GroupShallowSchema>;
// #endregion Group

// #region Side
export const SideSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SidePlain = z.infer<typeof SideSchema>;
export class Side {
  #pieceId!: string;
  #designPieceId?: string;
  #connectorId?: string;
  constructor(plain: SidePlain) { const p = SideSchema.parse(plain); this.#pieceId = p.piece.id; this.#designPieceId = p.designPiece?.id; this.#connectorId = p.connector?.id; }
  get piece(): PieceId { return { id: this.#pieceId }; }
  get designPiece(): PieceId | undefined { if (!this.#designPieceId) return undefined; return { id: this.#designPieceId }; }
  get connector(): ConnectorId | undefined { return this.#connectorId !== undefined ? { id: this.#connectorId } : undefined; }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Side { return new Side(SideSchema.parse(JSON.parse(json))); }
  static from(plain: SidePlain): Side { return new Side(plain); }
  static fromPlain(plain: SidePlain): Side { return new Side(plain); }
  toPlain(): SidePlain { return SideSchema.parse({ piece: { id: this.#pieceId }, designPiece: this.#designPieceId ? { id: this.#designPieceId } : undefined, connector: this.#connectorId ? { id: this.#connectorId } : undefined }); }
}
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = z.infer<typeof SideDiffSchema>;
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SideIdPlain = z.infer<typeof SideIdSchema>;
export class SideId implements SideIdPlain {
  piece!: PieceId; designPiece?: PieceId; connector?: ConnectorId;
  constructor(plain: SideIdPlain) { Object.assign(this, SideIdSchema.parse(plain)); }
  static from(plain: SideIdPlain): SideId { return new SideId(plain); }
  toPlain(): SideIdPlain { return SideIdSchema.parse(this as unknown as SideIdPlain); }
}
export const SidesDiffSchema = z.object({ removed: z.array(SideIdSchema).optional(), updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
// #endregion Side

// #region Connection
export const ConnectionSchema = z.object({ id: z.string(), connected: SideSchema, connecting: SideSchema, gap: z.number().optional(), shift: z.number().optional(), rise: z.number().optional(), rotation: z.number().optional(), turn: z.number().optional(), tilt: z.number().optional(), u: z.number().optional(), v: z.number().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type ConnectionPlain = z.infer<typeof ConnectionSchema>;
export class Connection implements ConnectionPlain {
  id!: string; connected!: Side; connecting!: Side; gap?: number; shift?: number; rise?: number; rotation?: number; turn?: number; tilt?: number; u?: number; v?: number; description?: string; attributes?: Attribute[];
  constructor(plain: ConnectionPlain) { const p = ConnectionSchema.parse(plain); Object.assign(this, p); this.connected = new Side(p.connected); this.connecting = new Side(p.connecting); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Connection { return new Connection(ConnectionSchema.parse(JSON.parse(json))); }
  static from(plain: ConnectionPlain): Connection { return new Connection(plain); }
  static fromPlain(plain: ConnectionPlain): Connection { return new Connection(plain); }
  static createId(id: string): ConnectionId { return { id }; }
  static areSameId(a: ConnectionId, b: ConnectionId): boolean { return a.id === b.id; }
  toPlain(): ConnectionPlain { return ConnectionSchema.parse({ id: this.id, connected: this.connected.toPlain(), connecting: this.connecting.toPlain(), gap: this.gap, shift: this.shift, rise: this.rise, rotation: this.rotation, turn: this.turn, tilt: this.tilt, u: this.u, v: this.v, description: this.description, attributes: this.attributes?.map((a) => a.toPlain()) } as unknown as ConnectionPlain); }
}
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ id: true, connected: true, connecting: true, attributes: true }).extend({ connected: SideDiffSchema.optional(), connecting: SideDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
export const ConnectionsDiffSchema = z.object({ removed: z.array(ConnectionIdSchema).optional(), updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export const ConnectionMetadataDtoSchema = ConnectionSchema.omit({ attributes: true });
export type ConnectionMetadataDto = z.infer<typeof ConnectionMetadataDtoSchema>;
export const ConnectionShallowSchema = ConnectionSchema;
export type ConnectionShallow = z.infer<typeof ConnectionShallowSchema>;
// #endregion Connection

// #region Stat
export const StatSchema = z.object({ id: z.string(), quality: QualityIdSchema, unit: z.string().optional(), min: z.number().optional(), minExcluded: z.boolean().optional(), max: z.number().optional(), maxExcluded: z.boolean().optional() });
export type StatPlain = z.infer<typeof StatSchema>;
export class Stat implements StatPlain {
  id!: string; quality!: QualityId; unit?: string; min?: number; minExcluded?: boolean; max?: number; maxExcluded?: boolean;
  constructor(plain: StatPlain) { Object.assign(this, StatSchema.parse(plain)); }
  static from(plain: StatPlain): Stat { return new Stat(plain); }
  static fromPlain(plain: StatPlain): Stat { return new Stat(plain); }
  static createId(id: string): StatId { return { id }; }
  static areSameId(a: StatId, b: StatId): boolean { return a.id === b.id; }
  toPlain(): StatPlain { return StatSchema.parse(this as unknown as StatPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Stat { return new Stat(StatSchema.parse(JSON.parse(json))); }
}
export const StatDiffSchema = StatSchema.partial();
export type StatDiff = z.infer<typeof StatDiffSchema>;
export const StatsDiffSchema = z.object({ removed: z.array(StatIdSchema).optional(), updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type StatsDiff = z.infer<typeof StatsDiffSchema>;
export const StatMetadataDtoSchema = StatSchema;
export type StatMetadataDto = z.infer<typeof StatMetadataDtoSchema>;
export const StatShallowSchema = StatSchema;
export type StatShallow = z.infer<typeof StatShallowSchema>;
// #endregion Stat

// #region Design
export const DesignSchema = z.object({ id: z.string(), name: z.string(), families: z.array(FamilyIdSchema).optional(), isAbstract: z.boolean().optional(), folder: z.string().optional(), pieces: z.array(PieceSchema).optional(), connections: z.array(ConnectionSchema).optional(), stats: z.array(StatSchema).optional(), props: z.array(PropSchema).optional(), layers: z.array(LayerSchema).optional(), activeLayer: LayerIdSchema.optional(), groups: z.array(GroupSchema).optional(), canScale: z.boolean().optional(), canMirror: z.boolean().optional(), unit: z.string().optional(), location: LocationIdSchema.optional(), authors: z.array(AuthorIdSchema).optional(), concepts: z.array(ConceptIdSchema).optional(), icon: z.string().optional(), image: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional(), createdAt: DateProperty(), updatedAt: DateProperty() });
export type DesignPlain = z.infer<typeof DesignSchema>;
export class Design {
  id!: string; name!: string; families?: FamilyId[]; isAbstract?: boolean; folder?: string; pieces?: Piece[]; _connections?: Connection[]; stats?: Stat[]; props?: Prop[]; layers?: Layer[]; activeLayer?: LayerId; groups?: Group[]; canScale?: boolean; canMirror?: boolean; unit?: string; location?: LocationId; authors?: AuthorId[]; concepts?: ConceptId[]; icon?: string; image?: string; description?: string; attributes?: Attribute[]; createdAt!: string; updatedAt!: string;
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
  static fromPlain(plain: DesignPlain): Design { return new Design(plain); }
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
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Design { return new Design(DesignSchema.parse(JSON.parse(json))); }
  static createId(id: string): DesignId { return { id }; }
  static areSameId(a: DesignId, b: DesignId): boolean { return a.id === b.id; }
}
export const DesignMetadataDtoSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true, authors: true, concepts: true });
export type DesignMetadataDto = z.infer<typeof DesignMetadataDtoSchema>;
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true }).extend({ pieces: z.array(PieceMetadataDtoSchema).optional(), connections: z.array(ConnectionMetadataDtoSchema).optional(), stats: z.array(StatMetadataDtoSchema).optional(), props: z.array(PropMetadataDtoSchema).optional(), layers: z.array(LayerMetadataDtoSchema).optional(), groups: z.array(GroupMetadataDtoSchema).optional(), attributes: z.array(AttributeMetadataDtoSchema).optional() });
export type DesignShallow = z.infer<typeof DesignShallowSchema>;
export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).partial().extend({ pieces: PiecesDiffSchema.optional(), connections: ConnectionsDiffSchema.optional(), stats: StatsDiffSchema.optional(), props: PropsDiffSchema.optional(), layers: LayersDiffSchema.optional(), groups: GroupsDiffSchema.optional(), authors: AuthorsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
export const DesignsDiffSchema = z.object({ removed: z.array(DesignIdSchema).optional(), updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;
// Removed: addPieceToDesignDiff, setPieceInDesignDiff, removePieceFromDesignDiff, addPiecesToDesignDiff, setPiecesInDesignDiff, removePiecesFromDesignDiff, addConnectionToDesignDiff, setConnectionInDesignDiff, removeConnectionFromDesignDiff, addConnectionsToDesignDiff, setConnectionsInDesignDiff, removeConnectionsFromDesignDiff, mergeDesigns, orientDesign, duplicateDesignDiffForIsolation — design-diff builder functions moved to semio/rs (Requirement 3.7)
// #endregion Design

// #region Kit
export const KitKindSchema = z.enum(["dev", "local", "archive", "remote", "transport"]);
export type KitKind = z.infer<typeof KitKindSchema>;
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;

export const KitFullDtoSchema = z.object({ id: z.string(), name: z.string(), version: z.string().optional(), types: z.array(TypeSchema).optional(), designs: z.array(DesignSchema).optional(), tags: z.array(TagSchema).optional(), concepts: z.array(ConceptSchema).optional(), families: z.array(FamilySchema).optional(), qualities: z.array(QualitySchema).optional(), files: z.array(FileSchema).optional(), folders: z.array(FolderSchema).optional(), authors: z.array(AuthorSchema).optional(), remote: z.string().optional(), homepage: z.string().optional(), license: z.string().optional(), preview: z.string().optional(), icon: z.string().optional(), image: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional(), createdAt: DateProperty(), updatedAt: DateProperty() });
export type KitFullDto = z.infer<typeof KitFullDtoSchema>;

export class Kit {
  id!: string; name!: string; version?: string; types?: Type[]; designs?: Design[]; tags?: Tag[]; concepts?: Concept[]; families?: Family[]; qualities?: Quality[]; files?: File[]; folders?: Folder[]; authors?: Author[]; remote?: string; homepage?: string; license?: string; preview?: string; icon?: string; image?: string; description?: string; attributes?: Attribute[]; createdAt!: string; updatedAt!: string;
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
  static fromPlain(data: KitFullDto): Kit { return new Kit(data); }
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
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Kit { return Kit.fromPlain(KitFullDtoSchema.parse(JSON.parse(json))); }
  toJSON(): KitFullDto { return this.toPlain(); }
  static createId(id: string): KitId { return { id }; }
  static areSameId(a: KitId, b: KitId): boolean { return a.id === b.id; }
}
export type KitLike = Kit | KitFullDto;
export const KitDiffSchema = z.object({ types: TypesDiffSchema.optional(), designs: DesignsDiffSchema.optional() }).passthrough();
export type KitDiff = z.infer<typeof KitDiffSchema>;
// #endregion Kit

// #region KitStorePipeline
// Worker-hosted WASM kit store client, structured SetResult / WriteStatus types, and JSON fallback for Node/tests.

export type SetErrorKind = "IllegalName" | "NameTooLong" | "InvalidUrl" | "InvalidValue" | "DuplicateId" | "NotFound" | "CyclicReference" | "PortFamilyMismatch" | "Readonly" | "Disposed" | "Timeout" | "LockPoisoned" | "Internal" | "NotSupported";
export type SetError = { kind: SetErrorKind; message: string; field?: string; entity?: { kind: string; id: string } };
export type KitCommandRequestId = string;
export type SetResult = ({ ok: true; requestId?: KitCommandRequestId } | { ok: false; error: SetError; requestId?: KitCommandRequestId });
export type KitCommandLifecyclePhase = "accepted" | "succeeded" | "failed";
export type KitCommandLifecycleEvent = { semioKitCommand: { requestId: KitCommandRequestId; commandKind: string; phase: KitCommandLifecyclePhase; result?: unknown; error?: SetError } };

let nextKitCommandRequestSerial = 0;
export function createKitCommandRequestId(): KitCommandRequestId {
  nextKitCommandRequestSerial += 1;
  return `kit-command-${Date.now().toString(36)}-${nextKitCommandRequestSerial.toString(36)}`;
}

export function isKitCommandLifecycleEvent(event: unknown): event is KitCommandLifecycleEvent {
  return normalizeKitCommandLifecycleEvent(event) != null;
}

export function normalizeKitCommandLifecycleEvent(event: unknown): KitCommandLifecycleEvent | undefined {
  const command = (event as { semioKitCommand?: unknown } | null)?.semioKitCommand ?? (event as { SemioKitCommand?: unknown } | null)?.SemioKitCommand;
  if (command == null || typeof command !== "object") return undefined;
  const value = command as Record<string, unknown>;
  if (typeof value.requestId !== "string" || typeof value.commandKind !== "string" || typeof value.phase !== "string") return undefined;
  const error = value.error && typeof value.error === "object" ? normalizeRustSetError(value.error) : undefined;
  return { semioKitCommand: { requestId: value.requestId, commandKind: value.commandKind, phase: value.phase as KitCommandLifecyclePhase, result: value.result, error } };
}

export type KitStoreExecuteResult = { ok: true; result: unknown } | { ok: false; error: SetError };
export type BackboneConfig = { dev: { path: string } } | { local: { folder: string } } | { remote: { url: string; sessionId: string } };
export type ConflictResolution = { dropWip: null } | { forceOverwriteBackbone: null };
export type BackboneStatusDto = { attached: boolean; kind?: string | null; tip?: string | null };
export type KitConflict = { id: string; wipCheckpoint: unknown; backboneTip?: string | null; reason: string; createdAt: string };

function parseKitStoreBackboneStatusResult(raw: unknown): BackboneStatusDto {
  if (raw == null || typeof raw !== "object") throw new Error("backboneStatus: unexpected result");
  const o = raw as Record<string, unknown>;
  const inner = o.backboneStatus as Record<string, unknown> | undefined;
  if (!inner || typeof inner !== "object") throw new Error("backboneStatus: missing backboneStatus field");
  return { attached: Boolean(inner.attached), kind: inner.kind != null ? String(inner.kind) : null, tip: inner.tip != null && inner.tip !== "" ? String(inner.tip) : null };
}

function parseKitStoreListConflictsResult(raw: unknown): KitConflict[] {
  if (raw == null || typeof raw !== "object") throw new Error("listConflicts: unexpected result");
  const o = raw as Record<string, unknown>;
  const inner = o.listConflicts as { items?: unknown[] } | undefined;
  if (!inner || !Array.isArray(inner.items)) throw new Error("listConflicts: missing listConflicts.items");
  return inner.items.map((row) => {
    if (row == null || typeof row !== "object") throw new Error("listConflicts: invalid row");
    const r = row as Record<string, unknown>;
    return { id: String(r.id ?? ""), wipCheckpoint: r.wipCheckpoint, backboneTip: r.backboneTip != null ? String(r.backboneTip) : null, reason: String(r.reason ?? ""), createdAt: String(r.createdAt ?? "") };
  });
}

export type WriteStatus = { kind: "idle"; pending: 0; lastError?: undefined } | { kind: "pending"; pending: number; lastError?: SetError } | { kind: "error"; pending: 0; lastError: SetError } | { kind: "readonly"; pending: 0 };
export type HookTriad<T> = readonly [T, (next: T | ((prev: T) => T)) => Promise<SetResult>, WriteStatus];

export function normalizeRustSetError(raw: any): SetError {
  if (!raw || typeof raw !== "object") return { kind: "Internal", message: "invalid error payload" };
  const kind = typeof raw.kind === "string" ? (raw.kind as SetErrorKind) : "Internal";
  const message = typeof raw.message === "string" ? raw.message : JSON.stringify(raw);
  return { kind, message };
}

export function normalizeWasmThrownKitError(err: unknown): SetError {
  const message = String(err).replace(/^Error:\s*/, "").trim();
  const lower = message.toLowerCase();
  if (lower.includes("illegal name") || lower.includes("cannot be empty")) return { kind: "IllegalName", message };
  if (lower.includes("name too long") || (lower.includes("exceeds") && lower.includes("char"))) return { kind: "NameTooLong", message };
  return { kind: "Internal", message };
}

export function settleSetPromise(p: Promise<unknown>): Promise<SetResult> {
  return p.then((v: any) => {
    if (v && typeof v === "object" && v.ok === true) return { ok: true } as const;
    if (v && typeof v === "object" && v.ok === false && v.error) return { ok: false, error: normalizeRustSetError(v.error) } as const;
    return { ok: false, error: { kind: "Internal", message: "unexpected setField result" } } as const;
  });
}

/** Boundary contract consumed by @semio/react and sketchpad. */
export interface KitStoreClient {
  getDto(): any;
  getSnapshot(): Promise<any>;
  setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult>;
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult>;
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult>;
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
  executeRead(commands: ReadCommandBatch): Promise<ReadCommandBatchResult>;
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
  wasmSpecifier?: string;
  timeoutMs?: number;
  forceFallback?: boolean;
  workerFactory?: () => Worker;
};

function withTimeout<T>(p: Promise<T>, ms: number, label: string): Promise<T> {
  if (!ms || ms <= 0) return p;
  return new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(label)), ms);
    p.then((v) => { clearTimeout(t); resolve(v); }, (e) => { clearTimeout(t); reject(e); });
  });
}

/** In-process mirror of KitStoreHandle for Node/tests. */
export class FallbackKitStoreClient implements KitStoreClient {
  private handle: any;
  private listeners: Set<(ev: any) => void> = new Set();
  private cached: any;
  private timeoutMs: number;
  private subscribed = false;
  private gqlUnsub: (() => void) | undefined;

  constructor(handle: any, cachedDto: any, timeoutMs: number) { this.handle = handle; this.cached = cachedDto; this.timeoutMs = timeoutMs; }

  private gql(): KitGraphqlHandle { return { execute: (requestJson: string, onMessage: (line: string) => void) => this.handle.execute(requestJson, onMessage) }; }
  kitGraphql(): KitGraphqlHandle { return this.gql(); }
  getDto() { return this.cached; }

  async getSnapshot() {
    try { this.cached = await withTimeout(Promise.resolve(this.handle.snapshot()), this.timeoutMs, "snapshot timeout"); } catch { /* keep cached */ }
    return this.cached;
  }

  subscribe(cb: (ev: any) => void): () => void {
    this.listeners.add(cb);
    if (!this.subscribed) {
      this.subscribed = true;
      this.gqlUnsub = kitGraphqlSubscribeLoop(this.gql(), (ev: any) => { for (const listener of this.listeners) { try { listener(ev); } catch { /* ignore */ } } });
    }
    return () => { this.listeners.delete(cb); if (this.listeners.size === 0) { this.gqlUnsub?.(); this.gqlUnsub = undefined; this.subscribed = false; } };
  }

  dispose() { this.listeners.clear(); if (typeof this.handle?.free === "function") { try { this.handle.free(); } catch { /* ignore */ } } }

  private async submitSetResult(commandKind: string, request: { query: string; variables?: Record<string, unknown>; operationName?: string }): Promise<SetResult> {
    try { const receipt = await withTimeout(kitGraphqlSubmitCommandShell(this.gql(), commandKind, request), this.timeoutMs, "timeout"); return { ok: true, requestId: receipt.requestId }; }
    catch { const error: SetError = { kind: "Timeout", message: "timeout" }; return { ok: false, error }; }
  }

  async setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult> {
    try { const cmds = this.handle.changeKitCommandsForFieldPatch(kind, id, field, value); return this.submitSetResult("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands: cmds } }); }
    catch (e) { return { ok: false, error: normalizeWasmThrownKitError(e) }; }
  }
  async addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult> {
    try { const cmds = this.handle.changeKitCommandsForAddChild(parentKind, parentId, childKind, dto); return this.submitSetResult("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands: cmds } }); }
    catch (e) { return { ok: false, error: normalizeWasmThrownKitError(e) }; }
  }
  async removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult> {
    try { const cmds = this.handle.changeKitCommandsForRemoveChild(parentKind, parentId, childKind, childId); return this.submitSetResult("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands: cmds } }); }
    catch (e) { return { ok: false, error: normalizeWasmThrownKitError(e) }; }
  }
  async clusterPieces(designId: string, pieceIds: string[], clusterName: string): Promise<SetResult> { return this.submitSetResult("clusterPieces", { query: `mutation($designId: String!, $pieceIds: [String!]!, $clusterName: String!) { clusterPieces(designId: $designId, pieceIds: $pieceIds, clusterName: $clusterName) }`, variables: { designId, pieceIds, clusterName } }); }
  async dragPieces(designId: string, pieceIds: string[], du: number, dv: number): Promise<SetResult> { return this.submitSetResult("dragPieces", { query: `mutation($designId: String!, $pieceIds: [String!]!, $du: Float!, $dv: Float!) { dragPieces(designId: $designId, pieceIds: $pieceIds, du: $du, dv: $dv) }`, variables: { designId, pieceIds, du, dv } }); }
  async movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number): Promise<SetResult> { return this.submitSetResult("movePieces", { query: `mutation($designId: String!, $pieceIds: [String!]!, $gap: Float!, $shift: Float!, $rise: Float!) { movePieces(designId: $designId, pieceIds: $pieceIds, gap: $gap, shift: $shift, rise: $rise) }`, variables: { designId, pieceIds, gap, shift, rise } }); }
  async fixPieces(designId: string, pieceIds: string[]): Promise<SetResult> { return this.submitSetResult("fixPieces", { query: `mutation($designId: String!, $pieceIds: [String!]!) { fixPieces(designId: $designId, pieceIds: $pieceIds) }`, variables: { designId, pieceIds } }); }
  async flattenDesign(designId: string): Promise<SetResult> { return this.submitSetResult("flattenDesign", { query: `mutation($designId: String!) { flattenDesign(designId: $designId) }`, variables: { designId } }); }
  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> { return this.submitSetResult("expandDesign", { query: `mutation($parentDesignId: String!, $nestedDesignId: String!) { expandDesign(parentDesignId: $parentDesignId, nestedDesignId: $nestedDesignId) }`, variables: { parentDesignId, nestedDesignId } }); }
  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> { return this.submitSetResult("deleteConnection", { query: `mutation($designId: String!, $connectionId: String!) { deleteConnection(designId: $designId, connectionId: $connectionId) }`, variables: { designId, connectionId } }); }
  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> { return this.submitSetResult("changePieceType", { query: `mutation($designId: String!, $pieceId: String!, $newTypeId: String!) { changePieceType(designId: $designId, pieceId: $pieceId, newTypeId: $newTypeId) }`, variables: { designId, pieceId, newTypeId } }); }
  async pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> { return this.submitSetResult("pasteDesignSelection", { query: `mutation($designId: String!, $selection: JSON!, $plane: JSON) { pasteDesignSelection(designId: $designId, selection: $selection, plane: $plane) }`, variables: { designId, selection, plane } }); }
  async createHangingPieces(designId: string, typeIds: string[], plane: unknown): Promise<SetResult> { return this.submitSetResult("createHangingPieces", { query: `mutation($designId: String!, $typeIds: [String!]!, $plane: JSON!) { createHangingPieces(designId: $designId, typeIds: $typeIds, plane: $plane) }`, variables: { designId, typeIds, plane } }); }
  async createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> { return this.submitSetResult("createConnectedPiece", { query: `mutation($designId: String!, $parentPiece: String!, $parentPort: String!, $childType: String!, $childPort: String!) { createConnectedPiece(designId: $designId, parentPiece: $parentPiece, parentPort: $parentPort, childType: $childType, childPort: $childPort) }`, variables: { designId, parentPiece, parentPort, childType, childPort } }); }
  async createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> { return this.submitSetResult("createFixedPiece", { query: `mutation($designId: String!, $typeId: String!, $plane: JSON!) { createFixedPiece(designId: $designId, typeId: $typeId, plane: $plane) }`, variables: { designId, typeId, plane } }); }
  async undo(): Promise<SetResult> { return this.submitSetResult("undo", { query: `mutation { undo }` }); }
  async redo(): Promise<SetResult> { return this.submitSetResult("redo", { query: `mutation { redo }` }); }
  async canUndo(): Promise<boolean> { try { return Boolean(await withTimeout(Promise.resolve(this.handle.canUndo()), this.timeoutMs, "timeout")); } catch { return false; } }
  async canRedo(): Promise<boolean> { try { return Boolean(await withTimeout(Promise.resolve(this.handle.canRedo()), this.timeoutMs, "timeout")); } catch { return false; } }

  private unwrapQuery(raw: any) { if (raw && typeof raw === "object" && raw.ok === false && raw.error) throw new Error(typeof raw.error?.message === "string" ? raw.error.message : JSON.stringify(raw.error)); return raw; }

  async getPiecesMetadata(designId: string) { return this.unwrapQuery(await withTimeout(kitGraphqlKitDesignPiecesMetadata(this.gql(), designId), this.timeoutMs, "timeout")); }
  async getPieces(designId: string) { return this.unwrapQuery(await withTimeout((async () => { const out = await readKitDesign(this, designId, { readDesignPiecesFullCommand: null }); if (!("readDesignPiecesFullCommand" in out) || out.readDesignPiecesFullCommand == null) throw new Error("readDesignPiecesFullCommand: missing output"); return out.readDesignPiecesFullCommand.pieces; })(), this.timeoutMs, "timeout")); }
  async getConnections(designId: string) { return this.unwrapQuery(await withTimeout((async () => { const out = await readKitDesign(this, designId, { readDesignConnectionsFullCommand: null }); if (!("readDesignConnectionsFullCommand" in out) || out.readDesignConnectionsFullCommand == null) throw new Error("readDesignConnectionsFullCommand: missing output"); return out.readDesignConnectionsFullCommand.connections; })(), this.timeoutMs, "timeout")); }
  async getDesigns() { return this.unwrapQuery(await withTimeout((async () => { const out = await readKit(this, { readKitDesignsShallowCommand: null }); if (!("readKitDesignsShallowCommand" in out) || out.readKitDesignsShallowCommand == null) throw new Error("readKitDesignsShallowCommand: missing output"); return out.readKitDesignsShallowCommand.designs; })(), this.timeoutMs, "timeout")); }
  async getTypes() { return this.unwrapQuery(await withTimeout((async () => { const out = await readKit(this, { readKitTypesShallowCommand: null }); if (!("readKitTypesShallowCommand" in out) || out.readKitTypesShallowCommand == null) throw new Error("readKitTypesShallowCommand: missing output"); return out.readKitTypesShallowCommand.types; })(), this.timeoutMs, "timeout")); }
  async getAuthors() { return this.unwrapQuery(await withTimeout((async () => { const out = await readKit(this, { readKitAuthorsShallowCommand: null }); if (!("readKitAuthorsShallowCommand" in out) || out.readKitAuthorsShallowCommand == null) throw new Error("readKitAuthorsShallowCommand: missing output"); return out.readKitAuthorsShallowCommand.authors; })(), this.timeoutMs, "timeout")); }
  async getKitMetadata() { return this.unwrapQuery(await withTimeout((async () => { const out = await readKit(this, { readKitMetadataCommand: null }); if (!("readKitMetadataCommand" in out) || out.readKitMetadataCommand == null) throw new Error("readKitMetadataCommand: missing output"); return out.readKitMetadataCommand.metadata; })(), this.timeoutMs, "timeout")); }
  async executeRead(cmds: ReadCommandBatch): Promise<ReadCommandBatchResult> { return await withTimeout(kitGraphqlExecuteRead(this.gql(), cmds), this.timeoutMs, "timeout"); }
  async execute(cmd: unknown): Promise<KitStoreExecuteResult> { try { const result = await withTimeout(kitGraphqlExecuteStoreCommand(this.gql(), cmd), this.timeoutMs, "timeout"); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  async vcsState(): Promise<any> { return await withTimeout(Promise.resolve(this.handle.vcsState()), this.timeoutMs, "timeout"); }
  async theKitDto(): Promise<any> { return await withTimeout(Promise.resolve(this.handle.theKitDto()), this.timeoutMs, "timeout"); }
  async materializeAt(id: string): Promise<any> { const at = id.trim() === "" ? undefined : id; return await withTimeout(Promise.resolve(this.handle.materializeAt(at)), this.timeoutMs, "timeout"); }
  async attachBackbone(cfg: BackboneConfig): Promise<SetResult> { const r = await this.execute({ attachBackbone: { config: cfg } }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.attachBackbone as { ok?: boolean } | undefined; if (inner?.ok === true) { await this.getSnapshot(); return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "attachBackbone: unexpected result" } }; }
  async detachBackbone(): Promise<SetResult> { const r = await this.execute({ detachBackbone: null }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.detachBackbone as { ok?: boolean } | undefined; if (inner?.ok === true) { await this.getSnapshot(); return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "detachBackbone: unexpected result" } }; }
  async backboneStatus(): Promise<BackboneStatusDto> { const r = await this.execute({ backboneStatus: null }); if (!r.ok) throw new Error(r.error.message); return parseKitStoreBackboneStatusResult(r.result); }
  async listConflicts(): Promise<KitConflict[]> { const r = await this.execute({ listConflicts: null }); if (!r.ok) throw new Error(r.error.message); return parseKitStoreListConflictsResult(r.result); }
  async resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> { const r = await this.execute({ resolveConflict: { id, strategy } }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.resolveConflict as { ok?: boolean } | undefined; if (inner?.ok === true) { await this.getSnapshot(); return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "resolveConflict: unexpected result" } }; }
  async syncNow(): Promise<SetResult> { const r = await this.execute({ syncNow: null }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.syncNow as { ok?: boolean } | undefined; if (inner?.ok === true) { await this.getSnapshot(); return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "syncNow: unexpected result" } }; }
}

/** Comlink-backed client; falls back if worker fails to boot. */
export class WorkerKitStoreClient implements KitStoreClient {
  private worker: Worker;
  private api: any;
  private listeners: Set<(ev: any) => void> = new Set();
  private cached: any;
  private timeoutMs: number;
  private workerGqlSubStarted = false;

  constructor(worker: Worker, api: any, cachedDto: any, timeoutMs: number) { this.worker = worker; this.api = api; this.cached = cachedDto; this.timeoutMs = timeoutMs; }

  kitGraphql(): KitGraphqlHandle {
    return { execute: async (requestJson: string, onMessage: (line: string) => void) => { const Comlink = await import("comlink"); await this.api.graphqlExecute(requestJson, Comlink.proxy(onMessage)); } };
  }
  getDto() { return this.cached; }
  async getSnapshot() { try { this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "snapshot timeout"); } catch { /* keep cached */ } return this.cached; }
  subscribe(cb: (ev: any) => void): () => void {
    this.listeners.add(cb);
    if (!this.workerGqlSubStarted) {
      this.workerGqlSubStarted = true;
      void import("comlink").then((Comlink) => { void this.api.subscribe(Comlink.proxy((ev: any) => { for (const l of this.listeners) { try { l(ev); } catch { /* ignore */ } } })); });
    }
    return () => { this.listeners.delete(cb); if (this.listeners.size === 0) this.workerGqlSubStarted = false; };
  }
  dispose() { this.listeners.clear(); this.worker.terminate(); }

  private async wrapMutation(fn: () => Promise<any>): Promise<SetResult> {
    try { const raw = await withTimeout(fn(), this.timeoutMs, "timeout"); const r = await settleSetPromise(Promise.resolve(raw)); if (r.ok) { try { this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout"); } catch { /* ignore */ } } return r; }
    catch { return { ok: false, error: { kind: "Timeout", message: "timeout" } }; }
  }

  async setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult> { return this.wrapMutation(() => this.api.setField(kind, id, field, value)); }
  async addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult> { return this.wrapMutation(() => this.api.addChild(parentKind, parentId, childKind, dto)); }
  async removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult> { return this.wrapMutation(() => this.api.removeChild(parentKind, parentId, childKind, childId)); }
  async clusterPieces(designId: string, pieceIds: string[], clusterName: string): Promise<SetResult> { return this.wrapMutation(() => this.api.clusterPieces(designId, pieceIds, clusterName)); }
  async dragPieces(designId: string, pieceIds: string[], du: number, dv: number): Promise<SetResult> { return this.wrapMutation(() => this.api.dragPieces(designId, pieceIds, du, dv)); }
  async movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number): Promise<SetResult> { return this.wrapMutation(() => this.api.movePieces(designId, pieceIds, gap, shift, rise)); }
  async fixPieces(designId: string, pieceIds: string[]): Promise<SetResult> { return this.wrapMutation(() => this.api.fixPieces(designId, pieceIds)); }
  async flattenDesign(designId: string): Promise<SetResult> { return this.wrapMutation(() => this.api.flattenDesign(designId)); }
  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> { return this.wrapMutation(() => this.api.expandDesign(parentDesignId, nestedDesignId)); }
  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> { return this.wrapMutation(() => this.api.deleteConnection(designId, connectionId)); }
  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> { return this.wrapMutation(() => this.api.changePieceType(designId, pieceId, newTypeId)); }
  async pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> { return this.wrapMutation(() => this.api.pasteDesignSelection(designId, selection, plane)); }
  async createHangingPieces(designId: string, typeIds: string[], plane: unknown): Promise<SetResult> { return this.wrapMutation(() => this.api.createHangingPieces(designId, typeIds, plane)); }
  async createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> { return this.wrapMutation(() => this.api.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort)); }
  async createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> { return this.wrapMutation(() => this.api.createFixedPiece(designId, typeId, plane)); }
  async undo(): Promise<SetResult> { return this.wrapMutation(() => this.api.undo()); }
  async redo(): Promise<SetResult> { return this.wrapMutation(() => this.api.redo()); }
  async canUndo(): Promise<boolean> { try { return Boolean(await withTimeout(this.api.canUndo(), this.timeoutMs, "timeout")); } catch { return false; } }
  async canRedo(): Promise<boolean> { try { return Boolean(await withTimeout(this.api.canRedo(), this.timeoutMs, "timeout")); } catch { return false; } }

  private asKitExecuteReadClient(): KitExecuteRead { return { executeRead: (batch) => kitGraphqlExecuteRead(this.kitGraphql(), batch) }; }

  async getPiecesMetadata(designId: string) { return await withTimeout(kitGraphqlKitDesignPiecesMetadata(this.kitGraphql(), designId), this.timeoutMs, "timeout"); }
  async getPieces(designId: string) { return await withTimeout((async () => { const out = await readKitDesign(this.asKitExecuteReadClient(), designId, { readDesignPiecesFullCommand: null }); if (!("readDesignPiecesFullCommand" in out) || out.readDesignPiecesFullCommand == null) throw new Error("missing"); return out.readDesignPiecesFullCommand.pieces; })(), this.timeoutMs, "timeout"); }
  async getConnections(designId: string) { return await withTimeout((async () => { const out = await readKitDesign(this.asKitExecuteReadClient(), designId, { readDesignConnectionsFullCommand: null }); if (!("readDesignConnectionsFullCommand" in out) || out.readDesignConnectionsFullCommand == null) throw new Error("missing"); return out.readDesignConnectionsFullCommand.connections; })(), this.timeoutMs, "timeout"); }
  async getDesigns() { return await withTimeout((async () => { const out = await readKit(this.asKitExecuteReadClient(), { readKitDesignsShallowCommand: null }); if (!("readKitDesignsShallowCommand" in out) || out.readKitDesignsShallowCommand == null) throw new Error("missing"); return out.readKitDesignsShallowCommand.designs; })(), this.timeoutMs, "timeout"); }
  async getTypes() { return await withTimeout((async () => { const out = await readKit(this.asKitExecuteReadClient(), { readKitTypesShallowCommand: null }); if (!("readKitTypesShallowCommand" in out) || out.readKitTypesShallowCommand == null) throw new Error("missing"); return out.readKitTypesShallowCommand.types; })(), this.timeoutMs, "timeout"); }
  async getAuthors() { return await withTimeout((async () => { const out = await readKit(this.asKitExecuteReadClient(), { readKitAuthorsShallowCommand: null }); if (!("readKitAuthorsShallowCommand" in out) || out.readKitAuthorsShallowCommand == null) throw new Error("missing"); return out.readKitAuthorsShallowCommand.authors; })(), this.timeoutMs, "timeout"); }
  async getKitMetadata() { return await withTimeout((async () => { const out = await readKit(this.asKitExecuteReadClient(), { readKitMetadataCommand: null }); if (!("readKitMetadataCommand" in out) || out.readKitMetadataCommand == null) throw new Error("missing"); return out.readKitMetadataCommand.metadata; })(), this.timeoutMs, "timeout"); }
  async executeRead(cmds: ReadCommandBatch): Promise<ReadCommandBatchResult> { return await withTimeout(this.api.executeRead(cmds) as Promise<ReadCommandBatchResult>, this.timeoutMs, "timeout"); }
  async execute(cmd: unknown): Promise<KitStoreExecuteResult> { try { const result = await withTimeout(kitGraphqlExecuteStoreCommand(this.kitGraphql(), cmd), this.timeoutMs, "timeout"); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  async vcsState(): Promise<any> { return await withTimeout(this.api.vcsState(), this.timeoutMs, "timeout"); }
  async theKitDto(): Promise<any> { return await withTimeout(this.api.theKitDto(), this.timeoutMs, "timeout"); }
  async materializeAt(id: string): Promise<any> { const at = id.trim() === "" ? undefined : id; return await withTimeout(this.api.materializeAt(at), this.timeoutMs, "timeout"); }
  async attachBackbone(cfg: BackboneConfig): Promise<SetResult> { const r = await this.execute({ attachBackbone: { config: cfg } }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.attachBackbone as { ok?: boolean } | undefined; if (inner?.ok === true) { try { this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout"); } catch { /* ignore */ } return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "attachBackbone: unexpected result" } }; }
  async detachBackbone(): Promise<SetResult> { const r = await this.execute({ detachBackbone: null }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.detachBackbone as { ok?: boolean } | undefined; if (inner?.ok === true) { try { this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout"); } catch { /* ignore */ } return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "detachBackbone: unexpected result" } }; }
  async backboneStatus(): Promise<BackboneStatusDto> { const r = await this.execute({ backboneStatus: null }); if (!r.ok) throw new Error(r.error.message); return parseKitStoreBackboneStatusResult(r.result); }
  async listConflicts(): Promise<KitConflict[]> { const r = await this.execute({ listConflicts: null }); if (!r.ok) throw new Error(r.error.message); return parseKitStoreListConflictsResult(r.result); }
  async resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> { const r = await this.execute({ resolveConflict: { id, strategy } }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.resolveConflict as { ok?: boolean } | undefined; if (inner?.ok === true) { try { this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout"); } catch { /* ignore */ } return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "resolveConflict: unexpected result" } }; }
  async syncNow(): Promise<SetResult> { const r = await this.execute({ syncNow: null }); if (!r.ok) return r; const o = r.result as Record<string, unknown>; const inner = o.syncNow as { ok?: boolean } | undefined; if (inner?.ok === true) { try { this.cached = await withTimeout(this.api.snapshot(), this.timeoutMs, "timeout"); } catch { /* ignore */ } return { ok: true } as const; } return { ok: false, error: { kind: "Internal", message: "syncNow: unexpected result" } }; }
}

const semioWasmInitBySpecifier = new Map<string, Promise<void>>();
async function ensureSemioWasmInitialized(wasmSpecifier: string, mod: any, tryNodeFsWasm: boolean): Promise<void> {
  let flight = semioWasmInitBySpecifier.get(wasmSpecifier);
  if (!flight) {
    flight = (async () => {
      if (typeof mod.default !== "function") return;
      if (tryNodeFsWasm) {
        try { const fs = await import("node:fs/promises"); const { fileURLToPath } = await import("node:url"); const wasmPath = fileURLToPath(new URL("../rs/pkg/semio_bg.wasm", import.meta.url)); const wasmBytes = await fs.readFile(wasmPath); await mod.default({ module_or_path: wasmBytes }); if (typeof mod.boot === "function") mod.boot(); return; }
        catch { /* fall through */ }
      }
      await mod.default();
      if (typeof mod.boot === "function") mod.boot();
    })();
    semioWasmInitBySpecifier.set(wasmSpecifier, flight);
  }
  await flight;
}

async function importWasmModule(specifier: string) {
  if (specifier === "@semio/rs-wasm") return import("@semio/rs-wasm");
  return import(/* @vite-ignore */ specifier);
}

export async function createKitStoreClient(opts: CreateKitStoreClientOptions): Promise<KitStoreClient> {
  const dto = JSON.parse(JSON.stringify(opts.initialKit)) as KitFullDto;
  const wasmSpecifier = opts.wasmSpecifier ?? (globalThis as any).__SEMIO_WASM_SPECIFIER__ ?? "@semio/rs-wasm";
  const timeoutMs = opts.timeoutMs ?? 60_000;
  const isNodeRuntime = (typeof process !== "undefined" && !!process.versions?.node) || (typeof navigator !== "undefined" && /jsdom/i.test(navigator.userAgent ?? ""));
  const useFallback = opts.forceFallback === true || typeof Worker === "undefined" || isNodeRuntime;
  if (useFallback) {
    const mod = await importWasmModule(wasmSpecifier);
    await ensureSemioWasmInitialized(wasmSpecifier, mod, isNodeRuntime);
    return new FallbackKitStoreClient(mod.KitStoreHandle.create(dto), dto, timeoutMs);
  }
  try {
    const Comlink = await import("comlink");
    const worker = opts.workerFactory?.() ?? new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
    const api = Comlink.wrap(worker);
    await api.init(wasmSpecifier, dto);
    return new WorkerKitStoreClient(worker, api, dto, timeoutMs);
  } catch {
    const mod = await importWasmModule(wasmSpecifier);
    await ensureSemioWasmInitialized(wasmSpecifier, mod, isNodeRuntime);
    return new FallbackKitStoreClient(mod.KitStoreHandle.create(dto), dto, timeoutMs);
  }
}

// #endregion KitStorePipeline

//#region ReadCommandTypes
export type IdDto = { readonly id: string };
export type DesignFlattenMapEntryDto = { readonly pieceId: string; readonly plane: unknown; readonly center: unknown };

export type ReadAttributeCommand = { readonly readAttributeFullCommand: null } | { readonly readAttributeShallowCommand: null } | { readonly readAttributeMetadataCommand: null } | { readonly readAttributeIdCommand: null } | { readonly readAttributeKeyCommand: null } | { readonly readAttributeValueCommand: null } | { readonly readAttributeDefinitionCommand: null };
export type ReadAuthorCommand = { readonly readAuthorFullCommand: null } | { readonly readAuthorShallowCommand: null } | { readonly readAuthorMetadataCommand: null } | { readonly readAuthorIdCommand: null } | { readonly readAuthorNameCommand: null } | { readonly readAuthorEmailCommand: null } | { readonly readAuthorRoleCommand: null } | { readonly readAuthorRankCommand: null };
export type ReadBenchmarkCommand = { readonly readBenchmarkFullCommand: null } | { readonly readBenchmarkShallowCommand: null } | { readonly readBenchmarkMetadataCommand: null } | { readonly readBenchmarkIdCommand: null } | { readonly readBenchmarkNameCommand: null } | { readonly readBenchmarkMinCommand: null } | { readonly readBenchmarkMaxCommand: null } | { readonly readBenchmarkMinExcludedCommand: null } | { readonly readBenchmarkMaxExcludedCommand: null };
export type ReadConceptCommand = { readonly readConceptFullCommand: null } | { readonly readConceptShallowCommand: null } | { readonly readConceptMetadataCommand: null } | { readonly readConceptIdCommand: null } | { readonly readConceptNameCommand: null } | { readonly readConceptDescriptionCommand: null } | { readonly readConceptOrderCommand: null };
export type ReadSideCommand = { readonly readSideFullCommand: null } | { readonly readSideShallowCommand: null } | { readonly readSideMetadataCommand: null } | { readonly readSideIdCommand: null } | { readonly readSidePieceIdCommand: null } | { readonly readSidePortIdCommand: null } | { readonly readSideDesignPieceIdCommand: null };
export type ReadConnectionCommand = { readonly readConnectionFullCommand: null } | { readonly readConnectionShallowCommand: null } | { readonly readConnectionMetadataCommand: null } | { readonly readConnectionIdCommand: null } | { readonly readConnectionConnectedSideMetadataCommand: null } | { readonly readConnectionConnectingSideMetadataCommand: null } | { readonly readConnectionConnectedSideFullCommand: null } | { readonly readConnectionConnectingSideFullCommand: null } | { readonly readConnectionGapCommand: null } | { readonly readConnectionShiftCommand: null } | { readonly readConnectionRiseCommand: null } | { readonly readConnectionRotationCommand: null } | { readonly readConnectionTurnCommand: null } | { readonly readConnectionTiltCommand: null } | { readonly readConnectionUCommand: null } | { readonly readConnectionVCommand: null } | { readonly readConnectionDescriptionCommand: null } | { readonly readConnectionAttributesFullCommand: null } | { readonly readConnectionAttributesShallowCommand: null } | { readonly readConnectionChildPlaneMatrixCommand: null } | { readonly readConnectionFlatSidesForChildCommand: { readonly childPieceId: IdDto } } | { readonly readConnectionAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } } | { readonly readConnectionConnectedSideCommands: { readonly commands: ReadonlyArray<ReadSideCommand> } } | { readonly readConnectionConnectingSideCommands: { readonly commands: ReadonlyArray<ReadSideCommand> } };
export type ReadConnectorCommand = { readonly readConnectorFullCommand: null } | { readonly readConnectorShallowCommand: null } | { readonly readConnectorMetadataCommand: null } | { readonly readConnectorIdCommand: null } | { readonly readConnectorCodeCommand: null } | { readonly readConnectorDescriptionCommand: null } | { readonly readConnectorPortIdCommand: null } | { readonly readConnectorQualitiesFullCommand: null } | { readonly readConnectorQualitiesShallowCommand: null } | { readonly readConnectorAttributesFullCommand: null } | { readonly readConnectorAttributesShallowCommand: null } | { readonly readConnectorQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } } | { readonly readConnectorAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadQualityCommand = { readonly readQualityFullCommand: null } | { readonly readQualityShallowCommand: null } | { readonly readQualityMetadataCommand: null } | { readonly readQualityIdCommand: null } | { readonly readQualityKeyCommand: null } | { readonly readQualityValueCommand: null } | { readonly readQualityUnitCommand: null } | { readonly readQualityDefinitionCommand: null } | { readonly readQualityDescriptionCommand: null } | { readonly readQualityBenchmarksFullCommand: null } | { readonly readQualityBenchmarksShallowCommand: null } | { readonly readQualityBenchmarkCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadBenchmarkCommand> } };
export type ReadPropCommand = { readonly readPropFullCommand: null } | { readonly readPropShallowCommand: null } | { readonly readPropIdCommand: null } | { readonly readPropKeyCommand: null } | { readonly readPropValueCommand: null } | { readonly readPropUnitCommand: null } | { readonly readPropQualityIdCommand: null };
export type ReadTagCommand = { readonly readTagFullCommand: null } | { readonly readTagShallowCommand: null } | { readonly readTagMetadataCommand: null } | { readonly readTagIdCommand: null } | { readonly readTagNameCommand: null } | { readonly readTagOrderCommand: null };
export type ReadStatCommand = { readonly readStatFullCommand: null } | { readonly readStatShallowCommand: null } | { readonly readStatMetadataCommand: null } | { readonly readStatIdCommand: null } | { readonly readStatKeyCommand: null } | { readonly readStatValueCommand: null } | { readonly readStatUnitCommand: null } | { readonly readStatDescriptionCommand: null };
export type ReadRepresentationCommand = { readonly readRepresentationFullCommand: null } | { readonly readRepresentationShallowCommand: null } | { readonly readRepresentationMetadataCommand: null } | { readonly readRepresentationIdCommand: null } | { readonly readRepresentationUrlCommand: null } | { readonly readRepresentationDescriptionCommand: null } | { readonly readRepresentationFileIdCommand: null } | { readonly readRepresentationTagsFullCommand: null } | { readonly readRepresentationTagsShallowCommand: null } | { readonly readRepresentationQualitiesFullCommand: null } | { readonly readRepresentationQualitiesShallowCommand: null } | { readonly readRepresentationAttributesFullCommand: null } | { readonly readRepresentationAttributesShallowCommand: null } | { readonly readRepresentationTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } } | { readonly readRepresentationQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } } | { readonly readRepresentationAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadFileCommand = { readonly readFileFullCommand: null } | { readonly readFileShallowCommand: null } | { readonly readFileMetadataCommand: null } | { readonly readFileIdCommand: null } | { readonly readFileUrlCommand: null } | { readonly readFileMimeCommand: null } | { readonly readFileSizeCommand: null } | { readonly readFileHashCommand: null } | { readonly readFileDescriptionCommand: null } | { readonly readFileCreatedCommand: null } | { readonly readFileUpdatedCommand: null };
export type ReadFolderCommand = { readonly readFolderFullCommand: null } | { readonly readFolderShallowCommand: null } | { readonly readFolderMetadataCommand: null } | { readonly readFolderIdCommand: null } | { readonly readFolderPathCommand: null } | { readonly readFolderDescriptionCommand: null };
export type ReadLocationCommand = { readonly readLocationFullCommand: null } | { readonly readLocationShallowCommand: null } | { readonly readLocationMetadataCommand: null } | { readonly readLocationIdCommand: null } | { readonly readLocationLongitudeCommand: null } | { readonly readLocationLatitudeCommand: null } | { readonly readLocationAltitudeCommand: null } | { readonly readLocationAttributesFullCommand: null } | { readonly readLocationAttributesShallowCommand: null } | { readonly readLocationAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadFamilyCommand = { readonly readFamilyFullCommand: null } | { readonly readFamilyShallowCommand: null } | { readonly readFamilyMetadataCommand: null } | { readonly readFamilyIdCommand: null } | { readonly readFamilyNameCommand: null } | { readonly readFamilyDescriptionCommand: null } | { readonly readFamilyIconCommand: null } | { readonly readFamilyPortsFullCommand: null } | { readonly readFamilyPortsShallowCommand: null } | { readonly readFamilyAttributesFullCommand: null } | { readonly readFamilyAttributesShallowCommand: null } | { readonly readFamilyPortCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPortCommand> } } | { readonly readFamilyAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadPortCommand = { readonly readPortFullCommand: null } | { readonly readPortShallowCommand: null } | { readonly readPortMetadataCommand: null } | { readonly readPortIdCommand: null } | { readonly readPortNameCommand: null } | { readonly readPortDescriptionCommand: null } | { readonly readPortIconCommand: null } | { readonly readPortCompatibleFamiliesCommand: null } | { readonly readPortMandatoryCommand: null } | { readonly readPortTCommand: null } | { readonly readPortPointCommand: null } | { readonly readPortDirectionCommand: null } | { readonly readPortCompatiblePortsCommand: null } | { readonly readPortQualitiesFullCommand: null } | { readonly readPortQualitiesShallowCommand: null } | { readonly readPortAttributesFullCommand: null } | { readonly readPortAttributesShallowCommand: null } | { readonly readPortQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } } | { readonly readPortAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadGroupCommand = { readonly readGroupFullCommand: null } | { readonly readGroupShallowCommand: null } | { readonly readGroupMetadataCommand: null } | { readonly readGroupIdCommand: null } | { readonly readGroupNameCommand: null } | { readonly readGroupDescriptionCommand: null } | { readonly readGroupColorCommand: null } | { readonly readGroupIconCommand: null } | { readonly readGroupPiecesCommand: null };
export type ReadLayerCommand = { readonly readLayerFullCommand: null } | { readonly readLayerShallowCommand: null } | { readonly readLayerMetadataCommand: null } | { readonly readLayerIdCommand: null } | { readonly readLayerNameCommand: null } | { readonly readLayerDescriptionCommand: null } | { readonly readLayerColorCommand: null } | { readonly readLayerOrderCommand: null } | { readonly readLayerVisibleCommand: null } | { readonly readLayerLockedCommand: null };
export type ReadPieceCommand = { readonly readPieceFullCommand: null } | { readonly readPieceShallowCommand: null } | { readonly readPieceMetadataCommand: null } | { readonly readPieceIdCommand: null } | { readonly readPieceNameCommand: null } | { readonly readPieceDescriptionCommand: null } | { readonly readPiecePlaneCommand: null } | { readonly readPieceCenterCommand: null } | { readonly readPieceScaleCommand: null } | { readonly readPieceMirrorPlaneCommand: null } | { readonly readPieceHiddenCommand: null } | { readonly readPieceLockedCommand: null } | { readonly readPieceColorCommand: null } | { readonly readPieceTypeCommand: null } | { readonly readPieceDesignCommand: null } | { readonly readPiecePropsFullCommand: null } | { readonly readPiecePropsShallowCommand: null } | { readonly readPieceAttributesFullCommand: null } | { readonly readPieceAttributesShallowCommand: null } | { readonly readPieceFlatPlaneCommand: null } | { readonly readPieceFlatCenterCommand: null } | { readonly readPieceFlatPoseCommand: null } | { readonly readPiecePathCommand: null } | { readonly readPieceParentPieceIdCommand: null } | { readonly readPieceParentConnectionIdCommand: null } | { readonly readPieceParentConnectionFullCommand: null } | { readonly readPieceParentDesignIdCommand: null } | { readonly readPieceFixedCommand: null } | { readonly readPieceConnectedCommand: null } | { readonly readPieceAlternativesCommand: null } | { readonly readPieceAlternativeTypesCommand: null } | { readonly readPieceAlternativeDesignsCommand: null } | { readonly readPiecePropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } } | { readonly readPieceAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadDesignCommand = { readonly readDesignFullCommand: null } | { readonly readDesignShallowCommand: null } | { readonly readDesignMetadataCommand: null } | { readonly readDesignIdCommand: null } | { readonly readDesignNameCommand: null } | { readonly readDesignDescriptionCommand: null } | { readonly readDesignIconCommand: null } | { readonly readDesignImageCommand: null } | { readonly readDesignLocationCommand: null } | { readonly readDesignUnitCommand: null } | { readonly readDesignCreatedCommand: null } | { readonly readDesignUpdatedCommand: null } | { readonly readDesignKitCommand: null } | { readonly readDesignFamiliesCommand: null } | { readonly readDesignPiecesFullCommand: null } | { readonly readDesignPiecesShallowCommand: null } | { readonly readDesignConnectionsFullCommand: null } | { readonly readDesignConnectionsShallowCommand: null } | { readonly readDesignLayersFullCommand: null } | { readonly readDesignLayersShallowCommand: null } | { readonly readDesignGroupsFullCommand: null } | { readonly readDesignGroupsShallowCommand: null } | { readonly readDesignAuthorsFullCommand: null } | { readonly readDesignAuthorsShallowCommand: null } | { readonly readDesignConceptsFullCommand: null } | { readonly readDesignConceptsShallowCommand: null } | { readonly readDesignTagsFullCommand: null } | { readonly readDesignTagsShallowCommand: null } | { readonly readDesignQualitiesFullCommand: null } | { readonly readDesignQualitiesShallowCommand: null } | { readonly readDesignPropsFullCommand: null } | { readonly readDesignPropsShallowCommand: null } | { readonly readDesignAttributesFullCommand: null } | { readonly readDesignAttributesShallowCommand: null } | { readonly readDesignStatsFullCommand: null } | { readonly readDesignStatsShallowCommand: null } | { readonly readDesignFlattenMapCommand: null } | { readonly readDesignClusterableGroupsCommand: { readonly selection: ReadonlyArray<IdDto> } } | { readonly readDesignIncludedDesignsCommand: null } | { readonly readDesignReplaceableCatalogCommand: { readonly selection: ReadonlyArray<IdDto> } } | { readonly readDesignIncludedDesignIdsCommand: null } | { readonly readDesignQualitySumCommand: { readonly qualityId: IdDto } } | { readonly readDesignFamilyCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFamilyCommand> } } | { readonly readDesignPieceCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPieceCommand> } } | { readonly readDesignConnectionCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConnectionCommand> } } | { readonly readDesignLayerCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadLayerCommand> } } | { readonly readDesignGroupCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadGroupCommand> } } | { readonly readDesignAuthorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAuthorCommand> } } | { readonly readDesignConceptCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConceptCommand> } } | { readonly readDesignTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } } | { readonly readDesignQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } } | { readonly readDesignPropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } } | { readonly readDesignAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } } | { readonly readDesignStatCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadStatCommand> } };
export type ReadTypeCommand = { readonly readTypeFullCommand: null } | { readonly readTypeShallowCommand: null } | { readonly readTypeMetadataCommand: null } | { readonly readTypeIdCommand: null } | { readonly readTypeNameCommand: null } | { readonly readTypeDescriptionCommand: null } | { readonly readTypeIconCommand: null } | { readonly readTypeImageCommand: null } | { readonly readTypeStockCommand: null } | { readonly readTypeVirtualCommand: null } | { readonly readTypeUnitCommand: null } | { readonly readTypeLocationCommand: null } | { readonly readTypeCreatedCommand: null } | { readonly readTypeUpdatedCommand: null } | { readonly readTypeFamiliesCommand: null } | { readonly readTypeConnectorsFullCommand: null } | { readonly readTypeConnectorsShallowCommand: null } | { readonly readTypeRepresentationsFullCommand: null } | { readonly readTypeRepresentationsShallowCommand: null } | { readonly readTypeAuthorsFullCommand: null } | { readonly readTypeAuthorsShallowCommand: null } | { readonly readTypeConceptsFullCommand: null } | { readonly readTypeConceptsShallowCommand: null } | { readonly readTypeTagsFullCommand: null } | { readonly readTypeTagsShallowCommand: null } | { readonly readTypeQualitiesFullCommand: null } | { readonly readTypeQualitiesShallowCommand: null } | { readonly readTypePropsFullCommand: null } | { readonly readTypePropsShallowCommand: null } | { readonly readTypeAttributesFullCommand: null } | { readonly readTypeAttributesShallowCommand: null } | { readonly readTypePortsFullCommand: null } | { readonly readTypeConnectorForPortIdCommand: { readonly portId: IdDto } } | { readonly readTypeBestRepresentationCommand: { readonly tagIds: ReadonlyArray<string> } } | { readonly readTypeFamilyCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFamilyCommand> } } | { readonly readTypeConnectorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConnectorCommand> } } | { readonly readTypeRepresentationCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadRepresentationCommand> } } | { readonly readTypePortCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPortCommand> } } | { readonly readTypeAuthorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAuthorCommand> } } | { readonly readTypeConceptCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConceptCommand> } } | { readonly readTypeTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } } | { readonly readTypeQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } } | { readonly readTypePropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } } | { readonly readTypeAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };
export type ReadKitCommand = { readonly readKitFullCommand: null } | { readonly readKitShallowCommand: null } | { readonly readKitMetadataCommand: null } | { readonly readKitIdCommand: null } | { readonly readKitNameCommand: null } | { readonly readKitDescriptionCommand: null } | { readonly readKitIconCommand: null } | { readonly readKitImageCommand: null } | { readonly readKitPreviewCommand: null } | { readonly readKitRemoteCommand: null } | { readonly readKitHomepageCommand: null } | { readonly readKitLicenseCommand: null } | { readonly readKitUriCommand: null } | { readonly readKitCreatedCommand: null } | { readonly readKitUpdatedCommand: null } | { readonly readKitTypesFullCommand: null } | { readonly readKitTypesShallowCommand: null } | { readonly readKitTypeIdsCommand: null } | { readonly readKitTypesMetadataCommand: null } | { readonly readKitDesignsFullCommand: null } | { readonly readKitDesignsShallowCommand: null } | { readonly readKitDesignIdsCommand: null } | { readonly readKitDesignsMetadataCommand: null } | { readonly readKitFilesFullCommand: null } | { readonly readKitFilesShallowCommand: null } | { readonly readKitFoldersFullCommand: null } | { readonly readKitFoldersShallowCommand: null } | { readonly readKitLocationsFullCommand: null } | { readonly readKitLocationsShallowCommand: null } | { readonly readKitFamiliesFullCommand: null } | { readonly readKitFamiliesShallowCommand: null } | { readonly readKitPortsFullCommand: null } | { readonly readKitAuthorsFullCommand: null } | { readonly readKitAuthorsShallowCommand: null } | { readonly readKitConceptsFullCommand: null } | { readonly readKitConceptsShallowCommand: null } | { readonly readKitTagsFullCommand: null } | { readonly readKitTagsShallowCommand: null } | { readonly readKitQualitiesFullCommand: null } | { readonly readKitQualitiesShallowCommand: null } | { readonly readKitPropsFullCommand: null } | { readonly readKitPropsShallowCommand: null } | { readonly readKitAttributesFullCommand: null } | { readonly readKitAttributesShallowCommand: null } | { readonly readKitColoredConnectorsCommand: null } | { readonly readKitTypeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTypeCommand> } } | { readonly readKitDesignCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadDesignCommand> } } | { readonly readKitFileCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFileCommand> } } | { readonly readKitFolderCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFolderCommand> } } | { readonly readKitLocationCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadLocationCommand> } } | { readonly readKitFamilyCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadFamilyCommand> } } | { readonly readKitPortCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPortCommand> } } | { readonly readKitAuthorCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAuthorCommand> } } | { readonly readKitConceptCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadConceptCommand> } } | { readonly readKitTagCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTagCommand> } } | { readonly readKitQualityCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadQualityCommand> } } | { readonly readKitPropCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPropCommand> } } | { readonly readKitAttributeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadAttributeCommand> } };

// Output types (simplified — full output types omitted for brevity, using `any` for command outputs)
export type ReadKitCommandOutput = Record<string, any>;
export type ReadDesignCommandOutput = Record<string, any>;
export type ReadTypeCommandOutput = Record<string, any>;
export type ReadPieceCommandOutput = Record<string, any>;
export type ReadSideCommandOutput = Record<string, any>;

export type ReadRootCommand = ReadKitCommand;
export type ReadCommandBatch = ReadonlyArray<ReadKitCommand>;
export type ReadCommandBatchResult = ReadonlyArray<ReadKitCommandOutput>;
//#endregion ReadCommandTypes

//#region KitGraphqlWire
export type KitGraphqlHandle = { execute(requestJson: string, onMessage: (line: string) => void): Promise<void> };
export type KitCommandReceipt = { requestId: KitCommandRequestId; commandKind: string; accepted: boolean };

export async function kitGraphqlRun(handle: KitGraphqlHandle, body: { query: string; variables?: Record<string, unknown>; operationName?: string }): Promise<unknown[]> {
  const out: unknown[] = [];
  await handle.execute(JSON.stringify(body), (line: string) => { out.push(JSON.parse(line)); });
  return out;
}

export function kitGraphqlFirstData(msgs: unknown[]): Record<string, unknown> {
  for (const m of msgs) {
    if (m == null || typeof m !== "object") continue;
    const r = m as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
    if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
    if (r.data != null && typeof r.data === "object") return r.data as Record<string, unknown>;
  }
  throw new Error("kitGraphql: no data in response");
}

export async function kitGraphqlSubmitCommandShell(handle: KitGraphqlHandle, commandKind: string, request: { query: string; variables?: Record<string, unknown>; operationName?: string }): Promise<KitCommandReceipt> {
  const data = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `mutation($input: KitCommandShellInput!) { submitKitCommand(input: $input) { requestId commandKind accepted } }`, variables: { input: { commandKind, request } } })) as { submitKitCommand?: Partial<KitCommandReceipt> };
  const receipt = data.submitKitCommand;
  if (!receipt || typeof receipt.requestId !== "string" || typeof receipt.commandKind !== "string" || receipt.accepted !== true) throw new Error("submitKitCommand: invalid receipt");
  return { requestId: receipt.requestId, commandKind: receipt.commandKind, accepted: true };
}

function storePayload(cmd: unknown): { tag: string; value: unknown } {
  if (cmd == null || typeof cmd !== "object" || Array.isArray(cmd)) throw new Error("kit store command: expected object");
  const o = cmd as Record<string, unknown>;
  const keys = Object.keys(o);
  if (keys.length !== 1) throw new Error("kit store command: expected a single tagged variant");
  return { tag: keys[0]!, value: o[keys[0]!] };
}

export async function kitGraphqlExecuteStoreCommand(handle: KitGraphqlHandle, cmd: unknown): Promise<unknown> {
  const { tag, value } = storePayload(cmd);
  const data = await kitGraphqlRun(handle, (() => {
    switch (tag) {
      case "newSession": return { query: `mutation { newSession }` };
      case "endSession": { const id = (value as { id?: string } | null)?.id; if (typeof id !== "string") throw new Error("endSession: need id"); return { query: `mutation($id: String!) { endSession(id: $id) }`, variables: { id } }; }
      case "newAlternative": { const v = value as { fromCheckpoint?: string | null; name: string } | null; if (v == null || typeof v.name !== "string") throw new Error("newAlternative"); return { query: `mutation($fromCheckpoint: String, $name: String!) { newAlternative(fromCheckpoint: $fromCheckpoint, name: $name) }`, variables: { fromCheckpoint: v.fromCheckpoint ?? null, name: v.name } }; }
      case "attachBackbone": { const cfg = (value as { config?: unknown } | null)?.config; return { query: `mutation($config: JSON!) { attachBackbone(config: $config) }`, variables: { config: cfg } }; }
      case "detachBackbone": return { query: `mutation { detachBackbone }` };
      case "listConflicts": return { query: `mutation { listConflicts }` };
      case "resolveConflict": { const v = value as { id?: string; strategy?: unknown } | null; if (typeof v?.id !== "string") throw new Error("resolveConflict"); return { query: `mutation($id: String!, $strategy: JSON!) { resolveConflict(id: $id, strategy: $strategy) }`, variables: { id: v.id, strategy: v.strategy } }; }
      case "backboneStatus": return { query: `mutation { backboneStatus }` };
      case "syncNow": return { query: `mutation { syncNow }` };
      case "batch": { const cmds = (value as { commands?: unknown[] } | null)?.commands; if (!Array.isArray(cmds)) throw new Error("batch.commands"); return { query: `mutation($commands: [JSON!]!) { kitStoreBatch(commands: $commands) }`, variables: { commands: cmds } }; }
      default: throw new Error(`kitGraphqlExecuteStoreCommand: unhandled ${tag}`);
    }
  })());
  const root = kitGraphqlFirstData(data);
  const op = Object.keys(root)[0];
  if (op === undefined) throw new Error("kitGraphql: empty mutation data");
  return root[op];
}

export function kitGraphqlSubscribeLoop(handle: KitGraphqlHandle, sink: (payload: unknown) => void): () => void {
  let cancelled = false;
  void handle.execute(JSON.stringify({ query: "subscription { eventStream }" }), (line: string) => {
    if (cancelled) return;
    try { const msg = JSON.parse(line) as { data?: { eventStream?: unknown } | null; errors?: unknown[] }; if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return; if (msg.data && "eventStream" in msg.data && msg.data.eventStream !== undefined) sink(normalizeKitCommandLifecycleEvent(msg.data.eventStream) ?? msg.data.eventStream); } catch { /* ignore */ }
  }).catch(() => { });
  return () => { cancelled = true; };
}

export async function kitGraphqlKitDesignPiecesMetadata(handle: KitGraphqlHandle, designId: string): Promise<Record<string, unknown>> {
  const root = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query($id: String!) { kitStore { designForId(id: $id) { piecesMetadataJson } } }`, variables: { id: designId } })) as { kitStore?: { designForId?: { piecesMetadataJson?: unknown } | null } };
  const v = root.kitStore?.designForId?.piecesMetadataJson;
  if (v && typeof v === "object" && !Array.isArray(v)) return v as Record<string, unknown>;
  return {};
}

function kitGraphqlCatalogJsonArray(v: unknown): unknown[] {
  if (Array.isArray(v)) return v;
  if (typeof v === "string") { try { const p = JSON.parse(v) as unknown; return Array.isArray(p) ? p : []; } catch { return []; } }
  return [];
}

function kitGraphqlJsonToReadonlyArray(v: unknown): ReadonlyArray<unknown> {
  if (Array.isArray(v)) return v;
  if (v == null) return [];
  if (typeof v === "string") { try { const p = JSON.parse(v) as unknown; return Array.isArray(p) ? p : []; } catch { return []; } }
  return [];
}

export function idDto(id: string): IdDto { return { id }; }

export async function kitGraphqlExecuteRead(handle: KitGraphqlHandle, batch: ReadCommandBatch): Promise<ReadCommandBatchResult> {
  const out: ReadKitCommandOutput[] = [];
  for (const c of batch) out.push(await kitGraphqlMapReadCommand(handle, c));
  return out;
}

export async function kitGraphqlMapReadCommand(handle: KitGraphqlHandle, c: ReadKitCommand): Promise<ReadKitCommandOutput> {
  if ("readKitTypeIdsCommand" in c && c.readKitTypeIdsCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { typeIds } }` })) as { kitStore?: { typeIds?: unknown } }; return { readKitTypeIdsCommand: { typeIds: kitGraphqlJsonToReadonlyArray(d.kitStore?.typeIds) } as any }; }
  if ("readKitDesignIdsCommand" in c && c.readKitDesignIdsCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { designIds } }` })) as { kitStore?: { designIds?: unknown } }; return { readKitDesignIdsCommand: { designIds: kitGraphqlJsonToReadonlyArray(d.kitStore?.designIds) } as any }; }
  if ("readKitTypesMetadataCommand" in c && c.readKitTypesMetadataCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { typesMetadata } }` })) as { kitStore?: { typesMetadata?: unknown } }; return { readKitTypesMetadataCommand: { types: kitGraphqlJsonToReadonlyArray(d.kitStore?.typesMetadata) } as any }; }
  if ("readKitDesignsMetadataCommand" in c && c.readKitDesignsMetadataCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { designsMetadata } }` })) as { kitStore?: { designsMetadata?: unknown } }; return { readKitDesignsMetadataCommand: { designs: kitGraphqlJsonToReadonlyArray(d.kitStore?.designsMetadata) } as any }; }
  if ("readKitTypesShallowCommand" in c && c.readKitTypesShallowCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { typesShallowJson } }` })) as { kitStore?: { typesShallowJson?: unknown } }; return { readKitTypesShallowCommand: { types: kitGraphqlJsonToReadonlyArray(d.kitStore?.typesShallowJson) } }; }
  if ("readKitDesignsShallowCommand" in c && c.readKitDesignsShallowCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { designsShallowJson } }` })) as { kitStore?: { designsShallowJson?: unknown } }; return { readKitDesignsShallowCommand: { designs: kitGraphqlJsonToReadonlyArray(d.kitStore?.designsShallowJson) } }; }
  if ("readKitAuthorsShallowCommand" in c && c.readKitAuthorsShallowCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { authorsShallowJson } }` })) as { kitStore?: { authorsShallowJson?: unknown } }; return { readKitAuthorsShallowCommand: { authors: kitGraphqlJsonToReadonlyArray(d.kitStore?.authorsShallowJson) } }; }
  if ("readKitMetadataCommand" in c && c.readKitMetadataCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query { kitStore { kitMetadataJson } }` })) as { kitStore?: { kitMetadataJson?: unknown } }; return { readKitMetadataCommand: { metadata: d.kitStore?.kitMetadataJson } }; }
  if ("readKitDesignCommands" in c && c.readKitDesignCommands) { const { id, commands } = c.readKitDesignCommands; const out: ReadDesignCommandOutput[] = []; for (const sub of commands) out.push(await kitGraphqlMapDesignRead(handle, id.id, sub)); return { readKitDesignCommands: { results: out } }; }
  if ("readKitTypeCommands" in c && c.readKitTypeCommands) { const { id, commands } = c.readKitTypeCommands; const out: ReadTypeCommandOutput[] = []; for (const sub of commands) out.push(await kitGraphqlMapTypeRead(handle, id.id, sub)); return { readKitTypeCommands: { results: out } }; }
  throw new Error(`kitGraphql: unsupported read command ${Object.keys(c).join(",")}`);
}

async function kitGraphqlMapDesignRead(handle: KitGraphqlHandle, designId: string, cmd: ReadDesignCommand): Promise<ReadDesignCommandOutput> {
  if ("readDesignPiecesFullCommand" in cmd && cmd.readDesignPiecesFullCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query($id: String!) { kitStore { designForId(id: $id) { piecesFullJson } } }`, variables: { id: designId } })) as { kitStore?: { designForId?: { piecesFullJson?: unknown } | null } | null }; return { readDesignPiecesFullCommand: { pieces: kitGraphqlJsonToReadonlyArray(d.kitStore?.designForId?.piecesFullJson) } }; }
  if ("readDesignConnectionsFullCommand" in cmd && cmd.readDesignConnectionsFullCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query($id: String!) { kitStore { designForId(id: $id) { connectionsFullJson } } }`, variables: { id: designId } })) as { kitStore?: { designForId?: { connectionsFullJson?: unknown } | null } | null }; return { readDesignConnectionsFullCommand: { connections: kitGraphqlJsonToReadonlyArray(d.kitStore?.designForId?.connectionsFullJson) } }; }
  if ("readDesignPieceCommands" in cmd && cmd.readDesignPieceCommands) { const { id, commands } = cmd.readDesignPieceCommands; const results: ReadPieceCommandOutput[] = []; for (const pc of commands) results.push(await kitGraphqlMapPieceRead(handle, designId, id.id, pc)); return { readDesignPieceCommands: { results } }; }
  throw new Error(`kitGraphqlMapDesignRead: ${Object.keys(cmd).join(",")}`);
}

async function kitGraphqlMapPieceRead(handle: KitGraphqlHandle, designId: string, pieceId: string, cmd: ReadPieceCommand): Promise<ReadPieceCommandOutput> {
  if ("readPieceFlatPlaneCommand" in cmd && cmd.readPieceFlatPlaneCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatPlane } } } }`, variables: { d: designId, p: pieceId } })) as any; return { readPieceFlatPlaneCommand: { flatPlane: d.kitStore?.designForId?.pieceForId?.flatPlane } }; }
  if ("readPieceFlatCenterCommand" in cmd && cmd.readPieceFlatCenterCommand === null) { const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatCenter } } } }`, variables: { d: designId, p: pieceId } })) as any; return { readPieceFlatCenterCommand: { flatCenter: d.kitStore?.designForId?.pieceForId?.flatCenter } }; }
  throw new Error(`kitGraphqlMapPieceRead: ${Object.keys(cmd).join(",")}`);
}

async function kitGraphqlMapTypeRead(handle: KitGraphqlHandle, typeId: string, cmd: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
  if ("readTypeBestRepresentationCommand" in cmd && cmd.readTypeBestRepresentationCommand) { const tags = cmd.readTypeBestRepresentationCommand.tagIds; const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: `query($id: String!, $tags: [String!]!) { kitStore { typeForId(id: $id) { bestRepresentation(tagIds: $tags) } } }`, variables: { id: typeId, tags: [...tags] } })) as any; return { readTypeBestRepresentationCommand: { representation: d.kitStore?.typeForId?.bestRepresentation } }; }
  throw new Error(`kitGraphqlMapTypeRead: ${Object.keys(cmd).join(",")}`);
}

export type KitExecuteRead = { executeRead(commands: ReadCommandBatch): Promise<ReadCommandBatchResult> };

function assertSingleReadResult(results: ReadCommandBatchResult): ReadKitCommandOutput {
  if (results.length !== 1) throw new Error(`read batch: expected 1 result, got ${results.length}`);
  return results[0]!;
}

export async function readKit(client: KitExecuteRead, command: ReadKitCommand): Promise<ReadKitCommandOutput> { return assertSingleReadResult(await client.executeRead([command])); }
export async function readKitDesign(client: KitExecuteRead, designId: string, command: ReadDesignCommand): Promise<ReadDesignCommandOutput> { const out = await readKit(client, { readKitDesignCommands: { id: idDto(designId), commands: [command] } }); if (!("readKitDesignCommands" in out) || out.readKitDesignCommands == null) throw new Error("read path: expected readKitDesignCommands"); return out.readKitDesignCommands.results[0]!; }
export async function readKitDesignPiece(client: KitExecuteRead, designId: string, pieceId: string, command: ReadPieceCommand): Promise<ReadPieceCommandOutput> { const d0 = await readKitDesign(client, designId, { readDesignPieceCommands: { id: idDto(pieceId), commands: [command] } }); if (!("readDesignPieceCommands" in d0) || d0.readDesignPieceCommands == null) throw new Error("read path: expected readDesignPieceCommands"); return d0.readDesignPieceCommands.results[0]!; }
export async function readKitType(client: KitExecuteRead, typeId: string, command: ReadTypeCommand): Promise<ReadTypeCommandOutput> { const out = await readKit(client, { readKitTypeCommands: { id: idDto(typeId), commands: [command] } }); if (!("readKitTypeCommands" in out) || out.readKitTypeCommands == null) throw new Error("read path: expected readKitTypeCommands"); return out.readKitTypeCommands.results[0]!; }
//#endregion KitGraphqlWire

//#region LiveFacades
export class LivePieceView { constructor(private readonly gql: KitGraphqlHandle, readonly designId: string, readonly pieceId: string) { } async readFlatPlane(): Promise<unknown> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatPlane } } } }`, variables: { d: this.designId, p: this.pieceId } })) as any; return d.kitStore?.designForId?.pieceForId?.flatPlane; } async readFlatCenter(): Promise<unknown> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatCenter } } } }`, variables: { d: this.designId, p: this.pieceId } })) as any; return d.kitStore?.designForId?.pieceForId?.flatCenter; } }
export class LiveDesignView { constructor(private readonly gql: KitGraphqlHandle, readonly designId: string) { } async readClusterableGroups(selection: ReadonlyArray<string>): Promise<ReadonlyArray<ReadonlyArray<IdDto>>> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { clusterableGroups(selection: $sel) } } }`, variables: { id: this.designId, sel: [...selection] } })) as any; const g = d.kitStore?.designForId?.clusterableGroups; if (!Array.isArray(g)) throw new Error("clusterableGroups"); return g.map((row: string[]) => row.map((id: string) => idDto(id))); } }
export class LiveTypeView { constructor(private readonly gql: KitGraphqlHandle, readonly typeId: string) { } async readBestRepresentation(tagIds: ReadonlyArray<string>): Promise<unknown | null | undefined> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query($id: String!, $tags: [String!]!) { kitStore { typeForId(id: $id) { bestRepresentation(tagIds: $tags) } } }`, variables: { id: this.typeId, tags: [...tagIds] } })) as any; return d.kitStore?.typeForId?.bestRepresentation; } }
/** 🧭 Live read helpers for kit-level catalog fields (used by @semio/react and hosts). */
export class LiveKitRoot {
  constructor(readonly gql: KitGraphqlHandle) { }
  piece(designId: string, pieceId: string) { return new LivePieceView(this.gql, designId, pieceId); }
  design(designId: string) { return new LiveDesignView(this.gql, designId); }
  type(typeId: string) { return new LiveTypeView(this.gql, typeId); }
  async readTypeIds(): Promise<readonly string[]> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { typeIds } }` })) as any; const typeIds = d.kitStore?.typeIds; if (!Array.isArray(typeIds)) throw new Error("typeIds"); return typeIds; }
  async readDesignIds(): Promise<readonly string[]> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { designIds } }` })) as any; const designIds = d.kitStore?.designIds; if (!Array.isArray(designIds)) throw new Error("designIds"); return designIds; }
  async readTypesMetadata(): Promise<readonly unknown[]> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { typesMetadata } }` })) as any; const rows = d.kitStore?.typesMetadata; if (!Array.isArray(rows)) throw new Error("typesMetadata"); return rows; }
  async readDesignsMetadata(): Promise<readonly unknown[]> { const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: `query { kitStore { designsMetadata } }` })) as any; const rows = d.kitStore?.designsMetadata; if (!Array.isArray(rows)) throw new Error("designsMetadata"); return rows; }
}
//#endregion LiveFacades

// #region KitViewStore
/** 🏷️Cache keys for {@link SemioKitViewStore} (catalog projections — align with `ReadKit*Command` names). */
export type KitViewCatalogKey = "typeIds" | "designIds" | "typesMetadata" | "designsMetadata";
const KIT_VIEW_EMPTY_ROW: readonly unknown[] = Object.freeze([]);

const semioKitViewStoreByClient = new WeakMap<KitStoreClient, SemioKitViewStore>();

/**
 * 🧭 Thin read-model store: subscribes to the kit event stream, refetches a catalog projection on activity,
 * and only notifies when the JSON snapshot for that key actually changes.
 */
export class SemioKitViewStore {
  private cache = new Map<KitViewCatalogKey, { serial: string; value: readonly unknown[] }>();
  private readonly listeners = new Map<KitViewCatalogKey, Set<() => void>>();
  private clientUnsub: (() => void) | undefined;
  private inFlight = new Set<KitViewCatalogKey>();
  private constructor(private readonly client: KitStoreClient) { }

  /** @emoji 🧭 Reuse one store instance per `KitStoreClient` so hooks share a cache. */
  static forClient(client: KitStoreClient) {
    let s = semioKitViewStoreByClient.get(client);
    if (!s) { s = new SemioKitViewStore(client); semioKitViewStoreByClient.set(client, s); }
    return s;
  }

  getSnapshot(key: KitViewCatalogKey): readonly unknown[] {
    return this.cache.get(key)?.value ?? KIT_VIEW_EMPTY_ROW;
  }

  private async loadKey(gql: KitGraphqlHandle, key: KitViewCatalogKey): Promise<unknown> {
    if (key === "typeIds") { const d = kitGraphqlFirstData(await kitGraphqlRun(gql, { query: `query { kitStore { typeIds } }` })) as any; return d.kitStore?.typeIds ?? []; }
    if (key === "designIds") { const d = kitGraphqlFirstData(await kitGraphqlRun(gql, { query: `query { kitStore { designIds } }` })) as any; return d.kitStore?.designIds ?? []; }
    if (key === "typesMetadata") { const d = kitGraphqlFirstData(await kitGraphqlRun(gql, { query: `query { kitStore { typesMetadata } }` })) as any; return d.kitStore?.typesMetadata ?? []; }
    const d = kitGraphqlFirstData(await kitGraphqlRun(gql, { query: `query { kitStore { designsMetadata } }` })) as any;
    return d.kitStore?.designsMetadata ?? [];
  }

  private commit(key: KitViewCatalogKey, raw: unknown): boolean {
    const serial = (() => { try { return JSON.stringify(raw); } catch { return String(raw); } })();
    const prev = this.cache.get(key);
    if (prev && prev.serial === serial) return false;
    const arr = Array.isArray(raw) ? (Object.freeze(raw.slice()) as readonly unknown[]) : KIT_VIEW_EMPTY_ROW;
    this.cache.set(key, { serial, value: arr });
    return true;
  }

  private async refreshKey(key: KitViewCatalogKey): Promise<boolean> {
    if (this.inFlight.has(key)) return false;
    this.inFlight.add(key);
    try { const v = await this.loadKey(this.client.kitGraphql(), key); return this.commit(key, v); }
    finally { this.inFlight.delete(key); }
  }

  private notify(key: KitViewCatalogKey) {
    const s = this.listeners.get(key);
    if (!s) return;
    for (const l of s) { try { l(); } catch { /* ignore */ } }
  }

  private touchAllWatched() {
    const keys = [...this.listeners.keys()].filter((k) => (this.listeners.get(k)?.size ?? 0) > 0);
    for (const key of keys) { void this.refreshKey(key as KitViewCatalogKey).then((c) => { if (c) this.notify(key as KitViewCatalogKey); }); }
  }

  private ensureEventPipe() {
    if (this.clientUnsub) return;
    this.clientUnsub = this.client.subscribe(() => { this.touchAllWatched(); });
  }

  /** @emoji 🧭 Subscribe to one catalog key; first subscriber triggers a load. */
  subscribe(key: KitViewCatalogKey, onChange: () => void) {
    this.ensureEventPipe();
    let set = this.listeners.get(key);
    if (!set) { set = new Set(); this.listeners.set(key, set); }
    set.add(onChange);
    void this.refreshKey(key).then((c) => { if (c) this.notify(key); else if (set!.size > 0 && !this.cache.has(key)) { this.commit(key, []); this.notify(key); } });
    return () => {
      set!.delete(onChange);
      if (set!.size === 0) { this.listeners.delete(key); this.cache.delete(key); }
      if (this.listeners.size === 0 && this.clientUnsub) { this.clientUnsub(); this.clientUnsub = undefined; this.cache.clear(); }
    };
  }
}

export function getSemioKitViewStore(client: KitStoreClient) { return SemioKitViewStore.forClient(client); }
// #endregion KitViewStore

//#region KitWorker
function settle(p: Promise<any>): Promise<any> { return p.catch((e: any) => ({ ok: false, error: { kind: "Internal", message: String(e) } })); }

export class KitWorkerApi {
  private handle: any = null;
  private eventListeners = new Map<number, (ev: unknown) => void>();
  private nextEventListenerId = 0;
  private eventGqlStarted = false;
  private gql(): KitGraphqlHandle { if (!this.handle) throw new Error("KitStoreHandle not initialized"); return { execute: (requestJson: string, onMessage: (line: string) => void) => this.handle.execute(requestJson, onMessage) }; }
  private requireHandle(): any { if (!this.handle) throw new Error("KitStoreHandle not initialized"); return this.handle; }
  private async submitSetResult(commandKind: string, request: { query: string; variables?: Record<string, unknown> }): Promise<SetResult> { try { const receipt = await kitGraphqlSubmitCommandShell(this.gql(), commandKind, request); return { ok: true, requestId: receipt.requestId }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  async init(wasmSpecifier: string, dto: unknown) { const mod = await importWasmModule(wasmSpecifier); if (typeof mod.default === "function") await mod.default(); if (typeof mod.boot === "function") mod.boot(); this.handle = mod.KitStoreHandle.create(dto as any); }
  snapshot() { return this.requireHandle().snapshot(); }
  setField(kind: string, id: string, field: string, value: unknown) { this.requireHandle(); try { const cmds = this.handle.changeKitCommandsForFieldPatch(kind, id, field, value); return this.submitSetResult("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands: cmds } }); } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown) { this.requireHandle(); try { const cmds = this.handle.changeKitCommandsForAddChild(parentKind, parentId, childKind, dto); return this.submitSetResult("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands: cmds } }); } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string) { this.requireHandle(); try { const cmds = this.handle.changeKitCommandsForRemoveChild(parentKind, parentId, childKind, childId); return this.submitSetResult("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands: cmds } }); } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  clusterPieces(designId: string, pieceIds: string[], clusterName: string) { this.requireHandle(); return this.submitSetResult("clusterPieces", { query: `mutation($designId: String!, $pieceIds: [String!]!, $clusterName: String!) { clusterPieces(designId: $designId, pieceIds: $pieceIds, clusterName: $clusterName) }`, variables: { designId, pieceIds, clusterName } }); }
  dragPieces(designId: string, pieceIds: string[], du: number, dv: number) { this.requireHandle(); return this.submitSetResult("dragPieces", { query: `mutation($designId: String!, $pieceIds: [String!]!, $du: Float!, $dv: Float!) { dragPieces(designId: $designId, pieceIds: $pieceIds, du: $du, dv: $dv) }`, variables: { designId, pieceIds, du, dv } }); }
  flattenDesign(designId: string) { this.requireHandle(); return this.submitSetResult("flattenDesign", { query: `mutation($designId: String!) { flattenDesign(designId: $designId) }`, variables: { designId } }); }
  undo() { this.requireHandle(); return this.submitSetResult("undo", { query: `mutation { undo }` }); }
  redo() { this.requireHandle(); return this.submitSetResult("redo", { query: `mutation { redo }` }); }
  canUndo() { return this.requireHandle().canUndo(); }
  canRedo() { return this.requireHandle().canRedo(); }
  graphqlExecute(requestJson: string, onMessage: (line: string) => void) { this.requireHandle(); return this.handle.execute(requestJson, onMessage); }
  subscribe(cb: (ev: unknown) => void) { this.requireHandle(); const proxy = Comlink.proxy(cb); const id = this.nextEventListenerId++; const forward = (payload: unknown) => { try { proxy(payload); } catch { /* ignore */ } }; this.eventListeners.set(id, forward); if (!this.eventGqlStarted) { this.eventGqlStarted = true; kitGraphqlSubscribeLoop(this.gql(), (payload) => { for (const fn of this.eventListeners.values()) fn(payload); }); } return () => { this.eventListeners.delete(id); if (this.eventListeners.size === 0) this.eventGqlStarted = false; }; }
  async execute(cmd: unknown) { this.requireHandle(); try { const result = await kitGraphqlExecuteStoreCommand(this.gql(), cmd); return { ok: true, result }; } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; } }
  async executeRead(cmds: ReadCommandBatch) { this.requireHandle(); return await kitGraphqlExecuteRead(this.gql(), cmds); }
  vcsState() { return this.requireHandle().vcsState(); }
  theKitDto() { return this.requireHandle().theKitDto(); }
  materializeAt(at: unknown) { return this.requireHandle().materializeAt(at); }
}

export const kitWorkerApi = new KitWorkerApi();
export function bootKitWorker() { Comlink.expose(kitWorkerApi); }
//#endregion KitWorker

// #region EmbeddedTests
if (process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  describe("semio-js thin client", () => {
    it("round-trips an empty kit through KitSchema", () => {
      const dto = { id: "kit-embedded-1", name: "Embedded", createdAt: "2020-01-01T00:00:00.000Z", updatedAt: "2020-01-01T00:00:00.000Z" };
      const k = Kit.fromPlain(dto);
      expect(k.toPlain().id).toBe("kit-embedded-1");
    });

    describe("export surface", () => {
      it("exports all entity classes", () => {
        const entityClasses = [Coordinate, Vec, Point, Vector, Plane, Camera, Attribute, Location, Author, File, Folder, Benchmark, Quality, Port, Family, Prop, Tag, Concept, Representation, Connector, Type, Piece, Connection, Design, Layer, Group, Side, Stat, Kit];
        for (const cls of entityClasses) expect(typeof cls).toBe("function");
      });
      it("exports key schemas", () => { expect(CoordinateSchema).toBeDefined(); expect(KitFullDtoSchema).toBeDefined(); expect(TypeSchema).toBeDefined(); expect(DesignSchema).toBeDefined(); expect(PieceSchema).toBeDefined(); expect(ConnectionSchema).toBeDefined(); });
      it("exports SetResult wire type shape", () => { const ok: SetResult = { ok: true }; const fail: SetResult = { ok: false, error: { kind: "NotFound", message: "x" } }; expect(ok.ok).toBe(true); expect(fail.ok).toBe(false); });
      it("exports bridge classes", () => { expect(typeof FallbackKitStoreClient).toBe("function"); expect(typeof WorkerKitStoreClient).toBe("function"); expect(typeof KitWorkerApi).toBe("function"); });
      it("exports Semio utility class", () => { expect(typeof Semio).toBe("function"); expect(typeof Semio.normalizeName).toBe("function"); expect(typeof Semio.round).toBe("function"); expect(typeof Semio.generateId).toBe("function"); });
      it("exports constants", () => { expect(typeof ICON_WIDTH).toBe("number"); expect(ICON_WIDTH).toBe(50); expect(typeof TOLERANCE).toBe("number"); expect(TOLERANCE).toBe(1e-5); });
      it("does NOT export deleted domain logic", async () => {
        const mod = await import("./index.ts") as Record<string, unknown>;
        const denylist = ["KitImpl", "KitEntity", "KitEntityIndexes", "KitEntityCaches", "hashKit", "hashType", "hashDesign", "hashPiece", "hashConnection", "computeChildPlane", "flattenPlacementWalkDesignOrderRoots", "validateKitGraphDiff", "expandSemanticCommandToDiff", "Generator", "SeededRandom", "round", "jaccard", "deepEqual", "arraysEqual", "toArray", "InMemoryKitStore", "asKitInstance", "selectBestRepresentation", "findRepresentation", "arePortsCompatible", "areConnectorsCompatible", "isFixedPiece", "findPiece", "findConnection", "mergeDesigns", "orientDesign"];
        for (const name of denylist) expect(mod[name]).toBeUndefined();
      });
    });

    describe("entity class methods", () => {
      const classesWithSerialize = [
        { name: "Coordinate", cls: Coordinate }, { name: "Vec", cls: Vec }, { name: "Point", cls: Point }, { name: "Vector", cls: Vector }, { name: "Plane", cls: Plane }, { name: "Camera", cls: Camera },
        { name: "Location", cls: Location }, { name: "Author", cls: Author }, { name: "File", cls: File }, { name: "Folder", cls: Folder }, { name: "Quality", cls: Quality }, { name: "Port", cls: Port },
        { name: "Family", cls: Family }, { name: "Prop", cls: Prop }, { name: "Tag", cls: Tag }, { name: "Concept", cls: Concept }, { name: "Representation", cls: Representation }, { name: "Connector", cls: Connector },
        { name: "Type", cls: Type }, { name: "Piece", cls: Piece }, { name: "Connection", cls: Connection }, { name: "Design", cls: Design }, { name: "Layer", cls: Layer }, { name: "Group", cls: Group },
        { name: "Side", cls: Side }, { name: "Stat", cls: Stat }, { name: "Kit", cls: Kit },
      ];
      for (const { name, cls } of classesWithSerialize) {
        it(`${name} has serialize() instance and deserialize() static`, () => {
          if (name === "Attribute" || name === "Benchmark") { expect(typeof cls.prototype.toJson).toBe("function"); expect(typeof (cls as any).fromJson).toBe("function"); }
          else { expect(typeof cls.prototype.serialize).toBe("function"); expect(typeof (cls as any).deserialize).toBe("function"); }
        });
        it(`${name} has toPlain() instance`, () => { expect(typeof cls.prototype.toPlain).toBe("function"); });
      }
      const classesWithFromPlain = [Vec, Point, Vector, Plane, Camera, Attribute, Location, Author, File, Quality, Port, Family, Prop, Tag, Concept, Representation, Connector, Type, Piece, Connection, Design, Layer, Group, Side, Stat, Kit];
      for (const cls of classesWithFromPlain) { it(`${cls.name} has fromPlain() static`, () => { expect(typeof (cls as any).fromPlain).toBe("function"); }); }
      const classesWithId = [Attribute, Location, Author, File, Quality, Port, Family, Prop, Tag, Concept, Representation, Connector, Type, Piece, Connection, Design, Layer, Group, Stat, Kit];
      for (const cls of classesWithId) { it(`${cls.name} has createId() and areSameId() static`, () => { expect(typeof (cls as any).createId).toBe("function"); expect(typeof (cls as any).areSameId).toBe("function"); }); }
    });

    describe("WASM bridge integration", () => {
      it("creates a FallbackKitStoreClient and performs basic operations", async () => {
        const minimalKit = { id: "test-kit", name: "TestKit", createdAt: "2020-01-01T00:00:00.000Z", updatedAt: "2020-01-01T00:00:00.000Z", types: [{ id: "type-1", name: "Wall", connectors: [] }], designs: [{ id: "design-1", name: "Floor1", pieces: [], connections: [] }] };
        const client = await createKitStoreClient({ initialKit: minimalKit, forceFallback: true });
        expect(client).toBeInstanceOf(FallbackKitStoreClient);
        const commandEvents: KitCommandLifecycleEvent[] = [];
        const unsubscribe = client.subscribe((event) => { if (isKitCommandLifecycleEvent(event)) commandEvents.push(event); });
        const setResult = await client.setField("Type", "type-1", "name", "BigWall");
        expect(typeof setResult.ok).toBe("boolean");
        expect(setResult.requestId === undefined || typeof setResult.requestId === "string").toBe(true);
        const types = await client.getTypes();
        expect(Array.isArray(types)).toBe(true);
        expect(types.length).toBeGreaterThanOrEqual(1);
        const designs = await client.getDesigns();
        expect(Array.isArray(designs)).toBe(true);
        expect(designs.length).toBeGreaterThanOrEqual(1);
        try { const undoResult = await client.undo(); expect(typeof undoResult.ok).toBe("boolean"); } catch { /* undo not available */ }
        const readBatch: ReadCommandBatch = [{ readKitTypesShallowCommand: null }];
        const readResults = await client.executeRead(readBatch);
        expect(Array.isArray(readResults)).toBe(true);
        expect(readResults.length).toBe(1);
        unsubscribe();
        client.dispose();
      });
    });
  });

  describe("semio-js property-based tests", async () => {
    const fc = await import("fast-check");
    describe("Property 1: Entity toPlain/fromPlain round-trip", () => {
      const safeDouble = () => fc.double({ min: -1e6, max: 1e6, noNaN: true }).filter((n) => !Object.is(n, -0));
      const coordinateArb = fc.record({ u: safeDouble(), v: safeDouble() });
      it("Coordinate.from(data).toPlain() deep-equals input", () => { fc.assert(fc.property(coordinateArb, (data) => { expect(Coordinate.from(data).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      const vecArb = fc.record({ u: safeDouble(), v: safeDouble() });
      it("Vec.fromPlain(data).toPlain() deep-equals input", () => { fc.assert(fc.property(vecArb, (data) => { expect(Vec.fromPlain(data).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      const pointArb = fc.record({ x: safeDouble(), y: safeDouble(), z: safeDouble() });
      it("Point.fromPlain(data).toPlain() deep-equals input", () => { fc.assert(fc.property(pointArb, (data) => { expect(Point.fromPlain(data).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      const vectorArb = fc.record({ x: safeDouble(), y: safeDouble(), z: safeDouble() });
      it("Vector.fromPlain(data).toPlain() deep-equals input", () => { fc.assert(fc.property(vectorArb, (data) => { expect(Vector.fromPlain(data).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      const attributeArb = fc.record({ id: fc.string({ minLength: 1, maxLength: 20 }), key: fc.string({ minLength: 1, maxLength: 20 }), value: fc.option(fc.string({ maxLength: 50 }), { nil: undefined }), definition: fc.option(fc.string({ maxLength: 50 }), { nil: undefined }) });
      it("Attribute.fromPlain(data).toPlain() deep-equals input", () => { fc.assert(fc.property(attributeArb, (data) => { expect(Attribute.fromPlain(data).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      const statArb = fc.record({ id: fc.string({ minLength: 1, maxLength: 20 }), quality: fc.record({ id: fc.string({ minLength: 1, maxLength: 20 }) }), unit: fc.option(fc.string({ maxLength: 20 }), { nil: undefined }), min: fc.option(safeDouble(), { nil: undefined }), minExcluded: fc.option(fc.boolean(), { nil: undefined }), max: fc.option(safeDouble(), { nil: undefined }), maxExcluded: fc.option(fc.boolean(), { nil: undefined }) });
      it("Stat.fromPlain(data).toPlain() deep-equals input", () => { fc.assert(fc.property(statArb, (data) => { expect(Stat.fromPlain(data).toPlain()).toEqual(data); }), { numRuns: 100 }); });
    });
    describe("Property 2: Entity serialize/deserialize round-trip", () => {
      const safeDouble = () => fc.double({ min: -1e6, max: 1e6, noNaN: true }).filter((n) => !Object.is(n, -0));
      it("Coordinate serialize/deserialize round-trip", () => { fc.assert(fc.property(fc.record({ u: safeDouble(), v: safeDouble() }), (data) => { const c = Coordinate.from(data); expect(Coordinate.deserialize(c.serialize()).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      it("Vec serialize/deserialize round-trip", () => { fc.assert(fc.property(fc.record({ u: safeDouble(), v: safeDouble() }), (data) => { const v = Vec.fromPlain(data); expect(Vec.deserialize(v.serialize()).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      it("Attribute toJson/fromJson round-trip", () => { fc.assert(fc.property(fc.record({ id: fc.string({ minLength: 1, maxLength: 20 }), key: fc.string({ minLength: 1, maxLength: 20 }), value: fc.option(fc.string({ maxLength: 50 }), { nil: undefined }), definition: fc.option(fc.string({ maxLength: 50 }), { nil: undefined }) }), (data) => { const a = Attribute.fromPlain(data); expect(Attribute.fromJson(a.toJson()).toPlain()).toEqual(data); }), { numRuns: 100 }); });
      it("Stat serialize/deserialize round-trip", () => { fc.assert(fc.property(fc.record({ id: fc.string({ minLength: 1, maxLength: 20 }), quality: fc.record({ id: fc.string({ minLength: 1, maxLength: 20 }) }), unit: fc.option(fc.string({ maxLength: 20 }), { nil: undefined }), min: fc.option(safeDouble(), { nil: undefined }), minExcluded: fc.option(fc.boolean(), { nil: undefined }), max: fc.option(safeDouble(), { nil: undefined }), maxExcluded: fc.option(fc.boolean(), { nil: undefined }) }), (data) => { const s = Stat.fromPlain(data); expect(Stat.deserialize(s.serialize()).toPlain()).toEqual(data); }), { numRuns: 100 }); });
    });
    describe("Property 3: Entity ID factory and comparison", () => {
      const entityClassesWithId: Array<{ name: string; cls: { createId: (id: string) => { id: string }; areSameId: (a: { id: string }, b: { id: string }) => boolean } }> = [
        { name: "Attribute", cls: Attribute }, { name: "Location", cls: Location }, { name: "Author", cls: Author }, { name: "File", cls: File }, { name: "Quality", cls: Quality }, { name: "Port", cls: Port },
        { name: "Family", cls: Family }, { name: "Prop", cls: Prop }, { name: "Tag", cls: Tag }, { name: "Concept", cls: Concept }, { name: "Representation", cls: Representation }, { name: "Connector", cls: Connector },
        { name: "Type", cls: Type }, { name: "Piece", cls: Piece }, { name: "Connection", cls: Connection }, { name: "Design", cls: Design }, { name: "Layer", cls: Layer }, { name: "Group", cls: Group },
        { name: "Stat", cls: Stat }, { name: "Kit", cls: Kit },
      ];
      for (const { name, cls } of entityClassesWithId) {
        it(`${name}.createId(a) produces { id: a } and areSameId works`, () => {
          fc.assert(fc.property(fc.string({ minLength: 1, maxLength: 50 }), fc.string({ minLength: 1, maxLength: 50 }), (a, b) => {
            const idA = cls.createId(a); const idB = cls.createId(b);
            expect(idA).toEqual({ id: a }); expect(cls.areSameId(idA, idB)).toBe(a === b);
          }), { numRuns: 100 });
        });
      }
    });
  });
}
// #endregion EmbeddedTests
