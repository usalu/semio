// 🔷️ Example `hex-ring` — the TypeScript twin of `🦀️.rs`'s builder, same rounding, same ids.

import type { Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import { WFC_2D_DOCUMENT_SCHEMA } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import { filledSquare } from "../🚪️two-room-corridor/🟦️.ts";

export const ID = "hex-ring";
export const SEED = 2026;
export const RELATION_RING = "ring";
const RADIUS = 6;
const SLOT_SIZE = 2;

const round = (value: number): number => Math.round(value * 1000) / 1000;

/** 📐 The `index`-th vertex of a regular hexagon, rounded exactly as Rust rounds it. */
function hexSlot(index: number): Wfc2dSlot {
  const angle = (Math.PI / 3) * index;
  return { id: `hex-${index}`, x: round(RADIUS * Math.cos(angle)), y: round(RADIUS * Math.sin(angle)), width: SLOT_SIZE, height: SLOT_SIZE };
}

function ringEdge(index: number): Wfc2dSlotEdge {
  const next = (index + 1) % 6;
  return { id: `edge-${index}-${next}`, fromSlotId: `hex-${index}`, toSlotId: `hex-${next}`, relation: RELATION_RING };
}

/** 🔷️ The authored problem spec. Both collections come out in ascending `id` order by construction. */
export function document(): Wfc2dSnapshot {
  const indices = [0, 1, 2, 3, 4, 5];
  return {
    schema: WFC_2D_DOCUMENT_SCHEMA,
    seed: SEED,
    slots: indices.map(hexSlot),
    edges: indices.map(ringEdge),
    tiles: [
      { id: "cap", label: "Cap", weight: 1, media: filledSquare({ r: 250, g: 204, b: 21, a: 255 }) },
      { id: "link", label: "Link", weight: 2, media: filledSquare({ r: 52, g: 211, b: 153, a: 255 }) },
    ],
    rules: [
      { id: "rule-cap-cap", tileAId: "cap", tileBId: "cap", relation: RELATION_RING, allowed: false },
      { id: "rule-cap-link", tileAId: "cap", tileBId: "link", allowed: true },
      { id: "rule-link-link", tileAId: "link", tileBId: "link", allowed: true },
    ],
  };
}
