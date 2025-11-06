// #region Header

// GoldenLayoutCanvas.tsx

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

import { FC, ReactNode, useCallback, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { createRoot, Root } from "react-dom/client";
import { useLocation, useNavigate, MemoryRouter, Router, Navigation } from "react-router";
import { ReactFlowProvider } from "@xyflow/react";
import { AppWindowConfig, WindowTypeDefinition } from "./Canvas";
import { useCanvasContext } from "./Canvas";
import { SketchpadScopeProvider, useSketchpadScope } from "./store";
import { DesignScopeProvider, KitScopeProvider, useDesignScope, useKitScope } from "./kits/store";
import { Action } from "../elements/input/Action";
import * as React from "react";

interface GoldenLayoutCanvasProps {
  windowConfig: AppWindowConfig;
  layoutState?: any;
  onLayoutChange?: (config: any) => void;
  className?: string;
}

function normalizeDimensions(dimensions: any): any {
  if (!dimensions || typeof dimensions !== "object") return dimensions;

  const normalized = { ...dimensions };

  // Convert numeric dimension values with units to string format
  const dimensionFields = [
    { value: "defaultMinItemHeight", unit: "defaultMinItemHeightUnit" },
    { value: "defaultMinItemWidth", unit: "defaultMinItemWidthUnit" },
  ];

  dimensionFields.forEach(({ value, unit }) => {
    if (normalized[value] !== undefined && normalized[unit]) {
      // Convert to string format that golden-layout expects
      normalized[value] = `${normalized[value]}${normalized[unit]}`;
      // Remove the separate unit field
      delete normalized[unit];
    }
  });

  return normalized;
}

function normalizeGoldenLayoutConfig(config: any, isTopLevel: boolean = false): any {
  if (!config || typeof config !== "object") return config;

  // Handle top-level config with root property (full config from toConfig())
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

  // Handle top-level config with settings/dimensions/header but no root
  if (isTopLevel && (config.settings || config.dimensions || config.header)) {
    // If it has content, normalize the content structure
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
    // Otherwise just add settings
    return {
      ...config,
      settings: {
        ...config.settings,
        hasHeaders: true,
      },
      dimensions: normalizeDimensions(config.dimensions),
    };
  }
  
  // Handle item-level config (component, row, column, stack)
  // Don't process if this looks like a top-level config at item level (shouldn't happen, but guard against recursion)
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
  
  // Convert size/sizeUnit to width/height for GoldenLayout initialization
  // GoldenLayout expects width/height as numbers (percentages), not size/sizeUnit
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

const GoldenLayoutCanvas: FC<GoldenLayoutCanvasProps> = ({ windowConfig, layoutState, onLayoutChange, className = "" }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const layoutRef = useRef<any | null>(null);
  const componentRootsRef = useRef<Map<string, Root>>(new Map());
  const componentPropsRef = useRef<Map<string, any>>(new Map());
  const controlRootsRef = useRef<Map<HTMLElement, Root>>(new Map());
  const observerRef = useRef<MutationObserver | null>(null);
  const { setWindowError, setWindowLoading } = useCanvasContext();
  const [hoveredSplitter, setHoveredSplitter] = useState<{ element: HTMLElement; direction: "horizontal" | "vertical" } | null>(null);
  const [goldenLayoutLoaded, setGoldenLayoutLoaded] = useState(false);
  const hoveredSplitterElementRef = useRef<HTMLElement | null>(null);
  const sketchpadScope = useSketchpadScope();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const location = useLocation();
  const navigate = useNavigate();
  const contextRef = useRef({ sketchpadScope, kitScope, designScope, location, navigate });
  const setActiveSplitter = useCallback(
    (element: HTMLElement, direction: "horizontal" | "vertical") => {
      if (hoveredSplitterElementRef.current && hoveredSplitterElementRef.current !== element) {
        hoveredSplitterElementRef.current.classList.remove("relative", "overflow-visible");
      }
      hoveredSplitterElementRef.current = element;
      element.classList.add("relative", "overflow-visible");
      setHoveredSplitter({ element, direction });
    },
    [setHoveredSplitter],
  );
  const clearHoveredSplitter = useCallback(() => {
    if (hoveredSplitterElementRef.current) {
      hoveredSplitterElementRef.current.classList.remove("relative", "overflow-visible");
      hoveredSplitterElementRef.current = null;
    }
    setHoveredSplitter(null);
  }, [setHoveredSplitter]);
  
  useEffect(() => {
    contextRef.current = { sketchpadScope, kitScope, designScope, location, navigate };
  }, [sketchpadScope, kitScope, designScope, location, navigate]);

  useEffect(() => {
    let isMounted = true;
    let handleSplitterHover: ((e: MouseEvent) => void) | null = null;
    let handleSplitterLeave: ((e: MouseEvent) => void) | null = null;
    
    // Dynamically import golden-layout to handle CommonJS module
    import("golden-layout")
      .then((module: any) => {
        if (!isMounted) return;
        const GoldenLayout = module.default || module.GoldenLayout || module;
        setGoldenLayoutLoaded(true);
        
        if (!containerRef.current) return;

        let config = layoutState || windowConfig.defaultLayout;
        
        // Normalize config: convert size/sizeUnit back to width/height if needed
        config = normalizeGoldenLayoutConfig(config, true);
        
        try {
          // Ensure headers are shown in GoldenLayout
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
          const layout = new GoldenLayout(layoutConfig, containerRef.current);

        windowConfig.windowTypes.forEach((windowType) => {
          layout.registerComponent(windowType.id, (container, componentState) => {
            try {
              const element = container.getElement();
              let domElement: HTMLElement | null = null;
              
              if (element) {
                if (Array.isArray(element) || (element as any).length !== undefined) {
                  domElement = element[0] as HTMLElement;
                } else if (element instanceof HTMLElement) {
                  domElement = element;
                } else if ((element as any).get && typeof (element as any).get === 'function') {
                  domElement = (element as any).get(0) as HTMLElement;
                }
              }
              
              if (!domElement || !(domElement instanceof HTMLElement)) {
                console.error("[DEBUG] Invalid container element for", windowType.id, element);
                return;
              }

              if (!domElement.ownerDocument || !domElement.parentNode) {
                console.error("[DEBUG] Container element not attached to DOM for", windowType.id);
                return;
              }

              const rootId = `${windowType.id}-${Date.now()}-${Math.random()}`;
              let root: Root;
              try {
                root = createRoot(domElement);
              } catch (error) {
                console.error("[DEBUG] Failed to create root for", windowType.id, error, domElement);
                return;
              }
              componentRootsRef.current.set(rootId, root);
              componentPropsRef.current.set(rootId, componentState || {});

              const Component = windowType.component;
              const props = componentState || {};
              
              const ctx = contextRef.current;
              
              console.log("[DEBUG] Rendering component", windowType.id, "with props:", props, "location:", ctx.location.pathname, "has designScope:", !!ctx.designScope, "has kitScope:", !!ctx.kitScope);
              
              let wrappedComponent = <Component {...props} />;
              
              const currentPath = ctx.location.pathname + ctx.location.search + ctx.location.hash;
              
              const routerWrapper = (
                <MemoryRouter initialEntries={[currentPath]} initialIndex={0}>
                  <ReactFlowProvider>
                    {wrappedComponent}
                  </ReactFlowProvider>
                </MemoryRouter>
              );
              
              if (ctx.designScope && ctx.kitScope) {
                wrappedComponent = (
                  <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
                    <KitScopeProvider guid={ctx.kitScope.guid}>
                      <DesignScopeProvider guid={ctx.designScope.guid}>
                        {routerWrapper}
                      </DesignScopeProvider>
                    </KitScopeProvider>
                  </SketchpadScopeProvider>
                );
              } else if (ctx.kitScope) {
                wrappedComponent = (
                  <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
                    <KitScopeProvider guid={ctx.kitScope.guid}>
                      {routerWrapper}
                    </KitScopeProvider>
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
                root.render(wrappedComponent);
                console.log("[DEBUG] Successfully rendered component", windowType.id);
              } catch (error) {
                console.error("[DEBUG] Error rendering component", windowType.id, error);
              }

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

        layout.on("stateChanged", () => {
          if (onLayoutChange) {
            const config = layout.toConfig();
            onLayoutChange(config);
          }
        });

          layout.init();

          layoutRef.current = layout;

          const customizeHeaders = () => {
            if (!containerRef.current || !layoutRef.current) return;
            const headers = containerRef.current.querySelectorAll(".lm_header");
            headers.forEach((header) => {
              const controls = header.querySelector(".lm_controls") as HTMLElement;
              if (!controls) return;
              
              const closeTab = controls.querySelector(".lm_close_tab");
              if (closeTab) {
                closeTab.remove();
              }
              
              const existingPopout = controls.querySelector(".lm_popout");
              const existingMaximise = controls.querySelector(".lm_maximise");
              const existingClose = controls.querySelector(".lm_close");
              
              if (existingPopout || existingMaximise || existingClose) {
                if (controlRootsRef.current.has(controls)) {
                  return;
                }
                
                const findLayoutItem = (element: Element): any => {
                  const stackEl = element.closest(".lm_stack");
                  const componentEl = element.closest(".lm_component");
                  const targetEl = stackEl || componentEl;
                  if (!targetEl) return null;
                  
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
                    return itemEl && (itemEl === targetEl || itemEl.contains(targetEl));
                  });
                };
                
                const layoutItem = findLayoutItem(header);
                if (!layoutItem) return;
                
                controls.innerHTML = "";
                const root = createRoot(controls);
                controlRootsRef.current.set(controls, root);
                
                root.render(
                  <React.Fragment>
                    <Action
                      as="div"
                      className="lm_popout"
                      title="open in new window"
                      level="base"
                      onClick={() => {
                        if (layoutItem.popout) {
                          layoutItem.popout();
                        }
                      }}
                    />
                    <Action
                      as="div"
                      className="lm_maximise"
                      title="maximise"
                      level="base"
                      onClick={() => {
                        if (layoutItem.toggleMaximise) {
                          layoutItem.toggleMaximise();
                        }
                      }}
                    />
                    <Action
                      as="div"
                      className="lm_close"
                      title="close"
                      level="base"
                      onClick={() => {
                        if (layoutItem.close) {
                          layoutItem.close();
                        }
                      }}
                    />
                  </React.Fragment>
                );
              }
            });
          };
          
          setTimeout(customizeHeaders, 0);
          
          const observer = new MutationObserver(() => {
            customizeHeaders();
          });
          observerRef.current = observer;
          
          if (containerRef.current) {
            observer.observe(containerRef.current, {
              childList: true,
              subtree: true,
            });
          }
          
          layout.on("stateChanged", () => {
            setTimeout(customizeHeaders, 0);
          });
        } catch (error) {
          console.error("[DEBUG] Error initializing golden-layout:", error);
          if (error instanceof Error && error.message.includes("trimStart")) {
            console.error("[DEBUG] Config that caused trimStart error:", JSON.stringify(config, null, 2));
          }
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
      })
      .catch((error) => {
        console.error("[DEBUG] Failed to load golden-layout:", error);
      });

    return () => {
      isMounted = false;
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
    };
  }, []);

  useEffect(() => {
    if (!layoutRef.current || !layoutState || !goldenLayoutLoaded) return;
    const currentConfig = layoutRef.current.toConfig();
    if (JSON.stringify(currentConfig) !== JSON.stringify(layoutState)) {
      import("golden-layout")
        .then((module: any) => {
          const GoldenLayout = module.default || module.GoldenLayout || module;
          const rootsToUnmount = Array.from(componentRootsRef.current.values());
          componentRootsRef.current.clear();
          componentPropsRef.current.clear();
          layoutRef.current?.destroy();
          queueMicrotask(() => {
            rootsToUnmount.forEach((root) => {
              try {
                root.unmount();
              } catch (error) {
                console.error("[DEBUG] Error unmounting root during layout reload:", error);
              }
            });
          });
          
          try {
            const normalizedConfig = normalizeGoldenLayoutConfig(layoutState, true);
            const layoutConfig = {
              ...normalizedConfig,
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
                ...normalizedConfig.settings,
              },
            };
            const layout = new GoldenLayout(layoutConfig, containerRef.current!);
            windowConfig.windowTypes.forEach((windowType) => {
              layout.registerComponent(windowType.id, (container, componentState) => {
                try {
                  const element = container.getElement();
                  let domElement: HTMLElement | null = null;
                
                if (element) {
                  if (Array.isArray(element) || (element as any).length !== undefined) {
                    domElement = element[0] as HTMLElement;
                  } else if (element instanceof HTMLElement) {
                    domElement = element;
                  } else if ((element as any).get && typeof (element as any).get === 'function') {
                    domElement = (element as any).get(0) as HTMLElement;
                  }
                }
                
                if (!domElement || !(domElement instanceof HTMLElement)) {
                  console.error("[DEBUG] Invalid container element for", windowType.id, element);
                  return;
                }

                if (!domElement.ownerDocument || !domElement.parentNode) {
                  console.error("[DEBUG] Container element not attached to DOM for", windowType.id);
                  return;
                }

                const rootId = `${windowType.id}-${Date.now()}-${Math.random()}`;
                let root: Root;
                try {
                  root = createRoot(domElement);
                } catch (error) {
                  console.error("[DEBUG] Failed to create root for", windowType.id, error, domElement);
                  return;
                }
                componentRootsRef.current.set(rootId, root);
                componentPropsRef.current.set(rootId, componentState || {});

                const Component = windowType.component;
                const props = componentState || {};
                
                const ctx = contextRef.current;
                let wrappedComponent = <Component {...props} />;
                
                const navigator: Navigation = {
                  createHref: (to: any) => {
                    if (typeof to === "string") return to;
                    const pathname = to.pathname || ctx.location.pathname;
                    const search = to.search || ctx.location.search;
                    const hash = to.hash || ctx.location.hash;
                    return pathname + search + hash;
                  },
                  encodeLocation: (to: any) => {
                    if (typeof to === "string") {
                      const url = new URL(to, "http://localhost");
                      return { pathname: url.pathname, search: url.search, hash: url.hash };
                    }
                    return {
                      pathname: to.pathname || ctx.location.pathname,
                      search: to.search || ctx.location.search,
                      hash: to.hash || ctx.location.hash,
                    };
                  },
                  go: (delta: number) => {
                    window.history.go(delta);
                  },
                  push: (to: any, state?: any) => {
                    ctx.navigate(to, { state, replace: false });
                  },
                  replace: (to: any, state?: any) => {
                    ctx.navigate(to, { state, replace: true });
                  },
                  createKey: () => String(Date.now()),
                  state: ctx.location.state,
                };
                
                const navigationType = (ctx.location.state as any)?.navigationType || 
                                       (ctx.location.key ? "POP" : "PUSH");
                
                const routerWrapper = (
                  <Router location={ctx.location} navigationType={navigationType} navigator={navigator}>
                    <ReactFlowProvider>
                      {wrappedComponent}
                    </ReactFlowProvider>
                  </Router>
                );
                
                if (ctx.designScope && ctx.kitScope) {
                  wrappedComponent = (
                    <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
                      <KitScopeProvider guid={ctx.kitScope.guid}>
                        <DesignScopeProvider guid={ctx.designScope.guid}>
                          {routerWrapper}
                        </DesignScopeProvider>
                      </KitScopeProvider>
                    </SketchpadScopeProvider>
                  );
                } else if (ctx.kitScope) {
                  wrappedComponent = (
                    <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
                      <KitScopeProvider guid={ctx.kitScope.guid}>
                        {routerWrapper}
                      </KitScopeProvider>
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
                
                root.render(wrappedComponent);

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
            layout.init();
            layoutRef.current = layout;

            const customizeHeaders = () => {
              if (!containerRef.current || !layoutRef.current) return;
              const headers = containerRef.current.querySelectorAll(".lm_header");
              headers.forEach((header) => {
                const controls = header.querySelector(".lm_controls") as HTMLElement;
                if (!controls) return;
                
                const closeTab = controls.querySelector(".lm_close_tab");
                if (closeTab) {
                  closeTab.remove();
                }
                
                const existingPopout = controls.querySelector(".lm_popout");
                const existingMaximise = controls.querySelector(".lm_maximise");
                const existingClose = controls.querySelector(".lm_close");
                
                if (existingPopout || existingMaximise || existingClose) {
                  if (controlRootsRef.current.has(controls)) {
                    return;
                  }
                  
                  const findLayoutItem = (element: Element): any => {
                    const stackEl = element.closest(".lm_stack");
                    const componentEl = element.closest(".lm_component");
                    const targetEl = stackEl || componentEl;
                    if (!targetEl) return null;
                    
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
                      return itemEl && (itemEl === targetEl || itemEl.contains(targetEl));
                    });
                  };
                  
                  const layoutItem = findLayoutItem(header);
                  if (!layoutItem) return;
                  
                  controls.innerHTML = "";
                  const root = createRoot(controls);
                  controlRootsRef.current.set(controls, root);
                  
                  root.render(
                    <React.Fragment>
                      <Action
                        as="div"
                        className="lm_popout"
                        title="open in new window"
                        level="base"
                        onClick={() => {
                          if (layoutItem.popout) {
                            layoutItem.popout();
                          }
                        }}
                      />
                      <Action
                        as="div"
                        className="lm_maximise"
                        title="maximise"
                        level="base"
                        onClick={() => {
                          if (layoutItem.toggleMaximise) {
                            layoutItem.toggleMaximise();
                          }
                        }}
                      />
                      <Action
                        as="div"
                        className="lm_close"
                        title="close"
                        level="base"
                        onClick={() => {
                          if (layoutItem.close) {
                            layoutItem.close();
                          }
                        }}
                      />
                    </React.Fragment>
                  );
                }
              });
            };
            
            setTimeout(customizeHeaders, 0);
            
            if (observerRef.current) {
              observerRef.current.disconnect();
            }
            const observer = new MutationObserver(() => {
              customizeHeaders();
            });
            observerRef.current = observer;
            
            if (containerRef.current) {
              observer.observe(containerRef.current, {
                childList: true,
                subtree: true,
              });
            }
            
            layout.on("stateChanged", () => {
              setTimeout(customizeHeaders, 0);
            });
          } catch (error) {
            console.error("[DEBUG] Error reloading golden-layout:", error);
            if (error instanceof Error && error.message.includes("trimStart")) {
              console.error("[DEBUG] Config that caused trimStart error:", JSON.stringify(layoutState, null, 2));
            }
          }
        })
        .catch((error) => {
          console.error("[DEBUG] Failed to reload golden-layout:", error);
        });
    }
  }, [layoutState, windowConfig.windowTypes, goldenLayoutLoaded]);

  useEffect(() => {
    if (!goldenLayoutLoaded || !layoutRef.current) return;
    queueMicrotask(() => {
      componentRootsRef.current.forEach((root, rootId) => {
        const ctx = contextRef.current;
        const windowTypeId = rootId.split("-")[0];
        const windowType = windowConfig.windowTypes.find((wt) => wt.id === windowTypeId);
        if (!windowType) return;
        
        const Component = windowType.component;
        const props = componentPropsRef.current.get(rootId) || {};
        let wrappedComponent = <Component {...props} />;
        
        const currentPath = ctx.location.pathname + ctx.location.search + ctx.location.hash;
        
        const routerWrapper = (
          <MemoryRouter initialEntries={[currentPath]} initialIndex={0}>
            <ReactFlowProvider>
              {wrappedComponent}
            </ReactFlowProvider>
          </MemoryRouter>
        );
        
        if (ctx.designScope && ctx.kitScope) {
          wrappedComponent = (
            <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
              <KitScopeProvider guid={ctx.kitScope.guid}>
                <DesignScopeProvider guid={ctx.designScope.guid}>
                  {routerWrapper}
                </DesignScopeProvider>
              </KitScopeProvider>
            </SketchpadScopeProvider>
          );
        } else if (ctx.kitScope) {
          wrappedComponent = (
            <SketchpadScopeProvider id={ctx.sketchpadScope?.id} remote={ctx.sketchpadScope?.remote} onWindowEvents={ctx.sketchpadScope?.onWindowEvents}>
              <KitScopeProvider guid={ctx.kitScope.guid}>
                {routerWrapper}
              </KitScopeProvider>
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
          root.render(wrappedComponent);
        } catch (error) {
          console.error("[DEBUG] Error re-rendering component", windowTypeId, error);
        }
      });
    });
  }, [location, goldenLayoutLoaded, windowConfig.windowTypes]);

  const handleAddWindow = (windowTypeId: string, direction: "horizontal" | "vertical") => {
    if (!layoutRef.current || !hoveredSplitterElementRef.current) {
      console.log("[DEBUG] Cannot add window: no layout or hovered splitter");
      return;
    }

    const windowType = windowConfig.windowTypes.find((wt) => wt.id === windowTypeId);
    if (!windowType) {
      console.log("[DEBUG] Cannot add window: window type not found", windowTypeId);
      return;
    }

    const newItemConfig = {
      type: "component",
      componentName: windowTypeId,
      title: windowType.label,
    };

    const splitter = hoveredSplitterElementRef.current;

    try {
      console.log("[DEBUG] Adding window next to splitter", { direction, windowTypeId });

      // Recursively find all items in the layout tree
      const findAllItems = (item: any): any[] => {
        const items = [item];
        if (item.contentItems && Array.isArray(item.contentItems)) {
          item.contentItems.forEach((child: any) => {
            items.push(...findAllItems(child));
          });
        }
        return items;
      };

      // Recursively serialize an item to a config
      const itemToConfig = (item: any): any => {
        if (!item) return null;

        console.log("[DEBUG] Serializing item:", {
          type: item.type,
          componentName: item.componentName,
          componentType: item.componentType,
          title: item.title,
          hasConfig: !!item.config,
          configKeys: item.config ? Object.keys(item.config) : []
        });

        // If the item already has a valid config, use it
        if (item.config && typeof item.config === 'object' && item.config.type) {
          console.log("[DEBUG] Using existing config:", item.config);
          return item.config;
        }

        // Build config from the item
        const config: any = {
          type: item.type,
        };

        // Add type-specific properties
        if (item.type === 'component') {
          const componentName = item.componentName || item.componentType || item.config?.componentName || item.config?.componentType;
          const title = item.title || item.config?.title || 'Untitled';

          if (!componentName) {
            console.error("[DEBUG] Component missing componentName:", item);
            return null;
          }

          config.componentName = componentName;
          config.title = title;
          // Include componentState - golden-layout expects this property
          config.componentState = item.config?.componentState || item.componentState || {};

          console.log("[DEBUG] Created component config:", config);
        } else if (item.type === 'stack') {
          config.content = [];
          config.activeItemIndex = item.activeItemIndex || 0;
          // Serialize all children in the stack
          if (item.contentItems && Array.isArray(item.contentItems)) {
            config.content = item.contentItems.map((child: any) => itemToConfig(child)).filter((c: any) => c !== null);
          }
          console.log("[DEBUG] Created stack config with", config.content.length, "children");
        } else if (item.type === 'row' || item.type === 'column') {
          config.content = [];
          // Serialize all children in the row/column
          if (item.contentItems && Array.isArray(item.contentItems)) {
            config.content = item.contentItems.map((child: any) => itemToConfig(child)).filter((c: any) => c !== null);
          }
          console.log("[DEBUG] Created", item.type, "config with", config.content.length, "children");
        }

        // Add width/height if present
        if (item.width !== undefined) config.width = item.width;
        if (item.height !== undefined) config.height = item.height;

        return config;
      };

      // Find the parent container that contains this splitter
      // Splitters exist between items in a row or column container
      let parent = null;
      const allItems = findAllItems(layoutRef.current.root);

      // Look for a row or column that could be the parent of this splitter
      for (const item of allItems) {
        if (item.type === "row" || item.type === "column") {
          const itemEl = item.element?.[0] || item.element;
          if (itemEl && itemEl.contains && itemEl.contains(splitter)) {
            parent = item;
            break;
          }
        }
      }

      if (parent) {
        console.log("[DEBUG] Found parent container", { type: parent.type, numChildren: parent.contentItems?.length });

        // Simply add the config - let GoldenLayout handle item creation
        parent.addChild(newItemConfig);
        console.log("[DEBUG] Added window as sibling in container");
      } else {
        // Fallback: add to root
        console.log("[DEBUG] No parent found, adding to root");
        const rootContent = layoutRef.current.root.contentItems[0];
        if (rootContent) {
          rootContent.addChild(newItemConfig);
        } else {
          console.log("[DEBUG] No root content, cannot add window");
        }
      }
    } catch (error) {
      console.error("[DEBUG] Error adding window:", error);
    }

    clearHoveredSplitter();
  };

  return (
    <div className={`relative z-0 h-full w-full ${className}`}>
      <div ref={containerRef} className="h-full w-full" />
      {hoveredSplitter &&
        createPortal(
          <div className="pointer-events-auto absolute left-1/2 top-1/2 flex -translate-x-1/2 -translate-y-1/2 gap-1 border border-border bg-temporary p-1">
            {windowConfig.windowTypes.map((windowType) => (
              <Action key={windowType.id} level="temporary" onClick={() => handleAddWindow(windowType.id, hoveredSplitter.direction)} id={windowType.id}>
                {windowType.icon ?? windowType.label}
              </Action>
            ))}
          </div>,
          hoveredSplitter.element,
        )}
    </div>
  );
};

export default GoldenLayoutCanvas;

