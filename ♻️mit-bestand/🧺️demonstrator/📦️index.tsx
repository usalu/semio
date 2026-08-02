// #region 🧲️Header
/** @emoji 🎪️ Entwerfen mit Bestand demonstrator landing — general introduction, three-app preview strip, glass name overlay. */
// #endregion 🧲️Header

import { createRoot } from "react-dom/client";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { UIIntroduction, cn, Icon, type IconName } from "@semio-tech/ui-react";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "./⚛️footer.tsx";
import { ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION, ENTWERFEN_MIT_BESTAND_LOGO_SVG } from "./🟦️brand.ts";
import "./🎨️globals.css";

//#region 🎪️DemonstratorApps
type DemonstratorAppPane = {
  readonly id: string;
  readonly label: string;
  readonly devPort: number;
  readonly href: string;
  readonly icon: IconName;
  readonly tagline: string;
};

const DEMONSTRATOR_APP_PANES: readonly DemonstratorAppPane[] = [
  { id: "generator", label: "Generator", devPort: 6027, href: "/generator/", icon: "workflow", tagline: "Parametrische Abläufe" },
  { id: "koordinator", label: "Koordinator", devPort: 6028, href: "/koordinator/", icon: "cad-shape", tagline: "Modelle koordinieren" },
  { id: "aggregator", label: "Aggregator", devPort: 6023, href: "/aggregator/", icon: "puzzle", tagline: "Bestand zusammensetzen" },
];

function paneAppUrl(pane: DemonstratorAppPane): string {
  if (import.meta.env.DEV) return `http://localhost:${pane.devPort}/`;
  return pane.href;
}

function paneEmbedUrl(pane: DemonstratorAppPane): string {
  return paneAppUrl(pane);
}

/** @emoji 🧭️ Scroll offset (vw) that centers the given pane in the viewport. */
function scrollOffsetForPaneIndex(paneIndex: number): number {
  return Math.min(200, Math.max(0, paneIndex * 100));
}

/** @emoji 🧭️ Pane index nearest the current horizontal scroll position. */
function paneIndexFromScrollOffset(scrollOffsetVw: number): number {
  return Math.min(DEMONSTRATOR_APP_PANES.length - 1, Math.max(0, Math.round(scrollOffsetVw / 100)));
}

function paneIdFromLocationHash(): string | null {
  const raw = window.location.hash.replace(/^#/, "").trim();
  if (!raw) return null;
  return DEMONSTRATOR_APP_PANES.some((pane) => pane.id === raw) ? raw : null;
}

function replaceLocationHash(paneId: string): void {
  const url = new URL(window.location.href);
  if (url.hash === `#${paneId}`) return;
  url.hash = paneId;
  window.history.replaceState(window.history.state, "", url);
}
//#endregion 🎪️DemonstratorApps

//#region 🎪️DemonstratorStripGeometry
type PaneScreenBounds = {
  readonly leftVw: number;
  readonly rightVw: number;
  readonly centerVw: number;
  readonly visible: boolean;
};

/** @emoji 📐 Maps a 100vw strip pane into the current viewport after horizontal scroll (vw). */
function paneScreenBounds(paneIndex: number, scrollOffsetVw: number): PaneScreenBounds {
  const stripLeft = paneIndex * 100 - scrollOffsetVw;
  const stripRight = (paneIndex + 1) * 100 - scrollOffsetVw;
  const leftVw = Math.max(0, stripLeft);
  const rightVw = Math.min(100, stripRight);
  const visible = rightVw > leftVw;
  const centerVw = visible ? (leftVw + rightVw) / 2 : stripLeft + 50;
  return { leftVw, rightVw, centerVw, visible };
}

type TintSegment = { readonly leftVw: number; readonly widthVw: number };

/** @emoji 🪟️ Glass veil segments: full screen when idle; left/right flaps when a pane is revealed under a hovered name. */
function demonstratorTintSegments(scrollOffsetVw: number, hoveredPaneIndex: number | null): readonly TintSegment[] {
  if (hoveredPaneIndex == null) return [{ leftVw: 0, widthVw: 100 }];
  const { leftVw, rightVw, visible } = paneScreenBounds(hoveredPaneIndex, scrollOffsetVw);
  if (!visible) return [{ leftVw: 0, widthVw: 100 }];
  const segments: TintSegment[] = [];
  if (leftVw > 0) segments.push({ leftVw: 0, widthVw: leftVw });
  if (rightVw < 100) segments.push({ leftVw: rightVw, widthVw: 100 - rightVw });
  return segments.length > 0 ? segments : [];
}
//#endregion 🎪️DemonstratorStripGeometry

//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
  const [introductionStep, setIntroductionStep] = useState(0);
  const [showIntroduction, setShowIntroduction] = useState(true);
  const [hoveredPaneId, setHoveredPaneId] = useState<string | null>(null);
  const scrollTargetRef = useRef(0);
  const scrollCurrentRef = useRef(0);
  const [scrollOffsetVw, setScrollOffsetVw] = useState(0);
  const syncedHashPaneRef = useRef<string | null>(null);

  const applyPaneScroll = useCallback((paneIndex: number, syncHash: boolean) => {
    const offset = scrollOffsetForPaneIndex(paneIndex);
    scrollTargetRef.current = offset;
    scrollCurrentRef.current = offset;
    setScrollOffsetVw(offset);
    if (syncHash) {
      const paneId = DEMONSTRATOR_APP_PANES[paneIndex]?.id;
      if (paneId) {
        syncedHashPaneRef.current = paneId;
        replaceLocationHash(paneId);
      }
    }
  }, []);

  useEffect(() => {
    const hashPaneId = paneIdFromLocationHash();
    if (hashPaneId) {
      const index = DEMONSTRATOR_APP_PANES.findIndex((pane) => pane.id === hashPaneId);
      if (index >= 0) applyPaneScroll(index, false);
    } else {
      syncedHashPaneRef.current = DEMONSTRATOR_APP_PANES[0]?.id ?? null;
      replaceLocationHash(DEMONSTRATOR_APP_PANES[0]!.id);
    }
    const onHashChange = () => {
      const paneId = paneIdFromLocationHash();
      if (!paneId) return;
      const index = DEMONSTRATOR_APP_PANES.findIndex((pane) => pane.id === paneId);
      if (index >= 0) applyPaneScroll(index, false);
    };
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  }, [applyPaneScroll]);

  const hoveredPaneIndex = useMemo(() => {
    if (hoveredPaneId == null) return null;
    const index = DEMONSTRATOR_APP_PANES.findIndex((pane) => pane.id === hoveredPaneId);
    return index >= 0 ? index : null;
  }, [hoveredPaneId]);

  const tintSegments = useMemo(() => demonstratorTintSegments(scrollOffsetVw, hoveredPaneIndex), [scrollOffsetVw, hoveredPaneIndex]);

  useEffect(() => {
    const onMove = (event: MouseEvent) => {
      scrollTargetRef.current = (event.clientX / window.innerWidth) * 200;
    };
    window.addEventListener("mousemove", onMove, { passive: true });
    return () => window.removeEventListener("mousemove", onMove);
  }, []);

  useEffect(() => {
    let frame = 0;
    const tick = () => {
      const current = scrollCurrentRef.current;
      const target = scrollTargetRef.current;
      const next = current + (target - current) * 0.12;
      scrollCurrentRef.current = next;
      setScrollOffsetVw(next);
      const paneIndex = paneIndexFromScrollOffset(next);
      const paneId = DEMONSTRATOR_APP_PANES[paneIndex]?.id;
      if (paneId && paneId !== syncedHashPaneRef.current) {
        syncedHashPaneRef.current = paneId;
        replaceLocationHash(paneId);
      }
      frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, []);

  const dismissIntroduction = useCallback((_completed: boolean) => {
    setShowIntroduction(false);
  }, []);

  return (
    <div className="relative h-full w-full overflow-hidden">
      <div className="flex h-full" style={{ width: "300vw", transform: `translateX(-${scrollOffsetVw}vw)` }}>
        {DEMONSTRATOR_APP_PANES.map((pane) => (
          <iframe
            key={pane.id}
            title={pane.label}
            src={paneEmbedUrl(pane)}
            className="pointer-events-none h-full border-0"
            style={{ width: "100vw" }}
          />
        ))}
      </div>

      <div className="pointer-events-none absolute inset-0 z-30">
        {tintSegments.map((segment, index) => (
          <div
            key={`tint-${index}-${segment.leftVw}`}
            className="ui-veil absolute top-0 bottom-0"
            style={{ left: `${segment.leftVw}vw`, width: `${segment.widthVw}vw` }}
          />
        ))}
      </div>

      <div className="pointer-events-none absolute inset-0 z-[31] grid grid-cols-3 items-center">
        {DEMONSTRATOR_APP_PANES.map((pane) => {
          const active = hoveredPaneId === pane.id;
          return (
            <div key={pane.id} className="flex justify-center px-double">
              <a
                href={paneAppUrl(pane)}
                onMouseEnter={() => setHoveredPaneId(pane.id)}
                onMouseLeave={() => setHoveredPaneId((current) => (current === pane.id ? null : current))}
                className={cn(
                  "pointer-events-auto group flex min-h-[11rem] w-full max-w-[18rem] flex-col items-center justify-center gap-double",
                  "rounded-xl border border-border-normal px-triple py-huge text-center",
                  "ui-glass shadow-md outline-none",
                  "transition-[transform,box-shadow,border-color,background-color] duration-200",
                  "hover:-translate-y-0.5 hover:border-border-emphasized hover:shadow-xl",
                  "focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
                  active && "-translate-y-0.5 border-border-emphasized shadow-xl",
                )}
              >
                <span
                  className={cn(
                    "flex size-huge items-center justify-center rounded-lg border border-border-normal bg-background/60 p-double transition-colors",
                    "group-hover:border-border-emphasized group-hover:bg-hover-window",
                    active && "border-border-emphasized bg-hover-window",
                  )}
                >
                  <Icon icon={pane.icon} size="xl" className="text-foreground" title={pane.label} />
                </span>
                <span className="flex flex-col gap-half">
                  <span className="text-3xl font-semibold tracking-tight text-foreground">{pane.label}</span>
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
