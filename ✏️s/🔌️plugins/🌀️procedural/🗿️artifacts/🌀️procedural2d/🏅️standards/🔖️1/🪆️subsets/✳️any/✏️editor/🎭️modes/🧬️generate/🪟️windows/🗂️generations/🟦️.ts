/** 🗂️ Procedural2d editor — Generations window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors `render(generation: &GenerationPlayState, locale: Locale, terminology:
 * Terminology)` — the generations list tree, add/remove/rename/select actions dispatched by row. */

export interface Procedural2dGenerationsViewModel {
  windowKindId: "procedural2d-generations";
  bodyKey: "procedural2d.play.generations";
  generationIds: string[];
  selectedGenerationId: string | null;
}

export const PROCEDURAL2D_PLAY_WINDOW_GENERATIONS = "procedural2d-generations" as const;
export const PROCEDURAL2D_PLAY_BODY_GENERATIONS = "procedural2d.play.generations" as const;
