/** 👁️ Generation2d viewer — Preview window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the schematic canvas scene `render()` produces — no mutation-shaped fields (no eval
 * session, no camera persistence), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One schematic widget box, read straight off `Generation2dSnapshot.fixture`. */
export interface Generation2dViewPreviewLayer {
  id: string;
  kind: "node";
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

/** 👁️ The Preview window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Generation2dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Generation2dViewPreviewViewModel {
  windowKindId: "generation2d-view-preview";
  bodyKey: "generation2d.view.preview";
  layers: Generation2dViewPreviewLayer[];
}

export const GENERATION2D_VIEW_PREVIEW_WINDOW_KIND_ID = "generation2d-view-preview" as const;
export const GENERATION2D_VIEW_PREVIEW_BODY_KEY = "generation2d.view.preview" as const;
