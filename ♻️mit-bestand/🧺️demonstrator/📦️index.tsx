// #region 🧲️Header
/** @emoji 🎪️ Entwerfen mit Bestand demonstrator landing — general introduction, six-app preview grid, glass name overlay. */
// #endregion 🧲️Header

import { createRoot } from "react-dom/client";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
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
  type IconName,
} from "@semio-tech/ui-react";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "./⚛️footer.tsx";
import { DEMONSTRATOR_LOCALE, ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION, ENTWERFEN_MIT_BESTAND_LOGO_SVG } from "./🟦️brand.ts";
import "./🎨️globals.css";

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance());
// 🇩🇪️ The whole demonstrator is German-locked (see 🟦️brand.ts) — resolve synchronously before the
// first render so the landing page's own chrome (Skip/Back/Next/Done) never flashes English.
initUiLocaleSync(DEMONSTRATOR_LOCALE);

//#region 🎪️DemonstratorApps
type DemonstratorAppPane = {
  readonly id: string;
  readonly label: string;
  readonly devPort: number;
  readonly href: string;
  readonly icon: IconName;
  readonly tagline: string;
};

/** @emoji 🔢️ Columns and rows of the demonstrator preview grid; the strip spans `columns * 100vw` by `rows * 100vh`. */
const DEMONSTRATOR_GRID_COLUMNS = 3;
const DEMONSTRATOR_GRID_ROWS = 2;

const DEMONSTRATOR_APP_PANES: readonly DemonstratorAppPane[] = [
  { id: "generator", label: "Generator", devPort: 6027, href: "/generator/", icon: "workflow", tagline: "Parametrische Abläufe" },
  { id: "koordinator", label: "Koordinator", devPort: 6028, href: "/koordinator/", icon: "cad-shape", tagline: "Modelle koordinieren" },
  { id: "aggregator", label: "Aggregator", devPort: 6023, href: "/aggregator/", icon: "puzzle", tagline: "Bestand zusammensetzen" },
  { id: "aussuchen", label: "Aussuchen", devPort: 6030, href: "/aussuchen/", icon: "library", tagline: "Bestand sichten" },
  { id: "bearbeiten", label: "Bearbeiten", devPort: 6031, href: "/bearbeiten/", icon: "hammer", tagline: "Bauteile anpassen" },
  { id: "verfolgen", label: "Verfolgen", devPort: 6032, href: "/verfolgen/", icon: "gis2d", tagline: "Herkunft verfolgen" },
];

function paneColumn(paneIndex: number): number {
  return paneIndex % DEMONSTRATOR_GRID_COLUMNS;
}

function paneRow(paneIndex: number): number {
  return Math.floor(paneIndex / DEMONSTRATOR_GRID_COLUMNS);
}

/** @emoji 🔗 Navigates into a pane. Dev uses the playground's own Vite origin so absolute `/@vite`,
 * `/asset`, and `/@fs` URLs resolve; production keeps the static same-origin `/slug/` path. */
function paneAppUrl(pane: DemonstratorAppPane): string {
  if (import.meta.env.DEV) return `http://127.0.0.1:${pane.devPort}/`;
  return pane.href;
}

/** @emoji 🖼️ Iframe preview src — same origin rules as {@link paneAppUrl}; path-prefix proxies break Vite. */
function paneEmbedUrl(pane: DemonstratorAppPane): string {
  return paneAppUrl(pane);
}

function paneIdFromLocationHash(): string | null {
  const raw = window.location.hash.replace(/^#/, "").trim();
  if (!raw) return null;
  return DEMONSTRATOR_APP_PANES.some((pane) => pane.id === raw) ? raw : null;
}

const DEMONSTRATOR_RELOAD_ON_RETURN_KEY = "mit-bestand.demonstrator.reload-on-return";

/** @emoji ♻️ Marks the overview so a later back-navigation reloads a fresh landing page. */
function markOverviewReloadOnReturn(): void {
  try {
    sessionStorage.setItem(DEMONSTRATOR_RELOAD_ON_RETURN_KEY, "1");
  } catch {
    /* ignore quota / private mode */
  }
}

/** @emoji ♻️ Reloads the overview when returning from an app via back-forward cache; clears the return marker after a fresh back load. */
function installOverviewReturnReload(): void {
  window.addEventListener("pageshow", (event) => {
    let shouldReload = false;
    try {
      shouldReload = sessionStorage.getItem(DEMONSTRATOR_RELOAD_ON_RETURN_KEY) === "1";
    } catch {
      return;
    }
    if (!shouldReload) return;
    try {
      sessionStorage.removeItem(DEMONSTRATOR_RELOAD_ON_RETURN_KEY);
    } catch {
      /* ignore */
    }
    if (event.persisted) window.location.reload();
  });
}

installOverviewReturnReload();
//#endregion 🎪️DemonstratorApps

//#region 🎪️DemonstratorGridGeometry
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

//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
  const surfaceChrome = useMemo(
    () => ({
      appearance: readStoredUiChromeAppearance(),
      device: (readStoredUiChromeLayout() === "tablet" ? "tablet" : "desktop") as const,
      driver: readStoredUiDriver(),
    }),
    [],
  );
  useElementsSurfaceChrome(surfaceChrome);

  const [introductionStep, setIntroductionStep] = useState(0);
  const [showIntroduction, setShowIntroduction] = useState(true);
  const [hoveredPaneId, setHoveredPaneId] = useState<string | null>(null);
  const [revealRect, setRevealRect] = useState<RevealRectPx | null>(null);
  const hoveredPaneIdRef = useRef<string | null>(null);
  const scrollTargetRef = useRef<ScrollOffset>({ x: 0, y: 0 });
  const scrollCurrentRef = useRef<ScrollOffset>({ x: 0, y: 0 });
  const [scrollOffset, setScrollOffset] = useState<ScrollOffset>({ x: 0, y: 0 });

  const applyPaneScroll = useCallback((paneIndex: number) => {
    const offset = scrollOffsetForPaneIndex(paneIndex);
    scrollTargetRef.current = offset;
    scrollCurrentRef.current = offset;
    setScrollOffset(offset);
  }, []);

  useEffect(() => {
    const hashPaneId = paneIdFromLocationHash();
    if (hashPaneId) {
      const index = DEMONSTRATOR_APP_PANES.findIndex((pane) => pane.id === hashPaneId);
      if (index >= 0) applyPaneScroll(index);
    }
    const onHashChange = () => {
      const paneId = paneIdFromLocationHash();
      if (!paneId) return;
      const index = DEMONSTRATOR_APP_PANES.findIndex((pane) => pane.id === paneId);
      if (index >= 0) applyPaneScroll(index);
    };
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  }, [applyPaneScroll]);

  const refreshRevealRect = useCallback((paneId: string | null, offset: ScrollOffset) => {
    if (!paneId) {
      setRevealRect(null);
      return;
    }
    const paneIndex = DEMONSTRATOR_APP_PANES.findIndex((pane) => pane.id === paneId);
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

  useEffect(() => {
    const onMove = (event: MouseEvent) => {
      if (hoveredPaneIdRef.current) return;
      scrollTargetRef.current = {
        x: (event.clientX / window.innerWidth) * DEMONSTRATOR_MAX_SCROLL.x,
        y: (event.clientY / window.innerHeight) * DEMONSTRATOR_MAX_SCROLL.y,
      };
    };
    window.addEventListener("mousemove", onMove, { passive: true });
    return () => window.removeEventListener("mousemove", onMove);
  }, []);

  useEffect(() => {
    let frame = 0;
    const tick = () => {
      const current = scrollCurrentRef.current;
      const target = scrollTargetRef.current;
      const next = { x: current.x + (target.x - current.x) * 0.12, y: current.y + (target.y - current.y) * 0.12 };
      scrollCurrentRef.current = next;
      setScrollOffset(next);
      if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, next);
      frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
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
        }}
      >
        {DEMONSTRATOR_APP_PANES.map((pane) => (
          <iframe
            key={pane.id}
            title={pane.label}
            src={paneEmbedUrl(pane)}
            className="pointer-events-none border-0"
            style={{ width: "100vw", height: "100vh" }}
          />
        ))}
      </div>

      <div className="pointer-events-none absolute inset-0 z-30">
        {tintSegments.map((segment, index) => (
          <div
            key={`tint-${index}-${segment.top}-${segment.left}`}
            className="ui-veil absolute"
            style={{ top: segment.top, left: segment.left, width: segment.width, height: segment.height }}
          />
        ))}
      </div>

      <div
        className="pointer-events-none absolute inset-0 z-[31] grid items-center"
        style={{ gridTemplateColumns: `repeat(${DEMONSTRATOR_GRID_COLUMNS}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${DEMONSTRATOR_GRID_ROWS}, minmax(0, 1fr))` }}
      >
        {DEMONSTRATOR_APP_PANES.map((pane, paneIndex) => {
          const active = hoveredPaneId === pane.id;
          return (
            <div key={pane.id} className="flex justify-center px-double">
              <a
                href={paneAppUrl(pane)}
                onClick={() => {
                  markOverviewReloadOnReturn();
                }}
                onMouseEnter={() => {
                  hoveredPaneIdRef.current = pane.id;
                  setHoveredPaneId(pane.id);
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
              </a>
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
    </div>
  );
}
//#endregion 🎪️DemonstratorLanding

createRoot(document.getElementById("root")!).render(<DemonstratorLanding />);
