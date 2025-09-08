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
import { TooltipProvider } from "../Tooltip";
import DesignEditor from "./DesignEditor";

import { Design, DesignId, Kit, KitId } from "../../../semio";
import { DesignEditorId, DesignScopeProvider, KitScopeProvider, Layout, Mode, SketchpadScopeProvider, Theme, useKitCommands, useLayout, useMode, useSketchpadCommands, useTheme } from "../../../store";

interface NavbarContextType {
  navbarToolbar: ReactNode | null;
  setNavbarToolbar: (toolbar: ReactNode) => void;
}

const NavbarContext = createContext<NavbarContextType | null>(null);

export const useNavbar = () => {
  const context = useContext(NavbarContext);
  if (!context) {
    throw new Error("useNavbar must be used within a NavbarProvider");
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

// Component that uses basic store hooks (no kit context needed)
const SketchpadInner: FC = () => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);

  const { createKit, createDesignEditor, setActiveDesignEditor, setMode, setTheme, setLayout } = useSketchpadCommands();
  const { createDesign } = useKitCommands();

  const theme = useTheme();
  const layout = useLayout();
  const mode = useMode();

  const defaultKitId: KitId = { name: "Metabolism", version: "r25.07-1" };
  const defaultDesignId: DesignId = { name: "Nakagin Capsule Tower", variant: "", view: "" };

  useEffect(() => {
    let mounted = true;
    (async () => {
      await createKit(defaultKitId as Kit);
      await createDesign(defaultDesignId as Design);
      // await store.execute("semio.sketchpad.importKit", defaultKitId, "/metabolism.zip");
      await createDesignEditor({ kit: defaultKitId, design: defaultDesignId } as DesignEditorId);
      await setActiveDesignEditor({ kit: defaultKitId, design: defaultDesignId } as DesignEditorId);
      setIsImporting(false);
    })();
    return () => {
      mounted = false;
    };
  }, [defaultKitId, defaultDesignId]); // TODO add store to dependencies after debugging

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

  useEffect(() => {
    if (mode !== Mode.USER) setMode(mode);
    if (layout !== Layout.NORMAL) setLayout(layout);
    if (theme && theme !== Theme.SYSTEM) setTheme(theme);
    if (!theme && theme === Theme.SYSTEM && typeof window !== "undefined") {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      setTheme(prefersDark ? Theme.DARK : Theme.LIGHT);
    }
  }, [mode, theme, layout, setMode, setTheme, setLayout]);

  if (isImporting) return null;

  return (
    <KitScopeProvider id={defaultKitId}>
      <DesignScopeProvider id={defaultDesignId}>
        <NavbarContext.Provider
          value={{
            navbarToolbar: navbarToolbar,
            setNavbarToolbar: setNavbarToolbar,
          }}
        >
          <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
            <DesignEditor />
          </div>
        </NavbarContext.Provider>
      </DesignScopeProvider>
    </KitScopeProvider>
  );
};

const Sketchpad: FC<SketchpadProps> = ({ userId, onWindowEvents }) => {
  return (
    <TooltipProvider>
      <SketchpadScopeProvider id={userId || "default-user"} persisted={false}>
        <SketchpadInner />
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;

// Export Sketchpad state management types for external use
export {};
