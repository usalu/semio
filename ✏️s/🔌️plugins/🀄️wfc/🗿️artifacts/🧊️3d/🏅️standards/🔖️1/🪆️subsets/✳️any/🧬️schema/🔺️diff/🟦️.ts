/** 🔺️ wfc3d sparse structural delta. Each `*Upserted` entry is a `[index, value]` tuple — the index
 * is the member's insertion/replacement position, which is what makes an inverse position-exact. */
import type { GraphRule, Slot3d, SlotEdge, Tile } from "../📸️snapshot/🟦️";

export interface Wfc3dDiff {
  schema: string | null;
  seed: number | null;
  slotsRemoved: string[];
  slotsUpserted: [number, Slot3d][];
  edgesRemoved: string[];
  edgesUpserted: [number, SlotEdge][];
  tilesRemoved: string[];
  tilesUpserted: [number, Tile][];
  rulesRemoved: string[];
  rulesUpserted: [number, GraphRule][];
}
