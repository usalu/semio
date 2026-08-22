// #region 🧲️Header
/** @emoji 🎪️ Entwerfen mit Bestand demonstrator landing — general introduction, six live app panes, glass name overlay. */
// #endregion 🧲️Header

import { createUiErrorBoundary, mountUiRoot, useUiCallback as useCallback, useUiEffect as useEffect, useUiMemo as useMemo, useUiRef as useRef, useUiState as useState, type UiNode } from "@semio-tech/ui-react/runtime";
import {
  Navbar,
  ShellBrandLogo,
  UIIntroduction,
  bootstrapElementsSurfaceChromeDocument,
  CanvasSkeleton,
  chromeStatusBorderClass,
  cn,
  loadingBorderClass,
  WindowBodySkeleton,
  elementSkeleton,
  Icon,
  initUiLocaleSync,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiDriver,
  UI_MOBILE_MEDIA_QUERY,
  useElementsSurfaceChrome,
  useMediaQuery,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort, resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "@semio-tech/plugin-registry/catalog";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "./⚛️footer.tsx";
import { DEMONSTRATOR_LOCALE, DEMONSTRATOR_PANES, ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION, ENTWERFEN_MIT_BESTAND_LOGO_SVG, demonstratorPaneRuntimeVariant, type DemonstratorPaneSpec } from "./🟦️brand.ts";
import "./🎨️globals.css";

// 🎪️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
const demonstratorStorage = createBrowserStoragePort();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(demonstratorStorage));
// 🇩🇪️ The whole demonstrator is German-locked (see 🟦️brand.ts) — resolve synchronously before the
// first render so the landing page's own chrome (Skip/Back/Next/Done) never flashes English.
initUiLocaleSync(DEMONSTRATOR_LOCALE);

/** @emoji 📱️ Touch-first viewports use the vertical snap list even when wider than {@link UI_MOBILE_MEDIA_QUERY}. */
const DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY = `${UI_MOBILE_MEDIA_QUERY} and (hover: none) and (pointer: coarse)`;

//#region 🎪️DemonstratorGridGeometry
/** @emoji 🔢️ Columns and rows of the demonstrator preview grid; the strip spans `columns * 100vw` by `rows * 100vh`. */
const DEMONSTRATOR_GRID_COLUMNS = 3;
const DEMONSTRATOR_GRID_ROWS = 2;

function paneColumn(paneIndex: number): number {
  return paneIndex % DEMONSTRATOR_GRID_COLUMNS;
}

function paneRow(paneIndex: number): number {
  return Math.floor(paneIndex / DEMONSTRATOR_GRID_COLUMNS);
}

function paneIndexById(id: string): number {
  return DEMONSTRATOR_PANES.findIndex((pane) => pane.id === id);
}

function paneIdFromLocationHash(): string | null {
  const raw = window.location.hash.replace(/^#/, "").trim();
  if (!raw) return null;
  return DEMONSTRATOR_PANES.some((pane) => pane.id === raw) ? raw : null;
}

/** @emoji 🧭️ Horizontal (vw) and vertical (vh) scroll offset into the demonstrator grid. */
type ScrollOffset = { readonly x: number; readonly y: number };

/** @emoji 🧭️ Largest scroll offset that still keeps the last column and row flush with the viewport edge. */
const DEMONSTRATOR_MAX_SCROLL: ScrollOffset = { x: (DEMONSTRATOR_GRID_COLUMNS - 1) * 100, y: (DEMONSTRATOR_GRID_ROWS - 1) * 100 };

/** @emoji 🎞 Programmatic pane pin / focus glide duration — one rAF timeline owns the transform; never pair this with a CSS `transition` on the same property. */
const DEMONSTRATOR_SCROLL_GLIDE_MS = 500;

/** @emoji 🎞 Exponential follow factor while the cursor freely pans the overview. */
const DEMONSTRATOR_SCROLL_FOLLOW_LERP = 0.12;

/** @emoji 🎞 Settle epsilon (vw/vh) for the free-pan follow loop. */
const DEMONSTRATOR_SCROLL_FOLLOW_EPSILON = 0.01;

/** @emoji 🎞 Either free-pan follow (exponential) or a timed ease-in-out glide to a pane — mutually exclusive so the grid never fights itself. */
type ScrollDrive =
  | { readonly mode: "follow" }
  | {
      readonly mode: "glide";
      readonly from: ScrollOffset;
      readonly to: ScrollOffset;
      readonly startedAt: number;
      readonly durationMs: number;
    };

/** @emoji 🎞 Cubic ease-in-out for focus / hover pin glides. */
function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

/** @emoji 🧭️ Scroll offset that brings the given pane fully into the viewport. */
function scrollOffsetForPaneIndex(paneIndex: number): ScrollOffset {
  return {
    x: Math.min(DEMONSTRATOR_MAX_SCROLL.x, Math.max(0, paneColumn(paneIndex) * 100)),
    y: Math.min(DEMONSTRATOR_MAX_SCROLL.y, Math.max(0, paneRow(paneIndex) * 100)),
  };
}

/** @emoji 🧭️ Linear blend of two scroll offsets. */
function lerpScrollOffset(from: ScrollOffset, to: ScrollOffset, t: number): ScrollOffset {
  return { x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t };
}

type PaneAxisBounds = { readonly start: number; readonly end: number; readonly visible: boolean };

/** @emoji 📐 Maps one axis of a grid cell into the current viewport after scrolling (percent of that axis). */
function paneAxisBounds(cellIndex: number, scrollPercent: number): PaneAxisBounds {
  const cellStart = cellIndex * 100 - scrollPercent;
  const start = Math.max(0, cellStart);
  const end = Math.min(100, cellStart + 100);
  return { start, end, visible: end > start };
}

type TintSegmentPx = { readonly top: number; readonly left: number; readonly width: number; readonly height: number };

type RevealRectPx = { readonly top: number; readonly left: number; readonly width: number; readonly height: number };

/** @emoji 👁 Visible on-screen bounds of a grid pane — the region that stays untinted while its card is hovered. */
function demonstratorPaneRevealRect(paneIndex: number, scrollOffset: ScrollOffset): RevealRectPx {
  const horizontal = paneAxisBounds(paneColumn(paneIndex), scrollOffset.x);
  const vertical = paneAxisBounds(paneRow(paneIndex), scrollOffset.y);
  if (!horizontal.visible || !vertical.visible) return { top: 0, left: 0, width: 0, height: 0 };
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  const left = (horizontal.start / 100) * vw;
  const right = (horizontal.end / 100) * vw;
  const top = (vertical.start / 100) * vh;
  const bottom = (vertical.end / 100) * vh;
  return { top, left, width: Math.max(0, right - left), height: Math.max(0, bottom - top) };
}

/** @emoji 🪟️ Full-viewport veil pieces; optional rectangular cutout leaves the hovered app pane untinted. */
function demonstratorTintSegmentsPx(revealRect: RevealRectPx | null): readonly TintSegmentPx[] {
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  if (!revealRect) return [{ top: 0, left: 0, width: vw, height: vh }];
  const holeLeft = Math.max(0, revealRect.left);
  const holeTop = Math.max(0, revealRect.top);
  const holeRight = Math.min(vw, revealRect.left + revealRect.width);
  const holeBottom = Math.min(vh, revealRect.top + revealRect.height);
  if (holeRight <= holeLeft || holeBottom <= holeTop) return [{ top: 0, left: 0, width: vw, height: vh }];
  const segments: TintSegmentPx[] = [];
  if (holeTop > 0) segments.push({ top: 0, left: 0, width: vw, height: holeTop });
  if (holeBottom < vh) segments.push({ top: holeBottom, left: 0, width: vw, height: vh - holeBottom });
  if (holeLeft > 0) segments.push({ top: holeTop, left: 0, width: holeLeft, height: holeBottom - holeTop });
  if (holeRight < vw) segments.push({ top: holeTop, left: holeRight, width: vw - holeRight, height: holeBottom - holeTop });
  return segments;
}
//#endregion 🎪️DemonstratorGridGeometry

//#region 📱️DemonstratorMobileList
/** @emoji 🌫️ Touch overview keeps a full veil over each live pane — settled sections stay blurred so background apps never read clearly through the card. */
const DEMONSTRATOR_MOBILE_OVERVIEW_VEIL_OPACITY = 1;
//#endregion 📱️DemonstratorMobileList

//#region 🎪️DemonstratorPaneBoot
/** @emoji 🐢️ `requestIdleCallback` isn't universal (Safari); falls back to a short timeout so the warm-boot
 * queue still staggers instead of booting every pane synchronously back-to-back. */
function scheduleIdle(callback: () => void, timeoutMs: number): () => void {
  const withIdle = window as typeof window & { requestIdleCallback?: (cb: () => void, opts?: { timeout: number }) => number; cancelIdleCallback?: (handle: number) => void };
  if (withIdle.requestIdleCallback) {
    const handle = withIdle.requestIdleCallback(callback, { timeout: timeoutMs });
    return () => withIdle.cancelIdleCallback?.(handle);
  }
  const handle = window.setTimeout(callback, timeoutMs);
  return () => window.clearTimeout(handle);
}

/** @emoji 🐢️ Boots panes one at a time (hash-target pane first, if any) instead of all six simultaneously —
 * six live WASM plugin boots at once would make the very first paint of the page janky. `promote` lets a
 * hover/focus jump a not-yet-booted pane to the front, since the user is about to look at it right now. */
function useSequentialPaneBoot(
  initialFocusId: string | null,
  options?: { readonly skipIdleQueue?: boolean },
): { readonly bootedIds: ReadonlySet<string>; readonly promote: (id: string) => void } {
  const [bootedIds, setBootedIds] = useState<ReadonlySet<string>>(() => new Set(initialFocusId ? [initialFocusId] : []));
  const queueRef = useRef<string[]>(DEMONSTRATOR_PANES.map((pane) => pane.id).filter((id) => id !== initialFocusId));
  const cancelRef = useRef<(() => void) | null>(null);
  const skipIdleQueue = options?.skipIdleQueue ?? false;

  const boot = useCallback((id: string) => {
    setBootedIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
    queueRef.current = queueRef.current.filter((queuedId) => queuedId !== id);
  }, []);

  useEffect(() => {
    if (skipIdleQueue) return;
    const bootNext = () => {
      const nextId = queueRef.current[0];
      if (!nextId) return;
      boot(nextId);
      cancelRef.current = scheduleIdle(bootNext, 1500);
    };
    cancelRef.current = scheduleIdle(bootNext, 1500);
    return () => cancelRef.current?.();
  }, [boot, skipIdleQueue]);

  const promote = useCallback((id: string) => boot(id), [boot]);
  return { bootedIds, promote };
}
//#endregion 🎪️DemonstratorPaneBoot

//#region 🎪️DemonstratorSuspension
/** @emoji 🎪️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: a booted pane that's fully offscreen or the
 * tab is backgrounded releases its live shell (plugin worker, WASM instances, WebGL contexts — see
 * the framework's teardown-on-unmount path) and shows a static poster instead, revived instantly on
 * hover/focus. Only PRISTINE panes (never interacted with) are ever suspended: there is no document
 * round-trip yet (`readAppDocument`/`loadAppDocument` are an unimplemented, documented Wave-1 gap in
 * the framework core), so suspending a pane the user actually used would silently discard their work.
 * This covers exactly the idle-tab case the demonstrator is mostly used for (an unattended
 * kiosk/booth screen) without any feature loss for interactive use — see `DemonstratorPane`'s
 * `onPointerDownCapture`/`onKeyDownCapture`, which permanently exempt a pane the moment it's touched. */
const DEMONSTRATOR_SUSPENSION_POLICY = {
  /** Booted pane fully offscreen (another pane is focused) — safe to release quickly. */
  offscreenSuspendDelayMs: 30_000,
  /** Booted pane sitting idle on the overview grid (nothing focused, no recent input). */
  overviewIdleSuspendMs: 5 * 60_000,
  /** Tab backgrounded — release aggressively regardless of the other two timers. */
  hiddenTabSuspendMs: 60_000,
  /** How often the suspension sweep re-evaluates every booted pane. */
  sweepIntervalMs: 5_000,
} as const;

/** @emoji 🖼️ Composites every canvas inside a pane's container into one offscreen 2D canvas and
 * returns it as a data URL — must run synchronously (not after a `requestAnimationFrame`, by which
 * point a `preserveDrawingBuffer: false` WebGL backbuffer may already be cleared). Returns `null`
 * when the pane has no canvases yet or every one samples blank; callers fall back to the existing
 * "wird vorbereitet" placeholder visual in that case — no new failure mode. */
function capturePanePoster(container: HTMLElement): string | null {
  const canvases = container.querySelectorAll("canvas");
  if (canvases.length === 0) return null;
  const containerRect = container.getBoundingClientRect();
  if (containerRect.width <= 0 || containerRect.height <= 0) return null;
  const poster = document.createElement("canvas");
  poster.width = Math.round(containerRect.width);
  poster.height = Math.round(containerRect.height);
  const ctx = poster.getContext("2d");
  if (!ctx) return null;
  let drewSomething = false;
  canvases.forEach((canvas) => {
    if (canvas.width === 0 || canvas.height === 0) return;
    const rect = canvas.getBoundingClientRect();
    try {
      ctx.drawImage(canvas, rect.left - containerRect.left, rect.top - containerRect.top, rect.width, rect.height);
      drewSomething = true;
    } catch {
      /* tainted canvas or a lost GPU context — skip it; other canvases (or the placeholder fallback) still work */
    }
  });
  if (!drewSomething) return null;
  try {
    return poster.toDataURL("image/png");
  } catch {
    return null;
  }
}

/** @emoji 🎪️ Tracks which booted panes are pristine (never interacted with), suspended (poster shown,
 * live shell released), and their captured posters — plus the sweep that suspends eligible panes on
 * the {@link DEMONSTRATOR_SUSPENSION_POLICY} schedule. `focusedId` and (while nothing is focused) the
 * most-recently-focused pane are always exempt, matching the policy's `keepLiveCount: 1`. */
function usePaneSuspension(bootedIds: ReadonlySet<string>, focusedId: string | null, initialFocusId: string | null): {
  readonly dirtyIds: ReadonlySet<string>;
  readonly suspendedIds: ReadonlySet<string>;
  readonly postersById: ReadonlyMap<string, string>;
  readonly markDirty: (id: string) => void;
  readonly registerContainer: (id: string, el: HTMLDivElement | null) => void;
  readonly resumePane: (id: string) => void;
} {
  const [dirtyIds, setDirtyIds] = useState<ReadonlySet<string>>(new Set());
  const [suspendedIds, setSuspendedIds] = useState<ReadonlySet<string>>(new Set());
  const [postersById, setPostersById] = useState<ReadonlyMap<string, string>>(new Map());
  const containersRef = useRef<Map<string, HTMLDivElement>>(new Map());
  const unfocusedSinceRef = useRef<Map<string, number>>(new Map());
  const prevFocusedIdRef = useRef<string | null>(focusedId);
  const mostRecentFocusedIdRef = useRef<string | null>(initialFocusId ?? DEMONSTRATOR_PANES[0]?.id ?? null);

  const markDirty = useCallback((id: string) => {
    setDirtyIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
  }, []);

  const registerContainer = useCallback((id: string, el: HTMLDivElement | null) => {
    if (el) containersRef.current.set(id, el);
    else containersRef.current.delete(id);
  }, []);

  const suspendPane = useCallback((id: string) => {
    const container = containersRef.current.get(id);
    if (container) {
      const poster = capturePanePoster(container);
      if (poster) setPostersById((prev) => new Map(prev).set(id, poster));
    }
    setSuspendedIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
  }, []);

  const resumePane = useCallback((id: string) => {
    unfocusedSinceRef.current.delete(id);
    setSuspendedIds((prev) => {
      if (!prev.has(id)) return prev;
      const next = new Set(prev);
      next.delete(id);
      return next;
    });
  }, []);

  // Records when a pane most recently stopped being focused (absence from the map = currently focused).
  useEffect(() => {
    const prev = prevFocusedIdRef.current;
    if (prev && prev !== focusedId) unfocusedSinceRef.current.set(prev, Date.now());
    if (focusedId) {
      unfocusedSinceRef.current.delete(focusedId);
      mostRecentFocusedIdRef.current = focusedId;
    }
    prevFocusedIdRef.current = focusedId;
  }, [focusedId]);

  // A pane booted via the warm-boot queue (never focused) still needs an "unfocused since" baseline.
  useEffect(() => {
    for (const id of bootedIds) {
      if (id !== focusedId && !unfocusedSinceRef.current.has(id)) unfocusedSinceRef.current.set(id, Date.now());
    }
  }, [bootedIds, focusedId]);

  useEffect(() => {
    const sweep = () => {
      const now = Date.now();
      const hidden = document.hidden;
      for (const id of bootedIds) {
        if (id === focusedId) continue;
        if (!focusedId && id === mostRecentFocusedIdRef.current) continue;
        if (dirtyIds.has(id) || suspendedIds.has(id)) continue;
        const since = unfocusedSinceRef.current.get(id);
        if (since == null) continue;
        const threshold = hidden ? DEMONSTRATOR_SUSPENSION_POLICY.hiddenTabSuspendMs : focusedId ? DEMONSTRATOR_SUSPENSION_POLICY.offscreenSuspendDelayMs : DEMONSTRATOR_SUSPENSION_POLICY.overviewIdleSuspendMs;
        if (now - since >= threshold) suspendPane(id);
      }
    };
    const interval = window.setInterval(sweep, DEMONSTRATOR_SUSPENSION_POLICY.sweepIntervalMs);
    return () => window.clearInterval(interval);
  }, [bootedIds, focusedId, dirtyIds, suspendedIds, suspendPane]);

  return { dirtyIds, suspendedIds, postersById, markDirty, registerContainer, resumePane };
}
//#endregion 🎪️DemonstratorSuspension

//#region 🎪️PaneErrorBoundary
/** @emoji 🛟️ One pane crashing (a plugin boot failure, a render error) must never take down the other five
 * or the landing chrome around them — React error boundaries are the only mechanism that can catch a
 * render-phase throw, and they must be class components. */
type PaneErrorBoundaryProps = { readonly paneLabel: string; readonly children: UiNode };
type PaneErrorBoundaryState = { readonly error: Error | null };

const PaneErrorBoundary = createUiErrorBoundary<PaneErrorBoundaryProps, PaneErrorBoundaryState>({
  initialState: { error: null },
  deriveState: (error) => ({ error }),
  didCatch: (props, error) => console.error(`[DEBUG] demonstrator pane "${props.paneLabel}" crashed`, error),
  render: (props, state) => {
    if (state.error) {
      return (
        <div className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
          {props.paneLabel} konnte nicht geladen werden.
        </div>
      );
    }
    return props.children;
  },
});
//#endregion 🎪️PaneErrorBoundary

//#region 🎪️DemonstratorPane
/** @emoji 🎪️ One grid cell: either the brand-logo placeholder (not booted yet) or the live shell, wrapped
 * `inert` while not focused so it never steals pointer/keyboard/focus from whichever pane IS focused (or
 * from the overview's own hover cards) — the shell still renders and animates underneath, just inertly. */
function DemonstratorPane({
  pane,
  booted,
  focused,
  suspended,
  posterDataUrl,
  onDirty,
  onContainerElement,
}: {
  readonly pane: DemonstratorPaneSpec;
  readonly booted: boolean;
  readonly focused: boolean;
  readonly suspended: boolean;
  readonly posterDataUrl: string | null;
  readonly onDirty: () => void;
  readonly onContainerElement: (id: string, el: HTMLDivElement | null) => void;
}) {
  const bootVariant = demonstratorPaneRuntimeVariant(pane.variant);
  const boot = useMemo(() => resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariant), [bootVariant]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);
  const live = booted && !suspended;

  return (
    <div
      ref={(el) => onContainerElement(pane.id, el)}
      className="relative h-full w-full overflow-hidden bg-background"
      inert={!focused}
      // 🎪️ Only a focused (non-inert) pane can ever actually receive these — capture-phase so a click
      // deep inside the shell (a button, a canvas) still marks the pane dirty before anything inside
      // it can stop propagation. See `usePaneSuspension`'s docstring for why this permanently exempts
      // the pane from suspension.
      onPointerDownCapture={onDirty}
      onKeyDownCapture={onDirty}
    >
      {live ? (
        <PaneErrorBoundary paneLabel={pane.label}>
          <FrameworkOsShell
            pluginFilter={bootVariant}
            plugins={boot.plugins}
            appId={boot.defaultAppId}
            locks={locks}
            defaults={defaults}
            brand={pane.brand}
            shellId={pane.id}
            storageNamespace={pane.id}
            suppressAutoIntroduction={!focused}
          />
        </PaneErrorBoundary>
      ) : booted && suspended && posterDataUrl ? (
        <img src={posterDataUrl} alt="" className="h-full w-full object-cover" aria-hidden />
      ) : (
        <div className={cn("flex h-full w-full flex-col items-center justify-center gap-double bg-background", loadingBorderClass)} role="status" aria-busy="true">
          <div className="size-huge text-foreground opacity-40 [&_svg]:h-full [&_svg]:w-full" dangerouslySetInnerHTML={{ __html: ENTWERFEN_MIT_BESTAND_LOGO_SVG }} aria-hidden />
          <div className="h-full min-h-0 w-full max-w-4xl flex-1 p-double">
            <CanvasSkeleton label={`${pane.label} wird vorbereitet`} />
          </div>
        </div>
      )}
    </div>
  );
}
//#endregion 🎪️DemonstratorPane

//#region 🎪️DemonstratorCard
/** @emoji 🃏️ Shared glass card for desktop grid cells and mobile list sections. */
function DemonstratorCard({
  pane,
  active,
  onClick,
  onMouseEnter,
  onMouseLeave,
  className,
}: {
  readonly pane: DemonstratorPaneSpec;
  readonly active?: boolean;
  readonly onClick: () => void;
  readonly onMouseEnter?: () => void;
  readonly onMouseLeave?: () => void;
  readonly className?: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      className={cn(
        "pointer-events-auto group flex min-h-[8.5rem] w-full max-w-[15rem] flex-col items-center justify-center gap-single",
        "rounded-xl border border-border-normal px-double py-triple text-center",
        "ui-glass shadow-md outline-none",
        "transition-[transform,box-shadow,border-color,background-color] duration-200",
        "hover:-translate-y-0.5 hover:border-border-emphasized hover:shadow-xl",
        "focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
        active && "-translate-y-0.5 border-border-emphasized shadow-xl",
        className,
      )}
    >
      <span className="flex size-workbench shrink-0 items-center justify-center rounded-md border border-border-normal/80 bg-background/50">
        <Icon icon={pane.icon} size="large" className="text-muted-foreground group-hover:text-foreground" title={pane.label} />
      </span>
      <span className="flex flex-col gap-half">
        <span className="text-2xl font-semibold tracking-tight text-foreground">{pane.label}</span>
        <span className="text-sm text-muted-foreground">{pane.tagline}</span>
      </span>
      <span className="inline-flex items-center gap-single text-sm font-medium text-muted-foreground transition-colors group-hover:text-foreground">
        Demonstrator öffnen
        <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5" />
      </span>
    </button>
  );
}
//#endregion 🎪️DemonstratorCard

//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
  const viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const touchListMode = useMediaQuery(DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY);

  const surfaceChrome = useMemo(
    () => ({
      appearance: readStoredUiChromeAppearance(demonstratorStorage),
      device: (viewportMobile ? "mobile" : readStoredUiChromeLayout(demonstratorStorage) === "tablet" ? "tablet" : "desktop") as const,
      driver: readStoredUiDriver(demonstratorStorage),
    }),
    [viewportMobile],
  );
  useElementsSurfaceChrome(surfaceChrome);

  const initialFocusId = useMemo(() => paneIdFromLocationHash(), []);
  const { bootedIds, promote } = useSequentialPaneBoot(initialFocusId, { skipIdleQueue: touchListMode });

  const [introductionStep, setIntroductionStep] = useState(0);
  const [showIntroduction, setShowIntroduction] = useState(!initialFocusId);
  const [focusedId, setFocusedId] = useState<string | null>(initialFocusId);
  const { suspendedIds, postersById, markDirty, registerContainer, resumePane } = usePaneSuspension(bootedIds, focusedId, initialFocusId);
  const promoteAndResume = useCallback(
    (id: string) => {
      promote(id);
      resumePane(id);
    },
    [promote, resumePane],
  );
  const [hoveredPaneId, setHoveredPaneId] = useState<string | null>(null);
  const [revealRect, setRevealRect] = useState<RevealRectPx | null>(null);
  const hoveredPaneIdRef = useRef<string | null>(null);
  const scrollTargetRef = useRef<ScrollOffset>(initialFocusId ? scrollOffsetForPaneIndex(paneIndexById(initialFocusId)) : { x: 0, y: 0 });
  const scrollCurrentRef = useRef<ScrollOffset>(scrollTargetRef.current);
  const scrollDriveRef = useRef<ScrollDrive>({ mode: "follow" });
  /** @emoji 🎞 Bumped on every drive change so a stale follow `setScrollOffset` cannot paint after a glide has taken ownership. */
  const scrollEpochRef = useRef(0);
  const [scrollOffset, setScrollOffset] = useState<ScrollOffset>(scrollTargetRef.current);
  const listScrollRef = useRef<HTMLDivElement>(null);
  const listProgressRafRef = useRef<number | null>(null);
  const [listProgress, setListProgress] = useState(0);
  const [listScrollLocked, setListScrollLocked] = useState(Boolean(initialFocusId));
  // 📽 One rAF loop drives the grid transform; it stops once settled so an idle tab isn't animating forever.
  // `ensureScrollLoopRef` restarts it whenever follow or glide needs another frame.
  const scrollLoopRunningRef = useRef(false);
  const ensureScrollLoopRef = useRef<() => void>(() => {});

  const commitScrollOffset = useCallback((epoch: number, next: ScrollOffset) => {
    scrollCurrentRef.current = next;
    setScrollOffset((prev) => {
      if (scrollEpochRef.current !== epoch) return prev;
      return prev.x === next.x && prev.y === next.y ? prev : next;
    });
  }, []);

  const applyPaneScroll = useCallback(
    (paneIndex: number) => {
      const offset = scrollOffsetForPaneIndex(paneIndex);
      const from = scrollCurrentRef.current;
      scrollTargetRef.current = offset;
      scrollEpochRef.current += 1;
      const epoch = scrollEpochRef.current;
      if (Math.abs(offset.x - from.x) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON && Math.abs(offset.y - from.y) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON) {
        scrollDriveRef.current = { mode: "follow" };
        commitScrollOffset(epoch, offset);
        return;
      }
      scrollDriveRef.current = {
        mode: "glide",
        from: { x: from.x, y: from.y },
        to: offset,
        startedAt: performance.now(),
        durationMs: DEMONSTRATOR_SCROLL_GLIDE_MS,
      };
      // Pin the origin immediately so any already-queued follow paint is superseded before the first glide frame.
      commitScrollOffset(epoch, from);
      ensureScrollLoopRef.current();
    },
    [commitScrollOffset],
  );

  const scrollListToPaneIndex = useCallback((paneIndex: number) => {
    const el = listScrollRef.current;
    if (!el) return;
    const height = el.clientHeight;
    if (height <= 0) return;
    el.scrollTop = paneIndex * height;
    setListProgress(paneIndex);
  }, []);

  const focusPane = useCallback(
    (id: string) => {
      promoteAndResume(id);
      setFocusedId(id);
      setShowIntroduction(false);
      hoveredPaneIdRef.current = null;
      setHoveredPaneId(null);
      setRevealRect(null);
      const paneIndex = paneIndexById(id);
      if (touchListMode) {
        scrollListToPaneIndex(paneIndex);
        setListScrollLocked(true);
      } else {
        applyPaneScroll(paneIndex);
      }
      window.history.replaceState(null, "", `#${id}`);
    },
    [promoteAndResume, applyPaneScroll, touchListMode, scrollListToPaneIndex],
  );

  const returnToOverview = useCallback(() => {
    const previousFocus = focusedId;
    setFocusedId(null);
    if (touchListMode) {
      setListScrollLocked(false);
      if (previousFocus) {
        const paneIndex = paneIndexById(previousFocus);
        if (paneIndex >= 0) requestAnimationFrame(() => scrollListToPaneIndex(paneIndex));
      }
    }
    window.history.replaceState(null, "", window.location.pathname + window.location.search);
  }, [touchListMode, focusedId, scrollListToPaneIndex]);

  useEffect(() => {
    const onHashChange = () => {
      const paneId = paneIdFromLocationHash();
      if (paneId) focusPane(paneId);
      else returnToOverview();
    };
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  }, [focusPane, returnToOverview]);

  useEffect(() => {
    if (!focusedId) return;
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") returnToOverview();
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [focusedId, returnToOverview]);

  useEffect(() => {
    if (!touchListMode || !initialFocusId) return;
    const paneIndex = paneIndexById(initialFocusId);
    if (paneIndex < 0) return;
    requestAnimationFrame(() => scrollListToPaneIndex(paneIndex));
  }, [touchListMode, initialFocusId, scrollListToPaneIndex]);

  const handleListScroll = useCallback(() => {
    const el = listScrollRef.current;
    if (!el || listScrollLocked) return;
    if (listProgressRafRef.current != null) return;
    listProgressRafRef.current = requestAnimationFrame(() => {
      listProgressRafRef.current = null;
      const height = el.clientHeight;
      if (height > 0) setListProgress(el.scrollTop / height);
    });
  }, [listScrollLocked]);

  useEffect(() => {
    if (!touchListMode || focusedId) return;
    const current = Math.round(listProgress);
    const pane = DEMONSTRATOR_PANES[current];
    if (pane) promoteAndResume(pane.id);
    const next = DEMONSTRATOR_PANES[current + 1];
    if (next) promoteAndResume(next.id);
  }, [touchListMode, listProgress, focusedId, promoteAndResume]);

  const refreshRevealRect = useCallback((paneId: string | null, offset: ScrollOffset) => {
    if (!paneId) {
      setRevealRect(null);
      return;
    }
    const paneIndex = paneIndexById(paneId);
    if (paneIndex < 0) {
      setRevealRect(null);
      return;
    }
    setRevealRect(demonstratorPaneRevealRect(paneIndex, offset));
  }, []);

  const tintSegments = useMemo(() => demonstratorTintSegmentsPx(revealRect), [revealRect]);

  useEffect(() => {
    if (touchListMode) return;
    const onResize = () => {
      if (hoveredPaneId) refreshRevealRect(hoveredPaneId, scrollCurrentRef.current);
      else setRevealRect(null);
    };
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, [touchListMode, hoveredPaneId, refreshRevealRect]);

  // 🖱️ Mouse-follow panning only makes sense in overview — a focused pane owns the mouse.
  useEffect(() => {
    if (touchListMode || focusedId) return;
    const onMove = (event: MouseEvent) => {
      if (hoveredPaneIdRef.current) return;
      if (scrollDriveRef.current.mode !== "follow") scrollEpochRef.current += 1;
      scrollDriveRef.current = { mode: "follow" };
      scrollTargetRef.current = {
        x: (event.clientX / window.innerWidth) * DEMONSTRATOR_MAX_SCROLL.x,
        y: (event.clientY / window.innerHeight) * DEMONSTRATOR_MAX_SCROLL.y,
      };
      ensureScrollLoopRef.current();
    };
    window.addEventListener("mousemove", onMove, { passive: true });
    return () => window.removeEventListener("mousemove", onMove);
  }, [touchListMode, focusedId]);

  useEffect(() => {
    if (touchListMode) return;
    let frame = 0;
    const tick = (_frameTime: number) => {
      // Use `performance.now()` (same clock as glide `startedAt`) — the rAF timestamp can lag slightly
      // behind and yield a negative ease `t`, which lerps backward for one frame (visible flicker).
      const now = performance.now();
      const epoch = scrollEpochRef.current;
      const drive = scrollDriveRef.current;
      if (drive.mode === "glide") {
        const t = Math.min(1, Math.max(0, (now - drive.startedAt) / drive.durationMs));
        const next = lerpScrollOffset(drive.from, drive.to, easeInOutCubic(t));
        commitScrollOffset(epoch, next);
        if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, next);
        if (t >= 1) {
          scrollTargetRef.current = drive.to;
          scrollDriveRef.current = { mode: "follow" };
          scrollLoopRunningRef.current = false;
          return;
        }
        frame = requestAnimationFrame(tick);
        return;
      }
      const current = scrollCurrentRef.current;
      const target = scrollTargetRef.current;
      if (Math.abs(target.x - current.x) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON && Math.abs(target.y - current.y) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON) {
        if (current.x !== target.x || current.y !== target.y) {
          commitScrollOffset(epoch, target);
          if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, target);
        }
        scrollLoopRunningRef.current = false;
        return;
      }
      const next = lerpScrollOffset(current, target, DEMONSTRATOR_SCROLL_FOLLOW_LERP);
      // A focus/hover glide may have started after this follow sample was computed — never let the stale
      // follow frame yank the transform backward for one paint (that reads as a flicker).
      if (scrollDriveRef.current.mode === "glide" || scrollEpochRef.current !== epoch) {
        frame = requestAnimationFrame(tick);
        return;
      }
      commitScrollOffset(epoch, next);
      if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, next);
      frame = requestAnimationFrame(tick);
    };
    ensureScrollLoopRef.current = () => {
      if (scrollLoopRunningRef.current) return;
      scrollLoopRunningRef.current = true;
      frame = requestAnimationFrame(tick);
    };
    ensureScrollLoopRef.current();
    return () => {
      scrollLoopRunningRef.current = false;
      cancelAnimationFrame(frame);
    };
  }, [touchListMode, refreshRevealRect, commitScrollOffset]);

  const dismissIntroduction = useCallback((_completed: boolean) => {
    setShowIntroduction(false);
  }, []);

  const overviewChrome = (
    <>
      {!hoveredPaneId && (
        <div className="pointer-events-none absolute inset-x-0 bottom-0 z-40 flex items-center justify-between gap-tiny px-double py-single">
          <div className="pointer-events-auto">{aProjectOfLuhUdkFooterItem("landingProjectOf", "de", false).content}</div>
          <div className="pointer-events-auto">{fundedByZukunftBauFooterItem("landingFundedBy", "de", false).content}</div>
        </div>
      )}

      {showIntroduction && (
        <UIIntroduction
          introduction={ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION}
          stepIndex={introductionStep}
          completedInteractionIndices={[]}
          onStepIndexChange={setIntroductionStep}
          onDismiss={dismissIntroduction}
        />
      )}

      {!hoveredPaneId && (
        <Navbar
          items={[
            {
              key: "logoAndTitle",
              centered: true,
              content: (
                <div className="flex min-w-0 shrink-0 items-center gap-single">
                  <ShellBrandLogo svg={ENTWERFEN_MIT_BESTAND_LOGO_SVG} className="size-workbench shrink-0" />
                  <span data-slot="app-name" className="px-single text-sm font-semibold text-foreground">
                    Entwerfen mit Bestand
                  </span>
                </div>
              ),
            },
          ]}
          showFullscreenToggle={false}
          className="pointer-events-none absolute inset-x-0 top-0 z-40 bg-transparent"
        />
      )}
    </>
  );

  const overviewReturnButton = focusedId ? (
    <button
      type="button"
      onClick={returnToOverview}
      className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single rounded-md border border-border-normal px-single py-half text-sm font-medium text-foreground shadow-md outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring"
    >
      <Icon icon="layout-grid" size="small" />
      Übersicht
    </button>
  ) : null;

  if (touchListMode) {
    return (
      <div className="relative h-full w-full overflow-hidden bg-background text-foreground">
        <div
          ref={listScrollRef}
          data-demonstrator-list-scroll=""
          onScroll={handleListScroll}
          className={cn(
            "flex w-full flex-col overscroll-y-contain",
            listScrollLocked ? "overflow-hidden" : "snap-y snap-mandatory overflow-y-auto",
          )}
          style={{ height: "100dvh" }}
        >
          {DEMONSTRATOR_PANES.map((pane, paneIndex) => {
            const isFocused = focusedId === pane.id;
            const showOverviewLayer = !focusedId;
            const veilOpacity = showOverviewLayer ? DEMONSTRATOR_MOBILE_OVERVIEW_VEIL_OPACITY : 0;
            return (
              <section key={pane.id} className="relative w-full shrink-0 snap-start overflow-hidden" style={{ height: "100dvh", minHeight: "100dvh" }}>
                <DemonstratorPane
                  pane={pane}
                  booted={bootedIds.has(pane.id)}
                  focused={isFocused}
                  suspended={suspendedIds.has(pane.id)}
                  posterDataUrl={postersById.get(pane.id) ?? null}
                  onDirty={() => markDirty(pane.id)}
                  onContainerElement={registerContainer}
                />
                {showOverviewLayer && (
                  <>
                    <div className="pointer-events-none absolute inset-0 z-30">
                      <div className="ui-veil absolute inset-0" style={{ opacity: veilOpacity }} />
                    </div>
                    <div className="pointer-events-none absolute inset-0 z-[31] flex items-center justify-center px-double pb-[5.5rem]">
                      <DemonstratorCard pane={pane} className="max-w-[18rem]" onClick={() => focusPane(pane.id)} />
                    </div>
                  </>
                )}
              </section>
            );
          })}
        </div>

        {!focusedId && overviewChrome}

        {overviewReturnButton}
      </div>
    );
  }

  return (
    <div className="relative h-full w-full overflow-hidden bg-background text-foreground">
      <div
        className="grid"
        style={{
          gridTemplateColumns: `repeat(${DEMONSTRATOR_GRID_COLUMNS}, 100vw)`,
          gridTemplateRows: `repeat(${DEMONSTRATOR_GRID_ROWS}, 100vh)`,
          width: `${DEMONSTRATOR_GRID_COLUMNS * 100}vw`,
          height: `${DEMONSTRATOR_GRID_ROWS * 100}vh`,
          transform: `translate(-${scrollOffset.x}vw, -${scrollOffset.y}vh)`,
        }}
      >
        {DEMONSTRATOR_PANES.map((pane) => (
          <DemonstratorPane
            key={pane.id}
            pane={pane}
            booted={bootedIds.has(pane.id)}
            focused={focusedId === pane.id}
            suspended={suspendedIds.has(pane.id)}
            posterDataUrl={postersById.get(pane.id) ?? null}
            onDirty={() => markDirty(pane.id)}
            onContainerElement={registerContainer}
          />
        ))}
      </div>

      {!focusedId && (
        <>
          <div className="pointer-events-none absolute inset-0 z-30">
            {tintSegments.map((segment, index) => (
              <div key={`tint-${index}-${segment.top}-${segment.left}`} className="ui-veil absolute" style={{ top: segment.top, left: segment.left, width: segment.width, height: segment.height }} />
            ))}
          </div>

          <div
            className="pointer-events-none absolute inset-0 z-[31] grid items-center"
            style={{ gridTemplateColumns: `repeat(${DEMONSTRATOR_GRID_COLUMNS}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${DEMONSTRATOR_GRID_ROWS}, minmax(0, 1fr))` }}
          >
            {DEMONSTRATOR_PANES.map((pane, paneIndex) => {
              const active = hoveredPaneId === pane.id;
              return (
                <div key={pane.id} className="flex justify-center px-double">
                  <DemonstratorCard
                    pane={pane}
                    active={active}
                    onClick={() => focusPane(pane.id)}
                    onMouseEnter={() => {
                      hoveredPaneIdRef.current = pane.id;
                      setHoveredPaneId(pane.id);
                      promoteAndResume(pane.id);
                      applyPaneScroll(paneIndex);
                      refreshRevealRect(pane.id, scrollOffsetForPaneIndex(paneIndex));
                    }}
                    onMouseLeave={() => {
                      if (hoveredPaneIdRef.current === pane.id) {
                        hoveredPaneIdRef.current = null;
                        setHoveredPaneId(null);
                        setRevealRect(null);
                      }
                    }}
                  />
                </div>
              );
            })}
          </div>

          {overviewChrome}
        </>
      )}

      {overviewReturnButton}
    </div>
  );
}
//#endregion 🎪️DemonstratorLanding

mountUiRoot(document.getElementById("root")!, <DemonstratorLanding />);
