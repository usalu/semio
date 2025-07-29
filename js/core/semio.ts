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

import cytoscape from 'cytoscape'
import * as THREE from 'three'
import { z } from 'zod'

// #region Constants

export const ICON_WIDTH = 50
export const TOLERANCE = 1e-5

// #endregion Constants

//#region Types

// 📏 A quality is a named value with a unit and a definition.
export const QualitySchema = z.object({
  // 📛 The name of the quality.
  name: z.string(),
  // 🔢 The optional value [ text | url ] of the quality. No value is equivalent to true for the name.
  value: z.string().optional(),
  // Ⓜ️ The optional unit of the value of the quality.
  unit: z.string().optional(),
  // 📖 The optional definition [ text | uri ] of the quality.
  definition: z.string().optional()
})

export const QualityIdSchema = z.object({
  // 📛 The name of the quality.
  name: z.string()
})

export const QualityIdLikeSchema = z.union([QualitySchema, QualityIdSchema, z.string()])

// 💾 A representation is a link to a resource that describes a type for a certain level of detail and tags.
export const RepresentationSchema = z.object({
  // 🔗 The Unique Resource Locator (URL) to the resource of the representation.
  url: z.string(),
  // 💬 The optional human-readable description of the representation.
  description: z.string().optional(),
  // 🏷️ The optional tags to group representations. No tags means default.
  tags: z.array(z.string()).optional(),
  // 📏 The optional qualities of the representation.
  qualities: z.array(QualitySchema).optional()
})

export const RepresentationIdSchema = z.object({
  // 🏷️ The optional tags to group representations. No tags means default.
  tags: z.array(z.string()).optional()
})

export const RepresentationIdLikeSchema = z.union([RepresentationSchema, RepresentationIdSchema, z.array(z.string()), z.string(), z.null(), z.undefined()])

// 📺 A 2d-point (xy) of floats in the diagram. One unit is equal the width of a piece icon.
export const DiagramPointSchema = z.object({
  // 🎚️ The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
  x: z.number(),
  // 🎚️ The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
  y: z.number()
})

// ✖️ A 3-point (xyz) of floating point numbers.
export const PointSchema = z.object({
  // 🎚️ The x-coordinate of the point.
  x: z.number(),
  // 🎚️ The y-coordinate of the point.
  y: z.number(),
  // 🎚️ The z-coordinate of the point.
  z: z.number()
})

// ➡️ A 3d-vector (xyz) of floating point numbers.
export const VectorSchema = z.object({
  // 🎚️ The x-coordinate of the vector.
  x: z.number(),
  // 🎚️ The y-coordinate of the vector.
  y: z.number(),
  // 🎚️ The z-coordinate of the vector.
  z: z.number()
})

// ◳ A plane is an origin (point) and an orientation (x-axis and y-axis).
export const PlaneSchema = z.object({
  // ⌱ The origin of the plane.
  origin: PointSchema,
  // ➡️ The x-axis of the plane.
  xAxis: VectorSchema,
  // ➡️ The y-axis of the plane.
  yAxis: VectorSchema
})

// 🔌 A port is a connection point (with a direction) of a type.
export const PortSchema = z.object({
  // 🆔 The optional local identifier of the port within the type. No id means the default port.
  id_: z.string().optional(),
  // 💬 The optional human-readable description of the port.
  description: z.string().optional(),
  // 👨‍👩‍👧‍👦 The optional family of the port. This allows to define explicit compatibility with other ports.
  family: z.string().optional(),
  // 💯 Whether the port is mandatory. A mandatory port must be connected in a design.
  mandatory: z.boolean().optional(),
  // 💍 The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.
  t: z.number(),
  // ✅ The optional other compatible families of the port. An empty list means this port is compatible with all other ports.
  compatibleFamilies: z.array(z.string()).optional(),
  // ✖️ The connection point of the port that is attracted to another connection point.
  point: PointSchema,
  // ➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.
  direction: VectorSchema,
  // 📏 The optional qualities of the port.
  qualities: z.array(QualitySchema).optional()
})

// 🔌 The optional local identifier of the port within the type. No id means the default port.
export const PortIdSchema = z.object({
  // 🆔 The optional local identifier of the port within the type. No id means the default port.
  id_: z.string().optional()
})

export const PortIdLikeSchema = z.union([PortSchema, PortIdSchema, z.string(), z.null(), z.undefined()])

// 👨‍💻 The information about the author.
export const AuthorSchema = z.object({
  // 📛 The name of the author.
  name: z.string(),
  // 📧 The email of the author.
  email: z.string()
})

// 📍 A location on the earth surface (longitude, latitude).
export const LocationSchema = z.object({
  // ↔️ The longitude of the location in degrees.
  longitude: z.number(),
  // ↕️ The latitude of the location in degrees.
  latitude: z.number()
})

// 🧩 A type is a reusable element that can be connected with other types over ports.
export const TypeSchema = z.object({
  // 📛 The name of the type
  name: z.string(),
  // 💬 The optional human-readable description of the type
  description: z.string().optional(),
  // 🪙 The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.
  icon: z.string().optional(),
  // 🖼️ The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.
  image: z.string().optional(),
  // 🔀 The optional variant of the type. No variant means the default variant.
  variant: z.string().optional(),
  // 📦 The optional number of items in stock. 2147483647 (=2^31-1) means infinite stock.
  stock: z.number().optional(),
  // 👻 Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type.
  virtual: z.boolean().optional(),
  // Ⓜ️ The length unit of the point and the direction of the ports of the type.
  unit: z.string(),
  // 🕒 The creation date of the type
  created: z.string().transform((val) => new Date(val)).or(z.date()).optional(),
  // 🕒 The last update date of the type
  updated: z.string().transform((val) => new Date(val)).or(z.date()).optional(),
  // 📍 The optional location of the type.
  location: LocationSchema.optional(),
  // 💾 The optional representations of the type.
  representations: z.array(RepresentationSchema).optional(),
  // 🔌 The optional ports of the type.
  ports: z.array(PortSchema).optional(),
  // 👨‍💻 The optional authors of the type.
  authors: z.array(AuthorSchema).optional(),
  // 📏 The optional qualities of the type.
  qualities: z.array(QualitySchema).optional()
})

// 🧩 identifier of the type within the kit.
export const TypeIdSchema = z.object({
  // 📛 The name of the type.
  name: z.string(),
  // 🔀 The optional variant of the type. No variant means the default variant.
  variant: z.string().optional()
})

export const TypeIdLikeSchema = z.union([TypeSchema, TypeIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()])

// ⭕ A piece is a 3d-instance of a type in a design.
export const PieceSchema = z.object({
  // 🆔 The optional local identifier of the piece within the design. No id means the default piece.
  id_: z.string(),
  // 💬 The optional human-readable description of the piece.
  description: z.string().optional(),
  // 🧩 The local identifier of the type of the piece within the kit.
  type: TypeIdSchema,
  // ◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.
  plane: PlaneSchema.optional(),
  // ⌖ The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.
  center: DiagramPointSchema.optional(),
  // 📏 The optional qualities of the piece.
  qualities: z.array(QualitySchema).optional()
})

// ⭕ The optional local identifier of the piece within the design. No id means the default piece.
export const PieceIdSchema = z.object({
  // 🆔 The optional local identifier of the piece within the design. No id means the default piece.
  id_: z.string()
})

export const PieceIdLikeSchema = z.union([PieceSchema, PieceIdSchema, z.string()])

// 🧱 A side of a piece in a connection.
export const SideSchema = z.object({
  // ⭕ The piece-related information of the side.
  piece: PieceIdSchema,
  // 🔌 The local identifier of the port within the type.
  port: PortIdSchema
})

// 🧱 Identifier for a side within a connection.
export const SideIdSchema = z.object({
  // ⭕ The piece-related information of the side.
  piece: PieceIdSchema
})

export const SideIdLikeSchema = z.union([SideSchema, SideIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()])

// 🔗 A bidirectional connection between two pieces of a design.
export const ConnectionSchema = z.object({
  // 🧲 The connected side of the piece of the connection.
  connected: SideSchema,
  // 🧲 The connected side of the piece of the connection.
  connecting: SideSchema,
  // 💬 The optional human-readable description of the connection.
  description: z.string().optional(),
  // ↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece.
  gap: z.number().optional(),
  // ↔️ The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.
  shift: z.number().optional(),
  // 🪜 The optional vertical rise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.
  rise: z.number().optional(),
  // 🔄 The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.
  rotation: z.number().optional(),
  // 🛞 The optional turn perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.
  turn: z.number().optional(),
  // ∡ The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.
  tilt: z.number().optional(),
  // ➡️ The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.
  x: z.number().optional(),
  // ⬆️ The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.
  y: z.number().optional(),
  // 📏 The optional qualities of the connection.
  qualities: z.array(QualitySchema).optional()
})

// 🔗 Identifier for a connection within a design.
export const ConnectionIdSchema = z.object({
  // 🧲 The connected side of the piece of the connection.
  connected: SideIdSchema,
  // 🧲 The connected side of the piece of the connection.
  connecting: SideIdSchema
})

export const ConnectionIdLikeSchema = z.union([ConnectionSchema, ConnectionIdSchema, z.tuple([z.string(), z.string()]), z.string()])

// 🏙️ A design is a collection of pieces that are connected.
export const DesignSchema = z.object({
  // 📛 The name of the design.
  name: z.string(),
  // 💬 The optional human-readable description of the design.
  description: z.string().optional(),
  // 🪙 The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.
  icon: z.string().optional(),
  // 🖼️ The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.
  image: z.string().optional(),
  // 🔀 The optional variant of the design. No variant means the default variant.
  variant: z.string().optional(),
  // 🥽 The optional view of the design. No view means the default view.
  view: z.string().optional(),
  // 📍 The optional location of the design.
  location: LocationSchema.optional(),
  // Ⓜ️ The length unit for all distance-related information of the design.
  unit: z.string(),
  // 🕒 The creation date of the design
  created: z.string().transform((val) => new Date(val)).or(z.date()).optional(),
  // 🕒 The last update date of the design
  updated: z.string().transform((val) => new Date(val)).or(z.date()).optional(),
  // ⭕ The optional pieces of the design.
  pieces: z.array(PieceSchema).optional(),
  // 🔗 The optional connections of the design.
  connections: z.array(ConnectionSchema).optional(),
  // 👨‍💻 The optional authors of the design.
  authors: z.array(AuthorSchema).optional(),
  // 📏 The optional qualities of the design.
  qualities: z.array(QualitySchema).optional()
})

// 🏙️ The local identifier of the design within the kit.
export const DesignIdSchema = z.object({
  // 📛 The name of the design.
  name: z.string(),
  // 🔀 The optional variant of the design. No variant means the default variant.
  variant: z.string().optional(),
  // 🥽 The optional view of the design. No view means the default view.
  view: z.string().optional()
})

export const DesignIdLikeSchema = z.union([DesignSchema, DesignIdSchema, z.tuple([z.string(), z.string().optional(), z.string().optional()]), z.tuple([z.string(), z.string().optional()]), z.string()])

// 🗃️ A kit is a collection of types and designs.
export const KitSchema = z.object({
  // 📛 The name of the kit.
  name: z.string(),
  // 💬 The optional human-readable description of the kit.
  description: z.string().optional(),
  // 🪙 The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.
  icon: z.string().optional(),
  // 🖼️ The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.
  image: z.string().optional(),
  // 🔮 The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB.
  preview: z.string().optional(),
  // 🔀 The optional version of the kit. No version means the latest version.
  version: z.string().optional(),
  // ☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely.
  remote: z.string().optional(),
  // 🏠 The optional Unique Resource Locator (URL) of the homepage of the kit.
  homepage: z.string().optional(),
  // ⚖️ The optional license [ spdx id | url ] of the kit.
  license: z.string().optional(),
  // 🕒 The creation date of the kit
  created: z.string().transform((val) => new Date(val)).or(z.date()).optional(),
  // 🕒 The last update date of the kit
  updated: z.string().transform((val) => new Date(val)).or(z.date()).optional(),
  // 🧩 The optional types of the kit.
  types: z.array(TypeSchema).optional(),
  // 🏙️ The optional designs of the kit.
  designs: z.array(DesignSchema).optional(),
  // 📏 The optional qualities of the kit.
  qualities: z.array(QualitySchema).optional()
})

// 🗃️ Identifier for a kit.
export const KitIdSchema = z.object({
  // 📛 The name of the kit.
  name: z.string(),
  // 🔀 The optional version of the kit. No version means the latest version.
  version: z.string().optional()
})

export const KitIdLikeSchema = z.union([KitSchema, KitIdSchema, z.tuple([z.string(), z.string().optional()]), z.string()])

export const PieceDiffSchema = z.object({
  id_: z.string(),
  name: z.string().optional(),
  description: z.string().optional(),
  type: TypeIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: DiagramPointSchema.optional(),
  qualities: z.array(QualitySchema).optional()
})

export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(PieceDiffSchema).optional(),
  added: z.array(PieceSchema).optional()
})

export const SideDiffSchema = z.object({
  piece: PieceIdSchema,
  port: PortIdSchema.optional()
})

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
  y: z.number().optional()
})

export const ConnectionsDiffSchema = z.object({
  removed: z.array(ConnectionIdSchema).optional(),
  updated: z.array(ConnectionDiffSchema).optional(),
  added: z.array(ConnectionSchema).optional()
})

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
  connections: ConnectionsDiffSchema.optional()
})

export const DiffStatusSchema = z.enum(['unchanged', 'added', 'removed', 'modified'])

// Export TypeScript types inferred from Zod schemas
export type Quality = z.infer<typeof QualitySchema>
export type QualityId = z.infer<typeof QualityIdSchema>
export type QualityIdLike = z.infer<typeof QualityIdLikeSchema>
export type Representation = z.infer<typeof RepresentationSchema>
export type RepresentationId = z.infer<typeof RepresentationIdSchema>
export type RepresentationIdLike = z.infer<typeof RepresentationIdLikeSchema>
export type DiagramPoint = z.infer<typeof DiagramPointSchema>
export type Point = z.infer<typeof PointSchema>
export type Vector = z.infer<typeof VectorSchema>
export type Plane = z.infer<typeof PlaneSchema>
export type Port = z.infer<typeof PortSchema>
export type PortId = z.infer<typeof PortIdSchema>
export type PortIdLike = z.infer<typeof PortIdLikeSchema>
export type Author = z.infer<typeof AuthorSchema>
export type Location = z.infer<typeof LocationSchema>
export type Type = z.infer<typeof TypeSchema>
export type TypeId = z.infer<typeof TypeIdSchema>
export type TypeIdLike = z.infer<typeof TypeIdLikeSchema>
export type Piece = z.infer<typeof PieceSchema>
export type PieceId = z.infer<typeof PieceIdSchema>
export type PieceIdLike = z.infer<typeof PieceIdLikeSchema>
export type Side = z.infer<typeof SideSchema>
export type SideId = z.infer<typeof SideIdSchema>
export type SideIdLike = z.infer<typeof SideIdLikeSchema>
export type Connection = z.infer<typeof ConnectionSchema>
export type ConnectionId = z.infer<typeof ConnectionIdSchema>
export type ConnectionIdLike = z.infer<typeof ConnectionIdLikeSchema>
export type Design = z.infer<typeof DesignSchema>
export type DesignId = z.infer<typeof DesignIdSchema>
export type DesignIdLike = z.infer<typeof DesignIdLikeSchema>
export type Kit = z.infer<typeof KitSchema>
export type KitId = z.infer<typeof KitIdSchema>
export type KitIdLike = z.infer<typeof KitIdLikeSchema>
export type PieceDiff = z.infer<typeof PieceDiffSchema>
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>
export type SideDiff = z.infer<typeof SideDiffSchema>
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>
export type DesignDiff = z.infer<typeof DesignDiffSchema>

export enum DiffStatus {
  Unchanged = 'unchanged',
  Added = 'added',
  Removed = 'removed',
  Modified = 'modified'
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
  plane: (value: Plane): string => JSON.stringify(PlaneSchema.parse(value), null, 2),
  point: (value: Point): string => JSON.stringify(PointSchema.parse(value), null, 2),
  vector: (value: Vector): string => JSON.stringify(VectorSchema.parse(value), null, 2)
}
export const deserialize = {
  kit: (json: string): Kit => KitSchema.parse(JSON.parse(json)),
  design: (json: string): Design => DesignSchema.parse(JSON.parse(json)),
  type: (json: string): Type => TypeSchema.parse(JSON.parse(json)),
  piece: (json: string): Piece => PieceSchema.parse(JSON.parse(json)),
  connection: (json: string): Connection => ConnectionSchema.parse(JSON.parse(json)),
  port: (json: string): Port => PortSchema.parse(JSON.parse(json)),
  quality: (json: string): Quality => QualitySchema.parse(JSON.parse(json)),
  plane: (json: string): Plane => PlaneSchema.parse(JSON.parse(json)),
  point: (json: string): Point => PointSchema.parse(JSON.parse(json)),
  vector: (json: string): Vector => VectorSchema.parse(JSON.parse(json))
}

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
  vector: (data: unknown): Vector => VectorSchema.parse(data)
}

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
  vector: (data: unknown) => VectorSchema.safeParse(data)
}

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
  DiffStatus: DiffStatusSchema
}

//#endregion Serialization

//#region Functions

const normalize = (val: string | undefined | null): string => ((val === undefined || val === null) ? '' : val)
const round = (value: number): number => Math.round(value / TOLERANCE) * TOLERANCE
export const jaccard = (a: string[] | undefined, b: string[] | undefined) => {
  if ((a === undefined && b === undefined) || (a?.length === 0 && b?.length === 0)) return 1
  const setA = new Set(a)
  const setB = new Set(b)
  const intersection = [...setA].filter((x) => setB.has(x)).length
  const union = setA.size + setB.size - intersection
  if (union === 0) return 0
  return intersection / union
}

//#region Mapping

export const qualityIdLikeToQualityId = (qualityId: QualityIdLike): QualityId => {
  if (typeof qualityId === 'string') return { name: qualityId }
  return { name: qualityId.name }
}

export const representationIdLikeToRepresentationId = (representationId: RepresentationIdLike): RepresentationId => {
  if (representationId === undefined || representationId === null) return { tags: [] }
  if (typeof representationId === 'string') return { tags: [representationId] }
  if (Array.isArray(representationId)) return { tags: representationId }
  return { tags: representationId.tags ?? [] }
}

export const portIdLikeToPortId = (portId: PortIdLike): PortId => {
  if (portId === undefined || portId === null) return { id_: '' }
  if (typeof portId === 'string') return { id_: portId }
  return { id_: portId.id_ }
}

export const typeIdLikeToTypeId = (typeId: TypeIdLike): TypeId => {
  if (typeof typeId === 'string') return { name: typeId }
  if (Array.isArray(typeId)) return { name: typeId[0], variant: typeId[1] ?? undefined }
  return { name: typeId.name, variant: typeId.variant ?? undefined }
}

export const pieceIdLikeToPieceId = (pieceId: PieceIdLike): PieceId => {
  if (typeof pieceId === 'string') return { id_: pieceId }
  return { id_: pieceId.id_ }
}

export const connectionIdLikeToConnectionId = (connectionId: ConnectionIdLike): ConnectionId => {
  if (typeof connectionId === 'string') {
    const [connectedPieceId, connectingPieceId] = connectionId.split('--')
    return { connected: { piece: { id_: connectedPieceId } }, connecting: { piece: { id_: connectingPieceId } } }
  }
  if (Array.isArray(connectionId)) {
    const [connectedPieceId, connectingPieceId] = connectionId
    return { connected: { piece: { id_: connectedPieceId } }, connecting: { piece: { id_: connectingPieceId } } }
  }
  return { connected: { piece: { id_: connectionId.connected.piece.id_ } }, connecting: { piece: { id_: connectionId.connecting.piece.id_ } } }
}

export const designIdLikeToDesignId = (designId: DesignIdLike): DesignId => {
  if (typeof designId === 'string') return { name: designId }
  if (Array.isArray(designId)) return { name: designId[0], variant: designId[1] ?? undefined, view: designId[2] ?? undefined }
  return { name: designId.name, variant: designId.variant ?? undefined, view: designId.view ?? undefined }
}

export const kitIdLikeToKitId = (kitId: KitIdLike): KitId => {
  if (typeof kitId === 'string') return { name: kitId }
  if (Array.isArray(kitId)) return { name: kitId[0], version: kitId[1] ?? undefined }
  return { name: kitId.name, version: kitId.version ?? undefined }
}

//#endregion Mapping

//#region CRUDs

//#region Design

export const updateDesignInKit = (kit: Kit, design: Design): Kit => {
  return { ...kit, designs: (kit.designs || []).map(d => isSameDesign(d, design) ? design : d) }
}

export const addPieceToDesign = (design: Design, piece: Piece): Design => ({ ...design, pieces: [...(design.pieces || []), piece] })
export const setPieceInDesign = (design: Design, piece: Piece): Design => ({ ...design, pieces: (design.pieces || []).map(p => p.id_ === piece.id_ ? piece : p) })
export const removePieceFromDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Design => {
  throw new Error("Not implemented");
}

export const addPiecesToDesign = (design: Design, pieces: Piece[]): Design => ({ ...design, pieces: [...(design.pieces || []), ...pieces] })
export const setPiecesInDesign = (design: Design, pieces: Piece[]): Design => ({ ...design, pieces: (design.pieces || []).map(p => pieces.find(p2 => p2.id_ === p.id_) || p) })
export const removePiecesFromDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[]): Design => {
  throw new Error("Not implemented");
}

export const addConnectionToDesign = (design: Design, connection: Connection): Design => ({ ...design, connections: [...(design.connections || []), connection] })
export const setConnectionInDesign = (design: Design, connection: Connection): Design => {
  return ({ ...design, connections: (design.connections || []).map(c => isSameConnection(c, { connected: connection.connected, connecting: connection.connecting }) ? connection : c) })
}
export const removeConnectionFromDesign = (kit: Kit, designId: DesignIdLike, connectionId: ConnectionIdLike): Design => {
  throw new Error("Not implemented");
}


export const addConnectionsToDesign = (design: Design, connections: Connection[]): Design => ({ ...design, connections: [...(design.connections || []), ...connections] })
export const setConnectionsInDesign = (design: Design, connections: Connection[]): Design => {
  const connectionsMap = new Map(connections.map(c => [`${c.connected.piece.id_}:${c.connected.port.id_ || ''}:${c.connecting.piece.id_}:${c.connecting.port.id_ || ''}`, c]))
  return ({ ...design, connections: (design.connections || []).map(c => connectionsMap.get(`${c.connected.piece.id_}:${c.connected.port.id_ || ''}:${c.connecting.piece.id_}:${c.connecting.port.id_ || ''}`) || c) })
}
export const removeConnectionsFromDesign = (kit: Kit, designId: DesignIdLike, connectionIds: ConnectionIdLike[]): Design => {
  throw new Error("Not implemented");
}

export const removePiecesAndConnectionsFromDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[], connectionIds: ConnectionIdLike[]): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId)
  const normalizedPieceIds = pieceIds.map(pieceIdLikeToPieceId)
  const normalizedConnectionIds = connectionIds.map(connectionIdLikeToConnectionId)
  const design = findDesignInKit(kit, normalizedDesignId)
  const metadata = piecesMetadata(kit, normalizedDesignId)
  const connectionsToRemove = findConnectionsInDesign(design, normalizedConnectionIds)
  const updatedDesign = { ...design, pieces: (design.pieces || []).filter(p => !normalizedPieceIds.some(p2 => p2.id_ === p.id_)), connections: (design.connections || []).filter(c => !normalizedConnectionIds.some(c2 => isSameConnection(c, c2))) }
  const staleConnections = findStaleConnectionsInDesign(updatedDesign)
  const removedConnections = [...connectionsToRemove, ...staleConnections]
  const updatedConnections = (design.connections || []).filter(c => !removedConnections.some(c2 => isSameConnection(c, c2)))
  const updatedPieces: Piece[] = updatedDesign.pieces.map(p => {
    const pieceMetadata = metadata.get(p.id_)!
    if (pieceMetadata.parentPieceId) {
      try {
        findConnection(removedConnections, { connected: { piece: { id_: pieceMetadata.parentPieceId } }, connecting: { piece: { id_: p.id_ } } })
        return { ...p, plane: pieceMetadata.plane, center: pieceMetadata.center }
      } catch (error) { }
    }
    return p
  })
  return { ...updatedDesign, pieces: updatedPieces, connections: updatedConnections }
}
//#endregion Design

//#region DesignDiff
export const addPieceToDesignDiff = (designDiff: any, piece: Piece): any => {
  return { ...designDiff, pieces: { ...designDiff.pieces, added: [...(designDiff.pieces?.added || []), piece] } }
}
export const setPieceInDesignDiff = (designDiff: any, pieceDiff: PieceDiff): any => {
  const existingIndex = (designDiff.pieces?.updated || []).findIndex((p: PieceDiff) => p.id_ === pieceDiff.id_)
  const updated = [...(designDiff.pieces?.updated || [])]
  if (existingIndex >= 0) {
    updated[existingIndex] = pieceDiff
  } else {
    updated.push(pieceDiff)
  }
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } }
}
export const removePieceFromDesignDiff = (designDiff: any, pieceId: PieceId): any => {
  return { ...designDiff, pieces: { ...designDiff.pieces, removed: [...(designDiff.pieces?.removed || []), pieceId] } }
}

export const addPiecesToDesignDiff = (designDiff: any, pieces: Piece[]): any => {
  return { ...designDiff, pieces: { ...designDiff.pieces, added: [...(designDiff.pieces?.added || []), ...pieces] } }
}
export const setPiecesInDesignDiff = (designDiff: any, pieceDiffs: PieceDiff[]): any => {
  const updated = [...(designDiff.pieces?.updated || [])]
  pieceDiffs.forEach((pieceDiff: PieceDiff) => {
    const existingIndex = updated.findIndex((p: PieceDiff) => p.id_ === pieceDiff.id_)
    if (existingIndex >= 0) {
      updated[existingIndex] = pieceDiff
    } else {
      updated.push(pieceDiff)
    }
  })
  return { ...designDiff, pieces: { ...designDiff.pieces, updated } }
}
export const removePiecesFromDesignDiff = (designDiff: any, pieceIds: PieceId[]): any => {
  return { ...designDiff, pieces: { ...designDiff.pieces, removed: [...(designDiff.pieces?.removed || []), ...pieceIds] } }
}


export const addConnectionToDesignDiff = (designDiff: any, connection: Connection): any => {
  return { ...designDiff, connections: { ...designDiff.connections, added: [...(designDiff.connections?.added || []), connection] } }
}
export const setConnectionInDesignDiff = (designDiff: any, connectionDiff: ConnectionDiff): any => {
  const existingIndex = (designDiff.connections?.updated || []).findIndex((c: ConnectionDiff) => isSameConnection(c, connectionDiff))
  const updated = [...(designDiff.connections?.updated || [])]
  if (existingIndex >= 0) {
    updated[existingIndex] = connectionDiff
  } else {
    updated.push(connectionDiff)
  }
  return ({ ...designDiff, connections: { ...designDiff.connections, updated } })
}
export const removeConnectionFromDesignDiff = (designDiff: any, connectionId: ConnectionId): any => {
  return { ...designDiff, connections: { ...designDiff.connections, removed: [...(designDiff.connections?.removed || []), connectionId] } }
}

export const addConnectionsToDesignDiff = (designDiff: any, connections: Connection[]): any => {
  return { ...designDiff, connections: { ...designDiff.connections, added: [...(designDiff.connections?.added || []), ...connections] } }
}
export const setConnectionsInDesignDiff = (designDiff: any, connectionDiffs: ConnectionDiff[]): any => {
  const updated = [...(designDiff.connections?.updated || [])]
  connectionDiffs.forEach((connectionDiff: ConnectionDiff) => {
    const existingIndex = updated.findIndex((c: ConnectionDiff) => isSameConnection(c, connectionDiff))
    if (existingIndex >= 0) {
      updated[existingIndex] = connectionDiff
    } else {
      updated.push(connectionDiff)
    }
  })
  return ({ ...designDiff, connections: { ...designDiff.connections, updated } })
}
export const removeConnectionsFromDesignDiff = (designDiff: any, connectionIds: ConnectionId[]): any => {
  return { ...designDiff, connections: { ...designDiff.connections, removed: [...(designDiff.connections?.removed || []), ...connectionIds] } }
}
//#endregion DesignDiff

//#endregion CRUDs

//#region Querying

//#region Propositional

export const arePortsCompatible = (port: Port, otherPort: Port): boolean => {
  if ((normalize(port.family) === '' || normalize(otherPort.family) === '')) return true
  return (port.compatibleFamilies ?? []).includes(normalize(otherPort.family)) || (otherPort.compatibleFamilies ?? []).includes(normalize(port.family)) || false
}

export const isPortInUse = (design: Design, piece: Piece | PieceId, port: Port | PortId): boolean => {
  const normalizedPieceId = pieceIdLikeToPieceId(piece)
  const normalizedPortId = portIdLikeToPortId(port)
  const connections = findPieceConnectionsInDesign(design, piece)
  for (const connection of connections) {
    const isPieceConnected = connection.connected.piece.id_ === normalizedPieceId.id_
    const isPortConnected = isPieceConnected ? connection.connected.port.id_ === normalizedPortId.id_ : connection.connecting.port.id_ === normalizedPortId.id_
    if (isPortConnected) return true
  }
  return false
}

export const isConnectionInDesign = (design: Design, connection: Connection | ConnectionId): boolean => {
  return design.connections?.some((c) => isSameConnection(c, connection)) ?? false
}

export const isFixedPiece = (piece: Piece): boolean => {
  const isPlaneSet = piece.plane !== undefined
  const isCenterSet = piece.center !== undefined
  if (isPlaneSet !== isCenterSet) throw new Error(`Piece ${piece.id_} has inconsistent plane and center`)
  return isPlaneSet
}

export const isSameRepresentation = (representation: Representation, other: Representation): boolean => {
  return representation.tags?.every(tag => other.tags?.includes(tag)) ?? true
}
export const isSamePort = (port: Port | PortId, other: Port | PortId): boolean => {
  const p1 = portIdLikeToPortId(port)
  const p2 = portIdLikeToPortId(other)
  return normalize(p1.id_) === normalize(p2.id_)
}
export const isSameType = (type: Type | TypeId, other: Type | TypeId): boolean => {
  const t1 = typeIdLikeToTypeId(type)
  const t2 = typeIdLikeToTypeId(other)
  return t1.name === t2.name && normalize(t1.variant) === normalize(t2.variant)
}
export const isSamePiece = (piece: Piece | PieceId, other: Piece | PieceId): boolean => {
  const p1 = pieceIdLikeToPieceId(piece)
  const p2 = pieceIdLikeToPieceId(other)
  return normalize(p1.id_) === normalize(p2.id_)
}
export const isSameConnection = (connection: Connection | ConnectionId | ConnectionDiff, other: Connection | ConnectionId | ConnectionDiff, strict: boolean = false): boolean => {
  const getConnectedPieceId = (conn: typeof connection) =>
    'connected' in conn && conn.connected && 'piece' in conn.connected ? conn.connected.piece.id_ : ''
  const getConnectingPieceId = (conn: typeof connection) =>
    'connecting' in conn && conn.connecting && 'piece' in conn.connecting ? conn.connecting.piece.id_ : ''

  const connectedPiece1 = getConnectedPieceId(connection)
  const connectingPiece1 = getConnectingPieceId(connection)
  const connectedPiece2 = getConnectedPieceId(other)
  const connectingPiece2 = getConnectingPieceId(other)

  const isExactMatch = connectingPiece1 === connectingPiece2 && connectedPiece1 === connectedPiece2
  if (strict) return isExactMatch
  const isSwappedMatch = connectingPiece1 === connectedPiece2 && connectedPiece1 === connectingPiece2
  return isExactMatch || isSwappedMatch
}
export const isSameDesign = (design: DesignIdLike, other: DesignIdLike): boolean => {
  const d1 = designIdLikeToDesignId(design)
  const d2 = designIdLikeToDesignId(other)
  return d1.name === d2.name && normalize(d1.variant) === normalize(d2.variant) && normalize(d1.view) === normalize(d2.view)
}
export const isSameKit = (kit: Kit | KitId, other: Kit | KitId): boolean => {
  return kit.name === other.name && normalize(kit.version) === normalize(other.version)
}

//#endregion Propositional

//#region Predicates

export const findQualityValue = (entity: Kit | Type | Design | Piece | Connection | Representation | Port, name: string, defaultValue?: string | null): string | null => {
  const quality = entity.qualities?.find((q) => q.name === name)
  if (!quality && defaultValue === undefined) throw new Error(`Quality ${name} not found in ${entity}`)
  if (quality?.value === undefined && defaultValue === null) return null
  return quality?.value ?? defaultValue ?? ''
}
export const findPort = (ports: Port[], portId: PortIdLike): Port => {
  const normalizedPortId = portIdLikeToPortId(portId)
  const port = ports.find((p) => normalize(p.id_) === normalize(normalizedPortId.id_))
  if (!port) throw new Error(`Port ${normalizedPortId.id_} not found in ports`)
  return port
}
export const findPortInType = (type: Type, portId: PortIdLike): Port => findPort(type.ports ?? [], portId)
export const findPiece = (pieces: Piece[], pieceId: PieceIdLike): Piece => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId)
  const piece = pieces.find((p) => p.id_ === normalizedPieceId.id_)
  if (!piece) throw new Error(`Piece ${normalizedPieceId.id_} not found in pieces`)
  return piece
}
export const findPortForPieceInConnection = (type: Type, connection: Connection, pieceId: PieceIdLike): Port => {
  const portId = connection.connected.piece.id_ === pieceId ? connection.connected.port.id_ : connection.connecting.port.id_
  return findPortInType(type, portId)
}
export const findPieceInDesign = (design: Design, pieceId: PieceIdLike): Piece => findPiece(design.pieces ?? [], pieceId)
export const findConnection = (connections: Connection[], connectionId: ConnectionIdLike, strict: boolean = false): Connection => {
  const normalizedConnectionId = connectionIdLikeToConnectionId(connectionId)
  const connection = connections.find((c) => isSameConnection(c, normalizedConnectionId, strict))
  if (!connection) throw new Error(`Connection ${normalizedConnectionId.connected.piece.id_} -> ${normalizedConnectionId.connecting.piece.id_} not found in connections`)
  return connection
}
export const findConnectionInDesign = (design: Design, connectionId: ConnectionIdLike, strict: boolean = false): Connection => {
  return findConnection(design.connections ?? [], connectionId, strict)
}
export const findConnectionsInDesign = (design: Design, connectionIds: ConnectionIdLike[]): Connection[] => {
  return connectionIds.map((connectionId) => findConnectionInDesign(design, connectionId))
}
export const findPieceConnections = (connections: Connection[], pieceId: PieceIdLike): Connection[] => {
  const normalizedPieceId = pieceIdLikeToPieceId(pieceId)
  return connections.filter((c) => c.connected.piece.id_ === normalizedPieceId.id_ || c.connecting.piece.id_ === normalizedPieceId.id_)
}
export const findPieceConnectionsInDesign = (design: Design, pieceId: PieceIdLike): Connection[] => {
  return findPieceConnections(design.connections ?? [], pieceId)
}
export const findConnectionPiecesInDesign = (design: Design, connection: Connection | ConnectionId): { connecting: Piece, connected: Piece } => {
  return { connected: findPieceInDesign(design, connection.connected.piece), connecting: findPieceInDesign(design, connection.connecting.piece) }
}
export const findStaleConnectionsInDesign = (design: Design): Connection[] => {
  return design.connections?.filter(c => {
    try {
      findPieceInDesign(design, c.connected.piece)
      findPieceInDesign(design, c.connecting.piece)
      return false
    } catch (e) {
      return true
    }
  }) ?? []
}
export const findTypeInKit = (kit: Kit, typeId: TypeIdLike): Type => {
  const normalizedTypeId = typeIdLikeToTypeId(typeId)
  const type = kit.types?.find(
    (t) => t.name === normalizedTypeId.name && normalize(t.variant) === normalize(normalizedTypeId.variant)
  )
  if (!type) throw new Error(`Type ${normalizedTypeId.name} not found in kit ${kit.name}`)
  return type
}
export const findDesignInKit = (kit: Kit, designId: DesignIdLike): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId)
  const design = kit.designs?.find(
    (d) =>
      d.name === normalizedDesignId.name &&
      normalize(d.variant) === normalize(normalizedDesignId.variant) &&
      normalize(d.view) === normalize(normalizedDesignId.view)
  )
  if (!design) throw new Error(`Design ${normalizedDesignId.name} not found in kit ${kit.name}`)
  return design
}
export const findUsedPortsByPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike): Port[] => {
  const design = findDesignInKit(kit, designId)
  const piece = findPieceInDesign(design, pieceId)
  const type = findTypeInKit(kit, piece.type)
  const connections = findPieceConnectionsInDesign(design, pieceId)
  return connections.map(c => findPortForPieceInConnection(type, c, pieceId))
}
export const findReplacableTypesForPieceInDesign = (kit: Kit, designId: DesignIdLike, pieceId: PieceIdLike, variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designId)
  const connections = findPieceConnectionsInDesign(design, pieceId)
  const requiredPorts: Port[] = []
  for (const connection of connections) {
    try {
      const otherPieceId = connection.connected.piece.id_ === pieceId ? connection.connecting.piece.id_ : connection.connected.piece.id_
      const otherPiece = findPieceInDesign(design, otherPieceId)
      const otherType = findTypeInKit(kit, otherPiece.type)
      const otherPortId = connection.connected.piece.id_ === pieceId ? connection.connecting.port.id_ : connection.connected.port.id_
      const otherPort = findPortInType(otherType, otherPortId || '')
      requiredPorts.push(otherPort)
    } catch (error) { continue }
  }
  return kit.types?.filter(replacementType => {
    if (variants !== undefined && !variants.includes(replacementType.variant ?? '')) return false
    if (!replacementType.ports || replacementType.ports.length === 0) return requiredPorts.length === 0
    return requiredPorts.every(requiredPort => {
      return replacementType.ports!.some(replacementPort => arePortsCompatible(replacementPort, requiredPort))
    })
  }) ?? []
}
export const findReplacableTypesForPiecesInDesign = (kit: Kit, designId: DesignIdLike, pieceIds: PieceIdLike[], variants?: string[]): Type[] => {
  const design = findDesignInKit(kit, designId)
  const normalizedPieceIds = pieceIds.map(id => typeof id === 'string' ? id : id.id_)
  const pieces = pieceIds.map(id => findPieceInDesign(design, id))
  const externalConnections: Array<{ connection: Connection, requiredPort: Port }> = []
  for (const piece of pieces) {
    const connections = findPieceConnectionsInDesign(design, piece.id_)
    for (const connection of connections) {
      const otherPieceId = connection.connected.piece.id_ === piece.id_ ? connection.connecting.piece.id_ : connection.connected.piece.id_
      if (!normalizedPieceIds.includes(otherPieceId)) {
        try {
          const otherPiece = findPieceInDesign(design, otherPieceId)
          const otherType = findTypeInKit(kit, otherPiece.type)
          const otherPortId = connection.connected.piece.id_ === piece.id_
            ? connection.connecting.port.id_
            : connection.connected.port.id_
          const otherPort = findPortInType(otherType, otherPortId || '')
          externalConnections.push({ connection, requiredPort: otherPort })
        } catch (error) { continue }
      }
    }
  }
  return kit.types?.filter(replacementType => {
    if (variants !== undefined && !variants.includes(replacementType.variant ?? '')) return false
    if (!replacementType.ports || replacementType.ports.length === 0) return externalConnections.length === 0
    return externalConnections.every(({ requiredPort }) => {
      return replacementType.ports!.some(replacementPort => arePortsCompatible(replacementPort, requiredPort))
    })
  }) ?? []
}


//#endregion Predicates

//#endregion Querying

/**
 * Sets a quality in a qualities array. If a quality with the same name exists, it is overwritten.
 * @param qualities - The array of qualities to modify
 * @param name - The name of the quality to set
 * @param value - The value of the quality
 * @param unit - Optional unit of the quality
 * @param definition - Optional definition of the quality
 * @returns The updated qualities array
 */
export const setQuality = (
  quality: Quality,
  qualities: Quality[] | undefined,
): Quality[] => {
  const qualitiesArray = qualities || []
  const existingIndex = qualitiesArray.findIndex(q => q.name === quality.name)
  if (existingIndex >= 0) qualitiesArray[existingIndex] = quality
  else qualitiesArray.push(quality)
  return qualitiesArray
}

/**
 * Sets multiple qualities in a qualities array. For each quality, if one with the same name exists, it is overwritten.
 * @param qualities - The array of qualities to modify
 * @param newQualities - Array of qualities to set
 * @returns The updated qualities array
 */
export const setQualities = (
  qualities: Quality[] | undefined,
  newQualities: Quality[]
): Quality[] => {
  return newQualities.reduce((acc, quality) => setQuality(quality, acc), qualities || [])
}

export const mergeDesigns = (designs: Design[]): Design => {
  const pieces = designs.flatMap(d => d.pieces ?? [])
  const connections = designs.flatMap(d => d.connections ?? [])
  return { ...designs[0], pieces, connections }
}

export const orientDesign = (design: Design, plane?: Plane, center?: DiagramPoint): Design => {
  let fixedPieces = design.pieces?.filter(isFixedPiece) ?? []
  if (plane !== undefined) fixedPieces = fixedPieces.map(p => ({ ...p, plane: matrixToPlane(planeToMatrix(plane).premultiply(planeToMatrix(p.plane!))) }))
  if (center !== undefined) fixedPieces = fixedPieces.map(p => ({ ...p, center: { x: p.center!.x + center.x, y: p.center!.y + center.y } }))
  return { ...design, pieces: design.pieces?.map(p => fixedPieces.find(fp => fp.id_ === p.id_) ?? p) ?? [] }
}

const roundPlane = (plane: Plane): Plane => ({
  origin: { x: round(plane.origin.x), y: round(plane.origin.y), z: round(plane.origin.z) },
  xAxis: { x: round(plane.xAxis.x), y: round(plane.xAxis.y), z: round(plane.xAxis.z) },
  yAxis: { x: round(plane.yAxis.x), y: round(plane.yAxis.y), z: round(plane.yAxis.z) }
})

export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1)
export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1)
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476)

export const planeToMatrix = (plane: Plane): THREE.Matrix4 => {
  const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z)
  const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z)
  const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z)
  const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis).normalize()
  const orthoYAxis = new THREE.Vector3().crossVectors(zAxis, xAxis).normalize()
  const matrix = new THREE.Matrix4().makeBasis(xAxis.normalize(), orthoYAxis, zAxis).setPosition(origin)
  return matrix
}
export const matrixToPlane = (matrix: THREE.Matrix4): Plane => {
  const origin = new THREE.Vector3()
  const xAxis = new THREE.Vector3()
  const yAxis = new THREE.Vector3()
  const zAxis = new THREE.Vector3()
  matrix.decompose(origin, new THREE.Quaternion(), new THREE.Vector3())
  matrix.extractBasis(xAxis, yAxis, zAxis)
  return {
    origin: { x: origin.x, y: origin.y, z: origin.z },
    xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
    yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z }
  }
}

export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z)

const computeChildPlane = (parentPlane: Plane, parentPort: Port, childPort: Port, connection: Connection): Plane => {
  const parentMatrix = planeToMatrix(parentPlane)
  const parentPoint = vectorToThree(parentPort.point)
  const parentDirection = vectorToThree(parentPort.direction).normalize()
  const childPoint = vectorToThree(childPort.point)
  const childDirection = vectorToThree(childPort.direction).normalize()

  const { gap, shift, rise, rotation, turn, tilt } = connection
  const rotationRad = THREE.MathUtils.degToRad(rotation ?? 0)
  const turnRad = THREE.MathUtils.degToRad(turn ?? 0)
  const tiltRad = THREE.MathUtils.degToRad(tilt ?? 0)

  const reverseChildDirection = childDirection.clone().negate()

  let alignQuat: THREE.Quaternion
  if (new THREE.Vector3().crossVectors(parentDirection, reverseChildDirection).length() < 0.01) {
    // Parallel vectors
    // Idea taken from: // https://github.com/dfki-ric/pytransform3d/blob/143943b028fc776adfc6939b1d7c2c6edeaa2d90/pytransform3d/rotations/_utils.py#L253
    if (Math.abs(parentDirection.z) < TOLERANCE) {
      alignQuat = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 0, 1), Math.PI) // 180* around z axis
    } else {
      // 180* around cross product of z and parentDirection
      const axis = new THREE.Vector3(0, 0, 1).cross(parentDirection).normalize()
      alignQuat = new THREE.Quaternion().setFromAxisAngle(axis, Math.PI)
    }
  } else {
    alignQuat = new THREE.Quaternion().setFromUnitVectors(reverseChildDirection, parentDirection)
  }

  const directionT = new THREE.Matrix4().makeRotationFromQuaternion(alignQuat)

  const yAxis = new THREE.Vector3(0, 1, 0)
  const parentPortQuat = new THREE.Quaternion().setFromUnitVectors(yAxis, parentDirection)
  const parentRotationT = new THREE.Matrix4().makeRotationFromQuaternion(parentPortQuat)

  const gapDirection = new THREE.Vector3(0, 1, 0).applyMatrix4(parentRotationT)
  const shiftDirection = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT)
  const raiseDirection = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT)
  const turnAxis = new THREE.Vector3(0, 0, 1).applyMatrix4(parentRotationT)
  const tiltAxis = new THREE.Vector3(1, 0, 0).applyMatrix4(parentRotationT)

  let orientationT = directionT.clone()

  const rotateT = new THREE.Matrix4().makeRotationAxis(parentDirection, -rotationRad)
  orientationT.premultiply(rotateT)

  turnAxis.applyMatrix4(rotateT)
  tiltAxis.applyMatrix4(rotateT)

  const turnT = new THREE.Matrix4().makeRotationAxis(turnAxis, turnRad)
  orientationT.premultiply(turnT)

  const tiltT = new THREE.Matrix4().makeRotationAxis(tiltAxis, tiltRad)
  orientationT.premultiply(tiltT)

  const centerChildT = new THREE.Matrix4().makeTranslation(-childPoint.x, -childPoint.y, -childPoint.z)
  let transform = new THREE.Matrix4().multiplyMatrices(orientationT, centerChildT)

  const gapTransform = new THREE.Matrix4().makeTranslation(
    gapDirection.x * (gap ?? 0),
    gapDirection.y * (gap ?? 0),
    gapDirection.z * (gap ?? 0)
  )
  const shiftTransform = new THREE.Matrix4().makeTranslation(
    shiftDirection.x * (shift ?? 0),
    shiftDirection.y * (shift ?? 0),
    shiftDirection.z * (shift ?? 0)
  )
  const raiseTransform = new THREE.Matrix4().makeTranslation(
    raiseDirection.x * (rise ?? 0),
    raiseDirection.y * (rise ?? 0),
    raiseDirection.z * (rise ?? 0)
  )

  const translationT = raiseTransform.clone().multiply(shiftTransform).multiply(gapTransform)
  transform.premultiply(translationT)
  const moveToParentT = new THREE.Matrix4().makeTranslation(parentPoint.x, parentPoint.y, parentPoint.z)
  transform.premultiply(moveToParentT)
  const finalMatrix = new THREE.Matrix4().multiplyMatrices(parentMatrix, transform)

  return matrixToPlane(finalMatrix)
}

/**
 * Returns a 'flattened' version of a design from a kit, with all piece positions and planes resolved.
 *
 * Given a kit and a designId, this function finds the corresponding design and computes the absolute
 * placement (plane and center) for each piece by traversing the design's connection graph. It uses
 * breadth-first search to propagate placement information from root pieces (those with a defined plane)
 * to all connected pieces, resolving their positions in 3D space.
 *
 * Throws an error if the design is not found in the kit.
 *
 * This is useful for preparing a design for visualization or further processing, ensuring all pieces
 * have explicit placement information.
 *
 * @param kit - The kit containing the design and types
 * @param designId - The identifier for the design to flatten
 * @returns The flattened Design object with resolved piece positions and planes
 * @throws If the design is not found in the kit
 */
export const flattenDesign = (kit: Kit, designId: DesignIdLike): Design => {
  const normalizedDesignId = designIdLikeToDesignId(designId)
  const design = findDesignInKit(kit, normalizedDesignId)
  if (!design) {
    throw new Error(`Design ${normalizedDesignId.name} not found in kit ${kit.name}`)
  }
  const types = kit.types ?? []
  if (!design.pieces || design.pieces.length === 0) return design

  const typesDict: { [key: string]: { [key: string]: Type } } = {}
  types.forEach((t) => {
    if (!typesDict[t.name]) typesDict[t.name] = {}
    typesDict[t.name][t.variant || ''] = t
  })
  const getType = (typeId: TypeId): Type | undefined => {
    return typesDict[typeId.name]?.[typeId.variant || '']
  }
  const getPort = (type: Type | undefined, portId: PortId | undefined): Port | undefined => {
    if (!type?.ports) return undefined
    return portId?.id_ ? type.ports.find((p) => p.id_ === portId.id_) : type.ports[0]
  }

  const flatDesign: Design = JSON.parse(JSON.stringify(design))
  if (!flatDesign.pieces) flatDesign.pieces = []

  const piecePlanes: { [pieceId: string]: Plane } = {}
  const pieceMap: { [pieceId: string]: Piece } = {}
  flatDesign.pieces!.forEach((p) => {
    if (p.id_) pieceMap[p.id_] = p
  })

  const cy = cytoscape({
    elements: {
      nodes: flatDesign.pieces!.map((piece) => ({
        data: { id: piece.id_, label: piece.id_ }
      })),
      edges:
        flatDesign.connections?.map((connection, index) => {
          const sourceId = connection.connected.piece.id_
          const targetId = connection.connecting.piece.id_
          return {
            data: {
              id: `${sourceId}--${targetId}`,
              source: sourceId,
              target: targetId,
              connectionData: connection
            }
          }
        }) ?? []
    },
    headless: true
  })

  const components = cy.elements().components()
  let isFirstRoot = true

  components.forEach((component) => {
    let roots = component.nodes().filter((node) => {
      const piece = pieceMap[node.id()]
      return piece?.plane !== undefined
    })
    let rootNode = roots.length > 0 ? roots[0] : component.nodes().length > 0 ? component.nodes()[0] : undefined
    if (!rootNode) return
    const rootPiece = pieceMap[rootNode.id()]
    if (!rootPiece || !rootPiece.id_) return
    rootPiece.qualities = setQuality({ name: 'semio.fixedPieceId', value: rootPiece.id_ }, rootPiece.qualities)
    rootPiece.qualities = setQuality({ name: 'semio.depth', value: '0' }, rootPiece.qualities)
    let rootPlane: Plane
    if (rootPiece.plane) {
      rootPlane = rootPiece.plane
    } else if (isFirstRoot) {
      const identityMatrix = new THREE.Matrix4().identity()
      rootPlane = matrixToPlane(identityMatrix)
      isFirstRoot = false
    } else {
      console.warn(
        `Root piece ${rootPiece.id_} has no defined plane and is not the first root. Defaulting to identity plane.`
      )
      const identityMatrix = new THREE.Matrix4().identity()
      rootPlane = matrixToPlane(identityMatrix)
    }

    piecePlanes[rootPiece.id_] = rootPlane
    const rootPieceIndex = flatDesign.pieces!.findIndex((p) => p.id_ === rootPiece.id_)
    if (rootPieceIndex !== -1) {
      flatDesign.pieces![rootPieceIndex].plane = rootPlane
    }

    const bfs = cy.elements().bfs({
      roots: `#${rootNode.id()}`,
      visit: (v, e, u, i, depth) => {
        if (!e) return
        const edgeData = e.data()
        const connection: Connection | undefined = edgeData.connectionData
        if (!connection) return
        const parentNode = u
        const childNode = v
        const parentId = parentNode.id()
        const childId = childNode.id()
        const parentPiece = pieceMap[parentId]
        const childPiece = pieceMap[childId]
        if (!parentPiece || !childPiece || !parentPiece.id_ || !childPiece.id_) return
        if (piecePlanes[childPiece.id_]) return
        const parentPlane = piecePlanes[parentPiece.id_]
        if (!parentPlane) {
          console.error(`Error during flatten: Parent piece ${parentPiece.id_} plane not found.`)
          return
        }
        const parentSide = connection.connected.piece.id_ === parentId ? connection.connected : connection.connecting
        const childSide = connection.connecting.piece.id_ === childId ? connection.connecting : connection.connected
        const parentType = getType(parentPiece.type)
        const childType = getType(childPiece.type)
        const parentPort = getPort(parentType, parentSide.port)
        const childPort = getPort(childType, childSide.port)
        if (!parentPort || !childPort) {
          console.error(
            `Error during flatten: Ports not found for connection between ${parentId} and ${childId}. Parent Port: ${parentSide.port.id_}, Child Port: ${childSide.port.id_}`
          )
          return
        }
        const childPlane = roundPlane(computeChildPlane(parentPlane, parentPort, childPort, connection))
        piecePlanes[childPiece.id_] = childPlane
        const direction = vectorToThree({
          // icon offset in direction
          x: connection.x ?? 0,
          y: connection.y ?? 0,
          z: 0
        }).normalize()
        const childCenter = {
          x: round(parentPiece.center!.x + (connection.x ?? 0) + direction.x),
          y: round(parentPiece.center!.y + (connection.y ?? 0) + direction.y)
        }

        const flatChildPiece: Piece = {
          ...childPiece,
          plane: childPlane,
          center: childCenter,
          qualities: setQualities(childPiece.qualities, [
            {
              name: 'semio.fixedPieceId',
              value: parentPiece.qualities?.find((q) => q.name === 'semio.fixedPieceId')?.value ?? ''
            },
            {
              name: 'semio.parentPieceId',
              value: parentPiece.id_
            },
            {
              name: 'semio.depth',
              value: depth.toString()
            }
          ])
        }
        pieceMap[childId] = flatChildPiece
      },
      directed: false
    })
  })
  flatDesign.pieces = flatDesign.pieces?.map((p) => pieceMap[p.id_ ?? ''])
  flatDesign.connections = []
  return flatDesign
}

export const applyDesignDiff = (base: Design, diff: DesignDiff, inplace: boolean = false): Design => {
  if (inplace) {
    const effectivePieces: Piece[] = base.pieces
      ? base.pieces
        .map((p: Piece) => {
          const pd = diff.pieces?.updated?.find((up: PieceDiff) => up.id_ === p.id_)
          const isRemoved = diff.pieces?.removed?.some((rp: PieceId) => rp.id_ === p.id_)
          const baseWithUpdate = pd ? { ...p, ...pd } : p
          const diffStatus = isRemoved ? DiffStatus.Removed : pd ? DiffStatus.Modified : DiffStatus.Unchanged
          return {
            ...baseWithUpdate,
            qualities: setQuality({ name: 'semio.diffStatus', value: diffStatus }, baseWithUpdate.qualities)
          }
        })
        .concat(
          (diff.pieces?.added || []).map((p: Piece) => ({
            ...p,
            qualities: setQuality({ name: 'semio.diffStatus', value: DiffStatus.Added }, p.qualities)
          }))
        )
      : (diff.pieces?.added || []).map((p: Piece) => ({
        ...p,
        qualities: setQuality({ name: 'semio.diffStatus', value: DiffStatus.Added }, p.qualities)
      }))

    const effectiveConnections: Connection[] = base.connections
      ? base.connections
        .map((c: Connection) => {
          const cd = diff.connections?.updated?.find(
            (ud: ConnectionDiff) =>
              ud.connected?.piece?.id_ === c.connected.piece.id_ &&
              ud.connecting?.piece?.id_ === c.connecting.piece.id_ &&
              (ud.connected?.port?.id_ || '') === (c.connected.port?.id_ || '') &&
              (ud.connecting?.port?.id_ || '') === (c.connecting.port?.id_ || '')
          )
          const isRemoved = diff.connections?.removed?.some(
            (rc: ConnectionId) =>
              rc.connected.piece.id_ === c.connected.piece.id_ && rc.connecting.piece.id_ === c.connecting.piece.id_
          )
          const baseWithUpdate = cd ? {
            ...c,
            ...cd,
            connected: { piece: cd.connected.piece, port: cd.connected.port || c.connected.port },
            connecting: { piece: cd.connecting.piece, port: cd.connecting.port || c.connecting.port }
          } : c
          const diffStatus = isRemoved ? DiffStatus.Removed : cd ? DiffStatus.Modified : DiffStatus.Unchanged
          return {
            ...baseWithUpdate,
            qualities: setQuality({ name: 'semio.diffStatus', value: diffStatus }, baseWithUpdate.qualities)
          }
        })
        .concat(
          (diff.connections?.added || []).map((c: Connection) => ({
            ...c,
            qualities: setQuality({ name: 'semio.diffStatus', value: DiffStatus.Added }, c.qualities)
          }))
        )
      : (diff.connections?.added || []).map((c: Connection) => ({
        ...c,
        qualities: setQuality({ name: 'semio.diffStatus', value: DiffStatus.Added }, c.qualities)
      }))

    return { ...base, pieces: effectivePieces, connections: effectiveConnections }
  } else {
    const effectivePieces: Piece[] = base.pieces
      ? base.pieces
        .map((p: Piece) => {
          const pd = diff.pieces?.updated?.find((up: PieceDiff) => up.id_ === p.id_)
          return pd ? { ...p, ...pd } : p
        })
        .filter((p: Piece) => !diff.pieces?.removed?.some((rp: PieceId) => rp.id_ === p.id_))
        .concat(diff.pieces?.added || [])
      : diff.pieces?.added || []

    const effectiveConnections: Connection[] = base.connections
      ? base.connections
        .map((c: Connection) => {
          const cd = diff.connections?.updated?.find(
            (ud: ConnectionDiff) =>
              ud.connected?.piece?.id_ === c.connected.piece.id_ &&
              ud.connecting?.piece?.id_ === c.connecting.piece.id_ &&
              (ud.connected?.port?.id_ || '') === (c.connected.port?.id_ || '') &&
              (ud.connecting?.port?.id_ || '') === (c.connecting.port?.id_ || '')
          )
          return cd ? {
            ...c,
            ...cd,
            connected: { piece: cd.connected.piece, port: cd.connected.port || c.connected.port },
            connecting: { piece: cd.connecting.piece, port: cd.connecting.port || c.connecting.port }
          } : c
        })
        .filter(
          (c: Connection) =>
            !diff.connections?.removed?.some(
              (rc: ConnectionId) =>
                rc.connected.piece.id_ === c.connected.piece.id_ && rc.connecting.piece.id_ === c.connecting.piece.id_
            )
        )
        .concat(diff.connections?.added || [])
      : diff.connections?.added || []

    return { ...base, pieces: effectivePieces, connections: effectiveConnections }
  }
}

const selectRepresentation = (representations: Representation[], tags: string[]): Representation => {
  const indices = representations.map((r) => jaccard(r.tags, tags))
  const maxIndex = Math.max(...indices)
  const maxIndexIndex = indices.indexOf(maxIndex)
  return representations[maxIndexIndex]
}

/**
 * 🔗 Returns a map of piece ids to representation urls for the given design and types.
 * @param design - The design with the pieces to get the representation urls for.
 * @param types - The types of the pieces with the representations.
 * @returns A map of piece ids to representation urls.
 */
export const getPieceRepresentationUrls = (design: Design, types: Type[], tags: string[] = []): Map<string, string> => {
  const representationUrls = new Map<string, string>()
  const normalizeVariant = (v: string | undefined | null) => v ?? ''
  design.pieces?.forEach((p) => {
    const type = types.find(
      (t) => t.name === p.type.name && normalizeVariant(t.variant) === normalizeVariant(p.type.variant)
    )
    if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`)
    if (!type.representations)
      throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} has no representations`)
    const representation = selectRepresentation(type.representations, tags)
    representationUrls.set(p.id_, representation.url)
  })
  return representationUrls
}


export const piecesMetadata = (kit: Kit, designId: DesignIdLike): Map<string, { plane: Plane, center: DiagramPoint, fixedPieceId: string, parentPieceId: string | null, depth: number }> => {
  const normalizedDesignId = designIdLikeToDesignId(designId)
  const flatDesign = flattenDesign(kit, normalizedDesignId)
  const fixedPieceIds = flatDesign.pieces?.map((p) => findQualityValue(p, 'semio.fixedPieceId') || p.id_)
  const parentPieceIds = flatDesign.pieces?.map((p) => findQualityValue(p, 'semio.parentPieceId', null))
  const depths = flatDesign.pieces?.map((p) => parseInt(findQualityValue(p, 'semio.depth', '0')!))
  return new Map(flatDesign.pieces?.map((p, index) => [p.id_, { plane: p.plane!, center: p.center!, fixedPieceId: fixedPieceIds![index], parentPieceId: parentPieceIds![index], depth: depths![index] }]))
}

//#endregion