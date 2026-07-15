import { useCallback, useEffect, useLayoutEffect, useRef, useState, type DragEvent, type MouseEvent } from "react";
import { CATALOGUE_DRAG_MIME, ContextMenuController, getActiveCatalogueDragPayload, pickMostSpecificCanvasTarget, useCanvasAppearanceSync, type CanvasPickTarget } from "@semio-tech/ui-react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import type { ComponentSceneHostProps, Puzzle2dBoardScene } from "@semio-tech/framework-core";
import type { Puzzle2dBoardWasmSession } from "../os-shell.tsx";
import { createPuzzle2dBoardSession } from "../os-shell.tsx";

//#region Types
type BoardCamera = { readonly x: number; readonly y: number; readonly zoom: number };
type BoardEventRow = { readonly name: string; readonly payload?: unknown };
type Puzzle2dSelectionMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly action: string;
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

export function puzzle2dBoardCameraActionArgs(cameraJson: string): { readonly camera: BoardCamera } | null {
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

/** @emoji 🐢 Live cross-pane mirror payload extracted from a batch of freshly-drained rows — positions/selection/preselect only, everything else (camera, brush/link chrome, hover) stays pane-local. */
export type Puzzle2dLiveMirrorOps = {
  readonly positions: readonly { readonly id: string; readonly x: number; readonly y: number }[];
  readonly selectionIds: readonly string[] | null;
  readonly preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null;
  readonly clearPreselect: boolean;
};

function stringArray(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === "string") : [];
}

/**
 * @emoji 🐢 Classifies a batch of raw board-event rows (as seen straight off `drainEventsJson`, before
 * the transient-event filter/coalescer runs) into the subset worth mirroring imperatively into sibling
 * panes: latest node position per id (from `nodeMove` frames and/or a terminal `nodeDragEnd`), and the
 * live selection/preselect state (`select`/`preselectCancel` commit or restore selection and clear
 * preselect; `preselect` sets the live marquee highlight). Multiple rows of the same kind in one batch
 * collapse to the latest.
 */
export function collectPuzzle2dLiveMirrorOps(rows: readonly BoardEventRow[]): Puzzle2dLiveMirrorOps {
  const positionsById = new Map<string, { readonly id: string; readonly x: number; readonly y: number }>();
  let selectionIds: readonly string[] | null = null;
  let preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null = null;
  let clearPreselect = false;

  for (const row of rows) {
    const payload = row.payload as Record<string, unknown> | undefined;
    switch (row.name) {
      case "nodeMove": {
        const id = payload?.id;
        const x = payload?.x;
        const y = payload?.y;
        if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        break;
      }
      case "nodeDragEnd": {
        const moves = payload?.moves;
        if (!Array.isArray(moves)) break;
        for (const move of moves as readonly Record<string, unknown>[]) {
          const id = move.id;
          const x = move.x;
          const y = move.y;
          if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        }
        break;
      }
      case "preselect": {
        preselect = { ids: stringArray(payload?.ids), removedIds: stringArray(payload?.removedIds) };
        clearPreselect = false;
        break;
      }
      case "preselectCancel": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      case "select": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      default:
        break;
    }
  }

  return { positions: [...positionsById.values()], selectionIds, preselect, clearPreselect };
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
    return [{ id: "selectAll", label: "Select all", action: "selectAll" }];
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
    { id: "toggleHidden", label: anyVisible ? "Hide" : "Show", action: "setSelectionFlag", args: { flag: "hidden", value: anyVisible } },
    { id: "toggleLocked", label: anyUnlocked ? "Lock" : "Unlock", action: "setSelectionFlag", args: { flag: "locked", value: anyUnlocked } },
    { id: "duplicate", label: "Duplicate", action: "duplicateSelection", disabled: !hasSelectedNode },
    { id: "selectSameKind", label: "Select all of same kind", action: "selectSameKind" },
    { id: "focusSelection", label: "Zoom to selection", action: "focusSelection" },
    { id: "deleteSelection", label: "Delete", action: "deleteSelection", destructive: true },
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

//#region PeerSync
/** @emoji 🐢 One triptych pane, registered so siblings can mirror its live gesture state without a plugin round trip. */
type Puzzle2dBoardPeer = {
  readonly session: Puzzle2dBoardWasmSession;
  /** @emoji 🫧 `flushed` is true when the gesture that just ended pushed a commit to the plugin — in that case a fresh scene is already in flight and any stashed pending echo should be dropped rather than applied, or it would flash the stale in-between state for one frame before the fresh one supersedes it. */
  readonly onPeerGestureEnded: (flushed: boolean) => void;
};

/** @emoji 🗺️ controllerId -> surfaceId -> peer. Assumes one puzzle2d-play triptych on screen at a time (matches the existing controllerId/surfaceId scoping used for action routing). */
const puzzle2dBoardPeerRegistry = new Map<string, Map<string, Puzzle2dBoardPeer>>();
/** @emoji 🔒 controllerId -> surfaceId of the pane currently owning a live pointer gesture (drag/marquee), so siblings can defer conflicting echoes. */
const puzzle2dBoardGestureOwner = new Map<string, string>();

export function registerPuzzle2dBoardPeer(controllerId: string, surfaceId: string, peer: Puzzle2dBoardPeer): void {
  let peers = puzzle2dBoardPeerRegistry.get(controllerId);
  if (!peers) {
    peers = new Map();
    puzzle2dBoardPeerRegistry.set(controllerId, peers);
  }
  peers.set(surfaceId, peer);
}

export function unregisterPuzzle2dBoardPeer(controllerId: string, surfaceId: string): void {
  const peers = puzzle2dBoardPeerRegistry.get(controllerId);
  if (!peers) return;
  peers.delete(surfaceId);
  if (peers.size === 0) puzzle2dBoardPeerRegistry.delete(controllerId);
  if (puzzle2dBoardGestureOwner.get(controllerId) === surfaceId) puzzle2dBoardGestureOwner.delete(controllerId);
}

export function puzzle2dBoardPeers(controllerId: string, excludeSurfaceId: string): readonly Puzzle2dBoardPeer[] {
  const peers = puzzle2dBoardPeerRegistry.get(controllerId);
  if (!peers) return [];
  const result: Puzzle2dBoardPeer[] = [];
  for (const [surfaceId, peer] of peers) if (surfaceId !== excludeSurfaceId) result.push(peer);
  return result;
}

export function beginPuzzle2dPeerGesture(controllerId: string, surfaceId: string): void {
  puzzle2dBoardGestureOwner.set(controllerId, surfaceId);
}

export function endPuzzle2dPeerGesture(controllerId: string, surfaceId: string): void {
  if (puzzle2dBoardGestureOwner.get(controllerId) === surfaceId) puzzle2dBoardGestureOwner.delete(controllerId);
}

/** @emoji 🙅 True when a *different* pane owns the live gesture for this controller — the caller should defer applying an echoed scene. */
export function puzzle2dPeerOwnsGesture(controllerId: string, surfaceId: string): boolean {
  const owner = puzzle2dBoardGestureOwner.get(controllerId);
  return owner !== undefined && owner !== surfaceId;
}

export function pushPuzzle2dLiveMirrorOps(controllerId: string, surfaceId: string, ops: Puzzle2dLiveMirrorOps): void {
  if (ops.positions.length === 0 && !ops.selectionIds && !ops.preselect && !ops.clearPreselect) return;
  const peers = puzzle2dBoardPeers(controllerId, surfaceId);
  if (peers.length === 0) return;
  const positionsJson = ops.positions.length > 0 ? JSON.stringify(ops.positions) : null;
  const selectionJson = ops.selectionIds ? JSON.stringify(ops.selectionIds) : null;
  const preselectJson = ops.preselect ? JSON.stringify(ops.preselect) : ops.clearPreselect ? JSON.stringify({ ids: [], removedIds: [] }) : null;
  for (const peer of peers) {
    try {
      if (positionsJson) peer.session.setNodePositionsJson?.(positionsJson);
      if (selectionJson) peer.session.setSelectionIdsJsonSilent?.(selectionJson);
      if (preselectJson) peer.session.setPreselectStateJsonSilent?.(preselectJson);
    } catch {
      /* peer session not ready */
    }
  }
}

export function notifyPuzzle2dPeersGestureEnded(controllerId: string, surfaceId: string, flushed: boolean): void {
  for (const peer of puzzle2dBoardPeers(controllerId, surfaceId)) {
    try {
      peer.onPeerGestureEnded(flushed);
    } catch {
      /* peer session not ready */
    }
  }
}
//#endregion PeerSync

//#region Puzzle2dBoardHost
export function Puzzle2dBoardHost({ node, onAction }: ComponentSceneHostProps) {
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
  const renderScheduledRef = useRef(false);
  const pendingCameraDispatchRef = useRef<{ readonly camera: BoardCamera } | null>(null);
  const pendingSelectionJsonRef = useRef<string | null>(null);
  const onPeerGestureEndedRef = useRef<() => void>(() => {});
  const [sessionEpoch, setSessionEpoch] = useState(0);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly Puzzle2dSelectionMenuItem[] } | null>(null);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  /** @emoji 🎞️ Coalesces renderFrame() to at most one per animation frame, no matter how many raw pointer/wheel events fire in between — mirrors the premigration `scheduleInputInvalidate()` pattern. */
  const scheduleRender = useCallback((): void => {
    if (renderScheduledRef.current) return;
    renderScheduledRef.current = true;
    requestAnimationFrame(() => {
      renderScheduledRef.current = false;
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
    });
  }, []);

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
      pushPuzzle2dLiveMirrorOps(node.controllerId, node.surfaceId, collectPuzzle2dLiveMirrorOps(rows));
    } catch {
      /* session not ready */
    }
  }, [node.controllerId, node.surfaceId]);

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

  const applyPendingFixtureIfReady = useCallback(
    (session: Puzzle2dBoardWasmSession): void => {
      const pendingScene = pendingFixtureSceneRef.current;
      if (!pendingScene) return;
      if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) return;
      pendingFixtureSceneRef.current = null;
      applyToSession(session, (s) => applyFixtureToSession(s, pendingScene));
    },
    [node.controllerId, node.surfaceId],
  );

  /** @emoji 🐢 Mirror of `applyPendingFixtureIfReady` for the selection-only echo — a peer-owned gesture defers the plugin's `selectionJson` so it doesn't clobber a mirrored preselect highlight mid-marquee. */
  const applyPendingSelectionIfReady = useCallback(
    (session: Puzzle2dBoardWasmSession): void => {
      const pendingSelectionJson = pendingSelectionJsonRef.current;
      if (pendingSelectionJson === null) return;
      if (puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) return;
      pendingSelectionJsonRef.current = null;
      applyToSession(session, (s) => {
        if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(pendingSelectionJson);
        else s.setSelectionIdsJson(pendingSelectionJson);
      });
    },
    [node.controllerId, node.surfaceId],
  );

  onPeerGestureEndedRef.current = (flushed: boolean): void => {
    const session = sessionRef.current;
    if (!session) return;
    if (flushed) {
      pendingFixtureSceneRef.current = null;
      pendingSelectionJsonRef.current = null;
      return;
    }
    applyPendingFixtureIfReady(session);
    applyPendingSelectionIfReady(session);
  };

  /**
   * @emoji 🫧 Call when a gesture on this pane ends, right before flushing. Drains first so we know
   * whether a commit is about to go out; if so, drops any pending fixture/selection stashed mid-gesture
   * instead of applying it — that stashed snapshot is stale (typically from an early mid-gesture flush,
   * e.g. the `select` event a node-drag's pointerdown pushes) and the flush response due back in a moment
   * will supersede it anyway, so applying it here would flicker: correct live state -> stale snapshot ->
   * correct committed state. Returns whether a flush is pending, so the caller can pass it on to peers.
   */
  const settleGestureEnd = useCallback(
    (session: Puzzle2dBoardWasmSession): boolean => {
      drainIntoBuffer();
      const flushed = pendingEventRowsRef.current.length > 0;
      if (flushed) {
        pendingFixtureSceneRef.current = null;
        pendingSelectionJsonRef.current = null;
      } else {
        applyPendingFixtureIfReady(session);
        applyPendingSelectionIfReady(session);
      }
      return flushed;
    },
    [applyPendingFixtureIfReady, applyPendingSelectionIfReady, drainIntoBuffer],
  );

  /** @emoji 🐁 Marks a wheel-zoom gesture in flight so scene-driven camera echoes (which lag several ticks behind during a fast scroll) don't fight the live local zoom — mirrors `defersDescriptorSyncFromJs` for pan/drag, which the engine doesn't track for wheel. */
  const beginCameraInteraction = useCallback((): void => {
    cameraInteractionActiveRef.current = true;
    if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    cameraSettleTimeoutRef.current = setTimeout(() => {
      cameraInteractionActiveRef.current = false;
      cameraSettleTimeoutRef.current = null;
      const session = sessionRef.current;
      if (session) applyPendingFixtureIfReady(session);
      const pendingCamera = pendingCameraDispatchRef.current;
      if (pendingCamera) {
        pendingCameraDispatchRef.current = null;
        dispatch("setCamera", pendingCamera);
      }
    }, 350);
  }, [applyPendingFixtureIfReady, dispatch]);

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
      registerPuzzle2dBoardPeer(node.controllerId, node.surfaceId, { session, onPeerGestureEnded: () => onPeerGestureEndedRef.current() });

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
      unregisterPuzzle2dBoardPeer(node.controllerId, node.surfaceId);
      sessionRef.current?.free();
      sessionRef.current = null;
    };
  }, [node.controllerId, node.surfaceId, readContainerSize]);
  //#endregion SessionLifecycle

  //#region SceneSync
  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) {
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
  }, [sessionEpoch, scene?.fixtureJson, node.controllerId, node.surfaceId]);

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
    const session = sessionRef.current;
    if (!session) return;
    if (puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) {
      pendingSelectionJsonRef.current = scene.selectionJson;
      return;
    }
    applyToSession(session, (s) => {
      if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(scene.selectionJson);
      else s.setSelectionIdsJson(scene.selectionJson);
    });
  }, [sessionEpoch, scene?.selectionJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session || session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current) return;
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

  useCanvasAppearanceSync(() => {
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
      beginPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
      session.pointerDownScreen(point.x, point.y, event.button, event.shiftKey, event.metaKey || event.ctrlKey);
      scheduleRender();
    };

    const onPointerMove = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      scheduleRender();
      drainAndMaybeFlush();
    };

    const onPointerUp = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      endPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(node.controllerId, node.surfaceId, flushed);
    };

    const onPointerEnter = (): void => {
      hoverActiveRef.current = true;
    };

    const onPointerLeave = (event: PointerEvent): void => {
      hoverActiveRef.current = false;
      const session = sessionRef.current;
      if (!session) return;
      session.pointerLeaveScreen?.(event.altKey);
      endPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(node.controllerId, node.surfaceId, flushed);
    };

    /** @emoji 🐁 Wheel-zoom stays instant locally (WASM renders every tick via `scheduleRender`); only the React-visible camera echo and event flush are deferred until the gesture settles via `beginCameraInteraction`'s timeout. */
    const onWheel = (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      beginCameraInteraction();
      const point = clientToLocal(event.clientX, event.clientY);
      const delta = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      session.wheelScreen(point.x, point.y, delta);
      scheduleRender();
      const cameraArgs = puzzle2dBoardCameraActionArgs(session.cameraJson());
      if (cameraArgs) pendingCameraDispatchRef.current = cameraArgs;
      drainIntoBuffer();
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
  }, [beginCameraInteraction, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, node.controllerId, node.surfaceId, scheduleRender, scene?.interactive, settleGestureEnd]);
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
          endPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
          const flushed = settleGestureEnd(session);
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          dispatchBufferedEvents();
          notifyPuzzle2dPeersGestureEnded(node.controllerId, node.surfaceId, flushed);
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
  }, [dispatch, dispatchBufferedEvents, flushBoardEvents, node.controllerId, node.surfaceId, scene?.activeTool, scene?.interactive, scene?.selectionJson, settleGestureEnd]);
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
          onSelect: () => dispatch(item.action, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Puzzle2dBoardHost
