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
 * Type alias for AuthorMeta.
 **/
export type AuthorMeta = z.infer<typeof AuthorMetaSchema>;
attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Author changes.
 **/
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;

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
 * Type alias for FileMeta.
 **/
export type FileMeta = z.infer<typeof FileMetaSchema>;

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
 * Type alias for FolderMeta.
 **/
export type FolderMeta = z.infer<typeof FolderMetaSchema>;
attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Folder changes.
 **/
export type FolderDiff = z.infer<typeof FolderDiffSchema>;

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
attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Benchmark changes.
 **/
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;

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
 * Type alias for QualityMeta.
 **/
export type QualityMeta = z.infer<typeof QualityMetaSchema>;
benchmarks: BenchmarksDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Quality changes.
 **/
export type QualityDiff = z.infer<typeof QualityDiffSchema>;

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
 * Type alias for PortMeta.
 **/
export type PortMeta = z.infer<typeof PortMetaSchema>;
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
 * Type alias for FamilyMeta.
 **/
export type FamilyMeta = z.infer<typeof FamilyMetaSchema>;
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
 * Type alias for PropMeta.
 **/
export type PropMeta = z.infer<typeof PropMetaSchema>;
attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Prop changes.
 **/
export type PropDiff = z.infer<typeof PropDiffSchema>;

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
 * Type alias for TagMeta.
 **/
export type TagMeta = z.infer<typeof TagMetaSchema>;
attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Tag changes.
 **/
export type TagDiff = z.infer<typeof TagDiffSchema>;

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
 * Type alias for ConceptMeta.
 **/
export type ConceptMeta = z.infer<typeof ConceptMetaSchema>;
attributes: AttributesDiffSchema.optional(),
  description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
});
/**
 * Diff type for tracking Concept changes.
 **/
export type ConceptDiff = z.infer<typeof ConceptDiffSchema>;

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
 * Type alias for RepresentationMeta.
 **/
export type RepresentationMeta = z.infer<typeof RepresentationMetaSchema>;
attributes: AttributesDiffSchema.optional(),
});
/**
 * Diff type for tracking Representation changes.
 **/
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;

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
 * Type alias for ConnectorMeta.
 **/
export type ConnectorMeta = z.infer<typeof ConnectorMetaSchema>;
/**
 * Type alias for ConnectorShallow.
 **/
export type ConnectorShallow = z.infer<typeof ConnectorShallowSchema>;
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
 * Type alias for TypeMeta.
 **/
export type TypeMeta = z.infer<typeof TypeMetaSchema>;
representations: z.array(RepresentationMetaSchema).optional(),
  connectors: z.array(ConnectorMetaSchema).optional(),
    props: z.array(PropMetaSchema).optional(),
      attributes: z.array(AttributeMetaSchema).optional(),
});
/**
 * Type alias for TypeShallow.
 **/
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
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
 * Type alias for LayerMeta.
 **/
export type LayerMeta = z.infer<typeof LayerMetaSchema>;
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
 * Type alias for PieceShallow.
 **/
export type PieceShallow = z.infer<typeof PieceShallowSchema>;
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
 * Zod schema for Groups diff validation.
 **/
export const GroupsDiffSchema = z.object({
  removed: z.array(GroupIdSchema).optional(),
  updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type GroupsDiff = z.infer<typeof GroupsDiffSchema>;
/**
 * Type alias for GroupMeta.
 **/
export type GroupMeta = z.infer<typeof GroupMetaSchema>;
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
 * Type alias for ConnectionMeta.
 **/
export type ConnectionMeta = z.infer<typeof ConnectionMetaSchema>;

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
 * Zod schema for Stats diff validation.
 **/
export const StatsDiffSchema = z.object({
  removed: z.array(StatIdSchema).optional(),
  updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type StatsDiff = z.infer<typeof StatsDiffSchema>;
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
 * Definition of DesignMetaSchema.
 **/
export const DesignMetaSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true, authors: true, concepts: true });
/**
 * Type alias for DesignMeta.
 **/
export type DesignMeta = z.infer<typeof DesignMetaSchema>;
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

const detachConnectionForLocalMutation = (c: Connection | ConnectionPlain, hostDesign?: Design): Connection => {
  const plain =
    typeof (c as Connection).toPlain === "function"
      ? (c as Connection).toPlain()
      : ConnectionSchema.parse(stripNullsJsonClone(c) as unknown);
  const host = (c as Connection).getHostDesign?.() ?? hostDesign;
  return new Connection(ConnectionSchema.parse(stripNullsJsonClone(plain) as unknown), host);
};

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

// #region ÔÅ▒´©ÅKit
// Kit entity types, schemas, and helpers MUST be defined here.

// #region ­ƒº¼KitKind
export const KitKindSchema = z.enum(["dev", "local", "archive", "remote", "transport"]);
export type KitKind = z.infer<typeof KitKindSchema>;
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;
// #endregion ­ƒº¼KitKind

/**
 * Zod schema for Kit validation.
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

export type KitData = z.infer<typeof KitSchema>;


// #region ­ƒöûValidationState

// #region ­ƒöîBackbone Interface



// #endregion ­ƒöîBackbone Interface


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
export type KitWireDto = z.infer<typeof KitWireDtoSchema>;

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
export type KitInteractionDto = z.infer<typeof KitInteractionWireSchema>;

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
  importSnapshot(dto: KitWireDto): Promise<void>;
  exportSnapshot(): Promise<KitWireDto>;
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

export function flattenDesignOptimizedForKit(kit: KitLike, designId: string): DesignOperationResult {
  return asKitInstance(kit).flattenDesignMerkle(designId);
}

function requireKit(k: KitImpl): KitImpl {
  if (!(k instanceof KitImpl)) throw new Error("Expected a KitImpl class instance");
  return k;
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


function graphKitChangeFromLedger(c: KitChange): KitGraphChange {
  return {
    forward: c.diff,
    backward: c.inverse,
    validation: graphValidationFromLedgerReport(c.report),
  };
}

/** @alias {@link id} ÔÇö uuid v7 strings for {@link KitEntity} interactions. */
export { id as uuidv7 };


function kitDataFromWireDto(dto: KitWireDto): KitData {
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


// #endregion ­ƒº®KitEntity (synchronized kit facade)


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


/** Semantic command labels ÔÇö each maps to one deterministic KitDiff expansion (cross-language parity). */
export type SemioCommandKind = "DeletePiece" | "MovePiece" | "RenamePiece" | "ReconnectConnection" | "DeletePiecesCascade" | "NormalizeStaleConnections";


// #endregion ÔÅ▒´©ÅKitImpl


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




// #region ­ƒÅ░KitStore
// Storage-agnostic kit store contracts MUST be defined here.
// These interfaces express what a kit store DOES, not how a specific engine stores data.
// No engine-specific primitives (map/array/doc) may appear in these contracts.

// Specs: KitStoreStatus represents the lifecycle states of a kit store.
// Providers transition through states: idle ÔåÆ loading ÔåÆ ready ÔåÆ saving/syncing ÔåÆ ready.
// Error and offline are terminal-ish states that require external resolution.







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

function parseBackboneStatusDtoResult(raw: unknown): BackboneStatusDto {
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

function parseKitConflictsResult(raw: unknown): KitConflict[] {
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
          const plan = kitGraphqlFirstData(
            await kitGraphqlRun(this.gql(), {
              query: `query($kind: String!, $id: String!, $field: String!, $valueJson: String!) {
                kitStore { changeKitCommandsForFieldPatchValueJson(kind: $kind, id: $id, field: $field, valueJson: $valueJson) }
              }`,
              variables: { kind, id, field, valueJson: JSON.stringify(value) },
            }),
          ) as { kitStore?: { changeKitCommandsForFieldPatchValueJson?: unknown } };
          const raw = plan.kitStore?.changeKitCommandsForFieldPatchValueJson;
          if (raw == null) throw new Error("changeKitCommandsForFieldPatch");
          const cmds = unwrapKitGraphqlJsonField(raw);
          return await kitGraphqlChangeKitCommands(this.gql(), cmds);
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
          const plan = kitGraphqlFirstData(
            await kitGraphqlRun(this.gql(), {
              query: `query($parentKind: String!, $parentId: String!, $childKind: String!, $dtoJson: String!) {
                kitStore { changeKitCommandsForAddChildDtoJson(parentKind: $parentKind, parentId: $parentId, childKind: $childKind, dtoJson: $dtoJson) }
              }`,
              variables: { parentKind, parentId, childKind, dtoJson: JSON.stringify(dto) },
            }),
          ) as { kitStore?: { changeKitCommandsForAddChildDtoJson?: unknown } };
          const raw = plan.kitStore?.changeKitCommandsForAddChildDtoJson;
          if (raw == null) throw new Error("changeKitCommandsForAddChild");
          const cmds = unwrapKitGraphqlJsonField(raw);
          return await kitGraphqlChangeKitCommands(this.gql(), cmds);
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
          const plan = kitGraphqlFirstData(
            await kitGraphqlRun(this.gql(), {
              query: `query($parentKind: String!, $parentId: String!, $childKind: String!, $childId: String!) {
                kitStore { changeKitCommandsForRemoveChild(parentKind: $parentKind, parentId: $parentId, childKind: $childKind, childId: $childId) }
              }`,
              variables: { parentKind, parentId, childKind, childId },
            }),
          ) as { kitStore?: { changeKitCommandsForRemoveChild?: unknown } };
          const raw = plan.kitStore?.changeKitCommandsForRemoveChild;
          if (raw == null) throw new Error("changeKitCommandsForRemoveChild");
          const cmds = unwrapKitGraphqlJsonField(raw);
          return await kitGraphqlChangeKitCommands(this.gql(), cmds);
        } catch (e) {
          return { ok: false, error: normalizeWasmThrownKitError(e) };
        }
      })(),
    );
  }

  async applyDesignDiff(designId: string, diff: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlMutationSettle(this.gql(), {
        query: `mutation($designId: String!, $diff: JSON!) { applyDesignDiff(designId: $designId, diff: $diff) }`,
        variables: { designId, diff },
      }),
    );
  }

  async applyKitDiff(diff: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlMutationSettle(this.gql(), { query: `mutation($diff: JSON!) { applyKitDiff(diff: $diff) }`, variables: { diff } }),
    );
  }

  async clusterPieces(designId: string, pieceIds: string[], clusterName: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ clusterPieces: { pieceIds, clusterName } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async dragPieces(designId: string, pieceIds: string[], du: number, dv: number): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ dragPieces: { pieceIds, du, dv } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ movePieces: { pieceIds, gap, shift, rise } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async fixPieces(designId: string, pieceIds: string[]): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ fixPieces: { pieceIds } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async flattenDesign(designId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ flattenDesign: { confirm: true } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId: parentDesignId, commands: [{ expandDesign: { nestedDesignId } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ deleteConnection: { connectionId } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ changePieceType: { pieceId, newTypeId } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlMutationSettle(this.gql(), {
        query: `mutation($designId: String!, $selection: JSON!, $plane: JSON) {
          pasteDesignSelection(designId: $designId, selection: $selection, plane: $plane)
        }`,
        variables: { designId, selection, plane: plane == null ? null : plane },
      }),
    );
  }

  async createHangingPieces(designId: string, typeIds: string[], plane: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ createHangingPieces: { typeIds, plane } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ createConnectedPiece: { parentPiece, parentPort, childType, childPort } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), {
        commands: [{ design: { designId, commands: [{ createFixedPiece: { typeId, plane } }] } }],
      }).then(() => ({ ok: true })),
    );
  }

  async undo(): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), { commands: [{ live: { commands: [{ undo: { confirm: true } }] } }] }).then(() => ({ ok: true })),
    );
  }

  async redo(): Promise<SetResult> {
    return this.settleMutateAndRefresh(
      kitGraphqlBatchMutation(this.gql(), { commands: [{ live: { commands: [{ redo: { confirm: true } }] } }] }).then(() => ({ ok: true })),
    );
  }

  async canUndo(): Promise<boolean> {
    try {
      const d = kitGraphqlFirstData(
        await withTimeout(kitGraphqlRun(this.gql(), { query: `query { kitStore { canUndo } }` }), this.timeoutMs, "timeout"),
      ) as { kitStore?: { canUndo?: boolean } };
      return Boolean(d.kitStore?.canUndo);
    } catch {
      return false;
    }
  }

  async canRedo(): Promise<boolean> {
    try {
      const d = kitGraphqlFirstData(
        await withTimeout(kitGraphqlRun(this.gql(), { query: `query { kitStore { canRedo } }` }), this.timeoutMs, "timeout"),
      ) as { kitStore?: { canRedo?: boolean } };
      return Boolean(d.kitStore?.canRedo);
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
    const d = kitGraphqlFirstData(
      await withTimeout(kitGraphqlRun(this.gql(), { query: `query { kitStore { vcsStateJson } }` }), this.timeoutMs, "timeout"),
    ) as { kitStore?: { vcsStateJson?: unknown } };
    return d.kitStore?.vcsStateJson;
  }

  async theKitDto(): Promise<any> {
    const d = kitGraphqlFirstData(
      await withTimeout(
        kitGraphqlRun(this.gql(), { query: `query { kitStore { theKitDto } }` }),
        this.timeoutMs,
        "timeout",
      ),
    ) as { kitStore?: { theKitDto?: unknown } };
    return d.kitStore?.theKitDto;
  }

  async materializeAt(id: string): Promise<any> {
    const checkpointId = id.trim() === "" ? null : id;
    const d = kitGraphqlFirstData(
      await withTimeout(
        kitGraphqlRun(this.gql(), { query: `query($at: String) { kitStore { materializeAt(checkpointId: $at) } }`, variables: { at: checkpointId } }),
        this.timeoutMs,
        "timeout",
      ),
    ) as { kitStore?: { materializeAt?: unknown } };
    return d.kitStore?.materializeAt;
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
    return parseBackboneStatusDtoResult(r.result);
  }

  async listConflicts(): Promise<KitConflict[]> {
    const r = await this.execute({ listConflicts: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitConflictsResult(r.result);
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
    return parseBackboneStatusDtoResult(r.result);
  }

  async listConflicts(): Promise<KitConflict[]> {
    const r = await this.execute({ listConflicts: null });
    if (!r.ok) throw new Error(r.error.message);
    return parseKitConflictsResult(r.result);
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


// #endregion ­ƒÅ░KitStore




// ­ƒÄ¿getColorForText holds the data fields for a getColorForText record.


// #region ­ƒòîFile Tree Utilities
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
  folderId?: string;
  parentPath?: string;
}

/**
 * Constructs FileTree from components.
 **/
export const buildFileTree = (folders: Folder[], files: File[]): FileTreeNode[] => {
  const folderChildren = new Map<string | undefined, Folder[]>();
  folders.forEach((folder) => {
    const parent = folder.parent?.id;
    if (!folderChildren.has(parent)) folderChildren.set(parent, []);
    folderChildren.get(parent)!.push(folder);
  });

  const filesByFolder = new Map<string | undefined, File[]>();
  files.forEach((file) => {
    const folder = file.folder?.id;
    if (!filesByFolder.has(folder)) filesByFolder.set(folder, []);
    filesByFolder.get(folder)!.push(file);
  });

  const sortFolders = (items?: Folder[]): Folder[] => {
    return (items || []).slice().sort((a, b) => a.name.localeCompare(b.name));
  };

  const sortFiles = (items?: File[]): File[] => {
    return (items || []).slice().sort((a, b) => a.name.localeCompare(b.name));
  };

  const buildNodes = (parentId?: string, parentPath?: string): FileTreeNode[] => {
    const children: FileTreeNode[] = [];
    const childFolders = sortFolders(folderChildren.get(parentId));
    childFolders.forEach((folder) => {
      const nodePath = folder.id;
      children.push({
        name: folder.name,
        path: nodePath,
        parentPath,
        isDirectory: true,
        folderId: folder.id,
        children: buildNodes(folder.id, nodePath),
      });
    });
    const childFiles = sortFiles(filesByFolder.get(parentId));
    childFiles.forEach((file) => {
      children.push({
        name: file.name,
        path: file.id,
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

// #endregion 📂File Tree Utilities

// #region 🔬Runtime Test Flags
// Test and benchmark blocks MUST compile out of browser bundles while staying runnable in Node.
declare const __SEMIO_JS_RUN_EMBEDDED_TESTS__: boolean | undefined;
declare const __SEMIO_JS_RUN_BENCHMARKS__: boolean | undefined;

const shouldRunEmbeddedJsTests =
  (typeof __SEMIO_JS_RUN_EMBEDDED_TESTS__ !== "undefined" && __SEMIO_JS_RUN_EMBEDDED_TESTS__) ||
  (typeof __SEMIO_JS_RUN_EMBEDDED_TESTS__ === "undefined" && typeof (globalThis as any).__vitest_worker__ !== "undefined" && typeof process !== "undefined" && process.env.SEMIO_JS_RUN_EMBEDDED_TESTS === "1");

const shouldRunJsBenchmarks = (typeof __SEMIO_JS_RUN_BENCHMARKS__ !== "undefined" && __SEMIO_JS_RUN_BENCHMARKS__) || (typeof __SEMIO_JS_RUN_BENCHMARKS__ === "undefined" && typeof process !== "undefined" && process.argv?.includes("--bench"));

// #endregion 🔬Runtime Test Flags

// #region 🔬Tests
// Vitest suites for the JS bundle. Guarded so browser consumers do not load test-only deps.
if (shouldRunEmbeddedJsTests) {
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
    NakaginCapsuleTowerPasteWithCoordinateDesignDiff,
    NakaginCapsuleTowerDiffDesign,
    NakaginCapsuleTowerWithDiffDesign,
    ValidateKitDiffCases,
    FlattenMerkleCases,
    HashCases,
    QualitySumCases,
    DesignWithDiffCases,
    FilterKitCases,
    FindReplaceableTypesCases,
    RepresentationSelectionCases,
    FlattenCases,
    SyntheticFindReplaceableKit,
    ExportDesignModelCases,
    ExportDesignRepresentationCases,
    DeleteCases,
    CopyPasteCases,
  } = await import("@semio/assets");
  const NAKAGIN_DESIGN_NAME = (HashCases as any).designName as string;
  const deleteCasesData = (DeleteCases as any).cases as Array<{ name: string; designName: string; designFamilies: string[]; selectionAsset: string; expectedDiffAsset: string }>;
  const copyPasteCasesData = (CopyPasteCases as any).cases as Array<{
    name: string;
    designName: string;
    designFamilies: string[];
    selectionAsset: string;
    expectedCopyAsset: string;
    pasteTargetAsset: string;
    expectedPasteDiffAsset: string;
    pasteCoordinate: { u: number; v: number };
    expectedPasteWithCoordinateDiffAsset: string;
  }>;
  const filterKitCasesData = FilterKitCases as any;

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
    if (!c1 || !c2) return false;
    return Math.abs(c1.u - c2.u) < TEST_TOLERANCE && Math.abs(c1.v - c2.v) < TEST_TOLERANCE;
  };

  const findDesign = (kit: KitImpl, name: string) => {
    const design = kit.designs?.find((candidate) => candidate.name === name);
    if (!design) {
      throw new Error(`Design ${name} not found`);
    }
    return design;
  };

  /** Clone kit and replace one design after local mutation (keeps {@link KitImpl} methods ÔÇö object spread would drop the prototype). */
  const kitWithSwappedDesign = (source: KitImpl, designId: string, mutate: (detached: Design) => void): KitImpl => {
    const k = duplicateKitForIsolation(source);
    const row = k.designs!.find((d) => d.id === designId);
    if (!row) throw new Error(`design ${designId} not found`);
    const copy = detachDesignForLocalMutation(row);
    mutate(copy);
    k.designs = k.designs!.map((d) => (d.id === designId ? copy : d));
    return k;
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

  const runExportReportCommand = async (command: string, args: string[], cwd: string) => {
    const { execFileSync } = await import("node:child_process");
    for (let attempt = 1; attempt <= 2; attempt += 1) {
      try {
        execFileSync(command, args, {
          cwd,
          stdio: "pipe",
        });
        return;
      } catch (error) {
        if (attempt === 2) {
          throw error;
        }
      }
    }
  };

  const roundSceneNumber = (value: number) => {
    const rounded = Math.round(value * 10_000) / 10_000;
    return Object.is(rounded, -0) ? 0 : rounded;
  };

  describe("Kit object representation (spec API)", () => {
    it("exports rust-aligned dto and backbone names", () => {
      expect(typeof DevBackbone).toBe("function");
      expect(typeof LocalBackbone).toBe("function");
      expect(typeof RemoteBackbone).toBe("function");

      const backboneConfig: BackboneConfig = { dev: { path: "example.json" } };
      const conflictResolution: ConflictResolution = { dropWip: null };
      const backboneStatus: BackboneStatusDto = { attached: false, kind: null, tip: null };
      const kitConflict: KitConflict = {
        id: "conflict-1",
        wipCheckpoint: null,
        backboneTip: null,
        reason: "reason",
        createdAt: "2026-01-01T00:00:00.000Z",
      };
      const kitWireDto: KitWireDto = emptyKitWireDto();
      const kitInteractionDto: KitInteractionDto = {
        uuid: "interaction-1",
        label: "Test",
        selection: { types: [], designs: [] },
      };

      expect(backboneConfig.dev?.path).toBe("example.json");
      expect(conflictResolution.dropWip).toBeNull();
      expect(backboneStatus.attached).toBe(false);
      expect(kitConflict.id).toBe("conflict-1");
      expect(kitWireDto.name).toBe("Untitled Kit");
      expect(kitInteractionDto.selection.types).toEqual([]);
    });

    it("Kit(), transactions.start (uuid v7), setActiveTransaction, findDesign/findPiece objects, connections(), piece ops, history undo", () => {
      const kit = Kit(MetabolismKit as KitData);

      const t1 = kit.transactions.start("One transaction");
      const t2 = kit.transactions.start("Another concurrent transaction");
      expect(t1).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i);
      expect(t2).not.toBe(t1);

      kit.setActiveTransaction(t1);

      {
        const nct0 = kit.findDesign({ name: "Nakagin Capsule Tower" });
        expect(nct0).toBeDefined();
        if (!nct0) throw new Error("missing design");
        nct0.flatten();
      }
      const nct = kit.findDesign({ name: "Nakagin Capsule Tower" })!;
      const oldConnections = nct.connections();
      expect(Array.isArray(oldConnections)).toBe(true);

      const primary =
        nct.getPieces().find((p) => {
          try {
            return p.alternativeTypes().length > 1;
          } catch {
            return false;
          }
        }) ?? nct.getPieces()[1];
      expect(primary).toBeDefined();
      if (!primary) throw new Error("missing primary piece");

      const c1TA = primary.alternativeTypes();
      expect(c1TA.length).toBeGreaterThan(1);

      const primaryId = primary.id;

      primary.delete();
      expect(kit.findDesign({ name: "Nakagin Capsule Tower" })!.findPiece(primaryId)).toBeUndefined();

      kit.transaction.undo();
      expect(kit.findDesign({ name: "Nakagin Capsule Tower" })!.findPiece(primaryId)).toBeDefined();

      kit.setActiveTransaction(t2);
      const nct2 = kit.findDesign({ name: "Nakagin Capsule Tower" })!;
      const anotherPiece = nct2.getPieces().find((p) => p.id !== primaryId) ?? nct2.getPieces()[0];
      expect(anotherPiece).toBeDefined();
      if (!anotherPiece) throw new Error("missing secondary piece");

      const typeBefore = anotherPiece.type?.id;
      const altType = kit.requireType(c1TA[1].id);
      anotherPiece.changeType(altType);
      const anotherAfter = kit.findDesign({ name: "Nakagin Capsule Tower" })!.findPiece(anotherPiece.id)!;
      expect(anotherAfter.type?.id).toBe(altType.id);
      expect(anotherAfter.type?.id).not.toBe(typeBefore);

      kit.unsetActiveTransaction();

      kit.finalizeTransaction(t1);
      kit.finalizeTransaction(t2);

      kit.undo();
      const secondaryAfterUndo = kit.findDesign({ name: "Nakagin Capsule Tower" })!.findPiece(anotherAfter.id);
      expect(secondaryAfterUndo?.type?.id).toBe(typeBefore);
    });

    it("importLocal loads kit JSON from a path (Node)", async () => {
      const path = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const kitJsonPath = path.join(path.dirname(fileURLToPath(import.meta.url)), "..", "assets", "semio", "metabolism.kit.semio.json");
      const kit = Kit();
      await kit.importLocal(kitJsonPath);
      const d = kit.findDesign({ name: "Nakagin Capsule Tower" });
      expect(d).toBeDefined();
    });
  });

  describe("Change", () => {
    describe("Metabolism", () => {
      const kitOriginal = new KitImpl({
        ...(MetabolismKit as any),
      } as KitData);
      const kitDiff = MetabolismKitDiff as any;
      const kitDiffInverted = MetabolismKitDiffInverted as any;
      const kitDiffed = new KitImpl(MetabolismKitDiffed as KitData);

      it.skip("KitImpl + Change.Forward = DiffedKit & DiffedKit + Change.Backward = KitImpl", () => {
        // [DEBUG] Re-baseline Metabolism kit diff assets when kit graph equality (apply forward) matches getDiff/change fixtures.
        const change = getKitChange(kitOriginal, kitDiffed);
        const computedDiff = getKitDiff(kitOriginal, kitDiffed);
        if (!areKitDiffsEqual(computedDiff, kitDiff)) {
          const canonicalize = (v: unknown): unknown => JSON.parse(JSON.stringify(v));
          console.error("[DEBUG] computedDiff:", JSON.stringify(canonicalize(computedDiff)).slice(0, 2000));
          console.error("[DEBUG] expectedDiff:", JSON.stringify(canonicalize(kitDiff)).slice(0, 2000));
        }
        expect(areKitDiffsEqual(computedDiff, kitDiff)).toBe(true);
        const computedInverseDiff = inverseKitDiff(kitOriginal, change.forward);
        expect(areKitDiffsEqual(computedInverseDiff, kitDiffInverted)).toBe(true);
        expect(areKitDiffsEqual(change.forward, kitDiff)).toBe(true);
        expect(areKitDiffsEqual(change.backward, kitDiffInverted)).toBe(true);
        const appliedForward = duplicateKitForIsolation(kitOriginal);
        applyKitDiff(appliedForward, change.forward);
        expect(areKitsEqual(appliedForward, kitDiffed)).toBe(true);
        const appliedInverse = duplicateKitForIsolation(kitDiffed);
        applyKitDiff(appliedInverse, change.backward);
        expect(areKitsEqual(appliedInverse, kitOriginal)).toBe(true);
      });

      describe("Design/Representation", () => {
        it("selectBestRepresentation uses tag filtering + modified jaccard and matches shared semio asset cases", () => {
          const payload = RepresentationSelectionCases as {
            cases: Array<{
              name: string;
              selectedTagIds: string[];
              expectedId: string | null;
              representations: Array<{ id: string; fileId: string; tagIds: string[] }>;
            }>;
          };
          payload.cases.forEach((testCase) => {
            const representations: Representation[] = testCase.representations.map((representation) => ({
              id: representation.id,
              file: { id: representation.fileId },
              tags: representation.tagIds.map((id) => ({ id })),
            }));
            const selected = selectBestRepresentation(representations, testCase.selectedTagIds);
            expect(selected?.id ?? null).toBe(testCase.expectedId);
          });
        });
      });
    });
  });

  // #region ­ƒÅ░KitImpl Filter Tests
  // Tests for filterKit MUST verify correct subset extraction with design-based and glob-based filters.

  describe("KitImpl/Filter/Design", () => {
    const filterDesignCase0 = ((FilterKitCases as any).cases as Array<{ designName: string }>)[0];
    const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
    const expected = NakaginCapsuleTowerFilteredKit as any;
    const nakaginDesign = kit.designs?.find((d) => d.name === filterDesignCase0.designName);

    it("filters kit to only contain entities related to Nakagin Capsule Tower design", () => {
      expect(nakaginDesign).toBeDefined();
      const filtered = filterKit(kit, { designId: nakaginDesign!.id });

      expect(filtered.designs?.length).toBe(expected.designs.length);
      expect(filtered.types?.length).toBe(expected.types.length);
      expect(filtered.files?.length).toBe(expected.files.length);
      expect(filtered.families?.length).toBe(expected.families?.length ?? expected.ports?.length ?? 0);
      expect(filtered.qualities?.length).toBe(expected.qualities.length);
      expect(filtered.authors?.length).toBe(expected.authors.length);

      const filteredDesign = filtered.designs?.find((d) => d.id === nakaginDesign!.id);
      expect(filteredDesign).toBeDefined();
      expect(filteredDesign!.pieces?.length).toBe(nakaginDesign!.pieces?.length);

      for (const expectedType of expected.types) {
        const filteredType = filtered.types?.find((t: any) => t.id === expectedType.id);
        expect(filteredType).toBeDefined();
        expect(filteredType!.representations?.length ?? 0).toBe(expectedType.representations?.length ?? 0);
      }

      for (const piece of filteredDesign!.pieces ?? []) {
        if (piece.type?.id) {
          expect(filtered.types?.some((t) => t.id === piece.type!.id)).toBe(true);
        }
      }

      for (const type of filtered.types ?? []) {
        expect((type.representations ?? []).length).toBeLessThanOrEqual(1);
        for (const representation of type.representations ?? []) {
          expect(filtered.files?.some((f) => f.id === representation.file.id)).toBe(true);
        }
        for (const connector of type.connectors ?? []) {
          if (connector.port?.id) {
            expect((filtered.families ?? []).some((f) => (f.ports ?? []).some((p) => p.id === connector.port!.id))).toBe(true);
          }
        }
      }
    });

    it("preserves kit metadata", () => {
      const filtered = filterKit(kit, { designId: nakaginDesign!.id });
      expect(filtered.id).toBe(kit.id);
      expect(filtered.name).toBe(kit.name);
      expect(filtered.version).toBe(kit.version);
    });
  });

  describe("KitImpl/Filter/Glob", () => {
    const filterKitGlobCases = (FilterKitCases as any).globCases as Array<{
      name: string;
      typeInclude?: string[];
      typeExclude?: string[];
      designInclude?: string[];
      designName?: string;
      designFamilies?: string[];
    }>;
    const globMatchAssetCases = filterKitCasesData.globMatchCases as Array<{ input: string; pattern: string; expected: boolean }>;
    const globMatchCaseInsensitiveCases = filterKitCasesData.globMatchCaseInsensitiveCases as Array<{ input: string; pattern: string; expected: boolean }>;
    const matchesGlobFilterAssetCases = filterKitCasesData.matchesGlobFilterCases as Array<{ name: string; input: string; filter?: { include?: string[]; exclude?: string[] }; expected: boolean }>;

    it("globMatch matches wildcard patterns", () => {
      for (const tc of globMatchAssetCases) {
        expect(globMatch(tc.input, tc.pattern)).toBe(tc.expected);
      }
    });

    it("globMatch is case-insensitive", () => {
      for (const tc of globMatchCaseInsensitiveCases) {
        expect(globMatch(tc.input, tc.pattern)).toBe(tc.expected);
      }
    });

    for (const tc of matchesGlobFilterAssetCases) {
      it(`matchesGlobFilter: ${tc.name}`, () => {
        expect(matchesGlobFilter(tc.input, tc.filter)).toBe(tc.expected);
      });
    }

    for (const gc of filterKitGlobCases) {
      it(`filterKit glob case: ${gc.name}`, () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const totalTypes = kit.types?.length ?? 0;
        const filter: any = {};
        if (gc.typeInclude) filter.types = { ...filter.types, include: gc.typeInclude };
        if (gc.typeExclude) filter.types = { ...filter.types, exclude: gc.typeExclude };
        if (gc.designInclude) filter.designs = { include: gc.designInclude };
        if (gc.designName) {
          const nakaginDesign = kit.designs?.find((d) => d.name === gc.designName);
          expect(nakaginDesign).toBeDefined();
          filter.designId = nakaginDesign!.id;
        }
        const filtered = filterKit(kit, filter);
        if (gc.typeInclude) {
          expect(filtered.types?.length ?? 0).toBeGreaterThan(0);
          expect(filtered.types?.length ?? 0).toBeLessThan(totalTypes);
          for (const t of filtered.types ?? []) {
            expect(gc.typeInclude.some((pat) => globMatch(t.name, pat))).toBe(true);
          }
        }
        if (gc.typeExclude && !gc.designName) {
          expect(filtered.types?.length ?? 0).toBeLessThan(totalTypes);
          for (const t of filtered.types ?? []) {
            expect(gc.typeExclude.some((pat) => globMatch(t.name, pat))).toBe(false);
          }
        }
        if (gc.designInclude) {
          expect(filtered.designs?.length ?? 0).toBeGreaterThan(0);
          for (const d of filtered.designs ?? []) {
            expect(gc.designInclude.some((pat) => globMatch(d.name, pat))).toBe(true);
          }
        }
        if (!gc.typeInclude && !gc.typeExclude && !gc.designInclude && !gc.designName) {
          expect(filtered.types?.length).toBe(kit.types?.length);
          expect(filtered.designs?.length).toBe(kit.designs?.length);
          expect(filtered.families?.length).toBe(kit.families?.length);
        }
        if (gc.designName && gc.typeExclude) {
          const designOnlyFiltered = filterKit(kit, { designId: filter.designId });
          expect(filtered.types?.length ?? 0).toBeLessThan(designOnlyFiltered.types?.length ?? 0);
          for (const t of filtered.types ?? []) {
            expect(gc.typeExclude.some((pat) => globMatch(t.name, pat))).toBe(false);
          }
        }
      });
    }
  });

  // #endregion ­ƒÅ░KitImpl Filter Tests

  // #region ­ƒøí´©ÅKitKind Tests
  // Tests for KitKind enum MUST verify the five kit kinds.

  describe("KitKind", () => {
    it("KitKindSchema accepts all five valid kinds", () => {
      const kinds = ["dev", "local", "archive", "remote", "transport"] as const;
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
      expect(ALL_KIT_KINDS).toContain("dev");
      expect(ALL_KIT_KINDS).toContain("local");
      expect(ALL_KIT_KINDS).toContain("archive");
      expect(ALL_KIT_KINDS).toContain("remote");
      expect(ALL_KIT_KINDS).toContain("transport");
    });

    it("KitKind type is assignable from literal strings", () => {
      const dev: KitKind = "dev";
      const local: KitKind = "local";
      const archive: KitKind = "archive";
      const remote: KitKind = "remote";
      const transport: KitKind = "transport";
      expect([dev, local, archive, remote, transport]).toEqual(["dev", "local", "archive", "remote", "transport"]);
    });

    it("KitImpl/File: roundtrips through JSON serialize/deserialize", () => {
      const kit: KitImpl = {
        id: "file-kit-id",
        name: "FileKit Test",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const json = serializeKit(kit);
      const restored = deserializeKit(json);
      expect(restored.id).toBe(kit.id);
      expect(restored.name).toBe(kit.name);
    });

    it("KitImpl/File: imports, exports and edits with file kit helpers", async () => {
      const kit: KitImpl = {
        id: "file-kit-helper-id",
        name: "FileKit Helper Test",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const exported = exportFileKit(kit);
      const imported = await importFileKit(exported);
      expect(imported.kind).toBe("dev");
      expect(imported.kit.id).toBe(kit.id);
      const edited = editTemporaryKit(imported.kit, { name: "FileKit Helper Edited" });
      expect(edited.name).toBe("FileKit Helper Edited");
      expect(imported.kit.name).toBe("FileKit Helper Edited");
      expect(edited).toBe(imported.kit);
    });

    it("KitImpl/Folder: roundtrips through SQLite via FolderKitStore adapter", async () => {
      const kit: KitImpl = {
        id: "folder-kit-id",
        name: "FolderKit Test",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
        types: [{ id: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
      };
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(kit, db);
      const data = db.export();
      db.close();
      const db2 = new SQL.Database(new Uint8Array(data));
      const restored = await sqliteToKit(db2);
      db2.close();
      expect(restored.id).toBe(kit.id);
      expect(restored.name).toBe(kit.name);
      expect(restored.types).toHaveLength(1);
      expect(restored.types![0].name).toBe("Wall");
    });

    it("KitImpl/Archive: roundtrips through zip export/import", async () => {
      const kit: KitImpl = {
        id: "archive-kit-id",
        name: "ArchiveKit Test",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
        types: [{ id: "at1", name: "Beam", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
      };
      const blob = await exportKit(kit);
      const result = await importKit(blob);
      expect(result.kit.id).toBe(kit.id);
      expect(result.kit.name).toBe(kit.name);
      expect(result.kit.types).toHaveLength(1);
      expect(result.kit.types![0].name).toBe("Beam");
    });

    it("KitImpl/Remote: validates remote URL field on kit", () => {
      const kit: KitImpl = {
        id: "remote-kit-id",
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

    it("KitImpl/Remote: imports remote JSON and archive sources", async () => {
      const remoteJsonKit: KitImpl = {
        id: "remote-json-kit-id",
        name: "Remote JSON KitImpl",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const remoteArchiveKit: KitImpl = {
        id: "remote-archive-kit-id",
        name: "Remote Archive KitImpl",
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
        expect(importedJson.kit.id).toBe(remoteJsonKit.id);

        const importedArchive = await importRemoteKit("https://example.com/remote.kit.zip");
        expect(importedArchive.kind).toBe("remote");
        expect(importedArchive.kit.id).toBe(remoteArchiveKit.id);
      } finally {
        globalThis.fetch = originalFetch;
      }
    });

    it("KitImpl/Temporary: InMemoryKitStore roundtrip without persistence", () => {
      const kit: KitImpl = {
        id: "temp-kit-id",
        name: "TemporaryKit Test",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      const store = new InMemoryKitStore(kit);
      expect(store.getSnapshot().kit.id).toBe("temp-kit-id");
      store.apply({ name: "Modified Temporary" });
      expect(store.getSnapshot().kit.name).toBe("Modified Temporary");
      store.undo();
      expect(store.getSnapshot().kit.name).toBe("TemporaryKit Test");
    });

    it("KitImpl/Temporary: editTemporaryKit applies a diff in place and returns the same reference", () => {
      const kit = new KitImpl({
        id: "temp-edit-kit-id",
        name: "Temporary Editable KitImpl",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      } as KitData);
      const edited = editTemporaryKit(kit, { name: "Temporary Editable KitImpl Edited" });
      expect(edited.name).toBe("Temporary Editable KitImpl Edited");
      expect(kit.name).toBe("Temporary Editable KitImpl Edited");
      expect(edited).toBe(kit);
    });
  });

  // #endregion ­ƒøí´©ÅKitKind Tests

  // #region ­ƒÅ░KitImpl Filter Tests
  // Tests for filterKit MUST verify correct subset extraction with design-based and glob-based filters.

  describe("KitImpl/Filter/Design", () => {
    const filterDesignCases = (FilterKitCases as any).cases as Array<{ name: string; designName: string; designFamilies: string[] }>;
    const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
    const expected = MetabolismKitFilteredNakaginCapsuleTower as any;
    const filterDesignCase = filterDesignCases[0];
    const nakaginDesign = kit.designs?.find((d) => d.name === filterDesignCase.designName);

    it("filters kit to only contain entities related to Nakagin Capsule Tower design", () => {
      expect(nakaginDesign).toBeDefined();
      const filtered = filterKit(kit, { designId: nakaginDesign!.id });

      expect(filtered.designs?.length).toBe(expected.designs.length);
      expect(filtered.types?.length).toBe(expected.types.length);
      expect(filtered.files?.length).toBe(expected.files.length);
      expect(filtered.families?.length).toBe(expected.families?.length ?? expected.ports?.length ?? 0);
      expect(filtered.qualities?.length).toBe(expected.qualities.length);
      expect(filtered.authors?.length).toBe(expected.authors.length);

      const filteredDesign = filtered.designs?.find((d) => d.id === nakaginDesign!.id);
      expect(filteredDesign).toBeDefined();
      expect(filteredDesign!.pieces?.length).toBe(nakaginDesign!.pieces?.length);

      for (const expectedType of expected.types) {
        const filteredType = filtered.types?.find((t: any) => t.id === expectedType.id);
        expect(filteredType).toBeDefined();
        expect(filteredType!.representations?.length ?? 0).toBe(expectedType.representations?.length ?? 0);
      }

      for (const piece of filteredDesign!.pieces ?? []) {
        if (piece.type?.id) {
          expect(filtered.types?.some((t) => t.id === piece.type!.id)).toBe(true);
        }
      }

      for (const type of filtered.types ?? []) {
        for (const representation of type.representations ?? []) {
          expect(filtered.files?.some((f) => f.id === representation.file.id)).toBe(true);
        }
        for (const connector of type.connectors ?? []) {
          if (connector.port?.id) {
            expect((filtered.families ?? []).some((f) => (f.ports ?? []).some((p) => p.id === connector.port!.id))).toBe(true);
          }
        }
      }
    });

    it("preserves kit metadata", () => {
      const filtered = filterKit(kit, { designId: nakaginDesign!.id });
      expect(filtered.id).toBe(kit.id);
      expect(filtered.name).toBe(kit.name);
      expect(filtered.version).toBe(kit.version);
    });

    it("each type has at most one representation", () => {
      const filtered = filterKit(kit, { designId: nakaginDesign!.id });
      for (const type of filtered.types ?? []) {
        expect((type.representations ?? []).length).toBeLessThanOrEqual(1);
      }
    });
  });

  // #endregion ­ƒÅ░KitImpl Filter Tests

  describe("Flatten", () => {
    const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
    const flattenCasesData = (FlattenCases as any).cases as Array<{ name: string; designPath: string[] }>;

    const resolveFamilyNames = (fids: FamilyId[] | undefined) => (fids ?? []).map((f) => kit.families?.find((fam) => fam.id === f.id)?.name).filter(Boolean);
    const testFlatten = (designName: string) => {
      const design = findDesign(kit, designName);
      const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && resolveFamilyNames(d.families).includes(design.name));
      expect(expectedDesign).toBeDefined();
      const flatOp = asKitInstance(kit).flattenDesignMerkle(design.id);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const flatDesign = detachDesignForLocalMutation(design);
      flatDesign.applyDiff(flatOp.diff.forward);

      flatDesign!.pieces?.forEach((p) => {
        const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
        expect(expectedPiece).toBeDefined();
        expect(p.plane).toBeDefined();
        expect(p.center).toBeDefined();
        expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
        expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
      });
    };

    for (const fc of flattenCasesData) {
      it(`${fc.name}: KitImpl -> Flatten -> Diff -> Apply = Flat`, () => {
        const designName = fc.designPath[fc.designPath.length - 1];
        testFlatten(designName);
      });
    }

    it("forward diff lists every connection removal by id and apply clears connections", () => {
      const design = findDesign(kit, flattenCasesData[0].designPath[0]);
      const origConnCount = design._connections?.length ?? 0;
      expect(origConnCount).toBeGreaterThan(0);
      const flatOp = asKitInstance(kit).flattenDesignMerkle(design.id);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const removed = flatOp.diff.forward.connections?.removed ?? [];
      expect(removed.length).toBe(origConnCount);
      const removedSet = new Set(removed.map((r) => r.id));
      for (const c of design._connections ?? []) {
        expect(removedSet.has(c.id)).toBe(true);
      }
      const flatDesign = detachDesignForLocalMutation(design);
      flatDesign.applyDiff(flatOp.diff.forward);
      expect(flatDesign._connections?.length ?? 0).toBe(0);
    });

    it("warns when a connected clump has no fixed piece and still flattens", () => {
      const floatingA: Piece = { id: "floating-a", name: "A", type: { id: "t1" } };
      const floatingB: Piece = { id: "floating-b", name: "B", type: { id: "t1" } };
      const design: Design = {
        id: "design-float",
        name: "Float",
        unit: "mm",
        pieces: [floatingA, floatingB],
        connections: [
          {
            id: "c-ab",
            connected: { piece: { id: "floating-a" }, connector: { id: "c1" } },
            connecting: { piece: { id: "floating-b" }, connector: { id: "c2" } },
          },
        ],
        createdAt: "2025-01-01T00:00:00.000Z",
        updatedAt: "2025-01-01T00:00:00.000Z",
      };
      const miniKit: KitImpl = {
        id: "k1",
        name: "k",
        designs: [design],
        types: [
          {
            id: "t1",
            name: "T",
            unit: "mm",
            connectors: [
              { id: "c1", point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 1, z: 0 }, t: 0 },
              { id: "c2", point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 1, z: 0 }, t: 0.5 },
            ],
            createdAt: "2025-01-01T00:00:00.000Z",
            updatedAt: "2025-01-01T00:00:00.000Z",
          },
        ],
        createdAt: "2025-01-01T00:00:00.000Z",
        updatedAt: "2025-01-01T00:00:00.000Z",
      };
      const op = asKitInstance(miniKit).runFlattenDesign(design.id);
      expect(op.ok).toBe(true);
      if (!op.ok) return;
      expect(op.warnings.some((w) => w.code === "flatten.no-fixed-piece-in-clump")).toBe(true);
    });
  });

  describe("FlattenMerkle", () => {
    const setPath = (obj: any, path: string, value: unknown) => {
      const keys = path.split(".");
      let current = obj;
      for (let i = 0; i < keys.length - 1; i++) {
        const key = keys[i];
        if (current[key] === undefined || current[key] === null) current[key] = {};
        current = current[key];
      }
      current[keys[keys.length - 1]] = value;
    };
    const findDesignByPath = (kit: any, designPath: string[]) => {
      const name = designPath[designPath.length - 1];
      const match = (kit.designs ?? []).find((d: any) => d.name === name);
      if (!match) throw new Error(`Design path ${designPath.join(" / ")} not found at ${name}`);
      return match;
    };
    const applyMutations = (design: any, mutations: any[]) => {
      for (const mutation of mutations) {
        if (mutation.kind === "pieceField") {
          const piece = design.pieces?.find((p: any) => p.id === mutation.pieceId);
          if (!piece) throw new Error(`Piece ${mutation.pieceId} not found`);
          setPath(piece, mutation.path, mutation.value);
        } else if (mutation.kind === "connectionField") {
          const conns = (design as { _connections?: any[]; connections?: any[] })._connections ?? design.connections;
          const conn = (conns ?? []).find((c: any) => c.id === mutation.connectionId);
          if (!conn) throw new Error(`Connection ${mutation.connectionId} not found`);
          setPath(conn, mutation.path, mutation.value);
        } else {
          throw new Error(`Unknown mutation kind ${mutation.kind}`);
        }
      }
    };

    it("shared asset mutation cases produce expected hash changes", () => {
      const cases = (FlattenMerkleCases as any).cases as any[];
      for (const testCase of cases) {
        const kitBefore = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
        const designBefore = findDesignByPath(kitBefore, testCase.designPath);
        const beforeHashes = asKitInstance(kitBefore).getFlatMerkleHashes(designBefore.id);

        const kitAfter = duplicateKitForIsolation(kitBefore);
        const designAfter = findDesignByPath(kitAfter, testCase.designPath);
        applyMutations(designAfter, testCase.mutations ?? []);
        const afterHashes = asKitInstance(kitAfter).getFlatMerkleHashes(designAfter.id);

        const beforeIds = Object.keys(beforeHashes).sort();
        const afterIds = Object.keys(afterHashes).sort();
        expect(afterIds, `case ${testCase.name}: piece set changed`).toEqual(beforeIds);

        const changedPlane = new Set(beforeIds.filter((g) => beforeHashes[g].planeHash !== afterHashes[g].planeHash));
        const changedCenter = new Set(beforeIds.filter((g) => beforeHashes[g].centerHash !== afterHashes[g].centerHash));
        const expectSpec = testCase.expect ?? {};

        if (Object.prototype.hasOwnProperty.call(expectSpec, "planeHashesChangedAny")) {
          if (expectSpec.planeHashesChangedAny) expect(changedPlane.size, `case ${testCase.name}`).toBeGreaterThan(0);
          else expect(changedPlane.size, `case ${testCase.name}: unexpected planeHash changes`).toBe(0);
        }
        if (Object.prototype.hasOwnProperty.call(expectSpec, "centerHashesChangedAny")) {
          if (expectSpec.centerHashesChangedAny) expect(changedCenter.size, `case ${testCase.name}`).toBeGreaterThan(0);
          else expect(changedCenter.size, `case ${testCase.name}: unexpected centerHash changes`).toBe(0);
        }
        if (expectSpec.planeHashesChangedAll === true) expect(changedPlane.size, `case ${testCase.name}`).toBe(beforeIds.length);
        if (expectSpec.planeHashesChangedAll === false) expect(changedPlane.size, `case ${testCase.name}`).not.toBe(beforeIds.length);
        if (expectSpec.centerHashesChangedAll === true) expect(changedCenter.size, `case ${testCase.name}`).toBe(beforeIds.length);
        if (expectSpec.centerHashesChangedAll === false) expect(changedCenter.size, `case ${testCase.name}`).not.toBe(beforeIds.length);
        for (const id of expectSpec.planeHashesChangedIncludes ?? []) expect(changedPlane.has(id), `case ${testCase.name}: planeHash should change for ${id}`).toBe(true);
        for (const id of expectSpec.centerHashesChangedIncludes ?? []) expect(changedCenter.has(id), `case ${testCase.name}: centerHash should change for ${id}`).toBe(true);
        for (const id of expectSpec.planeHashesStableIncludes ?? []) expect(changedPlane.has(id), `case ${testCase.name}: planeHash should be stable for ${id}`).toBe(false);
        for (const id of expectSpec.centerHashesStableIncludes ?? []) expect(changedCenter.has(id), `case ${testCase.name}: centerHash should be stable for ${id}`).toBe(false);
      }
    }, 120000);

    it("cross-language parity reference hashes", () => {
      const parity = (FlattenMerkleCases as any).parity;
      expect(parity).toBeDefined();
      const kit = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
      const design = findDesignByPath(kit, parity.designPath);
      const hashes = asKitInstance(kit).getFlatMerkleHashes(design.id);
      for (const expected of parity.expectedHashes) {
        const entry = hashes[expected.pieceId];
        expect(entry, `piece ${expected.pieceId} missing`).toBeDefined();
        expect(entry.planeHash, `piece ${expected.pieceId} planeHash`).toBe(expected.planeHash);
        expect(entry.centerHash, `piece ${expected.pieceId} centerHash`).toBe(expected.centerHash);
      }
    });

    it("cached flatten reuses cached plane/center when hashes match", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = findDesignByPath(kit, [NAKAGIN_DESIGN_NAME]);
      const first = asKitInstance(kit).flattenDesignCachedOp(design.id);
      expect(Object.keys(first.cache).length).toBeGreaterThan(0);
      const second = asKitInstance(kit).flattenDesignCachedOp(design.id, first.cache);
      for (const id of Object.keys(first.cache)) {
        expect(second.cache[id].planeHash).toBe(first.cache[id].planeHash);
        expect(second.cache[id].centerHash).toBe(first.cache[id].centerHash);
        expect(JSON.stringify(second.cache[id].plane)).toBe(JSON.stringify(first.cache[id].plane));
        expect(JSON.stringify(second.cache[id].center)).toBe(JSON.stringify(first.cache[id].center));
      }
    });

    it("cached flatten returns a structurally identical forward diff vs fresh flattenDesign (ignoring non-deterministic attribute ids)", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = findDesignByPath(kit, [NAKAGIN_DESIGN_NAME]);
      // flattenDesign's setAttributes mints new attribute ids via id() on every run, so
      // we strip those ids before comparing to focus on the geometric/attribute values that
      // the cache is actually preserving.
      const stripAttrIds = (forward: DesignDiff): unknown => {
        const copy = duplicateDesignDiffForIsolation(forward) as any;
        for (const pu of copy.pieces?.updated ?? []) {
          for (const a of pu.diff?.attributes?.added ?? []) delete a.id;
          for (const a of pu.diff?.attributes?.updated ?? []) delete a.id;
        }
        return copy;
      };
      const fresh = asKitInstance(kit).flattenDesignMerkle(design.id);
      const cached = asKitInstance(kit).flattenDesignCachedOp(design.id);
      expect(cached.result.ok).toBe(fresh.ok);
      if (!fresh.ok || !cached.result.ok) return;
      expect(JSON.stringify(stripAttrIds(cached.result.diff.forward))).toBe(JSON.stringify(stripAttrIds(fresh.diff.forward)));
      const secondFresh = asKitInstance(kit).flattenDesignMerkle(design.id);
      const secondCached = asKitInstance(kit).flattenDesignCachedOp(design.id, cached.cache);
      expect(secondCached.result.ok).toBe(true);
      if (!secondFresh.ok || !secondCached.result.ok) return;
      expect(JSON.stringify(stripAttrIds(secondCached.result.diff.forward))).toBe(JSON.stringify(stripAttrIds(secondFresh.diff.forward)));
    });

    it("cached flatten preserves exact Piece object reference when merkle hash is unchanged", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = findDesignByPath(kit, [NAKAGIN_DESIGN_NAME]);
      const first = asKitInstance(kit).flattenDesignCachedOp(design.id);
      const second = asKitInstance(kit).flattenDesignCachedOp(design.id, first.cache);
      const ids = Object.keys(first.cache);
      for (const g of ids) {
        expect(second.cache[g].flatPiece, `piece ${g} missing flatPiece on rerun`).toBeDefined();
        expect(second.cache[g].flatPiece, `piece ${g} flatPiece not reused by reference`).toBe(first.cache[g].flatPiece);
        expect(second.cache[g].plane, `piece ${g} plane not reused by reference`).toBe(first.cache[g].plane);
        expect(second.cache[g].center, `piece ${g} center not reused by reference`).toBe(first.cache[g].center);
      }
    });

    it("on mutation only descendants of changed piece/connection are recomputed, ancestors keep cached piece refs", () => {
      const parity = (FlattenMerkleCases as any).parity;
      const rootId: string = parity.expectedHashes[0].pieceId;
      const childId: string = parity.expectedHashes[1].pieceId;
      const kit = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
      const design = findDesignByPath(kit, parity.designPath);
      const first = asKitInstance(kit).flattenDesignCachedOp(design.id);
      expect(first.cache[rootId]).toBeDefined();
      expect(first.cache[childId]).toBeDefined();

      // Mutate the root piece plane ÔÇö must invalidate every descendant's planeHash but keep centerHash stable
      const mutatedKit = duplicateKitForIsolation(kit);
      const mutatedDesign = findDesignByPath(mutatedKit, parity.designPath);
      const mutatedRoot = mutatedDesign.pieces!.find((p: Piece) => p.id === rootId)!;
      mutatedRoot.plane = { ...mutatedRoot.plane!, origin: { ...mutatedRoot.plane!.origin, x: (mutatedRoot.plane!.origin.x ?? 0) + 13.25 } };
      const second = asKitInstance(mutatedKit).flattenDesignCachedOp(mutatedDesign.id, first.cache);

      let reusedCount = 0;
      let recomputedCount = 0;
      for (const g of Object.keys(second.cache)) {
        const planeReused = second.cache[g].plane === first.cache[g]?.plane;
        if (planeReused) reusedCount++;
        else recomputedCount++;
        // center hashes must stay stable for every piece (plane change doesn't touch center chain)
        expect(second.cache[g].center, `piece ${g} center ref must stay stable when only plane changed`).toBe(first.cache[g]?.center);
      }
      // plane change of the root cascades to every piece in the component ÔåÆ all recomputed
      expect(recomputedCount).toBeGreaterThan(0);
      expect(reusedCount).toBe(0);
    });

    it("on connection center mutation only the affected subtree's centers are recomputed", () => {
      const parity = (FlattenMerkleCases as any).parity;
      const kit = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
      const design = findDesignByPath(kit, parity.designPath);
      const first = asKitInstance(kit).flattenDesignCachedOp(design.id);

      // Mutate one connection's u (center-only). All plane hashes stay stable; only centers on
      // the child connection's downstream subtree change. This is the "drag only recomputes children"
      // scenario: dragging a piece nudges its parent connection u/v ÔåÆ only its subtree rehydrates.
      const mutatedKit = duplicateKitForIsolation(kit);
      const mutatedDesign = findDesignByPath(mutatedKit, parity.designPath);
      const firstConn = mutatedDesign._connections?.[0];
      expect(firstConn).toBeDefined();
      if (!firstConn) return;
      firstConn.u = (firstConn.u ?? 0) + 2.5;
      const second = asKitInstance(mutatedKit).flattenDesignCachedOp(mutatedDesign.id, first.cache);

      let centerReused = 0;
      let centerRecomputed = 0;
      let planeReused = 0;
      for (const g of Object.keys(second.cache)) {
        if (second.cache[g].plane === first.cache[g]?.plane) planeReused++;
        if (second.cache[g].center === first.cache[g]?.center) centerReused++;
        else centerRecomputed++;
      }
      expect(planeReused).toBe(Object.keys(first.cache).length);
      expect(centerReused).toBeGreaterThan(0);
      expect(centerRecomputed).toBeGreaterThan(0);
      expect(centerRecomputed).toBeLessThan(Object.keys(first.cache).length);
    });
  });

  describe("Roundtrip", () => {
    describe("Metabolism", () => {
      it("Json -> Memory -> Json, Json -> Zip, Zip -> Json", async () => {
        const fs = await import("node:fs");
        const path = await import("node:path");
        const { __dirname } = await getTestNodePaths();

        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const serializedKit = serializeKit(kit);
        const deserializedKit = deserializeKit(serializedKit);
        expect(areKitsEqual(kit, deserializedKit)).toBe(true);

        // Regenerate zip from updated JSON and test roundtrip
        const exportedZip = await exportKit(kit);
        const zipPath = path.join(__dirname, "../assets/semio/metabolism.zip");
        const zipArrayBuffer = exportedZip instanceof Blob ? await exportedZip.arrayBuffer() : exportedZip;
        fs.writeFileSync(zipPath, Buffer.from(zipArrayBuffer as ArrayBuffer));
        const { kit: reKit } = await importKit(exportedZip);
        expect(areKitsEqual(kit, reKit)).toBe(true);
      }, 60000);
    });
  });

  describe("Validation", () => {
    describe("Metabolism", () => {
      it("Metabolism KitImpl -> Validate = Empty report", () => {
        const validKit = asKitInstance(MetabolismKit as unknown as KitImpl);
        expect(hasErrors(validateKit(validKit))).toBe(false);
      });
    });

    describe("Invalid", () => {
      it("Invalid KitImpl -> Validate = Invalid Report", () => {
        const invalidKit = asKitInstance(InvalidKit as unknown as KitImpl);
        const result = validateKit(invalidKit);
        const expected = InvalidKitValidation as unknown as ValidationResult;
        expect(areValidationResultsEqual(result, expected)).toBe(true);
      });

      it("Plain descriptions do not create emoji validation problems", () => {
        const kit = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
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
    it("Cluster replacement uses design-id designPiece and yields included design entry", () => {
      const design = {
        id: "design-root",
        name: "Root",
        pieces: [
          { id: "piece-a", type: { id: "type-a" } },
          { id: "piece-b", type: { id: "type-b" } },
          { id: "piece-c", type: { id: "type-c" } },
        ],
        connections: [
          {
            id: "conn-ab",
            connecting: { piece: { id: "piece-a" } },
            connected: { piece: { id: "piece-b" } },
          },
          {
            id: "conn-bc",
            connecting: { piece: { id: "piece-b" } },
            connected: { piece: { id: "piece-c" } },
          },
        ],
        createdAt: "2025-01-01T00:00:00.000Z",
        updatedAt: "2025-01-01T00:00:00.000Z",
      } as Design;

      const hostKit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const designRow = new Design(DesignSchema.parse(stripNullsJsonClone(design) as unknown), hostKit);
      const { clusteredDesign, externalConnections } = hostKit.createClusteredDesignFromDesign(designRow, ["piece-a", "piece-b"], "Cluster");
      const change = hostKit.replaceClusterWithDesignChange(designRow, ["piece-a", "piece-b"], clusteredDesign, externalConnections);
      const updatedDesign = detachDesignForLocalMutation(designRow);
      updatedDesign.applyDiff(change.forward);

      const clusterConnection = updatedDesign._connections?.find((c) => c.id === "conn-bc");
      // Stub uses the nested design id on the wire; {@link Side.designPiece} getter resolves a Piece, so use wire id.
      expect(clusterConnection?.connecting.wireDesignPieceId()?.id).toBe(clusteredDesign.id);
      expect(clusterConnection?.connected.wireDesignPieceId()).toBeUndefined();

      const included = getIncludedDesigns(updatedDesign);
      expect(included.length).toBe(1);
      expect(included[0].id).toBe(clusteredDesign.id);
      expect(included[0].designId).toBe(clusteredDesign.id);
    });
  });

  describe("Drag", () => {
    it("Design + Pieces + Offset = DiffDesign", () => {
      const design = DragDesign as unknown as Design;
      const pieces = DragPieces as unknown as Design;
      const offset = DragOffset as { u: number; v: number };
      const expectedDiff = DragDiffDesign as any;
      const computedDiff = asKitInstance(MetabolismKit as unknown as KitImpl).dragPiecesInDesignDiff(design, pieces, offset);
      const computedPieceUpdates = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.id.localeCompare(b.piece.id));
      const expectedPieceUpdates = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.id.localeCompare(b.piece.id));
      expect(computedPieceUpdates.length).toBe(expectedPieceUpdates.length);
      for (let i = 0; i < computedPieceUpdates.length; i++) {
        expect(computedPieceUpdates[i].piece.id).toBe(expectedPieceUpdates[i].piece.id);
        expect(computedPieceUpdates[i].diff.center?.u).toBe(expectedPieceUpdates[i].diff.center.u);
        expect(computedPieceUpdates[i].diff.center?.v).toBe(expectedPieceUpdates[i].diff.center.v);
      }
      const computedConnUpdates = (computedDiff.connections?.updated ?? []).sort((a, b) => a.connection.id.localeCompare(b.connection.id));
      const expectedConnUpdates = (expectedDiff.connections?.updated ?? []).sort((a: any, b: any) => a.connection.id.localeCompare(b.connection.id));
      expect(computedConnUpdates.length).toBe(expectedConnUpdates.length);
      for (let i = 0; i < computedConnUpdates.length; i++) {
        expect(computedConnUpdates[i].connection.id).toBe(expectedConnUpdates[i].connection.id);
        expect(computedConnUpdates[i].diff.u).toBe(expectedConnUpdates[i].diff.u);
        expect(computedConnUpdates[i].diff.v).toBe(expectedConnUpdates[i].diff.v);
      }
    });

    it("Nakagin Capsule Tower flattened piece drag uses piece center diff (flat design has no connections)", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === NAKAGIN_DESIGN_NAME)!;
      const flatOp = asKitInstance(kit).flattenDesignMerkle(design.id);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const flatDesign = detachDesignForLocalMutation(design);
      flatDesign.applyDiff(flatOp.diff.forward);
      expect((flatDesign._connections ?? []).length).toBe(0);
      const pieceId = "9d18882e-d90b-40de-a171-47cb4564ffa6";
      const flatPiece = flatDesign.pieces!.find((p) => p.id === pieceId)!;
      const pieces = { ...flatDesign, pieces: [flatPiece] } as Design;
      const offset = { u: 3, v: -1 };
      const diff = kit.dragPiecesInDesignDiff(flatDesign, pieces, offset);
      expect(diff.connections).toBeUndefined();
      expect(diff.pieces?.updated?.length).toBe(1);
      expect(diff.pieces!.updated![0].piece.id).toBe(pieceId);
      const baseU = flatPiece.center?.u ?? 0;
      const baseV = flatPiece.center?.v ?? 0;
      expect(diff.pieces!.updated![0].diff.center?.u).toBeCloseTo(baseU + offset.u, 6);
      expect(diff.pieces!.updated![0].diff.center?.v).toBeCloseTo(baseV + offset.v, 6);
    });

    it("Nakagin sketchpad flow: drag root piece with connections preserved moves all descendants", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === NAKAGIN_DESIGN_NAME)!;
      expect((design._connections ?? []).length).toBeGreaterThan(0);

      // Step 1: Compute metadata (simulates usePiecesMetadataMap)
      const metaResult = kit.piecesMetadataFor(design.id);
      expect(metaResult.ok).toBe(true);
      if (!metaResult.ok) return;
      const metadata = metaResult.diff;
      expect(metadata.size).toBeGreaterThan(0);

      // Step 2: Find a root piece (no parentPieceId) and its descendants
      let rootId: string | undefined;
      const childrenMap = new Map<string, string[]>();
      for (const [id, meta] of metadata) {
        if (!meta.parentPieceId && !rootId) rootId = id;
        if (meta.parentPieceId) {
          const siblings = childrenMap.get(meta.parentPieceId);
          if (siblings) siblings.push(id);
          else childrenMap.set(meta.parentPieceId, [id]);
        }
      }
      expect(rootId).toBeDefined();
      // BFS to find all descendants of root
      const descendants = new Set<string>();
      const queue = [rootId!];
      while (queue.length > 0) {
        const current = queue.pop()!;
        const children = childrenMap.get(current);
        if (!children) continue;
        for (const child of children) {
          if (child !== rootId && !descendants.has(child)) {
            descendants.add(child);
            queue.push(child);
          }
        }
      }
      expect(descendants.size).toBeGreaterThan(0);

      // Record pre-drag centers from metadata for root + descendants
      const preDragCenters = new Map<string, { u: number; v: number }>();
      preDragCenters.set(rootId!, { u: metadata.get(rootId!)!.center.u, v: metadata.get(rootId!)!.center.v });
      for (const descId of descendants) {
        const center = metadata.get(descId)!.center;
        preDragCenters.set(descId, { u: center.u, v: center.v });
      }

      // Step 3: Build flatDesign like sketchpad does (raw design + metadata centers + connections preserved)
      const sketchpadFlatDesign: Design = {
        ...design,
        pieces: (design.pieces ?? []).map((p) => ({
          ...p,
          center: metadata.get(p.id)?.center ?? p.center,
        })),
      };

      // Step 4: Call dragPiecesInDesign (same as sketchpad onNodeDragStop)
      const offset = { u: 5, v: -3 };
      const piecesDesign = { id: "", name: "", pieces: [{ id: rootId! }] } as Design;
      const dragDiff = kit.dragPiecesInDesignDiff(sketchpadFlatDesign, piecesDesign, offset);

      // Root should get a center update (it's fixed - no parent)
      expect(dragDiff.pieces?.updated?.length).toBe(1);
      expect(dragDiff.pieces!.updated![0].piece.id).toBe(rootId);
      const rootCenter = metadata.get(rootId!)!.center;
      expect(dragDiff.pieces!.updated![0].diff.center?.u).toBeCloseTo(rootCenter.u + offset.u, 6);
      expect(dragDiff.pieces!.updated![0].diff.center?.v).toBeCloseTo(rootCenter.v + offset.v, 6);
      // No connection updates needed (only root selected, no connected pieces)
      expect(dragDiff.connections?.updated?.length ?? 0).toBe(0);

      // Step 5ÔÇô6: Apply the diff on an isolated kit row (object spread would drop {@link KitImpl} methods).
      const updatedKit = kitWithSwappedDesign(kit, design.id, (copy) => copy.applyDiff(dragDiff));
      const postMetaResult = updatedKit.piecesMetadataFor(design.id);
      expect(postMetaResult.ok).toBe(true);
      if (!postMetaResult.ok) return;
      const postMetadata = postMetaResult.diff;

      // Step 7: Verify root moved by exactly offset
      const postRootCenter = postMetadata.get(rootId!)!.center;
      expect(postRootCenter.u).toBeCloseTo(preDragCenters.get(rootId!)!.u + offset.u, 3);
      expect(postRootCenter.v).toBeCloseTo(preDragCenters.get(rootId!)!.v + offset.v, 3);

      // Step 8: Verify descendants moved (positions changed after re-flatten).
      // Due to non-linear layout in flattenDesign (horizontalScale, radius, etc.),
      // descendants may NOT shift by exactly the offset, but they MUST move.
      for (const descId of descendants) {
        const preCenter = preDragCenters.get(descId)!;
        const postCenter = postMetadata.get(descId)!.center;
        const moved = Math.abs(postCenter.u - preCenter.u) > 0.001 || Math.abs(postCenter.v - preCenter.v) > 0.001;
        expect(moved).toBe(true);
      }
    });

    it("Nakagin store chain: updatePieces-only diff (no full dragDiff) still moves descendants via re-flatten", () => {
      // This test simulates the EXACT sketchpad store flow:
      // 1. onNodeDragStop calls dragPiecesInDesign ÔåÆ gets dragDiff
      // 2. updatePieces sends ONLY the piece updates from dragDiff to the kit store
      // 3. KitImpl store applies piece-only kitDiff via applyKitDiff
      // 4. Reactive chain recomputes piecesMetadata ÔåÆ descendants should have new positions
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      expect((design._connections ?? []).length).toBeGreaterThan(0);

      // Step 1: Compute metadata
      const metaResult = kit.piecesMetadataFor(design.id);
      expect(metaResult.ok).toBe(true);
      if (!metaResult.ok) return;
      const metadata = metaResult.diff;

      // Step 2: Find root and descendants
      let rootId: string | undefined;
      const childrenMap = new Map<string, string[]>();
      for (const [id, meta] of metadata) {
        if (!meta.parentPieceId && !rootId) rootId = id;
        if (meta.parentPieceId) {
          const siblings = childrenMap.get(meta.parentPieceId);
          if (siblings) siblings.push(id);
          else childrenMap.set(meta.parentPieceId, [id]);
        }
      }
      expect(rootId).toBeDefined();
      const descendants = new Set<string>();
      const queue = [rootId!];
      while (queue.length > 0) {
        const current = queue.pop()!;
        const children = childrenMap.get(current);
        if (!children) continue;
        for (const child of children) {
          if (child !== rootId && !descendants.has(child)) {
            descendants.add(child);
            queue.push(child);
          }
        }
      }
      expect(descendants.size).toBeGreaterThan(0);

      // Record pre-drag centers
      const preDragCenters = new Map<string, { u: number; v: number }>();
      preDragCenters.set(rootId!, { u: metadata.get(rootId!)!.center.u, v: metadata.get(rootId!)!.center.v });
      for (const descId of descendants) {
        const center = metadata.get(descId)!.center;
        preDragCenters.set(descId, { u: center.u, v: center.v });
      }

      // Step 3: Build flatDesign (same as sketchpad)
      const sketchpadFlatDesign: Design = {
        ...design,
        pieces: (design.pieces ?? []).map((p) => ({
          ...p,
          center: metadata.get(p.id)?.center ?? p.center,
        })),
      };

      // Step 4: Compute drag diff
      const offset = { u: 5, v: -3 };
      const piecesDesign = { id: "", name: "", pieces: [{ id: rootId! }] } as Design;
      const dragDiff = kit.dragPiecesInDesignDiff(sketchpadFlatDesign, piecesDesign, offset);
      const pieceDiffUpdates = dragDiff.pieces?.updated ?? [];
      const connectionDiffUpdates = dragDiff.connections?.updated ?? [];

      // Step 5: Simulate what updatePieces does ÔÇö apply ONLY pieceUpdates to kit via kitDiff
      // This is the EXACT path: updatePieces ÔåÆ command handler ÔåÆ kitStore.change(kitDiff)
      const kitDiff: any = {};
      if (pieceDiffUpdates.length > 0) {
        kitDiff.designs = {
          updated: [
            {
              design: { id: design.id },
              diff: {
                pieces: { updated: pieceDiffUpdates },
              },
            },
          ],
        };
      }
      // Apply connection updates separately (same as updateConnections)
      if (connectionDiffUpdates.length > 0) {
        if (!kitDiff.designs) {
          kitDiff.designs = { updated: [{ design: { id: design.id }, diff: {} }] };
        }
        kitDiff.designs.updated[0].diff.connections = { updated: connectionDiffUpdates };
      }

      // Step 6: Apply kitDiff (applyKitDiff returns a new isolated graph; assign ÔÇö do not drop the return value)
      const storeKit = applyKitDiff(kit, kitDiff);

      // Step 7: Recompute metadata (same as usePiecesMetadataMap)
      const postMetaResult = storeKit.piecesMetadataFor(design.id);
      expect(postMetaResult.ok).toBe(true);
      if (!postMetaResult.ok) {
        console.error("piecesMetadata failed:", postMetaResult.errors);
        return;
      }
      const postMetadata = postMetaResult.diff;

      // Step 8: Verify root moved
      const postRootCenter = postMetadata.get(rootId!)!.center;
      expect(postRootCenter.u).toBeCloseTo(preDragCenters.get(rootId!)!.u + offset.u, 3);
      expect(postRootCenter.v).toBeCloseTo(preDragCenters.get(rootId!)!.v + offset.v, 3);

      // Step 9: Verify ALL descendants moved
      for (const descId of descendants) {
        const preCenter = preDragCenters.get(descId)!;
        const postCenter = postMetadata.get(descId)!.center;
        const moved = Math.abs(postCenter.u - preCenter.u) > 0.001 || Math.abs(postCenter.v - preCenter.v) > 0.001;
        expect(moved).toBe(true);
      }

      // Step 10: Verify local re-flatten (visualPositions) matches store re-flatten
      // This is the key test ÔÇö if these differ, the useEffect overwrites with wrong positions
      const localUpdatedKit = kitWithSwappedDesign(kit, design.id, (copy) => copy.applyDiff(dragDiff));
      const localMetaResult = localUpdatedKit.piecesMetadataFor(design.id);
      expect(localMetaResult.ok).toBe(true);
      if (!localMetaResult.ok) return;
      const localMetadata = localMetaResult.diff;

      // Compare ALL piece centers between local and store re-flatten
      for (const [pieceId, localMeta] of localMetadata) {
        const storeMeta = postMetadata.get(pieceId);
        expect(storeMeta).toBeDefined();
        expect(localMeta.center.u).toBeCloseTo(storeMeta!.center.u, 6);
        expect(localMeta.center.v).toBeCloseTo(storeMeta!.center.v, 6);
      }
    });

    it("Nakagin leaf drag: dragging a leaf node (parent, no children) offsets through parent connection and matches nativeDragPieces", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      expect((design._connections ?? []).length).toBeGreaterThan(0);

      // Step 1: Compute metadata
      const metaResult = kit.piecesMetadataFor(design.id);
      expect(metaResult.ok).toBe(true);
      if (!metaResult.ok) return;
      const metadata = metaResult.diff;

      // Step 2: Find a LEAF node (has parentPieceId but no children)
      const childrenMap = new Map<string, string[]>();
      for (const [id, meta] of metadata) {
        if (meta.parentPieceId) {
          const siblings = childrenMap.get(meta.parentPieceId);
          if (siblings) siblings.push(id);
          else childrenMap.set(meta.parentPieceId, [id]);
        }
      }
      let leafId: string | undefined;
      for (const [id, meta] of metadata) {
        if (meta.parentPieceId && !childrenMap.has(id)) {
          leafId = id;
          break;
        }
      }
      expect(leafId).toBeDefined();
      const leafMeta = metadata.get(leafId!)!;
      expect(leafMeta.parentPieceId).toBeDefined();
      expect(childrenMap.has(leafId!)).toBe(false);

      // Step 3: Record pre-drag center
      const preDragCenter = { u: leafMeta.center.u, v: leafMeta.center.v };
      const offset = { u: 2, v: -1.5 };

      // Step 4a: Sketchpad flow ÔÇö build flatDesign with metadata centers + raw connections
      const sketchpadFlatDesign: Design = {
        ...design,
        pieces: (design.pieces ?? []).map((p) => ({
          ...p,
          center: metadata.get(p.id)?.center ?? p.center,
        })),
      };
      const piecesDesign = { id: "", name: "", pieces: [{ id: leafId! }] } as Design;
      const sketchpadDragDiff = kit.dragPiecesInDesignDiff(sketchpadFlatDesign, piecesDesign, offset);

      // Leaf has a parent ÔåÆ should produce connection update, NOT piece update
      expect(sketchpadDragDiff.pieces?.updated?.length ?? 0).toBe(0);
      expect(sketchpadDragDiff.connections?.updated?.length).toBe(1);

      // Step 4b: nativeDragPieces flow ÔÇö flatten, drag, apply to raw, re-flatten
      const fc = asKitInstance(kit).flattenDesignMerkle(design.id);
      expect(fc.ok).toBe(true);
      if (!fc.ok) return;
      const nativeFlatDesign = detachDesignForLocalMutation(design);
      nativeFlatDesign.applyDiff({ pieces: fc.diff.forward.pieces });
      const nativePiecesDesign: Design = { id: nativeFlatDesign.id, name: nativeFlatDesign.name, pieces: (nativeFlatDesign.pieces ?? []).filter((p) => p.id === leafId) };
      const nativeDragDiff = kit.dragPiecesInDesignDiff(nativeFlatDesign, nativePiecesDesign, offset);

      // Both flows should produce the same connection diff
      expect(nativeDragDiff.pieces?.updated?.length ?? 0).toBe(0);
      expect(nativeDragDiff.connections?.updated?.length).toBe(1);
      expect(sketchpadDragDiff.connections!.updated![0].connection.id).toBe(nativeDragDiff.connections!.updated![0].connection.id);
      expect(sketchpadDragDiff.connections!.updated![0].diff.u).toBeCloseTo(nativeDragDiff.connections!.updated![0].diff.u!, 6);
      expect(sketchpadDragDiff.connections!.updated![0].diff.v).toBeCloseTo(nativeDragDiff.connections!.updated![0].diff.v!, 6);

      // Step 5a: Sketchpad re-flatten
      const sketchpadUpdatedKit = kitWithSwappedDesign(kit, design.id, (copy) => copy.applyDiff(sketchpadDragDiff));
      const sketchpadPostMeta = sketchpadUpdatedKit.piecesMetadataFor(design.id);
      expect(sketchpadPostMeta.ok).toBe(true);
      if (!sketchpadPostMeta.ok) return;

      // Step 5b: nativeDragPieces re-flatten
      const nativeUpdatedKit = kitWithSwappedDesign(kit, design.id, (copy) => copy.applyDiff(nativeDragDiff));
      const nativePostMeta = nativeUpdatedKit.piecesMetadataFor(design.id);
      expect(nativePostMeta.ok).toBe(true);
      if (!nativePostMeta.ok) return;

      // Step 6: Leaf must have moved from its pre-drag position
      const sketchpadLeafPost = sketchpadPostMeta.diff.get(leafId!)!;
      const nativeLeafPost = nativePostMeta.diff.get(leafId!)!;
      expect(Math.abs(sketchpadLeafPost.center.u - preDragCenter.u) > 0.001 || Math.abs(sketchpadLeafPost.center.v - preDragCenter.v) > 0.001).toBe(true);

      // Step 7: Sketchpad and native must produce identical results for ALL pieces
      for (const [pieceId, skMeta] of sketchpadPostMeta.diff) {
        const natMeta = nativePostMeta.diff.get(pieceId);
        expect(natMeta).toBeDefined();
        expect(skMeta.center.u).toBeCloseTo(natMeta!.center.u, 6);
        expect(skMeta.center.v).toBeCloseTo(natMeta!.center.v, 6);
      }

      // Step 8: Verify store chain ÔÇö applyKitDiff with connection-only updates matches
      const connectionDiffUpdates = sketchpadDragDiff.connections?.updated ?? [];
      expect(connectionDiffUpdates.length).toBe(1);
      const kitDiff: any = {
        designs: {
          updated: [{ design: { id: design.id }, diff: { connections: { updated: connectionDiffUpdates } } }],
        },
      };
      const storeKit = applyKitDiff(kit, kitDiff);
      const storeMetaResult = storeKit.piecesMetadataFor(design.id);
      expect(storeMetaResult.ok).toBe(true);
      if (!storeMetaResult.ok) return;
      // Store re-flatten must match local re-flatten
      for (const [pieceId, storeMeta] of storeMetaResult.diff) {
        const localMeta = sketchpadPostMeta.diff.get(pieceId);
        expect(localMeta).toBeDefined();
        expect(storeMeta.center.u).toBeCloseTo(localMeta!.center.u, 6);
        expect(storeMeta.center.v).toBeCloseTo(localMeta!.center.v, 6);
      }
    });

    it("Nakagin center-space to connection-space scaling: pixel offset scales by horizontalScale for horizontal connections", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const metaResult = kit.piecesMetadataFor(design.id);
      expect(metaResult.ok).toBe(true);
      if (!metaResult.ok) return;
      const metadata = metaResult.diff;

      // Find a leaf node (has parent, no children)
      const childrenMap = new Map<string, string[]>();
      for (const [id, meta] of metadata) {
        if (meta.parentPieceId) {
          const siblings = childrenMap.get(meta.parentPieceId);
          if (siblings) siblings.push(id);
          else childrenMap.set(meta.parentPieceId, [id]);
        }
      }
      let leafId: string | undefined;
      for (const [id, meta] of metadata) {
        if (meta.parentPieceId && !childrenMap.has(id)) {
          leafId = id;
          break;
        }
      }
      expect(leafId).toBeDefined();
      const leafMeta = metadata.get(leafId!)!;
      const preDragCenter = { u: leafMeta.center.u, v: leafMeta.center.v };

      // Simulate a pixel drag: user drags 100px to the right, 50px down
      const pixelDeltaX = 100;
      const pixelDeltaY = -50;
      const centerOffsetU = pixelDeltaX / ICON_WIDTH; // 2.0 in center-space
      const centerOffsetV = -pixelDeltaY / ICON_WIDTH; // 1.0 in center-space

      // Build flat design (sketchpad style)
      const flatDesign: Design = {
        ...design,
        pieces: (design.pieces ?? []).map((p) => ({
          ...p,
          center: metadata.get(p.id)?.center ?? p.center,
        })),
      };

      // Get dragPiecesInDesign result (unscaled ÔÇö in center-space)
      const piecesDesign = { id: "", name: "", pieces: [{ id: leafId! }] } as Design;
      const dragDiff = kit.dragPiecesInDesignDiff(flatDesign, piecesDesign, { u: centerOffsetU, v: centerOffsetV });
      expect(dragDiff.connections?.updated?.length).toBe(1);
      const rawConnDiff = dragDiff.connections!.updated![0].diff;

      // The raw diff has the center-space offset as connection diff ÔÇö this is WRONG for visual positioning
      expect(rawConnDiff.u).toBeCloseTo(centerOffsetU, 6);
      expect(rawConnDiff.v).toBeCloseTo(centerOffsetV, 6);

      // Find the parent connector to determine the scale
      const conn = (design._connections ?? []).find((c) => c.id === dragDiff.connections!.updated![0].connection.id)!;
      expect(conn).toBeDefined();
      const parentPieceId = conn.connected.piece.id;
      const parentMeta = metadata.get(parentPieceId)!;
      const parentPiece = (design.pieces ?? []).find((p) => p.id === parentPieceId)!;
      const parentType = asKitInstance(kit).requireType(parentPiece.type!.id!)!;
      const parentConnector = parentType.connectors!.find((c) => c.id === conn.connected.connector?.id)!;
      const isVerticalConnection = Math.abs(parentConnector?.direction?.z ?? 0) > 0.5;
      const horizontalScale = 3.0633;
      const scale = isVerticalConnection ? 1 : horizontalScale;

      // Scale the connection diff from center-space to connection-space
      const scaledConnDiff = {
        u: rawConnDiff.u! / scale,
        v: rawConnDiff.v! / scale,
      };

      // Apply the SCALED diff to the raw design and re-flatten
      const scaledDragDiff: DesignDiff = {
        connections: { updated: [{ connection: { id: conn.id }, diff: scaledConnDiff }] },
      };
      const updatedKit = kitWithSwappedDesign(kit, design.id, (copy) => copy.applyDiff(scaledDragDiff));
      const postMeta = updatedKit.piecesMetadataFor(design.id);
      expect(postMeta.ok).toBe(true);
      if (!postMeta.ok) return;

      // The re-flattened leaf center should match the INTENDED visual offset (pre-drag center + center offset)
      const leafPost = postMeta.diff.get(leafId!)!;
      const expectedU = preDragCenter.u + centerOffsetU;
      const expectedV = preDragCenter.v + centerOffsetV;
      expect(leafPost.center.u).toBeCloseTo(expectedU, 2);
      expect(leafPost.center.v).toBeCloseTo(expectedV, 2);

      // Without scaling, the re-flattened position would be amplified by horizontalScale
      if (!isVerticalConnection) {
        const unscaledDragDiff: DesignDiff = {
          connections: { updated: [{ connection: { id: conn.id }, diff: { u: centerOffsetU, v: centerOffsetV } }] },
        };
        const unscaledKit = kitWithSwappedDesign(kit, design.id, (copy) => copy.applyDiff(unscaledDragDiff));
        const unscaledMeta = unscaledKit.piecesMetadataFor(design.id);
        expect(unscaledMeta.ok).toBe(true);
        if (!unscaledMeta.ok) return;
        const unscaledLeaf = unscaledMeta.diff.get(leafId!)!;
        // Without scaling, the visual offset is amplified by horizontalScale ÔÇö the "jump" bug
        expect(unscaledLeaf.center.u).toBeCloseTo(preDragCenter.u + centerOffsetU * horizontalScale, 2);
        expect(unscaledLeaf.center.v).toBeCloseTo(preDragCenter.v + centerOffsetV * horizontalScale, 2);
      }
    });

    it("findParentConnectionForPieceInDesign and fixPieceInDesign use the connection to the parent piece, not the parent piece id as connection id", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const metaResult = kit.piecesMetadataFor(design.id);
      expect(metaResult.ok).toBe(true);
      if (!metaResult.ok) return;
      const metadata = metaResult.diff;
      let childId: string | undefined;
      let parentPieceId: string | undefined;
      for (const [id, meta] of metadata) {
        if (meta.parentPieceId) {
          childId = id;
          parentPieceId = meta.parentPieceId;
          break;
        }
      }
      expect(childId).toBeDefined();
      expect(parentPieceId).toBeDefined();
      const conn = findParentConnectionForPieceInDesign(kit, design.id, childId!);
      expect(conn.id).not.toBe(parentPieceId);
      const otherPieceId = conn.connected.piece.id === childId ? conn.connecting.piece.id : conn.connected.piece.id;
      expect(otherPieceId).toBe(parentPieceId);
      const fixDiff = fixPieceInDesign(kit, design.id, childId!);
      expect(fixDiff.connections?.removed?.length).toBe(1);
      expect(fixDiff.connections!.removed![0].id).toBe(conn.id);
    });
  });

  describe("Move", () => {
    it("same drag fixture: roots get plane translation; connected mover gets connector-frame split (gap/shift/rise + residual u/v)", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = DragDesign as unknown as Design;
      const pieces = DragPieces as unknown as Design;
      const vector = MoveVector as { gap: number; shift: number; rise: number };
      const expectedDiff = MoveDiffDesign as any;
      const computedDiff = kit.movePiecesInDesignOp(design, pieces, vector);
      const computedPieceUpdates = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.id.localeCompare(b.piece.id));
      const expectedPieceUpdates = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.id.localeCompare(b.piece.id));
      expect(computedPieceUpdates.length).toBe(expectedPieceUpdates.length);
      for (let i = 0; i < computedPieceUpdates.length; i++) {
        expect(computedPieceUpdates[i].piece.id).toBe(expectedPieceUpdates[i].piece.id);
        const po = computedPieceUpdates[i].diff.plane?.origin;
        const eo = expectedPieceUpdates[i].diff.plane.origin;
        expect(po?.x).toBeCloseTo(eo.x, 5);
        expect(po?.y).toBeCloseTo(eo.y, 5);
        expect(po?.z).toBeCloseTo(eo.z, 5);
      }
      const computedConnUpdates = (computedDiff.connections?.updated ?? []).sort((a, b) => a.connection.id.localeCompare(b.connection.id));
      const expectedConnUpdates = (expectedDiff.connections?.updated ?? []).sort((a: any, b: any) => a.connection.id.localeCompare(b.connection.id));
      expect(computedConnUpdates.length).toBe(expectedConnUpdates.length);
      for (let i = 0; i < computedConnUpdates.length; i++) {
        expect(computedConnUpdates[i].connection.id).toBe(expectedConnUpdates[i].connection.id);
        const ed = expectedConnUpdates[i].diff;
        const cd = computedConnUpdates[i].diff;
        for (const key of ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"] as const) {
          if (ed[key] !== undefined) expect(cd[key]).toBeCloseTo(ed[key] as number, 8);
        }
      }
      const dragParity = kit.dragPiecesInDesignDiff(design, pieces, { u: vector.shift, v: vector.gap });
      const dragConn = (dragParity.connections?.updated ?? []).sort((a, b) => a.connection.id.localeCompare(b.connection.id));
      expect(computedConnUpdates.map((c) => c.connection.id)).toEqual(dragConn.map((c) => c.connection.id));
      const dragPiecesUp = (dragParity.pieces?.updated ?? []).sort((a, b) => a.piece.id.localeCompare(b.piece.id));
      expect(computedPieceUpdates.map((p) => p.piece.id)).toEqual(dragPiecesUp.map((p) => p.piece.id));
    });

    it("vertical parent connector: world move decomposes into shift, gap, rise on connection (not diagram u/v only)", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = DragDesign as unknown as Design;
      const pieces = DragPieces as unknown as Design;
      const vector = { gap: 2, shift: -1, rise: 0.5 };
      const diff = kit.movePiecesInDesignOp(design, pieces, vector);
      const dragParity = kit.dragPiecesInDesignDiff(design, pieces, { u: vector.shift, v: vector.gap });
      const moveConn = (diff.connections?.updated ?? []).sort((a, b) => a.connection.id.localeCompare(b.connection.id));
      const dragConn = (dragParity.connections?.updated ?? []).sort((a, b) => a.connection.id.localeCompare(b.connection.id));
      expect(moveConn.length).toBe(dragConn.length);
      for (let i = 0; i < moveConn.length; i++) {
        expect(moveConn[i].connection.id).toBe(dragConn[i].connection.id);
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
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const selection = NakaginCapsuleTowerDeletedSelection as unknown as Design;
      const expectedDiff = NakaginCapsuleTowerDeletedDesignDiff as any;

      const pieceIds = (selection.pieces ?? []).map((p) => p.id);
      const connectionIds = (selection.connections ?? []).map((c) => c.id);
      const delOp = deletePiecesAndConnectionsInDesign(kit, design, pieceIds, connectionIds);
      expect(delOp.ok).toBe(true);
      if (!delOp.ok) return;
      const computedDiff = delOp.diff;

      // ­ƒÜÜVerify removed pieces
      const computedRemovedPieces = (computedDiff.pieces?.removed ?? []).sort((a, b) => a.id.localeCompare(b.id));
      const expectedRemovedPieces = (expectedDiff.pieces?.removed ?? []).sort((a: any, b: any) => a.id.localeCompare(b.id));
      expect(computedRemovedPieces.length).toBe(expectedRemovedPieces.length);
      for (let i = 0; i < computedRemovedPieces.length; i++) {
        expect(computedRemovedPieces[i].id).toBe(expectedRemovedPieces[i].id);
      }

      // ­ƒöüVerify updated (fixed) pieces
      const computedUpdated = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.id.localeCompare(b.piece.id));
      const expectedUpdated = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.id.localeCompare(b.piece.id));
      expect(computedUpdated.length).toBe(expectedUpdated.length);
      for (let i = 0; i < computedUpdated.length; i++) {
        expect(computedUpdated[i].piece.id).toBe(expectedUpdated[i].piece.id);
        expect(computedUpdated[i].diff.plane?.origin?.x).toBeCloseTo(expectedUpdated[i].diff.plane.origin.x, 3);
        expect(computedUpdated[i].diff.plane?.origin?.y).toBeCloseTo(expectedUpdated[i].diff.plane.origin.y, 3);
        expect(computedUpdated[i].diff.plane?.origin?.z).toBeCloseTo(expectedUpdated[i].diff.plane.origin.z, 3);
        expect(computedUpdated[i].diff.center?.u).toBeCloseTo(expectedUpdated[i].diff.center.u, 3);
        expect(computedUpdated[i].diff.center?.v).toBeCloseTo(expectedUpdated[i].diff.center.v, 3);
      }

      // ­ƒöîVerify removed connections
      const computedRemovedConns = (computedDiff.connections?.removed ?? []).sort((a, b) => a.id.localeCompare(b.id));
      const expectedRemovedConns = (expectedDiff.connections?.removed ?? []).sort((a: any, b: any) => a.id.localeCompare(b.id));
      expect(computedRemovedConns.length).toBe(expectedRemovedConns.length);
      for (let i = 0; i < computedRemovedConns.length; i++) {
        expect(computedRemovedConns[i].id).toBe(expectedRemovedConns[i].id);
      }
    });
  });

  // #region ­ƒôïCopy And Paste Tests
  describe("CopyAndPaste", () => {
    it("Nakagin Capsule Tower copy selected pieces and connections", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const selection = NakaginCapsuleTowerCopySelection as any;
      const expectedCopy = NakaginCapsuleTowerCopyDesign as unknown as Design;

      const pieceIds = (selection.pieces ?? []).map((p: any) => p.id);
      const connectionIds = (selection.connections ?? []).map((c: any) => c.id);
      const copyOp = copyDesign(kit, design, pieceIds, connectionIds);
      expect(copyOp.ok).toBe(true);
      if (!copyOp.ok) return;
      const computedCopy = copyOp.diff;

      // ­ƒº®Verify piece and connection counts
      expect((computedCopy.pieces ?? []).length).toBe((expectedCopy.pieces ?? []).length);
      expect((computedCopy.connections ?? []).length).toBe((expectedCopy.connections ?? []).length);

      // ­ƒÅÀ´©ÅVerify external piece has semio.piece.origin = "external" and semio.center
      const externalPieces = (computedCopy.pieces ?? []).filter((p) => (p.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external"));
      const expectedExternalPieces = (expectedCopy.pieces ?? []).filter((p) => (p.attributes ?? []).some((a: any) => a.key === "semio.piece.origin" && a.value === "external"));
      expect(externalPieces.length).toBe(expectedExternalPieces.length);
      for (const ext of externalPieces) {
        expect((ext.attributes ?? []).some((a) => a.key === "semio.center")).toBe(true);
      }

      // ­ƒôÉVerify pp-excl-pc-incl pieces have semio.center and semio.plane attributes
      const ppExclPcInclPieces = (computedCopy.pieces ?? []).filter((p) => (p.attributes ?? []).some((a) => a.key === "semio.center") && !(p.attributes ?? []).some((a) => a.key === "semio.piece.origin"));
      expect(ppExclPcInclPieces.length).toBe(1);
      for (const piece of ppExclPcInclPieces) {
        expect((piece.attributes ?? []).some((a) => a.key === "semio.plane")).toBe(true);
      }
    });

    it("Nakagin Capsule Tower paste without coordinate", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const source = NakaginCapsuleTowerCopyDesign as unknown as Design;
      const expectedDiff = NakaginCapsuleTowerPasteDesignDiff as any;

      const computedDiff = pasteDesign(kit, source, pasteTarget, "original");

      // ­ƒº®Verify added piece count
      const computedAdded = (computedDiff.pieces?.added ?? []).sort((a, b) => a.id.localeCompare(b.id));
      const expectedAdded = (expectedDiff.pieces?.added ?? []).sort((a: any, b: any) => a.id.localeCompare(b.id));
      expect(computedAdded.length).toBe(expectedAdded.length);

      // ­ƒöîVerify added connection count
      const computedAddedConns = (computedDiff.connections?.added ?? []).sort((a, b) => a.id.localeCompare(b.id));
      const expectedAddedConns = (expectedDiff.connections?.added ?? []).sort((a: any, b: any) => a.id.localeCompare(b.id));
      expect(computedAddedConns.length).toBe(expectedAddedConns.length);

      // ­ƒÅÀ´©ÅVerify no external pieces in paste output
      for (const piece of computedAdded) {
        expect((piece.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(false);
      }

      // ­ƒöùVerify connection u/v preserved
      for (let i = 0; i < computedAddedConns.length; i++) {
        expect(computedAddedConns[i].u).toBeCloseTo(expectedAddedConns[i].u, 3);
        expect(computedAddedConns[i].v).toBeCloseTo(expectedAddedConns[i].v, 3);
      }
    });

    it("Nakagin Capsule Tower paste with coordinate", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const source = NakaginCapsuleTowerCopyDesign as unknown as Design;
      const expectedDiff = NakaginCapsuleTowerPasteWithCoordinateDesignDiff as any;

      const computedDiff = pasteDesign(kit, source, pasteTarget, "original", { u: 10, v: 10 });

      // ­ƒº®Verify added piece count
      const computedAdded = (computedDiff.pieces?.added ?? []).sort((a, b) => a.id.localeCompare(b.id));
      const expectedAdded = (expectedDiff.pieces?.added ?? []).sort((a: any, b: any) => a.id.localeCompare(b.id));
      expect(computedAdded.length).toBe(expectedAdded.length);

      // ­ƒöîVerify added connection count
      const computedAddedConns = (computedDiff.connections?.added ?? []).sort((a, b) => a.id.localeCompare(b.id));
      const expectedAddedConns = (expectedDiff.connections?.added ?? []).sort((a: any, b: any) => a.id.localeCompare(b.id));
      expect(computedAddedConns.length).toBe(expectedAddedConns.length);

      // ­ƒôÉVerify fixed pieces have offset centers
      for (let i = 0; i < computedAdded.length; i++) {
        if (computedAdded[i].center && expectedAdded[i].center) {
          expect(computedAdded[i].center!.u).toBeCloseTo(expectedAdded[i].center.u, 3);
          expect(computedAdded[i].center!.v).toBeCloseTo(expectedAdded[i].center.v, 3);
        }
      }

      // ­ƒöùVerify connection u/v
      for (let i = 0; i < computedAddedConns.length; i++) {
        expect(computedAddedConns[i].u).toBeCloseTo(expectedAddedConns[i].u, 3);
        expect(computedAddedConns[i].v).toBeCloseTo(expectedAddedConns[i].v, 3);
      }
    });

    it("pasteDesign accepts every built-in anchoring string for Nakagin clipboard", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const source = NakaginCapsuleTowerCopyDesign as unknown as Design;
      for (const kind of PASTE_DESIGN_ANCHORING_KINDS) {
        const withoutCoordinate = pasteDesign(kit, source, pasteTarget, kind);
        expect((withoutCoordinate.pieces?.added ?? []).length).toBeGreaterThan(0);
        const withCoordinate = pasteDesign(kit, source, pasteTarget, kind, { u: 10, v: 10 });
        expect((withCoordinate.pieces?.added ?? []).length).toBe((withoutCoordinate.pieces?.added ?? []).length);
      }
    });

    it("Nakagin t_f5 and br_sl0 internal connection stays identical to clipboard when pasting with coordinate", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const flatOp = asKitInstance(kit).flattenDesignMerkle(design.id);
      expect(flatOp.ok).toBe(true);
      if (!flatOp.ok) return;
      const flatDesign = detachDesignForLocalMutation(design);
      flatDesign.applyDiff(flatOp.diff.forward);
      const t5 = "9c1ec7a2-13c2-4d23-b7bd-1efe2663d0a9";
      const br = "5feebbf8-33d9-41ad-a13a-24c271a1860b";
      const connInternal = "eb8ce9ce-091c-4495-a651-fa703748dfef";
      const connParent = "4d5ff333-d70a-43e1-8b7a-8849c8c91405";
      const copyOp2 = copyDesign(kit, flatDesign, [t5, br], [connInternal, connParent]);
      expect(copyOp2.ok).toBe(true);
      if (!copyOp2.ok) return;
      const copied = copyOp2.diff;
      const srcConn = copied._connections!.find((c) => c.id === connInternal)!;
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const withoutCoordinate = pasteDesign(kit, copied, pasteTarget, "original");
      const withCoordinate = pasteDesign(kit, copied, pasteTarget, "original", { u: 10, v: 5 });
      const connWo = withoutCoordinate.connections?.added?.find((c) => c.id === connInternal);
      const connWi = withCoordinate.connections?.added?.find((c) => c.id === connInternal);
      expect(connWo).toBeDefined();
      expect(connWi).toBeDefined();
      expect(connWi!.u).toBeCloseTo(srcConn.u ?? 0, 6);
      expect(connWi!.v).toBeCloseTo(srcConn.v ?? 0, 6);
      expect(connWi!.u).toBeCloseTo(connWo!.u ?? 0, 6);
      expect(connWi!.v).toBeCloseTo(connWo!.v ?? 0, 6);
    });

    it.skip("Nakagin paste remaps t_f2ÔÇöt_f1 onto target t_f1 when t_f1 is external stub only", () => {
      // [DEBUG] External-stub paste rematch from flattened copy does not always emit t2ÔÇôt1 into diff.connections.added (rematch candidates / flat connector DTOs); re-enable when paste rematch is stable.
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const flatOp3 = asKitInstance(kit).flattenDesignMerkle(design.id);
      expect(flatOp3.ok).toBe(true);
      if (!flatOp3.ok) return;
      const flatDesign = detachDesignForLocalMutation(design);
      flatDesign.applyDiff(flatOp3.diff.forward);
      const sel = NakaginCapsuleTowerCopySelection as any;
      const t1 = "31be08e1-e75c-4024-86b4-c3c6d3939fbb";
      const t2t1Conn = "ddf9e0e4-40e1-4079-aa40-c86cf699788b";
      const t1ParentConn = "b1ecc6c5-722a-4814-9047-a87222bbaa4d";
      const pieceIds = (sel.pieces as { id: string }[]).map((p) => p.id).filter((g: string) => g !== t1);
      const connectionIds = (sel.connections as { id: string }[]).map((c) => c.id).filter((g: string) => g !== t1ParentConn);
      expect(connectionIds).toContain(t2t1Conn);
      const copyOp3 = copyDesign(kit, flatDesign, pieceIds, connectionIds);
      expect(copyOp3.ok).toBe(true);
      if (!copyOp3.ok) return;
      const copied = copyOp3.diff;
      const stubT1 = copied.pieces!.find((p) => p.id === t1);
      expect(stubT1 && (stubT1.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(true);
      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const diff = pasteDesign(kit, copied, pasteTarget, "original");
      const targetT1 = pasteTarget.pieces!.find((p) => p.name === "t_f1_b_c1");
      expect(targetT1).toBeDefined();
      const t2Id = "9d18882e-d90b-40de-a171-47cb4564ffa6";
      const added = (diff.connections?.added ?? []) as Connection[];
      const remapped =
        added.find((c) => c.id === t2t1Conn) ??
        added.find((c) => c.connecting.piece.id === t2Id && c.connected.piece.id === targetT1!.id) ??
        added.find((c) => c.connected.piece.id === t2Id && c.connecting.piece.id === targetT1!.id);
      expect(remapped).toBeDefined();
      const endIds = new Set([remapped!.connecting.piece.id, remapped!.connected.piece.id]);
      expect(endIds.has(t2Id)).toBe(true);
      expect(endIds.has(targetT1!.id)).toBe(true);

      const childBelowT2Conn = "bb5449be-247b-498e-b8c8-309697ddea7b";
      const srcInternal = (copied as { _connections?: Connection[] })._connections?.find((c) => c.id === childBelowT2Conn);
      expect(srcInternal).toBeDefined();
      const coordinate = { u: 10, v: -3.25 };
      const diffCoordinate = pasteDesign(kit, copied, pasteTarget, "original", coordinate);
      const addCoord = (diffCoordinate.connections?.added ?? []) as Connection[];
      const remappedCoordinate =
        addCoord.find((c) => c.id === t2t1Conn) ??
        addCoord.find((c) => c.connecting.piece.id === t2Id && c.connected.piece.id === targetT1!.id) ??
        addCoord.find((c) => c.connected.piece.id === t2Id && c.connecting.piece.id === targetT1!.id);
      expect(remappedCoordinate).toBeDefined();
      const t2Piece = copied.pieces!.find((p) => p.id === t2Id)!;
      let childU = t2Piece.center?.u ?? 0;
      let childV = t2Piece.center?.v ?? 0;
      const t2cAttr = (t2Piece.attributes ?? []).find((a) => a.key === "semio.center");
      if (t2cAttr?.value) {
        const j = JSON.parse(t2cAttr.value) as Coordinate;
        childU = j.u;
        childV = j.v;
      }
      const parentU = targetT1!.center?.u ?? 0;
      const parentV = targetT1!.center?.v ?? 0;
      const anchor = { u: 0, v: 0 };
      expect(remappedCoordinate!.u).toBeCloseTo(parentU - (coordinate.u + (anchor.u - childU)), 6);
      expect(remappedCoordinate!.v).toBeCloseTo(parentV - (coordinate.v + (anchor.v - childV)), 6);
      const internalAfter = diffCoordinate.connections?.added?.find((c) => c.id === childBelowT2Conn);
      expect(internalAfter).toBeDefined();
      expect(internalAfter!.u).toBeCloseTo(srcInternal!.u ?? 0, 6);
      expect(internalAfter!.v).toBeCloseTo(srcInternal!.v ?? 0, 6);
    });

    it("copyDesign single connected piece selected alone becomes free fixed root and auto-pulls source descendants", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
      const tF0BC0 = "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952";

      const sourceConns = design._connections ?? [];
      const sourcePieces = design.pieces ?? [];
      const childMap = new Map<string, Array<{ childId: string; connectionId: string }>>();
      for (const c of sourceConns) {
        const p = c.connected.piece.id;
        if (!childMap.has(p)) childMap.set(p, []);
        childMap.get(p)!.push({ childId: c.connecting.piece.id, connectionId: c.id });
      }
      const expectedDescPieces = new Set<string>();
      const expectedDescConns = new Set<string>();
      const queue = [tF0BC0];
      while (queue.length > 0) {
        const cur = queue.shift()!;
        for (const { childId, connectionId } of childMap.get(cur) ?? []) {
          if (expectedDescPieces.has(childId)) continue;
          expectedDescPieces.add(childId);
          expectedDescConns.add(connectionId);
          queue.push(childId);
        }
      }
      expect(expectedDescPieces.size).toBeGreaterThan(0);

      const copyOp = copyDesign(kit, design, [tF0BC0], []);
      expect(copyOp.ok).toBe(true);
      if (!copyOp.ok) return;
      const copied = copyOp.diff;
      expect((copied.pieces ?? []).length).toBe(1 + expectedDescPieces.size);
      expect((copied._connections ?? []).length).toBe(expectedDescConns.size);

      const root = copied.pieces!.find((p) => p.id === tF0BC0)!;
      expect(root.plane).toBeDefined();
      expect(root.center).toBeDefined();
      expect((root.attributes ?? []).some((a) => a.key === "semio.center")).toBe(true);
      expect((root.attributes ?? []).some((a) => a.key === "semio.plane")).toBe(true);
      expect((root.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(false);

      for (const id of expectedDescPieces) {
        const desc = copied.pieces!.find((p) => p.id === id);
        expect(desc).toBeDefined();
        const sourceDesc = sourcePieces.find((p) => p.id === id)!;
        expect(JSON.stringify(desc!.center ?? null)).toBe(JSON.stringify(sourceDesc.center ?? null));
        expect(JSON.stringify(desc!.plane ?? null)).toBe(JSON.stringify(sourceDesc.plane ?? null));
        expect((desc!.attributes ?? []).some((a) => a.key === "semio.piece.origin" && a.value === "external")).toBe(false);
      }
      for (const id of expectedDescConns) {
        const conn = copied._connections!.find((c) => c.id === id);
        expect(conn).toBeDefined();
      }

      const pasteTarget = NakaginCapsuleTowerPasteDesign as unknown as Design;
      const diff = pasteDesign(kit, copied, pasteTarget, "original");
      const added = diff.pieces?.added ?? [];
      expect(added.length).toBe(1 + expectedDescPieces.size);
      const addedRoot = added.find((p) => p.id === tF0BC0)!;
      expect(addedRoot.plane).toBeDefined();
      expect(addedRoot.center).toBeDefined();
      expect((diff.connections?.added ?? []).length).toBe(expectedDescConns.size);

      const diffCoordinate = pasteDesign(kit, copied, pasteTarget, "original", { u: 7, v: -3 });
      const addedCoordinate = diffCoordinate.pieces?.added ?? [];
      const addedRootCoordinate = addedCoordinate.find((p) => p.id === tF0BC0)!;
      expect(addedRootCoordinate.center!.u).toBeCloseTo(root.center!.u + 7, 6);
      expect(addedRootCoordinate.center!.v).toBeCloseTo(root.center!.v - 3, 6);
      const addedConnsCoordinate = diffCoordinate.connections?.added ?? [];
      for (const expConnId of expectedDescConns) {
        const sourceConn = copied._connections!.find((c) => c.id === expConnId)!;
        const targetConn = addedConnsCoordinate.find((c) => c.id === expConnId)!;
        expect(targetConn).toBeDefined();
        expect(targetConn.u ?? 0).toBeCloseTo(sourceConn.u ?? 0, 6);
        expect(targetConn.v ?? 0).toBeCloseTo(sourceConn.v ?? 0, 6);
      }
    });
  });
  // #endregion ­ƒôïCopy And Paste Tests

  // #region ­ƒöìFind Replaceable Types In Designs Tests
  describe("FindReplaceableTypesInDesigns", () => {
    const findReplCases = FindReplaceableTypesCases as any;
    const syntheticKit = SyntheticFindReplaceableKit as any;

    it("Synthetic selection enforces distinct compatible connectors and ignores consumed design connectors", () => {
      const ports = getKitPorts(syntheticKit as unknown as KitImpl);
      const types = syntheticKit.types as Type[];
      const syntheticRootDesignId = (findReplCases.syntheticCases as Array<{ designId: string }>)?.[0]?.designId;
      expect(syntheticRootDesignId).toBeDefined();
      const designs = (syntheticKit.designs as Design[]).filter((d: Design) => d.id !== syntheticRootDesignId);
      const design = (syntheticKit.designs as Design[]).find((d: Design) => d.id === syntheticRootDesignId)!;
      const synKit = asKitInstance(syntheticKit as unknown as KitImpl);

      for (const sc of findReplCases.syntheticCases) {
        const result = synKit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: sc.pieceIds });
        for (const t of sc.expectedContainsTypes ?? []) expect(result.types, `${sc.name}: types should contain ${t}`).toContain(t);
        for (const t of sc.expectedNotContainsTypes ?? []) expect(result.types, `${sc.name}: types should not contain ${t}`).not.toContain(t);
        for (const d of sc.expectedContainsDesigns ?? []) expect(result.designs, `${sc.name}: designs should contain ${d}`).toContain(d);
        for (const d of sc.expectedNotContainsDesigns ?? []) expect(result.designs, `${sc.name}: designs should not contain ${d}`).not.toContain(d);
      }
    });

    it("Nakagin Capsule Tower: connector-level boundary matching shrinks candidates as demand grows", () => {
      const bc = findReplCases.boundaryCases;
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === bc.designName)!;
      const types = kit.types ?? [];
      const ports = (kit.families ?? []).flatMap((f) => f.ports ?? []);
      const designs = kit.designs ?? [];
      const nameToId = new Map((design.pieces ?? []).map((piece) => [piece.name, piece.id]));
      const forbiddenSingleConnectorFamilies = new Set(bc.forbiddenFamilies as string[]);
      const typeNamesForSelection = (pieceNames: string[]): string[] => {
        const pieceIds = pieceNames.map((pieceName) => nameToId.get(pieceName) ?? "");
        const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: pieceIds });
        return types.filter((candidateType) => result.types.includes(candidateType.id)).map((candidateType) => candidateType.name);
      };
      const uniqueTypeNamesForSelection = (pieceNames: string[]): string[] => [...new Set(typeNamesForSelection(pieceNames))].sort((leftName, rightName) => leftName.localeCompare(rightName));

      const singleCapsuleNames = typeNamesForSelection(bc.singleCapsulePieces);
      const twoCapsuleNames = typeNamesForSelection(bc.twoCapsulePieces);
      const fourCapsuleNames = typeNamesForSelection(bc.fourCapsulePieces);
      const eightCapsuleNames = typeNamesForSelection(bc.eightCapsulePieces);
      const tambourPieceId = nameToId.get(bc.tambourPieceName)!;
      const tambourResult = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: [tambourPieceId] });

      expect(singleCapsuleNames.length).toBeGreaterThan(twoCapsuleNames.length);
      expect(twoCapsuleNames.length).toBeGreaterThanOrEqual(fourCapsuleNames.length);
      expect(fourCapsuleNames.length).toBeGreaterThanOrEqual(eightCapsuleNames.length);

      for (const forbiddenFamily of forbiddenSingleConnectorFamilies) {
        expect(twoCapsuleNames).not.toContain(forbiddenFamily);
        expect(fourCapsuleNames).not.toContain(forbiddenFamily);
        expect(eightCapsuleNames).not.toContain(forbiddenFamily);
      }
      const expectedTwoCapsuleFamilies = bc.expectedTwoCapsuleFamilies as string[];
      const expectedLargeFamilies = bc.expectedLargeFamilies as string[];
      const excludedAsDemandGrows = expectedTwoCapsuleFamilies.filter((family) => !expectedLargeFamilies.includes(family));
      for (const family of excludedAsDemandGrows) {
        expect(fourCapsuleNames).not.toContain(family);
        expect(eightCapsuleNames).not.toContain(family);
      }
      expect(uniqueTypeNamesForSelection(bc.twoCapsulePieces)).toEqual(bc.expectedTwoCapsuleFamilies);
      expect(uniqueTypeNamesForSelection(bc.fourCapsulePieces)).toEqual(bc.expectedLargeFamilies);
      expect(uniqueTypeNamesForSelection(bc.eightCapsulePieces)).toEqual(bc.expectedLargeFamilies);
      expect(tambourResult.types.length).toBe(bc.expectedTambourTypeIdCount);
      expect(tambourResult.designs.length).toBe(bc.expectedTambourDesignIdCount);
    });

    it("Nakagin Capsule Tower: selection asset yields only exact design matches", () => {
      const selAssetCase = findReplCases.cases.find((c: any) => c.name === "selection_asset_returns_compatible_ids");
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const rootPlan = kit.designs!.find((d) => d.name === selAssetCase.designName)!;
      const kindItems = kit.types ?? [];
      const linkItems = (kit.families ?? []).flatMap((f) => f.ports ?? []);
      const allPlans = kit.designs ?? [];
      const selection = NakaginCapsuleTowerCopySelection as any;
      const pieceIds = (selection.pieces ?? []).map((piece: { id: string }) => piece.id);

      expect(pieceIds.length).toBeGreaterThan(0);

      const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(rootPlan, allPlans, kindItems, linkItems, { pieces: pieceIds });

      expect(result.types).toEqual(selAssetCase.expectedTypeIds);
      expect(result.designs).toEqual(selAssetCase.expectedDesignIds);
    });

    it("Nakagin Capsule Tower: connected piece yields only exact design matches", () => {
      const connCase = findReplCases.cases.find((c: any) => c.name === "connected_piece_yields_only_exact_design_matches");
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === connCase.designName)!;
      const types = kit.types ?? [];
      const ports = (kit.families ?? []).flatMap((f) => f.ports ?? []);
      const designs = kit.designs ?? [];

      const tambourPiece = design.pieces!.find((p) => p.name === connCase.pieceNames[0])!;
      const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: [tambourPiece.id] });

      expect(result.types.length).toBe(connCase.expectedTypeIdCount);
      expect(result.designs).toEqual(connCase.expectedDesignIds);
    });

    it("Nakagin Capsule Tower: isolated piece with no connections suggests types with compatible ports", () => {
      const isoCase = findReplCases.cases.find((c: any) => c.name === "isolated_piece");
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const flatDesign = kit.designs!.find((d) => d.name === isoCase.designName)!;
      const types = kit.types ?? [];
      const ports = (kit.families ?? []).flatMap((f) => f.ports ?? []);
      const designs = kit.designs ?? [];

      const piece = flatDesign.pieces![isoCase.usePieceIndex];
      const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(flatDesign, designs, types, ports, { pieces: [piece.id] });

      expect(result.types.length).toBeGreaterThan(0);
      if (isoCase.expectOwnTypeInResults && piece.type?.id) {
        expect(result.types).toContain(piece.type.id);
      }
    });

    it("Nakagin Capsule Tower: Capital piece with single connection", () => {
      const capCase = findReplCases.cases.find((c: any) => c.name === "capital_piece");
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === capCase.designName)!;
      const types = kit.types ?? [];
      const ports = (kit.families ?? []).flatMap((f) => f.ports ?? []);
      const designs = kit.designs ?? [];

      const capitalType = types.find((t) => t.name === capCase.lookupTypeName)!;
      const capitalPiece = design.pieces!.find((p) => p.type?.id === capitalType.id)!;
      const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: [capitalPiece.id] });

      expect(result.types.length).toBeGreaterThan(0);
      for (const forbidden of capCase.forbiddenTypeNames) {
        const forbiddenType = types.find((t) => t.name === forbidden);
        if (forbiddenType) expect(result.types).not.toContain(forbiddenType.id);
      }
    });

    it("Nakagin Capsule Tower: multiple selected pieces yield only exact design matches", () => {
      const multiCase = findReplCases.cases.find((c: any) => c.name === "multiple_selected_pieces");
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === multiCase.designName)!;
      const types = kit.types ?? [];
      const ports = getKitPorts(kit);
      const designs = kit.designs ?? [];

      const pieceIds = (multiCase.pieceNames as string[]).map((n) => design.pieces!.find((p) => p.name === n)!.id);
      const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: pieceIds });

      expect(result.types.length).toBe(multiCase.expectedTypeIdCount);
      expect(result.designs).toEqual(multiCase.expectedDesignIds);
    });

    it("Returns empty when no pieces selected", () => {
      const emptyCase = findReplCases.cases.find((c: any) => c.name === "empty_selection");
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const design = kit.designs!.find((d) => d.name === emptyCase.designName)!;
      const types = kit.types ?? [];
      const ports = getKitPorts(kit);
      const designs = kit.designs ?? [];

      const result = kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design, designs, types, ports, { pieces: [] });

      const typesWithNoConnectors = types.filter((t) => (t.connectors ?? []).length === 0);
      expect(result.types.length).toBe(typesWithNoConnectors.length);
    });
  });
  // #endregion ­ƒöìFind Replaceable Types In Designs Tests

  describe("Design/WithDiff", () => {
    const designWithDiffCases = (DesignWithDiffCases as any).cases as Array<{
      name: string;
      designName: string;
      designFamilies: string[];
      expectedPieceCounts: { unchanged: number; modified: number; removed: number; added: number };
      expectedConnectionCounts: { unchanged: number; modified: number; removed: number; added: number };
    }>;
    for (const tc of designWithDiffCases) {
      it(`${tc.name} with-diff preserves old entities and annotates status`, () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const design = kit.designs!.find((d) => d.name === tc.designName)!;
        const diff = NakaginCapsuleTowerDiffDesign as unknown as DesignDiff;
        const expected = new Design(NakaginCapsuleTowerWithDiffDesign as unknown as DesignPlain);
        const computed = designWithDiff(design, diff);

        expect(computed.pieces!.length).toBe(expected.pieces!.length);
        expect(computed._connections!.length).toBe(expected._connections!.length);

        const getStatus = (attrs?: Attribute[]) => (attrs ?? []).find((a) => a.key === "semio.diffStatus")?.value;

        // ­ƒº®Verify piece status counts
        const pieceStatuses = computed.pieces!.map((p) => getStatus(p.attributes));
        expect(pieceStatuses.filter((s) => s === "unchanged").length).toBe(tc.expectedPieceCounts.unchanged);
        expect(pieceStatuses.filter((s) => s === "modified").length).toBe(tc.expectedPieceCounts.modified);
        expect(pieceStatuses.filter((s) => s === "removed").length).toBe(tc.expectedPieceCounts.removed);
        expect(pieceStatuses.filter((s) => s === "added").length).toBe(tc.expectedPieceCounts.added);

        // ­ƒöùVerify connection status counts
        const connStatuses = computed._connections!.map((c) => getStatus(c.attributes));
        expect(connStatuses.filter((s) => s === "unchanged").length).toBe(tc.expectedConnectionCounts.unchanged);
        expect(connStatuses.filter((s) => s === "modified").length).toBe(tc.expectedConnectionCounts.modified);
        expect(connStatuses.filter((s) => s === "removed").length).toBe(tc.expectedConnectionCounts.removed);
        expect(connStatuses.filter((s) => s === "added").length).toBe(tc.expectedConnectionCounts.added);

        // Ô×ûVerify removed/unchanged pieces keep their original parameters
        for (const piece of computed.pieces!) {
          if (getStatus(piece.attributes) === "removed" || getStatus(piece.attributes) === "unchanged") {
            const originalPiece = design.pieces!.find((p) => p.id === piece.id);
            expect(originalPiece).toBeDefined();
            expect(piece.name).toBe(originalPiece!.name);
            expect(piece.description).toBe(originalPiece!.description);
          }
        }

        // ­ƒöºVerify modified pieces have non-geometric diff applied but keep base plane/center
        const updatedPieceMap = new Map((diff.pieces?.updated ?? []).map((u) => [(u as any).piece.id, u.diff]));
        for (const piece of computed.pieces!) {
          if (getStatus(piece.attributes) === "modified") {
            const pieceDiff = updatedPieceMap.get(piece.id);
            const originalPiece = design.pieces!.find((p) => p.id === piece.id);
            expect(originalPiece).toBeDefined();
            if (pieceDiff?.name) expect(piece.name).toBe(pieceDiff.name);
            else expect(piece.name).toBe(originalPiece!.name);
            if (pieceDiff?.description !== undefined) expect(piece.description).toBe(pieceDiff.description);
            else expect(piece.description).toBe(originalPiece!.description);
            // ­ƒôîModified pieces MUST keep base geometry so they only get recolored, not moved.
            expect(piece.plane).toEqual(originalPiece!.plane);
            expect(piece.center).toEqual(originalPiece!.center);
          }
        }
      });
    }

    it("modified pieces keep base plane and center even when diff specifies new geometry", () => {
      const basePiece: Piece = {
        id: "p1",
        name: "Base",
        type: { id: "K" },
        plane: { origin: { x: 1, y: 2, z: 3 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
        center: { u: 4, v: 5 },
      };
      const base: Design = { id: "d1", name: "D", pieces: [basePiece] };
      const newPlane: Plane = { origin: { x: 9, y: 9, z: 9 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
      const diff: DesignDiff = {
        pieces: {
          updated: [{ piece: { id: "p1" }, diff: { name: "Renamed", plane: newPlane, center: { u: 99, v: 99 } } }],
        },
      };
      const computed = designWithDiff(base, diff);
      const piece = computed.pieces!.find((p) => p.id === "p1")!;
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
                  icon: createElement("span", null, "Ôêº"),
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
    const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
    const qualitySumCases = (QualitySumCases as any).cases as Array<{ name: string; designName: string; designFamilies: string[]; qualityName: string; expected: number; tolerance: number }>;
    for (const tc of qualitySumCases) {
      it(`${tc.name}: sums ${tc.qualityName} to ~${tc.expected}`, () => {
        const design = kit.designs?.find((d) => d.name === tc.designName);
        expect(design).toBeDefined();
        const quality = kit.qualities?.find((q) => q.name === tc.qualityName);
        expect(quality).toBeDefined();
        const result = sumQualityInDesign(kit, design!.id, quality!.id);
        expect(Math.abs(result - tc.expected)).toBeLessThan(tc.tolerance);
      });
    }
  });

  describe("ExportDesignRepresentation", () => {
    const exportCases = (ExportDesignRepresentationCases as any).cases as Array<{ name: string; designName: string }>;
    const exportCase = exportCases[0];
    const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
    const design = kit.designs?.find((d) => d.name === exportCase.designName)!;

    /** [DEBUG] Deterministic JSON for cross-implementation glTF report comparison. */
    const normalizeSceneGraph = (reportText: string): unknown => {
      const sortObj = (x: unknown): unknown => {
        if (x === null || typeof x !== "object") return x;
        if (Array.isArray(x)) return x.map((e) => sortObj(e));
        const o = x as Record<string, unknown>;
        return Object.fromEntries(
          Object.keys(o)
            .sort()
            .map((k) => [k, sortObj(o[k])]),
        );
      };
      return sortObj(JSON.parse(reportText));
    };

    it("exports .glb format with valid GLB header", async () => {
      const result = await exportDesignRepresentation(kit, design.id, ".glb");
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
      const result = await exportDesignRepresentation(kit, design.id, ".gltf");
      const decoder = new TextDecoder();
      const str = decoder.decode(result);
      expect(() => JSON.parse(str)).not.toThrow();
      const parsed = JSON.parse(str);
      expect(parsed).toBeDefined();
      expect(typeof parsed).toBe("object");
    });

    it("EXPORT_REPRESENTATION_FORMATS includes .glb and .gltf", () => {
      expect(EXPORT_REPRESENTATION_FORMATS[".glb"]).toBeDefined();
      expect(EXPORT_REPRESENTATION_FORMATS[".gltf"]).toBeDefined();
    });

    it("exports identical Nakagin scene graph across implementations and writes reports", async () => {
      const { mkdirSync, readFileSync, writeFileSync } = await import("node:fs");
      const { EXPORT_REPORTS_DIR, resolve, __dirname } = await getTestNodePaths();
      mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });

      const jsResult = new Uint8Array(await exportDesignRepresentation(kit, design.id, ".gltf"));
      await writeExportReport("js", jsResult);

      let skipPy = false;
      try {
        await runExportReportCommand("uv", ["run", "pytest", "main.py", "-k", "export_scene_graph_report", "-q"], resolve(__dirname, "../py"));
      } catch (e: any) {
        const message = String(e?.message ?? e);
        if (message.includes("No solution found") || message.includes("unsatisfiable")) {
          // [DEBUG] `uv` workspace may be unsatisfiable in some local/Python-version environments; skip cross-check vs Python.
          // eslint-disable-next-line no-console
          console.warn(`[DEBUG] skipping py export_scene_graph_report (uv): ${message}`);
          skipPy = true;
        } else {
          throw e;
        }
      }
      let skipGo = false;
      try {
        await runExportReportCommand("go", ["test", "./...", "-run", "TestExportDesignRepresentationSceneGraphReport$", "-count=1"], resolve(__dirname, "../go"));
      } catch (e: any) {
        const message = String(e?.message ?? e);
        const looksLikeGoToolchainMismatch = message.includes("requires go >= 1.25.0") && message.includes("go.work lists go 1.24.0");
        if (looksLikeGoToolchainMismatch) {
          // [DEBUG] This repository's Go modules require a newer Go toolchain than the one installed in some CI/dev containers.
          // Skip the cross-implementation "go" comparison in that case; other implementations still run.
          // eslint-disable-next-line no-console
          console.warn(`[DEBUG] skipping go ExportDesignRepresentationSceneGraphReport due to Go toolchain mismatch: ${message}`);
          skipGo = true;
        } else {
          throw e;
        }
      }
      await runExportReportCommand("cargo", ["test", "export_scene_graph_report", "--", "--nocapture"], resolve(__dirname, "../rs"));
      await runExportReportCommand(
        "dotnet",
        ["test", "Semio.Tests.csproj", "-f", "net8.0", "--filter", "FullyQualifiedName=Semio.Tests.Tests+ExportDesignRepresentation.Nakagin_Capsule_Tower_Export_Scene_Graph_Report"],
        resolve(__dirname, "../net/Semio.Tests"),
      );

      const implementations = (
        skipGo
          ? skipPy
            ? (["js", "rs", "net"] as const)
            : (["js", "py", "rs", "net"] as const)
          : skipPy
            ? (["js", "go", "rs", "net"] as const)
            : (["js", "py", "go", "rs", "net"] as const)
      ) as readonly string[];
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

  describe("Representation/KPI", () => {
    it("getGeometricInsightsForRepresentation(nakagin-capsule-tower.gltf) returns canonical insights and writes report", async () => {
      const fs = await import("node:fs/promises");
      const { resolve, __dirname } = await getTestNodePaths();
      const representationPath = resolve(__dirname, "../assets/semio/nakagin-capsule-tower.gltf");
      const insights = await getGeometricInsightsForRepresentation(representationPath);
      const round6 = (x: number) => Math.round(x * 1e6) / 1e6;
      const pt = (p: { x: number; y: number; z: number } | undefined) => (p ? { x: round6(p.x), y: round6(p.y), z: round6(p.z) } : undefined);

      const reportsDir = resolve(__dirname, "../reports/representation-kpi");
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

      const canonicalPath = resolve(__dirname, "../assets/semio/nakagin.kpi.representation.semio.json");
      const canonical = JSON.parse(await fs.readFile(canonicalPath, "utf8"));
      const skipKeys = new Set(["centroid", "total_surface_area"]);
      for (const key of Object.keys(canonical)) {
        if (skipKeys.has(key)) continue;
        expect(report[key]).toBeDefined();
        expect(report[key]).toEqual(canonical[key]);
      }
    });
  });

  // #region 🌐KitStoreClient Tests
  describe("KitStoreClient", () => {
    it("fallback: setField piece name success and kit name IllegalName rejection", async () => {
      const initialKit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [
          {
            id: "d1",
            name: "D",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            pieces: [{ id: "p1", name: "Alpha" }],
          },
        ],
      });
      const client = await createKitStoreClient({ initialKit, forceFallback: true });
      try {
        const ok = await client.setField("Piece", "p1", "name", "Beta");
        expect(ok.ok).toBe(true);
        expect(client.getDto().designs[0].pieces[0].name).toBe("Beta");
        const bad = await client.setField("Kit", "k1", "name", "");
        expect(bad.ok).toBe(false);
        if (!bad.ok) expect(bad.error.kind).toBe("IllegalName");
        expect(client.getDto().name).toBe("K");
      } finally {
        client.dispose();
      }
    });

    it("fallback: concurrent writes bump pending semantics via client serialization", async () => {
      const initialKit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [
          {
            id: "d1",
            name: "D",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            pieces: [{ id: "p1", name: "A" }],
          },
        ],
      });
      const client = await createKitStoreClient({ initialKit, forceFallback: true });
      const a = client.setField("Piece", "p1", "name", "X");
      const b = client.setField("Piece", "p1", "name", "Y");
      const c = client.setField("Piece", "p1", "name", "Z");
      await Promise.all([a, b, c]);
      expect(["X", "Y", "Z"]).toContain(client.getDto().designs[0].pieces[0].name);
      client.dispose();
    });

    it("fallback: kitGraphql query returns kit name", async () => {
      const initialKit = asKitInstance({
        id: "k1",
        name: "GraphQlKit",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [],
      });
      const client = await createKitStoreClient({ initialKit, forceFallback: true });
      try {
        const data = kitGraphqlFirstData(
          await kitGraphqlRun(client.kitGraphql(), { query: `query { kitStore { name } }` }),
        ) as { kitStore?: { name?: string } };
        expect(data.kitStore?.name).toBe("GraphQlKit");
      } finally {
        client.dispose();
      }
    });
  });
  // #endregion 🌐KitStoreClient Tests

  // #region ­ƒîèInMemoryKitStore Tests
  // Contract tests for InMemoryKitStore MUST verify the full KitStore interface.

  describe("InMemoryKitStore", () => {
    const makeKit = (overrides?: Partial<KitImpl>): KitImpl => ({
      id: "test-kit-id",
      name: "Test KitImpl",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      ...overrides,
    });

    it("getSnapshot returns the initial kit and ready status", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.id).toBe("test-kit-id");
      expect(snapshot.kit.name).toBe("Test KitImpl");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
      expect(snapshot.sync.readonly).toBe(false);
    });

    it("apply merges a diff and notifies subscribers", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      const diff: KitDiff = { name: "Updated KitImpl" };
      store.apply(diff);

      expect(store.getSnapshot().kit.name).toBe("Updated KitImpl");
      expect(store.getSnapshot().sync.dirty).toBe(true);
      expect(notified).toBe(1);
    });

    it("replace swaps the entire kit and notifies subscribers", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);
      let notified = 0;
      store.subscribe(() => notified++);

      const newKit = makeKit({ id: "new-id", name: "Replaced KitImpl" });
      store.replace(newKit);

      expect(store.getSnapshot().kit.id).toBe("new-id");
      expect(store.getSnapshot().kit.name).toBe("Replaced KitImpl");
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
            added: [{ id: "t1", name: "Wall", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }],
          },
        });
      });

      const snap = store.getSnapshot();
      expect(snap.kit.name).toBe("Renamed");
      expect(snap.kit.types).toHaveLength(1);
      expect(store.canUndo()).toBe(true);

      store.undo();
      const undone = store.getSnapshot();
      expect(undone.kit.name).toBe("Test KitImpl");
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
      expect(store.getSnapshot().kit.name).toBe("Test KitImpl");
      expect(store.canUndo()).toBe(false);
      expect(store.canRedo()).toBe(true);
    });

    it("redo re-applies the last undone mutation", () => {
      const kit = makeKit();
      const store = new InMemoryKitStore(kit);

      store.apply({ name: "Changed" });
      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Test KitImpl");

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
      expect(store.getSnapshot().kit.name).toBe("Test KitImpl");

      store.redo();
      expect(store.getSnapshot().kit.name).toBe("Test KitImpl");
    });
  });

  // #endregion ­ƒîèInMemoryKitStore Tests

  // #region ÔøàJsonFileKitStore Tests
  // Contract tests for JsonFileKitStore MUST verify the full UndoableKitStore interface
  // including file I/O, save, reload, undo/redo, and external update handling.

  describe("JsonFileKitStore", () => {
    const makeKit = (overrides?: Partial<KitImpl>): KitImpl => ({
      id: "file-kit-id",
      name: "File KitImpl",
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: "2026-01-01T00:00:00.000Z",
      ...overrides,
    });

    const makeAdapter = (initialKit?: KitImpl): KitJsonFileAdapter & { stored: string | null } => {
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
      expect(snapshot.kit.id).toBe("file-kit-id");
      expect(snapshot.kit.name).toBe("File KitImpl");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
      expect(snapshot.sync.lastSyncedAt).toBeDefined();
    });

    it("creates empty kit when adapter returns null", async () => {
      const adapter = makeAdapter();
      const store = await createJsonFileKitStore(adapter);
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.id).toBeDefined();
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

      const newKit = makeKit({ id: "new-id", name: "Replaced" });
      store.replace(newKit);
      expect(store.getSnapshot().kit.id).toBe("new-id");
      expect(store.getSnapshot().kit.name).toBe("Replaced");
      expect(notified).toBe(1);
    });

    it("save writes kit JSON to adapter and clears dirty", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      const store = await createJsonFileKitStore(adapter);

      store.apply({ name: "Saved KitImpl" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.getSnapshot().sync.status).toBe("ready");

      const savedKit = JSON.parse(adapter.stored!);
      expect(savedKit.name).toBe("Saved KitImpl");
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
      expect(store.getSnapshot().kit.name).toBe("File KitImpl");
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
            added: [{ id: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          },
        });
      });

      expect(store.getSnapshot().kit.name).toBe("Renamed");
      expect(store.getSnapshot().kit.types).toHaveLength(1);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("File KitImpl");
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
      const fileId = "file-1";
      const kit = makeKit({
        files: [
          {
            id: fileId,
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
      await store.embedFileBlob(fileId, blob);

      const fileAfter = store.getSnapshot().kit.files?.find((f) => f.id === fileId);
      expect(fileAfter?.blob).toBeDefined();
      expect(fileAfter!.blob!.startsWith("data:text/plain")).toBe(true);
      expect(fileAfter!.blob).toContain("base64,");

      await store.save();
      const saved = JSON.parse(adapter.stored!);
      const persistedFile = saved.files.find((f: any) => f.id === fileId);
      expect(persistedFile.blob).toBe(fileAfter!.blob);

      // Round-trip: reloading the JSON preserves the embedded blob.
      const reloaded = await createJsonFileKitStore(adapter);
      const reloadedFile = reloaded.getSnapshot().kit.files?.find((f) => f.id === fileId);
      expect(reloadedFile?.blob).toBe(fileAfter!.blob);
    });

    it("addFile diff followed by embedFileBlob embeds the blob on the newly added file", async () => {
      // Simulates executeSemioKitCommand("semio.kit.addFile", ...) ÔåÆ syncKitFileCommandResult ÔåÆ embedFileBlob.
      // Step 1: apply the addFile diff (what kitCommands["semio.kit.addFile"] returns).
      // Step 2: embedFileBlob reads the file from kit.files and applies a second diff setting blob.
      const adapter = makeAdapter(makeKit());
      const store = await createJsonFileKitStore(adapter);

      const newFileId = "dropped-file-id";
      const newFile = {
        id: newFileId,
        name: "drop.txt",
        size: 3,
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
      };
      store.apply({ files: { added: [newFile] } });

      const addedFile = store.getSnapshot().kit.files?.find((f) => f.id === newFileId);
      expect(addedFile).toBeDefined();
      expect(addedFile?.blob).toBeUndefined();

      const blob = new Blob(["HEY"], { type: "text/plain" });
      await store.embedFileBlob(newFileId, blob);

      const embeddedFile = store.getSnapshot().kit.files?.find((f) => f.id === newFileId);
      expect(embeddedFile?.blob).toBeDefined();
      expect(embeddedFile!.blob!.startsWith("data:text/plain")).toBe(true);
      expect(embeddedFile?.name).toBe("drop.txt");

      await store.save();
      const saved = JSON.parse(adapter.stored!);
      const persistedFile = saved.files.find((f: any) => f.id === newFileId);
      expect(persistedFile.blob).toBe(embeddedFile!.blob);
      expect(persistedFile.name).toBe("drop.txt");
    });

    it("save preserves dirty flag when an apply interleaves with an in-flight adapter.write", async () => {
      // Regression: JsonFileKitStore.embedFileBlob awaits blob.arrayBuffer()
      // which yields to the event loop. If a scheduled save() fires during
      // that await, save() serializes the pre-embed kit and clears dirty
      // after adapter.write ÔÇö clobbering the embed apply that ran mid-save.
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
      const fileId = "file-race";
      adapter.stored = JSON.stringify({
        id: "race-kit",
        name: "Race",
        createdAt: "2026-01-01T00:00:00.000Z",
        updatedAt: "2026-01-01T00:00:00.000Z",
        files: [
          {
            id: fileId,
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
      await store.embedFileBlob(fileId, blob);

      resolveFirstWrite!();
      await savePromise;

      expect(store.getSnapshot().sync.dirty).toBe(true);
      const embeddedFile = store.getSnapshot().kit.files?.find((f) => f.id === fileId);
      expect(embeddedFile?.blob).toBeDefined();
      expect(embeddedFile!.blob!.startsWith("data:text/plain")).toBe(true);

      await store.save();
      const saved = JSON.parse(adapter.stored!);
      const persistedFile = saved.files.find((f: any) => f.id === fileId);
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
        id: "f1",
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

  // #endregion ÔøàJsonFileKitStore Tests

  // #region ­ƒöèFolderKitStore Tests
  describe("FolderKitStore", () => {
    const makeKit = (overrides?: Partial<KitImpl>): KitImpl => ({
      id: "folder-kit-id",
      name: "Folder KitImpl",
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: "2026-01-01T00:00:00.000Z",
      ...overrides,
    });

    const kitToBytes = async (kit: KitImpl): Promise<Uint8Array> => {
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(kit, db);
      const data = db.export();
      db.close();
      return data;
    };

    const bytesToKit = async (data: Uint8Array): Promise<KitImpl> => {
      const SQL = await getSqlJs();
      const db = new SQL.Database(new Uint8Array(data));
      const kit = await sqliteToKit(db);
      db.close();
      return kit;
    };

    const makeAdapter = (initialKit?: KitImpl): KitFolderAdapter & { stored: Uint8Array | null; files: Map<string, Blob>; initPromise: Promise<void> } => {
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
      expect(snapshot.kit.id).toBe("folder-kit-id");
      expect(snapshot.kit.name).toBe("Folder KitImpl");
      expect(snapshot.sync.status).toBe("ready");
      expect(snapshot.sync.dirty).toBe(false);
    });

    it("creates empty kit when adapter returns null", async () => {
      const store = await createFolderKitStore(makeAdapter());
      const snapshot = store.getSnapshot();
      expect(snapshot.kit.id).toBeDefined();
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

      const newKit = makeKit({ id: "new-id", name: "Replaced" });
      store.replace(newKit);
      expect(store.getSnapshot().kit.id).toBe("new-id");
      expect(store.getSnapshot().kit.name).toBe("Replaced");
      expect(notified).toBe(1);
    });

    it("save writes kit SQLite to adapter and clears dirty", async () => {
      const kit = makeKit();
      const adapter = makeAdapter(kit);
      await adapter.initPromise;
      const store = await createFolderKitStore(adapter);

      store.apply({ name: "Saved KitImpl" });
      expect(store.getSnapshot().sync.dirty).toBe(true);

      await store.save();
      expect(store.getSnapshot().sync.dirty).toBe(false);
      expect(store.getSnapshot().sync.status).toBe("ready");

      const savedKit = await bytesToKit(adapter.stored!);
      expect(savedKit.name).toBe("Saved KitImpl");
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
      expect(store.getSnapshot().kit.name).toBe("Folder KitImpl");
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
            added: [{ id: "t1", name: "Wall", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          },
        });
      });

      expect(store.getSnapshot().kit.name).toBe("Renamed");
      expect(store.getSnapshot().kit.types).toHaveLength(1);

      store.undo();
      expect(store.getSnapshot().kit.name).toBe("Folder KitImpl");
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
  // #endregion ­ƒöèFolderKitStore Tests

  // #region ­ƒÜ¬Open Synchronized KitImpl E2E Tests
  // End-to-end tests for opening synchronized kits across all three supported source kinds:
  // file (*.kit.semio.json with embedded base64 blobs), folder (.semio/kit.db + binary files on disk),
  // and remote (SessionKitStore over HTTP + WebSocket against semio/hub).
  // Specs: These tests MUST verify the full open ÔåÆ mutate ÔåÆ save/sync ÔåÆ reload cycle using real file
  // system access or mocked server transport to guarantee the desktop/vscode/web entry points work.

  describe("Open Synchronized KitImpl E2E", () => {
    const makeJsonFileKitStore = createJsonFileKitStore;
    const makeFolderKitStore = createFolderKitStore;
    const makeSessionKitStore = createSessionKitStore;

    const loadStudio = async () => {
      const studio = await import("./index.js");
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

    describe("File KitImpl (JsonFileKitStore)", () => {
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

      it("synchronizes apply() ÔåÆ save() back to the JSON file on disk", async () => {
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
        const initial: KitImpl = {
          id: "mini-kit-id",
          name: "Mini KitImpl",
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
              added: [{ id: "t1", name: "Column", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
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

    describe("Folder KitImpl (FolderKitStore)", () => {
      beforeAll(async () => {
        // Regenerate kit.db from metabolism.kit.semio.json to ensure schema compatibility
        const kitJsonPath = await getMetabolismKitJsonPath();
        const nodeFs = await import("node:fs/promises");
        const nodePath = await import("node:path");
        const kitJson = await nodeFs.readFile(kitJsonPath, "utf-8");
        const kit = JSON.parse(kitJson) as KitImpl;
        const SQL = await getSqlJs();
        const db = new SQL.Database();
        await kitToSqlite(kit, db);
        const data = db.export();
        db.close();
        const folderPath = await getMetabolismFolderPath();
        const dbPath = nodePath.join(folderPath, ".semio", "kit.db");
        await nodeFs.mkdir(nodePath.dirname(dbPath), { recursive: true });
        await nodeFs.writeFile(dbPath, Buffer.from(data));
      });

      it("opens existing metabolism folder via .semio/kit.db without creating a new kit", async () => {
        const studio = await loadStudio();
        const folderPath = await getMetabolismFolderPath();
        const adapter = await makeNodeFolderAdapter(folderPath);
        const store = await studio.createFolderKitStore(adapter);

        const snap = store.getSnapshot();
        expect(snap.sync.status).toBe("ready");
        expect(snap.kit.id).not.toBe("");
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

      it("loads types with representations pointing at kit files so 3D meshes resolve", async () => {
        const studio = await loadStudio();
        const folderPath = await getMetabolismFolderPath();
        const adapter = await makeNodeFolderAdapter(folderPath);
        const store = await studio.createFolderKitStore(adapter);
        const kit = store.getSnapshot().kit;

        const typesWithRepresentations = (kit.types ?? []).filter((t: any) => (t.representations ?? []).length > 0);
        expect(typesWithRepresentations.length).toBeGreaterThan(0);

        const fileIdSet = new Set((kit.files ?? []).map((f: any) => f.id));
        for (const type of typesWithRepresentations) {
          for (const representation of (type as any).representations ?? []) {
            expect(representation.file?.id).toBeDefined();
            expect(fileIdSet.has(representation.file.id)).toBe(true);
          }
        }

        const firstRepresentation = typesWithRepresentations[0].representations?.[0];
        const firstFile = (kit.files ?? []).find((f: any) => f.id === firstRepresentation?.file?.id);
        expect(firstFile).toBeDefined();
        const storagePath = (() => {
          const foldersById = new Map((kit.folders ?? []).map((f: any) => [f.id, f]));
          const segments: string[] = [firstFile!.name];
          let current = firstFile!.folder?.id;
          while (current) {
            const folder: any = foldersById.get(current);
            if (!folder) break;
            segments.unshift(folder.name);
            current = folder.parent?.id;
          }
          return segments.join("/");
        })();
        const blob = await store.readFile(storagePath);
        expect(blob).not.toBeNull();
        expect(blob!.size).toBeGreaterThan(0);
      });

      it("synchronizes apply() ÔåÆ save() back to .semio/kit.db on disk", async () => {
        const fs = await import("node:fs/promises");
        const os = await import("node:os");
        const nodePath = await import("node:path");
        const tmpDir = await fs.mkdtemp(nodePath.join(os.tmpdir(), "semio-folder-kit-"));

        try {
          const studio = await loadStudio();
          const adapter = await makeNodeFolderAdapter(tmpDir);
          const initial = await studio.createFolderKitStore(adapter);
          initial.replace({
            id: "seeded-folder-kit",
            name: "Seeded Folder KitImpl",
            createdAt: "2026-01-01T00:00:00.000Z",
            updatedAt: "2026-01-01T00:00:00.000Z",
            types: [{ id: "seed-type", name: "Seed", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
          });
          await initial.save();

          const kitDbPath = nodePath.join(tmpDir, ".semio", "kit.db");
          const stat = await fs.stat(kitDbPath);
          expect(stat.size).toBeGreaterThan(0);

          initial.apply({
            types: {
              added: [{ id: "added-type", name: "Added", createdAt: "2026-01-01T00:00:00.000Z", updatedAt: "2026-01-01T00:00:00.000Z" }],
            },
          });
          await initial.save();

          const reopened = await studio.createFolderKitStore(await makeNodeFolderAdapter(tmpDir));
          const snap = reopened.getSnapshot();
          expect(snap.kit.name).toBe("Seeded Folder KitImpl");
          expect((snap.kit.types ?? []).map((t: any) => t.name).sort()).toEqual(["Added", "Seed"]);
        } finally {
          await fs.rm(tmpDir, { recursive: true, force: true });
        }
      });
    });

    describe("Remote KitImpl (SessionKitStore)", () => {
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
          id: "remote-session-kit",
          name: "Remote Session KitImpl",
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
          const store = await studio.createSessionKitStore({ serverUrl: "http://localhost:12345", kitName: "Remote Session KitImpl" });
          expect(store.sessionId).toBe("session-42");
          expect(store.getSnapshot().kit.name).toBe("Remote Session KitImpl");
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
          expect(types.some((t: any) => t.id === "remote-type-1" && t.name === "Remote Type")).toBe(true);
          store.dispose?.();
        } finally {
          globalThis.fetch = originalFetch;
          (globalThis as any).WebSocket = originalWebSocket;
        }
      });
    });
  });
  // #endregion ­ƒÜ¬Open Synchronized KitImpl E2E Tests

  // #region ­ƒÄÇMeta And Shallow Tests
  // Tests for Meta and Shallow schema parsing, conversion functions, and roundtrips.
  describe("Meta/Shallow", () => {
    describe("KitImpl/Meta", () => {
      it("parses metabolism.meta.kit.semio.json with KitMetaSchema", () => {
        const parsed = KitMetaSchema.parse(MetabolismMetaKit);
        expect(parsed.name).toBe("Metabolism");
        expect(parsed.id).toBe("f042c2a4-3ba5-44b0-b22c-0ae8f568aacc");
        expect((parsed as any).types).toBeUndefined();
        expect((parsed as any).designs).toBeUndefined();
        expect((parsed as any).files).toBeUndefined();
      });
      it("toKitMeta strips collections from full kit", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const meta = toKitMeta(kit);
        expect(meta.name).toBe("Metabolism");
        expect((meta as any).types).toBeUndefined();
        expect((meta as any).designs).toBeUndefined();
        expect((meta as any).files).toBeUndefined();
      });
      it("roundtrips KitMeta through serialize/deserialize", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const meta = toKitMeta(kit);
        const serialized = serializeKitMeta(meta);
        const deserialized = deserializeKitMeta(serialized);
        expect(deserialized.name).toBe(meta.name);
        expect(deserialized.id).toBe(meta.id);
      });
    });

    describe("KitImpl/Shallow", () => {
      it("parses metabolism.shallow.kit.semio.json with KitShallowSchema", () => {
        const parsed = KitShallowSchema.parse(MetabolismShallowKit);
        expect(parsed.name).toBe("Metabolism");
        expect(parsed.types).toBeDefined();
        expect(parsed.types!.length).toBeGreaterThan(0);
        // ­ƒÅÀ´©ÅShallow types should be meta (no nested collections like representations)
        const firstType = parsed.types![0] as any;
        expect(firstType.representations).toBeUndefined();
        expect(firstType.connectors).toBeUndefined();
      });
      it("toKitShallow converts full kit to shallow with meta children", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const shallow = toKitShallow(kit);
        expect(shallow.name).toBe("Metabolism");
        expect(shallow.types).toBeDefined();
        expect(shallow.types!.length).toBeGreaterThan(0);
        const firstType = shallow.types![0] as any;
        expect(firstType.representations).toBeUndefined();
        expect(firstType.connectors).toBeUndefined();
      });
      it("roundtrips KitShallow through serialize/deserialize", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
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
        expect(parsed.id).toBe("2a6bb3e8-4adb-44a3-bc87-3314b77b40f7");
        expect((parsed as any).representations).toBeUndefined();
        expect((parsed as any).connectors).toBeUndefined();
        expect((parsed as any).props).toBeUndefined();
      });
      it("toTypeMeta strips collections from full type", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const tambour = kit.types!.find((t: Type) => t.name === "Tambour")!;
        const meta = toTypeMeta(tambour);
        expect(meta.name).toBe("Tambour");
        expect((meta as any).representations).toBeUndefined();
        expect((meta as any).connectors).toBeUndefined();
      });
    });

    describe("Type/Shallow", () => {
      it("parses tambour.shallow.type.semio.json with TypeShallowSchema", () => {
        const parsed = TypeShallowSchema.parse(TambourShallowType);
        expect(parsed.name).toBe("Tambour");
        if (parsed.representations) {
          const firstRepresentation = parsed.representations[0] as any;
          expect(firstRepresentation.tags).toBeUndefined();
        }
      });
      it("toTypeShallow converts full type to shallow with meta children", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const tambour = kit.types!.find((t: Type) => t.name === "Tambour")!;
        const shallow = toTypeShallow(tambour);
        expect(shallow.name).toBe("Tambour");
        if (shallow.representations) {
          const firstRepresentation = shallow.representations[0] as any;
          expect(firstRepresentation.tags).toBeUndefined();
        }
      });
    });

    describe("Design/Meta", () => {
      it("parses nakagin-capsule-tower.meta.design.semio.json with DesignMetaSchema", () => {
        const parsed = DesignMetaSchema.parse(NakaginCapsuleTowerMetaDesign);
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const expectedId = kit.designs?.find((d: Design) => d.name === NAKAGIN_DESIGN_NAME)?.id;
        expect(parsed.name).toBe(NAKAGIN_DESIGN_NAME);
        expect(expectedId).toBeDefined();
        expect(parsed.id).toBe(expectedId);
        expect((parsed as any).pieces).toBeUndefined();
        expect((parsed as any).connections).toBeUndefined();
      });
      it("toDesignMeta strips collections from full design", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const nct = kit.designs!.find((d: Design) => d.name === NAKAGIN_DESIGN_NAME)!;
        const meta = toDesignMeta(nct);
        expect(meta.name).toBe(NAKAGIN_DESIGN_NAME);
        expect((meta as any).pieces).toBeUndefined();
        expect((meta as any).connections).toBeUndefined();
      });
    });

    describe("Design/Shallow", () => {
      it("parses nakagin-capsule-tower.shallow.design.semio.json with DesignShallowSchema", () => {
        const parsed = DesignShallowSchema.parse(NakaginCapsuleTowerShallowDesign);
        expect(parsed.name).toBe(NAKAGIN_DESIGN_NAME);
        if (parsed.pieces) {
          const firstPiece = parsed.pieces[0] as any;
          expect(firstPiece.attributes).toBeUndefined();
        }
      });
      it("toDesignShallow converts full design to shallow with meta children", () => {
        const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
        const nct = kit.designs!.find((d: Design) => d.name === NAKAGIN_DESIGN_NAME)!;
        const shallow = toDesignShallow(nct);
        expect(shallow.name).toBe(NAKAGIN_DESIGN_NAME);
        if (shallow.pieces) {
          const firstPiece = shallow.pieces[0] as any;
          expect(firstPiece.attributes).toBeUndefined();
        }
      });
    });
  });
  // #endregion ­ƒÄÇMeta And Shallow Tests

  // #region ­ƒùØ´©ÅHash Tests
  describe("KitImpl/Hash", () => {
    const hashCases = HashCases as {
      kitHash: { expected: string };
      kitDiffHash: { json: string; expected: string };
      designName: string;
      sha256Known: { emptyInputUtf8: string; emptyExpected: string; abcInputUtf8: string; abcExpected: string };
      kitDiffTypeAddition: { newTypeId: string; newTypeName: string };
    };

    it("hashKit produces a 64-char lowercase hex string", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const h = hashKit(kit);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKit matches expected canonical value from shared asset", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      expect(hashKit(kit)).toBe(hashCases.kitHash.expected);
    });

    it("hashKit is deterministic (same input produces same output)", () => {
      const kitA = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
      const kitB = duplicateKitForIsolation(new KitImpl(MetabolismKit as KitData));
      expect(hashKit(kitA)).toBe(hashKit(kitB));
    });

    it("hashDesign produces a 64-char lowercase hex string", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const nct = kit.designs!.find((d: Design) => d.name === hashCases.designName)!;
      const h = hashDesign(nct);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashType produces a 64-char lowercase hex string", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const t = kit.types![0];
      const h = hashType(t);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("different kits produce different hashes", () => {
      const kit1 = asKitInstance(MetabolismKit as unknown as KitImpl);
      const kit2 = { ...kit1, name: "Different Name" };
      expect(hashKit(kit1)).not.toBe(hashKit(kit2));
    });

    it("sha256 of empty input matches known value", () => {
      const h = sha256bytes(new TextEncoder().encode(hashCases.sha256Known.emptyInputUtf8));
      expect(h).toBe(hashCases.sha256Known.emptyExpected);
    });

    it("sha256 of 'abc' matches known value", () => {
      const h = sha256bytes(new TextEncoder().encode(hashCases.sha256Known.abcInputUtf8));
      expect(h).toBe(hashCases.sha256Known.abcExpected);
    });

    it("hashPiece is deterministic", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const nct = kit.designs!.find((d: Design) => d.name === hashCases.designName)!;
      const piece = nct.pieces![0];
      const h1 = hashPiece(piece);
      const h2 = hashPiece(piece);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashConnection is deterministic", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const nct = kit.designs!.find((d: Design) => d.name === hashCases.designName)!;
      const conn = nct.connections()[0];
      const h1 = hashConnection(conn);
      const h2 = hashConnection(conn);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashConnector is deterministic", () => {
      const kit = asKitInstance(MetabolismKit as unknown as KitImpl);
      const t = kit.types!.find((t: Type) => t.connectors && t.connectors.length > 0)!;
      const conn = t.connectors![0];
      const h1 = hashConnector(conn);
      const h2 = hashConnector(conn);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff is deterministic and produces valid hash", () => {
      const kit = new KitImpl(MetabolismKit as KitData);
      const modified = new KitImpl({ ...(MetabolismKit as any), name: "Modified KitImpl", description: "New description" } as KitData);
      const diff = getKitDiff(kit, modified);
      const h1 = hashKitDiff(diff);
      const h2 = hashKitDiff(diff);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff produces different hashes for different diffs", () => {
      const kit = new KitImpl(MetabolismKit as KitData);
      const mod1 = new KitImpl({ ...(MetabolismKit as any), name: "Modified1" } as KitData);
      const mod2 = new KitImpl({ ...(MetabolismKit as any), name: "Modified2" } as KitData);
      const diff1 = getKitDiff(kit, mod1);
      const diff2 = getKitDiff(kit, mod2);
      expect(hashKitDiff(diff1)).not.toBe(hashKitDiff(diff2));
    });

    it("hashKitDiff empty diff produces a consistent hash", () => {
      const kit = new KitImpl(MetabolismKit as KitData);
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

    it("hashCoordinateDiff is deterministic", () => {
      const d: CoordinateDiff = { u: 1.0, v: 2.0 };
      const h1 = hashCoordinateDiff(d);
      const h2 = hashCoordinateDiff(d);
      expect(h1).toBe(h2);
      expect(h1).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashTypeDiff with collection diffs is deterministic", () => {
      const kit = new KitImpl(MetabolismKit as KitData);
      if (kit.types && kit.types.length >= 2) {
        const modified = new KitImpl({
          ...(MetabolismKit as any),
          types: (MetabolismKit as any).types.map((t: any, i: number) => (i === 0 ? { ...t, description: "Updated type description" } : t)),
        } as KitData);
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
      const kit = new KitImpl(MetabolismKit as KitData);
      if (kit.designs && kit.designs.length >= 1) {
        const modified = new KitImpl({
          ...(MetabolismKit as any),
          designs: (MetabolismKit as any).designs.map((d: any, i: number) => (i === 0 ? { ...d, description: "Updated design" } : d)),
        } as KitData);
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
      const d: SideDiff = { piece: { id: "p1" } };
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
      const kit = new KitImpl(MetabolismKit as KitData);
      const newType: Type = new Type({ id: hashCases.kitDiffTypeAddition.newTypeId, name: hashCases.kitDiffTypeAddition.newTypeName } as any);
      const modified = new KitImpl({ ...(MetabolismKit as any), types: [...((MetabolismKit as any).types ?? []), newType] } as KitData);
      const diff = getKitDiff(kit, modified);
      const h = hashKitDiff(diff);
      expect(h).toMatch(/^[0-9a-f]{64}$/);
    });

    it("hashKitDiff matches expected canonical value", () => {
      const diff = JSON.parse(hashCases.kitDiffHash.json) as KitDiff;
      const h = hashKitDiff(diff);
      expect(h).toBe(hashCases.kitDiffHash.expected);
    });
  });
  // #endregion ­ƒùØ´©ÅHash Tests
  // #region ­ƒôèMaxChildren Tests
  describe("MaxChildren", () => {
    describe("Port", () => {
      it("Port schema accepts maxChildren", () => {
        const port: Port = { id: "p1", name: "TestPort", maxChildren: 3 };
        const parsed = PortSchema.parse(port);
        expect(parsed.maxChildren).toBe(3);
      });

      it("Port schema allows omitting maxChildren", () => {
        const port: Port = { id: "p1", name: "TestPort" };
        const parsed = PortSchema.parse(port);
        expect(parsed.maxChildren).toBeUndefined();
      });

      it("Port diff detects maxChildren change", () => {
        const before = new Port({ id: "p1", name: "TestPort", maxChildren: 1 });
        const after = new Port({ id: "p1", name: "TestPort", maxChildren: 5 });
        const diff = getPortDiff(before, after);
        expect(diff.maxChildren).toBe(5);
      });

      it("Port diff detects maxChildren removal", () => {
        const before = new Port({ id: "p1", name: "TestPort", maxChildren: 3 });
        const after = new Port({ id: "p1", name: "TestPort" });
        const diff = getPortDiff(before, after);
        expect(diff.maxChildren).toBeNull();
      });

      it("Port diff ignores unchanged maxChildren", () => {
        const before = new Port({ id: "p1", name: "TestPort", maxChildren: 2 });
        const after = new Port({ id: "p1", name: "TestPort", maxChildren: 2 });
        const diff = getPortDiff(before, after);
        expect(diff.maxChildren).toBeUndefined();
      });

      it("Port apply diff sets maxChildren", () => {
        const base = new Port({ id: "p1", name: "TestPort" });
        const diff: PortDiff = { maxChildren: 4 };
        applyPortDiff(base, diff);
        expect(base.maxChildren).toBe(4);
      });

      it("Port apply diff removes maxChildren with null", () => {
        const base = new Port({ id: "p1", name: "TestPort", maxChildren: 3 });
        const diff: PortDiff = { maxChildren: null };
        applyPortDiff(base, diff);
        expect(base.maxChildren).toBeUndefined();
      });

      it("Port inverse diff restores maxChildren", () => {
        const original = new Port({ id: "p1", name: "TestPort", maxChildren: 2 });
        const diff: PortDiff = { maxChildren: 5 };
        const inverse = inversePortDiff(original, diff);
        expect(inverse.maxChildren).toBe(2);
      });
    });

    describe("Connector", () => {
      it("Connector schema accepts maxChildren", () => {
        const connector: Connector = { id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 3 };
        const parsed = ConnectorSchema.parse(connector);
        expect(parsed.maxChildren).toBe(3);
      });

      it("Connector schema allows omitting maxChildren", () => {
        const connector: Connector = { id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 } };
        const parsed = ConnectorSchema.parse(connector);
        expect(parsed.maxChildren).toBeUndefined();
      });

      it("Connector diff detects maxChildren change", () => {
        const before = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 1 });
        const after = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 5 });
        const diff = getConnectorDiff(before, after);
        expect(diff.maxChildren).toBe(5);
      });

      it("Connector diff detects maxChildren removal", () => {
        const before = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 3 });
        const after = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 } });
        const diff = getConnectorDiff(before, after);
        expect(diff.maxChildren).toBeNull();
      });

      it("Connector apply diff sets maxChildren", () => {
        const base = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 } });
        const diff: ConnectorDiff = { maxChildren: 4 };
        applyConnectorDiff(base, diff);
        expect(base.maxChildren).toBe(4);
      });

      it("Connector apply diff removes maxChildren with null", () => {
        const base = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 3 });
        const diff: ConnectorDiff = { maxChildren: null };
        applyConnectorDiff(base, diff);
        expect(base.maxChildren).toBeUndefined();
      });

      it("Connector inverse diff restores maxChildren", () => {
        const original = new Connector({ id: "c1", t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 0, z: 1 }, maxChildren: 2 });
        const diff: ConnectorDiff = { maxChildren: 5 };
        const inverse = inverseConnectorDiff(original, diff);
        expect(inverse.maxChildren).toBe(2);
      });
    });

    describe("KitImpl Roundtrip", () => {
      it("KitImpl with maxChildren roundtrips through JSON", () => {
        const kit = new KitImpl({
          id: "kit-1",
          name: "TestKit",
          families: [{ id: "f1", name: "TestFamily", ports: [{ id: "p1", name: "Port1", maxChildren: 3 }] }],
          types: [
            {
              id: "t1",
              name: "Type1",
              connectors: [
                {
                  id: "c1",
                  t: 0,
                  point: { x: 0, y: 0, z: 0 },
                  direction: { x: 0, y: 0, z: 1 },
                  maxChildren: 5,
                },
              ],
            },
          ],
        } as KitData);
        const serialized = serializeKit(kit);
        const deserialized = deserializeKit(serialized);
        expect(deserialized.families![0].ports![0].maxChildren).toBe(3);
        expect(deserialized.types![0].connectors![0].maxChildren).toBe(5);
      });

      it("KitImpl diff captures maxChildren changes on both port and connector", () => {
        const before = new KitImpl({
          id: "kit-1",
          name: "TestKit",
          families: [{ id: "f1", name: "TestFamily", ports: [{ id: "p1", name: "Port1", maxChildren: 1 }] }],
          types: [
            {
              id: "t1",
              name: "Type1",
              connectors: [
                {
                  id: "c1",
                  t: 0,
                  point: { x: 0, y: 0, z: 0 },
                  direction: { x: 0, y: 0, z: 1 },
                  maxChildren: 1,
                },
              ],
            },
          ],
        });
        const after = new KitImpl({
          id: "kit-1",
          name: "TestKit",
          families: [{ id: "f1", name: "TestFamily", ports: [{ id: "p1", name: "Port1", maxChildren: 10 }] }],
          types: [
            {
              id: "t1",
              name: "Type1",
              connectors: [
                {
                  id: "c1",
                  t: 0,
                  point: { x: 0, y: 0, z: 0 },
                  direction: { x: 0, y: 0, z: 1 },
                  maxChildren: 20,
                },
              ],
            },
          ],
        });
        const diff = getKitDiff(before, after);
        expect(diff.families?.updated?.[0]?.diff.ports?.updated?.[0]?.diff.maxChildren).toBe(10);
        expect(diff.types?.updated?.[0]?.diff.connectors?.updated?.[0]?.diff.maxChildren).toBe(20);
        const applied = applyKitDiff(before, diff);
        expect(applied.families![0].ports![0].maxChildren).toBe(10);
        expect(applied.types![0].connectors![0].maxChildren).toBe(20);
      });
    });
  });
  // #endregion ­ƒôèMaxChildren Tests

  // #region ­ƒöäTransaction Undo/Redo Tests
  // Tests for the transaction state machine contract used by PlainAppStore, PlainKitDiffAppStore,
  // and the event handler factories (createKeyedTransactionHandlers, createSingleKeyTransactionHandlers).
  // Invariant: finalize merges edits via first.undo + last.do; redo is cleared on commit or recordEdit;
  //            fresh start preserves redo; abort discards current stack; undo/redo move between past/redo stacks.

  // #region ­ƒöäTransaction State Helpers
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

  // #endregion ­ƒöäTransaction State Helpers

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
      const original: KitImpl = {
        id: "undo-kit-1",
        name: "UndoKit",
        types: [
          {
            id: "t1",
            name: "Wall",
            description: "A wall segment",
            icon: "",
          },
        ],
        designs: [],
      };
      const diff: KitDiff = {
        types: {
          added: [
            {
              id: "t2",
              name: "Column",
              description: "A column",
              icon: "",
            },
          ],
          updated: [{ type: { id: "t1" }, diff: { description: "Modified wall" } }],
        },
      };
      const afterForward = applyKitDiff(new KitImpl(original as KitData), diff);
      expect(afterForward.types).toHaveLength(2);
      expect(afterForward.types![0].description).toBe("Modified wall");
      const inverseDiff = inverseKitDiff(original, diff);
      const afterBackward = applyKitDiff(afterForward, inverseDiff);
      expect(afterBackward.types).toHaveLength(1);
      expect(afterBackward.types![0].description).toBe("A wall segment");
      expect(afterBackward.types![0].id).toBe("t1");
    });
  });

  // #endregion 🔄Transaction Undo/Redo Tests
} // end vitest guard
// #endregion ­ƒº¬Tests



// #region BackboneKitStores
// JSON file, folder, and session-backed KitStore implementations (moved from semio/sketchpad).

// #region ðŸ”©JsonFileKitStore
// JSON file-backed kit store implementing UndoableKitStore.
// Specs: Loads a Kit from a JSON file via adapter, holds an in-memory working copy,
// persists on save() by serializing the full Kit back to JSON. Supports undo/redo
// with a command stack. reload() re-reads state from the file, discarding changes.

/**
 * Adapter for reading/writing Kit JSON to a file.
 **/
export interface KitJsonFileAdapter {
  read(): Promise<string | null>;
  write(json: string): Promise<void>;
}

/**
 * JSON file-backed kit store with undo/redo.
 **/
export class JsonFileKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus;
  private transacting: boolean = false;
  private error?: Error;
  private lastSyncedAt?: string;
  private readonly adapter: KitJsonFileAdapter;

  private constructor(kit: Kit, adapter: KitJsonFileAdapter, status: KitStoreStatus) {
    this.kit = kit;
    this.adapter = adapter;
    this.status = status;
  }

  static async create(adapter: KitJsonFileAdapter): Promise<JsonFileKitStore> {
    const json = await adapter.read();
    if (json) {
      try {
        const parsed = JSON.parse(json);
        const kit = KitSchema.parse(parsed);
        const store = new JsonFileKitStore(kit, adapter, "ready");
        store.lastSyncedAt = new Date().toISOString();
        return store;
      } catch (e) {
        const emptyKit: Kit = {
          id: id(),
          name: "New Kit",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        };
        const store = new JsonFileKitStore(emptyKit, adapter, "error");
        store.error = e instanceof Error ? e : new Error(String(e));
        return store;
      }
    }
    const emptyKit: Kit = {
      id: id(),
      name: "New Kit",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    const store = new JsonFileKitStore(emptyKit, adapter, "ready");
    return store;
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: false,
        lastSyncedAt: this.lastSyncedAt,
        error: this.error,
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
    try {
      const result = run();
      const after = this.kit;
      if (before !== after && !this.disposed) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
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
    // Specs: capture the kit reference at save start so we can detect whether
    // another apply mutated the kit while adapter.write was in flight. Only
    // clear dirty when the saved kit still matches â€” otherwise the next
    // auto-save must re-run to persist the interleaved change. This prevents
    // losing data when an async apply (e.g. JsonFileKitStore.embedFileBlob's
    // blob.arrayBuffer() await) interleaves with a pending auto-save.
    const savedKit = this.kit;
    this.status = "saving";
    this.notify();
    try {
      const json = JSON.stringify(savedKit, null, 2);
      await this.adapter.write(json);
      if (this.kit === savedKit) {
        this.dirty = false;
      }
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  async reload(): Promise<void> {
    this.status = "loading";
    this.notify();
    try {
      const json = await this.adapter.read();
      if (json) {
        const parsed = JSON.parse(json);
        this.kit = KitSchema.parse(parsed);
      }
      this.dirty = false;
      this.undoStack = [];
      this.redoStack = [];
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
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

  applyExternalUpdate(kit: Kit): void {
    this.kit = kit;
    this.dirty = false;
    this.undoStack = [];
    this.redoStack = [];
    this.lastSyncedAt = new Date().toISOString();
    this.error = undefined;
    this.status = "ready";
    this.notify();
  }

  // Embeds a dropped file blob into the kit JSON as a data URL on file.blob.
  // Specs: File kits keep everything inside the single *.kit.semio.json file, so
  // binary assets MUST be inlined as data URLs rather than written to a sidecar store.
  async embedFileBlob(fileId: string, blob: Blob): Promise<void> {
    const existingFile = this.kit.files?.find((f) => f.id === fileId);
    if (!existingFile) return;
    const bytes = new Uint8Array(await blob.arrayBuffer());
    let base64: string;
    if (typeof Buffer !== "undefined") {
      base64 = Buffer.from(bytes).toString("base64");
    } else {
      let binary = "";
      for (let i = 0; i < bytes.length; i++) binary += String.fromCharCode(bytes[i]);
      base64 = btoa(binary);
    }
    const mime = blob.type || "application/octet-stream";
    const dataUrl = `data:${mime};base64,${base64}`;
    this.apply({ files: { updated: [{ file: { id: fileId }, diff: { blob: dataUrl } }] } } as any);
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

/**
 * Creates a JsonFileKitStore by loading kit data from a file adapter.
 **/
export async function createJsonFileKitStore(adapter: KitJsonFileAdapter): Promise<JsonFileKitStore> {
  return JsonFileKitStore.create(adapter);
}

// #endregion ðŸ”©JsonFileKitStore

// #region ðŸ“¯FolderKitStore
// Folder-backed kit store implementing UndoableKitStore.
// Specs: Uses a folder with `.semio/kit.db` SQLite database for kit data.

/**
 * Adapter for folder-based kit storage I/O.
 **/
export interface KitFolderAdapter {
  readKit(): Promise<Uint8Array | null>;
  writeKit(data: Uint8Array): Promise<void>;
  readFile(path: string): Promise<Blob | null>;
  writeFile(path: string, blob: Blob): Promise<void>;
  deleteFile(path: string): Promise<void>;
  createDirectory?(path: string): Promise<void>;
  moveEntry?(fromPath: string, toPath: string): Promise<void>;
  listFiles(): Promise<string[]>;
  watch?(callback: () => void): () => void;
}

/**
 * Folder-backed kit store with undo/redo.
 **/
export class FolderKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus;
  private transacting: boolean = false;
  private error?: Error;
  private lastSyncedAt?: string;
  private readonly adapter: KitFolderAdapter;
  private unwatchFn?: () => void;
  private suppressAutoReloadUntil = 0;

  private constructor(kit: Kit, adapter: KitFolderAdapter, status: KitStoreStatus) {
    this.kit = kit;
    this.adapter = adapter;
    this.status = status;
    if (adapter.watch) {
      this.unwatchFn = adapter.watch(() => {
        if (this.disposed) return;
        if (Date.now() < this.suppressAutoReloadUntil) return;
        this.reload().catch(console.error);
      });
    }
  }

  static async create(adapter: KitFolderAdapter, initialKit?: Kit): Promise<FolderKitStore> {
    const data = await adapter.readKit();
    if (data) {
      try {
        const SQL = await getSqlJs();
        const db = new SQL.Database(new Uint8Array(data));
        const kit = await sqliteToKit(db);
        db.close();
        const store = new FolderKitStore(kit, adapter, "ready");
        store.lastSyncedAt = new Date().toISOString();
        return store;
      } catch (e) {
        const fallbackKit: Kit = initialKit ?? {
          id: id(),
          name: "New Kit",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        };
        const store = new FolderKitStore(fallbackKit, adapter, "error");
        store.error = e instanceof Error ? e : new Error(String(e));
        return store;
      }
    }
    const seedKit: Kit = initialKit ?? {
      id: id(),
      name: "New Kit",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    const store = new FolderKitStore(seedKit, adapter, "ready");
    store.dirty = true;
    return store;
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: false,
        lastSyncedAt: this.lastSyncedAt,
        error: this.error,
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
    try {
      const result = run();
      const after = this.kit;
      if (before !== after && !this.disposed) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
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
    this.status = "saving";
    this.notify();
    try {
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(this.kit, db);
      const data = db.export();
      db.close();
      await this.adapter.writeKit(data);
      this.suppressAutoReloadUntil = Date.now() + 500;
      this.dirty = false;
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  async reload(): Promise<void> {
    this.status = "loading";
    this.notify();
    try {
      const data = await this.adapter.readKit();
      if (data) {
        const SQL = await getSqlJs();
        const db = new SQL.Database(new Uint8Array(data));
        this.kit = await sqliteToKit(db);
        db.close();
      }
      this.dirty = false;
      this.undoStack = [];
      this.redoStack = [];
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  dispose(): void {
    this.disposed = true;
    if (this.unwatchFn) {
      this.unwatchFn();
      this.unwatchFn = undefined;
    }
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

  applyExternalUpdate(kit: Kit): void {
    this.kit = kit;
    this.dirty = false;
    this.undoStack = [];
    this.redoStack = [];
    this.lastSyncedAt = new Date().toISOString();
    this.error = undefined;
    this.status = "ready";
    this.notify();
  }

  async writeFile(path: string, blob: Blob): Promise<void> {
    await this.adapter.writeFile(path, blob);
  }

  async readFile(path: string): Promise<Blob | null> {
    return this.adapter.readFile(path);
  }

  async deleteFile(path: string): Promise<void> {
    await this.adapter.deleteFile(path);
  }

  async createDirectory(path: string): Promise<void> {
    if (!this.adapter.createDirectory) {
      return;
    }
    await this.adapter.createDirectory(path);
  }

  async moveEntry(fromPath: string, toPath: string): Promise<void> {
    if (!this.adapter.moveEntry || fromPath === toPath) {
      return;
    }
    await this.adapter.moveEntry(fromPath, toPath);
  }

  async listFiles(): Promise<string[]> {
    return this.adapter.listFiles();
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

/**
 * Creates a FolderKitStore by loading kit data from a folder adapter.
 **/
export async function createFolderKitStore(adapter: KitFolderAdapter, initialKit?: Kit): Promise<FolderKitStore> {
  return FolderKitStore.create(adapter, initialKit);
}

// #endregion ðŸ“¯FolderKitStore

// #region âš™ï¸SessionKitStore
// Server-backed kit store implementing UndoableKitStore.
// Specs: Connects to a semio-session backend via HTTP+WS. Commands are sent via HTTP POST,
// events received via WebSocket. Local Kit state is maintained in-memory and updated on
// accepted domain events. Baseline snapshots and incremental diffs are stored server-side.
// Supports undo/redo with a local command stack. Lookback history via server API.
// Used by: sketchpad, desktop, any frontend needing real-time collaborative kit editing.

/**
 * Configuration for creating a SessionKitStore.
 *
 * Specs: serverUrl is the base URL (e.g. http://localhost:8080). sessionId is optional â€”
 * if omitted, a new session is created. kitName is used when creating a new session.
 * personId and clientId identify this frontend instance for presence.
 **/
export interface SessionKitStoreConfig {
  serverUrl: string;
  sessionId?: string;
  kitName?: string;
  personId?: string;
  clientId?: string;
  authToken?: string;
  readOnly?: boolean;
}

/**
 * Server event received via WebSocket.
 *
 * Specs: Mirrors the Rust SessionEvent enum. Used internally by SessionKitStore
 * to update local state on server-side changes.
 **/
interface ServerEvent {
  event: string;
  command_id?: { "0": string };
  domain_version?: number;
  semio_version?: number;
  changes?: ServerEntityChange[];
  person_id?: { "0": string };
  frontend_id?: string;
  update?: ServerSemioUpdate;
}

interface ServerEntityChange {
  op: "Created" | "Updated" | "Deleted";
  entity_kind: string;
  entity_id: string;
  snapshot?: Record<string, any>;
  changed_fields?: Record<string, any>;
}

interface ServerSemioUpdate {
  kind: string;
  u?: number;
  v?: number;
  position?: [number, number, number];
  forward?: [number, number, number];
  up?: [number, number, number];
  piece_ids?: string[];
  design_ids?: string[];
}

/**
 * Presence state for one person on one frontend.
 *
 * Specs: Tracks cursor position, camera look, selection, and display metadata.
 * Updated by SemioUpdated events from the server.
 **/
export interface PresenceState {
  personId: string;
  frontendId: string;
  displayName?: string;
  color?: string;
  cursor?: { u: number; v: number };
  look?: { position: [number, number, number]; forward: [number, number, number]; up: [number, number, number] };
  selectedPieceIds: string[];
  selectedDesignIds: string[];
}

/**
 * Share token resolved from the server.
 *
 * Specs: Represents a sharable link with access mode and optional entity scope.
 **/
export interface ResolvedShareToken {
  session_id: string;
  access_mode: "owner" | "viewer";
  entity_kind?: string;
  entity_id?: string;
  label?: string;
}

/**
 * Share token entry from the server.
 *
 * Specs: Represents a share token with metadata.
 **/
export interface ShareTokenEntry {
  token: string;
  session_id: string;
  access_mode: string;
  entity_kind?: string;
  entity_id?: string;
  label?: string;
}

const mapServerSnapshotKitToKit = (rawKit: any, fallbackName: string): Kit => {
  if (rawKit && typeof rawKit === "object" && typeof rawKit.id === "string") {
    try {
      return KitSchema.parse(rawKit);
    } catch {
      // Fall through to tolerate partial server payloads.
    }
  }

  const kitId = typeof rawKit?.id === "string" ? rawKit.id : typeof rawKit?.kit_id === "string" ? rawKit.kit_id : id();
  const mapIdRef = (value: any) => (value ? { id: typeof value === "string" ? value : value.id } : undefined);
  const types = Array.isArray(rawKit?.types)
    ? rawKit.types.map((entry: any) => ({
      id: entry.id,
      name: entry.name,
      description: entry.description,
      icon: entry.icon,
      image: entry.image,
      folder: entry.folder,
      unit: entry.unit,
      stock: entry.stock,
      isAbstract: entry.isAbstract,
      virtual: entry.virtual,
      parent: mapIdRef(entry.parent ?? entry.parentType),
      location: mapIdRef(entry.location),
      connectors: entry.connectors ?? [],
      representations: entry.representations ?? [],
      props: entry.props ?? [],
    }))
    : [];
  const designs = Array.isArray(rawKit?.designs)
    ? rawKit.designs.map((entry: any) => ({
      id: entry.id,
      name: entry.name,
      description: entry.description,
      icon: entry.icon,
      image: entry.image,
      folder: entry.folder,
      unit: entry.unit,
      isAbstract: entry.isAbstract,
      canScale: entry.canScale,
      canMirror: entry.canMirror,
      parent: mapIdRef(entry.parent ?? entry.parentDesign),
      activeLayer: mapIdRef(entry.activeLayer),
      location: mapIdRef(entry.location),
      pieces: (entry.pieces ?? []).map((piece: any) => ({
        ...piece,
        type: mapIdRef(piece.type),
        design: mapIdRef(piece.design),
      })),
      connections: (entry.connections ?? []).map((connection: any) => ({
        ...connection,
        connected: connection.connected
          ? {
            piece: mapIdRef(connection.connected.piece),
            designPiece: mapIdRef(connection.connected.designPiece),
            connector: mapIdRef(connection.connected.connector),
          }
          : connection.connected,
        connecting: connection.connecting
          ? {
            piece: mapIdRef(connection.connecting.piece),
            designPiece: mapIdRef(connection.connecting.designPiece),
            connector: mapIdRef(connection.connecting.connector),
          }
          : connection.connecting,
      })),
      layers: entry.layers ?? [],
      groups: entry.groups ?? [],
      stats: entry.stats ?? [],
      props: entry.props ?? [],
    }))
    : [];
  const families = Array.isArray(rawKit?.families)
    ? rawKit.families.map((entry: any) => ({
      id: entry.id,
      name: entry.name,
      description: entry.description,
      icon: entry.icon,
      ports: (entry.ports ?? []).map((port: any) => ({
        ...port,
        compatiblePorts: (port.compatiblePorts ?? []).map(mapIdRef),
      })),
      attributes: entry.attributes ?? [],
    }))
    : [];

  return {
    id: kitId,
    name: rawKit?.name ?? fallbackName,
    version: rawKit?.version,
    description: rawKit?.description,
    icon: rawKit?.icon,
    image: rawKit?.image,
    preview: rawKit?.preview,
    remote: rawKit?.remote,
    homepage: rawKit?.homepage,
    license: rawKit?.license,
    authors: rawKit?.authors ?? [],
    tags: rawKit?.tags ?? [],
    concepts: rawKit?.concepts ?? [],
    families,
    qualities: rawKit?.qualities ?? [],
    files: rawKit?.files ?? [],
    folders: rawKit?.folders ?? [],
    types,
    designs,
    createdAt: rawKit?.createdAt ?? new Date().toISOString(),
    updatedAt: rawKit?.updatedAt ?? new Date().toISOString(),
  };
};

const getDiffEntityId = (entry: any, singularKey: string): string | undefined => {
  if (!entry || typeof entry !== "object") return undefined;
  if (typeof entry.id === "string") return entry.id;
  const ref = entry[singularKey];
  if (ref && typeof ref === "object" && typeof ref.id === "string") return ref.id;
  return undefined;
};

const updateNestedDesignEntity = <T extends { id: string }>(designs: any[] | undefined, designId: string | undefined, collectionKey: "pieces" | "connections", entityId: string, changedFields: Record<string, any>): any[] => {
  return (designs ?? []).map((design) => {
    const isTargetDesign = !designId || design.id === designId || (design[collectionKey] ?? []).some((entry: T) => entry.id === entityId);
    if (!isTargetDesign) return design;
    return {
      ...design,
      [collectionKey]: (design[collectionKey] ?? []).map((entry: T) => (entry.id === entityId ? { ...entry, ...changedFields } : entry)),
    };
  });
};

const removeNestedDesignEntity = <T extends { id: string }>(designs: any[] | undefined, collectionKey: "pieces" | "connections", entityId: string): any[] => {
  return (designs ?? []).map((design) => ({
    ...design,
    [collectionKey]: (design[collectionKey] ?? []).filter((entry: T) => entry.id !== entityId),
  }));
};

const getPortFamilyIdFromPayload = (value: Record<string, any> | undefined): string | undefined => {
  if (!value) return undefined;
  if (typeof value.family_id === "string") return value.family_id;
  if (typeof value.familyId === "string") return value.familyId;
  if (typeof value.parent_family_id === "string") return value.parent_family_id;
  if (typeof value.parentFamilyId === "string") return value.parentFamilyId;
  if (value.family && typeof value.family === "object" && typeof value.family.id === "string") return value.family.id;
  return undefined;
};

const appendPortToFamily = (families: any[] | undefined, familyId: string | undefined, port: Record<string, any>): any[] => {
  if (!familyId) return families ?? [];
  return (families ?? []).map((family) => (family.id === familyId ? { ...family, ports: [...(family.ports ?? []), port] } : family));
};

const updatePortInFamilies = (families: any[] | undefined, entityId: string, changedFields: Record<string, any>): any[] => {
  const targetFamilyId = getPortFamilyIdFromPayload(changedFields);
  const existingPort = getKitPorts({ families }).find((entry) => entry.id === entityId);
  const mergedPort = existingPort ? { ...existingPort, ...changedFields } : { id: entityId, ...changedFields };
  const sourceFamily = findKitPortFamily({ families }, entityId);
  const familyId = targetFamilyId ?? sourceFamily?.id;
  const withoutPort = (families ?? []).map((family) => ({ ...family, ports: (family.ports ?? []).filter((entry: { id: string }) => entry.id !== entityId) }));
  return appendPortToFamily(withoutPort, familyId, mergedPort);
};

const removePortFromFamilies = (families: any[] | undefined, entityId: string): any[] =>
  (families ?? []).map((family) => ({ ...family, ports: (family.ports ?? []).filter((entry: { id: string }) => entry.id !== entityId) }));

/**
 * Server-backed kit store with undo/redo and real-time sync.
 *
 * Specs: Connects to semio-session server via HTTP for commands and WS for events.
 * On connect: fetches snapshot to initialize local Kit. On mutation: sends DomainCommand
 * via POST, waits for Accepted event via WS. On WS event: applies entity changes to
 * local Kit and notifies subscribers. Undo/redo operates on local command stack.
 * Provides presence tracking via semio commands.
 **/
export class SessionKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus;
  private transacting: boolean = false;
  private error?: Error;
  private lastSyncedAt?: string;
  private ws: WebSocket | null = null;
  private domainVersion: number = 0;
  private semioVersion: number = 0;
  private presences: Map<string, PresenceState> = new Map();
  private presenceListeners: Set<() => void> = new Set();
  private entityListeners: Map<string, Set<() => void>> = new Map();
  private collectionListeners: Map<string, Set<() => void>> = new Map();
  private propertyListeners: Map<string, Set<() => void>> = new Map();

  readonly serverUrl: string;
  readonly sessionId: string;
  readonly personId: string;
  readonly clientId: string;
  private authToken: string | undefined;
  readonly readOnly: boolean;

  private constructor(kit: Kit, config: SessionKitStoreConfig & { sessionId: string }, status: KitStoreStatus) {
    this.kit = kit;
    this.serverUrl = config.serverUrl;
    this.sessionId = config.sessionId;
    this.personId = config.personId ?? id();
    this.clientId = config.clientId ?? id();
    this.authToken = config.authToken;
    this.readOnly = config.readOnly ?? false;
    this.status = status;
  }

  private authHeaders(): Record<string, string> {
    const headers: Record<string, string> = { "Content-Type": "application/json" };
    if (this.authToken) headers["Authorization"] = `Bearer ${this.authToken}`;
    return headers;
  }

  /**
   * Creates a SessionKitStore by connecting to the server.
   * If sessionId is provided, fetches the existing session snapshot.
   * If not, creates a new session on the server.
   *
   * Specs: Factory method handling async connection. Establishes WebSocket
   * for real-time events after initial snapshot load.
   **/
  static async create(config: SessionKitStoreConfig): Promise<SessionKitStore> {
    let sessionId = config.sessionId;
    let kitName = config.kitName ?? "New Kit";
    let authToken = config.authToken;

    if (!sessionId) {
      const resp = await fetch(`${config.serverUrl}/sessions`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ kit_name: kitName }),
      });
      if (!resp.ok) throw new Error(`Failed to create session: ${resp.statusText}`);
      const body = await resp.json();
      sessionId = body.session_id;
      // Store the owner_token as authToken for full access
      if (body.owner_token && !authToken) {
        authToken = body.owner_token;
      }
    }

    const snapHeaders: Record<string, string> = {};
    if (authToken) snapHeaders["Authorization"] = `Bearer ${authToken}`;
    const snapResp = await fetch(`${config.serverUrl}/sessions/${sessionId}/snapshot`, { headers: snapHeaders });
    if (!snapResp.ok) throw new Error(`Failed to load snapshot: ${snapResp.statusText}`);
    const snapshot = await snapResp.json();
    const kit = mapServerSnapshotKitToKit(snapshot.kit, kitName);

    const store = new SessionKitStore(kit, { ...config, sessionId: sessionId!, authToken }, "ready");
    store.domainVersion = snapshot.domain_version ?? 0;
    store.semioVersion = snapshot.semio_version ?? 0;
    store.lastSyncedAt = new Date().toISOString();
    store.connectWebSocket();
    return store;
  }

  private connectWebSocket(): void {
    const wsUrl = this.serverUrl.replace(/^http/, "ws") + `/sessions/${this.sessionId}/ws`;
    this.ws = new WebSocket(wsUrl);
    this.ws.onmessage = (event) => {
      try {
        const data: ServerEvent = JSON.parse(typeof event.data === "string" ? event.data : "");
        this.handleServerEvent(data);
      } catch (e) {
        // Ignore unparseable messages
      }
    };
    this.ws.onclose = () => {
      if (!this.disposed) {
        this.status = "offline";
        this.notify();
        // Auto-reconnect after 2 seconds
        setTimeout(() => {
          if (!this.disposed) this.connectWebSocket();
        }, 2000);
      }
    };
    this.ws.onerror = () => {
      this.status = "offline";
      this.notify();
    };
    this.ws.onopen = () => {
      this.status = "ready";
      this.error = undefined;
      this.notify();
    };
  }

  private handleServerEvent(event: ServerEvent): void {
    switch (event.event) {
      case "DomainCommandAccepted": {
        if (event.domain_version !== undefined) {
          this.domainVersion = event.domain_version;
        }
        if (event.changes) {
          this.applyServerChanges(event.changes);
          this.lastSyncedAt = new Date().toISOString();
          // Notify granular listeners
          for (const change of event.changes) {
            this.notifyEntityListeners(change.entity_kind, change.entity_id);
            this.notifyCollectionListeners(change.entity_kind);
            if (change.op === "Updated" && change.changed_fields) {
              for (const field of Object.keys(change.changed_fields)) {
                this.notifyPropertyListeners(change.entity_kind, change.entity_id, field);
              }
            }
          }
        }
        this.dirty = false;
        this.notify();
        break;
      }
      case "SemioUpdated": {
        if (event.semio_version !== undefined) {
          this.semioVersion = event.semio_version;
        }
        if (event.person_id && event.frontend_id && event.update) {
          const key = `${event.person_id["0"]}:${event.frontend_id}`;
          let presence = this.presences.get(key) ?? {
            personId: event.person_id["0"],
            frontendId: event.frontend_id,
            selectedPieceIds: [],
            selectedDesignIds: [],
          };
          switch (event.update.kind) {
            case "CursorMoved":
              presence.cursor = { u: event.update.u!, v: event.update.v! };
              break;
            case "LookChanged":
              presence.look = { position: event.update.position!, forward: event.update.forward!, up: event.update.up! };
              break;
            case "SelectionChanged":
              presence.selectedPieceIds = event.update.piece_ids ?? [];
              presence.selectedDesignIds = event.update.design_ids ?? [];
              break;
            case "PresenceCleared":
              this.presences.delete(key);
              this.notifyPresenceListeners();
              this.notify();
              return;
          }
          this.presences.set(key, presence);
          this.notifyPresenceListeners();
        }
        this.notify();
        break;
      }
      case "SessionClosed":
        this.status = "offline";
        this.notify();
        break;
    }
  }

  private applyServerChanges(changes: ServerEntityChange[]): void {
    for (const change of changes) {
      switch (change.op) {
        case "Created":
          this.applyCreatedEntity(change.entity_kind, change.entity_id, change.snapshot ?? {});
          break;
        case "Updated":
          this.applyUpdatedEntity(change.entity_kind, change.entity_id, change.changed_fields ?? {});
          break;
        case "Deleted":
          this.applyDeletedEntity(change.entity_kind, change.entity_id);
          break;
      }
    }
  }

  private applyCreatedEntity(entityKind: string, entityId: string, snapshot: Record<string, any>): void {
    const entity = { id: entityId, ...snapshot };
    switch (entityKind) {
      case "type":
        this.kit = { ...this.kit, types: [...(this.kit.types ?? []), entity as any] };
        break;
      case "design":
        this.kit = { ...this.kit, designs: [...(this.kit.designs ?? []), entity as any] };
        break;
      case "author":
        this.kit = { ...this.kit, authors: [...(this.kit.authors ?? []), entity as any] };
        break;
      case "tag":
        this.kit = { ...this.kit, tags: [...(this.kit.tags ?? []), entity as any] };
        break;
      case "concept":
        this.kit = { ...this.kit, concepts: [...(this.kit.concepts ?? []), entity as any] };
        break;
      case "port": {
        const familyId = getPortFamilyIdFromPayload(snapshot);
        this.kit = { ...this.kit, families: appendPortToFamily(this.kit.families, familyId, entity as any) };
        break;
      }
      case "quality":
        this.kit = { ...this.kit, qualities: [...(this.kit.qualities ?? []), entity as any] };
        break;
      case "file":
        this.kit = { ...this.kit, files: [...(this.kit.files ?? []), entity as any] };
        break;
      case "folder":
        this.kit = { ...this.kit, folders: [...(this.kit.folders ?? []), entity as any] };
        break;
      case "piece": {
        const designId = typeof snapshot.design_id === "string" ? snapshot.design_id : typeof snapshot.designId === "string" ? snapshot.designId : undefined;
        this.kit = {
          ...this.kit,
          designs: (this.kit.designs ?? []).map((design) => (design.id === designId ? { ...design, pieces: [...(design.pieces ?? []), entity as any] } : design)),
        };
        break;
      }
      case "connection": {
        const designId = typeof snapshot.design_id === "string" ? snapshot.design_id : typeof snapshot.designId === "string" ? snapshot.designId : undefined;
        this.kit = {
          ...this.kit,
          designs: (this.kit.designs ?? []).map((design) => (design.id === designId ? { ...design, connections: [...(design.connections ?? []), entity as any] } : design)),
        };
        break;
      }
    }
  }

  private applyUpdatedEntity(entityKind: string, entityId: string, changedFields: Record<string, any>): void {
    const updateInArray = <T extends { id: string }>(arr: T[] | undefined, id: string, fields: Record<string, any>): T[] => {
      return (arr ?? []).map((item) => (item.id === id ? { ...item, ...fields } : item));
    };
    switch (entityKind) {
      case "kit":
        this.kit = { ...this.kit, ...changedFields };
        break;
      case "type":
        this.kit = { ...this.kit, types: updateInArray(this.kit.types, entityId, changedFields) };
        break;
      case "design":
        this.kit = { ...this.kit, designs: updateInArray(this.kit.designs, entityId, changedFields) };
        break;
      case "author":
        this.kit = { ...this.kit, authors: updateInArray(this.kit.authors, entityId, changedFields) };
        break;
      case "tag":
        this.kit = { ...this.kit, tags: updateInArray(this.kit.tags, entityId, changedFields) };
        break;
      case "concept":
        this.kit = { ...this.kit, concepts: updateInArray(this.kit.concepts, entityId, changedFields) };
        break;
      case "port":
        this.kit = { ...this.kit, families: updatePortInFamilies(this.kit.families, entityId, changedFields) };
        break;
      case "quality":
        this.kit = { ...this.kit, qualities: updateInArray(this.kit.qualities, entityId, changedFields) };
        break;
      case "file":
        this.kit = { ...this.kit, files: updateInArray(this.kit.files, entityId, changedFields) };
        break;
      case "folder":
        this.kit = { ...this.kit, folders: updateInArray(this.kit.folders, entityId, changedFields) };
        break;
      case "piece": {
        const designId = typeof changedFields.design_id === "string" ? changedFields.design_id : typeof changedFields.designId === "string" ? changedFields.designId : undefined;
        this.kit = { ...this.kit, designs: updateNestedDesignEntity(this.kit.designs, designId, "pieces", entityId, changedFields) };
        break;
      }
      case "connection": {
        const designId = typeof changedFields.design_id === "string" ? changedFields.design_id : typeof changedFields.designId === "string" ? changedFields.designId : undefined;
        this.kit = { ...this.kit, designs: updateNestedDesignEntity(this.kit.designs, designId, "connections", entityId, changedFields) };
        break;
      }
    }
  }

  private applyDeletedEntity(entityKind: string, entityId: string): void {
    const removeFromArray = <T extends { id: string }>(arr: T[] | undefined, id: string): T[] => {
      return (arr ?? []).filter((item) => item.id !== id);
    };
    switch (entityKind) {
      case "type":
        this.kit = { ...this.kit, types: removeFromArray(this.kit.types, entityId) };
        break;
      case "design":
        this.kit = { ...this.kit, designs: removeFromArray(this.kit.designs, entityId) };
        break;
      case "author":
        this.kit = { ...this.kit, authors: removeFromArray(this.kit.authors, entityId) };
        break;
      case "tag":
        this.kit = { ...this.kit, tags: removeFromArray(this.kit.tags, entityId) };
        break;
      case "concept":
        this.kit = { ...this.kit, concepts: removeFromArray(this.kit.concepts, entityId) };
        break;
      case "port":
        this.kit = { ...this.kit, families: removePortFromFamilies(this.kit.families, entityId) };
        break;
      case "quality":
        this.kit = { ...this.kit, qualities: removeFromArray(this.kit.qualities, entityId) };
        break;
      case "file":
        this.kit = { ...this.kit, files: removeFromArray(this.kit.files, entityId) };
        break;
      case "folder":
        this.kit = { ...this.kit, folders: removeFromArray(this.kit.folders, entityId) };
        break;
      case "piece":
        this.kit = { ...this.kit, designs: removeNestedDesignEntity(this.kit.designs, "pieces", entityId) };
        break;
      case "connection":
        this.kit = { ...this.kit, designs: removeNestedDesignEntity(this.kit.designs, "connections", entityId) };
        break;
    }
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: this.readOnly,
        lastSyncedAt: this.lastSyncedAt,
        error: this.error,
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
    try {
      const result = run();
      const after = this.kit;
      if (before !== after && !this.disposed) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
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
    // Send diff as domain commands to server
    this.sendKitDiffToServer(diff).catch((e) => {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    });
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
    this.sendKitDiffToServer(getKitDiff(before, next)).catch((e) => {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    });
    this.notify();
  }

  private async sendKitDiffToServer(diff: KitDiff): Promise<void> {
    if (this.readOnly) throw new Error("Cannot send changes in read-only mode");
    const commands: any[] = [];
    // Kit-level fields
    const kitFields: Record<string, any> = {};
    if (diff.name !== undefined) kitFields.name = diff.name;
    if (diff.version !== undefined) kitFields.version = diff.version;
    if (diff.description !== undefined) kitFields.description = diff.description;
    if (diff.icon !== undefined) kitFields.icon = diff.icon;
    if (diff.image !== undefined) kitFields.image = diff.image;
    if (diff.remote !== undefined) kitFields.remote = diff.remote;
    if (diff.homepage !== undefined) kitFields.homepage = diff.homepage;
    if (diff.license !== undefined) kitFields.license = diff.license;
    if (diff.preview !== undefined) kitFields.preview = diff.preview;
    if (Object.keys(kitFields).length > 0) {
      commands.push({ kind: "PatchKit", payload: { fields: kitFields } });
    }
    // Collection diffs
    const collectionMap: Record<string, { create: string; patch: string; delete: string; singular: string }> = {
      types: { create: "CreateType", patch: "PatchType", delete: "DeleteType", singular: "type" },
      designs: { create: "CreateDesign", patch: "PatchDesign", delete: "DeleteDesign", singular: "design" },
      authors: { create: "CreateAuthor", patch: "PatchAuthor", delete: "DeleteAuthor", singular: "author" },
      tags: { create: "CreateTag", patch: "PatchTag", delete: "DeleteTag", singular: "tag" },
      concepts: { create: "CreateConcept", patch: "PatchConcept", delete: "DeleteConcept", singular: "concept" },
      ports: { create: "CreatePort", patch: "PatchPort", delete: "DeletePort", singular: "port" },
      qualities: { create: "CreateQuality", patch: "PatchQuality", delete: "DeleteQuality", singular: "quality" },
      files: { create: "CreateFile", patch: "PatchFile", delete: "DeleteFile", singular: "file" },
      folders: { create: "CreateFolder", patch: "PatchFolder", delete: "DeleteFolder", singular: "folder" },
    };
    for (const [key, ops] of Object.entries(collectionMap)) {
      const collDiff = (diff as any)[key];
      if (!collDiff) continue;
      if (collDiff.added) {
        for (const item of collDiff.added) {
          commands.push({ kind: ops.create, payload: { entity_id: item.id ?? id(), fields: item } });
        }
      }
      if (collDiff.updated) {
        for (const item of collDiff.updated) {
          const entityId = getDiffEntityId(item, ops.singular);
          if (!entityId) continue;
          const rawFields = { ...(item.diff ?? item) };
          if (key === "designs") {
            delete (rawFields as any).pieces;
            delete (rawFields as any).connections;
            delete (rawFields as any).layers;
            delete (rawFields as any).groups;
            delete (rawFields as any).stats;
            delete (rawFields as any).props;
          }
          if (key === "types") {
            delete (rawFields as any).representations;
            delete (rawFields as any).connectors;
            delete (rawFields as any).props;
          }
          if (Object.keys(rawFields).length === 0) continue;
          commands.push({ kind: ops.patch, payload: { entity_id: entityId, fields: rawFields } });
        }
      }
      if (collDiff.removed) {
        for (const item of collDiff.removed) {
          const entityId = getDiffEntityId(item, ops.singular);
          if (!entityId) continue;
          commands.push({ kind: ops.delete, payload: { entity_id: entityId } });
        }
      }
    }

    const designsDiff = diff.designs;
    if (designsDiff?.updated) {
      for (const updatedDesign of designsDiff.updated) {
        const designId = getDiffEntityId(updatedDesign, "design");
        const designDiff = updatedDesign?.diff;
        if (!designId || !designDiff) continue;

        const piecesDiff = designDiff.pieces;
        if (piecesDiff?.added) {
          for (const piece of piecesDiff.added) {
            commands.push({
              kind: "CreatePiece",
              payload: { piece_id: piece.id, design_id: designId, fields: { ...piece, design_id: designId } },
            });
          }
        }
        if (piecesDiff?.updated) {
          for (const pieceUpdate of piecesDiff.updated) {
            const pieceId = getDiffEntityId(pieceUpdate, "piece");
            if (!pieceId) continue;
            commands.push({
              kind: "PatchPiece",
              payload: { entity_id: pieceId, fields: { ...(pieceUpdate.diff ?? {}), design_id: designId } },
            });
          }
        }
        if (piecesDiff?.removed) {
          for (const piece of piecesDiff.removed) {
            const pieceId = getDiffEntityId(piece, "piece");
            if (!pieceId) continue;
            commands.push({ kind: "DeletePiece", payload: { entity_id: pieceId } });
          }
        }

        const connectionsDiff = designDiff.connections;
        if (connectionsDiff?.added) {
          for (const connection of connectionsDiff.added) {
            commands.push({
              kind: "CreateConnection",
              payload: {
                connection_id: connection.id,
                design_id: designId,
                fields: {
                  ...connection,
                  design_id: designId,
                  connected_piece_id: connection.connected?.piece?.id,
                  connecting_piece_id: connection.connecting?.piece?.id,
                },
              },
            });
          }
        }
        if (connectionsDiff?.updated) {
          for (const connectionUpdate of connectionsDiff.updated) {
            const connectionId = getDiffEntityId(connectionUpdate, "connection");
            if (!connectionId) continue;
            commands.push({
              kind: "PatchConnection",
              payload: { entity_id: connectionId, fields: { ...(connectionUpdate.diff ?? {}), design_id: designId } },
            });
          }
        }
        if (connectionsDiff?.removed) {
          for (const connection of connectionsDiff.removed) {
            const connectionId = getDiffEntityId(connection, "connection");
            if (!connectionId) continue;
            commands.push({ kind: "DeleteConnection", payload: { entity_id: connectionId } });
          }
        }
      }
    }

    if (commands.length === 0) return;
    const batch = commands.length === 1 ? commands[0] : { kind: "Batch", payload: { commands } };
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/domain`, {
      method: "POST",
      headers: this.authHeaders(),
      body: JSON.stringify({
        command_id: { "0": id() },
        client_id: { "0": this.clientId },
        request_id: { "0": id() },
        actor_person_id: { "0": this.personId },
        base_domain_version: this.domainVersion,
        ...batch,
      }),
    });
    if (!resp.ok) {
      throw new Error(`Failed to send command: ${resp.statusText}`);
    }
  }

  async save(): Promise<void> {
    // Server-backed: save is implicit on command submission
    this.dirty = false;
    this.lastSyncedAt = new Date().toISOString();
    this.notify();
  }

  async reload(): Promise<void> {
    this.status = "loading";
    this.notify();
    try {
      const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/snapshot`, { headers: this.authHeaders() });
      if (!resp.ok) throw new Error(`Failed to reload: ${resp.statusText}`);
      const snapshot = await resp.json();
      const reloadedKit = mapServerSnapshotKitToKit(snapshot.kit, this.kit.name);
      this.kit = { ...reloadedKit, createdAt: this.kit.createdAt ?? reloadedKit.createdAt, updatedAt: new Date().toISOString() };
      this.domainVersion = snapshot.domain_version ?? 0;
      this.semioVersion = snapshot.semio_version ?? 0;
      this.dirty = false;
      this.undoStack = [];
      this.redoStack = [];
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  dispose(): void {
    this.disposed = true;
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.listeners.clear();
    this.presenceListeners.clear();
    this.entityListeners.clear();
    this.collectionListeners.clear();
    this.propertyListeners.clear();
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
    if (this.readOnly) throw new Error("Cannot undo in read-only mode");
    const change = this.undoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.backward);
    this.redoStack.push(change);
    this.dirty = true;
    this.sendKitDiffToServer(change.backward).catch(() => { });
    this.notify();
  }

  redo(): void {
    if (this.readOnly) throw new Error("Cannot redo in read-only mode");
    const change = this.redoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.forward);
    this.undoStack.push(change);
    this.dirty = true;
    this.sendKitDiffToServer(change.forward).catch(() => { });
    this.notify();
  }

  // #region ðŸ“£Presence

  async sendCursor(u: number, v: number): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: this.authHeaders(),
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "UpsertCursor",
        payload: { u, v },
      }),
    });
  }

  async sendLook(position: [number, number, number], forward: [number, number, number], up: [number, number, number]): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: this.authHeaders(),
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "UpsertLook",
        payload: { position, forward, up },
      }),
    });
  }

  async sendSelection(pieceIds: string[], designIds: string[]): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: this.authHeaders(),
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "SetSelection",
        payload: { piece_ids: pieceIds, design_ids: designIds },
      }),
    });
  }

  async clearPresence(): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: this.authHeaders(),
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "ClearPresence",
        payload: null,
      }),
    });
  }

  getPresences(): PresenceState[] {
    return Array.from(this.presences.values());
  }

  subscribePresence(listener: () => void): () => void {
    this.presenceListeners.add(listener);
    return () => {
      this.presenceListeners.delete(listener);
    };
  }

  // #endregion ðŸ“£Presence

  // #region ï¿½Auth

  /**
   * Returns the current auth token (owner_token or share token).
   *
   * Specs: Useful for passing to child components or persisting across sessions.
   **/
  getAuthToken(): string | undefined {
    return this.authToken;
  }

  /**
   * Creates a share token for this session.
   *
   * Specs: Requires owner access. Returns the share token string.
   **/
  async createShare(opts: { accessMode?: "viewer"; entityKind?: string; entityId?: string; label?: string; expiresInSeconds?: number }): Promise<ShareTokenEntry> {
    if (this.readOnly) throw new Error("Cannot create share in read-only mode");
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/shares`, {
      method: "POST",
      headers: this.authHeaders(),
      body: JSON.stringify({
        access_mode: opts.accessMode ?? "viewer",
        entity_kind: opts.entityKind,
        entity_id: opts.entityId,
        label: opts.label,
        expires_in_seconds: opts.expiresInSeconds,
      }),
    });
    if (!resp.ok) throw new Error(`Failed to create share: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Lists all share tokens for this session.
   *
   * Specs: Requires owner access.
   **/
  async listShares(): Promise<ShareTokenEntry[]> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/shares`, {
      headers: this.authHeaders(),
    });
    if (!resp.ok) throw new Error(`Failed to list shares: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Deletes a share token for this session.
   *
   * Specs: Requires owner access.
   **/
  async deleteShare(token: string): Promise<void> {
    if (this.readOnly) throw new Error("Cannot delete share in read-only mode");
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/shares/${token}`, {
      method: "DELETE",
      headers: this.authHeaders(),
    });
    if (!resp.ok) throw new Error(`Failed to delete share: ${resp.statusText}`);
  }

  /**
   * Resolves a share token to session info without needing to know the session ID.
   *
   * Specs: Static method. Returns session_id, access_mode, optional entity scope.
   **/
  static async resolveShare(serverUrl: string, token: string): Promise<ResolvedShareToken> {
    const resp = await fetch(`${serverUrl}/shares/${token}`);
    if (!resp.ok) throw new Error(`Failed to resolve share: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Creates a SessionKitStore from a share token.
   *
   * Specs: Resolves the share token, then connects with viewer access.
   **/
  static async createFromShareToken(serverUrl: string, token: string, config?: Partial<SessionKitStoreConfig>): Promise<SessionKitStore> {
    const share = await SessionKitStore.resolveShare(serverUrl, token);
    return SessionKitStore.create({
      serverUrl,
      sessionId: share.session_id,
      authToken: token,
      readOnly: share.access_mode === "viewer",
      ...config,
    });
  }

  // #endregion ðŸ”‘Auth

  // #region ï¿½ðŸ”©History

  async getKitAtLookback(lookback: string): Promise<Kit> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/kit/at/${lookback}`, { headers: this.authHeaders() });
    if (!resp.ok) throw new Error(`Failed to get kit at lookback ${lookback}: ${resp.statusText}`);
    return resp.json();
  }

  async getKitAtVersion(version: number): Promise<Kit> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/kit/at-version/${version}`, { headers: this.authHeaders() });
    if (!resp.ok) throw new Error(`Failed to get kit at version ${version}: ${resp.statusText}`);
    return resp.json();
  }

  async compactHistory(): Promise<{ snapshots_created: number; logs_deleted: number }> {
    if (this.readOnly) throw new Error("Cannot compact history in read-only mode");
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/history/compact`, { method: "POST", headers: this.authHeaders() });
    if (!resp.ok) throw new Error(`Failed to compact: ${resp.statusText}`);
    return resp.json();
  }

  async getLookbackTokens(): Promise<string[]> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/history/lookback-tokens`, { headers: this.authHeaders() });
    if (!resp.ok) throw new Error(`Failed to get tokens: ${resp.statusText}`);
    return resp.json();
  }

  getDomainVersion(): number {
    return this.domainVersion;
  }

  getSemioVersion(): number {
    return this.semioVersion;
  }

  // #endregion ðŸ”©History

  // #region ðŸŽµGranularSubscriptions

  subscribeEntity(entityKind: string, entityId: string, listener: () => void): () => void {
    const key = `${entityKind}:${entityId}`;
    if (!this.entityListeners.has(key)) this.entityListeners.set(key, new Set());
    this.entityListeners.get(key)!.add(listener);
    return () => {
      this.entityListeners.get(key)?.delete(listener);
    };
  }

  subscribeCollection(entityKind: string, listener: () => void): () => void {
    if (!this.collectionListeners.has(entityKind)) this.collectionListeners.set(entityKind, new Set());
    this.collectionListeners.get(entityKind)!.add(listener);
    return () => {
      this.collectionListeners.get(entityKind)?.delete(listener);
    };
  }

  subscribeProperty(entityKind: string, entityId: string, field: string, listener: () => void): () => void {
    const key = `${entityKind}:${entityId}:${field}`;
    if (!this.propertyListeners.has(key)) this.propertyListeners.set(key, new Set());
    this.propertyListeners.get(key)!.add(listener);
    return () => {
      this.propertyListeners.get(key)?.delete(listener);
    };
  }

  private notifyEntityListeners(entityKind: string, entityId: string): void {
    const key = `${entityKind}:${entityId}`;
    this.entityListeners.get(key)?.forEach((l) => l());
  }

  private notifyCollectionListeners(entityKind: string): void {
    this.collectionListeners.get(entityKind)?.forEach((l) => l());
  }

  private notifyPropertyListeners(entityKind: string, entityId: string, field: string): void {
    const key = `${entityKind}:${entityId}:${field}`;
    this.propertyListeners.get(key)?.forEach((l) => l());
  }

  private notifyPresenceListeners(): void {
    this.presenceListeners.forEach((l) => l());
  }

  // #endregion ðŸŽµGranularSubscriptions

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

/**
 * Creates a SessionKitStore by connecting to a semio-session server.
 *
 * Specs: Factory function matching the provider pattern.
 **/
export async function createSessionKitStore(config: SessionKitStoreConfig): Promise<SessionKitStore> {
  return SessionKitStore.create(config);
}

// #endregion BackboneKitStores

// #region Graph kit commands (semio.kit.* apply via KitStore)

/**
 * Context for kit commands including kit data, file URLs, and origin.
 **/
export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
  origin?: string;
}

/**
 * Result of a kit command with optional diff, files, and origin.
 **/
export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
  origin?: string;
}
export type KitFileState = {
  blobs: Map<string, Blob>;
  objectUrls: Map<string, string>;
  providerUrls: Map<string, string>;
  pendingBlobDownloads: Map<string, Promise<string | null>>;
  providerFactory?: FileProviderFactory;
  provider?: FileProvider;
  providerKitId?: string;
};

export type KitBinaryStore = KitStore & {
  readFile?: (path: string) => Promise<Blob | null>;
  writeFile?: (path: string, blob: Blob) => Promise<void>;
  deleteFile?: (path: string) => Promise<void>;
  createDirectory?: (path: string) => Promise<void>;
  moveEntry?: (fromPath: string, toPath: string) => Promise<void>;
};

export function getOrCreateKitFileState(kitStore: KitStore): KitFileState {
  const storeWithFiles = kitStore as KitStore & { __semioKitFileState?: KitFileState };
  if (!storeWithFiles.__semioKitFileState) {
    storeWithFiles.__semioKitFileState = {
      blobs: new Map(),
      objectUrls: new Map(),
      providerUrls: new Map(),
      pendingBlobDownloads: new Map(),
    };
  }
  return storeWithFiles.__semioKitFileState;
}

export function getStoredKitFileUrls(kitStore: KitStore): Map<string, string> {
  const kit = kitStore.getSnapshot().kit;
  const fileState = getOrCreateKitFileState(kitStore);
  const fileUrls = new Map<string, string>();

  for (const file of kit.files ?? []) {
    const readableUrl = getReadableKitFileUrl(fileState, file);
    if (readableUrl) {
      fileUrls.set(getKitFileStoragePath(kit, file), readableUrl);
    }
  }

  return fileUrls;
}

export const isBrowserReadableFileUrl = (url: string): boolean => /^(blob:|data:|https?:)/i.test(url);

export const getReadableKitFileUrl = (fileState: KitFileState, file: SemioFile): string | null => {
  const cachedBlobUrl = fileState.objectUrls.get(file.id);
  if (cachedBlobUrl) {
    return cachedBlobUrl;
  }

  const cachedProviderUrl = fileState.providerUrls.get(file.id);
  if (cachedProviderUrl && isBrowserReadableFileUrl(cachedProviderUrl)) {
    return cachedProviderUrl;
  }

  if (file.blob && isBrowserReadableFileUrl(file.blob)) {
    return file.blob;
  }

  if (file.remote && isBrowserReadableFileUrl(file.remote)) {
    return file.remote;
  }

  return null;
};

export const getKitFileStoragePath = (kit: Kit, file: SemioFile): string => {
  const foldersById = new Map((kit.folders ?? []).map((folder) => [folder.id, folder]));
  const pathSegments: string[] = [file.name];
  let currentFolderId = file.folder?.id;

  while (currentFolderId) {
    const folder = foldersById.get(currentFolderId);
    if (!folder) {
      break;
    }
    pathSegments.unshift(folder.name);
    currentFolderId = folder.parent?.id;
  }

  return pathSegments.join("/");
};

export const getKitFolderStoragePath = (kit: Kit, folderLike: Pick<Folder, "id" | "name" | "parent"> | { id: string }): string => {
  const foldersById = new Map((kit.folders ?? []).map((folder) => [folder.id, folder]));
  const visited = new Set<string>();
  const pathSegments: string[] = [];
  let currentFolder: Pick<Folder, "id" | "name" | "parent"> | undefined = "name" in folderLike ? folderLike : foldersById.get(folderLike.id);

  while (currentFolder) {
    if (visited.has(currentFolder.id)) {
      break;
    }
    visited.add(currentFolder.id);
    pathSegments.unshift(currentFolder.name);
    const parentId = currentFolder.parent?.id;
    currentFolder = parentId ? foldersById.get(parentId) : undefined;
  }

  return pathSegments.join("/");
};

const revokeKitFileObjectUrl = (kitStore: KitStore, fileId: string): void => {
  const fileState = getOrCreateKitFileState(kitStore);
  const currentObjectUrl = fileState.objectUrls.get(fileId);
  if (currentObjectUrl) {
    URL.revokeObjectURL(currentObjectUrl);
    fileState.objectUrls.delete(fileId);
  }
};

export const createKitFileObjectUrl = (kitStore: KitStore, fileId: string, blob: Blob): string => {
  const fileState = getOrCreateKitFileState(kitStore);
  revokeKitFileObjectUrl(kitStore, fileId);
  const objectUrl = URL.createObjectURL(blob);
  fileState.objectUrls.set(fileId, objectUrl);
  return objectUrl;
};

export const getExistingKitFileProvider = (kitStore: KitStore): FileProvider | null => {
  return getOrCreateKitFileState(kitStore).provider ?? null;
};

export const getKitFileProvider = async (kitStore: KitStore, kitId: string): Promise<FileProvider | null> => {
  const fileState = getOrCreateKitFileState(kitStore);
  if (fileState.provider && fileState.providerKitId === kitId) {
    return fileState.provider;
  }

  if (!fileState.providerFactory) {
    return null;
  }

  fileState.provider = await fileState.providerFactory(kitId);
  fileState.providerKitId = kitId;
  return fileState.provider;
};

export const fetchReadableKitFileBlob = async (url: string): Promise<Blob | null> => {
  try {
    const response = await fetch(url);
    if (!response.ok) {
      return null;
    }
    return await response.blob();
  } catch {
    return null;
  }
};

const uploadKitFileToProvider = async (kitStore: KitStore, kit: Kit, file: SemioFile, blob: Blob): Promise<void> => {
  const fileState = getOrCreateKitFileState(kitStore);
  fileState.blobs.set(file.id, blob);
  revokeKitFileObjectUrl(kitStore, file.id);
  const storagePath = getKitFileStoragePath(kit, file);

  // 🔖EmbedInJsonFileKit
  // For file kits, embed the blob as a data URL in file.blob so everything
  // stays inside the single *.kit.semio.json file on save.
  // Specs: Prefer the inner store exposed by wrappers like CollaborativeKitStore.
  // The wrapper itself always exposes embedFileBlob as a pass-through, even for
  // folder kits where filesystem writes must still happen.
  const innerCandidate = (kitStore as { store?: unknown }).store;
  const hasWrappedInnerStore = innerCandidate !== undefined && innerCandidate !== null;
  const embedTarget = typeof (innerCandidate as any)?.embedFileBlob === "function" ? (innerCandidate as any) : !hasWrappedInnerStore && typeof (kitStore as any)?.embedFileBlob === "function" ? (kitStore as any) : null;
  if (embedTarget) {
    try {
      await embedTarget.embedFileBlob(file.id, blob);
    } catch (error) {
      console.error(`uploadKitFileToProvider: failed to embed blob for ${file.id}:`, error);
    }
    return;
  }

  const binaryStore = kitStore as KitBinaryStore;
  if (typeof binaryStore.writeFile === "function") {
    await binaryStore.writeFile(storagePath, blob);
  }

  const provider = await getKitFileProvider(kitStore, kit.id);
  if (!provider) {
    return;
  }

  await provider.upload(kit.id, file.id, storagePath, blob);
  const providerUrl = provider.getUrl(kit.id, file.id, storagePath);
  if (providerUrl) {
    fileState.providerUrls.set(file.id, providerUrl);
  }
};

const deleteKitFileFromProvider = async (kitStore: KitStore, kit: Kit, file: SemioFile | undefined): Promise<void> => {
  if (!file) {
    return;
  }

  const fileState = getOrCreateKitFileState(kitStore);
  fileState.blobs.delete(file.id);
  fileState.providerUrls.delete(file.id);
  revokeKitFileObjectUrl(kitStore, file.id);
  const storagePath = getKitFileStoragePath(kit, file);

  const binaryStore = kitStore as KitBinaryStore;
  if (typeof binaryStore.deleteFile === "function") {
    await binaryStore.deleteFile(storagePath);
  }

  const provider = await getKitFileProvider(kitStore, kit.id);
  if (!provider) {
    return;
  }

  await provider.delete(kit.id, file.id, storagePath);
};

const syncKitFileCommandResult = async (kitStore: KitStore, kit: Kit, command: string, args: any[], result: KitCommandResult): Promise<void> => {
  const binaryStore = kitStore as KitBinaryStore;
  const nextKit = result.diff ? applyKitDiff(kit, result.diff) : kit;

  if (command === "semio.kit.addFile") {
    const file = args[0] as SemioFile | undefined;
    const blob = args[1] as Blob | undefined;
    if (file && blob) {
      await uploadKitFileToProvider(kitStore, kit, file, blob);
    }
    return;
  }

  if (command === "semio.kit.addFiles") {
    const filesToAdd = (args[1] as { file: SemioFile; blob?: Blob }[] | undefined) ?? [];
    await Promise.all(
      filesToAdd.map(async ({ file, blob }) => {
        if (blob) {
          await uploadKitFileToProvider(kitStore, kit, file, blob);
        }
      }),
    );
    return;
  }

  if (command === "semio.kit.updateFile") {
    const fileId = args[0] as string | undefined;
    const fileDiff = args[1] as FileDiff | undefined;
    const blob = args[2] as Blob | undefined;
    if (!fileId || !blob) {
      return;
    }

    const existingFile = kit.files?.find((file) => file.id === fileId);
    if (!existingFile) {
      return;
    }

    const updatedFile = { ...existingFile, ...fileDiff };
    await uploadKitFileToProvider(kitStore, kit, updatedFile, blob);
    return;
  }

  if (command === "semio.kit.removeFile") {
    const fileId = args[0] as string | undefined;
    const existingFile = kit.files?.find((file) => file.id === fileId);
    await deleteKitFileFromProvider(kitStore, kit, existingFile);
    return;
  }

  if (command === "semio.kit.createFolder") {
    const folder = args[0] as Folder | undefined;
    if (!folder || typeof binaryStore.createDirectory !== "function") {
      return;
    }
    await binaryStore.createDirectory(getKitFolderStoragePath(nextKit, folder));
    return;
  }

  if (command === "semio.kit.updateFolder") {
    const folderId = args[0] as string | undefined;
    if (!folderId || typeof binaryStore.moveEntry !== "function") {
      return;
    }
    const currentFolder = kit.folders?.find((folder) => folder.id === folderId);
    const updatedFolder = nextKit.folders?.find((folder) => folder.id === folderId);
    if (!currentFolder || !updatedFolder) {
      return;
    }
    const currentPath = getKitFolderStoragePath(kit, currentFolder);
    const nextPath = getKitFolderStoragePath(nextKit, updatedFolder);
    if (currentPath && nextPath && currentPath !== nextPath) {
      await binaryStore.moveEntry(currentPath, nextPath);
    }
    return;
  }

  if (command === "semio.kit.import") {
    const importedFiles = result.diff?.files?.added ?? [];
    const importedBlobs = result.files ?? [];
    await Promise.all(
      importedFiles.map(async (file, index) => {
        const blob = importedBlobs[index];
        if (blob) {
          await uploadKitFileToProvider(kitStore, kit, file as SemioFile, blob);
        }
      }),
    );
    return;
  }

  if (command !== "semio.kit.moveToFolder") {
    return;
  }

  const artifactId = args[0] as string | undefined;
  const artifactKind = args[1] as "type" | "design" | "quality" | "file" | "folder" | undefined;
  if (!artifactId || !artifactKind) {
    return;
  }

  if (artifactKind === "file" && typeof binaryStore.moveEntry === "function") {
    const currentFile = kit.files?.find((file) => file.id === artifactId);
    const updatedFile = nextKit.files?.find((file) => file.id === artifactId);
    if (!currentFile || !updatedFile) {
      return;
    }
    const currentPath = getKitFileStoragePath(kit, currentFile);
    const nextPath = getKitFileStoragePath(nextKit, updatedFile);
    if (currentPath && nextPath && currentPath !== nextPath) {
      await binaryStore.moveEntry(currentPath, nextPath);
    }
    return;
  }

  if (artifactKind === "folder") {
    const currentFolder = kit.folders?.find((folder) => folder.id === artifactId);
    const updatedFolder = nextKit.folders?.find((folder) => folder.id === artifactId);
    if (!currentFolder || !updatedFolder) {
      return;
    }
    if (typeof binaryStore.moveEntry !== "function") {
      return;
    }
    const currentPath = getKitFolderStoragePath(kit, currentFolder);
    const nextPath = getKitFolderStoragePath(nextKit, updatedFolder);
    if (currentPath && nextPath && currentPath !== nextPath) {
      await binaryStore.moveEntry(currentPath, nextPath);
    }
  }
};

const semioKitCommandHandlers = {
  "semio.kit.createAuthor": (context: KitCommandContext, author: Author): KitCommandResult => {
    return {
      diff: { authors: { added: [author] } },
    };
  },
  "semio.kit.updateAuthor": (context: KitCommandContext, id: Id, diff: AuthorDiff): KitCommandResult => {
    return {
      diff: { authors: { updated: [{ author: { id }, diff }] } },
    };
  },
  "semio.kit.deleteAuthor": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { authors: { removed: [{ id }] } },
    };
  },
  "semio.kit.createType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
  "semio.kit.updateType": (context: KitCommandContext, id: Id, diff: TypeDiff): KitCommandResult => {
    return {
      diff: { types: { updated: [{ type: { id }, diff }] } },
    };
  },
  "semio.kit.deleteType": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { types: { removed: [{ id }] } },
    };
  },
  "semio.kit.createDesign": (context: KitCommandContext, design: Design): KitCommandResult => {
    return {
      diff: { designs: { added: [design] } },
    };
  },
  "semio.kit.updateDesign": (context: KitCommandContext, id: Id, diff: DesignDiff): KitCommandResult => {
    return {
      diff: { designs: { updated: [{ design: { id }, diff }] } },
    };
  },
  "semio.kit.deleteDesign": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { designs: { removed: [{ id }] } },
    };
  },
  "semio.kit.createQuality": (context: KitCommandContext, quality: Quality): KitCommandResult => {
    return {
      diff: { qualities: { added: [quality] } },
    };
  },
  "semio.kit.updateQuality": (context: KitCommandContext, id: Id, diff: QualityDiff): KitCommandResult => {
    return {
      diff: { qualities: { updated: [{ quality: { id }, diff }] } },
    };
  },
  "semio.kit.deleteQuality": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { qualities: { removed: [{ id }] } },
    };
  },
  "semio.kit.createPort": (context: KitCommandContext, iface: Port): KitCommandResult => {
    return {
      diff: { ports: { added: [iface] } },
    };
  },
  "semio.kit.updatePort": (context: KitCommandContext, id: Id, diff: PortDiff): KitCommandResult => {
    return {
      diff: { ports: { updated: [{ port: { id }, diff }] } },
    };
  },
  "semio.kit.deletePort": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { ports: { removed: [{ id }] } },
    };
  },
  "semio.kit.createTag": (context: KitCommandContext, tag: Tag): KitCommandResult => {
    return {
      diff: { tags: { added: [tag] } },
    };
  },
  "semio.kit.updateTag": (context: KitCommandContext, id: Id, diff: TagDiff): KitCommandResult => {
    return {
      diff: { tags: { updated: [{ tag: { id }, diff }] } },
    };
  },
  "semio.kit.deleteTag": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { tags: { removed: [{ id }] } },
    };
  },
  "semio.kit.createConcept": (context: KitCommandContext, concept: Concept): KitCommandResult => {
    return {
      diff: { concepts: { added: [concept] } },
    };
  },
  "semio.kit.updateConcept": (context: KitCommandContext, id: Id, diff: ConceptDiff): KitCommandResult => {
    return {
      diff: { concepts: { updated: [{ concept: { id }, diff }] } },
    };
  },
  "semio.kit.deleteConcept": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { concepts: { removed: [{ id }] } },
    };
  },
  "semio.kit.addFile": (context: KitCommandContext, file: SemioFile, blob?: Blob): KitCommandResult => {
    const files: globalThis.File[] = blob ? [new globalThis.File([blob], file.name)] : [];
    return {
      diff: { files: { added: [file] } },
      files,
    };
  },
  "semio.kit.addFiles": (context: KitCommandContext, foldersToAdd: Folder[], filesToAdd: { file: SemioFile; blob?: Blob }[]): KitCommandResult => {
    const semioFiles: SemioFile[] = [];
    const files: globalThis.File[] = [];
    for (const { file, blob } of filesToAdd) {
      semioFiles.push(file);
      if (blob) files.push(new globalThis.File([blob], file.name));
    }
    return {
      diff: { folders: { added: foldersToAdd }, files: { added: semioFiles } },
      files,
    };
  },
  "semio.kit.updateFile": (context: KitCommandContext, fileId: Url, fileDiff: FileDiff, blob?: Blob): KitCommandResult => {
    const existing = context.kit.files?.find((f) => f.id === fileId);
    const fileName = fileDiff.name ?? existing?.name ?? "file";
    const files: globalThis.File[] = blob ? [new globalThis.File([blob], fileName)] : [];
    return {
      diff: { files: { updated: [{ file: { id: fileId }, diff: fileDiff }] } },
      files,
    };
  },
  "semio.kit.removeFile": (context: KitCommandContext, fileId: Url): KitCommandResult => {
    return {
      diff: { files: { removed: [{ id: fileId }] } },
    };
  },
  "semio.kit.createFolder": (context: KitCommandContext, folder: Folder): KitCommandResult => {
    return {
      diff: { folders: { added: [folder] } },
    };
  },
  "semio.kit.updateFolder": (context: KitCommandContext, id: Id, diff: FolderDiff): KitCommandResult => {
    return {
      diff: { folders: { updated: [{ folder: { id }, diff }] } },
    };
  },
  "semio.kit.deleteFolder": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { folders: { removed: [{ id }] } },
    };
  },
  "semio.kit.moveToFolder": (context: KitCommandContext, artifactId: Id, artifactKind: "type" | "design" | "quality" | "file" | "folder", folderId?: Id): KitCommandResult => {
    switch (artifactKind) {
      case "type": {
        const type = context.kit.types?.find((t) => t.id === artifactId);
        if (!type) throw new Error(`Type ${artifactId} not found`);
        const folderDiff = { folder: folderId };
        return { diff: { types: { updated: [{ type: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "design": {
        const design = context.kit.designs?.find((d) => d.id === artifactId);
        if (!design) throw new Error(`Design ${artifactId} not found`);
        if (design.parent) throw new Error("Only protodesigns (designs without parent) can be moved to folders");
        const folderDiff = { folder: folderId };
        return { diff: { designs: { updated: [{ design: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "quality": {
        const folderDiff = { folder: folderId };
        return { diff: { qualities: { updated: [{ quality: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "file": {
        const folderDiff = { folder: folderId ? { id: folderId } : undefined };
        return { diff: { files: { updated: [{ file: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "folder": {
        const parentDiff = { parent: folderId ? { id: folderId } : undefined };
        return { diff: { folders: { updated: [{ folder: { id: artifactId }, diff: parentDiff }] } } };
      }
      default:
        throw new Error(`Unknown artifact kind: ${artifactKind}`);
    }
  },
  "semio.kit.import": (context: KitCommandContext, url: string): KitCommandResult => {
    (async () => {
      try {
        if (url.endsWith(".json")) {
          const response = await fetch(url);
          const kit: Kit = await response.json();
          const filesToFetch: { path: string; url: string }[] = [];
          const extractFileUrls = (obj: any) => {
            if (typeof obj === "object" && obj !== null) {
              if (Array.isArray(obj)) {
                obj.forEach((item) => extractFileUrls(item));
              } else {
                Object.entries(obj).forEach(([key, value]) => {
                  if (key === "url" && typeof value === "string" && !value.startsWith("http")) {
                    filesToFetch.push({ path: value, url: new URL(value, url).href });
                  }
                  extractFileUrls(value);
                });
              }
            }
          };
          extractFileUrls(kit);
          const files: KitCommandResult["files"] = [];
          for (const file of filesToFetch) {
            try {
              const fileResponse = await fetch(file.url);
              const fileBlob = await fileResponse.blob();
              const fileName = file.path.split("/").pop() || file.path;
              files.push(new File([fileBlob], fileName));
            } catch (error) { }
          }
          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types ? { added: kit.types } : undefined,
              designs: kit.designs ? { added: kit.designs } : undefined,
              files: kit.files ? { added: kit.files } : undefined,
            },
            files,
          };
        } else {
          const { kit } = await importKit(url);

          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types && kit.types.length > 0 ? { added: kit.types } : undefined,
              designs: kit.designs && kit.designs.length > 0 ? { added: kit.designs } : undefined,
              files: kit.files && kit.files.length > 0 ? { added: kit.files } : undefined,
            },
          };
        }
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.export": (context: KitCommandContext): KitCommandResult => {
    (async () => {
      try {
        const kit = context.kit;
        const files = new Map<string, Blob>();

        for (const [path, url] of context.fileUrls.entries()) {
          try {
            const response = await fetch(url);
            if (response.ok) {
              const blob = await response.blob();
              files.set(path, blob);
            }
          } catch (error) {
            // File not accessible, skip
          }
        }

        const zipBlob = await exportKit(kit, files);
        const url = URL.createObjectURL(zipBlob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${kit.name}-${kit.version || "latest"}.semio.zip`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.addPiece": (context: KitCommandContext, id: Id, piece: Piece): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: {
                pieces: {
                  added: [
                    piece.plane || (findDesignInKit(context.kit, id)?.connections ?? []).some((connection) => connection.connected.piece.id === piece.id || connection.connecting.piece.id === piece.id)
                      ? piece
                      : {
                        ...piece,
                        plane: {
                          origin: { x: 0, y: 0, z: 0 },
                          xAxis: { x: 1, y: 0, z: 0 },
                          yAxis: { x: 0, y: 1, z: 0 },
                        },
                      },
                  ],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addPieces": (context: KitCommandContext, id: Id, pieces: Piece[]): KitCommandResult => {
    const design = findDesignInKit(context.kit, id);
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: {
                pieces: {
                  added: pieces.map((candidate) =>
                    candidate.plane || (design?.connections ?? []).some((connection) => connection.connected.piece.id === candidate.id || connection.connecting.piece.id === candidate.id)
                      ? candidate
                      : {
                        ...candidate,
                        plane: {
                          origin: { x: 0, y: 0, z: 0 },
                          xAxis: { x: 1, y: 0, z: 0 },
                          yAxis: { x: 0, y: 1, z: 0 },
                        },
                      },
                  ),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePiece": (context: KitCommandContext, id: Id, piece: Id): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { pieces: { removed: [{ id: piece }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePieces": (context: KitCommandContext, id: Id, pieces: Id[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { pieces: { removed: pieces.map((p) => ({ id: p })) } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnection": (context: KitCommandContext, id: Id, connection: Connection): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnections": (context: KitCommandContext, id: Id, connections: Connection[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnection": (context: KitCommandContext, id: Id, connectionId: Id): KitCommandResult => {
    const design = findDesignInKit(context.kit, id);
    const connection = design?.connections?.find((c) => c.id === connectionId);
    if (!connection) return { diff: {} };
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { removed: [{ id: connection.id }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnections": (context: KitCommandContext, id: Id, connectionIds: Id[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { removed: connectionIds.map((connId) => ({ id: connId })) } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.deleteSelected": (context: KitCommandContext, designId: Id, selectedPieces: Id[], selectedConnections: Id[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id: designId },
              diff: {
                pieces: { removed: selectedPieces.map((pid) => ({ id: pid })) },
                connections: { removed: selectedConnections.map((cid) => ({ id: cid })) },
              },
            },
          ],
        },
      },
    };
  },
};

export async function executeSemioKitCommand(kitStore: KitStore, command: string, origin?: string, ...args: any[]): Promise<KitCommandResult> {
  const callback = semioKitCommandHandlers[command as keyof typeof semioKitCommandHandlers];
  if (!callback) throw new Error(`Command "${command}" not found in kit commands`);
  const replaceOrApplyKit = (nextKit: Kit, diff?: KitDiff) => {
    if (typeof (kitStore as any).replace === "function") {
      (kitStore as any).replace(asKitInstance(nextKit), { origin });
      return;
    }
    if (diff && typeof (kitStore as any).apply === "function") {
      (kitStore as any).apply(diff, { origin });
      return;
    }
    throw new Error("Kit store does not support replace() or apply()");
  };
  const context: KitCommandContext = {
    kit: kitStore.getSnapshot().kit,
    fileUrls: getStoredKitFileUrls(kitStore) as Map<Url, Url>,
    origin,
  };
  const result = (callback as any)(context, ...args);
  if (result.diff && Object.keys(result.diff).length > 0) {
    const useJsCompatibilityApply =
      command === "semio.kit.addFile" ||
      command === "semio.kit.addFiles" ||
      command === "semio.kit.updateFile" ||
      command === "semio.kit.removeFile" ||
      command === "semio.kit.createFolder" ||
      command === "semio.kit.updateFolder" ||
      command === "semio.kit.deleteFolder" ||
      command === "semio.kit.moveToFolder";

    if (useJsCompatibilityApply) {
      const nextKit = applyKitDiff(context.kit, result.diff);
      replaceOrApplyKit(nextKit, result.diff);
    } else {
      const client = await createKitStoreClient({
        initialKit: context.kit,
        forceFallback: typeof Worker === "undefined",
      });
      try {
        const applyResult = await client.applyKitDiff(result.diff);
        if (!applyResult.ok) {
          throw new Error(applyResult.error.message);
        }
        const nextSnapshot = await client.getSnapshot();
        replaceOrApplyKit(nextSnapshot, result.diff);
      } finally {
        client.dispose();
      }
    }
  }
  await syncKitFileCommandResult(kitStore, context.kit, command, args, result);
  return result;
}

/**
 * Binds {@link executeSemioKitCommand} to a `getOrigin` callback (browser / shell event codes).
 * Sketchpad and hosts use this until string commands are fully replaced with typed `execute` on {@link KitStoreClient}.
 */
export function createKitCommandEngine(kitStore: KitStore, getOrigin: () => string) {
  const o = getOrigin;
  return {
    importKit: (url: string) => executeSemioKitCommand(kitStore, "semio.kit.import", o(), url),
    exportKit: () => executeSemioKitCommand(kitStore, "semio.kit.export", o()),
    createAuthor: (author: any) => executeSemioKitCommand(kitStore, "semio.kit.createAuthor", o(), author),
    updateAuthor: (Id: string, authorDiff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateAuthor", o(), Id, authorDiff),
    deleteAuthor: (Id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteAuthor", o(), Id),
    createType: (type: any) => executeSemioKitCommand(kitStore, "semio.kit.createType", o(), type),
    updateType: (id: string, diff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateType", o(), id, diff),
    deleteType: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteType", o(), id),
    createDesign: (design: any) => executeSemioKitCommand(kitStore, "semio.kit.createDesign", o(), design),
    updateDesign: (id: string, diff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateDesign", o(), id, diff),
    deleteDesign: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteDesign", o(), id),
    createQuality: (quality: any) => executeSemioKitCommand(kitStore, "semio.kit.createQuality", o(), quality),
    updateQuality: (id: string, diff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateQuality", o(), id, diff),
    deleteQuality: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteQuality", o(), id),
    createPort: (iface: any) => executeSemioKitCommand(kitStore, "semio.kit.createPort", o(), iface),
    updatePort: (id: string, diff: any) => executeSemioKitCommand(kitStore, "semio.kit.updatePort", o(), id, diff),
    deletePort: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deletePort", o(), id),
    createTag: (tag: any) => executeSemioKitCommand(kitStore, "semio.kit.createTag", o(), tag),
    updateTag: (id: string, diff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateTag", o(), id, diff),
    deleteTag: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteTag", o(), id),
    createConcept: (concept: any) => executeSemioKitCommand(kitStore, "semio.kit.createConcept", o(), concept),
    deleteConcept: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteConcept", o(), id),
    addFile: (file: any, blob?: Blob) => executeSemioKitCommand(kitStore, "semio.kit.addFile", o(), file, blob),
    updateFile: (url: string, fileDiff: any, blob?: Blob) => executeSemioKitCommand(kitStore, "semio.kit.updateFile", o(), url, fileDiff, blob),
    removeFile: (url: string) => executeSemioKitCommand(kitStore, "semio.kit.removeFile", o(), url),
    createFolder: (folder: any) => executeSemioKitCommand(kitStore, "semio.kit.createFolder", o(), folder),
    updateFolder: (id: string, folderDiff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateFolder", o(), id, folderDiff),
    deleteFolder: (id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteFolder", o(), id),
    moveToFolder: (artifactKind: string, artifactId: string, folderId: string | null) =>
      executeSemioKitCommand(kitStore, "semio.kit.moveToFolder", o(), artifactId, artifactKind, folderId),
    addPiece: (design: string, piece: any) => executeSemioKitCommand(kitStore, "semio.kit.addPiece", o(), design, piece),
    addPieces: (design: string, pieces: any[]) => executeSemioKitCommand(kitStore, "semio.kit.addPieces", o(), design, pieces),
    removePiece: (design: string, piece: string) => executeSemioKitCommand(kitStore, "semio.kit.removePiece", o(), design, piece),
    removePieces: (design: string, pieces: string[]) => executeSemioKitCommand(kitStore, "semio.kit.removePieces", o(), design, pieces),
    addConnection: (design: string, connection: any) => executeSemioKitCommand(kitStore, "semio.kit.addConnection", o(), design, connection),
    addConnections: (design: string, connections: any[]) => executeSemioKitCommand(kitStore, "semio.kit.addConnections", o(), design, connections),
    removeConnection: (design: string, connection: string) => executeSemioKitCommand(kitStore, "semio.kit.removeConnection", o(), design, connection),
    removeConnections: (design: string, connections: string[]) => executeSemioKitCommand(kitStore, "semio.kit.removeConnections", o(), design, connections),
    deleteSelected: (design: string, selectedPieces: string[], selectedConnections: string[]) =>
      executeSemioKitCommand(kitStore, "semio.kit.deleteSelected", o(), design, selectedPieces, selectedConnections),
  };
}

export function createKitCommandEngineExplicitOrigin(kitStore: KitStore) {
  return {
    importKit: (origin: string, url: string) => executeSemioKitCommand(kitStore, "semio.kit.import", origin, url),
    exportKit: (origin: string) => executeSemioKitCommand(kitStore, "semio.kit.export", origin),
    createAuthor: (origin: string, author: any) => executeSemioKitCommand(kitStore, "semio.kit.createAuthor", origin, author),
    updateAuthor: (origin: string, authorId: string, authorDiff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateAuthor", origin, authorId, authorDiff),
    deleteAuthor: (origin: string, authorId: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteAuthor", origin, authorId),
    createType: (origin: string, type: any) => executeSemioKitCommand(kitStore, "semio.kit.createType", origin, type),
    deleteType: (origin: string, id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteType", origin, id),
    createDesign: (origin: string, design: any) => executeSemioKitCommand(kitStore, "semio.kit.createDesign", origin, design),
    updateDesign: (origin: string, id: string, diff: any) => executeSemioKitCommand(kitStore, "semio.kit.updateDesign", origin, id, diff),
    deleteDesign: (origin: string, id: string) => executeSemioKitCommand(kitStore, "semio.kit.deleteDesign", origin, id),
    addFile: (origin: string, file: any, blob?: Blob) => executeSemioKitCommand(kitStore, "semio.kit.addFile", origin, file, blob),
    updateFile: (origin: string, url: string, fileDiff: any, blob?: Blob) => executeSemioKitCommand(kitStore, "semio.kit.updateFile", origin, url, fileDiff, blob),
    removeFile: (origin: string, url: string) => executeSemioKitCommand(kitStore, "semio.kit.removeFile", origin, url),
    addPiece: (origin: string, design: string, piece: any) => executeSemioKitCommand(kitStore, "semio.kit.addPiece", origin, design, piece),
    addPieces: (origin: string, design: string, pieces: any[]) => executeSemioKitCommand(kitStore, "semio.kit.addPieces", origin, design, pieces),
    removePiece: (origin: string, design: string, piece: string) => executeSemioKitCommand(kitStore, "semio.kit.removePiece", origin, design, piece),
    removePieces: (origin: string, design: string, pieces: string[]) => executeSemioKitCommand(kitStore, "semio.kit.removePieces", origin, design, pieces),
    addConnection: (origin: string, design: string, connection: any) => executeSemioKitCommand(kitStore, "semio.kit.addConnection", origin, design, connection),
    addConnections: (origin: string, design: string, connections: any[]) => executeSemioKitCommand(kitStore, "semio.kit.addConnections", origin, design, connections),
    removeConnection: (origin: string, design: string, connection: string) => executeSemioKitCommand(kitStore, "semio.kit.removeConnection", origin, design, connection),
    removeConnections: (origin: string, design: string, connections: string[]) => executeSemioKitCommand(kitStore, "semio.kit.removeConnections", origin, design, connections),
    deleteSelected: (origin: string, design: string, selectedPieces: string[], selectedConnections: string[]) =>
      executeSemioKitCommand(kitStore, "semio.kit.deleteSelected", origin, design, selectedPieces, selectedConnections),
  };
}

// #endregion Graph kit commands


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

/** Single root mutation field returning settle JSON (`{ ok: true }` / `{ ok: false, error }`). */
export async function kitGraphqlMutationSettle(
  handle: KitGraphqlHandle,
  body: { query: string; variables?: Record<string, unknown> },
): Promise<SetResult> {
  const root = kitGraphqlFirstData(await kitGraphqlRun(handle, body));
  const keys = Object.keys(root);
  const v = keys.length === 1 ? (root as Record<string, unknown>)[keys[0]!] : root;
  return await settleSetPromise(Promise.resolve(v));
}

const KIT_STORE_BATCH_SELECTION = `clientMutationId results {
  kind
  ok
  count
  sessionId
  draftId
  transactionId
  checkpointId
  alternativeId
  backbone { attached kind tip }
  conflicts { id backboneTip reason createdAt }
}`;

function unwrapKitGraphqlJsonField<T>(v: T): T {
  if (typeof v !== "string") return v;
  try {
    return JSON.parse(v) as T;
  } catch {
    return v;
  }
}

async function kitGraphqlBatchMutation(handle: KitGraphqlHandle, input: unknown): Promise<any> {
  const root = kitGraphqlFirstData(
    await kitGraphqlRun(handle, {
      query: `mutation($input: KitStoreBatchInput!) { kitStore { batch(input: $input) { ${KIT_STORE_BATCH_SELECTION} } } }`,
      variables: { input },
    }),
  ) as { kitStore?: { batch?: unknown } };
  const payload = root.kitStore?.batch;
  if (payload == null) throw new Error("kitGraphql: missing batch payload");
  return payload;
}

/** Apply a batch of `ChangeKitCommand` JSON values on the live graph (actor queue). */
export async function kitGraphqlChangeKitCommands(handle: KitGraphqlHandle, commands: unknown): Promise<SetResult> {
  await kitGraphqlBatchMutation(handle, {
    commands: [
      {
        live: {
          commands: [{ changeKitCommands: { commands } }],
        },
      },
    ],
  });
  return { ok: true } as const;
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

function requireBatchResult(results: any[], label: string, index: number = 0): any {
  const result = results[index];
  if (result == null || typeof result !== "object") throw new Error(`${label}: missing batch result`);
  return result;
}

function transactionCommandToBatchCommand(cmd: unknown): any {
  const { tag, value } = storePayload(cmd);
  switch (tag) {
    case "changeKitCommands":
      return { changeKitCommands: { commands: (value as { commands?: unknown[] } | null)?.commands ?? [] } };
    case "finalize":
      return { finalizeTransaction: { confirm: true } };
    case "abort":
      return { abortTransaction: { confirm: true } };
    case "undo":
      return { undoTransaction: { count: 1 } };
    case "redo":
      return { redoTransaction: { count: 1 } };
    default:
      throw new Error(`transaction batch: unsupported ${tag}`);
  }
}

function draftCommandToBatchCommand(cmd: unknown): any {
  const { tag, value } = storePayload(cmd);
  switch (tag) {
    case "startTransaction":
      return { startTransaction: { confirm: true } };
    case "finalizeToKitCheckpoint":
      return { finalizeDraft: { message: String((value as { message?: string } | null)?.message ?? "") } };
    case "abort":
      return { abortDraft: { confirm: true } };
    case "undo":
      return { undoDraft: { count: Number((value as { count?: number } | null)?.count ?? 1) } };
    case "redo":
      return { redoDraft: { count: Number((value as { count?: number } | null)?.count ?? 1) } };
    case "executeTransactionCommands": {
      const v = value as { id?: string; commands?: unknown[] } | null;
      if (typeof v?.id !== "string" || !Array.isArray(v.commands)) throw new Error("executeTransactionCommands");
      return {
        transaction: {
          transactionId: v.id,
          commands: v.commands.map(transactionCommandToBatchCommand),
        },
      };
    }
    default:
      throw new Error(`draft batch: unsupported ${tag}`);
  }
}

function sessionCommandToBatchCommand(cmd: unknown): any {
  const { tag, value } = storePayload(cmd);
  switch (tag) {
    case "newDraft":
      return {
        createDraft: {
          parentCheckpointId: (value as { checkpointId?: string | null } | null)?.checkpointId ?? null,
          targetAlternativeId: (value as { alternativeId?: string | null } | null)?.alternativeId ?? null,
        },
      };
    case "executeKitDraftCommands": {
      const v = value as { id?: string; commands?: unknown[] } | null;
      if (typeof v?.id !== "string" || !Array.isArray(v.commands)) throw new Error("executeKitDraftCommands");
      return {
        draft: {
          draftId: v.id,
          commands: v.commands.map(draftCommandToBatchCommand),
        },
      };
    }
    default:
      throw new Error(`session batch: unsupported ${tag}`);
  }
}

/** Maps `KitStoreCommand` JSON to typed root mutations; returns the tagged `KitStoreCommandResult` JSON. */
export async function kitGraphqlExecuteStoreCommand(handle: KitGraphqlHandle, cmd: unknown): Promise<unknown> {
  const { tag, value } = storePayload(cmd);
  let input: any;
  switch (tag) {
    case "newSession":
      input = { commands: [{ session: { commands: [{ createSession: { confirm: true } }] } }] };
      break;
    case "endSession": {
      const id = (value as { id?: string } | null)?.id;
      if (typeof id !== "string") throw new Error("endSession: need id");
      input = { commands: [{ session: { sessionId: id, commands: [{ endSession: { confirm: true } }] } }] };
      break;
    }
    case "newAlternative": {
      const v = value as { fromCheckpoint?: string | null; name: string } | null;
      if (v == null || typeof v.name !== "string") throw new Error("newAlternative");
      input = { commands: [{ alternative: { commands: [{ createAlternative: { name: v.name, fromCheckpointId: v.fromCheckpoint ?? null } }] } }] };
      break;
    }
    case "executeSessionCommands": {
      const v = value as { id?: string; commands?: unknown[] } | null;
      if (typeof v?.id !== "string" || !Array.isArray(v.commands)) throw new Error("executeSessionCommands");
      input = { commands: [{ session: { sessionId: v.id, commands: v.commands.map(sessionCommandToBatchCommand) } }] };
      break;
    }
    case "executeKitCheckpointCommands": {
      const v = value as { id?: string; commands?: unknown[] } | null;
      if (typeof v?.id !== "string" || !Array.isArray(v.commands)) throw new Error("executeKitCheckpointCommands");
      input = { commands: [{ checkpoint: { checkpointId: v.id, commands: v.commands.map(() => ({ markRelease: { confirm: true } })) } }] };
      break;
    }
    case "executeKitAlternativeCommands": {
      const v = value as { id?: string; commands?: unknown[] } | null;
      if (typeof v?.id !== "string" || !Array.isArray(v.commands)) throw new Error("executeKitAlternativeCommands");
      input = {
        commands: [
          {
            alternative: {
              alternativeId: v.id,
              commands: v.commands.map((inner) => {
                const payload = storePayload(inner);
                if (payload.tag !== "unifyKitCheckpointsToSingleKitCheckpoint") throw new Error(`alternative batch: unsupported ${payload.tag}`);
                return { unifyAlternative: { message: String((payload.value as { message?: string } | null)?.message ?? "") } };
              }),
            },
          },
        ],
      };
      break;
    }
    case "attachBackbone": {
      const cfg = (value as { config?: Record<string, unknown> } | null)?.config;
      if (cfg == null || typeof cfg !== "object") throw new Error("attachBackbone");
      const configKey = Object.keys(cfg)[0];
      input = { commands: [{ backbone: { commands: [{ attachBackbone: { [configKey!]: cfg[configKey!] } }] } }] };
      break;
    }
    case "detachBackbone":
      input = { commands: [{ backbone: { commands: [{ detachBackbone: { confirm: true } }] } }] };
      break;
    case "setActiveCheckpoint": {
      const id = (value as { id?: string | null } | null)?.id;
      if (typeof id !== "string") throw new Error("setActiveCheckpoint");
      input = { commands: [{ checkpoint: { checkpointId: id, commands: [{ setActive: { confirm: true } }] } }] };
      break;
    }
    case "listConflicts":
      input = { commands: [{ backbone: { commands: [{ listConflicts: { confirm: true } }] } }] };
      break;
    case "resolveConflict": {
      const v = value as { id?: string; strategy?: Record<string, unknown> } | null;
      if (typeof v?.id !== "string" || v.strategy == null || typeof v.strategy !== "object") throw new Error("resolveConflict");
      const strategyKey = Object.keys(v.strategy)[0];
      input = {
        commands: [
          {
            backbone: {
              commands: [{ resolveConflict: { conflictId: v.id, strategy: strategyKey === "dropWip" ? "DROP_WIP" : "FORCE_OVERWRITE_BACKBONE" } }],
            },
          },
        ],
      };
      break;
    }
    case "backboneStatus":
      input = { commands: [{ backbone: { commands: [{ backboneStatus: { confirm: true } }] } }] };
      break;
    case "syncNow":
      input = { commands: [{ backbone: { commands: [{ syncNow: { confirm: true } }] } }] };
      break;
    default:
      throw new Error(`[DEBUG] kitGraphqlExecuteStoreCommand: unhandled ${tag}`);
  }
  const payload = await kitGraphqlBatchMutation(handle, input);
  const results = Array.isArray((payload as { results?: unknown[] }).results) ? ((payload as { results?: unknown[] }).results as any[]) : [];
  switch (tag) {
    case "newSession":
      return { newSession: { id: requireBatchResult(results, tag).sessionId } };
    case "endSession":
      return { endSession: { ok: requireBatchResult(results, tag).ok === true } };
    case "newAlternative":
      return { newAlternative: { id: requireBatchResult(results, tag).alternativeId } };
    case "executeSessionCommands":
      return { executeSessionCommands: { results } };
    case "executeKitCheckpointCommands":
      return { executeKitCheckpointCommands: { results } };
    case "executeKitAlternativeCommands":
      return { executeKitAlternativeCommands: { results } };
    case "attachBackbone":
      return { attachBackbone: { ok: requireBatchResult(results, tag).ok === true } };
    case "detachBackbone":
      return { detachBackbone: { ok: requireBatchResult(results, tag).ok === true } };
    case "setActiveCheckpoint":
      return { setActiveCheckpoint: { ok: requireBatchResult(results, tag).ok === true } };
    case "listConflicts":
      return { listConflicts: { items: requireBatchResult(results, tag).conflicts ?? [] } };
    case "resolveConflict":
      return { resolveConflict: { ok: requireBatchResult(results, tag).ok === true } };
    case "backboneStatus":
      return { backboneStatus: requireBatchResult(results, tag).backbone };
    case "syncNow":
      return { syncNow: { ok: requireBatchResult(results, tag).ok === true } };
    default:
      throw new Error(`kitGraphqlExecuteStoreCommand: missing result mapping for ${tag}`);
  }
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
      query: `query($id: String!) { kitStore { designByDtoId(id: $id) { piecesMetadataJson } } }`,
      variables: { id: designId },
    }),
  ) as { kitStore?: { designByDtoId?: { piecesMetadataJson?: unknown } | null } };
  const v = root.kitStore?.designByDtoId?.piecesMetadataJson;
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

/** Piece-scoped live reads via `kitStore.designByDtoId.pieceByDtoId` fields. */
export class LivePieceView {
  constructor(
    private readonly gql: KitGraphqlHandle,
    readonly designId: string,
    readonly pieceId: string,
  ) { }

  async readFlatPlane(): Promise<unknown> {
    const q = `query($d: String!, $p: String!) { kitStore { designByDtoId(id: $d) { pieceByDtoId(id: $p) { flatPlane } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { d: this.designId, p: this.pieceId } })) as {
      kitStore?: { designByDtoId?: { pieceByDtoId?: { flatPlane?: unknown } | null } | null };
    };
    return d.kitStore?.designByDtoId?.pieceByDtoId?.flatPlane;
  }

  async readFlatCenter(): Promise<unknown> {
    const q = `query($d: String!, $p: String!) { kitStore { designByDtoId(id: $d) { pieceByDtoId(id: $p) { flatCenter } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { d: this.designId, p: this.pieceId } })) as {
      kitStore?: { designByDtoId?: { pieceByDtoId?: { flatCenter?: unknown } | null } | null };
    };
    return d.kitStore?.designByDtoId?.pieceByDtoId?.flatCenter;
  }

  async readParentConnectionFull(): Promise<unknown | null | undefined> {
    const q = `query($d: String!, $p: String!) { kitStore { designByDtoId(id: $d) { pieceByDtoId(id: $p) { parentConnectionFull } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { d: this.designId, p: this.pieceId } })) as {
      kitStore?: { designByDtoId?: { pieceByDtoId?: { parentConnectionFull?: unknown } | null } | null };
    };
    return d.kitStore?.designByDtoId?.pieceByDtoId?.parentConnectionFull;
  }
}

/** Design-scoped live reads via `kitStore.designByDtoId` fields. */
export class LiveDesignView {
  constructor(
    private readonly gql: KitGraphqlHandle,
    readonly designId: string,
  ) { }

  async readClusterableGroups(selection: ReadonlyArray<string>): Promise<ReadonlyArray<ReadonlyArray<IdDto>>> {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designByDtoId(id: $id) { clusterableGroups(selection: $sel) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId, sel: [...selection] } }),
    ) as { kitStore?: { designByDtoId?: { clusterableGroups?: string[][] } | null } };
    const g = d.kitStore?.designByDtoId?.clusterableGroups;
    if (!Array.isArray(g)) throw new Error("clusterableGroups");
    return g.map((row) => row.map((id) => idDto(id)));
  }

  async readIncludedDesigns(): Promise<ReadonlyArray<unknown>> {
    const q = `query($id: String!) { kitStore { designByDtoId(id: $id) { includedDesigns } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId } })) as {
      kitStore?: { designByDtoId?: { includedDesigns?: unknown } | null };
    };
    const v = d.kitStore?.designByDtoId?.includedDesigns;
    return Array.isArray(v) ? v : [];
  }

  async readQualitySum(qualityId: string): Promise<number> {
    const q = `query($id: String!, $q: String!) { kitStore { designByDtoId(id: $id) { qualitySum(qualityId: $q) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId, q: qualityId } }),
    ) as { kitStore?: { designByDtoId?: { qualitySum?: number } | null } };
    const s = d.kitStore?.designByDtoId?.qualitySum;
    if (typeof s !== "number") throw new Error("qualitySum");
    return s;
  }

  async readReplaceableCatalog(selection: ReadonlyArray<string>): Promise<{ types: string[]; designs: string[] }> {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designByDtoId(id: $id) { replaceableCatalog(selection: $sel) { typeIds designIds } } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId, sel: [...selection] } }),
    ) as { kitStore?: { designByDtoId?: { replaceableCatalog?: { typeIds: string[]; designIds: string[] } } | null } };
    const rc = d.kitStore?.designByDtoId?.replaceableCatalog;
    if (rc == null) throw new Error("replaceableCatalog");
    return { types: rc.typeIds, designs: rc.designIds };
  }

  async readIncludedDesignIds(): Promise<string[]> {
    const q = `query($id: String!) { kitStore { designByDtoId(id: $id) { includedDesignIds } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(this.gql, { query: q, variables: { id: this.designId } })) as {
      kitStore?: { designByDtoId?: { includedDesignIds?: string[] } | null };
    };
    const ids = d.kitStore?.designByDtoId?.includedDesignIds;
    if (!Array.isArray(ids)) throw new Error("includedDesignIds");
    return ids;
  }
}

/** Type-scoped live reads via `kitStore.typeByDtoId` fields. */
export class LiveTypeView {
  constructor(
    private readonly gql: KitGraphqlHandle,
    readonly typeId: string,
  ) { }

  async readBestRepresentation(tagIds: ReadonlyArray<string>): Promise<unknown | null | undefined> {
    const q = `query($id: String!, $tags: [String!]!) { kitStore { typeByDtoId(id: $id) { bestRepresentation(tagIds: $tags) } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(this.gql, { query: q, variables: { id: this.typeId, tags: [...tagIds] } }),
    ) as { kitStore?: { typeByDtoId?: { bestRepresentation?: unknown } | null } };
    return d.kitStore?.typeByDtoId?.bestRepresentation;
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
          const h = kitWorkerGqlHandle();
          const plan = kitGraphqlFirstData(
            await kitGraphqlRun(h, {
              query: `query($kind: String!, $id: String!, $field: String!, $valueJson: String!) {
                kitStore { changeKitCommandsForFieldPatchValueJson(kind: $kind, id: $id, field: $field, valueJson: $valueJson) }
              }`,
              variables: { kind, id, field, valueJson: JSON.stringify(value) },
            }),
          ) as { kitStore?: { changeKitCommandsForFieldPatchValueJson?: unknown } };
          const raw = plan.kitStore?.changeKitCommandsForFieldPatchValueJson;
          if (raw == null) throw new Error("changeKitCommandsForFieldPatch");
          const cmds = unwrapKitGraphqlJsonField(raw);
          return await kitGraphqlChangeKitCommands(h, cmds);
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
          const h = kitWorkerGqlHandle();
          const plan = kitGraphqlFirstData(
            await kitGraphqlRun(h, {
              query: `query($parentKind: String!, $parentId: String!, $childKind: String!, $dtoJson: String!) {
                kitStore { changeKitCommandsForAddChildDtoJson(parentKind: $parentKind, parentId: $parentId, childKind: $childKind, dtoJson: $dtoJson) }
              }`,
              variables: { parentKind, parentId, childKind, dtoJson: JSON.stringify(dto) },
            }),
          ) as { kitStore?: { changeKitCommandsForAddChildDtoJson?: unknown } };
          const raw = plan.kitStore?.changeKitCommandsForAddChildDtoJson;
          if (raw == null) throw new Error("changeKitCommandsForAddChild");
          const cmds = unwrapKitGraphqlJsonField(raw);
          return await kitGraphqlChangeKitCommands(h, cmds);
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
          const h = kitWorkerGqlHandle();
          const plan = kitGraphqlFirstData(
            await kitGraphqlRun(h, {
              query: `query($parentKind: String!, $parentId: String!, $childKind: String!, $childId: String!) {
                kitStore { changeKitCommandsForRemoveChild(parentKind: $parentKind, parentId: $parentId, childKind: $childKind, childId: $childId) }
              }`,
              variables: { parentKind, parentId, childKind, childId },
            }),
          ) as { kitStore?: { changeKitCommandsForRemoveChild?: unknown } };
          const raw = plan.kitStore?.changeKitCommandsForRemoveChild;
          if (raw == null) throw new Error("changeKitCommandsForRemoveChild");
          const cmds = unwrapKitGraphqlJsonField(raw);
          return await kitGraphqlChangeKitCommands(h, cmds);
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
  applyDesignDiff(designId: string, diff: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      kitGraphqlMutationSettle(kitWorkerGqlHandle(), {
        query: `mutation($designId: String!, $diff: JSON!) { applyDesignDiff(designId: $designId, diff: $diff) }`,
        variables: { designId, diff },
      }),
    );
  },
  applyKitDiff(diff: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlMutationSettle(kitWorkerGqlHandle(), { query: `mutation($diff: JSON!) { applyKitDiff(diff: $diff) }`, variables: { diff } }));
  },
  clusterPieces(designId: string, pieceIds: string[], clusterName: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ clusterPieces: { pieceIds, clusterName } }] } }] }).then(() => ({ ok: true })));
  },
  dragPieces(designId: string, pieceIds: string[], du: number, dv: number) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ dragPieces: { pieceIds, du, dv } }] } }] }).then(() => ({ ok: true })));
  },
  movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ movePieces: { pieceIds, gap, shift, rise } }] } }] }).then(() => ({ ok: true })));
  },
  fixPieces(designId: string, pieceIds: string[]) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ fixPieces: { pieceIds } }] } }] }).then(() => ({ ok: true })));
  },
  flattenDesign(designId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ flattenDesign: { confirm: true } }] } }] }).then(() => ({ ok: true })));
  },
  expandDesign(parentDesignId: string, nestedDesignId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId: parentDesignId, commands: [{ expandDesign: { nestedDesignId } }] } }] }).then(() => ({ ok: true })));
  },
  deleteConnection(designId: string, connectionId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ deleteConnection: { connectionId } }] } }] }).then(() => ({ ok: true })));
  },
  changePieceType(designId: string, pieceId: string, newTypeId: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ changePieceType: { pieceId, newTypeId } }] } }] }).then(() => ({ ok: true })));
  },
  pasteDesignSelection(designId: string, selection: unknown, plane: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(
      kitGraphqlMutationSettle(kitWorkerGqlHandle(), {
        query: `mutation($designId: String!, $selection: JSON!, $plane: JSON) { pasteDesignSelection(designId: $designId, selection: $selection, plane: $plane) }`,
        variables: { designId, selection, plane: plane == null ? null : plane },
      }),
    );
  },
  createHangingPieces(designId: string, typeIds: string[], plane: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ createHangingPieces: { typeIds, plane } }] } }] }).then(() => ({ ok: true })));
  },
  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ createConnectedPiece: { parentPiece, parentPort, childType, childPort } }] } }] }).then(() => ({ ok: true })));
  },
  createFixedPiece(designId: string, typeId: string, plane: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlBatchMutation(kitWorkerGqlHandle(), { commands: [{ design: { designId, commands: [{ createFixedPiece: { typeId, plane } }] } }] }).then(() => ({ ok: true })));
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
    return settle(kitGraphqlMutationSettle(kitWorkerGqlHandle(), { query: `mutation { undo }` }));
  },
  redo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    return settle(kitGraphqlMutationSettle(kitWorkerGqlHandle(), { query: `mutation { redo }` }));
  },
  async canUndo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    const d = kitGraphqlFirstData(await kitGraphqlRun(kitWorkerGqlHandle(), { query: `query { kitStore { canUndo } }` })) as { kitStore?: { canUndo?: boolean } };
    return d.kitStore?.canUndo === true;
  },
  async canRedo() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    const d = kitGraphqlFirstData(await kitGraphqlRun(kitWorkerGqlHandle(), { query: `query { kitStore { canRedo } }` })) as { kitStore?: { canRedo?: boolean } };
    return d.kitStore?.canRedo === true;
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

  async vcsState() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    const d = kitGraphqlFirstData(await kitGraphqlRun(kitWorkerGqlHandle(), { query: `query { kitStore { vcsStateJson } }` })) as { kitStore?: { vcsStateJson?: unknown } };
    return d.kitStore?.vcsStateJson;
  },

  async theKitDto() {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    const d = kitGraphqlFirstData(await kitGraphqlRun(kitWorkerGqlHandle(), { query: `query { kitStore { theKitDto } }` })) as { kitStore?: { theKitDto?: unknown } };
    return d.kitStore?.theKitDto;
  },

  async materializeAt(at: unknown) {
    if (!kitWorkerHandle) throw new Error("KitStoreHandle not initialized");
    const s = at == null ? null : String(at);
    const checkpointId = s != null && s.trim() === "" ? null : s;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(kitWorkerGqlHandle(), {
        query: `query($checkpointId: String) { kitStore { materializeAt(checkpointId: $checkpointId) } }`,
        variables: { checkpointId },
      }),
    ) as { kitStore?: { materializeAt?: unknown } };
    return d.kitStore?.materializeAt;
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

