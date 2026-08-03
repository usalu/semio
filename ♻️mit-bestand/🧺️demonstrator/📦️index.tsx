// #region 🧲️Header
/** @emoji 🎪️ Entwerfen mit Bestand demonstrator landing — general introduction, six live app panes, glass name overlay. */
// #endregion 🧲️Header

import { createRoot } from "react-dom/client";
import { Component, useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import {
  UIIntroduction,
  bootstrapElementsSurfaceChromeDocument,
  cn,
  Icon,
  initUiLocaleSync,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiDriver,
  useElementsSurfaceChrome,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort, resolvePlaygroundBoot } from "@semio-tech/framework-core";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "./⚛️footer.tsx";
import { DEMONSTRATOR_LOCALE, DEMONSTRATOR_PANES, ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION, ENTWERFEN_MIT_BESTAND_LOGO_SVG, type DemonstratorPaneSpec } from "./🟦️brand.ts";
import "./🎨️globals.css";

// 🎪️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
const demonstratorStorage = createBrowserStoragePort();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(demonstratorStorage));
// 🇩🇪️ The whole demonstrator is German-locked (see 🟦️brand.ts) — resolve synchronously before the
// first render so the landing page's own chrome (Skip/Back/Next/Done) never flashes English.
initUiLocaleSync(DEMONSTRATOR_LOCALE);

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

/** @emoji 🧭️ Scroll offset that brings the given pane fully into the viewport. */
function scrollOffsetForPaneIndex(paneIndex: number): ScrollOffset {
  return {
    x: Math.min(DEMONSTRATOR_MAX_SCROLL.x, Math.max(0, paneColumn(paneIndex) * 100)),
    y: Math.min(DEMONSTRATOR_MAX_SCROLL.y, Math.max(0, paneRow(paneIndex) * 100)),
  };
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
  return segments.length > 0 ? segments : [{ top: 0, left: 0, width: vw, height: vh }];
}
//#endregion 🎪️DemonstratorGridGeometry

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
function useSequentialPaneBoot(initialFocusId: string | null): { readonly bootedIds: ReadonlySet<string>; readonly promote: (id: string) => void } {
  const [bootedIds, setBootedIds] = useState<ReadonlySet<string>>(() => new Set(initialFocusId ? [initialFocusId] : []));
  const queueRef = useRef<string[]>(DEMONSTRATOR_PANES.map((pane) => pane.id).filter((id) => id !== initialFocusId));
  const cancelRef = useRef<(() => void) | null>(null);

  const boot = useCallback((id: string) => {
    setBootedIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
    queueRef.current = queueRef.current.filter((queuedId) => queuedId !== id);
  }, []);

  useEffect(() => {
    const bootNext = () => {
      const nextId = queueRef.current[0];
      if (!nextId) return;
      boot(nextId);
      cancelRef.current = scheduleIdle(bootNext, 1500);
    };
    cancelRef.current = scheduleIdle(bootNext, 1500);
    return () => cancelRef.current?.();
  }, [boot]);

  const promote = useCallback((id: string) => boot(id), [boot]);
  return { bootedIds, promote };
}
//#endregion 🎪️DemonstratorPaneBoot

//#region 🎪️PaneErrorBoundary
/** @emoji 🛟️ One pane crashing (a plugin boot failure, a render error) must never take down the other five
 * or the landing chrome around them — React error boundaries are the only mechanism that can catch a
 * render-phase throw, and they must be class components. */
class PaneErrorBoundary extends Component<{ readonly paneLabel: string; readonly children: ReactNode }, { readonly error: Error | null }> {
  constructor(props: { readonly paneLabel: string; readonly children: ReactNode }) {
    super(props);
    this.state = { error: null };
  }
  static getDerivedStateFromError(error: Error): { readonly error: Error } {
    return { error };
  }
  override componentDidCatch(error: Error): void {
    console.error(`[DEBUG] demonstrator pane "${this.props.paneLabel}" crashed`, error);
  }
  override render(): ReactNode {
    if (this.state.error) {
      return (
        <div className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
          {this.props.paneLabel} konnte nicht geladen werden.
        </div>
      );
    }
    return this.props.children;
  }
}
//#endregion 🎪️PaneErrorBoundary

//#region 🎪️DemonstratorPane
/** @emoji 🎪️ One grid cell: either the brand-logo placeholder (not booted yet) or the live shell, wrapped
 * `inert` while not focused so it never steals pointer/keyboard/focus from whichever pane IS focused (or
 * from the overview's own hover cards) — the shell still renders and animates underneath, just inertly. */
function DemonstratorPane({ pane, booted, focused }: { readonly pane: DemonstratorPaneSpec; readonly booted: boolean; readonly focused: boolean }) {
  const boot = useMemo(() => resolvePlaygroundBoot(pane.variant), [pane.variant]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);

  return (
    <div className="relative h-full w-full overflow-hidden bg-background" inert={!focused}>
      {booted ? (
        <PaneErrorBoundary paneLabel={pane.label}>
          <FrameworkOsShell
            pluginFilter={pane.variant}
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
      ) : (
        <div className="flex h-full w-full flex-col items-center justify-center gap-double bg-background">
          <div className="size-huge text-foreground opacity-40 [&_svg]:h-full [&_svg]:w-full" dangerouslySetInnerHTML={{ __html: ENTWERFEN_MIT_BESTAND_LOGO_SVG }} aria-hidden />
          <span className="text-sm text-muted-foreground">{pane.label} wird vorbereitet …</span>
        </div>
      )}
    </div>
  );
}
//#endregion 🎪️DemonstratorPane

//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
  const surfaceChrome = useMemo(
    () => ({
      appearance: readStoredUiChromeAppearance(demonstratorStorage),
      device: (readStoredUiChromeLayout(demonstratorStorage) === "tablet" ? "tablet" : "desktop") as const,
      driver: readStoredUiDriver(demonstratorStorage),
    }),
    [],
  );
  useElementsSurfaceChrome(surfaceChrome);

  const initialFocusId = useMemo(() => paneIdFromLocationHash(), []);
  const { bootedIds, promote } = useSequentialPaneBoot(initialFocusId);

  const [introductionStep, setIntroductionStep] = useState(0);
  const [showIntroduction, setShowIntroduction] = useState(!initialFocusId);
  const [focusedId, setFocusedId] = useState<string | null>(initialFocusId);
  const [hoveredPaneId, setHoveredPaneId] = useState<string | null>(null);
  const [revealRect, setRevealRect] = useState<RevealRectPx | null>(null);
  const hoveredPaneIdRef = useRef<string | null>(null);
  const scrollTargetRef = useRef<ScrollOffset>(initialFocusId ? scrollOffsetForPaneIndex(paneIndexById(initialFocusId)) : { x: 0, y: 0 });
  const scrollCurrentRef = useRef<ScrollOffset>(scrollTargetRef.current);
  const [scrollOffset, setScrollOffset] = useState<ScrollOffset>(scrollTargetRef.current);
  // 🪶️ Easing loop is stopped once settled so an idle tab isn't animating forever;
  // `ensureEasingLoopRef` restarts it whenever a new scroll target arrives.
  const easingRunningRef = useRef(false);
  const ensureEasingLoopRef = useRef<() => void>(() => {});

  const applyPaneScroll = useCallback((paneIndex: number) => {
    const offset = scrollOffsetForPaneIndex(paneIndex);
    scrollTargetRef.current = offset;
    ensureEasingLoopRef.current();
  }, []);

  const focusPane = useCallback(
    (id: string) => {
      promote(id);
      setFocusedId(id);
      setShowIntroduction(false);
      hoveredPaneIdRef.current = null;
      setHoveredPaneId(null);
      setRevealRect(null);
      applyPaneScroll(paneIndexById(id));
      window.history.replaceState(null, "", `#${id}`);
    },
    [promote, applyPaneScroll],
  );

  const returnToOverview = useCallback(() => {
    setFocusedId(null);
    window.history.replaceState(null, "", window.location.pathname + window.location.search);
  }, []);

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
    const onResize = () => {
      if (hoveredPaneId) refreshRevealRect(hoveredPaneId, scrollCurrentRef.current);
      else setRevealRect(null);
    };
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, [hoveredPaneId, refreshRevealRect]);

  // 🖱️ Mouse-follow panning only makes sense in overview — a focused pane owns the mouse.
  useEffect(() => {
    if (focusedId) return;
    const onMove = (event: MouseEvent) => {
      if (hoveredPaneIdRef.current) return;
      scrollTargetRef.current = {
        x: (event.clientX / window.innerWidth) * DEMONSTRATOR_MAX_SCROLL.x,
        y: (event.clientY / window.innerHeight) * DEMONSTRATOR_MAX_SCROLL.y,
      };
      ensureEasingLoopRef.current();
    };
    window.addEventListener("mousemove", onMove, { passive: true });
    return () => window.removeEventListener("mousemove", onMove);
  }, [focusedId]);

  useEffect(() => {
    const EASING_EPSILON = 0.01;
    let frame = 0;
    const tick = () => {
      const current = scrollCurrentRef.current;
      const target = scrollTargetRef.current;
      if (Math.abs(target.x - current.x) < EASING_EPSILON && Math.abs(target.y - current.y) < EASING_EPSILON) {
        if (current.x !== target.x || current.y !== target.y) {
          scrollCurrentRef.current = target;
          setScrollOffset(target);
          if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, target);
        }
        easingRunningRef.current = false;
        return;
      }
      const next = { x: current.x + (target.x - current.x) * 0.12, y: current.y + (target.y - current.y) * 0.12 };
      scrollCurrentRef.current = next;
      setScrollOffset(next);
      if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, next);
      frame = requestAnimationFrame(tick);
    };
    ensureEasingLoopRef.current = () => {
      if (easingRunningRef.current) return;
      easingRunningRef.current = true;
      frame = requestAnimationFrame(tick);
    };
    ensureEasingLoopRef.current();
    return () => {
      easingRunningRef.current = false;
      cancelAnimationFrame(frame);
    };
  }, [refreshRevealRect]);

  const dismissIntroduction = useCallback((_completed: boolean) => {
    setShowIntroduction(false);
  }, []);

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
          transition: focusedId ? "transform 500ms ease-in-out" : undefined,
        }}
      >
        {DEMONSTRATOR_PANES.map((pane) => (
          <DemonstratorPane key={pane.id} pane={pane} booted={bootedIds.has(pane.id)} focused={focusedId === pane.id} />
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
                  <button
                    type="button"
                    onClick={() => focusPane(pane.id)}
                    onMouseEnter={() => {
                      hoveredPaneIdRef.current = pane.id;
                      setHoveredPaneId(pane.id);
                      promote(pane.id);
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
                    className={cn(
                      "pointer-events-auto group flex min-h-[8.5rem] w-full max-w-[15rem] flex-col items-center justify-center gap-single",
                      "rounded-xl border border-border-normal px-double py-triple text-center",
                      "ui-glass shadow-md outline-none",
                      "transition-[transform,box-shadow,border-color,background-color] duration-200",
                      "hover:-translate-y-0.5 hover:border-border-emphasized hover:shadow-xl",
                      "focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
                      active && "-translate-y-0.5 border-border-emphasized shadow-xl",
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
                </div>
              );
            })}
          </div>

          <div className="pointer-events-none absolute inset-x-0 bottom-0 z-40 flex items-center justify-between gap-tiny px-double py-single">
            <div className="pointer-events-auto">{aProjectOfLuhUdkFooterItem("landingProjectOf", "de", false).content}</div>
            <div className="pointer-events-auto">{fundedByZukunftBauFooterItem("landingFundedBy", "de", false).content}</div>
          </div>

          {showIntroduction && (
            <UIIntroduction
              introduction={ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION}
              stepIndex={introductionStep}
              completedInteractionIndices={[]}
              onStepIndexChange={setIntroductionStep}
              onDismiss={dismissIntroduction}
            />
          )}

          <div
            className="pointer-events-none absolute left-double top-double z-20 size-workbench text-foreground [&_svg]:h-full [&_svg]:w-full"
            dangerouslySetInnerHTML={{ __html: ENTWERFEN_MIT_BESTAND_LOGO_SVG }}
            aria-hidden
          />
        </>
      )}

      {focusedId && (
        <button
          type="button"
          onClick={returnToOverview}
          className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single rounded-md border border-border-normal px-single py-half text-sm font-medium text-foreground shadow-md outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring"
        >
          <Icon icon="layout-grid" size="small" />
          Übersicht
        </button>
      )}
    </div>
  );
}
//#endregion 🎪️DemonstratorLanding

createRoot(document.getElementById("root")!).render(<DemonstratorLanding />);
