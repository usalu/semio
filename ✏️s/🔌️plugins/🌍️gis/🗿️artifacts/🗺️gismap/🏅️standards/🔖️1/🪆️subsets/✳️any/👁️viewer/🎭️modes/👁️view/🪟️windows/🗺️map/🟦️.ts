/** 🗺️ GIS map viewer — Map window: typed twin of `🦀️.rs`'s view-model. Read-only mirror of
 * the tiled-map scene payload `render()` produces — no mutation-shaped fields (no layer toggles, no
 * persisted camera), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ The Map window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `GisMapSnapshot`, no runtime/config state: a viewer has none of those). */
export interface GisMapViewMapViewModel {
  windowKindId: "gis2d-view-map";
  bodyKey: "gis2d.view.map";
  cameraJson: string;
  renderMode: "combined";
  vectorStyle: "colored";
  lodMode: "automatic";
}

export const GIS2D_VIEW_MAP_WINDOW_KIND_ID = "gis2d-view-map" as const;
export const GIS2D_VIEW_MAP_BODY_KEY = "gis2d.view.map" as const;
