/** 📝️ Generation2d editor — Form window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors `render(document: &Generation2dSnapshot, generation: &GenerationPlayState,
 * labels: &Generation2dLabels)` — the selected generation's input-value form, hinting when no
 * generation is selected yet. */

export interface Generation2dGenerateFormViewModel {
  windowKindId: "generation2d-generate-form";
  bodyKey: "generation2d.play.generate-form";
  selectedGenerationId: string | null;
}

export const GENERATION2D_PLAY_WINDOW_GENERATE_FORM = "generation2d-generate-form" as const;
export const GENERATION2D_PLAY_BODY_GENERATE_FORM = "generation2d.play.generate-form" as const;
