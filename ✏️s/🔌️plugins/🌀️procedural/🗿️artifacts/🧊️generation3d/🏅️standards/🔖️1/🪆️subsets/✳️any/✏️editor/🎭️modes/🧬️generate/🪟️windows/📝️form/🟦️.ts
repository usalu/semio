/** 📝️ Generation3d editor — Form window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors the pane's `render(fixture: &FlowFixture, generation: &GenerationPlayState,
 * labels: &Generation3dLabels)` boundary — the input-slider/note form derived from the fixture for
 * the currently selected generation, dispatching `updateGenerationValues` on edit. */

/** ✏️ The Form window's typed view-model. */
export interface Generation3dGenerateFormViewModel {
  windowKindId: "generation3d-generate-form";
  bodyKey: "procedural.play.generate-form";
  /** 🧬️ The selected generation id, or null when the hint text renders instead of a form. */
  selectedGenerationId: string | null;
  /** 📝️ Current field values for the selected generation, keyed by widget id. */
  values: Record<string, unknown>;
}

export const GENERATION3D_PLAY_GENERATE_FORM_WINDOW_KIND_ID = "generation3d-generate-form" as const;
export const GENERATION3D_PLAY_GENERATE_FORM_BODY_KEY = "procedural.play.generate-form" as const;
