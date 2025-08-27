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

// #region Components
export { default as Console, commandRegistry as inkCommandRegistry } from "./components/ui/Console";
export { default as DesignEditor } from "./components/ui/DesignEditor";
export { default as Diagram } from "./components/ui/Diagram";
export { default as File } from "./components/ui/File";
export { default as GrasshopperCatalogue } from "./components/ui/GrasshopperCatalogue";
export { default as Model } from "./components/ui/Model";
export { default as Sketchpad } from "./components/ui/Sketchpad";
// #endregion

// #region Utils
export { extractFilesAndCreateUrls } from "./lib/utils";
// #endregion

// #region Domain Logic (from semio.ts)
export {
  // Domain Logic Functions
  addConnectionsToDesign,
  addConnectionsToDesignDiff, addConnectionToDesign,
  addConnectionToDesignDiff, addDesignToKit, addPiecesToDesign,
  addPiecesToDesignDiff, addPieceToDesign,
  addPieceToDesignDiff, addTypeToKit,
  applyDesignDiff,
  arePortsCompatible, AttributeIdLikeSchema, attributeIdLikeToAttributeId, AttributeIdSchema,
  // Schemas
  AttributeSchema, AuthorSchema, CameraSchema, colorPortsForTypes, ConnectionIdLikeSchema, connectionIdLikeToConnectionId, ConnectionIdSchema, ConnectionSchema, createClusteredDesign, deserialize, DesignIdLikeSchema, designIdLikeToDesignId, DesignIdSchema, DesignSchema, DiagramPointSchema,
  DiagramVectorSchema, DiffStatus, expandDesignPieces, findAttributeValue, findConnectionInDesign,
  findDesignInKit,
  findPieceConnectionsInDesign,
  findPieceInDesign,
  findPieceTypeInDesign,
  findPortInType,
  // Helper Functions
  findReplacableDesignsForDesignPiece, findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  findStaleConnectionsInDesign,
  findTypeInKit,
  fixPieceInDesign,
  fixPiecesInDesign,
  flattenDesign,
  getClusterableGroups,
  getIncludedDesigns,
  getPieceRepresentationUrls,
  // Constants
  ICON_WIDTH, isConnectionInDesign,
  isPortInUse,
  isSameConnection,
  isSameDesign,
  isSamePiece,
  isSamePort,
  isSameRepresentation,
  isSameType, KitIdLikeSchema, kitIdLikeToKitId, KitIdSchema, KitSchema, LocationSchema, mergeDesigns, parseDesignIdFromVariant, PieceIdLikeSchema, pieceIdLikeToPieceId, PieceIdSchema, PieceSchema, piecesMetadata, PlaneSchema, planeToMatrix, PointSchema, PortIdLikeSchema, portIdLikeToPortId, PortIdSchema, PortSchema, removeConnectionFromDesign,
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
  replaceClusterWithDesign, RepresentationIdLikeSchema, representationIdLikeToRepresentationId, RepresentationIdSchema, RepresentationSchema, safeParse,
  // Utilities
  schemas, serialize, setAttribute, setAttributes, setConnectionInDesign,
  setConnectionInDesignDiff,
  setConnectionsInDesign,
  setConnectionsInDesignDiff,
  setDesignInKit,
  setPieceInDesign,
  setPieceInDesignDiff,
  setPiecesInDesign,
  setPiecesInDesignDiff, setTypeInKit, SideIdLikeSchema, SideIdSchema, SideSchema, TOLERANCE, toSemioQuaternion,
  toSemioRotation,
  toThreeQuaternion,
  toThreeRotation, TypeIdLikeSchema, typeIdLikeToTypeId, TypeIdSchema, TypeSchema, unifyPortFamiliesAndCompatibleFamiliesForTypes,
  updateDesignInKit, VectorSchema
} from "./semio";
// #endregion

// #region Types (from semio.ts)
export type {
  Attribute, AttributeId, Author,
  Camera,
  Connection,
  ConnectionDiff,
  ConnectionId,
  ConnectionsDiff,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  DiagramVector,
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
  PortId, Representation,
  RepresentationId,
  Side,
  SideDiff,
  SideId,
  Type,
  TypeId,
  Vector
} from "./semio";
// #endregion

// #region State Management (from store.tsx)
export {
  DesignEditorFullscreenPanel, Layout, Mode,
  Theme
} from "./store";

export type {
  DesignEditorPresence, DesignEditorPresenceOther, DesignEditorSelection, DesignEditorState
} from "./store";
// #endregion

// #region Hooks (from store.tsx)
export {
  ConnectionScopeProvider, DesignEditorScopeProvider, DesignScopeProvider, KitScopeProvider, PieceScopeProvider, PortypeScopeProvider, RepresentationScopeProvider,
  // Context providers
  SketchpadScopeProvider, TypeScopeProvider, useCommands, useConnection, useDesign, useDesignEditor, useDesignId, useDesigns,
  // Design Editor hooks
  useDiff, useFileUrls, useFullscreen, useKit, useLayout,
  // Concise hooks
  useMode, useOthers, usePiece, usePort, useRepresentation, useSelection,
  // Scope hooks
  useSketchpad,
  // Core hooks
  useTheme, useType
} from "./store";
// #endregion

// #region Configs
export { default as eslintConfig } from "./eslint.config";
export { default as postcssConfig } from "./postcss.config";
export { default as tailwindConfig } from "./tailwind.config";
// Exporting vite configs blows up storybook and nextjs
// export { default as viteConfig } from './vite.config';
// export { default as vitestConfig } from './vitest.workspace';
// #endregion
