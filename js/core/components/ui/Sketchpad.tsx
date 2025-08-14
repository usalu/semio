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
import DesignEditor, { createInitialDesignEditorCoreState, DesignEditorCoreState, DesignEditorDispatcher, designEditorReducer, DesignEditorState } from "./DesignEditor";

import { default as Metabolism } from "@semio/assets/semio/kit_metabolism.json";
import { addDesignToKit, clusterDesign, Connection, Design, DesignId, ensureDesignHasFixedPiece, findDesignInKit, Kit, updateDesignInKit } from "@semio/js";
import { extractFilesAndCreateUrls } from "../../lib/utils";

// Higher-level Sketchpad state management
export interface SketchpadState {
  isLoading: boolean;
  fileUrls: Map<string, string>;
  kit: Kit | null;
  designEditorCoreStates: DesignEditorCoreState[];
  activeDesign: number; // index of the active design
  designHistory: DesignId[]; // history of previously opened designs
}

export enum SketchpadAction {
  UrlsLoaded = "URLS_LOADED",
  ChangeActiveDesign = "CHANGE_ACTIVE_DESIGN",
  PreviousDesign = "PREVIOUS_DESIGN",
  UpdateActiveDesignEditorState = "UPDATE_ACTIVE_DESIGN_EDITOR_STATE",
  AddDesign = "ADD_DESIGN",
  UpdateDesign = "UPDATE_DESIGN",
  ClusterDesign = "CLUSTER_DESIGN",
  explodeDesign = "EXPAND_DESIGN",
}

export type SketchpadActionType =
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
      payload: DesignEditorState;
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
      type: SketchpadAction.explodeDesign;
      payload: DesignId;
    };

export const sketchpadReducer = (state: SketchpadState, action: SketchpadActionType): SketchpadState => {
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
        kit: action.payload.kit,
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

      // Use selection from the design editor state
      const pieceIdsToCluster = currentSelection.selectedPieceIds || [];

      try {
        // Use the new clusterDesign function from semio.ts
        const clusterResult = clusterDesign(state.kit, currentDesignId, pieceIdsToCluster);

        // Create a new DesignEditorCoreState for the clustered design
        const clusteredDesignEditorCoreState = createInitialDesignEditorCoreState({
          initialKit: clusterResult.updatedKit,
          designId: {
            name: clusterResult.clusteredDesign.name,
            variant: clusterResult.clusteredDesign.variant || undefined,
            view: clusterResult.clusteredDesign.view || undefined,
          },
          fileUrls: state.fileUrls,
        });

        // Update the current design's editor state to reflect changes and clear selection
        const updatedCurrentDesignEditorCoreState = createInitialDesignEditorCoreState({
          initialKit: clusterResult.updatedKit,
          designId: currentDesignId,
          fileUrls: state.fileUrls,
          // Clear selection after clustering
          initialSelection: {
            selectedPieceIds: [],
            selectedConnections: [],
            selectedPiecePortId: undefined,
            selectedDesignPieces: [],
          },
        });

        // Update the designEditorCoreStates array
        const updatedDesignEditorCoreStates = [...state.designEditorCoreStates];
        updatedDesignEditorCoreStates[state.activeDesign] = updatedCurrentDesignEditorCoreState;
        updatedDesignEditorCoreStates.push(clusteredDesignEditorCoreState);

        newState = {
          ...state,
          kit: clusterResult.updatedKit,
          designEditorCoreStates: updatedDesignEditorCoreStates,
        };

        console.log("Clustered design created:", clusterResult.clusteredDesign.name);
      } catch (error) {
        console.error("Error clustering design:", error);
        newState = state;
      }
      break;

    case SketchpadAction.explodeDesign:
      if (!state.kit) {
        console.error("Cannot explode design: kit is null");
        newState = state;
        break;
      }

      const designToExplodeId = action.payload;
      let designToExplode: Design;
      try {
        designToExplode = findDesignInKit(state.kit, designToExplodeId);
      } catch (error) {
        console.error("Design to explode not found in kit:", error);
        newState = state;
        break;
      }

      // Find all designs that have connections referencing the design to explode
      const affectedDesigns: Design[] = [];
      for (const design of state.kit.designs || []) {
        if (design.name === designToExplode.name) continue; // Skip the design itself

        const hasExternalConnections = (design.connections || []).some((connection: Connection) => connection.connected.designId === designToExplode.name || connection.connecting.designId === designToExplode.name);

        if (hasExternalConnections) {
          affectedDesigns.push(design);
        }
      }

      if (affectedDesigns.length === 0) {
        console.warn("No affected designs found for expansion");
        newState = state;
        break;
      }

      // Update all affected designs to remove designId references to the explodeed design
      const updatedAffectedDesigns: Design[] = affectedDesigns.map((affectedDesign) => {
        // Get all connections that reference the design to explode
        const externalConnections = (affectedDesign.connections || []).filter((connection: Connection) => connection.connected.designId === designToExplode.name || connection.connecting.designId === designToExplode.name);

        // Get all internal connections (not referencing the design to explode)
        const internalConnections = (affectedDesign.connections || []).filter((connection: Connection) => connection.connected.designId !== designToExplode.name && connection.connecting.designId !== designToExplode.name);

        // Remove designId from external connections to restore them as regular connections
        const restoredConnections = externalConnections.map((connection: Connection) => {
          const updatedConnection = { ...connection };

          if (connection.connected.designId === designToExplode.name) {
            updatedConnection.connected = {
              ...connection.connected,
              designId: undefined,
            };
          }

          if (connection.connecting.designId === designToExplode.name) {
            updatedConnection.connecting = {
              ...connection.connecting,
              designId: undefined,
            };
          }

          return updatedConnection;
        });

        return {
          ...affectedDesign,
          connections: [...internalConnections, ...restoredConnections],
          updated: new Date(),
        };
      });

      // For simplicity, explode into the first affected design (typically the original design that was clustered)
      const targetDesign = updatedAffectedDesigns[0];

      // Combine all pieces from the target design and the design to explode
      const combinedPieces = [...(targetDesign.pieces || []), ...(designToExplode.pieces || [])];

      // Combine all connections: target design connections and connections from the explodeed design
      const combinedConnections = [...(targetDesign.connections || []), ...(designToExplode.connections || [])];

      // Create the updated target design with explodeed content
      const explodeedTargetDesign: Design = {
        ...targetDesign,
        pieces: combinedPieces,
        connections: combinedConnections,
        updated: new Date(),
      };

      // Remove the design to explode from the kit
      const kitWithoutexplodedDesign: Kit = {
        ...state.kit!,
        designs: (state.kit!.designs || []).filter((design) => design.name !== designToExplode.name),
      };

      // Update all affected designs in the kit
      let finalKitAfterExpansion = kitWithoutexplodedDesign;

      // Update the target design with explodeed content
      finalKitAfterExpansion = updateDesignInKit(finalKitAfterExpansion, explodeedTargetDesign);

      // Update other affected designs (excluding the target design which was already updated)
      for (let i = 1; i < updatedAffectedDesigns.length; i++) {
        const affectedDesign = updatedAffectedDesigns[i];
        finalKitAfterExpansion = updateDesignInKit(finalKitAfterExpansion, affectedDesign);
      }

      // Remove the DesignEditorCoreState for the explodeed design
      const explodedDesignStateIndex = state.designEditorCoreStates.findIndex(
        (designState) => designState.designId.name === designToExplode.name && designState.designId.variant === designToExplode.variant && designState.designId.view === designToExplode.view,
      );

      let updatedDesignStatesAfterExpansion = [...state.designEditorCoreStates];
      if (explodedDesignStateIndex !== -1) {
        updatedDesignStatesAfterExpansion.splice(explodedDesignStateIndex, 1);
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
      if (explodedDesignStateIndex !== -1 && explodedDesignStateIndex <= state.activeDesign) {
        newActiveDesign = Math.max(0, state.activeDesign - 1);
      }

      newState = {
        ...state,
        kit: finalKitAfterExpansion,
        designEditorCoreStates: updatedDesignStatesAfterExpansion,
        activeDesign: newActiveDesign,
      };

      console.log("Design explodeed:", designToExplode.name, "into", targetDesign.name);
      break;

    default:
      newState = state;
      break;
  }

  console.log("SKETCHPAD NEW STATE:", newState);
  return newState;
};

export const createInitialSketchpadState = (): SketchpadState => {
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
  explodeDesign: (designId: DesignId) => void;
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

    sketchpadDispatch({
      type: SketchpadAction.UpdateActiveDesignEditorState,
      payload: newState,
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

  const explodeDesign = (designId: DesignId) => {
    sketchpadDispatch({
      type: SketchpadAction.explodeDesign,
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
          explodeDesign: explodeDesign,
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
