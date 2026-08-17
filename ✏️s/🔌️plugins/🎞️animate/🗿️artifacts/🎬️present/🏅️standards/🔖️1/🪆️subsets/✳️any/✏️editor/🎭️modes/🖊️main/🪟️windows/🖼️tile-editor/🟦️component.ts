/** 🖼️ Animate editor — tile-editor window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * canvas-2d scene `render(deck: &PresentSnapshot)` boundary — one background source-figure layer plus
 * one layer per crop tile, matching `TileCanvasLayer`/`deck_to_canvas_layers` exactly. */

/** 🖼️ One canvas-2d layer — the shared source figure backdrop (`kind: "image" | "source"`) or a crop
 * tile (`kind: "tile"`), mirroring Rust `TileCanvasLayer`. */
export interface AnimateTileCanvasLayer {
  id: string;
  kind: "image" | "source" | "tile";
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  dataUrl?: string;
}

/** 🖼️ The tile-editor window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * inputs (a bare `PresentSnapshot`, no runtime/config/utility state carried alongside it). */
export interface AnimateTileEditorViewModel {
  windowKindId: "tile-editor";
  bodyKey: "animate.present.play.main";
  surfaceId: "animate.present.play";
  layers: AnimateTileCanvasLayer[];
}

export const ANIMATE_TILE_EDITOR_WINDOW_KIND_ID = "tile-editor" as const;
export const ANIMATE_TILE_EDITOR_BODY_KEY = "animate.present.play.main" as const;
export const ANIMATE_TILE_EDITOR_SURFACE_ID = "animate.present.play" as const;
