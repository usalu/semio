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

import { FC, ReactNode, useEffect, useRef, useState } from "react";
import { createRoot, Root } from "react-dom/client";
import { useLocation, useNavigate, MemoryRouter, Router, Navigation } from "react-router";
import { ReactFlowProvider } from "@xyflow/react";
import { AppWindowConfig, WindowTypeDefinition } from "./Canvas";
import { useCanvasContext } from "./Canvas";
import { SketchpadScopeProvider, useSketchpadScope } from "./store";
import { DesignScopeProvider, KitScopeProvider, useDesignScope, useKitScope } from "./kits/store";

interface GoldenLayoutCanvasProps {
  windowConfig: AppWindowConfig;
  layoutState?: any;
  onLayoutChange?: (config: any) => void;
  className?: string;
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
      dimensions: config.dimensions,
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
      };
    }
    // Otherwise just add settings
    return {
      ...config,
      settings: {
        ...config.settings,
        hasHeaders: true,
      },
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
  const { setWindowError, setWindowLoading } = useCanvasContext();
  const [hoveredSplitter, setHoveredSplitter] = useState<{ element: HTMLElement; direction: "horizontal" | "vertical" } | null>(null);
  const [goldenLayoutLoaded, setGoldenLayoutLoaded] = useState(false);
  const sketchpadScope = useSketchpadScope();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const location = useLocation();
  const navigate = useNavigate();
  const contextRef = useRef({ sketchpadScope, kitScope, designScope, location, navigate });
  
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
        } catch (error) {
          console.error("[DEBUG] Error initializing golden-layout:", error);
          if (error instanceof Error && error.message.includes("trimStart")) {
            console.error("[DEBUG] Config that caused trimStart error:", JSON.stringify(config, null, 2));
          }
          return;
        }

        handleSplitterHover = (e: MouseEvent) => {
          const target = e.target as HTMLElement;
          const splitter = target.closest(".lm_splitter") as HTMLElement;
          if (splitter) {
            const isHorizontal = splitter.classList.contains("lm_splitter_horizontal");
            setHoveredSplitter({ element: splitter, direction: isHorizontal ? "horizontal" : "vertical" });
          } else {
            setHoveredSplitter(null);
          }
        };

        handleSplitterLeave = (e: MouseEvent) => {
          const target = e.target as HTMLElement;
          const splitter = target.closest(".lm_splitter");
          if (!splitter || !splitter.contains(e.relatedTarget as Node)) {
            setHoveredSplitter(null);
          }
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
      if (containerRef.current && handleSplitterHover && handleSplitterLeave) {
        containerRef.current.removeEventListener("mouseover", handleSplitterHover);
        containerRef.current.removeEventListener("mouseout", handleSplitterLeave);
      }
      const rootsToUnmount = Array.from(componentRootsRef.current.values());
      componentRootsRef.current.clear();
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
    if (!layoutRef.current || !hoveredSplitter) return;

    const windowType = windowConfig.windowTypes.find((wt) => wt.id === windowTypeId);
    if (!windowType) return;

    const newItemConfig = {
      type: "component",
      componentName: windowTypeId,
      title: windowType.label,
    };

    try {
      const splitter = hoveredSplitter.element;
      const splitterParent = splitter.parentElement;
      if (!splitterParent) return;

      const splitterParentParent = splitterParent.parentElement;
      if (!splitterParentParent) return;

      const contentItems = layoutRef.current.root.getItemsByFilter((item) => {
        const itemEl = item.element[0];
        return itemEl && (splitterParentParent.contains(itemEl) || itemEl === splitterParentParent);
      });

      const adjacentItem = contentItems.find((item) => {
        const itemEl = item.element[0];
        return itemEl && splitterParentParent.contains(itemEl) && (itemEl.nextElementSibling === splitter || itemEl.previousElementSibling === splitter);
      });

      if (adjacentItem && adjacentItem.parent) {
        const parent = adjacentItem.parent;
        const isRow = parent.type === "row";
        const itemIndex = parent.contentItems.indexOf(adjacentItem);

        if (direction === "horizontal" && !isRow) {
          const newRow = layoutRef.current.createContentItem(
            {
              type: "row",
              content: [adjacentItem.config, newItemConfig],
            },
            parent
          );
          parent.replaceChild(adjacentItem, newRow);
        } else if (direction === "vertical" && isRow) {
          const newColumn = layoutRef.current.createContentItem(
            {
              type: "column",
              content: [adjacentItem.config, newItemConfig],
            },
            parent
          );
          parent.replaceChild(adjacentItem, newColumn);
        } else {
          parent.addChild(newItemConfig, itemIndex + 1);
        }
      } else {
        const rootContent = layoutRef.current.root.contentItems[0];
        if (rootContent) {
          if (direction === "horizontal" && rootContent.type !== "row") {
            const newRow = layoutRef.current.createContentItem(
              {
                type: "row",
                content: [rootContent.config, newItemConfig],
              },
              layoutRef.current.root
            );
            layoutRef.current.root.replaceChild(rootContent, newRow);
          } else if (direction === "vertical" && rootContent.type === "row") {
            const newColumn = layoutRef.current.createContentItem(
              {
                type: "column",
                content: [rootContent.config, newItemConfig],
              },
              layoutRef.current.root
            );
            layoutRef.current.root.replaceChild(rootContent, newColumn);
          } else {
            rootContent.addChild(newItemConfig);
          }
        }
      }
    } catch (error) {
      console.error("[DEBUG] Error adding window:", error);
    }

    setHoveredSplitter(null);
  };

  return (
    <div className={`relative h-full w-full ${className}`} style={{ zIndex: 0 }}>
      <div ref={containerRef} className="h-full w-full" style={{ zIndex: 0 }} />
      {hoveredSplitter && (
        <div
          className="absolute flex gap-1 p-1 border"
          style={{
            left: hoveredSplitter.element.getBoundingClientRect().left + hoveredSplitter.element.getBoundingClientRect().width / 2 - 60,
            top: hoveredSplitter.element.getBoundingClientRect().top + hoveredSplitter.element.getBoundingClientRect().height / 2 - 20,
            zIndex: 100,
            backgroundColor: "var(--temporary)",
            borderColor: "var(--border-color)",
          }}
        >
          {windowConfig.windowTypes.map((windowType) => (
            <button
              key={windowType.id}
              className="px-2 py-1 text-xs border cursor-pointer"
              style={{
                backgroundColor: "var(--active-base)",
                color: "var(--active-foreground)",
                borderColor: "var(--border-color)",
              }}
              onMouseEnter={(e) => {
                (e.currentTarget as HTMLElement).style.backgroundColor = "var(--hover-temporary)";
              }}
              onMouseLeave={(e) => {
                (e.currentTarget as HTMLElement).style.backgroundColor = "var(--active-base)";
              }}
              onClick={() => handleAddWindow(windowType.id, hoveredSplitter.direction)}
              title={windowType.label}
            >
              {windowType.icon || windowType.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

export default GoldenLayoutCanvas;

