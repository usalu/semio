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
import { DndContext, DragEndEvent, DragOverlay, DragStartEvent } from "@dnd-kit/core";
import { createContext, FC, ReactNode, useContext, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { MemoryRouter, Outlet, Route, Routes, useParams } from "react-router";
import { TooltipProvider } from "../elements/display/Tooltip";

import { Design, Type } from "../../../semio";
import {
  DesignScopeProvider,
  KitScopeProvider,
  Layout,
  SketchpadScopeProvider,
  Theme,
  TypeScopeProvider,
  useEditorPanelVisibility,
  useEditorType,
  useIsMobile,
  useIsNavbarExpanded,
  useLayout,
  useNavigation,
  useSketchpad,
  useSketchpadCommands,
  useTheme,
  WindowEvents,
  YProviderFactory,
} from "../../../store";
import Details from "./panels/Details";
import DesignEditor from "./editors/design/Editor";
import Footer, { FooterItemProvider } from "./Footer";
import Home from "./Home";
import KitEditor from "./editors/kit/Editor";
import Navbar, { PanelSectionProvider } from "./Navbar";
import Chat from "./panels/Chat";
import Workbench, { DesignAvatar, TypeAvatar } from "./panels/Workbench";
import Settings from "./panels/Settings";
import TypeEditor from "./editors/type/Editor";

interface DragDropContextValue {
  activeDraggedType: Type | null;
  activeDraggedDesign: Design | null;
  setActiveDraggedType: (type: Type | null) => void;
  setActiveDraggedDesign: (design: Design | null) => void;
}

const DragDropContext = createContext<DragDropContextValue | null>(null);

export const DragDropProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null);
  const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null);

  return <DragDropContext.Provider value={{ activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign }}>{children}</DragDropContext.Provider>;
};

export const useDragDrop = () => {
  const context = useContext(DragDropContext);
  if (!context) throw new Error("useDragDrop must be used within DragDropProvider");
  return context;
};

export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

const KitRoute: FC = () => {
  let params = useParams();
  const { kit } = params;

  if (!kit) return null;

  return (
    <KitScopeProvider guid={kit}>
      <Outlet />
    </KitScopeProvider>
  );
};

const DesignRoute: FC = () => {
  let { design } = useParams();

  if (!design) return null;

  return (
    <DesignScopeProvider guid={design}>
      <Outlet />
    </DesignScopeProvider>
  );
};

const TypeRoute: FC = () => {
  let { type } = useParams();

  if (!type) return null;

  return (
    <TypeScopeProvider guid={type}>
      <Outlet />
    </TypeScopeProvider>
  );
};

const SketchpadBase: FC = () => {
  const layout = useLayout();
  const theme = useTheme();
  const editorType = useEditorType();
  const visiblePanels = useEditorPanelVisibility();
  const panelSizes = useSketchpad((s) => s.panelSizes);
  const isFullscreen = useSketchpad((s) => s.isFullscreen);
  const isNavbarExpanded = useIsNavbarExpanded();
  const isMobile = useIsMobile();
  const { setTheme, setLayout, setPanelSize, syncNavigation, setIsMobile: updateIsMobile } = useSketchpadCommands();
  const currentPath = useNavigation();
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  // Store the desktop layout preference when not on mobile
  const desktopLayoutRef = useRef<Layout>(layout);
  const navbarRef = useRef<HTMLDivElement>(null);
  const [navbarHeight, setNavbarHeight] = useState(48); // Default 48px (h-12)

  useEffect(() => {
    const checkMobile = () => {
      updateIsMobile(window.innerWidth < 768);
    };
    checkMobile();
    window.addEventListener("resize", checkMobile);
    return () => window.removeEventListener("resize", checkMobile);
  }, [updateIsMobile]);

  // Handle layout switching based on mobile state
  useEffect(() => {
    if (isMobile) {
      // Save current layout before switching to mobile
      if (layout !== Layout.TOUCH) {
        desktopLayoutRef.current = layout;
      }
      // Force touch layout on mobile
      if (layout !== Layout.TOUCH) {
        setLayout(Layout.TOUCH);
      }
    } else {
      // Restore desktop layout when switching back from mobile
      if (layout === Layout.TOUCH && desktopLayoutRef.current !== Layout.TOUCH) {
        setLayout(desktopLayoutRef.current);
      } else if (!isMobile && layout !== Layout.TOUCH) {
        // Update the saved desktop layout when user changes it on desktop
        desktopLayoutRef.current = layout;
      }
    }
  }, [isMobile, layout, setLayout]);

  // Sync React Router location to store navigation
  useEffect(() => {
    syncNavigation(currentPath);
  }, [currentPath, syncNavigation]);

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

  // Track navbar height for layout adjustments
  useEffect(() => {
    if (!navbarRef.current) return;

    const updateHeight = () => {
      if (navbarRef.current) {
        setNavbarHeight(navbarRef.current.offsetHeight);
      }
    };

    // Update height initially and when expanded state changes
    updateHeight();

    // Use ResizeObserver to track height changes
    const observer = new ResizeObserver(updateHeight);
    observer.observe(navbarRef.current);

    return () => observer.disconnect();
  }, [isMobile, isNavbarExpanded]);

  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event;
    const type = active.data.current?.type as Type | undefined;
    const design = active.data.current?.design as Design | undefined;

    if (type) setActiveDraggedType(type);
    if (design) setActiveDraggedDesign(design);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    console.log("[ORIGIN] handleDragEnd called in Sketchpad", event);
    window.dispatchEvent(new CustomEvent("design-drag-end", { detail: event }));
    setActiveDraggedType(null);
    setActiveDraggedDesign(null);
  };

  // Get the single visible panel on mobile
  const mobileVisiblePanel = isMobile ? Object.entries(visiblePanels).find(([_, isVisible]) => isVisible)?.[0] : null;

  return (
    <DndContext onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
      <PanelSectionProvider>
        <FooterItemProvider>
          <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-base text-foreground relative border">
            <div ref={navbarRef} className={`absolute top-0 left-0 right-0 z-50 ${isFullscreen ? "fixed" : ""}`}>
              <Navbar />
            </div>
            <div className="flex-1 flex overflow-hidden relative" style={{ marginTop: isFullscreen ? 0 : `${navbarHeight}px` }}>
              {isMobile ? (
                // Mobile layout: full-screen panel or editor
                <>
                  {mobileVisiblePanel ? (
                    <div className="absolute inset-1 z-30">
                      {mobileVisiblePanel === "workbench" && <Workbench visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                      {mobileVisiblePanel === "details" && <Details visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                      {mobileVisiblePanel === "chat" && <Chat visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                      {mobileVisiblePanel === "settings" && <Settings visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                    </div>
                  ) : (
                    <div className="flex-1 flex flex-col overflow-hidden">
                      <Outlet />
                    </div>
                  )}
                </>
              ) : (
                // Desktop layout: side-by-side panels
                <>
                  {visiblePanels.workbench && (
                    <div className="absolute left-1 top-0 bottom-1 z-20">
                      <Workbench visible={true} width={panelSizes.workbenchWidth} onWidthChange={(w) => setPanelSize("workbenchWidth", w)} />
                    </div>
                  )}
                  <div className="flex-1 flex flex-col overflow-hidden">
                    <Outlet />
                  </div>
                  {(visiblePanels.details || visiblePanels.chat || visiblePanels.settings) && (
                    <div className="absolute right-1 top-0 bottom-1 z-20 flex">
                      {visiblePanels.details && <Details visible={true} width={panelSizes.detailsWidth} onWidthChange={(w) => setPanelSize("detailsWidth", w)} />}
                      {visiblePanels.chat && <Chat visible={true} width={panelSizes.chatWidth} onWidthChange={(w) => setPanelSize("chatWidth", w)} />}
                      {visiblePanels.settings && <Settings visible={true} width={panelSizes.settingsWidth} onWidthChange={(w) => setPanelSize("settingsWidth", w)} />}
                    </div>
                  )}
                </>
              )}
            </div>
            <div className={`absolute bottom-0 left-0 right-0 z-50 ${isFullscreen ? "fixed" : ""}`}>
              <Footer />
            </div>
          </div>
        </FooterItemProvider>
      </PanelSectionProvider>
      {createPortal(
        <DragOverlay>
          {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
          {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
        </DragOverlay>,
        document.body,
      )}
    </DndContext>
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
        <DragDropProvider>
          <MemoryRouter>
            <Routes>
              <Route element={<SketchpadBase />}>
                <Route index element={<Home />} />
                <Route path="kits" element={<Home />} />
                <Route path="kits/:kit" element={<KitRoute />}>
                  <Route index element={<KitEditor />} />
                  <Route path="designs/:design" element={<DesignRoute />}>
                    <Route index element={<DesignEditor />} />
                  </Route>
                  <Route path="types/:type" element={<TypeRoute />}>
                    <Route index element={<TypeEditor />} />
                  </Route>
                </Route>
              </Route>
            </Routes>
          </MemoryRouter>
        </DragDropProvider>
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

export default Sketchpad;
