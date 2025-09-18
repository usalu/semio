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
import { FC, ReactNode, useEffect, useState } from "react";
import { TooltipProvider } from "../Tooltip";

import { Kit, KitId } from "../../../semio";
import { KitScopeProvider, Layout, SketchpadScopeProvider, Theme, useLayout, useMode, useSketchpadCommands, useTheme, YProviderFactory } from "../../../store";
import { NavbarContext } from "../Navbar";
import KitEditor from "./KitEditor";

const SketchpadInner: FC = () => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);

  const { createKit, setMode, setTheme, setLayout } = useSketchpadCommands();

  const theme = useTheme();
  const layout = useLayout();
  const mode = useMode();

  const defaultKitId: KitId = { name: "Metabolism", version: "r25.07-1" };

  useEffect(() => {
    let mounted = true;
    (async () => {
      await createKit(defaultKitId as Kit);
      // await store.execute("semio.sketchpad.importKit", defaultKitId, "/metabolism.zip");
      setIsImporting(false);
    })();
    return () => {
      mounted = false;
    };
  }, []); // TODO add store to dependencies after debugging

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
    if (!theme && theme === Theme.SYSTEM && typeof window !== "undefined") {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      setTheme(prefersDark ? Theme.DARK : Theme.LIGHT);
    }
  }, [theme, layout, setTheme, setLayout]);

  if (isImporting) return null;

  return (
    <NavbarContext.Provider
      value={{
        navbarToolbar: navbarToolbar,
        setNavbarToolbar: setNavbarToolbar,
      }}
    >
      <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
        <KitScopeProvider id={defaultKitId}>
          <KitEditor />
        </KitScopeProvider>
      </div>
    </NavbarContext.Provider>
  );
};

interface SketchpadProps {
  id?: string;
  yProviderFactory?: YProviderFactory;
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
}

const Sketchpad: FC<SketchpadProps> = ({ id, yProviderFactory, onWindowEvents }) => {
  return (
    <TooltipProvider>
      <SketchpadScopeProvider id={id} yProviderFactory={yProviderFactory}>
        <SketchpadInner />
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;

// Export Sketchpad state management types for external use
export {};
