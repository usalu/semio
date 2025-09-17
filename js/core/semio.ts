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

import cytoscape from "cytoscape";
import * as THREE from "three";
import { z } from "zod";
import CONSTANTS from "./constants.json";
import { jaccard, normalize, round } from "./lib/utils";

// #region Constants

export const ICON_WIDTH = CONSTANTS.icon.width;
export const TOLERANCE = CONSTANTS.tolerance;

// #endregion Constants

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


// #region Ids

export const DesignIdSchema = z.object({ name: z.string(), variant: z.string().optional(), view: z.string().optional() });

// #endregion Ids

const DateProperty = () => z.string().transform((val) => new Date(val)).or(z.date()).optional()

// #region Attribute
// https://github.com/usalu/semio#-attribute-

export const AttributeSchema = z.object({
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
export type Attribute = z.infer<typeof AttributeSchema>;
export const serializeAttribute = (attribute: Attribute): string => JSON.stringify(AttributeSchema.parse(attribute));
export const deserializeAttribute = (json: string): Attribute => AttributeSchema.parse(JSON.parse(json));

export const AttributeIdSchema = z.object({ key: z.string() });
export type AttributeId = z.infer<typeof AttributeIdSchema>;
export const attributeIdToString = (attribute: AttributeId): string => attribute.key;

export const AttributeDiffSchema = AttributeSchema.partial();
export type AttributeDiff = z.infer<typeof AttributeDiffSchema>;

export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ id: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(AttributeSchema).optional(),
});
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;

export const AttributeIdLikeSchema = z.union([AttributeIdSchema, AttributeSchema, AttributeDiffSchema, z.string()]);
export type AttributeIdLike = z.infer<typeof AttributeIdLikeSchema>;
export const attributeIdLikeToAttributeId = (attribute: AttributeIdLike): AttributeId => {
  if (typeof attribute === "string") return { key: attribute };
  if (typeof attribute === "object" && "key" in attribute) return { key: attribute.key };
  throw new Error("Invalid attribute id like");
};

// #endregion Attribute

// #region Coord
// https://github.com/usalu/semio#-coord-

export const CoordSchema = z.object({ x: z.number(), y: z.number() });
export type Coord = z.infer<typeof CoordSchema>;
export const serializeCoord = (coord: Coord): string => JSON.stringify(CoordSchema.parse(coord));
export const deserializeCoord = (json: string): Coord => CoordSchema.parse(JSON.parse(json));

export const CoordDiffSchema = CoordSchema.partial();
export type CoordDiff = z.infer<typeof CoordDiffSchema>;
export const getCoordDiff = (before: Coord, after: Coord): CoordDiff => { }
export const inverseCoordDiff = (original: Coord, appliedDiff: CoordDiff): CoordDiff => { }
export const mergeCoordDiff = (diff1: CoordDiff, diff2: CoordDiff): CoordDiff => { }
export const applyCoordDiff = (base: Coord, diff: CoordDiff): Coord => { }

// #endregion Coord

// #region Vec
// https://github.com/usalu/semio#-vec-

export const VecSchema = z.object({ x: z.number(), y: z.number() });
export type Vec = z.infer<typeof VecSchema>;
export const serializeVec = (vec: Vec): string => JSON.stringify(VecSchema.parse(vec));
export const deserializeVec = (json: string): Vec => VecSchema.parse(JSON.parse(json));

export const VecDiffSchema = VecSchema.partial();
export type VecDiff = z.infer<typeof VecDiffSchema>;
export const getVecDiff = (before: Vec, after: Vec): VecDiff => { }
export const inverseVecDiff = (original: Vec, appliedDiff: VecDiff): VecDiff => { }
export const mergeVecDiff = (diff1: VecDiff, diff2: VecDiff): VecDiff => { }
export const applyVecDiff = (base: Vec, diff: VecDiff): Vec => { }

// #endregion Vec

// #region Point
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
export const getPointDiff = (before: Point, after: Point): PointDiff => { }
export const inversePointDiff = (original: Point, appliedDiff: PointDiff): PointDiff => { }
export const mergePointDiff = (diff1: PointDiff, diff2: PointDiff): PointDiff => { }
export const applyPointDiff = (base: Point, diff: PointDiff): Point => { }

// #endregion Point

// #region Vector
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
export const getVectorDiff = (before: Vector, after: Vector): VectorDiff => { }
export const inverseVectorDiff = (original: Vector, appliedDiff: VectorDiff): VectorDiff => { }
export const mergeVectorDiff = (diff1: VectorDiff, diff2: VectorDiff): VectorDiff => { }
export const applyVectorDiff = (base: Vector, diff: VectorDiff): Vector => { }

// #endregion Vector

// #region Plane

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

export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true }).extend({
  origin: PointDiffSchema,
  xAxis: VectorDiffSchema,
  yAxis: VectorDiffSchema,
}).partial();
export type PlaneDiff = z.infer<typeof PlaneDiffSchema>;
export const getPlaneDiff = (before: Plane, after: Plane): PlaneDiff => { }
export const inversePlaneDiff = (original: Plane, appliedDiff: PlaneDiff): PlaneDiff => { }
export const mergePlaneDiff = (diff1: PlaneDiff, diff2: PlaneDiff): PlaneDiff => { }
export const applyPlaneDiff = (base: Plane, diff: PlaneDiff): Plane => { }

// #endregion Plane

// #region Camera
// https://github.com/usalu/semio#-camera-

export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});
export type Camera = z.infer<typeof CameraSchema>;
export const serializeCamera = (camera: Camera): string => JSON.stringify(CameraSchema.parse(camera));
export const deserializeCamera = (json: string): Camera => CameraSchema.parse(JSON.parse(json));

export const CameraDiffSchema = CameraSchema.partial();
export type CameraDiff = z.infer<typeof CameraDiffSchema>;
export const getCameraDiff = (before: Camera, after: Camera): CameraDiff => { }
export const inverseCameraDiff = (original: Camera, appliedDiff: CameraDiff): CameraDiff => { }
export const mergeCameraDiff = (diff1: CameraDiff, diff2: CameraDiff): CameraDiff => { }
export const applyCameraDiff = (base: Camera, diff: CameraDiff): Camera => { }

// #endregion Camera

// #region Location
// https://github.com/usalu/semio#-location-

export const LocationSchema = z.object({
  longitude: z.number(),
  latitude: z.number(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Location = z.infer<typeof LocationSchema>;
export const serializeLocation = (location: Location): string => JSON.stringify(LocationSchema.parse(location));
export const deserializeLocation = (json: string): Location => LocationSchema.parse(JSON.parse(json));

export const LocationDiffSchema = LocationSchema.partial();
export type LocationDiff = z.infer<typeof LocationDiffSchema>;
export const getLocationDiff = (before: Location, after: Location): LocationDiff => { }
export const inverseLocationDiff = (original: Location, appliedDiff: LocationDiff): LocationDiff => { }
export const mergeLocationDiff = (diff1: LocationDiff, diff2: LocationDiff): LocationDiff => { }
export const applyLocationDiff = (base: Location, diff: LocationDiff): Location => { }

// #endregion Location

// #region Author
// https://github.com/usalu/semio#-author-

export const AuthorSchema = z.object({ name: z.string(), email: z.string(), attributes: z.array(AttributeSchema).optional() });
export type Author = z.infer<typeof AuthorSchema>;
export const serializeAuthor = (author: Author): string => JSON.stringify(AuthorSchema.parse(author));
export const deserializeAuthor = (json: string): Author => AuthorSchema.parse(JSON.parse(json));

export const AuthorIdSchema = AuthorSchema.pick({ email: true });
export type AuthorId = z.infer<typeof AuthorIdSchema>;
export const authorIdToString = (author: AuthorId): string => author.email;

export const AuthorDiffSchema = AuthorSchema.partial();
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;
export const getAuthorDiff = (before: Author, after: Author): AuthorDiff => { };
export const inverseAuthorDiff = (original: Author, appliedDiff: AuthorDiff): AuthorDiff => { };
export const mergeAuthorDiff = (diff1: AuthorDiff, diff2: AuthorDiff): AuthorDiff => { };
export const applyAuthorDiff = (base: Author, diff: AuthorDiff): Author => { };

export const AuthorsDiffSchema = z.object({
  removed: z.array(AuthorIdSchema).optional(),
  updated: z.array(z.object({ id: AuthorIdSchema, diff: AuthorDiffSchema })).optional(),
  added: z.array(AuthorSchema).optional(),
});

export const AuthorIdLikeSchema = z.union([AuthorIdSchema, AuthorSchema, AuthorDiffSchema, z.string()]);
export type AuthorIdLike = z.infer<typeof AuthorIdLikeSchema>;
export const authorIdLikeToAuthorId = (author: AuthorIdLike): AuthorId => {
  if (typeof author === "string") return { email: author };
  if (typeof author === "object" && "email" in author) return { email: author.email };
  throw new Error("Invalid author id like");
};

// #endregion Author

// #region File
// https://github.com/usalu/semio#-file-

export const FileSchema = z.object({
  // path: z.url(),
  path: z.string(),
  // remote: z.url().optional(),
  remote: z.string().optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  createdAt: DateProperty(),
  createdBy: AuthorIdSchema.optional(),
  updatedAt: DateProperty(),
  updatedBy: AuthorIdSchema.optional(),
});
export type File = z.infer<typeof FileSchema>;
export const serializeFile = (file: File): string => JSON.stringify(FileSchema.parse(file));
export const deserializeFile = (json: string): File => FileSchema.parse(JSON.parse(json));

export const FileIdSchema = FileSchema.pick({ path: true });
export type FileId = z.infer<typeof FileIdSchema>;
export const fileIdToString = (file: FileId): string => file.path;

export const FileDiffSchema = FileSchema.partial();
export type FileDiff = z.infer<typeof FileDiffSchema>;
export const getFileDiff = (before: File, after: File): FileDiff => { };
export const inverseFileDiff = (original: File, appliedDiff: FileDiff): FileDiff => { };
export const mergeFileDiff = (diff1: FileDiff, diff2: FileDiff): FileDiff => { };
export const applyFileDiff = (base: File, diff: FileDiff): File => { };

export const FilesDiffSchema = z.object({
  removed: z.array(FileIdSchema).optional(),
  updated: z.array(z.object({ id: FileIdSchema, diff: FileDiffSchema })).optional(),
  added: z.array(FileSchema).optional(),
});
export type FilesDiff = z.infer<typeof FilesDiffSchema>;

export const FileIdLikeSchema = z.union([FileIdSchema, FileSchema, FileDiffSchema, z.string()]);
export type FileIdLike = z.infer<typeof FileIdLikeSchema>;
export const fileIdLikeToFileId = (file: FileIdLike): FileId => {
  if (typeof file === "string") return { path: file };
  if (typeof file === "object" && "path" in file) return { path: file.path };
  throw new Error("Invalid file id like");
};

// #endregion File

// #region Benchmark

// https://github.com/usalu/semio#-benchmark-
export const BenchmarkSchema = z.object({
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

export const BenchmarkIdSchema = BenchmarkSchema.pick({ name: true });
export type BenchmarkId = z.infer<typeof BenchmarkIdSchema>;
export const benchmarkIdToString = (benchmark: BenchmarkId): string => benchmark.name;

export const BenchmarkDiffSchema = BenchmarkSchema.partial();
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;
export const applyBenchmarkDiff = (base: Benchmark, diff: BenchmarkDiff): Benchmark => {
};
export const getBenchmarkDiff = (before: Benchmark, after: Benchmark): BenchmarkDiff => {
};
export const inverseBenchmarkDiff = (original: Benchmark, appliedDiff: BenchmarkDiff): BenchmarkDiff => {
};
export const mergeBenchmarkDiff = (diff1: BenchmarkDiff, diff2: BenchmarkDiff): BenchmarkDiff => {
};

export const BenchmarksDiffSchema = z.object({
  removed: z.array(BenchmarkIdSchema).optional(),
  updated: z.array(z.object({ id: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(),
  added: z.array(BenchmarkSchema).optional(),
});

export const BenchmarkIdLikeSchema = z.union([BenchmarkIdSchema, BenchmarkSchema, BenchmarkDiffSchema, z.string()]);
export type BenchmarkIdLike = z.infer<typeof BenchmarkIdLikeSchema>;
export const benchmarkIdLikeToBenchmarkId = (benchmark: BenchmarkIdLike): BenchmarkId => {
  if (typeof benchmark === "string") return { name: benchmark };
  if (typeof benchmark === "object" && "name" in benchmark) return { name: benchmark.name };
  throw new Error("Invalid benchmark id like");
};

// #endregion Benchmark

// #region QualityKind

// #endregion QualityKind

// #region Quality

// https://github.com/usalu/semio#-quality-
export const QualitySchema = z.object({
  key: z.string(),
  name: z.string(),
  description: z.string().optional(),
  uri: z.string().optional(),
  kind: z.number().optional(),
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
  variant: z.string().optional(),
  unit: z.string().optional(),
  benchmarks: z.array(BenchmarkSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Quality = z.infer<typeof QualitySchema>;
export const serializeQuality = (quality: Quality): string => JSON.stringify(QualitySchema.parse(quality));
export const deserializeQuality = (json: string): Quality => QualitySchema.parse(JSON.parse(json));

export const QualityIdSchema = QualitySchema.pick({ key: true });
export type QualityId = z.infer<typeof QualityIdSchema>;
export const qualityIdToString = (quality: QualityId): string => quality.key;

export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true, attributes: true }).extend({
  benchmarks: BenchmarksDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type QualityDiff = z.infer<typeof QualityDiffSchema>;
export const getQualityDiff = (before: Quality, after: Quality): QualityDiff => { };
export const inverseQualityDiff = (original: Quality, appliedDiff: QualityDiff): QualityDiff => { };
export const mergeQualityDiff = (diff1: QualityDiff, diff2: QualityDiff): QualityDiff => { };
export const applyQualityDiff = (base: Quality, diff: QualityDiff): Quality => { };

export const QualitiesDiffSchema = z.object({
  removed: z.array(QualityIdSchema).optional(),
  updated: z.array(z.object({ id: QualityIdSchema, diff: QualityDiffSchema })).optional(),
  added: z.array(QualitySchema).optional(),
});

export const QualityIdLikeSchema = z.union([QualityIdSchema, QualitySchema, QualityDiffSchema, z.string()]);
export type QualityIdLike = z.infer<typeof QualityIdLikeSchema>;
export const qualityIdLikeToQualityId = (quality: QualityIdLike): QualityId => {
  if (typeof quality === "string") return { key: quality };
  if (typeof quality === "object" && "key" in quality) return { key: quality.key };
  throw new Error("Invalid quality id like");
};

// #endregion Quality

// #region Prop
// https://github.com/usalu/semio#-prop-

export const PropSchema = z.object({
  key: z.string(),
  value: z.string(),
  unit: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Prop = z.infer<typeof PropSchema>;
export const serializeProp = (prop: Prop): string => JSON.stringify(PropSchema.parse(prop));
export const deserializeProp = (json: string): Prop => PropSchema.parse(JSON.parse(json));

export const PropIdSchema = PropSchema.pick({ key: true });
export type PropId = z.infer<typeof PropIdSchema>;
export const propIdToString = (prop: PropId): string => prop.key;

export const PropDiffSchema = PropSchema.partial();
export type PropDiff = z.infer<typeof PropDiffSchema>;
export const getPropDiff = (before: Prop, after: Prop): PropDiff => { };
export const inversePropDiff = (original: Prop, appliedDiff: PropDiff): PropDiff => { };
export const mergePropDiff = (diff1: PropDiff, diff2: PropDiff): PropDiff => { };
export const applyPropDiff = (base: Prop, diff: PropDiff): Prop => { };

export const PropsDiffSchema = z.object({
  removed: z.array(PropIdSchema).optional(),
  updated: z.array(z.object({ id: PropIdSchema, diff: PropDiffSchema })).optional(),
  added: z.array(PropSchema).optional(),
});


export const PropIdLikeSchema = z.union([PropIdSchema, PropSchema, PropDiffSchema, z.string()]);
export type PropIdLike = z.infer<typeof PropIdLikeSchema>;
export const propIdLikeToPropId = (prop: PropIdLike): PropId => {
  if (typeof prop === "string") return { key: prop };
  if (typeof prop === "object" && "key" in prop) return { key: prop.key };
  throw new Error("Invalid prop id like");
};

// #endregion Prop

// #region Representation
// https://github.com/usalu/semio#-representation-

export const RepresentationSchema = z.object({
  tags: z.array(z.string()).optional(),
  url: z.string(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Representation = z.infer<typeof RepresentationSchema>;
export const serializeRepresentation = (representation: Representation): string => JSON.stringify(RepresentationSchema.parse(representation));
export const deserializeRepresentation = (json: string): Representation => RepresentationSchema.parse(JSON.parse(json));

export const RepresentationIdSchema = RepresentationSchema.pick({ tags: true });
export type RepresentationId = z.infer<typeof RepresentationIdSchema>;
export const representationIdToString = (representation: RepresentationId): string => representation.tags?.join(",") ?? "";

export const RepresentationDiffSchema = RepresentationSchema.partial();
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;
export const getRepresentationDiff = (before: Representation, after: Representation): RepresentationDiff => { };
export const inverseRepresentationDiff = (original: Representation, appliedDiff: RepresentationDiff): RepresentationDiff => { };
export const mergeRepresentationDiff = (diff1: RepresentationDiff, diff2: RepresentationDiff): RepresentationDiff => { };
export const applyRepresentationDiff = (base: Representation, diff: RepresentationDiff): Representation => { };

export const RepresentationsDiffSchema = z.object({
  removed: z.array(RepresentationIdSchema).optional(),
  updated: z.array(z.object({ id: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(),
  added: z.array(RepresentationSchema).optional(),
});

export const RepresentationIdLikeSchema = z.union([RepresentationIdSchema, RepresentationSchema, RepresentationDiffSchema, z.array(z.string()), z.string(), z.null(), z.undefined()]);
export type RepresentationIdLike = z.infer<typeof RepresentationIdLikeSchema>;
export const representationIdLikeToRepresentationId = (representation: RepresentationIdLike): RepresentationId => {
  if (representation === null || representation === undefined) return { tags: [] };
  if (Array.isArray(representation)) return { tags: representation };
  if (typeof representation === "string") return { tags: representation.split(",") };
  if (typeof representation === "object" && "tags" in representation) return { tags: representation.tags };
  throw new Error("Invalid representation id like");
};


export const areSameRepresentation = (representation: Representation, other: Representation): boolean => {
  return representation.tags?.every((tag) => other.tags?.includes(tag)) ?? true;
};

const findRepresentation = (representations: Representation[], tags: string[]): Representation => {
  const indices = representations.map((r) => jaccard(r.tags, tags));
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return representations[maxIndexIndex];
};

// #endregion Representation

// #region Port
// https://github.com/usalu/semio#-port-

export const PortSchema = z.object({
  id_: z.string().optional(),
  description: z.string().optional(),
  family: z.string().optional(),
  mandatory: z.boolean().optional(),
  t: z.number(),
  compatibleFamilies: z.array(z.string()).optional(),
  point: PointSchema,
  direction: VectorSchema,
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Port = z.infer<typeof PortSchema>;
export const serializePort = (port: Port): string => JSON.stringify(PortSchema.parse(port));
export const deserializePort = (json: string): Port => PortSchema.parse(JSON.parse(json));

export const PortIdSchema = PortSchema.pick({ id_: true });
export type PortId = z.infer<typeof PortIdSchema>;
export const portIdToString = (port: PortId): string => port.id_ ?? "";

export const PortDiffSchema = PortSchema.partial();
export type PortDiff = z.infer<typeof PortDiffSchema>;
export const getPortDiff = (before: Port, after: Port): PortDiff => { };
export const mergePortDiff = (diff1: PortDiff, diff2: PortDiff): PortDiff => { };
export const inversePortDiff = (original: Port, appliedDiff: PortDiff): PortDiff => { };
export const applyPortDiff = (base: Port, diff: PortDiff): Port => { };

export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(z.object({ id: PortIdSchema, diff: PortDiffSchema })).optional(),
  added: z.array(PortSchema).optional(),
});

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
  const updated: { id_: TypeId; diff: TypeDiff }[] = [];

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

    updated.push({
      id_: { name: type.name, variant: type.variant },
      diff: {
        ports: updatedPorts
      }
    });
  }

  return { updated };
};
export const arePortsCompatible = (port: Port, otherPort: Port): boolean => {
  const normalizedPortFamily = normalize(port.family);
  const normalizedOtherPortFamily = normalize(otherPort.family);
  if (normalizedPortFamily === "" || normalizedOtherPortFamily === "") return true;
  return (port.compatibleFamilies ?? []).includes(normalizedOtherPortFamily) || (otherPort.compatibleFamilies ?? []).includes(normalizedPortFamily);
};

export const PortIdLikeSchema = z.union([PortSchema, PortIdSchema, z.string()]);
export type PortIdLike = z.infer<typeof PortIdLikeSchema>;
export const portIdLikeToPortId = (port: PortIdLike): PortId => {
  if (typeof port === "string") return { id_: port };
  return { id_: port.id_ };
};
export const areSamePort = (port: PortIdLike, other: PortIdLike): boolean => {
  const p1 = portIdLikeToPortId(port);
  const p2 = portIdLikeToPortId(other);
  return normalize(p1.id_) === normalize(p2.id_);
};
export const hasSamePort = (port: PortIdLike, others: PortIdLike[]): boolean => others.some((other) => areSamePort(port, other));
export const findPort = (ports: Port[], portId: PortIdLike): Port => {
  const normalizedPortId = portIdLikeToPortId(portId);
  const port = ports.find((p) => normalize(p.id_) === normalize(normalizedPortId.id_));
  if (!port) throw new Error(`Port ${normalizedPortId.id_} not found in ports`);
  return port;
};

// #endregion Port

// #region Type
// https://github.com/usalu/semio#-type-
export const TypeSchema = z.object({
  name: z.string(),
  variant: z.string().optional(),
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
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional()
});
export type Type = z.infer<typeof TypeSchema>;
export const serializeType = (type: Type): string => JSON.stringify(TypeSchema.parse(type));
export const deserializeType = (json: string): Type => TypeSchema.parse(JSON.parse(json));

export const TypeIdSchema = TypeSchema.pick({ name: true, variant: true });
export type TypeId = z.infer<typeof TypeIdSchema>;
export const typeIdToString = (type: TypeId): string => type.name;
export const TypeIdLikeSchema = z.union([TypeSchema, TypeIdSchema, z.string()]);
export type TypeIdLike = z.infer<typeof TypeIdLikeSchema>;
export const typeIdLikeToTypeId = (type: TypeIdLike): TypeId => {
  if (typeof type === "string") return { name: type };
  return { name: type.name };
};
export const TypeShallowSchema = TypeSchema.omit({ representations: true, ports: true }).extend({
  representations: z.array(RepresentationIdSchema).optional(),
  ports: z.array(PortIdSchema).optional(),
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
};

export const applyTypeDiff = (base: Type, diff: TypeDiff): Type => {
};

export const mergeTypeDiff = (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => {
};

export const inverseTypeDiff = (original: Type, appliedDiff: TypeDiff): TypeDiff => {
};

export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(z.object({ id_: TypeIdSchema, diff: TypeDiffSchema })).optional(),
  added: z.array(TypeSchema).optional(),
});
export type TypesDiff = z.infer<typeof TypesDiffSchema>;

export const areSameType = (type: TypeIdLike, other: TypeIdLike): boolean => {
  const t1 = typeIdLikeToTypeId(type);
  const t2 = typeIdLikeToTypeId(other);
  return t1.name === t2.name && normalize(t1.variant) === normalize(t2.variant);
};
export const hasSameType = (type: TypeIdLike, others: TypeIdLike[]): boolean => others.some((other) => areSameType(type, other));

export const findPortInType = (type: Type, portId: PortIdLike): Port => findPort(type.ports ?? [], portId);

// #endregion Type

// #region Layer
// https://github.com/usalu/semio#-layer-

export const LayerSchema = z.object({
  path: z.string(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional()
});
export type Layer = z.infer<typeof LayerSchema>;
export const serializeLayer = (layer: Layer): string => JSON.stringify(LayerSchema.parse(layer));
export const deserializeLayer = (json: string): Layer => LayerSchema.parse(JSON.parse(json));

export const LayerIdSchema = LayerSchema.pick({ path: true });
export type LayerId = z.infer<typeof LayerIdSchema>;
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({
  attributes: AttributesDiffSchema.optional(),
});
export type LayerDiff = z.infer<typeof LayerDiffSchema>;

export const getLayerDiff = (before: Layer, after: Layer): LayerDiff => { };
export const inverseLayerDiff = (original: Layer, appliedDiff: LayerDiff): LayerDiff => { };
export const mergeLayerDiff = (diff1: LayerDiff, diff2: LayerDiff): LayerDiff => { };
export const applyLayerDiff = (base: Layer, diff: LayerDiff): Layer => { };

export const LayersDiffSchema = z.object({
  removed: z.array(LayerIdSchema).optional(),
  updated: z.array(z.object({ id: LayerIdSchema, diff: LayerDiffSchema })).optional(),
  added: z.array(LayerSchema).optional(),
});
export type LayersDiff = z.infer<typeof LayersDiffSchema>;

export const LayerIdLikeSchema = z.union([LayerIdSchema, LayerSchema, LayerDiffSchema, z.string()]);
export type LayerIdLike = z.infer<typeof LayerIdLikeSchema>;
export const layerIdLikeToLayerId = (layer: LayerIdLike): LayerId => {
  if (typeof layer === "string") return { path: layer };
  if (typeof layer === "object" && "path" in layer) return { path: layer.path };
  throw new Error("Invalid layer id like");
};


// #endregion Layer

// #region Piece
// https://github.com/usalu/semio#-piece-

export const PieceSchema = z.object({
  id_: z.string(),
  type: TypeIdSchema.optional(),
  design: DesignIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: CoordSchema.optional(),
  scale: z.number().optional(),
  mirrorPlane: PlaneSchema.optional(),
  hidden: z.boolean().optional(),
  locked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional()
});
export type Piece = z.infer<typeof PieceSchema>;
export const serializePiece = (piece: Piece): string => JSON.stringify(PieceSchema.parse(piece));
export const deserializePiece = (json: string): Piece => PieceSchema.parse(JSON.parse(json));

export const PieceIdSchema = PieceSchema.pick({ id_: true });
export type PieceId = z.infer<typeof PieceIdSchema>;
export const pieceIdToString = (piece: PieceId): string => piece.id_;

export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, attributes: true }).extend({
  plane: PlaneDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export const getPieceDiff = (before: Piece, after: Piece): PieceDiff => { };
export const inversePieceDiff = (original: Piece, appliedDiff: PieceDiff): PieceDiff => { };
export const mergePieceDiff = (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => { };
export const applyPieceDiff = (base: Piece, diff: PieceDiff): Piece => { };

export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ id: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;

/**
 * 🔗 Returns a map of piece ids to representation urls for the given design and types.
 * @param design - The design with the pieces to get the representation urls for.
 * @param types - The types of the pieces with the representations.
 * @returns A map of piece ids to representation urls.
 */
export const getPieceRepresentationUrls = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const representationUrls = new Map<string, string>();
  const normalizeVariant = (v: string | undefined | null) => v ?? "";
  design.pieces?.forEach((p) => {
    if (!p.type) return;
    const type = types.find((t) => t.name === p.type!.name && normalizeVariant(t.variant) === normalizeVariant(p.type!.variant));
    if (!type) throw new Error(`Type (${p.type!.name}, ${p.type!.variant}) for piece ${p.id_} not found`);
    if (!type.representations) throw new Error(`Type (${p.type!.name}, ${p.type!.variant}) for piece ${p.id_} has no representations`);
    const representation = findRepresentation(type.representations, tags);
    representationUrls.set(p.id_, representation.url);
  });
  return representationUrls;
};
export const fixPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): DesignDiff => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const parentConnection = findParentConnectionForPieceInDesign(kit, normalizedDesignId, normalizedPieceId);

  return {
    connections: {
      removed: [{
        connected: { piece: { id_: parentConnection.connected.piece.id_ } },
        connecting: { piece: { id_: parentConnection.connecting.piece.id_ } }
      }]
    }
  };
};

export const fixPiecesInDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[]): DesignDiff => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const normalizedPieceIds = pieceIds.map(pieceIdLikeToPieceId);
  const parentConnections = normalizedPieceIds.map((pieceId) => findParentConnectionForPieceInDesign(kit, normalizedDesignId, pieceId.id_));

  return {
    connections: {
      removed: parentConnections.map(c => ({
        connected: { piece: { id_: c.connected.piece.id_ } },
        connecting: { piece: { id_: c.connecting.piece.id_ } }
      }))
    }
  };
};

export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined;
  const isCenterSet = piece.center !== undefined;
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.id_} has inconsistent plane and center`);
  return isPlaneSet;
};

export const PieceIdLikeSchema = z.union([PieceIdSchema, PieceSchema, PieceDiffSchema, z.string()]);
export type PieceIdLike = z.infer<typeof PieceIdLikeSchema>;
export const pieceIdLikeToPieceId = (piece: PieceIdLike): PieceId => {
  if (typeof piece === "string") return { id_: piece };
  if (typeof piece === "object" && "id_" in piece) return { id_: piece.id_ };
  throw new Error("Invalid piece id like");
};

export const areSamePiece = (piece: PieceIdLike, other: PieceIdLike): boolean => {
  const p1 = pieceIdLikeToPieceId(piece);
  const p2 = pieceIdLikeToPieceId(other);
  return normalize(p1.id_) === normalize(p2.id_);
};

export const hasSamePiece = (piece: PieceIdLike, others: PieceIdLike[]): boolean => others.some((other) => areSamePiece(piece, other));

export const findPiece = (pieces: Piece[], pieceId: PieceIdLike): Piece => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const piece = pieces.find((p) => p.id_ === normalizedPieceId.id_);
  if (!piece) throw new Error(`Piece ${normalizedPieceId.id_} not found in pieces`);
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
  attributes: z.array(AttributeSchema).optional()
});
export type Group = z.infer<typeof GroupSchema>;
export const GroupIdSchema = GroupSchema.pick({ pieces: true });
export type GroupId = z.infer<typeof GroupIdSchema>;
export const groupIdToString = (group: GroupId): string => group.pieces.join(",");
export const GroupIdLikeSchema = z.union([GroupSchema, GroupIdSchema, z.array(PieceIdSchema), z.string()]);
export type GroupIdLike = z.infer<typeof GroupIdLikeSchema>;
export const groupIdLikeToGroupId = (group: GroupIdLike): GroupId => {
  if (typeof group === "string") return { pieces: group.split(",").map(id => ({ id_: id })) };
  if (Array.isArray(group)) return { pieces: group };
  return { pieces: group.pieces };
};
export const GroupDiffSchema = GroupSchema.partial();
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
export const GroupsDiffSchema = z.object({
  removed: z.array(GroupIdSchema).optional(),
  updated: z.array(z.object({ id: GroupIdSchema, diff: GroupDiffSchema })).optional(),
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
export const SideIdSchema = SideSchema.pick({ piece: true, designPiece: true });
export type SideId = z.infer<typeof SideIdSchema>;
export const SideIdLikeSchema = z.union([SideSchema, SideIdSchema, z.string(), z.tuple([PieceIdSchema, PortIdSchema])]);
export type SideIdLike = z.infer<typeof SideIdLikeSchema>;
export const sideIdLikeToSideId = (side: SideIdLike): SideId => {
  if (typeof side === "string") return { piece: { id_: side.split(":")[0] }, designPiece: side.split(":")[1] ? { id_: side.split(":")[1] } : undefined };
  if (Array.isArray(side)) return { piece: side[0], designPiece: undefined };
  return { piece: side.piece, designPiece: side.designPiece };
};
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = z.infer<typeof SideDiffSchema>;
export const SidesDiffSchema = z.object({
  removed: z.array(SideIdSchema).optional(),
  updated: z.array(z.object({ id: SideIdSchema, diff: SideDiffSchema })).optional(),
  added: z.array(SideSchema).optional(),
});
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
export const serializeSide = (side: Side): string => JSON.stringify(SideSchema.parse(side));
export const deserializeSide = (json: string): Side => SideSchema.parse(JSON.parse(json));

// #endregion Side

// #region Connection

// https://github.com/usalu/semio#-connection-
export const ConnectionSchema = z.object({
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
  attributes: z.array(AttributeSchema).optional()
});
export type Connection = z.infer<typeof ConnectionSchema>;
export const ConnectionIdSchema = z.object({
  connected: z.object({ piece: PieceIdSchema }),
  connecting: z.object({ piece: PieceIdSchema })
});
export type ConnectionId = z.infer<typeof ConnectionIdSchema>;
export const connectionIdToString = (connection: ConnectionId): string => `${connection.connected.piece.id_}-${connection.connecting.piece.id_}`;
export const ConnectionIdLikeSchema = z.union([ConnectionSchema, ConnectionIdSchema, z.string()]);
export type ConnectionIdLike = z.infer<typeof ConnectionIdLikeSchema>;
export const connectionIdLikeToConnectionId = (connection: ConnectionIdLike): ConnectionId => {
  if (typeof connection === "string") {
    const parts = connection.split("-");
    return {
      connected: { piece: { id_: parts[0] } },
      connecting: { piece: { id_: parts[1] } }
    };
  }
  if ('connected' in connection) {
    return {
      connected: { piece: connection.connected.piece },
      connecting: { piece: connection.connecting.piece }
    };
  }
  return connection as ConnectionId;
};
export const ConnectionDiffSchema = ConnectionSchema.partial();
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
export const getConnectionDiff = (before: Connection, after: Connection): ConnectionDiff => {
};

export const applyConnectionDiff = (base: Connection, diff: ConnectionDiff): Connection => {
};

export const mergeConnectionDiff = (diff1: ConnectionDiff, diff2: ConnectionDiff): ConnectionDiff => {
};

export const inverseConnectionDiff = (original: Connection, appliedDiff: ConnectionDiff): ConnectionDiff => {
};

export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(z.object({ id: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(),
  added: z.array(ConnectionSchema).optional(),
});
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export const serializeConnection = (connection: Connection): string => JSON.stringify(ConnectionSchema.parse(connection));
export const deserializeConnection = (json: string): Connection => ConnectionSchema.parse(JSON.parse(json));

export const areSameConnection = (connection: Connection | ConnectionId | ConnectionDiff, other: Connection | ConnectionId | ConnectionDiff, strict: boolean = false): boolean => {
  const getConnectedPieceId = (conn: typeof connection) => ("connected" in conn && conn.connected && "piece" in conn.connected && conn.connected.piece ? conn.connected.piece.id_ : "");
  const getConnectingPieceId = (conn: typeof connection) => ("connecting" in conn && conn.connecting && "piece" in conn.connecting && conn.connecting.piece ? conn.connecting.piece.id_ : "");

  const connectedPiece1 = getConnectedPieceId(connection);
  const connectingPiece1 = getConnectingPieceId(connection);
  const connectedPiece2 = getConnectedPieceId(other);
  const connectingPiece2 = getConnectingPieceId(other);

  const isExactMatch = connectingPiece1 === connectingPiece2 && connectedPiece1 === connectedPiece2;
  if (strict) return isExactMatch;
  const isSwappedMatch = connectingPiece1 === connectedPiece2 && connectedPiece1 === connectingPiece2;
  return isExactMatch || isSwappedMatch;
};

export const findConnection = (connections: Connection[], connectionId: ConnectionIdLike, strict: boolean = false): Connection => {
  const normalizedConnectionId = connectionIdLikeToConnectionId(connectionId);
  const connection = connections.find((c) => areSameConnection(c, normalizedConnectionId, strict));
  if (!connection) throw new Error(`Connection ${normalizedConnectionId.connected.piece.id_} -> ${normalizedConnectionId.connecting.piece.id_} not found in connections`);
  return connection;
};

export const findPieceConnections = (connections: Connection[], pieceId: PieceIdLike): Connection[] => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  return connections.filter((c) => c.connected.piece.id_ === normalizedPieceId.id_ || c.connecting.piece.id_ === normalizedPieceId.id_);
};

export const findPortForPieceInConnection = (type: Type, connection: Connection, pieceId: PieceIdLike): Port => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const portId = connection.connected.piece.id_ === normalizedPieceId.id_ ? connection.connected.port.id_ : connection.connecting.port.id_;
  if (!portId) throw new Error('Port ID is required');
  return findPortInType(type, portId);
};

// #endregion Connection

// #region Stat
// https://github.com/usalu/semio#-stat-

export const StatSchema = z.object({
  key: z.string(),
  unit: z.string().optional(),
  min: z.number().optional(),
  minExcluded: z.boolean().optional(),
  max: z.number().optional(),
  maxExcluded: z.boolean().optional(),
});
export type Stat = z.infer<typeof StatSchema>;
export const StatIdSchema = StatSchema.pick({ key: true });
export type StatId = z.infer<typeof StatIdSchema>;
export const statIdToString = (stat: StatId): string => stat.key;
export const StatIdLikeSchema = z.union([StatSchema, StatIdSchema, z.string()]);
export type StatIdLike = z.infer<typeof StatIdLikeSchema>;
export const statIdLikeToStatId = (stat: StatIdLike): StatId => {
  if (typeof stat === "string") return { key: stat };
  return { key: stat.key };
};
export const StatDiffSchema = StatSchema.partial();
export type StatDiff = z.infer<typeof StatDiffSchema>;
export const StatsDiffSchema = z.object({
  removed: z.array(StatIdSchema).optional(),
  updated: z.array(z.object({ id: StatIdSchema, diff: StatDiffSchema })).optional(),
  added: z.array(StatSchema).optional(),
});
export const serializeStat = (stat: Stat): string => JSON.stringify(StatSchema.parse(stat));
export const deserializeStat = (json: string): Stat => StatSchema.parse(JSON.parse(json));

// #endregion Stat

// #region Design
// https://github.com/usalu/semio#-design-

export const DesignSchema = z.object({
  name: z.string(),
  variant: z.string().optional(),
  view: z.string().optional(),
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

export type DesignId = z.infer<typeof DesignIdSchema>;
export const designIdToString = (design: DesignId): string => design.name;
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true }).extend({
  pieces: z.array(PieceIdSchema).optional(),
  connections: z.array(ConnectionIdSchema).optional(),
  stats: z.array(StatIdSchema).optional(),
});

export type DesignShallow = z.infer<typeof DesignShallowSchema>;
export const serializeDesignShallow = (design: DesignShallow): string => JSON.stringify(DesignShallowSchema.parse(design));
export const deserializeDesignShallow = (json: string): DesignShallow => DesignShallowSchema.parse(JSON.parse(json));
export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).extend({
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
export const DesignIdLikeSchema = z.union([DesignSchema, DesignIdSchema, DesignShallowSchema, DesignDiffSchema, z.string()]);
export type DesignIdLike = z.infer<typeof DesignIdLikeSchema>;
export const designIdLikeToDesignId = (design: DesignIdLike): DesignId => {
  if (typeof design === "string") return { name: design };
  return { name: design.name, variant: design.variant, view: design.view };
};
export const getDesignDiff = (before: Design, after: Design): DesignDiff => {
};
export const mergeDesignDiff = (diff1: DesignDiff, diff2: DesignDiff): DesignDiff => {
};
export const inverseDesignDiff = (original: Design, appliedDiff: DesignDiff): DesignDiff => {
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
export const setPieceInDesignDiff = (designDiff: any, pieceDiff: { id_: PieceId, diff: PieceDiff }): any => {
  const existingIndex = (designDiff.pieces?.updated || []).findIndex((p: { id_: PieceId, diff: PieceDiff }) => {
    const pId = typeof p.id_ === 'string' ? p.id_ : p.id_.id_;
    const targetId = typeof pieceDiff.id_ === 'string' ? pieceDiff.id_ : pieceDiff.id_.id_;
    return pId === targetId;
  });
  const updated = [...(designDiff.pieces?.updated || [])];
  if (existingIndex >= 0) {
    updated[existingIndex] = pieceDiff;
  } else {
    updated.push(pieceDiff);
  }
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } };
};
export const removePieceFromDesignDiff = (designDiff: any, pieceId: PieceId): any => {
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
export const setPiecesInDesignDiff = (designDiff: any, pieceDiffs: { id_: PieceId, diff: PieceDiff }[]): any => {
  const updated = [...(designDiff.pieces?.updated || [])];
  pieceDiffs.forEach((pieceDiff: { id_: PieceId, diff: PieceDiff }) => {
    const existingIndex = updated.findIndex((p: { id_: PieceId, diff: PieceDiff }) => {
      const pId = typeof p.id_ === 'string' ? p.id_ : p.id_.id_;
      const targetId = typeof pieceDiff.id_ === 'string' ? pieceDiff.id_ : pieceDiff.id_.id_;
      return pId === targetId;
    });
    if (existingIndex >= 0) {
      updated[existingIndex] = pieceDiff;
    } else {
      updated.push(pieceDiff);
    }
  });
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } };
};
export const removePiecesFromDesignDiff = (designDiff: any, pieceIds: PieceId[]): any => {
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
export const removeConnectionFromDesignDiff = (designDiff: any, connectionId: ConnectionId): any => {
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
export const removeConnectionsFromDesignDiff = (designDiff: any, connectionIds: ConnectionId[]): any => {
  return {
    ...designDiff,
    connections: {
      ...designDiff.connections,
      removed: [...(designDiff.connections?.removed || []), ...connectionIds],
    },
  };
};

export const applyDesignDiff = (base: Design, diff: DesignDiff): Design => { };

export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(z.object({ id: DesignIdSchema, diff: DesignDiffSchema })).optional(),
  added: z.array(DesignSchema).optional(),
});
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;


export const mergeDesigns = (designs: Design[]): DesignDiff => {
  const pieces = designs.flatMap((d) => d.pieces ?? []);
  const connections = designs.flatMap((d) => d.connections ?? []);

  return {
    pieces: pieces.length > 0 ? { added: pieces } : undefined,
    connections: connections.length > 0 ? { added: connections } : undefined
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

export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[], connectionIds: ConnectionIdLike[]): DesignDiff => {
  const normalizedPieceIds = pieceIds.map(pieceIdLikeToPieceId);
  const normalizedConnectionIds = connectionIds.map(connectionIdLikeToConnectionId);

  return {
    pieces: {
      removed: normalizedPieceIds
    },
    connections: {
      removed: normalizedConnectionIds
    }
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
export const flattenDesign = (kit: Kit, designId: DesignIdLike): DesignDiff => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const design = findDesignInKit(kit, normalizedDesignId);
  if (!design) {
    throw new Error(`Design ${normalizedDesignId.name} not found in kit ${kit.name}`);
  }
  const types = kit.types ?? [];

  let expandedDesign = expandDesignPieces(design, kit);

  if (!expandedDesign.pieces || expandedDesign.pieces.length === 0) return {};

  const typesDict: { [key: string]: { [key: string]: Type } } = {};
  types.forEach((t) => {
    if (!typesDict[t.name]) typesDict[t.name] = {};
    typesDict[t.name][t.variant || ""] = t;
  });
  const getType = (typeId: TypeId): Type | undefined => {
    return typesDict[typeId.name]?.[typeId.variant || ""];
  };
  const getPort = (type: Type | undefined, portId: PortId | undefined): Port | undefined => {
    if (!type?.ports) return undefined;
    return portId?.id_ ? type.ports.find((p) => p.id_ === portId.id_) : type.ports[0];
  };

  const flatDesign: Design = JSON.parse(JSON.stringify(expandedDesign));
  if (!flatDesign.pieces) flatDesign.pieces = [];

  const piecePlanes: { [pieceId: string]: Plane } = {};
  const pieceMap: { [pieceId: string]: Piece } = {};
  flatDesign.pieces!.forEach((p) => {
    if (p.id_) pieceMap[p.id_] = p;
  });

  const cy = cytoscape({
    elements: {
      nodes: flatDesign.pieces!.map((piece) => ({
        data: { id_: piece.id_, label: piece.id_ },
      })),
      edges: flatDesign.connections?.map((connection, index) => {
        const sourceId = connection.connected.piece.id_;
        const targetId = connection.connecting.piece.id_;
        return {
          data: {
            id: `${sourceId}--${targetId}`,
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

  components.forEach((component) => {
    let roots = component.nodes().filter((node) => {
      const piece = pieceMap[node.id()];
      return piece?.plane !== undefined;
    });
    let rootNode = roots.length > 0 ? roots[0] : component.nodes().length > 0 ? component.nodes()[0] : undefined;
    if (!rootNode) return;
    const rootPiece = pieceMap[rootNode.id()];
    if (!rootPiece || !rootPiece.id_) return;
    const updatedRootPiece = setAttributes(rootPiece, [
      { key: "semio.fixedPieceId", value: rootPiece.id_ },
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
      console.warn(`Root piece ${rootPiece.id_} has no defined plane and is not the first root. Defaulting to identity plane.`);
      const identityMatrix = new THREE.Matrix4().identity();
      rootPlane = matrixToPlane(identityMatrix);
    }

    piecePlanes[rootPiece.id_] = rootPlane;
    const rootPieceIndex = flatDesign.pieces!.findIndex((p) => p.id_ === rootPiece.id_);
    if (rootPieceIndex !== -1) {
      flatDesign.pieces![rootPieceIndex].plane = rootPlane;
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
        if (!parentPiece || !childPiece || !parentPiece.id_ || !childPiece.id_) return;
        if (piecePlanes[childPiece.id_]) return;
        const parentPlane = piecePlanes[parentPiece.id_];
        if (!parentPlane) {
          console.error(`Error during flatten: Parent piece ${parentPiece.id_} plane not found.`);
          return;
        }
        const parentSide = connection.connected.piece.id_ === parentId ? connection.connected : connection.connecting;
        const childSide = connection.connecting.piece.id_ === childId ? connection.connecting : connection.connected;
        const parentType = getType(parentPiece.type);
        const childType = getType(childPiece.type);
        const parentPort = getPort(parentType, parentSide.port);
        const childPort = getPort(childType, childSide.port);
        if (!parentPort || !childPort) {
          console.error(`Error during flatten: Ports not found for connection between ${parentId} and ${childId}. Parent Port: ${parentSide.port.id_}, Child Port: ${childSide.port.id_}`);
          return;
        }
        const childPlane = roundPlane(computeChildPlane(parentPlane, parentPort, childPort, connection));
        piecePlanes[childPiece.id_] = childPlane;
        const direction = vectorToThree({
          x: connection.x ?? 0,
          y: connection.y ?? 0,
          z: 0,
        }).normalize();
        const childCenter = {
          x: round(parentPiece.center!.x + (connection.x ?? 0) + direction.x),
          y: round(parentPiece.center!.y + (connection.y ?? 0) + direction.y),
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
              value: parentPiece.id_,
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
  flatDesign.pieces = flatDesign.pieces?.map((p) => pieceMap[p.id_ ?? ""]);
  flatDesign.connections = [];

  // Return the diff between original expanded design and flattened design
  const updatedPieces = flatDesign.pieces?.map(flatPiece => {
    const originalPiece = expandedDesign.pieces?.find(p => p.id_ === flatPiece.id_);
    if (!originalPiece) return null;

    // Build piece diff for pieces that changed
    const pieceDiff: PieceDiff = {};
    if (flatPiece.plane !== originalPiece.plane) pieceDiff.plane = flatPiece.plane;
    if (flatPiece.center !== originalPiece.center) pieceDiff.center = flatPiece.center;
    if (JSON.stringify(flatPiece.attributes) !== JSON.stringify(originalPiece.attributes)) pieceDiff.attributes = flatPiece.attributes;

    // Only return diff if there are changes
    if (Object.keys(pieceDiff).length === 0) return null;

    return {
      id_: { id_: flatPiece.id_ },
      diff: pieceDiff
    };
  }).filter(update => update !== null) as Array<{ id_: PieceId; diff: PieceDiff }>;

  const removedConnections = expandedDesign.connections?.map(c => ({
    connected: { piece: { id_: c.connected.piece.id_ } },
    connecting: { piece: { id_: c.connecting.piece.id_ } }
  })) || [];

  return {
    pieces: updatedPieces.length > 0 ? { updated: updatedPieces } : undefined,
    connections: removedConnections.length > 0 ? { removed: removedConnections } : undefined
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
  const clusteredPieces = (originalDesign.pieces || []).filter((piece) => clusterPieceIds.includes(piece.id_));

  if (clusteredPieces.length === 0) {
    throw new Error("No pieces found matching the provided IDs");
  }

  // Find internal connections (both pieces in cluster)
  const internalConnections = (originalDesign.connections || []).filter((connection) => clusterPieceIds.includes(connection.connected.piece.id_) && clusterPieceIds.includes(connection.connecting.piece.id_));

  // Find external connections (one piece in cluster, one outside)
  const externalConnections = (originalDesign.connections || []).filter((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.id_);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.id_);
    return connectedInCluster !== connectingInCluster; // XOR - exactly one is in cluster
  });

  // Create the clustered design
  const clusteredDesign: Design = {
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
  const piecesToRemove = clusterPieceIds.map(id => ({ id_: id }));

  // Remove all connections involving clustered pieces
  const connectionsToRemove = (originalDesign.connections || []).filter((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.id_);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.id_);
    return connectedInCluster || connectingInCluster;
  }).map(c => ({
    connected: { piece: { id_: c.connected.piece.id_ } },
    connecting: { piece: { id_: c.connecting.piece.id_ } }
  }));

  // Update external connections to use direct design references
  const updatedExternalConnections = externalConnections.map((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.id_);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.id_);

    if (connectedInCluster) {
      // Keep original piece ID but add designId to reference the nested design
      return {
        ...connection,
        connected: {
          piece: { id_: connection.connected.piece.id_ }, // Keep original piece ID
          port: connection.connected.port,
          designId: clusteredDesign.name, // Reference to nested design
        },
      };
    } else if (connectingInCluster) {
      // Keep original piece ID but add designId to reference the nested design
      return {
        ...connection,
        connecting: {
          piece: { id_: connection.connecting.piece.id_ }, // Keep original piece ID
          port: connection.connecting.port,
          designId: clusteredDesign.name, // Reference to nested design
        },
      };
    }

    return connection;
  });

  return {
    pieces: {
      removed: piecesToRemove
    },
    connections: {
      removed: connectionsToRemove,
      added: updatedExternalConnections
    }
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
    const sourceId = connection.connecting.piece.id_;
    const targetId = connection.connected.piece.id_;

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
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.id_);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.id_);
  });

  if (designIds.size === 0) {
    return expandedDesign; // No design references found
  }

  // For each referenced design, expand it
  for (const designName of Array.from(designIds)) {
    // Find the design in the kit
    const referencedDesign = findDesignInKit(kit, { name: designName });
    if (!referencedDesign) continue;

    // Recursively expand the referenced design first
    const expandedReferencedDesign = expandDesignPieces(referencedDesign, kit);

    // For design connections, use the original pieces and connections without namespacing
    const transformedPieces = (expandedReferencedDesign.pieces || []).map((piece) => ({
      ...piece,
      center: piece.center || { x: 0, y: 0 },
    }));

    const transformedConnections = expandedReferencedDesign.connections || [];

    const updatedExternalConnections = (expandedDesign.connections || []).map((connection) => {
      if (connection.connected.designPiece?.id_ === designName) {
        return {
          ...connection,
          connected: {
            ...connection.connected,
            designPiece: undefined,
          },
        };
      }

      if (connection.connecting.designPiece?.id_ === designName) {
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
  id_: string;
  designId: DesignId;
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
    if (conn.connected.designPiece) designIds.add(conn.connected.designPiece.id_);
    if (conn.connecting.designPiece) designIds.add(conn.connecting.designPiece.id_);
  });

  // Add connected designs
  Array.from(designIds).forEach((designIdString) => {
    const externalConnections =
      design.connections?.filter((connection: Connection) => {
        const connectedToDesign = connection.connected.designPiece?.id_ === designIdString;
        const connectingToDesign = connection.connecting.designPiece?.id_ === designIdString;
        return connectedToDesign || connectingToDesign;
      }) ?? [];

    includedDesigns.push({
      id_: `design-${designIdString}`,
      designId: { name: designIdString },
      type: "connected",
      externalConnections,
    });
  });

  // Add fixed designs
  (design.fixedDesigns || []).forEach((fixedDesign: any) => {
    const { designId: fixedDesignId, center, plane } = fixedDesign;
    includedDesigns.push({
      id: `fixed-design-${fixedDesignId.name}-${fixedDesignId.variant || ""}-${fixedDesignId.view || ""}`,
      designId: fixedDesignId,
      type: "fixed",
      center,
      plane,
    });
  });

  return includedDesigns;
};

export const areSameDesign = (design: DesignIdLike, other: DesignIdLike): boolean => {
  const d1 = designIdLikeToDesignId(design);
  const d2 = designIdLikeToDesignId(other);
  return d1.name === d2.name && normalize(d1.variant) === normalize(d2.variant) && normalize(d1.view) === normalize(d2.view);
};
export const hasSameDesign = (design: DesignIdLike, others: DesignIdLike[]): boolean => others.some((other) => areSameDesign(design, other));

export const isPortInUse = (design: Design, piece: Piece | PieceId, port: Port | PortId): boolean => {
  const normalizedPieceId = pieceIdLikeToPieceId(piece);
  const normalizedPortId = portIdLikeToPortId(port);
  const connections = findPieceConnectionsInDesign(design, piece);
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.id_ === normalizedPieceId.id_;
    const isPortConnected = isPieceConnected ? connection.connected.port.id_ === normalizedPortId.id_ : connection.connecting.port.id_ === normalizedPortId.id_;
    if (isPortConnected) return true;
  }
  return false;
};

export const isConnectionInDesign = (design: Design, connection: Connection | ConnectionId): boolean => {
  return design.connections?.some((c) => areSameConnection(c, connection)) ?? false;
};

export const findPieceInDesign = (design: Design, pieceId: PieceIdLike): Piece => findPiece(design.pieces ?? [], pieceId);

export const findConnectionInDesign = (design: Design, connectionId: ConnectionIdLike, strict: boolean = false): Connection => {
  return findConnection(design.connections ?? [], connectionId, strict);
};

export const findConnectionsInDesign = (design: Design, connectionIds: ConnectionIdLike[]): Connection[] => {
  return connectionIds.map((connectionId) => findConnectionInDesign(design, connectionId));
};

export const findPieceConnectionsInDesign = (design: Design, pieceId: PieceIdLike): Connection[] => {
  return findPieceConnections(design.connections ?? [], pieceId);
};

export const findConnectionPiecesInDesign = (design: Design, connection: Connection | ConnectionId): { connecting: Piece; connected: Piece } => {
  return {
    connected: findPieceInDesign(design, connection.connected.piece),
    connecting: findPieceInDesign(design, connection.connecting.piece),
  };
};

export const findStaleConnectionsInDesign = (design: Design): Connection[] => {
  return (
    design.connections?.filter((c) => {
      try {
        findPieceInDesign(design, c.connected.piece);
        findPieceInDesign(design, c.connecting.piece);
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
  name: z.string(),
  version: z.string().optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  files: z.array(FileSchema).optional(),
  authors: z.array(AuthorIdSchema).optional(),
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

export const KitIdSchema = KitSchema.pick({ name: true, version: true });
export type KitId = z.infer<typeof KitIdSchema>;
export const kitIdToString = (kit: KitId): string => `${kit.name}@${kit.version ?? ""}`;

export const KitShallowSchema = KitSchema.omit({ types: true, designs: true, qualities: true, authors: true }).extend({
  types: z.array(TypeIdSchema).optional(),
  designs: z.array(DesignIdSchema).optional(),
  qualities: z.array(QualityIdSchema).optional(),
  authors: z.array(AuthorIdSchema).optional(),
});
export type KitShallow = z.infer<typeof KitShallowSchema>;
export const serializeKitShallow = (kit: KitShallow): string => JSON.stringify(KitShallowSchema.parse(kit));
export const deserializeKitShallow = (json: string): KitShallow => KitShallowSchema.parse(JSON.parse(json));
export const KitDiffSchema = KitSchema.partial().omit({ types: true, designs: true, qualities: true, authors: true, files: true }).extend({
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  qualities: QualitiesDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  files: FilesDiffSchema.optional(),
});
export type KitDiff = z.infer<typeof KitDiffSchema>;
export const getKitDiff = (before: Kit, after: Kit): KitDiff => { };
export const inverseKitDiff = (original: Kit, appliedDiff: KitDiff): KitDiff => { };
export const mergeKitDiff = (diff1: KitDiff, diff2: KitDiff): KitDiff => { };
export const applyKitDiff = (base: Kit, diff: KitDiff): Kit => { };

export const KitsDiffSchema = z.object({
  removed: z.array(KitIdSchema).optional(),
  updated: z.array(z.object({ id: KitIdSchema, diff: KitDiffSchema })).optional(),
  added: z.array(KitSchema).optional(),
});

export const addTypeToKit = (type: Type): KitDiff => ({
  types: {
    added: [type]
  }
});
export const setTypeInKit = (type: Type): KitDiff => ({
  types: {
    updated: [{
      id: { name: type.name, variant: type.variant },
      diff: { ...type }
    }]
  }
});
export const removeTypeFromKit = (typeId: TypeIdLike): KitDiff => ({
  types: { removed: [typeIdLikeToTypeId(typeId)] }
});


export const addDesignToKit = (design: Design): KitDiff => ({
  designs: {
    added: [design]
  }
});
export const setDesignInKit = (design: Design): KitDiff => ({
  designs: {
    updated: [{
      id: { name: design.name, variant: design.variant, view: design.view },
      diff: {
        name: design.name,
        description: design.description,
        icon: design.icon,
        image: design.image,
        variant: design.variant,
        view: design.view,
        location: design.location,
        unit: design.unit,
        pieces: design.pieces ? {
          added: design.pieces
        } : undefined,
        connections: design.connections ? {
          added: design.connections
        } : undefined,
        attributes: design.attributes,
        authors: design.authors
      }
    }]
  }
});
export const removeDesignFromKit = (designId: DesignIdLike): KitDiff => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  return {
    designs: {
      removed: [normalizedDesignId]
    }
  };
};

export const updateDesignInKit = (design: Design): KitDiff => ({
  designs: {
    updated: [{
      id: { name: design.name, variant: design.variant, view: design.view },
      diff: {
        name: design.name,
        description: design.description,
        icon: design.icon,
        image: design.image,
        variant: design.variant,
        view: design.view,
        location: design.location,
        unit: design.unit,
        pieces: design.pieces ? {
          added: design.pieces
        } : undefined,
        connections: design.connections ? {
          added: design.connections
        } : undefined,
        attributes: design.attributes,
        authors: design.authors
      }
    }]
  }
});

export const findFileInKit = (kit: Kit, fileId: FileIdLike): File => {
  const normalizedFileId = fileIdLikeToFileId(fileId);
  const file = (kit.files || []).find(f => f.path === normalizedFileId.path);
  if (!file) throw new Error(`File ${normalizedFileId.path} not found in kit`);
  return file;
};

export const addFileToKit = (file: File): KitDiff => ({ files: { added: [file] } });
export const setFileInKit = (file: File): KitDiff => ({ files: { updated: [{ id: { path: file.path }, diff: { ...file } }] } });
export const removeFileFromKit = (fileId: FileIdLike): KitDiff => ({
  files: { removed: [fileIdLikeToFileId(fileId)] }
});

export const setAttributeInKit = (kitId: KitIdLike, attribute: Attribute): KitDiff => ({
  attributes: [attribute]
});

export const findReplacableDesignsForDesignPiece = (kit: Kit, currentDesignId: DesignId, designPiece: Piece): Design[] => {
  if (!designPiece.type || designPiece.type.name !== "design") return [];

  // Parse the current design ID from the piece's type.variant
  const currentVariant = designPiece.type.variant || "";
  const parts = currentVariant.split("-");
  const currentDesignName = parts[0];
  const currentDesignVariant = parts[1] || "";
  const currentDesignView = parts[2] || "";

  // Find all designs in the kit that could be replacements
  const allDesigns = kit.designs || [];

  // For now, return designs with the same name but different variant/view
  // This is a simplified implementation - in the future we could add more sophisticated
  // compatibility checking based on piece IDs and port compatibility
  return allDesigns.filter((design) => {
    // Don't include the current design
    if (design.name === currentDesignName && (design.variant || "") === currentDesignVariant && (design.view || "") === currentDesignView) {
      return false;
    }

    // For now, allow any design to be a replacement
    // TODO: Add more sophisticated compatibility checking:
    // - Same piece IDs
    // - Compatible outgoing ports
    return true;
  });
};

export const areSameKit = (kit: KitIdLike, other: KitIdLike): boolean => {
  const id = kitIdLikeToKitId(kit);
  const otherId = kitIdLikeToKitId(other);
  return id.name === otherId.name && normalize(id.version) === normalize(otherId.version);
};
export const hasSameKit = (kit: KitIdLike, others: KitIdLike[]): boolean => others.some((other) => areSameKit(kit, other));

export const findTypeInKit = (kit: Kit, typeId: TypeIdLike): Type => {
  const normalizedTypeId = typeIdLikeToTypeId(typeId);
  const type = kit.types?.find((t) => t.name === normalizedTypeId.name && normalize(t.variant) === normalize(normalizedTypeId.variant));
  if (!type) throw new Error(`Type ${normalizedTypeId.name} not found in kit ${kit.name}`);
  return type;
};

export const findDesignInKit = (kit: Kit, designId: DesignIdLike): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const design = kit.designs?.find((d) => d.name === normalizedDesignId.name && normalize(d.variant) === normalize(normalizedDesignId.variant) && normalize(d.view) === normalize(normalizedDesignId.view));
  if (!design) throw new Error(`Design ${normalizedDesignId.name} not found in kit ${kit.name}`);
  return design;
};

export const findPieceTypeInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Type => {
  const piece = findPieceInDesign(findDesignInKit(kit, designId), pieceId);
  if (!piece.type) throw new Error(`Piece ${pieceIdLikeToPieceId(pieceId).id_} has no type`);
  return findTypeInKit(kit, piece.type);
};

export const findParentPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Piece => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const parentPieceId = piecesMetadata(kit, designId).get(normalizedPieceId.id_)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${normalizedPieceId.id_} has no parent piece`);
  return findPieceInDesign(findDesignInKit(kit, designId), parentPieceId);
};

export const findParentConnectionForPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Connection => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const parentPieceId = piecesMetadata(kit, designId).get(normalizedPieceId.id_)?.parentPieceId;
  if (!parentPieceId) throw new Error(`Piece ${normalizedPieceId.id_} has no parent piece and connection`);
  return findConnectionInDesign(findDesignInKit(kit, designId), parentPieceId);
};

export const findChildrenPiecesInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Piece[] => {
  const design = findDesignInKit(kit, designId);
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const metadata = piecesMetadata(kit, designId);
  const children: Piece[] = [];
  for (const [id, data] of Array.from(metadata)) {
    if (data.parentPieceId === normalizedPieceId.id_) {
      children.push(findPieceInDesign(design, id));
    }
  }
  return children;
};

export const findUsedPortsByPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Port[] => {
  const design = findDesignInKit(kit, designId);
  const piece = findPieceInDesign(design, pieceId);
  if (!piece.type) return [];
  const type = findTypeInKit(kit, piece.type);
  const connections = findPieceConnectionsInDesign(design, pieceId);
  return connections.map((c) => findPortForPieceInConnection(type, c, pieceId));
};

export const findReplacableTypesForPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike, variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designId);
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const connections = findPieceConnectionsInDesign(design, pieceId);
  const requiredPorts: Port[] = [];
  for (const connection of connections) {
    try {
      const otherPieceId = connection.connected.piece.id_ === normalizedPieceId.id_ ? connection.connecting.piece.id_ : connection.connected.piece.id_;
      const otherPiece = findPieceInDesign(design, otherPieceId);
      if (!otherPiece.type) continue;
      const otherType = findTypeInKit(kit, otherPiece.type.name);
      const otherPortId = connection.connected.piece.id_ === normalizedPieceId.id_ ? connection.connecting.port.id_ : connection.connected.port.id_;
      const otherPort = findPortInType(otherType, otherPortId || "");
      requiredPorts.push(otherPort);
    } catch (error) {
      continue;
    }
  }
  return (
    kit.types?.filter((replacementType) => {
      if (variants !== undefined && !variants.includes(replacementType.variant ?? "")) return false;
      if (!replacementType.ports || replacementType.ports.length === 0) return requiredPorts.length === 0;
      return requiredPorts.every((requiredPort) => {
        return replacementType.ports!.some((replacementPort) => arePortsCompatible(replacementPort, requiredPort));
      });
    }) ?? []
  );
};

export const findReplacableTypesForPiecesInDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[], variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designId);
  const normalizedPieceIds = pieceIds.map((id) => (typeof id === "string" ? id : id.id_));
  const pieces = normalizedPieceIds.map((id) => findPieceInDesign(design, id));
  const externalConnections: Array<{
    connection: Connection;
    requiredPort: Port;
  }> = [];
  for (const piece of pieces) {
    const connections = findPieceConnectionsInDesign(design, piece.id_);
    for (const connection of connections) {
      const otherPieceId = connection.connected.piece.id_ === piece.id_ ? connection.connecting.piece.id_ : connection.connected.piece.id_;
      if (!normalizedPieceIds.includes(otherPieceId)) {
        try {
          const otherPiece = findPieceInDesign(design, otherPieceId);
          if (!otherPiece.type) continue;
          const otherType = findTypeInKit(kit, otherPiece.type.name);
          const otherPortId = connection.connected.piece.id_ === piece.id_ ? connection.connecting.port.id_ : connection.connected.port.id_;
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
      if (variants !== undefined && !variants.includes(replacementType.variant ?? "")) return false;
      if (!replacementType.ports || replacementType.ports.length === 0) return externalConnections.length === 0;
      return externalConnections.every(({ requiredPort }) => {
        return replacementType.ports!.some((replacementPort) => arePortsCompatible(replacementPort, requiredPort));
      });
    }) ?? []
  );
};

export const piecesMetadata = (
  kit: Kit,
  designId: DesignIdLike,
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
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const design = findDesignInKit(kit, normalizedDesignId);
  if (!design) {
    throw new Error(`Design ${normalizedDesignId.name} not found in kit ${kit.name}`);
  }
  const flattenDiff = flattenDesign(kit, normalizedDesignId);
  const flatDesign = applyDesignDiff(design, flattenDiff, true);
  const fixedPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.fixedPieceId") || p.id_);
  const parentPieceIds = flatDesign.pieces?.map((p) => findAttributeValue(p, "semio.parentPieceId", null));
  const depths = flatDesign.pieces?.map((p) => parseInt(findAttributeValue(p, "semio.depth", "0")!));
  return new Map(
    flatDesign.pieces?.map((p, index) => [
      p.id_,
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
  if (!text || text === "") {
    return "var(--color-dark)";
  }

  // Create a simple hash from the family string
  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }

  // Generate color variations based on primary, secondary, tertiary
  const baseColors = [
    {
      base: "var(--color-primary)",
      variations: ["#ff344f", "#ff5569", "#ff7684", "#ff97a0"],
    },
    {
      base: "var(--color-secondary)",
      variations: ["#34d1bf", "#4dd7c9", "#66ddd3", "#80e3dd"],
    },
    {
      base: "var(--color-tertiary)",
      variations: ["#fa9500", "#fba320", "#fcb140", "#fdc060"],
    },
    {
      base: "var(--color-success)",
      variations: ["#7eb77f", "#8ec28f", "#9ecd9f", "#aed8af"],
    },
    {
      base: "var(--color-warning)",
      variations: ["#fccf05", "#fcd525", "#fddb45", "#fde165"],
    },
    {
      base: "var(--color-info)",
      variations: ["#dbbea1", "#e1c7ae", "#e7d0bb", "#edd9c8"],
    },
  ];

  const colorSetIndex = Math.abs(hash) % baseColors.length;
  const variationIndex = Math.abs(Math.floor(hash / baseColors.length)) % baseColors[colorSetIndex].variations.length;

  return baseColors[colorSetIndex].variations[variationIndex];
};

export const colorPortsForTypes = (types: Type[]): TypesDiff => {
  const updated: { id_: TypeId; diff: TypeDiff }[] = [];

  for (const type of types) {
    const coloredPorts: Port[] = [];
    for (const port of type.ports || []) {
      const coloredPort = setAttribute(port, {
        key: "semio.color",
        value: getColorForText(port.family),
      });
      coloredPorts.push(coloredPort);
    }

    updated.push({
      id_: { name: type.name, variant: type.variant },
      diff: {
        ports: coloredPorts
      }
    });
  }

  return { updated };
};

// Helper function to parse design ID from design piece variant
export const parseDesignIdFromVariant = (variant: string): DesignId => {
  const parts = variant.split("-");
  return {
    name: parts[0],
    variant: parts[1] || undefined,
    view: parts[2] || undefined,
  };
};

// File utility functions
export const createFileFromDataUri = (url: string, dataUri: string): File => {
  const sizeMatch = dataUri.match(/data:([^;]+)(;base64)?,(.+)/);
  let size = 0;
  if (sizeMatch) {
    const data = sizeMatch[3];
    if (sizeMatch[2] === ';base64') {
      size = Math.floor(data.length * 0.75);
    } else {
      size = data.length;
    }
  }

  // Simple hash calculation (not cryptographically secure, but sufficient for tracking)
  let hash = 0;
  for (let i = 0; i < dataUri.length; i++) {
    const char = dataUri.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }

  return {
    path: url,
    size,
    hash: hash.toString(36),
    createdAt: new Date(),
    updatedAt: new Date(),
  };
};

export const KitIdLikeSchema = z.union([KitIdSchema, KitSchema, KitDiffSchema, z.tuple([z.string(), z.string()]), z.string()]);
export type KitIdLike = z.infer<typeof KitIdLikeSchema>;
export const kitIdLikeToKitId = (kit: KitIdLike): KitId => {
  if (typeof kit === "object" && "name" in kit && "version" in kit) return { name: kit.name!, version: kit.version! };
  if (typeof kit === "object" && "name" in kit) return { name: kit.name!, version: "" };
  if (Array.isArray(kit)) return { name: kit[0], version: kit[1] };
  if (typeof kit === 'string') return { name: kit };
  throw new Error("Invalid kit id like");
};

// #endregion Kit