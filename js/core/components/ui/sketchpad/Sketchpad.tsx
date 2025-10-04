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
import { FC } from "react";
import { MemoryRouter, Outlet, Route, Routes, useParams } from "react-router";
import { TooltipProvider } from "../Tooltip";

import { DesignScopeProvider, KitScopeProvider, SketchpadScopeProvider, useLayout, WindowEvents, YProviderFactory } from "../../../store";
import DesignEditor from "./DesignEditor";
import Footer from "./Footer";
import KitEditor from "./KitEditor";
import Navbar from "./Navbar";
import TypeEditor from "./TypeEditor";

export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

const Home: FC = ({}) => {
  return <div>Home</div>;
};

const KitRoute: FC = () => {
  let { uuid } = useParams();
  // let [searchParams, setSearchParams] = useSearchParams();

  return (
    <KitScopeProvider uuid={uuid}>
      <Outlet />
    </KitScopeProvider>
  );
};

const DesignRoute: FC = () => {
  let { uuid } = useParams();
  // let [searchParams, setSearchParams] = useSearchParams();

  return (
    <DesignScopeProvider uuid={uuid}>
      <Outlet />
    </DesignScopeProvider>
  );
};

const TypeRoute: FC = () => {
  let { uuid } = useParams();
  // let [searchParams, setSearchParams] = useSearchParams();

  return (
    <DesignScopeProvider uuid={uuid}>
      <Outlet />
    </DesignScopeProvider>
  );
};

const SketchpadBase: FC = () => {
  const layout = useLayout();

  return (
    <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
      <Navbar />
      <Outlet />
      <Footer />
    </div>
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
          <Routes>
            <Route element={<SketchpadBase />}>
              <Route index element={<Home />} />
              <Route path=":uuid" element={<KitRoute />}>
                <Route index element={<KitEditor />} />
                <Route path="d/:uuid" element={<DesignRoute />}>
                  <Route index element={<DesignEditor />} />
                </Route>
                <Route path="t/:uuid" element={<TypeRoute />}>
                  <Route index element={<TypeEditor />} />
                </Route>
              </Route>
            </Route>
          </Routes>
        </MemoryRouter>
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;
