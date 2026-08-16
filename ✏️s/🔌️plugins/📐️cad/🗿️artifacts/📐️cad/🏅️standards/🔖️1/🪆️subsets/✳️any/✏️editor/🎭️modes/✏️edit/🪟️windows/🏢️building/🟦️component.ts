/** 🏢️ CAD editor — Building window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the pane's `render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions)`
 * boundary — the world-3d scene payload plus the Dislocate gumball/utility state a mutation-capable
 * surface carries (absent entirely from the viewer's read-only twin, see `👁️viewer/…/🟦️component.ts`). */

/** ✏️ Per-window Dislocate gumball handle toggles — mirrors Rust `CadDislocateOptions`. */
export interface CadDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}

/** ✏️ The Building window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface CadBuildingViewModel {
  windowKindId: "cad-play-building";
  bodyKey: "cad.play.building";
  surfaceId: "cad.play.scene3d/building";
  pane: "building";
  activeUtilityId: string | null;
  dislocateOptions: CadDislocateOptions;
}

export const CAD_PLAY_BUILDING_WINDOW_KIND_ID = "cad-play-building" as const;
export const CAD_PLAY_BUILDING_BODY_KEY = "cad.play.building" as const;
export const CAD_PLAY_BUILDING_SURFACE_ID = "cad.play.scene3d/building" as const;
