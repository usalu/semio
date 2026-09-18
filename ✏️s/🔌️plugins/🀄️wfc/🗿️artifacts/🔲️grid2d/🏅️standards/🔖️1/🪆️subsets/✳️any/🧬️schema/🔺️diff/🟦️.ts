/** 🔺️ Grid2dDiff schema — real facet mirror of the Rust `🦀️.rs` sibling: a sparse, key-addressed
 * structural delta, never a whole-snapshot capture. Cell lanes are keyed `"<x>,<y>"`. */
import type { Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcPinnedCell2d, WfcTile2d } from "../📸️snapshot/🟦️.ts";

export interface Grid2dDiff {
  /** @state artifact */ schema?: string;
  /** @state artifact */ seed?: number;
  /** @state artifact */ width?: number;
  /** @state artifact */ height?: number;
  /** @state artifact */ cellWidth?: number;
  /** @state artifact */ cellHeight?: number;
  /** @state artifact */ periodicX?: boolean;
  /** @state artifact */ periodicY?: boolean;
  /** @state artifact */ tilesRemoved: string[];
  /** @state artifact */ tilesUpserted: [number, WfcTile2d][];
  /** @state artifact */ rulesRemoved: string[];
  /** @state artifact */ rulesUpserted: [number, WfcAdjacencyRule2d][];
  /** @state artifact */ pinnedRemoved: string[];
  /** @state artifact */ pinnedUpserted: [number, WfcPinnedCell2d][];
  /** @state artifact */ maskedRemoved: string[];
  /** @state artifact */ maskedUpserted: [number, WfcCell2d][];
}

/** 🔑️ The removal key of a grid cell — the same `"<x>,<y>"` spelling both cell lanes use. */
export function cellId(x: number, y: number): string {
  return `${x},${y}`;
}

/** 🧬️ One key-addressed collection lane's apply — the TS twin of the Rust `apply_collection`:
 * removals first, then every upsert either replaces the row that already carries its key or is
 * INSERTED at the index the diff states. Position is part of the answer, not a detail. */
function applyLane<T>(base: readonly T[], removed: readonly string[], upserted: readonly [number, T][], key: (row: T) => string): T[] {
  const rows = base.filter((row) => !removed.includes(key(row)));
  for (const [index, value] of upserted) {
    const existing = rows.findIndex((row) => key(row) === key(value));
    if (existing === -1) rows.splice(index, 0, value);
    else rows[existing] = value;
  }
  return rows;
}

/** 🩹 Applies a committed `Grid2dDiff` to a snapshot — the cross-language half of the fixture
 * oracle: Rust computes the diff, TypeScript replays it, and the committed `after` is what both
 * must reach. */
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
    tiles: applyLane(base.tiles, diff.tilesRemoved, diff.tilesUpserted, (tile) => tile.id),
    rules: applyLane(base.rules, diff.rulesRemoved, diff.rulesUpserted, (rule) => rule.id),
    pinned: applyLane(base.pinned, diff.pinnedRemoved, diff.pinnedUpserted, (cell) => cellId(cell.x, cell.y)),
    masked: applyLane(base.masked, diff.maskedRemoved, diff.maskedUpserted, (cell) => cellId(cell.x, cell.y)),
  };
}
