/** 🧬 s.wfc.grid3d mutations — the TypeScript twin of the semantic vocabulary. The wire form is
 * internally tagged by the variant name, exactly as the committed fixture quintets carry it. */

import type { Grid3dAxis, Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dTile, Grid3dTileMedia } from "../📸️snapshot/🟦️.ts";

export type Grid3dMutation =
  | { ChangeSeed: { seed: number } }
  | { ResizeGrid: { width: number; height: number; depth: number } }
  | { ChangeCellSizes: { axis: Grid3dAxis; sizes: number[] } }
  | { ChangePeriodicity: { periodicX: boolean; periodicY: boolean; periodicZ: boolean } }
  | { CreateTile: { tile: Grid3dTile } }
  | { DeleteTile: { id: string } }
  | { ChangeTileWeight: { tileId: string; weight: number } }
  | { ChangeTileMedia: { tileId: string; media: Grid3dTileMedia } }
  | { CreateRule: { rule: Grid3dRule } }
  | { DeleteRule: { id: string } }
  | { PinCell: { pinned: Grid3dPinnedCell } }
  | { UnpinCell: { x: number; y: number; z: number } }
  | { MaskCell: { cell: Grid3dCell } }
  | { UnmaskCell: { x: number; y: number; z: number } };

/** 🏷️ The kebab-case spelling of every variant, in declaration order — the same roster the Rust
 * `KINDS` const and the oracle catalog declare. */
export const GRID3D_MUTATION_KINDS = [
  "change-seed",
  "resize-grid",
  "change-cell-sizes",
  "change-periodicity",
  "create-tile",
  "delete-tile",
  "change-tile-weight",
  "change-tile-media",
  "create-rule",
  "delete-rule",
  "pin-cell",
  "unpin-cell",
  "mask-cell",
  "unmask-cell",
] as const;

export type Grid3dMutationKind = (typeof GRID3D_MUTATION_KINDS)[number];

/** 🏷️ The semantic kind of one wire mutation. */
export function grid3dMutationKind(mutation: Grid3dMutation): Grid3dMutationKind {
  const variant = Object.keys(mutation)[0];
  const kebab = variant.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
  return kebab as Grid3dMutationKind;
}

/** 📐 Where a key belongs in an already-sorted collection — the twin of the Rust `ordered_index`. */
export function orderedIndex<T>(items: T[], key: string, itemKey: (item: T) => string): number {
  const at = items.findIndex((item) => itemKey(item) >= key);
  return at < 0 ? items.length : at;
}
