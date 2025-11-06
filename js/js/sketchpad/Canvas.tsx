// #region Header

// Canvas.tsx

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

import { useDroppable } from "@dnd-kit/core";
import { ReactFlowProvider } from "@xyflow/react";
import * as React from "react";
import { createContext, FC, Fragment, memo, ReactNode, useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { createRoot, Root } from "react-dom/client";
import { MemoryRouter, useLocation, useNavigate } from "react-router";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../elements/aggregation/Resizable";
import { Action, ActionDropdown } from "../elements/input/Action";
import BaseWindow, { WindowConfig as BaseWindowConfig } from "../elements/windows/Window";
import { DesignScopeProvider, KitScopeProvider, useDesignScope, useKitScope } from "./kits/store";
import { SketchpadScopeProvider, useSketchpadScope } from "./store";

// Base Canvas context for fullscreen window management
interface CanvasContextValue {
  fullscreenWindow: string | null;
  setFullscreenWindow: (windowId: string | null) => void;
  toggleFullscreenWindow: (windowId: string) => void;
  windowErrors: Map<string, Error>;
  setWindowError: (windowId: string, error: Error | null) => void;
  windowLoadingStates: Map<string, boolean>;
  setWindowLoading: (windowId: string, loading: boolean) => void;
}

const CanvasContext = createContext<CanvasContextValue | null>(null);

export const useCanvasContext = () => {
  const context = useContext(CanvasContext);
  if (!context) throw new Error("useCanvasContext must be used within Canvas");
  return context;
};

export interface WindowConfig extends BaseWindowConfig {
  defaultSize?: number;
}

export interface WindowControl {
  type: "toggle" | "dropdown";
  id: string;
  icon?: ReactNode;
  label?: string;
  tooltip?: string;
  value?: string;
  options?: {
    value: string;
    icon: ReactNode;
    label?: string;
  }[];
  onChange?: (value: string) => void;
}

export interface WindowTypeDefinition {
  id: string;
  label: string;
  icon?: ReactNode;
  component: (props: any) => ReactNode;
  controls?: WindowControl[];
  variants?: {
    id: string;
    label: string;
    icon?: ReactNode;
    componentProps?: any;
  }[];
}

export interface AppWindowConfig {
  windowTypes: WindowTypeDefinition[];
  defaultLayout: any;
}

export const Window = BaseWindow;

interface CanvasProps {
  children: ReactNode;
  className?: string;
}

export const Canvas: FC<CanvasProps> = ({ children, className = "" }) => {
  const [fullscreenWindow, setFullscreenWindow] = useState<string | null>(null);
  const [windowErrors, setWindowErrors] = useState<Map<string, Error>>(new Map());
  const [windowLoadingStates, setWindowLoadingStates] = useState<Map<string, boolean>>(new Map());

  const toggleFullscreenWindow = useCallback((windowId: string) => {
    setFullscreenWindow((current) => (current === windowId ? null : windowId));
  }, []);

  const setWindowError = useCallback((windowId: string, error: Error | null) => {
    setWindowErrors((prev) => {
      const next = new Map(prev);
      if (error) {
        next.set(windowId, error);
      } else {
        next.delete(windowId);
      }
      return next;
    });
  }, []);

  const setWindowLoading = useCallback((windowId: string, loading: boolean) => {
    setWindowLoadingStates((prev) => {
      const next = new Map(prev);
      if (loading) {
        next.set(windowId, loading);
      } else {
        next.delete(windowId);
      }
      return next;
    });
  }, []);

  const contextValue = useMemo(
    () => ({
      fullscreenWindow,
      setFullscreenWindow,
      toggleFullscreenWindow,
      windowErrors,
      setWindowError,
      windowLoadingStates,
      setWindowLoading,
    }),
    [fullscreenWindow, toggleFullscreenWindow, windowErrors, setWindowError, windowLoadingStates, setWindowLoading],
  );

  return (
    <CanvasContext.Provider value={contextValue}>
      <div className={`relative h-full w-full ${className}`}>{children}</div>
    </CanvasContext.Provider>
  );
};

interface HorizontalWindowsProps {
  windows: WindowConfig[];
  handleClassName?: string;
}

export const HorizontalWindows: FC<HorizontalWindowsProps> = memo(({ windows, handleClassName = "border-r" }) => {
  const { fullscreenWindow } = useCanvasContext();
  const visibleWindows = useMemo(() => (fullscreenWindow ? windows.filter((w) => w.id === fullscreenWindow) : windows), [fullscreenWindow, windows]);

  if (visibleWindows.length === 0) return null;
  if (visibleWindows.length === 1) {
    const window = visibleWindows[0];
    return <Window {...window} />;
  }

  return (
    <ResizablePanelGroup direction="horizontal">
      {visibleWindows.map((window, index) => (
        <Fragment key={window.id}>
          <ResizablePanel defaultSize={window.defaultSize ?? 100 / visibleWindows.length} className={fullscreenWindow && fullscreenWindow !== window.id ? "hidden" : "block"}>
            <Window {...window} isVisible={!fullscreenWindow || fullscreenWindow === window.id} />
          </ResizablePanel>
          {index < visibleWindows.length - 1 && <ResizableHandle className={`${handleClassName} ${fullscreenWindow ? "hidden" : "block"}`} />}
        </Fragment>
      ))}
    </ResizablePanelGroup>
  );
});

HorizontalWindows.displayName = "HorizontalWindows";

interface VerticalWindowsProps {
  windows: WindowConfig[];
  handleClassName?: string;
}

export const VerticalWindows: FC<VerticalWindowsProps> = memo(({ windows, handleClassName = "border-b" }) => {
  const { fullscreenWindow } = useCanvasContext();
  const visibleWindows = useMemo(() => (fullscreenWindow ? windows.filter((w) => w.id === fullscreenWindow) : windows), [fullscreenWindow, windows]);

  if (visibleWindows.length === 0) return null;
  if (visibleWindows.length === 1) {
    const window = visibleWindows[0];
    return <Window {...window} />;
  }

  return (
    <ResizablePanelGroup direction="vertical">
      {visibleWindows.map((window, index) => (
        <Fragment key={window.id}>
          <ResizablePanel defaultSize={window.defaultSize ?? 100 / visibleWindows.length} className={fullscreenWindow && fullscreenWindow !== window.id ? "hidden" : "block"}>
            <Window {...window} isVisible={!fullscreenWindow || fullscreenWindow === window.id} />
          </ResizablePanel>
          {index < visibleWindows.length - 1 && <ResizableHandle className={`${handleClassName} ${fullscreenWindow ? "hidden" : "block"}`} />}
        </Fragment>
      ))}
    </ResizablePanelGroup>
  );
});

VerticalWindows.displayName = "VerticalWindows";

// #region LayoutCanvas

function normalizeDimensions(dimensions: any): any {
  if (!dimensions || typeof dimensions !== "object") return dimensions;

  const normalized = { ...dimensions };

  const dimensionFields = [
    { value: "defaultMinItemHeight", unit: "defaultMinItemHeightUnit" },
    { value: "defaultMinItemWidth", unit: "defaultMinItemWidthUnit" },
  ];

  dimensionFields.forEach(({ value, unit }) => {
    if (normalized[value] !== undefined && normalized[unit]) {
      normalized[value] = `${normalized[value]}${normalized[unit]}`;
      delete normalized[unit];
    }
  });

  return normalized;
}

function normalizeGoldenLayoutConfig(config: any, isTopLevel: boolean = false): any {
  if (!config || typeof config !== "object") return config;

  if (config.root) {
    const normalized = normalizeGoldenLayoutConfig(config.root, false);
    return {
      ...normalized,
      settings: {
        ...config.settings,
        hasHeaders: true,
      },
      dimensions: normalizeDimensions(config.dimensions),
      header: config.header,
    };
  }

  if (isTopLevel && (config.settings || config.dimensions || config.header)) {
    if (config.content && Array.isArray(config.content)) {
      return {
        ...config,
        content: config.content.map((item: any) => normalizeGoldenLayoutConfig(item, false)),
        settings: {
          ...config.settings,
          hasHeaders: true,
        },
        dimensions: normalizeDimensions(config.dimensions),
      };
    }
    return {
      ...config,
      settings: {
        ...config.settings,
        hasHeaders: true,
      },
      dimensions: normalizeDimensions(config.dimensions),
    };
  }

  if (!isTopLevel && (config.settings || config.dimensions || config.header) && !config.type) {
    return config;
  }

  const normalized: any = {
    type: config.type,
  };

  if (config.content && Array.isArray(config.content)) {
    normalized.content = config.content.map((item: any) => normalizeGoldenLayoutConfig(item, false));
  }

  if (config.componentName) {
    normalized.componentName = config.componentName;
  }

  if (config.componentType) {
    normalized.componentName = config.componentType;
  }

  if (config.title) {
    normalized.title = config.title;
  }

  if (config.size !== undefined) {
    const sizeValue = typeof config.size === "string" ? parseFloat(config.size) : config.size;
    if (config.type === "row") {
      normalized.height = sizeValue;
    } else if (config.type === "column") {
      normalized.width = sizeValue;
    } else if (config.type === "component" || config.type === "stack") {
      normalized.width = sizeValue;
    }
  } else if (config.width !== undefined) {
    normalized.width = typeof config.width === "string" ? parseFloat(config.width) : config.width;
  } else if (config.height !== undefined) {
    normalized.height = typeof config.height === "string" ? parseFloat(config.height) : config.height;
  }

  return normalized;
}

interface LayoutCanvasProps {
  windowConfig: AppWindowConfig;
  layoutState?: any;
  onLayoutChange?: (config: any) => void;
  activeWindow?: string | null;
  onActiveWindowChange?: (windowId: string | null) => void;
  className?: string;
}

export const LayoutCanvas: FC<LayoutCanvasProps> = ({ windowConfig, layoutState, onLayoutChange, activeWindow, onActiveWindowChange, className = "" }) => {
  console.log("[DEBUG] LayoutCanvas RENDER - component rendering", {
    hasWindowConfig: !!windowConfig,
    hasLayoutState: !!layoutState,
    hasOnLayoutChange: !!onLayoutChange,
  });
  const containerRef = useRef<HTMLDivElement>(null);
  const layoutRef = useRef<any | null>(null);
  const initializingRef = useRef<boolean>(false); // Track initialization synchronously
  const componentRootsRef = useRef<Map<string, Root>>(new Map());
  const componentPropsRef = useRef<Map<string, any>>(new Map());
  const controlRootsRef = useRef<Map<HTMLElement, Root>>(new Map());
  const observerRef = useRef<MutationObserver | null>(null);
  const { setWindowError, setWindowLoading } = useCanvasContext();
  const [hoveredSplitter, setHoveredSplitter] = useState<{ element: HTMLElement; direction: "horizontal" | "vertical" } | null>(null);
  const [layoutLoaded, setLayoutLoaded] = useState(false);
  const hoveredSplitterElementRef = useRef<HTMLElement | null>(null);
  const sketchpadScope = useSketchpadScope();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const location = useLocation();
  const navigate = useNavigate();

  // Stable refs for everything - NEVER trigger re-initialization
  const contextRef = useRef({
    sketchpadScope,
    kitScope,
    designScope,
    location,
    navigate,
  });

  const configRef = useRef(windowConfig);
  const layoutStateRef = useRef(layoutState);
  const onLayoutChangeRef = useRef(onLayoutChange);
  const onActiveWindowChangeRef = useRef(onActiveWindowChange);
  const activeWindowRef = useRef(activeWindow);

  // Make the canvas a droppable zone for window templates
  const { setNodeRef: setDroppableRef, isOver } = useDroppable({
    id: "layout-canvas-drop-zone",
  });

  const setActiveSplitter = useCallback((element: HTMLElement, direction: "horizontal" | "vertical") => {
    if (hoveredSplitterElementRef.current && hoveredSplitterElementRef.current !== element) {
      hoveredSplitterElementRef.current.classList.remove("relative", "overflow-visible");
    }
    hoveredSplitterElementRef.current = element;
    element.classList.add("relative", "overflow-visible");
    setHoveredSplitter({ element, direction });
  }, []);

  const clearHoveredSplitter = useCallback(() => {
    if (hoveredSplitterElementRef.current) {
      hoveredSplitterElementRef.current.classList.remove("relative", "overflow-visible");
      hoveredSplitterElementRef.current = null;
    }
    setHoveredSplitter(null);
  }, []);

  // Update refs when values change - but DON'T trigger re-renders
  useEffect(() => {
    console.log("[DEBUG] Canvas.tsx context refs useEffect triggered", {
      sketchpadScopeId: sketchpadScope?.id,
      kitScopeGuid: kitScope?.guid,
      designScopeGuid: designScope?.guid,
      locationPathname: location.pathname,
    });
    contextRef.current.sketchpadScope = sketchpadScope;
    contextRef.current.kitScope = kitScope;
    contextRef.current.designScope = designScope;
    contextRef.current.location = location;
    contextRef.current.navigate = navigate;
  }, [sketchpadScope, kitScope, designScope, location, navigate]);

  useEffect(() => {
    console.log("[DEBUG] Canvas.tsx config refs useEffect triggered", {
      windowConfigTypesCount: windowConfig?.windowTypes?.length,
      hasLayoutState: !!layoutState,
      hasOnLayoutChange: !!onLayoutChange,
      activeWindow,
    });
    configRef.current = windowConfig;
    layoutStateRef.current = layoutState;
    onLayoutChangeRef.current = onLayoutChange;
    onActiveWindowChangeRef.current = onActiveWindowChange;
    activeWindowRef.current = activeWindow;
  }, [windowConfig, layoutState, onLayoutChange, onActiveWindowChange, activeWindow]);

  // Initialize GoldenLayout - runs ONCE
  useEffect(() => {
    console.log("[DEBUG] Canvas.tsx GoldenLayout initialization useEffect triggered - THIS SHOULD ONLY RUN ONCE!");

    // Prevent double initialization in React StrictMode - use synchronous flag
    if (initializingRef.current || layoutRef.current) {
      console.log("[DEBUG] Canvas.tsx Layout already initialized/initializing, skipping");
      return;
    }

    // Set flag IMMEDIATELY before async operation
    initializingRef.current = true;
    console.log("[DEBUG] Canvas.tsx Set initializingRef.current = true");

    let isMounted = true;
    let handleSplitterHover: ((e: MouseEvent) => void) | null = null;
    let handleSplitterLeave: ((e: MouseEvent) => void) | null = null;

    import("golden-layout")
      .then((module: any) => {
        try {
          console.log("[DEBUG] Canvas.tsx GoldenLayout module loaded");
          console.log("[DEBUG] Canvas.tsx Checking initializingRef.current:", initializingRef.current);
          // Check initializingRef instead of isMounted to survive StrictMode unmount
          if (!initializingRef.current) {
            console.log("[DEBUG] Canvas.tsx initializingRef is false, aborting (StrictMode cleanup)");
            return;
          }
          console.log("[DEBUG] Canvas.tsx Getting GoldenLayout from module");
          const GoldenLayout = module.default || module.GoldenLayout || module;
          console.log("[DEBUG] Canvas.tsx GoldenLayout extracted:", !!GoldenLayout);
          console.log("[DEBUG] Canvas.tsx Calling setLayoutLoaded(true)");
          setLayoutLoaded(true);
          console.log("[DEBUG] Canvas.tsx setLayoutLoaded(true) completed successfully");

          if (!containerRef.current) {
            console.error("[DEBUG] Canvas.tsx containerRef.current is null!");
            return;
          }

          console.log("[DEBUG] Canvas.tsx Getting config", {
            hasLayoutState: !!layoutStateRef.current,
            hasDefaultLayout: !!configRef.current.defaultLayout,
          });
          let config = layoutStateRef.current || configRef.current.defaultLayout;
          console.log("[DEBUG] Canvas.tsx Normalizing config");
          config = normalizeGoldenLayoutConfig(config, true);
          console.log("[DEBUG] Canvas.tsx Config normalized", { configType: config.content?.[0]?.type });

          try {
            const layoutConfig = {
              ...config,
              settings: {
                hasHeaders: true,
                constrainDragToContainer: true,
                reorderEnabled: true,
                popoutWholeStack: false,
                blockedPopoutsThrowError: true,
                closePopoutsOnUnload: true,
                responsiveMode: "none",
                tabOverlapAllowance: 0,
                reorderOnTabMenuClick: true,
                tabControlOffset: 10,
                popInOnClose: false,
                ...config.settings,
              },
            };
            console.log("[DEBUG] Canvas.tsx Creating GoldenLayout instance");
            const layout = new GoldenLayout(layoutConfig, containerRef.current);
            console.log("[DEBUG] Canvas.tsx GoldenLayout instance created successfully");

            console.log("[DEBUG] Canvas.tsx Registering components", { count: configRef.current.windowTypes.length });
            configRef.current.windowTypes.forEach((windowType: WindowTypeDefinition) => {
              console.log("[DEBUG] Canvas.tsx Registering component:", windowType.id);
              layout.registerComponent(windowType.id, (container: any, componentState: any) => {
                console.log("[DEBUG] Canvas.tsx Component factory called for:", windowType.id);
                try {
                  const element = container.getElement();
                  let domElement: HTMLElement | null = null;

                  if (element) {
                    if (Array.isArray(element) || (element as any).length !== undefined) {
                      domElement = element[0] as HTMLElement;
                    } else if (element instanceof HTMLElement) {
                      domElement = element;
                    } else if ((element as any).get && typeof (element as any).get === "function") {
                      domElement = (element as any).get(0) as HTMLElement;
                    }
                  }

                  if (!domElement || !(domElement instanceof HTMLElement)) {
                    return;
                  }

                  if (!domElement.ownerDocument || !domElement.parentNode) {
                    return;
                  }

                  const rootId = `${windowType.id}-${Date.now()}-${Math.random()}`;
                  let root: Root;
                  try {
                    root = createRoot(domElement);
                  } catch (error) {
                    return;
                  }
                  componentRootsRef.current.set(rootId, root);
                  componentPropsRef.current.set(rootId, componentState || {});

                  const renderComponent = () => {
                    console.log("[DEBUG] Canvas.tsx renderComponent() called for", windowType.id);
                    const Component = windowType.component;
                    const props = componentPropsRef.current.get(rootId) || {};

                    const ctx = contextRef.current;
                    let wrappedComponent = <Component {...props} />;

                    const currentPath = ctx.location.pathname + ctx.location.search + ctx.location.hash;

                    const routerWrapper = (
                      <MemoryRouter initialEntries={[currentPath]} initialIndex={0}>
                        <ReactFlowProvider>{wrappedComponent}</ReactFlowProvider>
                      </MemoryRouter>
                    );

                    if (ctx.designScope && ctx.kitScope) {
                      wrappedComponent = (
                        <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
                          <KitScopeProvider guid={ctx.kitScope.guid}>
                            <DesignScopeProvider guid={ctx.designScope.guid}>{routerWrapper}</DesignScopeProvider>
                          </KitScopeProvider>
                        </SketchpadScopeProvider>
                      );
                    } else if (ctx.kitScope) {
                      wrappedComponent = (
                        <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
                          <KitScopeProvider guid={ctx.kitScope.guid}>{routerWrapper}</KitScopeProvider>
                        </SketchpadScopeProvider>
                      );
                    } else if (ctx.sketchpadScope) {
                      wrappedComponent = (
                        <SketchpadScopeProvider id={ctx.sketchpadScope.id} remote={ctx.sketchpadScope.remote} onWindowEvents={ctx.sketchpadScope.onWindowEvents}>
                          {routerWrapper}
                        </SketchpadScopeProvider>
                      );
                    } else {
                      wrappedComponent = routerWrapper;
                    }

                    try {
                      console.log("[DEBUG] Canvas.tsx Rendering component with providers for", windowType.id);
                      root.render(wrappedComponent);
                      console.log("[DEBUG] Canvas.tsx Component rendered successfully for", windowType.id);
                    } catch (error) {
                      console.error("[DEBUG] Error rendering component", windowType.id, error);
                    }
                  };

                  renderComponent();

                  container.on("destroy", () => {
                    const rootToUnmount = root;
                    const rootIdToDelete = rootId;
                    componentRootsRef.current.delete(rootIdToDelete);
                    componentPropsRef.current.delete(rootIdToDelete);
                    queueMicrotask(() => {
                      try {
                        if (rootToUnmount) {
                          rootToUnmount.unmount();
                        }
                      } catch (error) {
                        console.error("[DEBUG] Error unmounting root:", error);
                      }
                    });
                  });
                } catch (error) {
                  console.error("[DEBUG] Error registering component", windowType.id, error);
                }
              });
            });

            let stateChangeCount = 0;
            console.log("[DEBUG] Canvas.tsx Attaching stateChanged event listener");
            layout.on("stateChanged", () => {
              stateChangeCount++;
              console.log("[DEBUG] Canvas.tsx GoldenLayout stateChanged event", {
                count: stateChangeCount,
                hasCallback: !!onLayoutChangeRef.current,
              });
              if (onLayoutChangeRef.current) {
                const config = layout.toConfig();
                onLayoutChangeRef.current(config);
              }
            });

            console.log("[DEBUG] Canvas.tsx Calling layout.init() - THIS IS WHERE IT MIGHT HANG");
            layout.init();
            console.log("[DEBUG] Canvas.tsx layout.init() completed successfully");
            layoutRef.current = layout;

            let customizeHeadersCount = 0;
            const customizeHeaders = () => {
              customizeHeadersCount++;
              console.log("[DEBUG] Canvas.tsx customizeHeaders() called, count:", customizeHeadersCount);
              if (!containerRef.current || !layoutRef.current) {
                console.log("[DEBUG] Canvas.tsx customizeHeaders() early return - no container or layout");
                return;
              }

              // Temporarily disconnect observer to prevent infinite loop
              if (observerRef.current) {
                observerRef.current.disconnect();
              }

              const headers = containerRef.current.querySelectorAll(".lm_header");
              console.log("[DEBUG] Canvas.tsx customizeHeaders() processing", headers.length, "headers");
              headers.forEach((header) => {
                const tabsContainer = header.querySelector(".lm_tabs") as HTMLElement;
                const tabs = header.querySelectorAll(".lm_tab");
                const controls = header.querySelector(".lm_controls") as HTMLElement;
                if (!controls) return;

                tabs.forEach((tab) => {
                  const tabTitle = tab.querySelector(".lm_title");
                  if (!tabTitle) return;

                  const stackEl = tab.closest(".lm_stack");
                  if (!stackEl) return;

                  const findStackItem = (element: Element): any => {
                    const findAllItems = (item: any): any[] => {
                      const items = [item];
                      if (item.contentItems && Array.isArray(item.contentItems)) {
                        item.contentItems.forEach((child: any) => {
                          items.push(...findAllItems(child));
                        });
                      }
                      return items;
                    };

                    const allItems = findAllItems(layoutRef.current.root);
                    return allItems.find((item: any) => {
                      const itemEl = item.element?.[0] || item.element;
                      return itemEl && itemEl === element;
                    });
                  };

                  const stackItem = findStackItem(stackEl);
                  if (!stackItem || !stackItem.contentItems) return;

                  const tabIndex = Array.from(tabs).indexOf(tab);
                  const contentItem = stackItem.contentItems[tabIndex];
                  if (!contentItem) return;

                  const componentName = contentItem.componentName || contentItem.config?.componentName;

                  // Add custom click handler if not already added (without cloning to preserve GoldenLayout handlers)
                  if (!(tab as HTMLElement).hasAttribute("data-tab-customized")) {
                    const handleTabClick = () => {
                      if (onActiveWindowChangeRef.current && componentName) {
                        onActiveWindowChangeRef.current(componentName);
                      }
                    };

                    (tab as HTMLElement).setAttribute("data-tab-customized", "true");
                    (tab as HTMLElement).addEventListener("click", handleTabClick);
                  }

                  // Update active state styling
                  const titleEl = (tab as HTMLElement).querySelector(".lm_title") as HTMLElement;
                  if (titleEl && activeWindowRef.current === componentName) {
                    titleEl.classList.add("bg-active");
                  } else if (titleEl) {
                    titleEl.classList.remove("bg-active");
                  }
                });

                const closeTab = controls.querySelector(".lm_close_tab");
                if (closeTab) {
                  closeTab.remove();
                }

                const existingPopout = controls.querySelector(".lm_popout");
                const existingMaximise = controls.querySelector(".lm_maximise");
                const existingClose = controls.querySelector(".lm_close");

                // Only customize if GoldenLayout buttons exist and not already customized
                if (existingPopout || existingMaximise || existingClose) {
                  // Skip if already customized (check for existing React root)
                  if (controlRootsRef.current.has(controls)) {
                    return;
                  }

                  controls.setAttribute("data-controls-customized", "true");

                  const findLayoutItem = (element: Element): any => {
                    const stackEl = element.closest(".lm_stack");
                    if (!stackEl) return null;

                    const findAllItems = (item: any): any[] => {
                      const items = [];
                      // Only include stacks (which have the methods we need)
                      if (item.type === "stack") {
                        items.push(item);
                      }
                      if (item.contentItems && Array.isArray(item.contentItems)) {
                        item.contentItems.forEach((child: any) => {
                          items.push(...findAllItems(child));
                        });
                      }
                      return items;
                    };

                    const allStacks = findAllItems(layoutRef.current.root);
                    console.log("[DEBUG] Finding stack for element, found stacks:", allStacks.length);

                    const matchedStack = allStacks.find((item: any) => {
                      const itemEl = item.element?.[0] || item.element;
                      const matches = itemEl && itemEl === stackEl;
                      console.log("[DEBUG] Checking stack:", item, "element matches:", matches);
                      return matches;
                    });

                    console.log("[DEBUG] Matched stack:", matchedStack, "has toggleMaximise:", !!matchedStack?.toggleMaximise, "has close:", !!matchedStack?.close);
                    return matchedStack;
                  };

                  const layoutItem = findLayoutItem(header);
                  if (!layoutItem) return;

                  const activeContentItem = layoutItem.getActiveContentItem?.();
                  const componentName = activeContentItem?.componentName || activeContentItem?.config?.componentName;
                  const windowType = configRef.current.windowTypes.find((wt: WindowTypeDefinition) => wt.id === componentName);

                  controls.innerHTML = "";
                  const root = createRoot(controls);
                  controlRootsRef.current.set(controls, root);

                  root.render(
                    <React.Fragment>
                      {windowType?.controls?.map((control) =>
                        control.type === "dropdown" && control.value && control.options ? <ActionDropdown key={control.id} id={control.id} level="base" options={control.options} value={control.value} onValueChange={control.onChange} /> : null,
                      )}
                      <Action
                        as="div"
                        className="lm_maximise"
                        title="maximise"
                        level="base"
                        onClick={(e: React.MouseEvent) => {
                          e.stopPropagation();
                          console.log("[DEBUG] Maximize clicked, layoutItem:", layoutItem, "has toggleMaximise:", !!layoutItem?.toggleMaximise);
                          try {
                            if (layoutItem?.toggleMaximise) {
                              layoutItem.toggleMaximise();
                            }
                          } catch (error) {
                            console.error("[DEBUG] Error toggling maximize:", error);
                          }
                        }}
                      >
                        <span className="sr-only">Maximize</span>
                      </Action>
                      <Action
                        as="div"
                        className="lm_close"
                        title="close"
                        level="base"
                        onClick={(e: React.MouseEvent) => {
                          e.stopPropagation();
                          console.log("[DEBUG] Close clicked, layoutItem:", layoutItem, "has close:", !!layoutItem?.close, "has remove:", !!layoutItem?.remove);
                          try {
                            // Stacks use remove() instead of close()
                            if (layoutItem?.remove) {
                              layoutItem.remove();
                            } else if (layoutItem?.close) {
                              layoutItem.close();
                            }
                          } catch (error) {
                            console.error("[DEBUG] Error closing window:", error);
                          }
                        }}
                      >
                        <span className="sr-only">Close</span>
                      </Action>
                    </React.Fragment>,
                  );
                }
              });

              // Reconnect observer after DOM changes have settled
              if (observerRef.current && containerRef.current) {
                observerRef.current.observe(containerRef.current, {
                  childList: true,
                  subtree: true,
                });
              }
            };

            setTimeout(customizeHeaders, 0);

            let mutationCount = 0;
            const observer = new MutationObserver((mutations) => {
              mutationCount++;
              console.log("[DEBUG] Canvas.tsx MutationObserver fired, count:", mutationCount, "mutations:", mutations.length);
              customizeHeaders();
            });
            observerRef.current = observer;

            if (containerRef.current) {
              observer.observe(containerRef.current, {
                childList: true,
                subtree: true,
              });
            }

            let stateChangedListenerCount = 0;
            layout.on("stateChanged", () => {
              stateChangedListenerCount++;
              console.log("[DEBUG] Canvas.tsx stateChanged listener (for customizeHeaders), count:", stateChangedListenerCount);
              setTimeout(customizeHeaders, 0);
            });
          } catch (error) {
            console.error("[DEBUG] Error initializing golden-layout:", error);
            return;
          }

          handleSplitterHover = (e: MouseEvent) => {
            const target = e.target as HTMLElement;
            const splitter = target.closest(".lm_splitter") as HTMLElement | null;
            if (splitter) {
              const isHorizontal = splitter.classList.contains("lm_splitter_horizontal");
              setActiveSplitter(splitter, isHorizontal ? "horizontal" : "vertical");
            } else {
              clearHoveredSplitter();
            }
          };

          handleSplitterLeave = (e: MouseEvent) => {
            const target = e.target as HTMLElement;
            const relatedTarget = e.relatedTarget as Node | null;
            const splitter = target.closest(".lm_splitter") as HTMLElement | null;
            if (splitter && relatedTarget && splitter.contains(relatedTarget)) {
              return;
            }
            clearHoveredSplitter();
          };

          if (containerRef.current) {
            containerRef.current.addEventListener("mouseover", handleSplitterHover);
            containerRef.current.addEventListener("mouseout", handleSplitterLeave);
          }
        } catch (outerError) {
          console.error("[DEBUG] Outer error in GoldenLayout initialization:", outerError);
        }
      })
      .catch((error) => {
        console.error("[DEBUG] Failed to load golden-layout:", error);
      });

    return () => {
      console.log("[DEBUG] Canvas.tsx GoldenLayout useEffect cleanup");
      isMounted = false;

      // Only actually cleanup if we have a layout (not a StrictMode test unmount)
      if (!layoutRef.current) {
        console.log("[DEBUG] Canvas.tsx No layout to clean up (StrictMode test unmount), keeping initializingRef to block duplicate initialization");
        // DO NOT reset initializingRef here! The async import might still be pending.
        // If we reset it, the second mount will start another import and both will initialize.
        return;
      }

      console.log("[DEBUG] Canvas.tsx Cleaning up actual layout");
      clearHoveredSplitter();
      if (containerRef.current && handleSplitterHover && handleSplitterLeave) {
        containerRef.current.removeEventListener("mouseover", handleSplitterHover);
        containerRef.current.removeEventListener("mouseout", handleSplitterLeave);
      }
      if (observerRef.current) {
        observerRef.current.disconnect();
        observerRef.current = null;
      }
      const rootsToUnmount = Array.from(componentRootsRef.current.values());
      const controlRootsToUnmount = Array.from(controlRootsRef.current.values());
      componentRootsRef.current.clear();
      controlRootsRef.current.clear();
      if (layoutRef.current) {
        layoutRef.current.destroy();
      }
      setTimeout(() => {
        rootsToUnmount.forEach((root) => {
          try {
            root.unmount();
          } catch (error) {
            console.error("[DEBUG] Error unmounting root in cleanup:", error);
          }
        });
        controlRootsToUnmount.forEach((root) => {
          try {
            root.unmount();
          } catch (error) {
            console.error("[DEBUG] Error unmounting control root in cleanup:", error);
          }
        });
      }, 0);
      layoutRef.current = null;
      initializingRef.current = false;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleAddWindow = useCallback(
    (windowTypeId: string, direction: "horizontal" | "vertical") => {
      console.log(
        "[DEBUG] handleAddWindow called with windowTypeId:",
        windowTypeId,
        "available types:",
        configRef.current.windowTypes.map((wt) => wt.id),
      );
      if (!layoutRef.current || !hoveredSplitterElementRef.current) {
        return;
      }

      const windowType = configRef.current.windowTypes.find((wt: WindowTypeDefinition) => wt.id === windowTypeId);
      if (!windowType) {
        console.log("[DEBUG] Window type not found for ID:", windowTypeId);
        return;
      }
      console.log("[DEBUG] Found window type:", windowType.label);

      const newItemConfig = {
        type: "stack",
        content: [
          {
            type: "component",
            componentName: windowTypeId,
            title: windowType.label,
          },
        ],
      };

      const splitter = hoveredSplitterElementRef.current;

      try {
        const findAllItems = (item: any): any[] => {
          const items = [item];
          if (item.contentItems && Array.isArray(item.contentItems)) {
            item.contentItems.forEach((child: any) => {
              items.push(...findAllItems(child));
            });
          }
          return items;
        };

        let parent = null;
        let insertIndex = -1;
        const allItems = findAllItems(layoutRef.current.root);

        console.log("[DEBUG] Looking for parent containing splitter");
        
        // Find the parent row/column that contains the splitter
        for (const item of allItems) {
          if (item.type === "row" || item.type === "column") {
            const itemEl = item.element?.[0] || item.element;
            console.log("[DEBUG] Checking", item.type, "element:", itemEl, "contains splitter:", itemEl?.contains?.(splitter));
            
            if (itemEl && itemEl.contains && itemEl.contains(splitter)) {
              parent = item;
              console.log("[DEBUG] Found parent", item.type, "with", parent.contentItems?.length, "content items");
              
              // The splitter should be a direct child of the itemEl
              // Find the splitter's position among itemEl's children
              if (parent.contentItems && Array.isArray(parent.contentItems)) {
                // Get all direct children of the parent element
                const children = Array.from(itemEl.children || []);
                
                // Find the splitter in the children
                let splitterIndex = -1;
                for (let i = 0; i < children.length; i++) {
                  if (children[i] === splitter) {
                    splitterIndex = i;
                    break;
                  }
                }
                
                console.log("[DEBUG] Splitter DOM index:", splitterIndex, "total children:", children.length);
                console.log("[DEBUG] Children classes:", children.map((c) => (c as HTMLElement).className));
                
                if (splitterIndex === -1) {
                  console.log("[DEBUG] Splitter not found as direct child, searching in nested elements");
                  // The splitter might be inside a wrapper, search recursively
                  for (let i = 0; i < children.length; i++) {
                    const child = children[i] as HTMLElement;
                    if (child.contains && child.contains(splitter)) {
                      splitterIndex = i;
                      console.log("[DEBUG] Found splitter inside child at index", i);
                      break;
                    }
                  }
                }
                
                if (splitterIndex >= 0) {
                  // Count non-splitter elements before this splitter
                  let itemsBeforeSplitter = 0;
                  for (let i = 0; i < splitterIndex; i++) {
                    const child = children[i] as HTMLElement;
                    if (!child.classList.contains('lm_splitter')) {
                      itemsBeforeSplitter++;
                      console.log("[DEBUG] Found item before splitter at DOM index", i);
                    }
                  }
                  
                  // Insert after the items that come before the splitter
                  insertIndex = itemsBeforeSplitter;
                  console.log("[DEBUG] Items before splitter:", itemsBeforeSplitter, "insertIndex:", insertIndex);
                } else {
                  console.log("[DEBUG] Could not determine splitter position");
                }
              }
              
              break;
            }
          }
        }

        console.log("[DEBUG] Final parent:", parent?.type, "insertIndex:", insertIndex, "parent children count:", parent?.contentItems?.length);

        if (parent && insertIndex >= 0) {
          layoutRef.current.root.addItem(newItemConfig, parent, insertIndex);
        } else if (parent) {
          layoutRef.current.root.addItem(newItemConfig, parent);
        } else {
          layoutRef.current.root.addItem(newItemConfig);
        }
      } catch (error) {
        console.error("[DEBUG] Error adding window:", error);
      }

      clearHoveredSplitter();
    },
    [clearHoveredSplitter],
  );

  return (
    <div ref={setDroppableRef} className={`relative z-0 h-full w-full ${className} ${isOver ? "ring-2 ring-primary" : ""}`}>
      <div ref={containerRef} className="h-full w-full" />
      {hoveredSplitter &&
        createPortal(
          <div className="pointer-events-auto absolute left-1/2 top-1/2 flex flex-row -translate-x-1/2 -translate-y-1/2 gap-1 border border-border bg-temporary p-1">
            {configRef.current.windowTypes.map((windowType: WindowTypeDefinition, index: number) => {
              const typeId = windowType.id;
              const direction = hoveredSplitter.direction;
              console.log(`[DEBUG] Rendering splitter button #${index} - typeId:`, typeId, "label:", windowType.label, "direction:", direction);
              
              return (
                <button
                  key={typeId}
                  type="button"
                  className="border border-border bg-panel p-1 text-xs hover:bg-hover-panel"
                  onClick={(e: React.MouseEvent) => {
                    e.stopPropagation();
                    console.log(`[DEBUG] Button #${index} onClick fired - typeId:`, typeId, "direction:", direction);
                    handleAddWindow(typeId, direction);
                  }}
                  title={windowType.label}
                >
                  {windowType.label}
                </button>
              );
            })}
          </div>,
          hoveredSplitter.element,
        )}
    </div>
  );
};

// #endregion
