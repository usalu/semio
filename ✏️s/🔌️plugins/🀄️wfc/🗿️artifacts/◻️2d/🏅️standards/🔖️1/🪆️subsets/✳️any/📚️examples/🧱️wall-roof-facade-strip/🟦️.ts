// 🧱️ Example `wall-roof-facade-strip` — the TypeScript twin of `🦀️.rs`'s builder.

import type { Wfc2dSnapshot } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import { WFC_2D_DOCUMENT_SCHEMA } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import { filledSquare } from "../🚪️two-room-corridor/🟦️.ts";

export const ID = "wall-roof-facade-strip";
export const SEED = 42;
export const RELATION_BESIDE = "beside";
export const RELATION_ABOVE = "above";

const slot = (id: string, x: number, y: number, pinnedTileId?: string) => (pinnedTileId === undefined ? { id, x, y, width: 2, height: 2 } : { id, x, y, width: 2, height: 2, pinnedTileId });
const edge = (id: string, fromSlotId: string, toSlotId: string, relation: string) => ({ id, fromSlotId, toSlotId, relation });

/** 🧱️ The authored problem spec, in canonical ascending `id` order. */
export function document(): Wfc2dSnapshot {
  return {
    schema: WFC_2D_DOCUMENT_SCHEMA,
    seed: SEED,
    slots: [slot("bay-0-ground", 0, 2), slot("bay-0-top", 0, 0), slot("bay-1-ground", 2, 2), slot("bay-1-top", 2, 0, "roof")],
    edges: [
      edge("edge-bay-0-stack", "bay-0-ground", "bay-0-top", RELATION_ABOVE),
      edge("edge-bay-1-stack", "bay-1-ground", "bay-1-top", RELATION_ABOVE),
      edge("edge-ground-row", "bay-0-ground", "bay-1-ground", RELATION_BESIDE),
      edge("edge-top-row", "bay-0-top", "bay-1-top", RELATION_BESIDE),
    ],
    tiles: [
      { id: "roof", label: "Roof", weight: 1, media: filledSquare({ r: 206, g: 84, b: 62, a: 255 }) },
      { id: "wall", label: "Wall", weight: 3, media: filledSquare({ r: 122, g: 126, b: 134, a: 255 }) },
    ],
    rules: [
      { id: "rule-roof-beside-roof", tileAId: "roof", tileBId: "roof", relation: RELATION_BESIDE, allowed: false },
      { id: "rule-wall-roof", tileAId: "wall", tileBId: "roof", allowed: true },
      { id: "rule-wall-wall", tileAId: "wall", tileBId: "wall", allowed: true },
    ],
  };
}
