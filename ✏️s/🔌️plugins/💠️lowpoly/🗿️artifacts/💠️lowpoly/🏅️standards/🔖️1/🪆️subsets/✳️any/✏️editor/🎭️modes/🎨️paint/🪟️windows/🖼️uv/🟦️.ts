/** 🖼️ Lowpoly editor — UV window: typed twin of `🦀️.rs`'s `render()` boundary. The 2D
 * UV-canvas paint surface — only the paint operations it shares with the Model window are scoped
 * here (no mesh-editing/transform ops). */

/** 🎨️ One paint layer's live pixel-cache entry, keyed by object id — the TS mirror of the Rust
 * `texture_cache: &HashMap<String, String>` render parameter (base64 PNG per active object). */
export type LowpolyUvTextureCache = Record<string, string>;

/** 🖼️ One 2D canvas overlay layer this window's scene builds (`uv_canvas_layers_json`) — the paint
 * texture image, or the UV wireframe polyline with its seam flags. */
export type LowpolyUvCanvasLayer =
  | { id: "uv-paint-texture"; kind: "image"; name: "Paint"; x: number; y: number; width: number; height: number; dataUrl: string }
  | { id: "uv-wireframe"; kind: "polyline"; name: "UV Wireframe"; points: [number, number][]; seams: number[] };

/** 🖼️ The UV window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (`LowpolyView` = snapshot + config, the loaded compute-session document, the per-object texture
 * cache). */
export interface LowpolyUvViewModel {
  windowKindId: "lowpoly-uv";
  bodyKey: "lowpoly.play.uv";
  surfaceId: "lowpoly.play.uv";
  textureCache: LowpolyUvTextureCache;
  layers: LowpolyUvCanvasLayer[];
}

export const LOWPOLY_UV_WINDOW_KIND_ID = "lowpoly-uv" as const;
export const LOWPOLY_UV_BODY_KEY = "lowpoly.play.uv" as const;
export const LOWPOLY_UV_SURFACE_ID = "lowpoly.play.uv" as const;
