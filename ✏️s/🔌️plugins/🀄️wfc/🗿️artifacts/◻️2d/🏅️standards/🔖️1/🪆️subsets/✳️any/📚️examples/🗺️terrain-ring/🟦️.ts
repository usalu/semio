// 🗺️ Example `terrain-ring` — the TypeScript twin of `🦀️.rs`'s builder, including the same
// palette-indexed 8×8 raster payloads (built the same way, so the base64 matches byte for byte).

import type { Wfc2dColor, Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import { WFC_2D_DOCUMENT_SCHEMA } from "../../🧬️schema/📸️snapshot/🟦️.ts";

export const ID = "terrain-ring";
export const SEED = 88;
export const RELATION_RING = "ring";
export const TILE_PIXELS = 8;
const RADIUS = 6;
const SLOT_SIZE = 2;

const round = (value: number): number => Math.round(value * 1000) / 1000;

/** 🎨 An 8×8 tile whose top `horizon` rows are palette index 0 and the rest index 1. */
function bandedTile(id: string, label: string, weight: number, top: Wfc2dColor, bottom: Wfc2dColor, horizon: number): Wfc2dTile {
  const indices: number[] = [];
  for (let row = 0; row < TILE_PIXELS; row += 1) {
    for (let column = 0; column < TILE_PIXELS; column += 1) indices.push(row >= horizon ? 1 : 0);
  }
  const pixels = btoa(String.fromCharCode(...indices));
  return { id, label, weight, media: { Bitmap: { width: TILE_PIXELS, height: TILE_PIXELS, palette: [top, bottom], pixels } } };
}

function hexSlot(index: number): Wfc2dSlot {
  const angle = (Math.PI / 3) * index;
  return { id: `hex-${index}`, x: round(RADIUS * Math.cos(angle)), y: round(RADIUS * Math.sin(angle)), width: SLOT_SIZE, height: SLOT_SIZE };
}

function ringEdge(index: number): Wfc2dSlotEdge {
  const next = (index + 1) % 6;
  return { id: `edge-${index}-${next}`, fromSlotId: `hex-${index}`, toSlotId: `hex-${next}`, relation: RELATION_RING };
}

const allow = (id: string, tileAId: string, tileBId: string): Wfc2dRule => ({ id, tileAId, tileBId, allowed: true });

/** 🗺️ The authored problem spec, in canonical ascending `id` order. */
export function document(): Wfc2dSnapshot {
  const indices = [0, 1, 2, 3, 4, 5];
  return {
    schema: WFC_2D_DOCUMENT_SCHEMA,
    seed: SEED,
    slots: indices.map(hexSlot),
    edges: indices.map(ringEdge),
    tiles: [
      bandedTile("grass", "Grass", 2, { r: 132, g: 204, b: 22, a: 255 }, { r: 77, g: 124, b: 15, a: 255 }, 5),
      bandedTile("rock", "Rock", 1, { r: 168, g: 162, b: 158, a: 255 }, { r: 87, g: 83, b: 78, a: 255 }, 2),
      bandedTile("shore", "Shore", 2, { r: 250, g: 232, b: 176, a: 255 }, { r: 214, g: 188, b: 120, a: 255 }, 4),
      bandedTile("water", "Water", 3, { r: 96, g: 165, b: 250, a: 255 }, { r: 30, g: 64, b: 175, a: 255 }, 3),
    ],
    rules: [
      allow("rule-grass-grass", "grass", "grass"),
      allow("rule-grass-rock", "grass", "rock"),
      allow("rule-rock-rock", "rock", "rock"),
      allow("rule-shore-grass", "shore", "grass"),
      allow("rule-shore-shore", "shore", "shore"),
      { id: "rule-water-rock", tileAId: "water", tileBId: "rock", relation: RELATION_RING, allowed: false },
      allow("rule-water-shore", "water", "shore"),
      allow("rule-water-water", "water", "water"),
    ],
  };
}
