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
import { FC, ReactNode, createContext, useContext, useEffect, useState } from "react";
import DesignEditor from "./DesignEditor";

import { DesignId } from "@semio/js";
import {
  DesignEditorScopeProvider,
  DesignScopeProvider,
  KitScopeProvider,
  Layout,
  Mode,
  SketchpadScopeProvider,
  SketchpadProvider as StoreProvider,
  Theme,
  useActiveDesignEditorId,
  useSketchpadCommands,
  useSketchpadLayout,
  useSketchpadMode,
  useSketchpadStore,
  useSketchpadTheme,
} from "../../store";

// Helper
const keyOf = (d: DesignId) => `${d.name}::${d.variant || ""}::${d.view || ""}`;

interface SketchpadContextType {
  navbarToolbar: ReactNode | null;
  setNavbarToolbar: (toolbar: ReactNode) => void;
}

const SketchpadContext = createContext<SketchpadContextType | null>(null);

export const useSketchpad = () => {
  const context = useContext(SketchpadContext);
  if (!context) {
    throw new Error("useSketchpadCommands must be used within a SketchpadProvider");
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

const SketchpadInner: FC<SketchpadProps> = ({ onWindowEvents }) => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);

  const store = useSketchpadStore();
  const mode = useSketchpadMode();
  const theme = useSketchpadTheme();
  const layout = useSketchpadLayout();
  const activeDesignEditorId = useActiveDesignEditorId();
  const { setMode, setTheme, setLayout } = useSketchpadCommands();

  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        await store.importFiles("metabolism.zip");
        await store.importKit("metabolism.json", true);
      } catch (e) {
      } finally {
        if (mounted) setIsImporting(false);
      }
    })();
    return () => {
      mounted = false;
    };
  }, []);

  useEffect(() => {
    if (mode !== Mode.USER) setMode(mode);
    if (layout !== Layout.NORMAL) setLayout(layout);
    if (theme && theme !== Theme.SYSTEM) setTheme(theme);
    if (!theme && theme === Theme.SYSTEM && typeof window !== "undefined") {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      setTheme(prefersDark ? Theme.DARK : Theme.LIGHT);
    }
  }, [mode, theme, layout, setMode, setTheme, setLayout]);

  // useEffect(() => {
  //   if (activeDesignEditorId) {
  //     // Convert string to DesignId
  //     const designId = designIdLikeToDesignId(activeDesignEditorId);
  //     setActiveDesignId(designId);
  //   } else {
  //     setActiveDesignId(null);
  //   }
  // }, [activeDesignEditorId]);

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
        <KitScopeProvider id={{ name: "Metabolism" }}>
          <DesignScopeProvider id={{ name: "Nakagin Capsule Tower", variant: "", view: "" }}>
            <DesignEditorScopeProvider>
              <SketchpadContext.Provider
                value={{
                  navbarToolbar: navbarToolbar,
                  setNavbarToolbar: setNavbarToolbar,
                }}
              >
                <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
                  <DesignEditor />
                </div>
              </SketchpadContext.Provider>
            </DesignEditorScopeProvider>
          </DesignScopeProvider>
        </KitScopeProvider>
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

const Sketchpad: FC<SketchpadProps> = (props) => (
  <StoreProvider>
    <SketchpadInner {...props} />
  </StoreProvider>
);

export default Sketchpad;

// Export Sketchpad state management types for external use
export {};
