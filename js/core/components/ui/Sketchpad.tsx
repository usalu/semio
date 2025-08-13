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
import { FC, ReactNode, createContext, useCallback, useContext, useEffect, useState } from "react";
import DesignEditor from "./DesignEditor";

import { DesignId, KitId } from "@semio/js";
import {
  DesignEditorStoreScopeProvider,
  DesignScopeProvider,
  KitScopeProvider,
  Layout,
  Mode,
  SketchpadScopeProvider,
  Theme,
  useSketchpadCommands,
  useSketchpadLayout,
  useSketchpadMode,
  useSketchpadStore,
  useSketchpadStoreActiveDesignEditorId,
  useSketchpadTheme,
} from "../../store";

// Helper to create design editor ID from design
const keyOf = (d: DesignId) => `${d.name}::${d.variant || ""}::${d.view || ""}`;

interface SketchpadContextType {
  navbarToolbar: ReactNode | null;
  setNavbarToolbar: (toolbar: ReactNode) => void;
  designEditors: Map<string, string>;
  getOrCreateDesignEditor: (kitId: { name: string; version?: string }, designId: DesignId) => string;
}

const SketchpadContext = createContext<SketchpadContextType | null>(null);

export const useSketchpad = () => {
  const context = useContext(SketchpadContext);
  if (!context) {
    throw new Error("useSketchpad must be used within a SketchpadProvider");
  }
  return context;
};

interface SketchpadProps {
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
}

const Sketchpad: FC<SketchpadProps> = ({ onWindowEvents }) => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);
  const [designEditors, setDesignEditors] = useState<Map<string, string>>(new Map());

  const store = useSketchpadStore();
  const mode = useSketchpadMode();
  const theme = useSketchpadTheme();
  const layout = useSketchpadLayout();
  const activeDesignEditorId = useSketchpadStoreActiveDesignEditorId();
  const { setMode, setTheme, setLayout, setActiveDesignEditorId } = useSketchpadCommands();

  const defaultKitId: KitId = { name: "Metabolism", version: "r25.07-1" };
  const defaultDesignId: DesignId = { name: "Nakagin Capsule Tower", variant: "", view: "" };

  const getOrCreateDesignEditor = useCallback(
    (kitId: { name: string; version?: string }, designId: DesignId): string => {
      const designKey = keyOf(designId);

      let editorId = designEditors.get(designKey);
      if (!editorId) {
        editorId = store.createDesignEditorStoreStore(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "");
        if (editorId) {
          setDesignEditors((prev) => new Map(prev).set(designKey, editorId!));
        }
      }

      return editorId || "";
    },
    [designEditors, store],
  );

  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        await store.importFiles("metabolism.zip", true);
        await store.importKit("metabolism.json", true, true);

        // Create design editor for the default design
        const editorId = getOrCreateDesignEditor(defaultKitId, defaultDesignId);
        setActiveDesignEditorId(editorId);
      } catch (e) {
        console.error("Failed to import default kit:", e);
      } finally {
        if (mounted) setIsImporting(false);
      }
    })();
    return () => {
      mounted = false;
    };
  }, [store, getOrCreateDesignEditor, setActiveDesignEditorId]);

  useEffect(() => {
    if (mode !== Mode.USER) setMode(mode);
    if (layout !== Layout.NORMAL) setLayout(layout);
    if (theme && theme !== Theme.SYSTEM) setTheme(theme);
    if (!theme && theme === Theme.SYSTEM && typeof window !== "undefined") {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      setTheme(prefersDark ? Theme.DARK : Theme.LIGHT);
    }
  }, [mode, theme, layout, setMode, setTheme, setLayout]);

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(Theme.DARK);
    if (theme === Theme.DARK) {
      root.classList.add(Theme.DARK);
    }
  }, [theme]);

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(Layout.TOUCH);
    if (layout === Layout.TOUCH) {
      root.classList.add(Layout.TOUCH);
    }
  }, [layout]);

  if (isImporting) return null;

  return (
    <TooltipProvider>
      <SketchpadScopeProvider>
        <KitScopeProvider id={defaultKitId}>
          <DesignScopeProvider id={defaultDesignId}>
            {activeDesignEditorId && (
              <DesignEditorStoreScopeProvider id={activeDesignEditorId}>
                <SketchpadContext.Provider
                  value={{
                    navbarToolbar: navbarToolbar,
                    setNavbarToolbar: setNavbarToolbar,
                    designEditors: designEditors,
                    getOrCreateDesignEditor: getOrCreateDesignEditor,
                  }}
                >
                  <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
                    <DesignEditor />
                  </div>
                </SketchpadContext.Provider>
              </DesignEditorStoreScopeProvider>
            )}
          </DesignScopeProvider>
        </KitScopeProvider>
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;

// Export Sketchpad state management types for external use
export {};
