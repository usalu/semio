// #region Header

// index.ts

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
export { default as Console, commandRegistry as inkCommandRegistry } from "@semio/js/components/ui/Console";
export { default as DesignEditor } from "@semio/js/components/ui/DesignEditor";
export { default as Diagram } from "@semio/js/components/ui/Diagram";
export { default as File } from "@semio/js/components/ui/File";
export { default as GrasshopperCatalogue } from "@semio/js/components/ui/GrasshopperCatalogue";
export { default as Model } from "@semio/js/components/ui/Model";
export { default as Sketchpad } from "@semio/js/components/ui/Sketchpad";
export { default as eslintConfig } from "@semio/js/eslint.config";
export { extractFilesAndCreateUrls } from "@semio/js/lib/utils";
export { default as postcssConfig } from "@semio/js/postcss.config";
export {
  addConnectionsToDesign,
  addConnectionsToDesignDiff, addConnectionToDesign,
  addConnectionToDesignDiff, addDesignToKit, addPiecesToDesign,
  addPiecesToDesignDiff, addPieceToDesign,
  addPieceToDesignDiff, addTypeToKit,
  applyDesignDiff,
  arePortsCompatible, CameraSchema, colorPortsForTypes,
  connectionIdLikeToConnectionId, ConnectionSchema, createClusteredDesign,
  deserialize,
  designIdLikeToDesignId, DesignSchema,
  DiffStatus, expandDesignPieces,
  findConnectionInDesign,
  findDesignInKit,
  findPieceConnectionsInDesign,
  findPieceInDesign,
  findPieceTypeInDesign,
  findPortInType,
  findQualityValue,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  findStaleConnectionsInDesign,
  findTypeInKit,
  fixPieceInDesign,
  fixPiecesInDesign,
  flattenDesign,
  getClusterableGroups,
  getIncludedDesigns,
  getPieceRepresentationUrls, ICON_WIDTH, isConnectionInDesign,
  isPortInUse,
  isSameConnection,
  isSameDesign,
  isSamePiece,
  isSamePort,
  isSameRepresentation,
  isSameType,
  kitIdLikeToKitId, KitSchema, mergeDesigns,
  pieceIdLikeToPieceId, PieceSchema, piecesMetadata, PlaneSchema, planeToMatrix, PointSchema, portIdLikeToPortId, PortSchema, qualityIdLikeToQualityId, QualitySchema, removeConnectionFromDesign,
  removeConnectionFromDesignDiff,
  removeConnectionsFromDesign,
  removeConnectionsFromDesignDiff,
  removeDesignFromKit,
  removePieceFromDesign,
  removePieceFromDesignDiff,
  removePiecesAndConnectionsFromDesign,
  removePiecesFromDesign,
  removePiecesFromDesignDiff,
  removeTypeFromKit,
  replaceClusterWithDesign,
  representationIdLikeToRepresentationId, RepresentationSchema, safeParse,
  schemas,
  serialize,
  setConnectionInDesign,
  setConnectionInDesignDiff,
  setConnectionsInDesign,
  setConnectionsInDesignDiff,
  setDesignInKit,
  setPieceInDesign,
  setPieceInDesignDiff,
  setPiecesInDesign,
  setPiecesInDesignDiff,
  setQualities,
  setQuality,
  setTypeInKit, TOLERANCE, toSemioQuaternion,
  toSemioRotation,
  toThreeQuaternion,
  toThreeRotation,
  typeIdLikeToTypeId, TypeSchema, unifyPortFamiliesAndCompatibleFamiliesForTypes,
  updateDesignInKit,
  validate, VectorSchema
} from "@semio/js/semio";
export type {
  Author,
  Camera,
  Connection,
  ConnectionDiff,
  ConnectionId,
  ConnectionsDiff,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  IncludedDesignInfo,
  Kit,
  KitId,
  Piece,
  PieceDiff,
  PieceId,
  PiecesDiff,
  Plane,
  Point,
  Port,
  PortId,
  Quality,
  Representation,
  Side,
  SideDiff,
  SideId,
  Type,
  TypeId,
  Vector
} from "@semio/js/semio";
export {
  DesignEditorScopeProvider, useDesignEditorDesignDiff,
  useDesignEditorFileUrls,
  useDesignEditorFullscreenPanel,
  useDesignEditorIsTransactionActive, useDesignEditorPresence, useDesignEditorPresenceOthers,
  useDesignEditorSelection, useDesignEditorStore as useDesignEditorState
} from "@semio/js/store";
export type { DesignEditorState } from "@semio/js/store";
export { default as tailwindConfig } from "@semio/js/tailwind.config";
// Exporting vite configs blows up storybook and nextjs
// export { default as viteConfig } from '@semio/js/vite.config';
// export { default as vitestConfig } from '@semio/js/vitest.workspace';
