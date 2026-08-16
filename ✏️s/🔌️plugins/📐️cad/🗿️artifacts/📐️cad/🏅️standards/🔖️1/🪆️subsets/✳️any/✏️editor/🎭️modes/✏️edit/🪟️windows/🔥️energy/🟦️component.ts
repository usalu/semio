/** 🔥️ CAD editor — Energy window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the pane's `render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions)`
 * boundary — the world-3d scene payload plus the Dislocate gumball/utility state a mutation-capable
 * surface carries (absent entirely from the viewer's read-only twin, see `👁️viewer/…/🟦️component.ts`). */

/** ✏️ Per-window Dislocate gumball handle toggles — mirrors Rust `CadDislocateOptions`. */
export interface CadDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}

/** ✏️ The Energy window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface CadEnergyViewModel {
  windowKindId: "cad-play-energy";
  bodyKey: "cad.play.energy";
  surfaceId: "cad.play.scene3d/energy";
  pane: "energy";
  activeUtilityId: string | null;
  dislocateOptions: CadDislocateOptions;
}

export const CAD_PLAY_ENERGY_WINDOW_KIND_ID = "cad-play-energy" as const;
export const CAD_PLAY_ENERGY_BODY_KEY = "cad.play.energy" as const;
export const CAD_PLAY_ENERGY_SURFACE_ID = "cad.play.scene3d/energy" as const;
