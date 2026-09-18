/** 🧬️ Wfc3dMutation — one discriminated-union member per `🧬️mutations/<slug>/` payload shape.
 * Mirrors the Rust `🦀️.rs` sibling's `Wfc3dMutation` enum, which carries only
 * `#[derive(dsl::Mutations)]` — no `#[value(tag = …)]` — so it serializes EXTERNALLY TAGGED:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`. None of the 15 leaf structs carry a
 * `rename_all`, so every leaf's field names are its literal Rust snake_case names. */
import type { GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d } from "../📸️snapshot/🟦️";

export interface CreateSlot { index: number; slot: Slot3d; }
export interface DeleteSlot { id: string; }
export interface MoveSlot { id: string; x: number; y: number; z: number; }
export interface ResizeSlot { id: string; width: number; height: number; depth: number; }
export interface ConnectSlots { index: number; edge: SlotEdge; }
export interface DisconnectSlots { id: string; }
export interface PinSlot { id: string; tile_id: string; }
export interface UnpinSlot { id: string; }
export interface CreateTile { index: number; tile: Tile; }
export interface DeleteTile { id: string; }
export interface ChangeTileWeight { id: string; weight: number; }
export interface ChangeTileMedia { id: string; media: TileMedia3d; }
export interface CreateRule { index: number; rule: GraphRule; }
export interface DeleteRule { id: string; }
export interface ChangeSeed { seed: number; }

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
  | { ChangeSeed: ChangeSeed };

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
] as const;
