// 🔺️ WFC 2D diff — the TypeScript twin of `🦀️.rs`, ported field by field (never generated): a field-sparse, id-keyed delta whose rows
// are removed identities, added rows (landing at their canonical position) and per-row field patches.

import type { Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { orderedIndex, type Wfc2dRule, type Wfc2dSlot, type Wfc2dSlotEdge, type Wfc2dSnapshot, type Wfc2dTile, type Wfc2dTileMedia } from "../📸️snapshot/🟦️.ts";

/** 🎚️ An optional field set to a value or cleared — the wire shape `{ "value": … }`. */
export type Wfc2dOptional<T> = { readonly value: T | null };

/** 🩹 One patched row, addressed by its identity. */
export type Wfc2dRowPatch<P> = { readonly id: string; readonly patch: P };

/** 📂 Id-keyed row delta: removed identities, added rows and per-row field patches. */
export type Wfc2dRows<T, P> = { readonly removed: readonly string[]; readonly added: readonly T[]; readonly patched: readonly Wfc2dRowPatch<P>[] };

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

/** 📂 One collection's delta with the named lanes filled in. */
export function wfc2dRows<T, P>(lanes: Partial<Wfc2dRows<T, P>> = {}): Wfc2dRows<T, P> {
  return { removed: lanes.removed ?? [], added: lanes.added ?? [], patched: lanes.patched ?? [] };
}

/** 🕳️ The identity delta — every lane present and empty, never an omitted key. */
export function emptyWfc2dDiff(): Wfc2dDiff {
  return { schema: null, seed: null, slots: wfc2dRows(), edges: wfc2dRows(), tiles: wfc2dRows(), rules: wfc2dRows() };
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
  const items = [...base];
  for (const id of rows.removed) {
    const at = items.findIndex((item) => item.id === id);
    if (at === -1) throw new RangeError(`removed ${id} does not exist`);
    items.splice(at, 1);
  }
  for (const row of rows.added) {
    if (items.some((item) => item.id === row.id)) throw new RangeError(`added ${row.id} already exists`);
    items.splice(orderedIndex(items, row.id, (item) => item.id), 0, row);
  }
  for (const entry of rows.patched) {
    const at = items.findIndex((item) => item.id === entry.id);
    if (at === -1) throw new RangeError(`patched ${entry.id} does not exist`);
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
