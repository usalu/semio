/** 🧬️ Wfc3dMutation — one discriminated-union member per `🧬️mutations/<slug>/` payload shape.
 * Mirrors the Rust `🦀️.rs` sibling's `Wfc3dMutation` enum, which carries only
 * `#[derive(dsl::Mutations)]` — no `#[value(tag = …)]` — so it serializes EXTERNALLY TAGGED:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`. None of the 17 leaf structs carry a
 * `rename_all`, so every leaf's field names are its literal Rust snake_case names. */
import type { GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d } from "../📸️snapshot/🟦️";

export interface CreateSlot { index: number; slot: Slot3d; }
export interface DeleteSlot { id: string; }
export interface MoveSlot { id: string; x: number; y: number; z: number; }
export interface ResizeSlot { id: string; width: number; height: number; depth: number; }
export interface ConnectSlots { index: number; edge: SlotEdge; }
export interface DisconnectSlots { id: string; }
export interface PinSlot { id: string; tileId: string; }
export interface UnpinSlot { id: string; }
export interface CreateTile { index: number; tile: Tile; }
export interface DeleteTile { id: string; }
export interface ChangeTileWeight { id: string; weight: number; }
export interface ChangeTileMedia { id: string; media: TileMedia3d; }
export interface CreateRule { index: number; rule: GraphRule; }
export interface DeleteRule { id: string; }
export interface ChangeSeed { seed: bigint; }
export interface DragSlots { targets: string[]; dx: number; dy: number; dz: number; }
export interface Wfc3dSlotPosition { id: string; x: number; y: number; z: number; }
export interface SetSlotPositions { positions: Wfc3dSlotPosition[]; }

export type Wfc3dMutation =
  | { CreateSlot: CreateSlot }
  | { DeleteSlot: DeleteSlot }
  | { MoveSlot: MoveSlot }
  | { ResizeSlot: ResizeSlot }
  | { ConnectSlots: ConnectSlots }
  | { DisconnectSlots: DisconnectSlots }
  | { PinSlot: PinSlot }
  | { UnpinSlot: UnpinSlot }
  | { CreateTile: CreateTile }
  | { DeleteTile: DeleteTile }
  | { ChangeTileWeight: ChangeTileWeight }
  | { ChangeTileMedia: ChangeTileMedia }
  | { CreateRule: CreateRule }
  | { DeleteRule: DeleteRule }
  | { ChangeSeed: ChangeSeed }
  | { DragSlots: DragSlots }
  | { SetSlotPositions: SetSlotPositions };

/** 🏷️ The kebab-case spelling of every variant, in the Rust enum's declaration order. */
export const WFC3D_MUTATION_KINDS = [
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
  "change-seed",
  "drag-slots",
  "set-slot-positions",
] as const;

/** 🧾️ The payload record of one object row: refuses a non-object, a missing key or any key outside `keys`. */
function payloadRecord(value: unknown, keys: readonly string[], what: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`Invalid ${what}`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !keys.includes(key)) || keys.some((key) => !(key in row))) throw new TypeError(`Invalid ${what}`);
  return row;
}

/** 🔢️ A finite number field, refused by name otherwise. */
function finiteNumber(raw: unknown, what: string): number {
  if (typeof raw !== "number" || !Number.isFinite(raw)) throw new TypeError(`${what} must be a finite number`);
  return raw;
}

/** ✋️ Decodes one `drag-slots` payload, refusing what its schema refuses: an unknown or missing key, no target or
 * one twice, an empty id, or a non-finite offset. */
export function parseDragSlots(value: unknown): DragSlots {
  const row = payloadRecord(value, ["targets", "dx", "dy", "dz"], "drag-slots payload");
  if (!Array.isArray(row.targets) || row.targets.length === 0 || row.targets.some((id) => typeof id !== "string" || id.length === 0)) throw new TypeError("A drag names at least one slot id");
  const targets = row.targets as string[];
  if (new Set(targets).size !== targets.length) throw new TypeError("A drag names each slot once");
  return { targets, dx: finiteNumber(row.dx, "dx"), dy: finiteNumber(row.dy, "dy"), dz: finiteNumber(row.dz, "dz") };
}

/** 🎯️ Decodes one `set-slot-positions` payload, refusing what its schema refuses: an unknown or missing key, no
 * position or one slot twice, an empty id, or a non-finite coordinate. */
export function parseSetSlotPositions(value: unknown): SetSlotPositions {
  const row = payloadRecord(value, ["positions"], "set-slot-positions payload");
  if (!Array.isArray(row.positions) || row.positions.length === 0) throw new TypeError("Positions name at least one slot");
  const positions = row.positions.map((raw): Wfc3dSlotPosition => {
    const entry = payloadRecord(raw, ["id", "x", "y", "z"], "slot position");
    if (typeof entry.id !== "string" || entry.id.length === 0) throw new TypeError("A slot position names its slot id");
    return { id: entry.id, x: finiteNumber(entry.x, "x"), y: finiteNumber(entry.y, "y"), z: finiteNumber(entry.z, "z") };
  });
  if (new Set(positions.map((position) => position.id)).size !== positions.length) throw new TypeError("Positions name each slot once");
  return { positions };
}
