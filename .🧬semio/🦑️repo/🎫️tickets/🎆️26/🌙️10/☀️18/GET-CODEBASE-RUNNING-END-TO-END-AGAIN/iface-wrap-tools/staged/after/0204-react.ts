/** 🪟️ Windowed containers: the window context and reporter, row and viewport measurement, request math, the served-memory rule, the window observer and the tree pick and open-state rules.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import {
  createContext,
  useContext,
  useEffect,
  useRef,
  type MouseEvent as ReactMouseEvent,
  type RefObject
} from "../../⚛️react-port/🟦️.ts";
import {
  TREE_WINDOW_BODY_NODE_BUDGET,
  TREE_WINDOW_OVERSCAN_ROWS,
  capTreeWindowRequests,
  interactionMergeFromModifiers,
  TREE_WINDOW_PATH_SEPARATOR,
  TREE_WINDOW_ROW_EXTENTS,
  treeWindowRequestsForViewport,
  treeWindowVisibleRowsForViewport,
  type TreeDataActivationContext,
  type TreeWindowContainerMeasure,
  type TreeWindowRequest,
  type TreeWindowRowExtent,
  type TreeWindowRowMeasure
} from "@semio-tech/ui-react";
import { domSizePx } from "@semio-tech/ui-styling";
import { type ActionBinding } from "@semio-tech/framework";
import { type MergeMode, type UiNodeRecord, type UiValue } from "@semio-tech/framework";
import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
//#region Tree
//#region 🪟️TreeWindows
/** 🪟️ One container's on-screen row window, keyed by the AUTHORED node key the guest stamped on
 * `data-tree-window-key` — never a DOM id. The host's map has to survive a body refresh and a store
 * re-mint, and `UiNodeRecord.id` is renumbered by every `builtNodeToSnapshot` (see {@link uiNodeDomId}). */
export type TreeWindowReportV1 = { readonly nodeKey: string; readonly offset: number; readonly rows: number };

/** 🪟️ The per-panel-body channel a guest tree reports expansion and scroll through, provided by
 * `ShellHost` around each `InterpretedUiNode` (`uiNodeToTreePanelConfig`). Absent (the default `null`)
 * means "nobody is listening": the tree keeps `<Tree>`'s own uncontrolled per-mount open state and
 * measures nothing, which is exactly what a story, a fixture or a wgpu-side mount wants.
 *
 * `openStates` is keyed by WINDOW PATH for a windowed container ({@link treeWindowPathOf}: its enclosing
 * windowed containers' node keys and its own, joined by `TREE_WINDOW_PATH_SEPARATOR`) and by authored node key
 * for anything else — the same identity `setOpen` reports back and `ViewModel::tree_windows` carries to the
 * guest. Never the node key alone for a window: that string is also the pick target id and two containers
 * under different parents share it by design (📓️f2-sdk-body-node-ledger.md §10). `TreeView` translates to and
 * from `<Tree>`'s DOM ids ({@link uiNodeDomId}) at the boundary, so no host map ever holds a volatile id. */
export type TreeWindowContextValue = {
  readonly bodyKey: string;
  readonly openStates: Readonly<Record<string, boolean>>;
  readonly setOpen: (nodeKey: string, open: boolean) => void;
  readonly reportWindows: (requests: readonly TreeWindowReportV1[], viewportRows: number) => void;
};

export const TreeWindowContext = createContext<TreeWindowContextValue | null>(null);

/** 🪟️ Merges mounted observers before publishing the body's complete viewport ledger. */
export function createTreeWindowReporterV1(report: TreeWindowContextValue["reportWindows"]) {
  const sources = new Map<object, { requests: readonly TreeWindowReportV1[]; viewportRows: number }>();
  const publish = () => {
    const rows = new Map<string, TreeWindowReportV1>();
    let viewport = 1;
    for (const source of sources.values()) {
      viewport = Math.max(viewport, source.viewportRows);
      for (const request of source.requests) {
        const previous = rows.get(request.nodeKey);
        const offset = Math.min(previous?.offset ?? request.offset, request.offset);
        const end = Math.max(previous ? previous.offset + previous.rows : 0, request.offset + request.rows);
        rows.set(request.nodeKey, { nodeKey: request.nodeKey, offset, rows: end - offset });
      }
    }
    report([...rows.values()], viewport);
  };
  return {
    publish(source: object, requests: readonly TreeWindowReportV1[], viewportRows: number) { sources.set(source, { requests, viewportRows }); publish(); },
    remove(source: object) { if (sources.delete(source)) publish(); },
  };
}

const treeWindowReporters = new WeakMap<TreeWindowContextValue["reportWindows"], ReturnType<typeof createTreeWindowReporterV1>>();

/** 🌳️ Each body channel owns one reporter shared by all of its mounted trees and tables. */
function treeWindowReporter(channel: TreeWindowContextValue) {
  let reporter = treeWindowReporters.get(channel.reportWindows);
  if (!reporter) { reporter = createTreeWindowReporterV1(channel.reportWindows); treeWindowReporters.set(channel.reportWindows, reporter); }
  return reporter;
}

/** 🪟️ The nearest {@link TreeWindowContext}, or `null` outside a host-provided panel body. */
export function useTreeWindowContext(): TreeWindowContextValue | null {
  return useContext(TreeWindowContext);
}

/** 🪟️ The ONE tree row pitch, read off the same `treeRowUiSpacing` design token the `🌳️Tree` element's
 * own `treeRowHeightPx` and the wgpu target's `TREE_ROW_HEIGHT` are computed from — a windowed tree is
 * a fixed-row-height virtualiser and all three renderers must agree on the pitch or the spacers and the
 * requested rows drift apart. Never zero, so the row arithmetic can never divide by it. */
export function treeWindowRowHeightPx(): number {
  const height = domSizePx("treeRowUiSpacing");
  return Number.isFinite(height) && height > 0 ? height : 1;
}

/** 🪟️ The bounded scroll container a guest tree actually scrolls inside — the ancestor `🖼️Panel`'s
 * `📜️Scrollable`, not the guest `<Tree>`'s own root div, whose `overflow-auto` never engages because the
 * chain above it is a natural-height stack (📓️audit-host-tree-pipeline.md §3).
 *
 * 🧯️ It is the element that CARRIES the overflow, `[data-slot="scroll-area"]`, never its inner
 * `[data-slot="scroll-area-viewport"]` content div: that inner div is unbounded
 * (`📜️Scrollable/🟦️.tsx:43` — `min-h-0 min-w-0 w-full`, no height), so its `scrollHeight` always equals
 * its `clientHeight` and its `scrollTop` is permanently `0`. Binding there measured 3720/3720 (extent 0)
 * against the real 466/3722 in a browser, which made every window request answer `offset: 0` and a
 * windowed tree could never stream past its first page (📓️w3-browser-verification.md §5).
 *
 * So the rule is behavioural, not a selector: collect every candidate scroller between the guest `<Tree>`'s
 * own root (it carries `overflow-auto` and IS the scroller wherever the chain above it bounds its height)
 * and the document, nearest first, then take
 * 1. the nearest one that ACTUALLY overflows (`scrollHeight − clientHeight > 1`) — the only proof,
 * 2. else the nearest `📜️Scrollable`, which is where the panel chain will scroll once the body has grown,
 * 3. else the nearest candidate at all, so a tree mounted outside a `📜️Scrollable` still streams,
 * 4. else the document's own scrolling element, for a tree on a plain scrolling page.
 * The effect re-runs on every store revision, so a first-paint body that overflows only after its rows
 * arrive is re-resolved against the grown document rather than measured forever against the wrong box. */
export function treeWindowScrollViewport(root: HTMLElement): HTMLElement | null {
  const view = root.ownerDocument?.defaultView ?? null;
  const isScroller = (candidate: HTMLElement): boolean => {
    if (["scroll-area", "table-window-scroll"].includes(candidate.getAttribute("data-slot") ?? "")) return true;
    const style = view?.getComputedStyle(candidate);
    return [style?.overflowX, style?.overflowY].some((overflow) => overflow === "auto" || overflow === "scroll" || overflow === "overlay");
  };
  const candidates: HTMLElement[] = [];
  if (isScroller(root)) candidates.push(root);
  const inner = root.firstElementChild;
  if (inner instanceof HTMLElement && isScroller(inner)) candidates.push(inner);
  for (let candidate = root.parentElement; candidate; candidate = candidate.parentElement) if (isScroller(candidate)) candidates.push(candidate);
  return candidates.find((candidate) => candidate.scrollHeight - candidate.clientHeight > 1) ?? candidates.find((candidate) => candidate.getAttribute("data-slot") === "scroll-area") ?? candidates[0] ?? treeWindowDocumentScroller(root);
}

/** 🪟️ The page itself, for a tree that has no scrolling ancestor at all. `scrollingElement` is the element
 * whose `scrollTop` a page scroll moves; the `scroll` event for it fires on the DOCUMENT, which is why
 * {@link useTreeWindowObserver} listens on the window too whenever this is the viewport. */
export function treeWindowDocumentScroller(root: HTMLElement): HTMLElement | null {
  const owner = root.ownerDocument ?? null;
  const scrolling = owner?.scrollingElement ?? owner?.documentElement ?? null;
  return scrolling instanceof HTMLElement ? scrolling : null;
}

/** 🪟️ The viewport's own visible box, in the client coordinates every `getBoundingClientRect` already
 * speaks: where its CONTENT box starts on screen, and how tall the part of it a reader can see is.
 *
 * 🧯️ `clientHeight`, never `getBoundingClientRect().height` — the rect is the BORDER box and includes the
 * element's borders and (when it has one) a horizontal scrollbar, so a rect-based `viewportRows` reports
 * rows the user cannot see and a rect-based overlap runs a row past the bottom edge. `clientTop` is the
 * top border, the offset between the rect's top and the first pixel of scrollable content.
 *
 * 🧯️ The document's scrolling element is the one box whose own rect MOVES with the scroll (its `top` is
 * `−scrollY`), so its content origin is the client origin `0` and its visible height is the window's, not
 * its own `clientHeight` (which is the whole document on a quirks-mode page). */
export function treeWindowViewportMetrics(viewport: HTMLElement): { readonly originTop: number; readonly height: number } {
  const owner = viewport.ownerDocument ?? null;
  if (viewport === owner?.scrollingElement || viewport === owner?.documentElement) return { originTop: 0, height: Math.max(0, owner?.defaultView?.innerHeight ?? viewport.clientHeight) };
  const rect = viewport.getBoundingClientRect();
  const inset = treeWindowStickyInsetPx(viewport);
  return { originTop: rect.top + viewport.clientTop + inset, height: Math.max(0, (viewport.clientHeight || rect.height) - inset) };
}

/** 📌️ The band a scroll viewport's own sticky header covers, as the viewport declares it (`scroll-padding-top`, which
 * also keeps focus and `scrollIntoView` from parking a row under that header): rows beneath it are not visible, so the
 * window rule must not spend the served window on them (the table's header row hid the first materialised row). */
function treeWindowStickyInsetPx(viewport: HTMLElement): number {
  const inset = Number.parseFloat(viewport.ownerDocument?.defaultView?.getComputedStyle(viewport).scrollPaddingTop ?? "");
  return Number.isFinite(inset) && inset > 0 ? inset : 0;
}

/** 🪟️ Every windowed container under `root`, measured RELATIVE TO the viewport's content origin — i.e.
 * `top: 0` is the first visible pixel of the scroll container — in the shape
 * `treeWindowRequestsForViewport` consumes with `viewportTop: 0`.
 *
 * Client coordinates, not scroll-content coordinates: only the DIFFERENCE `viewportTop − top` and the
 * container extents reach the rule, both are identical in either space, and this one needs no `scrollTop`
 * bookkeeping — which is what made the document scroller (whose own rect moves with the scroll) and any
 * bordered scroller misreport. A container with `total <= 0` is not windowed and is skipped: reporting it
 * would ask the guest for rows that do not exist. */
export function treeWindowContainersUnder(root: HTMLElement, viewport: HTMLElement): readonly TreeWindowContainerMeasure[] {
  const originTop = treeWindowViewportMetrics(viewport).originTop;
  const measured: TreeWindowContainerMeasure[] = [];
  for (const element of Array.from(root.querySelectorAll("[data-tree-window-key]"))) {
    if (!(element instanceof HTMLElement)) continue;
    if (element.getAttribute("data-tree-window-axis") === "column") continue;
    const key = element.getAttribute("data-tree-window-path") || element.getAttribute("data-tree-window-key");
    if (!key) continue;
    const total = treeWindowAttributeNumber(element, "data-tree-window-total");
    if (total <= 0) continue;
    const rect = element.getBoundingClientRect();
    if (rect.height <= 0) continue;
    measured.push({ key, total, rowExtent: treeWindowRowExtentAttribute(element), offset: treeWindowAttributeNumber(element, "data-tree-window-offset"), length: treeWindowAttributeNumber(element, "data-tree-window-length"), top: rect.top - originTop, height: rect.height, rows: treeWindowRowsUnder(element, originTop) });
  }
  return measured;
}

export const TABLE_COLUMN_WINDOW_WIDTH_PX = 192;
export const TABLE_COLUMN_WINDOW_OVERSCAN = 1;
export const TABLE_COLUMN_WINDOW_MAX = 16;

/** ↔️ Pure horizontal counterpart of one tree-window request. */
export function tableColumnWindowRequestV1(key: string, total: number, scrollLeft: number, viewportWidth: number, columnWidth: number = TABLE_COLUMN_WINDOW_WIDTH_PX): TreeWindowReportV1 {
  const count = Math.max(0, Math.floor(total));
  if (count === 0) return { nodeKey: key, offset: 0, rows: 0 };
  const width = Number.isFinite(columnWidth) && columnWidth > 0 ? columnWidth : TABLE_COLUMN_WINDOW_WIDTH_PX;
  const firstVisible = Math.min(count - 1, Math.max(0, Math.floor(Math.max(0, scrollLeft) / width)));
  const visible = Math.max(1, Math.ceil(Math.max(0, viewportWidth) / width));
  const rows = Math.min(count, TABLE_COLUMN_WINDOW_MAX, visible + 2 * TABLE_COLUMN_WINDOW_OVERSCAN);
  return { nodeKey: key, offset: Math.min(Math.max(0, firstVisible - TABLE_COLUMN_WINDOW_OVERSCAN), count - rows), rows };
}

function tableColumnWindowRequestsUnder(root: HTMLElement): readonly TreeWindowReportV1[] {
  const requests: TreeWindowReportV1[] = [];
  for (const element of Array.from(root.querySelectorAll("[data-tree-window-axis='column']"))) {
    if (!(element instanceof HTMLElement)) continue;
    const key = element.getAttribute("data-tree-window-path") || element.getAttribute("data-tree-window-key");
    const scroller = element.closest<HTMLElement>("[data-slot='table-window-scroll']");
    if (!key || !scroller) continue;
    const total = treeWindowAttributeNumber(element, "data-tree-window-total");
    if (total <= 0) continue;
    requests.push(tableColumnWindowRequestV1(key, total, scroller.scrollLeft, scroller.clientWidth || scroller.getBoundingClientRect().width));
  }
  return requests;
}

/** 📐️ One container's OWN materialised rows, as `{index, top}` in the container's own space. This is what
 * replaces "pixels ÷ one row height" inside a container: a materialised row that is itself an open windowed
 * group is many rows tall, and only its real rect says where the row after it begins
 * (📓️s3-review-streaming-loop.md §2).
 *
 * 🧯️ Ownership is `closest("[data-tree-window-key]")`, not `:scope >` — a row with a context menu is wrapped
 * in one and would vanish from a direct-child query, while a nested group's own rows must stay with that
 * group. Sorted by TOP, not by index: the position rule searches on pixels, and a `direction === "up"` tree
 * paints the slice reversed. */
export function treeWindowRowsUnder(container: HTMLElement, originTop: number): readonly TreeWindowRowMeasure[] {
  const rows: TreeWindowRowMeasure[] = [];
  for (const element of Array.from(container.querySelectorAll("[data-tree-window-row]"))) {
    if (!(element instanceof HTMLElement)) continue;
    if (element.closest("[data-tree-window-key]") !== container) continue;
    const index = Number(element.getAttribute("data-tree-window-row"));
    if (!Number.isFinite(index) || index < 0) continue;
    rows.push({ index: Math.floor(index), top: element.getBoundingClientRect().top - originTop });
  }
  rows.sort((left, right) => left.top - right.top || left.index - right.index);
  return rows;
}

function treeWindowAttributeNumber(element: HTMLElement, attribute: string): number {
  const parsed = Number(element.getAttribute(attribute) ?? "0");
  return Number.isFinite(parsed) && parsed > 0 ? Math.floor(parsed) : 0;
}

/** 🪟️ The closed row geometry the Tree element stamped on this container. A container rendered by an
 * older paint (or a target that never stamped the attribute) reads as `standard`, which is the
 * contract's own `TreeWindowRowExtent::default()` — the measure never carries `undefined`, because the
 * viewport rule prices every row through {@link treeWindowRowExtentPx} and an absent token would make
 * the whole container's geometry `NaN`. */
function treeWindowRowExtentAttribute(element: HTMLElement): TreeWindowRowExtent {
  const token = element.getAttribute("data-tree-window-row-extent");
  return TREE_WINDOW_ROW_EXTENTS.find((extent) => extent === token) ?? "standard";
}

/** 🪟️ The one line a report is compared on — the host is refreshed only when the ANSWER changed, not
 * every time a scroll frame recomputed the same answer (a panel scrolled one pixel inside a row still
 * wants exactly the rows it already has). */
export function treeWindowReportSignatureV1(requests: readonly TreeWindowReportV1[], viewportRows: number): string {
  return `${viewportRows}|${requests.map((request) => `${request.nodeKey}:${request.offset}:${request.rows}`).join(",")}`;
}

/** 🪟️ The rows an off-screen container that has already materialised something is asked for. Zero: it is in
 * the body, so it costs its own node in the guest's ledger either way, but nobody is looking at its rows and
 * every one of them would be a row a VISIBLE container does not get. It keeps the offset it already has, so
 * scrolling back to it lands where the reader left it rather than at row 0 (📓️s3-review-streaming-loop.md §3). */
const TREE_WINDOW_OFFSCREEN_ROWS = 0;

/** 🌱️ …but an off-screen container that has materialised NOTHING yet is asked for one row, not zero.
 *
 * 🧯️ A default-open container below the fold is answered `rows: 0` on first paint, and `rows: 0` renders as an
 * open container with a full-height spacer and no rows — indistinguishable, to a reader, from an empty or
 * collapsed one, wearing the pending ring for as long as it stays off screen (the cad lane's
 * `structure-classic`, 0/11 at a 440 px viewport). One row costs one node, proves the container is not empty
 * and clears the ring; the real window arrives the moment it is scrolled into view. */
const TREE_WINDOW_OFFSCREEN_SEED_ROWS = 1;

/** 🪟️ Every windowed container of one body, priced together: the ones the viewport covers with the rows
 * they need, the rest as spacer-only windows at the offset they already hold. The whole set goes through
 * {@link capTreeWindowRequests} because the guest's ledger charges `1 + rows` for EVERY container in the
 * body, on-screen or not — pricing only the visible ones is what made the host ask 112 and be answered 110
 * on every render (📓️s3-review-streaming-loop.md §1). */
export function treeWindowBodyRequestsV1(containers: readonly TreeWindowContainerMeasure[], viewportHeight: number): readonly TreeWindowRequest[] {
  const visible = treeWindowVisibleRowsForViewport(containers, 0, viewportHeight);
  const wanted = new Map(treeWindowRequestsForViewport(containers, 0, viewportHeight, TREE_WINDOW_OVERSCAN_ROWS).map((request) => [request.key, request] as const));
  const requests = containers.map((container) => {
    const known = wanted.get(container.key);
    if (known) return known;
    const total = Math.max(0, Math.floor(container.total));
    const rows = Math.floor(container.length) > 0 ? TREE_WINDOW_OFFSCREEN_ROWS : Math.min(TREE_WINDOW_OFFSCREEN_SEED_ROWS, total);
    return { key: container.key, offset: Math.min(Math.max(0, Math.floor(container.offset)), Math.max(0, total - rows)), rows };
  });
  return capTreeWindowRequests(requests, visible, TREE_WINDOW_BODY_NODE_BUDGET);
}

/** 🧠️ What the observer keeps of one windowed container between two measures: the window it last asked for, the
 * most rows the guest has proven it serves at the container's current `total` (`null` until an answer came back
 * short), which edge of the viewport a short window is pinned to, and the first row the viewport showed. */
export type TreeWindowServedMemoryV1 = {
  readonly asked: TreeWindowRequest;
  readonly capacity: number | null;
  readonly anchor: "start" | "end";
  readonly firstVisible: number | null;
  readonly total: number;
};

/**
 * 🧠️ {@link treeWindowBodyRequestsV1}, spent where the guest can actually answer it.
 *
 * 🧯️ The body's node ledger is not the guest's only ceiling: `TreeWindows` also prices every materialised row
 * against the reconciler's ITEM budget and ends a window early — a shorter run from the SAME offset — once a row's
 * subtree would overdraw it (`TREE_WINDOW_BODY_ITEM_BUDGET`, region `🔖️PanelWindowing` of `🔌️plugin/🦀️.rs`). Home's
 * spaces rows carry five row actions each, so 27 of them spend it. The viewport rule asks `visible + 2 × overscan`
 * rows starting `overscan` rows above the viewport, which for a list shorter than that is `{offset: 0, rows: total}`
 * wherever the reader scrolls: the guest answered rows 0–26 of 42 forever, the re-measure recomputed the same
 * question, and every row past the 27th was unreachable — a newly created space included (ticket 26/09/23, S18/SH2
 * finding W1).
 *
 * So a window the guest answered SHORT (it began where it was asked and holds fewer rows than were asked and
 * exist) teaches the container's capacity, and from then on the container's window is placed for that capacity:
 * centred on the visible rows when it covers them, else pinned to the edge the reader is scrolling towards (the
 * bottom edge while scrolling down, the top edge while scrolling up; the end of the list when the viewport opens
 * there), so every row of the list is reachable and a keyboard move into the edge row is served. Only the
 * placement moves — the window still asks the viewport rule's rows — so a guest that really stops at the capacity
 * answers the same short window again and the question never changes (silence), a capacity learned from the answer
 * to an older, smaller window is corrected by the very next answer, and a guest whose rows got cheaper simply
 * serves more. A changed `total` forgets the capacity — a different list is priced again.
 */
export function treeWindowServedRequestsV1(
  containers: readonly TreeWindowContainerMeasure[],
  viewportHeight: number,
  memory: ReadonlyMap<string, TreeWindowServedMemoryV1>,
): { readonly requests: readonly TreeWindowRequest[]; readonly memory: ReadonlyMap<string, TreeWindowServedMemoryV1> } {
  const visible = treeWindowVisibleRowsForViewport(containers, 0, viewportHeight);
  const measured = new Map(containers.map((container) => [container.key, container] as const));
  const next = new Map<string, TreeWindowServedMemoryV1>();
  const requests = treeWindowBodyRequestsV1(containers, viewportHeight).map((request) => {
    const container = measured.get(request.key);
    const total = Math.max(0, Math.floor(container?.total ?? 0));
    const offset = Math.max(0, Math.floor(container?.offset ?? 0));
    const length = Math.max(0, Math.floor(container?.length ?? 0));
    const known = memory.get(request.key);
    const priced = known !== undefined && known.total === total ? known : undefined;
    const short = priced !== undefined && length > 0 && offset === priced.asked.offset && length < Math.min(priced.asked.rows, total - offset);
    const capacity = short ? length : priced?.capacity == null ? null : Math.max(priced.capacity, length);
    const metrics = visible.get(request.key);
    const firstVisible = metrics?.firstVisibleRow ?? null;
    const lastVisible = metrics === undefined ? null : metrics.firstVisibleRow + metrics.visibleRows - 1;
    const previous = known?.firstVisible ?? null;
    const anchor: "start" | "end" =
      firstVisible === null || lastVisible === null
        ? (known?.anchor ?? "start")
        : previous === null
          ? firstVisible > 0 && lastVisible >= total - 1
            ? "end"
            : "start"
          : firstVisible > previous
            ? "end"
            : firstVisible < previous
              ? "start"
              : known!.anchor;
    const asked =
      capacity === null || request.rows <= capacity || metrics === undefined || lastVisible === null
        ? request
        : (() => {
            const start = capacity >= metrics.visibleRows ? metrics.firstVisibleRow - Math.floor((capacity - metrics.visibleRows) / 2) : anchor === "end" ? lastVisible - capacity + 1 : metrics.firstVisibleRow;
            const placed = Math.min(Math.max(0, start), Math.max(0, total - capacity));
            return { key: request.key, offset: placed, rows: Math.max(1, Math.min(request.rows, total - placed)) };
          })();
    next.set(request.key, { asked, capacity, anchor, firstVisible: firstVisible ?? previous, total });
    return asked;
  });
  return { requests, memory: next };
}

/** 🔑️ A duplicate `data-tree-window-key` inside one body is an authoring fault, not a host one: two
 * containers would share one open state and one window, and each other's measurements would silently
 * overwrite the other's in the report (📓️s3-review-streaming-loop.md §4). The host cannot repair it — the
 * key is what the guest addresses its containers by — so it says so, loudly, once per body. */
function reportDuplicateTreeWindowKeys(containers: readonly TreeWindowContainerMeasure[], bodyKey: string, reported: Set<string>): void {
  const seen = new Set<string>();
  for (const container of containers) {
    if (!seen.has(container.key)) {
      seen.add(container.key);
      continue;
    }
    const once = `${bodyKey} ${container.key}`;
    if (reported.has(once)) continue;
    reported.add(once);
    console.error(`[tree-window] duplicate key ${JSON.stringify(container.key.split(TREE_WINDOW_PATH_SEPARATOR).join(" › "))} in panel body ${JSON.stringify(bodyKey)} — two windowed containers share one window path, so they share one window and one open state; give sibling containers distinct node keys`);
  }
}

/** 🪟️ Measures the windowed containers under `rootRef` against their scroll viewport and reports the
 * rows the guest should materialise, coalesced to one measurement per animation frame and re-run on
 * every `scroll`, on a viewport resize, and on every store revision AND store identity (a body refresh
 * both bumps the revision and — when the panel config cache misses — hands `TreeView` a whole new
 * `UiDocumentStore` whose revision restarts at its own snapshot's, which a revision-only dependency can
 * read as "unchanged" and never re-measure the tree that just replaced the old one). The diff against
 * the last report is what keeps a scroll gesture from firing one partial refresh per frame, and what
 * makes a SETTLED window silent: the answer is a function of the container geometry alone, and the
 * geometry a window's own answer produces is the geometry that asked for it. */
export function useTreeWindowObserver(rootRef: RefObject<HTMLDivElement | null>, windows: TreeWindowContextValue | null, revision: unknown, store: unknown, onServed?: (memory: ReadonlyMap<string, TreeWindowServedMemoryV1>) => void): void {
  const windowsRef = useRef<TreeWindowContextValue | null>(windows);
  windowsRef.current = windows;
  const onServedRef = useRef(onServed);
  onServedRef.current = onServed;
  const lastReportRef = useRef<string>("");
  const sourceRef = useRef<object>({});
  const reporter = windows ? treeWindowReporter(windows) : null;
  useEffect(() => {
    lastReportRef.current = "";
    return () => reporter?.remove(sourceRef.current);
  }, [reporter]);
  const duplicateKeysRef = useRef<Set<string>>(new Set());
  const servedRef = useRef<ReadonlyMap<string, TreeWindowServedMemoryV1>>(new Map());
  const frameRef = useRef<number | null>(null);
  const bodyKey = windows?.bodyKey ?? null;
  useEffect(() => {
    const root = rootRef.current;
    if (!root || !windowsRef.current) return;
    const viewport = treeWindowScrollViewport(root);
    if (!viewport) return;
    const view = root.ownerDocument?.defaultView;
    if (!view) return;
    const measure = () => {
      frameRef.current = null;
      const channel = windowsRef.current;
      const live = rootRef.current;
      if (!channel || !live || !live.isConnected) return;
      const rowHeight = treeWindowRowHeightPx();
      const viewportHeight = treeWindowViewportMetrics(viewport).height;
      const containers = treeWindowContainersUnder(live, viewport);
      reportDuplicateTreeWindowKeys(containers, channel.bodyKey, duplicateKeysRef.current);
      const served = treeWindowServedRequestsV1(containers, viewportHeight, servedRef.current);
      servedRef.current = served.memory;
      onServedRef.current?.(served.memory);
      const requests = [...served.requests.map((request) => ({ nodeKey: request.key, offset: request.offset, rows: request.rows })), ...tableColumnWindowRequestsUnder(live)];
      const viewportRows = Math.max(1, Math.ceil(viewportHeight / rowHeight));
      const signature = treeWindowReportSignatureV1(requests, viewportRows);
      if (signature === lastReportRef.current) return;
      lastReportRef.current = signature;
      reporter?.publish(sourceRef.current, requests, viewportRows);
    };
    const schedule = () => {
      if (frameRef.current !== null) return;
      frameRef.current = typeof view.requestAnimationFrame === "function" ? view.requestAnimationFrame(measure) : (view.setTimeout(measure, 0) as unknown as number);
    };
    schedule();
    viewport.addEventListener("scroll", schedule, { passive: true });
    const pageScroller = viewport === treeWindowDocumentScroller(root) ? view : null;
    pageScroller?.addEventListener("scroll", schedule, { passive: true });
    const observer = typeof view.ResizeObserver === "function" ? new view.ResizeObserver(schedule) : null;
    observer?.observe(viewport);
    if (observer && root !== viewport) observer.observe(root.firstElementChild instanceof HTMLElement ? root.firstElementChild : root);
    return () => {
      viewport.removeEventListener("scroll", schedule);
      pageScroller?.removeEventListener("scroll", schedule);
      observer?.disconnect();
      if (frameRef.current === null) return;
      if (typeof view.cancelAnimationFrame === "function") view.cancelAnimationFrame(frameRef.current);
      else view.clearTimeout(frameRef.current);
      frameRef.current = null;
    };
  }, [rootRef, revision, store, bodyKey, reporter]);
}

/** 🕹️ What one conversion pass of a guest tree collects and carries down the recursion — every field is
 * filled WHILE walking, so a row's own click closure can read the whole tree's pick table (a range pick
 * resolves ids the walk had not reached yet when that row was converted). */
export type TreeWalkContextV1 = {
  /** 🪟️ Host-owned expansion, keyed by window path (windowed containers) or authored node key (everything else). */
  readonly openStates?: Readonly<Record<string, boolean>>;
  /** 🪟️ Filled while walking: the same expansion re-keyed onto `<Tree>`'s DOM ids. */
  readonly domOpenStates?: Record<string, boolean>;
  /** 🪟️ Filled while walking: DOM id → authored key, so an `onOpenStateChange` maps back. */
  readonly keysByDomId?: Map<string, string>;
  /** 🕹️ The tree root's own `activate` binding — the SDK's tree-level `interactionSelect`, bound once by
   * `PanelTreeBuilder::interaction_domain`, which is what makes a bare `granularity` row a pick target
   * that costs zero argument arena. */
  readonly pick?: { readonly record: UiNodeRecord; readonly binding: ActionBinding };
  /** 🕹️ Filled while walking: DOM id → the interaction target that row stands for. */
  readonly pickTargets?: Map<string, { readonly key: string; readonly granularity: string }>;
};

/** 🕹️ The interaction targets one pick dispatches. A plain pick is the clicked row alone; a `range`
 * pick is the `<Tree>`'s resolved selection (already including the clicked row — `handleSelectItem`
 * hands `onClick` the NEXT selection, not the previous one) mapped through the walk's table, with the
 * clicked row appended as the floor so a selection the table cannot resolve still picks something.
 * Deduplicated on `(granularity, id)`, exactly as `world3dSelectionActionArgs` deduplicates ids. */
export function treePickTargetsV1(walk: TreeWalkContextV1, key: string, granularity: string, merge: MergeMode, selectedIds: readonly string[]): readonly { readonly granularity: string; readonly id: string }[] {
  if (merge !== "range") return [{ granularity, id: key }];
  const seen = new Set<string>();
  const targets: { readonly granularity: string; readonly id: string }[] = [];
  const push = (granularityId: string, id: string) => {
    const dedupe = `${granularityId} ${id}`;
    if (seen.has(dedupe)) return;
    seen.add(dedupe);
    targets.push({ granularity: granularityId, id });
  };
  for (const domId of selectedIds) {
    const entry = walk.pickTargets?.get(domId);
    if (entry) push(entry.granularity, entry.key);
  }
  push(granularity, key);
  return targets;
}

/** 🕹️ The `UiValue` map a tree pick puts on the wire — the SAME shape `world3dSelectionActionArgs`
 * builds (`targets` a JSON string of `{granularity, id}` records, `method: "pick"`, `merge` a raw
 * `MergeMode` word the framework's `parse_merge_mode` accepts verbatim), minus `domainId`: the tree's
 * own `Activate` binding already carries `{domainId}` in its authored args, and `uiIntentPayload`
 * merges this map OVER them. `"range"` has no ordered topology on the wire, so a range gesture is
 * resolved host-side into the full id set and sent as a `"replace"`. */
export function treePickIntentInputV1(merge: MergeMode, targets: readonly { readonly granularity: string; readonly id: string }[]): UiValue {
  return { merge: merge === "range" ? "replace" : merge, method: "pick", targets: JSON.stringify(targets.map((target) => ({ granularity: target.granularity, id: target.id }))) } as unknown as UiValue;
}

export function dispatchTreePick(context: UiInterpreterContext, walk: TreeWalkContextV1, key: string, granularity: string, event: ReactMouseEvent, activation: TreeDataActivationContext): void {
  const pick = walk.pick;
  if (!pick) return;
  const merge = interactionMergeFromModifiers(event);
  void context.onIntent(context.store.buildIntent(pick.record, pick.binding, treePickIntentInputV1(merge, treePickTargetsV1(walk, key, granularity, merge, activation.selectedIds))));
}

/** 🔑️ `🌳️Tree` keys its expansion map by a ROLE-PREFIXED form of the row's DOM id
 * (`getTreeSectionStateId` → `tree-section-<id>`, `getTreeItemStateId` → `tree-item-<id>`; both are
 * private to that element and neither is exported). A host-controlled map therefore carries EVERY
 * spelling of a row it knows about and strips the prefix on the way back. Unambiguous because a
 * {@link uiNodeDomId} is `<surface>/<key>` and a panel surface is always `panel:…`, so no DOM id can
 * itself begin with one of these prefixes. */
const TREE_OPEN_STATE_ID_PREFIXES = ["tree-section-", "tree-item-"] as const;

export function treeOpenStateIdsForDomIdV1(domId: string): readonly string[] {
  return [domId, ...TREE_OPEN_STATE_ID_PREFIXES.map((prefix) => `${prefix}${domId}`)];
}

export function treeDomIdFromOpenStateIdV1(stateId: string): string {
  for (const prefix of TREE_OPEN_STATE_ID_PREFIXES) if (stateId.startsWith(prefix)) return stateId.slice(prefix.length);
  return stateId;
}

/** 🪟️ Records one converted row's identity in the walk's translation tables. `identity` is the row's WINDOW
 * PATH when it is a windowed container and its authored key otherwise — the same string the host's open map,
 * its window map and `ViewModel::tree_windows` are keyed by, and the one an `onOpenStateChange` reports back. */
export function registerTreeWalkRow(walk: TreeWalkContextV1 | undefined, identity: string, domId: string): void {
  if (!walk || !identity) return;
  const key = identity;
  walk.keysByDomId?.set(domId, key);
  const open = walk.openStates?.[key];
  if (open === undefined || !walk.domOpenStates) return;
  for (const stateId of treeOpenStateIdsForDomIdV1(domId)) walk.domOpenStates[stateId] = open;
}
//#endregion 🪟️TreeWindows
//#endregion Tree
//#endregion ComponentRenderers
//#endregion 🔖️UiInterpreter
