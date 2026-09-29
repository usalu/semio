/**
 * 🪟️ WG11 P5: writes the neutral tree-window request vectors from React's own rule (`🌳️Tree/🟦️.tsx`) — the oracle both hosts
 * are then held to. `treeWindowBodyRequestsV1` (os Interpreter, vite-only imports) is composed here from the same Tree
 * primitives it calls; the Interpreter's own vitest law asserts its answer equals the `body` column, so a drift in this
 * composition fails there. usage: bun run gen-window-requests.ts <out.json>
 */
import { writeFileSync } from "node:fs";
import {
  TREE_WINDOW_BODY_NODE_BUDGET,
  TREE_WINDOW_OVERSCAN_ROWS,
  TREE_WINDOW_ROW_EXTENTS,
  capTreeWindowRequests,
  treeWindowRequestsForViewport,
  treeWindowRowExtentPx,
  treeWindowVisibleRowsForViewport,
  type TreeWindowContainerMeasure,
} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx";

type Spec = { key: string; total: number; offset: number; length: number; topRows: number; rowExtent?: "standard" | "compactText" | "compactSmallControl" | "compactControl"; expanded?: { atRow: number; rows: number } };

const px = (extent: Spec["rowExtent"]) => treeWindowRowExtentPx(extent ?? "standard");

function measure(spec: Spec): TreeWindowContainerMeasure & { readonly rows: readonly { index: number; top: number }[] } {
  const pitch = px(spec.rowExtent);
  const top = spec.topRows * pitch;
  const extra = spec.expanded ? spec.expanded.rows * pitch : 0;
  const rows = Array.from({ length: spec.length }, (_, position) => {
    const index = spec.offset + position;
    const below = spec.expanded && index > spec.expanded.atRow ? extra : 0;
    return { index, top: top + index * pitch + below };
  });
  return { key: spec.key, total: spec.total, offset: spec.offset, length: spec.length, top, height: spec.total * pitch + extra, rowExtent: spec.rowExtent ?? "standard", rows };
}

function bodyRequests(containers: readonly TreeWindowContainerMeasure[], viewportHeight: number) {
  const visible = treeWindowVisibleRowsForViewport(containers, 0, viewportHeight);
  const wanted = new Map(treeWindowRequestsForViewport(containers, 0, viewportHeight, TREE_WINDOW_OVERSCAN_ROWS).map((request) => [request.key, request] as const));
  const requests = containers.map((container) => {
    const known = wanted.get(container.key);
    if (known) return known;
    const total = Math.max(0, Math.floor(container.total));
    const rows = Math.floor(container.length) > 0 ? 0 : Math.min(1, total);
    return { key: container.key, offset: Math.min(Math.max(0, Math.floor(container.offset)), Math.max(0, total - rows)), rows };
  });
  return capTreeWindowRequests(requests, visible, TREE_WINDOW_BODY_NODE_BUDGET);
}

const cases: { name: string; viewportRows: number; containers: Spec[] }[] = [
  { name: "a short list inside the viewport asks for all of it", viewportRows: 32, containers: [{ key: "spaces", total: 10, offset: 0, length: 10, topRows: 0 }] },
  { name: "a 10 000-row list scrolled to its middle asks the visible run plus one overscan per edge", viewportRows: 20, containers: [{ key: "rows", total: 10000, offset: 4980, length: 48, topRows: -5000 }] },
  { name: "a 10 000-row list scrolled to its end asks a window that ends on the last row", viewportRows: 20, containers: [{ key: "rows", total: 10000, offset: 9952, length: 48, topRows: -9980 }] },
  { name: "a viewport above the materialised band reads the leading spacer pitch", viewportRows: 20, containers: [{ key: "rows", total: 10000, offset: 9952, length: 48, topRows: -120 }] },
  {
    name: "two open sections over the body budget keep their visible runs and drop overscan first",
    viewportRows: 90,
    containers: [
      { key: "a", total: 50, offset: 0, length: 50, topRows: -10 },
      { key: "b", total: 400, offset: 0, length: 36, topRows: 40 },
    ],
  },
  {
    name: "three containers far over budget trim rows hardest from the one furthest from the viewport centre",
    viewportRows: 90,
    containers: [
      { key: "near", total: 500, offset: 0, length: 0, topRows: 0 },
      { key: "mid", total: 500, offset: 0, length: 0, topRows: -470 },
      { key: "far", total: 60, offset: 0, length: 0, topRows: 88 },
    ],
  },
  {
    name: "off-screen containers keep their offset: a materialised one asks zero rows, an empty one one seed row",
    viewportRows: 20,
    containers: [
      { key: "visible", total: 50, offset: 0, length: 36, topRows: 0 },
      { key: "held", total: 80, offset: 12, length: 20, topRows: 200 },
      { key: "seed", total: 80, offset: 0, length: 0, topRows: 400 },
    ],
  },
  { name: "an expanded row's subtree is read from the real row tops, never a uniform pitch", viewportRows: 16, containers: [{ key: "outline", total: 40, offset: 0, length: 40, topRows: -6, expanded: { atRow: 2, rows: 30 } }] },
  { name: "a viewport shorter than one row still asks one visible row", viewportRows: 0.25, containers: [{ key: "tiny", total: 30, offset: 0, length: 10, topRows: -3 }] },
  { name: "a compact-text window prices rows at its own pitch", viewportRows: 20, containers: [{ key: "log", total: 900, offset: 100, length: 60, topRows: -130, rowExtent: "compactText" }] },
  { name: "a container that ends above the viewport is not asked at all", viewportRows: 20, containers: [{ key: "above", total: 10, offset: 0, length: 10, topRows: -40 }, { key: "on", total: 100, offset: 0, length: 40, topRows: 0 }] },
];

const rowPx = 24;
const out = {
  $comment: "🪟️ Neutral tree-window request vectors (WG11 P5, ticket 26/09/23 session 14d): React's rule (🌳️Tree/🟦️.tsx region 🪟️TreeWindow + 🗣️Interpreter treeWindowBodyRequestsV1) wrote the visible/requests/body columns; the wgpu twin (🌳️Tree/🪟️window/🦀️.rs) must answer the same. Container geometry is in viewport space (content origin 0); rows lists each materialised row [index, top].",
  budget: TREE_WINDOW_BODY_NODE_BUDGET,
  overscan: TREE_WINDOW_OVERSCAN_ROWS,
  rowExtentPx: Object.fromEntries(TREE_WINDOW_ROW_EXTENTS.map((extent) => [extent, px(extent)])),
  cases: cases.map((law) => {
    const viewportHeight = law.viewportRows * rowPx;
    const containers = law.containers.map(measure);
    const visible = treeWindowVisibleRowsForViewport(containers, 0, viewportHeight);
    return {
      name: law.name,
      viewportHeight,
      containers: containers.map((container) => ({ key: container.key, total: container.total, offset: container.offset, length: container.length, top: container.top, height: container.height, rowExtent: container.rowExtent, rows: container.rows.map((row) => [row.index, row.top]) })),
      visible: [...visible.entries()].map(([key, rows]) => ({ key, ...rows })),
      requests: treeWindowRequestsForViewport(containers, 0, viewportHeight, TREE_WINDOW_OVERSCAN_ROWS),
      body: bodyRequests(containers, viewportHeight),
    };
  }),
};
writeFileSync(process.argv[2]!, JSON.stringify(out, null, 2) + "\n");
console.log(`wrote ${out.cases.length} cases`);
