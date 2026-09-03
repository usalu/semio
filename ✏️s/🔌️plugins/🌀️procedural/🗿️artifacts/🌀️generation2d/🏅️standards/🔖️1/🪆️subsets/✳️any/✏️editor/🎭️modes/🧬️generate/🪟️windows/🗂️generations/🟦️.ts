/** 🗂️ Generation2d editor — Generations window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors `render(generation: &GenerationPlayState, locale: Locale, terminology:
 * Terminology)` — the generations list tree, add/remove/rename/select actions dispatched by row. */

export interface Generation2dGenerationsViewModel {
  windowKindId: "generation2d-generations";
  bodyKey: "generation2d.play.generations";
  generationIds: string[];
  selectedGenerationId: string | null;
}

export const GENERATION2D_PLAY_WINDOW_GENERATIONS = "generation2d-generations" as const;
export const GENERATION2D_PLAY_BODY_GENERATIONS = "generation2d.play.generations" as const;
