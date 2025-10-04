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
import { FC, useEffect } from "react";
import { MemoryRouter, Outlet, Route, Routes, useParams } from "react-router";
import { TooltipProvider } from "../Tooltip";

import { DesignScopeProvider, KitScopeProvider, Layout, SketchpadScopeProvider, Theme, useEditorType, useLayout, useSketchpad, useSketchpadCommands, useTheme, WindowEvents, YProviderFactory } from "../../../store";
import { TreeItem } from "../Tree";
import Chat from "./Chat";
import Console from "./Console";
import DesignEditor from "./DesignEditor";
import Details from "./Details";
import Footer from "./Footer";
import KitEditor from "./KitEditor";
import Navbar from "./Navbar";
import { PanelSectionProvider, useAddPanelSection, useRemovePanelSection } from "./PanelSectionContext";
import Settings from "./Settings";
import TypeEditor from "./TypeEditor";
import Workbench from "./Workbench";

export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

const Home: FC = ({}) => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  // Add chat section
  useEffect(() => {
    addSection("chat", {
      id: "home-chat",
      label: "Welcome",
      order: 0,
      defaultOpen: true,
      content: (
        <TreeItem>
          <p className="text-sm text-muted-foreground">Welcome to Semio! Start by creating a kit or opening an existing one.</p>
        </TreeItem>
      ),
    });

    return () => {
      removeSection("chat", "home-chat");
    };
  }, [addSection, removeSection]);

  // Add settings section
  useEffect(() => {
    addSection("settings", {
      id: "home-settings",
      label: "Home",
      order: 0,
      defaultOpen: true,
      content: (
        <>
          <TreeItem>Version: 1.0.0</TreeItem>
          <TreeItem>Environment: Development</TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("settings", "home-settings");
    };
  }, [addSection, removeSection]);

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
  const theme = useTheme();
  const editorType = useEditorType();
  const visiblePanels = useSketchpad((s) => s.panelVisibility[editorType]) || {};
  const panelSizes = useSketchpad((s) => s.panelSizes);
  const { setTheme, setLayout, setPanelSize } = useSketchpadCommands();

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

  return (
    <PanelSectionProvider>
      <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
        <Navbar />

        {/* Main content area with panels */}
        <div className="flex-1 flex overflow-hidden relative">
          {/* LEFT PANELS */}
          {visiblePanels.workbench && <Workbench visible={true} width={panelSizes.workbenchWidth} onWidthChange={(w) => setPanelSize("workbenchWidth", w)} />}

          {/* MAIN CONTENT - Outlet renders DesignEditor/TypeEditor/KitEditor/Home */}
          <div className="flex-1 flex flex-col overflow-hidden">
            <Outlet />
          </div>

          {/* RIGHT PANELS (mutually exclusive: details OR chat OR settings) */}
          {(visiblePanels.details || visiblePanels.chat || visiblePanels.settings) && (
            <div className="flex">
              {visiblePanels.details && <Details visible={true} width={panelSizes.detailsWidth} onWidthChange={(w) => setPanelSize("detailsWidth", w)} />}
              {visiblePanels.chat && <Chat visible={true} width={panelSizes.chatWidth} onWidthChange={(w) => setPanelSize("chatWidth", w)} />}
              {visiblePanels.settings && <Settings visible={true} width={panelSizes.settingsWidth} onWidthChange={(w) => setPanelSize("settingsWidth", w)} />}
            </div>
          )}

          {/* BOTTOM PANELS */}
          {visiblePanels.console && <Console visible={true} height={panelSizes.consoleHeight} onHeightChange={(h) => setPanelSize("consoleHeight", h)} />}
        </div>

        <Footer />
      </div>
    </PanelSectionProvider>
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
