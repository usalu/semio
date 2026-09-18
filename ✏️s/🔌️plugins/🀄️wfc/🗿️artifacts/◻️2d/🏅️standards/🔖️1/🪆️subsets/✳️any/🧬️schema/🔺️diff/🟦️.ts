// 🔺️ WFC 2D diff — the TypeScript twin of `🦀️.rs`, ported field by field (never generated).

import type { Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile } from "../📸️snapshot/🟦️.ts";

/** 🔺️ One indexed upsert: the FINAL-state position plus the row that belongs there. */
export type Wfc2dUpsert<T> = readonly [number, T];

/** 🔺️ A sparse, id-keyed structural delta over `Wfc2dSnapshot` — never a whole-snapshot capture. */
export type Wfc2dDiff = {
  readonly schema: string | null;
  readonly seed: number | null;
  readonly slotsRemoved: readonly string[];
  readonly slotsUpserted: readonly Wfc2dUpsert<Wfc2dSlot>[];
  readonly edgesRemoved: readonly string[];
  readonly edgesUpserted: readonly Wfc2dUpsert<Wfc2dSlotEdge>[];
  readonly tilesRemoved: readonly string[];
  readonly tilesUpserted: readonly Wfc2dUpsert<Wfc2dTile>[];
  readonly rulesRemoved: readonly string[];
  readonly rulesUpserted: readonly Wfc2dUpsert<Wfc2dRule>[];
};

/** 🕳️ The identity delta — every lane present and empty, never an omitted key. */
export function emptyWfc2dDiff(): Wfc2dDiff {
  return {
    schema: null,
    seed: null,
    slotsRemoved: [],
    slotsUpserted: [],
    edgesRemoved: [],
    edgesUpserted: [],
    tilesRemoved: [],
    tilesUpserted: [],
    rulesRemoved: [],
    rulesUpserted: [],
  };
}

function applyCollection<T>(base: readonly T[], removed: readonly string[], upserted: readonly Wfc2dUpsert<T>[], key: (item: T) => string): T[] {
  const items = base.filter((item) => !removed.includes(key(item)));
  for (const [index, value] of upserted) {
    const at = items.findIndex((item) => key(item) === key(value));
    if (at === -1) items.splice(index, 0, value);
    else items[at] = value;
  }
  return items;
}

/** 🩹 Applies a delta to a snapshot — the Rust `MutationDiff::apply` twin. */
export function applyWfc2dDiff(diff: Wfc2dDiff, base: Wfc2dSnapshot): Wfc2dSnapshot {
  return {
    schema: diff.schema ?? base.schema,
    seed: diff.seed ?? base.seed,
    slots: applyCollection(base.slots, diff.slotsRemoved, diff.slotsUpserted, (slot) => slot.id),
    edges: applyCollection(base.edges, diff.edgesRemoved, diff.edgesUpserted, (edge) => edge.id),
    tiles: applyCollection(base.tiles, diff.tilesRemoved, diff.tilesUpserted, (tile) => tile.id),
    rules: applyCollection(base.rules, diff.rulesRemoved, diff.rulesUpserted, (rule) => rule.id),
  };
}
