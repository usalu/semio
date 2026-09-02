/** 👁️ Layout viewer — Preview window: typed twin of `🦀️.rs`'s view-model. Read-only mirror
 * of the canvas-2d layer payload `render()` produces — no camera/chrome/mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract (a viewer has no persisted per-session config). */

/** 👁️ One host canvas-2d layer, mirroring the Rust `host_layer` JSON shape. */
export interface LayoutViewPreviewLayer {
  id: string;
  segments: unknown;
  fill?: { color: [number, number, number, number] };
  stroke?: { color: [number, number, number, number]; width: number };
}

/** 👁️ The Preview window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * inputs (a bare `LayoutSnapshot`, no runtime/config/camera state: a viewer has none of those). */
export interface LayoutViewPreviewViewModel {
  windowKindId: "layout-view-preview";
  bodyKey: "layout.view.preview";
  surfaceId: "layout.view.preview";
  layers: LayoutViewPreviewLayer[];
}

export const LAYOUT_VIEW_PREVIEW_WINDOW_KIND_ID = "layout-view-preview" as const;
export const LAYOUT_VIEW_PREVIEW_BODY_KEY = "layout.view.preview" as const;
export const LAYOUT_VIEW_PREVIEW_SURFACE_ID = "layout.view.preview" as const;
