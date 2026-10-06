import {binary64,binary64Value,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {wfc2dMediaEqual} from "../📸️snapshot/🟦️.ts";
// 🧬️ WFC 2D mutations — the TypeScript twin of `🦀️.rs` and its seventeen triad leaves, ported branch
// by branch from the Rust diff builders (never generated). This is the SECOND implementation the
// cross-language fixture oracle replays every committed quintet through: it must produce the same
// sparse delta, the same diagnostics and the same inverse as Rust, from the same JSON.

import { applyWfc2dDiff, emptyWfc2dDiff, type Wfc2dDiff } from "../🔺️diff/🟦️.ts";
import { orderedIndex, type Wfc2dRule, type Wfc2dSlot, type Wfc2dSlotEdge, type Wfc2dSnapshot, type Wfc2dTile, type Wfc2dTileMedia } from "../📸️snapshot/🟦️.ts";

/** 🏷️ Every kind, in the Rust enum's declaration order — the binary tag order too. */
export const WFC_2D_MUTATION_KINDS = [
  "change-seed",
  "create-slot",
  "delete-slot",
  "move-slot",
  "resize-slot",
  "connect-slots",
  "disconnect-slots",
  "pin-slot",
  "unpin-slot",
  "create-tile",
  "delete-tile",
  "change-tile-weight",
  "change-tile-media",
  "create-rule",
  "delete-rule",
  "drag-slots",
  "set-slot-positions",
] as const;

/** 🧬️ Externally tagged, exactly as the Rust enum encodes: one variant key per object. */
export type Wfc2dMutation =
  | { readonly ChangeSeed: { readonly seed: bigint } }
  | { readonly CreateSlot: { readonly slot: Wfc2dSlot } }
  | { readonly DeleteSlot: { readonly id: string } }
  | { readonly MoveSlot: { readonly id: string; readonly x: Binary64; readonly y: Binary64 } }
  | { readonly ResizeSlot: { readonly id: string; readonly width: Binary64; readonly height: Binary64 } }
  | { readonly ConnectSlots: { readonly edge: Wfc2dSlotEdge } }
  | { readonly DisconnectSlots: { readonly id: string } }
  | { readonly PinSlot: { readonly id: string; readonly tileId: string } }
  | { readonly UnpinSlot: { readonly id: string } }
  | { readonly CreateTile: { readonly tile: Wfc2dTile } }
  | { readonly DeleteTile: { readonly id: string } }
  | { readonly ChangeTileWeight: { readonly tileId: string; readonly weight: Binary64 } }
  | { readonly ChangeTileMedia: { readonly tileId: string; readonly media: Wfc2dTileMedia } }
  | { readonly CreateRule: { readonly rule: Wfc2dRule } }
  | { readonly DeleteRule: { readonly id: string } }
  | { readonly DragSlots: { readonly targets: readonly string[]; readonly dx: Binary64; readonly dy: Binary64 } }
  | { readonly SetSlotPositions: { readonly positions: readonly Wfc2dSlotPosition[] } };

/** 📌️ One slot's absolute lower corner, in document units. */
export type Wfc2dSlotPosition = { readonly id: string; readonly x: Binary64; readonly y: Binary64 };

/** 🧾️ The payload record of one object row: refuses a non-object or any key outside `keys`. */
function payloadRecord(value: unknown, keys: readonly string[], what: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`Invalid ${what}`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !keys.includes(key)) || keys.some((key) => !(key in row))) throw new TypeError(`Invalid ${what}`);
  return row;
}

/** 🔢️ A finite number field, refused by name otherwise. */
function finiteNumber(raw: unknown, what: string): Binary64 {
  if (typeof raw !== "number" || !Number.isFinite(raw)) throw new TypeError(`${what} must be a finite number`);
  return binary64(raw);
}

/** ✋️ Decodes one `drag-slots` payload, refusing what its schema refuses: an unknown or missing key, no target or
 * one twice, an empty id, or a non-finite offset. */
export function parseDragSlots(value: unknown): { readonly targets: readonly string[]; readonly dx: Binary64; readonly dy: Binary64 } {
  const row = payloadRecord(value, ["targets", "dx", "dy"], "drag-slots payload");
  if (!Array.isArray(row.targets) || row.targets.length === 0 || row.targets.some((id) => typeof id !== "string" || id.length === 0)) throw new TypeError("A drag names at least one slot id");
  const targets = row.targets as string[];
  if (new Set(targets).size !== targets.length) throw new TypeError("A drag names each slot once");
  return { targets, dx: finiteNumber(row.dx, "dx"), dy: finiteNumber(row.dy, "dy") };
}

/** 🎯️ Decodes one `set-slot-positions` payload, refusing what its schema refuses: an unknown or missing key, no
 * position or one slot twice, an empty id, or a non-finite coordinate. */
export function parseSetSlotPositions(value: unknown): { readonly positions: readonly Wfc2dSlotPosition[] } {
  const row = payloadRecord(value, ["positions"], "set-slot-positions payload");
  if (!Array.isArray(row.positions) || row.positions.length === 0) throw new TypeError("Positions name at least one slot");
  const positions = row.positions.map((raw): Wfc2dSlotPosition => {
    const entry = payloadRecord(raw, ["id", "x", "y"], "slot position");
    if (typeof entry.id !== "string" || entry.id.length === 0) throw new TypeError("A slot position names its slot id");
    return { id: entry.id, x: finiteNumber(entry.x, "x"), y: finiteNumber(entry.y, "y") };
  });
  if (new Set(positions.map((position) => position.id)).size !== positions.length) throw new TypeError("Positions name each slot once");
  return { positions };
}

/** 🎯️ One diagnostic a diff builder raised, in the committed `🎯️outcome` shape. */
export type Wfc2dMessage = { readonly level: "info" | "warning" | "error" | "fatal"; readonly code: string };

/** 🎯️ What a diff builder answers: the sparse delta plus every message it raised. */
export type Wfc2dOutcome = { readonly diff: Wfc2dDiff; readonly messages: readonly Wfc2dMessage[] };

function ok(patch: Partial<Wfc2dDiff>, messages: readonly Wfc2dMessage[] = []): Wfc2dOutcome {
  return { diff: { ...emptyWfc2dDiff(), ...patch }, messages };
}

function refuse(level: "error" | "fatal", code: string): Wfc2dOutcome {
  return { diff: emptyWfc2dDiff(), messages: [{ level, code }] };
}

function noop(): Wfc2dOutcome {
  return { diff: emptyWfc2dDiff(), messages: [{ level: "warning", code: "mutation.no-op" }] };
}

function stripPin(slot: Wfc2dSlot): Wfc2dSlot {
  const { pinnedTileId: _dropped, ...rest } = slot;
  return rest;
}

/** ⚠️ The `mutation.partial` warning a multi-slot leaf raises when some of its slots are not in the document. */
function partial(missing: readonly string[]): readonly Wfc2dMessage[] {
  return missing.length === 0 ? [] : [{ level: "warning", code: "mutation.partial" }];
}

/** 🛂️ Whether a slot id list names at least one slot and none twice. */
function distinctIds(ids: readonly string[]): boolean {
  return ids.length > 0 && new Set(ids).size === ids.length;
}

/** 🔺️ The whole dispatch: one branch per kind, mirroring each `🔺️diff/🦀️.rs` leaf exactly. */
export function wfc2dDiff(mutation: Wfc2dMutation, base: Wfc2dSnapshot): Wfc2dOutcome {
  if ("DragSlots" in mutation) {
    const { targets, dx, dy } = mutation.DragSlots;
    if (!distinctIds(targets) || !Number.isFinite(binary64Value(dx)) || !Number.isFinite(binary64Value(dy))) return refuse("fatal", "mutation.invariant");
    const missing = targets.filter((id) => !base.slots.some((slot) => slot.id === id));
    if (missing.length === targets.length) return refuse("error", "mutation.target-missing");
    if (binary64Value(dx) === 0 && binary64Value(dy) === 0) return ok({}, [...partial(missing), { level: "warning", code: "mutation.no-op" }]);
    const moved = base.slots.flatMap((slot, index) => (targets.includes(slot.id) ? [[index, { ...slot, x: binary64(binary64Value(slot.x) + binary64Value(dx)), y: binary64(binary64Value(slot.y) + binary64Value(dy)) }] as const] : []));
    return ok({ slotsUpserted: moved }, partial(missing));
  }
  if ("SetSlotPositions" in mutation) {
    const { positions } = mutation.SetSlotPositions;
    const ids = positions.map((position) => position.id);
    if (!distinctIds(ids) || positions.some((position) => !Number.isFinite(binary64Value(position.x)) || !Number.isFinite(binary64Value(position.y)))) return refuse("fatal", "mutation.invariant");
    const missing = ids.filter((id) => !base.slots.some((slot) => slot.id === id));
    if (missing.length === ids.length) return refuse("error", "mutation.target-missing");
    const moved = base.slots.flatMap((slot, index) => {
      const position = positions.find((row) => row.id === slot.id);
      return position && (binary64Value(position.x) !== binary64Value(slot.x) || binary64Value(position.y) !== binary64Value(slot.y)) ? [[index, { ...slot, x: position.x, y: position.y }] as const] : [];
    });
    if (moved.length === 0) return ok({}, [...partial(missing), { level: "warning", code: "mutation.no-op" }]);
    return ok({ slotsUpserted: moved }, partial(missing));
  }
  if ("ChangeSeed" in mutation) {
    return base.seed === mutation.ChangeSeed.seed ? noop() : ok({ seed: mutation.ChangeSeed.seed });
  }
  if ("CreateSlot" in mutation) {
    const slot = mutation.CreateSlot.slot;
    if (base.slots.some((row) => row.id === slot.id)) return refuse("fatal", "mutation.duplicate-id");
    if (binary64Value(slot.width) <= 0 || binary64Value(slot.height) <= 0) return refuse("fatal", "mutation.invariant");
    if (slot.pinnedTileId !== undefined && !base.tiles.some((tile) => tile.id === slot.pinnedTileId)) return refuse("fatal", "mutation.invariant");
    return ok({ slotsUpserted: [[orderedIndex(base.slots, slot.id, (row) => row.id), slot]] });
  }
  if ("DeleteSlot" in mutation) {
    const id = mutation.DeleteSlot.id;
    if (!base.slots.some((row) => row.id === id)) return refuse("error", "mutation.target-missing");
    const incident = base.edges.filter((edge) => edge.fromSlotId === id || edge.toSlotId === id).map((edge) => edge.id);
    return ok({ slotsRemoved: [id], edgesRemoved: incident }, incident.length === 0 ? [] : [{ level: "info", code: "mutation.cascade" }]);
  }
  if ("MoveSlot" in mutation) {
    const { id, x, y } = mutation.MoveSlot;
    const index = base.slots.findIndex((row) => row.id === id);
    if (index === -1) return refuse("error", "mutation.target-missing");
    const slot = base.slots[index]!;
    if (binary64Value(slot.x) === binary64Value(x) && binary64Value(slot.y) === binary64Value(y)) return noop();
    return ok({ slotsUpserted: [[index, { ...slot, x, y }]] });
  }
  if ("ResizeSlot" in mutation) {
    const { id, width, height } = mutation.ResizeSlot;
    const index = base.slots.findIndex((row) => row.id === id);
    if (index === -1) return refuse("error", "mutation.target-missing");
    if (binary64Value(width) <= 0 || binary64Value(height) <= 0) return refuse("fatal", "mutation.invariant");
    const slot = base.slots[index]!;
    if (binary64Value(slot.width) === binary64Value(width) && binary64Value(slot.height) === binary64Value(height)) return noop();
    return ok({ slotsUpserted: [[index, { ...slot, width, height }]] });
  }
  if ("ConnectSlots" in mutation) {
    const edge = mutation.ConnectSlots.edge;
    if (base.edges.some((row) => row.id === edge.id)) return refuse("fatal", "mutation.duplicate-id");
    for (const endpoint of [edge.fromSlotId, edge.toSlotId]) {
      if (!base.slots.some((slot) => slot.id === endpoint)) return refuse("fatal", "mutation.invariant");
    }
    if (edge.relation.length === 0) return refuse("fatal", "mutation.invariant");
    return ok({ edgesUpserted: [[orderedIndex(base.edges, edge.id, (row) => row.id), edge]] });
  }
  if ("DisconnectSlots" in mutation) {
    const id = mutation.DisconnectSlots.id;
    if (!base.edges.some((row) => row.id === id)) return refuse("error", "mutation.target-missing");
    return ok({ edgesRemoved: [id] });
  }
  if ("PinSlot" in mutation) {
    const { id, tileId } = mutation.PinSlot;
    const index = base.slots.findIndex((row) => row.id === id);
    if (index === -1) return refuse("error", "mutation.target-missing");
    if (!base.tiles.some((tile) => tile.id === tileId)) return refuse("fatal", "mutation.invariant");
    const slot = base.slots[index]!;
    if (slot.pinnedTileId === tileId) return noop();
    return ok({ slotsUpserted: [[index, { ...slot, pinnedTileId: tileId }]] });
  }
  if ("UnpinSlot" in mutation) {
    const id = mutation.UnpinSlot.id;
    const index = base.slots.findIndex((row) => row.id === id);
    if (index === -1) return refuse("error", "mutation.target-missing");
    const slot = base.slots[index]!;
    if (slot.pinnedTileId === undefined) return noop();
    return ok({ slotsUpserted: [[index, stripPin(slot)]] });
  }
  if ("CreateTile" in mutation) {
    const tile = mutation.CreateTile.tile;
    if (base.tiles.some((row) => row.id === tile.id)) return refuse("fatal", "mutation.duplicate-id");
    if (!Number.isFinite(binary64Value(tile.weight)) || binary64Value(tile.weight) < 0) return refuse("fatal", "mutation.invariant");
    return ok({ tilesUpserted: [[orderedIndex(base.tiles, tile.id, (row) => row.id), tile]] });
  }
  if ("DeleteTile" in mutation) {
    const id = mutation.DeleteTile.id;
    if (!base.tiles.some((row) => row.id === id)) return refuse("error", "mutation.target-missing");
    const orphaned = base.rules.filter((rule) => rule.tileAId === id || rule.tileBId === id).map((rule) => rule.id);
    const released = base.slots.map((slot, index) => [index, slot] as const).filter(([, slot]) => slot.pinnedTileId === id).map(([index, slot]) => [index, stripPin(slot)] as const);
    const cascaded = orphaned.length + released.length;
    return ok({ tilesRemoved: [id], rulesRemoved: orphaned, slotsUpserted: released.map(([index, slot]) => [index, slot] as const) }, cascaded === 0 ? [] : [{ level: "info", code: "mutation.cascade" }]);
  }
  if ("ChangeTileWeight" in mutation) {
    const { tileId, weight } = mutation.ChangeTileWeight;
    const index = base.tiles.findIndex((row) => row.id === tileId);
    if (index === -1) return refuse("error", "mutation.target-missing");
    if (!Number.isFinite(binary64Value(weight)) || binary64Value(weight) < 0) return refuse("fatal", "mutation.invariant");
    const tile = base.tiles[index]!;
    if (binary64Value(tile.weight) === binary64Value(weight)) return noop();
    return ok({ tilesUpserted: [[index, { ...tile, weight }]] });
  }
  if ("ChangeTileMedia" in mutation) {
    const { tileId, media } = mutation.ChangeTileMedia;
    const index = base.tiles.findIndex((row) => row.id === tileId);
    if (index === -1) return refuse("error", "mutation.target-missing");
    const tile = base.tiles[index]!;
    if (wfc2dMediaEqual(tile.media, media)) return noop();
    return ok({ tilesUpserted: [[index, { ...tile, media }]] });
  }
  if ("CreateRule" in mutation) {
    const rule = mutation.CreateRule.rule;
    if (base.rules.some((row) => row.id === rule.id)) return refuse("fatal", "mutation.duplicate-id");
    for (const tileId of [rule.tileAId, rule.tileBId]) {
      if (!base.tiles.some((tile) => tile.id === tileId)) return refuse("fatal", "mutation.invariant");
    }
    return ok({ rulesUpserted: [[orderedIndex(base.rules, rule.id, (row) => row.id), rule]] });
  }
  const id = mutation.DeleteRule.id;
  if (!base.rules.some((row) => row.id === id)) return refuse("error", "mutation.target-missing");
  return ok({ rulesRemoved: [id] });
}

/** ▶️ Applies a mutation by way of its own diff — the Rust `apply_wfc2d_mutation` twin. */
export function applyWfc2dMutation(mutation: Wfc2dMutation, base: Wfc2dSnapshot): Wfc2dSnapshot {
  return applyWfc2dDiff(wfc2dDiff(mutation, base).diff, base);
}

/** ↩️ The inverse steps, mirroring each `↩️inverse/🦀️.rs` leaf exactly. */
export function wfc2dInverse(mutation: Wfc2dMutation, base: Wfc2dSnapshot): readonly Wfc2dMutation[] {
  if ("DragSlots" in mutation) {
    const { targets, dx, dy } = mutation.DragSlots;
    if (!distinctIds(targets) || !Number.isFinite(binary64Value(dx)) || !Number.isFinite(binary64Value(dy)) || (binary64Value(dx) === 0 && binary64Value(dy) === 0)) return [];
    const positions = base.slots.filter((slot) => targets.includes(slot.id)).map((slot) => ({ id: slot.id, x: slot.x, y: slot.y }));
    return positions.length === 0 ? [] : [{ SetSlotPositions: { positions } }];
  }
  if ("SetSlotPositions" in mutation) {
    const requested = mutation.SetSlotPositions.positions;
    if (!distinctIds(requested.map((position) => position.id)) || requested.some((position) => !Number.isFinite(binary64Value(position.x)) || !Number.isFinite(binary64Value(position.y)))) return [];
    const positions = base.slots.filter((slot) => requested.some((position) => position.id === slot.id && (binary64Value(position.x) !== binary64Value(slot.x) || binary64Value(position.y) !== binary64Value(slot.y)))).map((slot) => ({ id: slot.id, x: slot.x, y: slot.y }));
    return positions.length === 0 ? [] : [{ SetSlotPositions: { positions } }];
  }
  if ("ChangeSeed" in mutation) return [{ ChangeSeed: { seed: base.seed } }];
  if ("CreateSlot" in mutation) return [{ DeleteSlot: { id: mutation.CreateSlot.slot.id } }];
  if ("DeleteSlot" in mutation) {
    const id = mutation.DeleteSlot.id;
    const slot = base.slots.find((row) => row.id === id);
    if (!slot) return [];
    const restore: Wfc2dMutation[] = [{ CreateSlot: { slot } }];
    for (const edge of base.edges) {
      if (edge.fromSlotId === id || edge.toSlotId === id) restore.push({ ConnectSlots: { edge } });
    }
    return restore;
  }
  if ("MoveSlot" in mutation) {
    const slot = base.slots.find((row) => row.id === mutation.MoveSlot.id);
    return slot ? [{ MoveSlot: { id: slot.id, x: slot.x, y: slot.y } }] : [];
  }
  if ("ResizeSlot" in mutation) {
    const slot = base.slots.find((row) => row.id === mutation.ResizeSlot.id);
    return slot ? [{ ResizeSlot: { id: slot.id, width: slot.width, height: slot.height } }] : [];
  }
  if ("ConnectSlots" in mutation) return [{ DisconnectSlots: { id: mutation.ConnectSlots.edge.id } }];
  if ("DisconnectSlots" in mutation) {
    const edge = base.edges.find((row) => row.id === mutation.DisconnectSlots.id);
    return edge ? [{ ConnectSlots: { edge } }] : [];
  }
  if ("PinSlot" in mutation) {
    const slot = base.slots.find((row) => row.id === mutation.PinSlot.id);
    if (!slot) return [];
    return slot.pinnedTileId === undefined ? [{ UnpinSlot: { id: slot.id } }] : [{ PinSlot: { id: slot.id, tileId: slot.pinnedTileId } }];
  }
  if ("UnpinSlot" in mutation) {
    const slot = base.slots.find((row) => row.id === mutation.UnpinSlot.id);
    return slot?.pinnedTileId === undefined ? [] : [{ PinSlot: { id: slot!.id, tileId: slot!.pinnedTileId! } }];
  }
  if ("CreateTile" in mutation) return [{ DeleteTile: { id: mutation.CreateTile.tile.id } }];
  if ("DeleteTile" in mutation) {
    const id = mutation.DeleteTile.id;
    const tile = base.tiles.find((row) => row.id === id);
    if (!tile) return [];
    const restore: Wfc2dMutation[] = [{ CreateTile: { tile } }];
    for (const rule of base.rules) {
      if (rule.tileAId === id || rule.tileBId === id) restore.push({ CreateRule: { rule } });
    }
    for (const slot of base.slots) {
      if (slot.pinnedTileId === id) restore.push({ PinSlot: { id: slot.id, tileId: id } });
    }
    return restore;
  }
  if ("ChangeTileWeight" in mutation) {
    const tile = base.tiles.find((row) => row.id === mutation.ChangeTileWeight.tileId);
    return tile ? [{ ChangeTileWeight: { tileId: tile.id, weight: tile.weight } }] : [];
  }
  if ("ChangeTileMedia" in mutation) {
    const tile = base.tiles.find((row) => row.id === mutation.ChangeTileMedia.tileId);
    return tile ? [{ ChangeTileMedia: { tileId: tile.id, media: tile.media } }] : [];
  }
  if ("CreateRule" in mutation) return [{ DeleteRule: { id: mutation.CreateRule.rule.id } }];
  const rule = base.rules.find((row) => row.id === mutation.DeleteRule.id);
  return rule ? [{ CreateRule: { rule } }] : [];
}
