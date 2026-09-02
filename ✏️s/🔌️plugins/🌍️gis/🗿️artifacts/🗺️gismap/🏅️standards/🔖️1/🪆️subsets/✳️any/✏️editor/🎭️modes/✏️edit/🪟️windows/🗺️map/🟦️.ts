/** 🗺️ GIS 2D editor — Map window: typed twin of `🦀️.rs`'s `render(document, cfg)`
 * boundary. Mirrors the Rust `TiledMapScene` payload the render function builds, plus the window
 * kind/body/surface ids the manifest and host both address this window by. */

/** 🧮️ Mirror of `Gis2dConfig`'s per-window-relevant fields — the editor's own session view state
 * (`✏️editor/🎚️config/🦀️.rs`), reused here rather than redeclared per window. */
export interface Gis2dMapWindowViewModel {
  windowKindId: "gis2d-main";
  bodyKey: "gis2d.play.composite";
  surfaceId: "gis2d.play.composite";
  mapFixtureJson: string;
  cameraJson: string;
  renderMode: "image" | "vector" | "combined";
  vectorStyle: "colored" | "figureGround" | "invertedFigure";
  lodMode: string;
  layerVisibilityJson: string;
  layerStrokeScaleJson: string;
}

export const GIS2D_PLAY_WINDOW_MAIN = "gis2d-main" as const;
export const GIS2D_PLAY_BODY_COMPOSITE = "gis2d.play.composite" as const;

export * as vectorStyleOption from "./🎚️options/🎨️vector-style/🟦️component";
export * as layersOption from "./🎚️options/👁️layers/🟦️component";
export * as layerWeightsOption from "./🎚️options/📏️layer-weights/🟦️component";
export * as lodModeOption from "./🎚️options/🔽️lod-mode/🟦️component";
export * as renderModeOption from "./🎚️options/🖼️render-mode/🟦️component";
