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

// https://github.com/usalu/semio#-quality-
export const QualitySchema = z.object({
  name: z.string(),
  value: z.string().optional(),
  unit: z.string().optional(),
  definition: z.string().optional(),
});
export const QualityIdSchema = z.object({ name: z.string() });
export const QualityIdLikeSchema = z.union([QualitySchema, QualityIdSchema, z.string()]);

// https://github.com/usalu/semio#-representation-
export const RepresentationSchema = z.object({
  url: z.string(),
  description: z.string().optional(),
  tags: z.array(z.string()).optional(),
  qualities: z.array(QualitySchema).optional(),
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
  qualities: z.array(QualitySchema).optional(),
});
export const PortIdSchema = z.object({ id_: z.string().optional() });
export const PortIdLikeSchema = z.union([PortSchema, PortIdSchema, z.string(), z.null(), z.undefined()]);

// https://github.com/usalu/semio#-author-
export const AuthorSchema = z.object({ name: z.string(), email: z.string() });

// https://github.com/usalu/semio#-location-
export const LocationSchema = z.object({
  longitude: z.number(),
  latitude: z.number(),
});

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
  qualities: z.array(QualitySchema).optional(),
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
  qualities: z.array(QualitySchema).optional(),
});
export const PieceIdSchema = z.object({ id_: z.string() });
export const PieceIdLikeSchema = z.union([PieceSchema, PieceIdSchema, z.string()]);

// https://github.com/usalu/semio#-side-
export const SideSchema = z.object({
  piece: PieceIdSchema,
  port: PortIdSchema,
  designId: z.string().optional(),
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
  qualities: z.array(QualitySchema).optional(),
});
export const ConnectionIdSchema = z.object({
  connected: SideIdSchema,
  connecting: SideIdSchema,
});
export const ConnectionIdLikeSchema = z.union([ConnectionSchema, ConnectionIdSchema, z.tuple([z.string(), z.string()]), z.string()]);

// https://github.com/usalu/semio#-design-
export const DesignSchema: z.ZodType<any> = z.object({
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
  authors: z.array(AuthorSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
});
export const DesignIdSchema = z.object({
  name: z.string(),
  variant: z.string().optional(),
  view: z.string().optional(),
});
export const DesignIdLikeSchema = z.union([DesignSchema, DesignIdSchema, z.tuple([z.string(), z.string().optional(), z.string().optional()]), z.tuple([z.string(), z.string().optional()]), z.string()]);

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
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
});
export const KitIdSchema = z.object({
  name: z.string(),
  version: z.string().optional(),
});
export const KitIdLikeSchema = z.union([KitSchema, KitIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()]);

//#endregion Persistence

//#region Awareness
export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});

export const AuthorDiffSchema = z.object({
  name: z.string().optional(),
  email: z.string().optional(),
});
export const AuthorsDiffSchema = z.object({
  removed: z.array(z.string()).optional(),
  updated: z.array(AuthorDiffSchema).optional(),
  added: z.array(AuthorSchema).optional(),
});
export const LocationDiffSchema = z.object({
  longitude: z.number().optional(),
  latitude: z.number().optional(),
});
export const QualityDiffSchema = z.object({
  name: z.string().optional(),
  value: z.string().optional(),
  unit: z.string().optional(),
  definition: z.string().optional(),
});
export const QualitiesDiffSchema = z.object({
  removed: z.array(QualityIdSchema).optional(),
  updated: z.array(QualityDiffSchema).optional(),
  added: z.array(QualitySchema).optional(),
});
export const RepresentationDiffSchema = z.object({
  url: z.string().optional(),
  description: z.string().optional(),
  tags: z.array(z.string()).optional(),
  qualities: z.array(QualitySchema).optional(),
});
export const RepresentationsDiffSchema = z.object({
  removed: z.array(RepresentationIdSchema).optional(),
  updated: z.array(RepresentationDiffSchema).optional(),
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
  qualities: z.array(QualitySchema).optional(),
});
export const PortsDiffSchema = z.object({
  removed: z.array(PortIdSchema).optional(),
  updated: z.array(PortDiffSchema).optional(),
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
  qualities: z.array(QualitySchema).optional(),
});
export const TypesDiffSchema = z.object({
  removed: z.array(TypeIdSchema).optional(),
  updated: z.array(TypeDiffSchema).optional(),
  added: z.array(TypeSchema).optional(),
});
export const PieceDiffSchema = z.object({
  id_: z.string(),
  name: z.string().optional(),
  description: z.string().optional(),
  type: TypeIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: DiagramPointSchema.optional(),
  qualities: z.array(QualitySchema).optional(),
});
export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(PieceDiffSchema).optional(),
  added: z.array(PieceSchema).optional(),
});
export const SideDiffSchema = z.object({
  piece: PieceIdSchema,
  port: PortIdSchema.optional(),
  designId: z.string().optional(),
});
export const ConnectionDiffSchema = z.object({
  connected: SideDiffSchema,
  connecting: SideDiffSchema,
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
  updated: z.array(ConnectionDiffSchema).optional(),
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
});
export const DesignsDiffSchema = z.object({
  removed: z.array(DesignIdSchema).optional(),
  updated: z.array(DesignDiffSchema).optional(),
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
  types: z.array(TypeDiffSchema).optional(),
  designs: z.array(DesignDiffSchema).optional(),
  qualities: z.array(QualityDiffSchema).optional(),
});

export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

//#endregion Awareness

export type Quality = z.infer<typeof QualitySchema>;
export type QualityId = z.infer<typeof QualityIdSchema>;
export type QualityIdLike = z.infer<typeof QualityIdLikeSchema>;
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
export type Side = z.infer<typeof SideSchema>;
export type SideId = z.infer<typeof SideIdSchema>;
export type SideIdLike = z.infer<typeof SideIdLikeSchema>;
export type Connection = z.infer<typeof ConnectionSchema>;
export type ConnectionId = z.infer<typeof ConnectionIdSchema>;
export type ConnectionIdLike = z.infer<typeof ConnectionIdLikeSchema>;
export type Design = z.infer<typeof DesignSchema>;
export type DesignId = z.infer<typeof DesignIdSchema>;
export type DesignIdLike = z.infer<typeof DesignIdLikeSchema>;
export type Kit = z.infer<typeof KitSchema>;
export type KitId = z.infer<typeof KitIdSchema>;
export type KitIdLike = z.infer<typeof KitIdLikeSchema>;
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;
export type SideDiff = z.infer<typeof SideDiffSchema>;
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
export type KitDiff = z.infer<typeof KitDiffSchema>;
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;
export type QualitiesDiff = z.infer<typeof QualitiesDiffSchema>;
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

// #region Serialization

export const serialize = {
  kit: (value: Kit): string => JSON.stringify(KitSchema.parse(value), null, 2),
  design: (value: Design): string => JSON.stringify(DesignSchema.parse(value), null, 2),
  type: (value: Type): string => JSON.stringify(TypeSchema.parse(value), null, 2),
  piece: (value: Piece): string => JSON.stringify(PieceSchema.parse(value), null, 2),
  connection: (value: Connection): string => JSON.stringify(ConnectionSchema.parse(value), null, 2),
  port: (value: Port): string => JSON.stringify(PortSchema.parse(value), null, 2),
  quality: (value: Quality): string => JSON.stringify(QualitySchema.parse(value), null, 2),
  diagramPoint: (value: DiagramPoint): string => JSON.stringify(DiagramPointSchema.parse(value), null, 2),
  diagramVector: (value: DiagramVector): string => JSON.stringify(DiagramVectorSchema.parse(value), null, 2),
  plane: (value: Plane): string => JSON.stringify(PlaneSchema.parse(value), null, 2),
  point: (value: Point): string => JSON.stringify(PointSchema.parse(value), null, 2),
  vector: (value: Vector): string => JSON.stringify(VectorSchema.parse(value), null, 2),
  author: (value: Author): string => JSON.stringify(AuthorSchema.parse(value), null, 2),
  location: (value: Location): string => JSON.stringify(LocationSchema.parse(value), null, 2),
  representation: (value: Representation): string => JSON.stringify(RepresentationSchema.parse(value), null, 2),
};
export const deserialize = {
  kit: (json: string): Kit => KitSchema.parse(JSON.parse(json)),
  design: (json: string): Design => DesignSchema.parse(JSON.parse(json)),
  type: (json: string): Type => TypeSchema.parse(JSON.parse(json)),
  piece: (json: string): Piece => PieceSchema.parse(JSON.parse(json)),
  connection: (json: string): Connection => ConnectionSchema.parse(JSON.parse(json)),
  port: (json: string): Port => PortSchema.parse(JSON.parse(json)),
  quality: (json: string): Quality => QualitySchema.parse(JSON.parse(json)),
  diagramPoint: (json: string): DiagramPoint => DiagramPointSchema.parse(JSON.parse(json)),
  diagramVector: (json: string): DiagramVector => DiagramVectorSchema.parse(JSON.parse(json)),
  plane: (json: string): Plane => PlaneSchema.parse(JSON.parse(json)),
  point: (json: string): Point => PointSchema.parse(JSON.parse(json)),
  vector: (json: string): Vector => VectorSchema.parse(JSON.parse(json)),
  author: (json: string): Author => AuthorSchema.parse(JSON.parse(json)),
  location: (json: string): Location => LocationSchema.parse(JSON.parse(json)),
  representation: (json: string): Representation => RepresentationSchema.parse(JSON.parse(json)),
};

export const validate = {
  kit: (data: unknown): Kit => KitSchema.parse(data),
  design: (data: unknown): Design => DesignSchema.parse(data),
  type: (data: unknown): Type => TypeSchema.parse(data),
  piece: (data: unknown): Piece => PieceSchema.parse(data),
  connection: (data: unknown): Connection => ConnectionSchema.parse(data),
  port: (data: unknown): Port => PortSchema.parse(data),
  quality: (data: unknown): Quality => QualitySchema.parse(data),
  plane: (data: unknown): Plane => PlaneSchema.parse(data),
  point: (data: unknown): Point => PointSchema.parse(data),
  vector: (data: unknown): Vector => VectorSchema.parse(data),
  author: (data: unknown): Author => AuthorSchema.parse(data),
  location: (data: unknown): Location => LocationSchema.parse(data),
  representation: (data: unknown): Representation => RepresentationSchema.parse(data),
};

export const safeParse = {
  kit: (data: unknown) => KitSchema.safeParse(data),
  design: (data: unknown) => DesignSchema.safeParse(data),
  type: (data: unknown) => TypeSchema.safeParse(data),
  piece: (data: unknown) => PieceSchema.safeParse(data),
  connection: (data: unknown) => ConnectionSchema.safeParse(data),
  port: (data: unknown) => PortSchema.safeParse(data),
  quality: (data: unknown) => QualitySchema.safeParse(data),
  plane: (data: unknown) => PlaneSchema.safeParse(data),
  point: (data: unknown) => PointSchema.safeParse(data),
  vector: (data: unknown) => VectorSchema.safeParse(data),
  author: (data: unknown) => AuthorSchema.safeParse(data),
  location: (data: unknown) => LocationSchema.safeParse(data),
  representation: (data: unknown) => RepresentationSchema.safeParse(data),
};

export const schemas = {
  Kit: KitSchema,
  Design: DesignSchema,
  Type: TypeSchema,
  Piece: PieceSchema,
  Connection: ConnectionSchema,
  Port: PortSchema,
  Quality: QualitySchema,
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
  QualityId: QualityIdSchema,
  RepresentationId: RepresentationIdSchema,
  PieceDiff: PieceDiffSchema,
  PiecesDiff: PiecesDiffSchema,
  SideDiff: SideDiffSchema,
  ConnectionDiff: ConnectionDiffSchema,
  ConnectionsDiff: ConnectionsDiffSchema,
  DesignDiff: DesignDiffSchema,
  DiffStatus: DiffStatusSchema,
};

//#endregion Serialization

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

export const qualityIdLikeToQualityId = (qualityId: QualityIdLike): QualityId => {
  if (typeof qualityId === "string") return { name: qualityId };
  return { name: qualityId.name };
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
  const getConnectedPieceId = (conn: typeof connection) => ("connected" in conn && conn.connected && "piece" in conn.connected ? conn.connected.piece.id_ : "");
  const getConnectingPieceId = (conn: typeof connection) => ("connecting" in conn && conn.connecting && "piece" in conn.connecting ? conn.connecting.piece.id_ : "");

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

export const findQualityValue = (entity: Kit | Type | Design | Piece | Connection | Representation | Port, name: string, defaultValue?: string | null): string | null => {
  const quality = entity.qualities?.find((q) => q.name === name);
  if (!quality && defaultValue === undefined) throw new Error(`Quality ${name} not found in ${entity}`);
  if (quality?.value === undefined && defaultValue === null) return null;
  return quality?.value ?? defaultValue ?? "";
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
  throw new Error("Not implemented");
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
  const fixedPieceIds = flatDesign.pieces?.map((p) => findQualityValue(p, "semio.fixedPieceId") || p.id_);
  const parentPieceIds = flatDesign.pieces?.map((p) => findQualityValue(p, "semio.parentPieceId", null));
  const depths = flatDesign.pieces?.map((p) => parseInt(findQualityValue(p, "semio.depth", "0")!));
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
      const coloredPort = setQuality(port, {
        name: "semio.color",
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

//#region Quality

export const setQuality = <T extends Kit | Design | Type | Piece | Connection | Representation | Port>(entity: T, quality: Quality): T => {
  const qualitiesArray = entity.qualities || [];
  const existingIndex = qualitiesArray.findIndex((q) => q.name === quality.name);
  if (existingIndex >= 0) qualitiesArray[existingIndex] = quality;
  else qualitiesArray.push(quality);
  return { ...entity, qualities: qualitiesArray };
};

export const setQualities = <T extends Kit | Design | Type | Piece | Connection | Representation | Port>(entity: T, qualities: Quality[]): T => {
  return qualities.reduce((acc, quality) => setQuality(acc, quality), entity);
};

//#endregion Quality

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
  throw new Error("Not implemented");
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
  throw new Error("Not implemented");
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
  throw new Error("Not implemented");
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
  throw new Error("Not implemented");
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
      } catch (error) {}
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

  // First, flatten all included designs and expand design pieces
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
    },
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
    const updatedRootPiece = setQualities(rootPiece, [
      { name: "semio.fixedPieceId", value: rootPiece.id_ },
      { name: "semio.depth", value: "0" },
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

        const flatChildPiece: Piece = setQualities(
          {
            ...childPiece,
            plane: childPlane,
            center: childCenter,
          },
          [
            {
              name: "semio.fixedPieceId",
              value: parentPiece.qualities?.find((q) => q.name === "semio.fixedPieceId")?.value ?? "",
            },
            {
              name: "semio.parentPieceId",
              value: parentPiece.id_,
            },
            {
              name: "semio.depth",
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
  // Check if there are any connections with designId (indicating clustered pieces)
  const hasDesignConnections = design.connections?.some((conn) => conn.connected.designId || conn.connecting.designId);
  if (!hasDesignConnections) {
    return design; // No design connections to expand
  }

  let expandedDesign = { ...design };

  // Find all unique designIds referenced in connections
  const designIds = new Set<string>();
  design.connections?.forEach((conn) => {
    if (conn.connected.designId) designIds.add(conn.connected.designId);
    if (conn.connecting.designId) designIds.add(conn.connecting.designId);
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

    // Use original connections without namespacing
    const transformedConnections = expandedReferencedDesign.connections || [];

    // Update external connections that reference this design piece
    const updatedExternalConnections = (expandedDesign.connections || []).map((connection) => {
      if (connection.connected.designId === designName) {
        // Use the original piece ID directly (no namespacing)
        return {
          ...connection,
          connected: {
            ...connection.connected,
            designId: undefined, // Remove designId since we've expanded
          },
        };
      }

      if (connection.connecting.designId === designName) {
        // Use the original piece ID directly (no namespacing)
        return {
          ...connection,
          connecting: {
            ...connection.connecting,
            designId: undefined, // Remove designId since we've expanded
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
            return setQuality(baseWithUpdate, {
              name: "semio.diffStatus",
              value: diffStatus,
            });
          })
          .concat(
            (diff.pieces?.added || []).map((p: Piece) =>
              setQuality(p, {
                name: "semio.diffStatus",
                value: DiffStatus.Added,
              }),
            ),
          )
      : (diff.pieces?.added || []).map((p: Piece) => setQuality(p, { name: "semio.diffStatus", value: DiffStatus.Added }));

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
                    piece: cd.connected.piece,
                    port: cd.connected.port || c.connected.port,
                  },
                  connecting: {
                    piece: cd.connecting.piece,
                    port: cd.connecting.port || c.connecting.port,
                  },
                }
              : c;
            const diffStatus = isRemoved ? DiffStatus.Removed : cd ? DiffStatus.Modified : DiffStatus.Unchanged;
            return setQuality(baseWithUpdate, {
              name: "semio.diffStatus",
              value: diffStatus,
            });
          })
          .concat(
            (diff.connections?.added || []).map((c: Connection) =>
              setQuality(c, {
                name: "semio.diffStatus",
                value: DiffStatus.Added,
              }),
            ),
          )
      : (diff.connections?.added || []).map((c: Connection) => setQuality(c, { name: "semio.diffStatus", value: DiffStatus.Added }));

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
                    piece: cd.connected.piece,
                    port: cd.connected.port || c.connected.port,
                  },
                  connecting: {
                    piece: cd.connecting.piece,
                    port: cd.connecting.port || c.connecting.port,
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

//#endregion
