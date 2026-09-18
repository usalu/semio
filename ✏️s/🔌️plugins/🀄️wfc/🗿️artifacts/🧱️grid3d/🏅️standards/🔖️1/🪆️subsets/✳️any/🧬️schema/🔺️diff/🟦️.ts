/** 🔺 s.wfc.grid3d diff — a sparse, key-keyed delta. An indexed upsert rides as a `[index, member]`
 * pair, exactly as the Rust `Vec<(usize, T)>` lanes encode it. */

import type { Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile } from "../📸️snapshot/🟦️.ts";

export type Indexed<T> = [number, T];

export interface Grid3dDiff {
  schema: string | null;
  seed: number | null;
  width: number | null;
  height: number | null;
  depth: number | null;
  cellSizesX: number[] | null;
  cellSizesY: number[] | null;
  cellSizesZ: number[] | null;
  periodicX: boolean | null;
  periodicY: boolean | null;
  periodicZ: boolean | null;
  tilesRemoved: string[];
  tilesUpserted: Indexed<Grid3dTile>[];
  rulesRemoved: string[];
  rulesUpserted: Indexed<Grid3dRule>[];
  pinnedRemoved: string[];
  pinnedUpserted: Indexed<Grid3dPinnedCell>[];
  maskedRemoved: string[];
  maskedUpserted: Indexed<Grid3dCell>[];
}

export function emptyGrid3dDiff(): Grid3dDiff {
  return {
    schema: null,
    seed: null,
    width: null,
    height: null,
    depth: null,
    cellSizesX: null,
    cellSizesY: null,
    cellSizesZ: null,
    periodicX: null,
    periodicY: null,
    periodicZ: null,
    tilesRemoved: [],
    tilesUpserted: [],
    rulesRemoved: [],
    rulesUpserted: [],
    pinnedRemoved: [],
    pinnedUpserted: [],
    maskedRemoved: [],
    maskedUpserted: [],
  };
}

/** 🧬 Applies one key-keyed collection delta — the twin of the Rust `apply_collection`, so the
 * cross-language fixture oracle compares two real implementations rather than one and a stub. */
export function applyCollection<T>(base: T[], removed: string[], upserted: Indexed<T>[], key: (item: T) => string): T[] {
  const items = base.filter((item) => !removed.includes(key(item)));
  for (const [index, value] of upserted) {
    const at = items.findIndex((item) => key(item) === key(value));
    if (at >= 0) items[at] = value;
    else items.splice(index, 0, value);
  }
  return items;
}

/** 🔺 Applies a whole diff to a snapshot. */
export function applyGrid3dDiff(base: Grid3dSnapshot, diff: Grid3dDiff): Grid3dSnapshot {
  const cell = (item: { x: number; y: number; z: number }) => `${item.x}:${item.y}:${item.z}`;
  return {
    ...base,
    schema: diff.schema ?? base.schema,
    seed: diff.seed ?? base.seed,
    width: diff.width ?? base.width,
    height: diff.height ?? base.height,
    depth: diff.depth ?? base.depth,
    cellSizesX: diff.cellSizesX ?? base.cellSizesX,
    cellSizesY: diff.cellSizesY ?? base.cellSizesY,
    cellSizesZ: diff.cellSizesZ ?? base.cellSizesZ,
    periodicX: diff.periodicX ?? base.periodicX,
    periodicY: diff.periodicY ?? base.periodicY,
    periodicZ: diff.periodicZ ?? base.periodicZ,
    tiles: applyCollection(base.tiles, diff.tilesRemoved, diff.tilesUpserted, (tile) => tile.id),
    rules: applyCollection(base.rules, diff.rulesRemoved, diff.rulesUpserted, (rule) => rule.id),
    pinned: applyCollection(base.pinned, diff.pinnedRemoved, diff.pinnedUpserted, cell),
    masked: applyCollection(base.masked, diff.maskedRemoved, diff.maskedUpserted, cell),
  };
}
