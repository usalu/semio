/** 👁️ Procedural2d viewer — Preview window: typed twin of `🦀️component.rs`'s view-model. Read-only
 * mirror of the schematic canvas scene `render()` produces — no mutation-shaped fields (no eval
 * session, no camera persistence), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One schematic widget box, read straight off `Procedural2dSnapshot.fixture`. */
export interface Procedural2dViewPreviewLayer {
  id: string;
  kind: "node";
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

/** 👁️ The Preview window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Procedural2dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Procedural2dViewPreviewViewModel {
  windowKindId: "procedural2d-view-preview";
  bodyKey: "procedural2d.view.preview";
  layers: Procedural2dViewPreviewLayer[];
}

export const PROCEDURAL2D_VIEW_PREVIEW_WINDOW_KIND_ID = "procedural2d-view-preview" as const;
export const PROCEDURAL2D_VIEW_PREVIEW_BODY_KEY = "procedural2d.view.preview" as const;
