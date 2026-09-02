/** 🌐️ Lowpoly editor — Model window: typed twin of `🦀️.rs`'s `render()` boundary. The live
 * 3D world-3d mesh scene — every mesh-editing/transform/UV-unwrap operation runs here; paint
 * operations are scoped on BOTH this window and the UV window since the paint utilities apply to
 * both (see the surface root `🦀️.rs`'s shared-options doc comment). */

/** 🎨️ One paint layer's live pixel-cache entry, keyed by object id — the TS mirror of the Rust
 * `texture_cache: &HashMap<String, String>` render parameter (base64 PNG per active object). */
export type LowpolyModelTextureCache = Record<string, string>;

/** 🧲️ The active transform/paint utility id driving the gumball/brush chrome — mirrors the Rust
 * `active_utility: &str` render parameter. */
export type LowpolyModelActiveUtility = "move" | "rotate" | "scale" | "brush" | "eraser" | "fill" | "eyedropper";

/** 🌐️ The Model window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (`LowpolyView` = snapshot + config, the loaded compute-session document, the active utility id, the
 * per-object texture cache). */
export interface LowpolyModelViewModel {
  windowKindId: "lowpoly-main";
  bodyKey: "lowpoly.play.main";
  surfaceId: "lowpoly.play.main";
  activeUtility: LowpolyModelActiveUtility;
  textureCache: LowpolyModelTextureCache;
}

export const LOWPOLY_MODEL_WINDOW_KIND_ID = "lowpoly-main" as const;
export const LOWPOLY_MODEL_BODY_KEY = "lowpoly.play.main" as const;
export const LOWPOLY_MODEL_SURFACE_ID = "lowpoly.play.main" as const;
