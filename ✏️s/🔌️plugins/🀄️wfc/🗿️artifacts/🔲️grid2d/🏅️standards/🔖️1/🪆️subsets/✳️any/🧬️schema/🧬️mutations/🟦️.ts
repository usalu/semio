/** 🧬️ Grid2dMutation — one discriminated-union member per `🧬️mutations/<slug>/` triad's payload.
 * Mirrors the Rust `🦀️.rs` sibling's `Grid2dMutation` enum, which carries `#[derive(dsl::Mutations)]`
 * with no enum-level tag attribute, so it serializes EXTERNALLY TAGGED:
 * `{ "<PascalCaseVariantName>": { ...payload fields } }`. Every leaf struct declares
 * `#[value(rename_all = "camelCase")]`, so payload keys are camelCase. */
import type { WfcAdjacencyRule2d, WfcTile2d, WfcTileMedia2d } from "../📸️snapshot/🟦️.ts";

export interface ChangeSeed {
  seed: number;
}

export interface ResizeGrid {
  width: number;
  height: number;
}

export interface ChangeCellSize {
  cellWidth: number;
  cellHeight: number;
}

export interface ChangePeriodicity {
  periodicX: boolean;
  periodicY: boolean;
}

export interface CreateTile {
  tile: WfcTile2d;
}

export interface DeleteTile {
  id: string;
}

export interface ChangeTileWeight {
  id: string;
  weight: number;
}

export interface ChangeTileMedia {
  id: string;
  media: WfcTileMedia2d;
}

export interface CreateRule {
  rule: WfcAdjacencyRule2d;
}

export interface DeleteRule {
  id: string;
}

export interface PinCell {
  x: number;
  y: number;
  tileId: string;
}

export interface UnpinCell {
  x: number;
  y: number;
}

export interface MaskCell {
  x: number;
  y: number;
}

export interface UnmaskCell {
  x: number;
  y: number;
}

export type Grid2dMutation =
  | { ChangeSeed: ChangeSeed }
  | { ResizeGrid: ResizeGrid }
  | { ChangeCellSize: ChangeCellSize }
  | { ChangePeriodicity: ChangePeriodicity }
  | { CreateTile: CreateTile }
  | { DeleteTile: DeleteTile }
  | { ChangeTileWeight: ChangeTileWeight }
  | { ChangeTileMedia: ChangeTileMedia }
  | { CreateRule: CreateRule }
  | { DeleteRule: DeleteRule }
  | { PinCell: PinCell }
  | { UnpinCell: UnpinCell }
  | { MaskCell: MaskCell }
  | { UnmaskCell: UnmaskCell };

/** 🏷️ The kebab-case spelling of every variant, in declaration order. */
export const GRID2D_MUTATION_KINDS = [
  "change-seed",
  "resize-grid",
  "change-cell-size",
  "change-periodicity",
  "create-tile",
  "delete-tile",
  "change-tile-weight",
  "change-tile-media",
  "create-rule",
  "delete-rule",
  "pin-cell",
  "unpin-cell",
  "mask-cell",
  "unmask-cell",
] as const;
