/** 🗂️ Generation3d editor — Generations window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors the pane's `render(generation: &GenerationPlayState, locale: Locale,
 * terminology: Terminology)` boundary — the generations tree with add/remove/rename/select actions. */

/** ✏️ The Generations window's typed view-model. */
export interface Generation3dGenerationsViewModel {
  windowKindId: "generation3d-generations";
  bodyKey: "procedural.play.generations";
  /** 🧬️ Every generation currently declared, in display order. */
  generations: Array<{ id: string; name: string }>;
  /** 🧬️ The selected generation id, or null when none is selected. */
  selectedGenerationId: string | null;
  /** 🗣️ BCP-47 locale tag driving label resolution. */
  locale: string;
}

export const GENERATION3D_PLAY_GENERATIONS_WINDOW_KIND_ID = "generation3d-generations" as const;
export const GENERATION3D_PLAY_GENERATIONS_BODY_KEY = "procedural.play.generations" as const;
