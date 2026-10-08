import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🔺️ Grid2dDiff schema — real facet mirror of the Rust `🦀️.rs` sibling: scalars are optional absolute values, every list carries removed keys,
 * added rows (landing at their canonical position) and per-row field patches — never a whole row or list copy. Cell rows are keyed `"<x>,<y>"`. */
import type { Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcPinnedCell2d, WfcTile2d } from "../📸️snapshot/🟦️.ts";

/** 🎚️ An optional field set to a value or cleared — the wire shape `{ "value": … }`. */
export interface Grid2dOptional<T> {
  value: T | null;
}

/** 🩹 One patched row, addressed by its identity. */
export interface Grid2dRowPatch<Q> {
  id: string;
  patch: Q;
}

/** 📂 Positional row delta (`protocol::list_delta`): removed keys at their BASE index, inserted rows at their AFTER index, moved keys, and key-addressed patches. */
export interface Grid2dRows<T, Q> {
  removed: { id: string; index: number }[];
  inserted: { index: number; row: T }[];
  moved: { id: string; from: number; to: number }[];
  modified: Grid2dRowPatch<Q>[];
}

/** 🕳️ An empty row delta. */
export function emptyGrid2dRows<T, Q>(): Grid2dRows<T, Q> {
  return { removed: [], inserted: [], moved: [], modified: [] };
}

/** 🧬️ One keyed collection's apply — the TS twin of the Rust positional list-delta apply: removed and moved keys are checked at their base index, inserted and moved rows take their after slots, survivors fill the rest in base order, then the patches write. */
function applyRows<T, Q>(base: readonly T[], rows: Grid2dRows<T, Q>, key: (row: T) => string, patched: (row: T, patch: Q) => T): T[] {
  const taken = new Set<number>();
  for (const entry of [...rows.removed, ...rows.moved.map((move) => ({ id: move.id, index: move.from }))]) {
    if (base[entry.index] === undefined || key(base[entry.index]!) !== entry.id || taken.has(entry.index)) throw new RangeError(`${entry.id} is not at base index ${entry.index}`);
    taken.add(entry.index);
  }
  const slots: (T | undefined)[] = new Array(base.length - rows.removed.length + rows.inserted.length).fill(undefined);
  for (const entry of rows.inserted) {
    if (entry.index >= slots.length || slots[entry.index] !== undefined) throw new RangeError(`inserted ${key(entry.row)} has no free after slot ${entry.index}`);
    slots[entry.index] = entry.row;
  }
  for (const move of rows.moved) slots[move.to] = base[move.from];
  const survivors = base.filter((_, index) => !taken.has(index));
  const items = slots.map((slot) => slot ?? survivors.shift()!);
  if (new Set(items.map(key)).size !== items.length) throw new RangeError("two rows of the after list carry the same key");
  for (const entry of rows.modified) {
    const at = items.findIndex((item) => key(item) === entry.id);
    if (at === -1) throw new RangeError(`modified ${entry.id} does not exist`);
    items[at] = patched(items[at]!, entry.patch);
  }
  return items;
}

export interface Grid2dTilePatch {
  weight: Binary64 | null;
  media: WfcTile2d["media"] | null;
  label: Grid2dOptional<string> | null;
}

export interface Grid2dRulePatch {
  tileAId: string | null;
  tileBId: string | null;
  direction: WfcAdjacencyRule2d["direction"] | null;
  allowed: boolean | null;
}

export interface Grid2dPinnedPatch {
  tileId: string | null;
}

export type Grid2dMaskedPatch = Record<string, never>;

export interface Grid2dDiff {
  /** @state artifact */ schema?: string | null;
  /** @state artifact */ seed?: bigint | null;
  /** @state artifact */ width?: number | null;
  /** @state artifact */ height?: number | null;
  /** @state artifact */ cellWidth?: Binary64 | null;
  /** @state artifact */ cellHeight?: Binary64 | null;
  /** @state artifact */ periodicX?: boolean | null;
  /** @state artifact */ periodicY?: boolean | null;
  /** @state artifact */ tiles: Grid2dRows<WfcTile2d, Grid2dTilePatch>;
  /** @state artifact */ rules: Grid2dRows<WfcAdjacencyRule2d, Grid2dRulePatch>;
  /** @state artifact */ pinned: Grid2dRows<WfcPinnedCell2d, Grid2dPinnedPatch>;
  /** @state artifact */ masked: Grid2dRows<WfcCell2d, Grid2dMaskedPatch>;
}

/** 🔑️ The removal key of a grid cell — the same `"<x>,<y>"` spelling both cell lanes use. */
export function cellId(x: number, y: number): string {
  return `${x},${y}`;
}

function patchedTile(row: WfcTile2d, patch: Grid2dTilePatch): WfcTile2d {
  const { label: _held, ...rest } = row;
  const next = { ...rest, weight: patch.weight ?? row.weight, media: patch.media ?? row.media };
  const label = patch.label === null ? row.label : patch.label.value ?? undefined;
  return label === undefined ? next : { ...next, label };
}

/** 🩹 Applies a committed `Grid2dDiff` to a snapshot — the cross-language half of the fixture oracle: Rust computes the diff, TypeScript replays it, and the
 * committed `after` is what both must reach. */
export function applyGrid2dDiff(base: Grid2dSnapshot, diff: Grid2dDiff): Grid2dSnapshot {
  return {
    ...base,
    schema: diff.schema ?? base.schema,
    seed: diff.seed ?? base.seed,
    width: diff.width ?? base.width,
    height: diff.height ?? base.height,
    cellWidth: diff.cellWidth ?? base.cellWidth,
    cellHeight: diff.cellHeight ?? base.cellHeight,
    periodicX: diff.periodicX ?? base.periodicX,
    periodicY: diff.periodicY ?? base.periodicY,
    tiles: applyRows(base.tiles, diff.tiles, (tile) => tile.id, patchedTile),
    rules: applyRows(base.rules, diff.rules, (rule) => rule.id, (row, patch) => ({ ...row, tileAId: patch.tileAId ?? row.tileAId, tileBId: patch.tileBId ?? row.tileBId, direction: patch.direction ?? row.direction, allowed: patch.allowed ?? row.allowed })),
    pinned: applyRows(base.pinned, diff.pinned, (cell) => cellId(cell.x, cell.y), (row, patch) => ({ ...row, tileId: patch.tileId ?? row.tileId })),
    masked: applyRows(base.masked, diff.masked, (cell) => cellId(cell.x, cell.y), (row) => row),
  };
}
