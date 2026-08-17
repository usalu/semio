/** 🏔️ GIS 3D editor — Terrain window: typed twin of `🦀️component.rs`'s `render(document, cfg)`
 * boundary. Mirrors the Rust `World3dScene` extension payload the render function builds (camera,
 * imported-overlay pin instances, terrain descriptor), plus the window kind/body/surface ids the
 * manifest and host both address this window by. */

/** 📍️ One imported overlay pin, mirrored from `instances_json`'s Rust shape. */
export interface Gis3dTerrainPin {
  id: string;
  meshId: "pin";
  position: [number, number, number];
  color: string;
  label: string;
}

export interface Gis3dTerrainWindowViewModel {
  windowKindId: "gis3d-main";
  bodyKey: "gis3d.play.composite";
  surfaceId: "gis3d.play.composite";
  cameraJson: string;
  pins: Gis3dTerrainPin[];
  terrainJson: string;
}

export const GIS3D_PLAY_WINDOW_MAIN = "gis3d-main" as const;
export const GIS3D_PLAY_BODY_COMPOSITE = "gis3d.play.composite" as const;
