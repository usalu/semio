/** 🖼️ Animate viewer — tile-editor window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the canvas-2d scene payload `render()` produces — no engagement-bar or selection-shaped
 * fields, matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One canvas-2d layer, mirroring Rust `AnimateViewTileLayer`. */
export interface AnimateViewTileLayer {
  id: string;
  kind: "image" | "source" | "tile";
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  dataUrl?: string;
}

/** 👁️ The tile-editor window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * inputs (a bare `PresentationSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface AnimateViewTileEditorViewModel {
  windowKindId: "animate-view-tile-editor";
  bodyKey: "animate.view.tile-editor";
  surfaceId: "animate.presentation.view";
  layers: AnimateViewTileLayer[];
}

export const ANIMATE_VIEW_TILE_EDITOR_WINDOW_KIND_ID = "animate-view-tile-editor" as const;
export const ANIMATE_VIEW_TILE_EDITOR_BODY_KEY = "animate.view.tile-editor" as const;
export const ANIMATE_VIEW_TILE_EDITOR_SURFACE_ID = "animate.presentation.view" as const;
