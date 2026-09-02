/** 📝️ Procedural2d editor — Form window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors `render(document: &Procedural2dSnapshot, generation: &GenerationPlayState,
 * labels: &Procedural2dLabels)` — the selected generation's input-value form, hinting when no
 * generation is selected yet. */

export interface Procedural2dGenerateFormViewModel {
  windowKindId: "procedural2d-generate-form";
  bodyKey: "procedural2d.play.generate-form";
  selectedGenerationId: string | null;
}

export const PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM = "procedural2d-generate-form" as const;
export const PROCEDURAL2D_PLAY_BODY_GENERATE_FORM = "procedural2d.play.generate-form" as const;
