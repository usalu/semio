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
import { DndContext, DragEndEvent, DragOverlay, DragStartEvent, PointerSensor, TouchSensor, useSensor, useSensors } from "@dnd-kit/core";
import React, { createContext, FC, ReactNode, useContext, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { MemoryRouter, Outlet, Route, Routes, useParams } from "react-router";
import { setTooltipModeProvider, TooltipProvider } from "../elements/display/Tooltip";

import { DraggableAvatar } from "../elements/display/Avatar";
import { Design, Type } from "../semio";
import "./apps";
import { appRegistry } from "./apps";
import { DesignAvatar, TypeAvatar } from "./apps/design/panels/Workbench";
import { HeadingsProvider } from "./apps/docs/mdx-provider";
import { QualityAvatar } from "./apps/quality/panels/Workbench";
import Footer, { FooterItemProvider } from "./Footer";
import Navbar, { FocusProvider, PanelSectionProvider } from "./Navbar";
import Chat from "./panels/Chat";
import Details from "./panels/Details";
import Hud from "./panels/Hud";
import Settings from "./panels/Settings";
import Stats from "./panels/Stats";
import Toolbar from "./panels/Toolbar";
import Tools from "./panels/Tools";
import Workbench from "./panels/Workbench";
import {
  DesignScopeProvider,
  KitScopeProvider,
  Layout,
  Mode,
  SketchpadScopeProvider,
  SketchpadState,
  Theme,
  TypeScopeProvider,
  useAppPanelVisibility,
  useAppType,
  useIsMobile,
  useIsNavbarExpanded,
  useLayout,
  useMode,
  useNavigation,
  useSketchpad,
  useSketchpadCommands,
  useTheme,
  WindowEvents,
  YProviderFactory,
} from "./store";

interface DragDropContextValue {
  activeDraggedType: Type | null;
  activeDraggedDesign: Design | null;
  activeDraggedFunction: { name: string; label: string } | null;
  activeDraggedQuality: any | null;
  setActiveDraggedType: (type: Type | null) => void;
  setActiveDraggedDesign: (design: Design | null) => void;
  setActiveDraggedFunction: (func: { name: string; label: string } | null) => void;
  setActiveDraggedQuality: (quality: any | null) => void;
}

const DragDropContext = createContext<DragDropContextValue | null>(null);

export const DragDropProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null);
  const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null);
  const [activeDraggedFunction, setActiveDraggedFunction] = useState<{ name: string; label: string } | null>(null);
  const [activeDraggedQuality, setActiveDraggedQuality] = useState<any | null>(null);

  return (
    <DragDropContext.Provider
      value={{
        activeDraggedType,
        activeDraggedDesign,
        activeDraggedFunction,
        activeDraggedQuality,
        setActiveDraggedType,
        setActiveDraggedDesign,
        setActiveDraggedFunction,
        setActiveDraggedQuality,
      }}
    >
      {children}
    </DragDropContext.Provider>
  );
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

const createScopeRoute = (ParamName: string, ScopeProvider?: React.ComponentType<{ guid: string; children: ReactNode }>): FC => {
  const ScopeRoute: FC = () => {
    const params = useParams();
    const guid = params[ParamName];
    if (!guid) return null;
    if (!ScopeProvider) return <Outlet />;
    return (
      <ScopeProvider guid={guid}>
        <Outlet />
      </ScopeProvider>
    );
  };
  return ScopeRoute;
};

const generateRoutes = (): ReactNode[] => {
  const apps = appRegistry.getAllApps();
  const buildRoute = (app: (typeof apps)[0]): ReactNode[] => {
    const { routeSegments, component: AppComponent, additionalPaths } = app;
    const routes: ReactNode[] = [];
    if (routeSegments.length === 0) {
      routes.push(<Route key={app.id} index element={<AppComponent />} />);
      if (additionalPaths) {
        additionalPaths.forEach((path) => {
          routes.push(<Route key={`${app.id}-${path}`} path={path} element={<AppComponent />} />);
        });
      }
      return routes;
    }
    // Build routes from innermost to outermost
    let currentElement: ReactNode;
    const lastSegment = routeSegments[routeSegments.length - 1];

    // For the last segment, place the component directly on it (not as index child)
    if (lastSegment.paramName && lastSegment.scopeProvider) {
      const ScopeRoute = createScopeRoute(lastSegment.paramName, lastSegment.scopeProvider);
      currentElement = (
        <Route key={`${app.id}-${routeSegments.length - 1}`} path={lastSegment.path} element={<ScopeRoute />}>
          <Route key={`${app.id}-content`} index element={<AppComponent />} />
        </Route>
      );
    } else {
      // For wildcard or regular paths, put the component directly on the route
      currentElement = <Route key={`${app.id}-${routeSegments.length - 1}`} path={lastSegment.path} element={<AppComponent />} />;
    }

    // Wrap with parent segments
    for (let i = routeSegments.length - 2; i >= 0; i--) {
      const segment = routeSegments[i];
      const ScopeRoute = segment.paramName && segment.scopeProvider ? createScopeRoute(segment.paramName, segment.scopeProvider) : undefined;
      if (ScopeRoute) {
        currentElement = (
          <Route key={`${app.id}-${i}`} path={segment.path} element={<ScopeRoute />}>
            {currentElement}
          </Route>
        );
      } else {
        currentElement = (
          <Route key={`${app.id}-${i}`} path={segment.path} element={<Outlet />}>
            {currentElement}
          </Route>
        );
      }
    }
    routes.push(currentElement);
    return routes;
  };
  const groupedRoutes: Record<string, ReactNode[]> = {};
  apps.forEach((app) => {
    const depth = app.routeSegments.length;
    if (!groupedRoutes[depth]) groupedRoutes[depth] = [];
    groupedRoutes[depth].push(...buildRoute(app));
  });
  const sortedDepths = Object.keys(groupedRoutes)
    .map(Number)
    .sort((a, b) => b - a);
  const allRoutes: ReactNode[] = [];
  sortedDepths.forEach((depth) => {
    allRoutes.push(...groupedRoutes[depth]);
  });
  return allRoutes;
};

const SketchpadBase: FC = () => {
  const { kit, design, type: typeParam, quality } = useParams();
  const layout = useLayout();
  const theme = useTheme();
  const appType = useAppType();
  const visiblePanels = useAppPanelVisibility();
  const sketchpad = useSketchpad() as SketchpadState;
  const panelSizes = sketchpad.panelSizes;
  const isFullscreen = sketchpad.isFullscreen;
  const isNavbarExpanded = useIsNavbarExpanded();
  const isMobile = useIsMobile();
  const { setTheme, setLayout, setPanelSize, syncNavigation, setIsMobile: updateIsMobile, setActiveInteraction } = useSketchpadCommands();
  const currentPath = useNavigation();
  const { activeDraggedType, activeDraggedDesign, activeDraggedFunction, activeDraggedQuality, setActiveDraggedType, setActiveDraggedDesign, setActiveDraggedFunction, setActiveDraggedQuality } = useDragDrop();
  const sensors = useSensors(useSensor(PointerSensor, { activationConstraint: { distance: 12 } }), useSensor(TouchSensor));

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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
      // On desktop, always update the saved desktop layout when user changes it
      desktopLayoutRef.current = layout;
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
    const data = active.data.current;
    const type = data?.type as Type | undefined;
    const design = data?.design as Design | undefined;
    const quality = data?.quality;
    // Check if it's a function drag (has name and type is "function")
    const isFunction = data?.name && (data?.type === "function" || data?.type === "quality" || data?.type === "variable" || data?.type === "unit" || data?.type === "value");

    if (type && !isFunction) setActiveDraggedType(type);
    if (design) setActiveDraggedDesign(design);
    if (quality) setActiveDraggedQuality(quality);
    if (isFunction && typeof data.name === "string") {
      setActiveDraggedFunction({ name: data.name, label: data.name });
    }
  };

  const handleDragEnd = (event: DragEndEvent) => {
    window.dispatchEvent(new CustomEvent("design-drag-end", { detail: event }));
    window.dispatchEvent(new CustomEvent("quality-drag-end", { detail: event }));
    setActiveDraggedType(null);
    setActiveDraggedDesign(null);
    setActiveDraggedFunction(null);
    setActiveDraggedQuality(null);
    setActiveInteraction(undefined);
  };

  // Get the single visible panel on mobile
  const mobileVisiblePanel = isMobile ? Object.entries(visiblePanels).find(([_, isVisible]) => isVisible)?.[0] : null;

  return (
    <DndContext sensors={sensors} onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
      <HeadingsProvider>
        <FocusProvider>
          <PanelSectionProvider>
            <FooterItemProvider>
            <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-base text-foreground relative border">
              <div ref={navbarRef} className={`absolute top-0 left-0 right-0 z-50 ${isFullscreen ? "fixed" : ""}`}>
                <Navbar />
              </div>
              <div className="flex-1 flex overflow-hidden relative" style={{ marginTop: isFullscreen ? 0 : `${navbarHeight}px` }}>
                {isMobile ? (
                  <>
                    {mobileVisiblePanel && mobileVisiblePanel !== "toolbar" ? (
                      <div className="absolute inset-1 z-30">
                        {mobileVisiblePanel === "workbench" && <Workbench visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                        {mobileVisiblePanel === "tools" && <Tools visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                        {mobileVisiblePanel === "hud" && <Hud visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                        {mobileVisiblePanel === "stats" && <Stats visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                        {mobileVisiblePanel === "details" && <Details visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                        {mobileVisiblePanel === "chat" && <Chat visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                        {mobileVisiblePanel === "settings" && <Settings visible={true} width={window.innerWidth - 8} onWidthChange={() => {}} />}
                      </div>
                    ) : (
                      <div className="flex-1 flex flex-col overflow-hidden">
                        <Outlet />
                        {visiblePanels.toolbar && (
                          <div className="absolute left-0 right-0 z-20" style={{ bottom: "calc(1.25rem + var(--spacing))" }}>
                            <Toolbar visible={true} leftOffset={0} rightOffset={0} />
                          </div>
                        )}
                      </div>
                    )}
                  </>
                ) : (
                  <>
                    {(visiblePanels.workbench || visiblePanels.tools) && (
                      <div
                        className="absolute left-1 top-1 z-20 flex"
                        style={{
                          bottom: "calc(1.25rem + var(--spacing))",
                        }}
                      >
                        {visiblePanels.workbench && <Workbench visible={true} width={panelSizes.workbenchWidth} onWidthChange={(w) => setPanelSize("workbenchWidth", w)} />}
                        {visiblePanels.tools && <Tools visible={true} width={panelSizes.toolsWidth} onWidthChange={(w) => setPanelSize("toolsWidth", w)} />}
                      </div>
                    )}
                    <div className="flex-1 flex flex-col overflow-hidden">
                      <Outlet />
                      {(visiblePanels.hud || visiblePanels.stats) && (
                        <div
                          className="absolute z-30 flex"
                          style={{
                            left: (visiblePanels.workbench ? panelSizes.workbenchWidth : 0) + (visiblePanels.tools ? panelSizes.toolsWidth : 0) + 4,
                            right: (visiblePanels.details ? panelSizes.detailsWidth : 0) + (visiblePanels.chat ? panelSizes.chatWidth : 0) + (visiblePanels.settings ? panelSizes.settingsWidth : 0) + 4,
                            top: 0,
                            bottom: "calc(1.25rem + var(--spacing))",
                          }}
                        >
                          {visiblePanels.hud && <Hud visible={true} width={panelSizes.hudWidth} onWidthChange={(w) => setPanelSize("hudWidth", w)} />}
                          {visiblePanels.stats && <Stats visible={true} width={panelSizes.statsWidth} onWidthChange={(w) => setPanelSize("statsWidth", w)} />}
                        </div>
                      )}
                      {visiblePanels.toolbar && (
                        <div className="absolute left-0 right-0 z-20" style={{ bottom: "calc(1.25rem + var(--spacing))" }}>
                          <Toolbar
                            visible={true}
                            leftOffset={(visiblePanels.workbench ? panelSizes.workbenchWidth : 0) + (visiblePanels.tools ? panelSizes.toolsWidth : 0) + (visiblePanels.workbench || visiblePanels.tools ? 4 : 0)}
                            rightOffset={
                              (visiblePanels.details ? panelSizes.detailsWidth : 0) +
                              (visiblePanels.chat ? panelSizes.chatWidth : 0) +
                              (visiblePanels.settings ? panelSizes.settingsWidth : 0) +
                              (visiblePanels.details || visiblePanels.chat || visiblePanels.settings ? 4 : 0)
                            }
                          />
                        </div>
                      )}
                    </div>
                    {(visiblePanels.details || visiblePanels.chat || visiblePanels.settings) && (
                      <div
                        className="absolute right-1 top-1 z-20 flex"
                        style={{
                          bottom: "calc(1.25rem + var(--spacing))",
                        }}
                      >
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
      </FocusProvider>
      </HeadingsProvider>
      {createPortal(
        design && kit ? (
          <KitScopeProvider guid={kit}>
            <DesignScopeProvider guid={design}>
              <DragOverlay>
                {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
                {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
                {activeDraggedQuality && <QualityAvatar quality={activeDraggedQuality} />}
                {activeDraggedFunction && <DraggableAvatar content={activeDraggedFunction.name.substring(0, 2).toUpperCase()} />}
              </DragOverlay>
            </DesignScopeProvider>
          </KitScopeProvider>
        ) : typeParam && kit ? (
          <KitScopeProvider guid={kit}>
            <TypeScopeProvider guid={typeParam}>
              <DragOverlay>
                {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
                {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
                {activeDraggedQuality && <QualityAvatar quality={activeDraggedQuality} />}
                {activeDraggedFunction && <DraggableAvatar content={activeDraggedFunction.name.substring(0, 2).toUpperCase()} />}
              </DragOverlay>
            </TypeScopeProvider>
          </KitScopeProvider>
        ) : kit ? (
          <KitScopeProvider guid={kit}>
            <DragOverlay>
              {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
              {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
              {activeDraggedQuality && <QualityAvatar quality={activeDraggedQuality} />}
              {activeDraggedFunction && <DraggableAvatar content={activeDraggedFunction.name.substring(0, 2).toUpperCase()} />}
            </DragOverlay>
          </KitScopeProvider>
        ) : (
          <DragOverlay>
            {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
            {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
            {activeDraggedQuality && <QualityAvatar quality={activeDraggedQuality} />}
            {activeDraggedFunction && <DraggableAvatar content={activeDraggedFunction.name.substring(0, 2).toUpperCase()} />}
          </DragOverlay>
        ),
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
        <TooltipModeProvider>
          <DragDropProvider>
            <MemoryRouter>
              <Routes>
                <Route element={<SketchpadBase />}>{generateRoutes()}</Route>
              </Routes>
            </MemoryRouter>
          </DragDropProvider>
        </TooltipModeProvider>
      </SketchpadScopeProvider>
    </TooltipProvider>
  );
};

const TooltipModeProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const mode = useMode();
  useEffect(() => {
    setTooltipModeProvider(() => mode);
  }, [mode]);
  return <>{children}</>;
};

export default Sketchpad;
