/** 🏛️ CAD editor — Structure Classic window: typed twin of `🦀️.rs`'s view-model. Mirrors
 * the pane's `render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions)`
 * boundary — the world-3d scene payload plus the Dislocate gumball/utility state a mutation-capable
 * surface carries (absent entirely from the viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

/** ✏️ Per-window Dislocate gumball handle toggles — mirrors Rust `CadDislocateOptions`. */
export interface CadDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}

/** ✏️ The Structure Classic window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface CadStructureClassicViewModel {
  windowKindId: "cad-play-structure-classic";
  bodyKey: "cad.play.structure-classic";
  surfaceId: "cad.play.scene3d/structure-classic";
  pane: "structure-classic";
  activeUtilityId: string | null;
  dislocateOptions: CadDislocateOptions;
}

export const CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_KIND_ID = "cad-play-structure-classic" as const;
export const CAD_PLAY_STRUCTURE_CLASSIC_BODY_KEY = "cad.play.structure-classic" as const;
export const CAD_PLAY_STRUCTURE_CLASSIC_SURFACE_ID = "cad.play.scene3d/structure-classic" as const;
