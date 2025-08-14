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
  AuthorSchema, CameraSchema, ConnectionIdLikeSchema, ConnectionIdSchema, ConnectionSchema, DesignIdLikeSchema, DesignIdSchema, DesignSchema, DiagramPointSchema,
  DiagramVectorSchema, DiffStatus,
  // Constants
  ICON_WIDTH, KitIdLikeSchema, KitIdSchema, KitSchema, LocationSchema, PieceIdLikeSchema, PieceIdSchema, PieceSchema, PlaneSchema, PointSchema, PortIdLikeSchema, PortIdSchema, PortSchema, QualityIdLikeSchema, QualityIdSchema,
  // Schemas
  QualitySchema, RepresentationIdLikeSchema, RepresentationIdSchema, RepresentationSchema, SideIdLikeSchema, SideIdSchema, SideSchema, TOLERANCE, TypeIdLikeSchema, TypeIdSchema, TypeSchema, VectorSchema, addConnectionToDesign,
  addConnectionToDesignDiff,
  // Domain Logic Functions
  addConnectionsToDesign,
  addConnectionsToDesignDiff, addDesignToKit, addPieceToDesign,
  addPieceToDesignDiff, addPiecesToDesign,
  addPiecesToDesignDiff, addTypeToKit,
  applyDesignDiff,
  arePortsCompatible, colorPortsForTypes, connectionIdLikeToConnectionId, createClusteredDesign, deserialize, designIdLikeToDesignId, expandDesignPieces,
  findConnectionInDesign,
  findDesignInKit,
  findPieceConnectionsInDesign,
  findPieceInDesign,
  findPieceTypeInDesign,
  findPortInType,
  findQualityValue,
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
  getPieceRepresentationUrls, isConnectionInDesign,
  isPortInUse,
  isSameConnection,
  isSameDesign,
  isSamePiece,
  isSamePort,
  isSameRepresentation,
  isSameType, kitIdLikeToKitId, mergeDesigns, parseDesignIdFromVariant, pieceIdLikeToPieceId, piecesMetadata, planeToMatrix, portIdLikeToPortId, qualityIdLikeToQualityId, removeConnectionFromDesign,
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
  replaceClusterWithDesign, representationIdLikeToRepresentationId, safeParse,
  // Utilities
  schemas, serialize, setConnectionInDesign,
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
  setTypeInKit, toSemioQuaternion,
  toSemioRotation,
  toThreeQuaternion,
  toThreeRotation, typeIdLikeToTypeId, unifyPortFamiliesAndCompatibleFamiliesForTypes,
  updateDesignInKit, validate
} from "./semio";
// #endregion

// #region Types (from semio.ts)
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
  PortId,
  Quality,
  QualityId,
  Representation,
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
  DesignEditorStoreFullscreenPanel, Layout, Mode,
  Theme
} from "./store";

export type {
  DesignEditorStoreOperationStackEntry, DesignEditorStorePresence,
  DesignEditorStorePresenceOther, DesignEditorStoreSelection, DesignEditorStoreState
} from "./store";
// #endregion

// #region Hooks (from store.tsx)
export {
  ConnectionScopeProvider, DesignEditorStoreScopeProvider, DesignScopeProvider, KitScopeProvider, PieceScopeProvider, PortypeScopeProvider, RepresentationScopeProvider,
  // Context providers
  SketchpadScopeProvider, TypeScopeProvider, useCommands, useConnection, useConnectionScope, useConnections, useDesign, useDesignEditorScope,
  // Design Editor hooks
  useDesignEditorStore, useDesignEditorStoreDesignDiff, useDesignEditorStoreFileUrls, useDesignEditorStoreFullscreenPanel, useDesignEditorStoreIsTransactionActive,
  useDesignEditorStorePresence,
  useDesignEditorStorePresenceOthers, useDesignEditorStoreSelection, useDesignId, useDesignScope, useDesigns, useDiff, useFileUrls, useFullscreen, useKit, useKitScope, useKits, useLayout,
  // Concise hooks
  useMode, useOthers, usePiece, usePieceScope, usePieces, usePort, usePortScope, usePorts, usePresence, useRepresentationScope, useRepresentations, useSelection,
  // Scope hooks
  useSketchpadScope,
  // Core hooks
  useSketchpadStore, useTheme, useTransaction, useType, useTypeScope, useTypes
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
