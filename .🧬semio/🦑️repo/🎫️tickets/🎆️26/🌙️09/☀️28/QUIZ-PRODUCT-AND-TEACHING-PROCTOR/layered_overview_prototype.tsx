/** 🥞️ PROTOTYPE of the proposed shared, domain-neutral `LayeredOverview` element (ticket QUIZ-PRODUCT-AND-TEACHING-PROCTOR).
 * Mechanics are the ones measured in `🎡️play/🟦️.tsx` and `🧺️demonstrator/🟦️.tsx`, minus everything domain-specific
 * (`FrameworkOsShell`, plugin catalog, brands): a strip of viewport-sized panes, one `ui-veil` glass layer above it with a hole
 * over the revealed pane, and an app-owned card grid above the glass. Improvements over the two landings, each deliberate:
 * transform and veil are written imperatively (no React render per frame), the hole is a single `clip-path` in percentages
 * (one backdrop-filter layer instead of four, resize-proof), the hole is computed from the CURRENT offset (no target-offset
 * flash), keyboard focus reveals AND blur conceals, reduced motion snaps, panes are labelled and inert. */

import { Component, useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, type CSSProperties, type ReactNode } from "react";
import { CanvasSkeleton, Icon, cn, loadingBorderClass, type IconName } from "@semio-tech/ui-react";

//#region 🧬️Contracts
/** 📍️ Zero-based cell of a pane on the strip. */
export interface LayeredCell {
  readonly column: number;
  readonly row: number;
}

/** 🥞️ One backdrop page: what a card opens. */
export interface LayeredPane {
  readonly id: string;
  /** 🏷️ Accessible name of the page once opened (and of its placeholder). */
  readonly label: string;
  readonly icon?: IconName;
  /** 📖️ The real page. Called only while the pane is mounted (booted and not suspended). */
  readonly render: (state: LayeredPaneState) => ReactNode;
  /** 🖼️ Optional still shown instead of the placeholder while the pane is not mounted. */
  readonly poster?: string | null;
}

export interface LayeredPaneState {
  /** 🔓️ The page is opened full size (interactive). Otherwise it is inert backdrop. */
  readonly opened: boolean;
  /** 👁️ The card of this page is hovered or focused: the page is shown clear. */
  readonly revealed: boolean;
}

export interface LayeredCardState {
  /** 🧭️ `strip` (card grid over the glass) or `list` (one card per snap section). */
  readonly mode: "strip" | "list";
  readonly revealed: boolean;
  readonly opened: boolean;
  readonly open: () => void;
}

/** 🧮️ When panes are mounted and released. All timings in ms; `budget` = most panes mounted at once. */
export interface LayeredLifecycle {
  readonly budget: number;
  readonly warmStartMs: number;
  readonly warmIntervalMs: number;
}

export interface LayeredOverviewProps {
  readonly panes: readonly LayeredPane[];
  /** 📍️ Strip cell per pane id. The strip is `max(column)+1` by `max(row)+1` viewport-sized cells. */
  readonly cells: Readonly<Record<string, LayeredCell>>;
  /** 🃏️ The card above the glass. It receives its state; hover/focus/blur wiring is owned by the wrapper. */
  readonly renderCard: (pane: LayeredPane, state: LayeredCardState) => ReactNode;
  /** 🧱️ Class and style of the card overlay grid — app-owned so the card layout can be responsive. */
  readonly overlayClassName?: string;
  readonly overlayStyle?: CSSProperties;
  /** 📱️ `list`: one snap section per pane, each with its own veil and card (touch phones). */
  readonly mode?: "strip" | "list";
  readonly pan?: "pointer" | "none";
  readonly routing?: "hash" | "none";
  readonly lifecycle?: Partial<LayeredLifecycle>;
  readonly overviewLabel: string;
  readonly gridLabel: string;
  readonly waitingLabel: (pane: LayeredPane) => string;
  readonly failedLabel: (pane: LayeredPane) => string;
  /** ⚙️ Controlled open state (optional). */
  readonly openedId?: string | null;
  readonly onOpenedIdChange?: (id: string | null) => void;
}
//#endregion 🧬️Contracts

//#region 🧮️Geometry
const GLIDE_MS = 500;
const FOLLOW_LERP = 0.12;
const EPSILON = 0.0001;
const DEFAULT_LIFECYCLE: LayeredLifecycle = { budget: Number.POSITIVE_INFINITY, warmStartMs: 1500, warmIntervalMs: 400 };

type Offset = { readonly x: number; readonly y: number };

const ease = (t: number): number => (t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2);
const lerp = (a: Offset, b: Offset, t: number): Offset => ({ x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t });

/** 🕳️ The `clip-path` of the glass for a hole over `cell` at `offset` (cell units), or `none` / `hidden`. Percent of the container. */
export function veilClip(cell: LayeredCell | null, offset: Offset): { readonly clipPath: string; readonly hidden: boolean } {
  if (cell === null) return { clipPath: "none", hidden: false };
  const x0 = Math.max(0, cell.column - offset.x);
  const x1 = Math.min(1, cell.column - offset.x + 1);
  const y0 = Math.max(0, cell.row - offset.y);
  const y1 = Math.min(1, cell.row - offset.y + 1);
  if (x1 <= x0 || y1 <= y0) return { clipPath: "none", hidden: false };
  if (x0 <= 0 && x1 >= 1 && y0 <= 0 && y1 >= 1) return { clipPath: "none", hidden: true };
  const p = (v: number): string => `${(v * 100).toFixed(3)}%`;
  return { clipPath: `polygon(evenodd, 0% 0%, 100% 0%, 100% 100%, 0% 100%, 0% 0%, ${p(x0)} ${p(y0)}, ${p(x1)} ${p(y0)}, ${p(x1)} ${p(y1)}, ${p(x0)} ${p(y1)}, ${p(x0)} ${p(y0)})`, hidden: false };
}
//#endregion 🧮️Geometry

//#region 🛟️PaneErrorBoundary
class PaneBoundary extends Component<{ readonly failed: string; readonly children: ReactNode }, { readonly error: Error | null }> {
  override state = { error: null as Error | null };
  static getDerivedStateFromError(error: Error) {
    return { error };
  }
  override render() {
    return this.state.error ? (
      <div role="alert" className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
        {this.props.failed}
      </div>
    ) : (
      this.props.children
    );
  }
}
//#endregion 🛟️PaneErrorBoundary

function usePrefersReducedMotion(): boolean {
  const [reduced, setReduced] = useState(() => typeof window !== "undefined" && window.matchMedia("(prefers-reduced-motion: reduce)").matches);
  useEffect(() => {
    const query = window.matchMedia("(prefers-reduced-motion: reduce)");
    const on = () => setReduced(query.matches);
    query.addEventListener("change", on);
    return () => query.removeEventListener("change", on);
  }, []);
  return reduced;
}

const hashId = (ids: readonly string[]): string | null => {
  const raw = window.location.hash.replace(/^#/, "").trim();
  return ids.includes(raw) ? raw : null;
};

/** 🥞️ Backdrop strip + glass + cards. Fills its (positioned) parent. */
export function LayeredOverview(props: LayeredOverviewProps) {
  const { panes, cells, renderCard, mode = "strip", pan = "pointer", routing = "hash" } = props;
  const policy = { ...DEFAULT_LIFECYCLE, ...props.lifecycle };
  const ids = useMemo(() => panes.map((pane) => pane.id), [panes]);
  const reduced = usePrefersReducedMotion();
  const columns = Math.max(...ids.map((id) => (cells[id]?.column ?? 0) + 1));
  const rows = Math.max(...ids.map((id) => (cells[id]?.row ?? 0) + 1));
  const cellUnits = useCallback((id: string): Offset => ({ x: cells[id]?.column ?? 0, y: cells[id]?.row ?? 0 }), [cells]);

  const [innerOpened, setInnerOpened] = useState<string | null>(() => (routing === "hash" ? hashId(ids) : null));
  const opened = props.openedId !== undefined ? props.openedId : innerOpened;
  const [revealedId, setRevealedId] = useState<string | null>(null);
  const [booted, setBooted] = useState<ReadonlySet<string>>(() => new Set(opened ? [opened] : []));
  const [suspended, setSuspended] = useState<ReadonlySet<string>>(new Set());
  const touchedAt = useRef(new Map<string, number>());
  const rootRef = useRef<HTMLDivElement | null>(null);
  const stripRef = useRef<HTMLDivElement | null>(null);
  const veilRef = useRef<HTMLDivElement | null>(null);
  const revealedRef = useRef<string | null>(null);
  const openedRef = useRef<string | null>(opened);
  openedRef.current = opened;
  const cur = useRef<Offset>(opened ? cellUnits(opened) : { x: 0, y: 0 });
  const target = useRef<Offset>(cur.current);
  const drive = useRef<{ mode: "follow" } | { mode: "glide"; from: Offset; to: Offset; start: number }>({ mode: "follow" });
  const running = useRef(false);
  const frame = useRef(0);

  const paint = useCallback(
    (offset: Offset) => {
      cur.current = offset;
      const strip = stripRef.current;
      if (strip) strip.style.transform = `translate3d(${(-offset.x * 100) / columns}%, ${(-offset.y * 100) / rows}%, 0)`;
      const veil = veilRef.current;
      if (veil) {
        const id = revealedRef.current;
        const clip = veilClip(id ? (cells[id] ?? null) : null, offset);
        veil.style.clipPath = clip.clipPath;
        veil.style.visibility = clip.hidden ? "hidden" : "visible";
      }
    },
    [cells, columns, rows],
  );

  const tick = useCallback(() => {
    const d = drive.current;
    if (d.mode === "glide") {
      const t = Math.min(1, Math.max(0, (performance.now() - d.start) / GLIDE_MS));
      paint(lerp(d.from, d.to, ease(t)));
      if (t >= 1) {
        target.current = d.to;
        drive.current = { mode: "follow" };
        running.current = false;
        return;
      }
      frame.current = requestAnimationFrame(tick);
      return;
    }
    const c = cur.current;
    const g = target.current;
    if (Math.abs(g.x - c.x) < EPSILON && Math.abs(g.y - c.y) < EPSILON) {
      if (c.x !== g.x || c.y !== g.y) paint(g);
      running.current = false;
      return;
    }
    paint(lerp(c, g, FOLLOW_LERP));
    frame.current = requestAnimationFrame(tick);
  }, [paint]);

  const drivePlay = useCallback(() => {
    if (running.current) return;
    running.current = true;
    frame.current = requestAnimationFrame(tick);
  }, [tick]);

  const glideTo = useCallback(
    (to: Offset) => {
      target.current = to;
      if (reduced) {
        drive.current = { mode: "follow" };
        paint(to);
        return;
      }
      drive.current = { mode: "glide", from: cur.current, to, start: performance.now() };
      drivePlay();
    },
    [drivePlay, paint, reduced],
  );

  useLayoutEffect(() => {
    paint(cur.current);
    return () => cancelAnimationFrame(frame.current);
  }, [paint]);

  const touch = useCallback((id: string) => {
    touchedAt.current.set(id, Date.now());
    setBooted((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
    setSuspended((prev) => {
      if (!prev.has(id)) return prev;
      const next = new Set(prev);
      next.delete(id);
      return next;
    });
  }, []);

  const reveal = useCallback(
    (id: string) => {
      if (openedRef.current) return;
      revealedRef.current = id;
      setRevealedId(id);
      touch(id);
      glideTo(cellUnits(id));
      paint(cur.current);
    },
    [cellUnits, glideTo, paint, touch],
  );
  const conceal = useCallback(
    (id: string) => {
      if (revealedRef.current !== id) return;
      revealedRef.current = null;
      setRevealedId(null);
      paint(cur.current);
    },
    [paint],
  );

  const setOpened = useCallback(
    (id: string | null) => {
      if (props.openedId === undefined) setInnerOpened(id);
      props.onOpenedIdChange?.(id);
      revealedRef.current = null;
      setRevealedId(null);
      if (id) {
        touch(id);
        glideTo(cellUnits(id));
      }
      if (routing === "hash") window.history.replaceState(null, "", id ? `#${id}` : window.location.pathname + window.location.search);
    },
    [cellUnits, glideTo, props, routing, touch],
  );

  useEffect(() => {
    if (routing !== "hash") return;
    const on = () => {
      const id = hashId(ids);
      if (id) setOpened(id);
      else setOpened(null);
    };
    window.addEventListener("hashchange", on);
    return () => window.removeEventListener("hashchange", on);
  }, [ids, routing, setOpened]);

  useEffect(() => {
    if (!opened) return;
    const on = (event: KeyboardEvent) => event.key === "Escape" && setOpened(null);
    window.addEventListener("keydown", on);
    return () => window.removeEventListener("keydown", on);
  }, [opened, setOpened]);

  useEffect(() => {
    if (mode !== "strip" || pan !== "pointer" || opened || reduced) return;
    const on = (event: PointerEvent) => {
      if (revealedRef.current || event.pointerType !== "mouse") return;
      const box = rootRef.current?.getBoundingClientRect();
      if (!box || box.width === 0) return;
      drive.current = { mode: "follow" };
      target.current = { x: ((event.clientX - box.left) / box.width) * (columns - 1), y: ((event.clientY - box.top) / box.height) * (rows - 1) };
      drivePlay();
    };
    window.addEventListener("pointermove", on, { passive: true });
    return () => window.removeEventListener("pointermove", on);
  }, [columns, drivePlay, mode, opened, pan, reduced, rows]);

  useEffect(() => {
    if (opened) return;
    const next = ids.find((id) => !booted.has(id));
    if (next === undefined || booted.size - suspended.size >= policy.budget) return;
    const timer = window.setTimeout(() => setBooted((prev) => new Set(prev).add(next)), booted.size === 0 ? policy.warmStartMs : policy.warmIntervalMs);
    return () => window.clearTimeout(timer);
  }, [booted, suspended, ids, opened, policy.budget, policy.warmIntervalMs, policy.warmStartMs]);

  useEffect(() => {
    const live = [...booted].filter((id) => !suspended.has(id));
    const excess = live.length - policy.budget;
    if (excess <= 0) return;
    const victims = live
      .filter((id) => id !== opened && id !== revealedId)
      .sort((a, b) => (touchedAt.current.get(a) ?? 0) - (touchedAt.current.get(b) ?? 0))
      .slice(0, excess);
    if (victims.length) setSuspended((prev) => new Set([...prev, ...victims]));
  }, [booted, suspended, opened, revealedId, policy.budget]);

  const paneNode = (pane: LayeredPane, style?: CSSProperties) => {
    const isOpened = opened === pane.id;
    const live = booted.has(pane.id) && !suspended.has(pane.id);
    return (
      <div
        key={pane.id}
        data-layered-pane={pane.id}
        role={isOpened ? "region" : undefined}
        aria-label={isOpened ? pane.label : undefined}
        aria-hidden={isOpened ? undefined : true}
        inert={!isOpened}
        className="relative h-full w-full overflow-hidden bg-background"
        style={style}
      >
        {live ? (
          <PaneBoundary failed={props.failedLabel(pane)}>{pane.render({ opened: isOpened, revealed: revealedId === pane.id })}</PaneBoundary>
        ) : pane.poster ? (
          <img src={pane.poster} alt="" aria-hidden className="h-full w-full object-cover" />
        ) : (
          <div className={cn("flex h-full w-full flex-col items-center justify-center gap-double bg-background", loadingBorderClass)} role="status" aria-busy={booted.has(pane.id)} aria-label={props.waitingLabel(pane)}>
            {pane.icon ? <Icon icon={pane.icon} size="large" className="text-foreground opacity-40" /> : null}
            <div className="h-full min-h-0 w-full max-w-4xl flex-1 p-double">
              <CanvasSkeleton label={props.waitingLabel(pane)} />
            </div>
          </div>
        )}
      </div>
    );
  };

  const cardNode = (pane: LayeredPane) => {
    const wrapper = (
      <div
        key={pane.id}
        data-layered-card={pane.id}
        className="pointer-events-none contents"
      >
        <CardHost pane={pane} mode={mode} revealed={revealedId === pane.id} opened={opened === pane.id} open={() => setOpened(pane.id)} reveal={reveal} conceal={conceal} render={renderCard} />
      </div>
    );
    return wrapper;
  };

  const overviewButton = opened ? (
    <button
      type="button"
      data-layered-overview-button=""
      onClick={() => setOpened(null)}
      className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single border border-border-normal px-single py-half text-sm font-medium text-foreground outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring"
    >
      <Icon icon="layout-grid" size="small" />
      {props.overviewLabel}
    </button>
  ) : null;

  if (mode === "list") {
    return (
      <div ref={rootRef} className="relative h-full w-full overflow-hidden">
        <div className={cn("flex h-full w-full flex-col overscroll-y-contain", opened ? "overflow-hidden" : "snap-y snap-mandatory overflow-y-auto")} aria-label={props.gridLabel}>
          {panes.map((pane) => (
            <section key={pane.id} className="relative h-full min-h-full w-full shrink-0 snap-start overflow-hidden">
              {paneNode(pane)}
              {opened ? null : (
                <>
                  <div className="pointer-events-none absolute inset-0 z-30">
                    <div className="ui-veil absolute inset-0" />
                  </div>
                  <div className="pointer-events-none absolute inset-0 z-[31] flex items-center justify-center px-double">{cardNode(pane)}</div>
                </>
              )}
            </section>
          ))}
        </div>
        {overviewButton}
      </div>
    );
  }

  return (
    <div ref={rootRef} className="relative h-full w-full overflow-hidden bg-background text-foreground">
      <div
        ref={stripRef}
        className="grid will-change-transform"
        style={{ width: `${columns * 100}%`, height: `${rows * 100}%`, gridTemplateColumns: `repeat(${columns}, 1fr)`, gridTemplateRows: `repeat(${rows}, 1fr)` }}
      >
        {panes.map((pane) => paneNode(pane, { gridColumn: (cells[pane.id]?.column ?? 0) + 1, gridRow: (cells[pane.id]?.row ?? 0) + 1 }))}
      </div>
      {opened ? null : (
        <>
          <div className="pointer-events-none absolute inset-0 z-30">
            <div ref={veilRef} className="ui-veil absolute inset-0" />
          </div>
          <div role="group" aria-label={props.gridLabel} className={cn("pointer-events-none absolute inset-0 z-[31]", props.overlayClassName)} style={props.overlayStyle}>
            {panes.map((pane) => cardNode(pane))}
          </div>
        </>
      )}
      {overviewButton}
    </div>
  );
}

/** 🃏️ Wires pointer enter/leave and focus/blur of ONE card wrapper to reveal/conceal; the card itself is the app's. */
function CardHost(props: {
  readonly pane: LayeredPane;
  readonly mode: "strip" | "list";
  readonly revealed: boolean;
  readonly opened: boolean;
  readonly open: () => void;
  readonly reveal: (id: string) => void;
  readonly conceal: (id: string) => void;
  readonly render: LayeredOverviewProps["renderCard"];
}) {
  const { pane } = props;
  return (
    <div
      className="contents"
      onPointerEnter={(event) => event.pointerType === "mouse" && props.reveal(pane.id)}
      onPointerLeave={(event) => event.pointerType === "mouse" && props.conceal(pane.id)}
      onFocus={() => props.reveal(pane.id)}
      onBlur={(event) => {
        if (!(event.currentTarget as HTMLElement).contains(event.relatedTarget as Node | null)) props.conceal(pane.id);
      }}
    >
      {props.render(pane, { mode: props.mode, revealed: props.revealed, opened: props.opened, open: props.open })}
    </div>
  );
}
