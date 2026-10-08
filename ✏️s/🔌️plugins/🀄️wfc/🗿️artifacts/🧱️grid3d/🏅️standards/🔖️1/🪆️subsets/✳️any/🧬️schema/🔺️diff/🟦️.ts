import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🔺 s.wfc.grid3d diff — a field-sparse delta: scalars are optional absolute values, cell sizes are sparse axis patches, every list carries removed keys,
 * added rows (landing at their canonical position) and per-row field patches. */

import type { Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile } from "../📸️snapshot/🟦️.ts";

/** 🎚️ An optional field set to a value or cleared — the wire shape `{ "value": … }`. */
export interface Grid3dOptional<T> {
  value: T | null;
}

/** 🩹 One patched row, addressed by its identity. */
export interface Grid3dRowPatch<Q> {
  id: string;
  patch: Q;
}

/** 📂 Id-keyed row delta: removed identities, added rows (landing at their canonical position) and per-row field patches. */
export interface Grid3dRows<T, Q> {
  removed: string[];
  added: T[];
  patched: Grid3dRowPatch<Q>[];
}

/** 🕳️ An empty row delta. */
export function emptyGrid3dRows<T, Q>(): Grid3dRows<T, Q> {
  return { removed: [], added: [], patched: [] };
}

/** 🧬️ One keyed collection's apply — the TS twin of the Rust rows apply: removals first, then canonical-position insertions, then field patches. */
function applyRows<T, Q>(base: readonly T[], rows: Grid3dRows<T, Q>, key: (row: T) => string, before: (existing: T, added: T) => boolean, patched: (row: T, patch: Q) => T): T[] {
  const items = [...base];
  for (const id of rows.removed) {
    const at = items.findIndex((item) => key(item) === id);
    if (at === -1) throw new RangeError(`removed ${id} does not exist`);
    items.splice(at, 1);
  }
  for (const row of rows.added) {
    if (items.some((item) => key(item) === key(row))) throw new RangeError(`added ${key(row)} already exists`);
    const at = items.findIndex((item) => before(item, row));
    items.splice(at === -1 ? items.length : at, 0, row);
  }
  for (const entry of rows.patched) {
    const at = items.findIndex((item) => key(item) === entry.id);
    if (at === -1) throw new RangeError(`patched ${entry.id} does not exist`);
    items[at] = patched(items[at]!, entry.patch);
  }
  return items;
}

/** 📏 One cell-size row of an axis: the size the cell at `index` takes. */
export interface Grid3dAxisSize {
  index: number;
  size: Binary64;
}

/** 📏 Sparse change of one axis' cell sizes: an optional new length, then per-cell size rows. */
export interface Grid3dAxisPatch {
  length: number | null;
  sizes: Grid3dAxisSize[];
}

export interface Grid3dTilePatch {
  weight: Binary64 | null;
  media: Grid3dTile["media"] | null;
  label: Grid3dOptional<string> | null;
}

export interface Grid3dRulePatch {
  tileAId: string | null;
  tileBId: string | null;
  direction: Grid3dRule["direction"] | null;
  allowed: boolean | null;
}

export interface Grid3dPinnedPatch {
  tileId: string | null;
}

export type Grid3dMaskedPatch = Record<string, never>;

export interface Grid3dDiff {
  schema: string | null;
  seed: bigint | null;
  width: number | null;
  height: number | null;
  depth: number | null;
  cellSizesX: Grid3dAxisPatch | null;
  cellSizesY: Grid3dAxisPatch | null;
  cellSizesZ: Grid3dAxisPatch | null;
  periodicX: boolean | null;
  periodicY: boolean | null;
  periodicZ: boolean | null;
  tiles: Grid3dRows<Grid3dTile, Grid3dTilePatch>;
  rules: Grid3dRows<Grid3dRule, Grid3dRulePatch>;
  pinned: Grid3dRows<Grid3dPinnedCell, Grid3dPinnedPatch>;
  masked: Grid3dRows<Grid3dCell, Grid3dMaskedPatch>;
}

function applyAxis(sizes: readonly Binary64[], patch: Grid3dAxisPatch | null): Binary64[] {
  if (patch === null) return [...sizes];
  const out = patch.length === null ? [...sizes] : Array.from({ length: patch.length }, (_, index) => sizes[index] ?? (0 as Binary64));
  for (const row of patch.sizes) {
    if (row.index >= out.length) throw new RangeError(`cell size ${row.index} is past the axis`);
    out[row.index] = row.size;
  }
  return out;
}

function patchedTile(row: Grid3dTile, patch: Grid3dTilePatch): Grid3dTile {
  const { label: _held, ...rest } = row;
  const next = { ...rest, weight: patch.weight ?? row.weight, media: patch.media ?? row.media };
  const label = patch.label === null ? row.label : patch.label.value ?? undefined;
  return label === undefined ? next : { ...next, label };
}

/** 🔺 Applies a whole diff to a snapshot. */
export function applyGrid3dDiff(base: Grid3dSnapshot, diff: Grid3dDiff): Grid3dSnapshot {
  const cell = (item: { x: number; y: number; z: number }) => `${item.x}:${item.y}:${item.z}`;
  const cellBefore = (existing: { x: number; y: number; z: number }, added: { x: number; y: number; z: number }) => cell(existing) >= cell(added);
  return {
    ...base,
    schema: diff.schema ?? base.schema,
    seed: diff.seed ?? base.seed,
    width: diff.width ?? base.width,
    height: diff.height ?? base.height,
    depth: diff.depth ?? base.depth,
    cellSizesX: applyAxis(base.cellSizesX, diff.cellSizesX),
    cellSizesY: applyAxis(base.cellSizesY, diff.cellSizesY),
    cellSizesZ: applyAxis(base.cellSizesZ, diff.cellSizesZ),
    periodicX: diff.periodicX ?? base.periodicX,
    periodicY: diff.periodicY ?? base.periodicY,
    periodicZ: diff.periodicZ ?? base.periodicZ,
    tiles: applyRows(base.tiles, diff.tiles, (tile) => tile.id, (existing, added) => existing.id >= added.id, patchedTile),
    rules: applyRows(base.rules, diff.rules, (rule) => rule.id, (existing, added) => existing.id >= added.id, (row, patch) => ({ ...row, tileAId: patch.tileAId ?? row.tileAId, tileBId: patch.tileBId ?? row.tileBId, direction: patch.direction ?? row.direction, allowed: patch.allowed ?? row.allowed })),
    pinned: applyRows(base.pinned, diff.pinned, cell, cellBefore, (row, patch) => ({ ...row, tileId: patch.tileId ?? row.tileId })),
    masked: applyRows(base.masked, diff.masked, cell, cellBefore, (row) => row),
  };
}
