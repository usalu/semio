// 🔺️ WFC 2D diff — the TypeScript twin of `🦀️.rs`, ported field by field (never generated): a field-sparse, id-keyed delta whose rows
// are positional: removed ids at their base index, inserted rows at their after index, moved ids, and per-row field patches.

import type { Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { orderedIndex, type Wfc2dRule, type Wfc2dSlot, type Wfc2dSlotEdge, type Wfc2dSnapshot, type Wfc2dTile, type Wfc2dTileMedia } from "../📸️snapshot/🟦️.ts";

/** 🎚️ An optional field set to a value or cleared — the wire shape `{ "value": … }`. */
export type Wfc2dOptional<T> = { readonly value: T | null };

/** 🩹 One patched row, addressed by its identity. */
export type Wfc2dRowPatch<P> = { readonly id: string; readonly patch: P };

/** 📂 Positional row delta (`protocol::list_delta`): removed ids at their BASE index, inserted rows at their AFTER index, moved ids from a base to an after index, and id-keyed patches. */
export type Wfc2dRows<T, P> = {
  readonly removed: readonly { readonly id: string; readonly index: number }[];
  readonly inserted: readonly { readonly index: number; readonly row: T }[];
  readonly moved: readonly { readonly id: string; readonly from: number; readonly to: number }[];
  readonly modified: readonly Wfc2dRowPatch<P>[];
};

/** 🩹 Field patch over a slot: `null` leaves a field alone. */
export type Wfc2dSlotPatch = { readonly x: Binary64 | null; readonly y: Binary64 | null; readonly width: Binary64 | null; readonly height: Binary64 | null; readonly pinnedTileId: Wfc2dOptional<string> | null };

/** 🩹 Field patch over an adjacency edge. */
export type Wfc2dEdgePatch = { readonly fromSlotId: string | null; readonly toSlotId: string | null; readonly relation: string | null };

/** 🩹 Field patch over a tile. */
export type Wfc2dTilePatch = { readonly weight: Binary64 | null; readonly media: Wfc2dTileMedia | null; readonly label: Wfc2dOptional<string> | null };

/** 🩹 Field patch over an adjacency rule. */
export type Wfc2dRulePatch = { readonly tileAId: string | null; readonly tileBId: string | null; readonly allowed: boolean | null; readonly relation: Wfc2dOptional<string> | null };

/** 🔺️ A field-sparse, id-keyed structural delta over `Wfc2dSnapshot` — never a whole-row or whole-list capture. */
export type Wfc2dDiff = {
  readonly schema: string | null;
  readonly seed: bigint | null;
  readonly slots: Wfc2dRows<Wfc2dSlot, Wfc2dSlotPatch>;
  readonly edges: Wfc2dRows<Wfc2dSlotEdge, Wfc2dEdgePatch>;
  readonly tiles: Wfc2dRows<Wfc2dTile, Wfc2dTilePatch>;
  readonly rules: Wfc2dRows<Wfc2dRule, Wfc2dRulePatch>;
};

/** 📂 One collection's positional delta built from payload and base reads: removals take their base index, insertions the canonical id position of the running list. */
export function wfc2dRows<T extends { readonly id: string }, P>(base: readonly T[], lanes: { readonly removed?: readonly string[]; readonly added?: readonly T[]; readonly patched?: readonly Wfc2dRowPatch<P>[] } = {}): Wfc2dRows<T, P> {
  const removed = (lanes.removed ?? []).flatMap((id) => {
    const index = base.findIndex((item) => item.id === id);
    return index === -1 ? [] : [{ id, index }];
  });
  const running = base.filter((item) => !removed.some((entry) => entry.id === item.id));
  for (const row of lanes.added ?? []) running.splice(orderedIndex(running, row.id, (item) => item.id), 0, row);
  const inserted = (lanes.added ?? []).map((row) => ({ index: running.findIndex((item) => item.id === row.id), row }));
  return { removed, inserted, moved: [], modified: lanes.patched ?? [] };
}

/** 📂 An empty positional delta. */
export function emptyWfc2dRows<T, P>(): Wfc2dRows<T, P> {
  return { removed: [], inserted: [], moved: [], modified: [] };
}

/** 🕳️ The identity delta — every lane present and empty, never an omitted key. */
export function emptyWfc2dDiff(): Wfc2dDiff {
  return { schema: null, seed: null, slots: emptyWfc2dRows(), edges: emptyWfc2dRows(), tiles: emptyWfc2dRows(), rules: emptyWfc2dRows() };
}

/** 🩹 A slot patch setting only the named fields. */
export function slotPatch(id: string, fields: { readonly x?: Binary64; readonly y?: Binary64; readonly width?: Binary64; readonly height?: Binary64; readonly pinnedTileId?: string | null }): Wfc2dRowPatch<Wfc2dSlotPatch> {
  return { id, patch: { x: fields.x ?? null, y: fields.y ?? null, width: fields.width ?? null, height: fields.height ?? null, pinnedTileId: fields.pinnedTileId === undefined ? null : { value: fields.pinnedTileId } } };
}

/** 🩹 A tile patch setting only the named fields. */
export function tilePatch(id: string, fields: { readonly weight?: Binary64; readonly media?: Wfc2dTileMedia }): Wfc2dRowPatch<Wfc2dTilePatch> {
  return { id, patch: { weight: fields.weight ?? null, media: fields.media ?? null, label: null } };
}

function patchedSlot(row: Wfc2dSlot, patch: Wfc2dSlotPatch): Wfc2dSlot {
  const { pinnedTileId: _held, ...rest } = row;
  const next = { ...rest, x: patch.x ?? row.x, y: patch.y ?? row.y, width: patch.width ?? row.width, height: patch.height ?? row.height };
  const pin = patch.pinnedTileId === null ? row.pinnedTileId : patch.pinnedTileId.value ?? undefined;
  return pin === undefined ? next : { ...next, pinnedTileId: pin };
}

function patchedEdge(row: Wfc2dSlotEdge, patch: Wfc2dEdgePatch): Wfc2dSlotEdge {
  return { ...row, fromSlotId: patch.fromSlotId ?? row.fromSlotId, toSlotId: patch.toSlotId ?? row.toSlotId, relation: patch.relation ?? row.relation };
}

function patchedTile(row: Wfc2dTile, patch: Wfc2dTilePatch): Wfc2dTile {
  const { label: _held, ...rest } = row;
  const next = { ...rest, weight: patch.weight ?? row.weight, media: patch.media ?? row.media };
  const label = patch.label === null ? row.label : patch.label.value ?? undefined;
  return label === undefined ? next : { ...next, label };
}

function patchedRule(row: Wfc2dRule, patch: Wfc2dRulePatch): Wfc2dRule {
  const { relation: _held, ...rest } = row;
  const next = { ...rest, tileAId: patch.tileAId ?? row.tileAId, tileBId: patch.tileBId ?? row.tileBId, allowed: patch.allowed ?? row.allowed };
  const relation = patch.relation === null ? row.relation : patch.relation.value ?? undefined;
  return relation === undefined ? next : { ...next, relation };
}

function applyRows<T extends { readonly id: string }, P>(base: readonly T[], rows: Wfc2dRows<T, P>, patched: (row: T, patch: P) => T): T[] {
  const taken = new Set<number>();
  for (const entry of [...rows.removed, ...rows.moved.map((move) => ({ id: move.id, index: move.from }))]) {
    if (base[entry.index]?.id !== entry.id || taken.has(entry.index)) throw new RangeError(`${entry.id} is not at base index ${entry.index}`);
    taken.add(entry.index);
  }
  const slots: (T | undefined)[] = new Array(base.length - rows.removed.length + rows.inserted.length).fill(undefined);
  for (const entry of rows.inserted) {
    if (entry.index >= slots.length || slots[entry.index] !== undefined) throw new RangeError(`inserted ${entry.row.id} has no free after slot ${entry.index}`);
    slots[entry.index] = entry.row;
  }
  for (const move of rows.moved) slots[move.to] = base[move.from];
  const survivors = base.filter((_, index) => !taken.has(index));
  const items = slots.map((slot) => slot ?? survivors.shift()!);
  if (new Set(items.map((item) => item.id)).size !== items.length) throw new RangeError("two rows of the after list carry the same id");
  for (const entry of rows.modified) {
    const at = items.findIndex((item) => item.id === entry.id);
    if (at === -1) throw new RangeError(`modified ${entry.id} does not exist`);
    items[at] = patched(items[at]!, entry.patch);
  }
  return items;
}

/** 🩹 Applies a delta to a snapshot — the Rust `MutationDiff::apply` twin. */
export function applyWfc2dDiff(diff: Wfc2dDiff, base: Wfc2dSnapshot): Wfc2dSnapshot {
  return {
    schema: diff.schema ?? base.schema,
    seed: diff.seed ?? base.seed,
    slots: applyRows(base.slots, diff.slots, patchedSlot),
    edges: applyRows(base.edges, diff.edges, patchedEdge),
    tiles: applyRows(base.tiles, diff.tiles, patchedTile),
    rules: applyRows(base.rules, diff.rules, patchedRule),
  };
}
