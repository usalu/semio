// #region 🧲️Header
// 💻️ framework/ui/elements/🎨️Canvas/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as ResizablePrimitive from "react-resizable-panels";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.forwardRef at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { CanvasSkeleton } from "../🦴️Skeletons/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../🔽️Select/🟦️.tsx";
import type { ResizableJoinCornerSpec } from "../↔️Resizable/🟦️.tsx";
import { Navbar, type NavbarItem } from "../🔝️Navbar/🟦️.tsx";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { interactiveActiveFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { shellFloorFillClass } from "../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
import { chromeStatusBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { useLabel, resolveTranslationLabel, useUiTranslation } from "../🏷️Label/🟦️.tsx";
import { useShellKeydown, useShellScopeOptional, NULL_SHELL_ROOT_REF } from "../🐚️ShellScope/🟦️.tsx";
import { isSurfaceActiveBackgroundPointer, useSurface, LevelProvider, setSurfaceActiveRoot } from "../🌈️Surface/🟦️.tsx";
import { type UiStatus } from "@semio-tech/ui-styling";
import type { WindowLayoutAxisNode, WindowLayoutNode, WindowLayoutStackNode, WindowStackCorner } from "../../🔨️modules/🗺️window-layout/🟦️.ts";
import { focusActiveSearchInput, routeWindowSearchEscape, routeWindowSearchKeydown, routeWindowSearchSpace } from "../🔍️WindowSearch/🟦️.tsx";
import type { ControlIcon } from "../🔣️Icons/🟦️.tsx";
import { useControlKeybinding } from "../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { ButtonGroup, ButtonGroupItem } from "../🔳️ButtonGroup/🟦️.tsx";
import { COMPOSE_WINDOW_TEMPLATE_MIME, MODE_TEMPLATE_PREVIEW_WINDOW_ID, type ModeCanvasDropTarget, type ModeWindowDescriptor, type WindowTemplateDropPayload, cancelWindowTemplatePointerDrag, endWindowTemplateDrag, readActiveWindowTemplateDragSession, windowTemplatePointerDragRef } from "./🪟️template-drag/🟦️.ts";
import { type ModeLayoutPath, collapseLayout, mapLayoutStacks, modeCollectWindowIds, removeWindowFromLayout, resolveModeLayout, resolveStackPathForWindowId, setActiveWindowInLayout } from "./🗺️layout/🟦️.ts";
import { type ModeDragState, type ModeDropZone, type ModePendingDrag, type ModeStackDropTargets, applyModeDrop, computeModeDropZone, computeModeSplitPreviewInBody, modeDockDragInsertTabs, modeDockOutLayout, resolveModeTabInsertPreview } from "./🎯️drop/🟦️.ts";
import { WINDOW_STACK_CORNERS, applyAxisSizes, safePanelGroupSetLayout } from "./📐️corners/🟦️.ts";
import { applyAxisGroupLayoutDelta, modeAxisGroupLayout, readModeAxisKind, readModeAxisPixelSize, resolveJoinCornerPeerCrossAxes, resolveJoinCornerResizeDeltas } from "./🔗️join-corners/🟦️.ts";
import { ModeDockContext, type ModeDockContextValue, ModeDockStack, type ModeRenderContext, renderModeDockNode } from "./⚓️dock/🟦️.tsx";
import { MODE_DRAG_CURSOR_OFFSET_X, MODE_DRAG_CURSOR_OFFSET_Y, ModeDockDragPreview } from "./🧲️drag-preview/🟦️.tsx";
// #endregion 🔌️Adapters
// #region ⚙️Canvas

/**
 * Container component for canvas window layout.
 **/
export const Canvas: React.FC<{ children: React.ReactNode; id?: string; status?: UiStatus }> = ({ children, id, status = "idle" }) => {
  const parent = useSurface();
  const bgClass = shellFloorFillClass(parent);
  const busy = status === "loading" || status === "waiting";
  return (
    <LevelProvider level="base">
      <div id={id} data-slot="canvas" data-level="base" data-ui-status={busy ? status : undefined} className={cn("box-border h-full w-full p-[var(--padding-standard)]", bgClass, chromeStatusBorderClass(status))}>
        {busy ? <CanvasSkeleton /> : children}
      </div>
    </LevelProvider>
  );
};

/**
 * Layout component arranging windows horizontally.
 **/
export const HorizontalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-row h-full w-full gap-[var(--padding-standard)]">{children}</div>;
};

/**
 * Layout component arranging windows vertically.
 **/
export const VerticalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-col h-full w-full gap-[var(--padding-standard)]">{children}</div>;
};

export interface ModeProps {
  windows: ModeWindowDescriptor[];
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string | null) => void;
  onWindowClose?: (windowId: string) => void;
  onWindowOpenInNewWindow?: (windowId: string) => void;
  layout?: WindowLayoutNode;
  onLayoutChange?: (layout: WindowLayoutNode) => void;
  onTemplateDrop?: (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => void;
  children?: React.ReactNode;
  className?: string;
  /** 📱️ Renders only the active window full-bleed with no tab bar, drag, dock, or maximize chrome. */
  mobile?: boolean;
}

//#region 🧭️ModeCanvasSpacing

/** 📐️ Canvas inset on {@link Mode} body; inter-panel splitters use the same {@link --spacing-single} step as navbar and footer chrome. */
export const MODE_CANVAS_INSET_CLASS = "p-[var(--panel-inset)]";

//#endregion 🧭️ModeRender

/** 🪟️ Golden-Layout-style docking mode shell with tab stacks, drag-dock, resize, maximize, and close. */
const Mode: React.FC<ModeProps> = ({ windows, activeWindowId, onActiveWindowChange, onWindowClose, onWindowOpenInNewWindow, layout, onLayoutChange, onTemplateDrop, children, className = "", mobile = false }) => {
  // 🐚️ Gates the active-window search-routing keydown listener below to this shell — absent outside a `ShellScopeProvider` (tests), where it simply stays inert.
  const shellScope = useShellScopeOptional();
  // 🐚️ Resolves via the nearest `I18nextProvider` (this shell's own instance), not the shared `uiI18n` singleton.
  const { t: modeT } = useUiTranslation();
  const parentSurface = useSurface();
  const modeBodyFillClass = shellFloorFillClass(parentSurface);
  const windowsById = reactHostPort.useMemo(() => new Map(windows.map((window) => [window.id, window])), [windows]);
  const windowsKey = reactHostPort.useMemo(() => windows.map((window) => window.id).join("|"), [windows]);
  const layoutKey = reactHostPort.useMemo(() => JSON.stringify(layout ?? null), [layout]);
  const layoutGeometryKey = reactHostPort.useMemo(() => JSON.stringify(layout ?? null, (key, value) => key === "title" ? undefined : value), [layout]);
  const initialLayout = reactHostPort.useMemo(() => resolveModeLayout(windows, layout), [layout, windows]);
  const [layoutState, setLayoutState] = reactHostPort.useState<WindowLayoutNode>(() => initialLayout);
  const [maximizedStackPath, setMaximizedStackPath] = reactHostPort.useState<ModeLayoutPath | null>(null);
  const [dragState, setDragState] = reactHostPort.useState<ModeDragState | null>(null);
  const [templateDrag, setTemplateDrag] = reactHostPort.useState<{ readonly label: string; readonly x: number; readonly y: number } | null>(null);
  const [pendingDrag, setPendingDrag] = reactHostPort.useState<ModePendingDrag | null>(null);
  const [dropZone, setDropZone] = reactHostPort.useState<ModeDropZone | null>(null);
  const dropZoneRef = reactHostPort.useRef<ModeDropZone | null>(null);
  const modeBodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const axisGroupRefsRef = reactHostPort.useRef(new Map<ModeLayoutPath, ResizablePrimitive.GroupImperativeHandle>());
  const axisGroupElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, HTMLDivElement>());
  const stackDropElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, { corners: Partial<Record<WindowStackCorner, HTMLElement | null>>; body: HTMLElement | null }>());
  const layoutStateRef = reactHostPort.useRef(layoutState);
  const dragLayoutSnapshotRef = reactHostPort.useRef<WindowLayoutNode | null>(null);
  const layoutKeyRef = reactHostPort.useRef(layoutKey);
  const layoutGeometryKeyRef = reactHostPort.useRef(layoutGeometryKey);
  const windowsKeyRef = reactHostPort.useRef(windowsKey);
  layoutStateRef.current = layoutState;

  // 🚧️ `onLayoutChange` is for user-driven layout mutations (drag/resize/close/activate) only.
  // Prop-derived states — the mount initializer and every prop resync — must never echo back:
  // during a session switch the windows prop changes before the layout prop, so the resync
  // reconciles against a stale layout and would write a pruned (possibly empty) layout into the
  // host's state, clobbering the layout the host is about to seed.
  const propDerivedLayoutJsonRef = reactHostPort.useRef(JSON.stringify(initialLayout));
  reactHostPort.useEffect(() => {
    if (JSON.stringify(layoutState) === propDerivedLayoutJsonRef.current) return;
    onLayoutChange?.(layoutState);
  }, [layoutState, onLayoutChange]);

  reactHostPort.useEffect(() => {
    const layoutChanged = layoutKeyRef.current !== layoutKey;
    const geometryChanged = layoutGeometryKeyRef.current !== layoutGeometryKey;
    const windowsChanged = windowsKeyRef.current !== windowsKey;
    if (!layoutChanged && !windowsChanged) return;
    layoutKeyRef.current = layoutKey;
    layoutGeometryKeyRef.current = layoutGeometryKey;
    windowsKeyRef.current = windowsKey;
    const resolved = resolveModeLayout(windows, layout);
    propDerivedLayoutJsonRef.current = JSON.stringify(resolved);
    setLayoutState(resolved);
    if (geometryChanged || windowsChanged) setMaximizedStackPath(null);
  }, [layout, layoutKey, layoutGeometryKey, windows, windowsKey]);

  reactHostPort.useEffect(() => {
    if (!activeWindowId) return;
    setLayoutState((prev) => setActiveWindowInLayout(prev, activeWindowId));
  }, [activeWindowId]);

  useShellKeydown(
    shellScope?.rootRef ?? NULL_SHELL_ROOT_REF,
    (event) => {
      if (!activeWindowId) return;
      const search = windowsById.get(activeWindowId)?.search;
      if (!search?.input) return;
      if (
        routeWindowSearchEscape(search, event, {
          chromeVisible: true,
          actionActive: true,
        })
      ) {
        event.preventDefault();
        event.stopPropagation();
        return;
      }
      if (routeWindowSearchSpace(search, event)) {
        event.preventDefault();
        event.stopPropagation();
        return;
      }
      if (!routeWindowSearchKeydown(search, event)) return;
      event.preventDefault();
      event.stopPropagation();
      queueMicrotask(() => focusActiveSearchInput());
    },
    [activeWindowId, windowsById, windowsKey],
  );

  const registerStackDropTargets = reactHostPort.useCallback((path: ModeLayoutPath, corner: WindowStackCorner, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => {
    if (!tabBarElement && !bodyElement) {
      const prev = stackDropElementsRef.current.get(path);
      if (!prev) return;
      const corners = { ...prev.corners, [corner]: null };
      const hasCorner = WINDOW_STACK_CORNERS.some((entry) => corners[entry]);
      if (!hasCorner) {
        stackDropElementsRef.current.delete(path);
        return;
      }
      stackDropElementsRef.current.set(path, { corners, body: prev.body });
      return;
    }
    const prev = stackDropElementsRef.current.get(path) ?? { corners: {}, body: null };
    stackDropElementsRef.current.set(path, {
      corners: { ...prev.corners, [corner]: tabBarElement ?? prev.corners[corner] ?? null },
      body: bodyElement ?? prev.body,
    });
  }, []);

  const activateWindow = reactHostPort.useCallback(
    (windowId: string) => {
      if (activeWindowId === windowId) return;
      setLayoutState((prev) => setActiveWindowInLayout(prev, windowId));
      onActiveWindowChange?.(windowId);
    },
    [activeWindowId, onActiveWindowChange],
  );

  const deactivateActiveWindow = reactHostPort.useCallback(() => {
    setSurfaceActiveRoot(null);
    if (activeWindowId === null) return;
    onActiveWindowChange?.(null);
  }, [activeWindowId, onActiveWindowChange]);

  const handleCanvasBackgroundPointerDown = reactHostPort.useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (isSurfaceActiveBackgroundPointer(event)) {
        deactivateActiveWindow();
        return;
      }
      if (!(event.target instanceof HTMLElement)) return;
      if (event.target.closest('[data-slot="mode-dock-stack"]')) return;
      if (event.target !== event.currentTarget && event.target.dataset.slot !== "resizable-panel") return;
      deactivateActiveWindow();
    },
    [deactivateActiveWindow],
  );

  const closeWindow = reactHostPort.useCallback(
    (windowId: string) => {
      onWindowClose?.(windowId);
      setLayoutState((prev) => {
        const next = collapseLayout(removeWindowFromLayout(prev, windowId)) ?? { kind: "stack", children: [] };
        const remaining = modeCollectWindowIds(next);
        if (activeWindowId === windowId) onActiveWindowChange?.(remaining[0] ?? null);
        return next;
      });
    },
    [activeWindowId, onActiveWindowChange, onWindowClose],
  );

  const toggleMaximize = reactHostPort.useCallback((stackPath: ModeLayoutPath) => {
    setMaximizedStackPath((prev) => (prev === stackPath ? null : stackPath));
  }, []);

  const refreshDropZone = reactHostPort.useCallback((clientX: number, clientY: number) => {
    const targets = new Map<ModeLayoutPath, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, path) => {
      const corners: ModeStackDropTargets["corners"] = {};
      for (const corner of WINDOW_STACK_CORNERS) {
        const element = elements.corners[corner];
        if (!element) continue;
        corners[corner] = { rect: element.getBoundingClientRect(), element };
      }
      targets.set(path, {
        corners,
        body: elements.body?.getBoundingClientRect() ?? null,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    const zone = computeModeDropZone(clientX, clientY, targets, modeRect);
    dropZoneRef.current = zone;
    setDropZone(zone);
  }, []);

  const finishDrag = reactHostPort.useCallback(
    (drag: ModeDragState, zone: ModeDropZone | null) => {
      if (!zone) return;
      setLayoutState((prev) => applyModeDrop(prev, drag, zone));
      activateWindow(drag.windowId);
    },
    [activateWindow],
  );

  const clearPendingDrag = reactHostPort.useCallback((pointerId: number) => {
    setPendingDrag((prev) => (prev?.pointerId === pointerId ? null : prev));
  }, []);

  const startTabDrag = reactHostPort.useCallback((windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => {
    if (event.button !== 0) return;
    setPendingDrag({
      dragKind: "tab",
      windowId,
      stackPath,
      tabIndex,
      pointerId: event.pointerId,
      ghostLabel: label,
      startX: event.clientX,
      startY: event.clientY,
    });
  }, []);

  reactHostPort.useEffect(() => {
    if (!pendingDrag && !dragState) return;
    const handleMove = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (pendingDrag && !dragState) {
        const distance = Math.hypot(event.clientX - pendingDrag.startX, event.clientY - pendingDrag.startY);
        if (distance < 6) return;
        dragLayoutSnapshotRef.current = layoutStateRef.current;
        setDragState({
          dragKind: pendingDrag.dragKind,
          windowId: pendingDrag.windowId,
          stackPath: pendingDrag.stackPath,
          tabIndex: pendingDrag.tabIndex,
          pointerId: pendingDrag.pointerId,
          ghostLabel: pendingDrag.ghostLabel,
          x: event.clientX,
          y: event.clientY,
        });
        setPendingDrag(null);
        refreshDropZone(event.clientX, event.clientY);
        return;
      }
      if (!dragState) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropZone(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (dragState) finishDrag(dragState, dropZoneRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [pendingDrag, dragState, finishDrag, refreshDropZone]);

  reactHostPort.useEffect(() => {
    if (!dragState) return;
    const cancelDrag = () => {
      if (dragLayoutSnapshotRef.current) setLayoutState(dragLayoutSnapshotRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      cancelDrag();
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [dragState]);

  const onAxisLayoutChanged = reactHostPort.useCallback((axisPath: ModeLayoutPath, sizes: Record<string, number>) => {
    setLayoutState((prev) => applyAxisSizes(prev, axisPath, sizes));
  }, []);

  const registerAxisGroup = reactHostPort.useCallback((axisPath: ModeLayoutPath, group: ResizablePrimitive.GroupImperativeHandle | null) => {
    if (group) axisGroupRefsRef.current.set(axisPath, group);
    else axisGroupRefsRef.current.delete(axisPath);
  }, []);

  const registerAxisElement = reactHostPort.useCallback((axisPath: ModeLayoutPath, element: HTMLDivElement | null) => {
    if (element) axisGroupElementsRef.current.set(axisPath, element);
    else axisGroupElementsRef.current.delete(axisPath);
  }, []);

  const onJoinCornerResize = reactHostPort.useCallback((spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number) => {
    const current = layoutStateRef.current;
    const mainKind = readModeAxisKind(current, spec.mainAxisPath);
    const crossKind = readModeAxisKind(current, spec.crossAxisPath);
    const mainGroup = axisGroupRefsRef.current.get(spec.mainAxisPath);
    const crossGroup = axisGroupRefsRef.current.get(spec.crossAxisPath);
    if (!mainKind || !crossKind || !mainGroup || !crossGroup) return;
    const peers = resolveJoinCornerPeerCrossAxes(current, spec);
    const deltas = resolveJoinCornerResizeDeltas(
      spec.parentKind,
      deltaXPx,
      deltaYPx,
      readModeAxisPixelSize(axisGroupElementsRef.current.get(spec.mainAxisPath), mainKind),
      readModeAxisPixelSize(axisGroupElementsRef.current.get(spec.crossAxisPath), crossKind),
    );
    const liveMainLayout = mainGroup.getLayout();
    const currentMainLayout = Object.keys(liveMainLayout).length > 0 ? liveMainLayout : modeAxisGroupLayout(current, spec.mainAxisPath);
    const mainLayout = applyAxisGroupLayoutDelta(currentMainLayout, spec.mainAxisPath, spec.mainSeparatorIndex, deltas.mainDeltaPct);
    safePanelGroupSetLayout(mainGroup, mainLayout);
    const peerLayouts: { path: ModeLayoutPath; layout: Record<string, number> }[] = [];
    for (const peer of peers) {
      const peerGroup = axisGroupRefsRef.current.get(peer.path);
      if (!peerGroup) continue;
      const livePeerLayout = peerGroup.getLayout();
      const currentPeerLayout = Object.keys(livePeerLayout).length > 0 ? livePeerLayout : modeAxisGroupLayout(current, peer.path);
      const peerLayout = applyAxisGroupLayoutDelta(currentPeerLayout, peer.path, peer.separatorIndex, deltas.crossDeltaPct);
      safePanelGroupSetLayout(peerGroup, peerLayout);
      peerLayouts.push({ path: peer.path, layout: peerLayout });
    }
    setLayoutState((prev) => peerLayouts.reduce((next, peer) => applyAxisSizes(next, peer.path, peer.layout), applyAxisSizes(prev, spec.mainAxisPath, mainLayout)));
  }, []);

  const clearTemplateDragPreview = reactHostPort.useCallback(() => {
    setTemplateDrag(null);
    dropZoneRef.current = null;
    setDropZone(null);
  }, []);

  reactHostPort.useEffect(() => {
    if (!templateDrag) return;
    const onDragEnd = () => {
      endWindowTemplateDrag();
      clearTemplateDragPreview();
    };
    document.addEventListener("dragend", onDragEnd);
    return () => document.removeEventListener("dragend", onDragEnd);
  }, [templateDrag, clearTemplateDragPreview]);

  const resolveTemplateDropZone = reactHostPort.useCallback((clientX: number, clientY: number): ModeDropZone | null => {
    const targets = new Map<ModeLayoutPath, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, path) => {
      const corners: ModeStackDropTargets["corners"] = {};
      for (const corner of WINDOW_STACK_CORNERS) {
        const element = elements.corners[corner];
        if (!element) continue;
        corners[corner] = { rect: element.getBoundingClientRect(), element };
      }
      targets.set(path, {
        corners,
        body: elements.body?.getBoundingClientRect() ?? null,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    return computeModeDropZone(clientX, clientY, targets, modeRect);
  }, []);

  const completeExternalTemplateDrop = reactHostPort.useCallback(
    (clientX: number, clientY: number, payload: WindowTemplateDropPayload): boolean => {
      if (!onTemplateDrop || typeof payload.windowKindId !== "string") {
        return false;
      }
      const zone = resolveTemplateDropZone(clientX, clientY);
      if (!zone) {
        return false;
      }
      endWindowTemplateDrag();
      windowTemplatePointerDragRef.active = false;
      clearTemplateDragPreview();
      onTemplateDrop(payload, zone);
      return true;
    },
    [clearTemplateDragPreview, onTemplateDrop, resolveTemplateDropZone],
  );

  const handleExternalTemplateDragOver = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      if (!onTemplateDrop || !event.dataTransfer.types.includes(COMPOSE_WINDOW_TEMPLATE_MIME)) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      setTemplateDrag({ label: session.label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    },
    [onTemplateDrop, refreshDropZone],
  );

  const handleExternalTemplateDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      if (!onTemplateDrop) return;
      const raw = event.dataTransfer.getData(COMPOSE_WINDOW_TEMPLATE_MIME);
      if (!raw) return;
      event.preventDefault();
      let payload: WindowTemplateDropPayload;
      try {
        payload = JSON.parse(raw) as WindowTemplateDropPayload;
      } catch {
        return;
      }
      completeExternalTemplateDrop(event.clientX, event.clientY, payload);
    },
    [completeExternalTemplateDrop, onTemplateDrop],
  );

  reactHostPort.useEffect(() => {
    if (!onTemplateDrop) return;
    const handlePointerMove = (event: PointerEvent) => {
      if (!windowTemplatePointerDragRef.active) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) return;
      setTemplateDrag({ label: session.label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    };
    const handlePointerUp = (event: PointerEvent) => {
      if (!windowTemplatePointerDragRef.active) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) {
        windowTemplatePointerDragRef.active = false;
        clearTemplateDragPreview();
        return;
      }
      const modeRect = modeBodyRef.current?.getBoundingClientRect();
      if (!modeRect || event.clientX < modeRect.left || event.clientX > modeRect.right || event.clientY < modeRect.top || event.clientY > modeRect.bottom) {
        cancelWindowTemplatePointerDrag();
        clearTemplateDragPreview();
        return;
      }
      if (!completeExternalTemplateDrop(event.clientX, event.clientY, session.payload)) {
        cancelWindowTemplatePointerDrag();
        clearTemplateDragPreview();
      }
    };
    window.addEventListener("pointermove", handlePointerMove, true);
    window.addEventListener("pointerup", handlePointerUp, true);
    window.addEventListener("pointercancel", handlePointerUp, true);
    return () => {
      window.removeEventListener("pointermove", handlePointerMove, true);
      window.removeEventListener("pointerup", handlePointerUp, true);
      window.removeEventListener("pointercancel", handlePointerUp, true);
    };
  }, [clearTemplateDragPreview, completeExternalTemplateDrop, onTemplateDrop, refreshDropZone]);

  const templatePreviewDrag = reactHostPort.useMemo((): ModeDragState | null => {
    if (!templateDrag) return null;
    return {
      dragKind: "tab",
      windowId: MODE_TEMPLATE_PREVIEW_WINDOW_ID,
      stackPath: "",
      tabIndex: -1,
      pointerId: -1,
      ghostLabel: templateDrag.label,
      x: templateDrag.x,
      y: templateDrag.y,
    };
  }, [templateDrag]);

  const previewDragState = dragState ?? templatePreviewDrag;

  const draggedPreviewTitle = previewDragState ? (windowsById.get(previewDragState.windowId)?.title ?? previewDragState.ghostLabel) : "";
  const draggedPreviewIconId = previewDragState ? windowsById.get(previewDragState.windowId)?.iconId : undefined;
  const tabInsertPreview = resolveModeTabInsertPreview(previewDragState, dropZone);
  const draggedInsertTabs = reactHostPort.useMemo<ModeDockContextValue["draggedInsertTabs"]>(() => {
    if (templateDrag) return [{ id: MODE_TEMPLATE_PREVIEW_WINDOW_ID, title: templateDrag.label, iconId: "app-window" }];
    if (!dragState) return [];
    return modeDockDragInsertTabs(
      layoutState,
      dragState,
      (windowId) => windowsById.get(windowId)?.title ?? windowId,
      (windowId) => windowsById.get(windowId)?.iconId ?? "app-window",
    );
  }, [dragState, layoutState, templateDrag, windowsById]);

  const noopDrag = reactHostPort.useCallback(() => {}, []);
  const canMaximize = modeCollectWindowIds(layoutState).length > 1;

  reactHostPort.useEffect(() => {
    if (canMaximize || maximizedStackPath === null) return;
    setMaximizedStackPath(null);
  }, [canMaximize, maximizedStackPath]);

  const openWindowInNewWindow = reactHostPort.useCallback(
    (windowId: string) => {
      onWindowOpenInNewWindow?.(windowId);
    },
    [onWindowOpenInNewWindow],
  );

  useControlKeybinding(
    "ui.window.close",
    () => {
      if (!activeWindowId) return;
      closeWindow(activeWindowId);
    },
    { enabled: Boolean(activeWindowId) },
    [activeWindowId, closeWindow],
  );

  useControlKeybinding(
    "ui.window.focus",
    () => {
      if (!activeWindowId) return;
      const stackPath = resolveStackPathForWindowId(layoutState, activeWindowId);
      if (stackPath === null) return;
      toggleMaximize(stackPath);
    },
    { enabled: Boolean(activeWindowId) && canMaximize },
    [activeWindowId, canMaximize, layoutState, toggleMaximize],
  );

  useControlKeybinding(
    "ui.window.newWindow",
    () => {
      if (!activeWindowId) return;
      openWindowInNewWindow(activeWindowId);
    },
    { enabled: Boolean(activeWindowId) && Boolean(onWindowOpenInNewWindow) },
    [activeWindowId, onWindowOpenInNewWindow, openWindowInNewWindow],
  );

  const dockContext = reactHostPort.useMemo<ModeDockContextValue>(
    () => ({
      dragState: previewDragState,
      tabInsertPreview,
      draggedInsertTabs,
      registerStackDropTargets,
      startTabDrag: mobile ? noopDrag : startTabDrag,
      clearPendingDrag,
      closeWindow,
      activateWindow,
      deactivateActiveWindow,
      maximizedStackPath,
      canMaximize,
      toggleMaximize,
    }),
    [mobile, noopDrag, previewDragState, tabInsertPreview, draggedInsertTabs, registerStackDropTargets, startTabDrag, clearPendingDrag, closeWindow, activateWindow, deactivateActiveWindow, maximizedStackPath, canMaximize, toggleMaximize],
  );

  const renderContext = reactHostPort.useMemo<ModeRenderContext>(
    () => ({ windowsById, activeWindowId, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement }),
    [windowsById, activeWindowId, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement],
  );

  const dockOutLayout = reactHostPort.useMemo(() => (dragState ? modeDockOutLayout(layoutState, dragState) : layoutState), [layoutState, dragState]);

  const maximizedStack =
    maximizedStackPath !== null
      ? (() => {
          let found: WindowLayoutStackNode | null = null;
          mapLayoutStacks(dockOutLayout, (stack, path) => {
            if (path === maximizedStackPath) found = stack;
            return stack;
          });
          return found;
        })()
      : null;

  const orderedWindowIds = modeCollectWindowIds(dockOutLayout);
  const hasWindows = orderedWindowIds.length > 0;
  const emptyShellNotice = resolveTranslationLabel(modeT("ui.display.emptyShell"));

  /** 📱️ Mobile has no split-pane window manager: every window collapses into one tab stack, rendered through the same {@link ModeDockStack} chrome (tab bar, utility bar, measures, engagement) as desktop. */
  const mobileFlatStack: WindowLayoutStackNode | null = mobile
    ? {
        kind: "stack",
        children: orderedWindowIds.map((id) => ({ kind: "window", id })),
        activeId: (activeWindowId && orderedWindowIds.includes(activeWindowId) ? activeWindowId : orderedWindowIds[0]) ?? undefined,
      }
    : null;

  const body =
    children ??
    (mobile ? (
      mobileFlatStack && mobileFlatStack.children.length > 0 ? (
        <ModeDockContext.Provider value={dockContext}>
          <ModeDockStack stackPath="" node={mobileFlatStack} windowsById={windowsById} activeWindowId={activeWindowId} mobile />
        </ModeDockContext.Provider>
      ) : null
    ) : maximizedStack ? (
      <ModeDockContext.Provider value={dockContext}>
        <ModeDockStack stackPath={maximizedStackPath!} node={maximizedStack} windowsById={windowsById} activeWindowId={activeWindowId} />
      </ModeDockContext.Provider>
    ) : (
      <ModeDockContext.Provider value={dockContext}>{renderModeDockNode(dockOutLayout as WindowLayoutAxisNode | WindowLayoutStackNode, "", renderContext)}</ModeDockContext.Provider>
    ));

  return (
    <div data-slot="mode" data-mobile={mobile ? "true" : undefined} data-dragging={previewDragState ? "true" : undefined} data-maximized-path={maximizedStackPath ?? undefined} className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      <LevelProvider level="base">
        <div
          ref={modeBodyRef}
          data-slot="mode-body"
          data-level="base"
          className={cn("relative box-border flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden", modeBodyFillClass, MODE_CANVAS_INSET_CLASS)}
          onPointerDownCapture={handleCanvasBackgroundPointerDown}
          onDragOver={!mobile && onTemplateDrop ? handleExternalTemplateDragOver : undefined}
          onDrop={!mobile && onTemplateDrop ? handleExternalTemplateDrop : undefined}
        >
          {!hasWindows ? (
            <div data-slot="mode-empty" className="flex flex-1 items-center justify-center p-large text-center text-sm text-muted-foreground">
              {emptyShellNotice}
            </div>
          ) : (
            body
          )}
          {!mobile && previewDragState ? (
            <>
              {dropZone?.kind !== "tab" ? (
                <ModeDockDragPreview
                  title={draggedPreviewTitle}
                  iconId={draggedPreviewIconId}
                  content={previewDragState.dragKind === "stack" ? windowsById.get(previewDragState.windowId)?.children : undefined}
                  tabOnly={previewDragState.dragKind === "tab"}
                  style={{
                    position: "fixed",
                    left: previewDragState.x + MODE_DRAG_CURSOR_OFFSET_X,
                    top: previewDragState.y - MODE_DRAG_CURSOR_OFFSET_Y,
                    zIndex: 70,
                  }}
                />
              ) : null}
              {dropZone && (dropZone.kind === "split" || dropZone.kind === "root-split") ? (
                <div data-slot="mode-dock-drop-indicator" className="pointer-events-none absolute inset-0 z-panel">
                  <div
                    className="absolute rounded-sm border-2 border-accent bg-accent/20"
                    style={(() => {
                      if (dropZone.kind === "root-split") {
                        const side = dropZone.side;
                        if (side === "left") return { left: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "right") return { right: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "top") return { left: 0, top: 0, width: "100%", height: "50%" };
                        return { left: 0, bottom: 0, width: "100%", height: "50%" };
                      }
                      const elements = stackDropElementsRef.current.get(dropZone.stackPath);
                      const rect = elements?.body?.getBoundingClientRect();
                      const modeRect = modeBodyRef.current?.getBoundingClientRect();
                      if (!rect || !modeRect) return { display: "none" };
                      const bodyOriginLeft = rect.left - modeRect.left;
                      const bodyOriginTop = rect.top - modeRect.top;
                      const preview = computeModeSplitPreviewInBody(rect.width, rect.height, dropZone.side);
                      return {
                        left: bodyOriginLeft + preview.left,
                        top: bodyOriginTop + preview.top,
                        width: preview.width,
                        height: preview.height,
                      };
                    })()}
                  />
                </div>
              ) : null}
            </>
          ) : null}
        </div>
      </LevelProvider>
    </div>
  );
};

export { Mode };

// #endregion 🧭️Mode

// #region 🧭️App

/** 📱️ Mode descriptor rendered inside {@link App}. */
export interface AppModeDescriptor {
  id: string;
  label?: UiLabel;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export interface AppProps {
  modes: AppModeDescriptor[];
  activeModeId: string;
  onActiveModeChange?: (modeId: string) => void;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** 📱️ App shell with optional mode switcher and one active mode body. */
const App: React.FC<AppProps> = ({ modes, activeModeId, onActiveModeChange, children, className = "", chrome = true }) => {
  const modeLabel = useLabel("ui.selection.mode");
  const activeMode = modes.find((mode) => mode.id === activeModeId) ?? modes[0];
  const body = children ?? activeMode?.children;
  const showModeNav = chrome && modes.length > 1 && !!onActiveModeChange;

  return (
    <div data-slot="app" className={cn("flex h-full min-h-0 w-full flex-col", className)}>
      {showModeNav ? (
        <div data-slot="app-mode-nav" className="flex shrink-0 items-center gap-[var(--padding-standard)] border-b p-[var(--padding-standard)]">
          <Select id="app.mode.select" value={activeModeId} onValueChange={onActiveModeChange}>
            <SelectTrigger className="w-[min(100%,16rem)]">
              <SelectValue placeholder={modeLabel} />
            </SelectTrigger>
            <SelectContent>
              {modes.map((mode) => (
                <SelectItem key={mode.id} value={mode.id}>
                  {mode.label ?? mode.id}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      ) : null}
      <div data-slot="app-body" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
    </div>
  );
};

export { App };

// #endregion 🧭️App


/** 🖥️ App descriptor rendered inside {@link Ui}. */
export interface UiAppDescriptor {
  id: string;
  label?: UiLabel;
  icon?: ControlIcon;
  children: React.ReactNode;
}

export interface UiProps {
  apps: UiAppDescriptor[];
  activeAppId: string;
  onActiveAppChange?: (appId: string) => void;
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** 🖥️ Top-level UI shell with optional app switcher and one active app body. */
const Ui: React.FC<UiProps> = ({ apps, activeAppId, onActiveAppChange, navbar, footer, children, className = "", chrome = true }) => {
  const activeApp = apps.find((app) => app.id === activeAppId) ?? apps[0];
  const body = children ?? activeApp?.children;
  const showAppNav = chrome && apps.length > 1 && !!onActiveAppChange;

  const navbarItems: NavbarItem[] = [];
  if (showAppNav) {
    navbarItems.push({
      key: "appNav",
      content: (
        <ButtonGroup id="ui.appNav">
          {apps.map((app) => (
            <ButtonGroupItem
              key={app.id}
              id={`ui.appNav.${app.id}`}
              className={cn(activeAppId === app.id && interactiveActiveFillClass)}
              data-state={activeAppId === app.id ? "on" : undefined}
              onClick={() => onActiveAppChange?.(app.id)}
              icon={app.icon ?? "layout-grid"}
              text={app.label}
            />
          ))}
        </ButtonGroup>
      ),
    });
  }
  if (navbar) {
    navbarItems.push({ key: "navbar", className: "flex-1 min-w-0", content: navbar });
  }

  return (
    <div data-slot="ui" className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      {navbarItems.length > 0 ? <Navbar items={navbarItems} /> : null}
      <div data-slot="ui-body" className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
      {footer && (
        <div data-slot="ui-footer" className="shrink-0">
          {footer}
        </div>
      )}
    </div>
  );
};

export { Ui };