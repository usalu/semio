/** 📐️ CAD editor — Shape window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the pane's `render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions)`
 * boundary — the world-3d scene payload plus the Dislocate gumball/utility state a mutation-capable
 * surface carries (absent entirely from the viewer's read-only twin, see `👁️viewer/…/🟦️component.ts`). */

/** ✏️ Per-window Dislocate gumball handle toggles — mirrors Rust `CadDislocateOptions`. */
export interface CadDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}

/** ✏️ The Shape window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface CadShapeViewModel {
  windowKindId: "cad-play-shape";
  bodyKey: "cad.play.shape";
  surfaceId: "cad.play.scene3d/shape";
  pane: "shape";
  activeUtilityId: string | null;
  dislocateOptions: CadDislocateOptions;
}

export const CAD_PLAY_SHAPE_WINDOW_KIND_ID = "cad-play-shape" as const;
export const CAD_PLAY_SHAPE_BODY_KEY = "cad.play.shape" as const;
export const CAD_PLAY_SHAPE_SURFACE_ID = "cad.play.scene3d/shape" as const;
