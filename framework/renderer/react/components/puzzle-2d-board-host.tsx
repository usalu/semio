import { useCallback, useEffect, useLayoutEffect, useRef, useState, type DragEvent, type MouseEvent } from "react";
import { CATALOGUE_DRAG_MIME, ContextMenuController, getActiveCatalogueDragPayload, pickMostSpecificCanvasTarget, useCanvasThemeSync, type CanvasPickTarget } from "@semio-tech/ui-react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import type { CommandDescriptor, Puzzle2dBoardScene, Puzzle2dBoardWasmSession, UiComponentSceneNode } from "../os-shell.tsx";
import { createPuzzle2dBoardSession } from "../os-shell.tsx";

//#region Types
type BoardCamera = { readonly x: number; readonly y: number; readonly zoom: number };
type BoardEventRow = { readonly name: string; readonly payload?: unknown };
type Puzzle2dSelectionMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly command: string;
  readonly args?: Record<string, unknown>;
  readonly destructive?: boolean;
  readonly disabled?: boolean;
};
type Puzzle2dFixtureDropPayload = {
  readonly kindId: string;
  readonly catalogSlice: string;
  readonly shape?: string;
  readonly radius?: number;
  readonly width?: number;
  readonly height?: number;
  readonly iconKind?: string;
};
//#endregion Types

//#region Parsing
function parseBoardCamera(json: string): BoardCamera | null {
  try {
    const parsed = JSON.parse(json) as Partial<BoardCamera>;
    if (typeof parsed.x !== "number" || typeof parsed.y !== "number" || typeof parsed.zoom !== "number") return null;
    return { x: parsed.x, y: parsed.y, zoom: parsed.zoom };
  } catch {
    return null;
  }
}

export function puzzle2dBoardCameraCommandArgs(cameraJson: string): { readonly camera: BoardCamera } | null {
  const camera = parseBoardCamera(cameraJson);
  return camera ? { camera } : null;
}

function parseSelectionIds(json: string): readonly string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((id): id is string => typeof id === "string") : [];
  } catch {
    return [];
  }
}

export function parsePuzzle2dCatalogueDragPayload(encoded: string | null | undefined): Puzzle2dFixtureDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Puzzle2dFixtureDropPayload>;
    if (typeof parsed.kindId !== "string") return null;
    return {
      kindId: parsed.kindId,
      catalogSlice: typeof parsed.catalogSlice === "string" ? parsed.catalogSlice : "nodes",
      shape: typeof parsed.shape === "string" ? parsed.shape : undefined,
      radius: typeof parsed.radius === "number" ? parsed.radius : undefined,
      width: typeof parsed.width === "number" ? parsed.width : undefined,
      height: typeof parsed.height === "number" ? parsed.height : undefined,
      iconKind: typeof parsed.iconKind === "string" ? parsed.iconKind : undefined,
    };
  } catch {
    return null;
  }
}
//#endregion Parsing

//#region BoardEvents
const PUZZLE2D_TRANSIENT_EVENT_NAMES = new Set(["preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing"]);
const PUZZLE2D_FLUSH_NOW_EVENT_NAMES = new Set(["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"]);

/** @emoji 📬 Drops transient rows, coalesces `camera` to its latest value and `nodeMove` to one row per id (unless a `nodeDragEnd` follows), and flags whether the buffer should flush immediately. */
export function coalescePuzzle2dBoardEvents(rows: readonly BoardEventRow[]): { readonly flushNow: boolean; readonly eventsJson: string } {
  const hasDragEnd = rows.some((row) => row.name === "nodeDragEnd");
  let flushNow = false;
  let lastCamera: BoardEventRow | null = null;
  const nodeMoveById = new Map<string, BoardEventRow>();
  const rest: BoardEventRow[] = [];

  for (const row of rows) {
    if (PUZZLE2D_TRANSIENT_EVENT_NAMES.has(row.name)) continue;
    if (row.name === "camera") {
      lastCamera = row;
      continue;
    }
    if (row.name === "nodeMove") {
      if (hasDragEnd) continue;
      const id = (row.payload as { readonly id?: unknown } | undefined)?.id;
      if (typeof id === "string") {
        nodeMoveById.set(id, row);
        continue;
      }
    }
    if (PUZZLE2D_FLUSH_NOW_EVENT_NAMES.has(row.name)) flushNow = true;
    rest.push(row);
  }

  const coalesced: BoardEventRow[] = [];
  if (lastCamera) coalesced.push(lastCamera);
  coalesced.push(...nodeMoveById.values());
  coalesced.push(...rest);
  return { flushNow, eventsJson: JSON.stringify(coalesced) };
}
//#endregion BoardEvents

//#region SelectionMenu
function puzzle2dEntityFlag(entity: Record<string, unknown> | undefined, key: "hidden" | "locked"): boolean {
  return Boolean(entity && entity[key] === true);
}

/** @emoji 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors the premigration canvas context menu. */
export function buildPuzzle2dSelectionMenuItems(fixtureJson: string, selectionJson: string): readonly Puzzle2dSelectionMenuItem[] {
  let fixture: { readonly nodes?: readonly Record<string, unknown>[]; readonly edges?: readonly Record<string, unknown>[] } = {};
  try {
    fixture = JSON.parse(fixtureJson) as typeof fixture;
  } catch {
    /* empty fixture */
  }
  const selected = parseSelectionIds(selectionJson);
  if (selected.length === 0) {
    return [{ id: "selectAll", label: "Select all", command: "selectAll" }];
  }

  const selectedSet = new Set(selected);
  const nodes = fixture.nodes ?? [];
  const edges = fixture.edges ?? [];
  const selectedEntities: Record<string, unknown>[] = [];
  let hasSelectedNode = false;
  for (const node of nodes) {
    const id = node.id;
    if (typeof id === "string" && selectedSet.has(id)) {
      selectedEntities.push(node);
      hasSelectedNode = true;
    }
    const handles = node.handles;
    if (Array.isArray(handles)) {
      for (const handle of handles as Record<string, unknown>[]) {
        const handleId = handle.id;
        if (typeof handleId === "string" && selectedSet.has(handleId)) selectedEntities.push(handle);
      }
    }
  }
  for (const edge of edges) {
    const id = edge.id;
    if (typeof id === "string" && selectedSet.has(id)) selectedEntities.push(edge);
  }

  const anyVisible = selectedEntities.some((entity) => !puzzle2dEntityFlag(entity, "hidden"));
  const anyUnlocked = selectedEntities.some((entity) => !puzzle2dEntityFlag(entity, "locked"));

  return [
    { id: "toggleHidden", label: anyVisible ? "Hide" : "Show", command: "setSelectionFlag", args: { flag: "hidden", value: anyVisible } },
    { id: "toggleLocked", label: anyUnlocked ? "Lock" : "Unlock", command: "setSelectionFlag", args: { flag: "locked", value: anyUnlocked } },
    { id: "duplicate", label: "Duplicate", command: "duplicateSelection", disabled: !hasSelectedNode },
    { id: "selectSameKind", label: "Select all of same kind", command: "selectSameKind" },
    { id: "focusSelection", label: "Zoom to selection", command: "focusSelection" },
    { id: "deleteSelection", label: "Delete", command: "deleteSelection", destructive: true },
  ];
}
//#endregion SelectionMenu

//#region FixtureDrop
export function puzzle2dFixtureDropPreviewJson(payload: Puzzle2dFixtureDropPayload, screenX: number, screenY: number): string {
  return JSON.stringify({ nodeKind: payload.kindId, screenX, screenY, shape: payload.shape, radius: payload.radius, width: payload.width, height: payload.height, iconKind: payload.iconKind });
}

/** @emoji 📐 Inverse of the canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across board renderers. */
export function puzzle2dScreenToWorld(cameraJson: string, containerSize: { readonly w: number; readonly h: number }, screen: { readonly x: number; readonly y: number }): { readonly x: number; readonly y: number } | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  const zoom = camera.zoom || 1;
  return {
    x: camera.x + (screen.x - containerSize.w / 2) / zoom,
    y: camera.y + (screen.y - containerSize.h / 2) / zoom,
  };
}
//#endregion FixtureDrop

//#region Sync
function applyToSession(session: Puzzle2dBoardWasmSession | null, action: (session: Puzzle2dBoardWasmSession) => void): void {
  if (!session) return;
  try {
    action(session);
    session.renderFrame();
  } catch {
    /* session not ready */
  }
}

/** @emoji 🔁 Re-parses the fixture and silently re-applies selection/camera, since `parseFixtureJson` resets both to the fixture's own defaults. */
function applyFixtureToSession(session: Puzzle2dBoardWasmSession, scene: Puzzle2dBoardScene): void {
  session.parseFixtureJson(scene.fixtureJson);
  session.setSelectionOptions?.(scene.selectionMethod, "replace", true, true, true);
  if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(scene.selectionJson);
  else session.setSelectionIdsJson(scene.selectionJson);
  const camera = parseBoardCamera(scene.cameraJson);
  if (camera) {
    if (session.setCameraSilent) session.setCameraSilent(camera.x, camera.y, camera.zoom);
    else session.setCamera(camera.x, camera.y, camera.zoom);
  }
}
//#endregion Sync

//#region Puzzle2dBoardHost
export function Puzzle2dBoardHost({ node, onCommand }: { readonly node: UiComponentSceneNode; readonly onCommand: (command: CommandDescriptor) => void }) {
  const scene = node.puzzle2dBoard;
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<Puzzle2dBoardWasmSession | null>(null);
  const bootSyncedRef = useRef(false);
  const pendingFixtureSceneRef = useRef<Puzzle2dBoardScene | null>(null);
  const pendingEventRowsRef = useRef<BoardEventRow[]>([]);
  const hoverActiveRef = useRef(false);
  const cameraInteractionActiveRef = useRef(false);
  const cameraSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [sessionEpoch, setSessionEpoch] = useState(0);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly Puzzle2dSelectionMenuItem[] } | null>(null);

  const dispatch = useCallback(
    (command: string, args?: Record<string, unknown>) => {
      onCommand({ controllerId: node.controllerId, command, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onCommand],
  );

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) return { w: 1, h: 1 };
    const rect = container.getBoundingClientRect();
    return {
      w: Math.max(1, Math.round(rect.width || container.clientWidth)),
      h: Math.max(1, Math.round(rect.height || container.clientHeight)),
    };
  }, []);

  //#region BoardEventFlush
  const drainIntoBuffer = useCallback((): void => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.drainEventsJson();
      if (!json || json === "[]") return;
      const rows = JSON.parse(json) as BoardEventRow[];
      pendingEventRowsRef.current.push(...rows);
    } catch {
      /* session not ready */
    }
  }, []);

  const dispatchBufferedEvents = useCallback((): void => {
    if (pendingEventRowsRef.current.length === 0) return;
    const { eventsJson } = coalescePuzzle2dBoardEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    if (eventsJson && eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
  }, [dispatch]);

  const drainAndMaybeFlush = useCallback((): void => {
    drainIntoBuffer();
    if (pendingEventRowsRef.current.length === 0) return;
    const { flushNow } = coalescePuzzle2dBoardEvents(pendingEventRowsRef.current);
    if (flushNow) dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const flushBoardEvents = useCallback((): void => {
    drainIntoBuffer();
    dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const applyPendingFixtureIfReady = useCallback((session: Puzzle2dBoardWasmSession): void => {
    const pendingScene = pendingFixtureSceneRef.current;
    if (!pendingScene) return;
    if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current) return;
    pendingFixtureSceneRef.current = null;
    applyToSession(session, (s) => applyFixtureToSession(s, pendingScene));
  }, []);

  /** @emoji 🐁 Marks a wheel-zoom gesture in flight so scene-driven camera echoes (which lag several ticks behind during a fast scroll) don't fight the live local zoom — mirrors `defersDescriptorSyncFromJs` for pan/drag, which the engine doesn't track for wheel. */
  const beginCameraInteraction = useCallback((): void => {
    cameraInteractionActiveRef.current = true;
    if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    cameraSettleTimeoutRef.current = setTimeout(() => {
      cameraInteractionActiveRef.current = false;
      cameraSettleTimeoutRef.current = null;
      const session = sessionRef.current;
      if (session) applyPendingFixtureIfReady(session);
    }, 350);
  }, [applyPendingFixtureIfReady]);

  useEffect(
    () => () => {
      if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    },
    [],
  );
  //#endregion BoardEventFlush

  //#region SessionLifecycle
  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return undefined;
    let disposed = false;
    let resizeObserver: ResizeObserver | null = null;
    let raf = 0;

    void createPuzzle2dBoardSession().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      sessionRef.current = session;

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        session.setSize(w, h, nextDpr);
      };

      const boot = async (): Promise<void> => {
        let { w, h } = readContainerSize();
        for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
          await new Promise<void>((resolve) => {
            if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
            else queueMicrotask(resolve);
          });
          if (disposed) return;
          ({ w, h } = readContainerSize());
        }
        const dpr = globalThis.devicePixelRatio || 1;
        await session.attach_canvas(canvas, w, h, dpr);
        if (disposed) {
          session.free();
          return;
        }
        applySize();
        syncSessionCanvasTheme(session);
        const tick = () => {
          if (disposed) return;
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          raf = requestAnimationFrame(tick);
        };
        raf = requestAnimationFrame(tick);
        setSessionEpoch((epoch) => epoch + 1);
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              applySize();
            });
      resizeObserver?.observe(container);
      void boot();
    });

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (raf) cancelAnimationFrame(raf);
      sessionRef.current?.free();
      sessionRef.current = null;
    };
  }, [readContainerSize]);
  //#endregion SessionLifecycle

  //#region SceneSync
  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current) {
      pendingFixtureSceneRef.current = scene;
      return;
    }
    applyToSession(session, (s) => applyFixtureToSession(s, scene));
    if (!bootSyncedRef.current) {
      bootSyncedRef.current = true;
      try {
        session.drainEventsJson();
      } catch {
        /* session not ready */
      }
    }
  }, [sessionEpoch, scene?.fixtureJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setKindCatalogsJson(scene.kindCatalogsJson));
  }, [sessionEpoch, scene?.kindCatalogsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHandleLinkCompatJson?.(scene.kindCompatibilityJson));
  }, [sessionEpoch, scene?.kindCompatibilityJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => {
      if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(scene.selectionJson);
      else session.setSelectionIdsJson(scene.selectionJson);
    });
  }, [sessionEpoch, scene?.selectionJson]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session || session.defersDescriptorSyncFromJs?.()) return;
    applyToSession(session, (s) => {
      const camera = parseBoardCamera(scene.cameraJson);
      if (!camera) return;
      if (s.setCameraSilent) s.setCameraSilent(camera.x, camera.y, camera.zoom);
      else s.setCamera(camera.x, camera.y, camera.zoom);
    });
  }, [sessionEpoch, scene?.cameraJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHoveredIdSilent?.(scene.hoveredId ?? null));
  }, [sessionEpoch, scene?.hoveredId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setActiveTool?.(scene.activeTool ?? "select"));
  }, [sessionEpoch, scene?.activeTool]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setSelectionOptions?.(scene.selectionMethod, "replace", true, true, true));
  }, [sessionEpoch, scene?.selectionMethod]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridSnapEnabled?.(scene.gridSnapEnabled));
  }, [sessionEpoch, scene?.gridSnapEnabled]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridFactor?.(scene.gridFactor));
  }, [sessionEpoch, scene?.gridFactor]);

  useEffect(() => {
    if (!scene || scene.suggestionOffset <= 0) return;
    applyToSession(sessionRef.current, (session) => session.setSuggestionOffset?.(scene.suggestionOffset));
  }, [sessionEpoch, scene?.suggestionOffset]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setBrushKindWeights?.(scene.brushKindWeightsJson));
  }, [sessionEpoch, scene?.brushKindWeightsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => {
      if (scene.lodMode === "automatic") {
        session.setAutomaticLod?.(true);
      } else {
        session.setAutomaticLod?.(false);
        session.setForcedDrawLodLabel?.(scene.lodMode);
      }
    });
  }, [sessionEpoch, scene?.lodMode]);
  //#endregion SceneSync

  useCanvasThemeSync(() => {
    syncSessionCanvasTheme(sessionRef.current);
    try {
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  });

  //#region Pointer
  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene?.interactive) return undefined;

    const clientToLocal = (clientX: number, clientY: number): { x: number; y: number } => {
      const rect = canvas.getBoundingClientRect();
      return { x: clientX - rect.left, y: clientY - rect.top };
    };

    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 0 || event.button === 1) {
        canvas.setPointerCapture?.(event.pointerId);
      }
      session.pointerDownScreen(point.x, point.y, event.button, event.shiftKey, event.metaKey || event.ctrlKey);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
    };

    const onPointerMove = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      drainAndMaybeFlush();
    };

    const onPointerUp = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      applyPendingFixtureIfReady(session);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      flushBoardEvents();
    };

    const onPointerEnter = (): void => {
      hoverActiveRef.current = true;
    };

    const onPointerLeave = (event: PointerEvent): void => {
      hoverActiveRef.current = false;
      const session = sessionRef.current;
      if (!session) return;
      session.pointerLeaveScreen?.(event.altKey);
      applyPendingFixtureIfReady(session);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      flushBoardEvents();
    };

    const onWheel = (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      const delta = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      session.wheelScreen(point.x, point.y, delta);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      const cameraArgs = puzzle2dBoardCameraCommandArgs(session.cameraJson());
      if (cameraArgs) dispatch("setCamera", cameraArgs);
      flushBoardEvents();
    };

    canvas.addEventListener("pointerdown", onPointerDown);
    canvas.addEventListener("pointerenter", onPointerEnter);
    canvas.addEventListener("pointerleave", onPointerLeave);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    container.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointerenter", onPointerEnter);
      canvas.removeEventListener("pointerleave", onPointerLeave);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      container.removeEventListener("wheel", onWheel);
    };
  }, [applyPendingFixtureIfReady, dispatch, drainAndMaybeFlush, flushBoardEvents, scene?.interactive]);
  //#endregion Pointer

  //#region Keyboard
  useEffect(() => {
    if (!scene?.interactive) return undefined;
    const isEditableTarget = (target: EventTarget | null): boolean => {
      if (!(target instanceof HTMLElement)) return false;
      return target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable;
    };
    const onKeyDown = (event: KeyboardEvent): void => {
      if (!hoverActiveRef.current || isEditableTarget(event.target)) return;
      const session = sessionRef.current;
      if (!session) return;
      if (event.key === "Escape") {
        if (session.cancelAreaSelect?.()) {
          event.preventDefault();
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          flushBoardEvents();
        }
        return;
      }
      if (event.key === "Tab" && scene.activeTool === "brush") {
        event.preventDefault();
        session.brushCycleCandidate?.(!event.shiftKey);
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        flushBoardEvents();
        return;
      }
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "a") {
        event.preventDefault();
        dispatch("selectAll");
        return;
      }
      if (event.key === "Delete" || event.key === "Backspace") {
        if (parseSelectionIds(scene.selectionJson).length === 0) return;
        event.preventDefault();
        session.deleteSelection?.();
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        flushBoardEvents();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [dispatch, flushBoardEvents, scene?.activeTool, scene?.interactive, scene?.selectionJson]);
  //#endregion Keyboard

  //#region ContextMenu
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      const session = sessionRef.current;
      if (!session?.pickTargetsAtScreenJson) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let targets: CanvasPickTarget[] = [];
      try {
        targets = JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        targets = [];
      }
      const best = pickMostSpecificCanvasTarget(targets);
      let selectionIds = parseSelectionIds(scene.selectionJson);
      if (best && !selectionIds.includes(best.id)) {
        selectionIds = [best.id];
        if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(JSON.stringify(selectionIds));
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        dispatch("setSelection", { ids: selectionIds });
      }
      const items = buildPuzzle2dSelectionMenuItems(scene.fixtureJson, JSON.stringify(selectionIds));
      setContextMenu({ x: event.clientX, y: event.clientY, items });
    },
    [dispatch, scene?.fixtureJson, scene?.interactive, scene?.selectionJson],
  );
  //#endregion ContextMenu

  //#region FixtureDropHandlers
  const onDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive || !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      const session = sessionRef.current;
      if (!session?.setFixtureDropPreviewJson) return;
      const payload = parsePuzzle2dCatalogueDragPayload(getActiveCatalogueDragPayload());
      if (!payload) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      applyToSession(session, (s) => s.setFixtureDropPreviewJson?.(puzzle2dFixtureDropPreviewJson(payload, event.clientX - rect.left, event.clientY - rect.top)));
    },
    [scene?.interactive],
  );

  const onDragLeave = useCallback((): void => {
    applyToSession(sessionRef.current, (session) => session.clearFixtureDropPreview?.());
  }, []);

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload();
      const payload = parsePuzzle2dCatalogueDragPayload(encoded);
      const session = sessionRef.current;
      applyToSession(session, (s) => s.clearFixtureDropPreview?.());
      if (!payload || !session) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      const world = puzzle2dScreenToWorld(session.cameraJson(), readContainerSize(), { x: event.clientX - rect.left, y: event.clientY - rect.top });
      dispatch("addNode", {
        kind: payload.kindId,
        x: world?.x,
        y: world?.y,
        shape: payload.shape,
        radius: payload.radius,
        width: payload.width,
        height: payload.height,
        iconKind: payload.iconKind,
      });
    },
    [dispatch, readContainerSize, scene?.interactive],
  );
  //#endregion FixtureDropHandlers

  if (!scene) return <div className="semio-puzzle2d-board-empty text-muted-foreground p-2 text-xs">No puzzle board scene</div>;

  return (
    <div
      ref={containerRef}
      className="semio-puzzle2d-board-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none"
      data-surface-id={node.surfaceId}
      style={{ touchAction: "none" }}
      onContextMenu={onContextMenu}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={(contextMenu?.items ?? []).map((item) => ({
          id: item.id,
          label: item.label,
          disabled: item.disabled,
          destructive: item.destructive,
          onSelect: () => dispatch(item.command, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Puzzle2dBoardHost
