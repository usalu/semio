/** 🎬️ PresentMutation — closed semantic mutation vocabulary for the present-deck document, mirrors
 *  `🧬️mutations/🦀️.rs`'s `PresentMutation` enum and its 9 per-verb leaf structs. The enum
 *  carries NO `#[serde(tag = ...)]` — confirmed absent — so it serializes with serde's default
 *  EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, proven by
 *  every committed `🧪️tests/*​/🦠️mutation/🔣️.json` fixture (e.g. `{"RenameTile":
 *  {"id":"t-hero","newName":"Hero"}}`). Every one of the 9 leaf structs DOES carry
 *  `#[serde(rename_all = "camelCase")]`, so each leaf's own fields are camelCase on the wire —
 *  unlike layout's snake_case leaves. */
import type { FigureTileDraft, FigureTileFrame, FigureTileSource } from "../🟦️.ts";

/** 🔲️ `resize-source-frame` payload — replaces `source.frame` with `newFrame`. */
export interface ResizeSourceFrame {
  newFrame: FigureTileFrame;
}

/** 🖼️ `replace-source` payload — replaces `source` with `newSource`. */
export interface ReplaceSource {
  newSource: FigureTileSource;
}

/** 🆕️ `create-tile` payload — inserts `tile` into `tiles` at `index` (final-state append order). */
export interface CreateTile {
  index: number;
  tile: FigureTileDraft;
}

/** 🗑️ `delete-tile` payload — removes the `tiles` entry addressed by `id`. */
export interface DeleteTile {
  id: string;
}

/** 🧹️ `delete-tiles` payload — removes every `tiles` entry addressed by `ids`. */
export interface DeleteTiles {
  ids: string[];
}

/** ✏️ `rename-tile` payload — sets the `tiles` entry addressed by `id`'s `name`. */
export interface RenameTile {
  id: string;
  newName: string;
}

/** ✂️ `resize-tile-crop` payload — replaces the `tiles` entry addressed by `id`'s `crop`. */
export interface ResizeTileCrop {
  id: string;
  newCrop: FigureTileFrame;
}

/** 🔀️ `reorder-tiles` payload — moves the `tiles` entry addressed by `id` to `toIndex`. */
export interface ReorderTiles {
  id: string;
  toIndex: number;
}

/** 🔁️ `replace-tiles` payload — replaces `tiles` with `newTiles` wholesale. */
export interface ReplaceTiles {
  newTiles: FigureTileDraft[];
}

export type PresentMutation =
  | { ResizeSourceFrame: ResizeSourceFrame }
  | { ReplaceSource: ReplaceSource }
  | { CreateTile: CreateTile }
  | { DeleteTile: DeleteTile }
  | { DeleteTiles: DeleteTiles }
  | { RenameTile: RenameTile }
  | { ResizeTileCrop: ResizeTileCrop }
  | { ReorderTiles: ReorderTiles }
  | { ReplaceTiles: ReplaceTiles };
