/** 🔺️ wfc3d field-sparse structural delta. Every list carries removed identities, added rows (landing at their canonical
 * position) and per-row field patches — never a whole row or list copy. */
import type { GraphRule, Slot3d, SlotEdge, Tile } from "../📸️snapshot/🟦️";

/** 🎚️ An optional field set to a value or cleared — the wire shape `{ "value": … }`. */
export interface Wfc3dOptional<T> {
  value: T | null;
}

/** 🩹 One patched row, addressed by its identity. */
export interface Wfc3dRowPatch<P> {
  id: string;
  patch: P;
}

/** 📂 Id-keyed row delta. */
export interface Wfc3dRows<T, P> {
  removed: string[];
  added: T[];
  patched: Wfc3dRowPatch<P>[];
}

/** 🩹 Field patch over a slot: `null` leaves a field alone. */
export interface Wfc3dSlotPatch {
  x: number | null;
  y: number | null;
  z: number | null;
  width: number | null;
  height: number | null;
  depth: number | null;
  pinnedTileId: Wfc3dOptional<string> | null;
}

/** 🩹 Field patch over an adjacency edge. */
export interface Wfc3dEdgePatch {
  fromSlotId: string | null;
  toSlotId: string | null;
  relation: string | null;
}

/** 🩹 Field patch over a tile. */
export interface Wfc3dTilePatch {
  weight: number | null;
  media: Tile["media"] | null;
  label: Wfc3dOptional<string> | null;
}

/** 🩹 Field patch over an adjacency rule. */
export interface Wfc3dRulePatch {
  tileAId: string | null;
  tileBId: string | null;
  allowed: boolean | null;
  relation: Wfc3dOptional<string> | null;
}

export interface Wfc3dDiff {
  schema: string | null;
  seed: bigint | null;
  slots: Wfc3dRows<Slot3d, Wfc3dSlotPatch>;
  edges: Wfc3dRows<SlotEdge, Wfc3dEdgePatch>;
  tiles: Wfc3dRows<Tile, Wfc3dTilePatch>;
  rules: Wfc3dRows<GraphRule, Wfc3dRulePatch>;
}
