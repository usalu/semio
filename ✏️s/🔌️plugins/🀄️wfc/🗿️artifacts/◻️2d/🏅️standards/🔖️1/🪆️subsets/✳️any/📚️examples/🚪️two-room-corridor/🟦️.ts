// 🚪️ Example `two-room-corridor` — the TypeScript twin of `🦀️.rs`'s builder, ported row by row.

import type { Wfc2dColor, Wfc2dSnapshot, Wfc2dTileMedia } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import { WFC_2D_DEFAULT_RELATION, WFC_2D_DOCUMENT_SCHEMA } from "../../🧬️schema/📸️snapshot/🟦️.ts";

export const ID = "two-room-corridor";
export const SEED = 7;

/** 🎨 A full-bleed filled square in tile space — the simplest honest vector tile. */
export function filledSquare(fill: Wfc2dColor): Wfc2dTileMedia {
  return {
    Vector: {
      paths: [
        {
          segments: [{ Move: { to: [0.05, 0.05] } }, { Line: { to: [0.95, 0.05] } }, { Line: { to: [0.95, 0.95] } }, { Line: { to: [0.05, 0.95] } }, "Close"],
          fill,
          strokeWidth: 0,
        },
      ],
    },
  };
}

const slot = (id: string, x: number) => ({ id, x, y: 0, width: 2, height: 2 });
const edge = (id: string, fromSlotId: string, toSlotId: string) => ({ id, fromSlotId, toSlotId, relation: WFC_2D_DEFAULT_RELATION });

/** 🚪️ The authored problem spec, in canonical ascending `id` order. */
export function document(): Wfc2dSnapshot {
  return {
    schema: WFC_2D_DOCUMENT_SCHEMA,
    seed: SEED,
    slots: [slot("corridor", 2), slot("room-a", 0), slot("room-b", 4)],
    edges: [edge("edge-a-corridor", "room-a", "corridor"), edge("edge-corridor-b", "corridor", "room-b")],
    tiles: [
      { id: "corridor", label: "Corridor", weight: 1, media: filledSquare({ r: 148, g: 163, b: 184, a: 255 }) },
      { id: "room", label: "Room", weight: 2, media: filledSquare({ r: 96, g: 165, b: 250, a: 255 }) },
    ],
    rules: [
      { id: "rule-corridor-corridor", tileAId: "corridor", tileBId: "corridor", allowed: false },
      { id: "rule-room-corridor", tileAId: "room", tileBId: "corridor", allowed: true },
      { id: "rule-room-room", tileAId: "room", tileBId: "room", allowed: false },
    ],
  };
}
