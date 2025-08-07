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
import { addDesignToKit, Connection, Design, DesignId, Kit, Piece, updateDesignInKit } from "@semio/js";
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
}

enum SketchpadAction {
  UrlsLoaded = "URLS_LOADED",
  ChangeActiveDesign = "CHANGE_ACTIVE_DESIGN",
  UpdateActiveDesignEditorState = "UPDATE_ACTIVE_DESIGN_EDITOR_STATE",
  AddDesign = "ADD_DESIGN",
  UpdateDesign = "UPDATE_DESIGN",
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
      };
      break;

    case SketchpadAction.ChangeActiveDesign:
      // Find the index of the design with the matching designId
      const designIndex = state.designEditorCoreStates.findIndex((designState) => designState.designId.name === action.payload.name && designState.designId.variant === action.payload.variant && designState.designId.view === action.payload.view);

      if (designIndex !== -1) {
        newState = {
          ...state,
          activeDesign: designIndex,
        };
      } else {
        newState = state;
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
