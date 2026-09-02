/** 🏔️ GIS terrain viewer — Terrain window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the World3d scene payload `render()` produces — no mutation-shaped fields (no
 * exaggeration control, no persisted camera), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One imported overlay pin, read straight off `GisTerrainSnapshot.importedFeaturesJson`. */
export interface GisTerrainViewTerrainPin {
  id: string;
  meshId: "pin";
  position: [number, number, number];
  color: string;
  label: string;
}

/** 👁️ The Terrain window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * inputs (a bare `GisTerrainSnapshot`, no runtime/config state: a viewer has none of those). */
export interface GisTerrainViewTerrainViewModel {
  windowKindId: "gis3d-view-terrain";
  bodyKey: "gis3d.view.terrain";
  cameraJson: string;
  pins: GisTerrainViewTerrainPin[];
}

export const GIS3D_VIEW_TERRAIN_WINDOW_KIND_ID = "gis3d-view-terrain" as const;
export const GIS3D_VIEW_TERRAIN_BODY_KEY = "gis3d.view.terrain" as const;
