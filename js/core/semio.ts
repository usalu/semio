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
// TODO: Group and reorder functions by similar functionality
// TODO: Conventionalize error throwing and logging

// #endregion TODOs

import cytoscape from "cytoscape";
import * as THREE from "three";
import { z } from "zod";

// #region Constants

export const ICON_WIDTH = 50;
export const TOLERANCE = 1e-5;

// #endregion Constants

//#region Types

//#region Persistence

const dataUriRegex = /^data:([a-z]+\/[a-z0-9\-\.+]+(;[a-z0-9\-\.+]+=[a-z0-9\-\.+]+)?)?(;base64)?,[a-z0-9!$&',()*+;=\-._~:@/?%\s]*$/i;
const DataUriSchema = z.string().regex(dataUriRegex, "Invalid data URI");

// https://github.com/usalu/semio#-file-
export const FileIdSchema = z.object({
  url: z.url(),
});
export const FileIdLikeSchema = z.union([FileIdSchema, z.string()]);
export const FileSchema = z.object({
  url: z.url(),
  data: DataUriSchema,
  size: z.number().optional(),
  hash: z.string().optional(),
  created: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  updated: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
});

// https://github.com/usalu/semio#-attribute-
export const AttributeSchema = z.object({
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
export const AttributeIdSchema = z.object({ key: z.string() });
export const AttributeIdLikeSchema = z.union([AttributeSchema, AttributeIdSchema, z.string()]);

// https://github.com/usalu/semio#-author-
export const AuthorSchema = z.object({ name: z.string(), email: z.string() });

// https://github.com/usalu/semio#-location-
export const LocationSchema = z.object({
  longitude: z.number(),
  latitude: z.number(),
});

// https://github.com/usalu/semio#-representation-
export const RepresentationSchema = z.object({
  url: z.string(),
  description: z.string().optional(),
  tags: z.array(z.string()).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const RepresentationIdSchema = z.object({
  tags: z.array(z.string()).optional(),
});
export const RepresentationIdLikeSchema = z.union([RepresentationSchema, RepresentationIdSchema, z.array(z.string()), z.string(), z.null(), z.undefined()]);

// https://github.com/usalu/semio#-diagram-
export const DiagramPointSchema = z.object({ x: z.number(), y: z.number() });
export const DiagramVectorSchema = z.object({ x: z.number(), y: z.number() });

// https://github.com/usalu/semio#-plane-
export const PointSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
export const VectorSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
export const PlaneSchema = z.object({
  origin: PointSchema,
  xAxis: VectorSchema,
  yAxis: VectorSchema,
});

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
  attributes: z.array(AttributeSchema).optional(),
});
export const PortIdSchema = z.object({ id_: z.string().optional() });
export const PortIdLikeSchema = z.union([PortSchema, PortIdSchema, z.string(), z.null(), z.undefined()]);

// https://github.com/usalu/semio#-type-
export const TypeSchema = z.object({
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  variant: z.string().optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string(),
  created: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  updated: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  location: LocationSchema.optional(),
  representations: z.array(RepresentationSchema).optional(),
  ports: z.array(PortSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const TypeIdSchema = z.object({
  name: z.string(),
  variant: z.string().optional(),
});
export const TypeIdLikeSchema = z.union([TypeSchema, TypeIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()]);

// https://github.com/usalu/semio#-piece-
export const PieceSchema = z.object({
  id_: z.string(),
  description: z.string().optional(),
  type: TypeIdSchema,
  plane: PlaneSchema.optional(),
  center: DiagramPointSchema.optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const PieceIdSchema = z.object({ id_: z.string() });
export const PieceIdLikeSchema = z.union([PieceSchema, PieceIdSchema, z.string()]);

// https://github.com/usalu/semio#-side-
export const SideSchema = z.object({
  piece: PieceIdSchema,
  designPiece: PieceIdSchema.optional(),
  port: PortIdSchema,
});
export const SideIdSchema = z.object({ piece: PieceIdSchema });

export const SideIdLikeSchema = z.union([SideSchema, SideIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()]);

// https://github.com/usalu/semio#-connection-
export const ConnectionSchema = z.object({
  connected: SideSchema,
  connecting: SideSchema,
  description: z.string().optional(),
  gap: z.number().optional(),
  shift: z.number().optional(),
  rise: z.number().optional(),
  rotation: z.number().optional(),
  turn: z.number().optional(),
  tilt: z.number().optional(),
  x: z.number().optional(),
  y: z.number().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const ConnectionIdSchema = z.object({
  connected: SideIdSchema,
  connecting: SideIdSchema,
});
export const ConnectionIdLikeSchema = z.union([ConnectionSchema, ConnectionIdSchema, z.tuple([z.string(), z.string()]), z.string()]);

// https://github.com/usalu/semio#-design-
export const DesignSchema = z.object({
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  variant: z.string().optional(),
  view: z.string().optional(),
  location: LocationSchema.optional(),
  unit: z.string(),
  created: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  updated: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  pieces: z.array(PieceSchema).optional(),
  connections: z.array(ConnectionSchema).optional(),
  fixedDesigns: z
    .array(
      z.object({
        designId: z.lazy(() => DesignIdSchema),
        plane: PlaneSchema.optional(),
        center: DiagramPointSchema.optional(),
      }),
    )
    .optional(),
  authors: z.array(AuthorSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const DesignIdSchema = z.object({
  name: z.string(),
  variant: z.string().optional(),
  view: z.string().optional(),
});
export const DesignIdLikeSchema = z.union([DesignSchema, DesignIdSchema, z.tuple([z.string(), z.string().optional(), z.string().optional()]), z.tuple([z.string(), z.string().optional()]), z.string()]);
export const DesignPieceSchema = z.object({
  designId: DesignIdSchema,
  plane: PlaneSchema.optional(),
  center: DiagramPointSchema.optional(),
});

// https://github.com/usalu/semio#-kit-
export const KitSchema = z.object({
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  preview: z.string().optional(),
  version: z.string().optional(),
  remote: z.string().optional(),
  homepage: z.string().optional(),
  license: z.string().optional(),
  created: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  updated: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  files: z.array(FileSchema).optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const KitIdSchema = z.object({
  name: z.string(),
  version: z.string().optional(),
});
export const KitIdLikeSchema = z.union([KitSchema, KitIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()]);

//#endregion Persistence

//#region Ephermal

export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});
export const FileDiffSchema = z.object({
  url: z.url().optional(),
  data: DataUriSchema.optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
});
export const FilesDiffSchema = z.object({
  removed: z.array(FileIdSchema).optional(),
  updated: z.array(z.object({ id: FileIdSchema, diff: FileDiffSchema })).optional(),
  added: z.array(FileSchema).optional(),
});
export const RepresentationDiffSchema = z.object({
  url: z.string().optional(),
  description: z.string().optional(),
  tags: z.array(z.string()).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const RepresentationsDiffSchema = z.object({
  removed: z.array(RepresentationIdSchema).optional(),
  updated: z.array(z.object({ id: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(),
  added: z.array(RepresentationSchema).optional(),
});
export const PortDiffSchema = z.object({
  id_: z.string().optional(),
  description: z.string().optional(),
  family: z.string().optional(),
  mandatory: z.boolean().optional(),
  t: z.number().optional(),
  compatibleFamilies: z.array(z.string()).optional(),
  point: PointSchema.optional(),
  direction: VectorSchema.optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(z.object({ id: PortIdSchema, diff: PortDiffSchema })).optional(),
  added: z.array(PortSchema).optional(),
});
export const TypeDiffSchema = z.object({
  name: z.string().optional(),
  description: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  variant: z.string().optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  created: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  updated: z
    .string()
    .transform((val) => new Date(val))
    .or(z.date())
    .optional(),
  location: LocationSchema.optional(),
  representations: z.array(RepresentationSchema).optional(),
  ports: z.array(PortSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(z.object({ id: TypeIdSchema, diff: TypeDiffSchema })).optional(),
  added: z.array(TypeSchema).optional(),
});
export const PieceDiffSchema = z.object({
  id_: z.string().optional(),
  name: z.string().optional(),
  description: z.string().optional(),
  type: TypeIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: DiagramPointSchema.optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ id: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
export const SideDiffSchema = z.object({
  piece: PieceIdSchema.optional(),
  designPiece: PieceIdSchema.optional(),
  port: PortIdSchema.optional(),
});
export const ConnectionDiffSchema = z.object({
  connected: SideDiffSchema.optional(),
  connecting: SideDiffSchema.optional(),
  description: z.string().optional(),
  gap: z.number().optional(),
  shift: z.number().optional(),
  rise: z.number().optional(),
  rotation: z.number().optional(),
  turn: z.number().optional(),
  tilt: z.number().optional(),
  x: z.number().optional(),
  y: z.number().optional(),
});
export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(z.object({ id: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(),
  added: z.array(ConnectionSchema).optional(),
});
export const DesignDiffSchema = z.object({
  name: z.string().optional(),
  description: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  variant: z.string().optional(),
  view: z.string().optional(),
  location: LocationSchema.optional(),
  unit: z.string().optional(),
  pieces: PiecesDiffSchema.optional(),
  connections: ConnectionsDiffSchema.optional(),
  attributes: z.array(AttributeSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
});
export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(z.object({ id: DesignIdSchema, diff: DesignDiffSchema })).optional(),
  added: z.array(DesignSchema).optional(),
});
export const KitDiffSchema = z.object({
  name: z.string().optional(),
  description: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  preview: z.string().optional(),
  version: z.string().optional(),
  remote: z.string().optional(),
  homepage: z.string().optional(),
  license: z.string().optional(),
  types: TypesDiffSchema.optional(),
  designs: DesignsDiffSchema.optional(),
  files: FilesDiffSchema.optional(),
  attributes: z.array(AttributeSchema).optional(),
});

export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

//#endregion Ephermal

export const schemas = {
  Kit: KitSchema,
  Design: DesignSchema,
  Type: TypeSchema,
  Piece: PieceSchema,
  Connection: ConnectionSchema,
  Port: PortSchema,
  Attribute: AttributeSchema,
  Plane: PlaneSchema,
  Point: PointSchema,
  Vector: VectorSchema,
  Author: AuthorSchema,
  Location: LocationSchema,
  Representation: RepresentationSchema,
  DiagramPoint: DiagramPointSchema,
  DiagramVector: DiagramVectorSchema,
  TypeId: TypeIdSchema,
  PieceId: PieceIdSchema,
  PortId: PortIdSchema,
  Side: SideSchema,
  SideId: SideIdSchema,
  ConnectionId: ConnectionIdSchema,
  DesignId: DesignIdSchema,
  KitId: KitIdSchema,
  AttributeId: AttributeIdSchema,
  RepresentationId: RepresentationIdSchema,
  PieceDiff: PieceDiffSchema,
  PiecesDiff: PiecesDiffSchema,
  SideDiff: SideDiffSchema,
  ConnectionDiff: ConnectionDiffSchema,
  ConnectionsDiff: ConnectionsDiffSchema,
  DesignDiff: DesignDiffSchema,
  FileDiff: FileDiffSchema,
  FilesDiff: FilesDiffSchema,
  DiffStatus: DiffStatusSchema,
};


export type Attribute = z.infer<typeof AttributeSchema>;
export type AttributeId = z.infer<typeof AttributeIdSchema>;
export type AttributeIdLike = z.infer<typeof AttributeIdLikeSchema>;
export type Representation = z.infer<typeof RepresentationSchema>;
export type RepresentationId = z.infer<typeof RepresentationIdSchema>;
export type RepresentationIdLike = z.infer<typeof RepresentationIdLikeSchema>;
export type DiagramPoint = z.infer<typeof DiagramPointSchema>;
export type DiagramVector = z.infer<typeof DiagramVectorSchema>;
export type Point = z.infer<typeof PointSchema>;
export type Vector = z.infer<typeof VectorSchema>;
export type Plane = z.infer<typeof PlaneSchema>;
export type Port = z.infer<typeof PortSchema>;
export type PortId = z.infer<typeof PortIdSchema>;
export type PortIdLike = z.infer<typeof PortIdLikeSchema>;
export type Author = z.infer<typeof AuthorSchema>;
export type Location = z.infer<typeof LocationSchema>;
export type Type = z.infer<typeof TypeSchema>;
export type TypeId = z.infer<typeof TypeIdSchema>;
export type TypeIdLike = z.infer<typeof TypeIdLikeSchema>;
export type Piece = z.infer<typeof PieceSchema>;
export type PieceId = z.infer<typeof PieceIdSchema>;
export type PieceIdLike = z.infer<typeof PieceIdLikeSchema>;
export type DesignPiece = { designId: DesignId; plane?: Plane; center?: DiagramPoint };
export type Side = z.infer<typeof SideSchema>;
export type SideId = z.infer<typeof SideIdSchema>;
export type SideIdLike = z.infer<typeof SideIdLikeSchema>;
export type Connection = z.infer<typeof ConnectionSchema>;
export type ConnectionId = z.infer<typeof ConnectionIdSchema>;
export type ConnectionIdLike = z.infer<typeof ConnectionIdLikeSchema>;
export type Design = z.infer<typeof DesignSchema>;
export type DesignId = z.infer<typeof DesignIdSchema>;
export type DesignIdLike = z.infer<typeof DesignIdLikeSchema>;
export type File = z.infer<typeof FileSchema>;
export type FileId = z.infer<typeof FileIdSchema>;
export type FileIdLike = z.infer<typeof FileIdLikeSchema>;
export type Kit = z.infer<typeof KitSchema>;
export type KitId = z.infer<typeof KitIdSchema>;
export type KitIdLike = z.infer<typeof KitIdLikeSchema>;
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;
export type PortDiff = z.infer<typeof PortDiffSchema>;
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;
export type SideDiff = z.infer<typeof SideDiffSchema>;
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;
export type FileDiff = z.infer<typeof FileDiffSchema>;
export type FilesDiff = z.infer<typeof FilesDiffSchema>;
export type KitDiff = z.infer<typeof KitDiffSchema>;
export type RepresentationsDiff = z.infer<typeof RepresentationsDiffSchema>;
export type PortsDiff = z.infer<typeof PortsDiffSchema>;
export type TypesDiff = z.infer<typeof TypesDiffSchema>;
export type Camera = z.infer<typeof CameraSchema>;

export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}

//#endregion Types

//#region Functions

const normalize = (val: string | undefined | null): string => (val === undefined || val === null ? "" : val);
const round = (value: number): number => Math.round(value / TOLERANCE) * TOLERANCE;
export const jaccard = (a: string[] | undefined, b: string[] | undefined) => {
  if ((a === undefined && b === undefined) || (a?.length === 0 && b?.length === 0)) return 1;
  const setA = new Set(a);
  const setB = new Set(b);
  const intersection = [...setA].filter((x) => setB.has(x)).length;
  const union = setA.size + setB.size - intersection;
  if (union === 0) return 0;
  return intersection / union;
};

//#region Mapping

export const attributeIdLikeToAttributeId = (attributeId: AttributeIdLike): AttributeId => {
  if (typeof attributeId === "string") return { key: attributeId };
  return { key: attributeId.key };
};

export const representationIdLikeToRepresentationId = (representationId: RepresentationIdLike): RepresentationId => {
  if (representationId === undefined || representationId === null) return { tags: [] };
  if (typeof representationId === "string") return { tags: [representationId] };
  if (Array.isArray(representationId)) return { tags: representationId };
  return { tags: representationId.tags ?? [] };
};

export const portIdLikeToPortId = (portId: PortIdLike): PortId => {
  if (portId === undefined || portId === null) return { id_: "" };
  if (typeof portId === "string") return { id_: portId };
  return { id_: portId.id_ };
};

export const typeIdLikeToTypeId = (typeId: TypeIdLike): TypeId => {
  if (typeof typeId === "string") return { name: typeId };
  if (Array.isArray(typeId)) return { name: typeId[0], variant: typeId[1] ?? undefined };
  return { name: typeId.name, variant: typeId.variant ?? undefined };
};

export const pieceIdLikeToPieceId = (pieceId: PieceIdLike): PieceId => {
  if (typeof pieceId === "string") return { id_: pieceId };
  return { id_: pieceId.id_ };
};

export const connectionIdLikeToConnectionId = (connectionId: ConnectionIdLike): ConnectionId => {
  if (typeof connectionId === "string") {
    const [connectedPieceId, connectingPieceId] = connectionId.split("--");
    return {
      connected: { piece: { id_: connectedPieceId } },
      connecting: { piece: { id_: connectingPieceId } },
    };
  }
  if (Array.isArray(connectionId)) {
    const [connectedPieceId, connectingPieceId] = connectionId;
    return {
      connected: { piece: { id_: connectedPieceId } },
      connecting: { piece: { id_: connectingPieceId } },
    };
  }
  return {
    connected: { piece: { id_: connectionId.connected.piece.id_ } },
    connecting: { piece: { id_: connectionId.connecting.piece.id_ } },
  };
};

export const designIdLikeToDesignId = (designId: DesignIdLike): DesignId => {
  if (typeof designId === "string") return { name: designId };
  if (Array.isArray(designId))
    return {
      name: designId[0],
      variant: designId[1] ?? undefined,
      view: designId[2] ?? undefined,
    };
  return {
    name: designId.name,
    variant: designId.variant ?? undefined,
    view: designId.view ?? undefined,
  };
};

export const fileIdLikeToFileId = (fileId: FileIdLike): FileId => {
  if (typeof fileId === "string") return { url: fileId };
  return { url: fileId.url };
};

export const kitIdLikeToKitId = (kitId: KitIdLike): KitId => {
  if (typeof kitId === "string") return { name: kitId };
  if (Array.isArray(kitId)) return { name: kitId[0], version: kitId[1] ?? undefined };
  return { name: kitId.name, version: kitId.version ?? undefined };
};

export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);
export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476);
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476);

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
export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z);

//#endregion Mapping

// #region Serializing

export const serialize = {
  attribute: (attribute: Attribute): string => JSON.stringify(AttributeSchema.parse(attribute), null, 2),
  author: (author: Author): string => JSON.stringify(AuthorSchema.parse(author), null, 2),
  diagramPoint: (diagramPoint: DiagramPoint): string => JSON.stringify(DiagramPointSchema.parse(diagramPoint), null, 2),
  diagramVector: (diagramVector: DiagramVector): string => JSON.stringify(DiagramVectorSchema.parse(diagramVector), null, 2),
  plane: (plane: Plane): string => JSON.stringify(PlaneSchema.parse(plane), null, 2),
  point: (point: Point): string => JSON.stringify(PointSchema.parse(point), null, 2),
  vector: (vector: Vector): string => JSON.stringify(VectorSchema.parse(vector), null, 2),
  location: (location: Location): string => JSON.stringify(LocationSchema.parse(location), null, 2),
  representation: (representation: Representation): string => JSON.stringify(RepresentationSchema.parse(representation), null, 2),
  port: (port: Port): string => JSON.stringify(PortSchema.parse(port), null, 2),
  piece: (piece: Piece): string => JSON.stringify(PieceSchema.parse(piece), null, 2),
  connection: (connection: Connection): string => JSON.stringify(ConnectionSchema.parse(connection), null, 2),
  type: (type: Type): string => JSON.stringify(TypeSchema.parse(type), null, 2),
  design: (design: Design): string => JSON.stringify(DesignSchema.parse(design), null, 2),
  file: (file: File): string => JSON.stringify(FileSchema.parse(file), null, 2),
  kit: (kit: Kit): string => JSON.stringify(KitSchema.parse(kit), null, 2),
};
export const deserialize = {
  attribute: (json: string): Attribute => AttributeSchema.parse(JSON.parse(json)),
  author: (json: string): Author => AuthorSchema.parse(JSON.parse(json)),
  diagramPoint: (json: string): DiagramPoint => DiagramPointSchema.parse(JSON.parse(json)),
  diagramVector: (json: string): DiagramVector => DiagramVectorSchema.parse(JSON.parse(json)),
  plane: (json: string): Plane => PlaneSchema.parse(JSON.parse(json)),
  point: (json: string): Point => PointSchema.parse(JSON.parse(json)),
  vector: (json: string): Vector => VectorSchema.parse(JSON.parse(json)),
  location: (json: string): Location => LocationSchema.parse(JSON.parse(json)),
  representation: (json: string): Representation => RepresentationSchema.parse(JSON.parse(json)),
  port: (json: string): Port => PortSchema.parse(JSON.parse(json)),
  piece: (json: string): Piece => PieceSchema.parse(JSON.parse(json)),
  connection: (json: string): Connection => ConnectionSchema.parse(JSON.parse(json)),
  type: (json: string): Type => TypeSchema.parse(JSON.parse(json)),
  design: (json: string): Design => DesignSchema.parse(JSON.parse(json)),
  kit: (json: string): Kit => KitSchema.parse(JSON.parse(json)),
};
export const parse = {
  attribute: (json: string): Attribute => AttributeSchema.parse(JSON.parse(json)),
  author: (json: string): Author => AuthorSchema.parse(JSON.parse(json)),
  diagramPoint: (json: string): DiagramPoint => DiagramPointSchema.parse(JSON.parse(json)),
  diagramVector: (json: string): DiagramVector => DiagramVectorSchema.parse(JSON.parse(json)),
  plane: (json: string): Plane => PlaneSchema.parse(JSON.parse(json)),
  point: (json: string): Point => PointSchema.parse(JSON.parse(json)),
  vector: (json: string): Vector => VectorSchema.parse(JSON.parse(json)),
  location: (json: string): Location => LocationSchema.parse(JSON.parse(json)),
  representation: (json: string): Representation => RepresentationSchema.parse(JSON.parse(json)),
  port: (json: string): Port => PortSchema.parse(JSON.parse(json)),
  piece: (json: string): Piece => PieceSchema.parse(JSON.parse(json)),
  connection: (json: string): Connection => ConnectionSchema.parse(JSON.parse(json)),
  type: (json: string): Type => TypeSchema.parse(JSON.parse(json)),
  design: (json: string): Design => DesignSchema.parse(JSON.parse(json)),
  file: (json: string): File => FileSchema.parse(JSON.parse(json)),
  kit: (json: string): Kit => KitSchema.parse(JSON.parse(json)),
};
export const safeParse = {
  attribute: (data: unknown) => AttributeSchema.safeParse(data),
  author: (data: unknown) => AuthorSchema.safeParse(data),
  diagramPoint: (data: unknown) => DiagramPointSchema.safeParse(data),
  diagramVector: (data: unknown) => DiagramVectorSchema.safeParse(data),
  plane: (data: unknown) => PlaneSchema.safeParse(data),
  point: (data: unknown) => PointSchema.safeParse(data),
  vector: (data: unknown) => VectorSchema.safeParse(data),
  location: (data: unknown) => LocationSchema.safeParse(data),
  representation: (data: unknown) => RepresentationSchema.safeParse(data),
  port: (data: unknown) => PortSchema.safeParse(data),
  piece: (data: unknown) => PieceSchema.safeParse(data),
  connection: (data: unknown) => ConnectionSchema.safeParse(data),
  type: (data: unknown) => TypeSchema.safeParse(data),
  design: (data: unknown) => DesignSchema.safeParse(data),
  file: (data: unknown) => FileSchema.safeParse(data),
  kit: (data: unknown) => KitSchema.safeParse(data),
};

//#endregion Serializing

//#region Diffing

export const diff = {
  get: {
    representation: (before: Representation, after: Representation): RepresentationDiff => {
      const diff: any = {};
      if (before.url !== after.url) diff.url = after.url;
      if (before.description !== after.description) diff.description = after.description;
      if (JSON.stringify(before.tags) !== JSON.stringify(after.tags)) diff.tags = after.tags;
      if (JSON.stringify(before.attributes) !== JSON.stringify(after.attributes)) diff.attributes = after.attributes;
      return diff;
    },
    port: (before: Port, after: Port): PortDiff => {
      const diff: any = {};
      if (before.id_ !== after.id_) diff.id_ = after.id_;
      if (before.description !== after.description) diff.description = after.description;
      if (before.family !== after.family) diff.family = after.family;
      if (before.mandatory !== after.mandatory) diff.mandatory = after.mandatory;
      if (before.t !== after.t) diff.t = after.t;
      if (JSON.stringify(before.compatibleFamilies) !== JSON.stringify(after.compatibleFamilies)) diff.compatibleFamilies = after.compatibleFamilies;
      if (JSON.stringify(before.point) !== JSON.stringify(after.point)) diff.point = after.point;
      if (JSON.stringify(before.direction) !== JSON.stringify(after.direction)) diff.direction = after.direction;
      if (JSON.stringify(before.attributes) !== JSON.stringify(after.attributes)) diff.attributes = after.attributes;
      return diff;
    },
    piece: (before: Piece, after: Piece): PieceDiff => {
      const diff: any = { id_: after.id_ };
      if (before.description !== after.description) diff.description = after.description;
      if (JSON.stringify(before.type) !== JSON.stringify(after.type)) diff.type = after.type;
      if (JSON.stringify(before.plane) !== JSON.stringify(after.plane)) diff.plane = after.plane;
      if (JSON.stringify(before.center) !== JSON.stringify(after.center)) diff.center = after.center;
      if (JSON.stringify(before.attributes) !== JSON.stringify(after.attributes)) diff.attributes = after.attributes;
      return diff;
    },
    connection: (before: Connection, after: Connection): ConnectionDiff => {
      const diff: any = {
        connected: { piece: after.connected.piece },
        connecting: { piece: after.connecting.piece }
      };
      if (JSON.stringify(before.connected.port) !== JSON.stringify(after.connected.port)) diff.connected.port = after.connected.port;
      if (JSON.stringify(before.connecting.port) !== JSON.stringify(after.connecting.port)) diff.connecting.port = after.connecting.port;
      if (before.description !== after.description) diff.description = after.description;
      if (before.gap !== after.gap) diff.gap = after.gap;
      if (before.shift !== after.shift) diff.shift = after.shift;
      if (before.rise !== after.rise) diff.rise = after.rise;
      if (before.rotation !== after.rotation) diff.rotation = after.rotation;
      if (before.turn !== after.turn) diff.turn = after.turn;
      if (before.tilt !== after.tilt) diff.tilt = after.tilt;
      if (before.x !== after.x) diff.x = after.x;
      if (before.y !== after.y) diff.y = after.y;
      if (JSON.stringify(before.attributes) !== JSON.stringify(after.attributes)) diff.attributes = after.attributes;
      return diff;
    },
    type: (before: Type, after: Type): TypeDiff => {
      const diff: any = {};
      if (before.name !== after.name) diff.name = after.name;
      if (before.description !== after.description) diff.description = after.description;
      if (before.icon !== after.icon) diff.icon = after.icon;
      if (before.image !== after.image) diff.image = after.image;
      if (before.variant !== after.variant) diff.variant = after.variant;
      if (before.stock !== after.stock) diff.stock = after.stock;
      if (before.virtual !== after.virtual) diff.virtual = after.virtual;
      if (before.unit !== after.unit) diff.unit = after.unit;
      if (before.created !== after.created) diff.created = after.created;
      if (before.updated !== after.updated) diff.updated = after.updated;
      if (JSON.stringify(before.location) !== JSON.stringify(after.location)) diff.location = after.location;
      if (JSON.stringify(before.representations) !== JSON.stringify(after.representations)) diff.representations = after.representations;
      if (JSON.stringify(before.ports) !== JSON.stringify(after.ports)) diff.ports = after.ports;
      if (JSON.stringify(before.authors) !== JSON.stringify(after.authors)) diff.authors = after.authors;
      if (JSON.stringify(before.attributes) !== JSON.stringify(after.attributes)) diff.attributes = after.attributes;
      return diff;
    },
    design: (before: Design, after: Design): DesignDiff => {
      const diff: any = {};
      if (before.name !== after.name) diff.name = after.name;
      if (before.description !== after.description) diff.description = after.description;
      if (before.icon !== after.icon) diff.icon = after.icon;
      if (before.image !== after.image) diff.image = after.image;
      if (before.variant !== after.variant) diff.variant = after.variant;
      if (before.view !== after.view) diff.view = after.view;
      if (JSON.stringify(before.location) !== JSON.stringify(after.location)) diff.location = after.location;
      if (before.unit !== after.unit) diff.unit = after.unit;

      // Handle pieces diff
      const beforePieces = before.pieces || [];
      const afterPieces = after.pieces || [];
      const piecesDiff: PiecesDiff = {};

      const removedPieces = beforePieces.filter(bp => !afterPieces.find(ap => ap.id_ === bp.id_));
      const addedPieces = afterPieces.filter(ap => !beforePieces.find(bp => bp.id_ === ap.id_));
      const updatedPieces = afterPieces.filter(ap => {
        const bp = beforePieces.find(bp => bp.id_ === ap.id_);
        return bp && JSON.stringify(bp) !== JSON.stringify(ap);
      }).map(ap => {
        const bp = beforePieces.find(bp => bp.id_ === ap.id_)!;
        return diff.get.piece(bp, ap);
      });

      if (removedPieces.length > 0) piecesDiff.removed = removedPieces.map(p => ({ id_: p.id_ }));
      if (addedPieces.length > 0) piecesDiff.added = addedPieces;
      if (updatedPieces.length > 0) piecesDiff.updated = updatedPieces;

      if (Object.keys(piecesDiff).length > 0) diff.pieces = piecesDiff;

      // Handle connections diff
      const beforeConnections = before.connections || [];
      const afterConnections = after.connections || [];
      const connectionsDiff: ConnectionsDiff = {};

      const removedConnections = beforeConnections.filter(bc => !afterConnections.find(ac =>
        ac.connected.piece.id_ === bc.connected.piece.id_ &&
        ac.connecting.piece.id_ === bc.connecting.piece.id_
      ));
      const addedConnections = afterConnections.filter(ac => !beforeConnections.find(bc =>
        bc.connected.piece.id_ === ac.connected.piece.id_ &&
        bc.connecting.piece.id_ === ac.connecting.piece.id_
      ));
      const updatedConnections = afterConnections.filter(ac => {
        const bc = beforeConnections.find(bc =>
          bc.connected.piece.id_ === ac.connected.piece.id_ &&
          bc.connecting.piece.id_ === ac.connecting.piece.id_
        );
        return bc && JSON.stringify(bc) !== JSON.stringify(ac);
      }).map(ac => {
        const bc = beforeConnections.find(bc =>
          bc.connected.piece.id_ === ac.connected.piece.id_ &&
          bc.connecting.piece.id_ === ac.connecting.piece.id_
        )!;
        return diff.get.connection(bc, ac);
      });

      if (removedConnections.length > 0) connectionsDiff.removed = removedConnections.map(c => ({
        connected: { piece: { id_: c.connected.piece.id_ } },
        connecting: { piece: { id_: c.connecting.piece.id_ } }
      }));
      if (addedConnections.length > 0) connectionsDiff.added = addedConnections;
      if (updatedConnections.length > 0) connectionsDiff.updated = updatedConnections;

      if (Object.keys(connectionsDiff).length > 0) diff.connections = connectionsDiff;

      return diff;
    },
    kit: (before: Kit, after: Kit): KitDiff => {
      const diff: any = {};
      if (before.name !== after.name) diff.name = after.name;
      if (before.description !== after.description) diff.description = after.description;
      if (before.icon !== after.icon) diff.icon = after.icon;
      if (before.image !== after.image) diff.image = after.image;
      if (before.preview !== after.preview) diff.preview = after.preview;
      if (before.version !== after.version) diff.version = after.version;
      if (before.remote !== after.remote) diff.remote = after.remote;
      if (before.homepage !== after.homepage) diff.homepage = after.homepage;
      if (before.license !== after.license) diff.license = after.license;

      // Handle types diff  
      const beforeTypes = before.types || [];
      const afterTypes = after.types || [];
      const typesDiff: TypesDiff = {};

      const removedTypes = beforeTypes.filter(bt => !afterTypes.find(at => at.name === bt.name && at.variant === bt.variant));
      const addedTypes = afterTypes.filter(at => !beforeTypes.find(bt => bt.name === at.name && bt.variant === at.variant));
      const updatedTypes = afterTypes.filter(at => {
        const bt = beforeTypes.find(bt => bt.name === at.name && bt.variant === at.variant);
        return bt && JSON.stringify(bt) !== JSON.stringify(at);
      }).map(at => {
        const bt = beforeTypes.find(bt => bt.name === at.name && bt.variant === at.variant)!;
        return diff.get.type(bt, at);
      });

      if (removedTypes.length > 0) typesDiff.removed = removedTypes.map(t => ({ name: t.name, variant: t.variant }));
      if (addedTypes.length > 0) typesDiff.added = addedTypes;
      if (updatedTypes.length > 0) typesDiff.updated = updatedTypes;

      if (Object.keys(typesDiff).length > 0) diff.types = typesDiff;

      // Handle designs diff
      const beforeDesigns = before.designs || [];
      const afterDesigns = after.designs || [];
      const designsDiff: DesignsDiff = {};

      const removedDesigns = beforeDesigns.filter(bd => !afterDesigns.find(ad => ad.name === bd.name && ad.variant === bd.variant && ad.view === bd.view));
      const addedDesigns = afterDesigns.filter(ad => !beforeDesigns.find(bd => bd.name === ad.name && bd.variant === ad.variant && bd.view === ad.view));
      const updatedDesigns = afterDesigns.filter(ad => {
        const bd = beforeDesigns.find(bd => bd.name === ad.name && bd.variant === ad.variant && bd.view === ad.view);
        return bd && JSON.stringify(bd) !== JSON.stringify(ad);
      }).map(ad => {
        const bd = beforeDesigns.find(bd => bd.name === ad.name && bd.variant === ad.variant && bd.view === ad.view)!;
        return diff.get.design(bd, ad);
      });

      if (removedDesigns.length > 0) designsDiff.removed = removedDesigns.map(d => ({ name: d.name, variant: d.variant, view: d.view }));
      if (addedDesigns.length > 0) designsDiff.added = addedDesigns;
      if (updatedDesigns.length > 0) designsDiff.updated = updatedDesigns;

      if (Object.keys(designsDiff).length > 0) diff.designs = designsDiff;

      // Handle files diff
      const beforeFiles = before.files || [];
      const afterFiles = after.files || [];
      const filesDiff: FilesDiff = {};

      const removedFiles = beforeFiles.filter(bf => !afterFiles.find(af => af.url === bf.url));
      const addedFiles = afterFiles.filter(af => !beforeFiles.find(bf => bf.url === af.url));
      const updatedFiles = afterFiles.filter(af => {
        const bf = beforeFiles.find(bf => bf.url === af.url);
        return bf && JSON.stringify(bf) !== JSON.stringify(af);
      }).map(af => {
        const bf = beforeFiles.find(bf => bf.url === af.url)!;
        return diff.get.file(bf, af);
      });

      if (removedFiles.length > 0) filesDiff.removed = removedFiles.map(f => ({ url: f.url }));
      if (addedFiles.length > 0) filesDiff.added = addedFiles;
      if (updatedFiles.length > 0) filesDiff.updated = updatedFiles;

      if (Object.keys(filesDiff).length > 0) diff.files = filesDiff;

      if (JSON.stringify(before.attributes) !== JSON.stringify(after.attributes)) diff.attributes = after.attributes;

      return diff;
    },
    file: (before: File, after: File): FileDiff => {
      const diff: any = {};
      if (before.url !== after.url) diff.url = after.url;
      if (before.data !== after.data) diff.data = after.data;
      if (before.size !== after.size) diff.size = after.size;
      if (before.hash !== after.hash) diff.hash = after.hash;
      return diff;
    }
  },
  apply: {
    representation: (base: Representation, diff: RepresentationDiff): Representation => ({
      url: diff.url ?? base.url,
      description: diff.description ?? base.description,
      tags: diff.tags ?? base.tags,
      attributes: diff.attributes ?? base.attributes,
    }),
    port: (base: Port, diff: PortDiff): Port => ({
      id_: diff.id_ ?? base.id_,
      description: diff.description ?? base.description,
      family: diff.family ?? base.family,
      mandatory: diff.mandatory ?? base.mandatory,
      t: diff.t ?? base.t,
      compatibleFamilies: diff.compatibleFamilies ?? base.compatibleFamilies,
      point: diff.point ?? base.point,
      direction: diff.direction ?? base.direction,
      attributes: diff.attributes ?? base.attributes,
    }),
    piece: (base: Piece, diff: PieceDiff): Piece => ({
      id_: base.id_,
      description: diff.description ?? base.description,
      type: diff.type ?? base.type,
      plane: diff.plane ?? base.plane,
      center: diff.center ?? base.center,
      attributes: diff.attributes ?? base.attributes,
    }),
    connection: (base: Connection, diff: ConnectionDiff): Connection => ({
      connected: {
        piece: diff.connected?.piece ?? base.connected.piece,
        port: diff.connected?.port ?? base.connected.port,
        designPiece: diff.connected?.designPiece ?? base.connected.designPiece
      },
      connecting: {
        piece: diff.connecting?.piece ?? base.connecting.piece,
        port: diff.connecting?.port ?? base.connecting.port,
        designPiece: diff.connecting?.designPiece ?? base.connecting.designPiece
      },
      description: diff.description ?? base.description,
      gap: diff.gap ?? base.gap,
      shift: diff.shift ?? base.shift,
      rise: diff.rise ?? base.rise,
      rotation: diff.rotation ?? base.rotation,
      turn: diff.turn ?? base.turn,
      tilt: diff.tilt ?? base.tilt,
      x: diff.x ?? base.x,
      y: diff.y ?? base.y,
      attributes: base.attributes,
    }),
    type: (base: Type, diff: TypeDiff): Type => ({
      name: diff.name ?? base.name,
      description: diff.description ?? base.description,
      icon: diff.icon ?? base.icon,
      image: diff.image ?? base.image,
      variant: diff.variant ?? base.variant,
      stock: diff.stock ?? base.stock,
      virtual: diff.virtual ?? base.virtual,
      unit: diff.unit ?? base.unit,
      created: diff.created ?? base.created,
      updated: diff.updated ?? base.updated,
      location: diff.location ?? base.location,
      representations: diff.representations ?? base.representations,
      ports: diff.ports ?? base.ports,
      authors: diff.authors ?? base.authors,
      attributes: diff.attributes ?? base.attributes,
    }),
    design: (base: Design, diff: DesignDiff): Design => {
      let pieces = base.pieces;
      let connections = base.connections;

      if (diff.pieces) {
        const basePieces = base.pieces || [];
        pieces = basePieces
          .map(p => {
            const updateDiff = diff.pieces?.updated?.find((up: PieceDiff) => up.id_ === p.id_);
            return updateDiff ? ({ ...p, ...updateDiff, id_: p.id_ }) : p;
          })
          .filter(p => !diff.pieces?.removed?.some((rp: PieceId) => rp.id_ === p.id_))
          .concat(diff.pieces?.added || []);
      }
      if (diff.connections) {
        const baseConnections = base.connections || [];
        connections = baseConnections
          .map(c => {
            const updateDiff = diff.connections?.updated?.find((uc: ConnectionDiff) =>
              uc.connected?.piece?.id_ === c.connected.piece.id_ &&
              uc.connecting?.piece?.id_ === c.connecting.piece.id_
            );
            return updateDiff ? ({
              ...c,
              ...updateDiff,
              connected: updateDiff.connected ? { ...c.connected, ...updateDiff.connected } : c.connected,
              connecting: updateDiff.connecting ? { ...c.connecting, ...updateDiff.connecting } : c.connecting
            }) : c;
          })
          .filter(c => !diff.connections?.removed?.some((rc: ConnectionId) =>
            rc.connected.piece.id_ === c.connected.piece.id_ &&
            rc.connecting.piece.id_ === c.connecting.piece.id_
          ))
          .concat(diff.connections?.added || []);
      }
      return {
        name: diff.name ?? base.name,
        description: diff.description ?? base.description,
        icon: diff.icon ?? base.icon,
        image: diff.image ?? base.image,
        variant: diff.variant ?? base.variant,
        view: diff.view ?? base.view,
        location: diff.location ?? base.location,
        unit: diff.unit ?? base.unit,
        created: base.created,
        updated: base.updated,
        pieces,
        connections,
        fixedDesigns: base.fixedDesigns,
        authors: base.authors,
        attributes: base.attributes,
      };
    },
    kit: (base: Kit, diff: KitDiff): Kit => {
      let types = base.types;
      let designs = base.designs;
      let files = base.files;
      if (diff.types) {
        const baseTypes = base.types || [];
        types = baseTypes
          .map(t => {
            const updateDiff = diff.types?.updated?.find((ut: TypeDiff) => ut.name === t.name && ut.variant === t.variant);
            return updateDiff ? ({ ...t, ...updateDiff }) : t;
          })
          .filter(t => !diff.types?.removed?.some((rt: TypeId) => rt.name === t.name && rt.variant === t.variant))
          .concat(diff.types?.added || []);
      }
      if (diff.files) {
        const baseFiles = base.files || [];
        files = baseFiles
          .map(f => {
            const updateDiff = diff.files?.updated?.find((uf: FileDiff) => uf.url === f.url);
            return updateDiff ? ({ ...f, ...updateDiff }) : f;
          })
          .filter(f => !diff.files?.removed?.some((rf: FileId) => rf.url === f.url))
          .concat(diff.files?.added || []);
      }
      if (diff.designs) {
        const baseDesigns = base.designs || [];
        designs = baseDesigns
          .map(d => {
            const updateDiff = diff.designs?.updated?.find((ud: DesignDiff) => ud.name === d.name && ud.variant === d.variant && ud.view === d.view);
            if (!updateDiff) return d;
            // Apply the design diff properly by using the apply function logic
            let pieces = d.pieces;
            let connections = d.connections;

            if (updateDiff.pieces) {
              const basePieces = d.pieces || [];
              pieces = basePieces
                .map(p => {
                  const pieceDiff = updateDiff.pieces?.updated?.find((up: PieceDiff) => up.id_ === p.id_);
                  return pieceDiff ? ({ ...p, ...pieceDiff, id_: p.id_ }) : p;
                })
                .filter(p => !updateDiff.pieces?.removed?.some((rp: PieceId) => rp.id_ === p.id_))
                .concat(updateDiff.pieces?.added || []);
            }
            if (updateDiff.connections) {
              const baseConnections = d.connections || [];
              connections = baseConnections
                .map(c => {
                  const connDiff = updateDiff.connections?.updated?.find((uc: ConnectionDiff) =>
                    uc.connected?.piece?.id_ === c.connected.piece.id_ &&
                    uc.connecting?.piece?.id_ === c.connecting.piece.id_
                  );
                  return connDiff ? ({
                    ...c,
                    ...connDiff,
                    connected: connDiff.connected ? { ...c.connected, ...connDiff.connected } : c.connected,
                    connecting: connDiff.connecting ? { ...c.connecting, ...connDiff.connecting } : c.connecting
                  }) : c;
                })
                .filter(c => !updateDiff.connections?.removed?.some((rc: ConnectionId) =>
                  rc.connected.piece.id_ === c.connected.piece.id_ &&
                  rc.connecting.piece.id_ === c.connecting.piece.id_
                ))
                .concat(updateDiff.connections?.added || []);
            }
            return {
              ...d,
              name: updateDiff.name ?? d.name,
              description: updateDiff.description ?? d.description,
              icon: updateDiff.icon ?? d.icon,
              image: updateDiff.image ?? d.image,
              variant: updateDiff.variant ?? d.variant,
              view: updateDiff.view ?? d.view,
              location: updateDiff.location ?? d.location,
              unit: updateDiff.unit ?? d.unit,
              pieces,
              connections
            };
          })
          .filter(d => !diff.designs?.removed?.some((rd: DesignId) => rd.name === d.name && rd.variant === d.variant && rd.view === d.view))
          .concat(diff.designs?.added || []);
      }

      return {
        name: diff.name ?? base.name,
        description: diff.description ?? base.description,
        icon: diff.icon ?? base.icon,
        image: diff.image ?? base.image,
        preview: diff.preview ?? base.preview,
        version: diff.version ?? base.version,
        remote: diff.remote ?? base.remote,
        homepage: diff.homepage ?? base.homepage,
        license: diff.license ?? base.license,
        created: base.created,
        updated: base.updated,
        types,
        designs,
        files,
        attributes: base.attributes,
      };
    },
    file: (base: File, diff: FileDiff): File => ({
      url: diff.url ?? base.url,
      data: diff.data ?? base.data,
      size: diff.size ?? base.size,
      hash: diff.hash ?? base.hash,
      created: base.created,
      updated: base.updated,
    })
  },
  merge: {
    representation: (diff1: RepresentationDiff, diff2: RepresentationDiff): RepresentationDiff => ({
      url: diff2.url ?? diff1.url,
      description: diff2.description ?? diff1.description,
      tags: diff2.tags ?? diff1.tags,
      attributes: diff2.attributes ?? diff1.attributes,
    }),

    port: (diff1: PortDiff, diff2: PortDiff): PortDiff => ({
      id_: diff2.id_ ?? diff1.id_,
      description: diff2.description ?? diff1.description,
      family: diff2.family ?? diff1.family,
      mandatory: diff2.mandatory ?? diff1.mandatory,
      t: diff2.t ?? diff1.t,
      compatibleFamilies: diff2.compatibleFamilies ?? diff1.compatibleFamilies,
      point: diff2.point ?? diff1.point,
      direction: diff2.direction ?? diff1.direction,
      attributes: diff2.attributes ?? diff1.attributes,
    }),

    piece: (diff1: PieceDiff, diff2: PieceDiff): PieceDiff => ({
      id_: diff2.id_ ?? diff1.id_,
      description: diff2.description ?? diff1.description,
      type: diff2.type ?? diff1.type,
      plane: diff2.plane ?? diff1.plane,
      center: diff2.center ?? diff1.center,
      attributes: diff2.attributes ?? diff1.attributes,
    }),

    connection: (diff1: ConnectionDiff, diff2: ConnectionDiff): ConnectionDiff => ({
      connected: {
        piece: diff2.connected?.piece ?? diff1.connected?.piece,
        port: diff2.connected?.port ?? diff1.connected?.port,
        designPiece: diff2.connected?.designPiece ?? diff1.connected?.designPiece
      },
      connecting: {
        piece: diff2.connecting?.piece ?? diff1.connecting?.piece,
        port: diff2.connecting?.port ?? diff1.connecting?.port,
        designPiece: diff2.connecting?.designPiece ?? diff1.connecting?.designPiece
      },
      description: diff2.description ?? diff1.description,
      gap: diff2.gap ?? diff1.gap,
      shift: diff2.shift ?? diff1.shift,
      rise: diff2.rise ?? diff1.rise,
      rotation: diff2.rotation ?? diff1.rotation,
      turn: diff2.turn ?? diff1.turn,
      tilt: diff2.tilt ?? diff1.tilt,
      x: diff2.x ?? diff1.x,
      y: diff2.y ?? diff1.y,
    }),

    type: (diff1: TypeDiff, diff2: TypeDiff): TypeDiff => ({
      name: diff2.name ?? diff1.name,
      description: diff2.description ?? diff1.description,
      icon: diff2.icon ?? diff1.icon,
      image: diff2.image ?? diff1.image,
      variant: diff2.variant ?? diff1.variant,
      stock: diff2.stock ?? diff1.stock,
      virtual: diff2.virtual ?? diff1.virtual,
      unit: diff2.unit ?? diff1.unit,
      created: diff2.created ?? diff1.created,
      updated: diff2.updated ?? diff1.updated,
      location: diff2.location ?? diff1.location,
      representations: diff2.representations ?? diff1.representations,
      ports: diff2.ports ?? diff1.ports,
      authors: diff2.authors ?? diff1.authors,
      attributes: diff2.attributes ?? diff1.attributes,
    }),

    design: (diff1: DesignDiff, diff2: DesignDiff): DesignDiff => ({
      name: diff2.name ?? diff1.name,
      description: diff2.description ?? diff1.description,
      icon: diff2.icon ?? diff1.icon,
      image: diff2.image ?? diff1.image,
      variant: diff2.variant ?? diff1.variant,
      view: diff2.view ?? diff1.view,
      location: diff2.location ?? diff1.location,
      unit: diff2.unit ?? diff1.unit,
      pieces: diff2.pieces ?? diff1.pieces,
      connections: diff2.connections ?? diff1.connections,
      attributes: diff2.attributes ?? diff1.attributes,
    }),

    kit: (diff1: KitDiff, diff2: KitDiff): KitDiff => ({
      name: diff2.name ?? diff1.name,
      description: diff2.description ?? diff1.description,
      icon: diff2.icon ?? diff1.icon,
      image: diff2.image ?? diff1.image,
      preview: diff2.preview ?? diff1.preview,
      version: diff2.version ?? diff1.version,
      remote: diff2.remote ?? diff1.remote,
      homepage: diff2.homepage ?? diff1.homepage,
      license: diff2.license ?? diff1.license,
      types: diff2.types ?? diff1.types,
      designs: diff2.designs ?? diff1.designs,
      files: diff2.files ?? diff1.files,
      attributes: diff2.attributes ?? diff1.attributes,
    }),

    file: (diff1: FileDiff, diff2: FileDiff): FileDiff => ({
      url: diff2.url ?? diff1.url,
      data: diff2.data ?? diff1.data,
      size: diff2.size ?? diff1.size,
      hash: diff2.hash ?? diff1.hash,
    }),
  },
  inverse: {
    representation: (original: Representation, appliedDiff: RepresentationDiff): RepresentationDiff => {
      const inverseDiff: any = {};
      if (appliedDiff.url !== undefined) inverseDiff.url = original.url;
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.tags !== undefined) inverseDiff.tags = original.tags;
      if (appliedDiff.attributes !== undefined) inverseDiff.attributes = original.attributes;
      return inverseDiff;
    },
    port: (original: Port, appliedDiff: PortDiff): PortDiff => {
      const inverseDiff: any = {};
      if (appliedDiff.id_ !== undefined) inverseDiff.id_ = original.id_;
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.family !== undefined) inverseDiff.family = original.family;
      if (appliedDiff.mandatory !== undefined) inverseDiff.mandatory = original.mandatory;
      if (appliedDiff.t !== undefined) inverseDiff.t = original.t;
      if (appliedDiff.compatibleFamilies !== undefined) inverseDiff.compatibleFamilies = original.compatibleFamilies;
      if (appliedDiff.point !== undefined) inverseDiff.point = original.point;
      if (appliedDiff.direction !== undefined) inverseDiff.direction = original.direction;
      if (appliedDiff.attributes !== undefined) inverseDiff.attributes = original.attributes;
      return inverseDiff;
    },
    piece: (original: Piece, appliedDiff: PieceDiff): PieceDiff => {
      const inverseDiff: any = { id_: original.id_ };
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.type !== undefined) inverseDiff.type = original.type;
      if (appliedDiff.plane !== undefined) inverseDiff.plane = original.plane;
      if (appliedDiff.center !== undefined) inverseDiff.center = original.center;
      if (appliedDiff.attributes !== undefined) inverseDiff.attributes = original.attributes;
      return inverseDiff;
    },
    connection: (original: Connection, appliedDiff: ConnectionDiff): ConnectionDiff => {
      const inverseDiff: any = {
        connected: { piece: original.connected.piece },
        connecting: { piece: original.connecting.piece }
      };
      if (appliedDiff.connected?.port !== undefined) inverseDiff.connected.port = original.connected.port;
      if (appliedDiff.connecting?.port !== undefined) inverseDiff.connecting.port = original.connecting.port;
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.gap !== undefined) inverseDiff.gap = original.gap;
      if (appliedDiff.shift !== undefined) inverseDiff.shift = original.shift;
      if (appliedDiff.rise !== undefined) inverseDiff.rise = original.rise;
      if (appliedDiff.rotation !== undefined) inverseDiff.rotation = original.rotation;
      if (appliedDiff.turn !== undefined) inverseDiff.turn = original.turn;
      if (appliedDiff.tilt !== undefined) inverseDiff.tilt = original.tilt;
      if (appliedDiff.x !== undefined) inverseDiff.x = original.x;
      if (appliedDiff.y !== undefined) inverseDiff.y = original.y;
      return inverseDiff;
    },
    type: (original: Type, appliedDiff: TypeDiff): TypeDiff => {
      const inverseDiff: any = {};
      if (appliedDiff.name !== undefined) inverseDiff.name = original.name;
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.icon !== undefined) inverseDiff.icon = original.icon;
      if (appliedDiff.image !== undefined) inverseDiff.image = original.image;
      if (appliedDiff.variant !== undefined) inverseDiff.variant = original.variant;
      if (appliedDiff.stock !== undefined) inverseDiff.stock = original.stock;
      if (appliedDiff.virtual !== undefined) inverseDiff.virtual = original.virtual;
      if (appliedDiff.unit !== undefined) inverseDiff.unit = original.unit;
      if (appliedDiff.created !== undefined) inverseDiff.created = original.created;
      if (appliedDiff.updated !== undefined) inverseDiff.updated = original.updated;
      if (appliedDiff.location !== undefined) inverseDiff.location = original.location;
      if (appliedDiff.representations !== undefined) inverseDiff.representations = original.representations;
      if (appliedDiff.ports !== undefined) inverseDiff.ports = original.ports;
      if (appliedDiff.authors !== undefined) inverseDiff.authors = original.authors;
      if (appliedDiff.attributes !== undefined) inverseDiff.attributes = original.attributes;
      return inverseDiff;
    },
    design: (original: Design, appliedDiff: DesignDiff): DesignDiff => {
      const inverseDiff: any = {};
      if (appliedDiff.name !== undefined) inverseDiff.name = original.name;
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.icon !== undefined) inverseDiff.icon = original.icon;
      if (appliedDiff.image !== undefined) inverseDiff.image = original.image;
      if (appliedDiff.variant !== undefined) inverseDiff.variant = original.variant;
      if (appliedDiff.view !== undefined) inverseDiff.view = original.view;
      if (appliedDiff.location !== undefined) inverseDiff.location = original.location;
      if (appliedDiff.unit !== undefined) inverseDiff.unit = original.unit;

      // Handle pieces diff inverse
      if (appliedDiff.pieces) {
        const originalPieces = original.pieces || [];
        const piecesDiff: PiecesDiff = {};

        // Swap added and removed
        if (appliedDiff.pieces.added) piecesDiff.removed = appliedDiff.pieces.added.map(p => ({ id_: p.id_ }));
        if (appliedDiff.pieces.removed) piecesDiff.added = appliedDiff.pieces.removed.map(rp => {
          return originalPieces.find(p => p.id_ === rp.id_)!;
        });

        // Inverse updated pieces
        if (appliedDiff.pieces.updated) {
          piecesDiff.updated = appliedDiff.pieces.updated.map(updatedPiece => {
            const originalPiece = originalPieces.find(p => p.id_ === updatedPiece.id_)!;
            return diff.inverse.piece(originalPiece, updatedPiece);
          });
        }

        if (Object.keys(piecesDiff).length > 0) inverseDiff.pieces = piecesDiff;
      }

      // Handle connections diff inverse
      if (appliedDiff.connections) {
        const originalConnections = original.connections || [];
        const connectionsDiff: ConnectionsDiff = {};

        // Swap added and removed
        if (appliedDiff.connections.added) connectionsDiff.removed = appliedDiff.connections.added.map(c => ({
          connected: { piece: { id_: c.connected.piece.id_ } },
          connecting: { piece: { id_: c.connecting.piece.id_ } }
        }));
        if (appliedDiff.connections.removed) connectionsDiff.added = appliedDiff.connections.removed.map(rc => {
          return originalConnections.find(c =>
            c.connected.piece.id_ === rc.connected.piece.id_ &&
            c.connecting.piece.id_ === rc.connecting.piece.id_
          )!;
        });

        // Inverse updated connections
        if (appliedDiff.connections.updated) {
          connectionsDiff.updated = appliedDiff.connections.updated.map(updatedConnection => {
            const originalConnection = originalConnections.find(c =>
              c.connected.piece.id_ === updatedConnection.connected?.piece?.id_ &&
              c.connecting.piece.id_ === updatedConnection.connecting?.piece?.id_
            )!;
            return diff.inverse.connection(originalConnection, updatedConnection);
          });
        }

        if (Object.keys(connectionsDiff).length > 0) inverseDiff.connections = connectionsDiff;
      }

      return inverseDiff;
    },
    kit: (original: Kit, appliedDiff: KitDiff): KitDiff => {
      const inverseDiff: any = {};
      if (appliedDiff.name !== undefined) inverseDiff.name = original.name;
      if (appliedDiff.description !== undefined) inverseDiff.description = original.description;
      if (appliedDiff.icon !== undefined) inverseDiff.icon = original.icon;
      if (appliedDiff.image !== undefined) inverseDiff.image = original.image;
      if (appliedDiff.preview !== undefined) inverseDiff.preview = original.preview;
      if (appliedDiff.version !== undefined) inverseDiff.version = original.version;
      if (appliedDiff.remote !== undefined) inverseDiff.remote = original.remote;
      if (appliedDiff.homepage !== undefined) inverseDiff.homepage = original.homepage;
      if (appliedDiff.license !== undefined) inverseDiff.license = original.license;

      // Handle types diff inverse
      if (appliedDiff.types) {
        const originalTypes = original.types || [];
        const typesDiff: TypesDiff = {};

        // Swap added and removed
        if (appliedDiff.types.added) typesDiff.removed = appliedDiff.types.added.map(t => ({ name: t.name, variant: t.variant }));
        if (appliedDiff.types.removed) typesDiff.added = appliedDiff.types.removed.map(rt => {
          return originalTypes.find(t => t.name === rt.name && t.variant === rt.variant)!;
        });

        // Inverse updated types
        if (appliedDiff.types.updated) {
          typesDiff.updated = appliedDiff.types.updated.map(updatedType => {
            const originalType = originalTypes.find(t => t.name === updatedType.name && t.variant === updatedType.variant)!;
            return diff.inverse.type(originalType, updatedType);
          });
        }

        if (Object.keys(typesDiff).length > 0) inverseDiff.types = typesDiff;
      }

      // Handle designs diff inverse
      if (appliedDiff.designs) {
        const originalDesigns = original.designs || [];
        const designsDiff: DesignsDiff = {};

        // Swap added and removed
        if (appliedDiff.designs.added) designsDiff.removed = appliedDiff.designs.added.map(d => ({ name: d.name, variant: d.variant, view: d.view }));
        if (appliedDiff.designs.removed) designsDiff.added = appliedDiff.designs.removed.map(rd => {
          return originalDesigns.find(d => d.name === rd.name && d.variant === rd.variant && d.view === rd.view)!;
        });

        // Inverse updated designs
        if (appliedDiff.designs.updated) {
          designsDiff.updated = appliedDiff.designs.updated.map(updatedDesign => {
            const originalDesign = originalDesigns.find(d => d.name === updatedDesign.name && d.variant === updatedDesign.variant && d.view === updatedDesign.view)!;
            return diff.inverse.design(originalDesign, updatedDesign);
          });
        }

        if (Object.keys(designsDiff).length > 0) inverseDiff.designs = designsDiff;
      }

      // Handle files diff inverse
      if (appliedDiff.files) {
        const originalFiles = original.files || [];
        const filesDiff: FilesDiff = {};

        // Swap added and removed
        if (appliedDiff.files.added) filesDiff.removed = appliedDiff.files.added.map(f => ({ url: f.url }));
        if (appliedDiff.files.removed) filesDiff.added = appliedDiff.files.removed.map(rf => {
          return originalFiles.find(f => f.url === rf.url)!;
        });

        // Inverse updated files
        if (appliedDiff.files.updated) {
          filesDiff.updated = appliedDiff.files.updated.map(updatedFile => {
            const originalFile = originalFiles.find(f => f.url === updatedFile.url)!;
            return diff.inverse.file(originalFile, updatedFile);
          });
        }

        if (Object.keys(filesDiff).length > 0) inverseDiff.files = filesDiff;
      }

      if (appliedDiff.attributes !== undefined) inverseDiff.attributes = original.attributes;

      return inverseDiff;
    },
    file: (original: File, appliedDiff: FileDiff): FileDiff => {
      const inverseDiff: any = {};
      if (appliedDiff.url !== undefined) inverseDiff.url = original.url;
      if (appliedDiff.data !== undefined) inverseDiff.data = original.data;
      if (appliedDiff.size !== undefined) inverseDiff.size = original.size;
      if (appliedDiff.hash !== undefined) inverseDiff.hash = original.hash;
      return inverseDiff;
    }
  }
};
//#endregion Diffing

//#region Querying

//#region Propositional

export const arePortsCompatible = (port: Port, otherPort: Port): boolean => {
  const normalizedPortFamily = normalize(port.family);
  const normalizedOtherPortFamily = normalize(otherPort.family);
  if (normalizedPortFamily === "" || normalizedOtherPortFamily === "") return true;
  return (port.compatibleFamilies ?? []).includes(normalizedOtherPortFamily) || (otherPort.compatibleFamilies ?? []).includes(normalizedPortFamily);
};

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
  return design.connections?.some((c) => isSameConnection(c, connection)) ?? false;
};

export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined;
  const isCenterSet = piece.center !== undefined;
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.id_} has inconsistent plane and center`);
  return isPlaneSet;
};

export const isSameRepresentation = (representation: Representation, other: Representation): boolean => {
  return representation.tags?.every((tag) => other.tags?.includes(tag)) ?? true;
};
export const isSamePort = (port: Port | PortId, other: Port | PortId): boolean => {
  const p1 = portIdLikeToPortId(port);
  const p2 = portIdLikeToPortId(other);
  return normalize(p1.id_) === normalize(p2.id_);
};
export const isSameType = (type: Type | TypeId, other: Type | TypeId): boolean => {
  const t1 = typeIdLikeToTypeId(type);
  const t2 = typeIdLikeToTypeId(other);
  return t1.name === t2.name && normalize(t1.variant) === normalize(t2.variant);
};
export const isSamePiece = (piece: Piece | PieceId, other: Piece | PieceId): boolean => {
  const p1 = pieceIdLikeToPieceId(piece);
  const p2 = pieceIdLikeToPieceId(other);
  return normalize(p1.id_) === normalize(p2.id_);
};
export const isSameConnection = (connection: Connection | ConnectionId | ConnectionDiff, other: Connection | ConnectionId | ConnectionDiff, strict: boolean = false): boolean => {
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
export const isSameDesign = (design: DesignIdLike, other: DesignIdLike): boolean => {
  const d1 = designIdLikeToDesignId(design);
  const d2 = designIdLikeToDesignId(other);
  return d1.name === d2.name && normalize(d1.variant) === normalize(d2.variant) && normalize(d1.view) === normalize(d2.view);
};
export const isSameKit = (kit: Kit | KitId, other: Kit | KitId): boolean => {
  return kit.name === other.name && normalize(kit.version) === normalize(other.version);
};

//#endregion Propositional

//#region Predicates

export const findAttributeValue = (entity: Kit | Type | Design | Piece | Connection | Representation | Port, name: string, defaultValue?: string | null): string | null => {
  const attribute = entity.attributes?.find((q) => q.key === name);
  if (!attribute && defaultValue === undefined) throw new Error(`Attribute ${name} not found in ${entity}`);
  if (attribute?.value === undefined && defaultValue === null) return null;
  return attribute?.value ?? defaultValue ?? "";
};
const findRepresentation = (representations: Representation[], tags: string[]): Representation => {
  const indices = representations.map((r) => jaccard(r.tags, tags));
  const maxIndex = Math.max(...indices);
  const maxIndexIndex = indices.indexOf(maxIndex);
  return representations[maxIndexIndex];
};
export const findPort = (ports: Port[], portId: PortIdLike): Port => {
  const normalizedPortId = portIdLikeToPortId(portId);
  const port = ports.find((p) => normalize(p.id_) === normalize(normalizedPortId.id_));
  if (!port) throw new Error(`Port ${normalizedPortId.id_} not found in ports`);
  return port;
};
export const findPortInType = (type: Type, portId: PortIdLike): Port => findPort(type.ports ?? [], portId);
export const findPiece = (pieces: Piece[], pieceId: PieceIdLike): Piece => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const piece = pieces.find((p) => p.id_ === normalizedPieceId.id_);
  if (!piece) throw new Error(`Piece ${normalizedPieceId.id_} not found in pieces`);
  return piece;
};
export const findPortForPieceInConnection = (type: Type, connection: Connection, pieceId: PieceIdLike): Port => {
  const portId = connection.connected.piece.id_ === pieceId ? connection.connected.port.id_ : connection.connecting.port.id_;
  return findPortInType(type, portId);
};
export const findPieceInDesign = (design: Design, pieceId: PieceIdLike): Piece => findPiece(design.pieces ?? [], pieceId);
export const findPieceTypeInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Type => findTypeInKit(kit, findPieceInDesign(findDesignInKit(kit, designId), pieceId).type);
export const findConnection = (connections: Connection[], connectionId: ConnectionIdLike, strict: boolean = false): Connection => {
  const normalizedConnectionId = connectionIdLikeToConnectionId(connectionId);
  const connection = connections.find((c) => isSameConnection(c, normalizedConnectionId, strict));
  if (!connection) throw new Error(`Connection ${normalizedConnectionId.connected.piece.id_} -> ${normalizedConnectionId.connecting.piece.id_} not found in connections`);
  return connection;
};
export const findConnectionInDesign = (design: Design, connectionId: ConnectionIdLike, strict: boolean = false): Connection => {
  return findConnection(design.connections ?? [], connectionId, strict);
};
export const findConnectionsInDesign = (design: Design, connectionIds: ConnectionIdLike[]): Connection[] => {
  return connectionIds.map((connectionId) => findConnectionInDesign(design, connectionId));
};
export const findPieceConnections = (connections: Connection[], pieceId: PieceIdLike): Connection[] => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  return connections.filter((c) => c.connected.piece.id_ === normalizedPieceId.id_ || c.connecting.piece.id_ === normalizedPieceId.id_);
};
export const findPieceConnectionsInDesign = (design: Design, pieceId: PieceIdLike): Connection[] => {
  return findPieceConnections(design.connections ?? [], pieceId);
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
  for (const [id, data] of metadata) {
    if (data.parentPieceId === normalizedPieceId.id_) {
      children.push(findPieceInDesign(design, id));
    }
  }
  return children;
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
export const findUsedPortsByPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Port[] => {
  const design = findDesignInKit(kit, designId);
  const piece = findPieceInDesign(design, pieceId);
  const type = findTypeInKit(kit, piece.type);
  const connections = findPieceConnectionsInDesign(design, pieceId);
  return connections.map((c) => findPortForPieceInConnection(type, c, pieceId));
};
export const findReplacableTypesForPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike, variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designId);
  const connections = findPieceConnectionsInDesign(design, pieceId);
  const requiredPorts: Port[] = [];
  for (const connection of connections) {
    try {
      const otherPieceId = connection.connected.piece.id_ === pieceId ? connection.connecting.piece.id_ : connection.connected.piece.id_;
      const otherPiece = findPieceInDesign(design, otherPieceId);
      const otherType = findTypeInKit(kit, otherPiece.type);
      const otherPortId = connection.connected.piece.id_ === pieceId ? connection.connecting.port.id_ : connection.connected.port.id_;
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
  const pieces = pieceIds.map((id) => findPieceInDesign(design, id));
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
          const otherType = findTypeInKit(kit, otherPiece.type);
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
    center: DiagramPoint;
    fixedPieceId: string;
    parentPieceId: string | null;
    depth: number;
  }
> => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const flatDesign = flattenDesign(kit, normalizedDesignId);
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

export const colorPortsForTypes = (types: Type[]): Type[] => {
  const coloredTypes: Type[] = [];
  for (const type of unifyPortFamiliesAndCompatibleFamiliesForTypes(types)) {
    const coloredType: Type = { ...type };
    for (const port of type.ports || []) {
      const coloredPort = setAttribute(port, {
        key: "semio.color",
        value: getColorForText(port.family),
      });
      coloredType.ports = [...(coloredType.ports || []), coloredPort];
    }
    coloredTypes.push(coloredType);
  }
  return coloredTypes;
};

//#endregion Predicates

//#endregion Querying

//#region CRUDs

//#region Plane

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

//#endregion Plane

//#region Attribute

export const setAttribute = <T extends Kit | Design | Type | Piece | Connection | Representation | Port>(entity: T, attribute: Attribute): T => {
  const attributesArray = entity.attributes || [];
  const existingIndex = attributesArray.findIndex((q) => q.key === attribute.key);
  if (existingIndex >= 0) attributesArray[existingIndex] = attribute;
  else attributesArray.push(attribute);
  return { ...entity, attributes: attributesArray };
};

export const setAttributes = <T extends Kit | Design | Type | Piece | Connection | Representation | Port>(entity: T, attributes: Attribute[]): T => {
  return attributes.reduce((acc, attribute) => setAttribute(acc, attribute), entity);
};

//#endregion Attribute

//#region Port

export const unifyPortFamiliesAndCompatibleFamiliesForTypes = (types: Type[]): Type[] => {
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
  for (const family of allFamilies) {
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
  for (const family of allFamilies) {
    familyToRepresentative.set(family, find(family));
  }

  // Update all types with unified port families
  return types.map((type) => ({
    ...type,
    ports: type.ports?.map((port) => {
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
    }),
  }));
};

//#endregion Port

//#region Piece

export const addPieceToDesign = (design: Design, piece: Piece): Design => ({
  ...design,
  pieces: [...(design.pieces || []), piece],
});
export const setPieceInDesign = (design: Design, piece: Piece): Design => ({
  ...design,
  pieces: (design.pieces || []).map((p) => (p.id_ === piece.id_ ? piece : p)),
});
export const removePieceFromDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Design => {
  const design = findDesignInKit(kit, designId);
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  return {
    ...design,
    pieces: (design.pieces || []).filter(p => p.id_ !== normalizedPieceId.id_),
    connections: (design.connections || []).filter(c =>
      c.connected.piece.id_ !== normalizedPieceId.id_ &&
      c.connecting.piece.id_ !== normalizedPieceId.id_
    )
  };
};

export const addPiecesToDesign = (design: Design, pieces: Piece[]): Design => ({
  ...design,
  pieces: [...(design.pieces || []), ...pieces],
});
export const setPiecesInDesign = (design: Design, pieces: Piece[]): Design => ({
  ...design,
  pieces: (design.pieces || []).map((p) => pieces.find((p2) => p2.id_ === p.id_) || p),
});
export const removePiecesFromDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[]): Design => {
  const design = findDesignInKit(kit, designId);
  const normalizedPieceIds = pieceIds.map(pieceIdLikeToPieceId).map(p => p.id_);
  return {
    ...design,
    pieces: (design.pieces || []).filter(p => !normalizedPieceIds.includes(p.id_)),
    connections: (design.connections || []).filter(c =>
      !normalizedPieceIds.includes(c.connected.piece.id_) &&
      !normalizedPieceIds.includes(c.connecting.piece.id_)
    )
  };
};

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
    const type = types.find((t) => t.name === p.type.name && normalizeVariant(t.variant) === normalizeVariant(p.type.variant));
    if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`);
    if (!type.representations) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} has no representations`);
    const representation = findRepresentation(type.representations, tags);
    representationUrls.set(p.id_, representation.url);
  });
  return representationUrls;
};
export const fixPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId);
  const parentConnection = findParentConnectionForPieceInDesign(kit, normalizedDesignId, normalizedPieceId);
  return removeConnectionFromDesign(kit, normalizedDesignId, parentConnection);
};
export const fixPiecesInDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[]): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const normalizedPieceIds = pieceIds.map(pieceIdLikeToPieceId);
  const parentConnections = normalizedPieceIds.map((pieceId) => findParentConnectionForPieceInDesign(kit, normalizedDesignId, pieceId));
  return removeConnectionsFromDesign(kit, normalizedDesignId, parentConnections);
};

//#endregion Piece

//#region Connection

export const addConnectionToDesign = (design: Design, connection: Connection): Design => ({
  ...design,
  connections: [...(design.connections || []), connection],
});
export const setConnectionInDesign = (design: Design, connection: Connection): Design => {
  return {
    ...design,
    connections: (design.connections || []).map((c) =>
      isSameConnection(c, {
        connected: connection.connected,
        connecting: connection.connecting,
      })
        ? connection
        : c,
    ),
  };
};
export const removeConnectionFromDesign = (kit: Kit, designId: DesignIdLike, connectionId: ConnectionIdLike): Design => {
  const design = findDesignInKit(kit, designId);
  const normalizedConnectionId = connectionIdLikeToConnectionId(connectionId);
  return {
    ...design,
    connections: (design.connections || []).filter(c => !isSameConnection(c, normalizedConnectionId))
  };
};

export const addConnectionsToDesign = (design: Design, connections: Connection[]): Design => ({
  ...design,
  connections: [...(design.connections || []), ...connections],
});
export const setConnectionsInDesign = (design: Design, connections: Connection[]): Design => {
  const connectionsMap = new Map(connections.map((c) => [`${c.connected.piece.id_}:${c.connected.port.id_ || ""}:${c.connecting.piece.id_}:${c.connecting.port.id_ || ""}`, c]));
  return {
    ...design,
    connections: (design.connections || []).map((c) => connectionsMap.get(`${c.connected.piece.id_}:${c.connected.port.id_ || ""}:${c.connecting.piece.id_}:${c.connecting.port.id_ || ""}`) || c),
  };
};
export const removeConnectionsFromDesign = (kit: Kit, designId: DesignIdLike, connectionIds: ConnectionIdLike[]): Design => {
  const design = findDesignInKit(kit, designId);
  const normalizedConnectionIds = connectionIds.map(connectionIdLikeToConnectionId);
  return {
    ...design,
    connections: (design.connections || []).filter(c =>
      !normalizedConnectionIds.some(nci => isSameConnection(c, nci))
    )
  };
};

//#endregion Connection

//#region Design

export const mergeDesigns = (designs: Design[]): Design => {
  const pieces = designs.flatMap((d) => d.pieces ?? []);
  const connections = designs.flatMap((d) => d.connections ?? []);
  return { ...designs[0], pieces, connections };
};

export const orientDesign = (design: Design, plane?: Plane, center?: DiagramPoint): Design => {
  let fixedPieces = design.pieces?.filter(isFixedPiece) ?? [];
  if (plane !== undefined)
    fixedPieces = fixedPieces.map((p) => ({
      ...p,
      plane: matrixToPlane(planeToMatrix(plane).premultiply(planeToMatrix(p.plane!))),
    }));
  if (center !== undefined)
    fixedPieces = fixedPieces.map((p) => ({
      ...p,
      center: { x: p.center!.x + center.x, y: p.center!.y + center.y },
    }));
  return {
    ...design,
    pieces: design.pieces?.map((p) => fixedPieces.find((fp) => fp.id_ === p.id_) ?? p) ?? [],
  };
};

export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[], connectionIds: ConnectionIdLike[]): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const normalizedPieceIds = pieceIds.map(pieceIdLikeToPieceId);
  const normalizedConnectionIds = connectionIds.map(connectionIdLikeToConnectionId);
  const design = findDesignInKit(kit, normalizedDesignId);
  const metadata = piecesMetadata(kit, normalizedDesignId);
  const connectionsToRemove = findConnectionsInDesign(design, normalizedConnectionIds);
  const updatedDesign = {
    ...design,
    pieces: (design.pieces || []).filter((p) => !normalizedPieceIds.some((p2) => p2.id_ === p.id_)),
    connections: (design.connections || []).filter((c) => !normalizedConnectionIds.some((c2) => isSameConnection(c, c2))),
  };
  const staleConnections = findStaleConnectionsInDesign(updatedDesign);
  const removedConnections = [...connectionsToRemove, ...staleConnections];
  const updatedConnections = (design.connections || []).filter((c) => !removedConnections.some((c2) => isSameConnection(c, c2)));
  const updatedPieces: Piece[] = updatedDesign.pieces.map((p) => {
    const pieceMetadata = metadata.get(p.id_)!;
    if (pieceMetadata.parentPieceId) {
      try {
        findConnection(removedConnections, {
          connected: { piece: { id_: pieceMetadata.parentPieceId } },
          connecting: { piece: { id_: p.id_ } },
        });
        return {
          ...p,
          plane: pieceMetadata.plane,
          center: pieceMetadata.center,
        };
      } catch (error) { }
    }
    return p;
  });
  return {
    ...updatedDesign,
    pieces: updatedPieces,
    connections: updatedConnections,
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
export const flattenDesign = (kit: Kit, designId: DesignIdLike): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  const design = findDesignInKit(kit, normalizedDesignId);
  if (!design) {
    throw new Error(`Design ${normalizedDesignId.name} not found in kit ${kit.name}`);
  }
  const types = kit.types ?? [];

  let expandedDesign = expandDesignPieces(design, kit);

  if (!expandedDesign.pieces || expandedDesign.pieces.length === 0) return expandedDesign;

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
        data: { id: piece.id_, label: piece.id_ },
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
  return flatDesign;
};

//#endregion Design

//#region Design Pieces

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
    created: new Date(),
    updated: new Date(),
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
export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds: string[], clusteredDesign: Design, externalConnections: Connection[]): Design => {
  // Remove clustered pieces
  const remainingPieces = (originalDesign.pieces || []).filter((piece) => !clusterPieceIds.includes(piece.id_));

  // Remove all connections involving clustered pieces
  const remainingConnections = (originalDesign.connections || []).filter((connection) => {
    const connectedInCluster = clusterPieceIds.includes(connection.connected.piece.id_);
    const connectingInCluster = clusterPieceIds.includes(connection.connecting.piece.id_);
    return !connectedInCluster && !connectingInCluster;
  });

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
    ...originalDesign,
    pieces: remainingPieces, // No design piece added
    connections: [...remainingConnections, ...updatedExternalConnections],
    updated: new Date(),
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
    for (const neighbor of neighbors) {
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
  for (const designName of designIds) {
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

//#endregion Design Pieces

//#region Type

export const addTypeToKit = (kit: Kit, type: Type): Kit => ({
  ...kit,
  types: [...(kit.types || []), type],
});
export const setTypeInKit = (kit: Kit, type: Type): Kit => ({
  ...kit,
  types: (kit.types || []).map((t) => (isSameType(t, type) ? type : t)),
});
export const removeTypeFromKit = (kit: Kit, typeId: TypeIdLike): Kit => {
  const normalizedTypeId = typeIdLikeToTypeId(typeId);
  return {
    ...kit,
    types: (kit.types || []).filter((t) => !isSameType(t, normalizedTypeId)),
  };
};

//#endregion Type

//#region Design

export const addDesignToKit = (kit: Kit, design: Design): Kit => ({
  ...kit,
  designs: [...(kit.designs || []), design],
});
export const setDesignInKit = (kit: Kit, design: Design): Kit => ({
  ...kit,
  designs: (kit.designs || []).map((d) => (isSameDesign(d, design) ? design : d)),
});
export const removeDesignFromKit = (kit: Kit, designId: DesignIdLike): Kit => {
  const normalizedDesignId = designIdLikeToDesignId(designId);
  return {
    ...kit,
    designs: (kit.designs || []).filter((d) => !isSameDesign(d, normalizedDesignId)),
  };
};

export const updateDesignInKit = (kit: Kit, design: Design): Kit => {
  return {
    ...kit,
    designs: (kit.designs || []).map((d) => (isSameDesign(d, design) ? design : d)),
  };
};

export type IncludedDesignInfo = {
  id: string;
  designId: DesignId;
  type: "connected" | "fixed";
  center?: DiagramPoint;
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
      id: `design-${designIdString}`,
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

//#endregion Design

//#region Kit

//#region DesignDiff

export const addPieceToDesignDiff = (designDiff: any, piece: Piece): any => {
  return {
    ...designDiff,
    pieces: {
      ...designDiff.pieces,
      added: [...(designDiff.pieces?.added || []), piece],
    },
  };
};
export const setPieceInDesignDiff = (designDiff: any, pieceDiff: PieceDiff): any => {
  const existingIndex = (designDiff.pieces?.updated || []).findIndex((p: PieceDiff) => p.id_ === pieceDiff.id_);
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
export const setPiecesInDesignDiff = (designDiff: any, pieceDiffs: PieceDiff[]): any => {
  const updated = [...(designDiff.pieces?.updated || [])];
  pieceDiffs.forEach((pieceDiff: PieceDiff) => {
    const existingIndex = updated.findIndex((p: PieceDiff) => p.id_ === pieceDiff.id_);
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
  const existingIndex = (designDiff.connections?.updated || []).findIndex((c: ConnectionDiff) => isSameConnection(c, connectionDiff));
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
    const existingIndex = updated.findIndex((c: ConnectionDiff) => isSameConnection(c, connectionDiff));
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

export const applyDesignDiff = (base: Design, diff: DesignDiff, inplace: boolean = false): Design => {
  if (inplace) {
    const effectivePieces: Piece[] = base.pieces
      ? base.pieces
        .map((p: Piece) => {
          const pd = diff.pieces?.updated?.find((up: PieceDiff) => up.id_ === p.id_);
          const isRemoved = diff.pieces?.removed?.some((rp: PieceId) => rp.id_ === p.id_);
          const baseWithUpdate = pd ? { ...p, ...pd } : p;
          const diffStatus = isRemoved ? DiffStatus.Removed : pd ? DiffStatus.Modified : DiffStatus.Unchanged;
          return setAttribute(baseWithUpdate, {
            key: "semio.diffStatus",
            value: diffStatus,
          });
        })
        .concat(
          (diff.pieces?.added || []).map((p: Piece) =>
            setAttribute(p, {
              key: "semio.diffStatus",
              value: DiffStatus.Added,
            }),
          ),
        )
      : (diff.pieces?.added || []).map((p: Piece) => setAttribute(p, { key: "semio.diffStatus", value: DiffStatus.Added }));

    const effectiveConnections: Connection[] = base.connections
      ? base.connections
        .map((c: Connection) => {
          const cd = diff.connections?.updated?.find(
            (ud: ConnectionDiff) =>
              ud.connected?.piece?.id_ === c.connected.piece.id_ &&
              ud.connecting?.piece?.id_ === c.connecting.piece.id_ &&
              (ud.connected?.port?.id_ || "") === (c.connected.port?.id_ || "") &&
              (ud.connecting?.port?.id_ || "") === (c.connecting.port?.id_ || ""),
          );
          const isRemoved = diff.connections?.removed?.some((rc: ConnectionId) => rc.connected.piece.id_ === c.connected.piece.id_ && rc.connecting.piece.id_ === c.connecting.piece.id_);
          const baseWithUpdate = cd
            ? {
              ...c,
              ...cd,
              connected: {
                piece: cd.connected?.piece ?? c.connected.piece,
                port: cd.connected?.port ?? c.connected.port,
                designPiece: cd.connected?.designPiece ?? c.connected.designPiece,
              },
              connecting: {
                piece: cd.connecting?.piece ?? c.connecting.piece,
                port: cd.connecting?.port ?? c.connecting.port,
                designPiece: cd.connecting?.designPiece ?? c.connecting.designPiece,
              },
            }
            : c;
          const diffStatus = isRemoved ? DiffStatus.Removed : cd ? DiffStatus.Modified : DiffStatus.Unchanged;
          return setAttribute(baseWithUpdate, {
            key: "semio.diffStatus",
            value: diffStatus,
          });
        })
        .concat(
          (diff.connections?.added || []).map((c: Connection) =>
            setAttribute(c, {
              key: "semio.diffStatus",
              value: DiffStatus.Added,
            }),
          ),
        )
      : (diff.connections?.added || []).map((c: Connection) => setAttribute(c, { key: "semio.diffStatus", value: DiffStatus.Added }));

    return {
      ...base,
      pieces: effectivePieces,
      connections: effectiveConnections,
    };
  } else {
    const effectivePieces: Piece[] = base.pieces
      ? base.pieces
        .map((p: Piece) => {
          const pd = diff.pieces?.updated?.find((up: PieceDiff) => up.id_ === p.id_);
          return pd ? { ...p, ...pd } : p;
        })
        .filter((p: Piece) => !diff.pieces?.removed?.some((rp: PieceId) => rp.id_ === p.id_))
        .concat(diff.pieces?.added || [])
      : diff.pieces?.added || [];

    const effectiveConnections: Connection[] = base.connections
      ? base.connections
        .map((c: Connection) => {
          const cd = diff.connections?.updated?.find(
            (ud: ConnectionDiff) =>
              ud.connected?.piece?.id_ === c.connected.piece.id_ &&
              ud.connecting?.piece?.id_ === c.connecting.piece.id_ &&
              (ud.connected?.port?.id_ || "") === (c.connected.port?.id_ || "") &&
              (ud.connecting?.port?.id_ || "") === (c.connecting.port?.id_ || ""),
          );
          return cd
            ? {
              ...c,
              ...cd,
              connected: {
                piece: cd.connected?.piece ?? c.connected.piece,
                port: cd.connected?.port ?? c.connected.port,
                designPiece: cd.connected?.designPiece ?? c.connected.designPiece,
              },
              connecting: {
                piece: cd.connecting?.piece ?? c.connecting.piece,
                port: cd.connecting?.port ?? c.connecting.port,
                designPiece: cd.connecting?.designPiece ?? c.connecting.designPiece,
              },
            }
            : c;
        })
        .filter((c: Connection) => !diff.connections?.removed?.some((rc: ConnectionId) => rc.connected.piece.id_ === c.connected.piece.id_ && rc.connecting.piece.id_ === c.connecting.piece.id_))
        .concat(diff.connections?.added || [])
      : diff.connections?.added || [];

    return {
      ...base,
      pieces: effectivePieces,
      connections: effectiveConnections,
    };
  }
};

//#endregion DesignDiff

//#endregion CRUDs

//#region Helper Functions

// Helper function to find replaceable designs for design piece
export const findReplacableDesignsForDesignPiece = (kit: Kit, currentDesignId: DesignId, designPiece: Piece): Design[] => {
  if (designPiece.type.name !== "design") return [];

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
    url,
    data: dataUri,
    size,
    hash: hash.toString(36),
    created: new Date(),
    updated: new Date(),
  };
};

export const findFileInKit = (kit: Kit, fileId: FileIdLike): File => {
  const normalizedFileId = fileIdLikeToFileId(fileId);
  const file = (kit.files || []).find(f => f.url === normalizedFileId.url);
  if (!file) throw new Error(`File ${normalizedFileId.url} not found in kit`);
  return file;
};

export const addFileToKit = (kit: Kit, file: File): Kit => ({
  ...kit,
  files: [...(kit.files || []), file],
});

export const setFileInKit = (kit: Kit, file: File): Kit => ({
  ...kit,
  files: (kit.files || []).map(f => f.url === file.url ? file : f),
});

export const removeFileFromKit = (kit: Kit, fileId: FileIdLike): Kit => {
  const normalizedFileId = fileIdLikeToFileId(fileId);
  return {
    ...kit,
    files: (kit.files || []).filter(f => f.url !== normalizedFileId.url),
  };
};

//#endregion Helper Functions

//#endregion
