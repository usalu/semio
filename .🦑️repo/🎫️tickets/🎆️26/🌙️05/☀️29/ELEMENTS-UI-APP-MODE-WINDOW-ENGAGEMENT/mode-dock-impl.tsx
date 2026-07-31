// #region 🧭️Mode

/** @emoji 🪟️ Window descriptor rendered inside {@link Mode}. */
export interface ModeWindowDescriptor extends Omit<WindowConfig, "children" | "onOpenInNewWindow" | "onMaximize" | "onMinimize" | "onClose"> {
  title?: string;
  children: React.ReactNode;
}

export interface ModeProps {
  windows: ModeWindowDescriptor[];
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string) => void;
  layout?: WindowLayoutNode;
  children?: React.ReactNode;
  className?: string;
}

//#region 🧭️ModeLayoutUtils

type ModeLayoutPath = string;
type ModeDockSide = "left" | "right" | "top" | "bottom";

function modePathSegments(path: ModeLayoutPath): number[] {
  return path ? path.split(".").map((segment) => Number(segment)) : [];
}

function modeJoinPath(parent: ModeLayoutPath, index: number): ModeLayoutPath {
  return parent ? `${parent}.${index}` : String(index);
}

function modeCollectWindowIds(node: WindowLayoutNode): string[] {
  if (node.kind === "window") return [node.id];
  if (node.kind === "stack") return node.children.map((child) => child.id);
  return node.children.flatMap(modeCollectWindowIds);
}

/** @emoji 🪟️ Ensures every window leaf sits inside a tab stack. */
function normalizeLayoutToStacks(node: WindowLayoutNode): WindowLayoutNode {
  if (node.kind === "window") return { kind: "stack", children: [node], activeId: node.id };
  if (node.kind === "stack") return { ...node, activeId: node.activeId ?? node.children[0]?.id };
  return { ...node, children: node.children.map((child) => normalizeLayoutToStacks(child) as WindowLayoutAxisNode | WindowLayoutStackNode) };
}

/** @emoji 🪟️ Collapses empty axes and hoists single-child axes. */
function collapseLayout(node: WindowLayoutNode | null): WindowLayoutNode | null {
  if (!node) return null;
  if (node.kind === "window") return node;
  if (node.kind === "stack") return node.children.length === 0 ? null : node;
  const children = node.children.map((child) => collapseLayout(child)).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => child !== null);
  if (children.length === 0) return null;
  if (children.length === 1) {
    const only = children[0]!;
    return { ...only, size: only.size ?? node.size };
  }
  return { ...node, children };
}

function updateLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath, updater: (node: WindowLayoutNode) => WindowLayoutNode): WindowLayoutNode {
  if (!path) return updater(layout);
  const [head, ...rest] = modePathSegments(path);
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return updater(layout);
  const child = layout.children[head!];
  if (!child) return layout;
  if (rest.length === 0) {
    const nextChildren = [...layout.children];
    nextChildren[head!] = updater(child as WindowLayoutNode) as WindowLayoutAxisNode | WindowLayoutStackNode;
    return { ...layout, children: nextChildren };
  }
  const nextChildren = [...layout.children];
  nextChildren[head!] = updateLayoutAtPath(child as WindowLayoutNode, rest.join("."), updater) as WindowLayoutAxisNode | WindowLayoutStackNode;
  return { ...layout, children: nextChildren };
}

function mapLayoutStacks(layout: WindowLayoutNode, mapper: (stack: WindowLayoutStackNode, path: ModeLayoutPath) => WindowLayoutStackNode, path = ""): WindowLayoutNode {
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return mapper(layout, path);
  return {
    ...layout,
    children: layout.children.map((child, index) => mapLayoutStacks(child as WindowLayoutNode, mapper, modeJoinPath(path, index)) as WindowLayoutAxisNode | WindowLayoutStackNode),
  };
}

/** @emoji 🪟️ Adds missing windows and removes stale ones from the layout tree. */
function reconcileWindows(layout: WindowLayoutNode, windowIds: readonly string[]): WindowLayoutNode {
  const normalized = normalizeLayoutToStacks(layout);
  const allowed = new Set(windowIds);
  let result = windowIds.length === 0 ? normalized : removeAbsentWindowsFromLayout(normalized, allowed);
  const present = new Set(modeCollectWindowIds(result));
  const missing = windowIds.filter((id) => !present.has(id));
  if (missing.length === 0) return collapseLayout(result) ?? { kind: "stack", children: [] };
  const newStacks: WindowLayoutStackNode[] = missing.map((id) => ({ kind: "stack", children: [{ kind: "window", id }], activeId: id }));
  if (result.kind === "row" || result.kind === "column") {
    result = { ...result, children: [...result.children, ...newStacks] };
  } else if (result.kind === "stack") {
    if (modeCollectWindowIds(result).length === 0) result = newStacks[0]!;
    else result = { kind: "row", children: [result, ...newStacks] };
  } else {
    result = newStacks[0]!;
  }
  return collapseLayout(result) ?? { kind: "stack", children: [] };
}

function removeAbsentWindowsFromLayout(layout: WindowLayoutNode, allowed: ReadonlySet<string>): WindowLayoutNode {
  if (layout.kind === "window") return allowed.has(layout.id) ? layout : { kind: "stack", children: [] };
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => allowed.has(child.id));
    const activeId = layout.activeId && allowed.has(layout.activeId) ? layout.activeId : children[0]?.id;
    return { ...layout, children, activeId };
  }
  return {
    ...layout,
    children: layout.children.map((child) => removeAbsentWindowsFromLayout(child as WindowLayoutNode, allowed)).filter((child) => child.kind !== "stack" || child.children.length > 0) as (WindowLayoutAxisNode | WindowLayoutStackNode)[],
  };
}

/** @emoji 🪟️ Removes a window from the layout tree and collapses empty nodes. */
function removeWindowFromLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode | null {
  if (layout.kind === "window") return layout.id === windowId ? null : layout;
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => child.id !== windowId);
    if (children.length === 0) return null;
    const activeId = layout.activeId === windowId ? children[0]?.id : layout.activeId;
    return { ...layout, children, activeId };
  }
  const children = layout.children.map((child) => removeWindowFromLayout(child as WindowLayoutNode, windowId)).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => child !== null);
  if (children.length === 0) return null;
  return collapseLayout({ ...layout, children });
}

function insertWindowAsTab(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, index?: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const insertAt = index === undefined || index < 0 ? children.length : index;
    children.splice(insertAt, 0, { kind: "window", id: windowId });
    return { ...node, children, activeId: windowId };
  });
}

function reorderTabInStack(layout: WindowLayoutNode, stackPath: ModeLayoutPath, fromIndex: number, toIndex: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const [item] = children.splice(fromIndex, 1);
    if (!item) return node;
    children.splice(toIndex, 0, item);
    return { ...node, children };
  });
}

/** @emoji 🪟️ Splits a stack with a dragged window on the given side. */
function splitWithWindow(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  return updateLayoutAtPath(without, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
    const horizontal = side === "left" || side === "right";
    const children = side === "left" || side === "top" ? [incoming, node] : [node, incoming];
    return { kind: horizontal ? "row" : "column", children, size: node.size } as WindowLayoutAxisNode;
  });
}

function splitRootWithWindow(layout: WindowLayoutNode, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
  const horizontal = side === "left" || side === "right";
  const children = side === "left" || side === "top" ? [incoming, without] : [without, incoming];
  return { kind: horizontal ? "row" : "column", children: children as (WindowLayoutAxisNode | WindowLayoutStackNode)[] };
}

/** @emoji 🪟️ Writes resizable panel percentages back onto axis children. */
function applyAxisSizes(layout: WindowLayoutNode, axisPath: ModeLayoutPath, sizes: Record<string, number>): WindowLayoutNode {
  return updateLayoutAtPath(layout, axisPath, (node) => {
    if (node.kind !== "row" && node.kind !== "column") return node;
    const children = node.children.map((child, index) => {
      const panelKey = modeJoinPath(axisPath, index);
      const size = sizes[panelKey] ?? sizes[String(index)] ?? child.size;
      return { ...child, size };
    });
    return { ...node, children };
  });
}

function setActiveWindowInLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode {
  return mapLayoutStacks(layout, (stack) => {
    if (!stack.children.some((child) => child.id === windowId)) return stack;
    return { ...stack, activeId: windowId };
  });
}

function resolveModeLayout(windows: readonly ModeWindowDescriptor[], layout?: WindowLayoutNode): WindowLayoutNode {
  const base = layout ?? createEvenWindowLayout(windows.map((window) => window.id));
  return reconcileWindows(
    base,
    windows.map((window) => window.id),
  );
}

//#endregion 🧭️ModeLayoutUtils

//#region 🧭️ModeDockDrag

type ModeDropZone = { kind: "tab"; stackPath: ModeLayoutPath; index: number } | { kind: "split"; stackPath: ModeLayoutPath; side: ModeDockSide } | { kind: "root-split"; side: ModeDockSide };

interface ModeDragState {
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  x: number;
  y: number;
}

function computeModeDropZone(pointerX: number, pointerY: number, stackRects: ReadonlyMap<ModeLayoutPath, DOMRect>, modeRect: DOMRect | null): ModeDropZone | null {
  for (const [stackPath, rect] of stackRects) {
    if (pointerX < rect.left || pointerX > rect.right || pointerY < rect.top || pointerY > rect.bottom) continue;
    const relX = (pointerX - rect.left) / rect.width;
    const relY = (pointerY - rect.top) / rect.height;
    const edge = 0.25;
    if (relX < edge) return { kind: "split", stackPath, side: "left" };
    if (relX > 1 - edge) return { kind: "split", stackPath, side: "right" };
    if (relY < edge) return { kind: "split", stackPath, side: "top" };
    if (relY > 1 - edge) return { kind: "split", stackPath, side: "bottom" };
    return { kind: "tab", stackPath, index: -1 };
  }
  if (!modeRect) return null;
  const relX = (pointerX - modeRect.left) / modeRect.width;
  const relY = (pointerY - modeRect.top) / modeRect.height;
  const edge = 0.12;
  if (relX < edge) return { kind: "root-split", side: "left" };
  if (relX > 1 - edge) return { kind: "root-split", side: "right" };
  if (relY < edge) return { kind: "root-split", side: "top" };
  if (relY > 1 - edge) return { kind: "root-split", side: "bottom" };
  return null;
}

function applyModeDrop(layout: WindowLayoutNode, drag: ModeDragState, zone: ModeDropZone): WindowLayoutNode {
  const { windowId, stackPath: sourcePath, tabIndex } = drag;
  if (zone.kind === "root-split") return splitRootWithWindow(layout, windowId, zone.side);
  if (zone.kind === "split") {
    if (zone.stackPath === sourcePath) return layout;
    return splitWithWindow(layout, zone.stackPath, windowId, zone.side);
  }
  if (zone.stackPath === sourcePath) {
    const targetIndex = zone.index < 0 ? tabIndex : zone.index;
    if (targetIndex === tabIndex) return layout;
    return reorderTabInStack(layout, sourcePath, tabIndex, targetIndex);
  }
  const without = removeWindowFromLayout(layout, windowId);
  if (!without) return layout;
  return insertWindowAsTab(without, zone.stackPath, windowId, zone.index < 0 ? undefined : zone.index);
}

//#endregion 🧭️ModeDockDrag

//#region 🧭️ModeDockTabBar

interface ModeDockContextValue {
  registerStackRect: (path: ModeLayoutPath, element: HTMLElement | null) => void;
  startTabDrag: (windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => void;
  closeWindow: (windowId: string) => void;
  activateWindow: (windowId: string) => void;
  maximizedStackPath: ModeLayoutPath | null;
  toggleMaximize: (stackPath: ModeLayoutPath) => void;
}

const ModeDockContext = React.createContext<ModeDockContextValue | null>(null);

interface ModeDockTabBarProps {
  stackPath: ModeLayoutPath;
  tabs: readonly { id: string; title: string }[];
  activeId: string | undefined;
  onSelectTab: (windowId: string) => void;
}

const ModeDockTabBar: React.FC<ModeDockTabBarProps> = ({ stackPath, tabs, activeId, onSelectTab }) => {
  const dock = React.useContext(ModeDockContext);
  const isMaximized = dock?.maximizedStackPath === stackPath;

  return (
    <div data-slot="mode-dock-tabbar" className="flex h-medium shrink-0 items-stretch border-b border-element bg-base">
      <div data-slot="mode-dock-tabs" className="flex min-w-0 flex-1 items-stretch overflow-x-auto">
        {tabs.map((tab, index) => (
          <div
            key={tab.id}
            data-slot="mode-dock-tab"
            data-active={activeId === tab.id ? "true" : undefined}
            className={cn(
              "group flex max-w-[12rem] shrink-0 cursor-pointer items-center gap-half border-r border-element px-single text-xs select-none",
              activeId === tab.id ? "bg-window text-foreground" : "bg-base text-muted-foreground hover:bg-hover-window",
            )}
            onPointerDown={(event) => {
              if ((event.target as HTMLElement).closest("[data-slot='mode-dock-tab-close']")) return;
              onSelectTab(tab.id);
            }}
            onPointerDownCapture={(event) => {
              if ((event.target as HTMLElement).closest("[data-slot='mode-dock-tab-close']")) return;
              if (event.button !== 0) return;
              dock?.startTabDrag(tab.id, stackPath, index, tab.title, event);
            }}
          >
            <span className="truncate">{tab.title}</span>
            <button
              type="button"
              data-slot="mode-dock-tab-close"
              className="ml-auto flex size-small shrink-0 items-center justify-center rounded opacity-60 hover:bg-hover-window hover:opacity-100"
              onPointerDown={(event) => event.stopPropagation()}
              onClick={(event) => {
                event.stopPropagation();
                dock?.closeWindow(tab.id);
              }}
            >
              <CloseIcon className="size-tiny" />
            </button>
          </div>
        ))}
      </div>
      <div data-slot="mode-dock-stack-controls" className="flex shrink-0 items-stretch border-l border-element">
        <button type="button" data-slot="mode-dock-maximize" className="flex size-medium items-center justify-center hover:bg-hover-window" onClick={() => dock?.toggleMaximize(stackPath)}>
          {isMaximized ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />}
        </button>
      </div>
    </div>
  );
};

//#endregion 🧭️ModeDockTabBar

//#region 🧭️ModeDockStack

interface ModeDockStackProps {
  stackPath: ModeLayoutPath;
  node: WindowLayoutStackNode;
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
}

const ModeDockStack: React.FC<ModeDockStackProps> = ({ stackPath, node, windowsById, activeWindowId }) => {
  const dock = React.useContext(ModeDockContext);
  const stackRef = React.useRef<HTMLDivElement>(null);
  const activeId = node.activeId ?? node.children[0]?.id;
  const tabs = node.children.map((child) => ({
    id: child.id,
    title: child.title ?? windowsById.get(child.id)?.title ?? child.id,
  }));

  React.useLayoutEffect(() => {
    dock?.registerStackRect(stackPath, stackRef.current);
    return () => dock?.registerStackRect(stackPath, null);
  }, [dock, stackPath, node.children.length]);

  const activeDescriptor = activeId ? windowsById.get(activeId) : undefined;

  return (
    <div ref={stackRef} data-slot="mode-dock-stack" data-stack-path={stackPath} className="flex h-full min-h-0 w-full min-w-0 flex-col overflow-hidden">
      <ModeDockTabBar stackPath={stackPath} tabs={tabs} activeId={activeId} onSelectTab={(windowId) => dock?.activateWindow(windowId)} />
      <div data-slot="mode-dock-stack-body" className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {activeDescriptor
          ? (() => {
              const { children, engagement, ...windowProps } = activeDescriptor;
              return (
                <Window {...windowProps} engagement={engagement} active={activeWindowId === activeId} onActivate={() => dock?.activateWindow(activeId!)}>
                  {children}
                </Window>
              );
            })()
          : null}
      </div>
    </div>
  );
};

//#endregion 🧭️ModeDockStack

//#region 🧭️ModeRender

interface ModeRenderContext {
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  onAxisLayoutChanged: (axisPath: ModeLayoutPath, sizes: Record<string, number>) => void;
}

function renderModeDockNode(node: WindowLayoutNode, path: ModeLayoutPath, ctx: ModeRenderContext): React.ReactNode {
  if (node.kind === "stack") {
    return <ModeDockStack key={path || "root-stack"} stackPath={path} node={node} windowsById={ctx.windowsById} activeWindowId={ctx.activeWindowId} />;
  }
  const orientation = node.kind === "row" ? "horizontal" : "vertical";
  const panels: React.ReactNode[] = [];
  node.children.forEach((child, index) => {
    const childPath = modeJoinPath(path, index);
    if (index > 0) panels.push(<ResizableHandle key={`sep-${childPath}`} />);
    panels.push(
      <ResizablePanel key={childPath} id={childPath} defaultSize={child.size ?? 100 / node.children.length} minSize={8}>
        {renderModeDockNode(child as WindowLayoutNode, childPath, ctx)}
      </ResizablePanel>,
    );
  });
  return (
    <ResizablePanelGroup key={path || "root-axis"} id={`mode-axis-${path || "root"}`} orientation={orientation} onLayoutChanged={(sizes) => ctx.onAxisLayoutChanged(path, sizes)} className="h-full min-h-0 w-full min-w-0">
      {panels}
    </ResizablePanelGroup>
  );
}

//#endregion 🧭️ModeRender

/** @emoji 🪟️ Golden-Layout-style docking mode shell with tab stacks, drag-dock, resize, maximize, and close. */
const Mode: React.FC<ModeProps> = ({ windows, activeWindowId, onActiveWindowChange, layout, children, className = "" }) => {
  const windowsById = React.useMemo(() => new Map(windows.map((window) => [window.id, window])), [windows]);
  const initialLayout = React.useMemo(() => resolveModeLayout(windows, layout), [layout, windows]);
  const [layoutState, setLayoutState] = React.useState<WindowLayoutNode>(() => initialLayout);
  const [maximizedStackPath, setMaximizedStackPath] = React.useState<ModeLayoutPath | null>(null);
  const [dragState, setDragState] = React.useState<ModeDragState | null>(null);
  const [dropZone, setDropZone] = React.useState<ModeDropZone | null>(null);
  const modeBodyRef = React.useRef<HTMLDivElement>(null);
  const stackElementsRef = React.useRef(new Map<ModeLayoutPath, HTMLElement>());
  const layoutPropRef = React.useRef(layout);
  const windowsKeyRef = React.useRef(windows.map((window) => window.id).join("|"));

  React.useEffect(() => {
    const nextKey = windows.map((window) => window.id).join("|");
    const layoutChanged = layoutPropRef.current !== layout;
    if (layoutChanged || windowsKeyRef.current !== nextKey) {
      layoutPropRef.current = layout;
      windowsKeyRef.current = nextKey;
      setLayoutState(resolveModeLayout(windows, layout));
      setMaximizedStackPath(null);
    }
  }, [layout, windows]);

  React.useEffect(() => {
    if (!activeWindowId) return;
    setLayoutState((prev) => setActiveWindowInLayout(prev, activeWindowId));
  }, [activeWindowId]);

  const registerStackRect = React.useCallback((path: ModeLayoutPath, element: HTMLElement | null) => {
    if (element) stackElementsRef.current.set(path, element);
    else stackElementsRef.current.delete(path);
  }, []);

  const activateWindow = React.useCallback(
    (windowId: string) => {
      setLayoutState((prev) => setActiveWindowInLayout(prev, windowId));
      onActiveWindowChange?.(windowId);
    },
    [onActiveWindowChange],
  );

  const closeWindow = React.useCallback(
    (windowId: string) => {
      setLayoutState((prev) => {
        const next = collapseLayout(removeWindowFromLayout(prev, windowId)) ?? { kind: "stack", children: [] };
        const remaining = modeCollectWindowIds(next);
        if (activeWindowId === windowId) {
          const fallback = remaining[0] ?? null;
          if (fallback) onActiveWindowChange?.(fallback);
          else onActiveWindowChange?.(windowId);
        }
        return next;
      });
      setMaximizedStackPath((prev) => (prev ? prev : null));
    },
    [activeWindowId, onActiveWindowChange],
  );

  const toggleMaximize = React.useCallback((stackPath: ModeLayoutPath) => {
    setMaximizedStackPath((prev) => (prev === stackPath ? null : stackPath));
  }, []);

  const refreshDropZone = React.useCallback((clientX: number, clientY: number) => {
    const rects = new Map<ModeLayoutPath, DOMRect>();
    stackElementsRef.current.forEach((element, path) => rects.set(path, element.getBoundingClientRect()));
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    setDropZone(computeModeDropZone(clientX, clientY, rects, modeRect));
  }, []);

  const finishDrag = React.useCallback(
    (drag: ModeDragState, zone: ModeDropZone | null) => {
      if (!zone) return;
      setLayoutState((prev) => applyModeDrop(prev, drag, zone));
      activateWindow(drag.windowId);
    },
    [activateWindow],
  );

  const startTabDrag = React.useCallback(
    (windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => {
      event.preventDefault();
      event.currentTarget.setPointerCapture(event.pointerId);
      setDragState({ windowId, stackPath, tabIndex, pointerId: event.pointerId, ghostLabel: label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    },
    [refreshDropZone],
  );

  React.useEffect(() => {
    if (!dragState) return;
    const handleMove = (event: PointerEvent) => {
      if (event.pointerId !== dragState.pointerId) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropZone(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      if (event.pointerId !== dragState.pointerId) return;
      finishDrag(dragState, dropZone);
      setDragState(null);
      setDropZone(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [dragState, dropZone, finishDrag, refreshDropZone]);

  const onAxisLayoutChanged = React.useCallback((axisPath: ModeLayoutPath, sizes: Record<string, number>) => {
    setLayoutState((prev) => applyAxisSizes(prev, axisPath, sizes));
  }, []);

  const dockContext = React.useMemo<ModeDockContextValue>(
    () => ({
      registerStackRect,
      startTabDrag,
      closeWindow,
      activateWindow,
      maximizedStackPath,
      toggleMaximize,
    }),
    [registerStackRect, startTabDrag, closeWindow, activateWindow, maximizedStackPath, toggleMaximize],
  );

  const renderContext = React.useMemo<ModeRenderContext>(() => ({ windowsById, activeWindowId, onAxisLayoutChanged }), [windowsById, activeWindowId, onAxisLayoutChanged]);

  const maximizedStack =
    maximizedStackPath !== null
      ? (() => {
          let found: WindowLayoutStackNode | null = null;
          mapLayoutStacks(layoutState, (stack, path) => {
            if (path === maximizedStackPath) found = stack;
            return stack;
          });
          return found;
        })()
      : null;

  const body =
    children ??
    (maximizedStack ? (
      <ModeDockContext.Provider value={dockContext}>
        <ModeDockStack stackPath={maximizedStackPath!} node={maximizedStack} windowsById={windowsById} activeWindowId={activeWindowId} />
      </ModeDockContext.Provider>
    ) : (
      <ModeDockContext.Provider value={dockContext}>{renderModeDockNode(layoutState, "", renderContext)}</ModeDockContext.Provider>
    ));

  return (
    <div data-slot="mode" className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      <div ref={modeBodyRef} data-slot="mode-body" className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
        {dragState ? (
          <div data-slot="mode-dock-ghost" className="pointer-events-none fixed z-panel rounded border border-accent bg-window px-single py-half text-xs shadow-md" style={{ left: dragState.x + 12, top: dragState.y + 12 }}>
            {dragState.ghostLabel}
          </div>
        ) : null}
        {dragState && dropZone ? (
          <div data-slot="mode-dock-drop-indicator" className="pointer-events-none absolute inset-0 z-panel">
            {dropZone.kind === "split" || dropZone.kind === "root-split" ? (
              <div
                className="absolute bg-accent/30 border-2 border-accent"
                style={(() => {
                  if (dropZone.kind === "root-split") {
                    const side = dropZone.side;
                    if (side === "left") return { left: 0, top: 0, width: "30%", height: "100%" };
                    if (side === "right") return { right: 0, top: 0, width: "30%", height: "100%" };
                    if (side === "top") return { left: 0, top: 0, width: "100%", height: "30%" };
                    return { left: 0, bottom: 0, width: "100%", height: "30%" };
                  }
                  const rect = stackElementsRef.current.get(dropZone.stackPath)?.getBoundingClientRect();
                  const modeRect = modeBodyRef.current?.getBoundingClientRect();
                  if (!rect || !modeRect) return { display: "none" };
                  const left = rect.left - modeRect.left;
                  const top = rect.top - modeRect.top;
                  if (dropZone.side === "left") return { left, top, width: rect.width * 0.5, height: rect.height };
                  if (dropZone.side === "right") return { left: left + rect.width * 0.5, top, width: rect.width * 0.5, height: rect.height };
                  if (dropZone.side === "top") return { left, top, width: rect.width, height: rect.height * 0.5 };
                  return { left, top: top + rect.height * 0.5, width: rect.width, height: rect.height * 0.5 };
                })()}
              />
            ) : (
              <div className="absolute inset-x-[20%] top-0 h-medium border-b-2 border-accent bg-accent/20" />
            )}
          </div>
        ) : null}
      </div>
    </div>
  );
};

export { Mode, removeWindowFromLayout, splitWithWindow, reconcileWindows, normalizeLayoutToStacks, collapseLayout };

// #endregion 🧭️Mode
