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
import DesignEditor, { createInitialDesignEditorState, DesignEditorDispatcher, designEditorReducer, DesignEditorState } from "./DesignEditor";

import { default as Metabolism } from "@semio/assets/semio/kit_metabolism.json";
import { DesignId, Kit } from "@semio/js";
import { extractFilesAndCreateUrls } from "../../lib/utils";

// Higher-level Sketchpad state management
interface SketchpadState {
  isLoading: boolean;
  fileUrls: Map<string, string>;
  designEditorStates: DesignEditorState[];
  activeDesign: number; // index of the active design
}

enum SketchpadAction {
  UrlsLoaded = "URLS_LOADED",
  ChangeActiveDesign = "CHANGE_ACTIVE_DESIGN",
  UpdateActiveDesignEditorState = "UPDATE_ACTIVE_DESIGN_EDITOR_STATE",
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
      payload: DesignEditorState;
    };

const sketchpadReducer = (state: SketchpadState, action: SketchpadActionType): SketchpadState => {
  switch (action.type) {
    case SketchpadAction.UrlsLoaded:
      const kit = Metabolism as unknown as Kit;
      const designEditorStates =
        kit.designs?.map((design) =>
          createInitialDesignEditorState({
            initialKit: kit,
            designId: {
              name: design.name,
              variant: design.variant || undefined,
              view: design.view || undefined,
            },
            fileUrls: action.payload.fileUrls,
          }),
        ) || [];

      return {
        ...state,
        isLoading: false,
        fileUrls: action.payload.fileUrls,
        designEditorStates,
      };
    case SketchpadAction.ChangeActiveDesign:
      // Find the index of the design with the matching designId
      const designIndex = state.designEditorStates.findIndex((designState) => designState.designId.name === action.payload.name && designState.designId.variant === action.payload.variant && designState.designId.view === action.payload.view);

      if (designIndex !== -1) {
        return {
          ...state,
          activeDesign: designIndex,
        };
      }
      return state;
    case SketchpadAction.UpdateActiveDesignEditorState:
      const updatedStates = [...state.designEditorStates];
      updatedStates[state.activeDesign] = action.payload;
      return {
        ...state,
        designEditorStates: updatedStates,
      };
    default:
      return state;
  }
};

const createInitialSketchpadState = (): SketchpadState => {
  return {
    isLoading: true,
    fileUrls: new Map(),
    designEditorStates: [],
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
  designEditorState: DesignEditorState | null;
  designEditorDispatch: DesignEditorDispatcher | null;
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
  const { designEditorState, designEditorDispatch, sketchpadDispatch, sketchpadState } = useSketchpad();

  const onDesignIdChange = (newDesignId: DesignId) => {
    sketchpadDispatch({
      type: SketchpadAction.ChangeActiveDesign,
      payload: newDesignId,
    });
  };

  const availableDesigns = sketchpadState.designEditorStates.map((state) => state.designId);

  return (
    <DesignEditor
      designId={designEditorState!.designId}
      fileUrls={designEditorState!.fileUrls}
      state={designEditorState}
      dispatch={designEditorDispatch}
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
  const activeDesignEditorState = sketchpadState.designEditorStates[sketchpadState.activeDesign];

  const designEditorDispatch: DesignEditorDispatcher = (action) => {
    const newState = designEditorReducer(activeDesignEditorState, action);
    sketchpadDispatch({
      type: SketchpadAction.UpdateActiveDesignEditorState,
      payload: newState,
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
          designEditorState: activeDesignEditorState,
          designEditorDispatch: designEditorDispatch,
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
