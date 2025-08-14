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
import { FC, ReactNode, createContext, useContext, useEffect, useState } from "react";
import DesignEditor from "./DesignEditor";
import { TooltipProvider } from "./Tooltip";

import { DesignId, DesignScopeProvider, KitId, KitScopeProvider, Layout, Mode, SketchpadProvider, SketchpadScopeProvider, Theme, useCommands, useLayout, useMode, useSketchpadStore, useTheme } from "@semio/js";

interface SketchpadContextType {
  navbarToolbar: ReactNode | null;
  setNavbarToolbar: (toolbar: ReactNode) => void;
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
  userId?: string;
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
}

const Sketchpad: FC<SketchpadProps> = ({ userId, onWindowEvents }) => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);

  const store = useSketchpadStore();
  const mode = useMode();
  const theme = useTheme();
  const layout = useLayout();
  const { setMode, setTheme, setLayout } = useCommands();

  const defaultKitId: KitId = { name: "Metabolism", version: "r25.07-1" };
  const defaultDesignId: DesignId = { name: "Nakagin Capsule Tower", variant: "", view: "" };

  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        await store.importKit("metabolism.json", true, true);
        await store.importFiles("metabolism.zip", defaultKitId, true);
      } catch (e) {
        console.error("Failed to import default kit:", e);
      } finally {
        if (mounted) setIsImporting(false);
      }
    })();
    return () => {
      mounted = false;
    };
  }, [store]);

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
      <SketchpadProvider id={userId}>
        <SketchpadScopeProvider id={userId}>
          <KitScopeProvider id={defaultKitId}>
            <DesignScopeProvider id={defaultDesignId}>
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
            </DesignScopeProvider>
          </KitScopeProvider>
        </SketchpadScopeProvider>
      </SketchpadProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;

// Export Sketchpad state management types for external use
export {};
