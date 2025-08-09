// #region Header

// Sketchpad.tsx

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
import { TooltipProvider } from "@semio/js/components/ui/Tooltip";
import { createContext, FC, ReactNode, useContext, useEffect, useReducer, useState } from "react";
import DesignEditor, { createInitialDesignEditorCoreState, DesignEditorAction, DesignEditorCoreState, DesignEditorDispatcher, designEditorReducer, DesignEditorState } from "./DesignEditor";

import { default as Metabolism } from "@semio/assets/semio/kit_metabolism.json";
import { addDesignToKit, Connection, Design, DesignId, findDesignInKit, getClusterableGroups, Kit, Piece, updateDesignInKit } from "@semio/js";
import { extractFilesAndCreateUrls } from "../../lib/utils";

// Function to ensure design has at least one fixed piece using breadth-first search
const ensureDesignHasFixedPiece = (design: Design): Design => {
  if (!design.pieces || design.pieces.length === 0) {
    return design;
  }

  // Check if any piece is already fixed
  const hasFixedPiece = design.pieces.some((piece: Piece) => piece.plane && piece.center);
  if (hasFixedPiece) {
    return design;
  }

  // Build adjacency list for BFS
  const adjacencyList = new Map<string, string[]>();
  design.pieces.forEach((piece: Piece) => {
    if (piece.id_) {
      adjacencyList.set(piece.id_, []);
    }
  });

  // Add connections to adjacency list
  design.connections?.forEach((connection: Connection) => {
    const connectedId = connection.connected.piece.id_;
    const connectingId = connection.connecting.piece.id_;

    if (connectedId && connectingId) {
      adjacencyList.get(connectedId)?.push(connectingId);
      adjacencyList.get(connectingId)?.push(connectedId);
    }
  });

  // Find the piece with the most connections using BFS
  let maxConnections = -1;
  let parentPieceId: string | null = null;

  for (const piece of design.pieces) {
    if (!piece.id_) continue;

    const visited = new Set<string>();
    const queue = [piece.id_];
    visited.add(piece.id_);
    let connectionCount = 0;

    while (queue.length > 0) {
      const currentId = queue.shift()!;
      const neighbors = adjacencyList.get(currentId) || [];

      connectionCount += neighbors.length;

      for (const neighborId of neighbors) {
        if (!visited.has(neighborId)) {
          visited.add(neighborId);
          queue.push(neighborId);
        }
      }
    }

    if (connectionCount > maxConnections) {
      maxConnections = connectionCount;
      parentPieceId = piece.id_;
    }
  }

  // If no connections exist, just pick the first piece
  if (!parentPieceId && design.pieces.length > 0) {
    parentPieceId = design.pieces[0].id_ || null;
  }

  if (!parentPieceId) {
    return design;
  }

  // Fix the parent piece with center and plane
  const updatedPieces = design.pieces.map((piece: Piece) => {
    if (piece.id_ === parentPieceId) {
      return {
        ...piece,
        center: piece.center || { x: 0, y: 0 },
        plane: piece.plane || {
          origin: { x: 0, y: 0, z: 0 },
          xAxis: { x: 1, y: 0, z: 0 },
          yAxis: { x: 0, y: 1, z: 0 },
        },
      };
    }
    return piece;
  });

  return {
    ...design,
    pieces: updatedPieces,
  };
};

// Higher-level Sketchpad state management
interface SketchpadState {
  isLoading: boolean;
  fileUrls: Map<string, string>;
  kit: Kit | null;
  designEditorCoreStates: DesignEditorCoreState[];
  activeDesign: number; // index of the active design
  designHistory: DesignId[]; // history of previously opened designs
}

enum SketchpadAction {
  UrlsLoaded = "URLS_LOADED",
  ChangeActiveDesign = "CHANGE_ACTIVE_DESIGN",
  PreviousDesign = "PREVIOUS_DESIGN",
  UpdateActiveDesignEditorState = "UPDATE_ACTIVE_DESIGN_EDITOR_STATE",
  AddDesign = "ADD_DESIGN",
  UpdateDesign = "UPDATE_DESIGN",
  ClusterDesign = "CLUSTER_DESIGN",
  ExpandDesign = "EXPAND_DESIGN",
}

type SketchpadActionType =
  | {
      type: SketchpadAction.UrlsLoaded;
      payload: { fileUrls: Map<string, string> };
    }
  | {
      type: SketchpadAction.ChangeActiveDesign;
      payload: DesignId;
    }
  | {
      type: SketchpadAction.PreviousDesign;
      payload: null;
    }
  | {
      type: SketchpadAction.UpdateActiveDesignEditorState;
      payload: DesignEditorCoreState;
    }
  | {
      type: SketchpadAction.AddDesign;
      payload: Design;
    }
  | {
      type: SketchpadAction.UpdateDesign;
      payload: Design;
    }
  | {
      type: SketchpadAction.ClusterDesign;
      payload: null;
    }
  | {
      type: SketchpadAction.ExpandDesign;
      payload: DesignId;
    };

const sketchpadReducer = (state: SketchpadState, action: SketchpadActionType): SketchpadState => {
  console.log("SKETCHPAD ACTION:", action.type, action.payload);

  let newState: SketchpadState;

  switch (action.type) {
    case SketchpadAction.UrlsLoaded:
      const kit = Metabolism as unknown as Kit;
      const designEditorCoreStates =
        kit.designs?.map((design) =>
          createInitialDesignEditorCoreState({
            initialKit: kit,
            designId: {
              name: design.name,
              variant: design.variant || undefined,
              view: design.view || undefined,
            },
            fileUrls: action.payload.fileUrls,
          }),
        ) || [];

      newState = {
        ...state,
        isLoading: false,
        fileUrls: action.payload.fileUrls,
        kit,
        designEditorCoreStates,
        designHistory: [],
      };
      break;

    case SketchpadAction.ChangeActiveDesign:
      // Find the index of the design with the matching designId
      const designIndex = state.designEditorCoreStates.findIndex((designState) => designState.designId.name === action.payload.name && designState.designId.variant === action.payload.variant && designState.designId.view === action.payload.view);

      if (designIndex !== -1) {
        // Get the current active design to add to history
        const currentActiveDesignState = state.designEditorCoreStates[state.activeDesign];
        const currentDesignId = currentActiveDesignState?.designId;

        // Only add to history if we're changing to a different design
        let updatedHistory = [...state.designHistory];
        if (currentDesignId && (currentDesignId.name !== action.payload.name || currentDesignId.variant !== action.payload.variant || currentDesignId.view !== action.payload.view)) {
          updatedHistory.push(currentDesignId);
        }

        newState = {
          ...state,
          activeDesign: designIndex,
          designHistory: updatedHistory,
        };
      } else {
        newState = state;
      }
      break;

    case SketchpadAction.PreviousDesign:
      if (state.designHistory.length === 0) {
        console.warn("No previous design in history");
        newState = state;
        break;
      }

      // Get the last design from history
      const previousDesignId = state.designHistory[state.designHistory.length - 1];

      // Find the index of the previous design
      const previousDesignIndex = state.designEditorCoreStates.findIndex(
        (designState) => designState.designId.name === previousDesignId.name && designState.designId.variant === previousDesignId.variant && designState.designId.view === previousDesignId.view,
      );

      if (previousDesignIndex !== -1) {
        // Remove the last item from history (we're going back to it)
        const updatedHistoryAfterBack = state.designHistory.slice(0, -1);

        newState = {
          ...state,
          activeDesign: previousDesignIndex,
          designHistory: updatedHistoryAfterBack,
        };
      } else {
        // Remove the invalid design from history and try again if there are more
        const cleanedHistory = state.designHistory.slice(0, -1);
        newState = {
          ...state,
          designHistory: cleanedHistory,
        };
      }
      break;

    case SketchpadAction.UpdateActiveDesignEditorState:
      const updatedStates = [...state.designEditorCoreStates];
      updatedStates[state.activeDesign] = action.payload;
      newState = {
        ...state,
        designEditorCoreStates: updatedStates,
      };
      break;

    case SketchpadAction.AddDesign:
      if (!state.kit) {
        console.error("Cannot add design: kit is null");
        newState = state;
        break;
      }
      const newDesign = action.payload;

      // Ensure at least one piece is fixed using breadth-first search
      const processedDesign = ensureDesignHasFixedPiece(newDesign);

      const updatedKit = addDesignToKit(state.kit, processedDesign);

      // Create a new DesignEditorCoreState for the newly added design
      const newDesignEditorCoreState = createInitialDesignEditorCoreState({
        initialKit: updatedKit,
        designId: {
          name: processedDesign.name,
          variant: processedDesign.variant || undefined,
          view: processedDesign.view || undefined,
        },
        fileUrls: state.fileUrls,
      });

      newState = {
        ...state,
        kit: updatedKit,
        designEditorCoreStates: [...state.designEditorCoreStates, newDesignEditorCoreState],
      };
      break;

    case SketchpadAction.UpdateDesign:
      if (!state.kit) {
        console.error("Cannot update design: kit is null");
        newState = state;
        break;
      }
      const designToUpdate = action.payload;
      const updatedKitWithDesign = updateDesignInKit(state.kit, designToUpdate);

      newState = {
        ...state,
        kit: updatedKitWithDesign,
      };
      break;

    case SketchpadAction.ClusterDesign:
      if (!state.kit) {
        console.error("Cannot cluster design: kit is null");
        newState = state;
        break;
      }

      // Get the current active design and its state
      const activeDesignEditorState = state.designEditorCoreStates[state.activeDesign];
      if (!activeDesignEditorState) {
        console.error("No active design found for clustering");
        newState = state;
        break;
      }

      const currentDesignId = activeDesignEditorState.designId;
      const currentSelection = activeDesignEditorState.selection;

      let design: Design;
      try {
        design = findDesignInKit(state.kit, currentDesignId);
      } catch (error) {
        console.error("Current design not found in kit:", error);
        newState = state;
        break;
      }

      // Use selection from the design editor state
      const pieceIdsToCluster = currentSelection.selectedPieceIds || [];

      // Validate clustering is possible
      const clusterableGroups = getClusterableGroups(design, pieceIdsToCluster);
      if (clusterableGroups.length === 0) {
        console.warn("No clusterable groups found with current selection");
        newState = state;
        break;
      }

      // Use the first (and typically only) clusterable group
      const finalClusterPieceIds = clusterableGroups[0];

      const designName = `Cluster-${Date.now()}`;

      // Separate regular pieces from design nodes
      const regularPieceIds = finalClusterPieceIds.filter((id) => !id.startsWith("design-"));
      const designNodeIds = finalClusterPieceIds.filter((id) => id.startsWith("design-"));

      // Collect all pieces to include in the cluster
      let allPiecesToCluster: Piece[] = [];
      let allConnectionsToCluster: Connection[] = [];
      let allExternalConnections: Connection[] = [];

      // Add regular pieces
      if (regularPieceIds.length > 0) {
        const regularPieces = (design.pieces || []).filter((piece: Piece) => regularPieceIds.includes(piece.id_));
        allPiecesToCluster.push(...regularPieces);

        // Find connections involving regular pieces
        const regularConnections = (design.connections || []).filter((connection: Connection) => regularPieceIds.includes(connection.connected.piece.id_) || regularPieceIds.includes(connection.connecting.piece.id_));

        // Separate internal and external connections for regular pieces
        const internalRegularConnections = regularConnections.filter((connection: Connection) => regularPieceIds.includes(connection.connected.piece.id_) && regularPieceIds.includes(connection.connecting.piece.id_));

        const externalRegularConnections = regularConnections.filter((connection: Connection) => {
          const connectedInCluster = regularPieceIds.includes(connection.connected.piece.id_);
          const connectingInCluster = regularPieceIds.includes(connection.connecting.piece.id_);
          return connectedInCluster !== connectingInCluster; // XOR - exactly one is in cluster
        });

        allConnectionsToCluster.push(...internalRegularConnections);
        allExternalConnections.push(...externalRegularConnections);
      }

      // Add pieces from design nodes
      for (const designNodeId of designNodeIds) {
        // Extract design name from design node ID (format: "design-DesignName")
        const referencedDesignName = designNodeId.replace("design-", "");

        let referencedDesign: Design | null = null;
        try {
          referencedDesign = findDesignInKit(state.kit, { name: referencedDesignName });
        } catch (error) {
          console.warn(`Referenced design ${referencedDesignName} not found in kit:`, error);
          continue;
        }

        if (referencedDesign && referencedDesign.pieces) {
          // Simply use the original pieces and connections
          allPiecesToCluster.push(...referencedDesign.pieces);

          // Use the original connections as-is
          if (referencedDesign.connections) {
            allConnectionsToCluster.push(...referencedDesign.connections);
          }

          // Find external connections that connect to this design node
          const designExternalConnections = (design.connections || []).filter((connection: Connection) => connection.connected.designId === referencedDesignName || connection.connecting.designId === referencedDesignName);

          allExternalConnections.push(...designExternalConnections);
        }
      }

      // Create the clustered design with all collected pieces and connections
      const clusteredDesign: Design = {
        name: designName,
        unit: design.unit,
        description: `Hierarchical cluster with ${allPiecesToCluster.length} pieces`,
        pieces: allPiecesToCluster,
        connections: allConnectionsToCluster,
        created: new Date(),
        updated: new Date(),
      };

      // Ensure at least one piece is fixed
      const processedClusteredDesign = ensureDesignHasFixedPiece(clusteredDesign);

      // Remove all clustered items from the current design
      const remainingPieces = (design.pieces || []).filter((piece: Piece) => !regularPieceIds.includes(piece.id_));

      // Remove connections involving clustered regular pieces or design nodes
      const remainingConnections = (design.connections || []).filter((connection: Connection) => {
        const connectedInRegularCluster = regularPieceIds.includes(connection.connected.piece.id_);
        const connectingInRegularCluster = regularPieceIds.includes(connection.connecting.piece.id_);
        const connectedInDesignCluster = designNodeIds.some((designId) => {
          const designName = designId.replace("design-", "");
          return connection.connected.designId === designName;
        });
        const connectingInDesignCluster = designNodeIds.some((designId) => {
          const designName = designId.replace("design-", "");
          return connection.connecting.designId === designName;
        });

        return !connectedInRegularCluster && !connectingInRegularCluster && !connectedInDesignCluster && !connectingInDesignCluster;
      });

      // Update external connections to reference the new clustered design
      const updatedExternalConnections = allExternalConnections.map((connection: Connection) => {
        const connectedInCluster =
          regularPieceIds.includes(connection.connected.piece.id_) ||
          designNodeIds.some((designId) => {
            const designName = designId.replace("design-", "");
            return connection.connected.designId === designName;
          });

        const connectingInCluster =
          regularPieceIds.includes(connection.connecting.piece.id_) ||
          designNodeIds.some((designId) => {
            const designName = designId.replace("design-", "");
            return connection.connecting.designId === designName;
          });

        if (connectedInCluster) {
          return {
            ...connection,
            connected: {
              piece: { id_: connection.connected.piece.id_ },
              port: connection.connected.port,
              designId: processedClusteredDesign.name,
            },
          };
        } else if (connectingInCluster) {
          return {
            ...connection,
            connecting: {
              piece: { id_: connection.connecting.piece.id_ },
              port: connection.connecting.port,
              designId: processedClusteredDesign.name,
            },
          };
        }

        return connection;
      });

      const updatedCurrentDesign: Design = {
        ...design,
        pieces: remainingPieces,
        connections: [...remainingConnections, ...updatedExternalConnections],
        updated: new Date(),
      };

      // Add the clustered design to the kit
      const kitWithClusteredDesign = addDesignToKit(state.kit, processedClusteredDesign);

      // Update the current design in the kit
      const finalKit = updateDesignInKit(kitWithClusteredDesign, updatedCurrentDesign);

      // Create a new DesignEditorCoreState for the clustered design
      const clusteredDesignEditorCoreState = createInitialDesignEditorCoreState({
        initialKit: finalKit,
        designId: {
          name: processedClusteredDesign.name,
          variant: processedClusteredDesign.variant || undefined,
          view: processedClusteredDesign.view || undefined,
        },
        fileUrls: state.fileUrls,
      });

      // Update the current design's editor state to reflect changes and clear selection
      const updatedCurrentDesignEditorCoreState = createInitialDesignEditorCoreState({
        initialKit: finalKit,
        designId: currentDesignId,
        fileUrls: state.fileUrls,
        // Clear selection after clustering
        initialSelection: {
          selectedPieceIds: [],
          selectedConnections: [],
          selectedPiecePortId: undefined,
        },
      });

      // Update the designEditorCoreStates array
      const updatedDesignEditorCoreStates = [...state.designEditorCoreStates];
      updatedDesignEditorCoreStates[state.activeDesign] = updatedCurrentDesignEditorCoreState;
      updatedDesignEditorCoreStates.push(clusteredDesignEditorCoreState);

      newState = {
        ...state,
        kit: finalKit,
        designEditorCoreStates: updatedDesignEditorCoreStates,
      };

      console.log("Clustered design created:", processedClusteredDesign.name);
      break;

    case SketchpadAction.ExpandDesign:
      if (!state.kit) {
        console.error("Cannot expand design: kit is null");
        newState = state;
        break;
      }

      const designToExpandId = action.payload;
      let designToExpand: Design;
      try {
        designToExpand = findDesignInKit(state.kit, designToExpandId);
      } catch (error) {
        console.error("Design to expand not found in kit:", error);
        newState = state;
        break;
      }

      // Find all designs that have connections referencing the design to expand
      const affectedDesigns: Design[] = [];
      for (const design of state.kit.designs || []) {
        if (design.name === designToExpand.name) continue; // Skip the design itself

        const hasExternalConnections = (design.connections || []).some((connection: Connection) => connection.connected.designId === designToExpand.name || connection.connecting.designId === designToExpand.name);

        if (hasExternalConnections) {
          affectedDesigns.push(design);
        }
      }

      if (affectedDesigns.length === 0) {
        console.warn("No affected designs found for expansion");
        newState = state;
        break;
      }

      // For simplicity, expand into the first affected design (typically the original design that was clustered)
      const targetDesign = affectedDesigns[0];

      // Get all external connections that reference the design to expand
      const externalConnections = (targetDesign.connections || []).filter((connection: Connection) => connection.connected.designId === designToExpand.name || connection.connecting.designId === designToExpand.name);

      // Get all internal connections (not referencing the design to expand)
      const internalConnections = (targetDesign.connections || []).filter((connection: Connection) => connection.connected.designId !== designToExpand.name && connection.connecting.designId !== designToExpand.name);

      // Remove designId from external connections to restore them as regular connections
      const restoredConnections = externalConnections.map((connection: Connection) => {
        const updatedConnection = { ...connection };

        if (connection.connected.designId === designToExpand.name) {
          updatedConnection.connected = {
            ...connection.connected,
            designId: undefined,
          };
        }

        if (connection.connecting.designId === designToExpand.name) {
          updatedConnection.connecting = {
            ...connection.connecting,
            designId: undefined,
          };
        }

        return updatedConnection;
      });

      // Combine all pieces from the target design and the design to expand
      const combinedPieces = [...(targetDesign.pieces || []), ...(designToExpand.pieces || [])];

      // Combine all connections: internal connections, restored external connections, and connections from the expanded design
      const combinedConnections = [...internalConnections, ...restoredConnections, ...(designToExpand.connections || [])];

      // Create the updated target design with expanded content
      const expandedTargetDesign: Design = {
        ...targetDesign,
        pieces: combinedPieces,
        connections: combinedConnections,
        updated: new Date(),
      };

      // Remove the design to expand from the kit
      const kitWithoutExpandedDesign = {
        ...state.kit,
        designs: (state.kit.designs || []).filter((design) => design.name !== designToExpand.name),
      };

      // Update the target design in the kit
      const finalKitAfterExpansion = updateDesignInKit(kitWithoutExpandedDesign, expandedTargetDesign);

      // Remove the DesignEditorCoreState for the expanded design
      const expandedDesignStateIndex = state.designEditorCoreStates.findIndex(
        (designState) => designState.designId.name === designToExpand.name && designState.designId.variant === designToExpand.variant && designState.designId.view === designToExpand.view,
      );

      let updatedDesignStatesAfterExpansion = [...state.designEditorCoreStates];
      if (expandedDesignStateIndex !== -1) {
        updatedDesignStatesAfterExpansion.splice(expandedDesignStateIndex, 1);
      }

      // Update the target design's editor state
      const targetDesignStateIndex = updatedDesignStatesAfterExpansion.findIndex(
        (designState) => designState.designId.name === targetDesign.name && designState.designId.variant === targetDesign.variant && designState.designId.view === targetDesign.view,
      );

      if (targetDesignStateIndex !== -1) {
        const updatedTargetDesignEditorCoreState = createInitialDesignEditorCoreState({
          initialKit: finalKitAfterExpansion,
          designId: {
            name: targetDesign.name,
            variant: targetDesign.variant || undefined,
            view: targetDesign.view || undefined,
          },
          fileUrls: state.fileUrls,
        });
        updatedDesignStatesAfterExpansion[targetDesignStateIndex] = updatedTargetDesignEditorCoreState;
      }

      // Adjust activeDesign index if necessary
      let newActiveDesign = state.activeDesign;
      if (expandedDesignStateIndex !== -1 && expandedDesignStateIndex <= state.activeDesign) {
        newActiveDesign = Math.max(0, state.activeDesign - 1);
      }

      newState = {
        ...state,
        kit: finalKitAfterExpansion,
        designEditorCoreStates: updatedDesignStatesAfterExpansion,
        activeDesign: newActiveDesign,
      };

      console.log("Design expanded:", designToExpand.name, "into", targetDesign.name);
      break;

    default:
      newState = state;
      break;
  }

  console.log("SKETCHPAD NEW STATE:", newState);
  return newState;
};

const createInitialSketchpadState = (): SketchpadState => {
  return {
    isLoading: true,
    fileUrls: new Map(),
    kit: null,
    designEditorCoreStates: [],
    activeDesign: 0,
    designHistory: [],
  };
};

export enum Mode {
  USER = "user",
  GUEST = "guest",
}

export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

export enum Layout {
  NORMAL = "normal",
  TOUCH = "touch",
}

interface SketchpadContextType {
  mode: Mode;
  layout: Layout;
  setLayout: (layout: Layout) => void;
  theme: Theme;
  setTheme: (theme: Theme) => void;
  setNavbarToolbar: (toolbar: ReactNode) => void;
  sketchpadState: SketchpadState;
  sketchpadDispatch: (action: SketchpadActionType) => void;
  kit: Kit | null;
  designEditorState: DesignEditorState | null;
  designEditorDispatch: DesignEditorDispatcher | null;
  addDesign: (design: Design) => void;
  updateDesign: (design: Design) => void;
  clusterDesign: () => void;
  expandDesign: (designId: DesignId) => void;
  previousDesign: () => void;
}

const SketchpadContext = createContext<SketchpadContextType | null>(null);

export const useSketchpad = () => {
  const context = useContext(SketchpadContext);
  if (!context) {
    throw new Error("useSketchpad must be used within a SketchpadProvider");
  }
  return context;
};

interface ViewProps {}

const View = () => {
  const { kit, designEditorState, designEditorDispatch, sketchpadDispatch, sketchpadState } = useSketchpad();

  const onDesignIdChange = (newDesignId: DesignId) => {
    sketchpadDispatch({
      type: SketchpadAction.ChangeActiveDesign,
      payload: newDesignId,
    });
  };

  const availableDesigns = sketchpadState.designEditorCoreStates.map((state) => state.designId);

  if (!kit || !designEditorState) return null;

  return (
    <DesignEditor
      kit={kit}
      designId={designEditorState.designId}
      fileUrls={designEditorState.fileUrls}
      externalState={designEditorState}
      externalDispatch={designEditorDispatch}
      onDesignIdChange={onDesignIdChange}
      availableDesigns={availableDesigns}
      onToolbarChange={() => {}}
    />
  );
};

interface SketchpadProps {
  mode?: Mode;
  theme?: Theme;
  layout?: Layout;
  readonly?: boolean;
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
  userId: string;
}

const Sketchpad: FC<SketchpadProps> = ({ mode = Mode.USER, theme, layout = Layout.NORMAL, onWindowEvents, userId }) => {
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);
  const [currentLayout, setCurrentLayout] = useState<Layout>(layout);
  const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
    if (theme) return theme;
    if (typeof window !== "undefined") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? Theme.DARK : Theme.LIGHT;
    }
    return Theme.LIGHT;
  });

  const [sketchpadState, sketchpadDispatch] = useReducer(sketchpadReducer, createInitialSketchpadState());

  useEffect(() => {
    extractFilesAndCreateUrls("metabolism.zip").then((urls) => {
      sketchpadDispatch({
        type: SketchpadAction.UrlsLoaded,
        payload: { fileUrls: urls },
      });
    });
  }, []);
  const activeDesignEditorCoreState = sketchpadState.designEditorCoreStates[sketchpadState.activeDesign];
  const activeDesignEditorState = activeDesignEditorCoreState
    ? {
        ...activeDesignEditorCoreState,
        kit: sketchpadState.kit!,
      }
    : null;

  const designEditorDispatch: DesignEditorDispatcher = (action) => {
    if (!activeDesignEditorState) return;

    const newState = designEditorReducer(activeDesignEditorState, action);

    // If this is a SetDesign action, also update the design in the Sketchpad kit
    if (action.type === DesignEditorAction.SetDesign) {
      const updatedDesign = action.payload;
      sketchpadDispatch({
        type: SketchpadAction.UpdateDesign,
        payload: updatedDesign,
      });
    }

    // Extract core state (without kit) to store in Sketchpad
    const { kit, ...coreState } = newState;
    sketchpadDispatch({
      type: SketchpadAction.UpdateActiveDesignEditorState,
      payload: coreState,
    });
  };

  const addDesign = (design: Design) => {
    sketchpadDispatch({
      type: SketchpadAction.AddDesign,
      payload: design,
    });
  };

  const updateDesign = (design: Design) => {
    sketchpadDispatch({
      type: SketchpadAction.UpdateDesign,
      payload: design,
    });
  };

  const clusterDesign = () => {
    sketchpadDispatch({
      type: SketchpadAction.ClusterDesign,
      payload: null,
    });
  };

  const expandDesign = (designId: DesignId) => {
    sketchpadDispatch({
      type: SketchpadAction.ExpandDesign,
      payload: designId,
    });
  };

  const previousDesign = () => {
    sketchpadDispatch({
      type: SketchpadAction.PreviousDesign,
      payload: null,
    });
  };

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(Theme.DARK);
    if (currentTheme === Theme.DARK) {
      root.classList.add(Theme.DARK);
    }
  }, [currentTheme]);
  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(Layout.TOUCH);
    if (currentLayout === Layout.TOUCH) {
      root.classList.add(Layout.TOUCH);
    }
  }, [currentLayout]);

  if (sketchpadState.isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <TooltipProvider>
      {/* <StudioStoreProvider userId={userId}> */}
      <SketchpadContext.Provider
        value={{
          mode: mode,
          layout: currentLayout,
          setLayout: setCurrentLayout,
          theme: currentTheme,
          setTheme: setCurrentTheme,
          setNavbarToolbar: setNavbarToolbar,
          sketchpadState: sketchpadState,
          sketchpadDispatch: sketchpadDispatch,
          kit: sketchpadState.kit,
          designEditorState: activeDesignEditorState,
          designEditorDispatch: designEditorDispatch,
          addDesign: addDesign,
          updateDesign: updateDesign,
          clusterDesign: clusterDesign,
          expandDesign: expandDesign,
          previousDesign: previousDesign,
        }}
      >
        <div key={`layout-${currentLayout}`} className="h-full w-full flex flex-col bg-background text-foreground">
          <View />
        </div>
      </SketchpadContext.Provider>
      {/* </StudioStoreProvider> */}
    </TooltipProvider>
  );
};

export default Sketchpad;

// Export Sketchpad state management types for external use
export { SketchpadAction };
export type { SketchpadActionType, SketchpadState };
