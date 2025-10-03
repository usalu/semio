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
import { MemoryRouter, Routes, Route, Outlet } from "react-router";
import { TooltipProvider } from "../Tooltip";

import { Kit, KitId } from "../../../semio";
import { KitScopeProvider, Layout, SketchpadScopeProvider, Theme, useLayout, useMode, useSketchpadCommands, useTheme, WindowEvents, YProviderFactory } from "../../../store";
import { NavbarContext } from "../Navbar";
import KitEditor from "./KitEditor";
import DesignEditor from "./DesignEditor";
import TypeEditor from "./TypeEditor";

const Home: FC = ({  }) => {
  return (
    <div>
      Home
    </div>
  );
};

const KitRoute: FC = () => {
  let { uuid } = useParams();
  // let [searchParams, setSearchParams] = useSearchParams();

  return (
    <KitScopeProvider uuid={uuid}>
      <Outlet/>
    </ KitScopeProvider>
  )
}

const DesignRoute: FC = () => {
  let { uuid} = useParams();
  // let [searchParams, setSearchParams] = useSearchParams();

  return (
    <DesignScopeProvider uuid={uuid}>
      <Outlet/>
    </ DesignScopeProvider>
  )
}

const TypeRoute: FC = () => {
  let { uuid} = useParams();
  // let [searchParams, setSearchParams] = useSearchParams();

  return (
    <DesignScopeProvider uuid={uuid}>
      <Outlet/>
    </ DesignScopeProvider>
  )
}


const Home: FC = ({  }) => {
  return (
    <div>
      Home
    </div>
  );
};

const SketchpadInner: FC = () => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const [navbarToolbar, setToolbar] = useState<ReactNode>(null);

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
        setToolbar: setToolbar,
      }}
    >
      <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
          <KitEditor />
      </div>
    </NavbarContext.Provider>
  );
};

interface SketchpadProps {
  id?: string;
  yProviderFactory?: YProviderFactory;
  onWindowEvents?: WindowEvents;
}

const Sketchpad: FC<SketchpadProps> = ({ id, yProviderFactory, onWindowEvents }) => {
  return (
    <TooltipProvider>
      <SketchpadScopeProvider id={id} yProviderFactory={yProviderFactory} onWindowEvents={onWindowEvents}>
        <MemoryRouter>
          <Routes index element={<Home />}>
              <Route path=":uuid" element={<KitRoute />}>
                <Route index element={<KitEditor />} />
                <Route path="d/:uuid" element={<DesignRoute />}>
                  <Route index element={<DesignEditor />} />
                </Route>
                <Route path="t/:uuid" element={<TypeRoute />} >
                  <Route index element={<TypeEditor />} />
                </Route>
              </Route>
          </Routes>
        </MemoryRouter>,
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;